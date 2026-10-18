---
title: "MPLR-012: Simulation and assurance status"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-012
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-012 — Simulation and assurance status

## Required behavior

When a tool simulates or reports a workflow, it shall state which stages, calls and proof judgments were covered and keep prediction separate from accepted execution.

## Motivation and evidence status

Simulation may skip downstream AuthCall/withdrawal effects or return only a local engine result. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Abstract interpretation, trace refinement, effect summaries, proof-producing analysis. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Koka, Liquid Haskell, Aeon. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A local preview visibly reports unresolved external stages and does not claim settlement.
- Reject or report unresolved: An omitted external call is displayed as successfully executed or proved.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AI17: inspected source range](https://docs.near-intents.org/integration/verifier-contract/intent-types-and-execution).
- [AI17: inspected source range](https://docs.near-intents.org/integration/verifier-contract/introduction).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
