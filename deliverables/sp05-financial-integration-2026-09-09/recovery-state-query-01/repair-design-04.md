# Recovery latest-state query correction

The actual read-only local diagnostic failed with RECOVERY_STATE_TYPE before private access. The exact historical query response was not retained. Captured indexer source and a faithful actual-SDK fixture establish a concrete matching failure mechanism: an explicit block filter selects an action in that block, rather than state as of that block. A later empty block therefore returns null.

Read the deployment state using its actual deployment block. Read current state using the latest-action query, bracketed by two fresh exact indexed/finalized tip observations and a stable node finalized head. Require full serialized native state equality to the retained constructor for both states. Keep all original identity, history, canonicality, deadline and private restoration gates.

This relies on the trusted local indexer and RPC reporting coherent observations; it is not a cryptographic state inclusion proof or a timeless guarantee. No asynchronous observation proves that state cannot change afterward. The fixed retained contract identity, full-state check and existing call authority remain required. Latest lookup alone is insufficient without the coverage and head checks.

Three added regression cases failed on the old source and pass after the repair: later empty block, indexer behind finality, and coverage changing during observation. All 291 ledger tests pass with no skips (2936.84957 ms). The unchanged actual-SDK/full-preflight loopback fixture went from RECOVERY_STATE_TYPE to PUBLIC_STATE_VERIFIED, with zero trapped private accesses. Transport is synthetic; no financial or network acceptance follows from this result.

No build, deployment, wallet identity, original store, original password or historical charge is changed. Source review and the successor resource decision remain separate scopes even when reviewers receive one packet.
