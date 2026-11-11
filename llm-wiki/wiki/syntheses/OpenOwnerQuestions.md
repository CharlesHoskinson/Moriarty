---
title: "Open owner questions"
type: synthesis
tags: [open-questions]
sources: [dissent-c6-section, c6-outcome-ledger9-review, design-2-7-chip-leaf, general-purpose-definition, runlength-consensus-brief, lookup-cache-brief, gp-architecture, runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area2-memory-draft, astra-area3-recursion-draft, lookup-cache-consensus, lookup-cache-audit, c1-dense-memory-experiment, rule-maxima-experiment, sherlock-architecture, arch-audit-astra, arch-audit-response]
last_updated: 2026-11-11
---

# Open owner questions

Questions the owner must settle. Keep each until a source settles it. See [[WorkstreamStatus]]. The OD numbers are the open decisions of [[sherlock-architecture]] section 4; except R-4 and R-5, the OWNER marks there were added by the merge (lead labels, not owner words).

| # | Question | State | Pages |
|---|---|---|---|
| 1 | D3 run ceiling against GP-1 | Answered by the study and audit; lead decisions R-1 to R-3 and R-7 pending owner review; residual parts are R-4 and R-5 | [[RunLengthCeiling]], [[final-design-decisions]] |
| 2 | D-1 SI-12 to SI-14 under C = 8 (OD-5) | Open; no form measured; the architecture proposes contiguous dense slices as a third candidate, X6 measures; OWNER for the status change | [[dissent-c6-section]], [[ChipLeaf]] |
| 3 | D-2 type identity in a constructor digest (OD-14) | Open, OWNER | [[ValueDigest]], [[dissent-c6-section]] |
| 4 | D-6 leaf partition in the GP-4 shape commitment (N-7) | Open, against GPR-016; the architecture binds intervals, disclosure stays open | [[dissent-c6-section]], [[gp-architecture]] |
| 5 | Exhaust kind 7 against SVM-009 (OD-16, N-12) | Open conflict, OWNER; DEC refuses kind 7 under `/1` meanwhile | [[ValueDigest]], [[dissent-c6-section]] |
| 6 | Init mechanism for an input under the new digest (OD-17) | Open, OWNER; kit INIT owns it; lead recommends by-digest admission with an empty genesis memory; X11 pending | [[ValueDigest]], [[c6-outcome-ledger9-review]] |
| R-4 | Tree depth: TOP >= 63, or accept a memory bound below the step bound | Open, OWNER (OD-1). TOP = 63 now backed by M-2 (at most 3 appends per step); holds only if one record is one physical cell (A-5, OD-11); the lower-bound option is no longer offered as an acceptance path (A-6) | [[DenseTailMemory]], [[final-design-decisions]] |
| R-5 | Claim counter: no `n_claims < 2^64` bound in GP-001 | Open, OWNER (OD-2). The merge chose no count in any public statement; `set_root` binds the DAG; OWNER to confirm | [[GeneralPurposeRecursion]], [[final-design-decisions]] |
| R-6 | Arenas: one tagged arena or four | Lead decision L-9 (one arena), pending owner review; the non-interleaving check is done and holds (M-1) | [[DenseTailMemory]], [[final-design-decisions]] |
| L-2 | Cache-line counts per leaf (`K_page`, `K_rom`) and keeping the scalar `cold_read` | Lead decision, pending owner review; per-rule table done for heap reads (`k_page + n_cold >= 4`, M-3), `K_rom` open (X15) | [[BlockCache]], [[final-design-decisions]] |
| L-9 | Arena decision: one arena | Lead decision, pending owner review; non-interleaving check done (M-1, OD-4 decided by the lead) | [[DenseTailMemory]], [[final-design-decisions]] |
| OD-18 | One-arena Init field list in SRC-021 (one `(len, root, tail_d)` in place of four `(root, length)` pairs) | Open, OWNER; lead recommends amending SRC-021 after OD-17 | [[ModularArchitecture]], [[sherlock-architecture]] |
| OD-20 | Scope of GP-2: input consumption at the seal only, or also a mid-run consume rule; bind the dependency at genesis or at the seal | Open, OWNER, first among the new owner questions; lead built path is input consumption, "the lead's reading, not a settled fact" | [[GeneralPurposeRecursion]], [[arch-audit-response]] |
| OD-21 | The normative semantic reference for the full language | Open, OWNER; lead recommends a separate evaluator over `moriarty-sherlock-nominal/1`; X13 sizes it | [[ModularArchitecture]], [[sherlock-architecture]] |

