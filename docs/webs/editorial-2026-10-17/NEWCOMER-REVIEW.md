# Fresh newcomer review of the revised page

Reviewed 2026-10-17 UTC. Nonauthor review of root's candidate. Scope: newcomer comprehension, information architecture, wording and ordinary page controls. No application/Git edits and no native, cryptographic or security acceptance judgment.

## Result

The revision addresses the original newcomer failures. The page introduces Moriarty as a financial language, connects it to an invoice payment, shows the calculator, then names Webs and Strands. The main headings now form a coherent explanatory outline. The proof-to-transaction explanation is visible. The actual Moriarty illustration and restrained palette give the page a consistent identity.

The architecture section no longer makes *Proving Nothing* a prerequisite. Its opening explains why the seven concerns matter and says that the numbers are not execution steps. Plain subtitles make the technical names navigable, while the book is credited as a source rather than required reading. Groth16 appears before that section, so the main proof concepts are already introduced.

The controls make sense: the invoice slider is introduced by a concrete action; the funding buttons name the fee allowance; the explorer identifies contract checks; the coverage checkbox explicitly says the contract still requires its check; and the setup counter states its compatibility assumption and separates stages from ceremonies. I found no need for another structural rewrite.

## Remaining improvements

These are four bounded editorial refinements, in priority order. The first corrects a small inconsistency; the rest improve the advanced reading path. They do not negate the improvement above.

1. **Use “proof coverage” consistently in the explorer.** `COPY.json`'s explorer introduction still says “remove it,” and its incomplete status says “A required condition is missing.” The checkbox correctly removes coverage while leaving the condition required. Suggested introduction ending: “Select a condition to read its claim and where its inputs come from; remove its proof coverage to see why an incomplete Web cannot settle.” Suggested status opening: “A required condition has no proof coverage. This Web cannot authorize settlement.” Retain the claim-output handling sentence.
2. **Gloss “catalogue” at its first visible use.** Lifecycle Step 2 lists “the catalogue” among familiar transaction fields, before explaining it. Use “the catalogue of permitted claims and verification keys” there, or add a short adjacent definition. The frozen technical notes need no change for this newcomer fix.
3. **Remove internal review counts from the general setup explanation.** The Cardano-wide paragraph introduces “235 review identifiers” and “20 pattern families,” which the reader has never needed to understand. Replace that sentence with: “Claim types and architectural layers do not determine the circuit count.” Keep the phase-1/phase-2 rules, wrappers and frozen-inventory requirement. The exact historical counts can remain in technical source records.
4. **Give architecture Layer 4 a plain-language entry sentence and result gloss.** Its subtitle says “Equations the proof can check,” but the selected panel jumps directly to rank-1 constraints, prime fields and quadratic arithmetic programs, ending with the unexplained result “R1CS matrices.” A brief opening such as “The compiler expresses each financial rule as equations that a valid execution must satisfy” would connect that panel to the previous explanation. Its result can read “R1CS matrices: the circuit's compiled equations.” Preserve the technical body after that opening and leave cryptographic judgment to the assigned technical reviewer.

## Evidence inspected

Read the current `docs/webs/app/src/App.jsx`, `docs/webs/editorial-2026-10-17/COPY.json` and generated content. Refreshed guarded status. Inspected supplied `desktop-top.png`, `mobile-top.png` and `mobile-layer4.png` in `docs/webs/verification/editorial-v3/`.

Independently opened the self-contained `docs/webs/index.html` with Python Playwright and the specified local Chromium binary. All non-file/data requests were blocked; none were attempted. Inspected rendered h2 headings, removed and restored invoice proof coverage, selected architecture Layer 4, and changed the circuit counter to 8. Both coverage states and the circuit count updated as described above.

Exact reviewed HTML SHA-256: `28d9ab01c66f605987c992cab75341bf358ca1bd2d33edeb735128dea0c854a0`. It matches the existing verification receipt at review time. Subsequent changed bytes require their own review. This source/interaction review is not a participant usability study.
