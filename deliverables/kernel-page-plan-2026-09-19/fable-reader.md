# Plan: interactive Federated DeFi Kernel explainer (GitHub Pages)

## Context

The Moriarty site already publishes two reference pages (requirements, syntax and semantics) built by `site/scripts/build-docs.mjs`, plus a React/Vite index. Those pages explain the language formally. There is no route that lets a technically literate newcomer understand what the optional Federated DeFi Kernel does, what it cannot do, and where Moriarty, the kernel, Midnight and external chains each stop. This plan proposes one additional educational page. It is planning only. Nothing here claims new proof, ledger or deployment capability.

## 1. Repository facts (from the source packet only)

- Site stack: Vite 8, React 19, TypeScript 7, KaTeX rendered to MathML at build time. Scripts: `dev`, `build` (`docs` then `tsc -b` then `vite build`), `check`, `test` (node --test over `src/data/*.test.mjs`), `test:site` (Playwright via python), `verify`.
- `build-docs.mjs` builds exactly two HTML pages from `site/docs/*.html`, enforces one h1 per page, and throws on unrecognized local links. It also writes redirect aliases, including `kernel.html -> requirements.html#architecture`. A new page named `docs/kernel.html` would collide with that alias.
- README opens with `docs/assets/moriarty-banner.png`, links the two reference pages, and describes the kernel as "a separate, optional system".
- Repository status: local source/5 evaluation, finite K comparisons, scoped Preview loan/swap runs. Kernel integration, OWS and x402 adapters are planned, not implemented. The local demos do not sign, prove, submit or move assets.
- Design constraints the page must carry verbatim in meaning: hard intent constraints survive solver choice; federation is optional and permissionless programs need no maintainer license; ZK, MPC and TEE are distinct assumptions; a signature-only destination is exposed to threshold compromise; no global cross-chain rollback; partial progress retains fees, budgets and duties; ZKIRv3 is the current target and the ~March 2027 recursion date is a user planning assumption; Lean is not a dependency.

## 2. Target design being explained (not implemented)

CAKE/APSS layering: Applications express purpose, Permission records authority, Solvers search within it, Settlement adapters establish effects. The kernel coordinates collection of intentions, solver matching, evidence gathering, constrained signing, external submission, finality observation and authorized recovery. It cannot weaken a signed intention. Midnight verifies the native relation. Each external domain keeps its own execution and finality meaning. Unresolved outcomes are retained for reconciliation, never coerced into success or failure.

## 3. Format options and recommendation

**Option A: Scrollytelling walkthrough.** One long page, one worked scenario, diagrams update as the reader scrolls. Strong narrative, weak for reference and revisiting a specific concept.

**Option B: Layered explorer.** A fixed four-layer boundary diagram (Owner/Moriarty, Kernel, Midnight, External domains) plus a scenario stepper that lights up the responsible layer at each step and offers branches (success, failure, unknown). Each layer opens progressive-disclosure panels from one-sentence to formal detail. Good for reference and for showing boundaries, slightly less cinematic.

**Option C: Sandbox intent editor.** Reader edits intent fields and a solver proposal, and a checker shows which hard constraints pass or fail. Most engaging, but tempts fake fidelity: the site would appear to "verify" plans it cannot verify.

**Recommendation: Option B, with one bounded piece of Option C** (an intent-versus-plan checker limited to a fixed rule table, clearly labelled as a design model rather than the evaluator). Rationale: the user's controlling constraints are all boundary statements, and Option B makes the boundary the primary visual. Option A cannot show branching outcomes without long duplication. Option C alone invites a fake deployment feel.

## 4. Narrative and page structure (UI recommendation)

Entry point: a new route `site/kernel-explainer/index.html` (built by Vite as a second entry) linked from the README below the two reference links as a third, educational route. The README banner and both reference pages are untouched. Do not use `docs/kernel.html` (alias collision).

Sections, in order, each readable before its detail is opened:

1. **What you get** (benefit first). Two paragraphs: an owner signs what they will accept; anyone may propose how; nothing is accepted unless it fits. Then one sentence: the kernel is optional plumbing for the multi-party, multi-chain case.
2. **Who does what.** The four-layer boundary diagram. Tap or hover a layer to see its one-line duty, then "more" to the responsibility table rows from the consolidated design.
3. **One agreement, step by step.** Scenario stepper (Section 6).
4. **Why the solver cannot cheat.** Intent-versus-plan checker (bounded Option C).
5. **Three kinds of evidence, three kinds of trust.** ZK / MPC / TEE cards with explicit assumption lists and the signature-only-destination warning.
6. **When things go wrong.** Failure and unknown branches, recovery authority, exclusive terminal outcomes, no global rollback.
7. **Where this stands today.** Status ladder: implemented locally, run on Preview, specified only. Recursion horizon shown as a planning assumption with its recording date.
8. **Read further.** Links to the two reference pages and the roadmap.

Each section uses a three-tier disclosure: headline sentence, plain explanation, formal terms with a link into `docs/requirements.html` or `docs/language.html`.

## 5. Interactions (input / state / output)

**Boundary diagram.**
- Input: select layer; select a concern row (intent, proofs, atomicity, recovery, privacy...).
- State: `{layer, concern}` in URL hash so links are shareable.
- Output: highlighted cell text from the responsibility table, plus the neighbouring cells dimmed but visible so the reader sees the split, never one layer alone.

**Scenario stepper.**
- Input: Next/Back; at branch points choose Success, Failure, Unknown.
- State: step index, chosen branch, an accounting ledger `{gross spent, fees, reserved, remaining budget, open duties, unresolved legs}` held in React state; deterministic, no timers, no randomness.
- Output: the diagram lights the responsible layer; the ledger panel updates; a caption states exactly which fact was established and by whom ("Midnight authenticated the escrow state", "External domain reported inclusion; finality unknown").

**Intent-versus-plan checker.**
- Input: reader picks a plan from a short list (or adjusts three sliders: route fee, recipient, gross spend) against a fixed signed intent (spend at most 11 A, fees at most 1 A, receive at least 20 B, named recipient and domain, partial fills allowed with retained duty).
- State: the rule table and the candidate.
- Output: pass/fail per hard constraint, with the failing rule named in plain words. A persistent banner reads "Design model. Not the Moriarty evaluator, no proof, no ledger."

**Evidence cards.** Toggle "shared operators?" to show that stacking ZK, MPC and TEE on shared infrastructure does not add independent assurance.

No dashboards, counters or simulated live activity anywhere.

## 6. Worked scenario (content of the stepper)

Owner authorizes: spend at most 11 A including at most 1 A fees, receive at least 20 B, named recipient, partial fills allowed, recovery policy signed separately, escrow funded on Midnight.

Steps:
1. Owner signs intent and evidence policy. (Owner / Moriarty)
2. Kernel collects it and offers it to solvers. (Kernel)
3. Two solvers propose routes; reader picks one. Both fit; a third, over-fee plan is shown rejected. (Kernel proposes, Moriarty rules decide)
4. Budget reserved: 11 A gross, 1 A fee ceiling. (Moriarty accounting, Midnight state)
5. Leg 1 fills 12 B on Midnight, spending 6 A and 0.4 A fee; continuation records 8 B remaining and duties intact. (Midnight)
6. Leg 2 submitted to an external chain. Branch:
   - **Success**: authenticated result returns, 8 B received, agreement closes. Terminal outcome consumes the authority.
   - **Failure**: authenticated failure returns. Fees already paid stay paid; leg 1 stays committed; remaining budget and the residual duty to receive 8 B are visible; recovery policy is now the only usable authority.
   - **Unknown**: timeout. Caption states that a timeout proves nothing about execution. State shows "unresolved". Reader may attempt refund; the page shows that refund and a late success are mutually exclusive terminal outcomes and that the design requires one to consume the claim. No option offers rollback of leg 1 or of the external chain.
