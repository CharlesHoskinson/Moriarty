# Stage s5: quotient and NTT

> Sources: astra-s5-quotient-ntt wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki-03 prior study
> Raw: [findings](../../clusters/s5-quotient-ntt/wave-1/FINDINGS.md); [experiments](../../clusters/s5-quotient-ntt/wave-1/EXPERIMENTS.md); [cuSnark capture](../../raw/s5-quotient-ntt/cusnark.md); [cuSnark quotient](../../raw/s5-quotient-ntt/cusnark-quotient.md); [Loki-03 prior](../../raw/s5-quotient-ntt/earlier-loki-03.md)
> Updated: 2026-10-26

## Overview

Prioritize a **resident, bounded-memory quotient evaluator**, with **base-domain coset tiles** when the extended columns don't fit. Don't budget another 10× over an already accelerated quotient from a faster NTT alone. For an unaccelerated quotient, 8–30× is an experimental target. At a 60% stage share that gives only **2.11–2.38× overall**. A 10× end-to-end result needs other stages to contribute, or a measured s5 share above 90%.

## Key facts

- SOURCE: cuSnark brought Eval_h from 1,115.43 s to 36.19 s for 2^20 rows × 1,135 columns on four RTX 3090s. The full proof went from 1,762.75 s to 463.13 s, which is 3.81×.
- ESTIMATE: replacing only that quotient caps the gain at 2.72×. Improving the GPU quotient another 10× gives 1.08×.
- Crossing a real constraint-degree threshold (6 to 5) gives 1.26–1.72× on the stage. Going from 5 to 4 gives 1×.
- cuSnark uses an opcode interpreter split across kernel launches, because giant fused kernels compile for hours and can exceed device memory.

See also: [consensus tracker](../../CONSENSUS.md).
