# E2 results: the Fr recursion AIR set `L_Fr` that verifies one `L` proof

Plan: `PLAN.md` beside this file. Owner ruling 2026-11-12: OD-24 is option E; E1 passed.

## 1. Status

**E2 gate: NOT REACHED.** Increments I0 to I8 of `PLAN.md` section 4 pass their test gates. Every program is in one shared program table, and an `L_Fr` proof has 2 commitments, MEASURED (I5c, owner ruling 2026-11-12, `PLAN.md` D-4). The tape generator runs the `L` verifier on the real `L` proof through the preamble, LogUp GKR, the batched zerocheck with the DAG of all 39 `L` AIRs, and the stacked reduction (I6 to I8): 17,299 of the 18,772 transcript events of one `L` verification equal the stock record, every check of these stages is a tape assertion, and the tape proves with 2 commitments. I9 (WHIR) is next.

| # | Increment | Status | Evidence |
|---|---|---|---|
| I0 | Fixture: the real `L` proof, `L` vk, node vk on disk | PASS | section 3 |
| I1 | `L_Fr` hash tags u = 5, 6, 7 (D-3) | PASS | section 4 |
| I2 | `BbAlu` and `Range16`: BabyBear and `EF4` add, sub, mul emulated in Fr | PASS | section 5 |
| I3 | `PoseidonFr` AIR (Midnight Poseidon, one row per permutation) | PASS | section 6 |
| I4 | Tape, write-once memory bus, `InputAir`, `BbAluProgAir` | PASS | section 7 |
| I5 | Transcript of `L` on the tape: `FrInput`, `PoseidonProg`, `Pack`, `Split`, `Reshape` | PASS | section 8 |
| I5b | The stock verifier's transcript on the real `L` proof, on the tape | PASS | section 8a |
| I5c | D-4: one shared `ProgramTable` AIR for every program; 2 commitments | PASS | section 8b |
| I6 | Tape generator part 1: shape pin, preamble, LogUp PoW (`LowBits`), LogUp GKR, GKR claim checks (`AssertEq`) | PASS | section 8c |
| I7 | Part 2: batched zerocheck, the DAG of the 39 `L` AIRs, interaction evaluations | PASS | section 8d |
| I8 | Part 3: stacked reduction | PASS | section 8e |
| I9, I10 | WHIR (Merkle paths, queries, folds, final polynomial), then the gate | not started | section 10 |

Route (`PLAN.md` section 2): a dedicated verifier of the `L` vk with the `L` shape pinned in the `L_Fr` key (LEAD D-1), run as a straight-line tape over generic Fr AIRs. Not a port of OpenVM's recursion crate.

## 2. Code and commits

| Repository (all not pushed) | Branch | Commits |
|---|---|---|
| backend `~/.cache/m7-wrap/src/openvm-org_stark-backend` | `e2-lfr` (from `e1-fr-base` 3040761) | e8223ed I1 domain tags; 5f698b5 per-bus bound fix (section 9) |
| openvm `~/.cache/m7-wrap/src/openvm-org_openvm` | `e2-lfr` (from `main` 05f3b73) | 3cc1849 the M7 bench crate `stark-bench-ovm` (the existing uncommitted changes); e8fdf87 `sherlock-lfr` with I0, I2 to I5; 96a1897 I5b; 02bb7ff I5c program table; be38351 I6; e43c763 I7; db8bbd5 I8 |
| Moriarty `~/Projects/Moriarty-wt/e2-lfr` | `e2-lfr` | this directory |

**2a. Crate `crates/sherlock-lfr`** (openvm workspace; dependencies only from the openvm lock: the lock adds the crate itself and no other package, MEASURED by `git diff Cargo.lock`).

