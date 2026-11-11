---
title: "Pending study: run length and memory ceiling consensus"
type: source
tags: [pending, run-length, gp-1]
sources: [runlength-consensus-brief]
last_updated: 2026-11-10
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-runlength-consensus-20261111/BRIEF.md
source_url: local
---

# Pending study: run length and memory ceiling consensus

## Summary

**Status: pending.** Run `sherlock-runlength-consensus-20261111` is running. Only its brief is ingested; no lane output and no result is ingested. Five Gemini 4 Carbon High lanes study the run-length bound, then a consensus, then a Fable 5.2 audit. Its folder is `research/sherlock-runlength-consensus-20261111/`. See [[RunLengthCeiling]].

## Key Claims

What the brief states (the brief, not a result):
- The 2^64 in GP-1 is the width of the step counter `h` carried in each node proof. SRC-004 makes the node unsatisfiable if `prev_h`, `steps` or `h = prev_h + steps` is not below 2^64. That bound exists for soundness (no counter wraparound). No circuit has 2^64 rows.
- A run is proved by folding fixed-size proofs (leaf, then node, repeated, then the run decider). Proving cost grows linearly with run length.
- Under D3 the segment index fits 17 bits, so a run has fewer than 2^17 leaves and 2^20 memory cells.

Questions the study asks: Q1 the GP-1 and GP-001 run-length text, without narrowing GP-1; Q2 keep, raise or remove the per-run ceiling (continuations through GP-2 claims, a wider segment index, memory designs without a fixed segment per leaf); Q3 continuations; Q4 prior art in production zkVMs; Q5 the practical run length on this hardware.

## Key Quotes

Owner statements recorded in the brief (2026-11-11):
- "2^64 is physically impossible"
- "I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit"
- Owner request: "launch 5 gemini 4 carbon high agents that can call other agents and search online using scrapling to deeply study this and reach consensus on the design have fable 5.2 audit the answer".

## Connections

- [[RunLengthCeiling]], [[OpenOwnerQuestions]], [[GeneralPurposeRecursion]].
- Sister study: [[lookup-cache-brief]].

## Contradictions

None recorded. The study exists because of the open conflict between D3 and GP-1 ([[dissent-c6-section]]).
