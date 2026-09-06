# Gottlieb edit: 01-overview.md

Line numbers refer to the original chapter.

## Critical findings
1. **Sentence-length uniformity.** Line 137 is one ninety-word sentence carrying four negated claims on semicolons ("that no other witness ...; that a hash gadget ...; that an assigned JubjubScalar ...; or that keygen ..."), with no short sentence to reset the reader before the `synthErr` point. Fix: break at the natural joints into three sentences, keeping every claim and every "because".
2. **Sentence-length uniformity (paragraph density).** Line 96 is one paragraph doing four jobs: the runner's options, the format-error exit, the output of each of the two commands, and the numbers for `transient_hash.zkir`. Fix: split into two paragraphs at "The first command prints"; no sentence changes.
3. **Machinery in the prose.** Line 139, second sentence: the grammatical subject is a twenty-word "That ..., and that ..." clause and the verb ("is established") arrives at word twenty-two. Fix: put the verb first and the two clauses after a colon; the assertion is unchanged.

## Important findings
4. **The "not X but Y" reflex.** Line 5: "so a program can be run, not only read." The contrast is decoration; the claim is that it runs. Fix: cut "not only read".
5. **Appositive stacking.** Line 5, first sentence: three interruptions ("written in the K Framework", "of ZKIR v3", "the zero-knowledge intermediate representation that ...") before the main verb. Fix: two sentences, one for what ZKIR v3 is and one for where and how the definition is built.
6. **Passive by default.** Lines 7, 20 and 100 ("are covered by", "is measured by", "is checked by") hide an agent that is already in the sentence. Fix: active voice; same facts.
7. **Semicolon as the default joint.** Lines 23, 111, 135 and 137 each join two independent sentences with a semicolon where a period (or "and") serves. Fix: periods where the second clause stands alone; the semicolons in lines 83 and 96 are tight asides and stay.
8. **Docent's elbow (mild).** Line 23: "the last section of this chapter gives the consequences." A real pointer, so it stays, but as a plain "and" clause rather than a second semicolon sentence.
9. **Table density.** Line 46 (`zkir-ops.k`) is an eighty-word cell. Every clause is a fact the reader needs; left as it is.

## Tell audit
- Content: none. No inflated significance, no promotional framing, no vague authority; the numbers are cited to receipts.
- Language: one "not X but Y" (line 5). No stock AI vocabulary, no copula avoidance, no forced triads (the enumerations are real lists), no false ranges.
- Style: no dashes, no decorative bold. Semicolons as the default joint (finding 7). Uniform long-sentence rhythm in lines 96 and 137 (findings 1 and 2).
- Communication: "the last section of this chapter" (line 23) is the only self-pointing; no recaps, no warm-up sentences after headings.
- Filler and hedging: none of note. "at the level of instructions" (line 18) and "concretely" (lines 45, 137) are technical, not adverbial assertion.

## Notes on voice and cadence
The entry-point paragraph (line 77) sets the right pace: "The three entry points share the instruction rules. `job` runs any program the crate's `IrSource::load` accepts." Short, then longer, then the K rule as the landing. Line 57 does the same in two sentences. The opening of the closing section, "establishes one thing:", and "Those relations are not the circuit." (line 137) are the two places a short sentence does the work, and both stay. The closing sentence of the chapter, "`zkir_run.py` never calls the crate", is the kind of concrete fact that certifies a claim without an adverb. Tables carry the parallel facts, so the prose is not doing a list's job anywhere.

## Verdict
A lean reference chapter whose facts are already in the right order. Nothing needs deleting beyond a four-word flourish in the first paragraph. The work is at the joints: one ninety-word sentence, one overloaded paragraph, one front-heavy subject, and a habit of joining sentences with semicolons. The fix is rewriting in four paragraphs and splitting one; everything else stays as written.
