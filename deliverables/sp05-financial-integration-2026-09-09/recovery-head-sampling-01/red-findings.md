# Finalized-head sampling race reproduction

Experiment observation: the actual public preflight wrapper, pinned indexer SDK and retained public native/build inputs fail with `RECOVERY_INDEXER_FINALITY` when synthetic finalized/indexed state advances immediately after the exact indexed-tip reader returns. The failure took 781.828264 ms; the subprocess exited 1 within the ten-second bound, stderr was empty, the private-path trap log was empty, and the ephemeral HTTP server closed.

`head-sampling-red.json` retains every public request and selected synthetic responses. Request 15 confirms canonical hash for indexed height 20323. At request 16, after that reader completes, the fixture advances finalized/indexed height once to 20324 and then remains stable. Request 16 returns the new finalized head; request 17 returns its header. The subsequent comparison with the older sampled tip fails before the latest-state query.

The fixture is `head-sampling-preflight-fixture.mjs`, SHA256 `a5b79e786e260d0c015fd0daa1097d68af13ee53e7189bf42d764a7f2e39be81`. Preserve these exact bytes for the green run after repair. The mechanism matches the reported actual error code but does not reconstruct or capture the actual recovery02 response history. No live blockchain services, wallets, proof server or real private files were used; no production source was changed.

Command: `timeout --signal=TERM --kill-after=1s 10s node deliverables/sp05-financial-integration-2026-09-09/recovery-head-sampling-01/head-sampling-preflight-fixture.mjs`.
