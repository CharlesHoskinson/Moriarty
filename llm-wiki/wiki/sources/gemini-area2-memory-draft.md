---
title: "Draft: area 2 memory and values (Gemini)"
type: source
tags: [draft, final-design, memory, value-digest, boundary, logup]
sources: [gemini-area2-memory-draft]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/drafts/gemini-area2.md
source_url: local
---

# Draft: area 2 memory and values (Gemini)

## Summary

Draft, not adopted. The Gemini lane of run `sherlock-final-design-20261111` wrote this area 2 draft (`GEMINI-DONE` lists it with the area 1 and area 3 drafts; the file itself names no model). It specifies four modules with frozen interfaces, invariants, row budgets, Quint models, named MockProver mutants, Lean obligations, code skeletons and OpenSpec capability specs: MEM (`WindowCfg`), MEMORY BACKEND (trait `MemBackend<F>`, implementations 2A `DensePageBackend` and 2B `ZmLogUpBackend`), VALUE DIGEST (`sherlock-value/1`) and BOUNDARY COMMITMENT (trait `BoundaryCommit<F, B>`, implementations 4A `PoseidonBoundary` and 4B `HornerBoundary`). Profile A (2A + 4A) is the recommended default; Profile B (2B + 4B) is a benchmark alternative. Every new row count, cache hit rate, RAM figure and wall-clock figure is ESTIMATE; nothing was built or run. It was merged into `ARCHITECTURE.md` of the same run ([[sherlock-architecture]]). Sister draft for the same area: [[astra-area2-memory-draft]].

## Key Claims

- Tree height: default `TOP = 63` (2^66 minus 1 cells per arena, 66-bit `WindowCfg::bits`), so the step counter `h < 2^64` binds before cell capacity "even at 4 cell allocations per step". Cost given as +2 permutations per walk, +140 rows per cache slot, about 840 leaf rows total at `W = 1, K_page = 4` (ESTIMATE). The draft calls this "Recommended & Adopted Here".
- Bus: splits the landed IDs (`CHIP_ALU = 1`, `CHIP_MEM = 2`, `CHIP_HASH = 3`) into `CHIP_NOP = 0`, `CHIP_ALU = 2`, `CHIP_MEM_ALLOC = 3`, `CHIP_MEM_READ = 4`, `CHIP_ROM = 5`, `CHIP_HASH = 6`, to be changed together with area 1. Memory key `key = (arena_id << 66) | addr`; ROM key `(tbl_id << 32) | row_idx` with `tbl_id` 1 to 15 covering the 14 `ProgramView` runtime tables plus one packed overflow table.
- MEM: dense +1 allocation, an 8-record tail `(len, tail_d)` padded with non-zero `pad_d()`, a `FlushPort (do_flush, q_pre, seg_d)` per crossed 8-record boundary, and hot reads only for `8 * q_pre <= addr < len_curr` (causal, no read-before-write). Reads below `base` go to the backend.
- 2A `DensePageBackend`: empty-before-write flush with a dual height-`TOP` Merkle walk and height-tagged `iv(DOM_NODE, h)`; `K_page` 8-record cache lines (4 at S = 16, 6 at S = 64) and `K_rom` ROM blocks (2 and 3) under `prog_root`, with zero loader leaves. Row figures (ESTIMATE): 8,820 rows per flush walker, 6,020 rows per PAGE slot, 2,730 rows per ROM block; one arena about 38,900 to 40,200 backend rows at S = 16.
- 2B `ZmLogUpBackend`: run-wide write-once LogUp with a Stage-1 `Instance(0)` pre-commitment and six mandatory hardenings (prog_root ROM, zeroed trailing rows plus a blinding scalar, `0 <= m_w < 2^64`, dual F_{r^2} challenges from a commitment-tree digest `d_com`, zero-denominator and inactive-inverse guards, a full G1 subgroup check on each `C_i` against order-11 torsion). Claimed soundness error below 2^-375 (unlabeled, a Schwartz-Zippel derivation).
- Profile comparison (ESTIMATE): Profile A leaf about 45,000 to 52,700 rows at S = 16 (`k_leaf = 16`) and 86,000 to 96,000 at S = 64 (`k_leaf = 17`); Profile B leaf about 12,500 to 16,000 (`k_leaf = 14`) but +9,500 to 20,200 node rows per leaf for the subgroup check, which may push a 2-leaf `RunLift` past 1,048,570 rows. Serial wall clock for `list123` is about 20.5 to 23.0 min (A) against 20.2 to 22.5 min (B), under 2% difference.
- Profile B needs two passes with a full-run barrier and is "Incompatible with O(1) Mode-1 stop-and-resume"; Profile A streams leaf by leaf with Mode-1 checkpoints `SEG + SEG -> SEG`.
- Arenas: open decision; one arena (`kfix = 0`, record kind in `f0` tag bits, 34-word `ArchState`) is the recommended default pending experiment P3; four arenas (`DATA`, `ENV`, `FRAME`, `SLOT`, shared `W_global = 2` flush walkers, 42 words) about 58k to 62k leaf rows (ESTIMATE). P3 picks four arenas only if they fit 62,000 rows and cut effective leaf count by at least 10%.
- VALUE DIGEST: one hardwired post-permutation prefix state `[C_0, C_1, C_2]` saves one permutation (70 rows) per dynamic value (ESTIMATE); kind tags 1 to 9 including `Tuple` 7, `Record` 8, `Array` 9 (SI-12 to SI-14, citing `SRC-025`); `Nil`, `Bool` and ROM constants cost zero runtime permutations; `outcome_d` under `iv(DOM_OUTCOME, tag)` with tags 1 Halted, 2 Rejected, 3 Exhausted. A pointer-keyed `AS_HASH` memo table is rejected.
- BOUNDARY COMMITMENT: `ArchState` is 29 registers, `fp_hash`, `tp_hash`, the window boundaries and the backend token, and excludes scratch `v0..v2` as step-local. 4A hashes it under `iv(DOM_STATE, W_state)` (2,380 rows for 34 words, ESTIMATE) with a 6-lane `Instance(1)` `[prog_root, pre_d, post_d, steps, halt_flag, outcome_d]`; 4B is sealed at compile time to backends implementing `PrecommitsInstance0`, with 14 lanes.
- Leaf overflow: when a step would need a `(K_page + 1)`-th cold block or a `(K_rom + 1)`-th ROM block, the leaf closes early; progress is claimed because one step issues at most 4 bus requests, which fits `K_page >= 4, K_rom >= 2`.
- Gating experiments: P2 freezes `(K_page, K_rom)` so under 2% of leaves close early and keeps Profile A unless B is at least 20% faster end to end; P5 checks the 2-leaf `RunLift` fits `k = 20`. Lean obligations: new files `DenseWindow.lean`, `WriteOnceLogUp.lean`, `ValueDigestCanonicity.lean`, `BoundaryCommitment.lean`, `PartitionIndependence.lean`.
- Conflicts it flags: `D-1` is at `dissent.md:198-202`, not `:27`; Poseidon is 68 rounds and 70 rows per permutation in `poseidon.rs`, not the "63-round" `hashchip.rs` the lookup consensus cites; `SRC-024` and `SRC-025` are not registered in `requirements.md`; the chip leaf's 6 lanes must supersede the 7-lane D5 leaf statement.

