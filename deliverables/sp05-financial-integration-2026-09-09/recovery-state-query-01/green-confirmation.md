# Unchanged reproducer after source repair

The same `exact-block-preflight-fixture.mjs` used for the red run completed against the repaired production source with `PUBLIC_STATE_VERIFIED` in 787.668079 ms. There were 25 loopback HTTP requests, no trapped private-path accesses, no stderr, and the server closed before process exit 0. The command remained bounded by `timeout --signal=TERM --kill-after=1s 10s`.

`exact-block-preflight-green.json` retains actual SDK requests, including the latest-state query with null offset and the additional current tip/finality coverage checks. No fixture changes were made between red and green. The red evidence remains unchanged. This is synthetic transport with real retained public native/build inputs, not a new actual-node observation or financial acceptance.
