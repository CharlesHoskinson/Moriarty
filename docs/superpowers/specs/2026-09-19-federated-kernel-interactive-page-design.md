# Interactive Federated DeFi Kernel page design

Status: consolidated planning proposal, 2026-09-19. Six independent planners contributed: three Claude Fable 5.1 medium and three GPT-6 Astra medium. This document selects and reconciles their recommendations; it does not represent six approvals of the final synthesis. No page implementation or deployment is claimed.

## Purpose and audience

Explain how the optional Federated DeFi Kernel helps people complete financial agreements across actors and chains while Moriarty preserves the meaning of their authorization. A reader should leave able to distinguish a proposed route, an accepted stage, external settlement, unresolved exposure and a remaining obligation. The primary audience is a DeFi developer or technically literate user. Treasury practitioners, solver builders and protocol engineers can expand detail without changing the underlying account.

Proposed title: **The Federated DeFi Kernel**. Opening: “You define the agreement. Solvers find possible routes. The kernel coordinates the work across participants and chains, while each accepted stage must satisfy the agreement.” Follow with a short explanation of partial progress and conditional settlement. Mark the experience as an interactive illustration of the target design; integrations remain development work.

The page explains a system, not a deployed service. Its examples neither sign nor submit transactions. Program authors need no project membership, reviewer approval or deployment license. Asset-owner authorization and application-specific conditions still apply. A direct Moriarty-to-Midnight path must be visible alongside the optional federation path.

## Format decision

| Format | Benefit | Cost | Decision |
|---|---|---|---|
| Illustrated article with a bounded scenario explorer | Coherent first reading, inspectable consequences, complete static fallback | Requires a small educational state model | Main format |
| Interactive architecture atlas | Useful ownership and trust reference | Starts with jargon and may imply central control | Use a compact diagram and inspector inside the article |
| Free-form intent editor or execution playground | More open exploration | Large input space and risk of appearing to validate real programs | Exclude from first delivery; link existing developer quickstart |

The article remains readable in document order. Interactions reveal consequences rather than gate access to prose. Use explicit buttons instead of scroll-controlled animation. Avoid a dashboard, simulated traffic, throughput counters, wallet connection or a security score.

## Reading sequence and visual design

1. **An agreement that spans transactions.** Introduce one buyer and consenting provider, a spending limit, delivery goal, signatures and document evidence. Distinguish a requested outcome from the route used to obtain it. Introduce the simple two-asset example before terminology.
2. **Who does what.** A compact diagram separates the owner/application, Moriarty rules, optional kernel services, Midnight and external systems. Label arrows by meaning: signed policy, candidate, required evidence, submitted effect, authenticated observation. State that this is a responsibility map, not an observed deployment topology.
3. **Follow the agreement.** Inspect a fixed intention, choose a candidate and advance through evidence, reservation, partial delivery and a second external attempt. Keep the event record and financial account adjacent.
4. **When an outcome is unknown.** Compare late success, authenticated failure and still-unknown outcomes. Explain what remains spent, reserved and owed. Recovery does not erase a committed prefix.
5. **What each form of evidence establishes.** Select ZK, threshold signing/MPC and TEE descriptions; inspect common statement bindings and each mechanism's assumptions. Compare external enforcement profiles.
6. **People and AI solvers use the same authority.** Explain planned OWS delegation and the x402 distinction between payment, availability and delivery. A small disclosure contains a request/retry trace. Solver identity changes no hard constraint.
7. **What exists, what remains.** Show current scoped local capabilities separately from target integration and native recursive/private history. Link requirements, syntax/semantics, roadmap and source evidence. Explain the DeFiFormal semantic reference without implying a deployed federation or a Lean dependency.

Use the existing site's typography, spacing and theme. A desktop view can place the agreement and accounting beside the event list; mobile uses the same content in a single column. Prefer labeled SVG or HTML lanes to a draggable canvas. Formal judgments and backend identities belong in disclosures after the practical explanation.

