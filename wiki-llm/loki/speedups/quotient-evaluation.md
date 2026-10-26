---
title: "Loki speed-up scout: quotient (vanishing argument) evaluation"
date: 2026-10-24
component: quotient-evaluation
sources:
  - https://github.com/axiom-crypto/halo2-gpu (halo2_proofs/cuda/src/quotient.cu, quotient_lookups.cu, permutation_quotient.cu, include/kernel/quotient.h) [captured](raw/quotient/)
  - https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/quotient-polynomial-evaluation [captured](raw/quotient/snarkify-quotient.html)
  - https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover (captured in ../raw/04/snarkify-docs.md)
  - https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2 (captured in ../raw/04/icicle-halo2-v2.md)
  - https://raw.githubusercontent.com/ingonyama-zk/halo2/main/halo2_proofs/src/icicle.rs (captured in ../raw/04/icicle-adapter.md)
  - https://github.com/Tao-Lu-123/cuZK (captured in ../raw/04/cuzk-code.md; Groth16, no quotient kernel)
  - Loki source: src/plonk/evaluation.rs, src/plonk/prover.rs, src/plonk/vanishing/prover.rs, src/poly/domain.rs, src/plonk/keygen.rs, src/plonk/lookup.rs, src/plonk/permutation.rs, src/plonk/trash.rs
---

## Question

Which published or open-source techniques speed up Loki's quotient step (`compute_nu_poly` -> `Evaluator::evaluate_numerator` -> `divide_by_vanishing_poly` -> `extended_to_coeff` -> h-piece split), and which are usable under the Track A rule (identical proof bytes, stock verifier, same VK)? This note feeds the W10 GPU quotient design.

## Summary

- **CODE:** Loki evaluates the quotient on a single extended domain of size `N = 2^extended_k`, where `extended_k` is the smallest `k' >= k` with `2^k' >= n*(d-1)` and `d = cs.degree()` (`src/poly/domain.rs:47-50`). At k20 the brief's `N=2^22` implies `d` is 4 or 5; the parent's real `d` is still an OPEN QUESTION (node 03).
- **CODE:** Lookups force `d >= 4`: `required_degree = max(2 + input_deg + table_deg, 4)` (`src/plonk/lookup.rs:52-75`). Permutations need `d >= 3`, chunk length `d-2` (`src/plonk/permutation.rs:46-80`). So any circuit with a lookup has `N = 4n` at best. `d=3` (`N=2n`) is impossible with lookups; it is only available to lookup-free circuits.
- **CODE:** The evaluator makes one full pass over the `N`-length `values` vector per argument: custom gates (`evaluation.rs:328`), permutations (`:358`), each lookup (`:446`), each trashcan (`:519`). Each pass reads and writes 2x`N` field elements. Each lookup also materializes three extended cosets (product, permuted input, permuted table) by coset NTT, inside the loop.
- **CODE:** Expressions are compiled to a `GraphEvaluator` with structural CSE (`add_calculation`, `evaluation.rs:607`). CSE is per graph. Gates share one graph; lookups and trashcans each have their own, so their advice/fixed `Store` nodes are not shared. The evaluator is a per-row interpreter, one `match` per calculation, with no batching across rows.
- **CODE:** `keygen.rs:336-340` stores `fixed_cosets` for every fixed and selector column on the extended domain in the proving key. At `N=2^22` that is 128 MiB per column, resident for the life of the pk.
- **Top GPU precedent, measured:** Snarkify cuSnark Eval_h, 4x RTX 3090 vs EPYC 7702 CPU: `1069.09 s -> 42.13 s` (25.4x) on 2^20 rows x 1135 columns, and `66.58 s -> 5.68 s` (11.7x) on 2^25 rows x 5 columns. The method is a single opcode-list interpreter kernel, with a Python script that splits the graph into launches sized to device memory. Licence and BLS12-381 support are unestablished. (raw/quotient/snarkify-quotient.html)
- **Axiom halo2-gpu (BN254):** a row-tile interpreter kernel with per-tile intermediates in global memory, row-range chunking with a rotation halo (`extend_before_start/end`), and a device-memory-driven max-rows query (`_halo2_evaluate_h_max_rows`). It has separate lookup and permutation quotient kernels. No published CPU/GPU ratio for the quotient alone. (raw/quotient/src_quotient.cu, include_kernel_quotient.h)
- **ICICLE-Halo2 (BN254 adapter):** a naive intermediate-per-node layout needs `2^k x 100 x 64 B`, about 26 GB at k=22, which failed on a 4090. The fix was batching rows. The adapter lowers Loki-style `Calculation` lists to full-vector ops, so it has the same memory shape unless batched. No ratio for this stage is published.
- **Recommendation:** Track A first. (1) Fuse the four `values` passes into one row loop, which is bit-identical. (2) Replace the per-graph `Store`s and build one shared CSE graph. (3) Build the GPU quotient as a row-tiled, row-chunked opcode interpreter with a halo, batched to device memory, and differential-tested against the CPU `h` vector. Track B only if the quotient degree changes (lever 5). Before W10 commits to a design, measure the parent's quotient share with `loki-profile`. The only split on record is a k16 fixture.

