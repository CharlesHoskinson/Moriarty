# Independent snapshot12 controls

All14 revised source controls pass. Missing hash, real/effective UID/GID, exact OS identity pair, fixed root, extra mode bits and unexamined compiled-directory entry defects now reject. Actual506-file memfd seals, temporal byte preservation, hash rejection and FD cleanup pass.

The metadata-race control uses actual read_bytes and verify_snapshot: after original bytes are read, the fixture is replaced; after verification returns, it is replaced again. Exactly one manifest read occurs, and sealing receives the identical verified object with all506 original entries. The injected final argv never includes the unverified /etc/passwd entry and retains the original diagnostic suffix. Correct continuation using verified original metadata is accepted. Other preflight layers are isolated only in this control; they are not claimed tested there.

The actual full506-file private bwrap envelope was then repeated because source changed. All six existing-runner/temporary-store cases and30 identity rejections pass under strong Foreman. The harmless Python child checks snapshot bytes/modes, real/effective UID/GID, HOME/Path.home/getpwuid, cwd,8MiB stack, read-only mounts and no leaked snapshotFDs. InvalidUTF8/124/overflow reject recovery; detached readiness is seen and no matching host process survives. Exact plans, packets, stored receipts, recovered accepted streams and isolated SQLite are retained in snapshot-envelope-controls-12.

Source hashes and all506 original file hashes remain unchanged. Prior11 red evidence is intact. No canonical edits, K/Java/strace invocation, source-audit approval or dispatch/admission claim.
