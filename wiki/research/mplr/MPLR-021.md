---
title: "MPLR-021: Bounded resource certificates"
type: requirement
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, simplicity, research]
---

# MPLR-021 — Bounded resource certificates

## Required behavior

Before accepting a stage under a bounded-execution claim, the verifier shall establish a resource bound for the concrete admitted artifact and any late-bound component under the bound cost model.

## Motivation and evidence

[Primary source](https://raw.githubusercontent.com/ElementsProject/simplicity/pdf/Simplicity-TR.pdf) and the [Simplicity findings](../simplicity/security-findings.md) motivate this proposal. Source facts do not constitute an implemented Moriarty guarantee.

## Positive and negative witnesses

Accept a bounded fixed artifact or checked bounded extension; reject a bound calculated before admitting an unconstrained dynamic component.

## Theory research and relationship

Refines MPLR-009 with static bound soundness. Investigate sized types, cost semantics and certified amortized analysis.

## Target obligation and history

Prove preservation under actual Midnight ZKIRv3 execution and composed PCD. Keep cryptographic, ledger and external premises explicit. Research-draft created 2026-09-19; no implementation or theorem accepted.

## Whole-language review research refinement — 2026-09-19

The whole-language review identifies local append-only ID/state caps separately from per-stage bounded work. Research bounded authenticated state rollover, nonmembership/uniqueness and legitimate continuation origins. Any successor must preserve residual duties and cumulative signed limits. PCD proof compression does not itself bound mutable state size. An explicitly finite instance remains an alternative if its closure/recovery behavior is adequate.

[Review evidence](../language-design-review/reference.md). This adds an open research experiment, not implemented behavior or a changed executable profile.
