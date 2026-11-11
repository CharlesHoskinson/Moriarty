---
title: "Animation version 2: running record"
type: synthesis
tags: [animation, visualization, v2, compact, zero-knowledge]
sources: [animation-v2-notes, sherlock-terminology, k21-recalibration-brief]
last_updated: 2026-11-11
---

# Animation version 2: running record

Owner, 2026-11-11: "keep a log in wiki-llm for version 2 of the animation that reflects all the new design and the compact and ZK compatibility". This page holds what version 2 of the Sherlock co-processor animation must show. Every design round that lands adds a dated entry under "Record" and updates the sections above it. The working notes on the animation branch are [[animation-v2-notes]].

## Terminology

Sherlock is a universal, elastic proving co-processor for Midnight ([[sherlock-terminology]]). Version 2 says "co-processor", not "pipeline", and uses the term table: Sherlock machine, Sherlock chip, leaf, node, decider, Sherlock prover, Sherlock contract, run, claim, Off-chain, On-chain. "Zero-knowledge" joins the name when [[ZeroKnowledge]] is in the architecture.

## Purpose

- One delegation and its return: Midnight hands work to Sherlock, Sherlock proves it off-chain, the chain checks one proof ([[animation-v2-notes]]).
- Universal: two different programs reach the same contract with the same keys.
- Elastic: one run on a laptop, this host and a cluster gives the same claim.
- Private: what the chain and each helper host see, and what they never see.

## Design to depict

| Area | What version 2 shows | Source | State |
|---|---|---|---|
| Circuit size | Node and decider at k = 21, 2,097,146 usable rows, gates derived from rows minus tables minus margins | [[k21-recalibration-brief]] | Owner decision; recalibration lanes queued |
| Leaf | Row cores as a frozen family of leaf heights (k 16, 18, 20, up to 1,024 steps per leaf), picked per leaf at run time | [[animation-v2-notes]] (elastic chip area A) | Proposal |
| Aggregation | `b` parallel block chains plus merges, then SEAL and the decider | [[animation-v2-notes]] (area B) | Proposal |
| Prover | Our own Sherlock prover; only the decider on the transaction path; resumable tasks | [[animation-v2-notes]] (area C) | Proposal |
| GPU | Quotient and NTT on the GPU, 57 to 71 percent of a node proof (measured) | [[animation-v2-notes]] (area D) | Proposal |
| Optimizations | Superinstructions, straight-line mode, fixed-base tables, SHA-256 spread tables in RANGE, compute once and consume as a GP-2 claim, pre-admitted standard library | [[k21-recalibration-brief]] | Under review |
| Ledger | Ledger 9 throughout | [[animation-v2-notes]], [[MidnightLedger9]] | Fact |

## Compact compatibility

- A Compact contract compiled to the Sherlock machine and accepted by the same `SherlockRun` contract, shown next to native Compact ([[animation-v2-notes]]).
- Three of the six optimizations target Compact directly: straight-line mode runs flattened Compact operations, fixed-base tables serve `ecMulGenerator` and Jubjub, and the pre-admitted standard library makes Compact's library functions ROM images that every contract reuses ([[k21-recalibration-brief]]).
- The compact feasibility round (feasible with named modules) is not yet ingested; its findings enter here when it is.

## Zero-knowledge compatibility

- Owner ruling: zero knowledge end to end ([[ZeroKnowledge]]).
- What version 2 shows: the salted `input_d`, the hiding outcome commitment, the `n_steps` rule, leaves on the user's machine and aggregation on helper hosts that see only zero-knowledge proofs and hiding commitments, with a location chip "Off-chain: helper host" ([[animation-v2-notes]], [[k21-recalibration-brief]]).
- Compact parity: Sherlock reveals no more than native Compact (only `disclose`d values public) ([[k21-recalibration-brief]]).

## Kept from version 1, fixed in version 2

Kept: the Off-chain, Crossing and On-chain location chip, the "what lives where" beat, addressable deterministic moments, Material 3 dark, Quint trace checks, the council step. Fixed: on-screen counts that differ from the design, the carrier overlay, per-scene draw budgets, phone layouts in the first build, location colors with contrast checks ([[animation-v2-notes]]).

## Record

- 2026-11-11: Version 2 notes started; terminology fixed; k = 21 decision and zero-knowledge requirement recorded ([[animation-v2-notes]], [[sherlock-terminology]], [[k21-recalibration-brief]]).
