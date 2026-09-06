# Gottlieb edit: 13-known-divergences.md

Line numbers refer to the original text (2534 words including code and
tables, 86 sentences as the metric counts them, sentence_length_variance
1351.9, passive_voice_ratio 0.14).

## Critical findings

None. Every finding names its Rust function, its file, its K symbol and
its case; every message string is quoted; every status is the one the
harness prints. Nothing undermines the chapter's authority.

## Important findings

1. **Sentence-length uniformity (the note-form case line).** Thirteen
   sections close on the same three-beat shorthand: "Case `x`: A on both
   sides, gate B. Completeness. Fixed in PR #656." (lines 45, 56, 64, 76,
   98, 120, 128, 132, 136, 140, 144, 148, 156). One-word verdicts between
   two colon-and-semicolon lists read as a telegraph, and the worst
   windows are lines 56, 76 and 98, where three fragments stack. Fix:
   turn the case line into a sentence where the shorthand is merely the
   default register ("Case `k03_...` gives the input ... and then runs
   `reverse_bytes`. Both sides panic and the gate is unknown.") and fold
   the stacked verdicts into one clause. Keep the shorthand where a
   single one-word verdict sits between two full sentences (K4, f08,
   f13, f05): there it lands.

2. **Machinery in the prose.** The opening sentence (line 3) carries
   three parenthetical definitions inside a five-item list and runs past
   ninety words; line 7 does the same with the anatomy of a case; line
   102 stacks a semicolon, a parenthetical K fragment and a "because"
   clause in one sentence. Fix: break at the joints. Put the three
   relevance classes in their own sentence; give the anatomy of a case
   its own sentence; split line 102 at the semicolon. The parentheticals
   themselves are citations and stay.

3. **Semicolon saturation.** Thirty-nine semicolons in 2500 words, many
   splicing two independent facts ("`; the extension crate does the
   same`", line 43; "`; `#panicNow` sets `<status>`", line 43; "`; the
   gate reports`", lines 132, 136, 140; "`; `immediate` in`", line 171).
   Fix: a period where the two halves are separate claims; keep the
   semicolon where it pairs a fix with what remains open (lines 132, 136,
   152, 156), because that pairing is the point.

4. **Assertion by adverb.** "rejects the element cleanly" (line 98) is
   the one evaluative adverb; the fact it stands for is on line 80 ("an
   ordinary decode error"). Fix: say that. "exactly `(x, y)`" (line 39)
   glosses an Agda lemma and stays; the fourteen "only"s are quantities.

5. **An unanchored reference.** "Not among the review's findings" (line
   39) and "Outside the review" (line 120) name a review the chapter has
   not yet introduced; the section that does is at line 122. Fix: "the
   arc-zkir review" at the first use.

6. **Passive where the actor is on the page.** "The preimages are not
   written to disk" (line 15; the script is the subject of the previous
   paragraph), "any bounded field element is accepted
   (`bytes32FromLowHighV`)" (line 140), "Limbs ... decode, reduced, as 5"
   (line 156). Fix: make the script, the function and both sides the
   subjects. Kept: "is recomputed" (line 156) and "is fixed in" (lines
   152, 156), where the actor is either the circuit or a release.

## Tell audit

- Content tells: none. No significance, no promotion; every claim is a
  function, a rule, a case or a printed string.
- Language tells: no "not X but Y" beyond "on the curve but not in the
  prime-order subgroup" (line 152), which is the mathematical fact. No
  synonym cycling: "gate", "verdict", "outcome", "case", "finding" keep
  the meanings COMMON.md assigns them. No rule of three that is not a
  count.
- Style tells: no dashes (double hyphens are command flags and table
  rules). No bold. Tables where facts are parallel. The rhythm problems
  are findings 1 and 3.
- Communication tells: "Reproduction: `hex.zkir`" and "This prints
  status" are captions on examples and stay. No recap outside the
  Summary table, which is the chapter's index and stays.
- Filler and hedging: finding 4. "in principle" (line 144) and "only"
  (line 45, "Availability only") are qualifications with content.

## Notes on voice and cadence

The chapter's best sentences are the short ones that follow a long one:
"In circuit, `point_from_coordinates` pins the exact pair." (line 25)
after the decompression sentence; "No false proof results;" (line 39);
"`usedChips` in `zkir-constraints.k` reproduces the scanner." (line 62).
The K6 section moves cleanly from the two crates to the definition to
the two reproductions to the verdict, and its closing sentence has the
right shape. The loader section says what happens on each side and then
"No semantic effect", which is the correct place to stop. The
classification paragraph at line 124 is four plain sentences that sort
thirteen findings; nothing in it needs to change.

## Verdict

This is a catalogue of divergences, each with its reproduction and its
upstream status, and it should stay one. It needs its case lines turned
from note-form into sentences where three fragments stack, its three
long list-sentences broken at their joints, about a third of its
semicolons replaced with periods, one adverb replaced with the fact it
gestures at, and one reference anchored. Nothing is deleted; every
number, path, symbol, message string, command and status stays as it is;
the code blocks, the JSON, and both tables are untouched.
