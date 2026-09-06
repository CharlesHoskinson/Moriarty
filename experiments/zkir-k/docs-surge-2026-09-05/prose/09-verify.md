# Preservation check of 09-extension-surface.md

Verdict: PRESERVED

## Meaning changes

None. The diff touches twelve prose passages; each is a sentence split or a rewording with identical meaning. The passages closest to a change were checked individually:

- Modules and representation, byte-string representation: "Values retain `bytes32(B)`" became "Values of that length keep the `bytes32(B)` form". The antecedent (32-byte values) is the same; the claim is unchanged.
- Modules and representation, removed instruction: "The removed instruction keeps its execution and gate rules" became "`reverse_bytes`, the removed instruction, keeps its execution and gate rules". The name is the one the version-boundary table already gives for the removed instruction; no new claim.
- Modules and representation, strict decoding: the `#canonical` sentence was split into three. The four behaviours (keeps a preceding `decErr`; `decPanic` to `decErr("Failed to decode as Bytes32")` for `bytes32()`; `decPanic` kept for other types; `decOk(V)` accepted only when `encodeValue(V) ==K L`, otherwise `The encoded value of type T is not in canonical form`) are all present with the same conditions and messages.
- and, Run-time checks: "requires nonempty, exclusively `boolV` inputs" became "requires at least one input and accepts only `boolV` inputs". Same two conditions; the `or` and `xor` entries still refer to "the same nonempty Boolean-input checks and messages as `and`", which remains consistent.
- load_constant, Run-time checks: "a curve constant can compute successfully while its gate reports a missing chip" became "the off-circuit computation of a curve constant can succeed while its gate reports a missing chip". Same claim, with the side of the computation made explicit.
- Running and testing: "prints `wfOk` here" became "prints `wfOk` for this program". Same referent (the `test_bool_gates.zkir` command shown above it).

Every number (61, 6, 9, 17, 418, 50, 364, four, 16777216, 31, `2^248`, priority 30), every path, every symbol, every message and every command is unchanged.

## Dropped material

None. No claim, sentence or list item present before is absent after. The word count rose from 1876 to 1896 because of the sentence splits and the added `reverse_bytes` name.

## Rule violations

None found.

- Em or en dashes: none.
- Placeholders or process words (draft, review, audit, persona, agent, editor, "this document", TODO): none.
- `#` titles: exactly one.
- Code blocks: byte-identical before and after.
- Tables: byte-identical before and after.
- Link targets: identical before and after.
- Inline code: identical apart from one added occurrence of `reverse_bytes` in the sentence about the removed instruction, which names an existing claim rather than altering one.

## Residual machine tells

None found. The remaining prose is declarative and specific throughout; no sentence in the edited chapter reads as machine-written by the patterns in the common brief (inflated framing, vague attribution, rule-of-three padding, negative parallelism, filler openers).
