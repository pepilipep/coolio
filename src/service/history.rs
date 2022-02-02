use std::cmp::min;
use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use chrono::{Duration, Utc};
use rspotify::model::TrackId;

use crate::models::ThrowbackPeriod;
use crate::{error::CoolioError, models::Listen, storage::Storage};
use rspotify::model::misc::TimeLimits;
use rspotify::prelude::*;
use rspotify::AuthCodeSpotify;

use super::spotify::Spotify;

pub struct HistoryService<S: Spotify> {
    spotify: S,
    storage: Box<dyn Storage>,
}

impl<S: Spotify> HistoryService<S> {
    async fn update(&self) -> Result<(), CoolioError> {
        let last_listen = self.storage.get_last_listen().await.ok().map(|x| x.time);

        let recent = self
            .spotify
            .current_user_recently_played(50, last_listen)
            .await?;

        for song in recent {
            self.storage
                .add_history(Listen {
                    song_id: song.track.id.unwrap().uri(),
                    time: song.played_at,
                })
                .await?;
        }
        Ok(())
    }

    async fn throwback(
        &self,
        name: Option<&str>,
        period: Option<ThrowbackPeriod>,
        size: Option<usize>,
    ) -> Result<(), CoolioError> {
        let offset = match period {
            None => Duration::weeks(25),
            Some(ThrowbackPeriod::Days(d)) => Duration::days(d as i64),
            Some(ThrowbackPeriod::Weeks(w)) => Duration::weeks(w as i64),
            Some(ThrowbackPeriod::Months(m)) => Duration::days((m * 30) as i64),
            Some(ThrowbackPeriod::Years(y)) => Duration::days((y * 365) as i64),
        };
        let before = Utc::now() - offset;

        let history = self.storage.get_history().await?;

        let mut blacklisted = HashSet::<String>::new();
        for h in &history {
            if h.time > before {
                blacklisted.insert(h.song_id.clone());
            }
        }

        let mut throwback = HashMap::<String, usize>::new();
        for h in &history {
            if !blacklisted.contains(&h.song_id) {
                let counter = throwback.entry(h.song_id.clone()).or_insert(0);
                *counter += 1;
            }
        }

        struct Entry {
            count: usize,
            id: String,
        }

        let mut entries = throwback
            .drain()
            .map(|(x, y)| Entry { id: x, count: y })
            .collect::<Vec<Entry>>();
        entries.sort_by(|a, b| b.count.cmp(&a.count));

        if entries.is_empty() {
            return Ok(());
        }

        let playlist = self
            .spotify
            .create_playlist(name.unwrap_or(&format!("Throwback - {}", Utc::today())))
            .await?;

        let size = min(size.unwrap_or(50), entries.len());

        let to_add = entries[..size].iter().map(|x| x.id.as_str());

        self.spotify
            .playlist_add_items(&playlist.id, to_add)
            .await?;

        Ok(())
    }
}

#[async_trait]
pub trait History {
    fn get_spotify(&self) -> &AuthCodeSpotify;
    fn get_storage(&self) -> &Box<dyn Storage>;

    async fn history_update(&self) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let last_listen = storage
            .get_last_listen()
            .await
            .ok()
            .map(|x| TimeLimits::After(x.time));

        let recent = spotify
            .current_user_recently_played(Some(50), last_listen)
            .await?;

        for song in recent.items {
            storage
                .add_history(Listen {
                    song_id: song.track.id.unwrap().uri(),
                    time: song.played_at,
                })
                .await?;
        }
        Ok(())
    }

    async fn throwback(
        &self,
        name: Option<&str>,
        period: Option<ThrowbackPeriod>,
        size: Option<usize>,
    ) -> Result<(), CoolioError> {
        let storage = self.get_storage();
        let spotify = self.get_spotify();

        let offset = match period {
            None => Duration::weeks(25),
            Some(ThrowbackPeriod::Days(d)) => Duration::days(d as i64),
            Some(ThrowbackPeriod::Weeks(w)) => Duration::weeks(w as i64),
            Some(ThrowbackPeriod::Months(m)) => Duration::days((m * 30) as i64),
            Some(ThrowbackPeriod::Years(y)) => Duration::days((y * 365) as i64),
        };
        let before = Utc::now() - offset;

        let history = storage.get_history().await?;

        let mut blacklisted = HashSet::<String>::new();
        for h in &history {
            if h.time > before {
                blacklisted.insert(h.song_id.clone());
            }
        }

        let mut throwback = HashMap::<String, usize>::new();
        for h in &history {
            if !blacklisted.contains(&h.song_id) {
                let counter = throwback.entry(h.song_id.clone()).or_insert(0);
                *counter += 1;
            }
        }

        struct Entry {
            count: usize,
            id: String,
        }

        let mut entries = throwback
            .drain()
            .map(|(x, y)| Entry { id: x, count: y })
            .collect::<Vec<Entry>>();
        entries.sort_by(|a, b| b.count.cmp(&a.count));

        if entries.is_empty() {
            return Ok(());
        }

        let me = spotify.current_user().await?;
        let playlist = spotify
            .user_playlist_create(
                &me.id,
                name.unwrap_or(&format!("Throwback - {}", Utc::today())),
                None,
                None,
                None,
            )
            .await?;

        let size = min(size.unwrap_or(50), entries.len());
        let please_live = entries[..size]
            .iter()
            .map(|x| TrackId::from_uri(&x.id).unwrap())
            .collect::<Vec<TrackId>>();

        let to_add = please_live.iter().map(|x| x as &dyn PlayableId);

        spotify
            .playlist_add_items(&playlist.id, to_add, Some(0))
            .await?;

        Ok(())
    }
}
