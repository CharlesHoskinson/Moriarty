# E5 results: the one BLS12-381 wrap that verifies `L_Fr` natively in Fr

Plan: `PLAN.md` beside this file. Owner ruling 2026-11-12: OD-24 is option E.

## 1. Status

**E5 gate: NOT REACHED.** I0 to I5 pass their test gates: the whole `L_Fr` verifier runs over `Alg`, and on both fixtures the native run reproduces every stock sample (252 of 252, 402 of 402) and refuses every changed proof part tried. The stock ledger 9.1.0.1 (`a2b30128`) reads a key built with the vendored permutation access and accepts a blinded proof with 1,026 public inputs (MEASURED, section 3). I6a passes: the whole-verifier wrap of `mix-small` is accepted by the stock ledger 9.1.0.1 with a 1,325-byte key and 1,026 public inputs (section 6e). I6b, the same run on the real E2 I10 `L_Fr` proof (fixture `lfr-real`, export queued), decides the gate rows.

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
| I6b | The same on the real `L_Fr` proof (`lfr-real`, widths 33 + 10), rows within the k = 21 gate | export queued | section 8 |

## 2. Code

| Repository (not pushed) | Branch | Content |
|---|---|---|
| Moriarty `~/Projects/Moriarty-wt/e5-wrap` | `e5-wrap` (from `e2-lfr` 6fc8e3c21) | `tools/sherlock-wrap`: the wrap crate in its own workspace and lock; `tools/sherlock-wrap/ledger-check`: the stock ledger 9.1.0.1 check in a second workspace; this directory |
| openvm `~/.cache/m7-wrap/src/openvm-e5` (a git worktree of the openvm clone, so the `../openvm-org_stark-backend` path patch resolves) | `e5-wrap` (from `e2-lfr` d46a137) | crate `crates/sherlock-wrap-export`: the exporter (`src/lib.rs`) and the fixtures (`examples/fixtures.rs`) |

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
| Keygen (ceremony SRS `bls_midnight_2p20`), prove (`OsRng`, Blake2b) | 51.0 s and 70.0 s; proof 2,912 bytes; library verify accepts |
| Key bytes in the `MidnightVK` framing | 1,325 |
| Ledger 9.1.0.1 `VerifierKey::verify` (stock crates, `ledger-check`) | `Ok(())` in 0.077 s; a changed public input refused (first, middle, last of 1,026); a flipped proof byte refused (first, middle, last) |

The key size does not depend on the circuit beyond the architecture and k, so the 50,000-byte limit holds with a wide margin at any k up to 21 (DERIVED from the I0 and I6a keys, both 1,325 bytes).

## 6f. Row budget at the measured `L_Fr` shape (DERIVED)

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

## 7. Soundness points (as built)

- Direction bits of a Merkle path are the low-bit cells of the sample cell (`merkle_verify` takes them from `low_bits`), not separate witnesses.
- `low_bits` is a canonical full decomposition, so `x + r` is refused.
- The PoW check asserts each sampled low bit is 0 (`assert_bit_false`).
- In I1 and I2 the roots and samples are witness cells (test relations). In the full circuit they come from the transcript (samples) and from the vk constants or the observed commitments (roots).

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
