**Plan: interactive GitHub Pages explainer for the Federated DeFi Kernel**

This plan proposes one additional educational page in the existing `site/` Vite build. It does not replace the requirements page, the syntax and semantics page, or the README banner. Throughout, statements are tagged as **repository fact**, **target design**, or **UI recommendation**, so the page never presents planned capability as shipped capability.

## Format options and recommendation

**Option A: scrollytelling narrative.** A long single page where each section reveals a small state diagram as the reader scrolls. Strong for the "benefit before formal detail" goal, weak for a practitioner who wants to poke at the escrow state and see what changes.

**Option B: stage-walk simulator.** One conditional-settlement workflow presented as a sequence of stages. The reader sets the owner's constraints, chooses which evidence arrives and in what order, and watches the escrow, remainder, fees, budgets and duties update. The state machine runs entirely in the browser from a hand-written table of allowed transitions.

**Option C: responsibility matrix with drill-downs.** A four-column boundary table, expanded from the consolidated design, where each cell opens an explanation. Precise but static, and it does not teach partial fulfillment well.

**Recommendation: Option B as the spine, with Option C's boundary table embedded as one section.** The audience cares about escrow mechanics, consent and residual duties, and those are best learned by driving a state machine. The boundary table answers the practitioner's second question, "who is actually responsible for what," without a separate page. Option A is rejected because scroll-driven animation is fragile on mobile and adds no information the simulator does not already convey.

The simulator is a **teaching model of the target design**, not a Moriarty evaluator. The page must say this on its first screen. Repository fact: the local demos evaluate loan and swap programs and do not sign, prove or settle. The page will not call the evaluator, will not fetch network data and will show no live metrics.

## Narrative and page structure

Seven sections in reading order. Sections one through three carry the benefit; four through seven carry the detail.

1. **The agreement behind the transaction.** Reuses the README's buyer, seller, issuer and delivery framing. Plain claim: a payment step only means something if the agreement says what was established, how much may be released, who still owes what and what remedies remain. No jargon yet.

2. **Who does what.** A short version of the responsibility boundary, in prose, with one sentence per role. Moriarty defines acceptable behavior and required evidence. The kernel coordinates solvers, evidence, signing and recovery, and is optional. Midnight checks the native proof relation and its own consumption rules. Each external domain keeps its own finality meaning. Repository fact: kernel integration and solver connections are design work, not shipped code.

3. **The simulator.** Described below.

4. **Evidence and what it proves.** Signatures, document predicates, proofs, recipient acceptance and external observations, each with issuer, domain, freshness and finality fields. A document hash establishes only the predicate checked, never legal truth or physical delivery. This section also states the ZK, MPC and TEE distinction: ZK proves the named relation, threshold signing distributes control under a corruption threshold, a TEE adds an attested-environment claim under hardware and rollback assumptions. Shared operators do not make these independent.

5. **The destination boundary.** The explicit threshold-compromise case. If a foreign account accepts only a threshold signature, a compromised threshold bypasses every policy honest signers would have checked, and a Moriarty proof cannot force that account to verify anything. Membership changes, thresholds, ordering, equivocation and epoch rules are named as integration-specific.

6. **Full boundary table.** Expanded from the consolidated design's table, filtered to the rows this audience needs: intent, financial meaning, permission, conditions, atomicity, recovery, privacy, release process. Each row is tagged repository fact or target design.

7. **Where the project stands and where to go next.** Links to the two existing reference pages, the README quickstart and the roadmap's U3 milestone. States that ZKIRv3 is the current target, that the March 2027 recursion horizon is a user planning assumption recorded on 2026-09-19, that Lean is not a dependency, and that no maintainer, council or registry approval is needed to deploy a supported program.

## Simulator interactions

**Inputs, set once before the walk.** Gross spending cap, fee cap, minimum net receipt, allowed recipients, partial fulfillment allowed or not, recipient acceptance required or not, document predicate required or not, and a separately signed recovery grant with its own expiry. These map to the signed-intent fields in the product contract. Changing an input after the walk starts resets the walk and shows a notice that intent is committed before execution and cannot be weakened mid-flight.

**Solver choice control.** A toggle between "exact plan" and "outcome intent." In outcome mode the reader picks one of two pre-authored routes with different fee profiles. The point taught: route choice is the solver's freedom, and every hard constraint survives that choice. Choosing a route can never raise the caps.

**Evidence arrival buttons.** Seller signature, issuer document attestation, recipient acceptance, external leg result. Each button offers arrived, not yet, and for the external leg, unknown. The state machine advances only when the policy's conjunction is met.

**State panel.** Six quantities, always visible: escrow balance, released so far, remainder, cumulative gross spent, fees incurred, outstanding duties as a list. This is the practitioner's ledger of record. It updates only on accepted transitions.

**Outcome panel.** Names the current typed status from the product contract's list: submitted unfunded, funded, partially fulfilled, waiting for evidence, eligible for release, in flight, delivered, unresolved, recovering. Rejected candidates are shown as "uncommitted, no state change." A separate line names which of the four judgments the transition passed: contract properties, intent refinement, transition validity, history compliance. UI recommendation: never render a green "verified" badge, since the source explicitly forbids treating a label as enforcement.

