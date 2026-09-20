# Federated DeFi Kernel page: frontend and accessibility proposal

Prepared 2026-09-19. Independent frontend architecture and educational visualization perspective. Planning only; no repository files changed, implementation performed, or product assurance established.

## Recommendation and evidence boundary

Build an additional **`/kernel.html` educational route**, with an illustrated agreement followed by a small, deterministic scenario explorer. Start with the benefit: “Let solvers find a route while your agreement keeps its limits.” Readers should understand that Moriarty defines acceptable behavior and the optional kernel coordinates execution before encountering cryptographic vocabulary.

**Repository observations.** The checkout inspected is `3c93e775a8e432aa995317e2428c169696257430`. The homepage is a React application with anchor navigation and no router. Vite uses relative asset paths. The documentation generator produces two script-free reference pages; importantly, `docs/kernel.html` already redirects to `requirements.html#architecture`. The Pages workflow builds and drives browser tests before publication. Existing documentation tests require exactly two documentation navigation links and no scripts. Preserve these contracts and the README banner. [S1–S4]

**Target design facts.** The frozen brief describes the kernel as optional coordination, not the Moriarty language or its execution target. Moriarty targets Midnight ZKIRv3. Full language-to-ledger correspondence, recursive/private history and federation integration remain unfinished. The March 2027 recursion horizon is a user planning assumption, not a release commitment. DeFiFormal is a semantic reference, and Lean is not a Moriarty dependency. [F1–F3]

**UI recommendations.** Everything below proposes an educational representation of that target, not an operational interface or evidence that the target works. The guarded CLI status was inspected: financial implementation dispatch is blocked by stale inputs and missing operational evidence; no pending transactions were returned. This does not prevent this planning task and must not become a warning about public developers needing maintainer permission.

## Formats considered

1. **Illustrated article with an embedded scenario explorer — recommended.** A complete static narrative establishes the responsibilities; readers then change a solver candidate or external observation and see consequences. It offers a coherent first reading, accessible fallback and a bounded implementation surface.
2. **Branching walkthrough.** Each decision reveals the next stage. This makes timing and recovery concrete, but hides unselected branches and makes comparison, printing and screen-reader orientation harder. Borrow its explicit “Next event” control without making the entire article a wizard.
3. **Free-form architecture map.** Selecting nodes could reveal responsibilities and trust assumptions. It supports specialists, but starts with terminology, risks turning the kernel into the apparent centre of every program and requires substantial responsive interaction work. Use a small static boundary diagram instead.

## Narrative and page structure

The opening gives one owner a comprehensible objective: exchange asset A for asset B within signed limits. A nearby sentence reads: “This page illustrates the proposed design. It does not submit transactions.” Follow it with the distinction between permissionless program use and optional federation services; supported direct Midnight programs do not require federation membership or a privileged solver. [F1]

Next show an **agreement card**: maximum gross spending 11 A, including at most 1 A fees; at least 20 B on successful completion; named asset domains and recipient; permitted partial outcomes; conditions and recovery policy. Display exact-plan authorization alongside outcome authorization in a short disclosure. Solvers can choose only the choices the owner left open. Every hard constraint remains binding through routing, aggregation and settlement.

Then show a compact boundary diagram with labelled arrows:

- Owner and application → Moriarty agreement: consent, constraints and evidence policy.
- Agreement → optional kernel: candidate search, reservations, evidence collection and coordination.
- Native stage → Midnight: verification and local ledger effects.
- Kernel → external adapter/domain: submitted transaction and authenticated observations under that domain’s rules.

A separate direct arrow from the agreement/native stage to Midnight demonstrates the federation-free path. This is a diagram of responsibilities, not a packet trace: caption it accordingly. Under the graphic, an ordinary ordered list states the same relationships. An optional “Who enforces this?” disclosure maps each condition to its actual enforcement boundary. Avoid a single green shield enclosing all chains. [F2]

