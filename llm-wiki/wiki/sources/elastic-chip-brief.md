---
title: "Elastic chip round brief: the elastic multicore Sherlock chip and its prover"
type: source
tags: [brief, elastic, multicore, gpu, prover]
sources: [elastic-chip-brief]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-elastic-chip-20261111/BRIEF.md
source_url: local
---

# Elastic chip round brief

## Summary

The owner asked for one more chip iteration: an elastic multicore chip that scales to the host's GPU and CPU cores, with the prover developed hand in glove with the chip. Twelve lanes: four areas, each answered independently by one Claude Fable 5.2, one GPT-6 Astra high and one Gemini 4 Carbon high lane. The lead merges them into one proposal. The twelve proposals get one wiki page from their merge; the area D Fable lane's prover findings have their own page ([[elastic-chip-gpu-fable]]).

## Key Claims

- Goal: elastic (the same key set and on-chain contract serve a laptop, this host and a GPU cluster, with no recompilation and no new key), multicore (independent work at every level maps to cores and GPU streams), compatible (proofs verify under the unchanged ledger 9 verifier, byte for byte as the stock prover would produce them, and the prover speaks the Midnight proof server API).
- Area A, the elastic multicore leaf: cores inside one leaf, larger S, several fixed leaf shapes in one key set, or leaves only; test the cost round's claim that "column cores break the node gate" ([[cost-lane-fable]]).
- Area B, elastic aggregation: binary or wider nodes, merge tree against chain, folding against full recursive verification, streaming aggregation, the height counter, the single seal path and the decider.
- Area C, the bespoke Midnight prover on the ledger 9 proof server's design (`/prove`, `/prove-tx`, `/check`, `/k`, `/fetch-params`), read through the prover graph ([[prover-graph]]).
- Area D, the GPU proving backend for `midnight-proofs` (KZG over BLS12-381), determinism and byte-identical proofs.
- Every area gives `T(cores, SMs)` for `zerocash` `spend` and for 10^6 and 10^9 steps, and its interfaces to the other areas.
- Starting facts: native `spend` k = 15, 24,025 rows (measured, [[n1-native-circuits]]); leaf 15,226 rows at S = 16 (ESTIMATE); node gate 950,000 and decider gate 1,000,000 rows; node step 61 to 67 s, decider 70 s (measured). Lanes must confirm or refute that a k = 20 decider proof verifies under the stock ledger 9 verifier and name the experiment.
- Sources: the revised design on branch `pipeline-anim` (24 kits, decisions P-1 to P-10, commit `f372ab74c`; [[council-revisions]]).
- Constraints: GP-1 to GP-4 never narrowed; one key set for every program and every host; a per-contract or per-host key is a finding, not a design; on-chain parts unchanged unless shown necessary.

## Key Quotes

- Owner, 2026-11-11: "Let's do one more iteration of the chip design to make it an elastic multicore chip that scales up to the host's GPU and Core core count."
- Owner: "if we go multicore with the chip, we should build a bespoke Midnight Prover".

## Connections

[[elastic-chip-gpu-fable]], [[prover-graph]], [[council-revisions]], [[cost-round-brief]], [[k21-recalibration-brief]], [[AnimationV2]], [[ZeroKnowledge]].

## Contradictions

- "Byte for byte as the stock prover" holds only at a fixed seed and rayon thread count on `midnight-proofs` 0.8.3 (F-D1 of [[elastic-chip-gpu-fable]]), and fresh production blinders make every proof differ ([[ZeroKnowledge]]).
