# Stage s2: witness synthesis

> Sources: astra-s2-witness wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki-09 and Astra-1 prior studies, 2026-10-24
> Raw: [findings](../../clusters/s2-witness/wave-1/FINDINGS.md); [experiments](../../clusters/s2-witness/wave-1/EXPERIMENTS.md); [bounds](../../clusters/s2-witness/wave-1/bounds.json); [Loki-09 prior](../../raw/s2-witness/prior-loki09.md); [Astra-1 prior](../../raw/s2-witness/prior-astra1.md)
> Updated: 2026-10-26

## Overview

Prioritize a **phase-aware, key-bound CPU witness schedule**, then **batch field inversions at dependency frontiers**. These are plausible parts of a combined 10× prover. Witness synthesis alone has no supported 10× end-to-end case. Before any rewrite, measure the real synthesis time of the parent, node and decider, plus dependency width and conversion costs. Do not start with an AVX-512 port, a generic GPU witness compiler, or a replacement reference machine.

## Ranked ideas (ESTIMATE; local gains, then chain effect C)

1. **Typed witness schedule with deterministic parallel regions and native hints:** 2–8× local, C = 1.49–2.38×. Scheduling alone reaches 10× only if at least 94.7% of the work parallelizes.
2. **Batch inversions at dependency frontiers:** up to about 24× on inverse arithmetic at width 256, and about 1.9× witness / C ≈ 1.46× if inversions are half the witness time.
3. **Frozen layout replay** (remove shape traversal): C ≈ 1.2×, bounded by the measured removable fraction.
4. **Active-phase storage and direct output buffers:** C ≈ 1.07×. One k20 scalar column is 32 MiB.
5. **Fused GPU witness kernels after the CPU recipe works:** a planning window of 1–8.5× witness, C ≈ 2.4× at best.
6. **G5 typed sink with adjacent-state reuse:** near 2× for its subtask, about 1.05× of the chain. It gives 1× for parent-only runs.
7. **Batched field ABI:** C ≈ 1.11×. Speculative SIMD gets 1× credit until measured.
8. **Borrow powdr's compiler architecture, but don't port its JIT:** 1× credited.

## See Also

- [s7 recursion and aggregation](s7-recursion.md)
- [Consensus tracker](../../CONSENSUS.md)