| File | Content |
|---|---|
| `src/fixture.rs`, `examples/l_fixture.rs` | I0: the M7 chain, write and load of the fixture |
| `src/bb.rs` | BabyBear and `EF4` integers (`p = 2^31 - 2^27 + 1`, `x^4 = 11`) |
| `src/range.rs` | `Range16Air` |
| `src/alu.rs` | `BbAluAir`, the canonical gadget |
| `src/poseidon.rs` | `PoseidonFrAir`, constants of midnight-circuits 7.3.0 |
| `src/tape.rs` | `Tape`, `Val`, `Op`, `InputAir`, `BbAluProgAir`, the memory bus |
| `src/fr.rs`, `src/machine.rs` | `Pack`, `Split`, `FrInputAir`, `PoseidonProgAir`, `PackAir`, `SplitAir`, `ReshapeAir`; `Machine` (all AIRs of one tape) |
| `src/transcript.rs` | `TapeTranscript`: the `L` transcript on the tape |
| `src/lverify.rs` | I6 to I8: the tape generator (`LShape`, `Gen`, `gen_part1`, `gen_part2`, `gen_part3`) |
| `src/record.rs` | the recording wrapper around the stock `L` transcript (`stock_log`) |
| `src/program.rs` | I5c: `ProgramTableAir` (the only preprocessed trace), the program bus, the AIR tags |
| `src/testing.rs` | small Fr parameter set; prove, verify and refusal helpers; `Range16` fill from the logical interactions; commitment count of a proof; per-bus balance check |
| `tests/i0_fixture.rs` to `tests/i5_transcript.rs`, `tests/i5_l_transcript.rs`, `tests/i6_part1.rs`, `tests/i7_part2.rs`, `tests/i8_part3.rs` | the gates |

Saved beside this file: `backend-e2.diff` (backend `git diff 3040761 5f698b5`), `sherlock-lfr.diff` (openvm `git diff 3cc1849 db8bbd5`), `run.sh` and `i0.sh` (the runners).

Runner: `~/.cache/e2-lfr/run.sh <stage> <mem> <dir> cmd...` (`flock ~/.cache/cost-opt/measure/heavy.lock heavy-run --mem <mem> --name e2-<stage>`, `CARGO_TARGET_DIR=~/.cache/e2-lfr/target`, `RUSTFLAGS=-C target-cpu=native`, logs `~/.cache/e2-lfr/logs/`). I0 ran as the systemd user unit `e2-i0` (`~/.cache/e2-lfr/i0.sh`).

Test parameters (`testing::small_params`): `L_Fr` domains (u = 5, 6, 7), UD, log blowup 1, `l_skip` 2, `n_stack` 14 (stacked height 2^16, so the 2^16-row range table fits), `k_whir` 4, query PoW 6, folding PoW 5, `mu` PoW 4, LogUp PoW 7, max constraint degree 3. A negative control counts as refused when the stock verifier returns an error or the prover stops on the witness (the error is printed in the log).

## 3. I0: the `L` fixture (MEASURED)

Example `l_fixture` (M7 `l_fr.rs` chain unchanged: two W1 proofs at app params, node `VerifierSubCircuit::<2>` at the OD-19 internal set, `L_cached` at the OD-19 `L` set with `BabyBearBls12381PoseidonCpuEngine`). Directory `~/.cache/e2-lfr/fixture/`: `l_proof.bin` (stark-backend codec), `l_vk.bitcode`, `node_vk.bitcode` (bitcode serde), `l_fixture.json`.

| Quantity | Value |
|---|---|
| `L` proof | 430,073 bytes (equal to M7 2b.3) |
| `L` vk / node vk | 80,439 / 79,145 bytes (bitcode) |
| `L` AIRs, heights | 39; log heights equal D10's `D10_L_LOG_HEIGHTS` AIR by AIR |
| `L` vk pre-hash | Fr `50455319171124432299989667883588395107320244478262132930075171308163370797377` |
| `L` prove | 292.9 s CPU, 20 threads, load 40 to 50 (other jobs on the host) |
| Verify from disk | accepts, 1.30 s |
| Test `i0_fixture_verifies_from_disk` | accepts from disk; 8 of 8 single-byte flips refused (1 decoder, 7 verifier) |

## 4. I1: `L_Fr` hash tags (D-3, LEAD pending OD-18)

`FrDomains::l_fr()` = u 5 (leaf), 6 (node), 7 (transcript), constants `U_LFR_*`; type-level domain sets `DefaultDomains` and `LfrDomains`; engines `Bls12381FrPoseidon{Cpu,Ref}EngineAt<D>` with the aliases `Bls12381FrPoseidonCpuEngine` (u = 1, 2, 3, unchanged) and `Bls12381FrPoseidonLfrCpuEngine` (u = 5, 6, 7). File `stark-sdk/src/config/bls12381_fr_poseidon.rs`.

Test `stark-sdk/tests/e2_domains.rs`, 4 of 4 pass (MEASURED, log `fr-tests.log`):

