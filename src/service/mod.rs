use std::sync::Arc;

use async_trait::async_trait;

use crate::{error::CoolioError, models::ThrowbackPeriod, storage::Storage};

use self::{history::HistoryService, playlists::PlaylistService, spotify::Spotify};

pub mod history;
pub mod playlists;
pub mod spotify;

pub struct Service<S: Spotify> {
    pub history: HistoryService<S>,
    pub playlists: PlaylistService<S>,
}

impl<S: Spotify> Service<S> {
    pub fn new(spotify: S, storage: Arc<dyn Storage>) -> Self {
        let s = Arc::new(spotify);
        Service {
            history: HistoryService::new(Arc::clone(&s), Arc::clone(&storage)),
            playlists: PlaylistService::new(Arc::clone(&s), Arc::clone(&storage)),
        }
    }
}

#[async_trait]
pub trait ServiceTrait: Send + Sync {
    async fn history_update(&self) -> Result<(), CoolioError>;

    async fn throwback(
        &self,
        name: Option<&str>,
        period: Option<ThrowbackPeriod>,
        size: Option<usize>,
    ) -> Result<(), CoolioError>;

    async fn playlists_list(&self) -> Result<(), CoolioError>;

    async fn playlists_show(&self, name: &str) -> Result<(), CoolioError>;

    async fn playlists_create(&self, name: &str) -> Result<(), CoolioError>;

    async fn playlists_automate(&self, name: &str) -> Result<(), CoolioError>;

    async fn link_playlist_to_artist(
        &self,
        playlist: &str,
        artist: &str,
        seed: Option<usize>,
    ) -> Result<(), CoolioError>;

    async fn unlink_artist_from_playlist(
        &self,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError>;

    async fn playlists_update(&self) -> Result<(), CoolioError>;
}

#[async_trait]
impl<S: Spotify> ServiceTrait for Service<S> {
    async fn history_update(&self) -> Result<(), CoolioError> {
        self.history.update().await
    }

    async fn throwback(
        &self,
        name: Option<&str>,
        period: Option<ThrowbackPeriod>,
        size: Option<usize>,
    ) -> Result<(), CoolioError> {
        self.history.throwback(name, period, size).await
    }

    async fn playlists_list(&self) -> Result<(), CoolioError> {
        self.playlists.list().await
    }

    async fn playlists_show(&self, name: &str) -> Result<(), CoolioError> {
        self.playlists.show(name).await
    }

    async fn playlists_create(&self, name: &str) -> Result<(), CoolioError> {
        self.playlists.create(name).await
    }

    async fn playlists_automate(&self, name: &str) -> Result<(), CoolioError> {
        self.playlists.automate(name).await
    }

    async fn link_playlist_to_artist(
        &self,
        playlist: &str,
        artist: &str,
        seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        self.playlists
            .link_playlist_to_artist(playlist, artist, seed)
            .await
    }

    async fn unlink_artist_from_playlist(
        &self,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError> {
        self.playlists
            .unlink_artist_from_playlist(playlist, artist)
            .await
    }

    async fn playlists_update(&self) -> Result<(), CoolioError> {
        self.playlists.update().await
    }
}
