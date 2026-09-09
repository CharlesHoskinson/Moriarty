# Storyboard: the pipeline

Proposal 2 of 5. Centrepiece angle: **one transaction traced from `.mori` to the
Midnight ledger**, with the value's representation and the guarantee owed at
every stage. Full centrepiece specification in [`CENTREPIECE.md`](CENTREPIECE.md);
working prototype in [`prototype.html`](prototype.html).

Every number, name and code sample below is taken from `site/CONTENT-SPEC.md`
or `site/CATEGORY-TABS.md`, or is integer arithmetic on the numbers stated
there. Nothing else is asserted.

---

## Design read

Two expert audiences, one page. The page is an explainer, not a marketing
funnel: it has no signup, no pricing and no social proof, and the brand is
exactness. So the visual system is the one a careful financial document would
use if it were interactive.

- **Palette.** Cool neutrals (off-white paper, off-black ink) and one accent, a
  desaturated cobalt, used for exactly three things: the value being traced,
  the currently selected stage, and links. Nothing on the page is ever green.
  There are no badges, ticks or "verified" states anywhere; the repository does
  not support them and the brand does not need them.
- **Type.** A system sans for prose; a monospace for every number, identifier,
  type, hash field and source line. Numbers are never abbreviated: 19,940,000,000,000
  is written in full. Thousands separators in prose, none in source.
- **Layout.** A 1120px content column with a wider 1360px allowance for the
  centrepiece rail and for dense tables. Left-aligned throughout. Section
  transitions are hairline rules, not colour blocks. No cards except where a
  container carries real meaning (a stage panel, a tab panel, a threat row).
- **Motion.** Used in three places only: the trace advancing along the rail,
  the assurance ladder filling as a stage is reached, and tab panels
  cross-fading. Everything else is static. Under `prefers-reduced-motion`
  every transition becomes an instant state change.
- **Radius.** One scale: 4px on inputs and chips, 8px on panels. No pills.

### Global chrome

**Top bar** (64px, sticky, translucent over content, hairline below):

- Left: the wordmark `Moriarty` and, in smaller monospace, `bounded financial
  contracts on Midnight`.
- Centre: section index as nine short labels, current one underlined
  (`Opening · Categories · Tabs · Composition · Formalization · Guarantees ·
  DSL · Intents · Roadmap`). Collapses to a `Sections` menu below 1024px.
- Right: the **mode switch**, a two-segment control labelled `Build` and
  `Verify`, with the active segment in ink and the other outlined. Switching is
  instant and keeps scroll position on the same section anchor. State lives at
  the app root and persists in `localStorage`; the default is Build.

Beside the switch, a one-line description appears for 4 seconds after a
switch, then fades (instantly hides under reduced motion):

- Build: `For protocol engineers. The source is the hero.`
- Verify: `For semanticists. The assurance chain is the hero.`

**Theme** follows `prefers-color-scheme`, with a manual toggle in the footer.

---

## Section 1: Opening

**Anchor:** `#opening`. **Height:** one viewport (`min-height: 100dvh`),
content top-aligned with 96px top padding.

### What the reader sees

An asymmetric split. Left column (7 of 12): the headline, one paragraph, one
button. Right column (5 of 12): a still, cropped preview of the centrepiece
rail, already showing the value chip `10,000 A → 19,743 B` sitting at the first
stage, greyed except for the chip. It is a real render of the component in
its resting state, not an illustration. Clicking it jumps to Section 5.

### Copy

Headline (both modes):

> **Financial meaning that survives compilation, proof and settlement.**

Build paragraph:

> Compact gives you circuits and state. It does not tell you which asset an
> amount denotes, how interest rounds, when a payment falls due, what a
> participant authorized, or which obligations survive the transaction.
> Moriarty puts those questions in the type system and the operational
> semantics instead of the audit.

Verify paragraph:

> A bounded financial-agreement language with a five-layer specification:
> lexical rules, EBNF syntax, typing and scoping judgments, an executable
> operational semantics in the K Framework, and explicit correctness claims
> over those semantics. Every transaction carries a proof that its execution,
> and the history it extends, satisfies the agreement.

Button (single primary CTA for the page): `Follow one transaction` → scrolls to
Section 5 and starts the trace.

