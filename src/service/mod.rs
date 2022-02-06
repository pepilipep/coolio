use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;
use std::io::Stdin;
use std::io::Stdout;

use async_trait::async_trait;

use crate::{error::CoolioError, models::ThrowbackPeriod, storage::StorageBehavior};

use self::io::Interactor;
use self::{history::HistoryService, playlists::PlaylistService, spotify::Spotify};

pub mod history;
pub mod io;
pub mod playlists;
pub mod spotify;

pub struct Service<'a, S: Spotify> {
    pub spotify: &'a S,
    pub storage: &'a StorageBehavior,
    history: HistoryService,
    playlists: PlaylistService,
}

impl<'a, S: Spotify> Service<'a, S> {
    pub fn new(spotify: &'a S, storage: &'a StorageBehavior) -> Self {
        Service {
            spotify,
            storage,
            history: HistoryService {},
            playlists: PlaylistService {},
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

impl<'a, S: Spotify> Service<'a, S> {
    fn new_interactor(&self) -> Interactor<BufReader<Stdin>, Stdout> {
        let r = BufReader::new(stdin());
        let w = stdout();
        Interactor::new(r, w)
    }
}

#[async_trait]
impl<'a, S: Spotify> ServiceTrait for Service<'a, S> {
    async fn history_update(&self) -> Result<(), CoolioError> {
        self.history.update(self.spotify, self.storage).await
    }

    async fn throwback(
        &self,
        name: Option<&str>,
        period: Option<ThrowbackPeriod>,
        size: Option<usize>,
    ) -> Result<(), CoolioError> {
        self.history
            .throwback(self.spotify, self.storage, name, period, size)
            .await
    }

    async fn playlists_list(&self) -> Result<(), CoolioError> {
        self.playlists
            .list(self.spotify, self.storage, &mut self.new_interactor())
            .await
    }

    async fn playlists_show(&self, name: &str) -> Result<(), CoolioError> {
        self.playlists
            .show(self.spotify, self.storage, &mut self.new_interactor(), name)
            .await
    }

    async fn playlists_create(&self, name: &str) -> Result<(), CoolioError> {
        self.playlists
            .create(self.spotify, self.storage, name)
            .await
    }

    async fn playlists_automate(&self, name: &str) -> Result<(), CoolioError> {
        self.playlists
            .automate(self.spotify, self.storage, name)
            .await
    }

    async fn link_playlist_to_artist(
        &self,
        playlist: &str,
        artist: &str,
        seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        self.playlists
            .link_playlist_to_artist(
                self.spotify,
                self.storage,
                &mut self.new_interactor(),
                playlist,
                artist,
                seed,
            )
            .await
    }

    async fn unlink_artist_from_playlist(
        &self,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError> {
        self.playlists
            .unlink_artist_from_playlist(self.spotify, self.storage, playlist, artist)
            .await
    }

    async fn playlists_update(&self) -> Result<(), CoolioError> {
        self.playlists.update(self.spotify, self.storage).await
    }
}
