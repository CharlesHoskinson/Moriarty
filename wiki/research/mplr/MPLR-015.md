---
title: "MPLR-015: Open programmable financial workflows"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-015
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-015 — Open programmable financial workflows

## Required behavior

When a developer composes supported constructs into a new workflow, validation shall use the versioned language and proof rules rather than a catalog of approved example programs.

## Motivation and evidence status

A fixed Intents enum is a useful application protocol but not the identity of a general DeFi language. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Algebraic effects, extensible DSLs, modular semantics, refinement interfaces. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, Koka, Aeon, Marlowe. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A newly authored conditional workflow passes the ordinary public pipeline.
- Reject or report unresolved: A valid program is rejected because it is absent from a project registry.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [IK01: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/engine/mod.rs#L32-L118).
- [IK01: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/state.rs#L20-L336).
- [AI15: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/auth_call.rs#L20-L44).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
