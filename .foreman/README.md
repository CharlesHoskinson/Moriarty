# Typed execution checkpoint

`session.ndjson` is the portable, versioned export of the local checkpoint database.
It contains durable facts, scoped measurements, and open obligations. Recheck
measurement freshness before quoting results after code changes.

The live SQLite database and its WAL files are local runtime state and are ignored.
The execution loop uses the Codex runtime goal. The checkpoint is a recovery
record, not an independent scheduler or a claim that a process is running.
