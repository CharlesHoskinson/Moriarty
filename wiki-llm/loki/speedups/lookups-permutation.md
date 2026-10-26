---
title: Lookup and permutation arguments: speed-up scout
date: 2026-10-24
component: lookups-permutation
sources:
  - /home/charles/repos/loki-proofs/src/plonk/lookup/prover.rs (lines 58-145, 155-289, 374-459)
  - /home/charles/repos/loki-proofs/src/plonk/permutation/prover.rs (lines 85-160)
  - /home/charles/repos/loki-proofs/src/plonk/trash/prover.rs
  - /home/charles/repos/loki-proofs/src/utils/arithmetic.rs (lines 172-205)
  - /home/charles/repos/loki-proofs/LOKI-CPU.md, LOKI-PROFILE.md, LOKI-QUINT.md
  - /home/charles/repos/Moriarty-factory/wiki-llm/loki/nodes/loki-01-prover-profile.md, loki-04, loki-06
  - /home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/astra-2/PROPOSAL.md, 10x/helpers/haiku-06/astra-2-2.md
  - /home/charles/repos/midnight-zk @ 31435e9 (proofs/src/plonk/logup.rs, logup/verifier.rs)
  - https://eprint.iacr.org/2022/1530 (LogUp, Haboeck)
  - https://eprint.iacr.org/2023/1216 (Lasso)
  - https://arxiv.org/pdf/2402.02675 (ezkl, LogUp in halo2 fork)
  - https://github.com/axiom-crypto/halo2-gpu (captured in raw/04/axiom-tree.md)
  - https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2 (captured in raw/04/icicle-halo2-v2.md)
  - https://github.com/matter-labs/zksync-crypto-gpu (captured in raw/04/zksync-gpu-tree.md)
---

# Lookup and permutation arguments

## Summary

- Track A (proof format, VK, SRS and transcript unchanged) can only touch the
  CPU preparation and grand-product phases. Most of the easy CPU work has
  already landed. The remaining Track A gains are small.
- The Sherlock parent is not yet inventoried. The "1.03M table rows" figure
  is a historical Fenrir proxy (two-child node, 1,032,465 table rows; four-child
  artifact, same count). It is not a measurement of the Sherlock parent. Treat it
  as a sizing hint only.
- LogUp and smaller limb lookups both change the protocol or the key. Both are
  Track B.
- Loki has no GPU sort, prefix-product or batch-inversion kernel. Only
  `cuda/msm.cu` and `cuda-ntt/cuda/ntt.cu` exist.

## Loki code, as it is now

### Permuted lookup preparation (`src/plonk/lookup/prover.rs`)

- `commit_permuted` (58-145) compresses input and table expressions with theta
  (`compress_expressions`, 81-99). It then calls `permute_expression_pair`
  (108) and commits both permuted columns (124-135).
- `permute_expression_pair` (374-459):
  - the input is truncated to `usable_rows` and sorted with `par_sort` (391);
  - table multiplicities go into a `HashMap` (395-398);
  - repeated inputs are assigned the leftover table values (401-423);
  - leftovers are sorted with `sort_unstable_by` before placement (426-432);
  - blinding rows are added from the RNG (435-436).
- The HashMap-plus-sorted-leftovers scheme was chosen to make the output
  deterministic. LOKI-QUINT.md and the wiki notes say the stock HashMap
  iteration broke byte-reproducibility.

### Grand products

- Lookup product (`commit_product`, 155-289): the denominators
  `(a'+beta)(s'+gamma)` are formed in parallel (186-194). They are
  batch-inverted with `batch_invert_parallel` (198). Numerators are multiplied in
  (205-212). `prefix_product` (231) gives the running product. The debug-only
  check is at 239-276. The product is committed and converted to coefficients
  (278-279).
- Permutation product (`src/plonk/permutation/prover.rs` 85-160): the same
  pattern. Denominators are batch-inverted (107), numerators are multiplied in
  (in the loop after 107), and `prefix_product` runs at 143.
- Trash (`src/plonk/trash/prover.rs`): one compressed expression, one
  commitment, one opening. Negligible.

### Shared CPU helpers (`src/utils/arithmetic.rs`)

- `batch_invert_parallel` (172-181): serial batch inversion below 1024 elements,
  otherwise independent chunks on Rayon.
- `prefix_product` (184-205): chunk-local scans, then a serial cross-chunk pass,
  then a parallel rescale. Zero factors and the initial value are preserved.