## Ranked table (top 5)

Gain is estimated for the k20 parent. "Track A" keeps the proof bytes and VK. "Effort" is in engineer-days, rough. Each entry is an estimate, not a measurement.

| # | Technique | Track | Expected gain (k20 parent) | Effort | Loki hook | Lean/correctness risk |
|---|---|---|---|---|---|---|
| 1 | Fuse gate, permutation, lookup and trash passes into one row-ordered loop (Horner order kept per row) | A | Removes `(2 + L + T)` full read/write passes of `values` (128 MiB each at N=2^22). CPU gain moderate, probably 1.2–1.8x on the evaluation stage (ESTIMATE). It is also the prerequisite for the GPU tile design. | 2–4 d | `evaluation.rs:317-520` (`evaluate_numerator`) | Low. Each `values[idx]` depends only on row idx plus read-only cosets. The per-row Horner order must match exactly. Test with `h` bit-equality. |
| 2 | Single shared CSE graph across gates, lookups and trashcans; drop `Store` nodes by letting consumers reference columns directly | A | Cuts interpreter dispatch and duplicate loads. Gain depends on the calc count (unmeasured). Likely 10–30% of the non-FFT evaluation time (ESTIMATE). | 1–3 d | `evaluation.rs:214-282` (`Evaluator::new`), `:607-660` (`add_calculation`, `add_expression`) | Low. The expression algebra is unchanged. Keep the `Mul/Add` operand ordering to preserve CSE hits. |
| 3 | GPU row-tile opcode interpreter with row-range chunking and rotation halo; intermediates batched to device memory (Axiom/Snarkify pattern) | A (BLS port) | Snarkify precedent: Eval_h 11.7x–25.4x (different CPU baseline and circuits). For Loki's `N=2^22` case, estimate 5–15x on the stage, pending the port. | 15–25 d | new `loki-cuda-ntt` quotient entry point beside `cuda-ntt/src/lib.rs:33`; hook from `compute_nu_poly` (`prover.rs:667`) | Medium. Must match the CPU `h` for `trash_challenge`, trash polys, committed instances and permutation `last_rotation`. Use Axiom's `_halo2_evaluate_h_max_rows` pattern to bound memory. |
| 4 | Keep `pk.fixed_cosets` and permutation/`l_*` cosets resident on device (or stream them by row chunk); avoid per-call H2D | A | Removes the largest per-proof transfers. At `N=2^22`, 128 MiB per fixed column, so 40 fixed columns is about 5 GiB of pk cosets. Saves transfer and VRAM. | 3–5 d | `keygen.rs:336-340`, `mod.rs:345` | Low. Read-only tables; bind the cache to the VK digest and domain. |
| 5 | Reduce the quotient degree `d` via gate layout, with care for the permutation and lookup costs | B (new VK) | `d=5 -> 4` keeps `N=4n` but cuts h pieces from 4 to 3, so one fewer size-n quotient MSM (about 25% of `quotient_commitment`, which was 16.9% on the k16 fixture). It adds permutation sets (chunk length 3 -> 2), so it can add commitments and quotient work. `d=3` halves `N` but only for lookup-free circuits. | 5–15 d (circuit work) | `domain.rs:37-50`, `lookup.rs:52-75`, `permutation.rs:46-80`, `vanishing/prover.rs:112-120` | Medium. New VK, new keys. Stock verifier reads the h count from the VK, so it still accepts, but the proof format changes with the VK. |

Techniques considered and rejected for the top five: CPU-side chunked evaluation that avoids the full `N` (see details, lever 6). Lookup-coset reuse (lever 7) is a small win.

## Details

### 1. Current Loki quotient structure (what the GPU must reproduce)

