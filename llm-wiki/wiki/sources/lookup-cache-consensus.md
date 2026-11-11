---
title: "Lookup cache consensus (five Gemini 4 Carbon High lanes)"
type: source
tags: [lookup, cache, memory, chip-leaf, consensus]
sources: [lookup-cache-consensus, lookup-cache-audit, lookup-cache-brief, runlength-consensus, runlength-consensus-audit, final-design-decisions]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-lookup-cache-20261111/CONSENSUS.md
source_url: local
---

# Lookup cache consensus (five Gemini 4 Carbon High lanes)

## Summary

Round 3 synthesis of run `sherlock-lookup-cache-20261111` over lanes A to E, which answers the brief [[lookup-cache-brief]]. The study ran no `cargo`, `cost_model`, MockProver, Quint or Lean; every unmeasured figure is marked ESTIMATE by the source. It recommends **Profile A**: the dense tail-buffered Merkle memory of [[runlength-consensus]] plus a few deduplicated 8-record cache lines per leaf (heap PAGE lines against the pre-state root, ROM lines against `prog_root`) and a Poseidon boundary digest. It keeps **Profile B** (hardened run-wide write-once ZmLogUp with a Horner boundary over a committed `Instance(0)`) behind the `MemBackend` and boundary interfaces as a benchmark alternative. It calls itself a compatible extension of the run-length consensus, not a replacement. The Fable 5.2 audit adopts the direction with changes and rejects sections 2.2 and 3 as written ([[lookup-cache-audit]]). The lead's decisions on it are L-1 to L-10 ([[final-design-decisions]]). See [[BlockCache]], [[DenseTailMemory]].

## Key Claims

- Profile A, said to be 5/5 lanes: an L0 hot window (the landed `WindowCfg`, no Merkle walk for reads at or above the tail base) and an L1 block cache of `K_page` heap lines and `K_rom` ROM lines, each opening publishing all 8 records on the bus. Line cost (ESTIMATE): 84 permutations, 5,880 rows at TOP = 61; 86 permutations, 6,020 rows at TOP = 63; "about 735 to 752.5 rows per record" when all 8 are used; repeat hits inside the leaf cost 0 rows.
- Q1: LogUp is rejected as the default because it needs a pre-pass over the whole run, gives 0% prove-while-executing overlap, cannot hand memory across a stop and resume without reloading live memory, needs a subgroup check on each `C_i` in the node (ESTIMATE +18,000 to +40,400 `RunLift` rows), and ties every segment proof to the run-wide challenge (loses GP-4 prefix reuse). A cofactor-11 torsion grinding attack on `assign_without_subgroup_check` is described. One-pass EC multiset hashes (`b2`) are rejected 5/5.
- Q3: both profiles need zero fork of `midnight-circuits` 7.3.0 and `midnight-proofs` 0.8.3 (single phase, no `Challenge`, rotations -1, 0, +1). Any new range check in `mori-recursion` must use byte chunks so the decider's `Pow2Range` table stays at 1,032,467 rows (16,103 rows headroom).
- Q4: freeze S = 16, `k_leaf = 16` for milestone M4 (SRC-014 unchanged) with `W_flush = 1`, `K_page = 4`, `K_rom = 2`; parameterize `LeafShape { s_max, w_flush, k_page, k_rom }` and benchmark S = 64 at `k_leaf = 17` after M4.
- Other proposals: a tagged DEC + RANGE fixed lookup (4 to 3 lookups, "-1 fixed column", ESTIMATE -29,062 node rows); row 6 of `Arch::Narrow` promoted to a second request row (4 bus requests per step, "+0 node rows"); a self-ranged 128-bit ALU with a 64-bit schoolbook multiply; value digest computed inline with one precomputed `(SHK_VAL, 1)` sponge prefix (-70 rows per dynamic digest); register-cached top frame and task over hash-chain stacks; 2-leaf `RunLift` (`U + U -> SEG`).
- Q1 rejects Jolt or Lasso subword ALU tables (they would add +45,000 to +90,000 node rows, ESTIMATE) and dynamic hash memoization (`AS_HASH`).
- Figures (ESTIMATE in the source): Profile A one arena at S = 16, TOP = 63: 42,100 to 45,600 leaf rows; four arenas with two shared flush units: 51,800 to 56,400 ("fits"); 1-leaf node 794,416 to 809,903; 2-leaf `RunLift` 994,820 to 1,022,000 ("`k = 20` safe"); G-C (303 steps) about 20.5 to 23.0 min serial assuming 16 steps per leaf. Profile B leaf 15,400 to 20,500 rows. The source labels the C1 cold-read figures 4,621 to 4,761 as ESTIMATE.
- Section 3 gives a ten-module plan (M1 CORE to M10 NODE INTEGRATION) with a bus key `(arena << 64) | addr`, chip ids 0 and 2 to 6, a `MemBackend` trait with one method `assign_backend`, and types `WindowBoundary`, `FlushPort`, `StateTok`; a build order in waves 0 to 4; and a whole-leaf differential harness against `sherlock-k-a/1` over two partitions.
- The study's own dissents D-1 to D-6 (not the `dissent.md` items of the same names): the study's D-1 arenas, lanes B, C, E for one arena against A, D for four, resolved to default four arenas with shared flush units until the non-interleaving check passes; the study's D-2 depth, 4 lanes for TOP = 61 against lane A and the run-length audit, resolved to TOP = 63; the study's D-3 RANGE limb width, default 12 bits; the study's D-4 rule count, 120 + 2 rules and 14 ROM tables adopted against 108 rules and 11 tables; the study's D-5, Mode 2 deferred and no `n_claims < 2^64` cap; the study's D-6, Profile B details.
- Not checked (section 5): live `cost_model` rows, cache miss rates on real workloads, the one-arena non-interleaving check, memory bandwidth at `k_leaf = 17`, owner sign-off on `dissent.md` D-1.

