pub mod models;
pub mod psql;

use async_trait::async_trait;

use crate::error::CoolioError;

use self::models::Listen;

#[async_trait]
pub trait Storage {
    async fn add_history(&self, listen: Listen) -> Result<(), CoolioError>;
}
