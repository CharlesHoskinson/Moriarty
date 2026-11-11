# Sherlock co-processor animation, version 2: notes

Owner, 2026-11-11: "for the Sherlock processor animation, we need to start taking notes for version 2 that covers the new terminology, design, and purpose including inspiration from the animation you are working on".

These are working notes. Version 2 starts when the k = 21 recalibration, the elastic chip round and the zero-knowledge design are merged into the architecture. Add notes here as the design moves.

## 1. Terminology

Use `research/sherlock-final-design-20261111/TERMINOLOGY.md` as written: Sherlock is a universal, elastic proving co-processor for Midnight. Sherlock machine, Sherlock chip, leaf, node, decider, Sherlock prover, Sherlock contract, run, claim, Off-chain, On-chain. Version 1 narration says "pipeline" and "system": version 2 says "co-processor" and opens on the one-line definition.

## 2. Purpose: the story version 2 tells

- The co-processor relation: Midnight is the main processor; Sherlock does the work off-chain; the chain checks one proof. Version 1 shows eight steps; version 2 frames them as one delegation and its return.
- Universal: one key set, one contract for every program. Show two different programs reach the same contract with the same keys.
- Elastic: one run, the same claim, on a laptop, on this host's CPU cores and GPU, and on a cluster. The statement and keys do not change; only the wall time does.
- Zero knowledge: what the chain and every helper host see, and what they never see. Version 1 has no privacy scene.

## 3. Design changes since version 1 (to depict when merged)

- Node and decider at k = 21 (owner, 2026-11-11). Gates derived from usable rows (2,097,146) minus tables minus named margins.
- Leaf: row cores, a frozen family of leaf heights (k 16, 18, 20, up to 1,024 steps per leaf), picked per leaf at run time (elastic chip area A, Fable lane). Version 1 shows fixed 16-step slices.
- Aggregation: `b` block chains in parallel plus merges, then SEAL and the decider; `b` chosen at start-up (area B).
- The Sherlock prover: our own proof server; only the decider is on the transaction path; leaves, nodes and SEAL are resumable tasks (area C).
- GPU backend: quotient and NTT on the GPU (57 to 71 percent of a node proof, MEASURED) (area D).
- Zero knowledge: salted `input_d`, hiding outcome commitment, the `n_steps` rule, leaves on the user's machine and aggregation on helpers that see only proofs and hiding commitments (recalibration round, section 4a).
- Optimizations under review: superinstructions, straight-line mode, fixed-base tables, SHA-256 spread tables in RANGE, compute once and consume as a claim (GP-2), a pre-admitted standard library.
- Compact on Sherlock: a Compact contract compiled to the Sherlock machine (compact feasibility round), shown next to native Compact.
- Ledger 9 throughout.

## 4. What to keep from version 1

- The location chip on every beat: "Off-chain: your machine", "Crossing: the transaction", "On-chain: Midnight", and the rail and boundary gate (STORYBOARD-AMEND-1). Version 2 adds "Off-chain: helper host" for elastic aggregation.
- Beat 8.9 "what lives where" as a recurring summary device.
- Every moment addressable by URL (`?scene=N&beat=B&t=T`), deterministic renders, stepped reduced-motion mode.
- Material 3 dark only, impeccable checks, narration as real DOM text.
- Quint lanes that check each animated sequence is a valid trace of the mechanism, and the council step that feeds design revisions.
- Fable 5.2 as creative director over writers, concept artists, builders and visual QA.

## 5. What to fix from version 1

- Counts on screen must match the design (version 1 had 5 cells against 7 slabs in scene 8.5, prover 3 wired to two leaves in scene 4, root-ring marks that differ between scenes 4 and 5).
- Reserve a carrier area from the start; the lower-left overlay covered content in scenes 2, 6 and 7.
- Budget geometries, materials and draw calls per scene in the contract (version 1 scene 8 wide drew 317 calls against 260).
- Phone layouts in the first build, not the fix round (labels clipped).
- The on-chain tertiary green sat close to the outcome teal; pick location colors with contrast checks.

## 6. Scene ideas

- Open: the co-processor in one shot. Midnight hands off, Sherlock works, one proof returns.
- Elasticity: the same run fanned out on three hardware tiers, with the identical claim at the end of each.
- Privacy: a split view, the user's machine with the witness against what a helper host and the chain see.
- The prover: tasks queued, resumed after a crash, only the decider on the transaction path.
- Compact: a Compact contract compiled to Sherlock and accepted by the same `SherlockRun` contract.
