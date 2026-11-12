# E1 results: stark-backend with a BLS12-381 Fr base field

Owner ruling 2026-11-12: OD-24 is option E. E1 is step 1 of the option E plan (`research/sherlock-m7-wrap-20261112/RESULTS.md` section 2c).

## 1. Gate status

| Gate item | Status | Evidence |
|---|---|---|
| stark-backend and cpu-backend accept `F = Bls12381Fr` with `EF = F` (degree 1) | PASS | config `bls12381_fr_poseidon.rs`; reference engine and CPU engine both prove and verify (section 5) |
| A small AIR proves and verifies end to end at the Fr base | PASS | Fibonacci AIR (3 public values) and a 2-AIR interaction (LogUp) fixture, both engines, every grinding site on (MEASURED) |
| Negative control: a flipped proof byte is refused | PASS | 256 of 256 single-byte flips refused: 1 by the decoder, 255 by the verifier (MEASURED) |
| Negative control: a changed public value is refused | PASS | 3 of 3 public values changed by +1, each refused; also refused at the `L_Fr` parameter set (MEASURED) |
| D3 terms at the configured Fr parameters equal M7 section 2b.2 | PASS | all 12 terms and the total equal the M7 values to 4 decimals in all 4 cases (h 2^20 and 2^22, `w_stack` 18 and 48) (MEASURED) |
| BabyBear behavior bit-identical | PASS | the 239 existing tests pass before and after, same test list; 4 BabyBear proofs byte-identical to the baseline (MEASURED) |

**E1 gate: PASS.**

## 2. Code

Backend clone `~/.cache/m7-wrap/src/openvm-org_stark-backend`, branch `e1-fr-base` from `main` a6b4761 (v2.1.0). Not pushed.

| Commit | Content |
|---|---|
| a16c9b3 | M7b diff (`lfr/stark-backend.diff` plus the untracked `baby_bear_bls12381_poseidon.rs`), unchanged |
| 7f50d96 | BabyBear proof-byte fingerprint test `crates/cpu-backend/tests/e1_bb_bytes.rs` |
| b281bfe | the base-field relaxation (section 3) |
| 975721c | the Fr config, feature `bls12381-fr-poseidon`, and test `crates/stark-sdk/tests/e1_fr.rs` |
| 3040761 | the exact statistical-distance statement in the `sample_bits` documentation |

