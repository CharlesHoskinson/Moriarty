# T20: batched Midnight writes

Row T20 of `research/sherlock-tested-design-20261113/LOOP.md`. Owner rule (2026-11-14): no design writes to Midnight per event; a design commits per checkpoint, epoch or batch with one root or a few fixed fields, keeps item data off chain and checks it against the root. Inputs: the audit `~/Projects/mpcexperiments/design/enhancement/midnight-writes-audit/sherlock.md` (branch `t10q` at `f8fd268e3`) and the wiki page `llm-wiki/wiki/syntheses/MidnightWritesAudit.md`.

Branch `t20-batched` from `t10q` `f8fd268e3`. Labels: MEASURED (run here or in a cited record), DERIVED (arithmetic on MEASURED values), ESTIMATE.

## Fixed figures used in every table

| Item | Value | Label and source |
|---|---|---|
| `acceptRun` transaction | 6,783 bytes, 40 transcript words, 42 lanes | MEASURED, devnet block 22,728 (`~/.cache/t10mm/devnet/t10a-state.json`); lanes MEASURED on the compiled contract (this row, `lanes.py`) |
| Wrap proof | 2,912 bytes, 1,026 public inputs | MEASURED (T10m RESULTS) |
| Warm call proving time, one wrap per ledger call | 175.5 s on huginn | DERIVED from MEASURED stages (LOOP lead decision O3) |
| Ledger calls one prover can feed | 86,400 / 175.5 = 492 per day | DERIVED |
| Preview transaction byte limit | 1,048,576 | MEASURED (T17 RUNBOOK, height 113,894) |

## Fix 5: LDG-003 to LDG-005 aligned with ARCHITECTURE (done first)

The OpenSpec spec still had `acceptCollection(reg_root, set_root, shape_d)` (LDG-003) and a `descend` entry with a `K_open` bound (LDG-004), against ARCHITECTURE section 1.11 FP-3 fact 3 ("`descend` is removed"), owner ruling O9 (no `shape_d`) and owner ruling FP3-3 (a). The spec now has: LDG-003 `acceptCollection(reg_pub, set_pub)` with no `shape_d`; LDG-004 no `descend`, depth inside the opening proof, no `K_open`; LDG-005 `openClaim(set_pub, C_P, input_pub, outcome_pub)` with the FOREIGN form as in ARCHITECTURE. The same stale `descend` text was fixed in ARCHITECTURE (freeze table, kit list), FINAL-DESIGN (kit table), tasks F.10 and the freeze-prerequisites fact 2 scenario. Fix 2 then replaces `openClaim` (below).

Writes per claim of depth `d` opened from an accepted collection:

| | Ledger calls per claim | Wrap proofs per claim | Label |
|---|---|---|---|
| Before (stale spec, `descend` with `K_open`) | `ceil(d / K_open) + 1` | `ceil(d / K_open) + 1` | DERIVED from the spec text |
| After this fix (ARCHITECTURE form) | 1 | 1 | DERIVED |
| After fix 2 | 0 (see fix 2) | 0 | DERIVED |

Quint: `models/sherlock/quint/t20-batched/setOpening.qnt` (SET grammar of any shape, opening by proof). Invariants `ClaimSettledOnce`, `OnlyClaimHeadsSettle`, `OpenedOnlyFromVerified`, `OneCallAnyDepth`, `OpenedAreMembers`. Mutants `descend_k_open`, `head_any_form`, `no_replay_check`, `unverified_root`. Non-vacuity witnesses: an open at depth 3 or more, a foreign open, a diamond. Result (`run-setOpening.out`, quint 0.32.0, 3,000 samples of 14 steps, seed 7): 6 positive and negative tests pass, the design holds `Inv`, every mutant violates `Inv` (and its named attack test passes), every witness is reached. The same opening relation is what a Sherlock consumer discharges in circuit (GP-2 DESCEND, DL-STEP) after fix 2.

## Fix 1: operator-settled jobs use one `acceptCollection` per epoch

Rule W-1 in ARCHITECTURE kit LEDGER CONTRACT, LDG-002 amended and LDG-006 added. `acceptRun` stays for one run whose own user submits and pays. The contract is unchanged apart from a comment. Epoch close (LEAD pick, for T15 confirmation): the earlier of 3,600 s after the epoch's first job and the claims-tree capacity of one collection (fix 2).

