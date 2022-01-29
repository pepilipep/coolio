use std::cmp::min;
use std::collections::HashMap;

use async_trait::async_trait;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

use rspotify::model::AlbumId;
use rspotify::model::ArtistId;
use rspotify::model::FullArtist;
use rspotify::model::Market;
use rspotify::model::PlayableItem;
use rspotify::model::PlaylistId;
use rspotify::model::SearchResult;
use rspotify::model::SearchType;
use rspotify::model::TrackId;
use rspotify::prelude::*;
use rspotify::AuthCodeSpotify;

use crate::error::CoolioError;
use crate::models::Playlist;
use crate::storage::Storage;

#[async_trait]
pub trait Playlists<S: Storage + Send + Sync> {
    fn get_spotify(&self) -> &AuthCodeSpotify;
    fn get_storage(&self) -> &S;

    async fn list_playlists(&self) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let limit = 50;
        let mut offset = 0;

        let mut playlists = Vec::<Playlist>::new();

        loop {
            let fetched = spotify
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await?;

            for playlist in fetched.items {
                playlists.push(Playlist {
                    id: playlist.id.uri(),
                    name: playlist.name,
                    artists: Vec::<String>::new(),
                    automated: false,
                })
            }

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

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

    async fn create_playlist(&self, name: &str) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();
        let me = spotify.current_user().await?;
        let playlist = spotify
            .user_playlist_create(&me.id, name, Some(false), None, None)
            .await?;
        storage
            .create_playlist(&playlist.id.uri(), &playlist.name)
            .await?;

        Ok(())
    }

    async fn automate_playlist(&self, name: &str) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let mut offset = 0;
        let limit = 50;
        let mut playlist_id: Option<String> = None;
        loop {
            let fetched = spotify
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await?;

            for item in fetched.items {
                if item.name == name {
                    playlist_id = Some(item.id.uri());
                    break;
                }
            }

            if fetched.next.is_none() {
                break;
            }
            offset += limit;
        }

        match playlist_id {
            None => Err(CoolioError::from("The playlist doesn't exist")),
            Some(id) => storage.create_playlist(&id, &name).await,
        }
    }

    async fn seed_artist_popular_into_playlist(
        &self,
        artist_id: &String,
        playlist_id: &String,
        seed: usize,
    ) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();

        let tracks = spotify
            .artist_top_tracks(&ArtistId::from_uri(artist_id)?, &Market::FromToken)
            .await?;

        let seed = min(seed, tracks.len());

        let seed_track_ids = tracks[..seed]
            .iter()
            .map(|x| x.id.as_ref().unwrap() as &dyn PlayableId)
            .collect::<Vec<&dyn PlayableId>>();

        spotify
            .playlist_add_items(&PlaylistId::from_uri(playlist_id)?, seed_track_ids, Some(0))
            .await?;

        Ok(())
    }

    async fn link_playlist_to_artist(
        &self,
        playlist: &str,
        artist: &str,
        seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let playlist = storage.get_playlist(playlist).await?;

        let potentials_artists = spotify
            .search(artist, &SearchType::Artist, None, None, Some(5), None)
            .await?;

        if let SearchResult::Artists(artists) = potentials_artists {
            let mut count_id = 1;
            println!("choose one of the following artists:");
            for art in &artists.items {
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
                    if choice >= 1 && choice < artists.items.len() {
                        chosen = choice;
                        break;
                    }
                }
                println!("Wrong choice. Try again")
            }

            let chosen_artist_id = &artists.items[chosen - 1].id;

            storage
                .link_artist(&playlist.id, &playlist.name, &chosen_artist_id.uri())
                .await?;

            if let Some(seed) = seed {
                self.seed_artist_popular_into_playlist(&chosen_artist_id.uri(), &playlist.id, seed)
                    .await?;
            }

            Ok(())
        } else {
            unreachable!()
        }
    }

    async fn unlink_artist_from_playlist(
        &self,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let playlist = storage.get_playlist(playlist).await?;

        let potentials_artists = spotify
            .search(artist, &SearchType::Artist, None, None, Some(5), None)
            .await?;

        if let SearchResult::Artists(artists) = potentials_artists {
            let mut potential = Vec::<&FullArtist>::new();
            for art in &artists.items {
                if playlist.artists.contains(&art.id.uri()) {
                    potential.push(art);
                }
            }

            match potential.len() {
                0 => Err(CoolioError::from(
                    "no artists in the playlist matched your search",
                )),
                1 => {
                    storage
                        .unlink_artist(&playlist.id, &potential[0].id.uri())
                        .await
                }
                _ => Err(CoolioError::from(
                    "ambigious artists found, try again more concrete",
                )),
            }
        } else {
            unreachable!()
        }
    }

    async fn artits_new_albums(
        &self,
        artist_id: &String,
        last_added: &DateTime<Utc>,
    ) -> Result<Vec<String>, CoolioError> {
        let spotify = self.get_spotify();
        let limit = 50;
        let mut offset = 0;
        let mut album_ids = Vec::<String>::new();

        loop {
            let fetched = spotify
                .artist_albums_manual(
                    &ArtistId::from_uri(artist_id)?,
                    None,
                    None,
                    Some(limit),
                    Some(offset),
                )
                .await?;

            for album in fetched.items {
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

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

        Ok(album_ids)
    }

    async fn albums_to_tracks(&self, albums: Vec<String>) -> Result<Vec<TrackId>, CoolioError> {
        let spotify = self.get_spotify();
        let mut tracks_to_add = Vec::<TrackId>::new();

        for album_id_to_add in albums {
            let mut offset = 0;
            let limit = 50;

            loop {
                let fetched = spotify
                    .album_track_manual(
                        &AlbumId::from_uri(&album_id_to_add)?,
                        Some(limit),
                        Some(offset),
                    )
                    .await?;

                for track in fetched.items {
                    tracks_to_add.push(track.id.unwrap());
                }

                if fetched.next.is_none() {
                    break;
                }

                offset += limit;
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
        let spotify = self.get_spotify();

        let album_ids = self.artits_new_albums(artist_id, last_added).await?;

        let tracks = self.albums_to_tracks(album_ids).await?;

        if tracks.len() > 0 {
            let tracks_as_playable_ids = tracks.iter().map(|x| x as &dyn PlayableId);

            spotify
                .playlist_add_items(
                    &PlaylistId::from_uri(playlist_id)?,
                    tracks_as_playable_ids,
                    Some(0),
                )
                .await?;
        }

        Ok(())
    }

    async fn playlist_artist_last_add(
        &self,
        playlist: &Playlist,
    ) -> Result<HashMap<String, DateTime<Utc>>, CoolioError> {
        let spotify = self.get_spotify();
        let external_playlist = spotify
            .playlist(&PlaylistId::from_uri(&playlist.id)?, None, None)
            .await?;

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
                None => {
                    self.seed_artist_popular_into_playlist(artist_id, &playlist.id, 5)
                        .await?
                }
                Some(last_added) => {
                    self.artist_add_last(artist_id, &playlist.id, last_added)
                        .await?
                }
            }
        }

        Ok(())
    }

    async fn playlists_update(&self) -> Result<(), CoolioError> {
        let storage = self.get_storage();

        let playlists = storage.get_playlists().await?;

        for playlist in playlists {
            self.playlist_update(&playlist).await?
        }

        Ok(())
    }
}