| Test | Checks |
|---|---|
| `e2_lfr_domain_codes` | codes 5, 6, 7; distinct from each other and from u = 0 to 4; initial capacities distinct from `cap_fixed(u, l)` and `cap_var(u)` for u = 0 to 4 at 5 lengths; the engines carry their domains |
| `e2_lfr_domains_bound_both_directions` | a Fibonacci proof at u = 5, 6, 7 verifies there and is refused at u = 1, 2, 3, and the reverse; with one code of the three set back to u = 1, 2 or 3, the `L_Fr` proof is refused (each code is bound on its own) |
| `e2_lfr_domains_interactions` | a LogUp fixture at the `L_Fr` domains, CPU and reference engines |
| `e2_fr_per_bus_threshold` | section 9 |

The E1 tests still pass (9 of 9, 1 ignored). The E1 note stays: with u = 1 and u = 3, the `L_Fr` leaf hash and transcript would share their initial capacity with `L`'s; u = 5, 6, 7 removes that. Inside the tape, `L`'s own hashes keep u = 1, 2, 3.

## 5. I2: `BbAlu` and `Range16` (MEASURED)

`BbAlu` row (43 columns, 24 range lookups, degree 3): `e_k(a, b) = q_k p + c_k` per coefficient, `e_k` = sum, difference plus `p`, or the `x^4 = 11` convolution; `q_k` in three 16-bit limbs; `c_k = lo + 2^16 hi` with `lo`, `hi`, `0x7800 - hi` in the table and `hi = 0x7800 -> lo = 0` (is-zero gadget). Argument (DERIVED, `src/alu.rs` header): `e_k < 34 p^2 < 2^68` and `q_k p + c_k < 2^80`, both below `r`, so the Fr identity is the integer identity, and `c_k <= p - 1` makes `c_k = e_k mod p` unique. `Range16`: `value` 0, step 1, last row `2^16 - 1` (pins the height to `2^16`).

| Test | Result |
|---|---|
| `i2_alu_results_equal_p3` | 3,006 ops (edge cases and random `EF4` and BabyBear add, sub, mul): every result equals p3 `BinomialExtensionField<BabyBear, 4>` and BabyBear |
| `i2_alu_honest_proves` | 506 ops, 512 rows: accepted, proof 275,909 bytes |
| `i2_alu_negative_controls` | refused: non-canonical `c + p` with `q - 1` (prover, `NonZeroRootSum`); wrong quotient (verifier); wrong result (verifier); limb out of range (prover); `c = p` with 16-bit limbs `hi = 0x7800, lo = 1` (verifier) |

## 6. I3: `PoseidonFr` (MEASURED)

One row per permutation: `in[3]`, per S-box `t = x^2` and `y = t^2 x` (84 S-boxes: 8 full rounds of 3, 60 partial of 1), `out[3]`, `mult`; 175 columns, degree 3; linear layers are expressions, not columns.

| Test | Result |
|---|---|
| `i3_poseidon_equals_midnight` | 64 states (zero, small, `-1`, `cap_var(3)`, `cap_fixed(1, 36)`, 59 random): AIR trace output equals `MidnightPoseidonFr` (midnight-circuits 7.3.0 `permutation_cpu`) |
| `i3_poseidon_proves_and_refuses` | 64 permutations with a caller AIR on the Poseidon bus: accepted, 183,397 bytes; refused: changed output in the AIR and the caller (verifier), in the caller only (prover), a changed partial-round S-box cell (verifier) |

## 7. I4: tape and write-once memory (MEASURED)

Each tape value has one address and is written once; the writer sends `[addr, v0..v3]` on the memory bus with the read count of `addr` as multiplicity, each read sends it with count `-1`. Addresses, op kinds and multiplicities come from the program table (section 8b), so the key fixes the tape; only values are witness. The numbers in sections 7, 8 and 8a are those of the I5c tree (program table in place). `InputAir` checks each input coefficient canonical (and zero above coefficient 0 for a BabyBear input, and equal to the constant for a constant).

Test `i4_tape_executes_and_proves`: Horner of a degree-40 `EF4` polynomial plus a BabyBear constant (126 ops: 43 inputs and constants, 83 ALU rows); the executed result equals p3 `EF4`. Machine shape (rows padded / main width / preprocessed width): Input 64/30/0, BbAlu 128/47/0, ProgramTable 128/1/20, Range16 65,536/2/0. Accepted, 353,977 bytes, 2 commitments. Refused (prover, `NonZeroRootSum`): an ALU row that reads `a + 1` (its own identities hold), a program that reads an address no op writes (a changed key), a non-canonical input `v + p` used consistently by its readers, and the two I5c controls of section 8b.

