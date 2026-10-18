---
title: "MPLR-031: Delegated solver capabilities"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, ows, x402, ai-solvers]
---

# MPLR-031: Delegated solver capabilities

## Required behavior

When a solver requests a financial effect, acceptance shall enforce the owner's bounded delegation across wallet, agent and signer boundaries.

## Acceptance witnesses

Accept an unregistered solver with a valid scoped capability. Reject arbitrary signing justified only by possession of an OWS token.

## Evidence and theory

See the [OWS/x402 synthesis](../ows-x402/explanation.md) and [source claims](../ows-x402/reference.md). This is a proposed behavior, not a feature imported unchanged from either standard. Research Capability security, authority attenuation, effect types and proof-indexed delegation.

## Target obligations

Bind the rule to actual Midnight ZKIRv3 execution, authenticated state and complete effects. Retain the signed trust, privacy, amendment and recovery policies. Prove positive usability and exclusion of invalid accepted witnesses. Implementation and proofs remain open.

