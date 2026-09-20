## Rationale

Developers and solver builders currently reach Moriarty through a long README and two dense reference pages. Neither shows, in one place, how a signed intention bounds a solver, how the optional kernel coordinates without gaining authority, and what happens when an external leg fails or stays unknown. One additional explainer page can teach the benefit first and link to the formal detail second. It must not imply that proofs, kernel adapters or deployment exist today.

## Format options and recommendation

1. **Guided scroll explainer with embedded islands.** Narrative sections with five small React widgets, each prerendered to static HTML.
2. **Single stateful lifecycle app.** One tabbed simulator holding intent, routes, stages and trust settings in shared state.
3. **Static HTML in docs/.** Built by the existing docs script with `<details>` and no JavaScript.

I recommend option 1. Option 2 turns a teaching page into a dashboard and hides narrative behind tabs. Option 3 cannot show route rejection or unknown outcomes interactively. Option 1 keeps reading order, degrades to static tables, and reuses the existing Vite and React setup.

## Repository facts

- The site uses Vite 8, React 19 and TypeScript 7 with scripts for check, test, build, verify and a Playwright site test.
- The docs script renders exactly two reference pages, enforces one h1 per page, rejects unknown local links, and writes alias pages. One alias already maps kernel.html to the requirements architecture anchor.
- The README banner lives at docs/assets/moriarty-banner.png and the README lists two reference pages.
- Local commands can evaluate the atomic loan and swap examples and the source/5 loan lifecycle. Scoped Preview loan and swap runs exist. Kernel, OWS and x402 adapters are planned only.
- The roadmap U3 discriminator is spend at most 11 A including at most 1 A fees to receive at least 20 B.

## Target design the page must state

- Hard constraints in signed intent survive every solver choice. Solvers choose only within authorized freedom.
- Federation is optional. No maintainer, council, registry or privileged solver approval is needed for a supported program.
- ZK, MPC and TEE carry separate assumptions. A destination that accepts only a threshold signature is bypassable if that threshold is compromised.
- There is no global cross-chain rollback. Compensation is a new authorized action with its own costs.
- Partial progress retains fees, budgets and duties. Refunds do not replenish gross authority.
- Timeout does not prove nonexecution. Refund and late success are exclusive terminal outcomes.
- ZKIRv3 is the current target. ZKIRv4 is a requirements label. The March 2027 recursion horizon is a user planning assumption.
- Lean is not a dependency. OWS gives wallet interoperability, not authority. x402 separates authorization, payment finality, result availability and delivery.

## Narrative and page structure

The page is a second Vite entry at site/kernel/index.html with components under site/src/kernel/. The existing alias page stays untouched.

1. **Hero.** One benefit sentence and a status banner stating the page is a design explainer with simulated checks that never sign or submit.
2. **From a transaction to an agreement.** The README buyer and seller story, condensed, showing remainder, incurred fees and once-only authority.
3. **Who does what.** A four-column table for Moriarty, kernel, Midnight and external domains, copied in substance from the consolidated design table.
4. **Island A, intent builder.**
5. **Island B, route checker.**
6. **Island C, stage walker.**
7. **Island D, trust boundary selector.**
8. **Island E, OWS delegation and x402 paid request.**
9. **Where this stands.** Planning assumptions, what the local demos actually do, links to both reference pages and the quickstart.

## Interactions

**Island A, intent builder.** Inputs are gross cap, fee cap, minimum net, asset pair, recipient and a partial-fill toggle. State is a canonical intent object. Output is a signing-display card listing each hard constraint, labelled as simulated and unsigned.

**Island B, route checker.** Two candidate routes are prefilled, one labelled human authored and one labelled AI solver. Each has fees, expected receipt, recipients and stage count. Toggles introduce a raised fee, a refund that tries to replenish budget, and a hidden second debit. Output is a per-constraint pass or fail list naming the violated clause. A rejected candidate is shown as uncommitted and never changes Island C state.

