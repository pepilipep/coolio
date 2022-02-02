use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rspotify::model::{FullPlaylist, PlaylistId, TrackId};
use rspotify::prelude::*;
use rspotify::{
    model::{PlayHistory, TimeLimits},
    AuthCodeSpotify,
};

use crate::error::CoolioError;

#[async_trait]
pub trait Spotify {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<PlayHistory>, CoolioError>;

    async fn create_playlist(&self, name: &str) -> Result<FullPlaylist, CoolioError>;

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &PlaylistId,
        items: impl IntoIterator<Item = &'a str> + Send + 'a,
    ) -> Result<(), CoolioError>;
}

pub struct HTTPSpotify {
    spotify: AuthCodeSpotify,
}

#[async_trait]
impl Spotify for HTTPSpotify {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<PlayHistory>, CoolioError> {
        let last_listen = time_limit.map(|x| TimeLimits::After(x));

        Ok(self
            .spotify
            .current_user_recently_played(Some(limit), last_listen)
            .await?
            .items)
    }

    async fn create_playlist(&self, name: &str) -> Result<FullPlaylist, CoolioError> {
        let me = self.spotify.current_user().await?;
        let playlist = self
            .spotify
            .user_playlist_create(&me.id, name, None, None, None)
            .await?;
        Ok(playlist)
    }

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &PlaylistId,
        items: impl IntoIterator<Item = &'a str> + Send + 'a,
    ) -> Result<(), CoolioError> {
        let please_live = items
            .into_iter()
            .map(|x| TrackId::from_uri(x).unwrap())
            .collect::<Vec<TrackId>>();

        let to_add = please_live
            .iter()
            .map(|x| x as &dyn PlayableId)
            .collect::<Vec<&dyn PlayableId>>();

        self.spotify
            .playlist_add_items(playlist_id, to_add, Some(0))
            .await?;
        Ok(())
    }
}
