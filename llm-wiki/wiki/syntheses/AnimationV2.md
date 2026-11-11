---
title: "Animation version 2: running record"
type: synthesis
tags: [animation, visualization, v2, compact, zero-knowledge]
sources: [animation-v2-notes, sherlock-terminology, k21-recalibration-brief, council-findings, council-revisions, cost-round-brief, n1-native-circuits, cost-lane-fable, cost-lane-astra, cost-lane-gemini, compact-graph, prover-graph, elastic-chip-brief, elastic-chip-gpu-fable, n1b-native-proving-time]
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
| Seal | One seal path: NODE seals a SET root, the decider opens its `LEAF` or `DEP` head; the M4 path runs one SEAL node first | [[council-revisions]] P-3 | Design text on branch `pipeline-anim` |
| Admission and input view | ADMIT-WORK admits each program once; VIEW-WORK certifies a read-only input view; SEAL checks both | [[council-revisions]] P-5, P-6 | Design text on branch; OD-17 OWNER |
| Contract | One LEDGER CONTRACT with `acceptRun`, `acceptCollection`, `descend`, `openClaim` over one `runs` map | [[council-revisions]] P-8, [[AcceptRunContract]] | Design text on branch |
| Foreign claims | `reg_root` equal across child slots; `work_root` operand store in the foreign verifier | [[council-revisions]] P-1, P-7 | Design text on branch |
| Cost | Today about 10^4 times native `spend` in rows; the node is more than 90 percent of the rows per step; a fixed floor of one node step and the decider per run | [[cost-lane-fable]], [[cost-lane-astra]] (ESTIMATE) | Proposals, not merged |
| GPU kernels | Five kits: kernel, quotient interpreter, batch seam, broker, parity gate; node proof about 3.5 times faster with the quotient on the GPU | [[elastic-chip-gpu-fable]] (ESTIMATE) | Lane proposal |
| Ledger | Ledger 9 throughout | [[animation-v2-notes]], [[MidnightLedger9]] | Fact |

## Compact compatibility

- A Compact contract compiled to the Sherlock machine and accepted by the same `SherlockRun` contract, shown next to native Compact ([[animation-v2-notes]]).
- Three of the six optimizations target Compact directly: straight-line mode runs flattened Compact operations, fixed-base tables serve `ecMulGenerator` and Jubjub, and the pre-admitted standard library makes Compact's library functions ROM images that every contract reuses ([[k21-recalibration-brief]]).
- Native Compact sizes, measured: `zerocash` `spend` k = 15, 24,025 rows; `vote$commit` k = 14, 15,552 rows; `tiny` `set` k = 13, 3,887 rows; verifier keys 2,121 bytes ([[n1-native-circuits]]). Native proving on this host: `spend` 1.755 s, `vote$commit` 1.022 s, `tiny` `set` 0.552 s, measured, CPU only ([[n1b-native-proving-time]]). Native cost is almost all SHA-256 ([[cost-lane-fable]]).
- The honest comparison for the native-Compact beat: Sherlock today is about 10,400 to 31,300 times native `spend` in rows (ESTIMATE, [[cost-lane-fable]]); 15,108 to 30,174 times at the gate figures (ESTIMATE, [[cost-lane-astra]]). The lanes disagree on how close the levers get (4 to 8 times, a 9.16 times TARGET, or 1.51 to 8.74 times) ([[cost-lane-fable]], [[cost-lane-astra]], [[cost-lane-gemini]]).
- Where Sherlock beats native: unbounded or data-dependent loops, composition, no per-entry key ceremony, resumable proving ([[cost-lane-fable]], [[cost-lane-gemini]]).
- The Compact compiler is graphed for the lanes: 6,907 nodes ([[compact-graph]]); the ledger 9 prover stack: 21,437 nodes ([[prover-graph]]).
- The compact feasibility round itself is not yet ingested; its findings enter here when it is.

## Zero-knowledge compatibility

- Owner ruling: zero knowledge end to end ([[ZeroKnowledge]]).
- What version 2 shows: the salted `input_d`, the hiding outcome commitment, the `n_steps` rule, leaves on the user's machine and aggregation on helper hosts that see only zero-knowledge proofs and hiding commitments, with a location chip "Off-chain: helper host" ([[animation-v2-notes]], [[k21-recalibration-brief]]).
- Compact parity: Sherlock reveals no more than native Compact (only `disclose`d values public) ([[k21-recalibration-brief]]).
- Blinding: every RNG draw stays on the CPU from the caller's RNG; in production proofs are seeded from the operating system, so no two are equal; seeded byte-identical proofs are a test mode, reproducible only at a fixed seed and thread count on `midnight-proofs` 0.8.3 ([[elastic-chip-gpu-fable]] F-D1 to F-D3, [[ZeroKnowledge]]).

## Kept from version 1, fixed in version 2

Kept: the Off-chain, Crossing and On-chain location chip, the "what lives where" beat, addressable deterministic moments, Material 3 dark, Quint trace checks, the council step. Fixed: on-screen counts that differ from the design, the carrier overlay, per-scene draw budgets, phone layouts in the first build, location colors with contrast checks ([[animation-v2-notes]]).

## Record

- 2026-11-11: Version 2 notes started; terminology fixed; k = 21 decision and zero-knowledge requirement recorded ([[animation-v2-notes]], [[sherlock-terminology]], [[k21-recalibration-brief]]).
- 2026-11-11: Council C1 to C10 and revisions P-1 to P-10 (branch `pipeline-anim`), the cost round brief, N1 and three lanes, the Compact and prover graphs, the elastic chip brief and the area D Fable lane added to the design table, the Compact section and the zero-knowledge section ([[council-findings]], [[council-revisions]], [[cost-round-brief]], [[n1-native-circuits]], [[cost-lane-fable]], [[cost-lane-astra]], [[cost-lane-gemini]], [[compact-graph]], [[prover-graph]], [[elastic-chip-brief]], [[elastic-chip-gpu-fable]]).