## Key Quotes

- On the run-length consensus: "It does NOT replace the run-length consensus" (bold in the source).
- "`DensePageBackend` (`Profile A`) is the unique one-pass, O(1)-Halt, O(1)-checkpoint memory design on this stack."

## Connections

- Brief: [[lookup-cache-brief]]. Audit: [[lookup-cache-audit]]. Lead decisions L-1 to L-10: [[final-design-decisions]].
- Builds on [[runlength-consensus]], [[runlength-consensus-audit]] and the landed C1 backend ([[c1-dense-memory-experiment]]), which the report does not cite.
- Concepts: [[BlockCache]], [[DenseTailMemory]], [[ValueDigest]], [[ChipLeaf]], [[Continuations]], [[GeneralPurposeRecursion]].

## Contradictions

Flagged by [[lookup-cache-audit]] (also on that page):
- Blocker 1: `K_page = 4`, `K_rom = 2` and 4 requests per step are frozen with no per-rule maximum, and the scalar cold read is replaced; a step that needs more lines than the leaf has is unprovable, which narrows GP-1. Lead L-2 keeps the scalar `cold_read` and does not freeze the counts.
- Blocker 2: the single precomputed prefix does not compute `sherlock-value/1` (the sponge puts the input length in its initial state); the bus re-hash passes one word of a three-word state; the reference file named does not exist; Tuple, Record and Array forms are not in `sherlock-value/1`. Lead L-3.
- Blocker 3: section 3 is not the landed C1 interface, its cited lines do not hold those names, and the key `(arena << 64) | addr` is not injective at TOP = 63; the landed key `8 addr + kind` is. Lead L-4.
- Row figures (finding 7): not of this design and do not use C1; the audit composes about 50,700 to 51,800 rows for one arena (ESTIMATE), margin about 14,000, not 19,900. The source calls the C1 cold-read rows ESTIMATE; C1 and the audit call them measured.
- Four arenas (finding 8): the source's 51,800 to 56,400 "fits" is not supported; the audit composes about 67,500 to 85,100 (ESTIMATE). The source's study D-1 resolution defaults to four arenas; lead L-9 picks one arena.
- "5/5" (finding 14) holds only for four core claims; only lane A uses TOP = 63 and cites the run-length audit, and lanes C and E keep slot-0 zeroing.
- Two-column against three-column tagged lookup (finding 6, lead L-6); ALU overflow flag dropped (finding 5, lead L-7); row 6 is not a passive replica (finding 11, lead L-8); ROM depth 32 bounds program size, against "`prog_root` handles arbitrary program size" (finding 12); SRC-025, the new value forms and `specs/quint/` do not exist (finding 13, lead L-10); the 2-leaf `RunLift` is unbuilt and has no margin against the 950,000 node gate (finding 10, lead L-10).
- Profile B: "a two-trait swap that changes zero lines" against the audit's list of changed modules (finding 9, lead L-5); "< 2^-250" is not derived; the torsion attack's loss of binding is overstated.
- Lane splits: here the arena split is lanes B, C, E against A, D; in [[runlength-consensus]] it is C, D, E against A, B. Rule count 120 + 2 here, 108 in R-6; the audit says the count source is wrong.
