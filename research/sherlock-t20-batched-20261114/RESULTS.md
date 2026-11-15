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
