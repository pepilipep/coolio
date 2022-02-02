use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

use rspotify::model::AlbumType;
use rspotify::model::FullArtist;
use rspotify::model::PlayableItem;
use rspotify::model::PlaylistId;
use rspotify::model::TrackId;
use rspotify::prelude::*;

use crate::error::CoolioError;
use crate::models::Playlist;
use crate::storage::Storage;

use super::spotify::Spotify;

pub struct PlaylistService<S: Spotify> {
    spotify: Rc<S>,
    storage: Rc<dyn Storage>,
}

impl<S: Spotify> PlaylistService<S> {
    pub fn new(spotify: Rc<S>, storage: Rc<dyn Storage>) -> Self {
        PlaylistService { spotify, storage }
    }

    pub async fn list(&self) -> Result<(), CoolioError> {
        let mut playlists = self.spotify.current_user_playlists().await?;
        let mut stored_playlists = self.storage.get_playlists().await?;

        stored_playlists.append(&mut playlists);
        stored_playlists.dedup_by_key(|p| p.id.clone());

        for playlist in stored_playlists {
            if playlist.automated {
                println!(
                    "{} [automated, number of artists: {}]",
                    playlist.name,
                    playlist.artists.len()
                )
            } else {
                println!("{}", playlist.name);
            }
        }

        Ok(())
    }

    pub async fn show(&self, name: &str) -> Result<(), CoolioError> {
        let playlist = self.storage.get_playlist(name).await?;
        let external_playlist = self.spotify.playlist(&playlist.id).await?;

        println!("Artists:");
        for art_id in playlist.artists {
            let artist = self.spotify.artist(&art_id).await?;
            println!(
                "\t{} (popularity: {}, followers: {})",
                artist.name, artist.popularity, artist.followers.total
            );
        }

        println!("Description: {:?}", external_playlist.description);
        println!("Number of tracks: {}", external_playlist.tracks.total);
        println!("Number of followers: {}", external_playlist.followers.total);
        println!("Is collaborative: {}", external_playlist.collaborative);
        println!("Is public: {:?}", external_playlist.public);

        Ok(())
    }

    pub async fn create(&self, name: &str) -> Result<(), CoolioError> {
        let playlist = self.spotify.create_playlist(name).await?;
        self.storage
            .create_playlist(&playlist.id.uri(), &playlist.name)
            .await?;

        Ok(())
    }

    pub async fn automate(&self, name: &str) -> Result<(), CoolioError> {
        let playlists = self.spotify.current_user_playlists().await?;
        for p in playlists {
            if p.name == name {
                return self.storage.create_playlist(&p.id, name).await;
            }
        }
        Err("The playlist doesn't exist".into())
    }

    async fn seed_artist_popular(
        &self,
        artist_id: &String,
        playlist_id: &String,
        seed: usize,
    ) -> Result<(), CoolioError> {
        let tracks = self.spotify.artist_top_tracks(artist_id).await?;
        let seed = min(seed, tracks.len());
        self.spotify
            .playlist_add_items(
                &PlaylistId::from_uri(playlist_id)?,
                tracks[..seed].iter().map(|x| x.id.as_ref().unwrap().uri()),
            )
            .await?;

        Ok(())
    }

    pub async fn link_playlist_to_artist(
        &self,
        playlist: &str,
        artist: &str,
        seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        let playlist = self.storage.get_playlist(playlist).await?;

        let artists = self.spotify.search_artists(artist).await?;

        let mut count_id = 1;
        println!("choose one of the following artists:");
        for art in &artists {
            println!(
                "[{}] {} (followers: {})",
                count_id, art.name, art.followers.total
            );
            count_id += 1;
        }

        let chosen: usize;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            input = input.trim().to_string();

            if let Ok(choice) = input.parse::<usize>() {
                if choice >= 1 && choice < artists.len() {
                    chosen = choice;
                    break;
                }
            }
            println!("Wrong choice. Try again")
        }

        let chosen_artist_id = &artists[chosen - 1].id;

        self.storage
            .link_artist(&playlist.id, &playlist.name, &chosen_artist_id.uri())
            .await?;

        if let Some(seed) = seed {
            self.seed_artist_popular(&chosen_artist_id.uri(), &playlist.id, seed)
                .await?;
        }

