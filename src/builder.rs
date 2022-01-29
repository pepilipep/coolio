use std::collections::HashSet;

use rspotify::{prelude::*, AuthCodeSpotify, Config, Credentials, OAuth};

use crate::error::CoolioError;
use crate::settings::{Spotify, Storage as StorageConf};
use crate::storage::fs::Fs;
use crate::storage::psql::Psql;
use crate::storage::Storage;

pub async fn new_spotify(conf: Spotify) -> AuthCodeSpotify {
    let creds = Credentials::new(&conf.client_id, &conf.client_secret);

    let oauth = OAuth {
        redirect_uri: conf.redirect_uri,
        scopes: HashSet::from_iter(conf.scopes),
        ..Default::default()
    };

    let conf = Config {
        token_cached: true,
        token_refreshing: true,
        ..Config::default()
    };
    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, conf);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    spotify
}

pub async fn new_storage(conf: StorageConf) -> Result<Box<dyn Storage + Send + Sync>, CoolioError> {
    match conf {
        StorageConf::Psql(db) => Ok(Box::new(Psql::new(db).await?)),
        StorageConf::Fs(ls) => Ok(Box::new(Fs::new(ls).await?)),
    }
}
