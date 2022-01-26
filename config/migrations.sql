CREATE DATABASE spotify;

CREATE TABLE IF NOT EXISTS listen(
    song_id TEXT,
    time TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS playlist(
    playlist_name TEXT,
    playlist_id TEXT,
    artist_id TEXT,
    UNIQUE (playlist_id, artist_id)
);