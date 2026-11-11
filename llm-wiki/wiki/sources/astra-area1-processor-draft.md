---
title: "Draft: area 1 processor modules (Astra)"
type: source
tags: [draft, final-design, processor, chip-leaf]
sources: [astra-area1-processor-draft]
last_updated: 2026-11-11
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/drafts/astra-area1.md
source_url: local
---

# Draft: area 1 processor modules (Astra)

## Summary

Draft, not adopted. Astra's area 1 draft for run `sherlock-final-design-20261111` defines the processor modules CORE, DEC, ALU with RANGE, HASH and BUS, for a later merge into the final architecture (merged into [[sherlock-architecture]], 2026-11-11). It is a proposed architecture: no file written, no build, test, model or proof run, and the signatures are not compiled. It was written before [[runlength-consensus]] existed ("The run-length `CONSENSUS.md` was absent when checked") and cites the round 2 lane files instead. Memory, boundary commitments and recursion are separate areas ([[astra-area2-memory-draft]], [[astra-area3-recursion-draft]]).

## Key Claims

- Facts about the current spike (from design 2.7): 12 advice, 13 equality and 20 fixed columns, four lookup arguments, degree six; leaf, node and decider 6,511, 823,466 and 929,166 rows; no full rule inventory, PROG, TRAP or Init.
- One bus means one five-field schema and provider namespace, not one PLONK lookup argument; keep the measured two request lanes. Chip ids: 0 disabled, 1 ALU, 2 CELL, 3 HASH2, 4 PROG, 5 and 6 reserved for DEC and RANGE.
- Memory key: `cell_key = index + 2^64 * arena + 2^67 * chunk`, four logical arenas DATA, FIELDS, ENV, SLOTS. The memory module may map them to one physical arena or four.
- CORE carries complete record schemas (frame 13 lanes, task 7, helper 23, result 4), not the spike's `Regs32`. Ten slot invariants, among them a prefix of enables, `steps = sum of enables` with `1 <= steps <= S`, and a leaf close that changes no counter, fuel, outcome or length. An ordered `AppendEvent` port shared by CORE and MEM, because set membership does not prove an append happened once.
- Scratch: reset at every slot entry and forbid read before write.
- Stacks: cached tops with Poseidon commitments below (domains `DOM_FRAME_V1` and a task domain). ESTIMATE frame spill or open about 560 rows, task about 350 rows, at 70 rows per permutation.
- DEC: finite priority network, one class per rule of the frozen inventory, a two-column ROM. ALU: ADD, LT (spike) plus proposed SUB, MUL (64-bit limbs), EQ, NOT. RANGE: 12-bit limbs, one lookup. HASH: the pinned Poseidon sponge, ESTIMATE `70 * ceil(N/2)` rows.
- Each rule must publish its maximum requests, appends, range checks and hashes; the leaf must fit the largest single step.
- Open decision recorded: the full GP-1 storage bound. "Multiple appends per step mean a 64-bit step bound alone does not establish a 64-bit storage bound."

## Key Quotes

- "No decision here narrows GP-1 through GP-4."
- "An early-close policy cannot repair a step that cannot fit an empty leaf."

## Connections

- [[ChipLeaf]], [[MoriChip]], [[DenseTailMemory]], [[GeneralPurposeRecursion]].
- Sister drafts: [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]].

## Contradictions

- Memory key encodings differ between drafts: this draft packs `index + 2^64 * arena + 2^67 * chunk`; [[astra-area2-memory-draft]] packs `8 * address + 2 * arena + lane`. The merge must pick one.
- Lists 14 spec conflicts, among them: literal "one lookup" in design 2.7 against the spike's lookups; D3 leaf-close allocation against dense execution; D7 memory links against hash stacks; the run-length lanes' proposed GP-1 text (lane B) as a narrowing; `n_steps` in the GP study's `out_d` against C6 ([[gp-architecture]]); exhaust kind 7 against profile `/1` ([[OpenOwnerQuestions]]).
- Its scratch reset per slot agrees with audit finding 9 and lead decision R-8 ([[final-design-decisions]]); the consensus's slot-0 zeroing claim is the one the audit rejects.
