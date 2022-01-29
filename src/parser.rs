use crate::error::CoolioError;
use crate::service::playlists::Playlists;
use crate::service::Service;
use crate::{service::history::History, storage::Storage};
use clap::{app_from_crate, arg, App, ArgMatches};

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
            .subcommand(
                App::new("playlists")
                    .about("Manage automated playlists")
                    .subcommand(App::new("list").about("Lists the playlists"))
                    .subcommand(
                        App::new("create")
                            .about("Creates an automated playlist")
                            .arg(arg!(<PLAYLIST> "name of the playlist")),
                    )
                    .subcommand(
                        App::new("link")
                            .about("Links the artist to an automated playlist")
                            .arg(arg!(<PLAYLIST> "name of the playlist"))
                            .arg(arg!(<ARTIST> "name of the artist"))
                            .arg(
                                arg!(-s --seed [SEED] "number of songs to seed")
                                    .validator(|x| x.parse::<usize>()),
                            ),
                    )
                    .subcommand(
                        App::new("unlink")
                            .about("Unlinks the artist from the playlist")
                            .arg(arg!(<PLAYLIST> "name of the playlist"))
                            .arg(arg!(<ARTIST> "name of the artist")),
                    )
                    .subcommand(
                        App::new("update").about("Adds new artists' songs to the playlists"),
                    )
                    .subcommand(
                        App::new("automate")
                            .about("Automates an already existing playlist in Spotify")
                            .arg(arg!(<PLAYLIST> "name of the playlist")),
                    ),
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
                Some(("update", _update_matches)) => service.history_update().await,
                _ => unreachable!(),
            },
            Some(("playlists", playlists_matches)) => match playlists_matches.subcommand() {
                Some(("list", _list_matches)) => service.list_playlists().await,
                Some(("create", create_matches)) => {
                    service
                        .create_playlist(create_matches.value_of("PLAYLIST").unwrap())
                        .await
                }
                Some(("link", link_matches)) => {
                    service
                        .link_playlist_to_artist(
                            link_matches.value_of("PLAYLIST").unwrap(),
                            link_matches.value_of("ARTIST").unwrap(),
                            link_matches.value_of_t("seed").ok(),
                        )
                        .await
                }
                Some(("unlink", unlink_matches)) => {
                    service
                        .unlink_artist_from_playlist(
                            unlink_matches.value_of("PLAYLIST").unwrap(),
                            unlink_matches.value_of("ARTIST").unwrap(),
                        )
                        .await
                }
                Some(("update", _update_matches)) => service.playlists_update().await,
                Some(("automate", automate_matches)) => {
                    service
                        .automate_playlist(automate_matches.value_of("PLAYLIST").unwrap())
                        .await
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
