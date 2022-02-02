mod builder;
mod error;
mod models;
mod parser;
mod service;
mod settings;
mod storage;

use std::env;

use error::CoolioError;
use parser::Parser;

use builder::{new_spotify, new_storage};
use service::Service;
use settings::Settings;

async fn execute() -> Result<(), CoolioError> {
    let parser = Parser::new(env::args_os());
    let settings = Settings::new()?;
    let spotify = new_spotify(settings.spotify).await;
    let storage = new_storage(settings.storage).await?;
    let service = Service::new(spotify, storage);

    parser.parse(service).await
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = execute().await {
        println!("{}", e)
    }
}

#[cfg(test)]
mod tests;
