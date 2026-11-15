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

## Fix 3: the `foreign` map and the FOREIGN opening (LDG-005) by membership

Rule W-3 in ARCHITECTURE; LDG-007 added. The `foreign: Map<fclaim_d, ()>` state and the FOREIGN opening were design text only (never in a compiled contract); both are removed. A foreign claim stays a `FOREIGN(fclaim_d)` member of `set_root`; its claims-tree leaf is `transientHash([SHK_CLAIM_FGN, fclaim_pub])` with `fclaim_pub = PoseidonFr_4(SHK_FCLAIM, pack(fclaim_d))` (a seventh `DIGEST` tag; LEAD pick for T15, it fixes the public form the GP-3 hook left open). The example consumer gains `useForeignClaim(fclaim_pub)` (64 lanes, MEASURED from its ZKIR), with the same path check, root read and nullifier.

Circuit delta (with the GP-3 foreign lift and T8b, neither built): `L` logs `DIGEST(SHK_FCLAIM)` and one `H2` per foreign member. The new tag changes the decoder's `DIGEST` tag bound, a wrap key constant; it rides the T8b rekey that the `L` collection form needs anyway. No key change for T20 on its own.

| | Transactions | Bytes on chain | Proofs | State growth | Label |
|---|---|---|---|---|---|
| Before, per foreign claim opened | 1 FOREIGN opening | about 6,897 (an `openClaim`-sized call) | 1 wrap | one `foreign` entry | ESTIMATE (design text, no compiled entry) |
| Before, per collection of `F` foreign claims | `1 + F` | about 6,797 + 6,897 `F` | `1 + F` | `F` entries | ESTIMATE |
| Before, per second | `1/T + lambda_fgn` | | | | DERIVED |
| After, per foreign claim used | 0 on SherlockLedger | 0; the consumer's own call adds one `rootAccepted` read | 0 | 0 | MEASURED (lanes, no state change) |
| After, per collection | 1 (shared with the run claims of the epoch) | about 6,797 | 1 | 180 bytes | ESTIMATE |
| After, per second | `1 / T` | about 6,797 / `T` | `1 / T` | 180 / `T` | DERIVED |

Tests (`t20-ledger-test.out`): 9 pass, the two new ones: `W-3 foreign claim` (used once, no foreign state, run members still usable) and `W-3 kind separation` (a run leaf offered as a foreign claim is refused; the control `consumer-control-no-kind-tag` accepts it).

Quint `claimsRoot.qnt` extension (`run-claimsRoot-fix3.out`): 4 foreign tests pass (use once, twice refused, run as foreign refused, foreign as run refused); `Inv` holds; mutant `no_kind_separation` breaks `UsedOnlyVerified`; witness "a foreign claim is used" reached.

## Fix 4: `L_TX` covers every measured entry; chunks are a capped fallback

