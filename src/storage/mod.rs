pub mod psql;

use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn create_playlist(&self) -> ();
}
