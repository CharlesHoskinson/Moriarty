# Context

**Consolidated schedule, 2026-09-19:** This file retains detailed requirements and historical planning. The [single U0–U7 roadmap](../../../ROADMAP.md) and [consolidated proposal](../consolidated-language-kernel/proposal.md) control current design and scheduling. Resolve former P/C/K owners through [traceability](../consolidated-language-kernel/traceability.md); do not dispatch this as a separate queue. Objective requirements and evidence remain in force.


The September 19 user direction defines Moriarty as a language for all Midnight DeFi developers. The APSS literature study informs boundaries; it does not import another protocol's guarantees.

# Goals / Non-Goals

Goals: open deployment of supported programs, explicit formal intent, complete effects, compositional proof obligations and inspectable assumptions.

Non-goals for this documentation change: implement new proof circuits, deploy contracts, guarantee arbitrary natural-language intent, import Aeon code, or mandate a managed solver service.

# Decisions

The [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md) defines the architecture. Its proof relation binds exact program, semantics, intent, predecessor, observations and effects. Public acceptance depends on objective evidence rather than author identity or project records. Owner authority remains mandatory.

Application UX formalizes constraints; permission supplies bounded cryptographic authority; solvers propose candidate executions; settlement verifies effects under native ledger rules. Moriarty supplies programmable semantics and proof interfaces across those boundaries. It is not identical to all four services.

Aeon contributes design ideas: refinement obligations, trust visibility, counterexamples and typed holes. Advisory checks preserve exact checked arithmetic and cannot replace runtime or ledger validation. Counterexample replay is part of the first checker. Synthesis follows a usefulness comparison and cannot weaken intent.

# Risks / Trade-offs

General supported-program proof feasibility and correspondence remain open. External facts require explicit assumptions. Private composition needs its own statements. Economic optimality, liquidity and inclusion require separate evidence. This change removes administrative approval of programs without weakening proof predicates.

# Migration Plan

Preserve prior roadmap and receipts. Correct active documents and label historical evidence. Add clean external-developer scenarios. Do not mark proof, K correspondence or ledger tasks complete from documentation review.

# Open Questions

Which complete supported Core relation fits the Midnight verifier budget? Which recursion construction preserves history privately? Which extension interface supports new financial modules without unchecked FFI? APSS research and the six-person PL review must resolve or retain these questions explicitly.

## Reviewed architecture

The [six-expert consensus](../../../deliverables/aeon-study-2026-09-19/review/FINAL-CONSENSUS.md) and its exact [revision 4](../../../deliverables/aeon-study-2026-09-19/review/CONSENSUS-CANDIDATE-v4.md) supply the detailed compiler, certified basis, APSS and advisory-tool obligations. Six endorsements are design evidence only. The [PR17 applicability audit](../../../deliverables/aeon-study-2026-09-19/PR17-APPLICABILITY.md) records the producer/witness premises and target-version limits.
