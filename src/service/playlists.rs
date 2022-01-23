use async_trait::async_trait;

use rspotify::prelude::*;
use rspotify::AuthCodeSpotify;

use crate::error::CoolioError;
use crate::models::Playlist;
use crate::storage::Storage;

#[async_trait]
pub trait Playlists<S: Storage + Send + Sync> {
    fn get_spotify(&self) -> &AuthCodeSpotify;
    fn get_storage(&self) -> &S;

    async fn list_playlists(&self) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();

        let limit = 50;
        let mut offset = 0;

        let mut playlists = Vec::<Playlist>::new();

        loop {
            let fetched = spotify
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await?;

            for playlist in fetched.items {
                playlists.push(Playlist {
                    id: playlist.id.uri(),
                    name: playlist.name,
                    artists: Vec::<String>::new(),
                })
            }

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

        println!("{:?}", playlists);

        Ok(())
    }
}
