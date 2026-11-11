---
title: "Lead decisions overnight 2026-11-11 (pending owner review)"
type: source
tags: [decisions, lead, pending-owner, run-length, memory]
sources: [final-design-decisions, runlength-consensus, runlength-consensus-audit, land-3x-merge]
last_updated: 2026-11-10
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/DECISIONS.md
source_url: local
---

# Lead decisions overnight 2026-11-11 (pending owner review)

## Summary

Lead decisions, not owner rulings. The owner is asleep and asked for finished, validated plans; where a choice was needed the lead took the recommended option and recorded it in `DECISIONS.md` of run `sherlock-final-design-20261111`. "The owner can reverse any of them." Entries marked OWNER (R-4 and R-5) are also questions in the morning report. Items R-1 to R-10 come from [[runlength-consensus]] and [[runlength-consensus-audit]]; O-1 records the land-3x merge ([[land-3x-merge]]).

## Key Claims

- R-1. 2^64 is the SRC-004 counter range, not a circuit size. Adopted (consensus Q1, confirmed by the audit).
- R-2. GP-1 and GP-001 text: use the audit's corrected text (audit section 5), not the consensus text, which narrows GP-001 and adds bounds the current text does not have. "Applied in night-plan step D3." (D3 here is a step of the run's night plan, not the D3 memory constraint.)
- R-3. Memory: dense tail-buffered memory with hash-chain FRAMES and TASKS stacks. The invariants of audit finding 8 become MEM module contract items and must be in the architecture and the specs.
- R-4. Tree depth: the address space must not bind before the step counter. Recommended: TOP >= 63 with 8-cell segments; measure the extra depth cost in experiment C1. OWNER: confirm TOP >= 63, or accept a memory bound below the step bound (that would narrow GP-1 and needs the owner).
- R-5. Claim counter: no `n_claims < 2^64` bound in GP-001. Recommended: count distinct claims, or carry no count; the architecture merge (B1) decides the mechanism. OWNER: confirm.
- R-6. Arenas: open. One arena (3 lanes) against four (2 lanes). The MEM interface hides the arena count; experiment C1 measures both and checks the non-interleaving claim mechanically; decide in C4.
- R-7. Continuations: tier 1 (segment tree) plus the retained-memory checkpoint mode only. Continuations compose under GP-4 and the checkpoint domain tag, not "via GP-002".
- R-8. Slot-0 zeroing does not replace write-before-read.
- R-9. D-1 (linked slices) stays open until measured (task 4.18).
- R-10. Row and throughput figures in the study are ESTIMATE and not of this design; experiment C1 measures leaf and node rows.
- O-1. Branch `land-3x` merged into main with a true merge (`e39bc7af8`) so the receipt's bound commit `2947727d6` stays valid. No task ticked: U023a, U023b and U023c are still `unimplemented.sh`, and the task 3.4 receipts and inventory flips are not written.

## Key Quotes

- "The owner is asleep and asked for finished, validated plans. Where a choice was needed, the lead took the recommended option and recorded it here."
- "The owner can reverse any of them; entries marked OWNER are also listed in the morning report as questions."

## Connections

- Inputs: [[runlength-consensus]], [[runlength-consensus-audit]], [[land-3x-merge]].
- Concepts: [[RunLengthCeiling]], [[GeneralPurposeRecursion]], [[DenseTailMemory]], [[Continuations]].
- Questions R-4, R-5, R-6: [[OpenOwnerQuestions]]. Status: [[WorkstreamStatus]].

## Contradictions

- R-6 keeps the arena count open; the audit (finding 4) would make four arenas the default until a contiguity invariant passes, and the Astra area 2 draft ([[astra-area2-memory-draft]]) recommends four arenas initially. Neither is a lead decision.
- Every item is a lead decision pending owner review, not an owner ruling. R-2 takes the audit's text, which keeps every current sentence and only appends; the R-4 alternative (a memory bound below the step bound) would narrow GP-1 and needs the owner ([[OwnerRules]], [[runlength-consensus-audit]]).