- **Extended domain.** `EvaluationDomain::new(j, k)` sets `quotient_poly_degree = j-1`, then grows `extended_k` until `2^extended_k >= n*(j-1)` (`src/poly/domain.rs:37-50`). `j = cs.degree()` (`src/plonk/mod.rs:176`, `keygen.rs:45-47`). The degree is the max over permutation (3), lookups (`max(2+in+tab, 4)`), trashcans (`max(2, deg)`), gates and `minimum_degree` (`circuit.rs:2325-2337`).
- **Input cosets.** `compute_nu_poly` materializes every advice and instance column on `N` (`prover.rs:667-700`). Fixed cosets come from the pk (`keygen.rs:336`). Each `coeff_to_extended` is a coset NTT of size `N` (`domain.rs:250-262`). These are global transforms, so no chunking removes them.
- **Evaluation.** `evaluate_numerator` (`evaluation.rs:285`) allocates `values` of size `N` (`:317`). Then it runs the gate, permutation, lookup and trash passes in that order, each computing `value = value*y + term` (Horner in `y`). Gates run through `GraphEvaluator::evaluate` per row (`:735`). Thread chunks are handled with `rayon::scope` (`:328-355`).
- **Lookups.** Per lookup: three coset NTTs (`:447-452`), then the constraint pass (`:446-518`).
- **Output.** `divide_by_vanishing_poly` multiplies by precomputed `t_evaluations` (`domain.rs:318-337`). `extended_to_coeff` is an inverse NTT of size `N` (`:268-283`). `h` is truncated to `(n-1)*(j-1)` and split into `j-1` pieces, each blinded (`vanishing/prover.rs:98-130`).

### 2. Memory traffic (k20, `N=2^22`, 32-byte field elements)

- One extended column is 128 MiB. `values` is 128 MiB.
- Each full pass reads and writes `values`, so 256 MiB per pass, plus its input columns. The gate pass reads every fixed, advice and instance coset once per row. With 64 columns that is 8 GiB of reads.
- Each lookup adds three transient 128 MiB cosets and their NTTs.
- Peak live memory is dominated by the advice/instance cosets, the pk fixed cosets, `values`, and transient lookup and permutation cosets. Node 03 estimates 19 GiB for 64 live columns at `N=2^22`.
- Fusing passes (lever 1) turns the repeated `values` traffic into one read/write per row, with per-row reads from the column cosets. Tile-level reuse of halo rows (Axiom) makes the column reads roughly one per tile.

### 3. GPU kernels in the field

- **Axiom halo2-gpu (BN254, Apache-2.0 OR MIT, CUDA >= 12.8).** `quotient.cu` splits rows into blocks of 64 tiles x 32 rows. Each thread processes one row set in sequence, and interprets a per-expression rule list stored as 128-bit `Rule`s. Intermediates live in global memory, sized `num_tiles * num_rules`. The row range is chunked; each chunk copies its rows plus a halo computed from the rotation min and max (`compute_range`, `set_range_info`, `extend_before_start/end`, `quotient.cu:85-135`). Value-part Horner is done in-kernel with `y` and an optional `prev_values` seed, so several circuits can fold into one polynomial (`include_kernel_quotient.h:264-300`). Lookups and permutations have separate kernels (`quotient_lookups.cu`, `permutation_quotient.cu`). The field is BN254 (`field/alt_bn128.hpp`), so a BLS port is needed.
- **Snarkify cuSnark (licence and BLS support unestablished).** Quotient evaluation uses a single kernel that runs an opcode list. A Python script breaks the graph into sequential launches, so the number of in/out arrays per launch fits device memory. Measured: Eval_h `1069.09 -> 42.13 s` (2^20 x 1135 columns) and `66.58 -> 5.68 s` (2^25 x 5 columns), CPU baseline on EPYC 7702, 4x 3090 (raw/quotient/snarkify-quotient.html).
- **ICICLE-Halo2.** Gate, permutation and lookup evaluation are moved to the device. The naive intermediate layout needs `2^k x 100 x 64 B`, about 26 GB at k=22, which the 4090 could not hold. Batching by rows fixed it at a small time cost (raw/04/icicle-halo2-v2.md). The adapter lowers Loki `Calculation` lists to device vector ops (`create_calculation_data`, `inplace_mul/add` on full-length vectors; raw/04/icicle-adapter.md). That layout is the memory-heavy form.
- **cuZK.** Groth16 MSM and NTT. It has no quotient kernel. Not a source for this component.
- **Gap.** No published BLS12-381 Halo2 quotient kernel was found. The Snarkify and Axiom kernels are the only ones with an interpreter design. Two searches for halo2 GPU quotient and ICICLE gate-batching returned nothing specific.

