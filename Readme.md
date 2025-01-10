# Dad Joke Collector

Silly little program that periodically scrapes icanhazdadjoke.com to generate a local joke DB.

Comes with a random joke utility, useful to make your shell a little more fun. 

## Building

You can build it as a debian package with `cargo deb`, otherwise just `cargo build; cargo install`.

## Configuration

Config can be supplied with either `-c` to the server binary, or picked up via `~/.config/collector/config.toml` for the query binary.

There are only two config parameters. 
`database_path` - Where should the server binary/query binaries look for the joke DB
`rate_limit_per_minute` - How many requests per minute should the crawler issue

The server can be run as a systemd service, see [dad-joke-collector.service](./debian/dad-joke-collector.service) for details.

## Usage

Start the crawler

`/usr/bin/dad-jokes-collector`

It should tell you as it adds jokes to the SQLite DB:

```
dad-jokes-collector[261831]: 2025-01-10T00:49:00.760186Z  INFO dad_jokes_collector: New joke stored: What did the green grape say to the purple grape?
dad-jokes-collector[261831]: BREATH!!
```

Getting a joke from the DB is easy:
```
$ dad-jokes-query
Someone broke into my house last night and stole my limbo trophy. How low can you go?
```

Stick it in your .bashrc to be entertained on the daily ^_^
