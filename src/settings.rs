use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Spotify {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub spotify: Spotify,
    pub db: Database,
}

impl Settings {
    pub fn new() -> Result<Settings, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/settings.toml"))?;

        s.try_into()
    }
}
