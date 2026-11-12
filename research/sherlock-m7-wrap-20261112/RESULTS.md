# M7 results

Plan: `PLAN.md` in this directory. Date: 2026-11-11 (night). Branch `m7-wrap`.

**Verdict: M7 stops at M7-0. The G4 wrap cannot be verified by the unchanged ledger 9, and the one shape the unchanged ledger reads does not fit k = 21 at the OD-19 set.** PROPOSAL section 9 says "ledger 9.1.0.1 needs no change" for the wrap. That holds only for a circuit whose constraint system is a `ZkStdLib` architecture, and G4 (4 lanes of custom gates) is not one. The lead or the owner must pick one of the options in section 2 before M7c to M7e mean anything. M7a and M7b (the G4 circuit itself) are needed under options A and C, so they can go on once the option is chosen.

## 1. Findings

### 1.1 The ledger rebuilds the gates from architecture flags (CODE, then MEASURED by the toy)

- `VerifierKey::verify` calls `force_init`, which reads the key bytes with `MidnightVK::read(.., SerdeFormat::Processed)` and refuses trailing bytes (`repos/midnightntwrk/midnight-ledger/transient-crypto/src/proofs.rs:521-543`, ledger-9 `f15fb9d`; ledger-8 is the same at `:544`).
- `MidnightVK::read` reads a `ZkStdLibArch` (13 chip flags and `nr_pow2range_cols`), `k` and the public-input count, then builds the constraint system with `ZkStdLib::configure(&mut cs, (architecture, k - 1))` and reads only the commitments into it with `VerifyingKey::read_from_cs` (`midnight-zk-stdlib-2.4.0/src/lib.rs:1573-1587`).
- So the key bytes name chips, not gates. Any circuit whose gates, columns or lookups differ from `ZkStdLib::configure` of some flag set cannot be read as itself. The ledger's own comment says so: "Longer-term, we'll need to store information about the circuit architecture in the verifier key ... those API endpoints do not currently exist in midnight-circuits" (`proofs.rs`, `DummyRelation`).
- `ZkStdLibArch` has no lane option. Advice columns are the maximum over the enabled chips (`NativeChip` 5 columns, Poseidon 8), not 4 lanes of 8; fixed columns are shared per chip, not per lane (`lib.rs:492-546`).
- Toy, MEASURED: see section 1.3.

### 1.2 The stdlib shape does not fit k = 21 at the OD-19 set (ESTIMATE)

Single lane, stdlib 19-row Poseidon (shape 1 of `wrap_model.py`, the only wrap shape a `ZkStdLibArch` can express), `L_cached`, log blowup 3, queries 98/83/82, Merkle depth `20 + b - 4 - r`:

| `w_stack` | Low | Mid | High | G4 mid (same model) |
|---|---:|---:|---:|---:|
| scenario 16/18/18 | 2,370,944 | 2,620,323 | 2,724,845 | 639,486 |
| 36 | 2,607,307 | 2,830,276 | 2,934,798 | 697,659 |
| 48 | 2,746,616 | 2,947,241 | 3,051,763 | 728,763 |

The k = 21 gate is 1,782,573 rows. The low case at the smallest `w_stack` is 133 percent of it. Even at the old list-decoding queries 57/28/19 this shape was 1,836,675 mid, over the gate (`L-STRUCTURE.md` section 4). No query or `w_stack` choice of the OD-19 family brings it under.

Command: `python3 -I s1ud.py` (scratch script; it imports `research/sherlock-stark-framework-20261111/wrap_model.py`, sets `LOG_BLOWUP = 3`, `DEPTH`, `QUERIES = 98,83,82` as `~/.cache/ud-sizing/wrap_ud.py` does, runs `measured_scen(L-STRUCTURE.json, "L_cached")` and prints `variants()[case]["s1_19"]`). The G4 column reproduces `wrap_ud.out` exactly.

### 1.3 Toy (MEASURED)

Binary `tools/gpu-prover/src/bin/m7_ledger_toy.rs` (workspace `tools/gpu-prover`, ledger `transient-crypto` at `a2b30128`). One lane of 8 advice columns, one degree-5 gate (`a^5 = b`), one lookup into a 256-row fixed table, a committed instance column plus 1,026 public inputs, k = 11, ceremony SRS `bls_midnight_2p11`, blinded proof with `OsRng`, Blake2b transcript. Output `~/.cache/m7-wrap/toy-run.json`:

