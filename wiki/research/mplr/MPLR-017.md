---
title: "MPLR-017: Complete staged financial accounting"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-017
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-017 — Complete staged financial accounting

## Required behavior

When a stage is accepted, its authenticated effect frame shall account for exact assets and custody, authorized supply changes, fees and separately typed liability evolution.

## Motivation and evidence status

Internal token-delta balance, external custody and residual economic duties are not interchangeable quantities. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Resource separation algebras, nominal asset types, linear accounting, invariant composition. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Move, Cadence, Daml, Marlowe. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Every asset transfer and liability change has its own well-typed conservation/evolution equation.
- Reject or report unresolved: Authority units are added to refund amounts, or internal zero-sum balance is called external settlement.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [IK12: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/state.rs#L20-L336).
- [IK12: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/tokens/nep141/withdraw.rs#L54-L226).
- [AR26: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L529-L540).
- [AR26: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L860-L904).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
