---
title: "MPLR-034: Logical paid-request identity"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, ows, x402, ai-solvers]
---

# MPLR-034: Logical paid-request identity

## Required behavior

When a request is retried or recovered, the workflow shall preserve one authenticated logical obligation and distinguish result retrieval from new billable work.

## Acceptance witnesses

Accept idempotent reconciliation of an already-settled request. Reject duplicated spending or new service grants from replayed transport data.

## Evidence and theory

See the [OWS/x402 synthesis](../ows-x402/explanation.md) and [source claims](../ows-x402/reference.md). This is a proposed behavior, not a feature imported unchanged from either standard. Research Trace semantics, session identities, idempotency and linear continuation protocols.

## Target obligations

Bind the rule to actual Midnight ZKIRv3 execution, authenticated state and complete effects. Retain the signed trust, privacy, amendment and recovery policies. Prove positive usability and exclusion of invalid accepted witnesses. Implementation and proofs remain open.

