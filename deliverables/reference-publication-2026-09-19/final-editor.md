**CHANGES_REQUESTED (reader: CLEAR)**

All three documents keep the protected inventory intact. I checked the banner position, the two links, the one-paragraph-per-ID discipline (35 MPLR, 16 ZR, 8 MNR), the ZKIRv4 and March 2027 status language, the Lean and Mina exclusions, the local-evaluator versus kernel distinction, and the equations. The repayment equations, the ProRata worked example, and the 18 and 37 control-step counts all check out against the displayed rules. The requested changes are three small wording fixes where a claim leans slightly past its evidence or blurs requirement status.

## Critical findings

None. No equation, quantifier, grammar production, or claim status is wrong.

## Important findings

1. **Status blur across contracts.** `requirements.html`, MPLR-014, third sentence: "Under ZR06 and ZR11, correct output from an honest witness generator is only one case". MPLR-014 is a language requirement pinned to the current ZKIRv3 model; ZR06 and ZR11 belong to the proposed ZKIRv4 contract. "Under" makes a current obligation conditional on a proposed one. Fix: "As ZR06 and ZR11 also require, correct output from an honest witness generator is only one case".

2. **Unverified label on evidence.** `language.html`, "Funded composition" paragraph: "The separate September 17 Core `/4` K lifecycle definition implements the six PRE-read constructors." The read-constructor evidence confirms the six `lxApply` rules in `lifecycle-v1.k`; it does not label that definition as Core /4. Fix: drop "Core `/4`" and write "The separate September 17 K lifecycle definition implements the six PRE-read constructors", keeping the line link.

3. **Inference past evidence scope.** `language.html`, "Typed financial reads" final sentence: "The September 17 K lifecycle results cover the corresponding scoped reads, extending the earlier bounded-kernel scope described in the /3 contract". The evidence scope states that source inspection confirms the rules and that the 104-case result is finite historical evidence, not that the fixtures exercise every read. Fix: "The September 17 K lifecycle definition implements these reads, and its 104-case result is finite evidence for that scope; general correspondence remains open."

## Optional preferences

- **Coda repetition.** `requirements.html`, MPLR-024: sentences two and three both state that a foreign transfer retains separate pending and final states. Cutting the clause "that transfer retains its own pending and final states" from sentence three loses nothing.
- **Agent mismatch.** `README.md`, "The problem", second paragraph: "It proves the formalized intention". The acceptance relation does not prove; a proof accepted under it does. "A proof under it establishes the formalized intention" is closer.
- **Em dashes.** `README.md` uses dashes in the two reference bullets. The active profile records zero em dashes and is diagnostic only, so this is a preference: a colon or period serves equally.

## Notes on voice and cadence

The requirement paragraphs consistently open with the obstruction, then state the obligation, then close on the failure it excludes (MPLR-004, MPLR-010, ZR10, MNR02 are clean examples). That is the "motivate the framework through the problem it must express" habit doing real work rather than decoration. The `language.html` semantics section exposes its reductions honestly: the stage-boundary paragraph after the primitive contractions explains why K instructions rather than eager helpers were chosen, and the presentation-bound paragraph says exactly what the step counts are not. Conjectural language stays exact throughout; "planning assumption", "proposed", "finite evidence", and "remains open" are used where the evidence stops. The README earns its permissionless claim in one paragraph and does not repeat it.

## Verdict

This is a disciplined requirements and reference set whose prose respects the claim inventory almost everywhere. The three requested edits are single-clause corrections that keep a proposed backend contract from governing a current language requirement and keep the K lifecycle evidence described at exactly its inspected scope. Nothing needs rewriting, and nothing needs deletion beyond one redundant clause.