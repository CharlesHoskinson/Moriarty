# E5 results: the one BLS12-381 wrap that verifies `L_Fr` natively in Fr

Plan: `PLAN.md` beside this file. Owner ruling 2026-11-12: OD-24 is option E.

## 1. Status

**E5 gate: PASS (I6b, section 6f; final circuit section 6g).** The one BLS12-381 wrap in `ZkStdLibArch` verifies the real E2 I10 `L_Fr` proof (widths 33 + 10, 15 traces) natively in Fr. With levers L1 and L2 it is 1,436,529 rows at k = 21, 80.6 percent of the k = 21 gate (1,782,573). The key is 1,325 bytes in the `MidnightVK` framing. The stock ledger 9.1.0.1 `VerifierKey::verify` accepts the blinded proof with 1,026 public inputs and refuses a changed public input and a flipped proof byte. Keygen takes 84.6 s and the blinded prove 183.8 s in an exclusive heavy slot. All figures are MEASURED. The `L_Fr` shape is a parameter, pinned from its key and its fixture proof (section 7).

**E3 map: PASS (I7, I8, section 6i).** On the E3 all-entries `L_Fr` proof (openvm e3-iface 2f09caf, 1,522 public words), PublicMap is replaced by the E3 map: the D-9 decoder in the circuit, and the statement is the decoded call vector (1,026 lanes). The wrap is 1,605,413 rows at k = 21, 90.1 percent of the gate, with 177,160 rows of headroom. Keygen takes 82.6 s and the blinded prove 131.6 s under `heavy-slot --exclusive`. The stock ledger 9.1.0.1 returns `Ok(())` in 0.062 s. All figures are MEASURED. **For owner confirmation:** the scope (every decoder check in the circuit; no J6 replay) and the pinned pre-hash (section 6i).

| # | Increment | Status | Evidence |
|---|---|---|---|
| I0 | Crate, vendored permutation access, ledger smoke (1,026 public inputs) | PASS | section 3 |
| I1 | Fr transcript replay | PASS | section 4 |
| I2 | Merkle paths | PASS | section 5 |
| I3 | Preamble, LogUp PoW, GKR, zerocheck rounds | PASS | section 6a |
| I4a | Column openings, DAG, interactions, final batch-constraint claim | PASS | section 6b |
| I4b | Stacked reduction | PASS | section 6c |
| I5 | WHIR (whole verifier) | PASS | section 6d |
| I6a | Full relation, keygen, blinded proof, stock ledger, on `mix-small` | PASS | section 6e |
| I6b | The same on the real `L_Fr` proof (`lfr-real`, widths 33 + 10), rows within the k = 21 gate | PASS | section 6f |
| I7 | The E3 map: D-9 decoder over `Alg`; native verdicts equal stock on 11 cases; gadget `MockProver` | PASS | section 6i |
| I8 | The full wrap of the E3 all-entries `L_Fr` proof (`lfr-e3`) on the E3 map, exclusive timing, stock ledger | PASS | section 6i |
| Q1 | Gemini Quint lane on the D-9 decoder circuit (`decode.rs`) | PASS: SOUND and COMPLETE, 8 of 8 mutants caught, no defect | section 6k |
| I9 | ENC wrap-side vectors: Fr vectors on the wrap gadgets; `pv` binding and a non-empty lane map through the decoder | PASS (33 of 55 pinned vectors apply to the wrap) | section 6j |

## 2. Code

| Repository (not pushed) | Branch | Content |
|---|---|---|
| Moriarty `~/Projects/Moriarty-wt/e5-wrap` | `e5-wrap` (from `e2-lfr` 6fc8e3c21) | `tools/sherlock-wrap`: the wrap crate in its own workspace and lock; `tools/sherlock-wrap/ledger-check`: the stock ledger 9.1.0.1 check in a second workspace; this directory |
| openvm `~/.cache/m7-wrap/src/openvm-e5` (a git worktree of the openvm clone, so the `../openvm-org_stark-backend` path patch resolves) | `e5-wrap`, rebased onto `e2-lfr` c4bd312 (the E2 gate) for the `lfr-real` export | crate `crates/sherlock-wrap-export`: the exporter (`src/lib.rs`), the fixtures (`examples/fixtures.rs`), `examples/lfr_e4.rs` (fixture `lfr-real` from the E4 CPU-baseline proof and key cache of the I10 tape, no new prove) and `examples/lfr_real.rs` (the same with a new keygen and prove) |

**`tools/sherlock-wrap`** (the M7 place: a standalone workspace beside `tools/gpu-prover`):

