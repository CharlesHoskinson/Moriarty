# Loki speed-up implementation plan: independent A3 ranking

**Goal:** reduce the real Sherlock parent's prove-and-verify wall time while preserving the deployed verifier on Track A; keep circuit replacements on a separate Track B.

**Architecture:** finish W9 hardening and W10's measured GPU quotient first. Establish attributed sequential A3 baselines, then remove repeated PK work and extend device residency only where the remaining phase budget supports a measurable gain. W11 retains sole ownership of KZG multiopen.

**Stack:** Midnight PLONK/KZG 0.8.3, BLS12-381 scalar arithmetic, Rust/Rayon/BLST, sppark/CUDA on the 32 GB RTX 5090, W2 differential harness, LokiLean.

This is a planning artifact, not an implementation or a new benchmark result. Code was inspected read-only; no proof jobs were launched. Package numbers below are proposed allocations, not claims on files. The user's one-file instruction overrides generic skill plan locations and the factory's code-edit/commit loop for this assignment.

## Evidence, arithmetic, and rejection rule

The supplied reference is **220.2 s stock, 252.6 s Loki CPU, 220.8 s Loki GPU**. A 13–16x isolated MSM improvement has produced no demonstrated A3 improvement; W3 kernel gains likewise failed to survive full proofs. Consequently, a fast kernel is evidence about that kernel only.

