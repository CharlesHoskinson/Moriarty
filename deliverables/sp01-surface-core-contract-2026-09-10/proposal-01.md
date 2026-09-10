# Surface and Core Boolean reconciliation

Status: **proposed successor revision; requires two independent design/source
reviews**. This document does not modify the reviewed expression-contract/0,
its EX-D1 choice, any registered profile, parser, evaluator or K definition.
It recommends a concrete resolution before full RP01 freeze.

## Recommended decision

Retain the source design's left-to-right short-circuit `and` and `or`, and give
the corresponding Core `And` and `Or` that same behavior in the next reviewed
contract revision. Keep all forty constructor names. No extra conditional
constructor, hidden thunk, host callback or alternate source dialect is needed.

The source design explicitly selects short-circuit behavior at LANGUAGE-DESIGN.md
line71. Reviewed expression-contract/0 instead chose strict And/Or under EX-D1
and left the disagreement open. Those were scoped proposals, not evidence of
source/Core correspondence. This recommendation changes EX-D1 deliberately,
preserves its historical approval and requires a new candidate and new votes.
We must not silently relabel the previous strict tests as passing short-circuit.

The reason is direct source/Core agreement: `a and b` and `a or b` already have
parser nodes and precedence. A developer should not need a different spelling
to retain the behavior promised by the source contract. Financial effects are
not expression operands, so skipped Boolean evaluation does not implicitly
skip a submitted transfer; the enclosing action's explicit guard and emission
order remains authoritative.

## Static judgment and admission

Keep the complete action's existing admission and static phases. Check both
Boolean operands in lexical order, including the branch that a runtime value
would skip. Both must have exactly Bool type. Unknown names, invalid literal
domains, bad spans and oversized supplied inputs reject before any reduction,
even in a skipped branch. This is short-circuit **evaluation**, not partial
validation or a change to schema/authority checks.

All existing simultaneous source/Core/value limits remain. Static work bounds
may conservatively count both branches; no runtime branch value can waive the
admission ceiling or the requirement to validate the full action. Neither
branch can mutate financial state by being a pure expression.

## Felleisen–Hieb reduction and finite work

For this proposed revision, enter the And/Or node once, consuming one work unit;
then evaluate its left operand. After it yields a Boolean, apply:

```text
And(false, e2)  -> false
And(true,  e2)  -> e2
Or(true,   e2)  -> true
Or(false,  e2)  -> e2
```

These contractions consume no extra work and create no chargeable literal node.
The original right operand remains its original node occurrence when selected.
Do not enter or inspect its dynamic value when skipped. An error while evaluating
the left operand propagates immediately. There is no right-operand evaluation
context before the left value determines selection. This replaces the strict
And/Or contexts only in the new proposed revision; other expression contexts
and their left-to-right work rules are unchanged.

The expressions returned by the selected-branch contractions retain the
original child nodePath and span. Entering And/Or at path p then selecting the
right operand resumes at p+[1], not p. Parent context contraction must not charge
the And/Or node again. A left-child rejection uses p+[0] plus its nested path.
If work is exhausted before entering the selected right child, reject there
without charging a missing work unit. Skipped nodes consume zero work.

For admitted expressions, with C(e) the actual ordinary work used:

```text
C(And(e1,e2)) = 1 + C(e1) + (C(e2) if e1 returns true else 0)
C(Or(e1,e2))  = 1 + C(e1) + (C(e2) if e1 returns false else 0)
B(And(e1,e2)) = B(Or(e1,e2)) = 1 + B(e1) + B(e2)
```

The first two equations apply only when the selected evaluations succeed; a
failure retains the actual entered-node prefix. B is a conservative finite
upper bound, not a proof-cost estimate. A source operator lowers to its named
Core operator exactly once; it does not expand into duplicated evaluation.

## Required migration and validation

- Preserve expression-contract/0 and its original strict-And rejection case.
  A revised contract must change the normative rules, signature evaluation-order
  metadata, context grammar, work derivations and expected cases together.
- In the revised candidate, the former strict-And denominator case becomes
  `false`, workUsed2; its direct eager arithmetic and selected-branch variants
  still reject. Do not claim that all old outcomes are preserved.
- Keep complete-action typing and snapshot validation before reduction; an
  unavailable/ill-typed input cannot be hidden by a literal false branch.
- Match parser AST, source elaboration, Core results, exact work/path/span and K
  on all four truth-table rows, both skipping controls, selected arithmetic
  failures, left failure, static dead-branch failure and work exhaustion.
- Retain source and semantic profile hashes separately. A compiler/profile
  lineage using strict And/Or is incompatible with this revision until explicit
  migration; updating prose cannot change an old proof's meaning.

The JSON cases are independent proposed expectations, not execution receipts.
The small arithmetic check only counts specified node-entry sequences. Full
RP01 still requires the financial-operation, signing, authority and history
contract. No Midnight gate is discharged here.
