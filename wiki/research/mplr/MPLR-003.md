---
title: "MPLR-003: Conditional settlement with composable evidence"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-003
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-003 — Conditional settlement with composable evidence

## Required behavior

When a submitted transaction specifies settlement conditions, Moriarty shall withhold the specified delivery until the required combination of authenticated signatures, documentary evidence, proofs, recipient actions and other supported predicates is satisfied.

## Motivation and evidence status

User definition: submit to an address; settlement waits for the configured combination of conditions. Funding or recording may precede final delivery. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Refinement/dependent types, proof-relevant predicates, evidence algebra, authorization logic. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, Marlowe, Scilla, Simplicity. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: A funded request remains pending until its bound recipient signature, document predicate and proof all validate at the policy-defined settlement point.
- Reject or report unresolved: A document hash alone, an unrelated signature, or an expired proof releases the funds.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

Terminology: **conditional settlement with composable evidence requirements** is the proposed main name; **contingent settlement** remains the user’s alias. **Programmable escrow** identifies the funded/locked variant. Evidence may combine conjunction, alternatives or thresholds only as explicitly specified. A document hash establishes identity, not truth; document acceptance needs a bound predicate, trusted attestation or proof. See [XRPL escrow](https://xrpl.org/docs/concepts/payment-types/escrow) and [Daml propose/accept](https://docs.digitalasset.com/build/3.4/sdlc-howtos/smart-contracts/develop/patterns/propose-accept.html) as different, narrower precedents.

## NEAR teardown evidence pointers

- [AI13: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/state.rs#L43-L89).
- [AI13: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/fund.rs#L11-L33).
- [AI15: inspected source range](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/auth_call.rs#L20-L44).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.