Rule W-5 in ARCHITECTURE (kit COMPACT FRONTEND call vector and row budget, kit LEDGER CONTRACT write rule); ANC-012 added to `sherlock-anchor-chunks`. `L_TX` stays 1,024, so no key changes (the wrap's 1,026 public inputs are a key constant that T10q is rekeying now).

Sizes (MEASURED here over the 105 ZKIR circuits of `mycroft/tests/compact-suite/native` on branch `mycroft`, and its checked transcripts): largest checked call 474 transcript words, 476 lanes (`d04-coracle` `withdraw`, vector `11-red-withdraw`); largest static impact count 699 words (`a07-historic-merkletree` `test`, which has no passing call). The audit's "largest `L_e` 699 of 1,015" (Mycroft a3) is this static count; 476 lanes is the measured one. With the 9-lane anchor prefix one call carries 1,015 effect words: 474 uses 47 %, 699 uses 69 %. No measured or static entry needs a chunk.

Cap `K_MAX` = 2 at the default 3,600 s intent TTL (LEAD pick, for T15 confirmation), the least of three limits, DERIVED:

| Limit | Rule | Chunks | Label |
|---|---|---|---|
| Intent TTL | TTL at least 10 times proving (kit COMPACT FRONTEND); `k` chunks prove in about `k` x 175.5 s warm | `floor(3,600 / 1,755)` = 2 | DERIVED |
| Block compute | 2 s per block (Preview, MEASURED) over 0.067 to 0.107 s per wrap verification (`ledger-check`, MEASURED off chain) | about 18 | ESTIMATE |
| Transaction bytes | 1,048,576 (Preview, MEASURED) over about 39 KB per full chunk (6,783 bytes plus about 33 bytes for each of 990 effect words) | about 26 | ESTIMATE |

A plan that sets a longer TTL may raise the cap to `min(floor(TTL / 1,755), 18)`. Above the cap `sherlock plan` refuses the entry with its name, `L_e` and `K_MAX`; below it, it prints `k` and the fee before proving. General Sherlock runs write the fixed 3-word run statement and never chunk, so the cap does not narrow GP-1 to GP-4. The chunk planner itself is not built (ANC-010 tests T3 are open); the rule is a planner requirement.

Writes per logical call of `L_e` effect words (each call is the user's own transaction; per second is the user's call rate `lambda_call`):

| | Calls (wrap proofs) per logical call | Anchor overwrites | Bytes | Label |
|---|---|---|---|---|
| Before, measured entry (`L_e` <= 699) | 1 | 0 | 6,783 for `acceptRun`-sized calls | MEASURED |
| Before, oversized entry | `ceil(L_e / 990)`, no cap (5 to 30 at the code defaults, audit) | `k` | about 39 KB `k`, up to about 1.18 MB at `k` = 30 (over the transaction limit) | ESTIMATE |
| After, measured entry | 1 | 0 | unchanged | MEASURED |
| After, 1,016 to 1,980 words | 2 | 2 | about 79 KB | ESTIMATE |
| After, above 1,980 words (at TTL 3,600 s) | 0: refused at plan time | 0 | 0 | DERIVED |
| Per second, before and after | `k` x `lambda_call` with `k` <= 2 after | | | DERIVED |

Quint `chunkCap.qnt` (`run-chunkCap.out`): 3 design tests pass (474, 699, 1,015 in one call with no anchor write; 1,016 and 1,980 in 2 chunks; 1,981 and 5,000 refused); `Inv` (`MeasuredEntriesOneCall`, `ChunksAtMostCap`, `NoSilentTruncation`, `TtlCoversProving`, `RefusedAtPlanTime`) holds; mutants `no_cap`, `small_ltx` (the 256-word assumption), `cap_truncate`, `cap_four` each break it; witnesses (a 2-chunk plan, a refused plan) reached.

## Fix 6: one `reg_root` update per registry epoch

Rule W-4 in ARCHITECTURE (kit LEDGER CONTRACT write rule and state list); LDG-008 added. Design rule only: the contract keeps `reg_root` sealed (no update entry) until N-8 decides the history and revocation rule; T20 adds no entry and changes no key. The strict reading (only the current root is accepted) is what the contract does today; a one-epoch grace window for the previous root stays an N-8 option.

Writes for `K` key changes (registrations and revocations) in a registry epoch of length `T_reg`:

| | Transactions | Bytes on chain | State growth | Label |
|---|---|---|---|---|
| Before (a per-key update, the form N-8 left open) | `K` | about 6,783 `K` | `K` root overwrites | ESTIMATE |
| After, per registry epoch with a change | 1 | about 6,783 | one root overwrite (32 bytes) | ESTIMATE |
| After, registry epoch with no change | 0 | 0 | 0 | DERIVED |
| After, per key | 0; the key is admitted by a membership path in circuit | 0 | 0 | DERIVED |
| After, per second | at most `1 / T_reg` | at most 6,783 / `T_reg` | | DERIVED |

Quint `registryEpoch.qnt` (`run-registryEpoch.out`): 4 design tests pass (three key changes in one write and the new key used; a collection under the replaced root refused; an unregistered key refused; an epoch with no change writes nothing); `Inv` (`RegWritesPerEpochAtMostOne`, `StaleOrForeignRegRootRefused`, `KeysUsedAreRegistered`) holds; mutants `per_key_write`, `no_reg_check`, `unbound_membership` each break it; witnesses (batched update, new key used) reached.
