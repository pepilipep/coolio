use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use rspotify::model::AlbumType;
use tokio::sync::Mutex;

use crate::{
    error::CoolioError,
    models::Listen,
    service::spotify::{
        SimpleAlbum, SimpleArtist, SimplePlayable, SimplePlaylist, SimpleTrack, Spotify,
    },
};

#[derive(Clone, Default)]
pub struct SpotifyState {
    pub playlists: Vec<SimplePlaylist>,
}

struct TestAlbum {
    pub album: SimpleAlbum,
    pub tracks: Vec<String>,
}

struct TestArtist {
    pub artist: SimpleArtist,
    pub albums: Vec<TestAlbum>,
}

pub struct MockSpotify {
    pub state: Mutex<SpotifyState>,
    artists: Vec<TestArtist>,
}

impl MockSpotify {
    pub fn new() -> Self {
        MockSpotify {
            state: Mutex::new(SpotifyState::default()),
            artists: vec![
                TestArtist {
                    artist: SimpleArtist {
                        id: "artist_1".to_string(),
                        name: "kendrick lamar".to_string(),
                        num_followers: 1123829,
                        popularity: 91,
                    },
                    albums: vec![
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_1_1".to_string(),
                                release_date: Utc.timestamp(1431648000, 0),
                            },
                            tracks: vec![
                                "track_1".to_string(),
                                "track_2".to_string(),
                                "track_3".to_string(),
                            ],
                        },
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_1_2".to_string(),
                                release_date: Utc.timestamp(1432648000, 0),
                            },
                            tracks: vec![
                                "track_4".to_string(),
                                "track_5".to_string(),
                                "track_6".to_string(),
                            ],
                        },
                    ],
                },
                TestArtist {
                    artist: SimpleArtist {
                        id: "artist_2".to_string(),
                        name: "rick ross".to_string(),
                        num_followers: 1234,
                        popularity: 44,
                    },
                    albums: vec![
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_2_1".to_string(),
                                release_date: Utc.timestamp(1431648000, 0),
                            },
                            tracks: vec![
                                "track_7".to_string(),
                                "track_8".to_string(),
                                "track_9".to_string(),
                            ],
                        },
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_2_2".to_string(),
                                release_date: Utc.timestamp(1432648000, 0),
                            },
                            tracks: vec![
                                "track_10".to_string(),
                                "track_11".to_string(),
                                "track_12".to_string(),
                            ],
                        },
                    ],
                },
                TestArtist {
                    artist: SimpleArtist {
                        id: "artist_3".to_string(),
                        name: "kali uchis".to_string(),
                        num_followers: 928173,
                        popularity: 72,
                    },
                    albums: vec![
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_3_1".to_string(),
                                release_date: Utc.timestamp(1431648000, 0),
                            },
                            tracks: vec![
                                "track_13".to_string(),
                                "track_14".to_string(),
                                "track_15".to_string(),
                            ],
                        },
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_3_2".to_string(),
                                release_date: Utc.timestamp(1432648000, 0),
                            },
                            tracks: vec![
                                "track_16".to_string(),
                                "track_17".to_string(),
                                "track_18".to_string(),
                            ],
                        },
                    ],
                },
                TestArtist {
                    artist: SimpleArtist {
                        id: "artist_4".to_string(),
                        name: "arctic monkeys".to_string(),
                        num_followers: 213728137,
                        popularity: 99,
                    },
                    albums: vec![
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_4_1".to_string(),
                                release_date: Utc.timestamp(1431648000, 0),
                            },
                            tracks: vec![
                                "track_19".to_string(),
                                "track_20".to_string(),
                                "track_21".to_string(),
                            ],
                        },
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_4_2".to_string(),
                                release_date: Utc.timestamp(1432648000, 0),
                            },
                            tracks: vec![
                                "track_22".to_string(),
                                "track_23".to_string(),
                                "track_24".to_string(),
                            ],
                        },
                    ],
                },
                TestArtist {
                    artist: SimpleArtist {
                        id: "artist_5".to_string(),
                        name: "dua lipa".to_string(),
                        num_followers: 8272722,
                        popularity: 98,
                    },
                    albums: vec![
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_5_1".to_string(),
                                release_date: Utc.timestamp(1431648000, 0),
                            },
                            tracks: vec![
                                "track_25".to_string(),
                                "track_26".to_string(),
                                "track_27".to_string(),
                            ],
                        },
                        TestAlbum {
                            album: SimpleAlbum {
                                id: "album_5_2".to_string(),
                                release_date: Utc.timestamp(1432648000, 0),
                            },
                            tracks: vec![
                                "track_28".to_string(),
                                "track_29".to_string(),
                                "track_30".to_string(),
                            ],
                        },
                    ],
                },
            ],
        }
    }
}