Other items the architecture marks OWNER without a row here: OD-6 (only if S changes), OD-10 (only if a rule is to stay out), OD-13 and OD-19 (only if X10 fails and a new leaf statement is needed), OD-22 (only if a second accumulator per SRS is wanted), N-6 (GP-4 inline work if X7 fails), N-11 (contract surface, SRC-007 per-program contract). See [[sherlock-architecture]].

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

Update (OD-17): the architecture adds kit INIT as the one owner and a contract any form must meet (I1 to I6: no scan, no input-size bound, every used node authenticated against `val_d`, outcome equal to the reference run of `apply(prog, x)`, canonical genesis memory, one `val_d` for shared copies). Candidates: (a) a loading prefix, which needs a scan or counted steps that change `n_steps`; (b) by-digest admission with an empty genesis memory (lead recommendation); (c) an input constant table, which is (b) with a hint. X11 measures. Still open, OWNER ([[sherlock-architecture]], [[ModularArchitecture]]).

## R-4. Tree depth

With at most 3 appends per step (unverified), `TOP = 61` (`2^64` cells) can run out before 2^64 steps (audit blocker 3). Lead recommendation: size the tree so cells >= appends per step x 2^64 (TOP >= 63 with 8-cell segments) and measure the extra depth cost in experiment C1. OWNER: confirm TOP >= 63, or accept a memory bound below the step bound, which would narrow GP-1 and needs the owner ([[final-design-decisions]], [[runlength-consensus-audit]]).

Update (OD-1): C1 measured TOP = 63 at +280 rows per flush and +140 per cold read (E-1); X1 measured at most 3 appended cells per step (M-2), so TOP = 63 (2^66 cells) is enough. The condition is `len_0 + a e 2^64 <= 2^(TOP+3) - 1`: it holds at TOP = 63 only with one physical cell per record; two would need TOP = 64 (A-5, OD-11). The architecture no longer offers the lower memory bound as an acceptance path; it could be taken only by an explicit owner decision that amends GP-1 (A-6) ([[sherlock-architecture]], [[rule-maxima-experiment]]). Still OWNER: confirm.

## R-5. Claim counter

With occurrence counts summed through joins, a shared DAG reaches 2^64 at depth 64 with about 128 proofs, which would limit GP-4's DAGs of any depth (audit blocker 2). Lead recommendation: count distinct claims, or carry no count in the public statement; the architecture merge (B1) decides the mechanism. OWNER: confirm ([[final-design-decisions]], [[gp-architecture]]).

Update (OD-2): the merge chose option (c): no count in the run or collection statement (`[reg_root, set_root, shape_d]`); `set_root` binds the claim DAG; a telemetry count, if wanted, counts distinct claims, is not range-checked and carries no soundness. To decide: a read of every consumer of `n_claims` and a Quint check of a diamond DAG deeper than 64. The audit agrees that dropping the counter avoids the limit ([[sherlock-architecture]], [[arch-audit-astra]]). Still OWNER: confirm.

## R-6. Arenas

