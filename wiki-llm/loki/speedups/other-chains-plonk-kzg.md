---
title: Loki speed-ups from other chains' PLONK/halo2/KZG provers
date: 2026-10-24
component: other-chains (multiopen, MSM scheduling, synthesis inversions, instances, memory)
sources: zcash/halo2 #796 #937 #938 #962; axiom-crypto/halo2 #17 #28 #36 #37; scroll-tech/halo2 #27 #71; taikoxyz/halo2 PR 6 (via axiom #17); privacy-scaling-explorations/halo2curves #130 #168; ethereum/c-kzg-4844 #299 #628; AztecProtocol/barretenberg #94 #550; Snarkify/Scroll GPU claims (vendor); ZPrize/TCHES 2024 MSM results (WebSearch)
raw: /home/charles/repos/Moriarty-factory/wiki-llm/loki/speedups/raw/other-chains-plonk-kzg/
---

# Summary

Most published prover speed-ups from the halo2 family were measured on Pasta or BN254, and most
of the MSM work targets code paths Loki no longer uses. Loki already has the big wins that other
chains found first: blst Pippenger for BLS12-381 G1 (`src/poly/kzg/msm.rs:178-215`), the lookup
counting map instead of a BTreeMap (`src/plonk/lookup/prover.rs:~383-400`), batch inversion
(`batch_invert_parallel`, `src/utils/arithmetic.rs:172`), Lagrange commits (`src/poly/kzg/mod.rs:83`)
and parallel quotient evaluation (`src/plonk/evaluation.rs:320-335`). The most useful findings for
Loki are therefore not new MSM kernels. They are in the serial parts of the KZG multiopen and
in the small-MSM scheduling policy.

Two corrections to the brief's assumptions:
- Loki's multiopen is **SHPLONK-style** (x1..x4 challenges, one `f_com`, one `pi`), not GWC.
  `src/poly/kzg/mod.rs:94-200`. The `test_roundtrip_gwc` name is historical.
- Loki's CPU MSM for BLS12-381 G1 is already blst, so the halo2curves batch-affine and Booth
  work (written for BN254 and Pasta) is not expected to beat it.

# Ranked table (top 5)

| Rank | Technique and source | Measured gain (source) | Loki component | Track | Effort | Correctness risk |
|---|---|---|---|---|---|---|
| 1 | Parallelise the serial multiopen work: the per-set `inner_product` with clones, the per-point `kate_division` loop and the `f_poly` build. Precedent: axiom #37 (parallel `compute_inner_product` above 32 elements). | None published for #37. Loki-side estimate unmeasured. `kzg_multiopen` is 211 ms of 1,127 ms at k16 (18.8%). | multiopen | A | S | Low if bit-exact. Field ops are exact, so reduction order must still match. |
| 2 | Stop splitting every small CPU MSM into per-thread Pippenger chunks. Run independent MSMs concurrently, or parallelise across windows. Precedent: zcash #796 (flattened bucket parallelism). | zcash #796: `best_multiexp` 20–56% faster on one run and 35–36% on the swapped run (k=8–14, generic Pasta path). The gain is not transferable to blst directly. | MSM | A | M | None for the result. Only the scheduling changes. |
| 3 | Vartime field inversions and affine conversions in witness synthesis. Precedent: zcash #962. | Not in the PR body. Loki's CPU bench uses rational advice assignments, so this is on the synthesis path. | synthesis | A | S | Low. Prover-side only. Remove the constant-time guarantee from proving, as zcash did. |
| 4 | Do not store extended fixed cosets in the pk, and recompute them per proof. Precedent: axiom #17 / taikoxyz/halo2 #6. | Speed loss: agg 79.4 s to 98.4 s (+24%), shard 16.2 s to 16.5 s. Memory and pk-size win. | quotient (memory) | A | S | Low. Only useful if RSS (16 GB) is the binding constraint, which it is not for speed. |
| 5 | Commit instances directly from supplied values and batch-normalise, with an invariant against regression. Precedent: zcash #938. | No numbers in the PR. Loki already filters zeros in `msm_cpu`, so the gain is expected to be small. | recursion (committed instances) | A | S | Low. Check that every construction path keeps the invariant. |

# Details

## 1. Parallelise the multiopen linear combinations and kate divisions (multiopen, Track A)

- **Loki hooks.** `src/poly/kzg/mod.rs:94-200` (`multi_open`):
  - the per-set `inner_product(polys, x1)` at about lines 96-106 clones each polynomial and runs serially;
  - the `f_polys` map at about lines 146-158 runs `kate_division` for each point in serial order with `.map().collect()`, not `par_iter`;
  - `inner_product(&polys, powers(x4))` for `final_poly` (about line 184) is serial over polynomials;
  - `kate_division(&(&final_poly - v).values, x3)` (line 196) is a single serial pass over size `n`.
  `inner_product` itself is `src/utils/arithmetic.rs:317-327`. It uses `reduce` over `p.clone() * s`,
  which allocates one full polynomial per term. `eval_polynomial` (`arithmetic.rs:55`) is already parallel.
