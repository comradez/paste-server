# paste-server

An online paste and file transfer service backend.

## Config

Requires redis, use environment variable `DATABASE_URL` to specify it.

Use environment variable `FILECACHE_DIR` to specify the file cache dir.

you may use a `.env` file for hygiene, which is also convenient for systemd to run the service.

## Run

Simply compile and run will do.
