---
title: "Revisions applied from the council: P-1 to P-10"
type: source
tags: [council, revisions, architecture, openspec]
sources: [council-revisions]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty-wt/pipeline-anim/research/sherlock-pipeline-animation-20261111/council/REVISIONS.md
source_url: local
---

# Revisions applied from the council: P-1 to P-10

## Summary

The work order of [[council-findings]], applied as design text to `ARCHITECTURE.md`, `DECISIONS.md` and the OpenSpec change `sherlock-modular-leaf-20261111`. Nothing was built, tested or proved. The placeholders K-1 to K-6 became a new decision series P-1 to P-10. The design now has 24 kits (a new LEDGER CONTRACT kit) and a Lean tally of 14 proved, 55 open, 69 total. These edits live on branch `pipeline-anim` (commit `f372ab74c`, per [[log]]) until merged.

## Key Claims

| Decision | Item | Change |
|---|---|---|
| P-1 | C5 part 1 | `reg_root` equal across child slots and parent (NODE-008); mutant `node_reg_root_mixed`; under 10 rows (ESTIMATE) |
| P-2 | C9 | FV shape case `(0, 0)`, FV payload `(fv_state_d)` only, statement-tag table (SHP-005, NODE-009) |
| P-3 | C4 | One seal path, full revision: the run entry opens a `LEAF` or `DEP` SET head with one hash; `open_terminal` removed; M4 path runs one SEAL node first, about 65 s (ESTIMATE) |
| P-4 | C8 | Inline kernel from the leaf kit code (SHP-006); separate emitter removed |
| P-5 | C1 | Program admission ADMIT-WORK (statement tag AW = 6) discharges PROG canonicality at the seal; seal check about 140 rows (ESTIMATE); new Lean NODE-L4 |
| P-6 | C2 | Certified read-only input view VIEW-WORK (tag VW = 7), read port `AuthIn`, `view_root` body word; seal view check about 280 to 350 rows, 20 to 64 hashes per read (ESTIMATE); adopted as the mechanism of OD-17 form (b); OWNER for OD-17 and I7 |
| P-7 | C3 | `work_root` operand store in the foreign-verifier state (FVR-005) |
| P-8 | C6 | LEDGER CONTRACT kit, spec `sherlock-ledger-contract` (LDG-001 to LDG-005); DEC-L2 moved as LDG-L1; N-11 reworded to two entry points of one contract |
| P-9 | C5 part 2 | Run-entry `reg_root` rule, effective only with OD-20 option (b) (DCD-007) |
| P-10 | C7 | `rules_d` consumer not decided: new row OD-23 with two options, OWNER |

- C10 refuted; no change.
- C1 consequence on M4: the M4 SEAL node also needs the admission claim, so M4 needs one ADMIT-WORK chain per distinct program.
- `openspec validate sherlock-modular-leaf-20261111 --strict` passes with no warning, after four over-long requirements were split into new ones.
- Checks recorded: no dash in added text; Owner cells of OD-14, OD-17 and OD-20 unchanged; GP-1 to GP-4 not narrowed; one key set for every program.
- New requirement ids: NODE-008 to NODE-013, DCD-006, DCD-007, SHP-005, SHP-006, CLM-007, CORE-013, INIT-006, FVR-005, LDG-001 to LDG-005.

## Key Quotes

- "Nothing was built, tested, proved or committed." (at the time the file was written; the commit came after)
- On OD-17: "the mechanism is written as the mechanism of form (b), and the Owner cell is unchanged."

## Connections

[[council-findings]], [[ModularArchitecture]], [[AcceptRunContract]], [[RunDecider]], [[IvcNode]], [[ValueDigest]], [[OpenOwnerQuestions]], [[AnimationV2]].

## Contradictions

- Two flags for the lead. (1) COUNCIL C2 puts `view_root` in the body and has the seal compare it, but `open_state_header` returns only `body_d` and the node never re-hashes the body; the seal read path is recorded as an open point of OD-17 and must be decided before NODE-013 freezes. (2) A lazy view is sound but its `view_root` is not canonical, so boundary values at common cuts are equal across partitions only for one view; open.
- The record tuple deviates from COUNCIL C2: `(idx, tag, ty_d, c, m, vd, child_pos)` plus child-vd leaves, so a parent's `vd` can be recomputed when a child is not entered.
- ARCH section 6 history tables still say 23 kits by design, and section 5.2 outline lines for NODE and SHAPE are not re-synced; the specs are authoritative.
- The one-seal path (P-3) and one-contract rule (P-8) stand against the cost round's leaf-direct and decider-direct levers ([[cost-lane-fable]] L3a, L3b; [[cost-lane-gemini]] L4c, `acceptDirectLeaf`), which were written against the pre-council design.
