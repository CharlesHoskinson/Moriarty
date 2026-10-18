---
title: "MPLR-028: Consent-preserving semantic evolution"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-028: Consent-preserving semantic evolution

## Required behavior

When code, verifier, policy or persistent continuation evolves, acceptance shall preserve the signed semantics, residual duties, recovery rights, privacy and resource constraints, or obtain applicable amendment authorization.

## Acceptance witnesses

Accept a proved authorized refinement with state continuity. Reject an interface-compatible upgrade that changes beneficiary, controller or unresolved recovery rights.

## Evidence and classification

Motivation: DP08–DP10; security direction. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Behavioral subtyping, semantic versioning, state migration and contextual refinement.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.

