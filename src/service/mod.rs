use std::rc::Rc;

use crate::storage::Storage;

use self::{history::HistoryService, playlists::PlaylistService, spotify::Spotify};

pub mod history;
pub mod playlists;
pub mod spotify;

pub struct Service<S: Spotify> {
    pub history: HistoryService<S>,
    pub playlists: PlaylistService<S>,
}

impl<S: Spotify> Service<S> {
    pub fn new(spotify: S, storage: Rc<dyn Storage>) -> Self {
        let s = Rc::new(spotify);
        Service {
            history: HistoryService::new(Rc::clone(&s), Rc::clone(&storage)),
            playlists: PlaylistService::new(Rc::clone(&s), Rc::clone(&storage)),
        }
    }
}
