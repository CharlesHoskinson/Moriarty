---
title: "MPLR-027: Authenticated origins and recursive compliance"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-027: Authenticated origins and recursive compliance

## Required behavior

When an accepted transition relies on recursive history, its proof shall establish legitimate initial state, compatible predecessor statements and well-founded composition, including every consumed resource and residual obligation.

## Acceptance witnesses

Accept a funded base with authenticated origin and preserving descendants. Reject fabricated already-approved genesis, a wrong predecessor relation, cyclic bootstrap or reused spend.

## Evidence and classification

Motivation: DA authorization study and security direction; proposed PCD requirement. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Inductive invariants, proof-carrying data, DAG semantics, separation logic and linear resources.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.


## Mina recursion research refinement — 2026-09-19

Research typed deferred-verification obligations with producer/consumer invariants, legitimate base masks, complete finalization and bounded-fan-in composition over growing finite history. Transcript/challenge schedules and batching soundness are part of the native contract; cumulative signed budgets and uniqueness remain separate state obligations.

[Source study](../mina/reference.md). This is a research direction and backend requirement, not an implemented theorem or feature.