Below the fold line, still inside the section, a three-column strip headed
**Design ancestry**, each column a short definition list rather than a card:

- **ACTUS.** Reference behavior for events, state transitions and cash flows.
  Moriarty must reproduce it including dates and rounding. Scope: 277
  fixtures, 18 executable types, 32 source-backed taxonomy dispositions.
- **The DeFi corpus.** 72 protocol rows across the historical 12-category set.
  Implementation and conformance requirements, not a runtime service.
- **Marlowe.** A financial-contract DSL built so behavior can be analyzed
  before execution. Moriarty adopts the goal and builds its own authoring,
  proof and settlement architecture for Midnight.

One closing sentence in both modes: *Moriarty is a new language. It is not a
renamed Marlowe and not a Compact dialect.*

### Interaction

None beyond the CTA and the preview link. No scroll cue.

### Mode differences

Paragraph swaps as above. In Verify mode the ancestry strip gains the
one-liner `Isabelle lineage pinned at marlowe-lang/marlowe
7b5b1e90c171eae2674a6fc08aa7d8caa92b16af` under the Marlowe entry.

---

## Section 2: The category model

**Anchor:** `#categories`.

### What the reader sees

A two-level diagram drawn in SVG from the data modules, occupying the full
content width. Top row: seven family tiles in a single line (`F1` to `F6`, `P`),
each with its name. Below, a row of eight facet labels. Between the rows, thin
connecting strokes from every family to every facet, so the diagram reads as a
matrix rather than a hierarchy: every instance has a value on all eight.

Two callouts are anchored to specific facets and stay visible without hover:

- On **Execution**: `Intents live here. An intent is a way of authorizing an
  action, not a kind of product.`
- On **Settlement**: `Bridges live here, and they split: message-verified
  trust and custodial trust are different assumptions. Collapsing them is a
  category error.`

### Copy

Headline: **Two levels: what kind of thing it is, and the eight properties
every instance has.**

Body (Build):

> Product labels are not categories. The same protocol carries several facets
> and can appear in more than one family. The families are the map of what you
> can build; the facets are what you have to answer about each one.

Body (Verify):

> The families organize packages and evidence. The facets make the model a
> classification rather than a list. Neither is Moriarty source syntax; the
> language has no `F2` keyword.

The families table and the facet list are rendered in full; the seven family
identifiers and eight facet names are never abbreviated or reordered.

### Interaction

Hovering or focusing a family tile emphasizes its strokes and shows its
example list from the spec (`AMMs, order books, aggregators, routing` for F1,
and so on). Clicking a family tile jumps to the matching tab in Section 3 and
selects it.

### Mode differences

Verify mode adds a line under the diagram: `Cross-cutting action targets DA20
to DA24 supply observation, authorization, governance, messaging and
composition to every family.` Build mode instead shows the sentence `24 action
targets sit under these families. Each has the test that separates a correct
implementation from a plausible wrong one.`

---

## Section 3: The category tabs

**Anchor:** `#tabs`. This is the longest section and carries the completeness
requirement. The centrepiece is elsewhere, so this section is designed to be
quiet, regular and exhaustive.

### What the reader sees

Headline: **Eight categories. Twenty-four action targets. Every one of them.**

A horizontal tab strip with eight tabs in the fixed order
`F1 · F2 · F3 · F4 · F5 · F6 · P · Cross-cutting`. Each tab label carries the
family identifier, the full family name, and its action-target count
(`3`, `7`, `2`, `2`, `1`, `3`, `1 + 1 shared`, `5`). At 400px the strip becomes
a horizontally scrolling row inside its own overflow container; the page body
never scrolls sideways.

Below the strip, a single tab panel. Every panel renders the same six blocks in
the same order with the same headings, so the eye learns the structure once:

1. **What this category is** (function, then boundary, as two short
   paragraphs).
2. **Facet profile**: an eight-row definition list, facet name left, position
   right. All eight rows always render, including `none` and
   `not applicable`.
3. **Action targets**: one block per target with ID, action, requirement, and
   the distinguishing test set in a larger weight. DA12 renders in both F3 and
   P with the tag `shared with P` / `shared with F3`.
4. **What goes wrong**: a mechanism, stated as prose, never as anecdote.
5. **What Moriarty does about it**: prose plus real `.mori` where the brief has
   it (F1 policy block and guards; F2 `payInterest`, `Transfer`/`Repay` emits,
   notional guard).
