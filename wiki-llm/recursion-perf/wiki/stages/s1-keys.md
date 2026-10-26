# Stage s1: key setup, loading and residency

> Sources: astra-s1-keys wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki node-keygen profile and design, 2026-10-24
> Raw: [findings](../../clusters/s1-keys/wave-1/FINDINGS.md); [experiments](../../clusters/s1-keys/wave-1/EXPERIMENTS.md); [plan](../../clusters/s1-keys/wave-1/PLAN.md); [sources](../../clusters/s1-keys/wave-1/SOURCES.json); [prior profile](../../raw/s1-keys/prior-profile.md); [prior design](../../raw/s1-keys/prior-design.md)
> Updated: 2026-10-26

## Overview

**Key work alone cannot deliver 10× end to end on the recorded workloads.** ESTIMATE: removing Sherlock's repeated 62.3 s proving-key setup gives `(62.3+882.6)/882.6 = 1.0706×`; a cold stream of 16 proofs gives 1.0659×. The Fenrir setup-only envelope is 1.0976×.

Keep prepared keys resident anyway: their share grows once the other stages speed up. The one large, narrow target is the **622.8 s cold verifying-key build** in the recorded recl9 node keygen. That run used midnight-proofs 0.8.2, whose MSM leaves BLST above 2^19 points. The 18 length-2^20 permutation (sigma) commitments cross that threshold.

## Ranked ideas

1. **Prepared-key registry, one initialization per identity.** Gain is 1.07–1.10× today and is a prerequisite for any warm service.
2. **Backport only the BLST dispatch correction to 0.8.2.** With `f` the VK share in affected MSMs and `s` their speedup, `T = 622.8[(1−f)+f/s]`. That gives 1.33×, 2.50× or 4.71× of the VK build for `(f,s) = (.5,2), (.8,4), (.9,8)`. This shortens cold keygen latency, not proving.
3. **Build VK and PK from one immutable keygen assembly.** The cap is 1.168× of VK+PK. The real saving is only the shared synthesis and assembly subset.
4. **Resident, identity-bound SRS bases.** The recorded SRS load is 0.8 s, so the gain is negligible for keygen. It matters only for repeated normalization and upload inside commitments.
5. **Prepared snapshot for restart and cache misses.** This trades about 7.4 GiB of extra read, uncompressed, against rebuilding.

## Notes

- Even deleting all 622.8 s leaves 336.7 s of the recorded 959.5 s keygen process: at most 2.85× for that process.
- Do not compare a 0.8.3 Sherlock run with the 0.8.2 recl9 baseline. Pin one version, its lockfile and its thread count.
- First experiments: E1 attributes the cold VK dispatch cliff; E2 measures prepared-key reuse and concurrent misses; E3 separates SRS I/O, prepared bases and device residency.
- The verbatim recl9 captures this wave used (report, logs, keygen stats, sources) are private Fenrir files. They stay on the research host under `raw/s1-keys/recl9-*.md` and are not published.

## See Also

- [s8 decider and settlement](s8-decider-settle.md), whose rank-1 persistent prover is the same remedy
- [s3 commitments and MSM](s3-commit-msm.md)
- [Consensus tracker](../../CONSENSUS.md)
