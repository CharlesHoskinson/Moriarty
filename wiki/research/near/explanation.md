---
title: "NEAR architecture and Moriarty implications"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, near-teardown, research]
---

# Four layers with different guarantees

NEAR's native runtime executes transactions and receipts, maintains state and schedules asynchronous continuations. Chain Signatures adds MPC signing under a configured policy. The Intents verifier interprets a defined intent algebra and checks authorization and token accounting. Hosted1Click and solver services coordinate quoting, routing and delivery. These layers have different failure, timing, administrative and evidence boundaries; see the six source-linked studies.

For Moriarty, the reusable ideas are explicit signed interfaces, staged execution, typed continuations, outcome accounting and open candidate construction. Native signature/accounting checks are not proofs that a general program fulfills its authenticated formal intention. Moriarty needs its own general semantics and correspondence to Midnight ZKIRv3.

Partial transactions preserve each committed prefix and every remaining obligation. Joins distinguish available results from successful outcomes. A failed receipt or timeout says only what the supported evidence establishes; it cannot erase other completed branches. Refunds consume still-controlled assets or enforceable claims; compensation is a new action with its own authority.

Conditional settlement is the user's more specific requirement: submit a request to a destination but withhold delivery until the specified combination of signatures, documents, proofs, recipient actions and other conditions holds. Funding an escrow or recording the request may happen earlier. The inspected escrow-swap contract offers a narrower funded/partial-fill precedent, not a general documentary-evidence language.

MPLR-001..018 separate these required behaviors from research choices such as session types, resource/authorization logic, compensation calculi and proof-relevant evidence. Those techniques remain hypotheses to test against primary literature and language implementations. [Sources and coverage](reference.md).
