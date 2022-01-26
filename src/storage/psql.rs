use crate::{error::CoolioError, models::Playlist, settings::Database};
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};

use super::Storage;
use crate::models::Listen;

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

        if res != 1 {
            Err(CoolioError::from("no values inserted"))
        } else {
            Ok(())
        }
    }

    async fn get_last_listen(&self) -> Result<Listen, CoolioError> {
        let query_text = "SELECT song_id, time FROM listen ORDER BY time DESC LIMIT 1";

        for row in self.client.query(query_text, &[]).await? {
            return Ok(Listen {
                song_id: row.get(0),
                time: row.get(1),
            });
        }

        Err(CoolioError::from("no listens found"))
    }

    async fn create_playlist(&self, id: &str, name: &str) -> Result<(), CoolioError> {
        let query_text =
            "INSERT INTO playlist(playlist_id, playlist_name, artist_id) VALUES($1, $2, NULL)";
        let res = self
            .client
            .execute(query_text, &[&id.to_string(), &name.to_string()])
            .await?;
        if res != 1 {
            Err(CoolioError::from("error in inserting of playlist"))
        } else {
            Ok(())
        }
    }

    async fn get_playlist(&self, name: &str) -> Result<Playlist, CoolioError> {
        let query_text = "SELECT playlist_id, artist_id FROM playlist WHERE playlist_name = $1";

        let mut artists = Vec::<String>::new();
        let mut id: Option<String> = None;
        for row in self.client.query(query_text, &[&name.to_string()]).await? {
            id = row.get(0);
            if let Some(artist) = row.get(1) {
                artists.push(artist);
            }
        }

        if let Some(id) = id {
            Ok(Playlist {
                id,
                artists,
                name: name.to_string(),
            })
        } else {
            Err(CoolioError::from("playlist doesnt exist"))
        }
    }

    async fn link_artist(
        &self,
        playlist_id: &str,
        playlist_name: &str,
        artist_id: &str,
    ) -> Result<(), CoolioError> {
        let query_text =
            "INSERT INTO playlist(playlist_id, playlist_name, artist_id) VALUES ($1, $2, $3)";

        let res = self
            .client
            .execute(
                query_text,
                &[
                    &playlist_id.to_string(),
                    &playlist_name.to_string(),
                    &artist_id.to_string(),
                ],
            )
            .await?;

        if res != 1 {
            Err(CoolioError::from("artist not linked to playlist"))
        } else {
            Ok(())
        }
    }
}
