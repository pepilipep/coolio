use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{error::CoolioError, models::ThrowbackPeriod, parser::Parser, service::ServiceTrait};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
struct Calls {
    history_update: u32,
    throwback: u32,
    playlists_list: u32,
    playlists_show: u32,
    playlists_create: u32,
    playlists_automate: u32,
    link_playlist_to_artist: u32,
    unlink_artist_from_playlist: u32,
    playlists_update: u32,
}

#[derive(Default)]
pub struct MockService {
    calls: Mutex<Calls>,
}

#[async_trait]
impl ServiceTrait for MockService {
    async fn history_update(&self) -> Result<(), CoolioError> {
        self.calls.lock().await.history_update += 1;
        Ok(())
    }

    async fn throwback(
        &self,
        _name: Option<&str>,
        _period: Option<ThrowbackPeriod>,
        _size: Option<usize>,
    ) -> Result<(), CoolioError> {
        self.calls.lock().await.throwback += 1;
        Ok(())
    }

    async fn playlists_list(&self) -> Result<(), CoolioError> {
        self.calls.lock().await.playlists_list += 1;
        Ok(())
    }

    async fn playlists_show(&self, _name: &str) -> Result<(), CoolioError> {
        self.calls.lock().await.playlists_show += 1;
        Ok(())
    }

    async fn playlists_create(&self, _name: &str) -> Result<(), CoolioError> {
        self.calls.lock().await.playlists_create += 1;
        Ok(())
    }

    async fn playlists_automate(&self, _name: &str) -> Result<(), CoolioError> {
        self.calls.lock().await.playlists_automate += 1;
        Ok(())
    }

    async fn link_playlist_to_artist(
        &self,
        _playlist: &str,
        _artist: &str,
        _seed: Option<usize>,
    ) -> Result<(), CoolioError> {
        self.calls.lock().await.link_playlist_to_artist += 1;
        Ok(())
    }

    async fn unlink_artist_from_playlist(
        &self,
        _playlist: &str,
        _artist: &str,
    ) -> Result<(), CoolioError> {
        self.calls.lock().await.unlink_artist_from_playlist += 1;
        Ok(())
    }

    async fn playlists_update(&self) -> Result<(), CoolioError> {
        self.calls.lock().await.playlists_update += 1;
        Ok(())
    }
}

#[tokio::test]
async fn test_parser_history_update() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "history", "update"]).unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.history_update += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_history_update() {
    Parser::new(vec!["coolio", "history", "update", "whatever"]).unwrap_err();
    Parser::new(vec!["coolio", "history", "update", "--whatever"]).unwrap_err();
}

#[tokio::test]
async fn test_parser_throwback() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "history", "throwback"]).unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.throwback += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());

    let parser = Parser::new(vec![
        "coolio",
        "history",
        "throwback",
        "--name",
        "playlist_name",
    ])
    .unwrap();
    parser.parse(&s).await.unwrap();
    expected.throwback += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());

    let parser = Parser::new(vec!["coolio", "history", "throwback", "--period", "5m"]).unwrap();
    parser.parse(&s).await.unwrap();
    expected.throwback += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());

    let parser = Parser::new(vec!["coolio", "history", "throwback", "--size", "2"]).unwrap();
    parser.parse(&s).await.unwrap();
    expected.throwback += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_throwback() {
    Parser::new(vec!["coolio", "history", "throwback", "whatever"]).unwrap_err();
    Parser::new(vec!["coolio", "history", "throwback", "--not", "wow"]).unwrap_err();
    Parser::new(vec!["coolio", "history", "throwback", "--size", "adkj"]).unwrap_err();
    Parser::new(vec!["coolio", "history", "throwback", "--period", "10p"]).unwrap_err();
}

#[tokio::test]
async fn test_parser_playlists_list() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "playlists", "list"]).unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.playlists_list += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_playlists_list() {
    Parser::new(vec!["coolio", "playlists", "list", "whatever"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "list", "--whatever"]).unwrap_err();
}

#[tokio::test]
async fn test_parser_playlists_link() {
    let s = MockService::default();
    let parser = Parser::new(vec![
        "coolio",
        "playlists",
        "link",
        "playlist_name",
        "artist_name",
    ])
    .unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.link_playlist_to_artist += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());

    let parser = Parser::new(vec![
        "coolio",
        "playlists",
        "link",
        "playlist_name",
        "artist_name",
        "--seed",
        "3",
    ])
    .unwrap();
    parser.parse(&s).await.unwrap();
    expected.link_playlist_to_artist += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_playlists_link() {
    Parser::new(vec!["coolio", "playlists", "link"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "link", "whatever"]).unwrap_err();
    Parser::new(vec![
        "coolio",
        "playlists",
        "link",
        "whatever",
        "--whatever",
    ])
    .unwrap_err();
    Parser::new(vec![
        "coolio",
        "playlists",
        "link",
        "whatever",
        "whatever",
        "whatever",
    ])
    .unwrap_err();
    Parser::new(vec![
        "coolio",
        "playlists",
        "link",
        "playlist_name",
        "artist_name",
        "--seed",
        "notanumber",
    ])
    .unwrap_err();
}

#[tokio::test]
async fn test_parser_playlists_unlink() {
    let s = MockService::default();
    let parser = Parser::new(vec![
        "coolio",
        "playlists",
        "unlink",
        "playlist_name",
        "artist_name",
    ])
    .unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.unlink_artist_from_playlist += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_playlists_unlink() {
    Parser::new(vec!["coolio", "playlists", "unlink"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "unlink", "whatever"]).unwrap_err();
    Parser::new(vec![
        "coolio",
        "playlists",
        "unlink",
        "whatever",
        "--whatever",
    ])
    .unwrap_err();
    Parser::new(vec![
        "coolio",
        "playlists",
        "unlink",
        "whatever",
        "whatever",
        "whatever",
    ])
    .unwrap_err();
    Parser::new(vec![
        "coolio",
        "playlists",
        "unlink",
        "playlist_name",
        "artist_name",
        "--seed",
        "notanumber",
    ])
    .unwrap_err();
}

#[tokio::test]
async fn test_parser_playlists_update() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "playlists", "update"]).unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.playlists_update += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_playlists_update() {
    Parser::new(vec!["coolio", "playlists", "update", "whatever"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "update", "--whatever"]).unwrap_err();
}

#[tokio::test]
async fn test_parser_playlists_automate() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "playlists", "automate", "playlist_name"]).unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.playlists_automate += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_playlists_automate() {
    Parser::new(vec!["coolio", "playlists", "automate"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "automate", "one", "two"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "automate", "one", "--notok"]).unwrap_err();
}

#[tokio::test]
async fn test_parser_playlists_show() {
    let s = MockService::default();
    let parser = Parser::new(vec!["coolio", "playlists", "show", "playlist_name"]).unwrap();
    parser.parse(&s).await.unwrap();
    let mut expected = Calls::default();
    expected.playlists_show += 1;
    assert_eq!(&expected, s.calls.lock().await.deref());
}

#[test]
fn test_parser_incorrect_playlists_show() {
    Parser::new(vec!["coolio", "playlists", "show"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "show", "one", "two"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists", "show", "one", "--notok"]).unwrap_err();
}

#[test]
fn test_parser_incorrect_overall_usage() {
    Parser::new(vec!["coolio", "unexisting-subcommand"]).unwrap_err();
    Parser::new(vec!["coolio", "playlists"]).unwrap_err();
    Parser::new(vec!["coolio", "history"]).unwrap_err();
}