### 4. Expression compilation and CSE

- `GraphEvaluator::add_expression` (`evaluation.rs:624-727`) flattens each expression to `Calculation`s with canonicalized `Add`/`Mul` operand order, constant folding for 0/1/2, and `Square`/`Double` specializations.
- `add_calculation` deduplicates by linear search (`:607-620`). The search is quadratic in the number of calcs, but compile-time only.
- CSE is per graph. Gates share `self.custom_gates`. Lookups (`:229-258`) and trashcans (`:260-282`) each get their own graph, so the `Store` nodes for the same advice column are repeated per graph. Also, `Store` nodes for every query are themselves intermediates, which costs one write and one read per row per query.
- Permutation constraints bypass the graph and read cosets directly (`:358-445`).
- **Action:** count calcs and intermediates per graph (a one-line debug print, not committed) to size lever 2 and the GPU intermediate buffer.

### 5. Chunked evaluation and avoiding the full extended domain

- **What cannot be avoided.** The output `h` is a degree `(j-1)n` polynomial, so its inverse NTT needs the full `N`. Extended columns for gates and lookups also need full coset NTTs. Chunking therefore does not reduce NTT count.
- **What chunking does reduce.** Evaluation can run over row ranges, provided each range carries a halo of `max|rot| * rot_scale` rows on each side (Axiom's `compute_range`). That bounds transient memory and enables streaming of the column cosets once they are resident. Axiom's `max_rows` query is the right model for W10.
- **Degree lever (lever 5).** Smaller `d` shrinks `N` only at thresholds: `N=4n` for `d in {4,5}`, `N=2n` for `d=3`, `N=n` for `d=2`. Lookups block `d=3`. Going `5 -> 4` reduces h pieces and quotient MSMs without changing `N`, but it lowers the permutation chunk length from 3 to 2. That adds permutation sets and their commitments, which can cost more than it saves on wide circuits (1135 columns in the Snarkify case). This must be measured on the parent.

### 6. Lookup coset reuse and early release (small wins)

- `B::coeff_to_self` for each lookup's `product`, `permuted_input` and `permuted_table` (`evaluation.rs:447-452`) are computed in the lookup loop. They are not reused across lookups, and they cannot be shared with the lookup commitments already computed.
- `advice_cosets` and `instance_cosets` are kept until the end of `evaluate_numerator`. Columns used only by gates could be dropped after the gate pass if the fusion in lever 1 is not done. Under lever 1 the drop order is not a concern.

### 7. Correctness constraints (Lean contract, Track A)

- The h polynomial must match bit-for-bit. Any kernel must reproduce: the `Horner` in `y` per argument and per row, the `r_last`/`r_next` rotation index mapping (`get_rotation_idx`, `evaluation.rs:12`), `rot_scale = N/n`, the `ZETA` coset (`domain.rs` `g_coset`), `t_evaluations` division, and blinding applied after the split.
- BN254 kernels (Axiom, ICICLE adapter) cannot be reused unchanged. Port the field arithmetic first, then run a differential test on random and edge inputs (node 03's arithmetic test list).
- Lookup and trash expressions use `theta`, `beta`, `gamma`, `trash_challenge` and committed-instance columns. A missing term breaks the proof only at verification, so the test must verify with the stock verifier, not only compare `h`.

## What would have to be true

- **Lever 1 (fusion):** the per-row Horner order is preserved. The gain holds only if the `values` passes, not the column reads, dominate the CPU evaluation. Profile first.
- **Lever 3 (GPU tile kernel):** the parent's quotient share is large enough (the k16 fixture is 18.6%; the parent is unmeasured). The BLS port must be exact. Per-tile intermediates must fit VRAM at `N=2^22` (Axiom sizes them as `num_tiles * num_rules`, so the rule count matters).
- **Lever 5 (degree):** the circuit's permutation column count is small enough that the added sets cost less than the saved h MSM. Otherwise the change is a loss.

## Open questions

- The parent's degree `d`, column counts, lookup count, number of trashcans, and calc count per graph. None is recorded in the repo. Run `loki-profile` on the parent before W10 fixes the design.
- Whether the k20 parent's quotient share resembles the k16 fixture's 18.6%. Snarkify's text says Eval_h is 1/3 to 2/3 of total proof time for their circuits, which is not evidence for Loki.
- cuSnark licence, and whether its BLS12-381 path is maintained.
