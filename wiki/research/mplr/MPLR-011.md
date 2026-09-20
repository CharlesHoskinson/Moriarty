---
title: "MPLR-011: Causality concurrency and interference"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-011
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-011 — Causality concurrency and interference

## Required behavior

When a suspended workflow resumes amid other activity, its proof shall preserve predecessor causality and revalidate the relevant state and authority under its signed policy.

## Motivation and evidence status

Other calls may change balances, keys, policy or custody between asynchronous stages. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Rely-guarantee reasoning, separation logic, serializability, conflict-aware refinement. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Iris-backed languages, Rust, Move. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Concurrent valid workflows cannot consume the same reserve or invalidate each other silently.
- Reject or report unresolved: A simulation snapshot is reused after its relevant balance or authority changed.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AA-builder: inspected source range](https://github.com/defuse-protocol/sdk-monorepo/blob/b6bab503e09027b40df3df14ca3610a11ff3d40d/packages/intents-sdk/src/intents/intent-payload-builder.ts#L84-L240).
- [AR30: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L1460-L1515).
- [AR30: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L1625-L1675).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
