mod settings;

use std::collections::HashSet;

use rspotify::{prelude::*, AuthCodeSpotify, Config, Credentials, OAuth};

use settings::Settings;

use clap::{app_from_crate, arg};

#[tokio::main]
async fn main() {
    let matches = app_from_crate!()
        .arg(arg!(--test <VALUE>))
        .arg(arg!(--test2 <VALUE>))
        .get_matches();

    println!(
        "test - {:?}, test2 - {:?}",
        matches.value_of("test").expect("required"),
        matches.value_of("test2").expect("required")
    );

    env_logger::init();

    let settings = Settings::new().unwrap();

    let creds = Credentials::new(&settings.spotify.client_id, &settings.spotify.client_secret);

    let oauth = OAuth {
        redirect_uri: settings.spotify.redirect_uri,
        scopes: HashSet::from_iter(settings.spotify.scopes),
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

    let artists = spotify
        .current_user_top_artists_manual(None, None, None)
        .await
        .unwrap();

    for artist in artists.items {
        println!("Artist {:?}", artist)
    }
}
