# M7 results

Plan: `PLAN.md` in this directory. Date: 2026-11-11 (night). Branch `m7-wrap`.

**Verdict: M7 stops at M7-0. The G4 wrap cannot be verified by the unchanged ledger 9, and the single-lane proxy for the one shape the unchanged ledger reads does not fit k = 21 at the OD-19 set.** PROPOSAL section 9 says "ledger 9.1.0.1 needs no change" for the wrap ("Verifier unchanged"). That holds only for a circuit whose constraint system is a `ZkStdLib` architecture, and G4 (4 lanes of custom gates) is not one. Section 9 needs revision. No on-chain path is chosen here: M7c to M7e are paused until the lead or the owner picks an option of section 2 (lead decision, owner to confirm). Whether G4 is built at all depends on that choice (option E may remove it). The work that holds under every option is the stark-backend Fr configuration and the real `L` proof (M7b, first half).

## 1. Findings

### 1.1 The ledger rebuilds the gates from architecture flags (CODE, then MEASURED by the toy)

- `VerifierKey::verify` calls `force_init`, which reads the key bytes with `MidnightVK::read(.., SerdeFormat::Processed)` and refuses trailing bytes (`repos/midnightntwrk/midnight-ledger/transient-crypto/src/proofs.rs:521-543`, ledger-9 `f15fb9d`; ledger-8 is the same at `:544`).
- `MidnightVK::read` reads a `ZkStdLibArch` (13 chip flags and `nr_pow2range_cols`), `k` and the public-input count, then builds the constraint system with `ZkStdLib::configure(&mut cs, (architecture, k - 1))` and reads only the commitments into it with `VerifyingKey::read_from_cs` (`midnight-zk-stdlib-2.4.0/src/lib.rs:1573-1587`).
- So the key bytes name chips, not gates. Any circuit whose gates, columns or lookups differ from `ZkStdLib::configure` of some flag set cannot be read as itself. The ledger's own comment says so: "Longer-term, we'll need to store information about the circuit architecture in the verifier key ... those API endpoints do not currently exist in midnight-circuits" (`proofs.rs`, `DummyRelation`).
- `ZkStdLibArch` has no lane option. Advice columns are the maximum over the enabled chips (`NativeChip` 5 columns, Poseidon 8), not 4 lanes of 8; fixed columns are shared per chip, not per lane (`lib.rs:492-546`).
- Toy, MEASURED: see section 1.3.

### 1.2 The stdlib shape does not fit k = 21 at the OD-19 set (ESTIMATE)

Single lane with the stdlib 19-row Poseidon (shape 1 of `wrap_model.py`). Shape 1 is the decider-style model: `NativeChip` with 5 advice columns, pow2range on 4 columns (`wrap_model.py:248-253`), so it is a stdlib-expressible shape; its per-primitive row costs are ESTIMATE (only the Poseidon rows are derived from the chip). Rows, `L_cached`, log blowup 3, queries 98/83/82, Merkle depth `20 + b - 4 - r`:

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

The toy binary has no accepting ledger case of its own. The accepting control at the same ledger pin `a2b30128` is the run decider (a stdlib-architecture key), which `VerifierKey::verify` accepts (`research/sherlock-gpu-wrap-20261111/RESULTS.md` section 1). The refusal is MEASURED; its cause is CODE (section 1.1). The proof is valid, and the ledger refuses it at key parsing: the stdlib constraint system that `MidnightVK::read` rebuilds does not match the key's commitments. A G4 key has the same defect at any size, because the G4 gates are not a `ZkStdLibArch` flag set.

## 2. Options for the lead (none chosen)

