---
title: "Pending study: lookup cache for the chip leaf"
type: source
tags: [pending, chip-leaf, lookup]
sources: [lookup-cache-brief]
last_updated: 2026-11-10
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-lookup-cache-20261111/BRIEF.md
source_url: local
---

# Pending study: lookup cache for the chip leaf

## Summary

**Status: pending.** Run `sherlock-lookup-cache-20261111` is running. Only its brief is ingested; no lane output and no result is ingested. Five Gemini 4 Carbon High lanes explore a lookup "cache" in the chip leaf, then a consensus, then a Fable 5.2 audit. Its folder is `research/sherlock-lookup-cache-20261111/`. See [[ChipLeaf]].

## Key Claims

What the brief states (the brief, not a result):
- The leaf is 6,511 rows at S = 16, and 6,090 of those rows are Poseidon (about 87 permutations at about 70 rows each). The brief calls these cost-model facts.
- "So the leaf is a hashing machine with a small CPU attached."

Questions the study asks: Q1 what the cache should hold (memory read cache, offline memory checking across the run, decode and program tables, ALU and RANGE as decomposed lookups, hash memoization); Q2 rows saved and node or decider cost for each; Q3 interaction with C6, SRC-023, the run-length study, SI-12 to SI-14 and GP-2 to GP-4; Q4 prior art; Q5 one design to prototype in `crates/mori-chip` first.

## Key Quotes

- Owner request (2026-11-11): "launch 5 more to explore a lookup design to add into the processor for fast processing (almost like processor cache)".

## Connections

- [[ChipLeaf]], [[MoriChip]], [[C6OutcomeOnlyPublication]].
- Sister study: [[runlength-consensus-brief]].

## Contradictions

None recorded.
