# E2 results: the Fr recursion AIR set `L_Fr` that verifies one `L` proof

Plan: `PLAN.md` beside this file. Owner ruling 2026-11-12: OD-24 is option E; E1 passed.

## 1. Status

**E2 gate: NOT REACHED.** Increments I0 to I5 of `PLAN.md` section 4 pass their test gates, and the whole transcript of the stock verifier on the real `L` proof runs and proves on the tape (I5b). I6 (the tape generator for the `L` verifier) is next.

| # | Increment | Status | Evidence |
|---|---|---|---|
| I0 | Fixture: the real `L` proof, `L` vk, node vk on disk | PASS | section 3 |
| I1 | `L_Fr` hash tags u = 5, 6, 7 (D-3) | PASS | section 4 |
| I2 | `BbAlu` and `Range16`: BabyBear and `EF4` add, sub, mul emulated in Fr | PASS | section 5 |
| I3 | `PoseidonFr` AIR (Midnight Poseidon, one row per permutation) | PASS | section 6 |
| I4 | Tape, write-once memory bus, `InputAir`, `BbAluProgAir` | PASS | section 7 |
| I5 | Transcript of `L` on the tape: `FrInput`, `PoseidonProg`, `Pack`, `Split`, `Reshape` | PASS | section 8 |
| I5b | The stock verifier's transcript on the real `L` proof, on the tape | PASS | section 8a |
| I6 to I10 | Tape generator (GKR, zerocheck, stacked reduction, WHIR), then the gate | not started | section 10 |

Route (`PLAN.md` section 2): a dedicated verifier of the `L` vk with the `L` shape pinned in the `L_Fr` key (LEAD D-1), run as a straight-line tape over generic Fr AIRs. Not a port of OpenVM's recursion crate.

## 2. Code and commits

| Repository (all not pushed) | Branch | Commits |
|---|---|---|
| backend `~/.cache/m7-wrap/src/openvm-org_stark-backend` | `e2-lfr` (from `e1-fr-base` 3040761) | e8223ed I1 domain tags; 5f698b5 per-bus bound fix (section 9) |
| openvm `~/.cache/m7-wrap/src/openvm-org_openvm` | `e2-lfr` (from `main` 05f3b73) | 3cc1849 the M7 bench crate `stark-bench-ovm` (the existing uncommitted changes); e8fdf87 `sherlock-lfr` with I0, I2 to I5; 96a1897 I5b |
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
| `src/testing.rs` | small Fr parameter set; prove, verify and refusal helpers; `Range16` fill from the logical interactions |
| `tests/i0_fixture.rs` to `tests/i5_transcript.rs`, `tests/i5_l_transcript.rs` | the gates |

Saved beside this file: `backend-e2.diff` (backend `git diff 3040761 5f698b5`), `sherlock-lfr.diff` (openvm `git diff 3cc1849 96a1897`), `run.sh` and `i0.sh` (the runners).

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

Each tape value has one address and is written once; the writer sends `[addr, v0..v3]` on the memory bus with the read count of `addr` as multiplicity, each read sends it with count `-1`. Addresses, op kinds and multiplicities are preprocessed columns, so the key fixes the tape; only values are witness. `InputAir` checks each input coefficient canonical (and zero above coefficient 0 for a BabyBear input, and equal to the constant for a constant).

Test `i4_tape_executes_and_proves`: Horner of a degree-40 `EF4` polynomial plus a BabyBear constant (126 ops: 43 inputs and constants, 83 ALU rows); the executed result equals p3 `EF4`; accepted, 429,457 bytes. Refused (prover, `NonZeroRootSum`): an ALU row that reads `a + 1` (its own identities hold), a program that reads an address no op writes, a non-canonical input `v + p` used consistently by its readers.

## 8. I5: the `L` transcript on the tape (MEASURED)

`TapeTranscript` emits the ops of the stock `MultiFieldTranscript<BabyBear, Fr, MidnightPoseidonFr, 3, 2>` from `(0, 0, cap_var(u))`: overwrite absorbs move state addresses (no op), full blocks and squeezes add a `Perm` op, 8 observed words add a `Pack` op, a squeeze adds a `Split` op (5 digits), `observe_ext` a `Decompose`, `sample_ext` a `Compose`. New AIRs: `FrInputAir` (1 + 4 preprocessed columns), `PoseidonProgAir` (175 + 10), `PackAir` (9 + 18), `SplitAir` (52 + 12, 39 lookups), `ReshapeAir` (4 + 12).

**Split uniqueness** (DERIVED, `src/fr.rs` header): `x = sum d_i p^i + p^5 h` in Fr with canonical digits; `h` and `H - h` in 112 bits (`H = floor((r - 1) / p^5) < 2^101`), and when `h = H`, `L - low` in 160 bits (`L = (r - 1) - p^5 H`). Then the integer is at most `r - 1`, so it equals `x` and the digits are the stock digits.

Test `i5_transcript_equals_stock_and_proves`: a seeded schedule of 400 steps (observes, `EF4` observes, commitment observes, samples, `EF4` samples, re-observes of sampled digits): all 147 samples on the tape equal the stock `L` transcript (u = 3). Machine shape (rows padded / main width / preprocessed width): Input 256/20/10, FrInput 64/1/4, PoseidonFr 128/175/10, Pack 128/9/18, Split 128/52/12, Reshape 64/4/12, Range16 65,536/2/0. Accepted, 742,257 bytes. Refused: the split witness of `x + r` (the Fr identity holds, asserted; verifier), a changed digit (verifier), a changed packed word (prover).