## Responsibility and evidence contract

| Participant or layer | Responsibility | Limit readers must understand |
|---|---|---|
| Owner and application | Choose and consent to constraints, counterparties, disclosures and remedies | A request cannot impose a duty on an unconsenting recipient |
| Moriarty | Specify financial meaning, authority, complete effects, required evidence and continuing duties | Cryptography cannot prove unexpressed wishes or external truth |
| Optional kernel | Coordinate solvers, evidence, reservations, configured custody/signing, submission, observation and authorized recovery | Cannot amend signed constraints or make service eligibility a language license |
| Midnight | Enforce the actual native proof relation and its own ledger's state and consumption rules | Does not impose those checks on a foreign destination |
| External chain, issuer or service | Supply its stated execution, finality, attestation or delivery evidence | Inclusion, successful execution, finality and recipient delivery are distinct |

Explain CAKE's Applications, Permission, Solvers and Settlement as concerns crossing these responsibilities. Permission means owner/application authority. APSS is not a required sequential pipeline or a deployment admission scheme.

Four target judgments remain distinct: contract properties, intent refinement, transition validity and history compliance. The page can explain them and show illustrative rule checks; it cannot display them as native proofs produced by JavaScript. Source, Core, ZKIR, verifier keys and effects have distinct identities connected by required correspondence.

## Worked scenario and exact accounting

Use the roadmap's illustrative goal: spend at most **11 A gross**, including at most **1 A fees**, and receive at least **20 B on successful completion**. Name toy domain-qualified assets and a recipient; make clear these are fixtures, not market prices. Internally represent amounts in integer hundredths: gross cap 1100 and fee cap 100. The signed fixture allows two partial deliveries. Each costs 5 A principal plus 0.5 A fee for 10 B. The provider explicitly accepts its delivery/remedy duties before those duties can arise.

The condition policy requires the named provider's signature, the recipient's acceptance and a fresh authorized issuer's document attestation. The document commitment identifies evidence; it does not establish legal truth or physical delivery by itself. A later threshold-condition example may explain alternatives, but the initial scenario uses an explicit conjunction.

The financial account distinguishes cumulative actual debit, incurred fees, active reservation, confirmed receipt, custody/location of remaining assets and accepted duties. An active reservation is exposure against authority, not another debit or proof that an asset is safely in escrow. Keep opening funded balance and actual refund separate from unspent authorization capacity. Funding/custody movements must have explicit treatment in the fixture; never silently count or exclude them from a gross cap. The first implementation can start after funding, with its funding accounting stated as a precondition, rather than invent an unmodeled deposit transition.

| Event | Actual gross A | Incurred fees A | Reserved A | Confirmed B | Meaning |
|---|---:|---:|---:|---:|---|
| Candidate proposed | 0 | 0 | 0 | 0 | No accepted effects or newly imposed duties |
| First fill finalized in illustration | 5.5 | 0.5 | 0 | 10 | Accepted prefix; remaining accepted duties persist |
| Second fill reserved/submitted | 5.5 | 0.5 | 5.5 | 10 | Spent plus reserved equals 11; another solver cannot reserve it again |
| Timeout; result unknown | 5.5 | 0.5 | 5.5 | 10 | Same exposure; timeout is not nonexecution |
| Late authenticated success | 11 | 1 | 0 | 20 | Completion only if every other required predicate also holds |
| Authenticated failed attempt; its 0.5 A fee retained | 6 | 1 | 0 | 10 | Principal nonexecution and reservation release established by the fixture evidence policy; remedy remains pending |
| Still unknown | 5.5 | 0.5 | 5.5 | 10 | No invented recovery or automatic release |

The two final-result rows are alternative branches from the same submitted attempt. Failure leaves 5 A ordinary capacity and no fee capacity. It does not imply every real adapter has this failure policy. Refunds preserve the cumulative debit and fee counters. The unfilled order and legally/semantically accepted obligation are distinct; name the consenting duty bearer, the duty and its discharge condition.

