# Stage s3: commitments and MSM

> Sources: astra-s3-commit-msm wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki GPU-MSM prior study, 2026-10-24
> Raw: [findings](../../clusters/s3-commit-msm/wave-1/FINDINGS.md); [experiments](../../clusters/s3-commit-msm/wave-1/EXPERIMENTS.md); [sources](../../clusters/s3-commit-msm/wave-1/SOURCES.json); [Nocy MSM prior](../../raw/s3-commit-msm/earlier-nocy-msm.md); [sppark prior](../../raw/s3-commit-msm/earlier-sppark.md)
> Updated: 2026-10-26

## Overview

Prioritize a **commitment-aware bucket MSM service**. Benchmark it against both the exact current BLST dispatch and Nocy's independent per-point multiplication. Keep sppark as the first reference, and treat open-ICICLE and WLC as challengers.

There is **no evidence for 10× end-to-end from this stage alone**. ESTIMATE: if MSM is 60% of the remaining GPU interval, a 5–20× MSM gain yields `(12.3/4.4)/(0.4+0.6/s)`. That is 1.92–2.33× over Nocy's GPU time, or 5.38–6.50× over its recorded CPU time. Reaching 10× needs other stages accelerated as well.

## Notes

- The MSM dispatch differs between midnight-proofs versions: 0.8.2 switches away from BLST above 2^19 points, while 0.8.3 always uses BLST. Any comparison must pin one version.
- Batch many commitments per launch and avoid synchronizing after each commitment, which is Nocy's measured weakness. Weigh precomputed bases and SRS residency against the 5090's 32 GB.

## See Also

- [s2 witness synthesis](s2-witness.md)
- [s7 recursion and aggregation](s7-recursion.md)
- [Consensus tracker](../../CONSENSUS.md)
