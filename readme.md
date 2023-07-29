# Wuxia2Kindle

A simple tool to take chapters from [Wuxia](https://www.wuxiaworld.com/) and make ePub with
them for convenient and offline reading.

The main issue I had with Wuxia it that I can only read on the web or with the iOS app.
The iOS app is great, but I want to use the kindle as it's the perfect size between my
iPhone and iPad Pro. Plus the e-ink screen is much better.

So this is how I started this small project.

## notes

For now, it's 'Wuxia' to 'Kindle' because this is the only usecase I have. It's not impossible that
I extend the features and integrations in the future.

## structure

### boost

The `boost` directory simply contains 2 files ([boost.js](boost/boost.js) and [boost.css](boost/boost.css)).
These files are the content of would be a [boost integration in Arc Browser](https://arc.net/boosts).

It's basically the same thing as an browser extension, but I don't have to deal with building, packaging and
shipping. I just copy/paste code in my browser.

### app

The (backend) app is seperated in 2 commands:
- ingest: HTTP server that exposes the ingest API
- worker: Long running process that polls for exports to be made and process them

#### ingest

The ingest receives chapters from the boost script and put them in the (postgres) DB.
There are some more CRUD endpoints for the client later.

#### worker

The worker(s ?) queries the DB to get unprocessed exports and start processing them:
- merging text for requested chapters
- creating the ePub
- sending it by mail to the `@kindle.com` mail

### client

The client is (for now nothing) a simple tool to help manage the content of the DB:
- add covers to books
- manage chapters
- create new exports

## start the whole thing

### database

_Requires_ [devenv](https://devenv.sh/)

_Requires_ [sqlx-cli](https://lib.rs/crates/sqlx-cli)

In one terminal:
```bash
# Starts the postgres DB, and should use the `schema.sql` file to prepare it. If not, bummer.
$> devenv up
```

In another one:
```bash
# Runs the DB migrations
$> sqlx migrate run -D postgres://localhost:5433/wuxia2kindle
```

At this point the DB should be ready with the most up to date schema.


### app

To start the `ingest` service:
```bash
$> cargo run -- ingest -p 3000 --database-url postgres://localhost:5433/wuxia2kindle
```

To start the `worker` service:
```bash
$> cargo run -- worker --database-url postgres://localhost:5433/wuxia2kindle --smtp-server "127.0.0.1" --smtp-port 1025 --smtp-user "your@email.com" -smtp-password "your_secure_passwd" --send-to "yourkindle@kindle.com"
```


### client

To start the client:
```bash
$> deno task start
```
