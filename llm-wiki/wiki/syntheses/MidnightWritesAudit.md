---
title: "Midnight writes audit"
type: synthesis
tags: [audit, ledger, midnight]
sources: [sherlock-architecture]
last_updated: 2026-11-14
---

# Midnight writes audit

Owner ruling of 2026-11-14: no design may write to Midnight per event. A design commits per checkpoint, epoch or batch (one Merkle root or a few fixed-size fields), keeps item data off chain and checks it against the root. This page lists the Sherlock sites that break the rule. The full audit, with every site, file and line, is `~/Projects/mpcexperiments/design/enhancement/midnight-writes-audit/sherlock.md`, read on branch `t10q` at `f8fd268e3`.

The design states no throughput (R-10 in [[sherlock-architecture]]). One ledger call needs one wrap proof of 175.5 s warm (DERIVED, LOOP lead decision O3), so one prover issues at most about 492 ledger calls per day (DERIVED). An `acceptRun` transaction is 6,783 bytes (MEASURED on the local devnet, run state file).

## PER-EVENT items

| Site | Writes per event | Proposed batched form (audit proposal, not a ruling) |
|---|---|---|
| `acceptRun` used for each co-processor job ([[AcceptRunContract]]) | 1 transaction, 1 proof, 1 permanent `runs` entry per run | settle jobs by one `acceptCollection` per epoch; keep `acceptRun` for a single run its own user submits |
| `openClaim` and the `foreign` opening ([[sherlock-architecture]] kit LEDGER CONTRACT) | 1 transaction, 1 wrap proof, 1 map insert per claim; a collection of N claims costs 1 + N transactions | consumers check membership against the collection root in their own proof (GP-2) or call (a Compact-checkable claims root in the collection statement), claim list off chain |
| Chunked calls (kit COMPACT FRONTEND) | `k` calls, `k` proofs, `k` anchor overwrites per logical call, `k` = 5 to 30 | size `L_TX` so one call covers every measured entry; keep chunks as a capped fallback |
| OpenSpec LDG-003 to LDG-005 still specify `descend` and `shape_d` | 1 call per `K_open` levels per claim | align the spec with ARCHITECTURE ("`descend` is removed"), then with the batched form above |

## Not per event

- `acceptCollection` is the batched form already: one `set_pub` per collection, no claim write. `L` over a SET root is still open ([[WorkstreamStatus]], row T8b), so only the per-run path is tested end to end.
- A Compact call proved by Sherlock replaces the native call one for one: OK.
- One contract deploy per program (N-11 pick): OK. `reg_root` updates wait on N-8: UNCLEAR.
- Leaves, nodes, `L`, `L_Fr`, continuations, checkpoints and GP-2 node modes write nothing on chain ([[MidnightLedger9]], [[ModularArchitecture]]).
- The stagenet anchors `sherlock-anchor.compact` (two transactions per two machine steps) and `rollup-anchor.compact` are legacy and superseded by the one `SherlockLedger` contract.
