use async_trait::async_trait;

use rspotify::model::SearchResult;
use rspotify::model::SearchType;
use rspotify::prelude::*;
use rspotify::AuthCodeSpotify;

use crate::error::CoolioError;
use crate::models::Playlist;
use crate::storage::Storage;

#[async_trait]
pub trait Playlists<S: Storage + Send + Sync> {
    fn get_spotify(&self) -> &AuthCodeSpotify;
    fn get_storage(&self) -> &S;

    async fn list_playlists(&self) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();

        let limit = 50;
        let mut offset = 0;

        let mut playlists = Vec::<Playlist>::new();

        loop {
            let fetched = spotify
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await?;

            for playlist in fetched.items {
                playlists.push(Playlist {
                    id: playlist.id.uri(),
                    name: playlist.name,
                    artists: Vec::<String>::new(),
                })
            }

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

        println!("{:?}", playlists);

        Ok(())
    }

    async fn create_playlist(&self, name: &str) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();
        let me = spotify.current_user().await?;
        let playlist = spotify
            .user_playlist_create(&me.id, name, Some(false), None, None)
            .await?;
        storage
            .create_playlist(&playlist.id.uri(), &playlist.name)
            .await?;

        Ok(())
    }

    async fn link_playlist_to_artist(
        &self,
        playlist: &str,
        artist: &str,
    ) -> Result<(), CoolioError> {
        let spotify = self.get_spotify();
        let storage = self.get_storage();

        let playlist = storage.get_playlist(playlist).await?;

        let potentials_artists = spotify
            .search(artist, &SearchType::Artist, None, None, Some(5), None)
            .await?;

        if let SearchResult::Artists(artists) = potentials_artists {
            let mut count_id = 1;
            println!("choose one of the following artists:");
            for art in &artists.items {
                println!(
                    "[{}] {} (followers: {})",
                    count_id, art.name, art.followers.total
                );
                count_id += 1;
            }

            let chosen: usize;
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                input = input.trim().to_string();

                if let Ok(choice) = input.parse::<usize>() {
                    if choice >= 1 && choice < artists.items.len() {
                        chosen = choice;
                        break;
                    }
                }
                println!("Wrong choice. Try again")
            }

            storage
                .link_artist(
                    &playlist.id,
                    &playlist.name,
                    &artists.items[chosen - 1].id.uri(),
                )
                .await
        } else {
            unreachable!()
        }
    }
}
