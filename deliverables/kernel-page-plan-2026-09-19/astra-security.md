# Federated DeFi Kernel explainer: security architect proposal

Planning only. Persona: distributed-systems and cryptographic security architect. Evidence basis: supplied September 19, 2026 frozen sources. No network execution, implementation, deployment assurance, or examination of other proposals. Required development skill and guarded CLI status were inspected; status reports unresolved implementation admission evidence and no pending transactions. Those internal controls do not constrain permissionless public programs.

## Recommendation and alternatives

Recommend an **interactive agreement journey**: “One agreement. Several systems. Duties that survive interruption.” Begin with a concrete benefit: an owner can let competing solvers choose an acceptable route without granting unrestricted spending, while keeping track of what still needs to happen if execution stops halfway. The page should teach that the kernel coordinates an agreement across independent enforcement boundaries; it does not make those boundaries disappear.

Three viable formats:

1. **Agreement journey, recommended.** A guided scenario reveals the architecture when the reader needs it. A small event selector changes outcomes, accounting, and available next actions together. This balances comprehension and honest treatment of failure.
2. **Trust-boundary atlas.** Clickable system compartments reveal claims, evidence, and assumptions. Strong for architects, but readers may encounter cryptographic vocabulary before understanding the benefit.
3. **Incident reconstruction.** Start with a delayed payment and work backward through causal history. Memorable, but makes the kernel appear primarily a disaster-recovery product.

Use the journey as the main spine, with a compact trust atlas and causal-history inspector embedded later. Avoid an unrestricted network simulator: its combinatorial state space would overwhelm readers and invite unsupported protocol assumptions.

## Evidence and page structure

Label substantive content as **repository observation**, **target design**, or **illustrative interaction**. Attach these labels to individual claims, not merely a footer disclaimer.

Repository observations: supported source programs have local evaluation; selected K comparisons and scoped Preview loan/swap results exist. General rich-source-to-ledger correspondence, complete mandatory native acceptance, recursive/private history and federation integrations remain unfinished. The current compilation target is ZKIRv3. ZKIRv4 names proposed requirements; the approximate March 2027 recursion horizon is the user's September 19 planning assumption, not an upstream release commitment. No Lean dependency is proposed. [1–4]

The page order:

1. **Benefit and agreement:** show an owner's immutable limits beside two permissible candidate routes. Explain solver freedom in ordinary language.
2. **Follow one execution:** advance through reservation, evidence, execution, observation, and continuation. Show partial and unknown outcomes before introducing cryptographic details.
3. **Who enforces what:** display the four boundaries below, grounded in the current event.
4. **Examine the evidence:** compare ZK, threshold signing and TEE claims; change one evidence binding or trust assumption.
5. **What survives:** inspect the causal history, outstanding duties, privacy disclosures and recovery authority.
6. **Today and target:** provide scoped status and links to the two existing references and source documents.

The README banner remains the first image. Add one educational link near the kernel explanation without replacing the requirements or syntax/semantics references.

## Exact responsibility boundaries

| Boundary | Page explanation | Explicit limit |
|---|---|---|
| Moriarty | Defines acceptable financial behavior, signed authority, complete effects, required evidence and persistent duties | Does not discover external truth or provide chain availability |
| Optional kernel | Coordinates solvers, observations, evidence production, configured signing/custody, submission and authorized recovery | Cannot weaken owner constraints or grant itself spending rights |
| Midnight | Checks its native proof relation and enforces its ledger state/consumption rules | Does not impose verification rules on another chain |
| External systems | Execute transactions and provide domain-specific observations, finality, custody and attestations | Inclusion, application success, finality and delivery are different facts |

Direct Midnight use remains possible without federation membership. Optional service eligibility is not language licensing. OWS is a planned wallet interoperability boundary; x402 is a planned payment interface, not evidence that a recipient received a result. Avoid drawing either as an implemented connector. [1–3]

## Worked scenario and state behavior

Use the roadmap's illustrative “at most 11 A, including at most 1 A fees, receive at least 20 B on successful completion” scenario. Qualify both assets by domain and use integer minor units internally. The signed scenario also names recipients, evidence predicates, partial-fill policy, recovery authority and disclosure policy. These are example policy choices, not proposed source keywords. [4]

Two candidate plans satisfy identical hard constraints: two fills costing 5 A plus 0.5 A fees each and delivering 10 B each; or one fill costing 10 A plus 1 A fees and delivering 20 B. Choosing a solver changes the proposal, never the signed constraints. A “cheaper proposal” that substitutes the recipient is rejected before commitment and leaves spending unchanged.

For the two-fill route, the first finalized fill records gross debit 5.5 A, fees 0.5 A and receipt 10 B. Its continuation retains the permitted remaining order and any explicitly consented counterparty duties. Do not invent debt merely because a solver proposed a trade. The second fill reserves 5.5 A. Display spent plus reserved as 11 A, leaving no additional ordinary spending capacity.

The event selector supports:

- **Finalized successful result:** record the second fill, consume its reservation and unique claim, and show 11 A gross debit, 1 A fees, 20 B receipt. Completion still depends on every required predicate.
- **Timeout / result unknown:** retain the first fill, second-leg exposure, reservation and claim identity. Show “Reconcile” as available under its separate authority; “Refund now” and “Charge again” explain why they are unavailable. Time elapsed is not proof of nonexecution.
- **Authenticated failed execution:** apply the selected domain's illustrative failure policy. For example, if a 0.2 A fee was incurred but principal did not move, display gross debit 5.7 A, fees 0.7 A, receipt 10 B and remaining usable budget 5.3 A after confirmed reservation release. The original second fill no longer fits. Retain the remaining order and applicable duties.
- **Late finalized success after timeout:** reconcile the original attempt exactly once. A repeated observation adds no financial effect. A later refund attempt against the same consumed claim is rejected.

