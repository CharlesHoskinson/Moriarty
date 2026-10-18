---
title: "MPLR-033: Payment and service completion"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, ows, x402, ai-solvers]
---

# MPLR-033: Payment and service completion

## Required behavior

When a service is purchased, the program shall distinguish payment finality, result availability and delivery to the authorized recipient, retaining each unresolved obligation.

## Acceptance witnesses

Accept independently evidenced payment and recipient delivery. Reject a payment receipt or result inside a TEE vault as proof of recipient delivery.

## Evidence and theory

See the [OWS/x402 synthesis](../ows-x402/explanation.md) and [source claims](../ows-x402/reference.md). This is a proposed behavior, not a feature imported unchanged from either standard. Research Fair exchange, evidence-indexed typestate, epistemic predicates and conditional liveness.

## Target obligations

Bind the rule to actual Midnight ZKIRv3 execution, authenticated state and complete effects. Retain the signed trust, privacy, amendment and recovery policies. Prove positive usability and exclusion of invalid accepted witnesses. Implementation and proofs remain open.