6. **Reference sources**: one paragraph.

Above the panel, a persistent one-line **coverage index** in monospace lists
DA01 to DA24 as small chips; the chips belonging to the open tab are filled,
the rest outlined, DA12 shown half-filled in both F3 and P. This is the
reader's proof that nothing is missing, and it is generated from the data
module so it cannot drift from the panels.

### Copy of the eight panels

The panel text is taken verbatim, block by block, from `CATEGORY-TABS.md`
sections F1 through Cross-cutting. Illustrative headline lines per tab, shown
under the tab name in the panel:

- F1: *Convert one asset into another and, in doing so, produce a price.*
- F2: *Nominal debt is not a transfer.*
- F3: *Payoffs that reference something else rather than conveying ownership
  of it.*
- F4: *An exit request is not immediate token delivery.*
- F5: *A representation does not establish a claim's economic substance.*
- F6: *"Vault" is the most overloaded word in the vocabulary. Always qualify
  it.*
- P: *Not "what is the number" but "what happened, and who says so".*
- Cross-cutting: *What makes the other seven composable.*

The Cross-cutting panel additionally carries the 5.14× composition result, the
five composition operators listed as five, the governance paragraph and the
messaging paragraph, exactly as in the brief.

### Interaction

- Tabs are a WAI-ARIA tablist: left/right arrows move, Home/End jump, the panel
  is `aria-labelledby` its tab.
- Switching tabs cross-fades the panel over 160ms; instant under reduced
  motion. Panel height changes are not animated.
- The open tab is written to the URL hash (`#tabs/f2`) so a link lands on a
  specific category.
- Each action-target block has a small `See it in the trace` link where the
  centrepiece covers that target (DA01 for the swap; DA06 for the loan
  repayment). It scrolls to Section 5 and sets the centrepiece example.
- In the F1 panel, the sentence `Requesting 19,744 produces a named slippage
  failure` is a link that opens the centrepiece with `min_out = 19,744`.

### Mode differences

Same panels, different emphasis:

- Build: block 4 (**What goes wrong**) is rendered first after the
  introduction, then block 3, then block 5. The failure leads.
- Verify: canonical order 1 to 6. Block 2 (**Facet profile**) is expanded by
  default; in Build it is collapsed behind a disclosure reading
  `Facet profile (8 facets)`.

### Acceptance

All eight tabs, all six blocks, no empty states. DA01 to DA24 each present, DA12
twice and marked shared. Identifiers presented as classification labels, never
as syntax. Five operators always listed as five. These are checked by a test
over the data module, not by eye.

---

## Section 4: Composition

**Anchor:** `#composition`.

### What the reader sees

A single large figure: 1,830 small squares in a grid, 1,645 outlined and 185
filled in ink, with the 182 cross-category failures grouped together and the
3 within-category failures set apart. Below the figure, the two rates and the
ratio in monospace:

```
within-category    2.10%
cross-category    10.79%
ratio              5.14×
```

Right of the figure, the five composition operators as a vertical list:
`sequence · parallel · interleave · synchronize · message`, each with the
one-line rule `operator-specific authority, duties, conflicts and fan-in`.

### Copy

Headline: **Composition across categories is where DeFi breaks.**

Body:

> Over 1,830 eligible protocol pairs, 1,645 compose cleanly and 185 fail. 182
> of the failures are cross-category, 3 within-category. That is a 5.14×
> difference, and it is exactly what a type system and an operational
> semantics can police.

Footnote, both modes: *An earlier informal "sixty times" claim was checked and
is wrong.*

### Interaction

Hovering the filled cluster shows `182 cross-category failures`; the three
apart show `3 within-category failures`. Nothing else.

### Mode differences

Verify adds `DA24 names the operators; split partitions work and claims, join
cannot duplicate resource, and that is checkable against Core's
value-conservation property.` Build adds `Each operator is a bug class: a join
that duplicates a resource, a parallel branch that spends the same authority
twice.`

---

## Section 5: The formalization model. The centrepiece.

**Anchor:** `#formalization`. This section is pinned while the trace runs.

### What the reader sees

Headline: **One transaction, seven representations, six guarantees.**

Sub-line: *10,000 A in. 19,743 B out. Follow the number.*