If recovery cannot proceed because evidence, liquidity or witnesses are unavailable, show an unresolved continuation rather than a “recovered” badge. Compensation is a newly authorized transition with its own effects and costs. Refunds do not reset cumulative gross spending. A distinct terminal “recovered remainder” outcome must not be labeled successful completion or pretend to erase the finalized first fill. [1–4]

## Meaningful security interactions

**Evidence inspector.** Selecting a stage reveals three separate claim cards. ZK establishes the pinned relation under the cryptographic, setup and verifier assumptions. Threshold signing distributes signing control under the selected corruption model. TEE attestation makes claims about a measured environment under hardware, freshness and rollback assumptions. Shared operators/infrastructure are displayed as shared dependencies, never multiplied into an invented security score.

A “change evidence binding” selector substitutes the domain, stage or federation epoch in one card. Output becomes “Required evidence does not match this stage”; execution remains unavailable. Common fields include intent, program, domain, stage, epoch and effects. Program/source/target identities have correspondence links, not falsely identical hashes. A TEE cannot replace a missing mandatory proof; a signature cannot turn an oracle claim into external truth. [2]

**Destination enforcement comparison.** Two clearly illustrative profiles: “signature-only foreign account” and “destination enforcing the specified policy.” For a toy 3-of-5 threshold, compromising three signers in the first profile highlights an unauthorized external transfer as possible even when Moriarty would reject the corresponding stage. The output says that local rejection does not undo foreign asset loss. The second profile names exactly which destination checks reject that transfer; do not assume all proofs, custody systems or adapters provide this profile. Show signing authority and asset location separately from solver identity. [1–2]

**Causal-history inspector.** A small branching graph shows authorized origin, two reserved branches, authenticated observations and their join. Selecting a node reveals predecessor references, consumed claims, cumulative spending and retained duties. Attempting to join two branches that spend the same claim fails; shared ancestry alone does not. Removing a predecessor obligation prevents a compliant join. Unknown branches remain visible and retain their exposure. “Compress evidence” may collapse the drawing, but cannot delete the history's accounting or imply current native recursion support. [2,5]

**Privacy and availability lens.** Switch observer among public ledger, authorized participant and configured service. A disclosure table shows what this particular illustrative policy reveals: public commitments/metadata, authorized private witnesses, and service-visible fields. No universal anonymity promise. Withhold successor witness access: evidence may remain valid, but the successor cannot produce the next proof. A separate missing-liability example illustrates why proving over selected visible positions is insufficient. These interactions distinguish confidentiality, historical completeness and ability to progress. [1–2]

## Delivery and acceptance

Recommend `docs/federated-kernel.html`, preserving the existing `docs/kernel.html` redirect and both reference URLs. The inspected builder explicitly creates that alias. Extend the existing site build narrowly, using its React/TypeScript/Vite toolchain for enhancement and generated semantic HTML for the full default explanation. No wallet connection, network calls, telemetry, live balances or real cryptographic verification are needed. [6]

Use a deterministic, typed scenario reducer with immutable signed policies, explicit events, and derived accounting. Keep synthetic observations distinct from page claims about repository evidence. Store explanatory copy and source references alongside scenario records. A small fixed event set is easier to audit than free-form amounts and arbitrary chain toggles.

On mobile, render the timeline vertically and the graph as a selectable ordered predecessor list; no hover-only content or mandatory dragging. Use native buttons, radio groups, visible focus, descriptive control labels, text plus shape for state, restrained live announcements and reduced-motion support. Without JavaScript, show the full default story and a comparison table of all four outcomes. Printing should retain assumptions and sources.

Acceptance checks should cover the semantic pitfalls: timeout preserves exposure; late success cannot coexist with refunded terminal consumption; refunds preserve gross counters; two solvers share reservations; altered recipients and evidence bindings reject; failures retain incurred fees; witness loss blocks progress without becoming invalidity. These test the educational model, not Moriarty's implementation. Also run the existing site checks/build, keyboard and narrow-screen checks, no-JavaScript fallback, reference-link/alias regression checks, and verify that no label implies measured deployment. Claim review must confirm the distinction between a local candidate rejection, a retained ledger-phase failure and a committed cross-domain prefix.

## Dissent and source references

I dissent from making ZK/MPC/TEE a triumphant three-shield hero image: it hides the destination's decisive enforcement boundary. I also oppose one global “verified” light, an automatic timeout refund, or recursive proof animation that makes duties vanish. These visual shortcuts teach the wrong security model. Conversely, leading with every caveat would obscure the benefit; reveal each assumption at the step where it matters.

References, all supplied frozen repository sources:

1. `README.md`: security model, kernel boundary, use cases and scoped implementation status.
2. `docs/MORIARTY-CONSOLIDATED-DESIGN.md`: responsibility matrix, stage statement, conditional execution and native history.
3. `docs/MORIARTY-PRODUCT-CONTRACT.md`: permissionlessness, ZKIRv3, mandatory claims and retained phase effects.
4. `ROADMAP.md`: U3 example, U4/U5 acceptance boundaries and user-supplied recursion planning assumption.
5. `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`: persistent continuations, private handoff and external observation contracts.
6. `site/package.json` and `site/scripts/build-docs.mjs`: current site tooling, two references and existing kernel alias.
7. `deliverables/defiformal-study-2026-09-19/RESULT.md`: semantic reference scope; no deployed federation or transferable native proof.