The centrepiece is the scenario explorer, followed by “What the evidence establishes” and “What exists today.” Finish with links to the preserved requirements and syntax/semantics references, the roadmap and source material. Technical details belong in disclosures and references, not blocking the first encounter with the benefit.

## Explorer: inputs, state and outputs

Use preset choices rather than arbitrary financial inputs. There is no wallet, network call, price feed or backend.

**Inputs:** a solver proposal selector; an external-observation selector available at the relevant step; ordinary Previous, Next event and Reset buttons. Candidate choices are “Route A,” “Route B” and “Candidate with an extra fee.” Treat solver labels as fictional. A candidate change resets the scenario to its initial uncommitted state and announces that reset; it cannot rewrite a committed history.

**State:** selected candidate, stage, observation, amounts already spent, fees already incurred, active reservations, confirmed receipts, residual duty and consumed terminal identifier. Represent event transitions in one typed pure reducer with explicit allowed events. Use scaled integers for A and B; formatting is separate. The numbers are illustrative fixtures, not market estimates or executable financial semantics.

**Outputs:** a one-sentence result, the named condition responsible, a chronological event list and a small accounting table. Show “candidate rejected,” “stage committed,” “external outcome unknown,” “recovery authorized” and “agreement completed” as distinct textual statuses. A short polite live-region announcement updates the result; the complete table remains normal document content. Avoid percentages, throughput counters and implied live balances.

## Worked scenario and failure branches

The signed illustration permits a partial first delivery. Route A proposes principal 10 A plus total fees 1 A for 20 B. Route B proposes principal 9.8 A plus fees 0.8 A for 20 B. Both can satisfy the stated numeric limits, but neither is accepted solely because its totals fit: recipients, domains, evidence, authority and stage/history requirements also apply. Changing solver preserves those checks. The third candidate proposes principal 10 A plus fees 1.2 A; reject it before commitment with both fee-cap and gross-cap explanations. [F3]

For Route A, the first committed stage spends 4 A plus 0.4 A fees and confirms receipt of 8 B. The continuation explicitly retains the agreed duty to deliver the remaining 12 B or take the signed remedy; identify the duty bearer as the illustration’s consenting provider. The remaining plan reserves 6 A principal and 0.6 A fees. Show incurred and reserved amounts separately. These figures are a proposed teaching fixture inspired by U3, not repository execution results.

The next external request becomes **unknown**. Keep its reservation and the continuing obligation. A timeout does not prove nonexecution and does not unlock a refund. A “Try recovery now” event explains why reconciliation is still needed under this example policy. [F2]

Readers then choose an authenticated observation:

- **Confirmed success:** receive the remaining 12 B, record the remaining expenditure and consume the terminal state once. Totals are 11 A gross, 1 A fees and 20 B received.
- **Confirmed nonexecution with authorized cancellation:** assume the example’s named evidence policy establishes nonexecution and release of the reservation. Previously spent 4.4 A and received 8 B remain. The 0.4 A fee remains incurred. The 12 B residual duty remains until the signed remedy actually discharges or amends it. Label this “partial agreement; remedy pending,” never “rolled back.”
- **Still unknown:** preserve the unresolved reservation and duty. Availability limitations remain visible; there is no invented successful recovery.

A final invalid-event preset attempts a late success after a terminal remedy. Explain exclusive terminal consumption and show rejection or reconciliation according to the explicitly illustrated policy, not a second payout. No interface event should suggest global cross-chain rollback. [F2]

## Trust explanation

Place three adjacent textual cards after the scenario. ZK proves the specified relation; MPC/threshold signing distributes control under a stated corruption threshold; TEE evidence concerns an attested environment under hardware, freshness and rollback assumptions. Each must bind the same intention, program, stage, domain, epoch and effects. Shared operators or infrastructure can correlate failures. [F1–F2]

A destination comparison shows the decisive difference: Midnight enforces its actual native relation and local consumption rules; an external destination enforces its own rules. If a foreign account accepts a threshold signature alone, compromising that threshold may bypass policies honest signers checked. The visual must show that bypass reaching the external effect, even if a Moriarty-side proof check would reject the candidate. Do not offer security toggles that silently relax the owner’s signed evidence policy.

