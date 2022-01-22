use std::collections::HashSet;

use rspotify::{prelude::*, AuthCodeSpotify, Config, Credentials, OAuth};

use crate::settings::Spotify;

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
