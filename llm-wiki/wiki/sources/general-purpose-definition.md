---
title: "General-purpose recursion definition (GP-1 to GP-4)"
type: source
tags: [owner-rule, gp]
sources: [general-purpose-definition]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md
source_url: local
---

# General-purpose recursion definition (GP-1 to GP-4)

## Summary

The owner's definition of the target for the Sherlock recursion, recorded on 2026-11-10. It replaces every narrower reading of "general purpose" in the change (`design.md` §3.6, SRC-017). It defines four properties. Each has an acceptance test and a list of the gaps in the code today. See [[GeneralPurposeRecursion]].

## Key Claims

- GP-1 Universal VM: one fixed key set proves every program, every input, every run length and every outcome. Programs and inputs are data (`prog_d`, `input_d`). Every outcome kind is provable. Capacity does not depend on the key.
- GP-2 Cross-run composition: a run can consume the claim `(prog_d', input_d', outcome_d')` of another verified run, checked in circuit, and many runs aggregate into one proof.
- GP-3 Foreign proofs: the recursion verifies and aggregates proofs of other circuits, admitted by verifying key from an authenticated registry, through a statement adapter.
- GP-4 Any shape (PCD): chains, trees and DAGs of any depth, arity at least 2, one key set, the shape bound into the statement, a constant final proof size.
- The consensus design meets none of GP-1 to GP-4 in full. It is the first milestone toward GP-1.
- The closed tree result (k=21 at arity 3) shows that the node size must be designed for GP-3 and GP-4 from the start.

## Key Quotes

- Owner, 2026-11-10: "It needs to be general purpose recursion. Make sure we understand what that means. And universal".
- GP-1: "The run length is bounded only by the height field (below 2^64)."
- "Every requirement and task in this change is to be checked against GP-1 to GP-4. A narrowing needs an explicit owner decision."

## Connections

- [[GeneralPurposeRecursion]] summarises the four properties.
- [[RunLengthCeiling]] is the open conflict between the D3 memory ceiling and the GP-1 run-length bullet.
- [[OwnerRules]] carries the no-narrowing rule.

## Contradictions

- The D3 ceiling of fewer than 2^17 leaves and 2^20 cells per run conflicts with the GP-1 bound of 2^64 steps ([[dissent-c6-section]], [[c6-outcome-ledger9-review]]). Open; see [[OpenOwnerQuestions]].
