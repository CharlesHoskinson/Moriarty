---
title: "MPLR-026: Behavioral contracts for settlement implementations"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-026: Behavioral contracts for settlement implementations

## Required behavior

When a workflow invokes an interchangeable settlement implementation, acceptance shall establish its required financial and authority postconditions; matching an interface alone shall not discharge these obligations.

## Acceptance witnesses

Accept a certified implementation with required delivery and consumption effects. Reject a Pending result presented as final delivery, or a matching method that omits a required leg.

## Evidence and classification

Motivation: DS05, DS08; DP09. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Refinement interfaces, contextual equivalence, proof-carrying modules and effect polymorphism.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.

