# Superseded contracts

These two contracts are superseded for new designs by the batched SherlockLedger (`sherlock-ledger.compact`, row T20, Midnight write rule W-1 to W-5 in `research/sherlock-final-design-20261111/ARCHITECTURE.md`). They stay in the tree byte-equal, because deploy, call and readback scripts and the sealed holdout cases pin their SHA-256; a header comment would break those pins.

| File | What it was | Why superseded |
|---|---|---|
| `sherlock-anchor.compact` | Fenrir anchor of the Sherlock linked parent: `acceptValidity` then `settleValidity` per decider-proved parent | Two writes per parent, and `validity_ok` and `settled` grow by one statement per parent; W-1 settles operator jobs by one `acceptCollection` per epoch |
| `rollup-anchor.compact` | S0 rollup anchor for `sherlock-v3-batch/1`, synthetic observations | The same two writes per batch and growing statement sets; superseded by the same rule |
| `sherlock-ledger-t10.compact` | The T10 SherlockLedger with `openClaim` (one write per claim used) | Kept as the source of the T17 Preview deployment; W-2 replaces `openClaim` by membership against `claims_root` |

New deployments use `sherlock-ledger.compact`. Existing deployments of the files above keep working; nothing reads them for new work.
