---
title: "MPLR-016: Application authority versus project access"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-016
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-016 — Application authority versus project access

## Required behavior

When an application restricts participants or evidence issuers, Moriarty shall enforce its authenticated policy without imposing project approval on public program deployment.

## Motivation and evidence status

Hosted relay screening, contract admin roles and owner authorization operate at different boundaries. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Capability security, authorization logic, module boundaries, policy refinement. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, E, object-capability languages. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A user can require a particular counterparty signature while independent developers deploy supported programs freely.
- Reject or report unresolved: A hosted service credential or reviewer receipt becomes mandatory proof evidence.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [IK08: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/mod.rs#L24-L64).
- [IK08: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/relayer.rs#L12-L41).
- [A-keys: inspected source range](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/protocol/accounts-contracts/access-keys.mdx#L17-L55).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
