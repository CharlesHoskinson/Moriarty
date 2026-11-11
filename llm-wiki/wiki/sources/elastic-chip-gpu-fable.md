---
title: "Elastic chip round, area D, Fable lane: the GPU proving backend"
type: source
tags: [lane-report, gpu, prover, determinism, zero-knowledge, ledger9]
sources: [elastic-chip-gpu-fable]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-elastic-chip-20261111/proposals/D-fable.md
source_url: local
---

# Elastic chip round, area D, Fable lane: the GPU proving backend

## Summary

The Claude Fable 5.2 answer for area D of [[elastic-chip-brief]]. No build, proof or commit was run; every GPU kernel time is an ESTIMATE (only the G0 toolchain smoke has run on this host). It keeps the existing `mori-gpu` track and adds five kits: GPU-KERNEL, GPU-QUOTIENT (a circuit-independent quotient interpreter, the headline), GPU-SEAM version 2 (batch calls on polynomial handles), GPU-BROKER (one device owner per process) and GPU-PARITY (the byte-identity gate). It also records three findings on prover determinism and blinding, F-D1 to F-D3, which the lane left for the wiki. A lane proposal; the round merges later.

## Key Claims

- **F-D1.** In `midnight-proofs` 0.8.3 a seeded proof depends on the rayon thread count: the vanishing argument's random polynomial is filled by per-chunk ChaCha streams whose boundaries come from `current_num_threads()` (DESIGN, `vanishing/prover.rs:51-73`). Two hosts with different core counts and the same seed give different proof bytes on the stock CPU prover, so byte identity holds only at a fixed (seed, thread count).
- **F-D2.** `midnight-zk` HEAD (`68a9354`, unreleased) draws the quotient-limb blinders from `OsRng` directly (DESIGN, `ZK/proofs/src/plonk/prover.rs:691`). After the next pin a seeded byte-identity test is impossible without a test-only RNG injection.
- **F-D3.** The proof format changes between 0.8.3 and HEAD (quotient limbs, no linearization, LogUp in place of the permuted-column lookup, grouped commitments). The ledger 9 lock file names `midnight-proofs` 0.8.2; the Moriarty workspace and the GPU track use 0.8.3. "Stock prover" means registry 0.8.3 with `midnight-curves` 0.3.1, and the GPU seam is cut again at every pin.
- Determinism rules: every RNG draw stays on the CPU in the stock order from the caller's RNG; the backend never draws randomness; MSM results equal BLST as group elements; NTT and quotient outputs are bit-equal; no call moves across a Fiat-Shamir challenge of its own proof; fallback calls the stock function.
- In production the proof server seeds from the operating system, so no two proofs are equal; the parity gate is a test property, run at rayon pool sizes 1, 4 and 20.
- Ledger 9 and k = 20: "confirmed from sources, not re-run by this lane." `ParamsVerifierKZG` holds only G2 elements, so the embedded 2^14 parameter file does not bound circuit size for non-committed instances; a prior run (`20261110T032716Z-A16-fold3-decider`) proved a k = 20 decider in 70.35 s and passed stock and installed ledger verifier checks (as recorded in `tasks.md` task 2.6). X-D0 repeats it on a GPU-produced proof.
- Where the time goes in one k = 20 node proof (measured, C0 profile): quotient plus NTT 0.57 to 0.71 of the wall, MSM 0.16 to 0.26; one proof uses 9.77 of 20 cores; three side by side take 1.73 times the solo wall.
- Speedup of one node proof (ESTIMATE on measured shares): MSM and FFT hooks only, 1.57 to 2.08 times; with the quotient on the GPU about 3.5 times (61 to 70 s down to 17 to 20 s); node throughput on this host from 1 per 35 s to 1 per 5 to 9 s. Leaves gain 3 to 5 times in a batch, then CPU witness generation binds.
- Elastic tiers from one binary: T0 no GPU, TU unified memory (this host), TD discrete GPUs; the CUDA core is a `dlopen`ed library, so the prover starts on any machine and no key or proof byte depends on the tier.
- `zerocash` `spend` at today's shape (S = 16, 563 leaves, ESTIMATE): 5.6 h CPU on this host, 0.8 to 1.4 h with GPU phase 2, 4 to 5 minutes on 64 GPUs with tree aggregation; 10^9 steps stays impractical at S = 16 (10 to 18 years on this host).
- Asks of area A: few distinct k values, price a degree-5 variant (halves the extended domain), flat witness generation; of area B: a uniform node shape and independent nodes; of area C: one process owns the device, the RNG is passed in, receipts carry the GPU tier.
- Experiments X-D0 to X-D9 and risks R-1 to R-4; first step X-D1, split the measured quotient share (one day, no GPU code). X-D8 confirms or refutes F-D1 by proving twice with one seed at pool sizes 4 and 20.
- Papers: nine listed, all UNREAD, ids mostly from memory.

## Key Quotes

- "For the same proving key, instances, witness, RNG seed and rayon thread count, the GPU path and the stock CPU path of the same `midnight-proofs` version write the same proof bytes."
- "In production the proof server seeds from the operating system, so no two proofs are equal anyway; the gate is a test property."

## Connections

[[elastic-chip-brief]], [[ZeroKnowledge]], [[MidnightLedger9]], [[RunDecider]], [[IvcNode]], [[prover-graph]], [[cost-lane-fable]], [[AnimationV2]].

## Contradictions

- The brief's goal "byte for byte as the stock prover would produce them" ([[elastic-chip-brief]]) is narrowed by F-D1 to a fixed seed and thread count.
- The 2^14 verifier question: confirmed from sources here; left open in [[cost-lane-fable]] section 7.
- F-D1 to F-D3 are readings of the code (DESIGN); X-D8 has not run.