- **Precedent.** axiom-crypto/halo2 PR #37, "optimize compute_inner_product with parallel computation"
  (merged 2025-02-03). Parallel above 32 elements, serial below, with tests against the serial path. No
  timing is given in the PR. Raw: `raw/other-chains-plonk-kzg/axiom-crypto_halo2-pr37.md`.
- **Plan.** (a) Fuse each linear combination into one buffer with a parallel chunked scale-and-add,
  without `clone()`. (b) Run the independent point sets in `f_polys` with `par_iter`. Each
  `kate_division` stays serial because it is a recurrence. (c) For a single long division, use a
  chunked prefix scheme if (a) and (b) are not enough. Not yet implemented.
- **Expected gain.** Unmeasured. The bound is the serial share of the 211 ms. Profile before and after
  with `examples/loki-profile` (the `kzg_multiopen` phase already exists).
- **What would have to be true.** The serial loops must be a material part of the 211 ms. The
  `clone()` allocations must cost more than the arithmetic, which is plausible at k20 with tens of
  polynomials of 2^20 elements.
- **Correctness.** Bit-exact by construction, since the field operations are the same. Test with the
  existing proof-equality checks against the unmodified stock verifier.

## 2. Small-MSM scheduling on CPU (MSM, Track A)

- **Loki hook.** `src/poly/kzg/msm.rs:199-210` (`msm_cpu`, blst branch). `chunk_size =
  max(1024, n / threads)`, so an MSM with n below 16384 is split into `threads` full Pippenger calls.
  The GPU threshold is 16384 (`LOKI-GPU-MSM.md`), so these small MSMs stay on CPU.
- **Why this matters.** A Pippenger chunk pays `2^c` bucket overhead per window. Splitting n into
  `t` chunks multiplies that overhead by `t`, and each chunk uses a smaller window. Per-chunk work
  therefore grows with `t`. Running independent MSMs concurrently (commitments that are not data
  dependent) keeps one window size per MSM.
- **Precedent.** zcash-halo2 PR #796 (merged 2025-12-04, commit `61cce59`) flattened the parallel
  task so each thread does one segment's bucket work, with the final sum in parallel. Its numbers
  (k=8–14) are 20–56% faster on one run and 35–36% on the swapped run. That is the generic Pasta
  path, not blst. Raw: `raw/other-chains-plonk-kzg/zcash_halo2-pr796.md`.
- **Plan.** First measure the MSM size histogram that `loki-profile` already records. Then try two
  variants on the blst path: (a) one MSM per thread with no intra-MSM split, and (b) per-window
  parallel accumulation in a Rust Pippenger built on blst point ops. The asm-level blst cost per
  add is the reason (b) may lose.
- **Caveat.** blst's own internal Pippenger may already do some of this. Verify before porting.
- **Correctness.** Scheduling only. The existing `msm_matches_individual_multiplications` test
  (`msm.rs:227`) covers the result.

## 3. Vartime inversions in synthesis (synthesis, Track A)

- **Precedent.** zcash-halo2 PR #962 (merged 2026-09-28, commit `190f43d`): "We've never provided
  constant-time guarantees for proving." It adds `Assigned::evaluate_vartime` and uses it throughout
  the circuit logic. No timing is in the PR body. Raw: `raw/other-chains-plonk-kzg/zcash_halo2-pr962.md`.
- **Loki hook.** The rational advice assignments in `src/plonk/traces.rs` and the floor planner
  (`src/circuit/floor_planner/`). Synthesis is in the phase `witness_synthesis`, which the profile
  already times.
- **Expected gain.** Unmeasured. Each rational inversion is a field inversion, which is expensive
  even when batched.
- **Correctness.** Prover-only. Zero denominators must still be rejected as before.

## 4. Fixed extended cosets: memory versus speed (quotient, Track A, negative for speed)

- **Precedent.** axiom-crypto/halo2 PR #17 (merged 2023-11-18), a cherry-pick of taikoxyz/halo2 PR 6 and
  scroll-tech/halo2 PR 28. The fixed extended-Lagrange coefficients are no longer stored in the pk, so
  the prover recomputes them with an FFT. The shard proof went from 16.175 s to 16.476 s, and the
  aggregation proof from 79.420 s to 98.437 s. The author chose it for memory bandwidth. Raw:
  `raw/other-chains-plonk-kzg/axiom-crypto_halo2-pr17.md`.
