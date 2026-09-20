---
title: "MPLR-009: Cumulative work fees and reserves"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-009
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-009 — Cumulative work fees and reserves

## Required behavior

When execution spans stages, Moriarty shall account for cumulative costs, reserved resources and permitted retained fees under the signed bounds and target phase rules.

## Motivation and evidence status

Receipt execution and failure can retain fees, rewards or storage consequences. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Amortized resource analysis, graded effects, quantitative type theory. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: AARA languages, Resource Aware ML, Move. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A failed stage records retained fees and remaining reserves without exceeding its approved envelope.
- Reject or report unresolved: Retrying or splitting stages resets the total budget.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR26: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L529-L540).
- [AR26: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L860-L904).
- [AR29: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs#L1283-L1435).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.

## Whole-language review research refinement — 2026-09-19

The whole-language review reproduces local origination with remaining work2 and closure reserve16, followed by insufficient-work repayment. This matches the local contract; no deployed loss is demonstrated. Research a separately authorized reserve-consuming closure/recovery rule or admission viability condition. Preserve obligations and consumed authority, and state liveness assumptions. Do not repair this by indiscriminately making reserved work spendable.

[Review evidence](../language-design-review/reference.md). This adds an open research experiment, not implemented behavior or a changed executable profile.
