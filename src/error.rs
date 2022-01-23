use std::fmt;

use config::ConfigError;

#[derive(Debug)]
pub struct CoolioError {
    msg: String,
}

impl fmt::Display for CoolioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<tokio_postgres::Error> for CoolioError {
    fn from(e: tokio_postgres::Error) -> Self {
        CoolioError {
            msg: format!("Db error received: {}", e),
        }
    }
}

impl From<rspotify::ClientError> for CoolioError {
    fn from(e: rspotify::ClientError) -> Self {
        CoolioError {
            msg: format!("Spotify API error received: {}", e),
        }
    }
}

impl From<ConfigError> for CoolioError {
    fn from(e: ConfigError) -> Self {
        CoolioError {
            msg: format!("Config error received: {}", e),
        }
    }
}

impl From<&str> for CoolioError {
    fn from(s: &str) -> Self {
        CoolioError { msg: s.to_string() }
    }
}