**Island C, stage walker.** A step button advances stages. Each stage has a success, failed or unknown radio. Output shows cumulative gross, fees, remaining order, retained duties and exposure. The recovery button enables only when the signed recovery policy toggle is on. After a refund, a late-success toggle displays the exclusive terminal rule instead of a second discharge.

**Island D, trust boundary selector.** Checkboxes for ZK, MPC and TEE, plus a radio for destination enforcement that either verifies the relation or accepts a signature only. Output is a prose statement of what a threshold or hardware compromise achieves. A shared-operator toggle removes any independence claim.

**Island E, wallet and paid request.** A stepper walks authorization, payment finality, result availability and delivery predicate. A retry keeps the same logical request identity. A duplicate charge is blocked until reconciliation completes. An OWS request outside delegated scope is refused with the scope shown.

## Worked scenario

The scenario reuses the U3 numbers so the page and roadmap agree.

| Step | Gross A | Fees A | B received | Remaining | Outcome |
|---|---|---|---|---|---|
| Signed intent | cap 11 | cap 1 | min 20 | | policy recorded |
| Route 1 fill | 6.5 | 0.4 | 12 | 8 B | committed partial |
| Route 2 external leg | 4.0 reserved | 0.5 | unknown | 8 B | unresolved after timeout |
| Rejected candidate | 11.4 | 1.4 | 21 | | uncommitted, fee cap violated |
| Refund path | 4.0 released | 0.5 retained | 12 | 8 B unfilled | duty retained |
| Late success after refund | | | | | excluded by terminal consumption |

The unknown row stays unknown until an authenticated observation arrives. The page says explicitly that the timeout changes which remedies may be attempted and proves nothing about the other chain.

## Boundaries

- **Moriarty** owns the signed intent, financial meaning, the four judgments, obligation persistence and recovery semantics.
- **Kernel** collects intentions, proposes routes, reserves budgets, transports evidence, operates optional threshold signing, observes finality and executes permitted remedies. It cannot weaken constraints.
- **Midnight** verifies the native relation and enforces its own state and consumption rules.
- **External domains** report only their stated evidence under their own finality and enforcement rules.

## Mobile, accessibility and static fallback

Each island prerenders its default state as a plain HTML table, and a noscript block shows the same table. Controls are native inputs with labels. Output panels use polite live regions. Focus order follows reading order. Animation respects reduced-motion settings. Below 720 pixels the layout is single column and tables scroll horizontally. No canvas, charts or live metrics appear anywhere.

## Technical delivery

- Add pure functions in site/src/data/kernel-scenarios.mjs covering route checking, stage advance, trust statements and x402 steps, with a test file picked up by the existing node test script.
- Add React components under site/src/kernel/ and a second entry in the Vite config.
- Add a nav link in the docs script nav array only. Both reference bodies remain unchanged.
- Add a third bullet to the README reference list. The banner stays.
- The page makes no network calls and holds no wallet.

No Pages deployment step is assumed beyond the existing build output.

## Acceptance checks

- The site verify script passes and the Playwright test gains a kernel page load, tab order and noscript render check.
- A grep rejects banned claims such as deployed, live, audited, ZKIRv4 released or Lean required.
- Every island shows a simulated label and the boundary table matches the design document table.
- Both reference pages are unchanged apart from the nav line, and the banner path is intact.
- A 360 pixel screenshot shows no overflow outside scrollable tables.
- Keyboard-only and reduced-motion runs pass manually.

## Tradeoffs and sources

Islands add JavaScript to a documentation site. Static prerender and a pure-function core keep that cost low and testable. Wiring islands to the real evaluator would imply native proof support, so the page links to the quickstart instead. Sources are the README, the consolidated design, the product contract, the requirements alignment note, the roadmap, the DeFiFormal result and the site build scripts. The requested review persona mix of three Fable 5.1 medium and three Astra medium reviewers is a review-process choice and does not affect page content.