## 8. I5: the `L` transcript on the tape (MEASURED)

`TapeTranscript` emits the ops of the stock `MultiFieldTranscript<BabyBear, Fr, MidnightPoseidonFr, 3, 2>` from `(0, 0, cap_var(u))`: overwrite absorbs move state addresses (no op), full blocks and squeezes add a `Perm` op, 8 observed words add a `Pack` op, a squeeze adds a `Split` op (5 digits), `observe_ext` a `Decompose`, `sample_ext` a `Compose`. New AIRs: `FrInputAir` (1 + 4 preprocessed columns), `PoseidonProgAir` (175 + 10), `PackAir` (9 + 18), `SplitAir` (52 + 12, 39 lookups), `ReshapeAir` (4 + 12).

**Split uniqueness** (DERIVED, `src/fr.rs` header): `x = sum d_i p^i + p^5 h` in Fr with canonical digits; `h` and `H - h` in 112 bits (`H = floor((r - 1) / p^5) < 2^101`), and when `h = H`, `L - low` in 160 bits (`L = (r - 1) - p^5 H`). Then the integer is at most `r - 1`, so it equals `x` and the digits are the stock digits.

Test `i5_transcript_equals_stock_and_proves`: a seeded schedule of 400 steps (observes, `EF4` observes, commitment observes, samples, `EF4` samples, re-observes of sampled digits): all 147 samples on the tape equal the stock `L` transcript (u = 3). Machine shape (rows padded / main width with the program columns / preprocessed width): Input 256/30/0, FrInput 64/5/0, PoseidonFr 128/185/0, Pack 128/27/0, Split 128/64/0, Reshape 64/16/0, ProgramTable 1,024/1/20, Range16 65,536/2/0. Accepted, 362,313 bytes, 2 commitments. Refused: the split witness of `x + r` (the Fr identity holds, asserted; verifier), a changed digit (verifier), a changed packed word (prover).

Final run of all `sherlock-lfr` tests on the tree of e8fdf87: 8 of 8 pass (I0 included with `--include-ignored`), log `~/.cache/e2-lfr/logs/tests-final.log`; `cargo clippy -p sherlock-lfr --all-targets -D warnings` clean. The I0 to I5 code went into one commit: the increments were written in order, and the commit was made after all their tests passed on the final tree.

## 8a. I5b: the real `L` transcript on the tape (MEASURED)

Test `i5_l_transcript_on_tape` (ignored by default, needs the fixture; log `i5b.log`): a recording `FiatShamirTranscript` wrapper around the stock `L` transcript logs every call while `verifier::verify` accepts the fixture proof. The log drives `TapeTranscript` with the observed values as tape inputs.

| Quantity | Value |
|---|---|
| Transcript events of one `L` verification | 18,772: 17,267 observes, 5 commitment observes, 1,500 samples |
| Samples equal on the tape | 1,500 of 1,500 |
| Tape ops | 1,192 `Perm`, 2,275 `Pack`, 362 `Split` |
| Machine shape (rows padded / main width with the program columns / preprocessed width) | Input 32,768/30/0, FrInput 8/5/0, PoseidonFr 2,048/185/0, Pack 4,096/27/0, Split 512/64/0, ProgramTable 32,768/1/20, Range16 65,536/2/0 |
| Prove and verify at the small Fr set | accepted, 19.3 s and 29.3 s in two runs (load about 20 to 30), proof 1,711,797 bytes |
| Commitments of the proof | 2 (`stacking_openings.len()`); 1 AIR with a preprocessed trace (the program table) |

So the Fiat-Shamir part of `L_Fr` costs about 1,200 permutation rows. The Merkle paths of WHIR (I9) are not in this count: the transcript does not absorb opened leaf values.

## 8b. I5c: one shared program table (owner ruling 2026-11-12, D-4; MEASURED)