7. Status footer for the scenario: "This flow is the target design. Today only local evaluation and scoped Preview examples exist."

## 7. Boundaries the page must draw exactly

- **Moriarty**: canonical signed intent, financial meaning, four acceptance judgments, obligations and continuations. Proves compliance, not profitability.
- **Kernel**: optional coordination; can refuse service; cannot alter validity or Midnight acceptance; not required for direct Midnight programs or private handoff.
- **Midnight**: verifies the bound native relation, authenticates local state, enforces consumption and phase semantics; ZKIRv3 target.
- **External domains and issuers**: their own finality, native validity, and named trust assumptions; a signature-only account is bypassed if its threshold is compromised.
- **Project process**: reviewers, Foreman, registries and licenses never gate a public program.

## 8. Mobile, accessibility, static fallback

- Diagram is inline SVG with `<title>` and `<desc>`; layer selection uses real buttons, keyboard reachable, `aria-pressed`, focus outlines. Stepper is a `<section>` with `aria-live="polite"` captions and visible headings.
- Below 720px the four layers stack vertically and the ledger panel moves under the diagram; nothing relies on hover.
- Colour never carries meaning alone; pass/fail uses icon and text.
- `prefers-reduced-motion` disables highlight transitions.
- Static fallback: all prose, the full responsibility table, the three scenario branches and the evidence assumption lists are server-rendered in the HTML at build time (or written as plain HTML in the entry file). With JavaScript off the reader gets the complete linear explanation; interactivity is enhancement only.

## 9. Technical delivery

- Add `site/kernel-explainer/index.html` as a Vite multi-page entry with `site/src/kernel/` React components: `BoundaryDiagram`, `ScenarioStepper`, `IntentChecker`, `EvidenceCards`.
- Put the responsibility table, scenario steps, ledger transitions and checker rules in `site/src/data/kernel-explainer.ts` so `npm test` can cover them with `node --test` like existing data tests.
- Reuse `docs/styles.css` tokens for visual continuity with the reference pages; add a header link back to the two references.
- README: add one line under the reference links pointing to the explainer as an educational route. Banner unchanged.
- No network calls, no analytics, no wallet libraries.

## 10. Acceptance checks

- `npm run verify` passes; the two reference pages build byte-identical to before.
- Data tests: each scenario branch ends with fees unchanged or increased, never reduced; reserved plus spent never exceeds 11 A; residual duty non-zero on Failure and Unknown; no branch produces a rollback of a committed leg; the over-fee plan fails exactly the fee rule.
- Playwright: page loads with JS disabled and still shows all three branches; keyboard-only navigation reaches every control; URL hash restores diagram state.
- Content lint (simple grep test): page contains "planning assumption" next to the 2027 date, "optional" next to "federation", "threshold" warning text, and no occurrence of "live", "real-time" or numeric metrics outside the scenario ledger.
- Manual review: a reader can state the kernel's benefit after section 1 without reading later formal detail.

## 11. Tradeoffs

- Limiting the checker to fixed rules avoids implying the site verifies programs, at the cost of less exploration.
- A separate Vite entry adds build surface but keeps `build-docs.mjs` and its link checker untouched.
- Server-rendered fallback duplicates scenario text in HTML and in data; the data test that both match guards drift.

## 12. Source references

README.md (kernel section, use cases, status); docs/MORIARTY-CONSOLIDATED-DESIGN.md (responsibility table, conditional settlement, ZK/MPC/TEE, recursion assumption); docs/MORIARTY-PRODUCT-CONTRACT.md (permissionlessness, ZKIRv3, no Lean, failure outcomes); docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md (recovery, external effects); ROADMAP.md (U3 discriminator scenario, U5 federation scope); deliverables/defiformal-study-2026-09-19/RESULT.md (no rollback from atomic model); site/package.json and site/scripts/build-docs.mjs (build facts, alias collision).