Final run of all `sherlock-lfr` tests on the tree of e8fdf87: 8 of 8 pass (I0 included with `--include-ignored`), log `~/.cache/e2-lfr/logs/tests-final.log`; `cargo clippy -p sherlock-lfr --all-targets -D warnings` clean. The I0 to I5 code went into one commit: the increments were written in order, and the commit was made after all their tests passed on the final tree.

## 8a. I5b: the real `L` transcript on the tape (MEASURED)

Test `i5_l_transcript_on_tape` (ignored by default, needs the fixture; log `i5b.log`): a recording `FiatShamirTranscript` wrapper around the stock `L` transcript logs every call while `verifier::verify` accepts the fixture proof. The log drives `TapeTranscript` with the observed values as tape inputs.

| Quantity | Value |
|---|---|
| Transcript events of one `L` verification | 18,772: 17,267 observes, 5 commitment observes, 1,500 samples |
| Samples equal on the tape | 1,500 of 1,500 |
| Tape ops | 1,192 `Perm`, 2,275 `Pack`, 362 `Split` |
| Machine shape (rows padded / main width / preprocessed width) | Input 32,768/20/10, FrInput 8/1/4, PoseidonFr 2,048/175/10, Pack 4,096/9/18, Split 512/52/12, Range16 65,536/2/0 |
| Prove and verify at the small Fr set | accepted, 54.2 s on a loaded host, proof 1,553,425 bytes |

So the Fiat-Shamir part of `L_Fr` costs about 1,200 permutation rows. The Merkle paths of WHIR (I9) are not in this count: the transcript does not absorb opened leaf values.

## 9. Backend fix found by I2

At keygen the per-bus trace-height bound was `SC::F::order().to_u32_digits()[0]`, the low 32 bits of the field order. At BLS12-381 Fr that is 1 (`r = 1 mod 2^32`), so every AIR with an interaction count weight was refused (`TraceHeightsTooLarge`). E1 did not see it: its LogUp fixture (`InteractionsFixture11`, `DummyInteractionAir::new`) has count weight 0, so the per-bus sum was 0 and passed the bound of 1. Fix (5f698b5, `stark-backend/src/keygen/mod.rs`): the order capped at `u32::MAX`, a stronger bound for a field above `2^32`, the same value for BabyBear. Checks (MEASURED): `cargo clippy -D warnings` of stark-backend and stark-sdk (Fr feature, all targets) clean; BabyBear suite 239 of 239 pass; the 4 BabyBear proofs of `e1_bb_bytes` at 1 thread are byte-identical to the E1 baseline `~/.cache/e1-fr/bb-base/` (`cmp`); test `e2_fr_per_bus_threshold`.

## 10. Next increment: I6

I6 is the tape generator, part 1: a mirror of `stark-backend/src/verifier/mod.rs` and `fractional_sumcheck_gkr.rs` that emits tape ops for the `L` vk and the pinned shape (D-1): the shape pin (each `log_height` and presence bit observed as a constant of the key), the preamble (`observe_commit` of the vk pre-hash and the common-main commitment, the cached commitment), the LogUp PoW check, and the GKR layers. Gate: on the fixture, every value the stock verifier observes or samples equals the tape value (the recording wrapper of `i5_l_transcript.rs` gives the reference log), and the op counts are MEASURED. In I6 to I9 the observed values stop being free tape inputs: each one becomes either a proof value (an `Input` op) or a value the tape computes, and the stage checks become tape ops. The 17,267 observes of I5b are the full list that I6 to I9 must account for. Then I7 (zerocheck and the DAG of the 39 `L` AIRs), I8 (stacked reduction), I9 (WHIR with a Merkle AIR at u = 1, 2 and query bits), I10 (the gate at the `L_Fr` set).

Items still open:

- Ops still missing in the machine: `sample_bits` and bit decomposition (grinding and query indices), the `eq` and selector evaluations at the univariate-skip points, the Merkle path (u = 1 leaves of packed words, u = 2 nodes), inversion in `EF4` (for the GKR fraction checks, as `a * a^-1 = 1` on the ALU).
- `BbAluProgAir` keeps the 3 selectors in the main trace and equates them to the preprocessed ones (3 columns per row that could go).
- The public values of `L_Fr` (D-2) are not wired yet; the public-word negative control belongs to I10.
- The `L_Fr` set has max constraint degree 4; every AIR here is degree 3.
- **Commitment count (DERIVED, `verifier/mod.rs`: the verifier adds one commitment per AIR with preprocessed data).** Every machine AIR except `Range16` has its own preprocessed program, so a machine proof has 1 + (number of machine AIRs other than `Range16`) commitments: 6 for the I5b machine, 8 when every AIR is present. Each is its own stacked matrix and Merkle tree at height `2^(l_skip + n_stack)`. The E5 wrap estimate (879,472 mid rows) assumed 2 commitments, and round-0 Merkle was already its largest line. LEAD item for I6 onward: either one program table AIR that feeds the execution AIRs over a bus (one preprocessed commitment), or keep the per-AIR programs and record the wrap cost as MEASURED at I10. This choice must be made before the AIR set freezes.
- Heights and the D3 rerun at the measured `L_Fr` shape wait for I10.
