# Moriarty website — plan

Branch: `website`. Worktree: `.worktrees/website`. Output: `site/`.

## Goal

A single-page React application that explains, to two different audiences at
once, what Moriarty is: the DeFi category model it targets, the formalization
model it uses, the guarantees that model produces, and the DSL a developer
writes — including how that DSL carries intents, multichain transactions and
safe DeFi.

Substance is fixed by [`CONTENT-SPEC.md`](CONTENT-SPEC.md) and, for the category
section, [`CATEGORY-TABS.md`](CATEGORY-TABS.md). Nothing goes on the site that is
not traceable to them. Register and diction are fixed by [`VOICE.md`](VOICE.md),
which governs every line of copy and forbids the tallies.

## Framing decision

The site tells the story of the **target architecture** — proof-carrying
financial settlement as designed — rather than annotating every element with its
current maturity. Delivery status lives in **one consolidated roadmap section**
at the end (CONTENT-SPEC §9), which carries the sprints and the milestones
already reached.

This is a narrative decision, not a licence to overclaim. The rules in
CONTENT-SPEC §10 still bind: every number real, every assumption named beside
its claim, no invented verified badges.

## Two modes

The site has a persistent mode switch. Both modes render the same nine sections
and the same underlying data; they differ in entry path, emphasis, depth and
vocabulary. A reader can switch at any point and stay in place.

### Mode A — **Build** (DeFi protocol engineers)

The default. Reader knows AMMs, lending, liquidations, ERC-4626, MEV; does not
necessarily know K, EBNF, operational semantics or ZK.

- Entry: the failure they recognize — a partial repayment that quietly touches
  principal, a rounding direction that drains a vault, a refund that restores
  spending authority.
- The DSL is the hero. Real `.mori` source, early and often.
- Formal machinery explained in their terms: "the type system will not let you
  add `Debt<USD>` to `Amount<USD>`" before "typing judgment `Γ ⊢ e : τ`".
- The action targets are the map of what you can build, and each distinguishing
  test is the bug that target catches.
- Guarantees framed as what breaks without them.

### Mode B — **Verify** (PL and formal-methods researchers)

Reader knows semantics; DeFi is the motivating application.

- Entry: the specification stack — lexical rules, EBNF, typing judgments, K
  operational semantics, correctness claims over those semantics.
- The layered assurance vocabulary (CONTENT-SPEC §4) is the hero, with the
  chain from semantic validity through to wallet correctness drawn explicitly.
- The property inventory shows the Marlowe/Isabelle evidence, the assumption
  that qualifies it, and the Moriarty obligation — three columns, no collapsing.
- Felleisen–Hieb reduction semantics, the K configuration, the guard counts
  (15 / 28), the ZKIR deferral decision and its reasoning.
- The action targets are the conformance surface.

Implementation: mode is one piece of state at the app root, persisted in
`localStorage`, defaulting to Build. It changes copy, section ordering within a
section, and which depth layer of a shared visualization is expanded — it does
not fork the component tree into two apps.

## Section spine (both modes)

1. **Opening** — what Moriarty is; the one-line claim; the ancestry (ACTUS, the
   DeFi corpus, Marlowe).
2. **The category model** — the economic families, the mandatory facets, intents
   as an execution facet, and bridges splitting by trust model.
3. **The category tabs** — the section that carries the weight. One tab per DeFi
   category, and a final tab for the cross-cutting targets. Every tab renders the
   same blocks in the same order: what the category is, where it sits on each
   facet, the action targets with their distinguishing tests, what goes wrong,
   what Moriarty does about it, and where the requirements come from. Specified
   in [`CATEGORY-TABS.md`](CATEGORY-TABS.md), whose acceptance criteria bind the
   build.
4. **Composition** — the measured cross-category failure result, and the
   operators DA24 names.
5. **The formalization model** — the specification layers, the compilation
   pipeline, Core against the surface language, and the Midnight realization,
   where a long-lived agreement is a sequence of bounded one-transition proofs.
6. **The guarantees** — the layered assurance vocabulary, the properties with the
   assumptions that qualify them, the mandatory proof claims, and the trust
   boundaries with their residual risk.
7. **The DSL** — profiles, real source, the `policy` block, the declaration set,
   the developer workflow.
8. **Intents · multichain · safe DeFi** — the five artifacts; affine gross
   authority; per-layer atomicity and the bridge-rollback challenge; the four
   safety mechanisms.
