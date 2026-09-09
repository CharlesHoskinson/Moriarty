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
not traceable to them.

## Framing decision

The site tells the story of the **target architecture** — proof-carrying
financial settlement as designed — rather than annotating every element with its
current maturity. Delivery status lives in **one consolidated roadmap section**
at the end (CONTENT-SPEC §9), which carries the twelve sprints and the
milestones actually reached.

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
- The 24 action targets are the map of *what you can build*; the distinguishing
  tests are *the bug this catches*.
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
- The 24 action targets are the conformance surface.

Implementation: mode is one piece of state at the app root, persisted in
`localStorage`, defaulting to Build. It changes copy, section ordering within a
section, and which depth layer of a shared visualization is expanded — it does
not fork the component tree into two apps.

## Section spine (both modes)

1. **Opening** — what Moriarty is; the one-line claim; the ancestry (ACTUS, the
   DeFi corpus, Marlowe).
2. **The category model** — F1–F6/P, the eight mandatory facets, intents-as-
   execution-facet and bridges-splitting-by-trust.
3. **The category tabs** — the section that carries the weight. One tab per DeFi
   category, eight tabs, complete: seven economic families plus one cross-cutting
   tab. Every tab renders the same six blocks (what it is · facet profile ·
   action targets with distinguishing tests · what goes wrong · what Moriarty
   does · reference sources), and all 24 action targets are accounted for.
   Fully specified in [`CATEGORY-TABS.md`](CATEGORY-TABS.md); the completeness
   checks at the end of that document are acceptance criteria, not suggestions.
4. **Composition** — the 5.14× cross-category failure result; the five operators.
5. **The formalization model** — five specification layers; the compilation
   pipeline; Core vs. surface; the Midnight realization (a sequence of bounded
   one-transition proofs).
6. **The guarantees** — layered assurance vocabulary; the twelve properties with
   assumptions; the four mandatory proof claims; the security/residual-risk table.
7. **The DSL** — profiles, real source, the `policy` block, the declaration set,
   the developer workflow.
8. **Intents · multichain · safe DeFi** — the five artifacts; affine gross
   authority; per-layer atomicity and the bridge-rollback challenge; the four
   safety mechanisms.
9. **Roadmap** — twelve sprints, three parallel tracks, milestones reached.

## Centrepiece — decided by competition

Five design agents each receive this plan and the content spec and propose a
different centrepiece visualization plus a full storyboard. Their proposals land
in `site/storyboards/<agent>/`. One is then selected (or synthesized) before any
application code is written.

Each agent owns a distinct angle so the five proposals are genuinely different
rather than five variants of one idea:

| Agent | Angle |
|---|---|
| 1 | **The category surface** — the eight tabs as one navigable object; how a reader moves across F1–F6/P × 8 facets × 24 action targets without losing place |
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

Completeness is an explicit requirement of this site, not an aspiration. The
category section must cover every DeFi category with the same structural depth —
a category with thinner repository evidence gets a shorter tab, never a missing
one or a placeholder. The acceptance checks are at the end of
[`CATEGORY-TABS.md`](CATEGORY-TABS.md).

The same rule applies to the other fixed sets: twelve properties, four mandatory
proof claims, thirteen threat rows, twelve sprints. A design that shows "the
main ones" and elides the rest has failed the brief.

## Constraints

- No AI, agent, model or assistant attribution anywhere — not in the site, its
  source, its comments, its assets or its commit messages.
- No third-party logos, no fabricated testimonials, no invented metrics.
- The counts are fixed: 7 families, 8 facets, 24 action targets, 5 composition
  operators, 4 mandatory proof claims, 12 properties, 12 sprints.

## Sequence

1. Knowledge graph over the repository — done, `graphify-out/`.
2. Plan and content spec — this document and `CONTENT-SPEC.md`.
3. Five storyboard proposals — `site/storyboards/`.
4. Selection or synthesis of the centrepiece and the storyboard.
5. Implementation of the single-page application.