A recovery grant alone does not establish safe refund eligibility. The default unknown branch permits reconciliation, not a payout. A remedy transition needs its signed authority, required authenticated evidence, controlled assets, remaining resources and an enforceable consumption rule. If these are absent, the outcome remains unresolved. If showing an actual refund in a later fixture, specify and account for the asset location, source of recovery fees and duty discharge before adding the button. No compensating payment or cancellation may masquerade as rollback.

Illustrate duplicate terminal observation on the successful branch: it records no second payment or discharge. A foreign signature-only account can still suffer a conflicting transfer under compromised signers. A local exclusive-state rule cannot retroactively stop that foreign effect; the trust inspector must show this limitation.

## Interaction specification

| Control | State change | Visible result |
|---|---|---|
| Candidate selector | Select one of two compliant presets or a wrong-recipient/over-fee candidate before commitment | Named illustrative checks; invalid candidate leaves agreement account untouched |
| Human/AI label selector | Presentation identity only | Identical authority and acceptance rules |
| Evidence selector | Fresh authorized, stale, wrong issuer, missing or unsupported evidence fixture | Specific predicate met/unmet/unknown/unsupported; missing is not false |
| Next event | Apply one permitted event to the scenario reducer | Event record, accounting delta, responsibility and remaining duty |
| Observation selector | Choose late success, authenticated failure or continued uncertainty | Branch-specific result with retained prior effects |
| Try conflicting event | Attempt duplicate reservation, replay or premature refund | Explanation and unchanged account; foreign compromise explored separately |
| Evidence inspector | Select mechanism, binding field or external enforcement profile | Claim, assumption, required common bindings and possible bypass |
| Reset illustration | Restore a new uncommitted fixture | Explicit announcement; never described as transaction rollback |

Back/forward controls, if present, navigate recorded explanatory snapshots only. They do not undo modeled effects. Changing a signed fixture or candidate after progress requires an explicit reset/new illustration. Avoid arbitrary post-signing cap sliders. Keep the policy immutable during a run.

Use a small typed event reducer. Keep candidate validity, evidence availability, external status, financial account and consumed identifiers separate. Derive totals from one scenario source. Tests must cover every reachable event in the bounded fixture, including adverse ordering; this does not establish formal correctness of Moriarty or the kernel.

## Trust, privacy and history views

ZK establishes the encoded relation under the actual cryptographic/verifier assumptions. Threshold signing distributes signing power under a corruption model. TEE attestation concerns a measured environment under hardware, freshness and rollback assumptions. Bind the same intention, program, domain, stage, epoch and effects. Show shared operators as correlated dependencies, not multiplying independent protection.

Compare a signature-only destination with a hypothetical destination that enforces specifically named additional checks. A toy 3-of-5 signing example is explicitly illustrative, not the chosen federation configuration. Compromising its threshold can bypass honest signers' policies at the signature-only destination. There is no generic “proof-enforcing chain” that automatically checks every Moriarty property.

A compact disclosure explains continuation history: accepted predecessors, remaining duties, consumed claims and a next stage. Private witness availability is needed for progress; a valid proof alone does not hand the next participant its witness. Shared ancestry is not duplicate spending. Do not implement an unrestricted split/join simulator in the first page. Keep full recursion linked to the requirements workstream. ZKIRv3 remains the target; proposed next-version requirements and the user's dated approximately March 2027 planning assumption are not release facts.

The AI/service disclosure traces a fixed logical request: scoped authorization, payment observation, result availability and recipient delivery. Retry preserves request identity and requires reconciliation; reconciliation alone does not authorize another charge. OWS and x402 are planned integrations, with no interoperability demonstration implied.

## Route and implementation outline

