---
title: "Lead decisions overnight 2026-11-11 (pending owner review)"
type: source
tags: [decisions, lead, pending-owner, run-length, memory, lookup, cache]
sources: [final-design-decisions, runlength-consensus, runlength-consensus-audit, land-3x-merge, c1-dense-memory-experiment, lookup-cache-consensus, lookup-cache-audit]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/DECISIONS.md
source_url: local
---

# Lead decisions overnight 2026-11-11 (pending owner review)

## Summary

Lead decisions, not owner rulings. The owner is asleep and asked for finished, validated plans; where a choice was needed the lead took the recommended option and recorded it in `DECISIONS.md` of run `sherlock-final-design-20261111`. "The owner can reverse any of them." Entries marked OWNER (R-4 and R-5) are also questions in the morning report. Items R-1 to R-10 come from [[runlength-consensus]] and [[runlength-consensus-audit]]; O-1 records the land-3x merge ([[land-3x-merge]]); E-1 to E-3 come from experiment C1 ([[c1-dense-memory-experiment]]); L-1 to L-10 come from [[lookup-cache-consensus]] and [[lookup-cache-audit]]. Only R-4 and R-5 are marked OWNER in the file; the wiki lists L-2 and L-9 as pending owner review too ([[OpenOwnerQuestions]]).

## Key Claims

- R-1. 2^64 is the SRC-004 counter range, not a circuit size. Adopted (consensus Q1, confirmed by the audit).
- R-2. GP-1 and GP-001 text: use the audit's corrected text (audit section 5), not the consensus text, which narrows GP-001 and adds bounds the current text does not have. "Applied in night-plan step D3." (D3 here is a step of the run's night plan, not the D3 memory constraint.)
- R-3. Memory: dense tail-buffered memory with hash-chain FRAMES and TASKS stacks. The invariants of audit finding 8 become MEM module contract items and must be in the architecture and the specs.
- R-4. Tree depth: the address space must not bind before the step counter. Recommended: TOP >= 63 with 8-cell segments; measure the extra depth cost in experiment C1. OWNER: confirm TOP >= 63, or accept a memory bound below the step bound (that would narrow GP-1 and needs the owner).
- R-5. Claim counter: no `n_claims < 2^64` bound in GP-001. Recommended: count distinct claims, or carry no count; the architecture merge (B1) decides the mechanism. OWNER: confirm.
- R-6. Arenas: open. One arena (3 lanes) against four (2 lanes). The MEM interface hides the arena count; experiment C1 measures both and checks the non-interleaving claim mechanically; decide in C4. (Later taken by L-9: one arena.)
- R-7. Continuations: tier 1 (segment tree) plus the retained-memory checkpoint mode only. Continuations compose under GP-4 and the checkpoint domain tag, not "via GP-002".
- R-8. Slot-0 zeroing does not replace write-before-read.
- R-9. D-1 (linked slices) stays open until measured (task 4.18).
- R-10. Row and throughput figures in the study are ESTIMATE and not of this design; experiment C1 measures leaf and node rows.
- O-1. Branch `land-3x` merged into main with a true merge (`e39bc7af8`) so the receipt's bound commit `2947727d6` stays valid. No task ticked: U023a, U023b and U023c are still `unimplemented.sh`, and the task 3.4 receipts and inventory flips are not written.

From experiment C1 (`EXPERIMENTS.md`):
- E-1. TOP = 63 is cheap: measured +280 rows per flush and +140 per old-memory read, no new column, no change of k in any leaf size (run `20261111T071045Z-c1-rows`). Supports R-4; still OWNER to confirm.
- E-2. Domain tags in the sponge capacity, not the first input: same separation at 0 extra permutations; first-input tags would cost about 2x on every tree node (ESTIMATE). Accepted for the prototype; the C1 Quint lane checks equivalence.
- E-3. Arenas lean to one arena: one arena with one flush unit 12,281 backend rows, four arenas 49,172 (measured). At S = 16 four arenas leave room for only 2 old-memory reads (or 6 with shared flush units, not built). Final choice in C4 after the non-interleaving check.