| File | Content |
|---|---|
| `Cargo.toml`, `Cargo.lock` | the lock was seeded from `tools/gpu-prover/Cargo.lock`; `scripts/lock-check.py` finds no package version outside the openvm, backend, Moriarty root and gpu-prover locks (MEASURED, both workspaces) |
| `vendor/midnight-circuits-7.3.0-perm`, `vendor/midnight-zk-stdlib-2.4.0-perm`, `patches/perm-access.diff` | visibility-only patch: `PoseidonChip::permutation` and `AssignedRegister` become `pub`; `ZkStdLib::poseidon_chip()` is added. `configure` is not changed. |
| `src/native.rs` | Midnight Poseidon (`permutation_cpu`), `cap_fixed`, `cap_var`, leaf and node hash, the native duplex mirror, hex codec |
| `src/alg.rs` | trait `Alg` (the verifier's operation set) with two back ends: `Native` (`Value<F>`, records failed checks: the plain verdict and the witness generator) and `Gadget` (`ZkStdLib` cells) |
| `src/transcript.rs` | the stock overwrite duplex over `Alg`: observe, sample, `sample_bits` (canonical decomposition), `check_witness` |
| `src/merkle.rs` | `hash_slice` (u = 5), `compress` (u = 6), `tree_compress`, `merkle_verify` with direction bits taken from the sampled bits |
| `src/poly.rs` | `poly_common` helpers over `Alg` (eq, interpolation at 0..1, 0..2 and 0..3, Horner, powers) |
| `src/verifier.rs` | the `L_Fr` verifier over `Alg`: `Shape::from_wrapin` (the pin: params, presence, heights, trace order, per-AIR DAG and interactions, opening widths from the key), `ProofData`, `part1` (preamble, LogUp PoW, GKR, zerocheck rounds), `part2` (`batch_constraints.rs` steps 7 to 9) |
| `src/verifier_test.rs` | test relation: `part1` (and `part2`) with every sample asserted equal to the stock sample |
| `src/stacked.rs` | `StackShape` (layouts from the key shape), `StackData`, `part3`: a port of `stacked_reduction.rs` (rotation kernel, `eq` prism and the `n < 0` indicator cached per `(n, need_rot)`) |
| `src/whir.rs` | `WhirShape`, `WhirData`, `part4`: a port of `whir.rs` (`mu` PoW and batching, folding sumchecks, OOD, query PoW, round-0 openings per commitment with leaf hash, tree compress and Merkle path, codeword rounds, `binary_k_fold` with `omega^index` and `omega^-index` taken from the index bit cells, final check) |
| `src/wrap.rs`, `src/bin/wrap.rs` | I6 `WrapRelation` (statement 1,026 words by `PublicMap`) and its run: cost model, MockProver, keygen, blinded proof, files for `ledger-check` |
| `src/wrapin.rs` | reader of the exported file |
| `src/smoke.rs`, `src/bin/smoke.rs` | I0 smoke relation and its prove and key writer |
| `src/replay.rs`, `src/merkle_test.rs` | I1 and I2 test relations |
| `tests/i0_native.rs`, `tests/i1_transcript.rs`, `tests/i2_merkle.rs`, `tests/i3_part1.rs`, `tests/i4_part2.rs`, `tests/i4b_stacked.rs`, `tests/i5_whir.rs` | the gates |
| `ledger-check/` | stock `midnight-transient-crypto` at rev `a2b30128` (tag `ledger-9.1.0.1`); no vendored midnight crate in its graph |

**Exporter (`wrapin` v1, JSON).** `export()` runs the stock `verifier::verify` with a recording transcript and writes `vk` and `proof` (serde form; an Fr value is 32 little-endian canonical bytes), the `layouts` of `verify_proof_shape`, the derived parameters, the domain codes and the transcript log (`o`, `c`, `s` events with 64-digit big-endian hex). Fixtures in `~/.cache/e5-wrap/fixtures/`: `MixtureFixture::standard(6)` (Fibonacci with public values, LogUp interactions, cached main, preprocessed trace, self interactions) proved with `Bls12381FrPoseidonLfrCpuEngine` (u = 5, 6, 7):

| Fixture | Parameters | Prove (MEASURED) | Log events |
|---|---|---:|---:|
| `mix-small` | UD, log blowup 1, `l_skip` 2, `n_stack` 8, `k_whir` 4, LogUp PoW 7, folding PoW 5, `mu` PoW 4, query PoW 6 | 1.2 s | 955 |
| `mix-lfr` | the `L_Fr` set of E1 and E2: UD, log blowup 3, `l_skip` 2, `n_stack` 18, `k_whir` 4, queries 98/83/82, query PoW 20, other PoW 0 | 320.9 s at 16 threads, load about 25 | 1,377 |

The exporter crate needs the `parallel` features of stark-backend and stark-sdk (the workspace dependencies are `default-features = false`). The first `mix-lfr` run was without them, ran single-threaded for 40 minutes, and was stopped. With the features, `examples/fixtures lfr` takes about 5 minutes (MEASURED above).

**Runner.** `~/.cache/e5-wrap/run.sh <stage> <mem> <dir> cmd...`: heavy lock, `heavy-run`, `CARGO_TARGET_DIR=~/.cache/e5-wrap/target`, `RUSTUP_AUTO_INSTALL=0`, `RUSTFLAGS=-C target-cpu=native`, logs `~/.cache/e5-wrap/logs/<stage>.log`. Scripts: `~/.cache/e5-wrap/i0.sh` (I0 to I2: tests, lock check, smoke, ledger check, I1 and I2 tests) and `~/.cache/e5-wrap/export.sh` (`WHICH=small|lfr|all`). Final logs: `i0c.log`, `export2.log`.

## 3. I0: permutation access and ledger acceptance (MEASURED)

| Check | Result |
|---|---|
| `i0_u0_sponge_equals_midnight_hash` | the u = 0 sponge with this crate's permutation equals `PoseidonChip::hash`, lengths 0 to 8 |
| `i0_hex_and_low_bits` | hex round trip, low bits, non-canonical value refused |
| Smoke relation | `ZkStdLibArch { poseidon, nr_pow2range_cols 1 }`; 1,025 witness words observed by the `L_Fr` transcript (u = 7) through the patched bare permutation; public inputs = the 1,025 words and the first sample (1,026); then a 12-bit `check_witness` |
| Cost model, k = 15 | 11,590 rows, 8 advice and 27 fixed columns, 1 lookup |
| `MockProver` | honest accepted; a changed public input refused; a changed PoW witness refused |
| Keygen and prove (ceremony SRS `bls_midnight_2p15`, `OsRng`, Blake2b) | 1.97 s and 3.52 s; proof 2,912 bytes; library verify accepts |
| Key bytes in the `MidnightVK` framing | 1,325 |
| Ledger 9.1.0.1 `VerifierKey::verify` (stock crates, `ledger-check`) | `Ok(())`, 0.067 s; a changed public input refused (first, middle and last of 1,026); a flipped proof byte refused (first, middle, last) |

So the vendored patch does not change the constraint system: the stock reader rebuilds it from the architecture flags and accepts the proof. The M7-0 blocker does not apply to this route.

## 4. I1: the `L_Fr` transcript (MEASURED)

| Test | Result |
|---|---|
| `i1_native_replay_equals_stock` | `mix-small`: 955 events, 252 of 252 samples equal; `mix-lfr`: 1,377 events, 402 of 402 samples equal |
| `i1_circuit_replay_equals_stock` (`mix-small`, k = 17) | every observe and commitment observe is a witness cell, every sample is computed in circuit and asserted equal to the recorded one; `MockProver` accepts; a changed observed value is refused. 10,015 rows for 955 events and 453 permutations |

`sample_bits` uses the stdlib canonical decomposition (`assigned_to_le_bits(x, None, enforce_canonical = true)`), so the alias `x + r` has no assignment. Its PoW use is in the smoke run (section 3). Its row cost is not measured separately yet.

## 5. I2: Merkle paths (MEASURED)

`merkle_test::openings` takes every WHIR opening of the proof: round 0 for each commitment (`2^k_whir` = 16 opened rows), then the codeword rounds (16 single values). The index sample of each opening is found among the recorded samples (test-only search; the circuit takes it from its transcript position in I5). The direction bits are the low bits of that sample cell.

| Test | Result |
|---|---|
| `i2_merkle_native_all_openings` | `mix-small`: 366 openings accepted (19,118 permutations); `mix-lfr`: 557 openings accepted (27,603 permutations); refused: a changed sibling, a changed opened value, a changed index bit |
| `i2_merkle_mockprover` (`mix-small`, one opening of each shape, k = 16) | 3 openings, 4,740 rows; accepted; refused: a changed sibling, a changed index bit |

## 6a. I3: preamble, LogUp PoW, GKR, zerocheck rounds (MEASURED)

| Test | Result |
|---|---|
| `i3_part1_native_equals_stock` | `mix-small`: 12 traces, `n_logup` 10, 99 samples equal to the stock record in order, 0 failed checks; `mix-lfr`: 12 traces, `n_logup` 10, 98 samples equal, 0 failed checks. Refused: a changed GKR layer claim, a changed GKR round polynomial, a changed univariate coefficient 0, a changed numerator term; a changed LogUp PoW witness is refused on `mix-small` (7 bits; `mix-lfr` has 0 bits) |
| `i3_part1_mockprover` (`mix-small`, k = 16) | 6,083 rows; accepted; a changed layer claim refused |

The univariate coefficient count uses `params.max_constraint_degree` (the first `mix-lfr` run used the AIR maximum and failed at the first sample after the coefficients; log `i3.log`). A coefficient off the `step_by(2^l_skip)` positions only changes later samples; the I4a final claim refuses it.

## 6b. I4a: column openings, DAG, interactions, final batch claim (MEASURED)

`part2` observes the common-main openings of every trace first, then the other parts per trace (`(claim, rot)` pairs; the rot value is the constant 0 without rotation). It evaluates every AIR's DAG at the lifted point (`is_first_row`, `is_last_row`, `is_transition`, the `n < 0` case with `r_0^(2^-n)` and the `1/2^-n` norm), the interactions (message by powers of `beta`, bus index plus 1, weighted by `eq_3b`), `eq_ns`, `eq_sharp_ns`, and asserts the `mu`-batched sum equals the last sumcheck claim. The opening widths come from the key (`params.width`) and equal the proof's (asserted in the test).

| Test | Result |
|---|---|
| `i4_part2_native_equals_stock` | both fixtures: the final batch claim holds, 0 failed checks; the first stacked-reduction sample after the openings equals the stock sample (100 samples on `mix-small`, 99 on `mix-lfr`); 500 DAG nodes and 52 interactions over 12 traces. Refused: each of the 15 opening parts changed (15 of 15), univariate coefficient 1, a round polynomial value; a changed public value changes the run |
| `i4_part2_mockprover` (`mix-small`, k = 17) | 12,995 rows for parts 1 and 2 (6,912 for part 2, DERIVED); accepted; a changed column opening refused |

Log: `~/.cache/e5-wrap/logs/i4.log` (with I3 rerun).

## 6c. I4b: stacked reduction (MEASURED)

| Test | Result |
|---|---|
| `i4b_stacked_native_equals_stock` | both fixtures: the stacked final sum holds, 0 failed checks; samples equal the stock record through `u_{n_stack}` (109 on `mix-small`, 118 on `mix-lfr`); 150 column claims over 4 commitments; key-derived univariate count and stacked widths equal the proof's. Refused: 8 of 8 changed stacking openings (first and last of each commitment), univariate coefficients 0 and 1, a round polynomial value |
| `i4b_stacked_mockprover` (`mix-small`, k = 17) | 16,017 rows for parts 1 to 3; accepted; a changed stacking opening refused |

## 6d. I5: WHIR, the whole verifier (MEASURED)

| Test | Result |
|---|---|
| `i5_whole_verifier_native_equals_stock` | `mix-small`: accepted, 252 of 252 stock samples equal in order, 19,571 permutations, queries 82/38; `mix-lfr`: accepted, 402 of 402 equal, 28,260 permutations, queries 98/83/82. Refused on both: a changed initial row value (first and last commitment), initial sibling, codeword value, codeword sibling, codeword commitment, OOD value, WHIR sumcheck value, final polynomial coefficient, stacking opening, common main commitment (11 of 11); a changed query PoW witness (6 and 20 bits) |
| `i5_whole_verifier_mockprover` (`mix-small`, k = 19) | 519,158 rows; accepted; a changed opened row refused |
| `i5_rows_at_lfr_params` (cost model, `mix-lfr`, k = 21) | 898,562 rows, 50.4 percent of the k = 21 gate (DERIVED from the MEASURED rows). This is the `L_Fr` parameter set but the mixture's stacked widths (1 per commitment, 4 commitments), not the E2 widths 33 + 10; `lfr-real` measures those |

The round-0 roots are the observed cells (common main, cached) and the key constants (preprocessed); a codeword round's root is the cell observed in the previous round. Log: `~/.cache/e5-wrap/logs/i5.log`.

## 6e. I6a: the full wrap of `mix-small` and the stock ledger (MEASURED)

`WrapRelation` (`src/wrap.rs`) runs `part1` to `part4` on the gadget back end; the `L_Fr` shape is pinned as circuit constants; the statement is 1,026 words by `PublicMap` (LEAD: the `L_Fr` public values in AIR order, the rest constrained zero; `mix-small` has its Fibonacci public values). Run `~/.cache/e5-wrap/i6.sh mix-small 20` (log `i6-small.log`):

| Quantity | Value |
|---|---|
| Cost model, k = 20 | 518,774 rows (29.1 percent of the k = 21 gate, DERIVED), 8 advice and 27 fixed columns, 1 lookup |
| `MockProver` | honest accepted; a changed public input refused; a changed opened WHIR row refused (13.9 s) |
| Keygen (ceremony SRS `bls_midnight_2p20`), prove (`OsRng`, Blake2b) | 51.0 s and 70.0 s on a shared host (not a clean timing); proof 2,912 bytes; library verify accepts |
| Key bytes in the `MidnightVK` framing | 1,325 |
| Ledger 9.1.0.1 `VerifierKey::verify` (stock crates, `ledger-check`) | `Ok(())` in 0.077 s; a changed public input refused (first, middle, last of 1,026); a flipped proof byte refused (first, middle, last) |

The key size does not depend on the circuit beyond the architecture and k, so the 50,000-byte limit holds with a wide margin at any k up to 21 (DERIVED from the I0 and I6a keys, both 1,325 bytes).

## 6f. I6b: the wrap of the real `L_Fr` proof, the E5 gate (MEASURED)

Fixture `lfr-real`: the E2 I10 `L_Fr` proof of the whole `L` verifier on the real bench `L` proof, at the `L_Fr` set (UD, log blowup 3, `l_skip` 2, `n_stack` 18, `k_whir` 4, queries 98/83/82, query PoW 20). It is exported by `examples/lfr_e4.rs` from the E4 key cache and CPU-baseline proof of the unchanged tree (openvm c4bd312), read only, with no new prove; the stock verifier accepts the pair (57.4 s, log `export-e4.log`). 2 commitments, stacked widths 33 and 10, 15 traces with `n` per trace 17, 17, 14, 14, 13, 12, 11, 11, 11, 11, 9, 8, 7, 7, -1; 3,123 transcript events; 1 public value (the `L` vk pre-hash, D-2).

| Check | Result |
|---|---|
| `i5_lfr_real_native` (log `i5real.log`) | the whole verifier accepts; 649 of 649 stock samples equal in order; 50,682 permutations; refused: a changed preprocessed row, a changed opening of the `n = -1` trace, a changed public value |
| Cost model, k = 21 (log `i6-real.log`) | **1,592,979 rows**, 89.4 percent of the k = 21 gate 1,782,573 (DERIVED from the MEASURED rows); 8 advice and 27 fixed columns, 1 lookup, table rows 1,056,771 |
| `MockProver`, k = 21 | honest accepted; a changed public input refused; a changed opened WHIR row refused (69.8 s) |
| Keygen (`bls_midnight_2p21`), blinded prove (`OsRng`, Blake2b) | 123.1 s and 210.3 s on a shared host (not a clean timing); proof 2,912 bytes; library verify accepts |
| Key bytes in the `MidnightVK` framing | **1,325** (limit 50,000) |
| Ledger 9.1.0.1 `VerifierKey::verify` (stock crates, `ledger-check`) | **`Ok(())`** in 0.079 s; a changed public input refused (first, middle, last of 1,026); a flipped proof byte refused (first, middle, last) |

The public-value carrier of E3 (about 1,522 Fr words for the full `L` interface) will grow `L_Fr` and these rows; with 189,594 rows of headroom (10.6 percent) the levers of section 8 matter.

## 6g. Row levers on `lfr-real` (MEASURED, cost model at k = 21)

Each lever is applied alone on top of the one before; each is checked by the I3 to I5 tests (native on all three fixtures, `MockProver` on `mix-small`) before its rows are taken.

| Lever | Rows | Percent of gate | Change |
|---|---:|---:|---:|
| none (I6b) | 1,592,979 | 89.4 | |
| L1 fused multiply-add: `Alg::mul_add` is the native chip's one-row `add_and_mul` (`x*y + z`), used in the WHIR `mu` batching, the final-polynomial Horner, the `gamma` and final-check sums, `binary_k_fold`, the DAG `lambda` sum, the interaction sums and the stacked sums. Needs a visibility-only accessor `ZkStdLib::native_gadget()` (patch `patches/perm-access.diff`; `configure` unchanged). Log `lever-fma.log` | 1,453,425 | 81.5 | -139,554 |
| L2 `omega^index` chains: the `omega^(2^j)` constants assigned once per round (not per query and chain); `zi = zi_root^(2^k)` by k squarings; `zi^-1` witnessed and pinned by `zi_root * zi_inv = 1` (was three select chains of key constants per query). Log `lever-omega.log` | 1,436,529 | 80.6 | -16,896 |

Final circuit (L1 + L2), run `i6.sh lfr-real 21` under `heavy-slot --exclusive` (log `i6-excl.log`; heavy-slot gave the job the host alone among slot jobs, host load 30 to 37 from work outside the slots, so the times are the cleanest available, not an idle host):

| Quantity | Value |
|---|---|
| Cost model, k = 21 | 1,436,529 rows (80.6 percent of the gate; 346,044 rows of headroom), 8 advice and 27 fixed columns |
| `MockProver` | honest accepted; a changed public input refused; a changed opened WHIR row refused (37.2 s) |
| Keygen (`bls_midnight_2p21`) | 84.6 s |
| Blinded prove (`OsRng`, Blake2b) | 183.8 s; proof 2,912 bytes; library verify accepts |
| Key bytes in the `MidnightVK` framing | 1,325 |
| Ledger 9.1.0.1 `VerifierKey::verify` (stock) | `Ok(())` in 0.107 s; a changed public input refused (3 positions); a flipped proof byte refused (3 positions) |

## 6h. Row budget at the measured `L_Fr` shape (DERIVED)

E2 I10 (`e2-lfr` RESULTS.md section 8g, openvm `e2-lfr` c4bd312): 15 AIRs, 2 commitments, stacked widths 33 (main) and 10 (preprocessed program table) at log stacked height 20, proof 2,516,530 bytes. The wrap model of M7 assumed width 18; the figures below use 33 + 10.

Unit costs (DERIVED from the MEASURED cost models): a permutation is at most 19.4 rows ((10,015 - 955 witness rows - 252 assertions) / 453 in I1); an addition, a multiplication, a witness or a `select` is taken as 1 row.

| Line | Per query | Queries | Rows |
|---|---|---:|---:|
| Round 0: 16 rows x 43 values (688 witnesses, 640 leaf additions), 16 x (17 + 5) leaf permutations, 2 x 15 tree compressions, 2 x 20 path levels (log RS domain 23 minus `k_whir` 4 is 19, plus 1 margin) with 2 `select` each | about 9,600 | 98 | about 941,000 |
| Round 0 `mu` batching of the 688 opened values | 688 | 98 | about 67,000 |
| Rounds 1 and 2: 16 values, 16 + 15 + about 18 permutations, `select`s | about 1,010 | 165 | about 167,000 |
| Transcript, GKR, zerocheck, DAG of 15 AIRs (at most 171 constraints each), stacked reduction over about 506 column claims, folds, final polynomial, 263 canonical query decompositions | | | about 100,000 to 200,000 (ESTIMATE) |
| Total | | | about 1.28 M to 1.38 M, 72 to 77 percent of the k = 21 gate (1,782,573) |

So the measured `L_Fr` shape fits k = 21 by this estimate, with less margin than the M7 model (879,472 mid). Round-0 leaf hashing at width 43 is about 70 percent of the total. It scales with the stacked width, so a wider `L_Fr` (E3 adds the public-value carrier, about 1,522 rows' worth for 12,164 words) moves it up. The public-value count stays a parameter. Levers if needed: fewer preprocessed columns in `L_Fr`, leaf additions folded into fewer rows, a smaller `k_whir` at round 0. The I6 `MockProver` replaces this estimate.

## 6i. I7 and I8: the E3 map on the E3 all-entries proof (MEASURED)

**Fixture.** `lfr_e3` is in openvm `e5-wrap` e147918, rebased on E3 2f09caf. It reruns the E3 J5 all-entries case (seed 14) and proves `L_Fr` at `lfr_params`: keygen 303.5 s, prove 732.7 s, 2 commitments, widths 33 + 10, 1,522 public words. The fixture is `~/.cache/e5-wrap/fixtures/lfr-e3.json`. The `decode` section holds the stock `iface::decode` verdict on the honest words and on 10 changed word vectors (`lfr-e3-decode.json`). Script: `export-e3.sh`.

**Decoder (`src/decode.rs`).** It is a port of `iface::decode`, and every check is a constraint:

1. Unpack. Each packed Fr is the sum of its 31-bit words (8 x 31 = 248 bits, so the sum cannot wrap, and high bits are refused). A limb word must be `< 2^30`; every other word must be `< P`.
2. `dec`. Each of the 1,026 lanes and 256 logged values is 9 limbs. `Canon` is a borrow chain of `(r - 1) - L`: 9 range checks below `2^30` and 8 borrow bits, with the last borrow fixed at 0.
3. Entries. The op is a one-hot over the 7 admitted codes. The bounds are `param - lo` and `hi - param` `< 2^11`. A used entry after a `NOP` is refused by a monotone `NOP` bit. The value counts and permutation counts accumulate (`ceil(param / 2)` from `2q = param + e`), and the capacity `off_n <= V`, `np <= P` is checked. Each value after `off_n` is zero (a monotone bit vector with count `V - off_n`).
4. Lane map, in stock order. A pair `(0, 0)` is skipped by an `is_zero` on `lane + 1026 ent`. Otherwise one-hots over the 1,026 lanes and 256 entries check that the entry is a `DIGEST`, that `s[lane] == 0`, and then set `s[lane] = vs[off + 1]`.
5. `PUB`. One log-derivative lookup holds the tables `(j, vs_j)` and `(256 + l, s_l)`. Here `s` is taken after the lane map. Each `PUB` entry queries `(off_i, v_i)` and `(256 + param_i, v_i)`, and each lane-map pair queries `(off + 1, val)`. The 1,282 multiplicities and the 260 queried values are observed into the wrap transcript after `part4`. The two challenges come after them. The `L_Fr` public values were observed earlier, in the preamble.

**Unit costs (MEASURED, cost model, `nr_pow2range_cols` 1).** A range check below `2^30` costs 3 rows with its assign; below `P` (not a power of two), 6 rows; a bit, 1 row. With 4 range columns the costs are 2 and 5 rows. The 4-column option is not used, because it changes the arch.

| Check (I7, `i7.sh`, log `i7.log`) | Result |
|---|---|
| Native verdict against stock `iface::decode`: honest, `free_lane_changed` (accepted); `pub_lane_changed`, `value_limb_out_of_range`, `lane_not_canonical`, `word_not_babybear`, `op_not_admitted`, `param_out_of_bound`, `used_entry_after_nop`, `lane_map_not_digest`, `packed_high_bits` (refused) | 11 of 11 equal; honest statement equal to stock |
| Gadget decoder alone (1,522 assigned words, own transcript) | 162,381 rows |
| `MockProver`, decoder alone, k = 18 | honest OK; changed statement, `PUB` mismatch, non-canonical lane refused |

| Full wrap on `lfr-e3` (I8, `i8.sh` and `i6.sh lfr-e3 21 1`, logs `i8.log`, `i8-excl.log`) | Result |
|---|---|
| Native verifier core | 649 of 649 stock samples equal; 51,497 permutations; 15 traces, n per trace [17,17,14,14,13,12,11,11,11,11,9,8,7,7,5]; queries [98,83,82]; changed preprocessed row, changed smallest-trace opening, changed public value refused |
| Rows, k = 21 (E3 map) | 1,605,413 (gate 1,782,573; 90.1 percent; headroom 177,160); table rows 1,256,451; 8 advice, 27 fixed columns |
| Verifier core rows (DERIVED: total minus the decoder alone) | about 1.44 M, the same as `lfr-real` (1,436,529) |
| `MockProver` | honest OK (44.8 s); changed public input and changed proof value refused |
| Exclusive slot (`heavy-slot --exclusive`) | keygen 82.6 s; blinded prove 131.6 s (OsRng); proof 2,912 bytes; key 1,325 bytes (`MidnightVK`); library verify OK |
| Stock ledger 9.1.0.1 (`ledger-check`) | `Ok(())` in 0.062 s, 1,026 public inputs; 3 changed public inputs and 3 flipped proof bytes refused |

**Recommended options (for owner confirmation).**
- The scope: `decode()` is ported exactly, with no check dropped. The native-Fr replay of the log values (E3 J6) is not in the wrap; it stays open.
- The `L` vk pre-hash (`L_Fr` public value 0) is pinned as a key constant, so the wrap key commits to one `L` key.
- The statement head words (0 to 113, other than the lane map) are range checked only (`< P`). Nothing reads them yet; that is open.

## 6j. I9: the ENC wrap-side vectors (MEASURED)

Source: `~/Projects/Moriarty-wt/enc-fp3/research/sherlock-enc-fp3-20261112/VECTORS.txt` (55 pinned values) and `fp3-closure/VALUES.txt` (FP-3 collection). Run: `export-enc.sh` (openvm `e5-wrap`, example `enc_wrap`, log `export-enc.log`), then `i9.sh` (tests `i9_enc` and `i7_decode`, log `i9.log`). The circuit is unchanged; only `dec` and `r_minus_1_limbs` are made public, and the test relation moves to `src/decode_test.rs`.

**Part A, Fr vectors on the wrap gadgets** (`i9_enc_fr_vectors`). These are the wrap's own `Alg` code: `merkle::hash_slice` and `compress` (u = 0, 1, 2, 4), `permute`, the `pack` of BabyBear words (each `< P`), `decode::dec` (9 x 30-bit limbs with `Canon`), and the `L_Fr` transcript (u = 7). There are 40 checks. All of them equal their pinned values natively, and `MockProver` accepts them at k = 15. The test changes 4 pinned values (the first, the fifth, the middle one and the last), and each change is refused.

| Group | Checks |
|---|---|
| ENC-1 tags (rule P-1, key constants) | `enc1.tag[0..3]` |
| `DIGEST` = `PoseidonFr_4(tagFr, pack(d2))` | `enc1.C_P`, `enc1.A_1`, `enc1.input_pub`, `enc1.outcome_pub` (from the pinned `d2_progc`, `d2_lock_j1`, `input_d`, `outhide_d2`), `enc1.digest_1to8` x 4 |
| Replay of the ENC-2 log from its pinned limbs (`enc2.enc[0..5]`) | `DIGEST(0)`: value 0 = `pack(d2_progc)`, output = replay = `C_P`; `H2`: output = replay = `u0.h2(3,5)`; `SPONGE(3)`: output = replay of the logged inputs = `SPONGE(3)(3, 5, 7)`; `PERM`: outputs = permutation of the inputs; `PUB(1)`: 0x1234; `DIGEST(2)`: value 0 = `pack(input_d)`, output = replay = `input_pub` |
| `Canon`, limbs | `enc2.limbs(r-1)` decodes to `r - 1`; the limbs of `r` are refused (Canon), and a limb `2^30` is refused (range) |
| Fr Merkle hashes | `g3.L.u1_leaf20` (u = 1, 3 packed words), `g3.L.u2_node(7,9)` (u = 2) |
| `L_Fr` transcript (u = 7) on the wrap `Transcript` | `g3.Lfr.s2`, `g3.Lfr.fresh3`, `g3.Lfr.obs1_sample`, `g3.Lfr.obs2_sample3` |
| FP-3 collection | `coll.run[0..2].claim_pub` (u = 0 `SPONGE(3)` over `C_P`, `input_pub`, `outcome_pub`), `coll.set_pub` (`SHK_SET`), `parity.u0[0,1,2,3,5,16]` |

**Part B, `pv` binding and the lane map** (`enc_wrap`, `i9_enc_pv_lane_map`).
- `enc_wrap` builds the `L` words. Their `pv` is the pinned ENC-2 log (6 entries, 18 values), and the head is zero except the lane map. The lane map has two pairs: lane 2 takes `C_P` from `DIGEST(0)`, lane 3 takes `input_pub` from `DIGEST(2)`. Lane 1 holds the `PUB(1)` value 0x1234.
- Checked against the pinned values: `D_EMPTY` (this confirms the BabyBear permutation is ENC's), `enc2.pv[0..12]` and `enc2.pv.p2_fingerprint` of the same 2,816 `pv` words, plus `limbs(r-1)`.
- The wrap test unpacks the `L_Fr` words and finds the pinned `pv`. On all 12 cases the wrap decoder gives the stock verdict and the stock statement. The honest statement has lane 2 = `C_P`, lane 3 = `input_pub` and lane 1 = 0x1234.
- `MockProver` (k = 18) accepts the honest case and refuses `pub_lane_changed` and `lane_map_to_sponge`.

| Case | Stock D-9 | Wrap |
|---|---|---|
| honest; `lane_r_minus_1` | OK | OK, the same statement |
| `digest_output_changed` (the logged `C_P` changed) | OK (no `DIGEST` replay in D-9) | OK, lane 2 differs from honest |
| `pub_lane_changed`, `pub_value_changed` | refused (`PUB(1)`) | refused (lookup) |
| `lane_map_to_sponge`, `lane_map_to_pub`, `lane_map_to_nop`, `lane_map_taken_lane`, `lane_map_same_lane_twice` | refused (lane map) | refused |
| `lane_r`, `value_limb_2_30` | refused (range or Canon) | refused |

**Not on the wrap side (22 of 55).** These are the BabyBear `P2` forms, which the wrap never computes: `enc1.VAL_NONE`, `enc1.outcome_d`, `enc1.claim_d`, `enc2.onion6`, `enc2.node(2,4)`, `enc3.layout_d`. Also the 16 `L` transcript vectors (u = 3: `g3.L.s2`, `fresh6`, `obs1..8_sample`, `obs16_sample2`, `obs3_commit_sample2`, `sample_obs_sample`, `prehash_sample`, `obs9_commit_sample`, `samples11`). The `L` verifier runs inside `L_Fr`; the wrap verifies `L_Fr` with the u = 7 transcript, so the u = 3 samples are an AIR-side obligation (ENC section 10 covers them on `TapeTranscript`).

**Findings (for owner and lead confirmation; recommended options marked).**
1. **The lane map admits only `DIGEST`, but ENC's `acceptRun` lane map has `SPONGE` pairs.** ENC (section 10) maps lanes 11 and 25 to a `SPONGE(3)` entry (`claim_pub`). D-9 (`iface::decode`) admits only `DIGEST` entries in the lane map and reads `vs[off + 1]`, and `lane_map_to_sponge` is refused on both sides. Recommended: E3 extends D-9 so a lane-map pair may name a `SPONGE(n)` entry and read its output `vs[off + n]`; the wrap then ports the change (a one-hot over two ops and an offset by `vals - 1`). The wrap keeps the stock D-9 rule until E3 changes it.
2. **E3 `iface` uses placeholder tags.** `iface::tag_fr(k) = k + 1` (plan D-8), not the ENC-1 ASCII rule P-1, so E3's logged `DIGEST` outputs differ from the ENC values. The wrap tests use P-1. Recommended: E3 adopts P-1 before the J6 replay.
3. **No `DIGEST` replay in the wrap (E3 J6 open).** The replay gadgets match the pinned vectors (part A), but the wrap does not yet replay every logged entry, so a changed logged output decodes into a changed statement (`digest_output_changed`). Recommended: J6 adds the per-entry replay in the wrap.
4. **`Canon` on `X`.** The architecture (`FD/ARCHITECTURE.md`, wrap checks) asks for range and `Canon` on the limbs of `X`. `X` (head words 97 to 105) is not fixed in E3, and its honest head words are random BabyBear values, so the wrap checks only `< P` there. The `dec` gadget that would check `X` passes the `Canon` vectors (part A). Recommended: add `Canon` on `X` when E3 fixes `X`.

## 6k. Gemini Quint lane on the decoder circuit (MEASURED by the lead)

The lane was `argon-run` (gemini-4-argon-high) in `~/.cache/gemini-lanes/e5-decode` with the quint-modeling and quint-lang skills (prompt `qa/gemini-quint/e5-decode/PROMPT.md`). Its report, model and the lead's runs are in `qa/gemini-quint/e5-decode/` of this worktree. The lane's verdict: SOUND on all 7 questions (unpack, Canon, entries, zero padding, lane map, PUB lookup, output), with no defect found. The model is reduced in size: F_359, 3 limbs of 3 bits, 2 words of 4 bits per packed element, B = V = 4, 3 lanes, 2 lane-map pairs. The lookup is modelled as the per-key weight equality that Schwartz-Zippel gives after the transcript observes everything.

The lane's sandbox refused every command, so the lead ran quint:
- 4 model fixes first: the typecheck, `mod_inv(1) = 360`, a `mut_skip_any_pair` counterexample that does not separate, and a fuzz step for `quint run`.
- `quint typecheck` OK on both files; `quint test` 13 of 13 pass.
- `quint run --invariant SOUND_AND_COMPLETE`: no violation in 10,000 traces (seeds 1 and 2, 5,000 samples, 10 steps).
- Each of the 8 mutants is violated: `no_canon_last_borrow`, `lo_only`, `nop_chain_off`, `g_not_monotone`, `skip_any_pair`, `no_taken_check`, `s_before_lanemap`, `v_s_keys_overlap`.

One fact came out of fix 3. `z u = 0` in the lane-map skip is not redundant: without it a prover takes `nz = 2` and writes one value into two lanes. The gated one-hot alone handles only `nz = 0`. The circuit has the check, so nothing changes. No defect means no circuit change, so sections 6i and 6j stand.

## 6l. The decoder on E3 9539418 (MEASURED unless marked)

**Port.** openvm `e5-wrap` is rebased onto E3 `e3-iface` 9539418: c1ddfc0, 8ce7774, 6ef8f47, 1f42114, then 2f1d8a4 (the export changes). `lfr_e3` now reads the E3 J8 fixture (`~/.cache/e3-iface/fixture-j8`; the `L` that E3 used at 9539418) and passes the `L` public values to `gen_r_check`. Script: `export-r2.sh`, log `r2-export.log`. Keygen took 334.4 s and prove 972.3 s (shared slot). The proof has 2 commitments, widths 33 + 10, and 1,522 public words. The decode cases grew from 11 to 30. The old fixtures are kept in `~/.cache/e5-wrap/fixtures/at-2f09caf/`.

**Decoder changes (`src/decode.rs`), from `iface.rs` 9539418:**
1. Lane map: the entry must be a `DIGEST` or a `SPONGE(3)`. The circuit checks `sel_dig + sel_sp == nz` and `sel_sp (p - 3) == 0`. The output is `vs[o + 1 + 2 sel_sp]`: `o + 1` for a `DIGEST`, `o + 3` for a `SPONGE(3)`.
2. A lane is set at most once (lane `e3-lanemap` finding 2a). For each earlier active pair, `nz_i nz_j ((lane_i - lane_j) inv - 1) == 0`. The `s[lane] == 0` check stays.
3. The 8 lane-map words are range checked `< 2^16` (finding 3). The gated one-hots already bound them; the checks keep the stock refusal.
4. Each mapped entry is replayed natively. A `DIGEST` uses the tag from a gated one-hot of `param` over the 6 tags; a `SPONGE(3)` uses the u = 0 sponge. Its inputs are read through the same lookup, and the reads at `o + 1` and `o + 2` are gated by the `SPONGE` bit.
5. `N_TAGS` is 6, and `tag_fr` follows the ASCII rule P-1.
6. New step 4, at static positions:
   - `mode` is a bit, and the `j`, `k` words are u16.
   - For a chunk, `1 <= j <= k`.
   - The `R` entries have fixed `(op, param)`, with `t_in = mode [j != 1]` and `t_out = [j != k]`.
   - The anchor entry words and the `claim_pub` inputs are checked.
   - Entries 0 to 4 (5 for a chunk) are replayed natively.

   `L_Fr` already checks `R` in its tape (`gen_r_check`, E3 J7). The wrap repeats the check so that its verdict equals stock `decode`.

**Open (recommended option, FOR OWNER CONFIRMATION).** Stock `decode` replays every entry (`PERM`, `H2`, `SPONGE`, `DIGEST`). The wrap replays only `R` and the mapped entries, so a wrong output in any other entry is accepted by the wrap and refused by stock. This is E3 J6 in the wrap. Such an output does not reach the statement: only mapped entries and `PUB` values reach it, and both are checked. ESTIMATE for a full replay through a 256-slot permutation tape with the sponge state carried between slots: about 15,000 to 25,000 rows. That fits the headroom below.

| Check | Result |
|---|---|
| I7 native vs stock (`r2-i9.log`) | 27 of 30 cases equal, honest statement equal. Accepted: `honest`, `free_lane_changed`, `tail_honest`, `lane_map_digest_reg`. Refused by both: `mode_2`, `jk_not_u16`, `chunk_j_0`, `anchor_word_e_in`, `r_entry_0_tag`, `claim_pub_input_changed`, `r_anchor_output_changed`, `r_sponge3_output_changed`, `lane_map_sponge_not_3`, `lane_map_perm`, `lane_map_digest_reg_output_changed`, `lane_map_not_u16`, `lane_map_twice`, `lane_map_twice_zero_output`, and the 11 earlier refusals. Open, as planned (stock refuses, wrap accepts): `open_replay_perm`, `open_replay_sponge`, `open_replay_digest` |
| Mapped twice with a first output of 0 | stock `apply_lane_map` alone refuses; the wrap records `lane map pair: lane mapped twice` (whole stock decode refuses earlier, at the replay) |
| Decoder alone, cost model k = 18 | 165,168 rows (was 162,381; +2,787) |
| `MockProver`, decoder alone | honest OK; changed statement, `pub_lane_changed`, `lane_not_canonical`, `lane_map_sponge_not_3`, `lane_map_twice`, `lane_map_not_u16`, `r_anchor_output_changed`, `chunk_j_0` refused |
| I9 Part A | pass (unchanged) |
| I9 Part B (`enc-wrap-decode`) | The ENC-2 log is not an `R` log (entry 1 is `H2`), so stock and the wrap refuse all 12 cases at `R entry 1`. The lane-map step alone gives the same verdict as the wrap's lane-map checks on 10 cases (2 fail `dec` first), and the same statement: lane 2 = `C_P`, lane 3 = `input_pub`, lane 1 = 0x1234. A changed `C_P` is refused by the replay of the mapped `DIGEST`. `MockProver` refuses the honest vector (recommended option, FOR OWNER CONFIRMATION: the ENC vectors stay pinned at Part A and at the lane-map step) |
| I8 native verifier core (`r2-i8.log`) | 675 of 675 stock samples equal; 51,547 permutations; queries [98, 83, 82]; 3 refusals |
| Full wrap rows, k = 21 | 1,610,038 (gate 1,782,573; 90.3 percent; headroom 172,535); table rows 1,260,547; 8 advice, 27 fixed columns |
| `MockProver`, full wrap | honest OK (44.6 s); changed public input and changed proof value refused |
| Exclusive slot (`r2-i6-excl.log`) | keygen 83.7 s; blinded prove 138.5 s; proof 2,912 bytes; key 1,325 bytes; library verify OK |
| Stock ledger 9.1.0.1 | `Ok(())` in 0.077 s, 1,026 public inputs; 3 changed public inputs and 3 flipped proof bytes refused |
| Gemini Quint lane round 2 (`qa/gemini-quint/e5-decode-r2/`) | The `argon-run` lane gives SOUND and COMPLETE on 7 questions (op test, output index, mapped twice, u16, mapped replay, step 4, output), with no defect apart from the open replay. The lane sandbox refused quint, so the lead ran it with no model fixes: typecheck OK; 14 of 14 tests pass; `SOUND_AND_COMPLETE` has no violation in 10,000 traces (seeds 1 and 2); all 8 mutants are violated (`no_sponge3_param`, `idx_plus1`, `no_twice`, `no_u16`, `no_mapped_replay`, `t_in_wrong`, `no_claim_pub`, `no_chunk_range`) |

## 7. Soundness points (as built)

- Direction bits of a Merkle path are the low-bit cells of the sample cell (`merkle_verify` takes them from `low_bits`), not separate witnesses.
- `low_bits` is a canonical full decomposition, so `x + r` is refused.
- The PoW check asserts each sampled low bit is 0 (`assert_bit_false`).
- In I1 and I2 the roots and samples are witness cells (test relations). In the full circuit they come from the transcript (samples) and from the vk constants or the observed commitments (roots).
- The shape pin. `Shape::from_wrapin` takes the parameters, the DAGs, the interactions, the widths, the preprocessed commitments and the pre-hash from the `L_Fr` key. It takes the trace heights, the presence of optional AIRs and the cached-commitment counts from the fixture proof's `trace_vdata`, and the stacked layouts from `verify_proof_shape` on that proof. These are fixed for the `L_Fr` machine (E2 pins the tape shape), so they are circuit constants: a proof of another shape is refused, not verified. The wrap key is therefore tied to one `L_Fr` key and one `L_Fr` height profile.
- `omega^index` and `omega^-index` in `binary_k_fold` are products of key constants selected by the index bit cells, so no inversion is witnessed.
- A codeword round's Merkle root is the commitment cell observed in the round before; the round-0 roots are the observed common-main and cached cells and the key's preprocessed constants.
- The statement. On a fixture with a `decode` section (E3 map), the public inputs are the 1,026 lanes that the in-circuit decoder computes from the `L_Fr` public-value cells, the same cells observed in the preamble (section 6i). The lookup challenges are sampled after its multiplicities and queried values are observed. On older fixtures it is `PublicMap` (LEAD): the public-value cells, then constant-zero cells.

## 8. Next increments (for the next agent)

1. **I3 (done).** Write `gkr.rs` over `Alg`, a port of `verifier/fractional_sumcheck_gkr.rs` (observe `q0_claim`, layer claims, cubic interpolation at 0..3, the `eq` update, `mu` and `lambda` samples), and the zerocheck rounds of `batch_constraints.rs` steps 1 to 6 (LogUp PoW, `alpha`, `beta`, `xi`, `lambda`, `mu`, the univariate round with the `step_by(2^l_skip)` sum, the multilinear rounds with constant inverse factorials). Test: the native back end on `mix-small` and `mix-lfr` reproduces every recorded sample in order up to the column openings, and the final GKR claims; refused: a changed layer claim, a changed round polynomial. `MockProver` once on `mix-small`.
2. **I4a, I4b (done); was I4b: `stacked_reduction.rs`, written in `src/stacked.rs` (register it in `lib.rs`, make `eq_uni` pub and add `eq_uni_at_one` in `verifier.rs`, test as I4a: native final sum, the first WHIR sample equal, negatives on stacking openings and round polys).** The DAG evaluator (`verifier/evaluator.rs`: nodes in order, `is_first_row`, `is_last_row`, `is_transition` at the lifted point with the `n < 0` case), interactions (message by powers of `beta`, bus index plus 1), `eq_3b`, `eq_ns`, `eq_sharp_ns`, the final `evaluated_claim`; then `stacked_reduction.rs`. The vk DAG is in `wrapin.vk.inner.per_air[i].symbolic_constraints` (serde form of `SymbolicConstraintsDag`).
3. **I5 (done).** `whir.rs`: `mu` PoW and batching, folding sumchecks, OOD, query PoW and indices (`sample_bits(log_rs - k_whir)`), round-0 openings per commitment (the I2 code), `binary_k_fold` (twiddles and `1/2` are key constants; `omega^index` from the index bits as `prod (1 + b_j (w^(2^j) - 1))`), `gamma`, the final check.
4. **I6.** Full relation: the shape from the `L_Fr` key (E2 I10 key; export it with `sherlock-wrap-export` from the E2 machine proof, as a third fixture), public inputs by `PublicMap` (PLAN section 4, LEAD), `MockProver` rows, keygen at the smallest k up to 21, key bytes, blinded proof, `ledger-check`.

## Log

- 2026-11-12 13:12. Worktree `e5-wrap` from `e2-lfr` 6fc8e3c21; openvm worktree `openvm-e5` on `e5-wrap` from d46a137.
- 2026-11-12 14:35. First `mix-lfr` export stopped (single-threaded, no `parallel` features); fixed and rerun (320.9 s).
- 2026-11-12 15:05. I0 to I2 pass (log `i0c.log`); ledger 9.1.0.1 accepts the smoke wrap.
- 2026-11-12 17:33. I3 and I4a pass (log `i4.log`; the job waited about 2 hours for the heavy lock).
- 2026-11-12 17:52. I4b and I5 pass (log `i5.log`); the whole verifier reproduces every stock sample on both fixtures. Builds move to `heavy-slot` (owner rule change). Openvm `e5-wrap` rebased onto E2 `c4bd312` for the `lfr-real` export.
- 2026-11-12 18:45. I6a passes on `mix-small` (log `i6-small.log`): ledger 9.1.0.1 accepts the whole-verifier wrap.
- 2026-11-12 19:39. I6b passes (log `i6-real.log`): 1,592,979 rows at k = 21; ledger 9.1.0.1 accepts the wrap of the real `L_Fr` proof. E5 gate PASS.
- 2026-11-12 19:40. Lever L1 (fused multiply-add): 1,453,425 rows on `lfr-real`, all I3 to I5 tests pass (log `lever-fma.log`).
- 2026-11-12 20:37. Lever L2 (`omega^index`): 1,436,529 rows on `lfr-real`, all I3 to I5 tests pass (log `lever-omega.log`).
- 2026-11-12 21:56. Final circuit (L1 + L2) under `heavy-slot --exclusive`: 1,436,529 rows, keygen 84.6 s, prove 183.8 s, ledger 9.1.0.1 accepts (log `i6-excl.log`).
- 2026-11-12 23:36. Openvm `e5-wrap` rebased onto E3 2f09caf; `lfr-e3` exported (log `export-e3.log`).
- 2026-11-12 23:54. I7 passes (log `i7.log`): decoder verdicts equal stock on 11 cases; gadget decoder 162,381 rows.
- 2026-11-13 00:10. I8 cost model: 1,605,413 rows on `lfr-e3` with the E3 map (log `i8.log`); commit 7a4dfb308.
- 2026-11-13 00:49. I8 under `heavy-slot --exclusive`: keygen 82.6 s, prove 131.6 s, ledger 9.1.0.1 accepts (log `i8-excl.log`).
- 2026-11-13 03:20. I9 passes (logs `export-enc.log`, `i9.log`): 40 Fr checks and 12 decoder cases on the ENC `pv` vector equal the pinned and stock values.

- 2026-11-13 03:50. Gemini Quint lane on `decode.rs`: lane verdict SOUND; the lead ran quint: 13 of 13 tests, SOUND_AND_COMPLETE holds on 10,000 traces, 8 of 8 mutants caught (`qa/gemini-quint/e5-decode/`).
- 2026-11-13 05:25. Openvm `e5-wrap` rebased onto E3 9539418; `lfr-e3` re-exported from the J8 fixture (log `r2-export.log`); I7 and I9 pass on the ported decoder (log `r2-i9.log`).
- 2026-11-13 05:35. Full wrap on the new `lfr-e3`: 1,610,038 rows at k = 21; exclusive keygen 83.7 s, prove 138.5 s; ledger 9.1.0.1 accepts (logs `r2-i8.log`, `r2-i6-excl.log`).
- 2026-11-13 05:55. Gemini Quint lane round 2 on the ported decoder: SOUND and COMPLETE; lead runs pass, 8 of 8 mutants caught (`qa/gemini-quint/e5-decode-r2/`).
