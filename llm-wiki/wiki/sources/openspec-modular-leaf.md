---
title: "OpenSpec change sherlock-modular-leaf-20261111 (implementation spec)"
type: source
tags: [openspec, spec, final-design, module-kits]
sources: [openspec-modular-leaf]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/openspec/changes/sherlock-modular-leaf-20261111/proposal.md
source_url: local
---

# OpenSpec change sherlock-modular-leaf-20261111 (implementation spec)

## Summary

The implementation spec of the modular Sherlock leaf and recursion. It implements the leaf and recursion design of `sherlock-vm-consensus-20261107`, which keeps the GP, SRC, SVM and SAS requirements. `design.md` is the short form of [[sherlock-architecture]]. The change has one capability spec per module kit (23 specs), and `tasks.md` follows the build order with one task per kit, each ticked only on its gate.

## Key Claims

- 23 capability specs, 119 requirements, 203 scenarios; every requirement is under 500 characters.
- Every scenario names its evidence: a MockProver test or named mutant, a Quint model, or a Lean obligation with its status (proved in C3, or open).
- Requirements that depend on an owner decision or an experiment say which one gates them (for example TOP = 63 on OD-1 and OD-11, `K_rom` on X15).
- Narrowing options (OD-1c, V-1b/c, two-node GP-4, adapter-only GP-3) are never acceptance paths; no GP text is narrowed.
- Profile B (ZmLogUp) is benchmark-only and not built before M4.
- `tasks.md` has 84 tasks in groups F, 0, R, 1 to 4, L, 5 to 8, plus experiment, prerequisite, acceptance and Profile B groups; none is ticked.
- `openspec validate sherlock-modular-leaf-20261111 --strict` passes with no errors and no warnings (OpenSpec 1.17.0).

## Open

- Some scenarios name evidence that the architecture leaves unnamed (the seam test, per-family CORE positives and mutants, HASH differential vectors).
- Values the architecture leaves open stay as constraints, not numbers: `K_rom`, `n_cold`, the final `K_page`, ROM depth, cells per record, the state schema lane count.

## Links

- [[sherlock-architecture]], [[ModularArchitecture]], [[final-design-decisions]], [[OpenOwnerQuestions]]
