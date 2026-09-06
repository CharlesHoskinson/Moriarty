# Preservation check of 07-instruction-reference.md

Verdict: PRESERVED

The unified diff has twelve hunks, all in prose lines. Every hunk splits a long sentence or paragraph, replaces a comma or parenthesis with a semicolon, or adds a connective; no hunk touches a code block, a table row, a heading, a link target or an inline code span.

Mechanical comparison of the two versions:

- one `#` title in both;
- 37 `###` headings in both, identical in text and order: the three machinery subsections plus the 34 instruction entries (`encode` through `output`, in `zkir-syntax.k` order);
- the part labels per entry are identical (34 Syntax, 34 Off-circuit, 34 Gate, 9 Checks, 2 Emission);
- every table row is byte-identical (the part table, the resolver table, the block table, the summary table, the corpus table);
- both fenced code blocks (`k` sequencing rules, `k` `#and` rules, the two `sh` blocks) are byte-identical;
- the multiset of inline code spans is identical, so every K function, cell, constructor, error message, `synthErr`/`violated`/`unknown` text, divergence id and corpus program name survives with the same multiplicity;
- the multiset of double-quoted labels is identical;
- the multiset of link targets is identical;
- no em or en dash in either version; no process word in either version.

## Meaning changes

None. The twelve hunks, checked one by one:

1. Introduction: "`reads(Instr)` and `writes(Instr)` give the operands ... and the identifiers it defines" split into two clauses, one per function. Same attribution.
2. "How to read an entry": the cross-reference sentence split after 08; same four targets, same subjects.
3. "Off-circuit resolution": one paragraph split into three (type names; `#put`/`#put2`/`#check`; `[owise]` rule). "turn a `vErr(S)` into `#fail(S)`" became "a `vErr(S)` becomes `#fail(S)`"; identical.
4. "Standard" gate description: "so the gate is ..., the dispatch of" became "The gate is therefore ...; this is the dispatch of". Identical claims.
5. `constrain_to_boolean` Off-circuit: "only the resolver's errors, as the crate's" became "the only errors are the resolver's, as in the crate's". Identical.
6. `copy` Off-circuit: "any type, the only error is" became "for any type; the only error is". Identical.
7. `impact` Emission and Off-circuit: the parenthetical on `#impactPush`/`#impactOne` was unpacked into its own sentence. The sequence (push each input, record `skipNone()`, then `#impactCheck`), the native requirement, the `cannot convert T to Native` message, the `<pi>` append, the `<pubInIdx>` advance, the partial-push behaviour and the mismatch message all survive in the same order.
8. `from_coordinates` Off-circuit: "Off the curve or outside" became "A point off the curve or outside". Identical.
9. `bytes32_from_low_high` Off-circuit: "`#lowHighErr`, the order of the crate's arm" became "which follows the order of the crate's arm". Identical.
10. `reconstitute_field` Off-circuit: commas between the `#recon`, `#recon3`, `#recon4` clauses became semicolons. Identical.
11. `less_than` Gate: "asserts it is at most 253, so ... is" became two sentences with "therefore"; "otherwise `#native` on both" became "otherwise the rule applies `#native` on both". Same padded-bound message, same check order, same `f04`/`k05` references.
12. `public_input` Off-circuit and Gate: "treats no guard as active and resolves one with" became "treats an absent guard as active and resolves a present one with" (same meaning, less ambiguous); the short-transcript sentence was reordered so the panic text comes first and the generation-mode alternative second, both preserved; "as `assign_incircuit`" became "as `assign_incircuit` does".
13. Corpus table introduction: the corpus-name sentence split in two after `divergence/`. Same names, same paths.

## Dropped material

None. Every sentence of the original is present in the edited version, either verbatim or reworded with the same content. No list item, table row, code line or claim was removed.

## Rule violations

None.

- No em or en dash in the edited chapter.
- No placeholder, TODO or process word (draft, review, audit, persona, agent, editor, "this document").
- Exactly one `#` title.
- Code blocks, tables, link targets and inline code are unchanged.

## Residual machine tells

None of consequence. The only pattern introduced by the pass is the connective "therefore", absent from the original and used three times in the edited version ("The gate is therefore `violated(...)`", "Positions therefore never depend on how far the witness got", "`#ltBits(N) > 253` is therefore `synthErr(...)`"). Each use states a real consequence of the preceding clause and reads naturally; it is noted here only as the one stylistic fingerprint of the pass, not as a finding.