**Adversarial attempts.** A "try to cheat" menu with fixed cases: add an unlisted fee, redirect to an unlisted recipient, replay the release authorization, reset the spending counter, refund after a late success. Each is rejected with a one-sentence reason tied to a source obligation. Positive controls are shown first so rejection is not the only behavior visible.

## Worked scenario

Buyer funds escrow with a gross cap of 11 A, at most 1 A in fees, and expects at least 20 B on completion, mirroring the roadmap's U3 discriminator. Partial fulfillment is allowed; recipient acceptance and an issuer document predicate guard each release.

- **Stage 1.** Seller signs, issuer attests the first shipping document, recipient accepts. Half the order releases. The state panel shows remainder retained, fees debited against the fee cap, and a duty "deliver second half" outstanding. Repository fact tag: local evaluators check this kind of financial action today; native proof of it is roadmap work.
- **Stage 2.** The second leg settles on another chain. The reader picks "unknown." Status becomes unresolved. The remainder stays escrowed. The reader tries a refund; the machine allows it only if the recovery grant is still valid and marks the outcome exclusive. The reader then injects a late success. The machine rejects a second discharge of the same claim and shows why: refund and late payment cannot both discharge one escrow claim.
- **Failure branch.** The reader chooses "timeout" for the external leg. The page says the timeout changes which authorized transitions may be attempted; it proves nothing about execution on the other chain. No global rollback is offered. Compensation, if chosen, appears as a new authorized action with its own recorded cost.
- **Residual duties.** Whatever branch is taken, the duty list never empties silently. Default, expiry and revocation leave debt in place.

## Boundaries the page must state exactly

- **Moriarty:** signed constraints, financial meaning, four acceptance judgments, persistent duties, typed condition policies.
- **Kernel:** optional; collects intentions, connects solvers, obtains evidence, arranges constrained signing, submits external transactions, executes authorized recovery. It cannot weaken constraints and confers no deployment license. Permissionless programs need no maintainer involvement.
- **Midnight:** native ZKIRv3 proof relation, state consumption and phase semantics. A failed fallible phase can retain guaranteed-phase effects and fees.
- **External domains:** own finality, own enforcement. Inclusion is not execution. A signature-only destination is the named weak point.

## Mobile, accessibility, static fallback

- The simulator is a React component rendering ordinary form controls and a definition list; no canvas. Panels stack vertically under 720 px.
- Every state change is announced through an `aria-live` region. Buttons carry text labels, not icons. Colour is never the sole carrier of accepted versus rejected.
- Static fallback: the page ships a pre-rendered HTML table of the full worked scenario, every stage and every adversarial case, visible when JavaScript is off and linked as "print view." The boundary table is plain HTML in both modes.
- No external fonts or scripts. Math, if any, uses the existing KaTeX-to-MathML pipeline.

## Technical delivery and acceptance checks

- **Location.** New `site/src/kernel/` component tree plus a `docs/kernel.md`-style prose source consumed by the same React app. `build-docs.mjs` currently rejects local links outside its two-page list; extend the allow-list rather than bypassing the check, so the new page is linkable from both references and vice versa.
- **Data.** Transition table and scenario as a typed JSON module with a Node test under `src/data/`, matching the existing `npm test` glob. Test assertions: caps never exceeded on any path, duty list monotone until explicit discharge, refund and late success mutually exclusive, replay rejected, every status string drawn from the product contract's list.
- **Site checks.** `npm run verify` passes. The Playwright spec asserts the README banner still loads on the index, both reference pages resolve unchanged, and the new page has exactly one `h1`.
- **Content checks.** Every paragraph carries a fact/design/UI tag. Grep for "verified," "live," "real-time" and "deployed" fails the build unless the sentence is a negation. ZKIRv4 appears only as a requirements label. No Lean dependency claim.
- **Review.** Six reviewer personas as the user requested, three Fable 5.1 medium and three Astra medium. Each reviews independently against this plan and the source packet, checking one axis each: treasury comprehension, boundary accuracy, adversarial cases, accessibility, build hygiene, roadmap honesty.

## Tradeoffs

The simulator's transition table is hand-written, so it can drift from the language as U0 freezes syntax. Mitigation: the table cites source sections, and the page states it teaches target design, not current grammar. Omitting a real evaluator call keeps the page honest and dependency-free but means readers cannot run their own program; the quickstart link covers that. Restricting to one two-asset scenario limits breadth but matches the roadmap's next discriminator exactly.

**Sources:** README.md; docs/MORIARTY-CONSOLIDATED-DESIGN.md responsibility boundary and conditional settlement sections; docs/MORIARTY-PRODUCT-CONTRACT.md partial transactions and status list; docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md target semantic interfaces; ROADMAP.md U3 brief; deliverables/defiformal-study-2026-09-19/RESULT.md; site/package.json and site/scripts/build-docs.mjs.