# Storyboard: the guarantee chain

Proposal 3 of 5. Centrepiece angle: the layered assurance vocabulary, drawn as
six separate obligations with the gaps between them made visible, and the
twelve properties, four proof claims, thirteen threat rows and seven audit
boundaries arranged around it.

Every sentence of copy below traces to `PLAN.md`, `CONTENT-SPEC.md` or
`CATEGORY-TABS.md`. Where the spec points to a repository file for a full set
(the thirteen threat rows in `wiki/security.md`), the four rows not reproduced
in the spec are taken from that file and are marked as such.

---

## Global frame

### The page

One scrolling document, nine sections, real anchors:
`#opening #model #categories #composition #formalization #guarantees #dsl
#intents #roadmap`. Section headings are `h2`. The URL hash updates as the
reader scrolls (IntersectionObserver on each section, threshold 0.4); the
sub-state of the tabs and the chain is also carried in the hash
(`#categories/f2`, `#guarantees/4`) so that a link into the middle of the site
restores the same view.

### The bar

A 64px top bar, full width, background matching the page, hairline underneath
only once the reader has scrolled past the opening. Left: the word
**Moriarty** in the display weight. Centre (desktop only): the nine section
names as plain text links; the current one carries a 2px underline in the
accent colour. Right: the mode switch, a two-segment control reading
**Build** | **Verify**. Below 900px the centre links collapse into a
"Sections" button that opens a full-height list; the mode switch stays
visible at all widths because it is the site's one persistent control.

### The two modes

State lives at the app root, persisted in `localStorage` under `moriarty.mode`,
default `build`. Switching mode never moves the reader: the section stays in
place, the scroll offset is preserved, and only copy, in-section ordering and
the expanded depth of shared visualizations change. Both modes render all nine
sections and every fixed set in full.

The switch is described for a screen reader as "Reading mode, Build or Verify"
and each segment is a `radio` in a `radiogroup`.

### Type and colour

System font stacks only: a grotesk stack for display and body
(`system-ui, "Segoe UI", Helvetica Neue, Arial` in that order of availability),
a monospace stack for source, identifiers and every number that is a
measurement (`ui-monospace, "SF Mono", Menlo, Consolas`). Numbers that are
counts or measurements are always set in the mono stack with tabular figures,
which is the visual expression of "19,743 is 19,743".

Palette: one neutral scale and one accent. Light theme on off-white
(`#f4f5f7`), ink (`#15181d`); dark theme on near-black (`#111317`), text
(`#e6e7ea`). The accent is a muted vermilion (`#b84a1f` light, `#e07a4a` dark)
and it is spent on exactly three things across the whole site: the current
section marker, the "does not establish" gaps in the chain, and the
residual-risk column. Nothing else is coloured. No green anywhere, because the
rules forbid any state that reads as "verified".

Corner radius is 6px on every interactive element and container. Hairlines are
1px at 12% of the ink colour. No drop shadows.

### Motion policy

