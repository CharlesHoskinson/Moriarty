# Stage s6: polynomial opening (multiopen)

> Sources: astra-s6-opening wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki-04 opening study, 2026-10-24
> Raw: [findings](../../clusters/s6-opening/wave-1/FINDINGS.md); [experiments](../../clusters/s6-opening/wave-1/EXPERIMENTS.md); [plan](../../clusters/s6-opening/wave-1/PLAN.md); [algebra check](../../clusters/s6-opening/wave-1/check_opening_algebra.py); [baseline](../../raw/s6-opening/task-baseline.md); [midnight KZG 0.8.3](../../raw/s6-opening/midnight-kzg-0.8.3.md); [midnight KZG 0.8.2](../../raw/s6-opening/midnight-kzg-0.8.2.md); [prior Loki-04](../../raw/s6-opening/prior-loki04.md)
> Updated: 2026-10-26

## Overview

Prioritize the **exact Midnight opening implementation**. First remove redundant work. Then port its reductions, evaluations, division and two MSMs to a resident GPU path. Keep the transcript schedule and the terminal PCS unchanged, because ledger 9 accepts PLONK/KZG over BLS12-381 with Blake2b and the official SRS.

Study fflonk only as an inner-layer circuit-size optimization, until it is shown to shrink parent or decider cost. There is **no measured basis for a 10× claim from this stage**: no s6 elapsed time was supplied.

## Ranked ideas

1. **Resident exact-Midnight multiopen.** ESTIMATE: an s6 speedup of 4–10× gives 1.03–1.37× overall, depending on the assumed opening share.
2. **Elide the final evaluation, remove copies, fuse CPU folds.** If 2–10% of s6 is eliminated, the gain is 1.001–1.031×.
3. **Sparse vanishing division plus a barycentric verifier backport.** For t=4 roots, `X⁴−x` has one non-leading term, so division work falls about 4n→n. If division is 10–40% of s6, the gain is 1.004–1.099× overall.
4. **fflonk t=2 or t=4 inside recursion only.** The published k14 prover penalties are 1.154× (t=2) and 1.292× (t=4); these are not k20 predictions. An inner switch pays only if `U·d > L·(p−1) + K/R` (outer saving versus inner penalty plus amortized key cost). With U=4L it needs an outer saving above 7.29%.
5. **Batch independent child openings over a shared SRS cache.** If upload equals compute, B=2 gives 1.33× and B=4 gives 1.60× for this component. The gain vanishes when bases are already resident.

## Notes

- A new decider circuit for inner fflonk still emits accepted KZG proofs, but it needs a new VK and a contract re-registration. Protocol acceptance and acceptance by the already-deployed contract VK are separate conditions; this is the A4 failure class.
- The running binaries' exact proofs versions, features and patches were not independently established. Experiment E1 measures s6 coverage and the cheapest byte-identical changes first.

## See Also

- [s3 commitments and MSM](s3-commit-msm.md)
- [s5 quotient and NTT](s5-quotient-ntt.md)
- [s7 recursion and aggregation](s7-recursion.md)
- [Consensus tracker](../../CONSENSUS.md)
