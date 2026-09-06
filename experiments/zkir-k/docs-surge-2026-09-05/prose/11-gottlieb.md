# Gottlieb edit: 11-tooling-reference.md

Line numbers refer to the original text (2492 words in the 50 prose
paragraphs outside tables and code, 3760 words outside code; the KPI
script counts 128 sentences and a sentence_length_variance of 1275.2,
inflated by table rows that it reads as single sentences).

## Critical findings

None. Every claim names a file, a symbol, a flag, a message string or an
exit code. Nothing is inflated, nothing is vague, and the tables carry the
parallel facts. The authority of the chapter is not in question; its
readability is, in the places below.

## Important findings

1. **Sentence-length uniformity (the long end).** Eight prose sentences
   run past forty-five words, and each is a chain of clauses hung on
   semicolons and parentheses rather than an argument that needs the
   length: the `compare` rule (line 222, 57 words from its subject to
   "not compared"), the `unit_values.py` check description (line 249, 70
   words), the `divergence_tests.py` preimage sentence (line 232, 48
   words, two parentheticals and a participle), the `gate_sub` sentence
   (line 243, 49 words), the case-output sentence (line 245, 51 words),
   the `cc` reading rule (line 47, 46 words), the serde list (line 86, 45
   words) and the hash-check comparison (line 259, 45 words). Fix: break
   each at its natural joint. `compare` splits by status (`error`/`panic`
   on one sentence, `ok` on the next); line 249 splits between "compares
   the pretty result with an expected value" and what that value is; line
   232 splits before the commitment step; line 245 splits before the
   footer. The serde list at line 86 is a real list of five conditions and
   stays.

2. **Machinery in the prose.** Parentheticals stacked inside a sentence,
   sometimes inside each other: the exit-1 sentence at line 126 has a
   parenthesis inside a parenthesis ("(a missing program or preimage
   file, a `communications_commitment` with fewer than two elements
   (`IndexError`) or one that is not indexable, such as a JSON number
   (`TypeError`), a missing kompiled directory)"); line 13 puts the build
   flags and `backend.txt` in one bracket after the verb; line 24 stops
   the `krun` sentence with "(which runs the compiled `interpreter`)";
   line 249 carries three brackets; line 308 brackets the `hash/` and
   `ecc/` condition. Fix: turn the outer bracket into a clause or a
   sentence of its own and keep every item. The short glosses
   (`ZkirFormatError`, `IndexError`, `TypeError`, the flag lists) stay in
   brackets; they are labels, not asides.