| Option | Ledger | Wrap | Cost and flags | Status |
|---|---|---|---|---|
| A. Add a G4 architecture to `midnight-zk-stdlib` (a new `ZkStdLibArch` flag or a key format that carries the lane layout, `ZKSTD_VERSION` 3) and bump the ledger's dependency | changed (dependency and key format; Midnight upstream) | G4 at k = 20 (ESTIMATE), one proof | upstream work and a ledger release; the 50,000-byte key limit still applies | open |
| B. Wrap as a `ZkStdLib` circuit (single lane) | unchanged | 2.37 M to 3.05 M rows on the proxy (section 1.2) | over the k = 21 gate (ESTIMATE) | rejected on the proxy |
| C. Two steps: G4 wrap of `L` off chain (custom gates, Poseidon transcript), then a stdlib decider (`bls12_381` and `poseidon` flags) that verifies the G4 proof and exposes the 1,026 public inputs | unchanged | G4 at k = 20 plus a decider | decider size and k NOT CHECKED (today's 929,170-row decider verifies stdlib-shaped proofs with 70 public inputs, not a G4 proof with 1,026); two BLS12-381 PLONK layers, against the owner rule of one BLS12-381 wrap; today's decider proves in 68.1 to 111.8 s on a quiet CPU (MEASURED, `M-k21.md:41`), against the 5 to 12 s wrap of PROPOSAL section 8 | open, three flags |
| D. Fewer queries in `L` (list decoding) to make B fit | unchanged | B at 57/28/19 is 1,836,675 mid | over the gate; also reverses OD-19 | rejected |
| E. The Fr-terminal layer between `L` and the wrap of PROPOSAL section 8 (owner item OD-11, form A): a STARK over Fr shrinks what the final wrap verifies, so a stdlib-architecture wrap may fit k <= 21 and G4 may not be needed | unchanged | stdlib-shaped wrap of a smaller proof | rows NOT CHECKED; adds a prover layer | open |

Under A the G4 key goes on chain and M7c to M7e run as planned. Under C the on-chain key is the decider's. Under E the G4 circuit is not built.

## 2b. Ledger-compatible wrap paths (coordinator request, 2026-11-12)

Constraints: OD-19 set, k <= 21, ZK, one universal key set, ledger 9.1.0.1 unchanged, key at most 50,000 bytes, one BLS12-381 layer. All rows ESTIMATE (`wrap_model.py` shape 1 with the 19-row stdlib Poseidon, OD-19 set, `L_cached`, the log at the OD-6 capacity 256/256/0 unless stated). Script: `split.py` beside `s1ud.py` (same patching as `~/.cache/ud-sizing/wrap_ud.py`).

**Where the single-lane rows go** (mid, scenario `w_stack`, log 1,024/1,024/16, total 2,620,323): EF products (32 rows each) 1.02 M; base products 0.44 M; Poseidon 22,161 permutations x 19 = 0.42 M; by line: final-polynomial Horner at 263 queries 552,415; stacked reduction 423,476; round-0 Merkle 290,041; folds 203,562; constraint DAG 192,992; round-0 batching 137,984. WHIR query work (Merkle, folds, batching, Horner) is 1.41 M of it.

| Candidate | Rows (low / mid / high), per proof | k | Fits | Cost and flags |
|---|---|---|---|---|
| P1. One stdlib wrap | 2,264,340 / 2,481,719 / 2,586,241 (scenario `w_stack`); 2,808,637 mid at 48 | none | no: 127 to 163 percent of the k = 21 gate | rejected |
| P2. Lower-row stdlib gadgets in one wrap: base-field Horner for the final polynomial (query points are in the base field, so EF x base, not EF x EF), the stacked-reduction and Horner batching rewrites of `wrap-rows.md` 5a and 5b, Karatsuba EF products | needs a 1.4 to 1.65 times cut to reach 1,782,573; the named rewrites cut about 0.4 to 0.6 M (ESTIMATE: Horner about 40 percent of 552,415, 5a about 30 percent of 423,476, 5b about 15 percent of 108,048, Karatsuba 9 of 16 base products on part of 1.02 M) | 21 at best | marginal at mid, fails high and `w_stack` 48 | not robust; useful inside P3 |
| **P3. Split the stdlib wrap into a fixed chain of chunk proofs, verified as consecutive contract calls in one transaction (the atomic chunked-call mechanism of PROPOSAL 7.3, modes 1F and 1G, on the unmodified ledger)** | 3 chunks: chunk 1 = transcript preamble, LogUp-GKR, zerocheck, constraint DAG, batching, stacked reduction: 1,066,635 / 1,066,971 / 1,071,533 (1,069,313 mid at 48); chunks 2 and 3 = the WHIR query phase, queries split in half (queries are independent once the transcript fixes them): 598,853 / 707,374 / 757,354 each (869,662 mid, 919,642 high at 48); plus about 2,000 rows per cut for the handoff (ESTIMATE) | 21 for all three | yes: largest chunk 60 percent of the k = 21 gate in every case; chunk 2 and 3 fit k = 20 at the scenario `w_stack` | 3 proofs, 3 calls, 3 fees per logical call; prove time about 3 x the k = 21 decider (111.8 s CPU each, MEASURED `M-k21.md:41`), far above the 5 to 12 s wrap of PROPOSAL 8 |
| P3' Same at k = 20 | 4 chunks: chunk 1 cut after the constraint batching (about 0.64 M) and stacked reduction alone (about 0.43 M); WHIR in 2 chunks at the scenario `w_stack` (707,374 mid), 3 chunks at `w_stack` 48 | 20 | yes at mid; high at `w_stack` 48 needs 5 chunks | more calls, smaller per-call prove (68.1 s CPU at k = 20, MEASURED) |
| P4. G4 off chain, stdlib decider on chain (option C) | decider NOT CHECKED | | | two BLS12-381 layers: violates the one-wrap rule |
| P5. Fr-terminal layer (option E, OD-11) | NOT CHECKED | | | adds a prover layer; no row model exists |

**How P3 keeps the rules.**

- Handoff: chunk `j` ends with the verifier state at the cut (the Fr transcript sponge state of `L`, the commitment roots of `L` that later chunks open against, the pending EF claims, a few dozen Fr values). It exposes `cut_j = PoseidonFr(SHK_CUT, salt_j, state)` as a public input and writes it as a claim; chunk `j + 1` takes the same state as private witness, recomputes `cut_j`, and reads the claim. The salt keeps the state hidden (ZK); each proof is blinded with a fresh CSPRNG. The `L` proof stays private to the prover. `SHK_CUT` is a new tag (LEAD, owner item OD-18).
- Binding: the transcript state at the cut absorbs every commitment and message before it, and the roots are in the state, so chunk 2 and 3 Merkle openings are bound to the same `L` proof.
- Keys: three fixed keys (pre, query half 1, query half 2), the same for every program, so one universal key set of three wrap keys. Each is a `ZkStdLibArch` key (`poseidon`, pow2range), the format the ledger reads; today's stdlib decider key is 2,217 bytes (MEASURED, XC-9), so each is far below 50,000.
- Statement: the 1,026 public inputs of `2 + L_TX` go on the chunk that absorbs `pv` (chunk 1); the other chunks carry only their cut values.
- Ledger: unchanged. The chain uses the same claim mechanism as chunked calls (`CO/ATOMIC-CHUNKS.md`); a failed chunk drops the whole segment (1F) or invalidates the transaction (1G).
- Not checked: whether one exported entry can name a chain of three keys, or each chunk needs its own entry label (E-1 binding, PROPOSAL 7.2); transaction bytes for three calls (today one call is 10,421 bytes of 1,048,576); the fee of three calls.

**Recommendation (lead decision, owner to confirm): P3 at k = 21 with 3 chunks**, with the P2 rewrites inside the chunks for margin. It is the only candidate found that meets every listed constraint with margin in every case (largest chunk 60 percent of the gate, so it holds even if the stdlib gadgets cost 1.6 times the model). Its price is call time and fee: three k = 21 proofs per logical call. Option A (a G4 architecture in midnight-zk-stdlib, a ledger change) remains the performance target and is filed as an upstream request, not a dependency.

## 2c. The benchmark `L` cannot qualify production rows

The audit is right: `L_cached` is the bench `VerifierSubCircuit::<1>` over a 2-child bench node, keyed only, with 0 public values, no log AIRs and no proof. Every row above inherits that. The full-interface `L` adds, against the keyed structure:

| Addition | Effect on the wrap | Status |
|---|---|---|
| The 2 log AIRs and their 69 columns | in the model as a fixed add-on (`LOG_ADDON`) | ESTIMATE |
| `pv` absorbed in the preamble: `W_pv = 2B + 9V` = 2,816 words at 256/256 | transcript rows and the `eq` binding of `pv` | in the model |
| Range and `Canon` on 256 logged values, replay of 256 permutation slots (4,864 rows), bounded-quotient checks | log lines of the model: 76,264 rows at 256/256/0 against 204,932 at 1,024/1,024/16 | ESTIMATE |
| The mandatory `DIGEST` entries and the anchor reservation `R` | inside the slot counts | in the model |
| The three continuation AIRs of `RootCircuit` (user public values, memory) | more AIRs, columns, interactions | not keyed (`L-STRUCTURE.md` section 1) |
| The real child: the internal layer at internal parameters (97/82/81 queries), or on path C the program's leaf layer, and the DAG key check of PROPOSAL 4.3 on path C | the shape of `L`'s own verifier AIRs, its heights and `w_stack` | not modelled (PROPOSAL 13) |
| Real trace heights (D10) | `w_stack`, GKR depth `T`, distinct heights | D10 running |

So the production rows need one keygen and one proof of the full-interface `L` at the OD-19 set on the Fr configuration. That is the resume point of M7b, and P3's chunk sizes have to be re-read from it.

## 3. Status per milestone

| Milestone | Status |
|---|---|
| M7-0 blocking checks | done: both checks hold (sections 1.1 to 1.3) |
| M7a gadget skeleton and row counter | not started; G4 only under options A and C |
| M7b full circuit on a real `L` proof | not started; its first half (Fr configuration and one real `L` proof) holds under every option and is the resume point; it needs a stark-backend Fr configuration (BLS12-381 Fr field for p3 0.4.3 and the Midnight Poseidon permutation), which does not exist in any checkout |
| M7c, M7d, M7e | blocked on the option choice |

## Log

- 2026-11-11 22:40. Worktree `m7-wrap` from `main` 4ea6fa29d; merged `l-structure` (3957ddf7c) and `gpu-wrap` (52ad95069) without conflict.
- 2026-11-11 22:50. Read `MidnightVK::read` and the ledger key path: the constraint system comes from `ZkStdLib::configure(arch, k - 1)`. CODE.
- 2026-11-11 22:55. Single-lane stdlib wrap at the OD-19 set: 2,620,323 mid rows, over the k = 21 gate. ESTIMATE.
- 2026-11-11 23:00. Toy `m7_ledger_toy` written; build under the heavy lock (target copy `~/.cache/m7-wrap/target-gp`), waited about 1 h on the lock.
- 2026-11-12 00:2x. Toy run: library verify accepts; ledger refuses under all three headers with `problem reading the verifier key`. MEASURED. M7 stops at M7-0.