## Delivery, accessibility and fallback

Add a second Vite HTML entry at `site/kernel.html`, a small `src/kernel/` entry and scoped styles. Keep the existing homepage and generated references intact; add a homepage link and, if wanted, a reference-footer link without converting the documentation navigation into another application. Preserve the old `docs/kernel.html` alias. Relative links must work under the GitHub project subpath as well as local preview. [S1–S4]

Write the article and the default scenario transcript as semantic HTML in the new entry. Mount React only into an initially hidden explorer container. Reveal controls and hide the duplicate static transcript only after successful initialization. A script failure leaves the article and complete success/unknown/failure transcript readable. Print styles include the static transcript and omit interactive controls.

Use fieldsets, legends, buttons and radio inputs. Avoid clickable SVG regions, drag requirements, hover-only explanations and colour-only statuses. Keep focus on the triggering control, announce concise outcomes and provide a jump link to the updated explanation. Avoid autoplay; reduced-motion removes nonessential transitions. At narrow widths, stack the agreement, diagram lanes and accounting rows into document order. Reserve a comfortable touch target around controls, test at 320 CSS pixels and 200% zoom, and verify focus indicators and text contrast in both themes. A long diagram is secondary to its readable textual equivalent.

## Scoped sequence and acceptance

1. Freeze copy, source mapping, scenario arithmetic and event expectations; distinguish target, observed and illustrative claims.
2. Add the static route, diagram and fallback transcript. Verify direct refresh, fragment links, project-subpath assets and unchanged reference routes.
3. Implement the pure scenario model and its small React island. Keep state local to the page; no storage or share-state parser is necessary initially.
4. Add model tests for cap violations, unknown outcomes, preserved fees/duties, reservation release and duplicate terminal events. Test each solver against identical hard constraints.
5. Extend the existing browser workflow with `kernel.spec.py`: all branches, keyboard-only operation, radio labels, focus visibility, live-region content, mobile overflow, zoom, reduced motion, dark theme, script-disabled fallback and failed-script fallback. Manually read with a screen reader; automated checks cannot establish understandable narration.
6. Run existing type checks, data tests, build and both existing browser suites plus the new suite. Review screenshots at phone and desktop widths and conduct a short comprehension check: can a new reader identify who chooses a route, who enforces each boundary and what unknown means?

All checks above are proposed acceptance work, not executed results. The principal tradeoff is a little duplicated transcript content in exchange for resilient reading; derive scenario summaries from one reviewed data source during a later build step only if drift becomes a real problem.

## Sources

- **F1:** [Frozen brief: README, especially “How the Federated DeFi Kernel works with Moriarty”](/home/charl/research/moriarty-kernel-page-2026-09-19/brief.txt:136).
- **F2:** [Frozen brief: consolidated design, responsibility boundary and recovery](/home/charl/research/moriarty-kernel-page-2026-09-19/brief.txt:224).
- **F3:** [Frozen brief: roadmap status, recursion assumption and U3 discriminator](/home/charl/research/moriarty-kernel-page-2026-09-19/brief.txt:501).
- **S1:** [App.tsx](/home/charl/Moriarty-pages-20260919/site/src/App.tsx), [Vite configuration](/home/charl/Moriarty-pages-20260919/site/vite.config.ts).
- **S2:** [Documentation generator](/home/charl/Moriarty-pages-20260919/site/scripts/build-docs.mjs).
- **S3:** [Existing documentation browser checks](/home/charl/Moriarty-pages-20260919/site/test/docs.spec.py), [homepage browser checks](/home/charl/Moriarty-pages-20260919/site/test/site.spec.py).
- **S4:** [Pages workflow](/home/charl/Moriarty-pages-20260919/.github/workflows/site.yml), [site voice](/home/charl/Moriarty-pages-20260919/site/VOICE.md).
