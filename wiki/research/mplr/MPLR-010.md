---
title: "MPLR-010: Time finality and unresolved outcomes"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-010
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-010 — Time finality and unresolved outcomes

## Required behavior

When a deadline or observation boundary is reached, Moriarty shall classify what is known and apply the signed expiry policy without treating missing evidence as proof of nonexecution.

## Motivation and evidence status

Timeout, receipt failure, source finality and destination settlement are different facts. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Temporal logic, epistemic logic, distributed failure detectors, timed session types. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, Scilla, TLA+/Quint models. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: An expired request retains unresolved external duties until reliable evidence establishes the relevant delivery, cancellation, nonexecution or recovery outcome; any other residual duties remain.
- Reject or report unresolved: A late successful transfer is followed by a refund justified only by timeout.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR21: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/chain/chain/src/chain.rs#L3150-L3185).
- [AR21: inspected source range](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/protocol/transactions/transaction-execution.mdx#L85-L89).
- [AA-refunddeadline: inspected source range](https://docs.near-intents.org/api-reference/oneclick/request-a-swap-quote).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

2026-09-19 review correction: established successful delivery resolves its corresponding uncertainty; expiry does not force known-success facts back to unknown.

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.

## Simplicity refinement — 2026-09-19

Simplicity documentation distinguishes absolute height/time from relative distance/duration and records deprecated relative-lock jets. Require typed temporal domains, units and version-specific enforcement; a temporal comparison in the program and ledger finality are separate obligations. See [source-based findings](../simplicity/security-findings.md).
