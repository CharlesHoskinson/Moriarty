# Gottlieb edit: 05-fields-curves-and-hashes.md

Line numbers refer to the original text (2549 words, 104 sentences,
sentence_length_variance 468.9).

## Critical findings

None. Nothing in the chapter undermines its authority. Every sentence
asserts a checkable fact and names where the fact lives.

## Important findings

1. **Sentence-length uniformity (the long end).** Four clauses run past
   forty words with no joint for the reader's breath: the `#tsLoop`
   description (lines 70 to 74, 54 words between semicolons), the `linear`
   round description (lines 172 to 176, 53 words), the `#svdwPick` /
   `#svdwSign` / `#edFromMont2` tail of the hash-to-curve paragraph (lines
   202 to 205) and the 23-check enumeration of `unit_values.py` (lines 285
   to 292, 66 words). Fix: break each at its natural joint. `#tsLoop`
   splits after the `t = 1` case; `linear` splits before "for the last
   layer"; the hash-to-curve tail becomes two sentences; the check list
   moves its "the other 20 are encodings" clause to the front so the
   enumeration ends on its last item.

2. **Machinery in the prose.** The Poseidon opening (lines 163 to 169) is
   one 39-word sentence carrying a file name, a generator script, a source
   file, a Sage script and a parameter triple. The provenance chain is
   required content, but the sentence stops mid-thought twice. Fix: give the
   generator its own sentence. Same pattern at lines 222 to 226, where the
   SHA-512 constants ride a "with ... in ..." tail behind the padding
   description; give them their own sentence.

3. **Assertion by adverb.** "simply not in `[0, 2^N)`" (line 84): the
   predicate already shows why no guard on `A` is needed; "simply" is doing
   the persuading. Fix: cut. "the exact test of" (line 140): "exact" asserts
   what "the same test as" states. Fix: replace. Kept: "strictly decreases"
   (line 74), "identically `true`" (line 134) and "consumed exactly once"
   (line 176) are mathematical qualifiers, not emphasis.

4. **Awkward guard phrasing.** "recurses on `N /Int 2` while `N > 0` is
   even" (line 63) reads as if `N > 0` were the thing that is even. Fix:
   "while `N` is positive and even".

5. **Same-shape run.** Lines 98 to 105 are eight clauses of the form "X is
   Y" or "X equals Y", each 6 to 17 words. This is the chapter's one
   stretch of telegraph. It is also a list of facts that resists joining
   without inventing connectives. Fix: leave the shapes, but cast the
   `#jubjubD` parenthetical as a clause with a subject ("the standalone
   symbol `#jubjubD` holds the same value") so the parenthetical stops
   reading as a fragment.

6. **Lint exposure.** The plan citation "(plan decision CLM-0709)" (line 4)
   trips the `CLM-` pattern because "plan" precedes the identifier on the
   line. The hit is permitted for this chapter, but the same fact written as
   "(decision CLM-0709 of the plan)" is equally clear and lints clean.

## Tell audit

- Content tells: none. No inflated significance, no promotional framing,
  every authority is a file path or a standard.
- Language tells: none. The copula is used freely ("`sbox(X)` is `X^5`").
  The "not X but Y" contrasts (declaration not proof, line 236; pre-NIST
  padding not the SHA-3 suffix, line 216; 64-byte value rather than
  `Bytes32`, line 228) are each a real distinction the reader needs, and
  they are three in 2500 words.
- Style tells: no dashes, no decorative bold, tables where facts are
  parallel and prose where they are not. Rhythm is varied at the clause
  level; the uniformity is confined to the runs named above.
- Communication tells: none. "The table gives what the rules return
  outside the intended domain" (line 239) is a table caption, not stage
  direction; keep.
- Filler and hedging: none beyond "simply" (finding 3).

## Notes on voice and cadence

The short sentences land where they should. "`sbox(X)` is `X^5`." opens
the permutation paragraph with the one fact that everything after it
elaborates. "Jubjub does not use `fromXY`." (line 144) turns the section.
"No caller passes a modulus other than the eight primes." (line 270)
closes the totality discussion with the sentence that makes the preceding
case analysis safe to read as trivia. "`fsqrt` has no `[owise]` rule."
(line 259) is a five-word paragraph opener that earns the twenty-seven-word
sentence after it.

The parentheticals that cite files and symbols are dense but never
decorative; each one answers "where do I look". The Jubjub decompression
bullet list (lines 150 to 154) is exactly the right form for a four-way
case split. The totality table, and the four-case list that follows it for
`fsqrt`, are the model for how to document an `[owise]`-free function.

## Verdict

This is a working reference chapter that already trusts the reader. What
it needs is deletion of one adverb and a handful of breaks at natural
joints in the four over-long clauses, plus two sentences of provenance
machinery given room to stand on their own. No rewriting of substance; no
sentence is cut. The facts, numbers and symbols are untouched throughout.
