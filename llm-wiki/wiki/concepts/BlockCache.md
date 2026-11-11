---
title: "Block cache: 8-record PAGE and ROM cache lines in the chip leaf"
type: concept
tags: [memory, cache, lookup, chip-leaf, lead-decision]
sources: [lookup-cache-consensus, lookup-cache-audit, final-design-decisions, lookup-cache-brief, c1-dense-memory-experiment]
last_updated: 2026-11-11
---

# Block cache: 8-record PAGE and ROM cache lines in the chip leaf

The "processor cache" the owner asked for ([[lookup-cache-brief]]). Status: **lead decision L-1, pending owner review** ([[final-design-decisions]]); not an owner ruling, not built, not measured. It extends [[DenseTailMemory]] and does not replace it.

## Mechanism

- Two levels ([[lookup-cache-consensus]]): the hot window of dense memory (reads at or above the tail base, no Merkle walk) and a small number of cache lines per leaf. A heap line opens one flushed 8-record segment against the leaf's pre-state root and serves all 8 records on the bus; a ROM line opens 8 program or type-table rows against `prog_root`. Repeat reads of an opened line inside the leaf cost no hash rows.
- A line changes no boundary value: `(len, root, tail_d)` and the public statement do not depend on which lines a leaf opens (SRC-023) ([[lookup-cache-audit]] rule 9).
- Profile B (run-wide write-once LogUp with a Horner boundary) stays behind the `MemBackend` seam as benchmark-only, not built before milestone M4 (L-5).

## Contract (audit section 5.1, taken into L-1)

The ten rules become MEM / PAGE contract items ([[final-design-decisions]] L-1; [[lookup-cache-audit]]):
1. A heap line opens against `pre.root`, never the running root; segments flushed in this leaf are served by the window.
2. The 8 line leaves are `H_REC(kind, a, b, c)` with the capacity tags of `dense.rs`; `Tree8` uses heights 1 to 3, the walk heights 4 to TOP + 3, as in `persist`.
3. Provider cells are the hashed cells; row `j` serves `(MEM, 8 (8 blk + j) + kind_j, a, b, c)`.
4. Walk bits are the TOP-bit decomposition of `blk`.
5. `en` boolean; with `en = 0` all five provider cells are zero and the root equality is off.
6. `blk < q_pre`, so window and cache are disjoint by a local rule.
7. `kind` in `0..=4` by induction from `persist` (needs an empty or Init-proved genesis root), or repeat the kind gate on line rows.
8. Deduplication and ordering of lines are not soundness conditions; add no constraint for them.
9. A line changes no boundary value.
10. Capacity: `K_page`, `K_rom` and the lane count are at least the per-step maxima.

CORE obligations with it: every read takes its address from a register, constrains the expected kind and checks `addr < len_cur` at that step; unused request lanes are zero in all five cells ([[lookup-cache-audit]] items 11 to 14).

## Lead decisions around it

- L-2: the scalar `cold_read` stays. An isolated old read costs 4,761 rows with the scalar walk (measured, [[c1-dense-memory-experiment]]) against 6,020 with a line (ESTIMATE). `K_page` and `K_rom` are not frozen until the per-rule maximum table is measured; a step must never need more lines than a leaf has (GP-1).
- L-4: the landed C1 interface (`membackend.rs`, `dense.rs`, key `8 addr + kind`) is the MEM interface; the study's section 3 interface is not adopted. The audit asks for one block-line method on `MemBackend`, keeping `persist`.
- L-9: one arena (see [[DenseTailMemory]]).

## Figures

- ESTIMATE ([[lookup-cache-consensus]], audit agrees on the arithmetic): one heap line is 86 Poseidon permutations, 6,020 rows at TOP = 63 (84, 5,880 at TOP = 61). The per-record figure (about 735 to 752.5 rows) is a best case with all 8 records used ([[lookup-cache-audit]] finding 7).
- ESTIMATE ([[lookup-cache-audit]]): a ROM line with its row hashes is about 55 permutations at depth 32 (about 3,850 rows), not 39. One-arena leaf at S = 16, TOP = 63 with 4 heap and 2 ROM lines: about 50,700 to 51,800 rows, inside `k_leaf <= 16` with about 14,000 rows margin. Four arenas: about 67,500 to 85,100, above the gate.
- No locality census or miss-rate measurement exists; the G-C margin depends on it ([[lookup-cache-audit]] finding 19).

## Open

- The per-rule maximum table (bus requests, old segments, ROM blocks per table, appends, stack spills) before any line count is frozen; owner review of L-2 ([[OpenOwnerQuestions]]).
- ROM depth: a fixed depth bounds program size against GP-001 "every program"; the depth, leaf domain, padding and path mapping are not stated ([[lookup-cache-audit]] finding 12). No lead decision records it.
- The block-line method on `MemBackend` is not built; nothing of the cache is measured.

## Contradictions

- The consensus freezes `K_page = 4`, `K_rom = 2` and replaces the scalar cold read; the audit (blocker 1) and lead L-2 do not ([[lookup-cache-consensus]], [[lookup-cache-audit]]).
- The consensus's leaf figures (42,100 to 45,600 rows for one arena; four arenas "fit" at 51,800 to 56,400) against the audit's composition (about 50,700 to 51,800; about 67,500 to 85,100), all ESTIMATE.
- The consensus says the ROM tree "handles arbitrary program size"; the audit says the fixed depth bounds it.

See also [[ChipLeaf]], [[ValueDigest]], [[MoriChip]].
