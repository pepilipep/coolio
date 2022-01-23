use rspotify::AuthCodeSpotify;

use crate::storage::Storage;

use self::{history::History, playlists::Playlists};

pub mod history;
pub mod playlists;

pub struct Service<S: Storage> {
    spotify: AuthCodeSpotify,
    storage: S,
}

impl<S: Storage> Service<S> {
    pub fn new(spotify: AuthCodeSpotify, storage: S) -> Self {
        Service { spotify, storage }
    }
}

impl<S: Storage + Send + Sync> History<S> for Service<S> {
    fn get_spotify(&self) -> &AuthCodeSpotify {
        return &self.spotify;
    }
    fn get_storage(&self) -> &S {
        return &self.storage;
    }
}

impl<S: Storage + Send + Sync> Playlists<S> for Service<S> {
    fn get_spotify(&self) -> &AuthCodeSpotify {
        return &self.spotify;
    }
    fn get_storage(&self) -> &S {
        return &self.storage;
    }
}
