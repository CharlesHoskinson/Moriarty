---
title: "MPLR-025: Netting preserves gross economics"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-025: Netting preserves gross economics

## Required behavior

When gross obligations are netted, the transition shall preserve each authorized asset, party, fee, liability and residual duty under an explicit gross-to-net relation.

## Acceptance witnesses

Accept an authorized equivalent net schedule. Reject equal final net balances that conceal an unauthorized fee, different issuer liability or missing gross consent.

## Evidence and classification

Motivation: DS04, DS07, DS18; settlement REPORT. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Resource semantics, accounting identities, trace equivalence and financial contract algebra.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.

