# Preservation check of 04-values-and-encoding.md

Verdict: PRESERVED

Method: `diff -u` of the original against the edited chapter, then both versions read in full. The diff touches 17 prose paragraphs in 6 hunks; every change is a sentence split, a semicolon or colon turned into a full stop, a reordering of clauses inside one sentence, or a pronoun swap. Every table row, code block, link target and heading is identical in both versions (checked mechanically: the `|` table lines, the fenced blocks, the chapter cross-references and the `#` headings all diff clean). The multiset of inline code spans differs by two tokens, both discussed below. Prose word count (tables and code excluded) moved from 1962 to 1988.

## Meaning changes

### C1 minor Inputs, transcripts, public inputs and outputs
Before: A short transcript gives `panic("range end index out of range: public transcript outputs too short")` or its private counterpart, as `preprocess` slices before checking (divergence K2, see 13-known-divergences.md).
After: A short transcript gives `panic("range end index out of range: public transcript outputs too short")` or its private counterpart, because `preprocess` slices before checking (divergence K2, see 13-known-divergences.md).
What changed: "as" (K panics in the same way that `preprocess` does, since it slices before checking) became "because" (the K panic is caused by the Rust behaviour). The Rust fact, the message, the status and the divergence reference are unchanged; only the connective moved from comparison to causation. Both readings are true of the semantics, since the K rule exists to mirror `preprocess`.
Fix: restore "or its private counterpart, as `preprocess` slices before checking" if the comparative reading is preferred.

### C2 minor The Value sort
Before: Twisted Edwards values (Jubjub, Curve25519) never carry `inf()`, their identity being `pt(0, 1)`; short Weierstrass values (secp256k1, secp256r1) use `inf()`.
After: Short Weierstrass values (secp256k1, secp256r1) use `inf()`; twisted Edwards values (Jubjub, Curve25519) never carry it, because their identity is `pt(0, 1)`.
What changed: the two clauses swapped order and the second occurrence of the inline code span `inf()` became the pronoun "it". The antecedent is the `inf()` of the preceding clause, so the claim is the same; the span is no longer greppable in that sentence.
Fix: restore "never carry `inf()`, because their identity is `pt(0, 1)`" if every mention of the constructor should stay in backticks.

### C3 minor Encoding to raw field elements
Before: and pack `F::CAPACITY / LOG2_BASE` limbs into each element, the capacity of the BLS12-381 scalar field being 254.
After: and pack `F::CAPACITY / LOG2_BASE` limbs into each element. For the BLS12-381 scalar field, `F::CAPACITY` is 254.
What changed: a second inline code span `F::CAPACITY` was added, naming the constant the original called "the capacity". The number 254, the field and the formula are unchanged, and the name already appears in the same sentence, so no new claim is introduced.
Fix: none needed; to restore the original wording use "into each element, the capacity of the BLS12-381 scalar field being 254."

No blocking finding: every file path, commit hash, symbol, number, error message, gate name, harness class, test-case name, command and link target in the edited chapter matches the original. The other reworded sentences were read side by side and carry identical meaning: the introduction now makes `encode_offcircuit` and `decode_offcircuit` the subject of "delegate" where the original attached "delegates" to `encode.rs`, which is the same claim; the Weierstrass identity sentence replaces "it encodes" with "the identity encodes", which names the antecedent the original relied on; the `#commCheck` sentence moves the gloss of `transientCommit(...)` into a following sentence; the `commGate` sentence prefixes its four outcomes with "Its outcome is"; and the `typeOf`, `typeName`, `tn` and `defaultValue` definitions are regrouped without change.

## Dropped material

- One inline `inf()` in the twisted Edwards sentence, replaced by "it" (see C2). Harmless cut.

No sentence, list item or table row is absent. Everything the chapter brief lists under "Must cover" is present in the edited version: the 13 constructors with payloads, `typeOf` and `typeName`; `encodeValue` with the per-type layouts, the Bytes32 split, the Jubjub scalar single batch, the foreign limb table with `LOG2_BASE`, `NB_LIMBS`, limbs per element and the value-minus-one shift; `decodeValue` and `decodeStrict` with the range checks, `decErr` against `decPanic`, the native range check, the foreign decoding guards and the `#canonical` re-encode check; `#loadInputs`, `<pi>`, `#seedPi`, `#input`, `#encode`, `#output1`, `#commCheck` and `#encodeAll`; the error message table with its harness classes; and the cross-check table with `encodedLen`, K rule, Rust function and `unit_values.py` checks.

## Rule violations

- Em or en dashes: none.
- Placeholders or TODO markers: none.
- Process words (draft, review, audit, persona, agent, editor, "this document"): none.
- `#` titles: exactly one (`# Values and encoding`).
- Code blocks: identical before and after.
- Tables: identical before and after.
- Link targets: identical before and after.
- Inline code: no span altered; one `inf()` dropped in favour of a pronoun (C2) and one `F::CAPACITY` added (C3).

## Residual machine tells

- "The payload column is the invariant the VM maintains, not a sort constraint" (The Value sort): contrastive "X, not Y". Present in the original; the contrast is load-bearing.
- "`<pi>` is the public-input vector of the proof and holds raw integers, never values." (Inputs, transcripts, public inputs and outputs): "X, never Y" contrast. Present in the original; load-bearing.
- "That term is the Poseidon hash of the randomness, the raw inputs exactly as given, and the encoded outputs" (Inputs, transcripts, public inputs and outputs): a fronted demonstrative opening a sentence created by the split reads slightly mechanical, but the referent (`transientCommit(...)`) is unambiguous and the meaning is unchanged.
- "This comparison covers the types that have no dedicated unit check in the table below." (Inputs, transcripts, public inputs and outputs): another demonstrative-fronted sentence produced by a split; clear and unchanged in meaning.
