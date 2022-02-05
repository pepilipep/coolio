use crate::models::ThrowbackPeriod;
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
    assert_eq!(listens.len(), 9);
    assert_eq!(listens[0].song_id, "track_2");
    assert_eq!(listens[1].song_id, "track_1");
}

#[tokio::test]
async fn test_history_throwback_year() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    s.history_update().await.unwrap();
    s.throwback(Some("oldstyle"), Some(ThrowbackPeriod::Years(1)), None)
        .await
        .unwrap();

    let sp_pl = &sp.state.lock().await.playlists;
    assert_eq!(sp_pl.len(), 1);
    assert_eq!(sp_pl[0].name, "oldstyle");
    let tracks = &sp_pl[0].tracks;
    assert_eq!(tracks.len(), 2);
    assert_eq!(tracks[0].track.id, "track_1");
    assert_eq!(tracks[1].track.id, "track_2");
}

#[tokio::test]
async fn test_history_throwback_day() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    s.history_update().await.unwrap();
    s.throwback(Some("newstyle"), Some(ThrowbackPeriod::Days(1)), None)
        .await
        .unwrap();

    let sp_pl = &sp.state.lock().await.playlists;
    assert_eq!(sp_pl.len(), 1);
    assert_eq!(sp_pl[0].name, "newstyle");
    let tracks = &sp_pl[0].tracks;
    assert_eq!(tracks.len(), 3);
    assert_eq!(tracks[0].track.id, "track_1");
    assert_eq!(tracks[1].track.id, "track_2");
    assert_eq!(tracks[2].track.id, "track_4");
}

#[tokio::test]
async fn test_history_throwback_no_history() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    s.throwback(Some("newstyle"), Some(ThrowbackPeriod::Days(1)), None)
        .await
        .unwrap();

    let sp_pl = &sp.state.lock().await.playlists;
    assert_eq!(sp_pl.len(), 0);
}