Then the centrepiece, **The Trace**, full width (up to 1360px). A horizontal
rail of seven stations:

```
.mori  →  Elaboration  →  Core  →  Compact + manifest  →  compactc / ZKIR 3  →  Proof  →  Ledger and wallet
```

A value chip (`in 10,000 A · out 19,743 B`) travels along the rail. Beneath the
rail, a stage panel with three regions:

- **Representation**: what the value looks like at this stage (source line,
  typing judgment, exact intermediate integers, Core action, manifest fields,
  circuit inputs, claim set, ledger fields, wallet summary).
- **Established here / still owed**: the six-layer assurance ladder (semantic
  validity → translation validity → compiler correctness → circuit/proof
  correctness → ledger feasibility → transaction construction and wallet
  correctness), with layers marked as fixed at this stage, fixed earlier, or
  owed downstream, and the residual risk that remains even when fixed.
- **Named failures at this stage**: the guards that can reject here, by their
  exact message.

Two controls above the rail:

- **Example**: `Swap` (default) or `Repayment`.
- **Input**: for the swap, `min_out`: `19,743` or `19,744`. For the repayment,
  `payment`: `10 USD` or `11 USD`.

Beneath the stage panel, the second half of the idea: **One agreement, many
proofs**. A strip of transition boxes for the agreement's lifetime, each box
being one bounded transition with its own four claims and a `HistoryCompliance`
arrow to its predecessor. The swap is one box; closure is another. This is the
visualization of *a sequence of bounded one-transition proofs, not one circuit
for a whole lifetime.*

Full details, states, keyboard and reduced-motion behavior:
[`CENTREPIECE.md`](CENTREPIECE.md).

### Scroll behavior

On desktop and tablet the section pins for the length of seven stages; the
scroll position selects the stage, so reading down the page walks the value
along the rail. The rail also accepts direct clicks and arrow keys, and a
`Run` button plays the trace unattended. On phones and under reduced motion the
section does not pin; stations are selected by tap or keyboard and the panel
stacks below the rail.

### Copy on the stage panels

The seven stage titles and their one-sentence claims (both modes):

1. **Source `.mori`**: *The developer wrote the fee, the rounding, the
   guards and the transfers. Nothing is implied.*
2. **Typed elaboration and certificate**: *Units are checked, bounds are
   certified, and every intermediate integer is exact.*
3. **Canonical Moriarty Core**: *Everything from the surface has elaborated
   away. What remains is what gets proved.*
4. **Generated Compact and correspondence manifest**: *A reviewable backend
   artifact, and a record of what it corresponds to.*
5. **compactc: ZKIR 3, TypeScript, proving and verifier artifacts**: *The
   supported compiler produces the circuit. Direct source-to-ZKIR was deferred
   on purpose.*
6. **Proof of one transition**: *Four claims, fixed names, fixed order, none
   optional.*
7. **Midnight ledger and wallet**: *Sealed fields, mutable public state, and a
   signing summary a person can read.*

Stage 5 carries the deferral decision as a two-column decision note:

> **Deferred: direct source-to-ZKIR generation.**
> ZKIR is a typed straight-line circuit IR with guarded impacts and no
> source-level financial concepts. It is ledger-coupled and evolving.
> **Chosen instead: generate Compact.** It preserves a reviewable backend
> artifact and reuses the supported compiler, source maps, runtime bindings and
> ledger operations.

Below the centrepiece, two compact reference tables that belong to this
section and are rendered in full:

- **The five specification layers** (Lexical · Syntax · Static semantics ·
  Dynamic semantics · Correctness claims), with method and what it fixes.
- **Core versus surface**: what Core retains, what the surface adds, and the
  eleven manifest fields.

### Mode differences

- Build: the Representation region leads with source and integers; the
  assurance ladder is collapsed to a one-line summary (`Fixed here: units and
  bounds. Still owed: five layers.`) that expands on click.
- Verify: the assurance ladder is fully expanded by default; the
  Representation region adds the typing judgment
  `Γ ⊢ arg.amount_in : Amount<AssetA_quantum>`, names the K configuration at
  stage 3, and shows the visibility classification of each datum
  (`public / private / committed / revealed`). The stage-3 panel also carries
  the sentence *The control layer is presented using Felleisen-Hieb reduction
  semantics.* The reference tables gain the "why EBNF, why K" paragraph from
  the brief.

