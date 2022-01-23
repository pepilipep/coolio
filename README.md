# Coolio

`coolio` is a CLI tool which enables some extra features for your Spotify.
It is implemented in Rust, so you can install it using `cargo`:

```
cargo install --path .
```

## Spinning things up

`coolio` uses `postgres` for storage, so you would need to do bring up a postgres instance for it. You can do that using `docker` for example:

```
cd config
docker-compose -f docker-compose-storage-yml up -d
```

After that you should run the migrations (located in `config/migrations.sql`) against the postgres db. 

## Listen history tracking

To enable one of its feature, it would need to record history for you for some period of time. That is because of the limitation of the Spotify API.

Update of the history happens with this command:

```
coolio history update
```

You need to somehow automate execution of this command consistently over time. I do this using `crontab`, here is my crontab setting (runs every 10 minutes):

```
0-59/10 * * * * cd <path-to-source>/coolio && <path-to-executable>/coolio history update
```
