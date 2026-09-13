# Independent snapshot11 controls

Scratch candidate11 was composed from canonical10 plus the author's exact diagnostic-only changes. Canonical files were not changed. Fourteen source/mechanism tests ran: six passed, eight failed. The script reports test failures in JSON; its exit0 means evidence writing completed, not that tests passed.

Confirmed missing predicates:

- snapshotSHA256 can be absent.
- Real UID and real GID drift are accepted; effective identity drift rejects.
- OS identity files need not be exactly /etc/passwd and /etc/nsswitch.conf.
- Snapshot root can broaden from binding.kRoot to /home/charl.
- Extra mode bits are ignored.
- Compiled directory names from os.walk are ignored. The regression injects an extra directory-name observation; it does not create a symlink in the original tree. Symlink-directory rejection is therefore absent.
- Temporal metadata replacement bypasses verified inventory: actual verify_snapshot checks the original, but the second manifest hash and subsequent JSON read are separate. Replacing fixture bytes after that hash lets /etc/passwd enter the sealed506-file set and final injected argv without verified union/root membership.

Required repair: require a valid digest, hash and parse the same raw bytes, validate exact root/modes/real and effective IDs/OS file set and directory types, then carry that verified object directly into sealing and argv. Do not reload mutable metadata after verification.

Positive controls establish actual506-file sealing, write-prohibiting seals, unchanged sealed bytes after temporary source mutation, mismatch rejection, partial-FD cleanup and original diagnostic argv suffix under injected exec. These are source controls, not native results.

The separate snapshot-envelope-controls-11/result.json passes six actual existing runner/temporary-store cases and30 identity rejections. Each runs candidate11's exact506-file bwrap construction around harmless Python under actual strong Foreman. The child verifies the private byte/mode inventory, preserved HOME/Path.home/getpwuid, real/effective UID/GID1000, cwd,8MiB stack, read-only view and no leaked snapshotmemfds. InvalidUTF8,124 and overflow reject recovery; detached-child readiness is seen and no matching host process survives. No K/Java/strace runs.

All506 original file bytes rehash unchanged after controls. Source hashes also remain unchanged. Full receipts, exact child code/argv, temporary SQLite snapshot and recovery decisions are preserved. This establishes the exercised envelope's harmless behavior; it does not approve the still-failing production metadata checks, full preflight, admission or native result.