#[async_trait]
impl Spotify for MockSpotify {
    async fn current_user_recently_played(
        &self,
        _limit: u32,
        _time_limit: Option<DateTime<Utc>>,
    ) -> Result<Vec<Listen>, CoolioError> {
        Ok(vec![
            Listen {
                song_id: "track_2".to_string(),
                time: Utc.timestamp(1580930644, 0),
            },
            Listen {
                song_id: "track_1".to_string(),
                time: Utc.timestamp(1580930644, 0),
            },
            Listen {
                song_id: "track_1".to_string(),
                time: Utc.timestamp(1580930644, 0),
            },
            Listen {
                song_id: "track_1".to_string(),
                time: Utc.timestamp(1580930644, 0),
            },
            Listen {
                song_id: "track_2".to_string(),
                time: Utc.timestamp(1580930644, 0),
            },
            Listen {
                song_id: "track_4".to_string(),
                time: Utc::now() - Duration::days(2),
            },
            Listen {
                song_id: "track_3".to_string(),
                time: Utc.timestamp(1580930644, 0),
            },
            Listen {
                song_id: "track_3".to_string(),
                time: Utc::now() - Duration::days(3),
            },
            Listen {
                song_id: "track_3".to_string(),
                time: Utc::now(),
            },
        ])
    }

    async fn create_playlist(&self, name: &str) -> Result<SimplePlaylist, CoolioError> {
        let mut p = SimplePlaylist::default();
        p.name = name.to_string();
        p.id = format!("{}_id", p.name);
        self.state.lock().await.playlists.push(p.clone());
        Ok(p)
    }

    async fn playlist_add_items<'a>(
        &self,
        playlist_id: &str,
        items: impl IntoIterator<Item = String> + Send + 'a,
    ) -> Result<(), CoolioError> {
        let ps = &mut self.state.lock().await.playlists;
        for p in ps {
            if p.id == playlist_id {
                let now = Utc::now();
                let mut playable_items = Vec::<SimplePlayable>::new();
                for i in items.into_iter() {
                    playable_items.push(SimplePlayable {
                        added_at: Some(now),
                        track: SimpleTrack {
                            id: i.clone(),
                            artists: vec![],
                        },
                    })
                }
                p.tracks.append(&mut playable_items);
                return Ok(());
            }
        }
        Err("playlist doesnt exist".into())
    }

    async fn current_user_playlists(&self) -> Result<Vec<SimplePlaylist>, CoolioError> {
        Ok(self.state.lock().await.playlists.to_vec())
    }

    async fn artist_top_tracks(&self, id: &str) -> Result<Vec<SimpleTrack>, CoolioError> {
        for art in &self.artists {
            if art.artist.id == id {
                let mut tracks = Vec::<SimpleTrack>::new();
                for alb in &art.albums {
                    for t in &alb.tracks {
                        tracks.push(SimpleTrack {
                            id: t.clone(),
                            artists: vec![art.artist.clone()],
                        })
                    }
                }
                return Ok(tracks);
            }
        }
        Err("artist doesnt exist".into())
    }
    async fn album_tracks(&self, id: &str) -> Result<Vec<SimpleTrack>, CoolioError> {
        for art in &self.artists {
            for alb in &art.albums {
                if alb.album.id == id {
                    let mut tracks = Vec::<SimpleTrack>::new();
                    for t in &alb.tracks {
                        tracks.push(SimpleTrack {
                            id: t.clone(),
                            artists: vec![art.artist.clone()],
                        })
                    }
                    return Ok(tracks);
                }
            }
        }
        Err("album doesnt exist".into())
    }
    async fn artist_albums(
        &self,
        id: &str,
        _album_type: &AlbumType,
    ) -> Result<Vec<SimpleAlbum>, CoolioError> {
        for a in &self.artists {
            if a.artist.id == id {
                let mut albums = Vec::<SimpleAlbum>::new();
                for alb in &a.albums {
                    albums.push(alb.album.clone());
                }
                return Ok(albums);
            }
        }
        Err("artist doesnt exist".into())
    }

    async fn playlist(&self, id: &str) -> Result<SimplePlaylist, CoolioError> {
        let ps = self.state.lock().await.playlists.to_vec();
        for p in ps {
            if p.id == id {
                return Ok(p);
            }
        }
        Err("playlist doesnt exist".into())
    }
    async fn artist(&self, id: &str) -> Result<SimpleArtist, CoolioError> {
        for a in &self.artists {
            if a.artist.id == id {
                return Ok(a.artist.clone());
            }
        }
        Err("artist doesnt exist".into())
    }
    async fn search_artists(&self, name: &str) -> Result<Vec<SimpleArtist>, CoolioError> {
        let mut artists = Vec::<SimpleArtist>::new();
        for a in &self.artists {
            if a.artist.name.contains(name) {
                artists.push(a.artist.clone())
            }
        }
        Ok(artists)
    }
}
