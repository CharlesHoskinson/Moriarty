---
title: "Loki speed-up scout: KZG commitments and the multiopen"
date: 2026-10-24
component: src/poly/kzg (multi_open, commit, MSM dispatch, CUDA adapter)
sources:
  - https://github.com/midnightntwrk/midnight-zk/pull/565 (fflonk optimizations, merged 2026-10-14; diff saved at speedups/raw/pr565-full.diff, comments at speedups/raw/pr565-comments.txt)
  - https://github.com/midnightntwrk/midnight-zk/pull/575 (fflonk verifier gadget, merged 2026-10-15)
  - https://github.com/midnightntwrk/midnight-zk/commit/050676dee (#554, prover-side query refactor; upstream main copy at speedups/raw/upstream-main-kzg-mod.rs and upstream-main-kzg-utils.rs)
  - https://zcash.github.io/halo2/design/proving-system/multipoint-opening.html
  - https://eprint.iacr.org/2021/1167 (fflonk paper, linked from search, not fetched in full)
  - https://blog.zksecurity.xyz/posts/kzg-1/ (BDFG20 batched openings, no prover numbers)
  - https://tches.iacr.org/index.php/TCHES/article/view/11791 (Elastic MSM, numbers from search snippet only)
---

# KZG commitments and the multiopen

## 0. Bottom line

The multiopen is about 19% of the k16 profile (211 ms of 1,127 ms). Even eliminating it entirely moves the k20 parent by at most about 1.5%. Scaling it 16x to k20 gives about 3.4 s against a 220 s parent. Items 1 to 4 below are worth doing because they are cheap and Track A, not because they move the headline number. The parent is dominated by phases outside this component.

## 1. What the crate does now

Loki's KZG is in `/home/charles/repos/loki-proofs/src/poly/kzg/`. The prover path is `KZGCommitmentScheme::multi_open` (`mod.rs:94-203`).

**Scheme.** The multiopen is the zcash halo2 multipoint-opening combination (per-set q_i and interpolated r_i, x1 within a set, x2 across sets, x3 evaluation point, x4 folding `f` with the q_i, final polynomial opened at x3). The difference from the halo2 book is the final opening: the book opens `final_poly` with an inner-product argument, while Loki opens it with a single KZG quotient `pi`. There is one combined quotient, so it is SHPLONK-shaped. GWC is not used. See section 4.

The test name `test_roundtrip_gwc` (`mod.rs:391`) is an inherited name, probably from PSE halo2's `kzg/multiopen/gwc` test. I did not verify that source. No GWC code path exists in this repo.

**Transcript and round structure** (`mod.rs:108-202`):

| Step | Lines | What happens |
|---|---|---|
| x1, x2 | 108-109 | Squeeze. x1 combines polys within a set, x2 combines sets. |
| Group queries | 111-117 | `construct_intermediate_sets` (`utils.rs:37-133`) groups queries by commitment and by point set. `q_polys[set]` is built with `poly.clone()` (line 116). |
| q_i(X) | 119-130 | `inner_product(polys, powers(x1))` per set. Each poly is cloned. |
| Sort sets | 140-146 | Clones `q_polys` and `point_sets` again. |
| f_i(X) | 148-164 | For each set, `kate_division` by each point in turn (line 154). Each division is a full serial O(n) pass. The result is zero-padded to 2^max_k. |
| f(X) | 163 | `inner_product(f_polys, powers(x2))`. |
| Commit f | 166-167 | `Self::commit` (MSM over the coefficient SRS). Written to transcript. |
| x3 | 169 | Squeeze. |
| q_i(x3) | 173-177 | `eval_polynomial` per set, written to transcript. |
| x4 | 179 | Squeeze. |
| final_poly | 181-191 | `inner_product` of q_polys and f with powers of x4. |
| v | 192 | `eval_polynomial(final_poly, x3)`. |
| pi | 194-200 | `kate_division(final_poly - v, x3)`, then `Self::commit`. Written to transcript. |