From the lookup-cache study and its audit:
- L-1. Memory, PROG and type tables stay on the dense tail-buffered Merkle design (R-3), plus a small number of 8-record cache lines per leaf opened against the pre-state root and served on the bus (Profile A). One-pass, Poseidon boundary digests; run-wide write-once LogUp does not replace it. The ten block-cache rules of audit section 5.1 become MEM / PAGE contract items ([[BlockCache]]).
- L-2. The scalar `cold_read` stays: an isolated old read costs less with the scalar walk (4,761 rows) than with a line (6,020, ESTIMATE). `K_page` and `K_rom` are not frozen until the per-rule maximum table is measured; a step must never need more lines than a leaf has (GP-1).
- L-3. Value digest: the study's single precomputed sponge prefix is rejected; one constant per preimage length is needed. VALUE DIGEST computes exactly `sherlock-value/1`; no 2-input bus re-hash.
- L-4. The landed C1 interface is the MEM interface (`membackend.rs`, `dense.rs`, key `8 addr + kind`); the study's section 3 interface is not adopted.
- L-5. Profile B (ZmLogUp + Horner) is benchmark-only behind the seam, not built before milestone M4; it changes the leaf statement, node, seal and continuation and replaces the `digest_inj` premise.
- L-6. Tagged DEC + RANGE lookup: the three-column form (about 22,600 node rows saved, ESTIMATE); the two-column form is not adopted.
- L-7. The ALU keeps the overflow flag in its tuple, so overflow (`Rejected(10)`) stays provable; ops are only the machine's ops.
- L-8. Row 6 promotion (4 bus requests per step) needs a new layout plan; not adopted until planned and measured.
- L-9. Arenas (R-6, E-3): one arena. Four arenas with 4 + 2 cache lines do not fit `k_leaf <= 16` (about 67,500 to 85,100 rows, ESTIMATE); one arena is about 50,700 to 51,800 (ESTIMATE). The non-interleaving check is still required before the spec freezes this.
- L-10. Items the study invented are dropped: `SRC-025`, `Value::Tuple/Record/Array`, `specs/quint/`. The 2-leaf `RunLift` is unbuilt and not part of the design until measured.

## Key Quotes

- "The owner is asleep and asked for finished, validated plans. Where a choice was needed, the lead took the recommended option and recorded it here."
- "The owner can reverse any of them; entries marked OWNER are also listed in the morning report as questions."

## Connections

- Inputs: [[runlength-consensus]], [[runlength-consensus-audit]], [[land-3x-merge]].
- Concepts: [[RunLengthCeiling]], [[GeneralPurposeRecursion]], [[DenseTailMemory]], [[Continuations]].
- Lookup cache: [[lookup-cache-consensus]], [[lookup-cache-audit]], [[BlockCache]], [[ValueDigest]]. Experiment C1: [[c1-dense-memory-experiment]].
- Questions R-4, R-5, R-6, L-2, L-9: [[OpenOwnerQuestions]]. Status: [[WorkstreamStatus]].

## Contradictions

- Arenas: R-6 kept the count open for C4 and E-3 says the final choice is in C4; L-9 now picks one arena before C4 and before the non-interleaving check. The lookup audit (finding 8, item 28) keeps the count open until C4; the run-length audit (finding 4) would make four arenas the default until a contiguity invariant passes; the lookup consensus defaults to four arenas with shared flush units; the Astra area 2 draft ([[astra-area2-memory-draft]]) recommends four arenas initially ([[lookup-cache-audit]], [[lookup-cache-consensus]]).
- L-2 keeps the scalar `cold_read`; the lookup consensus (P2) replaces it with block lines.
- The study's own dissents D-1 to D-6 reuse the names of `dissent.md` D-1 and D-2; R-9's D-1 is the `dissent.md` item, not the study's arena dissent.
- Every item is a lead decision pending owner review, not an owner ruling. R-2 takes the audit's text, which keeps every current sentence and only appends; the R-4 alternative (a memory bound below the step bound) would narrow GP-1 and needs the owner ([[OwnerRules]], [[runlength-consensus-audit]]).