3. **Semicolon as the default joint.** Forty-seven semicolons in fifty
   prose paragraphs, with four in the paragraph at line 243, three each at
   lines 152 and 212, and two each at lines 9, 24, 36, 222, 259, 294, 317,
   321, 331 and 359. Most join clauses that are independent statements
   ("`krun` must be on `PATH`", "the remaining fields are still filled",
   "callers must read `status`", "a missing binary is
   `FileNotFoundError`"). A semicolon every other sentence reads as one
   long breath. Fix: keep the semicolons that hold a real parallel (the
   two corpus lists at line 212, `panic` against `load-error`, the
   `PASS`/`FAIL` row against its footer) and give the rest a period or an
   "and".

4. **Docstring fragments.** A new pattern for the catalogue. Eleven
   sections open on a verb or noun phrase with no subject, in the manner
   of a Python docstring: "Preprocessor from a `.zkir` JSON file" (line
   86), "Runs `job`, `checkedJob` or `genJob`" (112), "Differential
   harness against the oracle. Protocol only;" (198), "Twenty executable
   cases" (226), "Forty-three checks" (249), "Eighteen known-answer
   checks" (259), "Well-formedness of every version-3 program" (269),
   "Writes the positive handmade programs" (298), "Regenerates" (308),
   "Attaches" (321), "Python reference for" (331). Nine sections close
   the same way: "Depends on ..." (108, 194, 222, 245, 255, 265, 294, 304,
   327), and three carry "Exit 0 if ..., else 1." as a fragment (255,
   265, 294). Each fragment is correct and each is the same shape, so the
   chapter reads as a manifest of eleven docstrings. Fix: give each opener
   its subject (the tool name the heading has just given, so the sentence
   reads as prose rather than as a repeat), and keep the dependency
   sentence in its closing slot as a sentence with a subject. The
   convention of where the facts sit is worth keeping; the fragment is
   not.

5. **Repetition without rhythm.** "No argument parsing: every argument,
   including `--help`, is ignored and the tool runs immediately" appears
   verbatim five times (lines 226, 249, 259, 298, 321), three of them
   with "(and writes its files)" attached. Each section must state the
   fact, because a reader lands on one tool at a time, so the refrain is
   legitimate; what marks it as assembled is that it is a fragment and
   that the bracket at the end is an afterthought. Fix: keep the refrain,
   word it identically each time so the repetition reads as deliberate,
   and fold "and writes its files" into the sentence.

6. **Assertion by adverb.** "the target gate is selected exactly from
   `all_verdicts`" (line 234): the code block that follows is the exact
   selection, and "exactly" adds nothing it does not show. Fix: cut, and
   let the colon point at the code. Everything else is restrictive and
   factual and stays: "only" at lines 47, 124, 152, 198, 255 and 259,
   "never" at 9, 32, 259 and 359, "still" at 141, 194 and 359, "exactly
   a two-element array" at 47 (a quantity).

7. **Voice in the workshop.** "(whitespace condensed here)" (line 167).
   The note is necessary, because `main` prints one element per line and
   the block does not, but "here" points at the page. Fix: "with
   whitespace condensed". "Protocol only;" at line 198 is a scope note
   with no subject; it becomes "Only its protocol is described below".

## Tell audit

- Content tells: none. No significance, no promotion, no vague authority.
  Every source is a file, a K symbol, a Python function, a flag or a
  message string, and every number is a count the reader can reproduce.
- Language tells: no stock vocabulary. The contrasts ("not a `preimage
  error`", line 36; "not against `encode_value`", 152; "are not
  compared", 222; "not the CLI's `format error:` prefix", 277; "not a
  CLI", 331; "does not use `zkir-check-kompiled`", 359) each name a
  distinction a user would otherwise get wrong, and none is the "not X
  but Y" shape. No synonym cycling: `status`, `gate`, `verdict`, `need`,
  `register` and the tool names keep their names throughout. "iff" is used
  as the mathematical convention and stays.
- Style tells: the uniform rhythm of findings 3 and 4 is the one visible
  habit. No dashes (every `--` is a flag). No decorative bold. Tables
  where facts are parallel and prose where they are not; the two tables
  of `zkir_run.py` output keys are the right form for that content.
- Communication tells: none. No recaps, no announcements; the
  cross-references to chapters 02, 03, 06, 12 and 13 are one clause each.
- Filler and hedging: "exactly" at line 234 (finding 6). Nothing else.

## Notes on voice and cadence

The short sentences arrive where a reader needs a rest and a fact at once.
"The CLI calls `run`." (line 130) closes the `Runner` paragraph on the one
fact that matters to someone reading the code. "No tool compares `variant`
with anything." (line 152) is seven words after sixty, and it is the
sentence a reader will quote. "Stdout is the module." (line 308) says in
four words what the shell command underneath it depends on. "`fsqrt 4`
accepts either square root." (line 249) is the exception stated as
briefly as the rule.

The `--gen` walkthrough (lines 49 to 65) is the best passage: it gives the
error a reader will hit first, the two-step remedy in the order to perform
it, the complete preimage, and the exact output, with nothing between. The
exit-code paragraphs (lines 105, 126, 212) say which failure produces which
code and which stream, which is what a caller writing a wrapper needs. The
paragraph at line 343 anticipates the `ENCODED_LEN` lookup a reader would
otherwise trip over. Line 194 explains the one surprising fact in the
example (`ok` with a `violated` gate) immediately under it.

## Verdict

This is a reference chapter that knows every flag, exit code and message
string of eleven tools and puts each where a reader will look for it. Its
only faults are of breath: eight sentences that run past forty-five words
on semicolons and brackets, a semicolon as the join of first resort, and
twenty-odd subjectless fragments at the openings and closings of sections
that make the prose read as a manifest of docstrings. The fix is
rewriting at the joints, not deletion: nothing in the chapter is
redundant, and no fact, number, path, symbol, flag or message string
changes. The tables, the code blocks and the `--gen` walkthrough stay as
they are.
