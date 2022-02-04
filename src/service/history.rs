use std::cmp::min;
use std::collections::{HashMap, HashSet};

use chrono::{Duration, Utc};

use crate::models::ThrowbackPeriod;
use crate::storage::StorageBehavior;
use crate::{error::CoolioError, storage::Storage};

use super::spotify::Spotify;

pub struct HistoryService {}

impl HistoryService {
    pub async fn update(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
    ) -> Result<(), CoolioError> {
        let last_listen = storage.get_last_listen().await.ok().map(|x| x.time);

        let recent = spotify
            .current_user_recently_played(50, last_listen)
            .await?;

        for l in recent {
            storage.add_history(l).await?;
        }
        Ok(())
    }

    pub async fn throwback(
        &self,
        spotify: &impl Spotify,
        storage: &StorageBehavior,
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

        let playlist = spotify
            .create_playlist(name.unwrap_or(&format!("Throwback - {}", Utc::today())))
            .await?;

        let size = min(size.unwrap_or(50), entries.len());

        let to_add = entries[..size].iter().map(|x| x.id.clone());

        spotify.playlist_add_items(&playlist.id, to_add).await?;

        Ok(())
    }
}
