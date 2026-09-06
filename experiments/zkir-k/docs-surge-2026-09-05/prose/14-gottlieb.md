# Gottlieb edit: 14-extending-the-semantics.md

Line numbers refer to the original text (3327 words including code and
tables, 91 sentences as the metric counts them, sentence_length_mean 36.6,
sentence_length_variance 1616.2, passive_voice_ratio 0.176, 186 semicolons
per ten thousand words).

## Critical findings

None. Every rule, function and constructor is located by file and symbol,
every message string is quoted, every count comes from a receipt. The
chapter's authority is intact; what needs work is its breath.

## Important findings

1. **Sentence-length uniformity, in the long direction.** The chapter's
   default sentence is thirty to ninety words, built as a colon followed by
   two or three semicolon-joined examples. The worst rolling windows are
   line 11 (one 70-word sentence carrying `less_than`, `reconstitute_field`
   and the hash instructions on two semicolons), line 98 (a 90-word chain of
   four `[owise]` dummies), line 131 (an 80-word sentence holding both
   removed overlaps, each with its fix in a parenthetical) and line 102 (a
   17-word parenthetical between "Under strict decoding" and its subject).
   Lines 7, 20, 30, 32, 54, 100, 106, 135, 137 and 163 follow the same
   pattern at smaller scale. Fix: break each chain at the semicolon where
   the examples are independent facts; keep a semicolon only where two
   clauses form one balanced pair (`vErr` off-circuit / `violated`
   in-circuit; `add` finishes / `transient_hash` does not terminate).
   Keep the list-sentences on lines 81, 133, 152 and 165, which are
   catalogues by design.

2. **Machinery in the prose.** Line 102 opens "Under strict decoding (the
   extension surface, and `load_constant` on either surface, which calls
   `decodeStrict(#immInts(Es), T, true)`)" and only then reaches
   `#canonical`. Line 106 hangs "(`ir_vm.rs`)" and a five-item list of
   unset fields on one sentence. Fix: state what strict decoding covers as
   its own sentence, then say what `#canonical` does; split line 106 at
   the semicolon. The parentheticals that are lists of names (lines 11,
   54, 163) stay; they are the content, not apparatus.

3. **Passive where the actor is on the page.** "is caught by `compare`"
   (line 20), "is forwarded by the `[owise]` of the six-ary `eval`" (line
   54), "are checked by `diff_test.py --ext`" (line 100), "is covered by
   `diff_test.py`" (line 163), "it must not be turned back into `holds()`"
   (line 135). Fix: make `compare`, the `[owise]`, `diff_test.py` and the
   reader the subjects. Kept: "is reserved for" (line 20), "is rejected"
   (lines 54, 133), "was reported as a success" (line 137), "is
   classified" (line 133), where naming the actor would add a claim the
   original does not make.

4. **The "rather than / instead of" reflex.** Ten contrasts in 3000 words:
   "rather than field mathematics" (9), "rather than `gate(I)`" (54),
   "instead of an outcome" (54), "rather than an `#exec` arm" (81),
   "instead of an error" and "rather than failing" and "instead of
   `violated`" and "never as a missing arm" (98), "not `alignedBytes`"
   (125), "without failing" (135). Each marks a distinction a maintainer
   would otherwise get wrong, and the chapter is about such distinctions.
   All kept; the density on line 98 is relieved by splitting the paragraph
   so that no single sentence carries two of them.

5. **Assertion by adverb.** "the chips the crate's `used_chips` actually
   enables" (table, line 47). The row's meaning is identical without
   "actually"; the contrast with chips the gates demand is made by line
   106. Fix: cut. "closely enough" (line 20) and "exactly one" (line 131)
   are measures, not emphasis, and stay.

6. **Filler.** "has to match" (line 133) for "must match"; "will steal"
   (line 129) for "steals"; "win over base `[owise]` fallbacks by being
   more specific" (line 32), where "specific" opens the same sentence.
   Fix: the plain forms.

## Tell audit

- Content tells: none. No significance, no promotion, no vague authority;
  every example is a named rule or a named tool.
- Language tells: no stock vocabulary; "actually" once (finding 5). No
  copula avoidance, no synonym cycling ("arm", "rule", "gate", "verdict",
  "outcome" keep their COMMON.md meanings throughout). Contrasts are
  finding 4.
- Style tells: no dashes (the double hyphens are command flags and table
  rules). Bold only as the pitfall lead-ins, which are structure. Tables
  where facts are parallel, prose where they are not. The rhythm problem
  is finding 1.
- Communication tells: "The files below move together" (line 36) is an
  instruction, not a stage direction; kept. No recap, no send-off; the
  chapter ends on where to read the totals.
- Filler and hedging: finding 6. No hedges; the one "None of this is
  implemented" (line 139) is a fact.

## Notes on voice and cadence

The short sentences are placed where a rule lands: "K merges every rule
for a function." (line 129), "None of this is implemented." (line 139),
"Skip a row only when the crate has no counterpart." (line 36). "A type is
an `IrType` constructor, a `Value` constructor, an encoding and a
decoder." (line 85) is the right opening for its section. The three-part
"A new `*V` formula or encoding belongs in the unit tools; a new program in
the differential harness; a new disagreement in the divergence suite."
(line 152) is deliberate parallelism and reads as such. The worked example
for `add` (lines 56 to 79) teaches the checklist by instance without a
word of commentary. The paragraph on stuck configurations (line 137) tells
a small history in the right order: what the runner does now, what
happened before, what rule closed the gap, what to check next time.

## Verdict

This is a maintainer's chapter: conventions, three checklists, a worked
example, six pitfalls and the commands to re-run. Nothing in it is wrong
and nothing in it is padding. Its one fault is that it was written at one
breath length, with semicolons doing the work periods should do, so that a
reader loses the second example of every sentence while parsing the first.
The fix is breaking, not cutting: roughly a dozen long sentences split at
their joints, five passives turned round, one adverb and two pieces of
filler removed. Every fact, path, symbol, message string and command stays;
the tables (one cell shortened by a word) and code blocks are untouched.
