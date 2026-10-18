---
title: "MPLR-004: Partial fulfillment and residual duties"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-004
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-004 — Partial fulfillment and residual duties

## Required behavior

When a signed policy permits partial fulfillment, each accepted fill shall satisfy its fill rules and preserve cumulative bounds, remaining authority and residual obligations.

## Motivation and evidence status

A completed portion of a swap or payment is not the entire economic obligation. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Linear logic, resource algebras, quantitative types, financial contract semantics. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Move, Cadence, Marlowe, Daml. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Two valid partial payments reduce the liability by exactly their authorized discharges.
- Reject or report unresolved: Splitting a fill bypasses a cumulative fee bound or erases remaining debt.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AI13: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/state.rs#L43-L89).
- [AI13: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/fund.rs#L11-L33).
- [AA-types: inspected source range](https://docs.near-intents.org/api-reference/oneclick/request-a-swap-quote).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
