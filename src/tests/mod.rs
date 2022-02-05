use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

use chrono::Utc;

use crate::models::Listen;
use crate::service::{Service, ServiceTrait};
use crate::storage::mock::Mock as MockStorage;
use crate::storage::StorageBehavior;

use self::mock_spotify::MockSpotify;

mod mock_spotify;
mod parser;

#[tokio::test]
async fn test_history_update() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    s.history_update().await.unwrap();

    let st = st_to.as_mock().unwrap();
    let listens = &st.state.lock().await.listens;

    assert_eq!(listens.len(), 2);

    assert_eq!(listens[0].song_id, "song_id_1");
    assert_eq!(listens[1].song_id, "song_id_2");
}

#[tokio::test]
async fn test_history_throwback() {}
