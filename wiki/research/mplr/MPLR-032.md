---
title: "MPLR-032: Concurrent service budgets"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, ows, x402, ai-solvers]
---

# MPLR-032: Concurrent service budgets

## Required behavior

When concurrent tasks or retries commit funds, acceptance shall reserve and account for spent, pending and residual amounts, including fees and sponsored execution.

## Acceptance witnesses

Accept two purchases within the combined budget. Reject individually valid purchases whose combined exposure exceeds it.

## Evidence and theory

See the [OWS/x402 synthesis](../ows-x402/explanation.md) and [source claims](../ows-x402/reference.md). This is a proposed behavior, not a feature imported unchanged from either standard. Research Linear resources, reservation semantics, quantitative types and concurrent separation logic.

## Target obligations

Bind the rule to actual Midnight ZKIRv3 execution, authenticated state and complete effects. Retain the signed trust, privacy, amendment and recovery policies. Prove positive usability and exclusion of invalid accepted witnesses. Implementation and proofs remain open.