Per-entry sizes on the compiled contract (`tools/sherlock-sdk/t20-measure.mjs`, compactc 0.36.0 `--skip-zk`, compact-runtime 0.21.0, ledger-v9; MEASURED here; the lane counts equal the T9 figures):

| Entry | Lanes | Transcript words | Proof-preimage bytes | Contract state growth |
|---|---|---|---|---|
| `acceptRun` | 42 | 40 | 512 | 267 bytes |
| `acceptCollection(reg_pub, set_pub)` | 48 | 46 | 526 | 180 bytes |
| `openClaim` | 57 | 55 | 626 | 345 bytes |

Transaction bytes of an entry other than `acceptRun` are ESTIMATE: 6,783 (MEASURED `acceptRun`) plus the preimage difference, so `acceptCollection` about 6,797 and `openClaim` about 6,897. The wrap proof (2,912 bytes) is the same in every entry.

Writes for `N` operator jobs per epoch of length `T` seconds, at job rate `lambda` per second:

| | Transactions | Bytes on chain | Wrap proofs | State growth | Label |
|---|---|---|---|---|---|
| Before, per request (job) | 1 | 6,783 | 1 (175.5 s) | 267 bytes, kept forever | MEASURED (tx, state), DERIVED (time) |
| Before, per epoch of `N` jobs | `N` | 6,783 `N` | `N` | 267 `N` | DERIVED |
| Before, per second | `lambda` | 6,783 `lambda` | `lambda` | 267 `lambda` | DERIVED |
| After, per request | `1 / N` | about 6,797 / `N` | `1 / N` wrap plus one JOIN node | 180 / `N` | ESTIMATE |
| After, per epoch | 1 (0 when no job) | about 6,797 | 1 (plus `N - 1` JOIN nodes, STARK only) | 180 bytes | ESTIMATE |
| After, per second | `1 / T` (at most 1 / 3,600 = 0.00028) | about 6,797 / `T` (at most 1.9 bytes per second) | `1 / T` | 180 / `T` | DERIVED |

Example (DERIVED): one prover at its ceiling of 492 jobs per day writes 492 transactions and 3.34 MB per day before; with a 3,600 s epoch it writes at most 24 transactions and about 163 KB per day after.

Quint: `epochSettle.qnt` (`run-epochSettle.out`): 3 design tests pass; `Inv` (`OperatorWritesPerEpochAtMostOne`, `LedgerWritesBounded`, `JobsSettledAtClose`, `NoJobInRuns`) holds on 3,000 samples of 14 steps; the mutants `per_job_accept_run`, `collection_drops_job` and `split_epoch` each break `Inv` and their attack tests pass; the batching witness (one collection of at least three jobs beside a user run) is reached.

## Fix 2: `openClaim` replaced by membership against the collection root

Rule W-2 in ARCHITECTURE kit LEDGER CONTRACT; LDG-001, LDG-003, LDG-004, LDG-005, LW-009 and DCD-004 amended. Contract `tools/sherlock-sdk/sherlock-ledger.compact` (batched form): `openClaim` removed, `verified_roots` replaced by `claim_roots` (one `claims_root` per epoch), read entries `rootAccepted` and `runAccepted` added. The T10 contract that T10 and T17 measured is kept byte-equal as `sherlock-ledger-t10.compact` (sha256 `4f002887...`, the source T17 builds), and `sherlock-ledger-build.sh` and the T10 tests use it, so the T17 Preview path is untouched.

Picks (LEAD, for T15 confirmation):
- Collection statement `[reg_pub, claims_root]` in place of `[reg_pub, set_pub]` (I-24, LW-009, DCD-004); `set_root` stays private. The compiled `acceptCollection` is byte-equal to the T10 form up to the argument name (48 lanes, `claims_root` at 29 and 43, MEASURED), and `acceptRun.zkir` and its compiled verifier key are byte-equal to the T10 build (MEASURED).
- Claims tree: fixed depth `D_C` = 20, run leaf `claim_pub`, node `H2(l, r)` (the `PATH` form of the deferred log; it fixes the placeholder in `iface.rs`), empty slot 0.
- Read entries deployed with their own compiled keys (native proofs), not the wrap key. The runtime refuses to bind a callee whose deployed key differs from the compiled one (test `W-2 read entry key`); on chain a wrap-keyed read entry would cost a 175.5 s wrap proof per consumer use.
- The at-most-once rule for collection claims (I-21, FROZEN under FP3-3 (a)) moves to the consumer's nullifier keyed by `claim_pub` alone.