`ProgramTableAir` (`src/program.rs`) is the only preprocessed trace of the machine: one row `[tag, cells[18], count]` per op of the tape, plus one nop row `[tag, 0, ..]` per AIR kind with count `padded(n) - n`. Each execution AIR keeps its program cells as main columns after its other columns (the old preprocessed columns, same order) and every row, padding included, receives `[tag, cells]` on the program bus (bus 3) with count 1. The bus balances only if each execution AIR has exactly one row per program row and the nop row on every other row. So the AIR height is pinned to `padded(n)`, a padding row has all program cells zero (no memory traffic), and the tag stops one AIR kind from taking another kind's row (DERIVED, `src/program.rs` header). `BbAluProgAir` puts its 3 ALU selectors straight into the message, so the 3 duplicated selector columns of I4 are gone. Tags: Input 1, FrInput 2, BbAlu 3, PoseidonFr 4, Pack 5, Split 6, Reshape 7. Message length 19 plus the bus, below the `2^7` bound of the `L_Fr` set (`log_max_message_length` 7 in `stark-sdk/src/config/log_up_params.rs`).

| Check (log `i5c-final.log`, rerun `i5c-commit-tree.log`) | Result |
|---|---|
| I5b on the real 430,073-byte `L` proof | 1,500 of 1,500 samples equal; accepted; **2 commitments** (common main and the program table), asserted in the test from the proof object and from the vk (1 AIR with a preprocessed trace) |
| I4 and I5 | accepted with 2 commitments (asserted); every earlier negative control still refused |
| New: a changed program cell in an execution row, key unchanged | refused (prover, `NonZeroRootSum`); the memory bus balances, the program bus does not (`bus_balances`, asserted) |
| New: a forged write from two `InputAir` padding rows (`(X, 8)` count `k`, `(X, 7)` count `-k`), so the constant 7 is read as 8 by the ALU | refused (prover); the memory bus balances, the program bus does not (asserted) |
| All `sherlock-lfr` tests, I0 and I5b included | 9 of 9 pass; `cargo clippy -D warnings` clean |

Cost (MEASURED, I5b tape): the table is 32,768 rows by 20 preprocessed columns (about 21,000 ops); the execution AIRs gain no row and 3 to 18 main columns each (the old preprocessed columns, minus 3 at `BbAlu`). Budget line for I6 to I9: the table height is the op count padded and must fit `2^(l_skip + n_stack)`.

## 8c. I6: tape generator part 1 (MEASURED)

`gen_part1` (`src/lverify.rs`) mirrors `verifier/mod.rs` (preamble) and `batch_constraints.rs` steps 1 to 3 with `fractional_sumcheck_gkr.rs`. The ops depend only on the `L` vk and the pinned shape `LShape` (D-1: per AIR its log height or absence); the proof gives the input values (`Input`, `FrInput`), and the generator refuses a proof of another shape. The vk pre-hash, each log height and each presence bit are constants of the key. Every check is an `AssertEq` op. `check_witness` is an observe, a sample, a `LowBits` op (`x mod 2^n`) and an `AssertEq` with 0. New AIRs: `AssertEqAir` (program `[a, b, active]`, reads both addresses with one value; tag 8) and `BitsAir` (program `[x, out, mult, n, active]`; 31 boolean bits of `x`, a non-increasing boolean mask with sum `n`, `low = sum mask_i b_i 2^i`; tag 9; DERIVED uniqueness: `x < p < 2^31 < r`). `Tape::execute_checked` returns the failed assertions (the plain-Rust verdict).

| Quantity (test `i6_part1_on_real_l`, log `i6-final.log`) | Value |
|---|---|
| `L` shape | 39 AIRs, all present; `n_logup` 19, GKR rounds 21, `n_global` 19 |
| Transcript events equal to the stock record, in order | 4,271 of 18,772 (3,251 observes, 3 commitment observes, 1,017 samples), up to the sample of `mu` |
| Tape ops | 8,552: 794 Input, 2 FrInput, 20 constants, 5,638 ALU, 258 Perm, 513 Pack, 254 Split, 254 Compose, 793 Decompose, 25 AssertEq, 1 LowBits; program table 8,561 rows |
| Machine shape (rows padded / main width) | Input 1,024/30, FrInput 8/5, BbAlu 8,192/47, PoseidonFr 512/185, Pack 1,024/27, Split 256/64, Reshape 2,048/16, AssertEq 32/7, Bits 2/69, ProgramTable 16,384/1 (20 preprocessed), Range16 65,536/2 |
| Prove and verify (small Fr set) | accepted, 2 commitments, 916,326 bytes, 13.8 s and 21.5 s |
| Changed GKR layer claim (layer 5, `p_xi_0 + 1`) | stock verifier refuses; tape: 18 assertions fail; prover refuses (`NonZeroRootSum`) |
| Changed LogUp PoW witness (`+ 1`) | stock verifier refuses; tape: 23 assertions fail (the PoW check first); prover refuses |

