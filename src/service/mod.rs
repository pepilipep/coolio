use rspotify::AuthCodeSpotify;

use crate::storage::Storage;

use self::{history::History, playlists::Playlists};

pub mod history;
pub mod playlists;
pub mod spotify;

pub struct Service {
    spotify: AuthCodeSpotify,
    storage: Box<dyn Storage>,
}

impl Service {
    pub fn new(spotify: AuthCodeSpotify, storage: Box<dyn Storage>) -> Self {
        Service { spotify, storage }
    }
}

impl History for Service {
    fn get_spotify(&self) -> &AuthCodeSpotify {
        return &self.spotify;
    }
    fn get_storage(&self) -> &Box<dyn Storage> {
        return &self.storage;
    }
}

impl Playlists for Service {
    fn get_spotify(&self) -> &AuthCodeSpotify {
        return &self.spotify;
    }
    fn get_storage(&self) -> &Box<dyn Storage> {
        return &self.storage;
    }
}
