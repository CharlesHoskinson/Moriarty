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

No option is recommended or adopted here: A changes the ledger, and C or any chain of stdlib wraps (P3 below) gives more than one BLS12-381 wrap proof. The choice is the owner's. Constraints: OD-19 set, k <= 21, ZK, one universal key set, ledger 9.1.0.1 unchanged, key at most 50,000 bytes, one BLS12-381 wrap. All rows ESTIMATE (`wrap_model.py` shape 1 with the 19-row stdlib Poseidon, OD-19 set, `L_cached`, the log at the OD-6 capacity 256/256/0 unless stated). Script: `split.py` beside `s1ud.py` (same patching as `~/.cache/ud-sizing/wrap_ud.py`).

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

P3 meets the row and ledger constraints but gives three BLS12-381 PLONK proofs per call, so it breaks the one-wrap rule; it is listed for the owner, not proposed.

### 2b.1 Option B on the D10 MEASURED inputs

D10 (`~/Projects/Moriarty-wt/d10-heights/research/sherlock-stark-framework-20261111/D10-HEIGHTS.json`, `primary.L_cached_bn254_ud`): `w_stack` 18, `T` 21, 14 distinct heights, maximum height 2^16, 17,263,856 main cells, MEASURED. Shape 1 (single lane, stdlib 19-row Poseidon) with those inputs, OD-19 set, `L_cached` structure; script `s1d10.py` (imports `d10_wrap.py`; its G4 column reproduces D10's 634,707 mid):

| Log capacity | Low | Mid | High | Mid against the k = 21 gate 1,782,573 |
|---|---:|---:|---:|---:|
| 1,024 / 1,024 / 16 | 2,369,556 | 2,596,249 | 2,696,547 | 145.6 percent |
| 256 / 256 / 0 (OD-6) | 2,262,952 | 2,457,645 | 2,557,943 | 137.9 percent |

Rows ESTIMATE (model) on MEASURED inputs. Option B fails k = 21 in every case, by 480,000 rows at the low end.

### 2b.2 Option E: an Fr-terminal layer `L_Fr` between `L` and the wrap

`L_Fr` is a STARK whose base field is BLS12-381 Fr, with Midnight Poseidon Merkle trees and transcript. It verifies `L` (emulating BabyBear in Fr), takes `L`'s 12,164 public words, and the wrap verifies `L_Fr` natively in Fr: no extension field, no BabyBear reduction, no packing. The wrap stays one stdlib-architecture BLS12-381 PLONK proof, which the unchanged ledger reads. `L_Fr` is hash-based, so it adds no BLS12-381 layer. No stark-backend configuration with an Fr base field exists (stark-sdk has BabyBear and BabyBear-with-BN254-hash configs only; `bn254_poseidon2.rs` holds permutations, not a base field).

**Wrap rows behind `L_Fr`** (script `optE.py`; ESTIMATE): `wrap_model.py` shape 1 lines re-costed for native Fr (one `NativeChip` row per product, adds at 2 per row, no range or `Canon` rows for transcript words, no packing; Fr Merkle leaves hash 2 values per permutation, so the round-0 leaf hash is `ceil(w_stack / 2)` permutations per row); OD-19 queries 98/83/82 and depths; D10 MEASURED heights and `w_stack` 18; the log at 256/256/0 kept in the wrap; `L`'s MEASURED AIR structure as a proxy for `L_Fr`'s (both are STARK-verifier AIR sets; `L_Fr`'s own structure is NOT MEASURED).

| Public words of `L` absorbed | Low | Mid | High | k |
|---|---:|---:|---:|---|
| 0 (the bench `L`) | 670,434 | 865,014 | 961,952 | 21 (high is 54 percent of the gate); k = 20 only at low |
| 12,164 (full interface, packed 8 per Fr) | 684,894 | 879,472 | 976,410 | 21 (55 percent of the gate) |

Mid lines: round-0 Merkle 432,729 (Fr leaves at `w_stack` 18 cost 9 permutations per row), final-polynomial Horner 138,530, rounds 1 and 2 Merkle 158,989, round-0 batching 28,224, transcript 24,857. Merkle hashing is now most of the wrap; the EF arithmetic that dominates option B is gone.

**Security terms of `L_Fr`** (DERIVED, not run): the query term does not depend on the field, so 98/83/82 with query PoW 20 keeps 101.1427 bits at log blowup 3. Each field-size term gains `log2 |Fr| - log2 |EF|` = 254.857 - 123.628 = 131.23 bits over the BabyBear D3 values (`D10-HEIGHTS.md` section 8.1): LogUp 216.0 without PoW, GKR sumcheck 253.3, GKR batching 254.9, zerocheck 251.0, constraint batching 244.9, stacked reduction 242.9, folding 232.9, `gamma` 248.2, OOD 239.9, `mu` about 227.8 without PoW. So only the query PoW is needed. This assumes each term is `log2 |F|` minus a field-independent amount; the D3 calculator must be rerun on an Fr configuration to make it MEASURED.

**Security terms of `L_Fr`, MEASURED by the D3 calculator** (2026-11-12): test `m7_fr_d3.rs` (copy of D10's `d10_d3.rs` in `~/.cache/m7-wrap/sb-d3`, the D3-patched stark-backend v2.1.0, with `base_field_order = r` and `challenge_field_bits = log2 r = 254.857`, no extension); output `fr-d3.out`. Shape bounds as PROPOSAL 11.2, heights 2^20 and 2^22, `w_stack` 18 and 48, UD, 1 OOD sample, every PoW 0 except query PoW 20:

| Term (b3, 98/83/82, h 2^22, `w_stack` 48) | Bits |
|---|---:|
| LogUp (PoW 0) | 215.9502 |
| GKR sumcheck / batching | 253.2721 / 254.8571 |
| zerocheck | 250.9502 |
| constraint batching | 244.8841 |
| stacked reduction | 242.8913 |
| `mu` (PoW 0) | 226.3025 |
| folding (PoW 0) | 232.8571 |
| query (rounds 101.3473 / 101.1435 / 101.7692) | 101.1435 |
| `gamma` / OOD | 248.2277 / 239.8571 |

Every gate passes in all 4 height and `w_stack` cases; the weakest term is always the query term, which does not depend on the field. The smallest query sets with every query term at least 101 bits are the same as for BabyBear `L`: 196/89/82 (b1), 120/85/82 (b2), 98/83/82 (b3), 89/82/82 (b4). So `L_Fr` needs no folding, `mu` or LogUp grinding, and the DERIVED figures above agree with the calculator to 0.1 bit (`mu` 226.3 against 227.8).

**What option E costs** (NOT MEASURED): a second proving engine (an Fr base field for stark-backend: NTT over Fr, whose two-adicity 32 covers the domain, and Poseidon over Fr; no GPU kernels exist); the `L_Fr` trace, which does the BabyBear-in-Fr verifier work that option B puts in PLONK rows (about 2.46 M stdlib-row equivalents of operations, ESTIMATE, at unknown AIR width); its prove time; and its own soundness premises (A2 for an Fr STARK). The deferred-log replay could move from the wrap into `L_Fr`, where Fr is native; that would cut the 76,264 log rows of the wrap (not modelled).

### 2b.3 A real `L` proof with the Midnight Poseidon Fr hash (MEASURED, 2026-11-12)

This is the first half of M7b, and it holds under every option. Code (copies under `~/.cache/m7-wrap/src/`; the files are saved in `lfr/` beside this file):

- `p3-bls12-381-fr`: a p3 0.4.3 field type `Bls12381Fr`, a transparent wrapper over `midnight_curves::Fq`, so every operation is the ff type's own.
- stark-sdk config `baby_bear_bls12381_poseidon.rs` (feature `baby-bear-bls12381-poseidon`): BabyBear base, EF degree 4, packing 8 BabyBear per Fr, 5 digits per squeeze. The permutation is midnight-circuits 7.3.0 `permutation_cpu`. The section 3.6 rules: leaf hash `(0, 0, l + 2^66)` with additive blocks of 2; node hash `(left, right, 2 + 2 * 2^66)`; transcript = stock `MultiFieldTranscript` from `(0, 0, 2^64 + 3 * 2^66)`. It needed one small stark-backend addition (`DuplexSponge::new_with_state`, `MultiFieldTranscript::new_with_sponge_state`, `packing_constants`; `lfr/stark-backend.diff`).
- Tests `m7_bls_fr.rs`, 6 of 6 pass (`lfr/tests.log`): the field against the ff type; the packing constants; the permutation against an independent no-skip implementation from midnight's constants; the u = 0 sponge against midnight's `PoseidonChip::hash` for lengths 0 to 8; the leaf, node and transcript rules; a small AIR proved, verified and refused after a tampered commitment.
- Program `l_fr.rs`: D10's chain (W1 2 x 2^20, then the node at the OD-19 internal set, then `L_cached` at the OD-19 `L` set with this engine), checked by the stock verifier. Run in systemd unit `m7-lfr` under the heavy lock, `RAYON_NUM_THREADS` 20, at load 11 to 35 (other jobs running).

| Quantity | Value |
|---|---|
| `L` parameters | UD, log blowup 3, queries 98/83/82 (263), folding PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18, `l_skip` 2, `n_stack` 18, `w_stack` bound 48 |
| Heights | 39 AIRs, maximum 2^16, 14 distinct heights, `T` 21 (`n_logup` 19), 17,263,856 main cells, `w_stack` needed 18; equal to D10's BN254 run AIR by AIR (`heights_match_d10` true) |
| `L` prove | 206.2 s on the CPU (D10's BN254-hash `L`: 119.7 s at load above 30; this run had other jobs on the host, so the ratio is not a hash cost) |
| `L` keygen / tracegen | 0.23 s / 0.37 s |
| Peak RSS of `L` keygen, tracegen and prove | 2.11 GB (VmHWM after a reset); whole process 11.67 GB including the leaf and node stages |
| `L` proof bytes | 430,073 (the model's ESTIMATE was 498.3 KiB at the scenario `w_stack` and 682.5 KiB at 48) |
| Verify | accepts, 0.78 s; 4 of 4 single-byte flips refused (one fails decoding, three `WhirError`); `L_cached` has no public values, so the public-value negative does not apply |
| Node (child) | arity 2, OD-19 internal set (97/82/81), proved in 24.2 s and verified |

Raw record: `lfr/l_fr.json` (`~/.cache/m7-wrap/lfr/l_fr.json`).

### 2b.4 What this changes for option E

- `L_Fr` itself was not built or proved. It needs a stark-backend whose base field is Fr. The new `Bls12381Fr` type is a p3 field, but it has not been tried as a STARK base field. It also needs a recursion AIR set over Fr that verifies a BabyBear-plus-Fr-hash STARK; OpenVM's recursion circuits are written for BabyBear. Neither exists, and neither fits in one night.
- So the option E wrap rows of section 2b.2 cannot be recomputed from a measured `L_Fr` shape. Their `L`-side inputs are now MEASURED on the real Fr-hash `L` (heights, `T`, `w_stack` 18, the same as D10), so 684,894 / 879,472 / 976,410 stands unchanged (ESTIMATE; `L_Fr`'s AIR structure is still proxied by `L`'s).
- What `L_Fr` must verify is now MEASURED: one 430,073-byte `L` proof with 263 queries over Midnight Poseidon Merkle trees. The Fr hashing inside `L_Fr` is native, so its trace carries the BabyBear arithmetic of `L`'s verifier and the Merkle paths at one Poseidon per node.
- `L_Fr`'s soundness terms are MEASURED by the D3 calculator at the Fr field (section 2b.2).

## 2c. Option E build plan (for the owner's OD-24 weighing; no option adopted)

Every effort figure is ESTIMATE, for one engineer with agent help. The facts behind each estimate are CODE (read 2026-11-12) unless marked.

**What exists to reuse.**

- `Bls12381Fr`, a p3 0.4.3 field over `midnight_curves::Fq` with a 2-adic generator, tested against the ff type (MEASURED, section 2b.3).
- The Midnight Poseidon rules of PROPOSAL 3.6 (leaf, node, transcript), tested against midnight-circuits (MEASURED).
- The D3 calculator at the Fr field (MEASURED, section 2b.2).
- midnight-circuits 7.3.0 chips for the wrap: `NativeChip`, the 19-row Poseidon chip and pow2range, all inside a `ZkStdLibArch` that ledger 9 reads.
- The ledger toy harness `m7_ledger_toy.rs` for the acceptance gate.
- OpenVM's halo2 `openvm-static-verifier` (about 12,000 lines; WHIR, batch-constraint, stacked-reduction and proof-shape stages) as a template for a PLONK verifier of a SWIRL proof.
- No OpenVM native or root config helps with the base field. Every v2 config, BN254 root included, has a BabyBear base and changes only the hash.

**Blockers found in the code.**

- stark-backend fixes the base field to 64 bits: `StarkProtocolConfig::F: TwoAdicField + PrimeField64` (`crates/stark-backend/src/config.rs:22`).
- 48 sites in `stark-backend/src` and `cpu-backend/src` use `PrimeField64`, `as_canonical_u64` or `ORDER_U64`, among them `sample_bits` and `grind` (`transcript/traits.rs:46-82`), the proof decoder (`verifier/transcript_extractor.rs`) and the soundness glue (`soundness/mod.rs:18`). Another 28 sites in `stark-backend/src` use 32-bit field calls.
- The OpenVM recursion circuit (`crates/recursion`, 35,297 lines) is written over BabyBear. It has BabyBear-native constraints and Poseidon2-BabyBear sub-AIRs, so it cannot verify `L` from an Fr base without a rewrite.

| Step | Component | Reuse | Gate | Effort (ESTIMATE) |
|---|---|---|---|---|
| E1 | stark-backend and cpu-backend with an Fr base field. Relax `PrimeField64` to `PrimeField` at the 48 sites. Sampling and grinding for a 255-bit field. `EF = F` (degree 1). A native Fr duplex transcript and Midnight Poseidon Merkle trees (u = 1, 2, 3). | `Bls12381Fr`; the section 3.6 rules; the Fr D3 test | A small AIR proves and verifies end to end at the Fr base, with negative controls. The D3 terms at the configured parameters equal section 2b.2. | 3 to 6 days |
| E2 | Fr recursion AIR set `L_Fr` that verifies one `L` proof: BabyBear and EF arithmetic emulated in Fr (range-checked reduction mod p), a Midnight Poseidon sub-AIR, proof-shape, GKR, zerocheck, stacked-reduction and WHIR AIRs. Either a port of OpenVM's 35,297-line recursion crate or a dedicated `L` verifier. | OpenVM recursion as the structural template; the real 430,073-byte `L` proof as the test input | `L_Fr` proves over the real `L` proof and the stock verifier accepts. Negative controls refuse a changed `L` proof byte and a changed public word. Heights MEASURED, then D3 at the measured shape. | 6 to 12 weeks |
| E3 | The full interface carried through `L_Fr`: the 12,164 public words of `L` taken in and exposed (packed), the log AIRs and the non-`NOP` reservation `R`, and an optional log replay in native Fr | E2 | An all-non-`NOP` log case and a forced-overflow case pass. The wrap's statement words equal the native ones. | 1 to 2 weeks |
| E4 | Prover speed of `L_Fr`: 255-bit NTT and Poseidon over Fr on the GPU (kernel K0 is shared with the wrap plan) | `crates/mori-gpu` sppark NTT for BLS12-381 Fr | Measured `L_Fr` prove time and memory against the call-time budget | 2 to 3 weeks (CPU-only first is possible) |
| E5 | Stdlib-architecture wrap that verifies `L_Fr` natively in Fr: `ZkStdLibArch` with `poseidon` and pow2range, Fr Merkle paths, sumcheck and WHIR verifier, Fr transcript replay, 1,026 public inputs | midnight-circuits chips; the static-verifier stages as a template; the model rows of section 2b.2 | `MockProver` rows within the k = 21 gate (the model: 879,472 mid). Key at most 50,000 bytes in the `MidnightVK` framing. Ledger 9 `VerifierKey::verify` accepts a blinded proof; the toy harness is ready. Negative controls on a public input and a proof byte. | 3 to 5 weeks |
| E6 | Keys and integration: the `L_Fr` key joins the universal key set; a new hash tag for `L_Fr`'s Fr transcript (OD-18); a soundness premise for the Fr STARK beside A2; Lean wording; a D3 rerun; the devnet deploy and call on the XC-9 path, then Preview | XC-9 and the Preview rehearsal | Devnet call included and refusals R1 to R3 pass; the D3 gates pass at the measured shapes | 1 to 2 weeks |

Order: E1, then E2 (the long pole; it needs E1), then E3 and E5 in parallel (E5 can start on a synthetic `L_Fr` proof from E1), then E4, then E6. Total about 3 to 5 months, dominated by E2.

For the same weighing (ESTIMATE, rougher):

- Option A: the G4 wrap circuit (M7a and M7b, 4 to 8 weeks), plus an upstream `ZkStdLibArch` lane flag and a ledger release, whose calendar time Midnight sets.
- Option C: the same G4 circuit, plus a stdlib decider over the G4 proof (1 to 2 weeks on the existing decider); two BLS12-381 layers.

**Step E1 tonight: not started.** It is self-contained and testable, but the 48-site `PrimeField64` relaxation and the new sampling and grinding code cannot be done and tested in about 2 hours (ESTIMATE 3 to 6 days). Per the instruction, it stops here.

## 2d. The benchmark `L` cannot qualify production rows

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
| M7b full circuit on a real `L` proof | first half done (section 2b.3): Fr-hash config, tests and one real `L` proof, MEASURED; the wrap circuit itself waits on the owner's OD-24 choice; it needed a stark-backend Fr configuration (BLS12-381 Fr field for p3 0.4.3 and the Midnight Poseidon permutation), which does not exist in any checkout |
| M7c, M7d, M7e | blocked on the option choice |

## Log

- 2026-11-11 22:40. Worktree `m7-wrap` from `main` 4ea6fa29d; merged `l-structure` (3957ddf7c) and `gpu-wrap` (52ad95069) without conflict.
- 2026-11-11 22:50. Read `MidnightVK::read` and the ledger key path: the constraint system comes from `ZkStdLib::configure(arch, k - 1)`. CODE.
- 2026-11-11 22:55. Single-lane stdlib wrap at the OD-19 set: 2,620,323 mid rows, over the k = 21 gate. ESTIMATE.
- 2026-11-11 23:00. Toy `m7_ledger_toy` written; build under the heavy lock (target copy `~/.cache/m7-wrap/target-gp`), waited about 1 h on the lock.
- 2026-11-12 00:2x. Toy run: library verify accepts; ledger refuses under all three headers with `problem reading the verifier key`. MEASURED. M7 stops at M7-0.
