pub mod fs;
pub mod psql;

use async_trait::async_trait;

use crate::error::CoolioError;

use crate::models::{Listen, Playlist};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn add_history(&self, listen: Listen) -> Result<(), CoolioError>;

    async fn get_history(&self) -> Result<Vec<Listen>, CoolioError>;

    async fn get_last_listen(&self) -> Result<Listen, CoolioError>;

    async fn create_playlist(&self, id: &str, name: &str) -> Result<(), CoolioError>;

    async fn get_playlists(&self) -> Result<Vec<Playlist>, CoolioError>;

    async fn get_playlist(&self, name: &str) -> Result<Playlist, CoolioError>;

    async fn link_artist(
        &self,
        playlist_id: &str,
        playlist_name: &str,
        artist_id: &str,
    ) -> Result<(), CoolioError>;

    async fn unlink_artist(&self, playlist_id: &str, artist_id: &str) -> Result<(), CoolioError>;
}
