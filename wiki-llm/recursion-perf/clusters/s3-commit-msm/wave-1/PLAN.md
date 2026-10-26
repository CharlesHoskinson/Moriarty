# Wave 1 source plan

All leads below are OPEN QUESTION until checked against primary source text. Research only; no build, install, proof run, or ledger call is authorized by this work plan.

1. Nocy current MSM and caller: determine actual per-point algorithm and timed transfer/synchronization costs, extending the historical audit.
2. sppark current Pippenger source: inspect scalar representation, bucket sizing, synchronization and integration boundary.
3. ICICLE batch/precompute API and memory: helper 1; distinguish shared bases, scratch replication and host-to-device costs.
4. cuZK and Yrrid benchmarks: helper 2; reject curve, GPU, size and timing mismatches.
5. WLC MSM (load-balanced cached-Jacobian bucket buffers): inspect BLS12-381 code, constant-time option and distribution-sensitive timings.
6. Open-ICICLE / gnark-cuda: inspect an actual BLS12-381 PLONK integration for deployment and conversion costs.
7. Blackwell CUDA compatibility and graph replay: determine native architecture requirements and whether launch savings can matter at k20.
8. Fixed-base reuse economics and commitment linearity: test memory/break-even arithmetic and whether any reuse preserves transcript commitments exactly.

SOURCE: earlier studies read at ../../wiki-llm/loki/nodes/loki-02-gpu-msm.md (the supplied 10x path was absent), ../../wiki-llm/loki/10x/astra-1/raw/haiku-03-h3-sppark.md and ../../wiki-llm/loki/10x/astra-1/raw/nocy-msm.md. No earlier-wave measurement is a new measurement here.
