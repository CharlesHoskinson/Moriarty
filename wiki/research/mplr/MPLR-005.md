---
title: "MPLR-005: Refunds and compensation"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-005
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-005 — Refunds and compensation

## Required behavior

When a settlement step fails or remains unresolved, recovery shall distinguish a refund of still-controlled assets from a new compensating action after effects have committed.

## Motivation and evidence status

A failed downstream promise may not imply that transferred value is available for refund. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Sagas, compensating calculi, exception/effect semantics, recovery invariants. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, Scilla, E. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A recovery transition spends only assets or claims it actually controls and retains unrecovered duties.
- Reject or report unresolved: Compensation rewrites history or issues a second refund after external delivery.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [IK06: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/tokens/nep141/withdraw.rs#L54-L226).
- [AI14: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/fill.rs#L91-L124).
- [AI14: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/state.rs#L266-L295).
- [AR29: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L1283-L1435).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