One arena (3 lanes) against four arenas (2 lanes) in the run-length study. The one-arena variant depends on a 108-rule non-interleaving claim that is a code reading. Lead R-6: the MEM interface hides the arena count; experiment C1 measures both and checks the claim mechanically; decide in C4. C1 measured 12,281 backend rows for one arena against 49,172 for four (E-3). The lead has since taken L-9 (one arena); see below. Not marked OWNER in the decision record ([[final-design-decisions]], [[DenseTailMemory]]).

## L-2. Cache-line count

The lookup consensus froze `K_page = 4` and `K_rom = 2` lines per leaf at S = 16 and replaced the scalar cold read ([[lookup-cache-consensus]]). The audit (blocker 1) says a step that needs more lines than the leaf has is unprovable, which narrows GP-1, and asks for a per-rule table (bus requests, distinct old segments, distinct ROM blocks per table, appends, stack spills) before any count is frozen ([[lookup-cache-audit]]). Lead L-2: the scalar `cold_read` stays; an isolated old read costs 4,761 rows with the scalar walk against 6,020 with a line (ESTIMATE); `K_page` and `K_rom` are not frozen until the table is measured. A lead decision pending owner review, not marked OWNER in the decision record ([[final-design-decisions]], [[BlockCache]]).

## L-9. Arena decision

Lead L-9: one arena. Four arenas with 4 + 2 cache lines do not fit `k_leaf <= 16` (about 67,500 to 85,100 rows, ESTIMATE); one arena is about 50,700 to 51,800 (ESTIMATE). The non-interleaving check is still required before the spec freezes this ([[final-design-decisions]]). Against it: the lookup audit keeps the arena count open until C4 (finding 8, item 28), the run-length audit would make four arenas the default meanwhile, and the lookup consensus defaults to four ([[lookup-cache-audit]], [[runlength-consensus-audit]], [[lookup-cache-consensus]]). A lead decision pending owner review, not marked OWNER in the decision record.

Update (M-1, OD-4): X2 found the non-interleaving property to hold for all 120 enabled rules (premises S1 to S4 mechanical, 0 violations over 9,031 slices; the induction a code reading), so one arena stands; the architecture marks OD-4 decided and not OWNER. Still a lead decision pending owner review ([[rule-maxima-experiment]], [[sherlock-architecture]]).

## OD-18. One-arena Init field list

SRC-021 and the Init table of the governing design name 39 fields with four `(root, length)` pairs; L-9 and M-1 decide one arena. Option (a), recommended: amend SRC-021 and the Init table to the one-arena list of kit INIT (one `(len, root, tail_d)`, the two stack constants in place of `NIL` links, `input_d` and `deps_d` as header words); option (b) keeps four roots against L-9. A spec edit after OD-17. OWNER ([[sherlock-architecture]], [[final-design-decisions]] A-3).

## OD-20. Mid-run claim consumption and the scope of GP-2

Option (a): input consumption only; a consumer takes a producer's Halted value as its input, checked by the input link at the seal; no owner-machine rule. Option (b): add a consume rule to the profile, with the `deps_d` chain. The lead builds (a) and specifies the circuit side of (b), so (b) needs no new node relation. The lead's reading is not settled: Init field 39 (`deps`) and SRC-021 ("the dependency claims" authenticated at the first segment) suggest more was contemplated. If the owner reads GP-2 as requiring (b), or the dependency bound at genesis, that is required work of this change, not a narrowing. OWNER ([[sherlock-architecture]], [[arch-audit-response]], [[final-design-decisions]] A-2).

## OD-21. Normative semantic reference

Options: (a) extend the dedicated `sherlock-k-a/1` evaluator `reference.rs`; (b) a separate full-language evaluator over `moriarty-sherlock-nominal/1`, the dedicated grammar unchanged (lead recommendation); (c) declare the owner machine its own reference, which would compare the owner machine with itself for exactly the forms GP-001 names. It decides what "the reference machine" is for GP-1's "every program of the Sherlock language". X13 first. OWNER ([[sherlock-architecture]], [[final-design-decisions]] A-4).
