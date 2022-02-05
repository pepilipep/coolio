use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rspotify::model::AlbumType;
use tokio::sync::Mutex;

use crate::{
    error::CoolioError,
    models::{Listen, Playlist},
    service::spotify::{SimpleAlbum, SimpleArtist, SimplePlaylist, SimpleTrack, Spotify},
};

#[derive(Clone, Default)]
pub struct SpotifyState {
    playlists: Vec<SimplePlaylist>,
}

pub struct MockSpotify {
    state: Mutex<SpotifyState>,
}

impl MockSpotify {
    pub fn new() -> Self {
        MockSpotify {
            state: Mutex::new(SpotifyState::default()),
        }
    }
}

#[async_trait]
impl Spotify for MockSpotify {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<Listen>, CoolioError> {
        Ok(vec![
            Listen {
                song_id: "song_id_1".to_string(),
                time: Utc::now(),
            },
            Listen {
                song_id: "song_id_2".to_string(),
                time: Utc::now(),
            },
        ])
    }

    async fn create_playlist(&self, name: &str) -> Result<SimplePlaylist, CoolioError> {
        let p = SimplePlaylist::default();
        Ok(p)
    }

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &str,
        items: impl IntoIterator<Item = String> + Send + 'a,
    ) -> Result<(), CoolioError> {
        Ok(())
    }

    async fn current_user_playlists(&self) -> Result<Vec<SimplePlaylist>, CoolioError> {
        let p = SimplePlaylist::default();
        Ok(vec![p])
    }

    async fn artist_top_tracks(&self, name: &str) -> Result<Vec<SimpleTrack>, CoolioError> {
        Err("bad".into())
    }
    async fn album_tracks(&self, id: &str) -> Result<Vec<SimpleTrack>, CoolioError> {
        Err("bad".into())
    }
    async fn artist_albums(
        &self,
        id: &str,
        album_type: &AlbumType,
    ) -> Result<Vec<SimpleAlbum>, CoolioError> {
        Err("bad".into())
    }

    async fn playlist(&self, id: &str) -> Result<SimplePlaylist, CoolioError> {
        Err("bad".into())
    }
    async fn artist(&self, id: &str) -> Result<SimpleArtist, CoolioError> {
        Err("bad".into())
    }
    async fn search_artists(&self, name: &str) -> Result<Vec<SimpleArtist>, CoolioError> {
        Err("bad".into())
    }
}
