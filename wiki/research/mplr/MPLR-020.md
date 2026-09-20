---
title: "MPLR-020: Certified primitive substitution"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, simplicity, research]
---

# MPLR-020 — Certified primitive substitution

## Required behavior

When a certified primitive substitutes for a reference expression, acceptance shall bind its version and establish preservation of reference semantics, preconditions, failure behavior and the relevant host/target correspondence.

## Motivation and evidence

[Primary source](https://blockstream.com/simplicity.pdf) and the [Simplicity findings](../simplicity/security-findings.md) motivate this proposal. Source facts do not constitute an implemented Moriarty guarantee.

## Positive and negative witnesses

Accept a substitution with discharged caller conditions and target correspondence; reject an optimized primitive that drops a carry, weakens an assertion or changes a state effect.

## Theory research and relationship

Refines MPLR-014 at the primitive boundary. Study refinement calculus, verified compilation and translation validation.

## Target obligation and history

Prove preservation under actual Midnight ZKIRv3 execution and composed PCD. Keep cryptographic, ledger and external premises explicit. Research-draft created 2026-09-19; no implementation or theorem accepted.

## Mina recursion research refinement — 2026-09-19

Mina motivates specification-to-constraint traceability for each used custom gate, lookup and binding. Historical recursion-layer audits do not qualify every primitive or changed dependency tuple.

[Source study](../mina/reference.md). This is a research direction and backend requirement, not an implemented theorem or feature.
