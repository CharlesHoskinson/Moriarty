---
title: "Sherlock terminology"
type: source
tags: [terminology, owner-rule]
sources: [sherlock-terminology]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty-wt/pipeline-anim/research/sherlock-final-design-20261111/TERMINOLOGY.md
source_url: local
---

# Sherlock terminology

## Summary

The category name and the term table for Sherlock, written at the owner's request ("We need some terminology", then "don't lose the terminology"). Sherlock is a universal, elastic proving co-processor for Midnight.

## Key Claims

- Co-processor: Midnight is the main processor; Sherlock runs the computation off-chain on the user's hardware; Midnight accepts the result by checking one proof.
- Universal: one key set and one contract accept every program (GP-1 to GP-4).
- Elastic: the same chip scales from a laptop to a GPU host to a cluster; the statement and keys do not change.
- "Zero-knowledge" joins the category name when the zero-knowledge design is in the architecture ([[ZeroKnowledge]]).
- "Recursion" and "modular" stay out of the name and stay correct inside the architecture documents.
- Terms: Sherlock, Sherlock machine, Sherlock chip, leaf, node, decider, Sherlock prover, Sherlock contract (`SherlockRun`, entry point `acceptRun`), run, claim, Off-chain, On-chain.

## Key Quotes

- "Sherlock is a universal, elastic proving co-processor for Midnight. It runs any program off-chain, on the hardware the user has, and Midnight accepts the result with one key set and one contract."

## Connections

[[AnimationV2]], [[OwnerRules]], [[GeneralPurposeRecursion]].

## Contradictions

Version 1 of the animation says "pipeline"; version 2 says "co-processor" ([[animation-v2-notes]]).
