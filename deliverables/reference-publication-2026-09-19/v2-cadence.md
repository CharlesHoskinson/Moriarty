**Verdict: CHANGES_REQUESTED (reader: CLEAR)**

One required correction, the rest optional. The three documents preserve the protected inventory: banner first, both public references linked, 35 + 16 + 8 paragraphs with IDs and titles intact, ZKIRv4 stated as a proposed workstream, March 2027 stated as a user planning assumption dated September 19, 2026, no Lean dependency, no Mina port, permissionless authoring separated from application consent, local evaluator separated from the federated kernel, formal grammar, `data-tex` strings, code blocks and static judgments untouched. Step counts, the ProRata worked example and the guard tallies in language.html are internally consistent.

**Required**

- **requirements.html, MPLR-024, last sentence.** "A locally atomic transition cannot make a foreign transfer part of the same atomic event without an additional justified protocol." The closing phrase implies an escape that the source clause does not offer; the source unconditionally requires separate pending and final states for relocation and independent-chain effects. Minimal fix: end the sentence at "atomic event" or write "cannot by itself make a foreign transfer part of the same atomic event; that transfer retains its own pending and final states."

**Optional**

- **README, "The problem", second paragraph.** The acceptance-relation sentence lists complete effects, gross spending, fees, net outcomes, disclosures and residual duties but omits the inventory's "separately typed debt." Consider inserting "typed liabilities" before "and residual duties" so the README matches MPLR-017 and the brief.
- **requirements.html, MPLR-003.** "at the policy-defined settlement point" is not in the source clause. Harmless, but "until that combination holds" alone is closer to the clause.
- **language.html, paragraph before the EBNF heading.** "ZKIRv4 requirements is a proposal" reads as an agreement slip because the link text is plural. Suggest "the ZKIRv4 requirements describe a proposal, not a new source-profile implementation."
- **requirements.html, cadence across the MPLR section.** Almost every paragraph follows the same three-beat shape: general observation, one "must" sentence, one negative closer beginning "A ... cannot" or "... alone ...". Read aloud, the closers accumulate into aphorism fatigue by MPLR-020. This is a requirements list, so scan-uniformity has value, and no claim is affected. If a later pass wants relief, vary three or four closers in the middle run (MPLR-011 through MPLR-018) rather than the whole set, and keep the strongest negatives (MPLR-019, MPLR-023, MPLR-029, MPLR-035). Do not touch the "must" sentences, which carry the obligations.

**Kept deliberately**

- The refrain "not a verified release commitment" recurs in README, the requirements status aside and the recursion introduction. The spacing is deliberate and the distinction is protected; leave it.
- Repetition of defined terms (residual duties, authenticated, consumption, signed policy) across paragraphs is exact writing under the active profile, not a repetition defect.
- Language.html's repeated closing disclaimers that finite matches do not establish general correspondence appear once per section and protect claim status; leave them.