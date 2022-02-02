use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rspotify::model::{
    AlbumId, AlbumType, ArtistId, FullArtist, FullPlaylist, FullTrack, Market, PlaylistId,
    SearchResult, SearchType, SimplifiedAlbum, SimplifiedTrack, TrackId,
};
use rspotify::prelude::*;
use rspotify::{
    model::{PlayHistory, TimeLimits},
    AuthCodeSpotify,
};

use crate::error::CoolioError;
use crate::models::Playlist;

#[async_trait]
pub trait Spotify {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<PlayHistory>, CoolioError>;

    async fn create_playlist(&self, name: &str) -> Result<FullPlaylist, CoolioError>;

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &PlaylistId,
        items: impl IntoIterator<Item = String> + Send + 'a,
    ) -> Result<(), CoolioError>;

    async fn current_user_playlists(&self) -> Result<Vec<Playlist>, CoolioError>;
    async fn playlist(&self, id: &str) -> Result<FullPlaylist, CoolioError>;
    async fn artist(&self, id: &str) -> Result<FullArtist, CoolioError>;
    async fn artist_top_tracks(&self, name: &str) -> Result<Vec<FullTrack>, CoolioError>;
    async fn search_artists(&self, name: &str) -> Result<Vec<FullArtist>, CoolioError>;
    async fn artist_albums(
        &self,
        id: &str,
        album_type: &AlbumType,
    ) -> Result<Vec<SimplifiedAlbum>, CoolioError>;
    async fn album_tracks(&self, id: &str) -> Result<Vec<SimplifiedTrack>, CoolioError>;
}

pub struct HTTPSpotify {
    spotify: AuthCodeSpotify,
}

impl HTTPSpotify {
    pub fn new(spotify: AuthCodeSpotify) -> Self {
        HTTPSpotify { spotify }
    }
}

#[async_trait]
impl Spotify for HTTPSpotify {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<PlayHistory>, CoolioError> {
        let last_listen = time_limit.map(|x| TimeLimits::After(x));

        Ok(self
            .spotify
            .current_user_recently_played(Some(limit), last_listen)
            .await?
            .items)
    }

    async fn create_playlist(&self, name: &str) -> Result<FullPlaylist, CoolioError> {
        let me = self.spotify.current_user().await?;
        let playlist = self
            .spotify
            .user_playlist_create(&me.id, name, None, None, None)
            .await?;
        Ok(playlist)
    }

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &PlaylistId,
        items: impl IntoIterator<Item = String> + Send + 'a,
    ) -> Result<(), CoolioError> {
        let please_live = items
            .into_iter()
            .map(|x| TrackId::from_uri(&x).unwrap())
            .collect::<Vec<TrackId>>();

        let to_add = please_live
            .iter()
            .map(|x| x as &dyn PlayableId)
            .collect::<Vec<&dyn PlayableId>>();

        self.spotify
            .playlist_add_items(playlist_id, to_add, Some(0))
            .await?;
        Ok(())
    }

    async fn current_user_playlists(&self) -> Result<Vec<Playlist>, CoolioError> {
        let limit = 50;
        let mut offset = 0;

        let mut playlists = Vec::<Playlist>::new();

        loop {
            let fetched = self
                .spotify
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

        Ok(playlists)
    }

    async fn playlist(&self, id: &str) -> Result<FullPlaylist, CoolioError> {
        let p = self
            .spotify
            .playlist(&PlaylistId::from_uri(id)?, None, None)
            .await?;
        Ok(p)
    }

    async fn artist(&self, id: &str) -> Result<FullArtist, CoolioError> {
        let p = self.spotify.artist(&ArtistId::from_uri(id)?).await?;
        Ok(p)
    }

    async fn artist_top_tracks(&self, id: &str) -> Result<Vec<FullTrack>, CoolioError> {
        let tracks = self
            .spotify
            .artist_top_tracks(&ArtistId::from_uri(id)?, &Market::FromToken)
            .await?;
        Ok(tracks)
    }

    async fn search_artists(&self, name: &str) -> Result<Vec<FullArtist>, CoolioError> {
        let r = self
            .spotify
            .search(name, &SearchType::Artist, None, None, Some(5), None)
            .await?;

        match r {
            SearchResult::Artists(a) => Ok(a.items),
            _ => unreachable!(),
        }
    }

    async fn artist_albums(
        &self,
        id: &str,
        album_type: &AlbumType,
    ) -> Result<Vec<SimplifiedAlbum>, CoolioError> {
        let limit = 50;
        let mut offset = 0;
        let mut albums = Vec::<SimplifiedAlbum>::new();

        loop {
            let mut fetched = self
                .spotify
                .artist_albums_manual(
                    &ArtistId::from_uri(id)?,
                    Some(album_type),
                    None,
                    Some(limit),
                    Some(offset),
                )
                .await?;

            albums.append(&mut fetched.items);

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

        Ok(albums)
    }

    async fn album_tracks(&self, id: &str) -> Result<Vec<SimplifiedTrack>, CoolioError> {
        let mut offset = 0;
        let limit = 50;

        let mut tracks = Vec::<SimplifiedTrack>::new();

        loop {
            let mut fetched = self
                .spotify
                .album_track_manual(&AlbumId::from_uri(id)?, Some(limit), Some(offset))
                .await?;

            tracks.append(&mut fetched.items);

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }
        Ok(tracks)
    }
}
