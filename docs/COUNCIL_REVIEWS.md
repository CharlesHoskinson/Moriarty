# Moriarty gate council reviews

Status: required by the user on 2026-09-04. Council execution is not yet complete.

The user requested GPT-6, Grok, and Fable 5.1 council review for each gate.
The subsequent model-update request fixes the requested identities:

| Provider | Requested model |
| --- | --- |
| OpenAI | `gpt-6-astra` |
| xAI | `grok-4.6` |
| Anthropic | `claude-fable-5-1` |

## Coverage and authority

Apply this review requirement to sprint acceptance gates and all 24 XML release gates.
Also apply it to the current implementation acceptance boundary before integration.
A shared review bundle may cover related gates.
Such a bundle must identify every covered gate and its separate acceptance evidence.
An overall verdict without gate coverage cannot close an omitted gate.

Backfill the completed S01 gates with the requested council review.
Keep their prior deterministic results and independent reviews as historical evidence.
Those results are not evidence of a three-provider council review.
No new gate closure or S02 integration may claim that review until it exists.

Council supplies advice, not proof or release authority.
Moriarty's validators still recompute the applicable artifact requirements.
Foreman owns provider dispatch, immutable evidence, lifecycle control, and integration gates.
The council must not write validator results, gate decisions, or checkpoint state.

## Required review record

Bind each review to the exact base commit, head commit, and diff content digest.
Verify the size and digest of every required artifact before dispatch.
Blind direct author, worker, model, and CLI identity in candidate evidence before prompt hashing.
Keep the identity mapping outside the reviewers' input.
Do not change substantive evidence when removing author identity.

Use Council ACE Profile 1 for the instruction contract.
Use Foreman-owned, bounded, tool-free canaries and current ready tokens.
Preserve the exact requested models. An alias or model self-report is insufficient.
Record any routed or substituted model as a mismatch, not a requested-member verdict.

Each requested member must provide a completed, identity-bound substantive verdict.
Infrastructure failures, malformed responses, missing responses, and abstentions do not count as approval.
Classify terminal transport before parsing structured advice.
Preserve every actionable dissent and its correction evidence.
Majority approval cannot override a material unresolved finding.

Use one council round per gate workstream contract.
Apply admissible corrections and deterministic re-verification under that contract.
Do not reset exhausted or terminal contracts to obtain another favorable review.
Request explicit replacement authority if the contract cannot close its findings.

The existing implementation used OpenAI-family agents.
The user nevertheless explicitly requested an Astra council member.
Use a fresh non-author Astra reviewer alongside both non-OpenAI reviewers.
Do not describe Astra as cross-family relative to an OpenAI author.
Preserve this same-family relationship in the sealed provenance record.

## Current queue

1. Confirm the three requested models through the installed provider paths.
2. Review the S02 effect foundation and its corrected execution receipts.
3. Backfill S01's completed gates with explicit per-gate council coverage.
4. Apply the same requirement to subsequent sprint and release gates.

The effect foundation is committed on `s02-model-comparison` through `1bd4bff`.
Its original process output was unavailable.
The correction labels reconstructed RED and incremental runs as retrospective.
The council must evaluate that limitation against the claimed acceptance boundary.
Neither that foundation nor its receipts establish a candidate-model or S02 gate result.

## Released-tool observation on 2026-09-05

Repository observation: the inspected Foreman checkout is
`00c342bd449948ab2ea5ca0b9d0c890614dd81d6` (PR 56 merge). Its installed-runtime
verification returned `Pass` with manifest digest
`c0350e9d6cb7a44014c68f1c8e8538af0caccf767772d0c56561eb9007f30418`.
The released Council README still states that the review coordinator and durable
runtime are not implemented. Its available runtime entry points are preflight
and specification-correctness admission, not a complete live-review coordinator.
No configured session tool supplied an additional Council review runtime.

This is an observed tooling limitation, not an approval, abstention, or a failed
Moriarty architecture. No Council review was dispatched or counted by this check.
Do not reopen the retired Foreman repair workstream or silently use its unmerged
repair candidate. Continue local Moriarty implementation and evidence collection
without integration or Council-gate claims. An operator-applied review path must
still satisfy the complete existing protocol before any result can be admitted;
the absence of a coordinator does not waive exact identity, ready tokens,
terminal transport, immutable artifacts, or dissent requirements.

The common-model review bundle must cover the accumulated observation, policy,
signing, execution, rejection, and installment units, not merely the historical
effect foundation. It must preserve each unit's narrower evidence limits.
Candidate A–D selection and S01 backfill remain separate acceptance obligations.
