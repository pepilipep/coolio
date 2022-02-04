use std::cmp::min;
use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;

use rspotify::model::AlbumType;

use crate::error::CoolioError;
use crate::models::Playlist;
use crate::storage::Storage;
use crate::storage::StorageBehavior;

use super::spotify::SimpleArtist;
use super::spotify::SimpleTrack;
use super::spotify::Spotify;

pub struct PlaylistService {}

impl PlaylistService {
    pub async fn list(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
    ) -> Result<(), CoolioError> {
        let mut playlists = spotify.current_user_playlists().await?;
        let mut stored_playlists = storage.get_playlists().await?;

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

    pub async fn show(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        name: &str,
    ) -> Result<(), CoolioError> {
        let playlist = storage.get_playlist(name).await?;
        let external_playlist = spotify.playlist(&playlist.id).await?;

        println!("Artists:");
        for art_id in playlist.artists {
            let artist = spotify.artist(&art_id).await?;
            println!(
                "\t{} (popularity: {}, followers: {})",
                artist.name, artist.popularity, artist.num_followers
            );
        }

        println!("Description: {:?}", external_playlist.description);
        println!("Number of tracks: {}", external_playlist.tracks.len());
        println!("Number of followers: {}", external_playlist.num_followers);
        println!("Is collaborative: {}", external_playlist.collaborative);
        println!("Is public: {:?}", external_playlist.public);

        Ok(())
    }

    pub async fn create(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        name: &str,
    ) -> Result<(), CoolioError> {
        let playlist = spotify.create_playlist(name).await?;
        storage
            .create_playlist(&playlist.id, &playlist.name)
            .await?;

        Ok(())
    }

    pub async fn automate(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        name: &str,
    ) -> Result<(), CoolioError> {
        let playlists = spotify.current_user_playlists().await?;
        for p in playlists {
            if p.name == name {
                return storage.create_playlist(&p.id, name).await;
            }
        }
        Err("The playlist doesn't exist".into())
    }

    async fn seed_artist_popular(
        &self,
        spotify: &impl Spotify,
        _storage: &StorageBehavior,
        artist_id: &String,
        playlist_id: &String,
        seed: usize,
    ) -> Result<(), CoolioError> {
        let tracks = spotify.artist_top_tracks(artist_id).await?;
        let seed = min(seed, tracks.len());
        spotify
            .playlist_add_items(playlist_id, tracks[..seed].iter().map(|x| x.id.clone()))
            .await?;

        Ok(())
    }

    pub async fn link_playlist_to_artist(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        playlist: &str,
        artist: &str,
        seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        let playlist = storage.get_playlist(playlist).await?;

        let artists = spotify.search_artists(artist).await?;

        let mut count_id = 1;
        println!("choose one of the following artists:");
        for art in &artists {
            println!(
                "[{}] {} (followers: {})",
                count_id, art.name, art.num_followers
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

        storage
            .link_artist(&playlist.id, &playlist.name, chosen_artist_id)
            .await?;

        if let Some(seed) = seed {
            self.seed_artist_popular(spotify, storage, chosen_artist_id, &playlist.id, seed)
                .await?;
        }

        Ok(())
    }

    pub async fn unlink_artist_from_playlist(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError> {
        let playlist = storage.get_playlist(playlist).await?;

        let potentials = spotify
            .search_artists(artist)
            .await?
            .into_iter()
            .filter(|x| playlist.artists.contains(&x.id))
            .collect::<Vec<SimpleArtist>>();

        match potentials.len() {
            0 => Err("no artists in the playlist matched your search".into()),
            1 => storage.unlink_artist(&playlist.id, &potentials[0].id).await,
            _ => Err("ambigious artists found, try again more concrete".into()),
        }
    }

    async fn artists_new_albums_filter(
        &self,
        spotify: &impl Spotify,
        _storage: &StorageBehavior,
        artist_id: &String,
        last_added: &DateTime<Utc>,
        album_type: &AlbumType,
    ) -> Result<Vec<String>, CoolioError> {
        let albums = spotify.artist_albums(artist_id, album_type).await?;
        let mut album_ids = Vec::<String>::new();

        for album in albums {
            if album.release_date > *last_added {
                album_ids.push(album.id);
            }
        }

        Ok(album_ids)
    }

    async fn artists_new_albums(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        artist_id: &String,
        last_added: &DateTime<Utc>,
    ) -> Result<Vec<String>, CoolioError> {
        let mut all = Vec::<String>::new();
        for t in &[AlbumType::Album, AlbumType::Single] {
            let f = self
                .artists_new_albums_filter(spotify, storage, artist_id, last_added, &t)
                .await?;

            for t in f {
                if !all.contains(&t) {
                    all.push(t);
                }
            }
        }
        Ok(all)
    }

    async fn albums_to_tracks(
        &self,
        spotify: &impl Spotify,
        _storage: &StorageBehavior,
        albums: Vec<String>,
    ) -> Result<Vec<SimpleTrack>, CoolioError> {
        let mut tracks_to_add = Vec::<SimpleTrack>::new();

        for album_id_to_add in albums {
            let mut tracks = spotify.album_tracks(&album_id_to_add).await?;
            tracks_to_add.append(&mut tracks);
        }

        Ok(tracks_to_add)
    }

    async fn artist_add_last(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        artist_id: &String,
        playlist_id: &String,
        last_added: &DateTime<Utc>,
    ) -> Result<(), CoolioError> {
        let album_ids = self
            .artists_new_albums(spotify, storage, artist_id, last_added)
            .await?;
        let tracks = self.albums_to_tracks(spotify, storage, album_ids).await?;
        if tracks.len() > 0 {
            spotify
                .playlist_add_items(playlist_id, tracks.into_iter().map(|x| x.id))
                .await?;
        }

        Ok(())
    }

    async fn playlist_artist_last_add(
        &self,
        spotify: &impl Spotify,
        _storage: &StorageBehavior,
        playlist: &Playlist,
    ) -> Result<HashMap<String, DateTime<Utc>>, CoolioError> {
        let external_playlist = spotify.playlist(&playlist.id).await?;

        let mut last_song_for_artist = HashMap::<String, DateTime<Utc>>::new();
        for track in external_playlist.tracks {
            if let Some(added_at) = track.added_at {
                for art in track.track.artists {
                    if let Some(added_last) = last_song_for_artist.get(&art.id) {
                        if added_at > *added_last {
                            last_song_for_artist.insert(art.id, added_at);
                        }
                    } else {
                        last_song_for_artist.insert(art.id, added_at);
                    }
                }
            }
        }
        Ok(last_song_for_artist)
    }

    async fn playlist_update(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
        playlist: &Playlist,
    ) -> Result<(), CoolioError> {
        let last_song_for_artist = self
            .playlist_artist_last_add(spotify, storage, playlist)
            .await?;

        for artist_id in &playlist.artists {
            match last_song_for_artist.get(artist_id) {
                None => {
                    self.seed_artist_popular(spotify, storage, artist_id, &playlist.id, 5)
                        .await?
                }
                Some(last_added) => {
                    self.artist_add_last(spotify, storage, artist_id, &playlist.id, last_added)
                        .await?
                }
            }
        }

        Ok(())
    }

    pub async fn update(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
    ) -> Result<(), CoolioError> {
        let playlists = storage.get_playlists().await?;
        for playlist in playlists {
            self.playlist_update(spotify, storage, &playlist).await?
        }
        Ok(())
    }
}
