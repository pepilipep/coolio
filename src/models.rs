use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Listen {
    pub song_id: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
}