- **Loki hook.** `src/plonk/keygen.rs:336-353` (`fixed_cosets`) and the `permutation_pk_cosets`
  that are stored in the pk. Loki's 16 GB RSS on the k20 parent is the reason to look at this. It is
  a memory lever, not a speed lever.
- **Rank rationale.** It is ranked 4th only because the RSS issue is real. It should not be used
  to make proofs faster.

## 5. Direct instance commits (recursion, Track A)

- **Precedent.** zcash-halo2 PR #938 (merged 2026-10-09, commit `2d54aaf`). `verify_proof` and
  `create_proof` commit to instance values directly, not after zero-padding to the domain, and
  batch-normalise at 4 or more. Raw: `raw/other-chains-plonk-kzg/zcash_halo2-pr938.md`.
- **Loki hook.** `src/plonk/prover.rs:~449-500` (`compute_instances`). Loki builds a full-domain
  Lagrange poly and calls `CS::commit_lagrange` for committed instances (line ~484).
- **Expected gain.** Small. The PR gives no number, and `msm_cpu` already drops zeros
  (`msm.rs:182-186`). The instance commitment is probably not on the k20 critical path.

# Items reviewed and not recommended for Loki

| Item | Source | Why not |
|---|---|---|
| CycloneMSM batch-affine with shared inversion | halo2curves #130, #168, #169 (merged 2024), axiom #36 | 30–40% gain on M1 against an older halo2curves MSM, and 20% behind a curve-specific implementation. blst already is that implementation for BLS12-381 G1. Mentions BN254 and Pasta only. |
| Signed Booth recoding | zcash #937 (merged 2026-10-09) | Only the generic `best_multiexp` path. Loki's G1 path is blst. Note: #937 also reports a serial single-worker predicate change (#938). |
| Scroll #71 logup with `par_invert` and no BTreeMap | scroll-tech/halo2 #71 (merged 2023-12) | Changes the lookup argument, so Track B. The counting-map part is already in Loki. |
| Prescale SRS bases for fixed bases | c-kzg-4844 #628 (7.7% at precompute=8 on cells proofs, M1 single thread) | Only for fixed bases known at keygen. Loki's per-proof commits use variable bases. |
| Speed up SRS loading (skip subgroup check) | c-kzg-4844 #299 (2050 ms to 71 ms, 28x) | Setup time only, not proving. `src/poly/kzg/params.rs` shows no subgroup check to remove. |
| Lagrange wire commits | barretenberg #94 | Loki already commits in Lagrange form (`src/poly/kzg/mod.rs:83`). |
| Parallel Gemini folding | barretenberg #550 | Gemini is Aztec's multilinear PCS, not SHPLONK. Track B at best. No timing in the PR. |
| Goblin / ECCVM, UltraHonk | Aztec | Different proof system and curve cycle. Not transferable. |
| Scroll GPU prover | Snarkify (vendor claim of 1.25x against a competitor) | No measured GPU-vs-CPU number for Scroll found. Loki already has sppark. |
| ZPrize MSM/NTT winners | Jane Street FPGA on BLS12-377, about 5.08 s per 2^26 MSM | FPGA and BLS12-377. Not a BLS12-381 GPU number. |
| Load-balanced GPU MSM on BLS12-381 | TCHES 2024 (1.8–2.0x over cuZK on V100) | Already covered by the GPU MSM track (`LOKI-GPU-MSM.md`). |
| OPTIMSM FPGA | TCHES 2025, BLS12-381 2^24 MSM in 914 ms on one U55C | Hardware. |
| plonky2/3, boojum, kimchi | Polygon, zkSync, Mina | FRI or IPA, not KZG. Structural ideas only. |
| supraseal | Filecoin | `supranational/supraseal` did not resolve with `gh`. Not checked further. |
| Varuna, snarkVM | Aleo | Not checked. Marlin-style, not halo2-KZG. |
| Quotient-side items (Scroll #27 partitioned cosetFFT, Axiom #28 work-stealing `evaluate_h`) | scroll-tech/halo2, axiom | Owned by the quotient scout. Loki already has per-thread chunked `evaluate_numerator` (`src/plonk/evaluation.rs:320-335`). |

# Not verified

- No measured gain was found for any halo2-KZG BLS12-381 technique in another chain's repo.
- No halo2 lookup benchmark comparing permuted lookup against logup was found.
- `midnightntwrk/midnight-zk` was not searched (`gh search prs` flag error). It is the upstream of
  Loki's fork and should be checked by the midnight-zk scout.
- Loki's per-set `clone()` cost and the MSM size histogram at k20 are not measured. Both need
  `loki-profile` runs before any of items 1 and 2 is implemented.
