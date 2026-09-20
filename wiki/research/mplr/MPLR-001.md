---
title: "MPLR-001: Staged and partial transaction semantics"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-001
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-001 — Staged and partial transaction semantics

## Required behavior

When a supported transaction spans several settlement steps, Moriarty shall represent each committed step and the outstanding transaction state explicitly.

## Motivation and evidence status

A local transaction succeeds while a later receipt or withdrawal fails. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Trace semantics, small-step operational semantics, refinement of distributed histories. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, Scilla, Motoko. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Prove each accepted prefix preserves authority, accounting and outstanding duties; do not label a successful prefix complete.
- Reject or report unresolved: An accepted debit followed by an unresolved withdrawal is reported as fully settled.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR26: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L529-L540).
- [AR26: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L860-L904).
- [IK12: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/state.rs#L20-L336).
- [IK12: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/tokens/nep141/withdraw.rs#L54-L226).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
