use crate::{error::CoolioError, settings::Database};
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};

use super::{models::Listen, Storage};

pub struct Psql {
    client: Client,
}

impl Psql {
    pub async fn new(conf: Database) -> Result<Self, CoolioError> {
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

        Ok(Psql { client })
    }
}

#[async_trait]
impl Storage for Psql {
    async fn add_history(&self, listen: Listen) -> Result<(), CoolioError> {
        let query_text = "INSERT INTO listen VALUES ($1, $2)";

        let res = self
            .client
            .execute(query_text, &[&listen.song_id, &listen.time])
            .await?;
        println!("{:?}", res);

        Ok(())
    }
}
