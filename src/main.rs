mod builder;
mod parser;
mod service;
mod settings;
mod storage;

use parser::Parser;

use builder::{new_spotify, new_storage};
use settings::Settings;

use rspotify::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let parser = Parser::new();
    let settings = Settings::new().unwrap();
    let spotify = new_spotify(settings.spotify).await;
    let storage = new_storage(settings.storage).await.unwrap();
    storage.create_playlist().await;

    parser.parse();
}