The authoritative phase-selection evidence is [W10's A3 profile](/home/charles/repos/loki-wt/w10-gpu-quotient/LOKI-A3-PROFILE.md:141). These are **exclusive CPU phase times**, collected during partially overlapping jobs, not phases of the 220.2 s reference run:

| Exclusive phase | CPU seconds | Share if a comparable A3 total were 220–250 s | Absolute elimination ceiling |
|---|---:|---:|---:|
| quotient_evaluation | 72.882 | 29.2–33.1% | 72.882 s |
| lookup_prepare | 25.038 | 10.0–11.4% | 25.038 s |
| compute_nu_poly | 14.103 | 5.6–6.4% | 14.103 s |
| permutation_products | 13.334 | 5.3–6.1% | 13.334 s |
| kzg_multiopen | 11.480 | 4.6–5.2% | 11.480 s |
| lookup_products | 11.311 | 4.5–5.1% | 11.311 s |
| quotient_commitment | 7.614 | 3.0–3.5% | 7.614 s |
| witness_synthesis.phase_0 | 7.262 | 2.9–3.3% | 7.262 s |
| other | 4.527 | 1.8–2.1% | 4.527 s |
| advice_commitments | 2.841 | 1.1–1.3% | 2.841 s |
| trash | 2.835 | 1.1–1.3% | 2.835 s |
| evaluations | 1.270 | 0.5–0.6% | 1.270 s |
| quotient_construct (excluding its commitment) | 0.894 | 0.4% | 0.894 s |
| multiopen wrapper (excluding kzg_multiopen) | 0.000 | 0% | 0 s |

The phase sum is **175.389 s**, but that run's actual `prove_verify_seconds` is **292.712 s**, leaving **117.323 s outside create_proof**. That residual includes cost-model work, PK setup, child checks, verification and cleanup; it is neither all keygen nor all witness synthesis. On that actual run quotient's elimination ceiling is 24.9%, not 33.1%. The GPU-feature exploratory run is unattributed and overlapping: **do not subtract its phase times from CPU times to claim acceleration**. Small k14 GPU dispatch and registry-verifier success later in the report establish functionality, not k20 acceleration.

All estimates below explicitly use the requested **220–250 s normalization**, conditional on a new matched baseline confirming comparable phase costs. For a phase of cost `p`, fraction `a` actually affected, and local acceleration `s`:

`saved_seconds = p * a * (1 - 1/s) - added_overhead`

`wall_reduction = saved_seconds / T`; `speedup = T / (T - saved_seconds)`.

Percentages denote wall-time reduction, not speedup. A 3% result requires **6.6–7.5 s saved** in this range; the measured 292.712 s run would require 8.781 s. For example, accelerating the entire 72.882 s quotient phase by 2–5x saves 36.441–58.306 s, or 14.6–26.5% at the normalized total, and only 12.4–19.9% at 292.712 s. Accelerating a 5 s kernel inside that phase cannot save 36 s.

**Admission:** DROP expected gains below 3% as standalone A3 performance work, regardless of convenience. Unknown gain is LATER or AFTER a named measurement, never implicitly >3%. Measurement/correctness prerequisites are explicitly exempt: they establish whether any speed claim is real. A component needed to make an admitted package correct can be included without inventing a separate speed claim. Re-profile after every promotion; overlapping row estimates are alternatives, never additive budgets.

## Source and contract keys used in every row

Each source cell is `report-key:line`, with the key resolving to the following local report. Lines are the scout snapshot inspected for this plan; duplicate discoveries retain separate rows and converge on one owner/package.

| Key | Source |
|---|---|
| Q | [quotient-evaluation.md](quotient-evaluation.md) |
| K | [kzg-multiopen.md](kzg-multiopen.md) |
| L | [lookups-permutation.md](lookups-permutation.md) |
| G | [gpu-system.md](gpu-system.md) |
| W | [witness-synthesis.md](witness-synthesis.md) |
| S | [recursion-structure.md](recursion-structure.md) |
| P | [other-chains-plonk-kzg.md](other-chains-plonk-kzg.md) |
| R | [other-chains-recursion.md](other-chains-recursion.md) |

Existing Lean identifiers below refer to files in `/home/charles/repos/loki-lean/LokiLean/`, not proposed Rust APIs. `Msm.*` is file-qualified shorthand: that file's declarations actually live directly under namespace `Loki`, so, for example, the full name is `Loki.chunkedMsm_eq`. Other namespace prefixes omit the common `Loki` root. `NEW` names are **required new obligations**, not theorems already proved.

| Contract key | Actual reliance / missing obligation |
|---|---|
| EQ | `Refinement.Refines`, `SameOrder`, `loki_proof_bytes_eq_stock`, `verifier_decision_unchanged`; lookup acceptance uses `loki_accepted_any_order` under its completeness assumptions. |
| QN | `Plonk.Domain.numeratorAt`, `numeratorAt_evaluates`, `EvaluatesNumerator`, `quotient_independent_of_strategy`, `quotient_contract`; `Plonk.hornerY_append` / `hornerY_swap`. Rust lowering to these identities still needs empirical validation. |
| NT | `Ntt.Domain.rotate_extended`, `coeffToExtended_eq_eval`, `extendedToCoeff_of_eval`; `Ntt.ifftPath_gpu_cpu_agree`, `fftPath_isNatural`, `ifftPath_isInverse`. |
| MS | `Msm.MeetsMsmSpecific`, `msmDispatch_meets`, `chunkedMsm_eq`, `msm_filter_nonzero`, `commitLagrange_eq_commit`; actual CUDA success must satisfy the kernel contract. |
| BI | `Refinement.montBatchInvert_eq`, `batchInvertParallel_eq`, `chunkedScan_eq`, `product_column_eq`, `rational_compact_eq`. Zero entries remain zero; reject policies must match their callers. |
| LP | `Refinement.lookupPermuteWith_valid`, `lookupPermuteWith_ord_unique`, `loki_lookup_valid`, `loki_lookup_order_free`; output must preserve Loki's deterministic placement. |
| TR | `Transcript.batch_emit_eq`, `computeIn_perm`, `lokiAdvicePhases_eq`, `loki_batched_same_output`, plus EQ. Compute scheduling may change; commitment/challenge emission and RNG consumption must retain serial semantics. |
| PK | `Refinement.Cache.fill_sound`, `Cache.lookup_eq_keygen`, `cached_prove_eq`: full `(params, circuit, k)` identity, equal fresh key. A bare pointer/hash is not this contract. |
| OPEN | NEW `KzgMultiopen.division_eq_stock`, `linearCombination_eq_stock`, `proverQueries_ignore_evals`, and concrete transcript equality for x1..x4, point-set order and padding. Existing spec abstracts PCS opening as `openPoly`; EQ alone does not prove these rewrites. |
| DEV | NEW `DeviceBuffer.readback_eq`, `ResidentCosets.lookup_eq`, `GpuFallback.input_preserved`: correct field representation, length, domain, device/context, allocation lifetime and unchanged inputs on failure; reuse QN/NT/MS. |
| WIT | NEW `Witness.orderedAssignment_eq` for layout, cells, copies, instances, RNG and challenges. Existing backend refinement starts after witness construction and does not justify skipping region closures. |
| CIR | NEW `Sherlock.RelationEquivalent`: the new circuit accepts exactly the intended transition/statement, with no weakened bounds or subgroup checks. Requires circuit semantics beyond today's prover backend spec. |
| TERM | NEW `Sherlock.FusedTerminalEquivalent`: preserve all 16 statement fields, linkage and hash domains, and terminal decision semantics; inherit and state A102/A80 open obligations. |
| LED | NEW `Sherlock.DeciderAccumulatorChecked`: both the outer proof and every carried accumulator pairing are enforced by the ledger-facing decision. An exposed point alone is insufficient. |

No `sorry` in the current Lean tree does not close its explicitly open modelling assumptions (`stock-trace-faithful`, `quot-inputs`, `stock-complete-any-order`, `keygen-deterministic`, [SPEC.md](/home/charles/repos/loki-lean/SPEC.md)). Its stated limits include one permutation argument, no trash commitments, and abstract openings. Packages covering actual multiple permutation sets, trash, witness layout, KZG internals or a new relation must connect these missing pieces rather than cite a general refinement theorem as a complete proof.

Ownership cells name all relevant current owners; `none` means no direct W9/W10/W11 file collision. Effort is planning effort in engineer-days (`d`) or weeks (`w`), including targeted differentials but excluding queue delay. **A** always means same SRS/VK and unmodified stock-verifier acceptance. **B** includes new keys/redeploy and, where stated, protocol/PCS changes requiring more than a VK update.

## Inventory: quotient evaluation

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| Q1 Fuse gate/permutation/lookup/trash passes | Q:37 | A | quotient_evaluation 72.882 | Conditional 4–9%: if 60% is row interpretation and that portion improves 1.3–1.8x, save 10.1–19.4 s. FFT time is excluded from affected fraction. No measured gain yet. | 2–4 d | W9 evaluation.rs rotation hardening; W10 evaluator/quotient | QN, NT, EQ; NEW fused-row lowering equality | AFTER W9+W10 and W12; W14 only if incremental saving >=3%, otherwise DROP. Do not run a rival quotient lane. |
| Q2 Shared graph CSE / direct column reads instead of Stores | Q:38 | A | quotient_evaluation 72.882 | At 60% non-FFT share and 10–30% removal there, save 4.4–13.1 s = 1.7–6.0%; low case fails. Count dispatch/load duplication first. | 1–3 d | W9/W10 evaluation.rs | QN, EQ; NEW graph-lowering equality | AFTER W12 row-cost inventory; optional W14 only with its own >=3% incremental A3 case. |
| Q3 Tiled/chunked GPU interpreter with rotation halo | Q:39 | A | quotient_evaluation 72.882 | Whole-phase 2–5x planning scenario saves 36.4–58.3 s = 14.6–26.5%; scout's 5–15x kernel expectation is not a whole-phase promise. Includes measured internal NTT costs in accounting. | 15–25 d | W10 owns; W9 rotation/domain/shared lock | QN, NT, DEV, EQ | DO NOW in existing W10, after attributed W12 baseline. Largest measured target. No duplicate W12+ kernel package. |
| Q4 Resident fixed/permutation/Lagrange-boundary cosets | Q:40 | A | quotient_evaluation 72.882; compute_nu_poly 14.103 (transfer subset unknown) | Transfer-only gain unmeasured, likely <3% by itself; resident pipeline scenario on extension gives 8.5–10.6 s at 2.5–4x = 3.4–4.8%, before extra costs. Not the sum of full Q and extension. | 3–5 d | W9 keygen/domain; W10 quotient/NTT | PK, NT, DEV | AFTER W10, W15 enabling component; DROP standalone cache-for-speed package without >=7.5 s transfers. |
| Q5 Reduce quotient degree / gate layout | Q:41 | B | quotient_commitment 7.614; quotient_evaluation 72.882 | d=5 to 4 saves at most one of four quotient MSMs: 1.904 s <0.9%, before extra permutation sets. No N change. d=3 not available with lookups. | 5–15 d | W9 domain/keygen; W10 evaluator; circuit owner outside Loki | CIR, QN, NT | DROP isolated degree tweak; revisit only within W18 if actual domain threshold falls. |
| Q6 CPU row chunking to avoid full extended domain | Q:79 | A | quotient_evaluation 72.882; compute_nu_poly 14.103 | No NTT-count reduction; no >=3% time case. Full-domain inverse transform remains required. | 3–5 d | W9 domain; W10 evaluator | QN, NT | DROP as speed proposal. Row tiling remains a W10 memory requirement. |
| Q7 Lookup coset reuse / early release | Q:85 | A | quotient_evaluation 72.882 (subset unmeasured) | No cross-lookup reuse demonstrated; release changes memory lifetime, not arithmetic. Expected <3%. | 1–2 d | W10 evaluator | QN, DEV | DROP standalone; use early release only to satisfy W10/W15 memory cap. |
| Q8 Borrow cuZK quotient implementation | Q:68 | A (intended) | quotient_evaluation 72.882 | 0 demonstrated: captured cuZK has no quotient kernel. | 0 implementation | W10 if attempted | QN, NT would be required | DROP; wrong component. |

## Inventory: KZG commitments and multiopen

The scout's K:19 estimate of 3.4 s/1.5% is superseded by **11.480 s**. Nevertheless a 30% multiopen saving is only **3.444 s / 1.4–1.6%**. A 2x whole-phase gain is only 2.3–2.6%. Meeting 3% requires removing **57.5–65.3% of this phase** at 220–250 s, or 76.5% at the observed 292.712 s. W11 should test that proposition before expanding scope.

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| K1 One-pass divide_by_roots, narrow #565 port | K:81 | A | kzg_multiopen 11.480 | Scout's bundled 30% phase improvement bounds this at 3.444 s = 1.4–1.6%; isolated part smaller. | 2–4 d | W11 KZG mod.rs; shared arithmetic.rs | OPEN, TR, EQ | DROP as new standalone package; W11 may evaluate already-in-flight work, no claimed >=3% return. |
| K2 Skip unused prover query evaluations; borrow polynomial references (#554) | K:82 | A | kzg_multiopen 11.480 | Unknown query share. 50% phase removal = 5.74 s, still <3%; >=65.3% removal needed at 250 s. Could dominate but must be timed. | 1–3 d | W11 utils.rs/mod.rs/query.rs | OPEN, EQ | AFTER W12 subphase timing, W11 only; DROP unless whole W11 candidate clears >=3%. Port narrow hunks, not label refactor. |
| K3 Clone-free f/final-poly folds and sort | K:83 | A | kzg_multiopen 11.480 | A few percent of phase = well below 0.5% A3. Memory traffic alone is insufficient evidence. | 1–2 d | W11 KZG mod.rs; arithmetic.rs | OPEN, TR | DROP standalone. |
| K4 Parallel kate_division scan | K:84 | A | kzg_multiopen 11.480 | Scout estimates 0.15 s = 0.06–0.07%; extra multiplications may lose at four workers. | 3–5 d | W11 arithmetic.rs/mod.rs | OPEN; NEW linear-recurrence scan equality | DROP. |
| K5 Device q-polys/division/f/pi residency | K:85 | A | kzg_multiopen 11.480; quotient_construct 0.894; compute_nu_poly 14.103 | Multiopen alone at 2–4x saves 5.74–8.61 s = 2.3–3.9%, before transfers. Conditional >=3% only with measured affected share, or genuine cross-phase residency saving. | 10–20 d | W11 multiopen; W10 domain/quotient; W9 GPU runtime | OPEN, NT, MS, DEV, TR | AFTER W10+W11, W15 optional tail; omit tail if incremental gain <3%. |
| K6 SHPLONK-shaped protocol to GWC | K:88 | B (protocol/verifier) | kzg_multiopen 11.480 | No measured saving; full elimination ceiling only 4.6–5.2%. f/pi depend on sequential challenges. | 3–6 w | W11 plus verifier/recursive gadget | NEW PCS soundness and transcript relation, CIR | DROP; no speed evidence, incompatible stock protocol. |
| K7 fflonk | K:89 | B (protocol/verifier) | kzg_multiopen 11.480; recursive circuit phases changed | Source has slower proving at every t; no positive A3 estimate. Smaller proof/VK is a separate goal. | 3–6 w | W11 and recursive verifier | NEW fflonk PCS/gadget soundness, CIR | DROP for A3 speed. |
| K8 Fixed-base SRS window precomputation | K:92 | A | MSM subsets across advice 2.841, quotient commitment 7.614, products 24.645, KZG 11.480 | Kernel time not isolated; once GPU MSM is ~0.058 s, all 45 calls would total ~2.6 s under that illustrative warm-rate assumption. Cannot justify >=3% extra gain. | 5–10 d | W9 MSM/cache; W11 commit dispatch; W10 memory | MS; NEW precomputed table equals SRS multiples | DROP standalone; memory competes with quotient. |
| K9 Fix two-entry SRS cache thrash and coordinate scans | K:94 | A | MSM portions of quotient commitment 7.614, products 24.645, KZG 11.480 | O(n) scans/misses unmeasured; even 45 calls at 0.247 s cold versus 0.058 s warm saves only illustrative 8.5 s (3.4–3.9%) if almost every call misses. Actual miss histogram decides. | 2–4 d | W9 backend.rs; W10 residency; W11 commits | MS, DEV; NEW immutable SRS identity/cache-hit equality | AFTER W12 counters; W15 only if large misses measured. Never replace equality by raw pointer identity. |
| K10 Build/use Lagrange SRS for openings | K:95 | A | setup unmeasured in residual 117.323; kzg_multiopen 11.480 | 0 new gain: advice/products already commit in Lagrange form; multiopen needs coefficient SRS. | 0 | W9 params/MSM; W11 KZG | MS, NT, PK | DROP duplicate/wrong phase. |
| K11 Merge f and pi MSMs | K:97 | A (intended) | kzg_multiopen 11.480 | 0 valid saving: pi requires challenges derived after f commitment. | 0 | W11 | TR, OPEN | DROP; violates dependency/transcript order. |
| K12 Verifier-side eval_interpolant | K:81 | A | parent verification outside create_proof, part of 117.323 residual, not separately measured | No >=3% A3 verifier-time evidence; do not charge whole residual to interpolation. | 1–2 d | W11 verifier KZG | OPEN; NEW interpolation evaluation equality | DROP standalone; preserve unmodified stock verifier gate. |

## Inventory: lookups and permutation

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| L-A1 GPU batch inversion + grand-product scan + commit | L:116 | A | permutation_products 13.334 + lookup_products 11.311 = 24.645 | Whole-phase 2–5x saves 12.3–19.7 s = 4.9–9.0%; if only half is arithmetic, 2x saves 6.16 s and fails 3%. Residency and subphase timing are prerequisites. | 7–12 d | W9 runtime/MSM; W10 resident inputs; W11 only commit interface | BI, MS, NT, DEV, TR | AFTER W10+W15, W16; require fresh residual arithmetic >=15 s for a 2x candidate at 250 s. |
| L-A2 GPU 255-bit permuted-input sort | L:117 | A | lookup_prepare 25.038, sort subset unmeasured | Scout ~0.1 s per lookup: even five lookups save <=0.5 s (<0.23%); actual parent lookup count not assumed from Fenrir. | 7–12 d | W10 consumes sorted columns; no direct W9/W11 file edit | LP, DEV | DROP standalone; 25 s phase includes compression/placement/commitments, not just sorting. |
| L-A3 GPU inversion alone | L:118 | A | products 24.645, inverse subset unmeasured | Already chunk-batched CPU; no >6.6–7.5 s inverse time demonstrated. Expected <3% including copies. | 3–5 d | W9 runtime; W10/W16 product pipeline | BI, DEV | DROP standalone; required inversion can be a W16 component. |
| L-A4 Hash counts + sorted leftovers + parallel input sort | L:119 | A | lookup_prepare 25.038 | 0 incremental gain: already on main; full-proof W3 result was slightly slower/noisy. | 0 | existing lookup code; W10 consumer | LP, EQ | DROP duplicate; retain deterministic behavior as baseline. |
| L-B1 LogUp | L:130 | B (argument and recursive/ledger verifier) | lookup_prepare 25.038 + lookup_products 11.311; quotient 72.882 partly changed | Eliminating all prepare would cap at 10.0–11.4%, but LogUp does not eliminate all preparation and adds new work. No net >=3% estimate established. | 4–8 w | W10 quotient; W11 query interface; circuit/verifier owners | NEW LogUp soundness and recursive gadget contract, CIR | LATER separate protocol migration; not a pinned-0.8.3 optimization. |
| L-B2 Smaller range-check limbs/tables | L:131 | B | domain-scaled work incl quotient 72.882, extension 14.103, prepare 25.038 | Same k20: unquantified and extra advice may lose. If all constraints/tables fit k19, halving only quotient+extension saves <=43.493 s (17.4–19.8%); conditional illustration, not measured gain. | 2–4 w | W9 domain/keygen; W10 degree/cosets; circuit owner | CIR with integer no-wrap bounds; QN, NT | AFTER W12 actual table/advice inventory; W18 only if lower-k fit demonstrated. |
| L-B3 Remove redundant lookups | L:132 | B | prepare 25.038, products 11.311, quotient 72.882 (per-lookup shares unknown) | A 200-row transcript saving at fixed k is tiny; removing expensive lookup argument work could help, but no measured count or time supports >=3%. | 3–7 d | W10 quotient; W11 opening queries; circuit owner | CIR: removed check implied by retained constraints | LATER W18 inventory, only if measured per-argument budget >7.5 s or crosses k threshold. |

## Inventory: whole-prover GPU/system work

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| G-H0 Prove actual GPU dispatch; compare real baselines | G:15,214 | A | all 175.389 inside; 117.323 outside; reference T 220–250 | No direct saving; prevents accepting a fallback/cross-version/thread-count artifact. Real counts: 45 MSMs and 79 transforms, not 14/17. | 1–2 d | W10 profile/launcher; W9 fallback/thread policy; W11 phase counters | EQ, NT, MS; profiling observational | DO NOW W12 after W10 H0 handoff; mandatory evidence prerequisite. |
| G1 Thread budget/affinity sweep, keep BLST no-threads | G:26 | A | all phases, total create_proof 175.389 | Unmeasured A3. W9 synthetic k20 CPU-budget is 40.3% slower; do not assume forcing BLST serial helps. Four Rayon workers alone does not cap BLST's stock pool. | 1–2 d measurement | W9 Cargo/MSM policy; W10 metadata | MS, TR; byte equality within each fixed thread count | DO NOW policy accounting in W12; DROP blanket no-threads recommendation. Larger-core tuning is a separate comparison, not GPU gain. |
| G2a Pinned reusable scalar/NTT staging | G:27,84 | A | transfer portions of extension 14.103, quotient 72.882 and MSM phases, unmeasured | 39 base + 40 extended transforms roundtrip ~12.4 GB; scout bandwidth assumptions imply ~1.0–1.6 s transfer reduction, plus scalar uploads ~0.1 s. Typically <1%; not 6–17% of A3. | 2–4 d | W9 MSM runtime; W10 NTT/quotient | DEV, NT, MS | DROP standalone; bounded reusable buffers can enable W15 safely. WSL pinning must be measured, not assumed. |
| G2b Immutable SRS identity instead of coordinate scans | G:27 | A | MSM host portions; quotient commitment 7.614 etc, no split | Unknown scan budget; raw pointer equality risks stale mutated bases. No >=3% case yet. | 2–4 d | W9 backend.rs owns validation; W10 cache budget | MS, DEV; NEW immutable owner/generation equality | AFTER W12, same K9 decision; no second cache package. |
| G3 SRS fixed-base precomputation | G:28 | A | MSM subsets of products 24.645, commitments 10.455, KZG 11.480 | Same K8; kernel already fast, expected additional <3% pending host attribution. External 192x/223x is not Loki evidence. | 1–2 w | W9 MSM; W10 VRAM; W11 commits | MS; NEW table correctness | DROP standalone. |
| G4 Async MSM handoff with ordered transcript writes | G:29 | A | quotient commitment 7.614, advice 2.841, KZG 11.480 plus product commits | Saving <=min(remaining GPU duration, independent CPU work). At illustrative 45 warm MSMs *0.058 s, hiding all GPU MSM is only ~2.6 s (<1.2%); dependent f/pi cannot overlap. | 1–2 w | W9 common mutex; W10 runtime/prover; W11 transcript | TR, MS, DEV | DROP standalone MSM async rewrite; permit chunk transfer overlap inside W15 only with measured >=3% total package. |
| G5 Resident extension/quotient/multiopen polynomial pipeline | G:30 | A | quotient 72.882 + extension 14.103 + KZG 11.480 + construct 0.894 | Distinguish W10 numerator gain from NEW residency gain. Extension at 2.5–4x alone saves 8.5–10.6 s (3.4–4.8%) if still present; KZG tail conditional K5. No additive Q3 double count. | 3–6 w full; W15 bounded 1–3 w | W10 quotient/domain, W9 runtime/keygen/MSM, W11 KZG | QN, NT, OPEN, DEV, TR | AFTER W9+W10+W11 ownership handoff, W15; keep only measured >=3% incremental work. |
| G6 Multi-proof scheduler / persistent context | G:109 | A | no single-A3 phase reduction; synthesis 7.262 per parent | 0 expected single-parent latency improvement from queue concurrency. Throughput hypothesis requires different benchmark and memory admission. | 2–4 w | W9 context; W10 VRAM; W11 workload scheduling | TR, PK, DEV; NEW job isolation | DROP from A3 latency plan; LATER if throughput explicitly becomes objective. |
| G7 NUMA tuning | G:143 | A | all phases, create_proof 175.389 | 0 plausible NUMA benefit on this single-NUMA-node WSL host. | hours | none | EQ observational policy | DROP. |
| G8 Replace/upgrade sppark for Blackwell support | G:124 | A | GPU MSM/NTT subsets of 175.389, unisolated | No measured gain; current Loki embeds sm_120 and works. No unsupported-architecture gap established. | 3–10 d | W9 CUDA build/runtime; W10 CUDA kernels | MS, NT, DEV | DROP speculative backend migration. |
| G9 Adopt Nocy native prover or alternate GPU prover wholesale | G:48,50 | A only if identical Midnight protocol; otherwise B | all create_proof 175.389 | External 2.8x/10.7x does not estimate Loki A3; captured Nocy FFT roundtrips and naive MSM do not establish a superior kernel. | 4–8 w | W9/W10/W11 all | full EQ plus concrete QN/NT/MS/OPEN obligations | DROP wholesale adoption; W10/W15 may borrow architecture after license/field checks. |

## Inventory: witness synthesis and layout

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| W0 Patch actual parent to Loki and profile closure work | W:153 | A | synthesis 7.262; create_proof 175.389 | No direct saving. W10 already provides command-line dependency override; scout's prerequisite was missing then, not now. | <=1 d reuse | W10 launcher/prover; none W9/W11 | EQ observational | DO NOW W12 reuse, do not edit recl9 Cargo.toml blindly or benchmark historical node. |
| W1 Region-parallel ordered fork/merge (Scroll) | W:154 | A | synthesis 7.262 | Even ideal 4x on ALL synthesis saves 5.447 s = 2.2–2.5%; region-only share lower. | 1–3 w | W10 prover timing; no direct W9/W11 floor-planner collision | WIT, TR | DROP: fails 3% even under generous four-worker assumption. |
| W2 Precomputed constant fixed-base MSM tables | W:155 | B | synthesis 7.262 plus structural quotient/lookup work 72.882/25.038 | Witness-only saving <3% at realistic speedups. Circuit-size benefit unknown; 945 adds and 63 bases are Fenrir assumptions, not actual two-child A3 counts. | 2–4 w | none direct; W9/W10 when keys/domain change | CIR: constant tables, selectors and subgroup validity | AFTER actual VK/row inventory; W18 only for lower-k fit or measured >=3% full-proof gain. |
| W3 Cache configure result | W:156 | A | synthesis 7.262, configure subset unmeasured | A small subset of 7.262 s cannot plausibly save 6.6–7.5 s. | 1 d | W10 prover.rs; none W9/W11 | PK-like full config identity; WIT | DROP standalone. |
| W4 Compact/direct-F advice buffers, phase allocation reuse | W:157 | A | synthesis 7.262 | Memory saving (~1.1 GB proxy), no time result; parent single-phase so cross-phase waste absent. Expected <3% A3. | 2–4 d | W10 prover.rs; shared poly/mod.rs | BI plus NEW assignment-buffer/inverse-consumption equality | DROP for speed; LATER only if measured RSS prevents admitted residency. |
| W5 Outer Rayon across advice commitments | W:159 | A | advice_commitments 2.841 | Even elimination <=1.3%; inner BLST/Rayon already parallel and GPU serialized. | 1–2 d | W9 MSM/thread policy; W10 prover | MS, TR | DROP. |
| W6 Remove region shape pass / closure replay | W:145,161 | A claimed | synthesis 7.262 | Even halving all synthesis saves 3.631 s = 1.5–1.7%; unsafe for copies/side effects as general rewrite. | 3–10 d | floor planner, W10 prover | WIT missing; arbitrary closures not proven replay-free | DROP; no generic semantics-preserving skip established. |
| W7 Move eager witness work into Value suppliers | W:145 | A only if cells/layout identical | synthesis 7.262 | Only duplicated non-value work affected; upper realistic half saving <1.7%. | 2–5 d | circuit sources; none direct W9/W10/W11 except timing | WIT, EQ | DROP standalone A3 work. |
| W8 Cache layout/fixed constants across proofs | W:144 | A only for key-bound data | synthesis 7.262 | Witness-only upper four-worker/partial savings <3%; key-cache saving belongs separately to S5. | 3–7 d | W9 keygen identity; W10 prover | PK plus WIT layout equality | DROP standalone. |
| W9 Cache parsed children/transcript witness values across levels | W:126,143 | A only with full proof/input binding | synthesis 7.262 | Different children have different values; no repeated same-input work measured, expected <3%. | 3–7 d | recursion caller; none direct W9/W10/W11 | NEW witness cache keyed by complete child proofs/public inputs; WIT | DROP; never reuse private per-proof witnesses by VK alone. |
| W10 Cross-circuit / halo2-base virtual-region witness parallelism | W:135,137,161 | A only if same layout; otherwise B | synthesis 7.262 | One circuit per parent: cross-circuit gain 0; even a 4x witness rewrite <2.5%. | 2–4 w | none direct except W10 prover integration | WIT, TR; CIR if layout changes | DROP from latency plan. |
| W11 Poseidon2 transcript swap | W:128,161 | B (protocol/gadget) | synthesis 7.262; quotient/recursive circuit subset unmeasured | No gain estimate; stock transcript changes and current Poseidon partial rounds already batched. | 2–4 w | W11 transcript boundary; recursive gadget | NEW transcript/hash relation and protocol soundness, CIR | DROP for A3. |

## Inventory: recursion structure

Sherlock's actual parent verifies k13 A102 and k12 A80 children, accumulates once, and **already defers pairing**. It does not contain the decider's measured 316,833-row residue pairing. The Fenrir arity-4 node's 958k advice/1.03M table rows and 45/18 column counts are not the actual heterogeneous parent inventory. Removing a decider gadget cannot be booked as saving parent quotient time.

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| S1 k20 to k19 by table AND advice cuts | S:133,254 | B | domain-scaled quotient 72.882, extension 14.103, prepare 25.038, etc | If just Q+extension halve, <=43.493 s (17.4–19.8%); if all 175.389 s halves, theoretical 87.7 s (35–40%). Neither assumes residual falls. Actual costs may rise with extra columns. | 2–4 w sizing + implementation | W9 domain/keygen; W10 quotient shape; W11 query shape | CIR, QN, NT | AFTER W12 actual per-table inventory; W18 conditional. No k19 claim from historical rows. |
| S2 Fused verifier-free k15 terminal | S:157,253 | B (new relation, VK/deployment) | replaces parent 175.389 inside plus setup/check residual 117.323; decider outside A3 unmeasured here | Amdahl on replaced parent: at 10x ONLY create_proof, save 157.85 s = 63–72% normalized, 54% of observed 292.712 s. k15/20,466 rows and 10x are unbuilt scenarios, not measured forecasts. Chain gain requires separate baseline. | 2–4 w plus deployment, estimate | none direct W9/W10/W11; new circuit outside Loki | TERM, LED if output/deferred obligations change | LATER W17, highest-leverage Track B route; require actual row/table sizing and relation equivalence. |
| S3 Remove decider pairing by native ledger decision | S:178,255 | B (ledger-facing statement) | no parent phase: 0 measured A3 seconds; decider timing not in A3 | 0 direct A3 parent gain. 38% decider row reduction is not a 38% parent or chain wall reduction. Ledger accumulator check unverified. | 1–3 w | none direct; recursive/ledger owners | LED, CIR | DROP from A3 plan; separate chain protocol research only. |
| S4 Change fan-in/tree shape | S:193 | B | domain remains k20, parent create_proof 175.389 | No established A3 saving; b=2 parent already minimum for its two distinct children. Extra layers add verification; b=8 historical node exceeds k20. | 1–2 w | none direct; keys affect W10/W11 shape | CIR, LED | DROP as standalone k20 speed-up. |
| S5 Key/PK/SRS reuse | S:205,252 | A | setup inside 117.323 residual; A3 setup seconds not separated | Historical 62.3/944.9 = 6.6% is not today's A3 share. Conditional saving equals measured repeated setup minus cache overhead; admit at >=7.5 s on T=250. Cold first proof has no cache saving. | 2–4 d after timing | W9 PK/domain validation; no need to edit W10/W11 kernels | PK, EQ | AFTER W12 residual split, W13; strongest cheap candidate, but no invented A3 setup time. |
| S6 CycleFold/Sangria/ProtoGalaxy/HyperNova folding | S:212 | B (proof/curve stack) | would replace create_proof 175.389 and recursion, no transferable split | No A3 prediction; source N=16 model loses. A BLS12-381 cycle partner is not established by captured evidence; do not assert mathematical impossibility. | 2–6 months | all owners if backend replaced | NEW folding soundness, curve bridge, terminal wrapper; TERM/LED | DROP from bounded speed-up plan. |
| S7 snark-verifier / UniPlonK aggregation | S:229 | B (protocol/curve) | recursive circuit portion unmeasured; no direct kernel mapping | No measured BLS12-381 A3 gain; foreign BN254 numbers not transferable. | 1–3 months | W11 PCS/recursive verifier; W10 circuit shape | CIR, LED plus NEW PCS/aggregation soundness | DROP. |
| S8 fflonk t=4 | S:239,256 | B (PCS/gadget) | KZG 11.480, recursive circuit cost unmeasured | Source proving 1.29x slower, no k drop demonstrated; smaller VK is not parent speed evidence. | 3–6 w | W11; circuit verifier | NEW fflonk contract, CIR | DROP; same decision as K7. |

## Inventory: other chains' PLONK/KZG techniques

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| P1 Parallel scale-add/independent set divisions, Axiom #37 | P:30,50 | A | kzg_multiopen 11.480 | Even 2x on entire phase saves 5.74 s <3%; targeted work less. | 2–4 d | W11 arithmetic/mod.rs | OPEN, TR | DROP standalone; same owner/evidence test as K1–K4. |
| P2 Small CPU MSM scheduling, zcash #796 | P:31,64 | A | 45 MSM entries: 44 at 1,048,576, one at 1,048,575; MSM subsets of phases above | A3 has no small MSM entry in this inventory; expected ~0% relevant gain. Pasta k8–14 numbers do not bound BLS BLST k20. | 3–7 d | W9 MSM; W11 commits | MS, TR | DROP for A3. |
| P3 Vartime inversions/affine witness conversion, zcash #962 | P:32,83 | A if same values and existing threat model permits | synthesis 7.262 | Batched inversions already present; even 4x ALL synthesis saves only 5.45 s <3%. Source's zero-denominator prose is not a substitute for BI zero semantics. | 2–4 d | W9 field/MSM interfaces; W10 prover | BI, WIT | DROP. |
| P4 Recompute fixed cosets rather than retain PK cosets | P:33,95 | A | quotient 72.882 plus setup residual subset unmeasured | Negative speed precedent (+24% source aggregation proof time); memory benefit cannot be counted as time reduction. | 1–3 d | W9 keygen/domain; W10 cosets | NT, PK, DEV | DROP for speed; streaming only if required to fit W15. |
| P5 Direct sparse instance commits / batch normalization, zcash #938 | P:34,108 | A | compute_instances part of other 4.527 (not isolated) | Even eliminating ALL other <=2.1%; actual instance subset smaller, zeros already filtered on CPU. | 1–3 d | W10 prover; W9 MSM | MS, TR, EQ | DROP. |
| P6 CycloneMSM / batch-affine inversion | P:122 | A if ported exactly | MSM subsets, no isolated timing | No demonstrated gain over current curve-specific BLST/GPU; expected incremental <3%. | 1–3 w | W9 MSM | MS, BI | DROP; different baseline/curve. |
| P7 Signed Booth recoding | P:123 | A | MSM subsets, no isolated timing | Generic path optimization does not affect current BLS G1 BLST path; 0 expected A3. | 3–7 d | W9 MSM | MS | DROP. |
| P8 Scroll LogUp / par_invert / counting map | P:124 | B for LogUp; A for helpers | prepare 25.038 + products 24.645 | Helper gains already taken; LogUp has no net A3 measurement. | 0 helpers; 4–8 w protocol | W10 quotient; W11 query interface | BI/LP existing; NEW LogUp soundness for B | DROP duplicate helper work; LATER protocol case as L-B1. |
| P9 Prescale fixed SRS bases, c-kzg #628 | P:125 | A | MSM subsets of commitments/products/KZG, unisolated | Per-commit scalars vary. Source fixed weighting needs an exact mapping; 7.7% cells-proof gain cannot transfer. No >=3% A3 case. | 3–7 d | W9 SRS/MSM | MS; NEW prescaled-base identity with correct compensating scalars | DROP. SRS bases themselves ARE fixed; source's claim that all Loki bases vary is too broad. |
| P10 Skip SRS load subgroup checks, c-kzg #299 | P:126 | A only if validation contract preserved | setup residual 117.323, subgroup subset unmeasured | No matching check to remove in Loki; W9 adds necessary validation. 0 established saving. | 0 | W9 SRS validation | MS and validated-SRS invariant | DROP; do not undo hardening for unrelated load benchmark. |
| P11 Lagrange wire commits, barretenberg #94 | P:127 | A | advice 2.841 + product commit portions | 0 incremental; already used. | 0 | W9 MSM; W11 commit interface | MS | DROP duplicate. |
| P12 Parallel Gemini folding | P:128 | B (multilinear PCS) | no matching A3 phase; KZG 11.480 is different PCS | No transferable timing or algorithm. | 1–3 months | W11 PCS | NEW Gemini PCS/recursive gadget soundness | DROP. |
| P13 Goblin/ECCVM/UltraHonk | P:129 | B (PCS/curve stack) | would replace recursion, no measured phase mapping | No measured A3 gain; not a drop-in kernel. | 2–3 months | W10/W11 if substituted | CIR/LED, NEW curve/PCS bridge | DROP as Loki speed package. |
| P14 Scroll/Snarkify vendor prover claim | P:130 | A only if same protocol after port | quotient 72.882 / extension 14.103 are intended targets | Vendor ratio against competitor yields no A3 prediction. | 0 separate package | W10 | QN, NT, DEV | DROP separate migration; use captured architecture within W10. |
| P15 ZPrize BLS12-377 FPGA MSM | P:131 | B if changing curve; A only new BLS12-381 hardware backend | MSM subsets, unisolated | 0 transferable gain to existing BLS12-381 RTX host. | hardware project | W9 MSM | MS and NEW hardware/field contract | DROP. |
| P16 Load-balanced BLS12-381 GPU MSM | P:132 | A | MSM subsets of commitments/products/KZG | Reported 1.8–2x over cuZK, not sppark; even halving illustrative 2.6 s warm MSM budget <0.6%. | 1–3 w | W9 MSM | MS | DROP incremental kernel chase. |
| P17 OPTIMSM FPGA | P:133 | A if exact same PCS/curve | MSM subsets, unisolated | Hardware benchmark is not RTX A3 evidence. | hardware project | W9 MSM | MS and NEW hardware interface contract | DROP. |
| P18 plonky2/3, boojum, kimchi | P:134 | B (FRI/IPA/curve) | no matching A3 kernel; would replace 175.389 s prover | No transferable A3 estimate. | 2–6 months | W10/W11 proof-system interfaces | NEW PCS/wrapper soundness, TERM/LED | DROP. |
| P19 Filecoin supraseal | P:135 | A only with verified BLS/PCS mapping | MSM/NTT portions unisolated | Scout did not resolve repo; no speed evidence. | unknown research | W9/W10 | MS, NT required | DROP unsupported source as work item. |
| P20 Aleo Varuna / snarkVM | P:136 | B | no matching halo2 KZG phase; whole prover replacement | No timing or integration evidence. | months, unsized | W11 PCS; W10 shape | NEW Marlin/batching/wrapper soundness, LED | DROP. |
| P21 Scroll #27 partitioned cosetFFT | P:137 | A if exact transform | extension 14.103 plus transforms inside quotient 72.882 | Extension at 2x saves 7.052 s = 2.8–3.2%, marginal before overhead; existing GPU FFT may already remove it. Need fresh remaining cost. | 3–7 d | W9 domain; W10 quotient/NTT | NT, DEV | AFTER W10/W12, alternative inside W15 only if >=3% incremental. |
| P22 Axiom #28 work-stealing evaluate_h / parallel transforms | P:137 | A | quotient 72.882 (transform/interpreter split unknown) | Existing row evaluator already parallel; unmeasured extra-transform parallelism at four workers may add contention. No forecast >3%. | 2–4 d | W9/W10 evaluation/domain | QN, NT, TR | AFTER W12 inventory, W14 alternative only if qualifying; no separate work-stealing package. |

## Inventory: other projects' recursion/aggregation

| Item | Source:line | Track | A3 phase and measured seconds | Expected end-to-end A3 gain and reasoning | Effort | W9/W10/W11 conflict | Lean contract | Verdict and reason |
|---|---|---|---|---|---|---|---|---|
| R3 Fan-in 3–4 / bounded tree | R:53 | B | domain remains k20; create_proof 175.389 | 0 demonstrated gain for existing two-child A3; table remains, more children add work. | 1–2 w | none direct; keys/query shape affect W10/W11 | CIR, LED | DROP; same structural limitation as S4. |
| R5 Expand/audit GLV scalar splitting | R:54 | B if changing gates; A for audit only | synthesis 7.262; quotient/lookup structural subset unmeasured | GLV already present; no scalar coverage or row reduction establishes >=3% or lower k. | 1 w audit | circuit owner; no direct W9/W10/W11 edits | CIR: scalar bounds, decomposition equality, subgroup assumptions | LATER W18 inventory, not a new GLV implementation by default. |
| R9 Fewer opening point sets / redundant lookups | R:55 | B | lookup 25.038/11.311; KZG 11.480; quotient 72.882 partly | ~200 Poseidon rows alone at unchanged k gives no >=3% time case; eliminated full argument may differ. | 1–2 w | W11 opening shape; W10 quotient | CIR, OPEN | LATER only same measured circuit-diet gate as L-B3. |
| R2 Fused terminal | R:56 | B | replaces 175.389 create_proof + part of 117.323 residual | Same S2 conditional 10x prover scenario; unbuilt row budget, no measured forecast. | 2–4 w plus deployment | none direct W9/W10/W11 | TERM, LED | LATER W17; one package, not two fused-terminal projects. |
| R1 One-shot native proofs of deferred non-native EC ops | R:57 | B (new relation/decision) | recursive circuit portion of quotient 72.882 etc, no measured EC split | External 10–50x gate claim is not time reduction. No A3 cost for auxiliary proof/terminal decision. | 6–12 w | circuit/decider owners; W11 wrapper interface | NEW deferred-op soundness, TERM, LED | LATER research only after W17 sizing; no unbound auxiliary witness acceptance. |
| R6 Limb/range-table choice | R:58 | B | domain-scaled Q 72.882 / extension 14.103 / lookup 25.038 | Same conditional W18 budget as L-B2/S1; no number from generic halo2-lib limb API. | 2–4 w | W9 keygen; W10 shape; circuit owner | CIR integer no-wrap/carry/range bounds | AFTER W12 inventory, W18 only on actual fitting domain or >=3% measured saving. |
| R8 Lazy reduction / CRT | R:59 | B for constraints; A only witness-side identical values | synthesis 7.262 for A; structural prover phases for B unmeasured | A variant even 4x all synthesis <3%. B variant has no counts, possible carry/range costs. | 2–6 w estimate | circuit owner; none direct W9/W10/W11 | CIR for B; WIT/BI for A | DROP A optimization; LATER B within measured circuit-diet study. |
| R10 UPA outer wrapper / Keccak digest | R:60 | B (ledger statement) | no isolated parent phase; replaces terminal interface | Vendor verification-cost claim not A3 proving gain; may add wrapper work. | >=4 w | W11 PCS wrapper; ledger/circuit owners | LED, CIR, NEW digest binding | DROP. |
| R11 Statement batching / Varuna | R:61 | B (protocol) | no matching phase; would replace parent 175.389 | No measured parent latency gain; throughput different objective. | unsized, months | W11 PCS; circuit owner | NEW batching soundness and ledger mapping | DROP. |
| R4 Cycle-curve EC split / ECCVM | R:62 | B (PCS/curve) | would replace recursive constraints, no measured phase split | No costed BLS-compatible curve/PCS bridge; no A3 estimate. | 2–3 months | W11 PCS; W10 shape | NEW curve bridge, CIR, LED | DROP from bounded plan; no claim that lack of known cycle is a proved impossibility. |
| R7 Telepathy naive foreign-field calibration | R:63,106 | B for any adopted circuit | synthesis 7.262 / structural recursive work unisolated | Reference only: 27M/68M Groth16 constraints and 124/180 s witness are not Midnight rows or timing. | 0 | none | CIR if ever ported | DROP as implementation item; retain only as context. |
| R12 Electron universal aggregation | R:64,103 | B | no phase mapping | No technique or measured number recovered. | unsized | none until specified | NEW aggregation/ledger contract would be needed | DROP unsupported item. |
| Already pairing deferral from child verification | R:37 | A to retain; B to change | existing parent 175.389; no in-parent pairing phase | 0 incremental; parent's accumulator already defers pairing. | 0 | W10/W11 consume current structure | EQ, LED | DROP duplicate. |
| Already one-shot collapse with fixed bases merged | R:38 | A to retain; B to change | existing parent 175.389; collapse synthesis subset of 7.262 | 0 incremental; shared merge/collapse already runs once. | 0 | none direct; W10/W11 consume structure | EQ, CIR | DROP duplicate. |
| Already residue check without final exponentiation | R:39 | A to retain; B to change | no parent phase; decider outside A3 | 0 direct A3 gain; decider already uses residue decision. | 0 | none direct | LED, CIR | DROP duplicate; never subtract decider rows from parent. |
| Already width-4 windowed MSM / conditional GLV | R:40 | A to retain; B to change | synthesis 7.262; structural quotient work unisolated | 0 incremental from reimplementing current window/GLV. | 0 | none direct | CIR scalar decomposition/bounds | DROP duplicate; coverage audit is R5. |
| Already Poseidon in-circuit transcript | R:41 | A to retain; B to change | synthesis 7.262; recursive constraint costs unisolated | 0 incremental; no SHA-to-Poseidon saving remains to claim. | 0 | W11 transcript boundary | TR, CIR | DROP duplicate. |
| Already four range-check lanes | R:42 | A to retain; B to change | current domain-scaled quotient 72.882 / lookup 25.038 | 0 incremental; changing lanes is a new-key circuit design. | 0 | W9 keys; W10 shape | CIR range soundness | DROP duplicate; actual limb/table sizing belongs to W18. |
| Already separate accumulation and decision | R:43 | A to retain; B to change | parent 175.389; decision in separate decider | 0 incremental; current chain already separates these obligations. | 0 | W11 PCS/recursive interface | LED, CIR | DROP duplicate; retain final accumulator decision. |
| Mina Pickles / Zcash IPA accumulation | R:70–76 | B (curve/PCS) | no matching BLS KZG phase | No measured A3 gain; shares R4 curve/PCS migration requirement. | months | W11 PCS | NEW cycle/IPA bridge, CIR/LED | DROP as separate work. |
| SP1/RISC Zero/zkSync/plonky2 STARK-to-SNARK wrap | R:87–95 | B (proof-system stack) | whole parent replacement; no compatible phase split | Published shard/wrap throughput or theoretical parallel latency does not predict single A3. | months | W10/W11 interfaces; circuit/ledger owners | NEW inner proof + BLS outer wrapper soundness, TERM/LED | DROP from Loki package list. |
| ClientIVC/protogalaxy, Scroll/Taiko, Polyhedra gaps | R:81,85,108,111 | B if adopted | no measured phase mapping | No recovered rows/k/time or complete implementation mapping. | unsized | none until a concrete proposal | CIR/LED plus protocol-specific NEW contracts | DROP unsupported gaps; no additional research lane by default. |

## Ordered packages and ownership handoffs

Order is by expected return on the current task, with conditional packages explicitly stopped when their evidence fails. **W10's existing quotient is the first optimization, not a newly numbered competing package.** W9 hardening remains a dependency for safe integration. W11 remains the only writer of multiopen; no W12+ package copies its scope while it is in flight. Do not edit another lane's dirty files or count a planned feature as present.

### W12 — Close the A3 measurement and verifier-attribution gap (DO NOW)

**Scope and deliverable:** one reproducible baseline/ablation harness using identical actual A2 children, parent relation, SRS and VK across stock, Loki CPU and Loki GPU. Split the 117.323 s residual before selecting PK caching. Record quotient's interpreter/NTT/transfer split, lookup prepare's compression/sort/count-placement/commit split, product inversion/scan/commit costs, multiopen query/evaluation/division/clone/MSM costs, and cache hits/misses/bytes. All are public counters/timing; do not export witnesses or transcript secrets.

**Files:** after W10 handoff, reuse `examples/loki-a3-profile.rs`, `src/profile.rs` and `tests/stock-profile-verifier/{Cargo.toml,src/main.rs}` from W10; use `loki-harness/{Cargo.toml,src/backend.rs,src/lib.rs,tests/differential.rs}`. Add boundary timers around `prove_link_parent` in `/home/charles/repos/Moriarty-loki/crates/mori-recursion/src/parent.rs:974` and its existing `tests/sherlock_parent.rs`; do not reimplement the historical recl9 workload. W10 owns profiler/prover changes until handoff. W11 supplies its subphase counters rather than permitting a concurrent edit of KZG files.

**Critical observed gap:** current `loki-harness/Cargo.toml` has `cuda = []`; running `--features cuda` alone does not exercise the GPU. Wire that feature to `loki/cuda` and `loki/cuda-ntt`, or pass the dependency features explicitly, then assert actual dispatch in the tested proof. Add quotient forwarding only after W10 actually supplies `cuda-quotient`. The A3 launcher's `gpu-after` string anticipates that feature; it does not establish an implemented kernel. Also pin an explicit A2 children directory rather than silently choosing the newest directory by modification time.

**Contract:** EQ, NT, MS; observational timers must not change values/RNG/transcript. W9 restores stock BLST defaults and makes `loki-cpu-budget` opt-in. Preserve its error-to-CPU NTT path during profiler merge, record fallback distinctly, and report effective BLST/Rayon/affinity policy. No automatic instruction to keep BLST single-threaded.

**Gates / sequence:**

- [ ] Check exact source revisions and Cargo dependency trees. Stock verifier must resolve registry `midnight-proofs 0.8.3` without Loki override. Record Moriarty test version and ledger verifier version separately; 0.8.1/0.8.2/0.8.3 are not interchangeable by assumption.
- [ ] Run full W2 k10/k14/k17 cross-verify, determinism controls and rejection controls with `RAYON_NUM_THREADS=1`. Default harness invocation is `cargo test --release --locked --manifest-path loki-harness/Cargo.toml --test differential -- --test-threads=1`; include real forwarded GPU features for the GPU matrix. k20 ignored test's implementation only calls `determinism(20)`, so it does not replace actual Sherlock A3.
- [ ] Verify pointwise NTT/MSM outputs, no-lookup stock proof byte equality, repeatable Loki outputs, and stock acceptance for lookup fixtures. Lookup byte inequality from stock HashMap order is documented; it does not exempt pointwise optimized-vs-same-Loki-oracle equality. Keep thread count, RNG stream and lookup order fixed when comparing bytes.
- [ ] Demonstrate GPU counters in the actual proof, including error/fallback injection with unchanged CPU inputs; CPU baseline must report no GPU dispatch. Nonzero GPU MSM/NTT alone will not prove a future quotient kernel ran: add `quotient.gpu`/row counts and require them for W10/W15.
- [ ] Export actual A3 proof/public instance/VK/verifier params, verify with unmodified stock 0.8.3 and full transcript consumption; include wrong-input/tampered/trailing-byte rejection. The existing k14 stock-profile example is a functional precedent, not the actual A3 verification adapter.
- [ ] Measure fresh stock, integrated CPU and GPU runs sequentially on the same host, four Rayon workers and explicitly recorded BLST policy. Use at least three paired runs; alternate order, print cold/warm cache state, exclude compile/queue time, retain phase sums, fallback/error counts, RSS/VRAM, all samples and medians. Repeat only if spread prevents deciding the 3% threshold. Fixed four-worker and any wider-core policy have separate baselines.

**Acceptance:** measurement package has no speed threshold. It succeeds only when baseline discrepancy and residual are attributed well enough to test the next package. Dependencies: W9 policy/fallback handoff and W10 H0 telemetry; no dependency on W10 kernel completion to measure its before state. Effort 1–2 d plus host queue.

### W13 — Reuse validated parent proving keys (AFTER W12, conditional)

**Scope:** add an explicit prepared-parent object or caller-owned cache so repeated proofs reuse the actual PK instead of `setup_pk(rel, vk)` followed by `drop(pk)` on each call. Preserve per-proof witness/child/public-input/accumulator checks. Full identity includes validated SRS, VK, relation including child VKs/architecture, and k; invalidate on any mismatch. Prefer in-process ownership first over a new serialized private-witness cache. Report cold and warm latency separately.

**Files:** Moriarty `crates/mori-recursion/src/parent.rs` (`prove_link_parent`, prepared-key construction) and `crates/mori-recursion/tests/sherlock_parent.rs`; add a focused integration fixture there for reuse and wrong-relation/key/SRS rejection. No W9 `keygen.rs`/domain edit required: consume its validated key APIs after merge. No W10/W11 code edits.

**Contract:** PK and EQ. Extend the concrete cache-key binding to actual relation/params identity; current mathematical `Key` abstraction is not a license to key only by k or an unverified digest. Never cache entropy, private witnesses or prior proof randomness.

**Gates:**

- [ ] W12 must show >=7.5 s removable repeated setup in a comparable <=250 s baseline, or >=3% of its actual measured total; otherwise DROP W13. Historical 62.3 s is insufficient.
- [ ] Compare fresh and reused key serialization, VK, fixed/permutation cosets, and resulting proof bytes under fixed RNG/threads/order. Count one key build across multiple same-key proofs; changed identity must miss or reject. Run negative witness/child checks again on every proof.
- [ ] Full W2 matrix plus actual A3 stock-verifier acceptance. Compare two real A3 proofs with different witnesses/entropy; both must verify, and reuse must not reuse randomness.
- [ ] Paired A3 measurement includes lookup, validation and cache overhead; >=3% warm full-request wall reduction versus same backend without cache. Publish cold result too. First proof is not advertised as faster.

**Dependencies:** W12 residual split; W9 validation. Effort 2–4 d. If W12 identifies repeated cost-model work as another large residual, assess separately with a full relation-bound contract; do not remove security checks to meet a cache target.

### W14 — Exact CPU quotient evaluator improvement (AFTER W9 + W10, conditional)

**Scope:** fuse row passes preserving the complete ordered y-Horner term stream across circuits, permutation sets, lookup identities and trash; eliminate duplicate loads/CSE only if counted dispatch work justifies it. Hoist coset preparation without increasing live memory beyond the measured cap. Keep an unoptimized CPU oracle for differential testing. This improves the CPU fallback; on the GPU path accept only a measured host-lowering or remaining CPU-stage gain.

**Files:** `src/plonk/evaluation.rs` (`Evaluator::new`, `GraphEvaluator`, `evaluate_numerator`, test module), `src/plonk/prover.rs` only for an agreed boundary if required. W9 rotation widening must survive; W10 owns these files until its kernel/lowering layout is frozen. No W11 KZG edits.

**Contract:** QN/NT/EQ; NEW `Plonk.fusedRows_eq_numeratorAt` and `Plonk.graphLowering_eq_expr` obligations (names proposed). Test multi-circuit Horner seeds and every challenge, including trash and committed instances; general backend proof alone does not cover omitted terms.

**Gates:**

- [ ] W12 must expose >=15 s genuinely affected work for a plausible 2x variant, or another explicit budget giving >=3% full A3. If W10 already consumes this share, DROP instead of double-counting.
- [ ] Pointwise equality of complete extended numerator and final h against unchanged CPU oracle for random/edge fixtures, negative/large rotations, last rotation, zero/empty arguments, multiple sets/circuits, trash and committed instances. Compare blinded h pieces/commitments under fixed RNG.
- [ ] Full W2 GPU/CPU cross-verify and deterministic controls; actual A3 accepted by stock verifier. Maintain W9 fallback/error tests.
- [ ] Fresh paired CPU A3 >=3% faster than stock-compatible pre-change CPU at same policy; GPU A3 non-regressing. If the package is proposed as a GPU speed-up, it must separately save >=3% versus W10 GPU. Otherwise label it CPU-fallback-only. DROP CSE add-on if it fails the incremental criterion.

**Dependencies:** W12 and exclusive handoff after W9+W10; W13 optional and held constant in comparisons. Effort 3–7 d; stop after the measured budget fails, not after a succession of tiny kernel wins.

### W15 — Resident polynomial pipeline beyond W10 (AFTER W10 + W11, conditional)

**Scope:** start with advice/instance extension into W10's resident numerator inputs and vanishing division/inverse transform, retaining exact h output. Own a bounded device arena/context and read-only key-coset cache. Row chunks carry signed rotation halos and wrap correctly; the arena must coexist with resident SRS and W10 intermediates on 32 GB. Include reusable staging as a memory/correctness component. Only add coefficient-polynomial/MSM handoff and KZG tail when their remaining timed budget independently clears 3%.

**Files:** `cuda-ntt/{src/lib.rs,cuda/ntt.cu}`, `src/poly/domain.rs`, `src/plonk/{prover.rs,evaluation.rs,vanishing/prover.rs}`, W10's quotient module once actually introduced, and the existing `loki-cuda-runtime` crate handed off by W9. Cache/commit changes may touch `src/poly/kzg/msm/cuda/backend.rs` and `cuda/msm.cu` after W9; a KZG tail touches `src/poly/kzg/{mod.rs,utils.rs}` and arithmetic helpers **only after W11 handoff**. Add corresponding differential tests next to each module. New paths for arena/cache should be selected against W10's landed layout, not invented as already-existing files.

**Contract:** QN, NT, MS, DEV, TR, PK for key-bound cosets; OPEN for any multiopen tail. Add concrete readback/rotation/normalization/lifetime/cache-hit obligations to Lean. Preserve W9's single shared device mutex, upload validation, unchanged-input fallback and mutation-at-stable-address regression. An immutable SRS handle can avoid repeated comparisons only if the API prevents mutation and proves identity/lifetime; pointer/length alone cannot.

**Gates:**

- [ ] Reprofile landed W10. At 14.103 s original extension, a 2x result is marginal (7.052 s); target >2.2x effective extension or a separately measured combined pipeline saving. Require >=3% incremental A3 saving after transfers/arena overhead; remove scopes already accelerated by W10.
- [ ] Pointwise natural-order transforms, ZETA scaling, every coset/rotation, inverse 1/n exactly once, numerator and h/pieces equal CPU. Test chunk-boundary rotations, minimum/maximum/partial chunks, low-VRAM streaming and absent GPU. Memory failure must release or safely disable resources and recover on CPU without partial input mutation.
- [ ] Full W2 matrix actually enabling GPU and quotient forwarding, stock 0.8.3 acceptance of actual A3, changed inputs/proof mutation rejection. If multiopen tail is included, compare q/f/final/pi coefficients, point commitments, set ordering, truncated-challenges on/off, and x1..x4 transcript events with W11 CPU oracle.
- [ ] A3 dispatch attribution includes quotient GPU row/tile counts, not just NTT/MSM. Record H2D/D2H bytes, peak device+host memory, cache identity misses and host conversion costs. One heavy proof at a time; do not infer two-proof fit from estimated 19 GB usage.
- [ ] Fresh paired before/after GPU A3 at same CPU policy and cache state >=3% wall reduction; no unmodified stock proof regression. Report full cold A3 and warm repeat, rather than amortizing startup invisibly.

**Dependencies:** W12; W9 runtime/domain handoff; W10 accepted quotient; W11 interface freeze (tail requires completed handoff). W14 optional. Effort 1–3 w for bounded first slice; a full 3–6 w rewrite is not pre-authorized by this ranking.

### W16 — Resident lookup/permutation grand products (AFTER W15, conditional)

**Scope:** device denominator construction, zero-preserving batch inversion, numerator multiplication and exclusive/initial-value-correct product scan, followed by commitments and resident quotient consumption. Cover lookup and every permutation set. Keep sorted-leftover lookup placement on CPU; no new standalone GPU sort lane.

**Files:** `src/plonk/lookup/prover.rs`, `src/plonk/permutation/prover.rs`, device arithmetic/scan modules adjacent to the landed W10/W15 CUDA arena; host adapters under existing CUDA crate(s). `src/utils/arithmetic.rs` remains the CPU oracle. W9 runtime/MSM interfaces remain fixed; no W11 multiopen implementation edits.

**Contract:** BI, MS, NT, DEV, TR and EQ. New CUDA kernels must meet the pointwise inversion and ordered product-column contracts, including zero denominators; do not replace zero skipping with a global product inversion that poisons other entries. Lookup permutation still satisfies LP.

**Gates:**

- [ ] Re-measure after W15. Combined original product phases total 24.645 s, but commits/transforms already accelerated cannot be claimed again. Proceed only if remaining arithmetic/transfer budget supports >=3% incremental A3; a 2x candidate needs >=15 s affected work at T=250.
- [ ] Pointwise CPU equality of factors, inverses, z columns, boundary/blinding rows and commitments. Test empty/singleton vectors, zeros at chunk boundaries, arbitrary initial product, multiple permutation sets and active/unusable-row boundaries. Retain sequential RNG draws and transcript writes.
- [ ] Full W2 matrix, actual A3 stock acceptance and negative controls; CPU-only/no-device fallback still works.
- [ ] Paired A3 demonstrates >=3% gain beyond W15, reports device memory/transfer costs and no fallback. If products are already cheap on the attributed GPU path, DROP rather than starting another kernel project.

**Dependencies:** W15 accepted residency and W12/W9 correctness gates. Effort 7–12 d.

### W17 — Fused terminal (LATER, separate Track B project)

**Scope:** directly prove A102→A80 transition fragments with linkage and statement hashing, replacing child-proof verification and the parent/decider chain for this fixture. First produce an actual circuit-size/cost prototype; k15/20,466 is a design budget, not a result. Do not silently substitute a single unrelated statement or omit child open obligations. Distinguish outer stock KZG verifier compatibility under a new VK from compatibility with the currently deployed key.

**Files:** `/home/charles/repos/Moriarty-loki/crates/mori-recursion/src/{parent.rs,decider.rs,reference.rs,inner.rs}`, source transition fragments corresponding to the cited `literal_bridge.rs`/`sherlock_link.rs`, and a new terminal relation module next to them; existing `crates/mori-recursion/tests/{sherlock_parent.rs,sherlock_terminal.rs}`. Add only a new relation module after confirming the actual fragment paths in the target worktree. This is outside W9/W10/W11 direct file ownership. Deployment templates and key manifests belong to the recursive/ledger owner, not Loki arithmetic lanes.

**Contract:** TERM and CIR; LED for whatever terminal decision/exposed accumulator remains. Today's LokiLean prover refinement has no A102/A80 relation-equivalence theorem. A new explicit transition/statement semantics contract is required; expose inherited open child assumptions. Do not claim it follows from `Refines`.

**Gates:**

- [ ] Inventory actual child constraints/rows and every lookup table; cost-model the fused relation and usable rows including blinding. Demonstrate k15 fit or reprice target at actual k. No row estimates from domain capacities alone.
- [ ] Prove/test statement equivalence for all 16 fields, envelope/context/work constraints and hash-domain order; reference comparison and mutation/rejection controls. Consume evaluator-owned holdouts read-only; do not modify `qa/holdout/`.
- [ ] W2 cross-verifies CPU/GPU proofs of the **new same-key fixture within Track B**. Old versus new circuit proof bytes are not required to match; equal intended statements and verifier decisions are required. Backend pointwise equality remains mandatory.
- [ ] Unmodified stock outer verifier accepts the new relation under its new VK, then actual ledger-9 local-twin acceptance demonstrates statement/template compatibility and checks all deferred obligations. A registry 0.8.3 success alone does not prove ledger-version interoperability.
- [ ] Measure both replacement request latency and complete old/new chain latency (children, parent, decider, setup, native verification). A3 k20 is removed, so report this as a replacement benchmark, not a faster unchanged k20 A3. Require >=3% full workload gain; any 10x claim needs measured <=0.1x full baseline.

**Dependencies:** W12 actual fixture inventory and baseline, concrete relation semantics; no need to wait for W15/W16 research. Deployment is a later owner-controlled operation; this plan performs none. Effort 2–4 w plus key generation/integration/deployment, low confidence.

### W18 — Retained-recursion circuit diet / lower k (LATER, alternative Track B)

**Scope:** if retaining general child-proof verification matters, inventory tables and optimize range limbs, constant fixed-base tables, scalar bounds/GLV coverage and provably redundant lookups as a single costed circuit design. Test table AND advice fit at k19. Smaller limbs can add advice; fewer rows without a k threshold do not automatically reduce FFT/MSM work. The degree-5→4 tweak alone is rejected.

**Files:** maintained/patched `midnight-circuits` sources `src/field/{foreign/params.rs,decomposition/pow2range.rs}`, `src/ecc/foreign/weierstrass_chip.rs`, `src/verifier/{msm.rs,verifier_gadget.rs}`, and Moriarty `mori-recursion` parent/decider architecture and fixtures. Use the repository's maintained circuit patch rather than editing the Cargo registry. Changes reaching Loki domain/evaluator/query shape consume W9/W10/W11 APIs after integration; do not mix circuit and arithmetic rewrites in one comparison.

**Contract:** CIR plus concrete bounded-integer carry/no-wrap/range lemmas, fixed-table correctness, scalar-decomposition equality and subgroup preservation; TERM/LED as applicable to outer decision. QN/NT/MS continue to apply to the new circuit's backend.

**Gates:**

- [ ] W12 actual table-by-table and gate/advice census, actual VK fixed-base counts and per-lookup costs. Do not substitute Fenrir's 63-base/1.03M table proxy or k12/k13 capacities as measured child rows.
- [ ] Demonstrate all rows/tables below 524,281 usable k19 rows (recompute actual blinding requirement), or explicit >=3% fixed-k whole-proof gain; otherwise DROP the proposed change. Check total columns, permutation sets and quotient-degree thresholds, not row count alone.
- [ ] Constraint/reference and mutation/holdout gates prove equivalent accepted statements and bounds. W2 CPU/GPU cross-verify on new keys, pointwise backend equality, and unmodified stock verifier acceptance under new VK.
- [ ] Full local ledger compatibility and actual carried-accumulator checks; new deployment/key-binding changes remain explicit. Measure old/new full A3-equivalent workload, including setup and verifier costs, at equal cache/thread policy. No old/new proof-byte equality requirement across VK changes.

**Dependencies:** W12 sizing and an explicit need to retain recursion rather than W17's fixture-specific fusion. Effort 2–4 w initial design/prototype; implementation continues only after a measured domain/gain case survives.

## Integration and stop gates

1. W9's hardening must survive merges into domain/evaluation/MSM, especially rotation width, shared GPU lock, SRS validation and error-to-CPU fallback. W10's profiler must count that fallback as failed GPU attribution, and its thread metadata must follow W9's actual opt-in policy.
2. W11 owns `src/poly/kzg/{mod.rs,utils.rs}` and multiopen arithmetic work. W11 HEAD is still `8eb0cee`, but during this audit its worktree gained an uncommitted `loki-multiopen` feature, `src/poly/kzg/multiopen.rs` and differential tests. It implements borrowed q groups, move-based stable set ordering, coefficient-chunk combinations and parallel independent sets; it explicitly leaves unused query evaluations and serial kate division intact. Its report records four differential tests with/without truncated challenges and synthetic measurement still queued. These are in-flight implementation/test reports, not measured A3 gain or merged code. Preserve first-polynomial-length semantics when combining padded f with shorter q; OPEN must cover it. W10 HEAD was `6b9ae6e` with additional local profiler/test changes, W9 `d9cc86e`, Loki main `8eb0cee`. Preserve all in-flight dirty files, including W11's harness feature forwarding, and re-read actual landed state before execution. Do not duplicate P1/K3 as a new package.
3. No package promotes on compile success or on a synthetic k16/k20 benchmark. Run W2, exact component differentials, actual A3 stock-verifier acceptance, then A3 wall measurement in that order. New Lean contracts are explicit work, and `scripts/drift.py` must review citations after merges; runtime tests are not a formal proof of CUDA kernels.
4. Route heavy execution through the factory's `scripts/doctorwatson heavy` measured caps/host runner, one heavy GPU job at a time. Preserve wallet/key/contract state; never remove another lane's lock. On failure read trace, record one-line hypothesis, retry with `--hypothesis`; after two same-kind failures shrink reproducer. Normal repository code packages follow applicable fmt/clippy/tests, predicate check, code/test commit and stop rule. This read-only planner neither executes that loop nor makes doc-only inner-loop commits.
5. Stop a performance package when the attributed remaining budget or paired A3 result is <3%. Do not rescue it by citing a larger kernel ratio, summing overlapping phase ceilings, changing core count, excluding setup only after optimization, or substituting amortized throughput for request latency.

**Priority summary:** (1) W12 trustworthy baseline/dispatch and W9 integration; (2) finish W10's existing quotient, the largest target; (3) W13 measured repeated-PK reuse; (4) W15 residency beyond the quotient, with W14 only where CPU work remains; (5) W16 resident products if their post-W15 budget survives. W17 is the strongest separate new-key replacement hypothesis, not an established speed-up of unchanged A3. W11 micro-optimizations and standalone sorts/MSM tuning do not get another package without a measured >=3% case.
