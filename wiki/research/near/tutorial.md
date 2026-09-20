---
title: "A conditional transfer through its lifecycle"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: tutorial
tags: [moriarty, near-teardown, research]
---

# Worked conceptual example

This is a proposed Moriarty semantic example, not executable syntax or a shipped interface.

Alice proposes delivery of50 units of an exact asset to Bob. The signed request requires Bob's signature on that request, an invoice matching a committed document and content predicate, and a valid proof of the named financial constraint. It fixes deadlines, evidence issuers, fees, permitted partial delivery and recovery rules.

First, the request is recorded. If this is the funded variant, a separate accepted stage reserves the value under the declared custody rule. Bob has not yet received final delivery. The state contains the request identity, condition policy, reserve, consumed/remaining authority and pending duties.

Next, evidence arrives. A document hash identifies bytes; the content predicate, issuer authority and proof statement still need their specified validation. Bob's signature must bind this request, destination, amount and conditions. Duplicated or unrelated evidence must not release value. Evidence validity is evaluated at the stage boundary required by the signed policy.

Once the required combination is valid, an authorized release stage may deliver the50 units. Its proof accounts for fees, custody changes, fulfilled obligations and remaining duties. A subsequent receipt or external delivery must not be assumed successful from local dispatch alone.

If evidence expires or a later delivery remains unresolved, the request follows its declared recovery policy. Still-controlled assets may be refundable; already-delivered assets cannot be refunded by pretending history rolled back. Compensation, if authorized, is a new action. Reliable evidence of actual success resolves the corresponding unknown outcome; other residual duties remain.

Exercise: change the request to two25-unit fills. Define whether each fill needs a fresh recipient acceptance, how cumulative fees are bounded, and what remains if only one fill completes. These are questions for MPLR-003/004/005/008/010/017, not assumptions hidden in a callback implementation.
