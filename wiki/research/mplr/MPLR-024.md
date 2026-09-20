---
title: "MPLR-024: Explicit settlement domains"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-024: Explicit settlement domains

## Required behavior

When a workflow claims atomic settlement, it shall identify the ledger/domain and the exact effects inside that atomic boundary; relocation and independent-chain effects shall retain separate pending and final states.

## Acceptance witnesses

One synchronizer executes all prepared legs atomically. Reject a claim that unassigning on one synchronizer proves assignment or external delivery.

## Evidence and classification

Motivation: DS15; settlement REPORT. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Distributed transaction semantics, effect regions, session types and refinement of relocation protocols.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.