---

## Section 6: The guarantees

**Anchor:** `#guarantees`.

### What the reader sees

Headline: **Each layer is a separate obligation. Establishing one does not
establish the next.**

The six-layer assurance chain drawn once more, this time horizontally and
alone, as the summary of what the trace just showed. Each layer is a plain
labelled box; between boxes, a short vertical tick with the word `then`. No
arrows suggesting inevitability.

Then three fixed sets, each rendered in full:

1. **The property inventory**: a twelve-row table with three columns
   (`Property · Assumption that qualifies it · Moriarty obligation`). No
   collapsing, no "top properties". The table sits in its own horizontal
   scroll container at narrow widths.
2. **The four mandatory proof claims** as a numbered list with the names in
   monospace: `ContractInvariant`, `IntentRefinement`, `TransitionValidity`,
   `HistoryCompliance`, each with its one-line meaning. Under the fourth:
   *A valid proof of a valid transition against an invalid history must not be
   accepted. This is what separates Moriarty from contract-level
   verification.*
3. **Security and trust boundaries**: the threat rows from the brief with
   `Threat · Required control · Residual risk` columns; the residual-risk
   column is set in the same weight as the threat so it cannot be skimmed
   past.

Closing line: *A single "Moriarty audit" is not an adequate claim.* followed by
the seven audit surfaces from the brief as a comma list.

### Interaction

Hovering a layer in the chain highlights the trace stage where it is
established (a small `stage 4` label appears) and offers `Show in the trace`,
which scrolls up to Section 5 at that stage.

### Mode differences

- Verify: this section is the hero after the centrepiece; the property table
  is expanded with the Isabelle lineage note. Order: chain, properties,
  claims, threats.
- Build: order is chain, claims, threats, properties; the property table is
  behind a disclosure `Twelve properties and their assumptions`, and each
  threat row gains a one-line *what breaks without it* rendered from the
  residual-risk column's phrasing.

---

## Section 7: The DSL

**Anchor:** `#dsl`.

### What the reader sees

Headline: **The language.**

A full-width source block with real `.mori` and line numbers, followed by
prose. Two blocks, both from the brief:

1. The successor-profile `PartialPayment` agreement in full. Line 14,
   `ensures post.principal == pre.principal;`, has a margin note:
   *The whole thesis in one line. Paying interest must not touch principal.*
2. The atomic-profile `swap` action excerpt, with its guards and the two
   `emit Transfer` lines. Margin notes on `floor_div` (*named rounding*) and on
   `"minimum output not met"` (*named failure*).

Between them, the **`policy` block** (`accrued_interest`, verbatim), with the
six fields annotated in the margin: unit, derivation, rounding direction,
remainder disposition, comparison basis, proof obligation. Caption: *Rounding is
a declared, named, provable artifact, not an accident of integer division.*

Then two short reference blocks:

- **Profiles**: `moriarty-bounded-atomic/1` (implemented: grammar, parser, type
  checker, canonical encoding, local evaluator, restricted Compact lowering;
  the loan and swap examples run under it) and `moriarty-successor-syntax/0`
  (provisional: separate lexical rules, EBNF grammar, bounded parser, canonical
  formatter, read-only CLI `check-syntax` and `format`). They are not
  interchangeable.
- **Declarations**: `unit, party, const, state, observation, settlement,
  status, policy, reserve, effect, action`; inside an action `guard, let, set,
  emit`; on the agreement `lifetime` and `horizon`.

Finally the **developer workflow** as a nine-step vertical list from the brief,
with the two workspace rules set apart in bold:

- *Simulation and static analysis are distinct. A successful example does not
  turn the property list green.*
- *A valid proof does not make a stale predecessor live.*

And the three-column workspace (`AGREEMENT · BEHAVIOR · TRANSACTION`) as a
labelled wireframe with the field lists from the brief.

### Interaction

Hovering a margin note highlights its source line. The `policy` block's six
fields can be stepped through with a small `next field` control that moves the
highlight; static under reduced motion.

### Mode differences

- Build: this section is the hero. It is placed immediately after the trace
  in the navigation index emphasis, the source blocks are opened at full
  height, and the workflow is expanded.
