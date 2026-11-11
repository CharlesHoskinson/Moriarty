---
title: "Run-length ceiling (D3 against GP-1)"
type: concept
tags: [run-length, gp-1, d3, lead-decision]
sources: [c6-outcome-ledger9-review, dissent-c6-section, design-2-7-chip-leaf, general-purpose-definition, quint-chipleaf-pairrow-m3, chip-spike-experiments, runlength-consensus-brief, gp-architecture, runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area2-memory-draft]
last_updated: 2026-11-10
---

# Run-length ceiling (D3 against GP-1)

The conflict between the proposed chip leaf memory (D3) and the owner's GP-1. The run-length study has answered it ([[runlength-consensus]], audited in [[runlength-consensus-audit]]), and the lead adopted a direction pending owner review ([[final-design-decisions]]). GP-1 is not narrowed; two parts wait on the owner (R-4, R-5).

## What 2^64 is

- 2^64 is the range of the step counter (the height field `h`) that every node proof carries. SRC-004 makes the node unsatisfiable unless `prev_h`, `steps` and `h = prev_h + steps` are below 2^64, so the count cannot wrap in the proof field back to 0 (genesis) ([[runlength-consensus]], confirmed by [[runlength-consensus-audit]]). Lead decision R-1: "2^64 is the SRC-004 counter range, not a circuit size" ([[final-design-decisions]]).
- It is not a circuit size and not a length a run must reach: no circuit has 2^64 rows, and no circuit domain, key, proof size or verification cost grows with the run length. A run of N steps folds fixed-size leaf proofs through the fixed-size node and seals once in the decider, so proving work grows linearly with N ([[runlength-consensus-audit]] section 5, [[runlength-consensus-brief]]).
- Proving 2^64 steps on this host: ESTIMATE about 2.5 x 10^12 years ([[runlength-consensus]]). Tested run lengths are evidence (SAS-001), not limits of GP-1 ([[runlength-consensus-audit]]).

The owner also said, 2026-11-11: "2^64 is physically impossible" ([[runlength-consensus-brief]]); the study agrees that proving 2^64 steps is physically impossible on this host. This answers the owner's question of the same day, "I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit" ([[runlength-consensus-brief]]): it is a limit on a counter, and nothing of that size is calculated.

## The D3 ceiling

- Under D3, `base / 8 < 2^17`, every leaf takes one segment (a zero-append leaf too), and the node requires at least one step per leaf. So a run has fewer than 2^17 leaves and 2^20 cells ([[c6-outcome-ledger9-review]]; [[chip-spike-experiments]]; [[quint-chipleaf-pairrow-m3]]).
- The C6 decision does not repair it; the prover can only fail to prove a long run, not change the outcome ([[c6-outcome-ledger9-review]], [[dissent-c6-section]]).
- All five study lanes reject keeping D3 and chaining runs through GP-2, and reject widening D3 while it still pads at leaf close ([[runlength-consensus]]).

## The adopted direction (lead, pending owner)

- R-3: dense tail-buffered memory with hash-chain FRAMES and TASKS stacks, which removes leaf-close padding and the per-leaf segment burn. See [[DenseTailMemory]].
- R-4 (OWNER): the address space must not bind before the step counter. With up to 3 appends per step (unverified), `TOP = 61` can run out before 2^64 steps (audit blocker 3). Recommended TOP >= 63; or the owner accepts a memory bound below the step bound, which would narrow GP-1.
- R-5 (OWNER): no `n_claims < 2^64` bound in GP-001, since an occurrence count in a shared DAG reaches 2^64 at depth 64 with about 128 proofs (audit blocker 2).
- R-7: long runs stop and resume through tier 1 and the retained-memory mode ([[Continuations]]).
- R-2: GP-1 and GP-001 text uses the audit's corrected text ([[GeneralPurposeRecursion]]).

## Contradictions

- [[runlength-consensus]] proposes bounding heap cells and claims below 2^64 in GP-1 and GP-001; [[runlength-consensus-audit]] shows both narrow GP-1 or GP-4. The lead adopted the audit (R-2, R-4, R-5).
- The consensus says D3 falsifies SRC-023; the audit says it only blocks the `det` premise at digest level (finding 10).
- The GP architecture study treats fixed Merkle depths as a capacity fixed by the relation; Astra held that this is still a narrowing ([[gp-architecture]]). The area 2 draft asks for a capacity theorem or a wider tree before freeze ([[astra-area2-memory-draft]]).

## Status

Answered by the study and audit; lead decisions R-1 to R-5 and R-7 pending owner review. On [[OpenOwnerQuestions]] as items 1, R-4 and R-5.

See also [[GeneralPurposeRecursion]], [[ChipLeaf]].
