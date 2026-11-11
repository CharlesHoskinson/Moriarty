---
title: "Value digest sherlock-value/1"
type: concept
tags: [c6, digest]
sources: [c6-outcome-ledger9-review, dissent-c6-section, lookup-cache-consensus, lookup-cache-audit, final-design-decisions]
last_updated: 2026-11-11
---

# Value digest sherlock-value/1

The canonical value encoding of the C6 review, used for both `input_d` and `outcome_d` (SRC-022, SRC-021 amended) ([[c6-outcome-ledger9-review]]).

- `H` is the Poseidon of zk-stdlib 2.4.0 over the BLS12-381 scalar field. Every value preimage starts with the header `[V, 1, tag, arity]`.
- Tags: `UInt` 0, `Bool` 1, `Nil` 2, `Cons` 3, `Leaf` 4, `Branch` 5, `Constructor` 6 (fields folded one per step, Carbon's form, D-5). A reference `Const(i)` or `Data(i)` hashes as the value it refers to, so addresses and sharing are erased.
- Outcome: `H(O, 1, 0, vd(v))` for `Halted(v)`, `H(O, 1, 1, code)` for `Rejected(code)`, `H(O, 1, 2, kind)` for `Exhausted(kind)` with `kind` in 0..7 (Fuel, DataNodes, DataFields, EnvironmentCells, HelperTasks, HelperWork, SerializationBytes, LogicalCap).
- The preferred mechanism computes the digest when a value is created and reads it once at halt. A traversal at halt is the alternative. Neither is measured (D-4, task 4.16) ([[dissent-c6-section]]).
- Cost, all ESTIMATE in the source: 210 rows for UInt, Bool, Leaf, Cons and Branch; 140 rows for Nil; at most +11% leaf rows on the census average; the decider changes by under 1,000 rows ([[c6-outcome-ledger9-review]]).

## Lookup cache study (lead L-3, pending owner review)

- The lookup consensus proposes one precomputed sponge prefix `PoseidonPerm(0, SHK_VAL, 1)` for every dynamic digest (-70 rows, ESTIMATE), a 2-input bus re-hash, and new Tuple, Record and Array forms ([[lookup-cache-consensus]]).
- The audit (blocker 2): the sponge puts the input length in its initial state and preimages have 4 to 7 words, so it needs one constant per preimage length; the bus chain is a different hash; the new forms are not in `sherlock-value/1`. The constant header `[V, 1, tag, arity]` lets two permutations fold into constants for `UInt`, `Bool`, `Leaf`, `Cons` and `Branch` ([[lookup-cache-audit]]).
- Lead L-3: the single prefix is rejected; VALUE DIGEST computes exactly `sherlock-value/1`, with no 2-input bus re-hash. L-10 drops the invented value forms ([[final-design-decisions]]).

## Open

- D-2: the type identity `ty_d` inside a constructor digest. The normal form of `decl_schema_d` must be fixed before any key is frozen ([[dissent-c6-section]]).
- The Init mechanism that relates `val_d` to the initial memory (lazy unfold or load steps); it must not scan the input in the Init check ([[c6-outcome-ledger9-review]]).

## Contradictions

- [[lookup-cache-consensus]] against [[lookup-cache-audit]] on the single sponge prefix and the new value forms; the lead took the audit (L-3, L-10).

- The `Exhausted(kind)` encoding lists kind 7 = `LogicalCap`; SVM-009 refuses exhaust reason 7 in profile `/1`. Not reconciled ([[dissent-c6-section]]).

See also [[C6OutcomeOnlyPublication]], [[OpenOwnerQuestions]].
