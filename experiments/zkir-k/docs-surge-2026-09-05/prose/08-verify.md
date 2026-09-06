# Preservation check of 08-constraints-and-verdicts.md

Verdict: PRESERVED

The unified diff has seventeen changed prose lines in nine hunks. Every change splits a sentence at a semicolon or colon, replaces a semicolon with ", and", swaps a participle for a relative clause ("matching" to "which matches"), reorders two adjacent sentences, or unpacks a parenthetical into its own sentence. No hunk touches a code block, a table row, a heading, a link target or an inline code span.

Mechanical comparison of the two versions:

- one `#` title in both; the heading list (seven `##`, one `###`) is identical in text and order;
- the section "What `holds` establishes" (from its heading up to "How the harness uses verdicts") is byte-identical: 2364 bytes and 348 words in both versions, `cmp` reports no difference, and a word-by-word diff is empty;
- every table row is byte-identical (the constructor table, the outcome table, the chip table);
- all three fenced code blocks (the `k` emission rule, the `k` `transientCommit` line, the `zkir_run.py` command) are byte-identical;
- the multiset of inline code spans is identical, so every K function, cell, constructor, message string, case id (`f01` to `f13`, `k01` to `k07`, `k01b`), evidence path, divergence id (K1, K4) and number survives with the same multiplicity;
- the multiset of chapter cross-references (07, 09, 13) is identical; neither version contains a markdown link;
- no em or en dash, no placeholder, no process word in either version;
- word count 2834 before, 2860 after.

## Meaning changes

### C1 minor Outcomes (from_coordinates paragraph)
Before: `Failures are `violated("from_coordinates: (x, y) is not on the curve")` (case `k01`; off-circuit Jubjub decompression uses only the parity of `x`, K1 in 13-known-divergences.md) and `violated("from_coordinates: point is not in the prime-order subgroup")` (case `f11`, the order-2 Curve25519 point).`
After: `A pair off the curve is `violated("from_coordinates: (x, y) is not on the curve")` (case `k01`). Off-circuit Jubjub decompression uses only the parity of `x`, recorded as K1 in 13-known-divergences.md. A point outside the subgroup is `violated("from_coordinates: point is not in the prime-order subgroup")` (case `f11`, the order-2 Curve25519 point).`
What changed: no fact, message, case id or divergence id differs. The parity remark was bound to case `k01` by the parenthesis; it is now a free-standing sentence between the `k01` and `f11` clauses, so its attachment to `k01` is by adjacency only. The edited text also states which condition produces which message ("A pair off the curve", "A point outside the subgroup"), which the message strings already say. Recorded for completeness; not a change of meaning.
Fix: none required. To restore the binding, replace the three sentences with the original sentence quoted above.

No other hunk changes a claim. The remaining hunks, checked one by one:

1. Introduction: "giving one `verdict(Constraint, Outcome)` per emitted constraint" became "and each emitted constraint receives one `verdict(Constraint, Outcome)`". Identical.
2. Module paragraph: "emission lives in `zkir-vm.k`" became "the emission rules are in `zkir-vm.k`". Identical.
3. Cells paragraph: the `job` sequencing sentence now precedes the cell list, and the colon after "witness failure" became a period. Same four cells, same four sequence steps, same `#exec` no-op condition, same `unknown` rather than `violated` outcome, same `checkedJob` behaviour.
4. Outcomes: semicolon between `verdicts` and `violations` became ", and". Identical.
5. `eval/6`: "`commGate` is described below" became "`commGate` has its own section below". Identical.
6. `eval/4`: "`rd` reads an operand; an absent name is" became "`rd` reads an operand, and an absent name is". Identical.
7. `#lowHighBounds`: "High must be Native; a foreign high is" became "High must be Native, and a foreign high is". Identical.
8. `alignedBytesCircuit`: two semicolons became periods; "both become `synthErr`" became "Both rejections become `synthErr`". Same two messages, same `#matches` route, same `k07`.
9. `assert` and `public_input`: "demands" became "requires"; two semicolons became periods. Same `f02` and `f03` claims.
10. Chip gating: "Value gates call ...; hash gates call" became ", and hash gates call". Identical.
11. `usedChips`: "The cases are `f13` (...), `k04` (...) and `k01b` (...); the last two are K4" became "Of the cases `f13` (...), `k04` (...) and `k01b` (...), the last two are K4". Same three cases, same K4 attribution.
12. `less_than`: "matching" became "which matches"; "The gate is `synthErr(...)` when that fails; `bits = 253` pads to 254" became "When that assertion fails the gate is `synthErr(...)`, and `bits = 253` pads to 254"; the `f04` sentence was split at the semicolon. Same padded-bound formula, same 253 and 254, same `k05` and `f04`.
13. `div_mod_power_of_two` and `reconstitute_field`: three semicolons became periods. Same 248, same messages, same `f01` and `f10`.
14. `commGate`: "matching" became "which matches"; "The `synthErr` branch is unreachable from `job`, because ...; the rule exists so that `eval` is total" was reordered to "The `synthErr` rule exists so that `eval` is total: that branch is unreachable from `job`, because ...". Same reason (`<doComm>` true implies Poseidon in `usedChips`).
15. Harness: semicolon before "`constraints` and `verdicts` are the lengths" became a period. Identical.
16. Closing paragraph: "gates that were actually emitted; status alone is not enough (case `f05` is `ok` with a `violated` gate)" became "gates that were emitted. Status alone is not enough: case `f05` is `ok` with a `violated` gate." The dropped "actually" carried emphasis only. Same `f05` claim, same three things the evidence does not prove.

## Dropped material

None. Every sentence of the original is present in the edited version, verbatim or reworded with the same content. No list item, table row, code line, case id, message or claim was removed. The seven "Must cover" items of the chapter brief (constructors and emission, outcomes and rule shapes, chip gating with K4, width limits, commitment gate versus witness commitment, what a verdict establishes, harness use) are each still covered in full.

## Rule violations

None.

- No em or en dash in the edited chapter.
- No placeholder, TODO or process word (draft, review, audit, persona, agent, editor, "this document").
- Exactly one `#` title.
- Code blocks, tables, link targets and inline code are unchanged.

## Residual machine tells

None of consequence. The one stylistic fingerprint of the pass is the relative clause "which matches", absent from the original and used twice in the edited version ("which matches `max(bits + bits % 2, 4)` of `std.lower_than`", "which matches the `encode_incircuit` loop"). Both state a real correspondence and read naturally; noted, not a finding.
