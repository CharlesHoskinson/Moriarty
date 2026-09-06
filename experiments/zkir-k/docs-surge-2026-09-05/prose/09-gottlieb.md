# Gottlieb edit: 09-extension-surface.md

Line numbers refer to the original text (2202 words by the KPI count, 109
sentences, sentence_length_variance 448.1; the variance is inflated by the
two-line K block and the table rows, which the sentence splitter joins into
single long "sentences", so it is reported but not chased).

## Critical findings

None. Every paragraph asserts checkable facts and names the file and symbol
where each fact lives. Nothing undermines the chapter's authority.

## Important findings

1. **Sentence-length uniformity (the long end).** Five sentences carry three
   or four clauses on semicolons with no joint for the reader's breath. The
   `#canonical` description (line 41, 63 words, four semicolon-separated
   cases: keeps `decErr`; turns `decPanic` into `decErr` for `bytes32()`;
   keeps `decPanic` otherwise; accepts `decOk(V)` only when canonical). The
   seven-gates sentence (line 49, 58 words: a parenthetical message, a
   cross-reference and a consequence clause after a semicolon). The `sha512`
   alignment sentence (line 122, 62 words). The unequal-length opener of the
   divergence section (line 129, 41 words, with the "adds no `test_eq` rule"
   clause hanging on a semicolon). The `zkir_kast.py` definition sentence
   (line 154, 55 words). Fix: break each at its natural joint. The
   `#canonical` sentence becomes three: the pass-through, the `bytes32()`
   case with its reason and the other-types case, then the canonical-form
   test. The seven-gates sentence drops its "for these seven gates"
   consequence into its own sentence. The others split at the semicolon.

2. **Same-shape run.** Lines 156 is three consecutive sentences of the form
   "X; Y" (`Runner` selects; `--gen` selects. Prints and exits; lists and is
   empty. Honest preimage; the run prints). The shape is the default
   register, not a choice. Fix: keep one semicolon where the second clause
   is the consequence of the first (the honest preimage and what the run
   prints), and make the other two ordinary sentences.

3. **Machinery in the prose.** The seven-gates sentence (line 49) stops
   mid-thought twice: once for the message string, once for the
   cross-reference. Both are required content. Fix: the split in finding 1
   lets the sentence end at the cross-reference and gives the consequence
   its own sentence, so neither interruption is followed by a further clause.

4. **Assertion by adverb.** "exclusively `boolV` inputs" (line 58): the two
   messages that follow already state the requirement; "exclusively" adds
   emphasis, not information. Fix: "at least one input and accepts only
   `boolV` inputs". "compute successfully" (line 114): the fact is that
   off-circuit computation succeeds while the gate reports a missing chip.
   Fix: say that. Kept: "checks reads and writes generically" (line 51),
   which distinguishes the generic `ZKIR-WF` checks from extension-specific
   ones; "thus" (line 41) and "therefore" (lines 122, 129), each of which
   marks a real consequence.

5. **Referent drift.** "The removed instruction" (line 30) refers to
   `reverse_bytes` in the table twelve lines above. "Values retain
   `bytes32(B)`" (line 30) leaves "values" unqualified when the sentence
   means values of length 32. "prints `wfOk` here" (line 154) leaves "here"
   to mean the program in the command block. Fix: name `reverse_bytes`, say
   "values of that length", say "for this program".

6. **Participial tails.** "calls `#encChunks`, packing successive
   little-endian chunks" (line 32) and "adds `negV(...)`. This is logical
   negation" (line 43) are a dangling participle and a one-sentence gloss.
   Fix: "which packs" for the first; fold the gloss into an appositive for
   the second. The other participial tails in the instruction entries
   ("producing", "appending", "selecting", "returning") describe what the
   named helper returns and are kept; the entries share one structure and
   the tails are where the result of each helper is stated.

## Tell audit

- Content tells: none. No inflated significance, no promotional framing.
  Every authority is a file, a symbol, a message string or a receipt.
- Language tells: none. The copula is used freely ("`encodedLen(bytesT(N))`
  is `(N + 30) / 31`"). The one "X, not Y" contrast (line 17, "separate
  selectable surfaces, not a change to the base entry module") is a real
  distinction. No synonym cycling: "gate", "verdict", "outcome" and
  "synthesis error" are used consistently.
- Style tells: no dashes (the `--` count in the metrics is the `--ext` and
  `--offline` flags and table rules), no decorative bold, tables where facts
  are parallel. The instruction entries are lists by design and stay lists.
- Communication tells: none. "Source facts below come from" (line 5) is a
  provenance statement, not stage direction; keep.
- Filler and hedging: none beyond the two adverbs in finding 4.

## Notes on voice and cadence

The short sentences arrive where they are needed. "All extension modules
live in `semantics/zkir-ext.k`." (line 21) opens its section with the one
fact the table elaborates. "Only `sha512`, through `#needAll`, keeps the
base convention." (line 49) closes the gate-evaluation paragraph on the one
exception. "Successful witness computation therefore does not establish
circuit constructibility." (line 129) is the sentence the divergence
section exists to reach, and it stands last.

The nine instruction entries are uniform on purpose and the uniformity
works: a reader who has learned the shape from `and` reads `sha512` without
re-orienting. The "same nonempty Boolean-input checks and messages as
`and`" line for `or` and `xor` is the right amount of cross-reference.

The two version-boundary tables and the corpus table are the correct form
for their content. The "Version boundary" opening, with its locator
columns, tells the reader where to look before telling them anything else.

## Verdict

This is a reference chapter that already trusts its reader and cites its
sources by file and symbol. What it needs is a handful of breaks at natural
joints in five over-long sentences, two adverbs removed, three pronouns and
demonstratives given their referents, and one same-shape run varied. No
sentence is cut; no fact, number, path, symbol, message string or command
changes. The instruction entries keep their structure and every message
string.
