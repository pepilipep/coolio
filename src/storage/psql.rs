use crate::settings::Database;
use postgres::{Client, Error, NoTls};
use std::format;

pub struct Psql {
    client: Client,
}

impl Psql {
    pub fn new(conf: Database) -> Result<Self, Error> {
        let conn_str = format!(
            "postgresql://{user}:{password}@{host}/{dbname}",
            user = conf.user,
            password = conf.password,
            host = conf.host,
            dbname = conf.name
        );
        let client = Client::connect(&conn_str, NoTls)?;
        Ok(Psql { client })
    }
}