Choose **`/Moriarty/kernel.html`**, sourced from `site/kernel.html`, as an additional educational route. Preserve the existing `docs/kernel.html` redirect and both reference URLs. Link from the homepage and the README's kernel explanation; preserve its banner and its two reference bullets. A link in the explanation avoids recasting the interactive article as a third normative reference.

Proposed files:

- `site/kernel.html`: complete static narrative, semantic diagram equivalent, default and adverse transcripts, source links.
- `site/src/kernel/main.tsx` and `KernelExplorer.tsx`: a small React enhancement mounted into a separate root.
- `site/src/kernel/styles.css`: scoped layout and accessible interaction styling.
- `site/src/data/kernel-scenarios.mjs` with a declaration file: immutable fixtures and pure transitions, consumed by React and existing Node test tooling.
- `site/src/data/kernel-scenarios.test.mjs`: independent expected account/authority outcomes for the fixture.
- `site/test/kernel.spec.py`: browser behavior, accessibility and fallback checks.
- `site/vite.config.ts`: second HTML entry while retaining the relative project-path asset strategy.
- `site/CONTENT-SPEC.md`: narrow precedence correction so its older Compact/history statements do not override the consolidated contract; link the kernel content source.
- `site/src/App.tsx`, `README.md`, `.github/workflows/site.yml`: navigation and browser check integration.

Delivery slices, all still unimplemented:

1. Freeze the educational policy, source annotations, branch outcomes and account treatment. Resolve any remaining fixture ambiguity before UI code.
2. Deliver the static article and second Vite entry with full fallback and working deep links.
3. Add and test the bounded transition model; enhance the article with controls, account and event display.
4. Add the evidence inspector and concise AI-service/history disclosures, using the same claim distinctions.
5. Perform content, browser and comprehension review; publish through the existing Pages workflow and check live bytes/routes.

Preserve the two reference pages' no-script and two-item documentation-navigation checks. A new browser suite covers the interactive page instead of weakening the reference suite. No wallet SDK, analytics, remote fonts, network dependency or proof service is needed for the educational model.

## Acceptance criteria

- A new reader can explain who proposes a route, who authorizes it, who enforces local and external effects, and why unknown does not mean failed.
- The default scenario completes within its stated caps; wrong recipient and excessive fee cases fail by their actual conditions.
- Failed and unknown branches retain the correct fees, reservations and accepted duties; reservation is not equated with custody.
- A second solver cannot reserve already consumed/pending authority; duplicate results cannot produce a second discharge.
- Timeout cannot enable a refund merely because time elapsed or a recovery signature exists. A signature-only compromise remains visible as a separate external risk.
- No model result is called a native proof or actual settlement. Status statements link to scoped repository sources.
- Controls work by keyboard, have visible focus and text labels, and do not rely on color, hover or dragging. Announce the concise result rather than rereading the whole page.
- At 320 CSS pixels and 200% zoom the reading order and account remain usable. Reduced motion, dark theme and print retain meaning. A screen-reader check supplements automation.
- With JavaScript disabled or initialization failing, the complete article and success/failure/unknown transcripts remain readable. Reveal controls only after successful initialization.
- Typecheck, existing data tests, production build, both existing browser suites and the new kernel suite pass. Test direct refresh and assets under `/Moriarty/`, not only at origin root.
- Existing reference URLs and redirect destinations still resolve. README banner remains intact. Browser tests validate educational behavior only.

## Source basis and proposal disposition

Controlling sources: [consolidated design](../../MORIARTY-CONSOLIDATED-DESIGN.md), [product contract](../../MORIARTY-PRODUCT-CONTRACT.md), [language reconciliation](../../MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md), [roadmap](../../../ROADMAP.md) and [DeFiFormal scope study](../../../deliverables/defiformal-study-2026-09-19/RESULT.md). The [six proposals and dispositions](../../../deliverables/kernel-page-plan-2026-09-19/SYNTHESIS.md) preserve differences and source identity. This plan does not modify language requirements or assert that a new protocol has been implemented.
