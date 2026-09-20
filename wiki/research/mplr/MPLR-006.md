---
title: "MPLR-006: Authenticated asynchronous outcomes"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-006
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-006 — Authenticated asynchronous outcomes

## Required behavior

When evidence resumes a stage, Moriarty shall authenticate its origin, correlation and declared finality, and enforce the specified consumption rule.

## Motivation and evidence status

Receipt IDs, returned values and detached calls have different authority and completion meaning. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Affine evidence tokens, capability security, event provenance, replay-safe protocols. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Move, Rust, Motoko. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A duplicated delivery is harmless or rejected without duplicate financial effects.
- Reject or report unresolved: An attacker substitutes a success flag or callback from another transaction.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR01: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/core/primitives/src/transaction.rs#L320-L343).
- [AA-fanout: inspected source range](https://github.com/near/mpc/blob/3499a2a44207df89699c9791e21623613a365a16/crates/contract/src/api/sign.rs#L165-L223).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
