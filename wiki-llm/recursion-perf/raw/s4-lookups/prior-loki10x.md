# Earlier Loki 10x verdict
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/10x/LOKI-10X.md
> Collected: 2026-10-26
> Published: Unknown

---
title: "Loki 10x: verdict"
project: loki-10x
date: 2026-10-24
sources: "astra-1/2/3 PROPOSAL.md (GPT-6 Astra high), 10 Haiku helpers (helpers/haiku-NN/), coordinated over AgentBus run loki10x"
---

# Can Loki make Midnight proofs 10x faster?

**Short answer:**
- **Yes for the Sherlock chain, by changing the circuit rather than only accelerating it.** This needs new keys and a redeployed verifier contract.
- **Possibly for a single proof with the same keys**, but only with a full GPU-resident prover *and* a faster witness builder.
- **Not by caching or a GPU MSM alone.**

Nothing here has been measured on the 5090 yet. Every figure is arithmetic on recorded CPU runs and published benchmarks.

## Baselines found (recorded, CPU)
- **Sherlock k20 parent:** 62.3 s proving-key setup and 882.6 s proving. Children A102 and A80 are only **k13 and k12** [astra-2, astra-3, haiku-09].
- **Fenrir recl9 16-transaction chain:** leaves 163.6 s, node 867.3 s, decider 586.4 s, **1,617.3 s total**. That is 2.23 batches per hour, and the box sits partly idle: the node averages 268% CPU [haiku-09].
- **Correction to LOKI-DESIGN L1.** Proving-key caching saves only the setup share, **≈1.07×**. That is a long way short of "120 s per proof" [astra-3].

## The three answers

| Target | Verdict | Best route | Ceiling and conditions |
|---|---|---|---|
| **(a) one k20 proof, same keys** | 10× credible, unproven (low–medium confidence) | GPU-resident everything (commitments, grand products, quotient, multiopen) **plus** witness synthesis about 4–5× faster | astra-1's conditional budget is 10.64×; it is 6.5× if the proving key is still rebuilt each time. The cuSnark evidence (a wide k20 circuit) caps at 4.85× when witness and lookup preparation stay unchanged. The work kept on the CPU must be ≤ ~4–9% of the time |
| **(b) chain latency** | **10× plausible (medium confidence)** | **New keys: a fused terminal.** Prove the A102→A80 transitions directly in one ~**k15** relation, instead of verifying k13/k12 proofs inside a k20 parent and then a decider. k15 has 32× fewer rows than k20 | Accelerating only the parent caps the chain at **2.16×**, because the leaves and decider dominate. The fused terminal keeps the outer PLONK/KZG/BLS12-381 protocol, the Blake2b transcript and the official SRS, so the ledger-9 verifier still accepts it. Cost: **2–4 engineer-weeks** plus a new verifier deployment |
| **(c) 16-tx batches per hour** | 10× plausible, unproven (low confidence) | 4–8 CPU witness workers feeding one GPU in one CUDA context, with prepared keys kept resident and memory checked before a job is admitted | The queue model gives 10.45×, but that drops to **7.4–9.7×** at measured core counts and to 3.8–4.6× if the GPU's bulk speed-up is only 3× [haiku-08]. Memory is tight: two heavy proofs peak at ~30.7 GB |

## The plan (the order the leads agree on)
1. **Measure first.**
   - Profile one k20 parent proof by phase: synthesis, MSM, quotient, lookups, multiopen.
   - Use the **same `midnight-proofs` version** for every comparison. The ledger resolves 0.8.1, recl9 pins 0.8.2 and Moriarty pins 0.8.3, and their MSM dispatch differs above 2^19 points.
   - Without this, no 10× claim can be checked.
2. **The fused terminal (astra-2).** This is the highest-leverage single change for Sherlock and the only route that clears 10× on (b) without heroic GPU work.
   - Reuse the `literal_bridge.rs:132` and `sherlock_link.rs:444` fragments.
   - Keep all 16 statement fields and the decider hashing.
   - Prove equivalence with the holdouts and mutation tests.
3. **The GPU prover (astra-1).**
   - Start with resident SRS and a GPU MSM and quotient.
   - Then move the grand products and the multiopen.
   - Then rewrite witness synthesis: closures currently run twice, and the advice columns of every phase are allocated up front.
   - sppark (Apache-2.0) is the base. The Nocy RTX 5090 server (2.8× at k17/k18) is the floor to beat: it uses a per-point windowed MSM instead of Pippenger and synchronizes after every commitment.
4. **The throughput scheduler (astra-3).** Several CPU jobs and one GPU context with bounded residency, memory checked at admission (WSL2 has 8 GB of swap in use), and the CPU baseline tuned before claiming any ratio.
5. **Later math.**
   - Upstream midnight-zk **PR #565/#575** adds fflonk (t=4), which cuts the VK from 2,258 to 578 bytes at a small prover cost, and an in-circuit fflonk verifier. Both postdate our checkout. Evaluate them for the inner layers.
   - The existing pairing-residue gadget (316,833 rows) puts a **k19 floor** on any terminal that still verifies a proof.
   - Folding loses at N=16. Nova doesn't support BLS12-381, and a full wrapper lands around k21–k22.

## Gotchas found
- **Byte-identical proofs aren't guaranteed even with a fixed RNG and fixed threads.** HashMap iteration in the lookup prover (`midnight-proofs-0.8.3 plonk/lookup/prover.rs:399-430`) breaks it. So the GPU-equivalence predicate should be "verifier-accepted plus identical commitments where deterministic".
- **The version split (0.8.1, 0.8.2, 0.8.3) is an unverified interoperability risk** for the stagenet call itself, separate from the speed work.
- **Published 25× and 900× figures don't transfer to Sherlock.** ICICLE's 25× was a lookup-only circuit; Snarkify's 900× was a rewrite measured as amortized throughput.

## Factory predicates to add (set L)
| ID | Predicate | Tier |
|---|---|---|
| L0 | k20 parent phase profile measured under one pinned proofs version | T2 heavy |
| L1 | Fused terminal ≤ k15 passes all `uireq`/`rejections` holdouts and the mutation harness | T1 |
| L2 | Fused terminal proof is accepted by the local ledger-9 verifier | T2 |
| L3 | GPU MSM and quotient match the CPU on randomized and edge inputs | T1 |
| L4 | CPU verifier accepts GPU-produced proofs | T2 |
| L5 | Measured chain latency ≤ 0.1× baseline (the 10× gate) | T2 |
| L6 | Measured batches per hour ≥ 10× the tuned-CPU baseline | T2 |
