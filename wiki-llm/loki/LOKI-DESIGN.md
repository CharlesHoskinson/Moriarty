---
title: "Loki: a recursion-oriented, GPU-accelerated Midnight prover (design)"
project: loki
date: 2026-10-24
sources: "nodes/loki-01 … loki-10 (GPT-6.1 Sol xhigh, Scrapling captures under raw/NN/)"
tags: [loki, prover, gpu, recursion, sherlock, doctorwatson]
---

# Loki: design synthesis

> **Update 2026-10-24:** the 10x exploration ([10x/LOKI-10X.md](10x/LOKI-10X.md)) supersedes the speed-up estimates here. Its strongest route is a new-key fused ~k15 terminal.

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