        Ok(())
    }

    pub async fn unlink_artist_from_playlist(
        &self,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError> {
        let playlist = self.storage.get_playlist(playlist).await?;

        let potentials = self
            .spotify
            .search_artists(artist)
            .await?
            .into_iter()
            .filter(|x| playlist.artists.contains(&x.id.uri()))
            .collect::<Vec<FullArtist>>();

        match potentials.len() {
            0 => Err("no artists in the playlist matched your search".into()),
            1 => {
                self.storage
                    .unlink_artist(&playlist.id, &potentials[0].id.uri())
                    .await
            }
            _ => Err("ambigious artists found, try again more concrete".into()),
        }
    }

    async fn artists_new_albums_filter(
        &self,
        artist_id: &String,
        last_added: &DateTime<Utc>,
        album_type: &AlbumType,
    ) -> Result<Vec<String>, CoolioError> {
        let albums = self.spotify.artist_albums(artist_id, album_type).await?;
        let mut album_ids = Vec::<String>::new();

        for album in albums {
            if let Some(release_date) = album.release_date {
                if let Some("day") = album.release_date_precision.as_ref().map(|x| x.as_str()) {
                    if DateTime::<Utc>::from_utc(
                        NaiveDateTime::parse_from_str(
                            &(release_date + " 00:00:00"),
                            "%Y-%m-%d %H:%M:%S",
                        )?,
                        Utc,
                    ) > *last_added
                    {
                        album_ids.push(album.id.unwrap().uri());
                    }
                }
            }
        }

        Ok(album_ids)
    }

    async fn artists_new_albums(
        &self,
        artist_id: &String,
        last_added: &DateTime<Utc>,
    ) -> Result<Vec<String>, CoolioError> {
        let mut all = Vec::<String>::new();
        for t in &[AlbumType::Album, AlbumType::Single] {
            let f = self
                .artists_new_albums_filter(artist_id, last_added, &t)
                .await?;

            for t in f {
                if !all.contains(&t) {
                    all.push(t);
                }
            }
        }
        Ok(all)
    }

    async fn albums_to_tracks(&self, albums: Vec<String>) -> Result<Vec<TrackId>, CoolioError> {
        let mut tracks_to_add = Vec::<TrackId>::new();

        for album_id_to_add in albums {
            let tracks = self.spotify.album_tracks(&album_id_to_add).await?;
            for track in tracks {
                tracks_to_add.push(track.id.unwrap());
            }
        }

        Ok(tracks_to_add)
    }

    async fn artist_add_last(
        &self,
        artist_id: &String,
        playlist_id: &String,
        last_added: &DateTime<Utc>,
    ) -> Result<(), CoolioError> {
        let album_ids = self.artists_new_albums(artist_id, last_added).await?;
        let tracks = self.albums_to_tracks(album_ids).await?;
        if tracks.len() > 0 {
            self.spotify
                .playlist_add_items(
                    &PlaylistId::from_uri(playlist_id)?,
                    tracks.into_iter().map(|x| x.uri()),
                )
                .await?;
        }

        Ok(())
    }

    async fn playlist_artist_last_add(
        &self,
        playlist: &Playlist,
    ) -> Result<HashMap<String, DateTime<Utc>>, CoolioError> {
        let external_playlist = self.spotify.playlist(&playlist.id).await?;

        let mut last_song_for_artist = HashMap::<String, DateTime<Utc>>::new();
        for track in external_playlist.tracks.items {
            if let Some(added_at) = track.added_at {
                if let PlayableItem::Track(track) = track.track.unwrap() {
                    for art in track.artists {
                        let art_id = art.id.unwrap().uri();
                        if let Some(added_last) = last_song_for_artist.get(&art_id) {
                            if added_at > *added_last {
                                last_song_for_artist.insert(art_id, added_at);
                            }
                        } else {
                            last_song_for_artist.insert(art_id, added_at);
                        }
                    }
                }
            }
        }
        Ok(last_song_for_artist)
    }

    async fn playlist_update(&self, playlist: &Playlist) -> Result<(), CoolioError> {
        let last_song_for_artist = self.playlist_artist_last_add(playlist).await?;

        for artist_id in &playlist.artists {
            match last_song_for_artist.get(artist_id) {
                None => self.seed_artist_popular(artist_id, &playlist.id, 5).await?,
                Some(last_added) => {
                    self.artist_add_last(artist_id, &playlist.id, last_added)
                        .await?
                }
            }
        }

        Ok(())
    }

    pub async fn update(&self) -> Result<(), CoolioError> {
        let playlists = self.storage.get_playlists().await?;
        for playlist in playlists {
            self.playlist_update(&playlist).await?
        }
        Ok(())
    }
}
