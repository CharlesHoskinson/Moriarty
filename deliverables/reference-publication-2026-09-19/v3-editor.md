**CHANGES_REQUESTED** (reader: **CLEAR**)

The three documents preserve claim status throughout. No obligation is promoted to a theorem, no bounded stage is read as bounded history, ZKIRv4 and March 2027 keep their proposed and planning statuses, Lean and Mina substitution are excluded, and the local Source/5 evaluator stays distinct from the federated kernel. Counts are correct: 35 MPLR, 16 ZR, 8 MNR sections, one paragraph each. The requested changes below are small fidelity corrections to three requirement paragraphs. Everything else is optional.

## Critical findings

None.

## Important findings (correct before publishing)

1. **Closed list where the source is open.** `requirements.html`, MPLR-003. The paragraph lists "authenticated signatures, document predicates, proofs and recipient actions" and stops. The source clause ends "and other supported predicates." Dropping that phrase turns an extensible requirement into a fixed enumeration. Fix: append "and other supported predicates" to the list.

2. **Scope imported from another requirement.** `requirements.html`, MPLR-014, third sentence. The adversarial-witness clause ("the argument must also exclude invalid behavior produced by other satisfying witnesses") is a ZR06/ZR11 obligation, not part of MPLR-014's source text, which concerns preserving "permitted traces, conditions, rejection behavior and costs." The second sentence also replaces "permitted traces" with "every retained effect," which is narrower (a trace includes order). Fix: restore "permitted traces" in sentence two. Either delete sentence three or prefix it with "Under ZR06 and ZR11," so the reader knows the quantifier over witnesses is discharged in the backend contract.

3. **Vaguer than the clause it explains.** `requirements.html`, MPLR-035, second sentence. "Preserve every applicable choice boundary" replaces the source's "refines every applicable signed constraint and permitted choice." The signed constraint is the object the whole requirement turns on. Fix: "must refine every applicable signed constraint and permitted choice."

## Important findings (verify; no rewrite needed if confirmed)

4. **Dated statement that a supplied source contradicts.** `language.html`, "Funded composition" and "Typed financial reads" paragraphs, state that the September 17 K lifecycle definition executes the scoped read constructors. The supplied `/3` contract says "The bounded K kernel does not implement the new read constructors." The two are reconcilable if the `/3` text predates the September 17 run, but the page should say so. Fix: add "as of the September 17 run; the `/3` contract text predates it" once, in the Typed financial reads paragraph.

5. **Two AST-node bounds.** `language.html` gives 8,192 AST nodes for the expression-source parser (matching the protected grammar header). The supplied `semantic-contract.md` gives 4,096 AST nodes for the Core contract. These may be different layers (source AST vs. Core node count). Fix: one clause in the bounds paragraph stating that the Core contract's own 4,096-node limit applies separately, or a confirmation that the numbers describe different objects.

6. **Unverifiable against supplied sources.** The `financial-expression-source/1` row ("eight additional pure constructors, including shares, variants, UInt256"), the "125 expression cases, 104 lifecycle cases and six Unicode probes" figure, the "42 prior … 22 new" case counts, and the `openspec/sprints/README.md` link have no backing in the provided clauses. I did not find contradictions. Note that `LitShares` already exists in the 40-constructor Core, so "including shares" should be checked against what `/1` actually adds.

## Optional findings

7. **Negative-parallelism reflex.** `requirements.html`. Roughly two paragraphs in three close on the same figure: "X alone cannot Y." Examples: MPLR-003, 015, 019, 020, 021, 024, 028, 029, 032, ZR05, ZR14, MNR03, MNR06. The sources use this shape because each requirement carries a hostile control, so it is partly inherent. Where it is decorative (MPLR-020 "a familiar primitive name," MPLR-026 "or method name"), the sentence would land harder without the redundant instance.

8. **Recap of protected text.** `language.html`, the paragraph beginning "The grammar below is the complete /5 syntax." It restates the EBNF header comment clause by clause, then the code block displays the same comment. Keep only the expansion of the compact bound spellings (which is genuinely useful) and drop the rest of the restatement.

9. **Additions with no source clause.** MPLR-030 "including correlated failures"; MPLR-005 "reconcile late delivery" (a MPLR-010 concern). Neither invents a theorem. Keep or cut at the author's discretion.

10. **Naming.** `language.html`, repayment section: "emits Transfer then Repayment" while the operation is called Repay elsewhere. Use one name.

11. **Em-dashes.** `README.md` uses two, as separators in the reference list. The active profile records none; the metric is diagnostic. Colons would serve.

## Notes on voice and cadence

The README's first paragraph does the Grothendieck move the profile actually documents: it names the object (the owner's intention), the space of permitted executions, and what acceptance must depend on, before any mechanism appears. "It proves the formalized intention; it cannot recover a wish that was never expressed" is the best sentence in the set because the qualification belongs to the claim. In `requirements.html`, MPLR-019 and MPLR-024 handle a distinction that is easy to blur (formation of duties vs. passive receipt; local atomicity vs. foreign settlement) in three sentences without hedging. In `language.html`, the control-step arithmetic (18 and 37) and the ProRata worked example (v = 6, settlement 2) check out against the protected rules, and the prose consistently says which counts are presentation counts rather than fees or termination proofs. The "Current lifecycle boundary" paragraph is a model of how to state a scope: what runs once, what stays stable, what is not published.

## Verdict

This is a requirements agenda and a formal-language reference that keep their promises about status: the reader cannot mistake a specified obligation for a delivered feature anywhere in the three files. What has to change is confined to three MPLR paragraphs where a paraphrase narrowed a list, borrowed a quantifier from the backend contract, or blurred the signed constraint that the requirement protects. Those are one-clause fixes, not rewrites. The remaining items are checks against sources not supplied to me and stylistic preferences the author may decline. After findings 1 through 3 are applied and 4 and 5 are confirmed, the set is publishable.