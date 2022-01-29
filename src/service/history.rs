use async_trait::async_trait;

use crate::{error::CoolioError, models::Listen, storage::Storage};
use rspotify::model::misc::TimeLimits;
use rspotify::prelude::*;
use rspotify::AuthCodeSpotify;

#[async_trait]
pub trait History {
    fn get_spotify(&self) -> &AuthCodeSpotify;
    fn get_storage(&self) -> &Box<dyn Storage + Send + Sync>;

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
}
