---
diataxis: explanation
id: apss.applications.explanation
title: Applications built with a permissionless financial language
status: draft
documentation_type: explanation
type: research
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Applications built with a permissionless financial language

Moriarty should make financial programs expressible and provable on Midnight. An application built with it can be a lending market, vault, exchange, payment agreement, conditional order, private asset or an intent router. No single application architecture should define who is allowed to use the language. This is a design recommendation grounded in the literature below, not a claim that these applications already compile or settle through Moriarty.

## The language is not a fixed catalog

Compositional financial languages address a problem that product catalogs cannot: developers will want a new combination of rights, obligations and contingencies. The foundational combinator work makes this motivation explicit. Its historical valuation model is not a ready-made execution kernel, but the language-versus-catalog distinction remains valuable. Required ACTUS and DeFi examples should test expressive coverage without becoming an exhaustive list of permitted programs. [APP02](notes/APP02.md)

A restricted supported semantic domain is compatible with permissionless development. A finite language may reject recursion, unsupported arithmetic or an unproved effect. It must not reject a program merely because its author has not obtained a maintainer review. Conversely, a new programming construct needs a defined semantic/proof meaning; “permissionless” does not make undefined foreign effects safe. This is our architectural inference, not a property established for Moriarty by the cited systems.

## Three meanings of intention

An application has a developer's specification: conservation rules, allowed debt transitions, auction rules or a pool invariant. A user has an authorization and outcome constraint: permitted asset debit, recipient, fees, deadline, liabilities and required receipt. A solver supplies a concrete plan. These objects overlap but are not interchangeable.

Anoma's resource-model explanation distinguishes desired objects from predicates describing acceptable objects. That supports leaving genuine choice to a solver. Resource logic adds a useful separation between shared machine rules and custom application rules. Neither lets a proof establish an unstated preference. [APP07](notes/APP07.md), [APP08](notes/APP08.md)

For Moriarty, a proof should connect the selected plan's complete observable effects to both the relevant program rules and authenticated user constraints. A proof that a swap preserves pool reserves is not proof that the recipient received enough after fees. A valid signature is authority for its exact statement, not a proof of the application invariant. A natural-language intention must be translated into a specification that the person can inspect; formal verification does not solve that translation automatically.

## Application autonomy and technical validation

CAKE's application/permission/solver/settlement decomposition is useful for understanding services. It does not require every language user to adopt one managed router. The public language interface should let developers build applications with distinct wallets, discovery systems and proposal engines while retaining common proof and ledger rules. [APP01](notes/APP01.md)

A current interoperability example reinforces the distinction. ERC-7683 describes a resolver interface and explicit assumptions without standardizing one settlement contract. Its trust model is therefore not itself a settlement correctness proof. A solver may decline a resolver, but that decision should not become global authority over who may deploy a Moriarty program. [APP12](notes/APP12.md)

The important boundary is what the acceptance relation checks, not which service discovered the candidate. Program commitments, verifier/statement bindings, authority and effects are objective inputs. Reviewer-provider identities and project campaign approvals are internal engineering records. Applications may have user-selected governance rules, but the language project must not silently become their mandatory governor.

## Completion, progress and financial obligation

A payment obligation and a currently spendable balance are different kinds of state. A partial action can legitimately advance one while leaving the other outstanding. ERC-7540 supplies a concrete asynchronous application model in which requests become claimable before users actually claim them. Treating those stages as one successful payment loses both ownership and recovery information. [APP11](notes/APP11.md)

Marlowe's timeout property illustrates another distinction: the existence of a closing transaction does not mean the network submits it automatically. An enabled remedy is a semantic fact; funded submission, witness availability and finality are additional conditions. A refund can return escrow while independent debt remains. [APP03](notes/APP03.md)

Findel provides a counterexample to the belief that combining familiar financial constructs automatically creates the intended bargain. Who controls a continuation can matter as much as the amounts in it. Application tests should include reward collection without reciprocal performance, optional continuation refusal and funding failure. [APP04](notes/APP04.md)

## Composability needs effect contracts

Applications compose when their assumptions and effects fit together. Merely sharing a token interface is not enough. Resource-oriented verification work examines collaborating contracts and adversarial external calls; its transferable lesson is to expose the resources and effects on which a module's reasoning depends. It is not a recommendation to import Ethereum re-entrancy into Moriarty. [APP05](notes/APP05.md)

Two application documents provide especially practical distinctions. Conditional-order handlers can generate fresh order identifiers while continuing to spend one allowance; an identifier alone does not establish renewed authority. An AMM's price oracle can help generate promising orders while a separate reserve predicate decides validity. These motivate cumulative authority accounting and a strict separation between proposal quality and acceptance. [APP09](notes/APP09.md), [APP10](notes/APP10.md)

For a first language implementation, same-domain atomic composition and asynchronous multi-transaction composition should have distinct judgments. This recommendation does not assume that local rollback can reverse a completed transaction elsewhere. Shared-state conflicts, obligations and resource budgets must remain explicit across both.

## Privacy changes application design

Private computation literature demonstrates why financial correctness and disclosure need separate statements. Zexe's application discussion distinguishes confidentiality of trade details from anonymity of participants and acknowledges assumptions about communication. A validity proof cannot hide data already sent to a solver or prover. [APP06](notes/APP06.md)

A Moriarty application should describe who receives each witness, which values are public, what metadata remains visible, and who can continue if a participant disappears. A private exchange and an auditable lending market may choose different disclosures using the same language. No universal key custodian or identity service follows from that requirement.

## Counter-position and unresolved questions

A narrow managed service can be easier to support than a general language: one application constrains interfaces, liquidity sources and recovery. That is a legitimate optional product, but it cannot satisfy the stated permissionless-language objective by itself. Likewise, a library of reviewed templates may improve usability without becoming an exclusive deployment catalog.

The literature does not resolve Moriarty's exact supported proof relation, circuit cost, source-to-ledger correspondence or private witness availability. It motivates concrete research questions:

1. What minimal property/effect language covers new developer programs while keeping proof generation bounded?
2. Which application restrictions are universal invariants, and which must remain user or contract policy?
3. Can a module declare a compositional effect summary that is proved against actual code and remains sound across versions?
4. How do pending requests and partial fills carry liabilities, unused authority and closure resources without accidental reset?
5. What information must a solver see, and can a private successor be produced without predecessor secrets?
6. Can users understand the canonical specification well enough to reject a formally valid but economically wrong bargain?

This is a 12-source first pass. It is not an exhaustive survey, current deployment audit or a recommendation to add foreign backends. The [reference](reference.md), [design audit guide](how-to.md) and [conceptual exercise](tutorial.md) serve different purposes; all remain draft material for separately reviewed vault ingestion.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