Note: LOKI-PROFILE.md (k16) still lists the permutation product as a serial
prefix scan (`loki-01` node says so too). That reading predates the
`prefix_product` change in LOKI-CPU.md. Re-profile before using its
permutation/product percentages.

## Measured numbers

| Source | Measurement |
|---|---|
| LOKI-PROFILE.md, k16, 4 threads, single proof, 1,127.3 ms total | lookup_prepare 54.4 ms (4.8%); permutation_products 45.4 ms (4.0%); lookup_products 78.0 ms (6.9%); trash 8.9 ms (0.8%). Lookup + permutation preparation and products together are about 16.5% of the k16 proof. |
| LOKI-CPU.md, k16 kernel bench (65,536 rows, 256 table values) | HashMap counts 0.99 ms; BTreeMap 5.67 ms; HashMap plus sorted leftovers 1.08 ms; stable input sort 18.2 ms; parallel stable sort 6.4 ms (selected, 2.85x). |
| LOKI-CPU.md, full proof, 15 proofs | Final stage 1781 ms vs reference 1767 ms median. The kernel gain did not show end to end on the shared host. |
| Fenrir node-06 proxy (k20, two-child) | 574,040 advice rows; 1,032,465 table rows. Source: `nodes/loki-06` and `[REPORT:117]` in Fenrir. |

Scaling: at k20 the Sherlock parent measures 220 s on 4 threads. If the k16
phase shares (about 16.5% for lookup and permutation work) carried over, that
work would be about 36 s of the 220 s. This is a proxy, not a measurement. The
Sherlock parent has heterogeneous children and an unknown lookup count.

## The table-size question (1.03M rows)

- `nodes/loki-06` says: "Fenrir's recorded two-child node has 574,040 advice rows
  and 1,032,465 table rows" and "These are historical homogeneous-node
  measurements, not a measurement of the current heterogeneous Sherlock parent."
- `nodes/loki-01` gives the same table count for the four-child artifact
  (`keygen.stats.json`), and says it is "a related workload, not the owner's
  exact Sherlock run."
- The design doc (`LOKI-DESIGN.md`, line 28) says the same thing about the two-child
  node.
- Conclusion: use 1.03M as the table-height proxy for the Sherlock parent. Do not
  quote it as the Sherlock value until the parent is inventoried.
- For scale: a k20 domain has 2^20 = 1,048,576 rows. Usable rows are
  2^20 minus blinding minus 1. So 1.03M table rows nearly fill the domain.

## Techniques

### Track A: keep proof format, VK, SRS and transcript

| # | Technique | Where it could hook | Expected gain on k20 | Effort | Correctness risk vs Lean contract |
|---|---|---|---|---|---|
| A1 | GPU grand products: device batch inversion plus device prefix scan for the permutation and lookup products, then commit on GPU | `commit_product` (lookup prover 155-289), permutation prover 85-160, `batch_invert_parallel`, `prefix_product` | Permutation and lookup products are about 11% of the k16 proof. The CPU share is already reduced. Realistic end-to-end gain is about a few percent, unless the grand products stay resident for the GPU quotient (LOKI-DESIGN L4), where they save transfers | Medium-high. Needs a Montgomery-exact BLS12-381 scalar-field kernel, a scan kernel, and a device-to-host path that writes `z` back in order | Low if the field arithmetic is checked exactly against CPU. The product must match `prefix_product` on zero factors and the initial value |
| A2 | GPU permuted-input sort (`par_sort` on 2^20 field elements) | `permute_expression_pair` 391 | Small. The CPU sort is about 6 ms at 65k rows, so about 100 ms per lookup at 1M rows (extrapolated, not measured). Per-lookup gain is tiny against 220 s | High: 255-bit keys, radix sort over four 64-bit limbs. Precedent: zkSync `device_radix_sort.cu` uses CUB on u64 keys, not field elements | Low. Equal field values are identical, so any correct sort gives the same permuted column |
| A3 | GPU batch inversion alone (ICICLE VecOps) | `batch_invert_parallel` | Small. One inversion plus about 3 multiplications per element on CPU is already cheap. The ICICLE "200x at 2^22" figure is a vendor claim for BN254 VecOps and includes transfer-free operation | Medium | Low |
| A4 | Keep the sorted-leftover HashMap placement and parallel stable sort (already landed in LOKI-CPU.md) | `permute_expression_pair` 395-432 | Already realised. LOKI-CPU reports 2.85x on the sort kernel and a noisy end-to-end result | Done | Done. Keep the deterministic order |

