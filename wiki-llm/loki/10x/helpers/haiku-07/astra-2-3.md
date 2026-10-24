# astra-2-3 (REVISED after astra-2 correction): Folding plus final Midnight wrapper

Path note: captures are under helpers/haiku-07/raw, not astra-2/raw, per this agent's write scope.
Correction: the earlier version said a Nova wrapper "needs a pairing" and forecast wrapper k21-k22 from a BN254-hosted R1CS pairing count. Both claims are withdrawn. The 2,088,277 figure is for a BN254-hosted circuit, the wrong field direction for a BLS12-381-scalar Midnight wrapper, and cannot imply a wrapper size.
Wrapper rows: UNKNOWN. No measured or sourced Midnight wrapper exists.

## Question
Cheapest complete final wrapper for Nova/HyperNova/CycleFold/ProtoGalaxy accepted by ledger-9 (old PLONK/KZG Blake2b, new key allowed). Compare options honestly, including the CycleFold secondary relation. Crossover for 16 tx and large repeated chunks. Direct fused 16-tx proof as comparison.

## Answer
- **Pairings are not universal.** A Nova wrapper can avoid pairings in two ways:
  - (a) Spartan-style compression over Pedersen/IPA commitments: no pairing in the compressed verifier (nova-readme, lines ~249-257, `helpers/haiku-07/raw/nova-readme.md`).
  - (b) Raw final relaxed-R1CS satisfiability plus commitment openings, linear in the shape. node07 describes this (`nodes/loki-07-folding-alternatives.md`, "Conditional hybrid", step 3 and the Nova check `Az ∘ Bz = uCz + E`). No pairing appears.
  - Pairing enters only through pairing-based PCS variants (HyperKZG/Mercury, supported "on curves with pairings, e.g., BN254"; nova-readme lines ~252-253). Ledger-side pairing verification happens on the host, not in circuit (per astra-2; node07 "Ledger and decider", loki-05 accumulator).
- **Cycle compatibility.** Nova's implemented cycles are Pallas/Vesta, BN254/Grumpkin and secp/secq (nova-readme, line ~247). None is BLS12-381. node07 confirms the conventional BLS12-381 two-cycle partner cannot exist (Hasse bound, node07 "Curves").
- **Midnight stdlib.** The stdlib has a PLONK/KZG verifier that defers the pairing into an accumulator (`midnight-circuits-7.2.4/src/verifier/accumulator.rs:136,148`). It has foreign ECC for BLS12-381 G1 (`midnight-zk-stdlib-2.3.5/src/lib.rs:137-144, 788`). No in-circuit pairing chip was found (grep of both crates for pairing/miller).
- **Wrapper cost options (all rows UNKNOWN):**
  - Option A, raw final R1CS: cost ∝ nnz(A,B,C) non-native muls, plus a witness-sized commitment opening (non-native MSM of size n), plus hash bindings, plus the CycleFold secondary (node07 CycleFold ~1,000-1,500 muls; loki/raw/07/cyclefold-paper.md:54). Shape of the final relation is not measured.
  - Option B, compressed verifier (Spartan/IPA): in-circuit verifier cost, foreign-field arithmetic over the BN254/Grumpkin side not present in Midnight's field. Not sourced.
  - Option C, pairing-based compression (HyperKZG/Mercury): needs an in-circuit pairing over the non-native pairing curve. No sourced BLS-scalar-hosted figure exists for this. The 2,088,277 R1CS number (gnark, `helpers/haiku-07/raw/emulated-pairing.md:12-15`) is BN254-hosted R1CS and is not applicable.