Motion appears in four places, each with a stated purpose: the chain expanding
from one bar into six obligations (explains the section's thesis), a tab panel
cross-fading (confirms the tab changed), the failure matrix in the composition
section filling in (shows the 182:3 ratio being counted), and the
one-transition-proof strip advancing (shows that a lifetime is many proofs, not
one). Under `prefers-reduced-motion: reduce`, all four resolve to their final
state instantly. Nothing loops. Nothing animates on hover except a colour.

### Responsiveness

Body never scrolls sideways. Below 720px every multi-column grid stacks to one
column. The three dense tables (properties, threats, sprints) keep their
columns down to 720px and become labelled stacked cards below it; a table is
never put in a horizontal scroller when a stacked card would preserve the
column labels, because the column labels are the point.

---

## 1. Opening (`#opening`)

### What the reader sees

Full-height section (`min-height: 100dvh`), content left-aligned in a 12-column
grid: the headline spans columns 1 to 8, the three ancestry entries sit in a
single row beneath it. Nothing is centred. There is no illustration in the
opening; the first `.mori` source appears here instead, in Build mode, at
column 9 to 12, right of the headline, small and complete.

### Headline (both modes)

> **Financial meaning that survives compilation, proof and settlement.**

### Subline

Build:

> A partial repayment that quietly touches principal. A rounding direction that
> drains a vault. A refund that restores spending authority. Moriarty makes
> these part of the type system and the operational semantics, not part of the
> audit.

Verify:

> Lexical rules, an EBNF grammar, typing judgments, an executable operational
> semantics in K, and explicit correctness claims over those semantics: a
> bounded financial-agreement language whose transactions carry proofs.

### The definition paragraph (both modes, below the subline)

> Moriarty is a language and toolchain for bounded financial contracts on
> Midnight. A developer describes financial state, permitted actions, payment
> obligations and authorization rules in a domain-specific language. Those
> descriptions compile to Compact, and every transaction carries a proof that
> its execution, and the contract history it extends, satisfies the agreement.
> It is a new language: not a renamed Marlowe and not a general-purpose Compact
> dialect.

### Right column, Build mode only

The `PartialPayment` agreement from CONTENT-SPEC §5.2, complete, in a bordered
block with the line `ensures post.principal == pre.principal;` carrying a
left rule in the accent colour and the caption beneath:

> One line, the whole thesis: paying interest must not silently touch principal.

In Verify mode the right column instead lists the five specification layers as
a plain vertical list (Lexical structure, Syntax, Static semantics, Dynamic
semantics, Correctness claims) with their methods; the same data drives the
table in section 5, so this is a preview, not a duplicate.

### Ancestry row (both modes)

Three entries, equal width, hairline above each, heading in display weight,
numbers in mono:

- **ACTUS.** Reference behavior for events, state transitions and cash flows:
  interest, principal repayment, maturity. Moriarty must reproduce that
  behavior including dates and rounding. Scope commitment: `277` fixtures,
  `18` executable types, `32` source-backed taxonomy dispositions.
- **The DeFi corpus.** `72` protocol rows across the historical `12`-category
  set. Implementation and conformance requirements, not a runtime service.
- **Marlowe.** A financial-contract DSL built so contract behavior can be
  analyzed before execution. Moriarty adopts that goal and builds its own
  authoring, proof and settlement architecture for Midnight.

### Interaction

None beyond the mode switch. The section has no scroll cue and no call to
action; the bar's section links are the navigation.

---

## 2. The category model (`#model`)

### What the reader sees

Heading, then a two-level diagram built from HTML, not SVG: a row of seven
family tiles, and under it a row of eight facet tiles. Between the rows a
single sentence explains the relationship. Two callouts follow, side by side on
desktop, stacked on mobile.

### Heading

> **Two levels: what kind of financial thing it is, and the eight properties
> every instance has.**

### Family row

Seven tiles, identifiers in mono, names in display weight. Tile order is fixed
and never abbreviated: F1 Exchange and price discovery · F2 Credit and
collateralized debt · F3 Derivatives · F4 Consensus-position claims · F5
Tokenized off-chain claims · F6 Delegated asset management · P Prediction
markets. Each tile's examples line is the spec's "examples of what lands here"
text. Beneath the row, in small type:

> These identifiers organize packages and evidence. They are not Moriarty
> source syntax; there is no `F2` keyword.

### Facet row

Eight tiles, numbered 1 to 8, names never reordered: Execution · Settlement ·
Custody · Legal dependence · Collateral and solvency · Oracles · Authorization
and mandate · Price discovery. Each carries its one-line gloss from §2.2.

Sentence between the rows:

> Product labels are not categories. The same protocol carries several facets
> and can appear in more than one family.

### Two callouts

Left:

> **Intents are an execution facet, not a family.** An intent is a way of
> authorizing an action, not a kind of financial product.

Right:

> **Bridges are a settlement facet, and they split by trust.** Message-verified
> and custodial bridges carry different trust assumptions. Collapsing them is a
> category error.

### Interaction

Hovering or focusing a family tile lifts the facet tiles that the family's
facet profile (from CATEGORY-TABS) marks as defining for that family: F2 lifts
Collateral and solvency; F5 lifts Custody and Legal dependence; P lifts Oracles;
F1 lifts Price discovery. This is a preview of the facet profile block inside
the tabs and links to it. Selecting a family tile scrolls to `#categories/<id>`.

### Mode differences

Build: the family row comes first (the map of what you can build). Verify: the
facet row comes first, with the sentence rewritten as "Eight orthogonal
dimensions; each instance is a point in their product. The families are the
human-facing partition." Same tiles, same data.

---

## 3. The category tabs (`#categories`)

This is the section that carries the weight, and it is complete regardless of
the centrepiece. Eight tabs, six blocks each, twenty-four action targets, DA12
shown twice and marked shared.

### What the reader sees

A tab list across the top of the section: eight tabs, each showing identifier,
label and the count of action targets in mono (`3`, `7`, `2`, `2`, `1`, `3`,
`1+1`, `5`). The identifier is presented as a classification label with the
visual treatment of a tag, never in the code style used for source. Below the
tab list, one panel.

Tab list order and labels are fixed: F1 Exchange and price discovery · F2
Credit and collateralized debt · F3 Derivatives · F4 Consensus-position claims
· F5 Tokenized off-chain claims · F6 Delegated asset management · P Prediction
markets · ⊥ Cross-cutting.

Below 720px the tab list becomes a horizontally scroll-snapping row of pills
inside its own scroll container; the page body does not scroll sideways. The
active pill is scrolled into view when the tab changes.

### The six blocks in every panel

Each block has a fixed heading, rendered as an `h3`:

1. **What this category is** (function paragraph, then boundary paragraph
   under the sub-heading "Boundary").
2. **Facet profile**: an eight-row two-column list, facet name left, this
   family's position right, all eight rows always present, including the rows
   whose honest value is "none". The facet the tab's copy marks as defining is
   set in the display weight.
3. **Action targets**: one card per target. Card header: identifier in mono,
   action name in display weight, family tag. Body: "Requirement" line, then
   "Distinguishing test" line, and the distinguishing test is the emphasized
   element: larger, ink-coloured, with the requirement in the secondary colour
   above it. DA12 in F3 and P carries a "shared with P" / "shared with F3" tag.
4. **What goes wrong**: the mechanism paragraph, verbatim from CATEGORY-TABS.
5. **What Moriarty does about it**: the construct paragraph plus the real
   `.mori` blocks where the spec gives them (F1: the `swap_output` policy and
   the two guards; F2: `payInterest`, the `Transfer`/`Repay` emits, the
   notional guard). The concrete numbers are set in mono and never rounded:
   reserves `1,000,000` A and `2,000,000` B, a `997/1000` fee, a `10,000` A
   input, output `19,743` B; requesting `19,744` produces a named slippage
   failure.
6. **Reference sources**: the pinned references list, verbatim.

### Mode differences

Build order: 1, 4, 3, 5, 2, 6 (what it is, what goes wrong, the targets, what
Moriarty does, the facet profile, sources). Verify order: 1, 2, 3, 5, 4, 6.
The blocks are the same components; only their order changes, and the section
announces the reorder to assistive technology with a single polite live
message ("Blocks reordered for Verify").

Build adds a one-line lead under each distinguishing test in the reader's
vocabulary, taken from the tab's "What goes wrong" text: for DA06, "Interest
and principal are allocated by whichever subtraction the code happened to
perform first." Verify does not add the lead; the test stands alone.

### Cross-cutting tab specifics

The ⊥ tab carries DA20 to DA24, the five composition operators always listed as
five (sequence, parallel, interleave, synchronize, message), the measured
composition result stated again in full, the paragraph "Governance is a
financial action", and the paragraph "Messaging is where cross-chain lives"
with a link to section 8.

### Interaction

Tabs are a proper `tablist`: Left/Right arrows move between tabs, Home/End jump
to the ends, the panel is `tabpanel` labelled by its tab. The hash updates to
`#categories/<id>`. Tab change cross-fades the panel over 160ms; under reduced
motion the swap is instant. Panel height is not animated.

A completeness strip sits under the tab list on both modes, plain text in mono:
`24 action targets · DA12 appears in F3 and P · 8 facets in every profile`.
This is not decoration; it is the acceptance criterion rendered.

### Tie to the centrepiece

At the end of block 5 in every tab, one line in secondary type: "Every
construct on this tab is checked at the first layer of the guarantee chain,
semantic validity. Whether it survives to settlement is a separate question,
answered in section 6." The line is the same in every tab and it is what makes
the category section and the centrepiece one argument rather than two
sections.

---

## 4. Composition (`#composition`)

### What the reader sees

A heading, the measured result stated in one paragraph, and a single
visualization: a matrix of `185` small squares, one per failing pair, laid out
in rows of 20. `182` of them are ink-coloured and `3` are accent-coloured. A
legend reads "182 cross-category · 3 within-category". The clean pairs are not
drawn; instead the caption states them.

### Heading

> **Composition across categories is where DeFi breaks.**

### The result paragraph

> Over `1,830` eligible protocol pairs, `1,645` compose cleanly and `185`
> fail. Of those failures `182` are cross-category and `3` are within-category.
> Within-category failure rate `2.10%`; cross-category `10.79%`: a `5.14×`
> difference. From the DeFiFormal audit, independently reproduced locally.

A footnote in small type: "An earlier informal claim of sixty times was checked
and is wrong."

### The five operators

Beneath the matrix, a row of five equal cells: sequence · parallel ·
interleave · synchronize · message. Caption: "DA24 names the five composition
operators. Each has its own authority rules, duty propagation, conflict
semantics and fan-in behavior."

### Interaction

The matrix fills in when the section enters the viewport, square by square,
ink first then the three accent squares last, over about 900ms. The purpose is
to show the count being made. Under reduced motion it renders complete.

### Mode differences

Build: the heading sentence leads and the operators are introduced as "what
the language can express" ("split partitions work and claims; join cannot
duplicate resource"). Verify: the sentence "that is exactly what a type system
and an operational semantics can police" is set as the heading's second line,
and the operators row links to DA24's conformance requirement in the ⊥ tab.

---

## 5. The formalization model (`#formalization`)

### What the reader sees

Three parts stacked: the five specification layers as a table; the compilation
pipeline as a vertical list with a greyed branch; the Midnight realization as a
horizontal strip of one-transition proofs.

### Heading

> **What a program looks like is separated from what it means, in five
> layers.**

### The layers table

Three columns (Layer · Specification method · What it fixes), five rows,
verbatim from §3. In Verify mode a fourth paragraph block follows the table
with the three "why each choice" points: EBNF versus ABNF, K configurations and
rewrite rules versus typing judgments and Hoare-style assertions, and
Felleisen-Hieb reduction semantics for the control layer. In Build mode those
points are behind a "Why these choices" disclosure.

### The pipeline

A vertical list of seven stages, each a row with a stage name in display weight
and its output in mono:

`Moriarty source (.mori)` → `typed elaboration + finite resource/lifetime
certificate` → `canonical Moriarty Core` → `readable generated Compact +
correspondence manifest` → `compactc` → `ZKIR 3 + generated TypeScript +
proving/verifier artifacts` → `Midnight ledger and wallet`.

To the right of the Core row a dotted branch leads to a greyed box reading
"direct source-to-ZKIR generation: deferred" with the reason in full:

> ZKIR is a typed straight-line circuit IR with guarded impacts and no
> source-level financial concepts, and it is ledger-coupled and evolving.
> Generating Compact preserves a reviewable backend artifact and reuses the
> supported compiler, source maps, runtime bindings and ledger operations.

This is shown as a decision with its reason, as PLAN.md asks.

### Core and surface

Two columns. Left, "Core retains": finite continuations, explicit actions and
waits, accounting, value conservation, explicit timeouts, typed warnings and
errors, a decreasing structural measure; plus the visibility sentence set in
display weight: "every datum is classified `public`, `private`, `committed` or
`revealed`, and every oracle or external effect has a named capability and
assurance boundary." Right, "Surface adds, and elaborates away": modules,
named definitions, schedules, token-indexed amounts, durations, records,
packages, bounded compile-time loops/folds, templates. Under both, the manifest
field list in mono.

### Midnight realization

Heading inside the section:

> **A long-lived agreement is a sequence of bounded one-transition proofs,
> not one circuit that executes an entire lifetime.**

A horizontal strip of five equal boxes each labelled "one transition · one
proof", joined by short connectors labelled "predecessor". The strip enters the
viewport with the boxes resolving left to right (transform and opacity only,
600ms); under reduced motion it renders complete. The five bullets from §3.3
(sealed ledger fields, mutable public state, unverified witness callbacks,
`disclose()` generated only from an explicit source visibility transition,
timeouts as permissionless exported transitions) sit under the strip as a
plain list.

### Mode differences

Build leads with the pipeline; Verify leads with the layers table. The strip
and the Core/surface split are the same in both.

---

## 6. The guarantees (`#guarantees`): the centrepiece

Fully specified in `CENTREPIECE.md` and implemented in `prototype.html`. This
entry describes how it sits in the page.

### What the reader sees on arrival

Heading:

> **Six obligations. Establishing one does not establish the next.**

Beneath it, the chain: six link-shaped elements in a vertical column on the
left, the gaps between them marked "does not establish" in the accent colour;
on the right, the detail panel for the selected obligation. On first entry into
the viewport the six links are drawn stacked into a single bar with no gaps,
and over 700ms they separate into six with the gaps opening. This motion is the
section's argument: collapsing the chain is the standard overclaim, and the
site un-collapses it in front of the reader. Under reduced motion the chain
renders already separated.

Below the chain and panel, in order: the four mandatory proof claims as a row
of four cells; the twelve-property table in three columns; the thirteen threat
rows in three columns with residual risk emphasized; the seven audit
boundaries.

### Mode differences

Verify mode is the chain's native mode: the detail panel opens on the
"Obligation" framing, the property table is expanded by default, and the panel
shows the wiki lineage note under the properties heading ("Evidence reproduced
from pinned Isabelle sources at `marlowe-lang/marlowe`
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`").

Build mode opens the detail panel on the "What breaks without it" framing, the
properties table is present but collapsed under a disclosure reading "The
twelve properties, with the assumption that qualifies each", and the threat
table is expanded, because residual risk is the part a protocol engineer needs.

Both modes render all twelve properties, all four claims, all thirteen rows and
all seven boundaries; the difference is which is open on arrival.

### Interaction summary

Selecting a link updates the panel and the hash (`#guarantees/<n>`), and
highlights the threat rows whose control sits at that boundary. A "collapse"
control re-stacks the six links into one bar to show the overclaim; the panel
then reads the overclaim caption and nothing else. Full keyboard model and
screen-reader behaviour in `CENTREPIECE.md`.

---

## 7. The DSL (`#dsl`)

### What the reader sees

Heading, the two profiles side by side, then three source panels in sequence
(the swap action, the policy block, the declaration set) and the developer
workflow as a vertical list with the two rules set apart. The workspace's three
columns are described in a compact three-cell row.

### Heading

Build:

> **What you write.**

Verify:

> **Two syntax profiles, one Core.**

### The two profiles

Two cells, equal width, hairline above each:

- **`moriarty-bounded-atomic/1`**, the implemented profile. Grammar, parser,
  type checker, canonical encoding, local evaluator and restricted Compact
  lowering. The loan and swap examples run under it.
- **`moriarty-successor-syntax/0`**, the provisional successor profile.
  Separate lexical rules, EBNF grammar, bounded parser, canonical formatter
  and a read-only CLI (`check-syntax`, `format`).

Caption: "They are not interchangeable."

### The swap source

The atomic-profile `swap` action verbatim from §5.2, with the guards' named
failure strings highlighted. Right of the source (below it on mobile) a
worked-numbers card in mono:

```
reserve_a   1,000,000 A
reserve_b   2,000,000 B
fee         997 / 1000
amount_in      10,000 A
output         19,743 B
```

and the sentence "Ask for 19,744 and you get `minimum output not met`, a named
slippage failure, not a silent adjustment."

### The policy block

The `accrued_interest` policy verbatim from §5.3, with each of its six clauses
(`unit`, `derivation`, `rounding`, `remainder`, `comparison`, `proof`)
annotated in the margin on desktop: "its unit", "its derivation", "its rounding
direction", "what happens to the remainder", "how it is compared", "the proof
obligation it discharges". Lead sentence: "Rounding is a declared, named,
provable artifact, not an accident of integer division."

### The declaration set

A wrapped row of mono chips: `unit` `party` `const` `state` `observation`
`settlement` `status` `policy` `reserve` `effect` `action`; a second row
labelled "inside an action": `guard` `let` `set` `emit`; a third: `lifetime`
`horizon`, captioned "explicit bounds on the agreement itself".

### The workflow

Nine steps as a vertical list, with "Inspect events and simulate" and "Analyze
named properties" drawn as a bracketed pair. The two rules follow, each in its
own bordered block in display weight:

> Simulation and static analysis are distinct. A successful example does not
> turn the property list green.

> A valid proof does not make a stale predecessor live.

The workspace row: AGREEMENT · BEHAVIOR · TRANSACTION, each cell listing its
contents from §5.4.

### Mode differences

Build: source first, profiles second, workflow last. Verify: profiles first,
then the policy block (as the concrete form of "financial semantics above
Compact"), then the swap, then the workflow. The second rule ("a valid proof
does not make a stale predecessor live") links back to the chain's fifth
obligation in both modes.

---

## 8. Intents, multichain and safe DeFi (`#intents`)

### What the reader sees

Three sub-sections under one heading, separated by hairlines: the five
artifacts, the multichain position, and the four mechanisms.

### Heading

> **Intent, permission, plan, execution, receipt: five artifacts most systems
> conflate.**

### The five artifacts

A row of five cells in the fixed order Intent · Permission · Plan · Execution ·
Receipt. Three of them expand with their IR definition from §6 (IntentIR,
PlanIR, Receipt). Below, the two signing profiles as two columns
(Exact-plan signing, the working restricted profile; Outcome-intent signing,
with "there is no global best-price claim" set in display weight).

Then the authority rules as a list of seven statements, verbatim, each led by
its display-weight phrase: Authority is affine · Gross, not net · Every
recipient must be on the permitted list · Goals compare net · Asset identity is
the entire `{domain, issuer, reference, kind}` tuple · Validity is
`notBefore <= now < expiresAt` · Pending progress has its own judgment. Plus
the anti-vacuity rule and the wallet-rendering sentence.

### Multichain

Lead paragraph, the contradiction register's three findings as three short
items, then Moriarty's three rules numbered, then the challenge set in a
bordered block:

> Construct a cross-chain swap trace in which the internal batch succeeds but
> the bridge withdrawal fails or is rolled back. Any model that cannot
> represent that trace, and reject it, is not modelling cross-chain
> settlement.

Closing line: "Operator, relay, treasury and bridge attestations cannot
discharge a proof obligation. Bridges split into message-verified and
custodial trust; those are different security stories."

### Safe DeFi

Four cells in a 2×2 grid, each with its mechanism name in display weight and
the failure class it kills: Typed asset identity and units · Declared rounding
and remainder policy · Affine authority with gross accounting · History
compliance as a proof obligation. Under the grid, the adversarial-modelling
requirement and the two rules that follow ("ABI shape does not establish
semantic compatibility", "a valid oracle signature does not establish economic
truth"), and the sentence "Flash borrowing is a capability with legitimate
uses, not a vulnerability."

### Interaction

The fourth mechanism cell, History compliance, links to the chain's fourth
obligation (`#guarantees/4`); the first, Typed asset identity, links to the
"Token or unit confusion" threat row. These are the two places the synthesis
section hands back to the centrepiece.

### Mode differences

Build leads with Safe DeFi (the four mechanisms), then intents, then multichain.
Verify leads with the artifacts and authority rules, then multichain, then the
mechanisms. The multichain challenge block is identical in both.

---

## 9. Roadmap (`#roadmap`)

### What the reader sees

The one consolidated status section. A heading, the three parallel tracks as
three short arrow lines, the twelve sprints as a table, and the milestones
reached as a plain list. No progress bars, no percentages, no checkmarks.

### Heading

> **Twelve sprints, three tracks running now.**

### The tracks

Three lines in mono:

```
SP01 → SP02 → SP03                                     language
SP01 F0 → SP04 → SP06                                  native feasibility and proofs
accepted atomic + loan/swap subset → SP05              financial Preview integration
```

### The sprints table

Three columns: Sprint · Deliverable · Decisive completion evidence. Twelve rows,
verbatim from §9. Below 720px each row becomes a card with the three labelled
fields.

### Milestones reached

A list headed "Reached", the four items from §9 in full, with the counts in
mono: all `16` frozen cases; `15` guards and `18` presentation steps for
Transfer-only; `28` guards and `37` steps for repayment; the successor profile's
five tools; the hello-world Compact deployment on a local Midnight network with
exact state readback and indexed blocks below the finalized head.

### Mode differences

None in content. Build shows the milestones list first; Verify shows the table
first.

---

## Footer

One line: "Moriarty", the repository path, and the three-word statement of the
site's evidentiary rule from CONTENT-SPEC §10: "Every number on this site is
real." No version string, no build stamp.

---

## Completeness checklist for this storyboard

- Nine sections, real anchors, both modes.
- 7 families, 8 facets, 24 action targets (DA12 twice, marked shared), 5
  operators, 4 proof claims, 12 properties, 13 threat rows, 7 audit
  boundaries, 12 sprints.
- Every number: 277, 18, 32, 72, 12, 1,830, 1,645, 185, 182, 3, 2.10%,
  10.79%, 5.14×, 1,000,000, 2,000,000, 997/1000, 10,000, 19,743, 19,744, 15,
  18, 28, 37, 16. Each traces to CONTENT-SPEC or CATEGORY-TABS.
- No checkmarks, no green states, no "audited" or "proven" badges.
- Note for the content editor: CONTENT-SPEC §4.3 names thirteen threat rows and
  reproduces nine. The site needs all thirteen (PLAN.md, Completeness). The four
  remaining rows (Invalid initial state, Backend version skew, Resource
  exhaustion, Governance capture) are taken from `wiki/security.md`, the file
  the spec cites, and should be added to CONTENT-SPEC before implementation so
  that the spec stays the single source.
