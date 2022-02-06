use std::io::{BufRead, Write};

use crate::{error::CoolioError, models::Playlist};

use super::spotify::{SimpleArtist, SimplePlaylist};

pub struct Interactor<'a, R: BufRead + Send + Sync, W: Write + Send + Sync> {
    reader: R,
    writer: &'a mut W,
}

impl<'a, R: BufRead + Send + Sync, W: Write + Send + Sync> Interactor<'a, R, W> {
    pub fn new(reader: R, writer: &'a mut W) -> Self {
        Interactor { reader, writer }
    }

    pub fn list_playlist(&mut self, ps: Vec<Playlist>) -> Result<(), CoolioError> {
        for playlist in ps {
            if playlist.automated {
                writeln!(
                    self.writer,
                    "{} [automated, number of artists: {}]",
                    playlist.name,
                    playlist.artists.len()
                )?;
            } else {
                writeln!(self.writer, "{}", playlist.name)?;
            }
        }
        Ok(())
    }

    pub fn show_playlist(
        &mut self,
        external_playlist: &SimplePlaylist,
        artists: &Vec<SimpleArtist>,
    ) -> Result<(), CoolioError> {
        writeln!(self.writer, "Artists:")?;
        for artist in artists {
            writeln!(
                self.writer,
                "\t{} (popularity: {}, followers: {})",
                artist.name, artist.popularity, artist.num_followers
            )?;
        }

        writeln!(
            self.writer,
            "Description: {:?}",
            external_playlist.description
        )?;
        writeln!(
            self.writer,
            "Number of tracks: {}",
            external_playlist.tracks.len()
        )?;
        writeln!(
            self.writer,
            "Number of followers: {}",
            external_playlist.num_followers
        )?;
        writeln!(
            self.writer,
            "Is collaborative: {}",
            external_playlist.collaborative
        )?;
        writeln!(self.writer, "Is public: {:?}", external_playlist.public)?;
        Ok(())
    }

    pub fn choose_artist(&mut self, artists: &Vec<SimpleArtist>) -> Result<String, CoolioError> {
        let mut count_id = 1;
        writeln!(self.writer, "choose one of the following artists:")?;
        for art in artists {
            writeln!(
                self.writer,
                "[{}] {} (followers: {})",
                count_id, art.name, art.num_followers
            )?;
            count_id += 1;
        }

        let chosen: usize;
        loop {
            let mut input = String::new();
            self.reader.read_line(&mut input)?;

            input = input.trim().to_string();

            if let Ok(choice) = input.parse::<usize>() {
                if choice >= 1 && choice <= artists.len() {
                    chosen = choice;
                    break;
                }
            }
            writeln!(self.writer, "Wrong choice. Try again")?;
        }

        Ok(artists[chosen - 1].id.clone())
    }
}
