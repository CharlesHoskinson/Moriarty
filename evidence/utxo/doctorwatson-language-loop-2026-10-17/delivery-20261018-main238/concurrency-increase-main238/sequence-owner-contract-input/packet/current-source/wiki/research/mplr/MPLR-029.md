---
title: "MPLR-029: Authenticated completeness of private state"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-029: Authenticated completeness of private state

## Required behavior

When a transition relies on absence, uniqueness or completeness, its proof shall bind an authenticated state domain and establish the required nonmembership or completeness property under explicit ledger assumptions.

## Acceptance witnesses

Accept nonmembership in the correct authenticated current domain. Reject a private lookup returning None as proof of global absence of debt, revocation or reservation.

## Evidence and classification

Motivation: DP11; privacy-proof REPORT. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Authenticated data structures, frame conditions, epistemic logic and zero-knowledge nonmembership.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.

