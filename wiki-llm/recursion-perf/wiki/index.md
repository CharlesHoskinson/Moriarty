# Recursion performance knowledge base

## stages

One article per recursion stage of the state machine in [stages.json](../stages.json). Each article is compiled from research-cluster waves and cited to raw/.

| Article | Summary | Updated |
|---------|---------|---------|
| [s2 witness synthesis](stages/s2-witness.md) | Phase-aware CPU witness schedule plus frontier batch inversions; no 10× from this stage alone | 2026-10-26 |
| [s3 commitments and MSM](stages/s3-commit-msm.md) | Commitment-aware GPU bucket MSM; Amdahl-capped near 2× over Nocy GPU on its own | 2026-10-26 |
| [s7 recursion and aggregation](stages/s7-recursion.md) | The fused fixed-pair terminal is the best 10× route, under explicit conditions | 2026-10-26 |

See also the [consensus tracker](../CONSENSUS.md).
