# Gottlieb edit: 07-instruction-reference.md

Line numbers refer to the original text (5335 words by `wc`, 5711 by the
KPI script which counts table cells; 188 sentences;
sentence_length_variance 4734.3, inflated by the three tables, each of
which the script reads as one sentence).

The chapter is a reference: an introduction, a shared-machinery section,
a summary table, 34 entries of identical shape and a corpus table. The
uniform entry shape is the design and stays. The prose work is in the
machinery section and in the sentences inside entries.

## Critical findings

None. Every sentence asserts a checkable fact and names the file or symbol
where it lives. Nothing undermines the chapter's authority.

## Important findings

1. **Machinery in the prose.** Three sentences carry a parenthetical or an
   appositive that stops the thought mid-sentence and makes the reader
   hold two clauses open at once.
   - Line 179, `impact` off-circuit: a 25-word parenthetical listing
     `#impactPush`, `#impactOne`, the error text, the `<pi>` append and the
     `<pubInIdx>` advance sits between "pushes each input in turn" and
     "records `skipNone()`", and the sentence then continues into
     `#impactCheck`. Fix: three sentences. The guard pushes and records;
     the two helpers do the pushing; `#impactCheck` compares.
   - Line 349, `public_input`: "A short transcript, a slice panic in the
     crate, sets `panic(...)`, or in generation mode stores the default".
     The appositive "a slice panic in the crate" splits subject from verb,
     and the "or in generation mode" branch hangs off a sentence that has
     already spent its structure. Fix: the panic first, its crate origin
     as a trailing clause, the generation-mode branch after a semicolon.
   - Line 74, the definition of "standard" gate: 70 words from "means:"
     to "`<op>_incircuit`", with the consequence ("so the gate is
     `violated(...)`") and the crate correspondence stacked onto the
     mechanism. Fix: break after the mechanism; the consequence becomes its
     own sentence with the crate correspondence after a semicolon.

2. **Sentence-length uniformity at the long end.** Besides finding 1, four
   sentences run past fifty words without a joint.
   - Line 47: one paragraph holds three topics (the `typeName` list, the
     `#put` / `#check` plumbing, the `[owise]` convention). The `#put`
     sentence has four verbs sharing one subject with a relative clause in
     the middle. Fix: three short paragraphs; the `#put` sentence gets a
     semicolon after `vOk` and `#check` gets its own sentence.
   - Line 178, `impact` emission: mechanism, consequence ("so positions
     never depend on how far the witness got") and the empty-impact case
     in one sentence. Fix: break at "so"; the consequence and the empty
     case share the second sentence.
   - Line 333, `less_than` gate: the padding formula, the crate
     counterpart, the 253 assertion, the resulting `synthErr` and the
     three-check happy path in one 75-word sentence. Fix: break after the
     253 assertion; "therefore" carries the consequence.
   - Line 370, the corpus-table legend: six definitions joined by commas.
     Fix: two sentences, the directory aliases in the first, the two
     program groups in the second.

3. **Comma-splice chains.** Line 263, `reconstitute_field`: `#recon`,
   `#recon3` and `#recon4` are three independent clauses joined by commas.
   Fix: semicolons. Line 18: four cross-references chained by commas after
   the first "are defined in". Fix: split into two sentences.

4. **Subjectless fragments.** Line 215: "Off the curve or outside the
   prime-order subgroup (`inSubgroup`) fails with". The sentence has no
   noun; the thing that fails is a point. Fix: "A point off the curve or
   outside ...". Line 170, `copy`: "`#put(O, resolve(A, M))`; any type,
   the only error is ...". Fix: "for any type; the only error is". Line
   162, `constrain_to_boolean`: "only the resolver's errors, as the
   crate's `drop(...)`" is a noun phrase doing a clause's job. Fix: "the
   only errors are the resolver's, as in the crate's ...".

5. **Coupled pair split by a clause.** Line 5: "`reads(Instr)` and
   `writes(Instr)` give the operands an instruction resolves, in the order
   of `IrSource::preprocess`, and the identifiers it defines." The reader
   has to pair the two subjects with the two objects across the order
   clause. Fix: one clause per function, joined by a semicolon.

6. **Small precision fixes.** Line 349: "treats no guard as active and
   resolves one with `resolveBool`" reads as "resolves no guard". Fix:
   "treats an absent guard as active and resolves a present one". Line
   247: "`#lowHighErr`, the order of the crate's arm:" is an appositive
   that does not name what it is in apposition to. Fix: "which follows
   the order of the crate's arm". Line 350: "as `assign_incircuit`". Fix:
   "as `assign_incircuit` does".

## Tell audit

- Content tells: none. No inflated significance, no promotional framing;
  every authority is a K symbol, a file or a Rust identifier.
- Language tells: none. The copula is used freely. There is no "not X
  but Y" contrast anywhere in the chapter; the one negative-then-positive
  pair ("closely, not always to the character", line 47) is a real
  qualification and stays. No synonym cycling: `gate`, `verdict`,
  `outcome`, `witness` each mean one thing throughout.
- Style tells: no dashes (the KPI script's 4.03 per thousand words is
  `--group` in the commands and the `|---|` table rules), no decorative
  bold, tables only where facts are parallel. Rhythm inside entries is
  varied; the uniformity is confined to the sentences named above.
- Communication tells: "This chapter has one entry for each of the 34
  instructions" (line 3) states scope, not process, and stays. No
  recapping, no codas: no entry or section ends by summarising itself.
- Filler and hedging: none. The adverbs present ("only", "still",
  "exact" in "pins the exact `(x, y)`", "closely") each carry a
  distinction; "exact" is the contrast with the parity-only path and is
  the point of that sentence.

## Notes on voice and cadence

The short sentences land where they should. "`isSpecialEmit` is true for
`impact` and `output` only" (line 32) is the one fact the reader needs
before the code block makes sense. "The guard plays no part." (line 350)
closes the `public_input` gate with the sentence a reader who has just
read the off-circuit half is waiting for. "Every type compares by value."
(line 166), "No chip." (several entries) and "Surplus trailing elements
are ignored." (line 281) are the right length for facts that admit no
elaboration.

The entries themselves are terse by design: "Syntax", "Off-circuit",
"Gate", "Checks", each a labelled clause. That shape is the chapter's
contract with the reader and is not touched. The divergence tags in
parentheses (`f02`, `k01b`) are dense but each one answers "where is the
test for this"; they stay. The `into_coordinates` and `persistent_hash`
case lists (lines 209 and 281) are long sentences that are lists, and
read as lists; they are left alone.

## Verdict

This is a working reference that already trusts the reader and never
sells anything. What it needs is a handful of breaks at natural joints in
seven over-long sentences, semicolons in two comma chains, subjects for
three fragments and two small clarity fixes. No sentence is cut; no fact,
number, symbol, message or check moves. The 34 entries keep their shape.
