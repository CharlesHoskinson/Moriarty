---
title: "MPLR-019: Consent to introduction of obligations"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-019 — Consent to introduction of obligations

## Required behavior

When a transition creates or increases a party's enforceable financial or operational obligation, it shall establish that party's consent to the obligation and its material terms, either directly or through an applicable prior authorization policy. A transfer to an address alone shall not manufacture such consent.

This is a distinct obligation-formation obligation refining the broader stage-authority requirement, not a requirement that every passive positive-value receipt needs recipient interaction.

## Evidence and motivation

The [Daml authorization tutorial](https://docs.canton.network/appdev/modules/m3-authorization), sections Prevent IOU revocation, Propose-Accept and role contracts, explains why authority is required before making the owner a signatory, including the negative-amount example. The [2023 author paper](https://arxiv.org/abs/2303.03749), visually inspected page 4, provides the offer-accept precedent. These sources motivate the requirement; they do not prove a Moriarty implementation or imply off-chain legal enforceability.

## Acceptance witnesses

- Accept: a recipient accepts a fixed obligation, or a valid standing policy authorizes the precise bounded class of obligations.
- Reject: a sender introduces a debt, guarantee, fee obligation or performance duty for the recipient by merely transferring a contract/address reference.
- Preserve: a plain positive-value receipt with no added duty may follow the application's existing authorization policy.

## Theory research

Investigate obligation and authorization logics, deontic/resource distinctions, effect types and signed contract formation. Compare Daml, capability languages and financial DSLs using the same counterexample. Distinguish asset ownership, issuer liability, receiver duty and user-visible consent.

## Midnight obligations and status

Prove that the accepted source-level transition preserves the consent judgment through ZKIRv3 lowering, including stage identity, policy version, amount, asset, expiry and evidence binding. Concrete syntax and proof representation remain open. Research-draft; no implementation or theorem accepted. Created during initial Daml intake on 2026-09-19.
