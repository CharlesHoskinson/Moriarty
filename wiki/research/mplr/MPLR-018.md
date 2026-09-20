---
title: "MPLR-018: Inspectable conditional intent"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-018
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-018 — Inspectable conditional intent

## Required behavior

When a participant authorizes a conditional workflow, the signing and reporting interface shall expose its canonical conditions, destination, evidence policy, allowed partial outcomes and recovery rules.

## Motivation and evidence status

A quote, signature request or pending status can hide distinctions important to the user’s actual intention. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Bidirectional transformations, contract elaboration, semantic UI correspondence, typed provenance. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, Marlowe, projectional language tools. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: The displayed request and proved canonical object bind the same documents, recipient consent, deadline and remedies.
- Reject or report unresolved: The UI promises automatic refund while the signed policy permits an unresolved or compensating outcome.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AA-quotecheck: inspected source range](https://github.com/defuse-protocol/sdk-monorepo/blob/b6bab503e09027b40df3df14ca3610a11ff3d40d/packages/internal-utils/src/solverRelay/getQuote.ts#L37-L139).
- [AA-refunddeadline: inspected source range](https://docs.near-intents.org/api-reference/oneclick/request-a-swap-quote).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.

## Simplicity refinement — 2026-09-19

The txmanifest documentation warns descriptions are not verified by the format. Bind displayed machine-checkable intent/effects to the signed formal statement; do not treat wallet prose as a proof. See [source-based findings](../simplicity/security-findings.md).
