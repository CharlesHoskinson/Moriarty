# Moriarty Candidate A handoff snapshot

Requested by the user on 2026-09-05. This directory preserves the typed session
state and otherwise uncommitted handoff material. It does not accept Candidate A.

- `session.db`: consistent SQLite online backup of the canonical main store.
- `session.ndjson`: portable typed snapshot produced by the released session CLI.
- `recovery.json`: point-in-time recovery, including freshness labels and open work.
- `manifest.json`: source anchors, file hashes, backup checks and source inventory.
- `openspec-validation.json`: terminal structural checks for the nine new changes.
- `handoff-validation.json`: XML, requirement mapping and recovery-backup checks.
- `preserved-boundary-and-drafts.tar.gz`: exact author report, RED/GREEN source
  closures, raw command receipts, and both unadopted next-unit draft plans from
  the S02 worktree. No new acceptance verdict follows from preservation.

See [roadmap](../../../docs/MORIARTY_ROADMAP.md) and
[Candidate A XML](../../../deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml).
All historical test counts remain tied to their original source. The session
database resolves both worktrees to one main store; verify branch ancestry and
scoped source hashes yourself before trusting a freshness label across branches.
Do not replay every obsolete historical measurement; use the latest relevant
source-bound command and retain the old result as history.

## Read and restore safely

The [EARS/OpenSpec index](../../../openspec/WORK-PACKAGES-EARS.md) names each work package.
The new specifications retain open implementation and acceptance tasks.

```bash
sqlite3 -readonly evidence/session-snapshots/2026-09-05-candidate-a-handoff/session.db 'PRAGMA integrity_check;'
tar -tzf evidence/session-snapshots/2026-09-05-candidate-a-handoff/preserved-boundary-and-drafts.tar.gz
node /home/charl/foreman/skills/foreman/runtime/dist/fm-session.js recover --json
```

The live canonical store remains `.foreman/session.db`; this snapshot is not a
replacement unless recovery is needed. Never copy a database over a live WAL
database. For an isolated restore, use the released `import-sidecar` command with
`--into` naming a new empty database outside the canonical store, then compare
records and integrity before any operator-approved replacement. Do not use force
or erase newer facts. Extract the tar only into a new temporary directory and
compare source hashes before restoring individual missing files.

The snapshot contains local operational paths and historical Foreman-related
records because it preserves the complete existing session database. It grants
no authority to resume dropped Foreman obligations. It is a local backup, not
a public publication, cloud backup, or guarantee of continued process execution.
