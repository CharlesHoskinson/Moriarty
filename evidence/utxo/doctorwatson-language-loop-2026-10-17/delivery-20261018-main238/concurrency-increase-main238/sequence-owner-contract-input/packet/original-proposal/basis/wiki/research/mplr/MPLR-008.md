---
title: "MPLR-008: Stage-specific authority and consent"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
mplr: MPLR-008
tags: [moriarty, mplr, pl-theory, near-teardown]
---
# MPLR-008 — Stage-specific authority and consent

## Required behavior

When a stage executes, it shall use only the authority and consent granted for that stage under the signed lifecycle, revocation and expiry policy.

## Motivation and evidence status

Callbacks do not freshly consume the originating access-key nonce; delegated workflow authority must be explicit. This requirement is a user-directed design draft informed by the NEAR teardown, not a claim of implemented Moriarty behavior. [NEAR source](https://github.com/near/nearcore/tree/a47cf412bf55b020421e99e5fc5b15ed970a7a61), [Intents source](https://github.com/near/intents/tree/32a7836f825e8c984c26149f4456793ec7e3d49a), and the [research log](index.md) anchor the investigation. Exact per-requirement source ranges and expert dispositions are consolidated in the teardown evidence; repository-wide links alone do not discharge a claim.

## Theory questions for later deep research

Investigate Authorization logic, capability attenuation, session fidelity, temporal capabilities. Which judgment represents this behavior? What assumptions does its soundness theorem need? What counterexample separates candidate approaches? What is preserved under composition and compilation to Midnight ZKIRv3?

Candidate language studies: Daml, E, Move. These are research leads, not assertions that those languages already solve the requirement or are directly compatible with Midnight.

## Acceptance examples

- Required: Recipient consent is bound to the exact request, amount, asset, conditions and allowed amendment version.
- Reject or report unresolved: A callback gains the full original signer authority or changes the recipient silently.

## Open design choices

Concrete syntax, static versus dynamic obligations, proof representation, completeness limits and target cost remain open. The behavioral requirement must survive a change of implementation technique. Preserve declared external assumptions and distinguish recorded request/funding, condition fulfillment and final delivery.

## Status history

2026-09-19: drafted during NEAR teardown. Await source-level review and comparative PL research; no implementation or theorem accepted.

## NEAR teardown evidence pointers

- [AR28: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/function_call.rs#L148-L189).
- [AR28: inspected source range](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/near-vm-runner/src/logic/context.rs#L11-L64).
- [A-keys: inspected source range](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/protocol/accounts-contracts/access-keys.mdx#L17-L55).

These source facts motivate the research problem; they do not prove the proposed Moriarty semantics. See the [NEAR study](../near/index.md).

[OpenSpec/EARS traceability](../../../openspec/changes/partial-and-conditional-transactions/requirements.md) records the planned behavior. Theory selection remains open.

## Daml refinement — 2026-09-19

The [nontransitive-authority example](https://docs.canton.network/appdev/modules/m3-authorization) motivates checking authority at nested action boundaries, not only at top-level stages. Investigate a judgment that prevents a helper from retaining unrelated outer-party authority and constrains standing delegation by its bound policy. This is a research refinement, not automatic adoption of Daml's exact rules. [MPLR-019](MPLR-019.md) separately covers consent to new obligations.