## 8d. I7: part 2, batched zerocheck (MEASURED)

`gen_part2` mirrors `batch_constraints.rs` steps 4 to 9: `mu` and the batched sum, the univariate round (sum over the skip domain) and the `n_max` multilinear rounds (Lagrange interpolation with constant inverse factorials), `eq_3b` per interaction, `eq_ns` and `eq_sharp_ns`, the column openings (common main of every trace first, then the other parts; the rotation claim of an AIR without rotation is the observed constant 0), and per trace the selectors `is_first_row`, `is_last_row`, `is_transition` at the lifted point (`n < 0` handled: `r_0^(2^-n)` and the norm factor), the DAG of the AIR (`SymbolicExpressionDag` nodes in order: variables are opening addresses, constants are key constants, each add, sub, neg, mul is one ALU op), the lambda-batched constraint value, and the interaction numerator and denominator (message by powers of `beta`, bus index plus 1). The final `cur_sum == evaluated_claim` is one `AssertEq`.

| Quantity (test `i7_part2_on_real_l`, log `i7-final.log`) | Value |
|---|---|
| DAG nodes of the 39 `L` AIRs | 15,312 |
| Transcript events equal to the stock record | 16,975 of 18,772 (15,891 observes, 3 commitment observes, 1,081 samples) |
| Tape ops | 57,404: 3,550 Input, 2 FrInput, 263 constants, 45,909 ALU, 1,059 Perm, 2,100 Pack, 270 Split, 270 Compose, 3,953 Decompose, 27 AssertEq, 1 LowBits; program table 57,413 rows |
| Machine shape | BbAlu 65,536/47, ProgramTable 65,536/1, Reshape 8,192/16, Input 4,096/30, Pack 4,096/27, PoseidonFr 2,048/185, Split 512/64, AssertEq 32/7, Bits 2/69, FrInput 8/5, Range16 65,536/2 |
| Prove and verify (small Fr set, `w_stack` raised from 64 to 128: 82 stacked columns) | accepted, 2 commitments, 3,653,510 bytes, 56.0 s and 61.2 s |
| Changed column opening (trace 3, common main, cell 7) | stock verifier refuses; tape: the final claim assertion fails; prover refuses |
| Changed univariate round coefficient (index 2) | stock verifier refuses; tape: the final claim assertion fails; prover refuses |

## 8e. I8: part 3, stacked reduction (MEASURED)

`gen_part3` mirrors `stacked_reduction.rs`. The layouts come from `verify_proof_shape` (they depend only on the vk and the shape). The tape computes lambda and its square powers, `s_0` from the `t` claims (the openings of part 2, in commit order), the univariate check, the `n_stack` quadratic rounds, and per stacked column `eq_mle(u, b)` with constant bits `b`, `eq_prism`, the rotation kernel `rot_kernel_prism` (when the AIR needs rotation) and the indicator `eval_in_uni` (one shared value per `(n, need_rot)`), then the observed stacking openings and the final sum check. `Part1` now returns the commitment addresses in the WHIR order.

| Quantity (test `i8_part3_on_real_l`, logs `i8-test.log`, `i8-final.log`) | Value |
|---|---|
| Transcript events equal to the stock record | 17,299 of 18,772 (16,135 observes, 3 commitment observes, 1,161 samples); the remaining 1,473 are WHIR |
| Tape ops | 97,944: 3,611 Input, 2 FrInput, 263 constants, 86,229 ALU, 1,084 Perm, 2,131 Pack, 290 Split, 290 Compose, 4,014 Decompose, 29 AssertEq, 1 LowBits; program table 97,953 rows |
| Machine shape | BbAlu 131,072/47, ProgramTable 131,072/1, others as I7 |
| Prove and verify (`small_params_n(3, 15)`: stacked height 2^17, since BbAlu and the table have 2^17 rows) | accepted, 2 commitments, 3,368,326 bytes, 105.7 s |
| Changed stacking opening (commit 0, column 5) | stock verifier refuses; tape: the final sum assertion fails; prover refuses |
| Changed stacking sumcheck evaluation (round 5, `s(2)`) | stock verifier refuses; tape: the final sum assertion fails; prover refuses |

All 12 `sherlock-lfr` tests (I0 and the fixture tests I5b, I6, I7, I8 included) pass on the tree of db8bbd5; `cargo clippy -D warnings` clean (log `i8-final.log`).