**Why the iterated division is correct.** Dividing `q` by `(X-p1)` and then by `(X-p2)` yields `(q - r)/((X-p1)(X-p2))`, where `r` is the Newton-form interpolant of `q` on the set. So the prover does not subtract `r` explicitly. The verifier computes `r` with `lagrange_interpolate` (`mod.rs:295`) and checks `f_eval = (q(x3) - r(x3)) / prod(x3 - p_i)` (`mod.rs:291-302`).

**Verifier** (`multi_prepare`, `mod.rs:205-364`). It builds `DualMSM` terms. There are no polynomial operations, so it is not on the prover's critical path.

**Division and evaluation helpers.**
- `kate_division` (`src/utils/arithmetic.rs:97-115`) is a plain serial loop. Each step depends on the previous one: `tmp = (r - tmp) * b`. It cannot use SIMD or threads as written.
- `eval_polynomial` (`arithmetic.rs:55-76`) is Horner, chunked over Rayon when `n*2 >= threads`.
- `inner_product` (`arithmetic.rs:317-326`) clones each poly and reduces serially across polys. Each `Mul<F>` is parallel (`poly/mod.rs:500-520`), but the clones are serial memcpys.
- `evals_inner_product` and `msm_inner_product` (`arithmetic.rs:329-370`) are the verifier-side combiners.

**Batching the openings.** Queries are grouped by commitment pointer (`PolynomialPointer`, `query.rs:75-93`), so no hashing is needed. Point sets are sorted by size (`mod.rs:140-146`). Only x1..x4 and the sort order change the bytes, so any reordering needs care.

## 2. Hidden costs on the prover path

These are the places where the code does more work than the transcript needs. Status against upstream main is noted for each.

**H1. The prover evaluates every query polynomial and never uses the result.** `construct_intermediate_sets` calls `query.get_eval()` once per query (`utils.rs:118`). For `ProverQuery` that is `eval_polynomial(poly, point)`, a full O(n) pass (`query.rs:88-89`). The `evals` field is then read only in `multi_prepare` (`mod.rs:237, 254`). The prover's `multi_open` reads only `.commitment.poly` and `.set_index` (`mod.rs:116`). Many of these points are already evaluated for the transcript in `write_evals_to_transcript` (`plonk/prover.rs:732-790`).
- **Upstream status:** already fixed on midnight-zk main by #554 (commit `050676dee`, "prepare the PCS interface for fflonk"). The prover now passes `E::Fr::ZERO` as the eval and comments that the evaluations are only read by the verifier (upstream `kzg/mod.rs:289-294`). It is not in #565 (the #565 diff does not touch `kzg/utils.rs`), so it is not an independent find upstream. It is still present in Loki's 0.8.3 fork, so it is a port, not a new idea.
- Track A. The prover never reads `evals`, so the output is identical.

