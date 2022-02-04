use async_trait::async_trait;
use std::borrow::{Borrow, BorrowMut};
use tokio::sync::Mutex;

use crate::{
    error::CoolioError,
    models::{Listen, Playlist},
    storage::Storage,
};

pub struct Mock {
    pub listens: Mutex<Vec<Listen>>,
    pub playlists: Mutex<Vec<Playlist>>,
}

impl Mock {
    pub fn new() -> Self {
        Mock {
            listens: Mutex::new(vec![]),
            playlists: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl Storage for Mock {
    async fn add_history(&self, listen: Listen) -> Result<(), CoolioError> {
        self.listens.lock().await.borrow_mut().push(listen);
        Ok(())
    }

    async fn get_history(&self) -> Result<Vec<Listen>, CoolioError> {
        Ok(self.listens.lock().await.borrow().to_vec())
    }

    async fn get_last_listen(&self) -> Result<Listen, CoolioError> {
        let h = self.listens.lock().await.borrow().to_vec();
        if h.len() == 0 {
            return Err("no history".into());
        }
        let mut last_listen = h[0].clone();
        for l in &h[1..] {
            if l.time > last_listen.time {
                last_listen = (*l).clone();
            }
        }
        Ok(last_listen)
    }

    async fn create_playlist(&self, id: &str, name: &str) -> Result<(), CoolioError> {
        self.playlists.lock().await.borrow_mut().push(Playlist {
            id: id.to_string(),
            name: name.to_string(),
            automated: true,
            artists: vec![],
        });
        Ok(())
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>, CoolioError> {
        Ok(self.playlists.lock().await.borrow().to_vec())
    }

    async fn get_playlist(&self, name: &str) -> Result<Playlist, CoolioError> {
        let ps = self.playlists.lock().await.borrow().to_vec();
        for p in ps {
            if p.name == name {
                return Ok(p);
            }
        }
        Err("no such playlist".into())
    }

    async fn link_artist(
        &self,
        playlist_id: &str,
        _playlist_name: &str,
        artist_id: &str,
    ) -> Result<(), CoolioError> {
        let ps = self.playlists.lock().await.borrow().to_vec();
        for mut p in ps {
            if p.id == playlist_id {
                if p.artists.contains(&artist_id.to_string()) {
                    return Err("duplicate artists".into());
                } else {
                    p.artists.push(artist_id.to_string());
                    return Ok(());
                }
            }
        }
        Err("playlist doesn't exist".into())
    }

    async fn unlink_artist(&self, playlist_id: &str, artist_id: &str) -> Result<(), CoolioError> {
        let mut ps = self.playlists.lock().await.borrow().to_vec();
        for (i, p) in ps.iter_mut().enumerate() {
            if p.id == playlist_id {
                if p.artists.contains(&artist_id.to_string()) {
                    p.artists.remove(i);
                    return Ok(());
                } else {
                    return Err("artist not linked to playlist".into());
                }
            }
        }
        Err("playlist doesn't exist".into())
    }
}
