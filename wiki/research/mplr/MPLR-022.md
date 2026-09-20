---
title: "MPLR-022: Program and evidence commitment separation"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, simplicity, research]
---

# MPLR-022 — Program and evidence commitment separation

## Required behavior

Acceptance shall bind program identity, semantic/profile version, signed intention, state and evidence according to their distinct commitment roles, and shall validate every permitted witness or program substitution against that binding.

## Motivation and evidence

[Primary source](https://docs.simplicity-lang.org/documentation/state/) and the [Simplicity findings](../simplicity/security-findings.md) motivate this proposal. Source facts do not constitute an implemented Moriarty guarantee.

## Positive and negative witnesses

Accept authorized witness variation preserving all required bindings; reject cross-program, cross-stage, cross-domain or stale-version evidence substitution.

## Theory research and relationship

Refines MPLR-014/018 with commitment semantics. Investigate intensional identity, authenticated data structures and proof-carrying code identities.

## Target obligation and history

Prove preservation under actual Midnight ZKIRv3 execution and composed PCD. Keep cryptographic, ledger and external premises explicit. Research-draft created 2026-09-19; no implementation or theorem accepted.

## Mina recursion research refinement — 2026-09-19

Research typed evidence objects that distinguish parsed proof, statement-bound value, private witness, auxiliary output and authenticated actual effect. Distinct representation identities require correspondence; VK integrity and authorization are separate.

[Source study](../mina/reference.md). This is a research direction and backend requirement, not an implemented theorem or feature.
