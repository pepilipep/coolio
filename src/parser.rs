use crate::error::CoolioError;
use crate::service::Service;
use crate::{service::history::History, storage::Storage};
use clap::{app_from_crate, App, ArgMatches};

pub struct Parser {
    matches: ArgMatches,
}

impl Parser {
    pub fn new() -> Self {
        let matches = app_from_crate!()
            .subcommand(
                App::new("history")
                    .about("History of listened tracks")
                    .subcommand(App::new("update").about("Updates the recent history")),
            )
            .get_matches();
        Parser { matches }
    }

    pub async fn parse<S: Storage + Sync + Send>(
        &self,
        service: Service<S>,
    ) -> Result<(), CoolioError> {
        match self.matches.subcommand() {
            Some(("history", history_matches)) => match history_matches.subcommand() {
                Some(("update", _update_matches)) => service.update().await,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
