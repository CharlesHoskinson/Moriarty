# Preservation check of 15-design-rationale-and-limits.md

Verdict: PRESERVED

Method: `diff -u` of the original against the edited chapter, then both read in full. Mechanical checks on the edited chapter: the sorted multiset of inline code spans is identical to the original; all link targets are identical; the code block and the fidelity table are byte-identical; every CLM identifier and every number occurs with the same multiplicity; no em or en dash; no process word; exactly one `#` title. Prose word count 1411 before, 1425 after.

Every limit and threat to validity keeps its stated strength. All eleven "not"/"does not"/"cannot"/"nothing"/"neither" negations that carry the limits survive unchanged in force: agreement is evidence not proof; the preprocessor is trusted and untested against Rust deserialization; gates do not allocate polynomial rows; verdicts are compared with nothing outside the definition; the K/Agda correspondence is structural only, no equivalence theorem; the Agda oracle has not run; the uninterpreted-hash treatment is not implemented; crate defects reproduced by the definition are undetectable; the small-input generator says nothing about unreached paths; one application witness only; no pin establishes later commits; the k-rust receipt does not separate unsupported construct from performance limit; open work is not discharged.

## Meaning changes

### C1 minor Relation to arc-zkir
Before: A specific contradiction also limits correspondence by reading.
After: One recorded contradiction also limits correspondence by reading.
What changed: "specific" became "recorded" and "A" became "One". "Recorded" is supported by the paragraph's own closing sentence ("This is the recorded K1 divergence"). "One" reads as an instance, not a count, though a reader could take it as "exactly one". No fact, symbol or strength changes.
Fix: if the count reading is unwanted, restore "A specific contradiction also limits correspondence by reading."

### C2 minor Threats to validity and open work
Before: a crate defect that the definition reproduces faithfully cannot be detected
After: a crate defect that the definition reproduces cannot be detected
What changed: the adverb "faithfully" is dropped. "Reproduces" already asserts the same behavior, so the blind spot is stated with the same strength.
Fix: optional; restore "reproduces faithfully" to keep the original wording.

### C3 minor Threats to validity and open work
Before: Generated inputs are small typed values (...), so agreement says nothing about paths they do not reach.
After: Generated inputs are small typed values (...); agreement says nothing about paths they do not reach.
What changed: the causal connective "so" became a semicolon; the limit itself ("says nothing about paths they do not reach") is verbatim.
Fix: optional; restore ", so agreement says nothing about paths they do not reach."

### C4 minor Threats to validity and open work
Before: the K6 divergence is the clearest case, since a non-canonical `Bytes<32>` element is an assertion panic at `92e8bdd3` and a decode error at `2ffe2d1`
After: The K6 divergence shows the difference: a non-canonical `Bytes<32>` element is an assertion panic at `92e8bdd3` and a decode error at `2ffe2d1`
What changed: the ranking "the clearest case" became "shows the difference". The facts (K6, `Bytes<32>`, panic at `92e8bdd3`, decode error at `2ffe2d1`, both modelled) are unchanged; only the superlative is gone. Not a limit.
Fix: optional; restore "the K6 divergence is the clearest case, since".

### C5 minor Symbolic reasoning
Before: `sbox` expands into field multiplication, ultimately exposing `modInt`.
After: `sbox` expands into field multiplication, which exposes `modInt`.
What changed: "ultimately" dropped; the unfolding chain (`absorbAll`, `permute`, `#rounds`, `sbox`, `modInt`) is intact.
Fix: optional; restore "ultimately exposing `modInt`".

No blocking finding: no fact, number, name, path, symbol, command, message or claim changed, and no claim was added.
No major finding: every "Must cover" item of briefs/15.md is present with the same content (plan decisions with CLM-0709, 0710, 0711, 0712, 0713, 0716, 0704, 0705, 0720, 0734; the fidelity table; the arc-zkir relation with the Assumptions record and the K1 contradiction; the Haskell experiment, hash unfolding and the unimplemented uninterpreted-hash approach; the oracle blind spots, the escrow, swap and micro-dao coverage figures, the abstraction limits, the version pinning; open work as repository facts).

## Dropped material

- "faithfully" (C2): harmless cut.
- "so" before "agreement says nothing" (C3): harmless cut.
- "the clearest case" (C4): harmless cut.
- "ultimately" (C5): harmless cut.
- No sentence, table row, code line or list item was dropped. All other edits are sentence joins with semicolons or "and", or reorderings within a paragraph (the intro's second paragraph and the arc-zkir paragraphs) with identical content.

## Rule violations

None. No em or en dash; no placeholder or TODO; no process word; one `#` title; code block, table, link targets and inline code identical to the original.

## Residual machine tells

- "The definition makes two things executable together for the Zero-Knowledge Intermediate Representation (ZKIR): concrete witness computation and instruction-level constraint checking." Pattern: announce-then-enumerate colon ("two things: ..."). The original stated the two items directly.
- "The K6 divergence shows the difference: a non-canonical `Bytes<32>` element is ..." Pattern: colon-fronted explanation replacing a causal clause.