- Verify: source blocks are rendered at the same size; the profile block gains
  the specification-layer references and the prose leads with `requires /
  next / post` and explicit pre- and post-state as the observable shape.

---

## Section 8: Intents, multichain, safe DeFi

**Anchor:** `#intents`. Three subsections under one headline.

Headline: **What you authorized, where it settles, and what cannot happen.**

### 8a. Intents

A five-box horizontal sequence: `Intent · Permission · Plan · Execution ·
Receipt`, with the three definitions (IntentIR, PlanIR, Receipt) underneath.
Then the two signing profiles side by side (`Exact-plan signing`,
`Outcome-intent signing`) as two columns of plain text with their properties.

Then **the authority rules**, rendered as a numbered list with the sentences
from the brief, and one worked line in monospace that the centrepiece's wallet
stage already showed:

```
gross debit    10,000 A    (fees count against gross authority)
net receipt    19,743 B    (goals compare net, fees count against the goal)
refund         never restores allowance
```

Closing: *Wallet rendering is derived from the canonical signed meaning: a
deterministic semantic signing summary, not a hex blob.*

### 8b. Multichain

Headline inside the subsection: **Atomicity is defined per layer and per
route.**

The contradiction register as a three-row "claimed / documented" table
(marketing claim on the left, verifier documentation on the right), then the
three rules, then the challenge set in a bordered block:

> Construct a cross-chain swap trace in which the internal batch succeeds but
> the bridge withdrawal fails or is rolled back. Any model that cannot
> represent that trace, and reject it, is not modelling cross-chain
> settlement.

And the bridge split, drawn as two separate labelled lines that never merge:
`message-verified trust` and `custodial trust`.

### 8c. Safe DeFi

Four mechanisms as a 2×2 grid (four items, four cells): typed asset identity
and units; declared rounding and remainder policy; affine authority with gross
accounting; history compliance as a proof obligation. Each cell names the
failure class it kills. Below: the adversarial-fixture requirement and the two
rules: *ABI shape does not establish semantic compatibility* and *a valid
oracle signature does not establish economic truth.* Then: *Flash borrowing is
a capability with legitimate uses, not a vulnerability.*

### Interaction

Each mechanism cell links to the category tab whose failure it addresses
(1 → F1/F5, 2 → F6, 3 → Cross-cutting DA21, 4 → F4).

### Mode differences

Build leads with 8c (the mechanisms), then 8a, then 8b. Verify leads with 8a,
adds the anti-vacuity rule (*trace inclusion and a feasible positive witness*)
and the validity predicate `notBefore <= now < expiresAt` in monospace, then
8b, then 8c.

---

## Section 9: Roadmap

**Anchor:** `#roadmap`. The single consolidated status section.

### What the reader sees

Headline: **Twelve sprints. Three tracks. What has been reached.**

A twelve-row table (`Sprint · Deliverable · Decisive completion evidence`)
rendered in full, in its own horizontal scroll container at narrow widths. No
progress bars, no percentages, no colour states.

Above the table, the three parallel tracks as three short horizontal chains in
monospace:

```
SP01 → SP02 → SP03                   language
SP01 F0 → SP04 → SP06                native feasibility and proofs
atomic + loan/swap subset → SP05     financial Preview integration
```

Below the table, **Milestones reached**, as three plain paragraphs from the
brief: the bounded K definition (16 frozen cases; 15 guards and 18 steps for
Transfer-only; 28 guards and 37 steps for repayment; AccrualFirst /
PrincipalFirst / ProRata; none/floor/ceil rounding), the successor syntax
profile, and the hello-world Compact deployment on a local Midnight network.

### Interaction

None. This section is deliberately still.

### Mode differences

Verify mode appends to the milestone paragraph the guard counts as the
conformance surface: `15 / 28`. Build mode appends `Both the loan and swap
examples run under the implemented profile today.`

---

## Footer

Wordmark, the one-line claim, section index repeated as links, theme toggle,
and the mode switch repeated. Nothing else.

---

## Section-to-layout families

To keep the page from feeling assembled: asymmetric split (1), matrix diagram
(2), tab panel (3), dot-grid figure with side list (4), pinned rail with
stage panel (5), chain plus three tables (6), annotated source (7), sequence
strip plus two-column plus 2×2 (8), plain table (9). No family repeats.