- **Crossover formula (node07 model).** T_t = N a + D_t; T_h = N f + W + D_h for one wrap over all N. Folding wins iff N(a-f) > W. For N=16: folding wins iff a-f > W/16 per chunk. W is UNKNOWN, so no minimum N can be stated. Table of threshold per-chunk margin: W=100 s -> 6.25 s; W=500 s -> 31.25 s; W=1000 s -> 62.5 s. These are arithmetic on the formula, not forecasts. a and f are node07 ESTIMATEs and are unsourced.
- **Direct fused 16-tx proof** (no folding, no wrapper). Measured leaf rows, verified in `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:104-108` (`Leaf (measured)`): 1-tx leaf 27,980 cost rows, k=15; 4-tx leaf 112,098 cost rows, k=17. Linear extrapolation: 16 x 27,980 = 447,680 rows (k19, 2^19 = 524,288); 4 x 112,098 = 448,392. astra-2 cites ~448,570 rows, k19 for the 16-tx fused native body (+ wrapper); I did not find that exact figure in the files read and mark it as astra-2's figure, not verified here. The earlier k24 claim is WITHDRAWN: it applied the k20 recursive parent, not a leaf, and is discarded.
- **Fused fusion privacy caveat.** The direct fused proof is valid only where one prover holds every spender's witness: `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/leaf.rs:7-9` (synthetic strawman only). It does not replace distributed private-witness semantics.

## Raw numbers table
| Quantity | Value | Source | Tag |
|---|---|---|---|
| Nova supported cycles | Pallas/Vesta, BN254/Grumpkin, secp/secq | helpers/haiku-07/raw/nova-readme.md ~l.247 | SOURCE |
| Nova compression | Spartan (w/ and w/o preprocessing); HyperKZG/Mercury on pairing curves | nova-readme ~l.249-257 | SOURCE |
| Nova verifier scalar muls (2 Nova, 1 HyperNova) | 2 / 1 | loki/raw/07/cyclefold-paper.md:54 | SOURCE |
| CycleFold secondary | ~1,000-1,500 muls | node07 citing cyclefold-paper | SOURCE (node07) |
| BN254-hosted R1CS, BLS12-381 pairing | 2,088,277 | helpers/haiku-07/raw/emulated-pairing.md:15 | SOURCE, BN254-hosted R1CS implementation result; not a lower bound or ceiling, not applicable to Midnight wrapper |
| k20 usable rows | 1,048,569 | loki-05 line 91 | CODE/derived |
| Midnight wrapper rows | UNKNOWN | none | UNKNOWN |
| 1-tx leaf cost rows / k | 27,980 / k15 | recl9 REPORT.md:104-108 | CODE (measured) |
| 4-tx leaf cost rows / k | 112,098 / k17 | recl9 REPORT.md:104-108 | CODE (measured) |
| 16-tx fused body rows (linear, 16 x 1-tx) | 447,680 (k19) | derived from REPORT.md; astra-2 cites ~448,570 | derived / astra-2 cited |
| Fused PK payload at k19 (k20 model / 2) | ~4.4 GiB | loki-01:103 scaled by 2^-1 | ESTIMATE |

## Caveats
- No benchmark measures a Midnight wrapper. Wrapper rows, time, memory are UNKNOWN.
- Sonobe decider and Nova compressed-SNARK benchmark numbers were not captured. Sonobe support matrix (helpers/haiku-07/raw/sonobe-readme.md:270-271): Nova/CycleFold stable; LegoGroth16 and HyperNova to be merged.
- a, f, and per-leaf k are ESTIMATEs or unsourced.
- Ledger-9 acceptance is unchanged. A wrapper needs a new key/VK and contract authorization (node07 "Conditional hybrid").

## Addendum: measured KZG decider rows (verified by haiku-07)
- `~/repos/projectfenrir/recursion/pairing-gadget/lanes/opus/RESULTS.md:17-19`: KZG decider with fixed G2 and residue check, A and B subgroup-checked: 316,833 rows, k=19, 6,432 B proof (measured). Without G1 subgroup checks: 299,573 rows, k=19. Model row (rc=3): 450,394 rows, k=19, 6,016 B (model, not measured).
- These are measured rows for a pairing-based decider of the existing protocol, not a wrapper bound. They show a retained pairing decider needs k19 (262,137 usable at k18 cannot hold 316,833 rows; k18 usable = 2^18-7 = 262,137 — verified arithmetic). This is a measured floor for that gadget, not a universal cryptographic lower bound.
