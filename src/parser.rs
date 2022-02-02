use crate::service::spotify::Spotify;
use crate::service::Service;
use crate::{error::CoolioError, models::ThrowbackPeriod};
use clap::{app_from_crate, arg, App, AppSettings, ArgMatches};

pub struct Parser {
    matches: ArgMatches,
}

impl Parser {
    pub fn new() -> Self {
        let matches = app_from_crate!()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                App::new("history")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .about("History of listened tracks")
                    .subcommand(App::new("update").about("Updates the recent history"))
                    .subcommand(
                        App::new("throwback")
                            .about("Create a playlist of throwback songs")
                            .arg(arg!(-n --name [NAME] "the name of the playlist"))
                            .arg(
                                arg!(-p --period [PERIOD] "period of the throwback")
                                    .validator(|x| x.parse::<ThrowbackPeriod>()),
                            )
                            .arg(
                                arg!(-s --size [SIZE] "size of the playlist")
                                    .validator(|x| x.parse::<usize>()),
                            ),
                    ),
            )
            .subcommand(
                App::new("playlists")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
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
                    )
                    .subcommand(
                        App::new("show")
                            .about("Shows info for a playlist")
                            .arg(arg!(<PLAYLIST> "name of the playlist")),
                    ),
            )
            .get_matches();
        Parser { matches }
    }

    pub async fn parse<S: Spotify>(&self, service: Service<S>) -> Result<(), CoolioError> {
        match self.matches.subcommand() {
            Some(("history", history_matches)) => match history_matches.subcommand() {
                Some(("update", _update_matches)) => service.history.update().await,
                Some(("throwback", throwback_matches)) => {
                    service
                        .history
                        .throwback(
                            throwback_matches.value_of("name"),
                            throwback_matches.value_of_t("period").ok(),
                            throwback_matches.value_of_t("size").ok(),
                        )
                        .await
                }
                _ => unreachable!(),
            },
            Some(("playlists", playlists_matches)) => match playlists_matches.subcommand() {
                Some(("list", _list_matches)) => service.playlists.list().await,
                Some(("create", create_matches)) => {
                    service
                        .playlists
                        .create(create_matches.value_of("PLAYLIST").unwrap())
                        .await
                }
                Some(("link", link_matches)) => {
                    service
                        .playlists
                        .link_playlist_to_artist(
                            link_matches.value_of("PLAYLIST").unwrap(),
                            link_matches.value_of("ARTIST").unwrap(),
                            link_matches.value_of_t("seed").ok(),
                        )
                        .await
                }
                Some(("unlink", unlink_matches)) => {
                    service
                        .playlists
                        .unlink_artist_from_playlist(
                            unlink_matches.value_of("PLAYLIST").unwrap(),
                            unlink_matches.value_of("ARTIST").unwrap(),
                        )
                        .await
                }
                Some(("update", _update_matches)) => service.playlists.update().await,
                Some(("automate", automate_matches)) => {
                    service
                        .playlists
                        .automate(automate_matches.value_of("PLAYLIST").unwrap())
                        .await
                }
                Some(("show", show_matches)) => {
                    service
                        .playlists
                        .show(show_matches.value_of("PLAYLIST").unwrap())
                        .await
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
