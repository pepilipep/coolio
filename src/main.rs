mod parser;
mod service;
mod settings;
mod spotify;
mod storage;

use parser::Parser;

use settings::Settings;
use spotify::new_spotify;

use rspotify::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let parser = Parser::new();
    let settings = Settings::new().unwrap();
    let spotify = new_spotify(settings.spotify).await;

    parser.parse();

    let artists = spotify
        .current_user_top_artists_manual(None, Some(1), None)
        .await
        .unwrap();

    for artist in artists.items {
        println!("Artist {:?}", artist)
    }
}
