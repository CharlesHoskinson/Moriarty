**Verdict: CHANGES_REQUESTED** (reader: FLAGS, minor)

The three documents preserve the protected inventory well: banner first, both references linked, exactly one paragraph per MPLR/ZR/MNR, ZKIRv4 and March 2027 status stated correctly in all three places, no Lean, no Mina substitution, local evaluator kept distinct from the federated kernel, grammar and formulas untouched. The step counts, guard counts and the ProRata worked example in language.html check out arithmetically. The required fixes below are small.

## Required findings

- **MPLR-035, requirements.html.** The paragraph drops a protected distinction from the source clause: failure to complete a candidate cannot roll back already committed external effects, and no membership list may replace proof checking. Minimal fix: after "An unfinished candidate remains distinct from an accepted partial stage," add "and abandoning it cannot roll back effects already committed elsewhere; no developer or solver membership list replaces proof checking."

- **MPLR-008, requirements.html.** "including the material terms covered by a counterparty's consent" imports MPLR-019 language into the stage-authority requirement, blurring the distinction MPLR-019 explicitly guards. Minimal fix: replace with "including the scope of any counterparty's consent."

- **language.html, ProRata example paragraph.** The bold "Presentation bound." run begins inside the same `<p>` as the worked example, so the two render as one paragraph. Minimal fix: close the paragraph after "rejects DUST." and open a new one.

- **language.html, EBNF notation table.** The alternatives row reads `<code>\|</code>`; the backslash is a leaked Markdown escape and will display. Minimal fix: `<code>|</code>`.

## Flags I could not verify

- **Implementation direction paragraph, requirements.html.** U3 ("partial and conditional settlement with recovery") and U6 ("financial-library conformance") do not appear in the supplied backend text, and U1 is described there as investigating compatibility and cost rather than establishing the certified primitive basis. These may come from ROADMAP.md, which I was not given. Confirm against that file; do not change on my say-so.

- **README, "scoped Midnight Preview financial results".** language.html says Preview financial settlement remains open. The two can coexist if the README means scoped receipts rather than settlement, but a reader may hear them as contradictory. Optional: "scoped Midnight Preview financial receipts."

## Optional preferences

- **MPLR-014.** The third sentence about honest versus adversarial witnesses is ZR06 material. It is consistent, but the clause itself does not require it. Cut if you want each paragraph to track its own clause strictly.
- **MPLR-020.** "or jet" introduces a term absent from the source. Harmless; cut if terminology must stay closed.
- **MPLR-029.** "bind the relevant state root" narrows the clause's "authenticated state domain" to one realization. Prefer "state domain."
- **Cadence, requirements.html.** Nearly all 59 requirement paragraphs close on a negated aphorism ("cannot", "must not", "establishes neither"). Under the Grothendieck profile, ending on the consequence is a supported habit, so I do not ask for a change. If you want relief, let five or six paragraphs end on the positive obligation instead; do not touch the second sentence, which carries the requirement.
- **language.html structure.** "Current lifecycle boundary" is an `<h2>` between the semantics `<h2>` and the small-step `<h3>`, which demotes the small-step section under the wrong heading. Make it `<h3>`. The two opening status asides also repeat each other; one would do.

## What was kept

The three-sentence requirement shape, the exact quantifier language in ZR03/ZR06/ZR08, the "user-supplied planning assumption" phrasing wherever March 2027 appears, and the language.html qualifications that separate finite K matches from general correspondence. None of these should be smoothed in a later pass.