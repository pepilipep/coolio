# Coolio

`coolio` is a CLI tool which enables some extra features for your Spotify.
It is implemented in Rust, so you can install it using `cargo`:

```bash
cargo install --path .
```

## Spinning things up

`coolio` can use either filesystem or `postgres` for storage. If you want to go with a database, you would need to do bring up a postgres instance for it. You can do that using `docker` for example:

```bash
cd config
docker-compose -f docker-compose-storage-yml up -d
```

After that you should run the migrations (located in `config/migrations.sql`) against the postgres db. 

## Playlists automation

Creating a playlist and linking artists to it is as simple as:

```bash
coolio playlists create <name>
coolio playlist link <playlist> <artist>
```

Bringing the automated playlists up-to-date happens with:
```bash
coolio playlists update
```

For full details on what you can do, just browse the help.

## Listen history tracking

To enable one of its feature, `coolio` would need to record history for you for some period of time. That is because of the limitation of the Spotify API.

Update of the history happens with this command:

```bash
coolio history update
```

## Automating calls

You need to somehow automate execution of the `update` commands consistently over time. I do this using `crontab` and here are my settings. It runs `history` updates every 30 minutes and `playlists` updates twice a day:

```bash
0-59/30 * * * * cd <path-to-source>/coolio && <path-to-executable>/coolio history update
15 0-23/12 * * * cd <path-to-source>/coolio && <path-to-executable>/coolio playlists update
```
