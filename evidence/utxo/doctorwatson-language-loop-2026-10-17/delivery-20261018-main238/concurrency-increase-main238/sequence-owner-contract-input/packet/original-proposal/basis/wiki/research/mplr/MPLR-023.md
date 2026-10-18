---
title: "MPLR-023: Mandatory conditions constrain acceptance"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, simplicity, research]
---

# MPLR-023 — Mandatory conditions constrain acceptance

## Required behavior

Every declared mandatory intention predicate shall constrain transaction acceptance through a proved control/effect relation; computing or displaying a predicate without enforcing it shall not satisfy that requirement.

## Motivation and evidence

[Primary source](https://docs.simplicity-lang.org/documentation/jets/) and the [Simplicity findings](../simplicity/security-findings.md) motivate this proposal. Source facts do not constitute an implemented Moriarty guarantee.

## Positive and negative witnesses

Accept when every required condition is enforced; reject an artifact that computes signature_ok=false but discards it and accepts anyway.

## Theory research and relationship

Refines MPLR-003/012 with non-vacuous acceptance. Investigate refinement effects, assertion dominance and proof-directed compilation.

## Target obligation and history

Prove preservation under actual Midnight ZKIRv3 execution and composed PCD. Keep cryptographic, ledger and external premises explicit. Research-draft created 2026-09-19; no implementation or theorem accepted.

## Mina recursion research refinement — 2026-09-19

Research a static/effect discipline that requires every mandatory predecessor and predicate to reach actual verification. Reading proof fields, false verification guards, dummy modes and ignored checks must not satisfy that obligation.

[Source study](../mina/reference.md). This is a research direction and backend requirement, not an implemented theorem or feature.
