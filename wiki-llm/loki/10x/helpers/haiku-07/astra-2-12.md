# astra-2-12: adversarial speed-budget audit

Scope: arithmetic and interpretation check only. No builds, no repo writes. All numbers are from the bus messages or from the cited files, recomputed here.

## Inputs (as given, with checks)
- Old actual parent: prove 882.589555363 s; PK setup 62.328385318 s (astra-2). Cited in `loki/nodes/loki-05-recursion-accumulation.md:129` (receipt), and `docs/sherlock/reference/recursion-evidence-2026-10-22.md:7` (k20, 6,432 B). 882.590/10 = 88.259 s, matches astra-2 threshold.
- Old synthetic 16-tx chain (Fenrir recl9 REPORT, historical): leaves prove sum 163.630576714 s (per-leaf 34.78, 35.30, 44.19, 49.37); node 867.279750364 s; decider 586.381689547 s. Sum 1617.292 s. astra-1 notes these are historical (.8.2 vs .8.3 kzg msm differences), so they are illustrative, not a controlled .8.3 baseline.
- Fractions in the general case: f_w = 0.20, f_b = 0.78, f_c = 0.02 (sum 1.00 of original T). Overhead o = 0.01 is added relative to original T, so T_new/T_old at zero acceleration is 1.01. This is intentional (astra-2 clarification), not an inconsistency. The gain formula uses control+overhead = 0.03.

## General formula (astra-2)
Gain = 1 / (0.20/W + 0.78/(D·G) + 0.03), with W = witness speedup, D = bulk fraction divisor (speedup of bulk factored into D·G), G = GPU speedup.

| Case (W, D, G) | Gain |
|---|---|
| W1 D1 G8 | 3.053 |
| W2 D2 G8 | 5.594 |
| W4 D4 G8 | 9.581 |
| W5 D4 G8 | 10.596 |
| W5 D2 G8 | 8.421 |

Interpretation: 10x in the general case needs roughly W ≥ 5 with D·G ≥ 32, or an equivalent combination. These are conditional on the fractions being right; the 1.01 sum should be resolved before any claim.

## Parent-only and leaves-only ceilings (historical chain, Amdahl)
- Total T = 1617.292 s.
- Node (parent) only, infinitely fast: T/(T−N) = 1617.292/750.012 = **2.156x**. At a 10x parent: 1.933x. This matches astra-1's numbers.
- Decider only, infinitely fast: T/(T−D) = **1.569x**.
- Node + decider together, infinitely fast: T/L = **9.884x** (leaves 163.63 s are the only remaining work).
- Free leaves + node, decider remaining: T/D = **2.758x** (astra-2's figure; confirmed).
- Leaves only, infinitely fast: T/(N+D) = **1.113x**.

So no parent-only accelerator reaches 10x on this chain. Node+decider together can reach 9.88x only if the leaves are unchanged and the two heavy stages are infinitely fast; that is a ceiling, not a target.

## Universal 10x impossibility?
No. Amdahl gives the constraint only: a 10x overall speedup needs the non-accelerated share s ≤ 0.1 for an infinitely fast accelerator (1/s ≥ 10). For finite accelerators, the condition is a weighted sum. Whether 10x is reachable depends on the workload's unaccelerated share, the accelerator speedups and the overhead. Nothing here proves or disproves 10x for a specific workload. In particular, the astra-2 fused fixed-pair route removes the old parent proof rather than accelerating it, so it is not an Amdahl argument on the old chain at all.

## Latency vs throughput
- Latency per proof and throughput (batches/h) are different. The 88.259 s threshold is a per-proof warm latency for the fused fixed pair. A throughput claim needs both a paired warm optimized baseline and a pipeline model; see astra-3's queue model (10.45x vs serial, a non-measured scenario, `10x/astra-3/queue_model.py`).
- The old parent 882.590 s is a prove interval with 62.328 s setup separated. Cache state is unknown (astra-2): do not call 882.590 s warm. A fair 10x claim needs the same setup/cache conditions on both sides (setup-included or setup-excluded in both).

## Paired-baseline requirement
- Optimized warm baseline on the same hardware, same key/caching state, same statement scope, same release flags. Historical chain (debug or older .8.2 numbers) does not qualify.
- "Old full chain" for the fused target: the 882.590 s parent alone is only a lower bound on the old full chain. The 88.259 s threshold is sufficient against the old full chain only if the old chain's leaves and decider are included in the measured baseline.

## Circuit budget
- 12,274 = (2^13−7)+(2^12−7) bounds the children's usable rows, not the fused rows. Glue allowance 8,192 is an unsourced estimate. 20,466 total vs k15 usable 32,761 is a design budget, not a measured bound. Unchanged.

## Findings
- The arithmetic in astra-2's threshold (882.590/10 = 88.259) and budget (12,274 + 8,192 = 20,466 < 32,761) is correct.
- The 1.01 total is intentional: overhead is relative to original T (astra-2 clarification). Withdrawn as an inconsistency.
- The historical chain numbers are illustrative, not a controlled baseline.
- No ceiling argument proves 10x impossible. The 10x claim is conditional on fractions, equivalence and a paired baseline not yet measured.

## Raw numbers table
| Quantity | Value | Source |
|---|---|---|
| Old parent prove | 882.589555363 s | astra-2 message; loki-05:129 |
| Old parent PK setup | 62.328385318 s | astra-2 message; loki-05:129 |
| 10x threshold | 88.259 s | 882.590/10 |
| Leaves sum | 163.630576714 s | astra-1 / astra-3, recl9 leaves.stats.json (cited by astra-1) |
| Node | 867.279750364 s | same |
| Decider | 586.381689547 s | same |
| Chain total | 1617.292 s | sum |
| Parent-only ceiling (node) | 2.156x | T/(T−N) |
| Decider-only ceiling | 1.569x | T/(T−D) |
| Node+decider ceiling | 9.884x | T/L |
| Leaves-only ceiling | 1.113x | T/(N+D) |
| Free leaves + node ceiling | 2.758x | T/D |
| General gain cases | 3.053 / 5.594 / 9.581 / 10.596 / 8.421 | formula above |
| Circuit budget | 20,466 vs 32,761 | design budget, unmeasured |

## Caveats
- The leaves/node/decider times are historical (per astra-1, possibly .8.2-era kzg msm). Not a controlled .8.3 baseline.
- No measurements in this audit.
