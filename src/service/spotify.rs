use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use rspotify::model::{
    AlbumId, AlbumType, ArtistId, FullArtist, FullPlaylist, FullTrack, Market, PlayableItem,
    PlaylistId, PlaylistItem, SearchResult, SearchType, SimplifiedArtist, SimplifiedPlaylist,
    SimplifiedTrack, TrackId,
};
use rspotify::prelude::*;
use rspotify::{model::TimeLimits, AuthCodeSpotify};

use crate::error::CoolioError;
use crate::models::{Listen, Playlist};

#[derive(Debug, Default, Clone)]
pub struct SimpleArtist {
    pub id: String,
    pub name: String,
    pub popularity: u32,
    pub num_followers: u32,
}

impl From<SimplifiedArtist> for SimpleArtist {
    fn from(a: SimplifiedArtist) -> Self {
        SimpleArtist {
            id: a.id.unwrap().uri(),
            name: a.name,
            popularity: 0,
            num_followers: 0,
        }
    }
}

impl From<FullArtist> for SimpleArtist {
    fn from(a: FullArtist) -> Self {
        SimpleArtist {
            id: a.id.uri(),
            name: a.name,
            popularity: a.popularity,
            num_followers: a.followers.total,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SimpleTrack {
    pub id: String,
    pub artists: Vec<SimpleArtist>,
}

#[derive(Debug, Default, Clone)]
pub struct SimplePlayable {
    pub added_at: Option<DateTime<Utc>>,
    pub track: SimpleTrack,
}

impl From<FullTrack> for SimpleTrack {
    fn from(t: FullTrack) -> Self {
        SimpleTrack {
            id: t.id.unwrap().uri(),
            artists: t.artists.into_iter().map(|a| a.into()).collect(),
        }
    }
}

impl From<SimplifiedTrack> for SimpleTrack {
    fn from(t: SimplifiedTrack) -> Self {
        SimpleTrack {
            id: t.id.unwrap().uri(),
            artists: t.artists.into_iter().map(|a| a.into()).collect(),
        }
    }
}

impl From<PlaylistItem> for SimplePlayable {
    fn from(pi: PlaylistItem) -> Self {
        SimplePlayable {
            added_at: pi.added_at,
            track: match pi.track {
                Some(PlayableItem::Track(t)) => t.into(),
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleAlbum {
    pub id: String,
    pub release_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Default)]
pub struct SimplePlaylist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub num_followers: u32,
    pub collaborative: bool,
    pub public: bool,
    pub tracks: Vec<SimplePlayable>,
}

impl From<FullPlaylist> for SimplePlaylist {
    fn from(p: FullPlaylist) -> Self {
        SimplePlaylist {
            id: p.id.uri(),
            name: p.name,
            description: p.description,
            num_followers: p.followers.total,
            collaborative: p.collaborative,
            public: p.public.unwrap_or(false),
            tracks: p.tracks.items.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<SimplifiedPlaylist> for SimplePlaylist {
    fn from(p: SimplifiedPlaylist) -> Self {
        SimplePlaylist {
            id: p.id.uri(),
            name: p.name,
            description: None,
            num_followers: 0,
            collaborative: p.collaborative,
            public: p.public.unwrap_or(false),
            tracks: vec![],
        }
    }
}

impl Into<Playlist> for SimplePlaylist {
    fn into(self) -> Playlist {
        Playlist {
            id: self.id,
            name: self.name,
            artists: vec![],
            automated: false,
        }
    }
}

#[async_trait]
pub trait Spotify: Send + Sync {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<Listen>, CoolioError>;

    async fn create_playlist(&self, name: &str) -> Result<SimplePlaylist, CoolioError>;

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &str,
        items: impl IntoIterator<Item = String> + Send + 'a,
    ) -> Result<(), CoolioError>;
    async fn current_user_playlists(&self) -> Result<Vec<SimplePlaylist>, CoolioError>;
    async fn artist_top_tracks(&self, name: &str) -> Result<Vec<SimpleTrack>, CoolioError>;
    async fn album_tracks(&self, id: &str) -> Result<Vec<SimpleTrack>, CoolioError>;
    async fn artist_albums(
        &self,
        id: &str,
        album_type: &AlbumType,
    ) -> Result<Vec<SimpleAlbum>, CoolioError>;

    async fn playlist(&self, id: &str) -> Result<SimplePlaylist, CoolioError>;
    async fn artist(&self, id: &str) -> Result<SimpleArtist, CoolioError>;
    async fn search_artists(&self, name: &str) -> Result<Vec<SimpleArtist>, CoolioError>;
}

pub struct HTTPSpotify {
    spotify: AuthCodeSpotify,
}

impl HTTPSpotify {
    pub fn new(spotify: AuthCodeSpotify) -> Self {
        HTTPSpotify { spotify }
    }
}

#[async_trait]
impl Spotify for HTTPSpotify {
    async fn current_user_recently_played(
        &self,
        limit: u32,
        time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<Listen>, CoolioError> {
        let last_listen = time_limit.map(|x| TimeLimits::After(x));

        Ok(self
            .spotify
            .current_user_recently_played(Some(limit), last_listen)
            .await?
            .items
            .into_iter()
            .map(|x| Listen {
                song_id: x.track.id.unwrap().uri(),
                time: x.played_at,
            })
            .collect::<Vec<Listen>>())
    }

    async fn create_playlist(&self, name: &str) -> Result<SimplePlaylist, CoolioError> {
        let me = self.spotify.current_user().await?;
        let playlist = self
            .spotify
            .user_playlist_create(&me.id, name, None, None, None)
            .await?;
        Ok(playlist.into())
    }

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &str,
        items: impl IntoIterator<Item = String> + Send + 'a,
    ) -> Result<(), CoolioError> {
        let please_live = items
            .into_iter()
            .map(|x| TrackId::from_uri(&x).unwrap())
            .collect::<Vec<TrackId>>();

        let to_add = please_live
            .iter()
            .map(|x| x as &dyn PlayableId)
            .collect::<Vec<&dyn PlayableId>>();

        self.spotify
            .playlist_add_items(&PlaylistId::from_uri(playlist_id)?, to_add, Some(0))
            .await?;
        Ok(())
    }

    async fn current_user_playlists(&self) -> Result<Vec<SimplePlaylist>, CoolioError> {
        let limit = 50;
        let mut offset = 0;

        let mut playlists = Vec::<SimplePlaylist>::new();

        loop {
            let fetched = self
                .spotify
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await?;

            for playlist in fetched.items {
                playlists.push(playlist.into())
            }

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

        Ok(playlists)
    }

    async fn playlist(&self, id: &str) -> Result<SimplePlaylist, CoolioError> {
        let p = self
            .spotify
            .playlist(&PlaylistId::from_uri(id)?, None, None)
            .await?;
        Ok(p.into())
    }

    async fn artist(&self, id: &str) -> Result<SimpleArtist, CoolioError> {
        let p = self.spotify.artist(&ArtistId::from_uri(id)?).await?;
        Ok(p.into())
    }

    async fn artist_top_tracks(&self, id: &str) -> Result<Vec<SimpleTrack>, CoolioError> {
        Ok(self
            .spotify
            .artist_top_tracks(&ArtistId::from_uri(id)?, &Market::FromToken)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<SimpleTrack>>())
    }

    async fn search_artists(&self, name: &str) -> Result<Vec<SimpleArtist>, CoolioError> {
        let r = self
            .spotify
            .search(name, &SearchType::Artist, None, None, Some(5), None)
            .await?;

        match r {
            SearchResult::Artists(a) => Ok(a.items.into_iter().map(|x| x.into()).collect()),
            _ => unreachable!(),
        }
    }

    async fn artist_albums(
        &self,
        id: &str,
        album_type: &AlbumType,
    ) -> Result<Vec<SimpleAlbum>, CoolioError> {
        let limit = 50;
        let mut offset = 0;
        let mut albums = Vec::<SimpleAlbum>::new();

        loop {
            let fetched = self
                .spotify
                .artist_albums_manual(
                    &ArtistId::from_uri(id)?,
                    Some(album_type),
                    None,
                    Some(limit),
                    Some(offset),
                )
                .await?;

            for a in fetched.items {
                if let Some(release_date) = a.release_date {
                    if let Some("day") = a.release_date_precision.as_ref().map(|x| x.as_str()) {
                        albums.push(SimpleAlbum {
                            id: a.id.unwrap().uri(),
                            release_date: DateTime::<Utc>::from_utc(
                                NaiveDateTime::parse_from_str(
                                    &(release_date + " 00:00:00"),
                                    "%Y-%m-%d %H:%M:%S",
                                )?,
                                Utc,
                            ),
                        })
                    }
                }
            }

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }

        Ok(albums)
    }

    async fn album_tracks(&self, id: &str) -> Result<Vec<SimpleTrack>, CoolioError> {
        let mut offset = 0;
        let limit = 50;

        let mut tracks = Vec::<SimpleTrack>::new();

        loop {
            let fetched = self
                .spotify
                .album_track_manual(&AlbumId::from_uri(id)?, Some(limit), Some(offset))
                .await?;

            for t in fetched.items {
                tracks.push(t.into())
            }

            if fetched.next.is_none() {
                break;
            }

            offset += limit;
        }
        Ok(tracks)
    }
}