| Check | Result |
|---|---|
| Library verify (`prepare` + `DualMSM::check`, true key) | accepts |
| Proof bytes / raw key bytes | 1,536 / 198 |
| Ledger `VerifierKey::verify`, header `poseidon`, `nr_pow2range_cols` 4 | `Err(problem reading the verifier key)` |
| Same, header `default` | `Err(problem reading the verifier key)` |
| Same, header `poseidon`, `nr_pow2range_cols` 1 | `Err(problem reading the verifier key)` |

The proof is valid, and the ledger refuses it at key parsing: the stdlib constraint system that `MidnightVK::read` rebuilds does not match the key's commitments. A G4 key has the same defect at any size, because the G4 gates are not a `ZkStdLibArch` flag set.

## 2. Options for the lead (recommended option marked)

| Option | Ledger | Wrap | Cost | Status |
|---|---|---|---|---|
| A. Add a G4 architecture to `midnight-zk-stdlib` (a new `ZkStdLibArch` flag or a key format that carries the lane layout, `ZKSTD_VERSION` 3) and bump the ledger's dependency | changed (dependency and key format; Midnight upstream) | G4 at k = 20 (ESTIMATE), one proof | upstream work and a ledger release; the 50,000-byte key limit still applies | contradicts PROPOSAL section 9 "no change" |
| B. Wrap as a `ZkStdLib` circuit (single lane) | unchanged | 2.37 to 3.05 M rows | does not fit k = 21 (section 1.2) | **rejected** |
| **C. Two steps: G4 wrap of `L` off chain (custom gates, Poseidon transcript), then a stdlib decider (`ZkStdLibArch` with `bls12_381` and `poseidon`, the shape of today's run decider) that verifies the one G4 proof and exposes the 1,026 public inputs** | **unchanged** | G4 at k = 20 plus a decider at k <= 21 | one more PLONK layer: today's k = 21 decider proves in 68.1 s (k = 20) and 111.8 s (k = 21) on a quiet CPU (MEASURED, `M-k21.md:41`), 77 to 83 s with the GPU path (ESTIMATE, `research/sherlock-gpu-wrap-20261111/RESULTS.md` section 2); this breaks the 5 to 12 s wrap of PROPOSAL section 8 | **recommended for the near term** (lead decision, owner to confirm); A as the target, filed upstream |
| D. Fewer queries in `L` (list decoding) to make B fit | unchanged | B at 57/28/19 is 1,836,675 mid | over the gate; also reverses OD-19 | rejected |

Why C first: it is the only option that keeps the ledger unchanged and fits k <= 21, and it uses the decider path that already passed ledger 9, the local devnet and Preview (XC-9). The G4 wrap circuit is the same work under A and C, so M7a and M7b do not wait on the choice. Under C the on-chain key is the decider's (2,217 bytes today, MEASURED `X9:29`), and the G4 key bytes are off chain; under A the G4 key goes on chain.

## 3. Status per milestone

| Milestone | Status |
|---|---|
| M7-0 blocking checks | done: both checks hold (sections 1.1 to 1.3) |
| M7a gadget skeleton and row counter | not started (stopped at M7-0 for the option choice) |
| M7b full circuit on a real `L` proof | not started; needs a stark-backend Fr configuration (BLS12-381 Fr field for p3 0.4.3 and the Midnight Poseidon permutation), which does not exist in any checkout |
| M7c, M7d, M7e | blocked on the option choice |

## Log

- 2026-11-11 22:40. Worktree `m7-wrap` from `main` 4ea6fa29d; merged `l-structure` (3957ddf7c) and `gpu-wrap` (52ad95069) without conflict.
- 2026-11-11 22:50. Read `MidnightVK::read` and the ledger key path: the constraint system comes from `ZkStdLib::configure(arch, k - 1)`. CODE.
- 2026-11-11 22:55. Single-lane stdlib wrap at the OD-19 set: 2,620,323 mid rows, over the k = 21 gate. ESTIMATE.
- 2026-11-11 23:00. Toy `m7_ledger_toy` written; build under the heavy lock (target copy `~/.cache/m7-wrap/target-gp`), waited about 1 h on the lock.
- 2026-11-12 00:2x. Toy run: library verify accepts; ledger refuses under all three headers with `problem reading the verifier key`. MEASURED. M7 stops at M7-0.
