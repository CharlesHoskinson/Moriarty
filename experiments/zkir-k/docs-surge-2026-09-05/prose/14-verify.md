# Preservation check of 14-extending-the-semantics.md

Verdict: PRESERVED

Method: `diff -u` of the original against the edited chapter (19 hunks, all in prose paragraphs and one table cell), then a full read of both versions. Mechanical checks: the multiset of inline code spans is identical on both sides; the six fenced code blocks are byte-identical; no markdown links exist on either side; the word count is unchanged (3045). Every row of the instruction, type and chip checklists and all six pitfall headings survive. The remaining differences are colons and semicolons turned into full stops, three passive constructions made active (`compare` catches the mis-reported panic; the six-ary `[owise]` forwards the constructor; `diff_test.py` covers the handmade positive), "has to" made "must", "will steal" made "steals", and one parenthetical unpacked into its own sentence (strict decoding applies on the extension surface and to `load_constant` on either surface). None of these changes a fact, number, symbol, path, command, message or claim.

## Meaning changes

None.

## Dropped material

- Table "Adding an instruction", row Chip, third column: "the chips the crate's `used_chips` actually enables" is now "the chips the crate's `used_chips` enables". The word "actually" carried emphasis only (the contrast with chips a gate demands but `used_chips` does not set is stated in full in "Adding a chip"). Harmless cut.

## Rule violations

- Altered table: the Chip row of the "Adding an instruction" table changed as described above (severity minor; meaning intact, no code span touched). Every other table row is byte-identical.
- Em or en dashes: none.
- Placeholders or process words (draft, review, audit, persona, agent, editor, "this document", TODO): none.
- `#` titles: exactly one.
- Code blocks: unchanged.
- Link targets: no links on either side; the plain file references (13-known-divergences.md, 12-oracles-and-differential-testing.md, 15-design-rationale-and-limits.md) are unchanged.
- Inline code: unchanged.

## Residual machine tells

- "A new `*V` formula or encoding belongs in the unit tools; a new program in the differential harness; a new disagreement in the divergence suite." Three parallel elided clauses (rule of three). Present unchanged in the original; not introduced by the prose pass.
- "Rebuild before recording; stale kompiled directories behind a receipt are a failed check." Aphoristic closing pair. Present unchanged in the original.
