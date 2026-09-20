# Six-agent kernel page planning synthesis

The user requested the same planning pattern: three Claude Fable 5.1 medium agents and three GPT-6 Astra medium agents. All six supplied substantive independent proposals. This is a synthesis of proposals, not six approvals of the final document and not a release audit.

The result is the [interactive kernel page design](../../docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md). The proposed page is an illustrated article with a bounded scenario explorer at `/Moriarty/kernel.html`. The two existing references and the `docs/kernel.html` redirect remain intact. The README's existing banner and two reference bullets stay; add the educational link in its kernel explanation when implementing.

## Contributions and decisions

| Planner | Contribution | Disposition |
|---|---|---|
| [Fable: interaction/storytelling](fable-reader.md) | Layered responsibility explorer, narrative and fixed policy checker | Retain diagram and progressive detail; use the agreement journey as main spine rather than a permanent architecture console |
| [Fable: DeFi developer/solver](fable-defi.md) | Candidate constraints, gross/fee accounting, OWS/x402 request lifecycle | Retain substance; consolidate separate widgets into one coherent scenario; avoid unconstrained numeric editors |
| [Fable: institutional settlement](fable-institutional.md) | Conditional escrow, evidence arrival, accepted duties and treasury account | Retain conditions and duties; correct unsafe suggestion that a recovery grant alone permits refund while external outcome is unknown |
| [Astra: programming languages](astra-pl.md) | Candidate/accepted/observed distinctions, coherent 5.5 A two-fill arithmetic, invariant-preserving controls | Use as primary scenario; explicitly qualify duty formation and account funding/custody rather than assuming them |
| [Astra: security/distributed systems](astra-security.md) | Separate ZK/MPC/TEE assumptions, signature-only bypass, correlated dependencies, privacy and history | Retain trust comparison; stage broad history/privacy simulation as explanatory disclosures to bound first delivery |
| [Astra: frontend/accessibility](astra-frontend.md) | Standalone root kernel.html route, React island, static fallback, preserve script-free references | Adopt route/integration and accessibility approach; use a single reviewed numeric fixture instead of its alternative numbers |

## Corrected or declined recommendations

- **Refund on unknown outcome:** recovery authority is necessary but insufficient. Require the signed policy's evidence, custody/resource conditions and enforceable exclusive consumption. The initial fixture remains unresolved or remedy-pending when those conditions are absent. A local guard does not prevent a compromised foreign signature-only account from transferring funds.
- **Automatic debt from an order:** distinguish unfilled requested quantity from a duty explicitly accepted by a named counterparty. Do not turn a candidate proposal into an obligation.
- **Numerical variants:** planner examples differ (5/5, 4/6 and 6/4 splits, retained fees of 0.2 or 0.5). Adopt one exact 5 A + 0.5 A per-fill fixture. Independent event expectations must precede implementation.
- **Reserved means escrowed:** reject this conflation. Track authority exposure and asset custody/location separately. Explain funding accounting as an explicit precondition or separately modeled event.
- **Interactive signing/policy edits:** use fixed scenarios. A reset creates a new illustration; editing caps cannot alter a committed history. Previous-step navigation is snapshot inspection, not rollback.
- **Green proof judgments:** replace with named illustrative checks and required evidence. The browser produces no native proof or settlement evidence.
- **Global word bans and status-enum tests:** reject grep bans on words such as “verified” or “deployed”; truthful scoped statements need those words. Review meaning and source scope. Educational UI states are not asserted as a newly frozen language enum.
- **Every paragraph tagged:** keep distinctions adjacent to material claims without burdening every sentence with administrative labels.
- **Navigation expansion:** retain the two-item static reference navigation. Link the educational route from the homepage and README kernel section; do not weaken existing static-reference tests to accommodate an unrelated page.
- **Broad simulator:** defer a free-form source editor, arbitrary chains, real wallets, live metrics and unrestricted history graph. These are not needed to explain the design.

## Evidence and limitations

`source-manifest.json` records the inspected base and source digests. `graph-grounding.json` records a bounded traversal of the pinned DeFiFormal textual import graph; it does not establish theorem validity or a federation deployment. `integration-findings.md` records current routing, build and older content-spec conflicts.

Fable canonical model identities and medium effort are recorded in the receipts. The institutional first response was a non-substantive tool-call attempt; one text-only retry supplied its plan. The reader response contained a substantive plan in a JSON content field, which was extracted. The DeFi response contained a substantive final proposal along with generated tool-call/approval text; only the proposal was extracted. No generated tool result or approval was treated as real. Original output is retained locally in `/home/charl/research/moriarty-kernel-page-2026-09-19`. Native Astra agent names and effort are recorded in `REVIEW-RECORD.json`.

The proposal files preserve their authors' recommendations, including errors corrected above. They are not controlling specifications. The consolidated design document controls this page plan. No implementation tests or deployment were performed for a page that has not been built; documentation links and publication integrity were checked.
