**Verdict: CHANGES_REQUESTED (reader: CLEAR).** One requirement paragraph narrows a source clause; everything else is a preference. Claim status, ZKIRv4 framing, the March 2027 assumption, the Lean exclusion, the permissionless boundary, the local-evaluator/kernel distinction, and every equation and step count check out.

## Critical findings

None.

## Important findings

1. **Scope narrowing in a requirement clause. requirements.html, MPLR-002.** The paragraph binds the continuation to "awaited evidence." The source clause says "awaited events." Events include deadlines, observation boundaries and recipient actions, which MPLR-010 and MPLR-006 treat as distinct from evidence. Fix: replace "awaited evidence" with "awaited events." One word; no other change.

## Optional (preferences, not defects)

2. **Omitted item in the effects list. README.md, "The problem," second paragraph.** The acceptance relation lists complete effects, gross spending, fees, net outcomes, disclosures and residual duties. The brief also names separately typed debt. Fix, if wanted: insert "typed liabilities" after "fees". MPLR-017 in requirements.html already carries the distinction, so the README omission is not a status error.

3. **Imported clause. requirements.html, MPLR-014, last sentence.** "The argument must also exclude invalid behavior produced by other satisfying witnesses" restates ZR06/ZR11 inside a language requirement whose source clause does not mention witnesses. The obligation exists in the protected backend text, so nothing is invented. If the reviewer prefers a strict one-clause-per-ID mapping, cut the sentence or recast it as "see ZR06 and ZR11."

4. **Cross-imported phrasing. requirements.html, MNR03, last sentence.** "Compatible dimensions alone" is MNR06's language; MNR03's hostile case is the same-shaped trivial relation. The sentence remains true. Optional fix: "A matching proof shape alone establishes neither permission nor semantic compatibility."

5. **Em dash in the README reference list.** Both bullets use a dash before the description. The Grothendieck sidecar records zero em dashes, but the profile is diagnostic only, so this is cosmetic. A colon or period works if the author wants consistency with the two HTML documents, which use none.

## Verified, no action

- **Counts.** 35 MPLR, 16 ZR, 8 MNR sections, one paragraph each, IDs intact.
- **MPLR-019 and MPLR-035.** Both distinguishing clauses survive: obligation formation is not passive receipt; precommit candidates are not accepted stages, no rollback of committed effects, no membership list, optimality and completion require separate claims.
- **language.html arithmetic.** ProRata example: v = ⌊700/110⌋ = 6, leaving 94, 9, 103. Settlement ⌊21/10⌋ = 2. Transfer-only 18 steps (1+1+15+1) and repayment 37 steps (8 contractions + 21 initial + 7 numeric guards + 1) both reconcile with the displayed rules. Bounds prose matches the EBNF header comment.
- **Profile separation.** Source/5 lifecycle, funded-source/0, the repayment projection and the 40-constructor Core each keep their scope; finite K matches are stated as not closing correspondence.

## Notes on voice

The requirement paragraphs consistently open with the obligation's motivating distinction and close with the failure the obligation excludes. That gives 59 short sections a common shape without mechanical repetition. The README's status paragraph carries the planning assumption plainly and without cheerleading.