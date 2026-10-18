---
title: "MPLR-014: Certified lowering of staged effects"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-014
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-014 — Certified lowering of staged effects

## Required behavior

When a staged program is compiled, correspondence shall preserve its permitted traces, conditions, rejection behavior and costs in the pinned Midnight ZKIRv3 execution model.

## Motivation and evidence status

NEAR receipt semantics cannot be assumed to match Midnight guaranteed/fallible phases. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Compiler correctness, observational refinement, certified primitives, effect preservation. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: CompCert, CakeML, Simplicity. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A target trace is related to a permitted source trace with every retained effect accounted for.
- Reject or report unresolved: A correct witness generator is presented as proof excluding invalid satisfying witnesses.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR31: inspected source range](https://github.com/near/NEPs/blob/dcc0bcbd452f68c936696e5f9552e567c3b162e0/neps/nep-0509.md#L13-L24).
- [AR31: inspected source range](https://github.com/near/NEPs/blob/dcc0bcbd452f68c936696e5f9552e567c3b162e0/neps/nep-0509.md#L104-L111).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
