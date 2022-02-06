use crate::service::spotify::Spotify;
use crate::service::{Service, ServiceTrait};
use crate::storage::mock::Mock as MockStorage;
use crate::storage::StorageBehavior;
use crate::tests::mock_spotify::MockSpotify;

#[tokio::test]
async fn test_playlists_create() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    s.playlists_create("maman").await.unwrap();

    let sp_pl = &sp.state.lock().await.playlists;
    assert_eq!(sp_pl.len(), 1);
    assert_eq!(sp_pl[0].name, "maman");
    let tracks = &sp_pl[0].tracks;
    assert_eq!(tracks.len(), 0);

    let st = st_to.as_mock().unwrap();
    let stored_playlists = &st.state.lock().await.playlists;
    assert_eq!(stored_playlists.len(), 1);
    assert_eq!(stored_playlists[0].name, "maman");
    assert_eq!(sp_pl[0].id, stored_playlists[0].id);
    assert_eq!(stored_playlists[0].artists.len(), 0);
}

#[tokio::test]
async fn test_playlists_list() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    sp.create_playlist("nostored").await.unwrap();
    s.playlists_create("maman").await.unwrap();
    s.playlists_create("thisguy").await.unwrap();
    s.playlists_list().await.unwrap();
}
