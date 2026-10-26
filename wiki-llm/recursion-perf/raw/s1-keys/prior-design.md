# prior-design — local source snapshot
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/LOKI-DESIGN.md
> Collected: 2026-10-26
> Published: Unknown

---
title: "Loki: a recursion-oriented, GPU-accelerated Midnight prover (design)"
project: loki
date: 2026-10-24
sources: "nodes/loki-01 … loki-10 (GPT-6.1 Sol xhigh, Scrapling captures under raw/NN/)"
tags: [loki, prover, gpu, recursion, sherlock, doctorwatson]
---

# Loki: design synthesis

> **Update 2026-10-24:** the 10x exploration ([10x/LOKI-10X.md](10x/LOKI-10X.md)) supersedes the speed-up estimates here. Its strongest route is a new-key fused ~k15 terminal.
>
> **Update 2026-10-25:** the work order now comes from the measured [Speed-up backlog (from the scouts)](#speed-up-backlog-from-the-scouts) at the end of this file. It covers every item from the eight scout reports in [speedups/](speedups/), with Amdahl gains measured against the real k20 A3 parent. The main finding: about 40% of the A3 prove+verify window is Moriarty harness work outside `create_proof`, so the GPU cannot reach it. The table in §"Design" below is the original plan; where the two disagree, the backlog wins.

This design combines the ten research nodes. Each claim carries its node number, and the node holds the code references, source captures and arithmetic behind it. Every speed-up below is an **ESTIMATE until measured** on this host.

## The hard constraint
The stagenet contract already has its verifying key. Loki therefore changes **how fast proofs are made, not what a proof is**:
- the same `midnight-proofs` 0.8.3 protocol (the version Moriarty pins; the `midnight-zk` checkout's LogUp/PCS interface differs) [01, 10];
- the same SRS, VK, transcript and proof encoding.

Any idea that requires new keys goes to Track B (§5).

## What is actually slow [01, 05, 06, 08, 09]
- **No single bottleneck.** The cost is spread over witness synthesis, the advice-commitment MSMs, the permutation and lookup grand products, the extended-domain quotient (`compute_nu_poly`/`Evaluator::evaluate_numerator`), the quotient commitments and the KZG multiopen [01].
- **MSM already runs on CPU BLST Pippenger** (`KZGCommitmentScheme::commit`/`commit_lagrange` → `msm_specific`) [02, 09]. FFT and quotient evaluation use Rayon. Sorted lookups, scans, synthesis and several orchestration loops are serial [01].
- **Memory:** at k20 each polynomial is 32 MiB, and the retained ordinary plus extended polynomials explain the ~12–15 GiB peak [01, 03].
- **Keys are rebuilt on every proof.** The Sherlock parent calls `setup_pk` each time and then drops the result (`projectfenrir recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1041`). That is the ~120 s k20 keygen, repeated [08].
- **The recursion shape is already good.** Children use Poseidon and the parent defers pairings to the decider (the accumulator comes from `VerifierGadget::prepare`). Switching to "defer pairings" or Poseidon gains nothing here [05, 06].
- **The circuit cost comes from foreign-field elliptic-curve arithmetic.** BLS G1 coordinates are 7×56-bit limbs inside the scalar circuit, plus the lookup tables. A recorded two-child node has 574k advice rows and 1.03M table rows [06]. BLS12-381 with Jubjub is not a recursion cycle [05, 07].

## Design (Track A, same keys; in order of return per effort)

| Step | What | Hook | Node | Estimated gain |
|---|---|---|---|---|
| **L0 Measure** | Time each phase inside `create_proof` (wall time, RSS, VRAM) behind a `loki-profile` feature. Make the uncapped k20 parent run the factory's F8 budget | `prover.rs` phase boundaries | 01 | makes everything else falsifiable |
| **L1 Key and SRS cache** | Serialize the PK/VK/SRS once, memory-map them, and reuse them across the whole recursion tree; stop calling `setup_pk` for each proof | `sherlock_link.rs` and recl9 leaf/node/decider; `doctorwatson heavy` holds the cache | 08, 09 | **≈1.07× measured** (62.3 s setup vs 882.6 s prove; corrected by [[LOKI-10X]]) |
| **L2 CPU quick wins** | Lazy and batch inversion, sparse denominators, reusable buffers, no double execution of region closures, one CPU budget shared by the Rayon and BLST pools | `prover.rs:520`, `single_pass.rs:89`, `multicore.rs` | 09 | 1.11–1.40× |
| **L3 GPU MSM** | `ProverBackend` trait (CPU by default; a `cuda` cargo feature). SRS bases stay resident in VRAM with fixed-base precomputation, and the columns of each transcript phase are batched. Output is ordinary G1 commitments. **sppark** (Apache-2.0) goes first and **ICICLE v4** is benchmarked against it (its GPU backend is separately licensed) | `KZGCommitmentScheme::commit*` → `msm_specific` | 02, 04, 10 | ~11–21 ms per warm 2^20 MSM; MSM-only ≈1.6× end to end |
| **L4 GPU quotient, resident** | Column extension (coset NTT with exact `ZETA`/stride), custom gates, permutation, lookups, vanishing division and inverse NTT all stay on the device. An FFT-only adapter serves as the differential prototype | `compute_nu_poly`, `evaluate_numerator`, `domain.rs` | 03 | together with L3, ≈3× and more; Halo2 precedents report 3.8–25× |
| **L5 Tree scheduler** | One persistent Rust scheduler inside one `heavy` invocation, owning one CUDA context and the shared keys. CPU synthesis, GPU arithmetic and verification overlap. It admits 1 heavy GPU proof plus 1 small proof, and 2 heavy proofs only after measuring their combined VRAM. Partial proofs are checkpointed | new `loki-sched` crate, driven by DoctorWatson | 08 | 16-transaction batch (4 leaves → 4-child node → decider): ~370–640 s cold, 260–520 s between batches |

**VRAM budget:** at n=2^20 and N=2^22, 64 extended columns take 8 GiB, and a worked total comes to ~19 GiB. N=2^23 takes 28 GiB, close to the 32 GB card. Measure before admitting two heavy proofs [03].

**Precedents to borrow from** [04]:
- Midnight's own GPU proof of concept, midnight-zk PR #157.
- Nocy's RTX 5090 proof server. Neither has been tested with Sherlock recursion.
- Axiom and ICICLE-halo2 for the architecture.
- Kernels written for BN254 or small fields need porting, because Midnight's scalar field is BLS12-381 `Fq`.

## Correctness is the gate, not the speed [10]
Promotion goes through factory predicates, cheapest first:
1. **Arithmetic differentials:** GPU against CPU for MSM, NTT and quotient on random and edge inputs (zero, identity, maximum scalar, non-canonical encodings).
2. **Byte-identical proofs** in controlled fixtures with fixed randomness and threads.
3. **CPU-verifier acceptance** of GPU proofs under fresh production randomness.
4. **The recursion accumulator** matches CPU at every tree level.
5. **The full k20 chain → decider** passes, and the decider is accepted by the real ledger-9 verifier on the local twin.
6. **Stagenet**, with effects read back.

The witness-mutation soundness harness (`soundness_mutation`) runs against the GPU backend too.

GPU-specific failure modes to test for: incorrect modular reduction, races, unchecked overflow, and wrong coset or rotation constants.

## Track B: only if new keys are allowed [05, 06, 07]
- **Circuit diet:** inventory the real Sherlock parent first. Then try smaller lookup limbs, keeping fixed-base VK points out of the variable-base MSM, and scalar-bound-aware MSM scheduling. Reaching k18 needs ~75–80% savings in the dominant EC component, which nobody has shown yet; k19 is plausible.
- **Shape:** verifier-free fused leaves and bounded fan-in (4).
- **Folding** (Nova/CycleFold strands → one checked wrapper per strand → the outer Midnight tree) only if its *complete* cost beats the accelerated tree. No surveyed folding scheme fits today's VK.

## Proposed factory predicates (to add to `qa/sherlock/predicates.yaml`, set `L`)
- **L1:** phase profile recorded for the k20 parent.
- **L2:** PK/VK cache gives a byte-identical proof, and keygen runs only once across a chain.
- **L3:** GPU MSM is bit-identical to BLST on 10⁴ randomized and edge cases.
- **L4:** GPU quotient is identical to CPU.
- **L5:** GPU proof is byte-identical under fixed randomness.
- **L6:** CPU verifier accepts the GPU proofs.
- **L7:** the GPU-built k20 chain → decider is accepted by the local ledger-9 node.
- **L8:** the 16-transaction batch has a measured end-to-end time under the CPU baseline.

## Open questions
- Measured phase shares on this host. L0 decides the order of L3 and L4.
- How well CUDA works under WSL2 on this card (driver 616.92, CUDA 13.4) [02, 10].
- Whether sppark is current on Blackwell, and ICICLE v4's licensing terms.
- Whether upstream Midnight wants a backend trait. PR #157 is the place to ask.

## Speed-up backlog (from the scouts)

Written 2026-10-25 by the integrator. Sources are the eight scout reports in [speedups/](speedups/), referred to below as
**QE** = quotient-evaluation, **LP** = lookups-permutation, **GS** = gpu-system, **WS** = witness-synthesis,
**KZ** = kzg-multiopen, **PK** = other-chains-plonk-kzg, **RS** = recursion-structure and **OR** = other-chains-recursion.
`QE#1` means rank 1 of that report. Lean contracts are listed per package in
[speedups/LEAN-OBLIGATIONS.md](speedups/LEAN-OBLIGATIONS.md).

### The measurement basis, and why the GPU did not move A3

- **Basis.** W10's exploratory CPU profile of the real k20 Sherlock parent (A3):
  `mori-recursion` `sherlock_parent` in `Moriarty-loki` `6ee918696`, Loki `8eb0cee` plus the W10 profiler, CPU features,
  `RAYON_NUM_THREADS=4`, BLST `no-threads`, on this host (Core Ultra 9 285K, 20 cores, RTX 5090). The run overlapped
  another job, so its prove+verify window was **292.7 s** (the clean reference runs are 220.2 s stock, 252.6 s Loki CPU and
  220.8 s Loki GPU). All shares below are against 292.7 s, with gains given in seconds as well so they can be re-based.
  Fresh sequential H0 runs with GPU counters (W10, pending) replace this basis when they land
  (`loki-wt/w10-gpu-quotient/LOKI-A3-PROFILE.md`).
- **`create_proof` is only 175.4 s (60%) of the window.** The remaining **117.3 s (40%)** is Moriarty's `prove_link_parent`
  (`crates/mori-recursion/src/parent.rs:974`), which runs before and after the prover:
  - `cost_model(rel, None)` is a degree guard. Through `MidnightCircuit::from_relation(rel, None)` it runs `optimal_k`, and so a
    full `circuit_model` layout pass for every k from 9 to 20. The identical call takes **78.4 s** on its own in the same run
    (`model_seconds`, measured outside the timer). Inside the window this is inferred, not yet timed;
  - `setup_pk(rel, vk)` rebuilds the proving key on every proof. Not timed separately; `setup_vk` alone took 41.4 s;
  - two child off-circuit verifications, the accumulator pairing check, and two verifications of the parent proof (small).
- **This is why GPU MSM at 13–16× did not move A3.** The GPU can only act inside `create_proof`, and inside it the largest
  phase (the quotient, 72.9 s) is still on the CPU. Also, the MSM-bearing phases add up to about 59 s on the CPU, yet the
  unattributed GPU-feature run cut `create_proof` by only 13 s (175.4 → 162.4 s), and `lookup_prepare` went *up*
  (25.0 → 52.4 s). Either the overlap contaminated that run, or part of the MSM work fell back or thrashed the two-slot SRS
  cache. W10's H0 counters decide which.
- **Where the MSMs are** (inferred: `lookup_products` holds 5 MSMs and fell 11.3 → 0.75 s on GPU, so about 2.1 s per
  2^20 CPU MSM in this configuration). `lookup_prepare` holds 10 (≈21 of its 25 s), `permutation_products` 6 (≈12.6 of 13.3 s),
  `quotient_commitment` 3–4 (≈7.3 s), `kzg_multiopen` 2 (≈4.7 s) and `advice_commitments` 15 (sparse, 2.8 s). That matches
  W10's count of 45 MSM entries per proof (44 of length 2^20, one of 2^20−1). So the CPU-side remainder of the lookup and
  permutation phases is only about 5 s; the sort, HashMap and inversion work the scouts ranked is not where the time is.

Exclusive phases of the basis run (s): quotient_evaluation 72.9 · lookup_prepare 25.0 · compute_nu_poly 14.1 ·
permutation_products 13.3 · kzg_multiopen 11.5 · lookup_products 11.3 · quotient_commitment 7.6 · witness_synthesis 7.3 ·
other 4.5 · advice_commitments 2.8 · trash 2.8 · evaluations 1.3 · quotient_construct 0.9 · outside `create_proof` 117.3.

### Backlog table

Gain = seconds saved on the 292.7 s basis = t·(1 − 1/s) for the phase time t and an assumed stage speed-up s, then
percent of the window and the Amdahl factor. **All gains are ESTIMATES until measured end to end on A3** (lesson W3).
Effort is in engineer-days. Status: **in W9/W10/W11** = in flight; **W12** = briefed, runnable now; **W13–W17** = queued
behind a merge; **later** = not scheduled; **dropped** = not worth doing for A3, with the reason.

| # | Item | Source | Track | Target phase (A3 s) | Expected A3 gain | Effort | Lean contract | Status |
|---|---|---|---|---|---|---|---|---|
| 1 | Drop the repeated `cost_model(rel, None)` guard inside `prove_link_parent`; thread the plan's `CircuitModel` in and keep a wrong-k rejection | integrator (W10 profile) | A (no prover change) | outside create_proof: 78.4 (inferred) | **−78 s, 26.8%, 1.37×** | 1 | O12.2 (Rust gate); `loki_proof_eq_stock` untouched | **W12** |
| 2 | Proving-key cache: build/serialize the PK once, bound to VK+relation digest+crate commit; reuse across proofs of the same key | RS#1, design L1, integrator | A | outside create_proof: setup_pk ≈30–39 (unmeasured) | −30–39 s (10–13%) **from the second proof on**; 0 for a single cold A3 run unless the cache is warm | 2–3 | O12.1 `pk-cache-equal` on open `keygen-deterministic` | **W12** |
| 3 | Per-step timers in `prove_link_parent` plus a parent inventory (`cs.degree()`, `extended_k`, permutation sets, per-lookup table rows, calc counts per graph) | WS#0, QE open questions, RS §6, LP | — (measurement) | whole window | decides 1, 2, 22, 33–35 | 0.5 | none | **W12** |
| 4 | Restore stock BLST threading by default (`loki-cpu-budget` opt-in); thread-count sweep | GS#1, lesson | A | all MSM phases | Loki CPU 252.6 → ~220 s (−12.8%) | done in W9 | `Msm.chunkedMsm_eq` | **in W9** |
| 5 | GPU quotient: row-tiled interpreter, resident extended columns, PK cosets uploaded once per proof | QE#3, QE#4, GS#5 (quotient part) | A | quotient_evaluation 72.9 (+ compute_nu_poly 14.1) | s=5–15: −58–68 s (20–23%, 1.25–1.30×), up to ≈−76 s with resident extension | 15–25 | `numeratorAt`, `EvaluatesNumerator`, `quotient_contract`, `hornerY_swap`, `rotate_extended`, `Refines.quotientEval` | **in W10** |
| 6 | H0: prove the GPU ran (dispatch counters, fallback attribution) | GS H0 | — | all MSM/NTT phases | decides 7 | done in W10 | none | **in W10** |
| 7 | Make GPU MSM real end to end: per-call counters (hit/miss, host ms, mutex wait), SRS identity key replacing the O(n) `same_coordinates` scan, no third-slot thrash for 2^20−1, parallel scalar conversion; pinned reusable staging only if measured | GS#2, KZ H4, GS §G H2/H3 | A | MSM-bearing phases ≈59 CPU; unrealised gap ≈40 | up to −40 s (14%) if H0 shows thrash or host overhead; ≈−1 s if not | 3–5 | `msmDispatch_meets`, `lokiMsm_meets`, `convertScalar_hostToDevice`; new O13.1 `residentCache_sound` | **W13** (after the W9 merge, which owns `backend.rs`, and the H0 profile) |
| 8 | Multiopen: borrowed q-sets, parallel per-set divisions, chunked order-preserving combinations | KZ#3, PK#1 (axiom #37) | A | kzg_multiopen 11.5 (≈6.8 non-MSM) | −3–4.5 s (1–1.5%) | 3 | Multiopen.lean (being written) | **in W11** |
| 9 | Port #565 `divide_by_roots` (one-pass division per set) | KZ#1 | A | kzg_multiopen | within 8; ≤−2 s | 1–2 | new O11.1 `divideByRoots_eq_kateFold` | **W11 extension** (proposed to the coordinator) |
| 10 | Port #554's prover side: zero evals in `construct_intermediate_sets` (the prover evaluates every query polynomial and never reads it), borrowed `q_polys` | KZ#2, KZ H1 | A | kzg_multiopen / evaluations-sized work, ≈1 | −0.5–1 s | 1 | new O11.2 `prover_evals_unused` | **W11 extension** (needs `kzg/utils.rs`) |
| 11 | Chunked parallel `kate_division` (linear-recurrence scan) | KZ#4 | A | kzg_multiopen | ≈−0.15 s at 4 threads | 2 | Rust differential vs the serial kernel | later (W11 keeps the serial kernel) |
| 12 | CPU fused numerator loop (one row loop over gates, permutations, lookups, trash; per-row Horner order kept) | QE#1 | A | quotient_evaluation 72.9 | CPU only: s=1.2–1.8 → −12–32 s (4–11%); ≈0 on a GPU build once W10 lands | 2–4 | new O14.1 `fused_numerator_eq` | **W14** (after the W9 merge; W10 keeps `evaluation/cuda.rs` and the dispatch) |
| 13 | One shared CSE graph across gates, lookups and trashcans; no `Store` intermediates | QE#2 | A | quotient_evaluation | CPU −5–15 s (2–5%); also shrinks W10's instruction list | 1–3 | new O15.1 `sharedGraph_eval_eq` | **W15** (after W10's compile step; it changes what W10 consumes) |
| 14 | Lookup preparation across lookups in parallel with in-order blinding and transcript writes; drop the `values.clone()` before `lagrange_to_coeff` | integrator, LP | A | non-MSM part of lookup_prepare ≈4 | −2 s (<1%) | 2 | new O16.1 `lookupPrepare_batch_eq` | **W16**, only if H0 shows ≥5 s of non-MSM work |
| 15 | GPU grand products (device batch inversion and prefix scan, resident with the quotient) | LP A1, GS#5 | A | non-MSM part of lookup/permutation products ≈1.4 | <−1.4 s | 10+ | `product_column_eq`, `chunkedScan_eq` | later (only inside W10-style residency) |
| 16 | GPU batch inversion alone (ICICLE VecOps style) | LP A3 | A | as 15 | ≪1 s | 5 | `batchInvertParallel_eq` | dropped: the inversion is cheap and already parallel |
| 17 | GPU sort of permuted input | LP A2 | A | sort ≈0.1 s per lookup | ≈−0.5 s | 10 | `sorted_perm_eq_insertionSort` | dropped: sub-percent for high effort |
| 18 | Sorted-leftover HashMap placement plus parallel stable sort | LP A4 | A | lookup_prepare | already realised | — | `loki_lookup_valid`, `loki_lookup_order_free` | done (Loki main) |
| 19 | Lookup coset reuse and early release of gate-only cosets | QE lever 6–7 | A | quotient_evaluation | small; mostly memory | 1 | as 5 | folded into W10 |
| 20 | Async MSM handoff overlapping CPU phases | GS#4 | A | bounded by total GPU MSM time 45×58 ms ≈2.6 | ≤−2.6 s (<1%) | 5–10 | `SameOrder`, Quint `scheduler.qnt` | later: GPU MSM is no longer the long pole |
| 21 | Fixed-base SRS precomputation on the device | GS#3, KZ (fixed-base) | A | as 20 | <−1.5 s; costs ≈1.5 GB VRAM per 2^20 bases | 5–10 | `msmDispatch_meets` | dropped for A3: VRAM is needed for the resident quotient |
| 22 | Pinned reusable host staging | GS#2 | A | ≈45×2.7 ms of transfers | ≈−0.2 s | 1 | none | folded into W13 only if trivial |
| 23 | Region-parallel `SingleChipLayouter::assign_regions` (Scroll `parallel_syn`) | WS#1 | A | witness_synthesis 7.3 | s=2–3: −3.6–4.9 s (1.2–1.7%) | 5–10 | new obligation on rng and transcript order (not yet written) | later: high byte-order risk for <2% |
| 24 | Cache the throwaway `configure` per PK | WS#3 | A | inside witness_synthesis/other | <−0.5 s | 0.5 | Rust gate (same `ConstraintSystem`) | later (in `prover.rs`, which W10 owns) |
| 25 | No per-column `Rational` allocation for single-phase circuits | WS#4 | A | memory (≈1.1 GB transient) | 0 s | 1–2 | `rational_compact_eq` | later (memory lever) |
| 26 | Vartime inversions in synthesis (zcash #962) | PK#3 | A | witness_synthesis | <−0.5 s | 1 | none | later |
| 27 | Small-MSM scheduling (zcash #796) | PK#2 | A | MSMs below 16,384 | 0: A3 has no MSM below 2^20−1 | 3 | `chunkedMsm_eq` | dropped for A3 |
| 28 | Do not store fixed extended cosets in the PK | PK#4 | A | memory | +24% slower in axiom #17 | 1 | as 5 | dropped for speed; a memory lever only if two heavy proofs must share the host |
| 29 | Direct instance commits (zcash #938) | PK#5 | A | 28 instance lanes | ≈0 | 1 | none | dropped |
| 30 | Loki-side `circuit_model` and `keygen_pk` speed (parallel or GPU fixed cosets, permutation pk) | integrator (78 s model, 41 s setup_vk) | A | keygen plan (outside the A3 timer once 1 lands) | 0 on A3 after 1–2; helps every keygen and redeploy | 3–5 | new O17.1/O17.2 | **W17** (after the W9 merge, which owns `keygen.rs`) |
| 31 | Device-resident multiopen polynomial work | GS#5, KZ#5 | A | kzg_multiopen | <−3 s | 10+ | Multiopen.lean | later |
| 32 | Lower the quotient degree d (5 → 4 cuts one h piece, adds permutation sets) | QE#5 | B | quotient_commitment 7.6, quotient | ±; needs d and permutation-set counts from the inventory (#3) | 5–15 | new relation (not a refinement) | later (after #3) |
| 33 | Cut the lookup table to reach k19 (smaller range limbs, fewer tags) | RS#3, LP B2, OR R6 | B | whole prover (halves n) | 2–4.4× on domain work (ESTIMATE); advice rows grow ≈20% | 10–20 | new relation | later: decided by the per-table inventory (#3) |
| 34 | Fused ~k15 terminal replacing parent+decider | RS#2, OR R2, LOKI-10X | B | whole chain | ≈10× chain latency (ESTIMATE, medium confidence) | 10–20 + deployment | new relation and statement equivalence to SPEC | later: the largest lever, but a Moriarty design decision; redeploys are authorised while the design is unfrozen |
| 35 | Fewer lookups or point sets (remove redundant checks) | LP B3, OR R9 | B | lookup phases, multiopen | ≈3 points and 5 evaluations per removed lookup | 5–10 | new relation | later (after #3) |
| 36 | Defer the final pairing to the native ledger verifier | RS#4 | B | decider rows (≈38%) | decider only | 10 | ledger trust statement | later: first confirm ledger-9 checks an exposed accumulator |
| 37 | In-circuit fixed-base tables as constants (≈945 foreign adds per parent proof) | WS#2 | B | witness_synthesis and rows | unmeasured | 10 | new relation | later |
| 38 | Fan-in 3–4 per layer | OR R3 | B | chain | does not lower k on its own (the table pins k20) | 5–10 | new relation | later |
| 39 | GLV scalar-split audit | OR R5 | B | in-circuit MSM | small (GLV already present) | 5 | none | later |
| 40 | Defer non-native EC ops (ePrint 2024/1651) | OR R1 | B | chain | 10–50× gates (claim) | 30–60 | new relation + decider | later |
| 41 | Lazy reduction and CRT tricks | OR R8 | B | foreign-field chips | unknown | ? | new relation | later |
| 42 | LogUp instead of permuted lookups | LP B1 | B | lookup phases | small for the prover; removes one commitment | 20+ | new protocol and verifier | dropped for now: the pinned ledger verifier has no LogUp |
| 43 | fflonk (midnight-zk #565/#575) | KZ (Track B), RS#5 | B | — | prover 1.29× **slower** | 10 | — | dropped: a VK/proof-size lever, not speed |
| 44 | SHPLONK → GWC | KZ (Track B) | B | multiopen | no evidence of a prover gain | 10 | — | dropped |
| 45 | UPA-style outer wrapper; statement batching (Varuna); cycle-curve ECCVM split | OR R10, R11, R4 | B | — | no number / no BLS12-381 cycle | — | — | dropped |
| 46 | Multi-proof scheduler with one persistent CUDA context (L5) | GS §D, design L5 | A | throughput, not A3 latency | 0 on A3 | 10+ | Quint `scheduler.qnt`, fixed thread count | later (throughput, after single-proof latency) |
| 47 | NUMA tuning; P/E-core affinity | GS §F | A | — | 0: one NUMA node in WSL | — | — | dropped (affinity sweep stays in W9's thread study) |
| 48 | sppark upgrade or replacement for Blackwell | GS §E | A | MSM/NTT | none: sm_120 builds and runs | 3–10 | `msmDispatch_meets` | dropped unless a defect appears |
| 49 | Adopt the Nocy native GPU prover or another wholesale GPU prover | GS §A | A? | all of create_proof | unknown; Nocy's MSM is a 1-thread naive launch and its FFT copies to the host | 20+ | the whole `Refines` | dropped |
| 50 | CPU row-chunked evaluation that avoids the full extended domain | QE lever 6 | A | quotient | none: the h inverse NTT needs the full N | — | — | dropped (chunking only bounds memory; that is used inside W10) |
| 51 | Scroll #27 partitioned coset FFT; Axiom #28 work-stealing `evaluate_h` | PK (quotient-side, not recommended) | A | quotient | superseded by W10 / W14 | — | as 5 | folded into W10/W14 |
| 52 | cuZK as a quotient source | QE | A | — | none: Groth16, no quotient kernel | — | — | dropped |
| 53 | Merge the f and pi MSMs | KZ §3 | A | multiopen | impossible: pi depends on x3/x4, which come after f | — | `SameOrder` | dropped |
| 54 | Lagrange SRS for openings; verifier-side `eval_interpolant` | KZ §3, KZ#1 | A | verifier / SRS memory | 0 on prover time | — | — | dropped for speed (the verifier is not on the critical path) |
| 55 | Remove the region shape pass; outer Rayon over the 15 advice commitments | WS §7, WS (demoted) | A | witness_synthesis, advice_commitments 2.8 | unsafe (stock semantics) / contends with inner Rayon | — | — | dropped |
| 56 | Move eager witness work into `Value` suppliers; cache parsed child proofs and transcript values across levels; cross-circuit or virtual-region (halo2-base) parallel synthesis | WS §6–7 | A/circuit | witness_synthesis 7.3 | <1–2% for one parent | 3–20 | circuit semantics | later (circuit-side, small for one parent) |
| 57 | Poseidon2 transcript hash | WS §5 | B | in-circuit transcript | changes the transcript | 10–20 | new transcript | dropped (breaks Track A; no evidence of a gain) |
| 58 | Folding (Nova/CycleFold/Sangria/ProtoGalaxy/HyperNova); snark-verifier or UniPlonK aggregation; Pickles/IPA accumulation; STARK-to-SNARK wraps (SP1, RISC Zero, zkSync, plonky2) | RS §4.6–4.7, OR | B | whole chain | no complete cost that beats the accelerated tree; no BLS12-381 cycle | months | new protocol | later at best (LOKI-10X keeps folding as a hypothesis only) |
| 59 | Other-chain kernels reviewed and not applicable: CycloneMSM batch-affine, signed Booth recoding, prescaled fixed bases (c-kzg #628), skipping SRS subgroup checks (c-kzg #299), Lagrange wire commits (already in Loki), Gemini folding, Goblin/UltraHonk, vendor claims (Snarkify, ICICLE), ZPrize/OPTIMSM FPGA, load-balanced GPU MSM, plonky2/3, boojum, kimchi, supraseal, Varuna; Telepathy calibration; Electron Labs | PK "not recommended", OR R7, R12 | — | — | none for BLS12-381 KZG on this host | — | — | dropped (reasons in the scout reports) |

### Roadmap (ordered)

1. **H0 sequential A3 profile** (W10, in flight). This is the denominator for everything else. W10 must pin its Moriarty-loki
   commit for its before/after pair, because W12 changes the same test's window.
2. **W12 a3-residual** (now, Moriarty side). Per-step timers, drop the repeated cost-model guard (−78 s), PK cache for
   repeated proofs, parent inventory. On the clean 220.8 s GPU run this alone should bring A3 to roughly 145 s (ESTIMATE).
3. **Merge W9.** Stock BLST threading by default, total NTT fallback, one GPU guard, SRS validation.
4. **W10 GPU quotient** (in flight). Next largest: −58–68 s.
5. **W13 msm-hostpath** after the W9 merge and H0. It recovers whatever part of the ≈59 s of CPU MSM work the GPU path
   does not yet realise.
6. **W11 multiopen** (in flight, with the #565/#554 extension).
7. **W14 CPU fused numerator** (after the W9 merge), then **W15 shared CSE** (after W10's compiler). These serve CPU-only
   hosts and the CPU oracle, and shrink W10's program.
8. **W16 / W17** only if their trigger holds (≥5 s of non-MSM lookup work in H0; keys rebuilt in production).
9. **Track B decision** from W12's inventory: the fused k15 terminal (#34) against the k19 table cut (#33). This is the
   only route on record to a large end-to-end factor; Track A after steps 2–5 is bounded at roughly 3× on A3 (ESTIMATE:
   220.8 → ≈70–90 s if every step realises its upper estimate).

Briefs: `loki-run/loki-w12.md` (runnable). W13–W17 are sequenced; their file sets and triggers are listed in
[speedups/LEAN-OBLIGATIONS.md](speedups/LEAN-OBLIGATIONS.md#file-sets-of-the-sequenced-packages-for-briefing-when-the-predecessor-merges), so they can be briefed the moment their predecessor merges.

### Reconciliation with the independent GPT plan ([speedups/PLAN-gpt.md](speedups/PLAN-gpt.md))

Package numbers here are this backlog's; PLAN-gpt.md numbers its own W12–W18 differently (its W13 is PK reuse, its W15 residency).
The two plans were written independently. They **agree** on: measure the real A3 first and split the 117 s residual; W10's
quotient is the largest prover target; reuse the PK only with full identity binding (VK, relation including the child VKs,
SRS, k) and with every per-proof check kept; LogUp, fflonk, GWC and fixed-base precompute are not speed work for A3; the
sort/inversion GPU kernels and standalone multiopen micro-work do not earn packages; the fused terminal and the lower-k
circuit diet are separate Track B projects that need the parent inventory first. The GPT point that `loki-harness` has
`cuda = []`, so `--features cuda` alone never exercises the GPU in W2, is correct and important. It is already task 7 of W10's
plan; keep it there, and make every GPU gate assert dispatch counts.

Where they **disagree**, and why:

1. **The 117 s residual is mostly identified, not unattributed.** GPT treats it as an unknown mix and makes PK reuse a
   conditional W13. The A3 log shows `cost_model(rel, None)` (an `optimal_k` sweep over k=9..20) costs 78.4 s, and
   `prove_link_parent` calls it again as a degree guard inside the window. This is the single largest Track A item (−78 s,
   1.37×). Removing the *recomputation* is not removing the check: the guard keeps its meaning through the memoised
   `CircuitModel` and a wrong-k rejection test. That is the "relation-bound contract" GPT asks for.
2. **W12 scope and ownership.** GPT's W12 is a broad measurement harness that edits W10's `profile.rs`, examples and verifier
   adapter, the W2 harness, and `Moriarty-loki` directly. W10 owns those files and measures A3 on `Moriarty-loki`, so that W12
   cannot start until W10 hands off. This backlog's W12 is a separate Moriarty worktree (timers, the guard, PK reuse,
   inventory) with no Loki files, so it starts now. W10 already does the H0 attribution and the harness forwarding.
3. **Normalisation.** GPT scales phase shares to a 220–250 s total. That assumes the clean runs have the same 117 s residual
   as the overlapped run, which inflates `create_proof`'s share (quotient 29–33% vs 24.9%). This backlog uses one consistent basis
   (the 292.7 s run) and gives seconds. Both are re-based when H0 lands.
4. **GPU MSM host path.** GPT folds the SRS-cache and coordinate-scan fixes into a large W15 residency package (1–3 weeks).
   This backlog keeps them as a small W13, triggered only if H0 confirms the ≈40 s gap between ≈59 s of CPU MSM work and the
   13 s actually saved. If the gap is real, it is the cheaper fix. If not, W13 is dropped, consistent with GPT's 3% rule.
5. **Residency of advice/instance extension (GPT W15 first slice).** W10's own plan already transforms advice and instance on the
   device and keeps them resident through the numerator. A separate W15 for that would duplicate W10. Only the multiopen
   tail (row 31) remains beyond W10, and it is below 3%.
6. **W14 split.** GPT keeps fusion and CSE together after W9 and W10. This backlog splits them: fusion can follow the W9 merge,
   because it rewrites only `evaluate_numerator`'s loop while W10 adds `evaluation/cuda.rs`; CSE changes what W10's compiler
   consumes, so it waits for W10. Both are CPU-fallback work and must meet GPT's ≥3% bar against the matching baseline.
7. **3% admission.** Adopted, with one exception: the #565/#554 multiopen ports (rows 9–10) are cheap, sit in files W11 already
   owns, and are proposed as a separately verified W11 follow-up rather than a new package. W11 agreed to that, subject to the
   coordinator. GPT drops them.
8. **GPU grand products (GPT W16).** Kept by GPT conditionally. Here the evidence already says no: the product phases are almost all
   MSM (5 + 6 commitments), leaving about 1.4 s of arithmetic. Marked later.
