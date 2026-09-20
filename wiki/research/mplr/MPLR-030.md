---
title: "MPLR-030: Common evidence statement and federation trust"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, daml, research]
---

# MPLR-030: Common evidence statement and federation trust

## Required behavior

When the kernel combines proofs, attestations and signing authority, it shall bind them to the same intention, domain, stage, epoch, artifact and exact effects, and expose the threshold, hardware, observation and recovery assumptions.

## Acceptance witnesses

Accept proof, attestation and threshold authorization for one allowed effect. Reject a proof for X combined with a signature for Y or an unauthorized weaker fallback.

## Evidence and classification

Motivation: Kernel boundary proposal; NEAR and Daml source comparisons. See the [Daml synthesis](../daml/synthesis.md) and [source/claim reference](../daml/reference.md). This is a proposed Moriarty requirement inferred from the evidence and user intent, not an implemented source feature or accepted theorem.

## Theory research

Proof-indexed capabilities, evidence composition, threshold authorization and assumption-aware refinement.

## Midnight proof obligation

Bind the requirement to the actual ZKIRv3 artifact and accepted ledger effects, including arbitrary satisfying witnesses, explicit external assumptions and positive usability witnesses. Keep language deployment permissionless. Formalization, implementation and target proof remain open.

