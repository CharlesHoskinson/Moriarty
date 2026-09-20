---
title: "AI solvers and the DeFi kernel"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, ows, x402, ai-solvers]
---

# AI solvers and the DeFi kernel

Research design, 2026-09-19. User direction adopts OWS interoperability and x402 payment workflows as first-class solver interfaces. This is not an implementation or proof claim. Moriarty remains a permissionless language compiled for Midnight ZKIRv3.

OWS supplies a common wallet/signing interface and local storage conventions. x402 supplies payment negotiation, authorization and settlement interfaces. Neither determines whether a proposed financial action realizes the user's complete formal intention. The Moriarty program states that intention; the kernel coordinates evidence and external effects under its constraints.

An AI solver can discover services, request quotes, choose routes, propose fills, purchase data or computation and submit candidate plans. Its optimizer can be probabilistic or computationally unbounded off chain. Acceptance remains a bounded, specified relation over explicit evidence. Solver output and retrieved documents are untrusted inputs; neither changes the owner's authority policy.

The owner delegates a bounded capability: purposes, assets, recipients or permitted classes, chains, fees, aggregate spending, expiry, allowed subdelegation, service predicates, disclosure policy and recovery rules. Not every stage needs another owner signature when the standing policy covers it. The language imposes no project approval or registered-solver membership prerequisite. Optional providers can impose service rules without turning them into public language admission.

OWS interoperability must not export an unrestricted fund-control key to an adversarial solver. Its current agent token plus disk can decrypt wallet secrets outside policy evaluation; owner credentials bypass policies. The kernel must document where the real restriction is enforced: destination contract, constrained account, or isolated signer/federation under explicit assumptions. A bare external threshold signature retains its corruption premise. An OWS-compatible remote signing bridge is proposed work, not current OWS core support for MPC or Midnight.

The adapter must interpret the exact bytes it signs. Unsupported or incomplete chain-effect decoding cannot silently become a proof of a spend cap. Compare the canonical signed intention, decoded material effects and actual target transaction. OWS's current partial effect extraction and placeholder cumulative spending are evidence for this obligation. Required custom policy data must fail closed when absent; schema evolution must not weaken an existing policy unnoticed.

x402 authorization, facilitator verification, submission and settlement are separate judgments. Schemes have different custody, reservation, timing and replay assumptions. Bind scheme version, network, asset, payee, amount bounds, validity, request, service predicate and evidence policy. Treat a facilitator response as its specified evidence, not unconditional chain finality. Preserve unresolved capture, withdrawal, void and delivery states. Service delivery and payment finality are independent unless a specified exchange protocol connects them.

Reserve budget before concurrent work and preserve it across retries, crashes and failover. Spent value, pending authorization, gas sponsorship, fees and residual liabilities need explicit accounting. A zero-valued local spending context cannot enforce an aggregate budget. Distinguish a new purchase from retrieval of the same paid result. An already-settled authorization submitted by another caller needs reconciliation under the signed policy, without payment duplication or unauthorized repeated service grants.

For a conditional purchase, record the request, reserve the authorized budget, gather the stated document/result/recipient evidence and execute the allowed payment and delivery stages. An unknown outcome retains duties. A timeout does not prove nonpayment or nonexecution. A refund uses still-controlled funds; compensation is a new authorized effect. The program can express partial fulfillment and persistent continuations without pretending all chains share one atomic transaction.

ZK establishes a specified relation, MPC distributes signing or computation, and TEE attestation supplies measured execution evidence under hardware assumptions. PCD must bind legitimate history, scoped authority, complete effects and remaining duties to the actual target relation. All evidence must refer to the same domain, stage, program/version and effect. Privacy, availability, external truth and finality retain separate assumptions. The NEAR, Daml, Simplicity and x402 sources motivate this architecture; they do not prove Moriarty's compiler or kernel.

## New MPLR candidates

- MPLR-031 — Delegated solver capabilities: acceptance shall enforce the owner's bounded delegation across wallet, agent and signer boundaries. Reject token possession as permission for arbitrary signing. Accept another unregistered solver using a valid capability.
- MPLR-032 — Concurrent service budgets: authorization shall reserve and account for spent, pending and residual amounts across concurrent tasks, retries and fee sponsorship. Reject two independently permitted purchases whose combined commitments exceed the budget.
- MPLR-033 — Payment and service completion: the program shall distinguish payment finality, result availability and delivery to the authorized recipient. Reject a payment receipt or TEE-vault result as proof of recipient delivery without the required evidence.
- MPLR-034 — Logical paid-request identity: retry and recovery shall preserve one authenticated logical obligation while distinguishing repeated result retrieval from new billable work. Reject duplicate spending or new service grants justified solely by replayed transport data.

Refine MPLR-014/018/028/030 with exact-byte effect decoding, policy migration and versioned adapter profiles. These candidates require source-grounded notes, hostile/positive traces, a modeled lifecycle, proof obligations and actual Midnight correspondence before implementation acceptance.

[Study index](index.md)
