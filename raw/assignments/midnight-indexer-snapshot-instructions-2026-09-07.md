# Midnight Preprod indexer snapshot instructions

Authority/status: user input relayed by the parent agent; superseded as an
execution target by the subsequent user direction “let's go all in on preview”.
The reviewer did not receive the full original user message. The following is
an explicitly labeled relay summary, not a verbatim transcript. The parent must
append the original message if verbatim preservation is required.

Relayed summary:

- Snapshot URL: https://snapshots.midnight.network/indexer/preprod/green/2026-08-01-1907432/indexer.dump
- Claimed archive size: 199 GB; claimed restored database size: 450 GB.
- PostgreSQL image/version: 17.9-bookworm.
- Restore tool: pg_restore 17, with `--no-owner --no-privileges --jobs 8`.
- Expected checks: blocks maximum height 1,907,432; maximum successful migration
  3; wallet count 0.

These are supplied instructions/claims, not reproduced results. No restore was
performed. The bounded URL and storage assessment is in
`evidence/midnight-indexer-snapshot-assessment-2026-09-07/README.md`.

## Original user message appended by parent

The following preserves the original supplied text; it supersedes the relay
summary for transcription accuracy. It is not evidence that a restore ran.

```text
here are the instructions I got about the snapshots Charles Hoskinson  [12:42 PM]
And the indexer snapshots?
Bob Blessing-Hartley  [12:43 PM]
That was this:
# 1. Download (~199 GB). Either via CloudFront (public, no creds):
curl -fLO https://snapshots.midnight.network/indexer/preprod/green/2026-08-01-1907432/indexer.dump

# 2. You need a Postgres 17 server and ~450 GB free for the restored DB. Scratch one via Docker:
docker run -d --name indexer-pg -e POSTGRES_PASSWORD=postgres -p 5432:5432 \
  -v "$PWD/pgdata:/var/lib/postgresql/data" postgres:17.9-bookworm

# 3. Create the database and restore (pg_restore must be v17; zstd-compressed archive):
createdb -h localhost -U postgres indexer
pg_restore -h localhost -U postgres --dbname=indexer \
  --no-owner --no-privileges --jobs=8 indexer.dump

Then confirm it's the real thing:

SELECT max(height) FROM blocks;                      -- expect 1907432
SELECT max(version) FROM _sqlx_migrations WHERE success;  -- expect 3
SELECT count(*) FROM wallets;                        -- expect 0 (schema present, data excluded)
```
