# Gottlieb edit: 06-configuration-and-run-lifecycle.md

Line numbers refer to the original text (2124 words outside code, 115
sentences, sentence_length_variance 302.8).

## Critical findings

None. The chapter asserts checkable facts, names the file and symbol for
each, and never inflates. Nothing undermines its authority.

## Important findings

1. **Sentence-length uniformity (the long end).** Four sentences run past
   forty-five words with no rest: the `checkedJob` description (line 46,
   58 words from the colon to the period), the missing-commitment branch of
   `#seedPi` (line 60, 47 words), the `#commCheck` sentence (line 89, 62
   words, three "where" clauses and then a semicolon) and the
   `build_preimage` procedure (line 97, 45 words). Fix: break each at its
   natural joint. `checkedJob` splits after "discarded"; the `#seedPi`
   sentence splits at its semicolon so the gate's later outcome stands on
   its own; `#commCheck` splits before `transientCommit`; `build_preimage`
   splits before "repeats". The `--gen` warning (line 97, second half) also
   carries its instruction after a semicolon; the instruction is a
   sentence.

2. **Machinery in the prose.** The citation form "`file`, `symbol`,
   verb" appears as the grammatical subject nine times (lines 50, 89, 91,
   93, 97 twice, 101, 143, 169). The file and symbol are required content,
   but a subject made of two appositives and two commas stops the sentence
   before its verb. Fix: recast as "`symbol` in `file`" wherever the pair is
   the subject (lines 50, 89, 91, 97, 101) and as "the ... check of `wf` in
   `zkir-syntax.k`" at line 93. Lines 143 and 169 are inside the traced
   example and stay as they are.

3. **Assertion by adverb.** "simply disappears" (line 93): the sentence
   already says what does not happen instead. "altogether" (line 73):
   "prevents job initialization" is complete without it. "genuine
   stuckness" (line 111): the contrast with a merely exhausted bound is
   already stated in the same sentence. Fix: cut all three. Kept:
   "exactly `encodedLen(T)`" (line 58) is a quantity, "eventually
   contains" (line 32) is temporal, "still" at lines 60, 73 and 95 carries
   the "despite the failure" meaning each time, and "independently of
   witness progress" (line 71) is the point of its sentence.

4. **The "not X but Y" reflex.** Ten contrasts of the form "X, not Y" or
   "X rather than Y" in 2100 words (lines 7, 32, 73, 75, 81, 93 twice, 95,
   111, 155). Each names a real distinction a K reader would otherwise get
   wrong (textual rule order, a `strict` attribute, snapshots of memory), so
   none is cut. One is a duplicate: line 111 states "not that it was
   reached" and then restates it as "does not distinguish bound exhaustion
   from genuine stuckness". Fix: join the two with a colon so the second
   reads as the consequence of the first, not a repetition.

5. **Awkward subject.** "The later `#verdicts`" (line 91): "later" is
   doing the work of a clause. Fix: "`#verdicts` runs after `#finish` and
   invokes ...", which states the ordering the chapter has already given
   at line 32. "Initial values precede entry-point execution" (line 7)
   reads as if the values were events. Fix: "The initial content is what
   each cell holds before the entry point runs."

6. **Same-shape run.** Line 95 is five sentences of 8, 15, 8, 14 and 14
   words, each "X does Y". The second and third describe the two
   generation-mode reads and join naturally. Line 85 says "uses ... uses"
   in one sentence; the second "uses" can become "compares ... in
   `#impactCheck`".

## Tell audit

- Content tells: none. No significance, no promotion, no vague authority;
  every source is a file, a symbol or a message string.
- Language tells: the "not X but Y" family (finding 4) is the one visible
  habit, and each instance is load-bearing. Copulas are used plainly. No
  synonym cycling: `gate`, `verdict`, `cursor` and `status` keep their names
  throughout.
- Style tells: no dashes (the `--checked`, `--gen` and `--depth` flags are
  the only double hyphens). One bold phrase, "**public transcript
  outputs**" at line 83, marks the one fact a reader will assume is a typo
  (`publicInput` reads the outputs); it is a warning, not decoration, and
  stays. Tables where facts are parallel, prose where they are not.
- Communication tells: "The descriptions below follow" (line 5), "The
  table gives" (line 7) and "The milestones below group" (line 155) are
  captions that set a convention; keep. No recapping, no cheerleading.
- Filler and hedging: the three adverbs of finding 3. "possibly alongside
  `<status> ok()`" (line 111) is a real possibility, not a hedge.

## Notes on voice and cadence

The short sentences land where the chapter turns. "`#fail` sets
`error(S)`; `#panicNow` sets `panic(S)`." (line 73) is a deliberate
parallel that the surrounding long sentences need. "It does not change
`<status>`." (line 91) is five words that stop a reader from assuming the
opposite. "`impact` uses public transcript inputs." (line 85) opens its
paragraph on the one fact that separates it from the paragraph before.
"emptiness says that processing ends, not that the witness succeeds" (line
32) is the sentence the whole status table depends on.

The failure paragraphs (lines 73 and 93) are the best writing in the
chapter: they say what survives a failure and what does not, in the order
a reader would ask. The traced example, its table and the three paragraphs
that read the table are exact and untouched.

## Verdict

This is a reference chapter that trusts its reader and cites its sources.
It needs breaks at the natural joints of four over-long sentences, nine
subjects unknotted from their citation commas, three adverbs cut, and two
awkward subjects rewritten. Nothing is deleted; no fact, symbol, number,
path or message changes; the traced example stays as it is.
