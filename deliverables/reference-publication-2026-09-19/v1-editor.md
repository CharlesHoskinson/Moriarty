**CHANGES_REQUESTED (reader: FLAGS)**

Verdict first, findings after. The three documents preserve the protected inventory almost everywhere: IDs and counts are complete (35, 16, 8), ZKIRv3 is the target, ZKIRv4 is a proposal, March 2027 is a planning assumption, no Lean, permissionless deployment, local evaluator distinct from the kernel, finite matches not correspondence. The arithmetic in language.html checks out (worked ProRata example, 18 and 37 step counts, 28 = 21 + 7 guards, obligation update equations). What has to change is small: one requirement is weakened by a conditional, one requirement drops two source clauses, one narrows technique against the section's own stated policy, and two rendering defects in language.html will confuse a reader.

## Critical findings

1. **Status weakened by a conditional (MNR05).** Third sentence: "A successful test-mode result must fail to qualify as final-valid evidence *when its claimed proof cannot pass an independent real verifier*." The source is unconditional: dummy/disabled/empty-key modes SHALL NOT qualify production acceptance, and the independent-verifier failure is the hostile test, not the trigger. As written, a test-mode result whose bytes happened to verify could qualify. Fix: "A test-mode success never qualifies as final-valid evidence; an independent real verifier must reject its dummy proof."

2. **Dropped distinctions (MPLR-035).** The source clause carries three points the paragraph omits: no developer or solver membership list may replace proof checking; failure to complete a candidate cannot roll back already committed external effects; candidates are "precommit". The paragraph keeps candidate-versus-stage and optimality/eventual completion. Fix, minimal: append "Failure of a candidate cannot roll back effects already committed, and no solver or developer membership list substitutes for proof checking."

## Important findings

3. **Technique narrowed against the section's own rule (MPLR-029).** The MPLR intro says the requirements "leave particular type systems, calculi and proof representations open." MPLR-029 then says "bind the relevant state root." The source says "bind an authenticated state domain." Fix: replace "state root" with "authenticated state domain".

4. **Term introduced without source (MPLR-020).** "A certified primitive, or jet, replaces…" "Jet" appears nowhere in the source clauses or the other two documents. Unless the project uses the term, cut ", or jet".

5. **Markdown escape leaked into HTML (language.html, EBNF notation table).** The alternatives row reads `<code>\|</code>`; the reader sees a backslash-pipe as the EBNF alternative symbol. Fix: `<code>|</code>`.

6. **Merged paragraphs (language.html, repayment example).** "…floor producing zero cash rejects DUST." runs straight into "**Presentation bound.**" inside one `<p>`. The example and the step-count claim are different objects. Fix: close the paragraph after "DUST." and open a new one.

7. **Heading and TOC drift (language.html).** The TOC entry is "Formal semantics", the heading is "Operational semantics", and "Current lifecycle boundary" is an `<h2>` at the same level as the section it belongs to, before the `<h3>` subsections. Fix: match the TOC text to the heading and demote "Current lifecycle boundary" to `<h3>`.

8. **Cross-requirement content placed inside a paragraph (MPLR-014, MPLR-027).** MPLR-014's last sentence states the adversarial-witness obligation that belongs to ZR06/ZR11; MPLR-027's last sentence states the current-state consumption obligation of ZR10. Neither is wrong under the inventory, and the "one paragraph per ID" rule tolerates motivation. But a reader judging MPLR-014 in isolation may take adversarial-witness soundness as part of that clause's acceptance. Optional: keep the sentences, or prefix "Under ZR06," / "Under ZR10," so the dependency is inspectable.

9. **Roadmap descriptions not checkable from supplied sources (requirements.html, "Implementation direction").** U3 and U6 descriptions ("partial and conditional settlement with recovery", "financial-library conformance") do not appear in the backend text I was given; only U0, U1, U2, U4, U5, U7 do. Not a defect if ROADMAP.md says so. Flagging so someone with the roadmap confirms.

## Optional preferences

- README reference list uses two em dashes; profile em-dash rate is zero (diagnostic only). Commas or a colon would do.
- README "Design choices": "make the meaning and cost of a supported program objects of analysis" reads awkwardly. "make the meaning and cost of a supported program analyzable" is plainer.
- README "deliberately bounds" — the adverb does no work; "bounds" suffices.
- README lists complete effects, disclosures and residual duties but not gross debit, fees-at-debit, net outcomes or separately typed debt as proof objects, though MPLR-017/025 cover them. Acceptable for a README; a half-sentence in "The problem" would close the gap.

## Notes on voice and cadence

The Grothendieck structural habits are present and earned rather than decorated. Each MPLR paragraph runs requirement, consequence, then the boundary the reader might wrongly cross ("A document hash identifies evidence; its presence alone does not establish the required property"). The recursion intro states the target, then the assumption, then the obligations in dependency order. Expectation stays separate from proof throughout: every "remains open" lands where the source says open. language.html's "The stage boundaries matter" paragraph is the best explanatory move in the set, showing why the K instruction staging exists rather than asserting it. No tell families of note; the negative-parallel "cannot X or Y" closer recurs across MPLRs but is doing real scoping work, not rhetoric.