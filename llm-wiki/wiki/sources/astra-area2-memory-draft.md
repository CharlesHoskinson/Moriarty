---
title: "Draft: area 2 memory and values (Astra)"
type: source
tags: [draft, final-design, memory, value-digest]
sources: [astra-area2-memory-draft]
last_updated: 2026-11-11
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/drafts/astra-area2.md
source_url: local
---

# Draft: area 2 memory and values (Astra)

## Summary

Draft, not adopted. Astra's area 2 draft for run `sherlock-final-design-20261111` proposes the modules MEM, MEMORY BACKEND, VALUE DIGEST and BOUNDARY COMMITMENT, for a later merge (merged into [[sherlock-architecture]], 2026-11-11). All new row budgets are ESTIMATE; no file written, nothing built or run. It was written before [[runlength-consensus]] and the lookup cache consensus existed ("absent when checked") and cites the lane reports as proposals.

## Key Claims

- Architecture: four dense append-only arenas DATA, FIELDS, ENV, SLOTS, each with an eight-record tail, length advanced only by real appends; FRAMES and TASKS in cached-top Poseidon hash stacks owned by CORE and HASH.
- MEMORY BACKEND with two implementations behind one interface: `DenseMerkle` (completed segments in a depth-61 tree, separate tail) and `WriteOnceLogUp` (run-wide write-once memory checking, records committed before challenges). BOUNDARY COMMITMENT with two: `PoseidonBoundary` and `HornerBoundary`. Start with `DenseMerkle + PoseidonBoundary`; select the production profile after measuring the whole leaf, node and decider.
- Bus encoding: `key = 8 * address + 2 * arena + lane`, records of six words served as two triples; `address < 2^64`, `length < 2^64`.
- Two shared flush units initially, because one step can append to DATA and FIELDS together. ESTIMATE rows: depth-61 old and new walk pair 8,540; one flush 9,030; cold DATA or ENV read 4,830; a conservative module allowance 51,940 rows, "not evidence that the whole leaf fits `k <= 16`".
- LogUp is not ruled out by the single-phase verifier: a two-pass committed-instance protocol may work, but its soundness is not established. LogUp does not support proving a final prefix before future execution is known; use Merkle memory for streaming and resumable prefixes.
- VALUE DIGEST: the adopted `sherlock-value/1` table (tags 0 to 6, constructor fold `acc`), outcomes `Halted`, `Rejected`, `Exhausted`; no outcome digest for `Running`. `decl_schema_d` has no adopted normal form; refuse key freeze until fixed.
- Scratch: include every future-readable register in the boundary; the spike omits `v0..v2`, and zeroing at a boundary needs a refinement argument.
- SRC-023: memory roots, tails, fingerprints, LogUp sums and challenges stay internal; the C6 statement `[prog_d, input_d, outcome_d, n_steps]` and `claim_d` are unchanged.
- Full memory domain: "A depth-61 segment tree has `2^64` record positions, but a `u64` frontier represents at most `2^64 - 1` allocated records." Multiple appends per step and the initial input mean this does not cover every run below 2^64 steps; establish a capacity theorem or widen the profile before freeze.

## Key Quotes

- "Keep GP-1 unchanged."
- "Early close cannot repair a step that exceeds capacity in an otherwise empty leaf."

## Connections

- [[DenseTailMemory]], [[ValueDigest]], [[Continuations]], [[RunLengthCeiling]], [[lookup-cache-brief]].
- Sister drafts: [[astra-area1-processor-draft]], [[astra-area3-recursion-draft]].

## Contradictions

- Recommends four arenas initially; [[final-design-decisions]] R-6 keeps the arena count open (decide in C4). The audit ([[runlength-consensus-audit]] finding 4) also makes four arenas the default until a contiguity invariant passes.
- Its memory key `8 * address + 2 * arena + lane` differs from [[astra-area1-processor-draft]] (`index + 2^64 * arena + 2^67 * chunk`).
- Its depth-61 capacity concern agrees with audit blocker 3 and lead R-4 (TOP >= 63 recommended, OWNER).
- Rejects the lanes' categorical claim that LogUp is impossible on this stack; [[runlength-consensus]] rejects offline LogUp on the pinned stack (option c5).
- Leaves exhaust kind 7 against profile `/1` open ([[OpenOwnerQuestions]]).