9. **Roadmap** — the sprints, the tracks running in parallel, and the milestones
   already reached.

## Centrepiece — decided

Five design agents each receive this plan and the content spec and propose a
different centrepiece visualization plus a full storyboard. Their proposals land
in `site/storyboards/<agent>/`. One is then selected, or synthesized, before any
application code is written.

**Decided: the Register, with the Fork inside it.**

The Register is the centrepiece and the navigation spine. Its columns are the
category tabs, its rows are the facets, and the action targets sit in the columns
they belong to, so family, facet and target are all reachable without leaving the
surface. A facet row header lifts that one facet across every family at once,
which is the move that shows the model is a classification.

Inside a target, the Fork renders the distinguishing test: one pre-state, one
action, two post-states. The left is what a plausible balance-based
implementation writes. The right is what the test requires. The line where they
part is the only coloured line on the page, and what the plausible model cannot
hold is set in grey rather than in red, because this is a precision argument and
not a scare page. Where the repository supports it the Fork computes rather than
quotes, so a reader who asks for 19,744 B gets the named guard failure.

The guarantee chain takes the guarantees section, and it keeps the three-column
property table and the threat table with residual risk set as the loudest column.
The pipeline trace takes the formalization section. The annotated source reader
takes the language section, with its line references opening the reader at the
line a claim depends on.

The Register's reading counters are cut. They were the strongest idea in that
proposal and they are still forbidden: a readout of targets read and tabs opened
is the counting habit moved into the interface. The grid shows its own extent, and
a cell the reader has opened is marked in neutral ink without a number anywhere.

Each agent owns a distinct angle so the five proposals are genuinely different
rather than five variants of one idea:

| Agent | Angle |
|---|---|
| 1 | **The category surface** — the tabs as one navigable object; how a reader moves across family, facet and action target without losing place |
| 2 | **The pipeline** — one transaction traced from `.mori` through Core, Compact, ZKIR, proof, ledger |
| 3 | **The guarantee chain** — the layered assurance vocabulary and what each layer does and does not establish |
| 4 | **The source** — the DSL itself as the centrepiece; code as the primary visual element |
| 5 | **The failure** — the distinguishing tests; each guarantee introduced by the bug it catches |

## Technical shape

- Single-page React application. Vite + React 19 + TypeScript. Node 24 is
  available; npm 11.
- No backend. All content ships as typed data modules under `site/src/data/`
  (families, facets, action targets, properties, threats, sprints, code
  samples), so the visualizations read from one source rather than hard-coded
  markup.
- Routing is in-page section navigation with real anchors and scroll sync.
- Visualization: prefer hand-authored SVG driven by the data modules over a
  charting dependency. Reach for a library only if a proposal genuinely needs
  force-directed layout.
- Motion is used to explain (a trace advancing, a layer resolving), never as
  decoration. Respect `prefers-reduced-motion`.
- Accessibility is a requirement, not a pass: keyboard-navigable diagrams,
  real text in SVG rather than paths, sufficient contrast in both themes.
- Responsive to ~400px. Dense tables get their own horizontal scroll container;
  the page body never scrolls sideways.

## Completeness

Completeness is a requirement of this site rather than an aspiration. The
category section covers every DeFi category at the same structural depth. A
category with thinner repository evidence gets a shorter tab, never a missing one
and never a placeholder. The same holds for the properties, the mandatory proof
claims, the threat rows and the sprints: a design that shows the main entries and
elides the rest has failed the brief.

The reader is never told that the coverage is complete. The surface shows it. A
tally of what has been covered, or of what the reader has opened, is forbidden by
[`VOICE.md`](VOICE.md), and that rule outranks any storyboard.

## Constraints

- No AI, agent, model or assistant attribution anywhere — not in the site, its
  source, its comments, its assets or its commit messages.
- No third-party logos, no fabricated testimonials, no invented metrics.
- The fixed sets are fixed. Nothing is rounded off, extended or dropped, and the
  test at `src/data/completeness.test.mjs` enforces this against the data layer.

## Sequence

1. Knowledge graph over the repository — done, `graphify-out/`.
2. Plan and content spec — this document and `CONTENT-SPEC.md`.
3. Five storyboard proposals — `site/storyboards/`.
4. Selection or synthesis of the centrepiece and the storyboard.
5. Implementation of the single-page application.
