use crate::settings::Database;
use async_trait::async_trait;
use std::format;
use tokio_postgres::{Client, Error, NoTls};

use super::Storage;

pub struct Psql {
    client: Client,
}

impl Psql {
    pub async fn new(conf: Database) -> Result<Box<dyn Storage>, Error> {
        let conn_str = format!(
            "postgresql://{user}:{password}@{host}/{dbname}",
            user = conf.user,
            password = conf.password,
            host = conf.host,
            dbname = conf.name
        );
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Box::new(Psql { client }))
    }
}

#[async_trait]
impl Storage for Psql {
    async fn create_playlist(&self) -> () {
        let results = self
            .client
            .query("SELECT * FROM listen", &[])
            .await
            .unwrap();
        for res in results {
            println!("{:?}", res);
        }
    }
}