Circuit delta (no code on this branch; it belongs to T8b "L over SET", which is not built): the `L` collection form logs the run entries of check (i) for each member head, `H2` entries for the tree (about `2N + 20` for `N` members) and two `PUB` entries for lanes 29 and 43. The wrap is unchanged: the E5 decoder already replays `H2`, `SPONGE`, `PATH` and binds a lane to a logged value with `PUB`. Keys: the T10q key set does not change for T20; the `L` key changes when T8b builds the collection form, as it would without T20. The `openClaim` opening form of `L` (`R_SET`, 5 lane pairs) becomes unused; it can stay until the T12 freeze. Log capacity, ESTIMATE: about 14 members per collection fit in `V = 256` values; the rest go to tier E with no bound (GP-1, GP-4).

Consumer (`tools/sherlock-sdk/sherlock-claim-consumer.compact`, MEASURED from its ZKIR): `useRunClaim` 64 lanes, 23 `transientHash` (claim, 20 path nodes, nullifier); `rootAccepted` 17 lanes, preimage 195 bytes, no state change; `runAccepted` 30 lanes, 403 bytes, no state change.

Writes for a collection of `N` claims, each used by one consumer:

| | SherlockLedger transactions | Bytes on chain | Wrap proofs | SherlockLedger state growth | Label |
|---|---|---|---|---|---|
| Before, per claim used | 1 `openClaim` | about 6,897 | 1 (175.5 s) | 345 bytes | ESTIMATE (bytes), MEASURED (state, runtime) |
| Before, per collection | `1 + N` | about 6,797 + 6,897 `N` | `1 + N` | 180 + 345 `N` | DERIVED |
| Before, per second | `1/T + lambda_use` | about 6,897 `lambda_use` + 6,797 / `T` | same as transactions | 345 `lambda_use` | DERIVED |
| After, per claim used | 0 | 0 on SherlockLedger; the consumer's own call adds one native `rootAccepted` read and 20 hashes in its own proof | 0 | 0 | MEASURED (lanes, no state change), ESTIMATE (native proof bytes) |
| After, per collection | 1 | about 6,797 | 1 | 180 bytes | ESTIMATE |
| After, per second | `1 / T` | about 6,797 / `T` | `1 / T` | 180 / `T` | DERIVED |

The consumer's own nullifier insert (one set entry in the consumer's state) is part of the consumer's call, which its user makes and pays for anyway: one write per user call, the OK class of the audit.

Data availability and epoch wait: see W-2 in ARCHITECTURE. Each job's submitter gets its slot and path with its receipt; the operator publishes the epoch's leaf list (32 bytes per claim) off chain; a withheld list delays use and never admits a non-member. Wait: at most the epoch (3,600 s) plus the collection proof (175.5 s warm for `L` to wrap, DERIVED, plus JOIN nodes) plus inclusion.

Tests (`t20-ledger-test.out`, compiled contracts, compact-runtime 0.21.0): 7 pass: entries (no `openClaim`), collection write and refusals (replay, other registry), consumer use once with the ledger unchanged, refusals (unaccepted root, forged outcome, another slot, flipped index bit), one use across two roots and `runs`, two controls that accept the defect (root-keyed nullifier, no root read), read-entry key binding.

Quint `claimsRoot.qnt` (`run-claimsRoot.out`, 4,000 samples of 16 steps): 8 design tests pass; `Inv` (`LedgerWritesBounded`, `UsedOnlyVerified`, `UsedOncePerConsumer`, `UseAfterEpochClose`) holds; mutants `per_claim_open`, `root_keyed_nullifier`, `no_root_check`, `unbound_root` each break it; witnesses reached: one claim used by two consumers, one claim in two roots, a withheld list. The model already carries the leaf kind for fix 3.
