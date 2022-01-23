CREATE DATABASE spotify;

CREATE TABLE IF NOT EXISTS listen(
    song_id TEXT,
    time TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS playlist_arists(
    playlist_id TEXT,
    artist_id TEXT
);