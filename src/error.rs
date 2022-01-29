use std::{fmt, io};

use chrono::ParseError;
use config::ConfigError;
use rspotify::model::IdError;

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

impl From<IdError> for CoolioError {
    fn from(e: IdError) -> Self {
        CoolioError {
            msg: format!("Spotify id extract error received: {}", e),
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

impl From<io::Error> for CoolioError {
    fn from(e: io::Error) -> Self {
        CoolioError {
            msg: format!("IO error received: {}", e),
        }
    }
}

impl From<ParseError> for CoolioError {
    fn from(e: ParseError) -> Self {
        CoolioError {
            msg: format!("Parsing time error: {}", e),
        }
    }
}

impl From<csv::Error> for CoolioError {
    fn from(e: csv::Error) -> Self {
        CoolioError {
            msg: format!("Persisting to file error: {}", e),
        }
    }
}