**H2. Clones.** Each polynomial is cloned into `q_polys` (`mod.rs:116`). `q_polys` and `point_sets` are cloned again for the sort (`mod.rs:143-144`). Each f_i is cloned in the fold (`mod.rs:153`), and in the zip (`mod.rs:151`). `final_poly` allocates again (`mod.rs:196`).
- **Upstream status:** the `q_polys` clone is also gone upstream (#554). Upstream uses `label_to_poly` and pushes `&Polynomial` references (upstream `kzg/mod.rs:296-304`). The f_i and final_poly copies remain in upstream; #565 changed the f_i path to `divide_by_roots` on slices (see section 3, item 1).
- At k20 each poly is 32 MiB, so the copies are real memory traffic.

**H3. Serial division.** There are sum over sets of |points in set|, plus one, serial divisions. Each runs over the full vector. The divisor is linear, so the division is a linear recurrence. It can be parallelized with a chunked scan, as in section 3.

**H4. Commit calls go through a host round trip on the GPU path.** `msm_specific` (`msm.rs:166-176`) converts the scalars to the wire format on the CPU for every call (`cuda/backend.rs`, `scalars.iter().map(convert::scalar)`). It also copies them to the device, and returns to the host. Nothing stays resident between the quotient, the openings, and the MSMs (section 3, item 5).

## 3. Techniques, ranked by expected gain / effort

Numbers from #565 are measured by the PR author at k=14 on one machine. The k20 figures below are my estimates, not measurements.

| Rank | Technique | Track | Hook (file:fn) | Effort | Expected gain on k20 parent | Correctness risk vs Lean contract |
|---|---|---|---|---|---|---|
| 1 | **Port #565's KZG-side multiopen change: one-pass `divide_by_roots`** replaces the per-point `kate_division` fold for each set's f_i. Also `eval_interpolant` on the verifier. | A. The quotient is unique, so the f commitment bytes cannot change whichever division algorithm is used. Only the transcript order must be preserved. | `kzg/mod.rs:multi_open` f_polys block, `utils/arithmetic.rs:divide_by_roots` | M | The PR's KZG path went 590 ms (main) to 415 ms at k=14, about -30%. That bundles other changes, so the part from `divide_by_roots` alone is not separable. Treat it as an upper bound on this item. | Low. The PR's `test_divide_by_roots` asserts equality with the iterated `kate_division` fold for arbitrary and 4th-root sets. Keep that test. |
| 2 | **Port #554's prover-side query path**: prover passes zero evals (H1) and references in `q_polys` (H2). | A. The prover never reads `evals`. | `kzg/utils.rs:118`, `kzg/mod.rs:113-117` | S-M | One full polynomial evaluation per query, and one poly copy per query. Q x n Horner steps. Measure Q first. | None on the verifier path. #554 is a wide label refactor, so port only the prover-side hunks. |
| 3 | **Clone-free f and final_poly combination** (remaining H2). Scale-add in place, drop the sort clones. | A | `kzg/mod.rs:140-191` | S-M | Memory traffic, a few full-size copies per set removed. Likely a few percent of the multiopen phase. | None if the arithmetic order is kept. |
| 4 | **Parallel `kate_division`** (H3). Chunked linear-recurrence scan: each chunk computes local values, then a fix-up pass multiplies by powers of b. Applies to the pi division and any remaining kate calls. | A | `utils/arithmetic.rs:kate_division` | M | Estimated 2 to 3 times the mults but spread across cores. At k20 a serial pass is about 30 ms (estimate: about 30 ns per dependent Montgomery step). About 8 to 10 passes per proof. Could save about 150 ms serial, less on 4 threads. | Low. Output must be identical. Test against the serial version. |
| 5 | **GPU residency for q-polys, division and f/pi** (H4). Keep h-pieces and advice in VRAM, do linear combinations on device, implement the division as a device scan. | A | `kzg/msm/cuda/*`, `domain.rs:gpu_fft`, `vanishing/prover.rs:construct` | L | Removes the NTT round trips (`domain.rs:377-397`) and per-MSM scalar conversion. Bounded by the CPU phases the brief names, so the gain is uncertain. | Medium. More code on the GPU path to validate. |

**Track B (new keys or new proof format), not recommended for speed alone:**
- **SHPLONK to GWC.** This changes the proof format and the verifier. Nothing in the sources I read shows a prover speedup for GWC over SHPLONK. I could not find cost numbers in search.
- **fflonk (midnight-zk #565/#575).** At k=14 in #565, KZG prove is 415 ms, while fflonk prove is 422 ms (t=1), 479 ms (t=2), 536 ms (t=4), and 955 ms (t=16). fflonk is slower to prove at every t. Verify is faster for t>=2 (2.28 ms at t=4 vs 2.68 ms KZG). Proofs are smaller (3840 B at t=4 vs 4384 B KZG). So fflonk is not a prover speedup. It is a Track B proof-size and verifier change. The KZG-side gain in the same PR is what ports (item 1).

**Fixed-base precomputation and the Lagrange SRS.**
- The CPU MSM (`msm.rs:178-220`) uses blst `multi_exp` Pippenger in parallel chunks. It has no precomputed window tables. Such tables for 2^20 bases are large. Search results give no halo2-specific number, and the Elastic MSM figures (TCHES 2024, search snippet only) are not verified against the paper. Gain unknown, effort L, memory cost high. Not in the top 5.
- The GPU adapter (`cuda/msm.cu`, `loki_gpu_prepare`) keeps the bases resident in VRAM. Its `resident_t` constructor is called with default arguments (`msm.cu:26-31`). I did not confirm whether sppark's precompute window is set there. Check `sppark` `msm_t` before assuming precomputation.
- `backend.rs` keeps a 2-entry cache of resident SRS arrays, keyed by pointer plus full coordinate compare. Each call does an O(n) coordinate compare (`same_coordinates`), and a host copy `source: bases.to_vec()` is kept per entry (about 150 MiB at 2^20). A third distinct base slice evicts one entry and re-uploads the SRS. Check that no MSM uses a different slice length, such as a downsized SRS.
- Lagrange SRS: `g_lagrange` is the inverse FFT of `g` (`utils/arithmetic.rs:34-50`), computed at setup and in `downsize` (`params.rs:40-49`). `commit_lagrange` (`mod.rs:83-92`) is used for advice, permutation and lookup columns, which are in evaluation form. The multiopen itself uses only the coefficient SRS (`commit`, `mod.rs:74-81`). The Lagrange SRS doubles SRS memory and is not on the multiopen path.

**Batching the MSMs.** The two multiopen MSMs (f and pi) are sequential. pi depends on x3 and x4, which depend on f's commitment. They cannot be merged.

## 4. Which variant is this crate?

**SHPLONK-shaped, not GWC.** The crate matches the halo2 multipoint-opening combination described in the zcash book, with a single KZG quotient `pi` for the final opening. Concretely: one combined `f`, one `pi`, x3 for the evaluation point, interpolated `r_i` on the verifier side. GWC instead commits one quotient per distinct point. The code has no GWC path, and the test name is inherited. Changing to GWC changes the proof bytes and the verifier, so it is Track B.

## 5. Risks against the Lean contract

- Any change to x1..x4, the set sort order, or the zero-padding (`mod.rs:156`) changes proof bytes. Items 1-4 must keep the transcript order.
- The `truncated-challenges` feature (`mod.rs:27-28, 122-185`) changes the challenge derivation. Test both feature settings. Note that upstream main removed this feature in #580, so the fork and upstream diverge here.
- Item 1 changes the division code. Test it against stock verifier output on the existing test suite and on a Sherlock k20 proof.

## 6. First measurements to take

1. Count queries Q and point sets S per proof, and time `construct_intermediate_sets`, the `get_eval` loop, `kate_division` alone, and the two MSMs separately. The `loki-profile` feature (`PhaseGuard "kzg_multiopen"`, `mod.rs:103-104`) already covers the phase as a whole.
2. Count `Vec` clones and their total bytes.
3. Check whether the GPU SRS cache ever sees a third entry during a proof.

These will confirm or reject items 2-4 before any code is written.

## 7. Raw captures

Saved under `/home/charles/repos/Moriarty-factory/wiki-llm/loki/speedups/raw/`:
- `pr565-full.diff` (full PR diff, 1,047 lines)
- `pr565-comments.txt` (review comments; reviewers approved, and the one substantive suggestion, precomputing interleaved bases in params, was split into issue #577)
- `upstream-main-kzg-mod.rs`, `upstream-main-kzg-utils.rs`, `upstream-main-arithmetic.rs` (current upstream main, post-#565)
