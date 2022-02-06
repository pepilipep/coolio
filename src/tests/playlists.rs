use std::str;

use crate::service::io::Interactor;
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
    let input: &[u8] = "neverread".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    sp.create_playlist("nostored").await.unwrap();
    s.playlists_create("maman").await.unwrap();
    s.playlists_create("thisguy").await.unwrap();
    s.playlists_list(&mut int).await.unwrap();
    let output_str = str::from_utf8(&output).unwrap();
    let split: Vec<&str> = output_str.split("\n").collect();
    assert_eq!(split.len(), 4);
    assert!(split[0].contains("automated"));
    assert!(split[0].contains("maman"));
    assert!(!split[1].contains("automated"));
    assert!(split[1].contains("nostored"));
    assert!(split[2].contains("automated"));
    assert!(split[2].contains("thisguy"));
    assert_eq!(split[3], "");
}

#[tokio::test]
async fn test_playlists_show() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);
    let input: &[u8] = "neverread".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.playlists_create("maman").await.unwrap();
    s.playlists_show(&mut int, "maman").await.unwrap();

    let output_str = str::from_utf8(&output).unwrap();
    let split: Vec<&str> = output_str.split("\n").collect();
    assert_eq!(split.len(), 7);

    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.playlists_show(&mut int, "doesnt exist")
        .await
        .unwrap_err();
    assert_eq!(output.len(), 0);
}

#[tokio::test]
async fn test_playlists_link() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);
    let input: &[u8] = "1\n".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.playlists_create("maman").await.unwrap();
    s.link_playlist_to_artist(&mut int, "maman", "ken", None)
        .await
        .unwrap();

    let output_str = str::from_utf8(&output).unwrap();
    let split: Vec<&str> = output_str.split("\n").collect();
    assert_eq!(split.len(), 3);
    assert!(split[1].contains("kendrick lamar"));
    assert_eq!(split[2], "");
    // unlocks mutex at the end of block
    {
        let st = st_to.as_mock().unwrap();
        let playlists = &st.state.lock().await.playlists;
        assert_eq!(playlists.len(), 1);
        assert_eq!(playlists[0].artists.len(), 1);
        assert_eq!(playlists[0].artists[0], "artist_1");
    }

    // assert no songs were added in the playlist because of no seed
    {
        let playlists = &sp.state.lock().await.playlists;
        assert_eq!(playlists.len(), 1);
        assert_eq!(playlists[0].tracks.len(), 0);
    }

    // link second artist

    let input: &[u8] = "2\n".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);
    s.link_playlist_to_artist(&mut int, "maman", "rick", Some(3))
        .await
        .unwrap();
    let output_str = str::from_utf8(&output).unwrap();
    let split: Vec<&str> = output_str.split("\n").collect();
    assert_eq!(split.len(), 4);
    assert!(split[1].contains("kendrick lamar"));
    assert!(split[2].contains("rick ross"));
    assert_eq!(split[3], "");

    {
        let st = st_to.as_mock().unwrap();
        let playlists = &st.state.lock().await.playlists;
        assert_eq!(playlists.len(), 1);
        assert_eq!(playlists[0].artists.len(), 2);
        assert_eq!(playlists[0].artists[1], "artist_2");
    }

    // assert songs were added in the playlist because of the seed
    {
        let playlists = &sp.state.lock().await.playlists;
        assert_eq!(playlists.len(), 1);
        assert_eq!(playlists[0].tracks.len(), 3);
    }
}

#[tokio::test]
async fn test_playlists_link_error_on_existing() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);
    let input: &[u8] = "1\n".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.playlists_create("maman").await.unwrap();
    s.link_playlist_to_artist(&mut int, "maman", "ken", None)
        .await
        .unwrap();

    let input: &[u8] = "1\n".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);
    s.link_playlist_to_artist(&mut int, "maman", "ken", None)
        .await
        .unwrap_err();
}

#[tokio::test]
async fn test_playlists_link_retry_wrong_input() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);
    let input: &[u8] = "incorrect\n5\n1\n".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.playlists_create("maman").await.unwrap();
    s.link_playlist_to_artist(&mut int, "maman", "ken", None)
        .await
        .unwrap();

    let output_str = str::from_utf8(&output).unwrap();
    let split: Vec<&str> = output_str.split("\n").collect();
    assert_eq!(split.len(), 5);
    assert!(split[1].contains("kendrick lamar"));
    assert_eq!(split[4], "");
}

#[tokio::test]
async fn test_playlists_link_wrong_playlist() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);
    let input: &[u8] = "neverread".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.link_playlist_to_artist(&mut int, "notthere", "kendrick", None)
        .await
        .unwrap_err();
    assert_eq!(output.len(), 0);
}

#[tokio::test]
async fn test_playlists_link_nonexisting_artist() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);
    let input: &[u8] = "neverread".as_bytes();
    let mut output = Vec::new();
    let mut int = Interactor::new(input, &mut output);

    s.link_playlist_to_artist(&mut int, "notthere", "idontexist", None)
        .await
        .unwrap_err();
    assert_eq!(output.len(), 0);
}

#[tokio::test]
async fn test_playlists_automate() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    sp.create_playlist("maman").await.unwrap();
    sp.create_playlist("later_automated").await.unwrap();
    s.playlists_automate("later_automated").await.unwrap();

    {
        let st = st_to.as_mock().unwrap();
        let stored_playlists = &st.state.lock().await.playlists;
        assert_eq!(stored_playlists.len(), 1);
        assert_eq!(stored_playlists[0].automated, true);
        assert_eq!(stored_playlists[0].name, "later_automated");
    }
}

#[tokio::test]
async fn test_playlists_automate_error_on_not_exists() {
    let st_to = StorageBehavior::from(MockStorage::new());
    let sp = MockSpotify::new();
    let s = Service::new(&sp, &st_to);

    sp.create_playlist("maman").await.unwrap();
    sp.create_playlist("later_automated").await.unwrap();
    s.playlists_automate("doesntexist").await.unwrap_err();
}