Saved beside this file: `e1.diff` (`git diff a16c9b3 3040761`, the E1 change on top of M7b), `Cargo.lock.e1` (the backend lock; `Cargo.lock` is ignored by the clone's `.gitignore`), and `run-tests.sh` (the test runner). The field crate `~/.cache/m7-wrap/src/p3-bls12-381-fr` is not changed by E1: its `src/lib.rs` and `Cargo.toml` are byte-identical to `research/sherlock-m7-wrap-20261112/lfr/p3_bls12_381_fr_lib.rs` and `p3_bls12_381_fr_Cargo.toml` (MEASURED, `cmp`).

**Lock file.** The M7b `Cargo.toml` change made the backend lock stale. `cargo metadata --offline` added 34 packages and changed none. 4 of the 34 resolved to versions not in the openvm lock, and were pinned back with `cargo update --offline --precise` to the openvm versions (base64ct 1.8.0, blst 0.3.15, const-oid 0.10.2, hybrid-array 0.4.12). After that, every added package (name and version) is in `~/.cache/m7-wrap/src/openvm-org_openvm/Cargo.lock` (MEASURED by script). Nothing was downloaded or installed.

## 3. Base-field relaxation

`StarkProtocolConfig::F: TwoAdicField + PrimeField64` becomes `TwoAdicField + PrimeField` (`crates/stark-backend/src/config.rs:22`).

**Site count.** At a6b4761, `PrimeField64`, `as_canonical_u64` and `ORDER_U64` occur on 21 lines in 5 files of `stark-backend/src` and `cpu-backend/src`, and on 0 lines in `cpu-backend/src` (MEASURED, grep). The "48 sites" of M7 section 2c is not reproduced. After the relaxation, the compiler shows no other site: `cargo check` and `cargo clippy -D warnings` of stark-backend, cpu-backend and stark-sdk (all targets, with the Fr feature) are clean (MEASURED). The 28 lines with `PrimeField32`, `as_canonical_u32` or `ORDER_U32` in `stark-backend/src` (MEASURED, grep; codec, multi-field transcript, multi-field hasher) are bounded helpers that only the BabyBear configs call; they are unchanged.

| Site | Change |
|---|---|
| `transcript/traits.rs` `sample_bits` | the low `bits` bits of the canonical representative (`canonical_low_u64`, from `as_canonical_biguint`); the range check is `bits < F::bits()` (`pow2_below_order`), exact for an odd prime |
| `transcript/traits.rs` `grind` | candidates `0 .. min(p, 2^64)` (`grind_candidate_count`); for BabyBear this is `0 .. p` as before |
| `verifier/transcript_extractor.rs` | bound `PrimeField`; `observe_usize` refuses any value that does not fit `usize` and never truncates |
| `soundness/mod.rs` `base_field_order` | `p` as an `f64` from the `BigUint` order (exact for BabyBear) |
| `test_utils/mod.rs` | the Fibonacci helpers compute in `F`, so the fixtures work at any field |

**BabyBear identity (MEASURED).** Baseline = a detached worktree at 7f50d96 (removed after the runs), the same commands and target directory. Its proofs are kept in `~/.cache/e1-fr/bb-base/` and its logs in `~/.cache/e1-fr/logs/base-*.log`.

- `cargo nextest run --release` of stark-backend, stark-sdk and cpu-backend: 239 of 239 pass before and after; the sorted test lists are equal.
- `e1_bb_bytes` proves a Fibonacci AIR and an interaction fixture with the BabyBear reference engine (stock `grind`) and the BabyBear CPU engine (p3 `DuplexChallenger` grind), at LogUp PoW 7, folding PoW 5, `mu` PoW 4 and query PoW 6, verifies each and writes the encoded proofs. At `RAYON_NUM_THREADS=1` the 4 proofs are byte-identical to the baseline and between two runs (`cmp`): 42,357 bytes (FNV-1a `6b4fc7ffed5c3310`) and 42,557 bytes (`3eaa8b2ca41e5c53`). The reference and CPU engines give the same bytes.
- Grinding uses `find_any` under rayon, so with several threads the PoW witness, and so the proof bytes, can differ between runs (DERIVED from the `find_any` contract; not measured). The identity check is therefore made single-threaded. This is a stock property, not an E1 change.

**Other profiles and configs (MEASURED).** A dev-profile (`opt-level` 1, debug assertions on) `cargo check --all-targets` of the three crates with the Fr and BN254 features is clean, and the 9 non-ignored `e1_fr` tests pass in the dev profile (`~/.cache/e1-fr/logs/dev-fr.log`). The M7b test `m7_bls_fr` (the BabyBear-base Midnight Poseidon config that produces `L`, whose transcript uses the changed default `sample_bits` and `grind`) passes 6 of 6 on this branch through the openvm checkout (`~/.cache/e1-fr/logs/m7-bls-fr.log`). The BabyBear-BN254 config is compile-checked only.

**Downstream.** The openvm checkout `~/.cache/m7-wrap/src/openvm-org_openvm` patches this clone by path. `cargo check --release --locked -p stark-bench-ovm --features bls --all-targets` (which builds openvm-recursion-circuit and the M7b `l_fr` example) passes with the relaxed bound (MEASURED). `cuda-backend` uses the concrete BabyBear `F` (`cuda-backend/src/utils.rs:47`) and was not built in E1 (`/opt/cuda/bin/nvcc` is present; the GPU path is E4).

## 4. Sampling and grinding for a 255-bit field

Let `r` be the BLS12-381 Fr modulus, `log2 r = 254.857` (MEASURED by `e1_field_facts` from the configured order), and `bits < 32` (asserted).

1. **`2^32` divides `r - 1`** (two-adicity 32; test `e1_field_facts` asserts `r mod 2^32 = 1`). So for every `bits < 32`, `r = c 2^bits + 1` with `c = floor(r / 2^bits)` (test `e1_sample_bits_counts` asserts this for `bits` 1, 4, 20, 31).
2. **Distribution of `sample_bits`.** Model the sampled Fr element `x` as uniform on `[0, r)` (the sponge premise, as for BabyBear). `sample_bits` returns `x mod 2^bits`. Residue 0 has `c + 1` preimages and each other residue has `c`. So `P(0) = (c + 1) / r` and `P(v) = c / r` for `v != 0`. The statistical distance from the uniform distribution on `[0, 2^bits)` is `(1/2) sum |P(v) - 2^-bits| = (1 - 2^-bits) / r < 2^-254` (DERIVED). This is the same structure as BabyBear (one heavy residue), with a far smaller bias. The query-phase term of the D3 calculator already uses these residue probabilities (`sample_bits_residue_probs` with the configured order).
3. **Grinding.** `check_witness(bits, w)` observes `w` and accepts if `sample_bits(bits) = 0`, probability `(c + 1) / r ≈ 2^-bits` per fresh witness. The verifier accepts any Fr element as a witness, so the candidate set changes only completeness, not soundness. The prover tries `0, 1, 2, ...` below `2^64`; the probability that none of them succeeds is about `exp(-2^(64 - bits)) < exp(-2^33)` (DERIVED). Test `e1_grind` finds a 12-bit witness, and the verifier check accepts it.
4. **Empirical check (MEASURED).** 2^16 four-bit samples from the Fr transcript: each of the 16 counts is within 5 standard deviations of 4,096.
5. **Large-field values elsewhere.** `canonical_low_u64` is used only by `sample_bits`. Every place that turns a field element into an integer for control flow (`observe_usize`) refuses values above `usize::MAX`.

## 5. The Fr configuration

File `crates/stark-sdk/src/config/bls12381_fr_poseidon.rs`, feature `bls12381-fr-poseidon` (enables the M7b feature and reuses its permutation `MidnightPoseidonFr`, the midnight-circuits 7.3.0 `permutation_cpu`, and `cap_fixed` / `cap_var`).

- `F = EF = Bls12381Fr`, `D_EF = 1`, digest one Fr element.
- **Merkle leaf (u = 1).** Midnight's fixed-length sponge over the leaf's `w` native Fr values, `l = w` (no packing): state `(0, 0, w + 2^66)`, blocks of 2 added into `(r_0, r_1)`, one permutation per block, output `r_0`.
- **Merkle node (u = 2).** `(left, right, 2 + 2 * 2^66)`, one permutation, output `r_0`.
- **Transcript (u = 3).** The stock overwrite duplex `DuplexSponge<Fr, MidnightPoseidonFr, 3, 2>` from `(0, 0, 2^64 + 3 * 2^66)`; one Fr word per cell; a squeeze returns `s_1`, then `s_0`; a commitment observe absorbs the one digest element. The stock `DuplexSponge` transcript impl needs `Digest = [F; RATE]` (2 elements), so a newtype `Transcript` carries the 1-element digest.
- **Codec.** Every Fr value (base, extension, digest) is 32 big-endian canonical bytes; the decoder refuses a value at or above `r`.
- **Use codes are configuration.** `FrDomains { u_leaf, u_node, u_transcript }` defaults to 1, 2, 3, as the brief says. Section 2c E6 lists a separate tag for the `L_Fr` transcript (OD-18) as an open owner item: with u = 1 and u = 3, the `L_Fr` leaf hash and transcript share their initial capacity with the leaf hash and transcript of `L` (BabyBear base, packed words). A different tag changes only the `FrDomains` constants. Test `e1_fr_negative_controls` shows that the domain is bound: a verifier transcript at u = 5 refuses a u = 3 proof.
- Engines: `Bls12381FrPoseidonRefEngine` (column-major reference) and `Bls12381FrPoseidonCpuEngine` (row-major CPU backend).

## 6. Tests (MEASURED)

Test `crates/stark-sdk/tests/e1_fr.rs`, 10 tests, all pass on the final tree 3040761 (`cargo test --release --features bls12381-fr-poseidon --test e1_fr -- --include-ignored`; log `~/.cache/e1-fr/logs/final-fr.log`).

| Test | What it checks | Result |
|---|---|---|
| `e1_field_facts` | `r` 255 bits, `r = 1 mod 2^32`, two-adicity 32, `D_EF = 1`, the order checks at Fr and BabyBear, the grind candidate counts, `canonical_low_u64`, `log2` of the configured order 254.857089413 and challenge bits equal to it | pass |
| `e1_sample_bits_counts` | `r mod 2^bits = 1` for `bits` 1, 4, 20, 31; 2^16 four-bit samples within 5 sigma of uniform | pass |
| `e1_grind` | a 12-bit witness is found and accepted; a following non-witness is refused | pass |
| `e1_hash_domains` | leaf hash equals Midnight's fixed-length sponge (u = 1) and a hand-rolled sponge for `w` 1 to 9; node equals u = 2; a node and a 2-value leaf differ; transcript initial state, first squeeze order, overwrite and permute at the second word | pass |
| `e1_codec_canonical` | 32-byte round trip; `r` and `2^256 - 1` refused | pass |
| `e1_fr_fib_ref_and_cpu` | Fibonacci AIR, `2^9` rows, 3 public values, both engines: prove, verify, codec round trip exact; proof 90,729 bytes; at 1 thread the two engines give the same bytes | pass |
| `e1_fr_interactions_ref_and_cpu` | 2-AIR LogUp fixture, both engines | pass |
| `e1_fr_negative_controls` | 3 of 3 public-value changes refused; 256 of 256 byte flips refused (1 decoder, 255 verifier); a u = 5 verifier transcript refuses | pass |
| `e1_fr_fib_at_lfr_params` (ignored by default) | Fibonacci AIR `2^12` rows at the `L_Fr` set: UD, log blowup 3, queries 98/83/82, query PoW 20, folding, `mu` and LogUp PoW 0, `l_skip` 2, `n_stack` 18, `w_stack` 48: proves, verifies, refuses a changed public value | pass |
| `e1_fr_d3_terms` | section 7 | pass |

Small test parameters: UD, log blowup 1, `l_skip` 2, `n_stack` 8, `k_whir` 4, LogUp PoW 7, folding PoW 5, `mu` PoW 4, query PoW 6, so every grinding site runs.

At the `L_Fr` parameter set (CPU engine, host load 17 to 33 from other jobs, so the times are not clean measures): keygen and prove 175.3 s and 173.3 s in two runs, verify 0.52 s and 0.51 s, proof 300,269 bytes (MEASURED; the final run is in `final-fr.log`, load 31 at its end). Stacked height is `2^(l_skip + n_stack) = 2^20` at blowup 8, so the commit is over `2^23` Fr rows regardless of the small trace.

## 7. D3 terms at the configured Fr parameters (MEASURED)

Test `e1_fr_d3_terms` uses the stock `SoundnessCalculator::calculate` with `base_field_order::<Bls12381FrPoseidonConfig>()` and `challenge_field_bits::<Bls12381FrPoseidonConfig>()`, that is, from the configuration, not hand-fed constants. Parameters as M7 `m7_fr_d3.rs` (`mk`): `params_with_100_bits_security(3, 2, 18, w, 0, 0, UD, 4, 20, 4)`, queries 98/83/82, LogUp PoW 0, shape bounds of PROPOSAL 11.2 (1,000 constraints, 50 AIRs, 2,000 columns, 100 interactions per AIR). M7 used a per-fold PoW vector of zeros and 1 OOD sample through its D3 extension; with folding PoW 0 that equals the stock calculator.

| Term | h 2^20 / 2^22, `w_stack` 18 | h 2^20 / 2^22, `w_stack` 48 | M7 2b.2 |
|---|---:|---:|---:|
| LogUp (PoW 0) | 215.9502 | 215.9502 | 215.9502 |
| GKR sumcheck / batching | 253.2721 / 254.8571 | 253.2721 / 254.8571 | 253.2721 / 254.8571 |
| zerocheck | 250.9502 | 250.9502 | 250.9502 |
| constraint batching | 244.8841 | 244.8841 | 244.8841 |
| stacked reduction | 242.8913 | 242.8913 | 242.8913 |
| `mu` (PoW 0) | 227.7696 | 226.3025 | 227.7696 / 226.3025 |
| folding (PoW 0) | 232.8571 | 232.8571 | 232.8571 |
| query (PoW 20) | 101.1435 | 101.1435 | 101.1435 |
| shift | 101.1435 | 101.1435 | 101.1435 |
| `gamma` / OOD | 248.2277 / 239.8571 | 248.2277 / 239.8571 | 248.2277 / 239.8571 |
| total | 101.1435 | 101.1435 | 101.1435 |

Every term equals the M7 value within 5 x 10^-5 in all 4 cases (asserted). The weakest term is the query term, which does not depend on the field.

## 8. Limits and open items

- **Domain tags.** u = 1, 2, 3 are shared with `L` (section 5). Owner item OD-18 (E6).
- **Not covered.** No GPU path (`cuda-backend` is BabyBear-only; E4). No Fr `DuplexSpongeRecorder` or `DuplexSpongeValidator` and no `extract_proof` test at Fr (the extractor's bounds are relaxed, but its digest type is `[F; RATE]`, so the 1-element Fr digest does not fit it). The Fr prover speed is not optimized (the permutation is called through `Bls12381Fr` to `midnight_curves::Fq` conversions; Fr has no SIMD packing, `Packing = Self`).
- **`sample_bits` cost at BabyBear.** It now converts through a `BigUint` per call. The proofs are byte-identical; the time cost was not measured separately (the 239-test suite ran in 6.2 s at the baseline and 5.9 s and 6.6 s after, MEASURED, nextest summary on a loaded host).

## 9. Next step

E2: the Fr recursion AIR set `L_Fr` that verifies one `L` proof (BabyBear and EF arithmetic emulated in Fr, a Midnight Poseidon sub-AIR, and the GKR, zerocheck, stacked-reduction and WHIR AIRs), proved on this configuration. E5 can start on a synthetic `L_Fr` proof from this configuration (for example `e1_fr_fib_at_lfr_params`).

## Commands

- Runner: `run-tests.sh <src> <tag>` (copy beside this file). Builds and tests run under `flock ~/.cache/cost-opt/measure/heavy.lock heavy-run --name e1-fr-<stage>`, `CARGO_TARGET_DIR=~/.cache/e1-fr/target`, logs in `~/.cache/e1-fr/logs/`.
- BabyBear suite: `cargo nextest run --release --locked --no-fail-fast -p openvm-stark-backend -p openvm-stark-sdk -p openvm-cpu-backend`.
- Bytes: `RAYON_NUM_THREADS=1 E1_PROOF_OUT=<dir> cargo test --release --locked -p openvm-cpu-backend --test e1_bb_bytes -- --nocapture`, then `cmp` against `~/.cache/e1-fr/bb-base/`.
- Fr: `cargo test --release --locked -p openvm-stark-sdk --features bls12381-fr-poseidon --test e1_fr -- --nocapture --test-threads=1 --include-ignored`.
- Downstream: in the openvm checkout, `cargo check --release --locked -p stark-bench-ovm --features bls --all-targets`.
