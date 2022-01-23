use async_trait::async_trait;

use crate::{
    error::CoolioError,
    storage::{models::Listen, Storage},
};
use rspotify::model::misc::TimeLimits;
use rspotify::prelude::*;
use rspotify::AuthCodeSpotify;

#[async_trait]
pub trait History<S: Storage + Send + Sync> {
    fn get_spotify(&self) -> &AuthCodeSpotify;
    fn get_storage(&self) -> &S;

    async fn update(&self) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let last_listen = storage.get_last_listen().await?;

        let recent = spotify
            .current_user_recently_played(Some(50), Some(TimeLimits::After(last_listen.time)))
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