Track A verdict: none of the A items has a large expected gain on the k20
parent. The sort and inversion are not the bottleneck. The grand products are
only a meaningful target when they run on the GPU together with the quotient
(LOKI-DESIGN L4).

### Track B: needs new keys, a new VK, or a new protocol

| # | Technique | Source | Why Track B | Expected gain | Effort | Risk |
|---|---|---|---|---|---|---|
| B1 | LogUp instead of halo2 permuted lookups | Haboeck, eprint 2022/1530; midnight-zk PR #480 (merged 31435e9, Jul 21 2026) and PR #515 (LogUp as a generic argument, merged Sep 24 2026); ezkl (arXiv 2402.02675) uses LogUp in a halo2 fork | Changes the transcript, the argument, and the VK. Removes the permuted columns, so there is no sort and no permute step. Commitments: 3 for both at c=1; evaluations: 4 for LogUp vs 5 for halo2 (haiku-06 `astra-2-2`). The installed `midnight-proofs` 0.8.3 and `midnight-circuits` 7.2.4 do not contain LogUp; the checkout is not a drop-in | Removes the sort and permute cost (small, per A2) and the permuted-table commitment. Helps verifier cost more than prover cost | High | Soundness analysis, new verifier, outer recursive gadget and ledger verifier must be updated. Breaks the immutable-verifier constraint |
| B2 | Smaller limb lookups (for example 15-bit limbs), reducing table rows | `loki-06` node: table rows come from `Pow2RangeChip` tags, limb sizes from `P2RDecompositionChip` and `compute_optimal_limb_sizes` | Changes the table and selectors, so new VKs. Node 06 says smaller limbs also raise advice rows by about 20% in its sensitivity model | Table rows fall from about 1.03M in the proxy. The k18 sizing hurdle needs about 54% row reduction (proxy). A limb change alone does not reach that | High | Range-check soundness, changed circuit. Requires migrating the recursive circuit |
| B3 | Fewer lookups (remove redundant lookups) | `loki-06` node | Changes the circuit and so the VK | Each removed lookup removes three points and five evaluations on the wire and one permuted pair of commitments | Medium | Only valid when the underlying check is truly redundant |

## Ranking (gain divided by effort)

1. A4, already landed. Keep it. No further effort.
2. A1, GPU grand products, only as part of GPU-resident quotient work. Medium-high effort; gain a few percent alone.
3. A3, GPU batch inversion. Low gain, medium effort.
4. A2, GPU sort. High effort for a sub-percent gain.
5. B2, smaller limb lookups. Potentially the largest structural gain for k, but it needs a new VK and a circuit migration.
6. B1, LogUp. Real protocol benefit, but a full migration.

## What would have to be true

- A1 pays off only if the grand products are on the GPU with the quotient, and
  the host-side product time is at least 5% of the Sherlock wall time (not yet
  measured).
- A2 pays off only if lookup count times sort time is at least a few percent of
  220 s. Current estimate: far below that.
- B2 pays off if a new recursive VK with fewer table rows is acceptable, and
  the parent is re-inventoried.
- B1 pays off if the recursive verifier and ledger are moved to a LogUp-capable
  protocol. Nothing in the pinned verifier supports it today.

## Other GPU precedents (not in Loki)

- Axiom `halo2-gpu` (BN254): kernels `prefix_scan.h`, `grand_product.cu`,
  `quotient_lookups.cu`, `batch_invert.rs`, `grand_product.rs`, `lookup.rs`, and
  a residency test `lookup_product_compressed_pair_residency_equivalence.rs`.
  This is the closest precedent for A1. Source: `raw/04/axiom-tree.md`.
- zkSync `zksync-crypto-gpu`: `device_radix_sort.cu` (CUB) and
  `columns_sorting.rs`. Precedent for A2 only in spirit.
- ICICLE halo2 v2 (Ingonyama): 25x on example circuits on RTX 4080, BN254.
  Vendor-reported, not Sherlock-comparable.
- Tachyon (halo2-compatible): its README lists halo2 lookups as CPU-only.
  Consistent with Loki's current state.

## Caveats

- Byte-equality is not a valid predicate for a GPU sort if the sort order of
  equal keys matters. For field values it does not matter, because equal values
  are identical.
- The Loki lookup code still has a `HashMap` for multiplicities. Its iteration
  order does not affect output after the sorted-leftover change.
- All numbers above are either k16 single-proof measurements, kernel benches,
  or extrapolations. None is a k20 Sherlock measurement.
