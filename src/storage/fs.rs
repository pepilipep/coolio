use async_trait::async_trait;
use chrono::{DateTime, Utc};
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{Listen, Playlist};
use crate::{error::CoolioError, settings::LocalStorage};

use super::Storage;

enum StorageFile {
    History,
    Playlist,
    Links,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Into<Playlist> for PlaylistRecord {
    fn into(self) -> Playlist {
        Playlist {
            id: self.id,
            name: self.name,
            artists: vec![],
            automated: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LinkRecord {
    playlist_id: String,
    artist_id: String,
}

pub struct Fs {
    path: String,
}

impl Fs {
    pub async fn new(conf: LocalStorage) -> Result<Self, CoolioError> {
        Ok(Fs { path: conf.path })
    }

    fn get_path(&self, sf: StorageFile) -> PathBuf {
        Path::new(&self.path).join(match sf {
            StorageFile::History => "history",
            StorageFile::Playlist => "playlist",
            StorageFile::Links => "links",
        })
    }

    fn get_writer(&self, sf: StorageFile, append: bool) -> Result<Writer<fs::File>, CoolioError> {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(!append)
            .append(append)
            .open(self.get_path(sf))?;

        Ok(csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file))
    }

    fn get_reader(&self, sf: StorageFile) -> Result<Reader<fs::File>, CoolioError> {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(self.get_path(sf))?;
        Ok(csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file))
    }
}

#[async_trait]
impl Storage for Fs {
    async fn add_history(&self, listen: Listen) -> Result<(), CoolioError> {
        let mut wtr = self.get_writer(StorageFile::History, true)?;
        wtr.serialize(&ListenRecord::from(listen))?;
        wtr.flush()?;
        Ok(())
    }

    async fn get_history(&self) -> Result<Vec<Listen>, CoolioError> {
        let mut rdr = self.get_reader(StorageFile::History)?;
        let mut history = Vec::<Listen>::new();
        for record in rdr.deserialize() {
            let l: ListenRecord = record?;
            history.push(l.into());
        }
        Ok(history)
    }

    async fn get_last_listen(&self) -> Result<Listen, CoolioError> {
        let mut rdr = self.get_reader(StorageFile::History)?;
        let mut listen: Option<ListenRecord> = None;
        for record in rdr.deserialize::<ListenRecord>() {
            let r = record?;
            if let Some(l) = listen.as_ref() {
                if l.time < r.time {
                    listen = Some(r);
                }
            } else {
                listen = Some(r);
            }
        }

        match listen {
            None => Err("no listen history".into()),
            Some(l) => Ok(l.into()),
        }
    }

    async fn create_playlist(&self, id: &str, name: &str) -> Result<(), CoolioError> {
        let mut wtr = self.get_writer(StorageFile::Playlist, true)?;
        wtr.serialize(&PlaylistRecord {
            id: id.to_string(),
            name: name.to_string(),
        })?;
        wtr.flush()?;
        Ok(())
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>, CoolioError> {
        let mut rdr = self.get_reader(StorageFile::Playlist)?;
        let mut playlists = Vec::<Playlist>::new();
        for record in rdr.deserialize::<PlaylistRecord>() {
            playlists.push(record?.into());
        }

        rdr = self.get_reader(StorageFile::Links)?;
        for record in rdr.deserialize() {
            let link: LinkRecord = record?;
            for p in playlists.iter_mut() {
                if p.id == link.playlist_id {
                    p.artists.push(link.artist_id);
                    break;
                }
            }
        }

        Ok(playlists)
    }

    async fn get_playlist(&self, name: &str) -> Result<Playlist, CoolioError> {
        let mut rdr = self.get_reader(StorageFile::Playlist)?;
        for record in rdr.deserialize() {
            let playlist: PlaylistRecord = record?;
            if playlist.name == name {
                let mut playlist: Playlist = playlist.into();
                let mut link_rdr = self.get_reader(StorageFile::Links)?;
                for link_rec in link_rdr.deserialize() {
                    let link: LinkRecord = link_rec?;
                    if link.playlist_id == playlist.id {
                        playlist.artists.push(link.artist_id);
                    }
                }
                return Ok(playlist);
            }
        }
        Err("playlist doesn't exist".into())
    }

    async fn link_artist(
        &self,
        playlist_id: &str,
        _playlist_name: &str,
        artist_id: &str,
    ) -> Result<(), CoolioError> {
        let mut wtr = self.get_writer(StorageFile::Links, true)?;
        wtr.serialize(&LinkRecord {
            playlist_id: playlist_id.to_string(),
            artist_id: artist_id.to_string(),
        })?;
        wtr.flush()?;
        Ok(())
    }

    async fn unlink_artist(&self, playlist_id: &str, artist_id: &str) -> Result<(), CoolioError> {
        let mut links = Vec::<LinkRecord>::new();
        let mut rdr = self.get_reader(StorageFile::Links)?;
        for record in rdr.deserialize() {
            let link: LinkRecord = record?;
            if link.playlist_id != playlist_id || link.artist_id != artist_id {
                links.push(link);
            }
        }

        let mut wtr = self.get_writer(StorageFile::Links, false)?;
        for link in links {
            wtr.serialize(&link)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
