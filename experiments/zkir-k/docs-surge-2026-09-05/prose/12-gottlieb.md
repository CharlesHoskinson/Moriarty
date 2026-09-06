# Gottlieb edit: 12-oracles-and-differential-testing.md

Line numbers refer to the original text (2039 words as the lint counts
them, 105 sentences as the metric counts them, sentence_length_variance
589.2, passive_voice_ratio 0.114, adverb_per_10000_words 85.1).

## Critical findings

None. Every claim is anchored to a file, a symbol, a receipt or a printed
line, and nothing in the chapter overstates what the evidence shows. The
chapter's thesis (what each oracle does and does not establish) is stated
once at the top and then earned, section by section.

## Important findings

1. **Sentence-length uniformity (the telegraph).** The worst rolling
   window is the oracle's failure paragraph (line 37): "The Python adapter
   maps exit code 101 to `panic`; other nonzero exits become `load-error`.
   Loading and preimage conversion fail before preprocessing. In
   particular, `fr_from_dec` rejects noncanonical field elements. The
   oracle therefore provides no partial Rust memory for failed runs." Four
   beats of the same length, and the third ("fail before preprocessing")
   can be read as "always fail". Protocol step 5 (line 51) has the same
   shape: "Every attempt is compared." and "Stop attempting after both
   sides return `ok`." are two four-to-seven-word sentences bracketing a
   longer one. Fix: join the loading sentence to its example and its
   consequence; fold the two short sentences of step 5 into the sentences
   they qualify.

2. **Voice in the workshop (mood drift).** The protocol is narrated in
   the declarative ("`compare` first requires equal statuses", line 66)
   but four steps slip into the imperative as if addressing an operator:
   "call the oracle" (line 45), "Execute `Runner.run(program, pre)`" and
   "Stop attempting" (line 51), "compare each applicable mutation" (line
   53), "require identical `pis`" (line 68), "inspect `D`" (line 76). The
   reader cannot tell whether the harness does these things or they are
   being asked to. Fix: give each its subject (the harness, `compare`)
   and keep the imperative only where a command is being handed to the
   reader (the reproduction commands and "redirect `OUT` to scratch").

3. **The "X, not Y" reflex.** Eight scope limits close on the same hinge:
   "not general parser equivalence" (45), "does not guarantee a
   successful witness" (49), "not interchangeable" (64), "not distinct
   successful programs" (87), "does not amount to transaction-context
   coverage" (87), "not an executable differential oracle" (139), "not
   Rust proof acceptance" (112), "not a maintained proof suite" (157).
   Each is a real claim and each stays; the chapter exists to draw these
   lines. The defect is the shape, which turns an argument into a
   refrain. Fix: keep the two that carry the most weight verbatim ("raw
   `job`, not `checkedJob`", "recorded experiments, not a maintained proof
   suite"), and let the others take "rather than", a subordinate clause or
   a plain negative so the ear stops predicting the tag.

4. **Machinery in the prose.** "without emitting K's intermediate
   representation, KORE, and a full-definition execution attempt" (line
   152) parks an abbreviation expansion in the middle of a three-clause
   sentence, so "KORE, and" reads as a list. "built from `Path.home()`
   independently of the current directory" (line 23) lets "built" attach
   to the binary when it is the path that is built. "The receipts do not
   record build durations" (line 23) sits between the binary's calling
   convention and the tools' path, interrupting both. Fix: move the
   expansion into its own clause and break the sentence at the
   semicolon; say the tools construct the path; leave the durations
   sentence where it is, since the build block it refers to is directly
   above.

5. **The over-long joint.** The Haskell paragraph (line 156) carries the
   receipt, the claim, its timing and its precondition in one 48-word
   sentence, then hangs the missing precondition ("Omitting the explicit
   cells leaves decoding stuck") two sentences later. Fix: split after
   "Haskell backend", pair the precondition with its converse, and let
   "The claim module is not in the repository" stand as its own sentence
   before the postcondition it governs.

6. **A dangling attribution.** "Budget approximately fifteen and thirteen
   minutes respectively from the receipts" (line 96) makes "from the
   receipts" modify "minutes" while the table two paragraphs above shows
   387 s and 281 s. The figures are what the receipts support (the slower
   recorded runs) and stay; the sentence should say the receipts support
   the budget, not that the minutes come from them.

7. **Assertion by adverb.** "explicitly uses `checked=True`" (line 64),
   "separately tests static expectations" (line 64), "independently of the
   current directory" (line 23), "In particular" (line 37). The concrete
   fact certifies each; the adverb is a nudge. Fix: cut "explicitly" and
   "separately" (the code passes the flag; the second tool is its own
   script), recast the other two. Kept: "temporarily bind defaults" (the
   binding is undone), "repeatedly reports" (the backend loops), "does not
   compare it directly" (it is compared through `type` and `encoded`).

## Tell audit

- Content tells: none. No significance, no promotion, no vague authority;
  every source is a file, a symbol, a receipt line or a printed field.
- Language tells: finding 3 (the "X, not Y" hinge). No stock vocabulary,
  no synonym cycling: "oracle", "harness", "adapter", "receipt", "gate"
  and "verdict" keep the meanings COMMON.md assigns them. "reside outside
  the repository" (line 157) is the one elevated verb; "are" does the job.
- Style tells: no dashes (the double hyphens are command flags and table
  rules). No decorative bold; the bold step names in the numbered list
  are labels, kept. Tables carry parallel facts and the prose reads them.
  The rhythm problem is finding 1.
- Communication tells: "The differential receipts record these experiment
  observations:" (line 80) is a caption that has swallowed two nouns;
  "record the following" says it. No recap, no cheerleading, no narrating
  of the writing.
- Filler and hedging: none beyond finding 7. "at most eight" (49), "up to
  eight attempts" (87), "about ten seconds" (156), "within ten minutes"
  (152) are quantities from the receipts and stay.

## Notes on voice and cadence

The opening does its job in three sentences and never has to repeat it:
the third, "Agreement with `preprocess` does not establish agreement with
a Rust circuit prover or equivalence for every program", is the line the
whole chapter rests on. "The differential harness uses raw `job`, not
`checkedJob`" (line 64) is the four-beat sentence a subsection should open
on. "Because the oracle never runs the crate's `circuit`, the K verdicts
are compared with nothing on the Rust side" (line 76) is the best sentence
in the chapter: a mechanism, its consequence, and no adjective. The
`expire.zkir` sentence (line 87) earns its length by carrying five numbers
the reader will want together. The two tables of the oracle section and
the results section are the right form for their facts, and the mutation
table's three columns say in twenty words what a paragraph would have
said in eighty.

## Verdict

This is a reference chapter about what the evidence is, how it is made and
where it stops. It needs one paragraph's telegraph broken up, six
imperatives given their subjects, the "X, not Y" tag rationed so it lands
where it matters, one abbreviation expansion moved out of a list, the
Haskell sentence split at its joint, and one attribution untangled.
Nothing is deleted; every fact, number, path, symbol, message string,
receipt name and command stays; the tables and code blocks are untouched.
