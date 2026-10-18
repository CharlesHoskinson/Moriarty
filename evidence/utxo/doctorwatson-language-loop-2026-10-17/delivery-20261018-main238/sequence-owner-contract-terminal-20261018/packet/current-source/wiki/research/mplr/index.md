---
title: Moriarty Programming Language Requirements theory log
type: moc
status: active
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, mplr, pl-theory]
---
# MPLR theory log

MPLRs are stable research requirements for Moriarty language behavior. They separate the required capability from candidate PL techniques and from actual implementation evidence. IDs are never recycled; revisions retain their history. A research-draft entry is not a proved theorem or completed feature. User intent controls behavior: permissionless Midnight DeFi programs compile to ZKIRv3 and carry proofs of authenticated formal intention, including partial transactions and conditional settlement.

The initial entries were drafted during the NEAR teardown. The bounded Daml comparative study is complete; its source bank and research direction are [here](../daml/index.md). Future research should seek primary PL literature, experimental language results and deployed-language semantics, including counterexamples and known limitations. A language name in a note is a lead to investigate, not evidence of feature equivalence.

## Requirements

| ID | Required behavior | Status |
|---|---|---|
| [MPLR-001](MPLR-001.md) | Staged and partial transaction semantics | research-draft |
| [MPLR-002](MPLR-002.md) | Typed persistent continuations | research-draft |
| [MPLR-003](MPLR-003.md) | Conditional settlement with composable evidence | research-draft |
| [MPLR-004](MPLR-004.md) | Partial fulfillment and residual duties | research-draft |
| [MPLR-005](MPLR-005.md) | Refunds and compensation | research-draft |
| [MPLR-006](MPLR-006.md) | Authenticated asynchronous outcomes | research-draft |
| [MPLR-007](MPLR-007.md) | Joins and late results | research-draft |
| [MPLR-008](MPLR-008.md) | Stage-specific authority and consent | research-draft |
| [MPLR-009](MPLR-009.md) | Cumulative work fees and reserves | research-draft |
| [MPLR-010](MPLR-010.md) | Time finality and unresolved outcomes | research-draft |
| [MPLR-011](MPLR-011.md) | Causality concurrency and interference | research-draft |
| [MPLR-012](MPLR-012.md) | Simulation and assurance status | research-draft |
| [MPLR-013](MPLR-013.md) | Private evidence and continuations | research-draft |
| [MPLR-014](MPLR-014.md) | Certified lowering of staged effects | research-draft |
| [MPLR-015](MPLR-015.md) | Open programmable financial workflows | research-draft |
| [MPLR-016](MPLR-016.md) | Application authority versus project access | research-draft |
| [MPLR-017](MPLR-017.md) | Complete staged financial accounting | research-draft |
| [MPLR-018](MPLR-018.md) | Inspectable conditional intent | research-draft |
| [MPLR-019](MPLR-019.md) | Consent to introduction of obligations | research-draft |
| [MPLR-020](MPLR-020.md) | Certified primitive substitution | research-draft |
| [MPLR-021](MPLR-021.md) | Bounded resource certificates | research-draft |
| [MPLR-022](MPLR-022.md) | Program and evidence commitment separation | research-draft |
| [MPLR-023](MPLR-023.md) | Mandatory conditions constrain acceptance | research-draft |
| [MPLR-024](MPLR-024.md) | Explicit settlement domains | research-draft |
| [MPLR-025](MPLR-025.md) | Netting preserves gross economics | research-draft |
| [MPLR-026](MPLR-026.md) | Behavioral contracts for settlement implementations | research-draft |
| [MPLR-027](MPLR-027.md) | Authenticated origins and recursive compliance | research-draft |
| [MPLR-028](MPLR-028.md) | Consent-preserving semantic evolution | research-draft |
| [MPLR-029](MPLR-029.md) | Authenticated completeness of private state | research-draft |
| [MPLR-030](MPLR-030.md) | Common evidence statement and federation trust | research-draft |
| [MPLR-031](MPLR-031.md) | Delegated solver capabilities | research-draft |
| [MPLR-032](MPLR-032.md) | Concurrent service budgets | research-draft |
| [MPLR-033](MPLR-033.md) | Payment and service completion | research-draft |
| [MPLR-034](MPLR-034.md) | Logical paid-request identity | research-draft |

| [MPLR-035](MPLR-035.md) | Constraint-preserving solver completion | research-draft |

## Filing and research workflow

For each MPLR: retain motivating source and exact version; state the behavioral requirement and positive/negative examples; compare competing theory approaches; distinguish observed behavior from inference; formulate proof obligations and target assumptions; map accepted behavior to OpenSpec/EARS and a bounded implementation package. Record disagreements and evidence that would change the decision. Never close a requirement from a model vote alone.

## Theory log

2026-09-19 — Created MPLR-001..018 from the NEAR teardown and the user’s explicit partial/conditional transaction requirements. Conditional settlement means submission to a destination whose delivery waits for a specified combination of signatures, documents, proofs, recipient actions and other supported conditions. Escrow funding and request recording are separate from final settlement. The initial theory techniques remain open for later research.

2026-09-19 — Initial Daml intake adds MPLR-019 and a scoped-authority research refinement to MPLR-008. Full documentation study, examples and target proofs remain open.

2026-09-19 — Simplicity study adds MPLR-020..023; explicit target, witness, resource and acceptance proof obligations remain open.

2026-09-19 — Completed bounded Daml source study; added MPLR-024..030 and federated-kernel security direction. Full corpus capture does not imply full semantic reading; implementation/proof obligations remain open.

2026-09-19 — OWS/x402 study adds MPLR-031..034 for first-class AI solvers. Wallet interoperability and payment protocol support remain distinct from proven intention and target correspondence.

2026-09-19 — Anoma study adds MPLR-035 and a source-backed refinement matrix for existing requirements. Candidate incompleteness, committed partial stages, proof aggregation, historical PCD and duty completeness remain distinct. [Study](../anoma/index.md).

2026-09-19 — [Mina recursion case study](../mina/index.md) refines MPLR-020/022/023/027. [Consolidated traceability](../../../openspec/changes/consolidated-language-kernel/traceability.md) maps all35 existing IDs to the single roadmap; no ID is recycled or closed by design review.
