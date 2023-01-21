# osu-api

Feature rich [`osu!`](https://osu.ppy.sh) API wrapper.

## Project structure

This project is separated by two part:

- api
    * v1
    * v2
- util
    * v1
    * v2

Each part of this project can be enable/disable by `--features` parameter.
The `api` part contains data structure serialize/deserialize and net IO.
The `util` part contains some useful utilities function.

## Custom client

If you don't like the default `reqwest::Client`, you can implement
the `OsuApiRequester` trait for your client.
