**CHANGES_REQUESTED (reader: FLAGS)**, narrowly. Two items need a factual check or a one-phrase fix; everything else is optional. The claim inventory survives in all three documents: all 35 MPLR, 16 ZR and 8 MNR paragraphs are present with stable IDs, ZKIRv4 and March 2027 keep their proposed and planning status, Lean and Mina substitution are excluded, permissionless authoring is stated, and the local evaluator is kept distinct from the kernel. The formal grammar, data-tex strings, step counts (18 and 37), the ProRata worked example and the C/B work equations all check against the source clauses.

## Critical findings

1. **Unsupported evidence claim (language.html, final paragraph of "Typed financial reads", and the "Funded composition" paragraph).** "The September 17 K lifecycle results cover the corresponding scoped reads" and "The separate September 17 Core /4 K lifecycle definition adds the scoped financial read constructors" are not supported by the supplied evidence. The k-lifecycle RESULT reports 104 lifecycle cases over Originate, Accrue and Repay with state, effects, work and rejection comparisons; it does not state that the read constructors execute in K. The /3 contract states the opposite for the bounded kernel. Fix: verify against the K definition. If reads are not natively evaluated, replace both sentences with what RESULT supports: "The September 17 K lifecycle run matched 104 finite lifecycle cases including complete financial state; the /3 contract records that the earlier bounded kernel does not implement the read constructors."

2. **Silent scope widening (requirements.html, "Implementation direction").** "general supported programs on Midnight (U2)" drops "single-stage" from the roadmap's U2 outcome. The brief protects the distinction between bounded stages and total history. Fix: "general supported single-stage programs on Midnight (U2)".

## Important findings

3. **Unverified link (requirements.html, "Implementation direction").** The SP01–SP12 link targets `openspec/sprints/README.md`, which no source clause names. Fix: confirm the file exists or link `openspec/changes/consolidated-language-kernel/proposal.md`, which the roadmap cites.

4. **Present-tense narration of future work (requirements.html, "Implementation direction").** "begins by establishing ... then establishes" reads as a report of progress and repeats "establish" twice in one sentence. The status aside mitigates but the paragraph should carry its own tense. Fix: "The consolidated U0–U7 sequence is to begin with ... (U0), then the certified primitive basis ... (U1)."

5. **Overstated foundation (README, "Where the project stands").** "executable K semantics" describes scoped projections. Fix: "scoped executable K definitions".

## Optional findings

6. **Coda by negation (requirements.html, MPLR-001 through MNR08).** Nearly every paragraph closes with "X alone cannot Y" or "must not". Most of these encode claim status from the source clauses and must stay; the pattern is inherited, not invented. Where the negative only restates the second sentence (MPLR-004, MPLR-009 closers), a plain sentence would vary the rhythm. No claim changes.

7. **Cross-requirement leakage (requirements.html, MPLR-002 "and the consumption rule"; MPLR-027 final sentence).** These import ZR10 and MPLR-006 content into paragraphs whose source clauses do not contain it. Accurate, but it blurs which ID owns which obligation. Optional trim.

8. **Em dashes in the README reference bullets.** Two occurrences; the profile records none. Replace with a colon or period if the metrics matter to the reader.

## Notes on voice and cadence

The requirement paragraphs consistently follow the profile's motivating habit: state what must remain true, give the mechanism, then name the distinction the obligation protects (MPLR-010, MPLR-024, ZR04 and MNR04 are the strongest). The language document exposes its reductions and keeps expectation apart from proof at every stage boundary. The README's "It proves the formalized intention; it cannot recover a wish that was never expressed" is the best sentence in the set.

## Verdict

These are three coherent documents that keep obligations, proposals and implemented scope separate, which is the hard part. One evidence claim about K read constructors must be verified or softened, and one roadmap outcome must recover the word "single-stage". After those two edits and the link check, the set is approvable without further prose rewriting.