# Gottlieb edit: 02-getting-started.md

Line numbers refer to the original chapter. Code blocks (lines 9-11, 17-39,
58-63, 71-75, 79-83, 87-223, 231-237, 247-255, 259-266, 270-340, 350-407,
411-437, 445-496, 504-539) and the two tables are out of scope.

## Critical findings

1. **Sentence-length uniformity.** Paragraph at line 225 (the walk through
   the first JSON object) is fourteen sentences of one shape, "`field` is
   value: reason", with three semicolon-spliced sub-clauses in the middle
   (`Bytes32`); ... finished; ... `error`; ...). The reader's breath never
   resets and the three distinct topics (status and memory; public inputs,
   cursors and the k cell; constraints and verdicts) run together. Fix: split
   into three paragraphs at the topic joints, turn the `bytes32V` parenthesis
   and the "on the extension surface" clause into sentences, join
   `pi_skips` and `cursors` (both "empty/zero because nothing happened"),
   and cut the residual-term sentence at "finished".
2. **Subjectless fragments.** Lines 508, 514, 520, 526: "Field, curve and
   encoding functions of `ZKIR-TEST` against ...", "Known-answer checks for
   ...", "Runs `ZKIR-CHECK` on ...", "Rewrites the twenty programs ...".
   Four consecutive tool descriptions with the subject dropped, each
   followed by the same two-beat coda "Ends with X. Needs the oracle."
   Fix: give each its subject (the script name) and fold the oracle
   requirement into the sentence that explains why it is needed.
3. **Machinery in the prose.** Line 245: "pass `--gen`, which runs
   `genJob` (`zkir-vm.k`, the rule for `genJob` sets `<genMode>` to
   true)". A parenthesis that contains a full clause with its own subject
   and verb stops the sentence mid-thought. The file-and-symbol citation is
   required; the clause is not a citation. Fix: close the sentence at
   `genJob` and state the rule as its own sentence.

## Important findings

4. **Voice in the workshop.** Line 3: "This chapter goes from a clean
   checkout to a first run". A chapter orientation sentence is acceptable
   in a reference set; "goes from" makes the chapter the traveller. Fix:
   "takes a clean checkout to a first run", one word, and leave the rest.
5. **The "not X but Y" reflex.** Line 225 has two in one paragraph ("Map
   order is the K `<mem>` walk, not instruction order"; "records that a
   limit was given, not that it was reached"); line 550 has a third. Each
   carries a real distinction, so none is cut, but the paragraph split in
   finding 1 separates the first two so they no longer read as a tic.
6. **Buried imperative.** Line 43: the sentence that tells the reader when
   to run `check_k_toolchain.sh` ("Run it after a K or pyk upgrade") is
   the last of three, after a 60-word inventory of what the script does.
   Fix: lead with the instruction, then the inventory.
7. **Loose term.** Line 439: "`reassignment.zkir` is the dual". "Dual" has
   a technical meaning that does not apply; the sentence means the opposite
   case (`job` accepts, `--checked` rejects). Fix: say "the opposite case".
8. **Signposting.** Line 225: "`needs` is filled only by `--gen` (below)".
   The next section is titled for `--ext` and `--gen`; the pointer is
   clutter. Fix: cut "(below)". The "below" on line 239 stays: it points
   two sections ahead, where the reader would not otherwise look.
9. **Assertion by verb.** Line 498: "demands that the condition pass
   `asBool`". "Demands" dramatises a guard rule. Fix: "requires the
   condition to pass".

## Tell audit

- Content tells: none. No inflated significance, no promotional framing;
  every example is a named file with its real output.
- Language tells: no stock vocabulary, no copula avoidance, no false
  ranges. The "not X but Y" pair is finding 5. Rule-of-three appears only
  where there are three things (three inputs, three cursors).
- Style tells: no dashes outside command flags, no decorative bold. The
  uniform rhythm of the line 225 paragraph and the four fragments are
  findings 1 and 2.
- Communication tells: no chatbot residue, no recap. "(below)" is finding
  8. "The quoted last lines are from live runs" (line 502) is a statement
  about the output shown, not about the writing, and stays.
- Filler and hedging: none. `hedge_word_density` is 0.0, adverb density
  0.004; the adverbs present ("only", "still", "strictly") each restrict a
  claim rather than certify one.

## Notes on voice and cadence

The short-sentence resets that exist are well placed: "`%a` was loaded."
(line 409) before the explanation of the emitted gate, and "Generation mode
produces that value." (line 245) as the answer to the problem the previous
paragraph set up. The two-paragraph "On a well-formed program ... On a
well-formedness error ..." at line 239 is deliberate parallelism and reads
as such. The "Common problems" table earns its shape: three columns of
parallel facts, no prose forced into cells. The failure section's order,
raw run first and `--checked` second, then the off-circuit/in-circuit split,
teaches the reader the two kinds of failure without saying so.

## Verdict

The chapter is a working getting-started guide that already trusts its
reader: it names files and symbols, shows real output, and does not
editorialise. What is wrong is local and rhythmic rather than structural:
one long paragraph that narrates a JSON object in a single unvarying
register, four tool descriptions written as fragments, one parenthesis
carrying a clause that belongs in a sentence, and a handful of words
("dual", "demands", "(below)") doing slightly the wrong job. The fix is
rewriting at sentence level, not deletion; nothing needs to go except the
one pointer. Every fact, number, command and code block stays as it is.
