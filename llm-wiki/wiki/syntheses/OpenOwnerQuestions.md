---
title: "Open owner questions"
type: synthesis
tags: [open-questions]
sources: [dissent-c6-section, c6-outcome-ledger9-review, design-2-7-chip-leaf, general-purpose-definition, runlength-consensus-brief, lookup-cache-brief, gp-architecture, runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area2-memory-draft, astra-area3-recursion-draft]
last_updated: 2026-11-10
---

# Open owner questions

Questions the owner must settle. Keep each until a source settles it. See [[WorkstreamStatus]].

| # | Question | State | Pages |
|---|---|---|---|
| 1 | D3 run ceiling against GP-1 | Answered by the study and audit; lead decisions R-1 to R-3 and R-7 pending owner review; residual parts are R-4 and R-5 | [[RunLengthCeiling]], [[final-design-decisions]] |
| 2 | D-1 SI-12 to SI-14 under C = 8 | Open; no form measured | [[dissent-c6-section]], [[ChipLeaf]] |
| 3 | D-2 type identity in a constructor digest | Open | [[ValueDigest]], [[dissent-c6-section]] |
| 4 | D-6 leaf partition in the GP-4 shape commitment | Open, against GPR-016 | [[dissent-c6-section]], [[gp-architecture]] |
| 5 | Exhaust kind 7 against SVM-009 | Open conflict | [[ValueDigest]], [[dissent-c6-section]] |
| 6 | Init mechanism for an input under the new digest | Open | [[ValueDigest]], [[c6-outcome-ledger9-review]] |
| R-4 | Tree depth: TOP >= 63, or accept a memory bound below the step bound | Open, OWNER (lead recommends TOP >= 63) | [[DenseTailMemory]], [[final-design-decisions]] |
| R-5 | Claim counter: no `n_claims < 2^64` bound in GP-001 | Open, OWNER (lead recommends distinct count or no count) | [[GeneralPurposeRecursion]], [[final-design-decisions]] |
| R-6 | Arenas: one tagged arena or four | Open; experiment C1 measures both, decide in C4 | [[DenseTailMemory]], [[final-design-decisions]] |

## 1. The D3 run ceiling against GP-1

- Fact: under D3 a run has fewer than 2^17 leaves and 2^20 cells. GP-1 says "The run length is bounded only by the height field (below 2^64)" ([[dissent-c6-section]], [[general-purpose-definition]]).
- Owner, 2026-11-11: "I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit" ([[runlength-consensus-brief]]). Answer from the study, confirmed by the audit: 2^64 is the range of the SRC-004 step counter, not a circuit size; no circuit has 2^64 rows ([[runlength-consensus]], [[runlength-consensus-audit]]). Lead decision R-1 ([[final-design-decisions]]).
- The dissent file's question "which change to the memory design removes the ceiling" is answered by a direction: dense tail-buffered memory with hash-chain stacks (lead decision R-3), continuations through tier 1 and the retained-memory mode (R-7), GP-1 and GP-001 text from the audit (R-2). These are lead decisions pending owner review, not owner rulings, so the question stays on this page until the owner confirms ([[RunLengthCeiling]], [[DenseTailMemory]], [[Continuations]]).
- GP-1 is not narrowed. "A narrowing needs an explicit owner decision" ([[general-purpose-definition]], [[OwnerRules]]).

## 2. D-1 SI-12 to SI-14

Dense logical identifiers (Astra), addresses as references (Fable), or linked slices (Carbon). The candidate to measure first is Carbon's linked-slice form, Astra's mapping the alternative (task 4.18). SI-12 blocks D3 adoption for the C6 machine, not the publication decision ([[dissent-c6-section]]). The run-length consensus rejects the linked-slice form from a code reading; the audit says D-1 needs the measurement first, and the lead keeps D-1 open until measured (R-9, [[final-design-decisions]]). The dense memory design is a third candidate form ([[runlength-consensus-audit]]).

## 3. D-2 type identity

The verdict takes the program-independent `ty_d`; Astra's dissent (program-scoped identity) stays. The normal form of `decl_schema_d` must be fixed before any key is frozen ([[dissent-c6-section]]).

## 4. D-6 leaf partition in the shape commitment

Carbon: bind only the inter-claim DAG publicly. Astra: the shape may differ between proofs of the same claims. The GP study binds segment bracketing into `shape_d`. It is a decision on GPR-016. Also open: whether the "local statement" in `shape_d` may contain `pre_d` or `post_d` ([[dissent-c6-section]], [[gp-architecture]]). The Astra area 3 draft recommends binding address-free execution intervals and bracketing, with an explicit address-free preimage, and keeps the disclosure choice open ([[astra-area3-recursion-draft]]).

## 5. Exhaust kind 7

SRC-022 lists `Exhausted` kind 7 = `LogicalCap`; SVM-009 refuses exhaust reason 7 in profile `/1`. The dissent record does not reconcile the two ([[dissent-c6-section]]). The Astra drafts keep the profile rejection of kind 7 until the specification resolves it ([[astra-area2-memory-draft]], [[astra-area3-recursion-draft]]).

## 6. Init mechanism

How `val_d` of an input relates to the initial memory (lazy unfold or load steps) is open. It must not scan the input in the Init check ([[c6-outcome-ledger9-review]], [[design-2-7-chip-leaf]]). The run-length consensus claimed its mode 2 continuation closes this; the audit strikes that claim (finding 5), so it stays open. The area 2 draft says input loading and lazy unfolding must use the value digest checks and leaves the choice to a reference-step accounting proof ([[astra-area2-memory-draft]]).

## R-4. Tree depth

With at most 3 appends per step (unverified), `TOP = 61` (`2^64` cells) can run out before 2^64 steps (audit blocker 3). Lead recommendation: size the tree so cells >= appends per step x 2^64 (TOP >= 63 with 8-cell segments) and measure the extra depth cost in experiment C1. OWNER: confirm TOP >= 63, or accept a memory bound below the step bound, which would narrow GP-1 and needs the owner ([[final-design-decisions]], [[runlength-consensus-audit]]).

## R-5. Claim counter

With occurrence counts summed through joins, a shared DAG reaches 2^64 at depth 64 with about 128 proofs, which would limit GP-4's DAGs of any depth (audit blocker 2). Lead recommendation: count distinct claims, or carry no count in the public statement; the architecture merge (B1) decides the mechanism. OWNER: confirm ([[final-design-decisions]], [[gp-architecture]]).

## R-6. Arenas

One arena (3 lanes) against four arenas (2 lanes). The one-arena variant depends on a 108-rule non-interleaving claim that is a code reading. Lead: the MEM interface hides the arena count; experiment C1 measures both and checks the claim mechanically; decide in C4. Not marked OWNER in the decision record ([[final-design-decisions]], [[DenseTailMemory]]).
