# Preservation check of 03-program-model.md

Verdict: PRESERVED

Method: `diff -u` of the original against the edited chapter, then both versions read in full. The diff touches 20 prose paragraphs. Every table row, code block, link target and inline code span is identical in both versions (checked mechanically: the multiset of backtick spans, the set of `](...)` targets, the `|` table lines and the fenced blocks all diff clean). Prose word count moved from 1363 to 1424.

## Meaning changes

### C1 minor Rejections and loader fidelity
Before: Parser equivalence is not established: `json.load` errors, the `0x0\n` immediate above and a non-string element of an extension `load_constant` encoding (an `AttributeError` from `immediate`) escape the `ZkirFormatError` handler.
After: Parser equivalence is not established, because some failures escape the `ZkirFormatError` handler: `json.load` errors, the `0x0\n` immediate above, and a non-string element of an extension `load_constant` encoding (an `AttributeError` from `immediate`).
What changed: the original juxtaposes the two facts with a colon (the escaping failures illustrate why equivalence is not claimed); the edit makes an explicit causal "because", which reads as the sole reason. The preceding sentence also gives another reason (serde-derived and `const_hex` messages the crate does not control). The listed facts, symbols and messages are unchanged, so this is a nuance of emphasis, not a changed claim.
Fix: restore "Parser equivalence is not established: `json.load` errors, the `0x0\n` immediate above and a non-string element of an extension `load_constant` encoding (an `AttributeError` from `immediate`) escape the `ZkirFormatError` handler."

### C2 minor Operands, guards and alignments
Before: Pinned `ir.rs`, `Operand::deserialize`, uses `const_hex::decode` and `Fr::from_le_bytes` for the same byte interpretation and range boundary.
After: `Operand::deserialize` in pinned `ir.rs` uses `const_hex::decode` and `Fr::from_le_bytes`, so the byte interpretation and the range boundary are the same.
What changed: "for the same" became "so ... are the same", turning a statement of shared behaviour into an inference from the two function names. The claim (same little-endian interpretation, same `r` bound) is unchanged and is what the original asserted.
Fix: restore "Pinned `ir.rs`, `Operand::deserialize`, uses `const_hex::decode` and `Fr::from_le_bytes` for the same byte interpretation and range boundary." if the inferential "so" is unwanted.

No blocking finding: every file path, symbol, number, line number, message template, command and link target in the edited chapter matches the original.

## Dropped material

- "not the reverse" after the `typedIds(typedId("%x", ...), .typedIds)` example (Abstract syntax and types). Emphasis only; the example and the head-of-list claim remain. Harmless cut.
- "direct" in "Six rows have a direct counterpart in `ir.rs`" (Rejections and loader fidelity). The six line-numbered counterparts remain listed. Harmless cut.

No sentence, list item or table row required by the chapter brief's "Must cover" list is absent: artifact shape and serde behaviour, the 13 types with `encodedLen`, `symbol(_)` and `terminator-symbol`, the absence of a K concrete syntax, HEX handling, `%` variables, unknown members, u8 version parts, `Bytes<n>`, the rejection table with messages, the `kore`, `kast` and `check` commands, loader fidelity and the differential harness, and the worked example are all present.

## Rule violations

- Em or en dashes: none.
- Placeholders or TODO markers: none.
- Process words (draft, review, audit, persona, agent, editor, "this document"): none.
- `#` titles: exactly one (`# Program model`).
- Code blocks: identical before and after.
- Tables: identical before and after.
- Link targets: identical before and after.
- Inline code: identical before and after.

## Residual machine tells

- "Oversized positive values are rejected, not reduced." (Operands, guards and alignments): contrastive "X, not Y" negation. Present in the original; the contrast is technically load-bearing.
- "`reads` is a structural inventory, not a guarantee that execution resolves every listed operand" (Instruction constructors): same "X, not Y" pattern, present in the original and load-bearing.
- "Lengths count raw native field elements, not bytes" (Abstract syntax and types): same pattern, present in the original, load-bearing.
- "Loading checks the tagged structure only. Whether a chosen hash operation can consume it is covered in ..." (Operands, guards and alignments): the split into two short sentences with a fronted "Whether" clause reads slightly mechanical, but the meaning is clear and unchanged.
