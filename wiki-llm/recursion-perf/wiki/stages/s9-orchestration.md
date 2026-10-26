# Stage s9: scheduling, memory and hardware

> Sources: astra-s9-orchestration wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki-08 and Astra-3 prior studies; a Muse Wonder brainstorm was requested in this wave
> Raw: [findings](../../clusters/s9-orchestration/wave-1/FINDINGS.md); [experiments](../../clusters/s9-orchestration/wave-1/EXPERIMENTS.md); [bound audit](../../clusters/s9-orchestration/wave-1/bound_audit.py); [recorded runs](../../raw/s9-orchestration/recorded-runs.md)
> Updated: 2026-10-26

## Overview

Scheduling evidence does not establish 10× end-to-end. The best next investment is a **measured, phase-aware scheduler** with a bounded number of unfinished batches and memory reserved for forward progress. It recovers idle CPU and GPU capacity without touching ledger cryptography. It cannot remove the parent→decider dependency.

## Key estimates

- Baselines: B5 runs prefixes 2+4+6, which is 12 batch instances, so 5,558.878 s / 12 = 463.24 s is suite amortization, not per-batch latency. The historical Fenrir chain is 1,617.29 s = 2.23 batches/hour, which puts the 10× interval at 161.73 s.
- **Phase-aware admission:** the throughput ceilings are W=1 → 3.21×, W=3 → 9.64×, W=4 → 12.85×, measured against 1,617.29 s. These are ceilings under favorable inherited assumptions, not forecasts.
- **Overlapping CPU and GPU** is at most 2× for one lane each. Removing idle time gives 1/u.
- **Key affinity, pressure-aware ordering, CUDA pools and graph replay** each give about 1.01–1.25×.
- **Profiling (L0) earns 1.00× on its own**, but it decides whether every other bound is reachable.

See also: [consensus tracker](../../CONSENSUS.md).
