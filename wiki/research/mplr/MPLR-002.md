---
title: "MPLR-002: Typed persistent continuations"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-002
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-002 — Typed persistent continuations

## Required behavior

When execution suspends for a later event, Moriarty shall persist a typed continuation bound to the program, predecessor, awaited events, authority and outstanding duties.

## Motivation and evidence status

NEAR promises and callbacks resume across transactions without a fresh original-user signature. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Session types, typestate, affine capabilities, defunctionalization, continuation typing. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: E, Koka, Rust session-type libraries. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Resume a valid continuation once with correctly typed evidence and the signed stage policy.
- Reject or report unresolved: Replay or attach an unrelated callback to consume reserved authority twice.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR30: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L1460-L1515).
- [AR30: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L1625-L1675).
- [AA-fanout: inspected source range](https://github.com/near/mpc/blob/3499a2a44207df89699c9791e21623613a365a16/crates/contract/src/api/sign.rs#L165-L223).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
