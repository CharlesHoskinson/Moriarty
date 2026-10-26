# Recursion performance knowledge base

## stages

One article per recursion stage of the state machine in [stages.json](../stages.json). Each article is compiled from research-cluster waves and cited to raw/.

| Article | Summary | Updated |
|---------|---------|---------|
| [s2 witness synthesis](stages/s2-witness.md) | Phase-aware CPU witness schedule plus frontier batch inversions; no 10× from this stage alone | 2026-10-26 |
| [s3 commitments and MSM](stages/s3-commit-msm.md) | Commitment-aware GPU bucket MSM; Amdahl-capped near 2× over Nocy GPU on its own | 2026-10-26 |
| [s4 lookups and grand products](stages/s4-lookups.md) | Structured table-ID permutation plus parallel scans; 1.1-1.3x of the proof; tables alone cannot drop k20 | 2026-10-26 |
| [s5 quotient and NTT](stages/s5-quotient-ntt.md) | Resident bounded-memory quotient with coset tiles; 2.1-2.4x overall at a 60% share | 2026-10-26 |
| [s9 scheduling, memory and hardware](stages/s9-orchestration.md) | Phase-aware scheduler with bounded window; throughput ceiling 12.8x at W=4, latency unchanged | 2026-10-26 |
| [s7 recursion and aggregation](stages/s7-recursion.md) | The fused fixed-pair terminal is the best 10× route, under explicit conditions | 2026-10-26 |

See also the [consensus tracker](../CONSENSUS.md).
