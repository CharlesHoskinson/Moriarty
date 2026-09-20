**Verdict: CHANGES_REQUESTED (reader: FLAGS)**

The README reads well and its security model is consistent with the source clauses. Two findings bear on publication; the rest are minor fidelity fixes.

## Critical findings

1. **Missing per-requirement paragraphs (brief vs. document).** The brief states each of the 35 MPLRs, 16 ZR requirements and 8 MNR refinements "gets exactly one explanatory prose paragraph" and that "IDs, distinctions and requirement status must survive rewriting." The README as supplied contains no requirement IDs at all. If those paragraphs live on the linked requirements.html page, this README is not the vehicle and the finding is void. If the brief intends them in the README, the document is incomplete and cannot be approved. Fix: confirm which document carries the 59 paragraphs; if the README, add them under "Requirements" with IDs, titles and "research requirements / specified-only" status preserved.

2. **Unsupported technical attribution ("Midnight as the execution target", para 2).** "Midnight's native Halo2-derived PLONK/KZG stack" adds "Halo2-derived," which appears nowhere in the backend or roadmap sources. The sources say only "Midnight's native PLONK/KZG interfaces." Fix: delete "Halo2-derived."

## Important findings

3. **Weakened Lean claim (same paragraph).** "does not introduce a mandatory Lean dependency" implies an optional one may exist. The roadmap says "Lean is not a Moriarty dependency or roadmap workstream." Fix: "does not introduce a Lean dependency."

4. **Approval list incomplete ("How the Federated DeFi Kernel works", final paragraph).** The brief lists project, council, registry and provider approval. The README names "project council, maintainer approval or privileged solver." Fix: "requires no project, council, registry or provider approval."

5. **Evidence status of scoped results ("Where the project stands", para 1).** "scoped Midnight Preview financial results" is accurate but omits that these target the fixed loan/swap fixtures. Optional clarification: "scoped Midnight Preview loan and swap results."

## Fidelity checks that pass

- ZKIRv4 stated as proposed requirements label; March 2027 stated as a planning assumption dated September 19, 2026.
- Bounded stages explicitly do not imply bounded total history or agreement completion.
- Local evaluator distinguished from optional kernel; direct Midnight use needs no federation.
- Proof scope limited to authenticated intention, complete effects, gross/fee/net, debt and residual duties; oracle truth and foreign finality excluded.
- MPLR-019 consent rule and passive-receipt carve-out rendered correctly.
- Four-obligation acceptance relation matches the U2 contract/refinement/transition/history judgments.
- Solver optimality and eventual completion disclaimed; recursion compression not equated with bounded state.
- Simplicity jets and CAKE references are motivational, not claims about Moriarty's sources.

## Notes on voice

The "From a transaction to an agreement" opening motivates the framework through the problem before defining terms, which suits the profile. Sentence rhythm varies; no stock AI vocabulary appears.

Resolve findings 1 and 2, apply 3 and 4, then this is publishable.