## Key Quotes

- "Every unmeasured row count, cache hit rate, host RAM footprint, or wall-clock timing projection is marked **`ESTIMATE`**."
- "Current analysis shows `< 2%` serial wall-clock difference (`CONSENSUS.md:76`), so `Profile A` dominates on simplicity, streaming, and `GP-1` checkpointing."
- "Adding a separate mutable cross-leaf `AS_HASH(ptr -> vd)` memoization table is rejected"

## Connections

- Concepts: [[DenseTailMemory]], [[BlockCache]], [[ValueDigest]], [[Continuations]], [[RunLengthCeiling]], [[C6OutcomeOnlyPublication]], [[GeneralPurposeRecursion]].
- Entities: [[ChipLeaf]], [[MoriChip]], [[IvcNode]], [[RunDecider]], [[AcceptRunContract]].
- Decisions and experiments: [[final-design-decisions]], [[c1-dense-memory-experiment]], [[rule-maxima-experiment]].
- Merged into [[sherlock-architecture]]. Sister draft: [[astra-area2-memory-draft]]. Questions and status: [[OpenOwnerQuestions]], [[WorkstreamStatus]].

## Contradictions

- Arenas: recommends one arena pending P3, which agrees with lead decision L-9 ([[final-design-decisions]]); the Astra draft ([[astra-area2-memory-draft]]) recommends four arenas initially. Its four-arena estimate (about 58k to 62k leaf rows, ESTIMATE) is lower than L-9's 67,500 to 85,100 (ESTIMATE).
- Key encoding: `(arena_id << 66) | addr` with new split bus IDs; Astra uses `8 * address + 2 * arena + lane`, and L-4 adopts the landed C1 interface with key `8 addr + kind`.
- Value digest: its single hardwired prefix state is what L-3 rejects (one constant per preimage length is needed). Its kind tags 1 to 9 with `Tuple`, `Record`, `Array` and `SRC-025` conflict with L-10, which drops `Value::Tuple/Record/Array` and `SRC-025`, and with Astra's adopted `sherlock-value/1` table (tags 0 to 6).
- Scratch registers: excludes `v0..v2` from `ArchState` as step-local; Astra says every future-readable register belongs in the boundary and that the spike's omission of `v0..v2` needs a refinement argument.
- Cold reads: serves all cold heap reads through `K_page` lines with `K_page = 4` as a default; L-2 keeps the scalar `cold_read` and does not freeze `K_page` or `K_rom` until the per-rule maxima are measured. [[rule-maxima-experiment]] leaves `K_rom` open (up to 6 ROM call sites in 4 tables in one step, STATIC), so the draft's claim that `K_rom >= 2` suffices for every step is not supported there.
- Profile B: specifies 2B with all hardenings as a task to land (task 1.3); L-5 keeps Profile B benchmark-only and not built before milestone M4. Astra says the soundness of a two-pass LogUp protocol is not established, where this draft gives a soundness bound.
- `TOP = 63` is called adopted here; lead R-4 recommends TOP >= 63 but marks it OWNER to confirm. Its cost of +140 rows per cache slot is ESTIMATE; E-1 measured +280 rows per flush and +140 per old-memory read.
- Relies on a 2-leaf `RunLift` (experiment P5); L-10 says the 2-leaf `RunLift` is unbuilt and not part of the design until measured.
