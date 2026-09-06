# Gottlieb edit: 10-well-formedness-and-static-checks.md

Line numbers refer to the original text (2044 words including code and
tables, 56 sentences as the metric counts them, sentence_length_variance
1328.3, passive_voice_ratio 0.196).

## Critical findings

None. Every predicate names its K symbol, every message is quoted, every
count is the one the tools print. Nothing undermines the chapter's
authority.

## Important findings

1. **Sentence-length uniformity (same opener, same shape).** The corpus
   check (lines 80 and 86) is the worst rolling window: eight consecutive
   sentences of the form "The X is Y" or "X is Z-ed" ("Programs ... are
   skipped", "Every file is loaded", "The expectations live", "The corpus
   manifests record", "The last line is", "the exit status is", "The
   receipt is", "The 63 programs are", "The five ledger tests ... are").
   Each is a fact, none is wrong, and together they read as a telegraph.
   Fix: join the two passives on line 80 into one active sentence with
   the consequence it already states; attach the receipt to the summary
   sentence on line 86; open the count sentence on the number, not on
   "The 63 programs".

2. **Passive voice where the actor is known.** "Checks 3 to 5 are applied
   ... by `#wfInstrs`" (line 17), "a run-time error is raised only when"
   (line 124), "The verdicts are evaluated with" (line 142), "cannot be
   produced from a file" (line 76). In each the actor is named in the same
   sentence or the one before. Fix: make `#wfInstrs`, the VM and a file the
   subjects. Kept: "is the one reported" (line 24), "is shortened" (line
   90) and "is rejected as well" (line 141), where the actor is either
   obvious or not the point.

3. **The over-long joint.** Line 104 carries `job` and `checkedJob` in one
   57-word sentence joined by a semicolon, then hangs `checkedJob`'s three
   actions on one "and ... and". Fix: split at the semicolon; inside the
   `checkedJob` sentence, separate "evaluates `wf(P)` first" from what
   happens on failure.

4. **Flourish at the close of an argument.** "enforces that condition, no
   more and no less" (line 141). The sentence has already made its point;
   the tag is rhythm without content, and the next sentence ("It does not
   inspect values") is the real "no more". Fix: "enforces that condition
   alone", which keeps the claim that `#wfWrites` checks nothing beyond
   single assignment.

5. **Two facts sharing a sentence.** Line 24 ends on `N`, which glosses
   the rule block above, after two sentences about the order of check 3.
   The gloss stands but is unanchored. Fix: "The `N` in these rules is
   ..." so the reader knows what it refers to.

## Tell audit

- Content tells: none. No significance, no promotion; the sources are
  files, symbols, commands and their printed output.
- Language tells: four "X, not Y" contrasts in 2000 words (lines 80, 124,
  137, 141). Each marks a distinction the reader would otherwise get wrong
  (manifests do not hold expectations; gates name registers, not values;
  gate outcome versus error). All kept. No synonym cycling: "predicate",
  "check", "gate", "verdict" and "outcome" keep the meanings COMMON.md
  assigns them.
- Style tells: no dashes (the only double hyphens are command flags and
  table rules). No bold. Tables where facts are parallel; the two
  paragraphs that read the tables are prose. The one rhythm problem is
  finding 1.
- Communication tells: "This chapter covers the first question" (line 3),
  "shows the difference" (line 124) and "shows the effect" (line 142) are
  captions that tell the reader what an example is for; kept. No recap, no
  cheerleading.
- Filler and hedging: finding 4. "exactly one predicate" (line 90),
  "exactly two output identifiers" (line 48) and "strictly fewer" (line
  104) are quantities, not emphasis, and stay. "(the time varies)" (line
  86) is a fact about the receipt.

## Notes on voice and cadence

The chapter opens on its thesis in one balanced sentence, "Whether a
program is well formed holds or fails for every preimage; whether a run
succeeds depends on the preimage" (line 3), and everything after it is
one side or the other of that line. "The crate has no static pass."
(line 104) is the four-word sentence the whole comparison table rests on.
"The crate agrees:" (line 142) is the right length for a confirmation.
The `add %x, %x -> %x` and `copy %a -> %a` pair (line 24) teaches the
check order by two examples where a paragraph of explanation would have
been slower. The closing paragraph earns its last sentence: it moves from
the trace to the crate to the general statement about verdicts, and stops.

## Verdict

This is a reference chapter that says what each check is, where it lives,
and what the reader sees when it fails. It needs one paragraph's rhythm
broken up, four passives turned active where the actor is already on the
page, one long sentence split, one flourish trimmed, and one dangling
gloss anchored. Nothing is deleted; every fact, count, path, symbol,
message string and command stays as it is; the tables and code blocks are
untouched.
