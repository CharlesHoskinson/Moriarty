---
title: "Run-length ceiling (D3 against GP-1)"
type: concept
tags: [run-length, gp-1, d3, open]
sources: [c6-outcome-ledger9-review, dissent-c6-section, design-2-7-chip-leaf, general-purpose-definition, quint-chipleaf-pairrow-m3, chip-spike-experiments, runlength-consensus-brief, gp-architecture]
last_updated: 2026-11-10
---

# Run-length ceiling (D3 against GP-1)

An open conflict between the proposed chip leaf memory (D3) and the owner's GP-1. Not resolved; GP-1 is not narrowed ([[design-2-7-chip-leaf]], [[dissent-c6-section]]).

## Two different bounds

- **The D3 ceiling.** Under D3, `base / 8 < 2^17`, every leaf takes one segment (a zero-append leaf too), and the node requires at least one step per leaf. So a run has fewer than 2^17 leaves and 2^20 cells ([[c6-outcome-ledger9-review]] `review.md:40`; the walker bound in [[chip-spike-experiments]]; the address space in [[quint-chipleaf-pairrow-m3]]).
- **The GP-1 bound.** "The run length is bounded only by the height field (below 2^64)" ([[general-purpose-definition]]). The brief of the running study says this 2^64 is the width of the step counter `h` in each node proof: SRC-004 makes the node unsatisfiable if `prev_h`, `steps` or `h = prev_h + steps` is not below 2^64, for soundness (no counter wraparound). It is not a circuit size: "No circuit has 2^64 rows." A run is proved by folding fixed-size proofs, and proving cost grows linearly with run length ([[runlength-consensus-brief]]).

## What the C6 decision says

- The C6 decision does not repair the ceiling. The prover can only fail to prove a long run; it cannot change the outcome ([[c6-outcome-ledger9-review]]).
- No requirement of the change states the ceiling as a bound. Task 4.19 records the candidate memory designs. The lead makes no recommendation ([[dissent-c6-section]]).
- The GP architecture study notes that fixed Merkle depths are a capacity fixed by the relation; Astra holds that this is still a narrowing ([[gp-architecture]]).

## Owner statements (2026-11-11)

- "2^64 is physically impossible"
- "I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit"

Source: [[runlength-consensus-brief]].

## Status

Pending. Two studies on the bound are running: [[runlength-consensus-brief]] (the bound itself) and [[lookup-cache-brief]] (which must say where its design touches the run-length study). No result is ingested. The question is on [[OpenOwnerQuestions]].

See also [[GeneralPurposeRecursion]], [[ChipLeaf]].
