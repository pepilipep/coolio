use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{Listen, Playlist};
use crate::{error::CoolioError, settings::LocalStorage};

use super::Storage;

static HISTORY: &str = "history";
static PLAYLIST: &str = "playlists";
static LINKS: &str = "links";

#[derive(Serialize, Deserialize)]
struct ListenRecord {
    song_id: String,
    time: DateTime<Utc>,
}

impl From<Listen> for ListenRecord {
    fn from(l: Listen) -> Self {
        ListenRecord {
            song_id: l.song_id,
            time: l.time,
        }
    }
}

impl Into<Listen> for ListenRecord {
    fn into(self) -> Listen {
        Listen {
            song_id: self.song_id,
            time: self.time,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlaylistRecord {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct LinkRecord {
    plalist_id: String,
    artist_id: String,
}

pub struct Fs {
    path: String,
}

impl Fs {
    pub async fn new(conf: LocalStorage) -> Result<Self, CoolioError> {
        Ok(Fs { path: conf.path })
    }

    fn history_path(&self) -> PathBuf {
        Path::new(&self.path).join(HISTORY)
    }

    fn playlists_path(&self) -> PathBuf {
        Path::new(&self.path).join(PLAYLIST)
    }

    fn links_path(&self) -> PathBuf {
        Path::new(&self.path).join(LINKS)
    }
}

#[async_trait]
impl Storage for Fs {
    async fn add_history(&self, listen: Listen) -> Result<(), CoolioError> {
        let file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.history_path())?;

        let mut wtr = csv::Writer::from_writer(file);

        wtr.serialize(&ListenRecord::from(listen))?;
        wtr.flush()?;
        Ok(())
    }

    async fn get_last_listen(&self) -> Result<Listen, CoolioError> {
        let file = fs::OpenOptions::new()
            .read(true)
            .open(self.history_path())?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut listen: Option<ListenRecord> = None;
        for record in rdr.deserialize() {
            listen = record?;
        }

        match listen {
            None => Err(CoolioError::from("no listen history")),
            Some(l) => Ok(l.into()),
        }
    }

    async fn create_playlist(&self, id: &str, name: &str) -> Result<(), CoolioError> {
        Ok(())
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>, CoolioError> {
        Ok(vec![])
    }

    async fn get_playlist(&self, name: &str) -> Result<Playlist, CoolioError> {
        Err(CoolioError::from("lol"))
    }

    async fn link_artist(
        &self,
        playlist_id: &str,
        playlist_name: &str,
        artist_id: &str,
    ) -> Result<(), CoolioError> {
        Ok(())
    }

    async fn unlink_artist(&self, playlist_id: &str, artist_id: &str) -> Result<(), CoolioError> {
        Ok(())
    }
}