ALU cost per stage (DERIVED from the op counts): part 1 5,638, part 2 40,271, part 3 40,320. Part 3 recomputes `eq_mle(u, b)` per stacked column (about 1,537 columns times up to 18 products); a shared product tree over the row bits would remove most of it (optimization, not done).

## 9. Backend fix found by I2

At keygen the per-bus trace-height bound was `SC::F::order().to_u32_digits()[0]`, the low 32 bits of the field order. At BLS12-381 Fr that is 1 (`r = 1 mod 2^32`), so every AIR with an interaction count weight was refused (`TraceHeightsTooLarge`). E1 did not see it: its LogUp fixture (`InteractionsFixture11`, `DummyInteractionAir::new`) has count weight 0, so the per-bus sum was 0 and passed the bound of 1. Fix (5f698b5, `stark-backend/src/keygen/mod.rs`): the order capped at `u32::MAX`, a stronger bound for a field above `2^32`, the same value for BabyBear. Checks (MEASURED): `cargo clippy -D warnings` of stark-backend and stark-sdk (Fr feature, all targets) clean; BabyBear suite 239 of 239 pass; the 4 BabyBear proofs of `e1_bb_bytes` at 1 thread are byte-identical to the E1 baseline `~/.cache/e1-fr/bb-base/` (`cmp`); test `e2_fr_per_bus_threshold`.

## 10. Next increment: I9 (WHIR)

I9 is part 4 of the tape generator: a mirror of `verifier/whir.rs` after `gen_part3` (inputs: `Part3.u`, `Part3.stacking_openings`, `Part3.commits`). Gate: on the real `L`, all 18,772 transcript events equal the stock record, the tape proves with 2 commitments, and a changed opened row, a changed Merkle sibling and a changed final-polynomial coefficient are refused by the stock verifier, by the plain-Rust tape run and by the prover.

Work, with the ops it needs (each new op is a small AIR that takes its program from the table):
- `mu` PoW, folding PoW per sumcheck round, query PoW per WHIR round: `check_witness` as in I6. Query indices: `sample_bits(log_rs_domain_size - k_whir)` (`LowBits`).
- Index bits: a `Halve` op (`x = 2 rest + bit`, `bit` boolean; program `[x, bit, rest, mult_bit, mult_rest, active]`); after `n` steps assert `rest = 0`, which makes the bits the unique binary form because `x < 2^n` (DERIVED).
- `zi_root = omega^index` and its inverse from the bits with key constants: `prod_j (1 + b_j (w^(2^j) - 1))`; no field inversion is needed.
- Leaf hash (u = 1): `hash_slice` packs 8 words per Fr word and absorbs 2 Fr words per permutation by addition from `(0, 0, cap_fixed(1, l))`; a later block needs Fr addition: an `FrAdd` op. Node hash (u = 2): `compress(l, r)` is one permutation of `(l, r, cap_fixed(2, 2))`; the path order needs a `CSwap` op (`l = a + bit (b - a)`, `r = b + bit (a - b)`). Check `tree_compress` in `stark-backend/src/hasher.rs` for the leaf-to-query-digest order. Root: `AssertEq` with the commitment address.
- `binary_k_fold` (twiddles and `halve` are key constants), the `gamma` batching, and the final check (`eval_mobius_eq_mle`, `eval_mle_evals_at_point` of the final polynomial, `exp_powers_of_2`, Horner) on the ALU.
- Size (ESTIMATE): 263 queries; about 20,000 to 25,000 permutations (the wrap model's 22,161), so `PoseidonFr` about 2^15 rows; the tape will need `n_stack` 16 or more at the small test set.

Then I10 (the gate at the `L_Fr` set: public values D-2, the `L_Fr` parameters, heights MEASURED, D3 rerun).

Items still open:

- The public values of `L_Fr` (D-2) are not wired yet; the public-word negative control belongs to I10. `L` has 0 public values; `gen_part1` already reads public values as `Input` ops, and part 2 stops on an `Entry::Public` variable (unimplemented until a fixture has public values).
- The `L_Fr` set has max constraint degree 4; every AIR here is degree 3.
- The ALU dominates the tape (86,229 of 97,944 ops after I8); the stacked-reduction `eq_mle` sharing above is the first optimization.
- Heights and the D3 rerun at the measured `L_Fr` shape wait for I10.
