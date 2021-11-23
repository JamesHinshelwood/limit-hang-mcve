# DB Setup

```sql
CREATE DATABASE hang;
CREATE TABLE users (id VARCHAR PRIMARY KEY, done BOOLEAN NOT NULL DEFAULT FALSE);
-- Populate the table with a bunch of random data
CREATE EXTENSION IF NOT EXISTS pgcrypto;
INSERT INTO users (id) SELECT gen_random_uuid()::VARCHAR FROM generate_series(1,1000);
```

# Connection Setup

Change the connection string on line 9 of `main.rs` to point to your Postgres instance.

# The Issue

This hopefully simple program picks a batch of rows out from the DB and tries to run an `UPDATE` on each one.
Note that the rows are queried using `query_raw` which returns a `RowStream`.
I was not able to reproduce the issue when using `query` which returns a `Vec<Row>`.

When executing the program, the query to grab a batch of rows succeeds, but the first `.execute` call to update a row never returns.
This only occurs when the `LIMIT` on the `SELECT` is set to a high number such as `500`.
When testing with a `LIMIT` of `20`, everything works fine.
It also only occurs when the specified table has a large number of rows in it, which will actually be returned by the `SELECT`.

# Logs

After running with trace logging enabled with `RUST_LOG=trace cargo run`, I recorded the following logs:

```
[2021-11-23T14:22:18Z TRACE mio::poll] registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] flushing framed transport
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] writing; remaining=58
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] framed transport flushed
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z DEBUG tokio_postgres::prepare] preparing query s0: SELECT * FROM users LIMIT 500
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_read: waiting on response
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] polled new request
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_write: waiting on request
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] flushing framed transport
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] writing; remaining=54
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] framed transport flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_flush: flushed
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_read: waiting on response
[2021-11-23T14:22:18Z DEBUG tokio_postgres::query] executing statement s0 with parameters: []
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_write: waiting on request
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] flushing framed transport
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] framed transport flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_flush: flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_read: waiting on response
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] polled new request
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_write: waiting on request
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] flushing framed transport
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] writing; remaining=34
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] framed transport flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_flush: flushed
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
Updating rows...
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z DEBUG tokio_postgres::prepare] preparing query s1: UPDATE users SET done = TRUE WHERE id = $1
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] attempting to decode a frame
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] frame decoded from buffer
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_read: waiting on sender
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] polled new request
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_write: waiting on request
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] flushing framed transport
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] writing; remaining=67
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] framed transport flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_flush: flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] retrying pending response
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_read: waiting on sender
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_write: waiting on request
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] flushing framed transport
[2021-11-23T14:22:18Z TRACE tokio_util::codec::framed_impl] framed transport flushed
[2021-11-23T14:22:18Z TRACE tokio_postgres::connection] poll_flush: flushed
[the program hangs here]
```

I believe it may be something to do with the `poll_read: waiting on sender` lines, as I don't observe these when running with a `LIMIT` of `20`, but I am not sure.
