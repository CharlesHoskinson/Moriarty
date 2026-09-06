# Gottlieb edit: 15-design-rationale-and-limits.md

Line numbers refer to the original text (1842 words including code and
tables, 103 sentences as the metric counts them, sentence_length_mean
17.9, sentence_length_variance 101.9, passive_voice_ratio 0.136).

## Critical findings

None. The chapter cites a file and a symbol for every rule it describes,
a receipt for every number, and the plan for every decision. The limits
are stated once each, at one strength, and nothing in the prose
undermines them.

## Important findings

1. **The "not X but Y" reflex.** The chapter is about limits, so
   negation is its content; the defect is the shape, not the substance.
   The appositive "X, not Y" closes eight sentences: "evidence ..., not a
   proof" (line 3), "operations on integers, not a simulation" and "a
   totality convention, not field mathematics" (line 17), "`preprocess`,
   not `Relation::circuit`" and "a separately encoded relation, not a
   second execution" (line 37), "the functional relation it guarantees,
   not its gate-level lowering" (line 55), "only for those witnesses, not
   universal completeness or soundness" (line 71), "they are not counts"
   (line 73). Two on line 17 sit three sentences apart. Fix: keep the
   thesis (line 3), the oracle pair (line 37) and the parenthesis on line
   55, where the contrast is the point; turn "not a simulation" (line 17)
   into a full clause ("they do not simulate ...") and fold "not counts"
   (line 73) into its sentence with "and".

2. **Sentence-length uniformity.** Mean 17.9 words with variance 101.9:
   most sentences run 11 to 20 words and the reader's breath never
   resets. The worst rolling windows are line 29 (four sentences of 15,
   13, 11 and 14 words, each "subject verb object"), line 67 (three
   sentences of the same shape), and line 73 (two receipts introduced by
   the identical "The X receipt records N agreeing comparisons, including
   M successful-run agreements", then two short verdicts). Fix: join the
   second and third sentences of line 29, which describe one mechanism;
   join the plan statement and its "not implemented" on line 67 with a
   semicolon; put both receipts into one sentence on line 73 and let
   "Neither records a successful K run with a non-holding gate" stand
   short.

3. **Voice in the workshop.** Line 5 opens with "Statements about the
   implementation describe the files as they are; statements about
   results cite the receipt ...; the reasons behind decisions are those
   recorded in ..." Three parallel clauses about the chapter's own
   sourcing, before any fact. The conventions are useful (the reader
   needs to know where reasons come from) but the sentence is a preface
   about the prose. Fix: lead with the plan, which is the source the
   reader will follow, and state the other two conventions in one plain
   sentence; keep the filename rule.

4. **Assertion by adverb.** "reproduces faithfully" (line 71): the
   sentence already says the defect is reproduced, and "faithfully" adds
   only emphasis. "A specific contradiction" (line 59): "specific" does
   no work that the next sentence, which names the contradiction, does
   not do better. "the clearest case" (line 77): a ranking the chapter
   does not defend. Fix: cut "faithfully"; "One recorded contradiction";
   "The K6 divergence shows the difference". Kept: "exactly one
   comparison" (line 75) is a count; "solely because" (line 35) carries
   the distinction between the runner's two checks; "merely because"
   (line 55) is the argument.

5. **Same connective three times running.** Line 71 hangs three
   consecutive consequences on ", so": "so a crate defect ... cannot be
   detected", "so no verdict is ever compared", "so agreement says
   nothing". Fix: keep two, and let the third be a semicolon.

6. **A stranded sentence.** Line 25: "The `#put` rule overwrites
   registers as the crate does." sits between static checking and single
   assignment with no joint on either side. The next sentence is its
   reason for being there. Fix: keep it, but make the sentence after it
   follow on ("Single assignment is a static restriction, needed when
   ..."), so the pair reads as one thought.

## Tell audit

- Content tells: none. No significance, no promotion, no vague
  authority; every source is a file, a symbol, a receipt or the plan.
- Language tells: finding 1. No stock vocabulary ("realizes" on line 29
  is used in its technical sense and is kept). No copula avoidance: the
  chapter says "is" where it means "is". The three-item list on line 37
  ("compare ..., evaluate ..., and obtain ...") is the plan's list, not
  a rule-of-three; kept. The three-item list on line 53 is the Agda
  correspondence and has three items; kept.
- Style tells: no dashes (the metric's 2.17 per thousand words counts the
  table rule `|---|---|---|`). No bold. One table where seven parallel
  facts belong in one. The rhythm problems are findings 2 and 5.
- Communication tells: finding 3. No recap, no cheerleading, no "this
  chapter". Cross-references ("See 08-...") are pointers, not narration.
- Filler and hedging: findings 4 and 5. "Thus" (line 31), "therefore"
  (line 37), "consequently" (line 25) each mark a real inference and are
  kept, though "consequently" becomes "therefore" so the chapter uses one
  word for one job. No stacked qualifiers; the limits are stated flat.

## Machinery kept on purpose

The `file`, `symbol`, appositives and the (CLM-nnnn) parentheticals are
citation machinery inline, and Gottlieb would footnote them. Here they
stay: the chapter rules require file and symbol for every rule described,
and the CLM identifiers are how the plan is cited. Neither is touched.

## Notes on voice and cadence

The chapter opens on its thesis in two sentences and never restates it.
The short sentences land where the argument turns: "The purposes differ."
(line 55), "The implementation explains the obstacle." (line 65),
"Oracle agreement has shared blind spots." (line 71), "Version pinning
bounds every result." (line 77). Each is followed by the evidence and
none is followed by a recap. The fidelity table (lines 41 to 49) puts the
scope of the model in seven rows with the limit in the same cell as the
treatment, which is where a reader looking for the limit will look. The
`expire.zkir` paragraph (line 75) gives the one successful application
witness with its attempt number, register count and public-input count,
and then says what one witness does not show; the specificity is what
makes the limit credible. The closing paragraph is a list of what is
missing and stops on the pointer to the next chapter.

## Verdict

This is a reference chapter that says why the definition is built the
way it is and what its results do and do not establish. It needs three
appositive negations turned into clauses, three paragraphs' rhythm
broken up, one preface about its own sourcing flattened into plain
statement, three adverbs cut, one connective varied and one stranded
sentence joined to its neighbour. Nothing is deleted; every number,
path, symbol, message string, command, claim identifier and limit stays
at its current strength; the table and the code block are untouched.
