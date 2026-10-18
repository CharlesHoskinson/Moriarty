---
title: "MPLR-013: Private evidence and continuations"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-013
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-013 — Private evidence and continuations

## Required behavior

Where evidence or workflow state is private, Moriarty shall define what each participant can observe and how later authorized stages obtain the witness needed to continue.

## Motivation and evidence status

Documents, signatures, receipt correlations and multi-party continuation state may reveal sensitive facts. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Information-flow control, noninterference, zero-knowledge relations, secure session types. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml/Canton, Zexe-style systems, Jif. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Prove a required document property without disclosing fields outside the authorized view.
- Reject or report unresolved: A public status or continuation identifier leaks a property claimed confidential.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AA-status: inspected source range](https://docs.near-intents.org/integration/distribution-channels/1click-api/quickstart/making-a-request).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.

## Simplicity refinement — 2026-09-19

Simplicity witnesses are publicly disclosed on confirmed transactions according to its witness documentation. Commitment/pruned branches are not equivalent to a general zero-knowledge confidentiality guarantee. State the target observation model independently. See [source-based findings](../simplicity/security-findings.md).
