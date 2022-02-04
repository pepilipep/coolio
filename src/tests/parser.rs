use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{error::CoolioError, models::ThrowbackPeriod, parser::Parser, service::ServiceTrait};

#[derive(Default)]
pub struct MockService {
    call_history_update: Mutex<u32>,
    call_throwback: Mutex<u32>,
    call_playlists_list: Mutex<u32>,
    call_playlists_show: Mutex<u32>,
    call_playlists_create: Mutex<u32>,
    call_playlists_automate: Mutex<u32>,
    call_link_playlist_to_artist: Mutex<u32>,
    call_unlink_artist_from_playlist: Mutex<u32>,
    call_playlists_update: Mutex<u32>,
}

#[async_trait]
impl ServiceTrait for MockService {
    async fn history_update(&self) -> Result<(), CoolioError> {
        *self.call_history_update.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn throwback(
        &self,
        _name: Option<&str>,
        _period: Option<ThrowbackPeriod>,
        _size: Option<usize>,
    ) -> Result<(), CoolioError> {
        *self.call_throwback.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn playlists_list(&self) -> Result<(), CoolioError> {
        *self.call_playlists_list.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn playlists_show(&self, _name: &str) -> Result<(), CoolioError> {
        *self.call_playlists_show.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn playlists_create(&self, _name: &str) -> Result<(), CoolioError> {
        *self.call_playlists_create.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn playlists_automate(&self, _name: &str) -> Result<(), CoolioError> {
        *self.call_playlists_automate.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn link_playlist_to_artist(
        &self,
        _playlist: &str,
        _artist: &str,
        _seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        *self.call_link_playlist_to_artist.lock().await.deref_mut() += 1;
        Ok(())
    }

    async fn unlink_artist_from_playlist(
        &self,
        _playlist: &str,
        _artist: &str,
    ) -> Result<(), CoolioError> {
        *self
            .call_unlink_artist_from_playlist
            .lock()
            .await
            .deref_mut() += 1;
        Ok(())
    }

    async fn playlists_update(&self) -> Result<(), CoolioError> {
        *self.call_playlists_update.lock().await.deref_mut() += 1;
        Ok(())
    }
}

#[tokio::test]
async fn test_parser_history_update() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "history", "update"]);
    parser.parse(&s).await.unwrap();
    assert_eq!(&1, s.call_history_update.lock().await.deref());
}

#[tokio::test]
async fn test_parser_throwback() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "history", "throwback"]);
    parser.parse(&s).await.unwrap();
    assert_eq!(&1, s.call_throwback.lock().await.deref());
}
