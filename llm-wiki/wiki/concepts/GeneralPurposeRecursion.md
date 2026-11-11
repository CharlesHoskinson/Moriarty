---
title: "General-purpose recursion: GP-1 to GP-4"
type: concept
tags: [gp, owner-rule]
sources: [general-purpose-definition, gp-architecture, c6-outcome-ledger9-review, dissent-c6-section]
last_updated: 2026-11-10
---

# General-purpose recursion: GP-1 to GP-4

Owner ruling, 2026-11-10: general purpose and universal means all four properties below. The owner selected all four of universal VM, cross-run composition, foreign proofs, and any shape (PCD). "A narrowing needs an explicit owner decision" ([[general-purpose-definition]]). See [[OwnerRules]].

| Property | Meaning | Gap today |
|---|---|---|
| GP-1 Universal VM | One fixed key set proves every program, input, run length and outcome; programs and inputs are data; capacity does not depend on the key; run length bounded only by the height field (below 2^64) | Keys bound to one inventory, layout, `Bounds`, profile and S; only `Halted` with a u128 result; no run input; one contract per `(program_d, init_d)` |
| GP-2 Cross-run composition | A run consumes another run's claim `(prog_d', input_d', outcome_d')`, checked in circuit; many runs aggregate into one proof | Linked segments share one `prog_d`; the node verifies one fixed leaf key and itself |
| GP-3 Foreign proofs | Verify and aggregate other circuits' proofs, admitted from an authenticated key registry, through a statement adapter | No registry and no adapter |
| GP-4 Any shape (PCD) | Chains, trees and DAGs of any depth, arity at least 2, one key set, shape bound into the statement, constant final proof | Linear chain only; the three-slot tree node needs k=21 and is closed |

Source of the table: [[general-purpose-definition]].

## How later work relates

- The C6 claim is `(version, prog_d, input_d, outcome_d)`, taken from the owner's GP-2 triple (D-3, [[dissent-c6-section]]). GP-001 to GP-004 additions come from [[c6-outcome-ledger9-review]].
- The GP architecture study recommends a binary PCD node at k=20 with claims, a key registry and committed memory; it lists four points not met as written ([[gp-architecture]]).
- Open conflict: the D3 run ceiling against GP-1 ([[RunLengthCeiling]]).
- Open: D-6, whether segment bracketing is public in the GP-4 shape commitment ([[dissent-c6-section]]).

See also [[OpenOwnerQuestions]].
