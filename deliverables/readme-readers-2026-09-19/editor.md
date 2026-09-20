## Critical findings

1. **Broken reference punctuation.** Opening section, both reference bullets: the link is followed by a period and a lowercase fragment ("...recursion](url). the language requirements..."). This sits in the first screen of the README, where authority is set. Fix: replace the period with a colon in both bullets. Proposed: "[Requirements: ...recursion](url): the language requirements, native backend obligations, recursion refinements and consolidated delivery direction." Same treatment for the syntax and semantics bullet.

2. **Protected claim absent from the document.** The protected list includes "historical README retained for coverage, not authoritative where superseded." That statement appears only in the comparison document. The README's final section says historical plans and receipts describe their recorded versions, but never mentions the retained README snapshot. Proposed addition, end of the second paragraph of "Finding the implementation and research": "The earlier README is retained in the research vault for coverage; where the two references above supersede it, they govern." Report this as a claim-inventory gap, not a style point.

3. **Undefined term introduced late.** "How the Federated DeFi Kernel works," final paragraph: "The local protected financial evaluator" appears nowhere else. Earlier sections say "local evaluators" and "supported source programs." If "protected evaluator" is a project name, define it at first use; if not, it reads as an invented label. Proposed: "The local evaluator checks a bounded stage and its financial actions as part of the language implementation."

## Important findings

4. **Assertion by adverb, "actual" family.** "Actual" or "actually" appears six times ("ledger actually accepts," "can actually establish," "actual participating mechanisms," "actual Midnight Preview effects," "actual ledger results," "actual effects"). The protected claims require the observed-versus-claimed distinction, so keep the three in "Where the project stands." Cut the two in the escrow use case and "Bind the proof": "what the ledger accepts" and "what the program can establish." The nouns carry the contrast.

5. **Contrast reflex.** "Rather than" and "not a claim that" constructions recur roughly a dozen times, mostly in protected distinctions that must stay. One is padded: "rather than treated as success or failure by convenience" (kernel section, third paragraph). "By convenience" is an odd tag. Proposed: "rather than treated as either success or failure."

6. **Modal monotone.** The security model runs paragraph after paragraph of "X must Y. Z cannot W." at near-identical length. The content is right and none of it should go. The cadence problem belongs to the Le Guin pass, but flag it here so the structural edit does not add more of the same shape. One relief point, no claim change: "Preserve authority and history," first paragraph, could open with the plain fact "A continuation inherits cumulative spending, outstanding liabilities and the conditions governing future action" before the obligations resume.

7. **Cumulative-authority sentence restated four times.** Bounded computation, financial meaning, preserve authority, and partial fills each repeat the gross-limit-across-continuations point. Keep the financial-meaning version, which is the fullest, and the partial-fills instance, which is an example. Trim the closing clause of the bounded-computation paragraph to: "each stage and each composition step must remain bounded." The authority claim survives in the other two places.

8. **Acronym order.** "The kernel's ZK, multi-party computation and trusted execution environments" expands two of three at first use and never expands ZK. Proposed: "The kernel's zero-knowledge (ZK), multi-party computation (MPC) and trusted execution environment (TEE) mechanisms provide different assurances."

9. **Quickstart pointer.** "The first command after `cd`" is accurate but forces the reader to count. Proposed: "The demo command evaluates the existing atomic-profile loan and swap examples."

10. **Slash in prose.** "finite native K expression/lifecycle comparisons" in "Where the project stands." Proposed: "finite native K expression and lifecycle comparisons."

## Tell audit

Content: no inflated significance; the "For developers... For owners... For solvers" paragraph is a rule-of-three shape but each clause carries a distinct claim, so it stays. Language: no stock vocabulary; copula is used freely; findings 4 and 5 cover the residue. Style: bold is confined to four defined terms; lists appear only where items are parallel; no dashes. Communication: no chatbot residue, no narration of the writing, no recap codas. Filler and hedging: the hedges present are claim status, not scaffolding; none should be cut.

## Notes on voice and cadence

The "From a transaction to an agreement" section is the piece's best writing. The buyer example does the motivating work the Grothendieck guide asks for, and "A transaction records one step in this process. The agreement supplies the meaning of that step" is exactly the reduction-then-generalization movement. The short sentences in "Bounded computation" ("Default does not erase that debt." "Termination of a stage does not establish completion of the agreement.") land because the surrounding sentences are long. The ZKIRv4 and March 2027 paragraph states expectation as expectation without apology.

## Verdict

This is a disciplined overview that already keeps every protected claim except the retained-README statement, and it earns its length. Nothing needs rewriting. The fixes are two punctuation marks, one added sentence, one term corrected, and perhaps thirty words of adverbs and repeated clauses removed. Apply findings 1 through 3 before anything else; the rest are improvements, not conditions.