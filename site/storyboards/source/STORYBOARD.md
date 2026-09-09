# Storyboard: the source

Proposal 4 of 5. The DSL is the hero. Real `.mori` source is the primary visual
element of the site, and every guarantee is introduced by the line that states
it. The centrepiece, Marginalia, is specified in `CENTREPIECE.md` and built in
`prototype.html`; this document is the whole page around it.

Design read: a developer-tool and formal-methods site for a trust-first
technical audience. Restrained neutrals, one accent (verdigris) for the
language's promises and one failure colour for named rejections, system
typography, a monospace face for every number and every line of source. Motion
only where it explains. Financial precision is the brand: exact integers,
named rounding, named failures.

## Global frame

**Header (64px, sticky).** Wordmark "Moriarty" left. Centre: the nine section
anchors as a single-line nav that collapses to a menu under 900px. Right: the
mode switch, a two-segment control reading **Build | Verify**, and a theme
button. Mode is one piece of state at the app root, persisted in
`localStorage`, default Build. Switching mode changes copy, ordering inside
sections and which depth of a shared visualization is expanded; it never forks
the page or scrolls the reader.

**Section anchors.** Real ids: `#opening`, `#model`, `#categories`,
`#composition`, `#formalization`, `#guarantees`, `#language`, `#intents`,
`#roadmap`. Scroll sync highlights the current anchor in the nav. Deep links
into the source reader use `#language:swap:80` style fragments.

**Typography and rhythm.** Section headline 32 to 44px, weight 600, tight
tracking. Body 15 to 17px, max 65ch. Source is 13px monospace with 1.75 line
height, the same face and size in every section so the reader learns one
texture for "this is real". Section padding 96px desktop, 56px mobile. No
cards where a hairline and space will do; tables get one bottom rule per row
group, never a box per row.

**Source blocks everywhere.** Every section that quotes a `.mori` line uses the
same component as the centrepiece, with the same highlighter, the file path
under it and the real line numbers. Clicking a line reference anywhere on the
page opens the centrepiece at that file and line. This is the connective tissue
of the proposal: the category tabs, the guarantees and the safety section all
point back into the reader.

**What never appears.** Checkmarks, "audited", "proven", green states not
supported by the repository. Logos, testimonials, TVL, benchmarks. Category
identifiers styled as code.

---

## 1. Opening  `#opening`

**On screen.** Left column (58%): the headline, one paragraph, two links. Right
column (42%): the partial-payment file, all fifteen lines, rendered by the
centrepiece component in a compact mode with line 13 already pinned and its
margin note open. The file is the first thing above the fold that is not a
headline. Under 900px the file drops below the paragraph.

**Build headline.**

> Financial meaning that survives compilation, proof and settlement.

**Build paragraph.**

> Moriarty is a language for bounded financial contracts on Midnight. You write
> the state, the permitted actions, the payment obligations and who may
> authorize what. It compiles to Compact, and every transaction carries a proof
> that its execution, and the history it extends, satisfies the agreement.

**Verify headline.**

> A bounded financial-agreement language with an operational semantics, a
> proof obligation per transition, and a proof obligation per history.

**Verify paragraph.**

> Lexical rules, an ISO/IEC 14977 grammar, typing and scoping judgments, an
> executable operational semantics in the K Framework, and explicit correctness
> claims over those semantics. Not a Marlowe dialect and not a Compact dialect.

**The pinned note on line 13 (Build).**

> Paying interest must not touch principal, and the file says so where the
> reader can see it.

**The pinned note on line 13 (Verify).**

> `postcondition = "ensures", expression, ";"`. Parsed and formatted under the
> successor profile; discharging it is an obligation on the later typed
> semantics and K.

**Ancestry strip.** Beneath the two columns, three short entries in one row,
each a name and one sentence. No logos.

- **ACTUS.** Reference behavior for events, state transitions and cash flows.
  Moriarty must reproduce it including dates and rounding: 277 fixtures, 18
  executable types, 32 source-backed taxonomy dispositions.
- **The DeFi corpus.** 72 protocol rows across the historical twelve-category
  set. Implementation and conformance requirements, not a runtime service.
- **Marlowe.** A financial-contract DSL built so behavior can be analyzed before
  execution. Moriarty adopts the goal and builds its own authoring, proof and
  settlement architecture for Midnight.

**Interaction.** The compact reader is live: hover other lines, switch mode,
and the note swaps. The two links are "Read the language" (to `#language`) and
"See the guarantees" (to `#guarantees`); in Verify mode the second becomes
"Read the specification stack" (to `#formalization`). One intent per link,
nowhere duplicated.

**Scroll.** No pinning. The file is static; the only motion is the margin card
following the pointer, and none under reduced motion.

---

## 2. The category model  `#model`

**On screen.** A stacked header, then a two-part figure: the seven families as
a vertical list on the left and the eight facets as a horizontal list along the
top, forming an empty grid that the reader fills by hovering. It is a grid,
not a chart: every cell is text. The figure is inline SVG with real `<text>`
so it is selectable and readable, sized to the column and stacking to a single
list under 700px.

**Headline (both modes).**

> Two levels: what kind of financial thing it is, and the eight properties
> every instance has.

**Build paragraph.**

> Product labels are not categories. A vault can be a debt position, a custody
> container or a strategy adapter. The same protocol carries several facets and
> can sit in more than one family.

**Verify paragraph.**

> Families are the human-facing partition; facets are orthogonal coordinates
> every instance carries. The identifiers organize packages and evidence. They
> are not Moriarty source syntax; the language has no `F2` keyword.

**Families** (always seven, in this order): F1 Exchange and price discovery,
F2 Credit and collateralized debt, F3 Derivatives, F4 Consensus-position
claims, F5 Tokenized off-chain claims, F6 Delegated asset management,
P Prediction markets.

**Facets** (always eight, in this order): Execution, Settlement, Custody, Legal
dependence, Collateral and solvency, Oracles, Authorization and mandate, Price
discovery.

**Interaction.** Hovering a family row fills its eight cells with the facet
profile from CATEGORY-TABS. Hovering a facet column highlights the two
placements the site must call out: **Intents** appears under Execution with the
sentence "An intent is a way of authorizing an action, not a kind of financial
product", and **Bridges** appears under Settlement split into two rows,
"message-verified" and "custodial", with the sentence "Different trust
assumptions. Collapsing them is a category error." Clicking a family row jumps
to its tab in section 3.

**Mode difference.** Build leads with the paragraph about product labels and
puts the Intents and Bridges callouts first in the facet hover. Verify leads
with the orthogonality sentence and adds a line under the figure: "Every
instance carries values on all eight, including where the honest value is none
or not applicable."

---

## 3. The category tabs  `#categories`

The section that carries the weight. Eight tabs, six blocks each, all 24
action targets, DA12 in two tabs and marked shared. Completeness is an
acceptance criterion; the checks at the end of CATEGORY-TABS.md are the test.

**On screen.** A full-width tab strip that becomes sticky under the header
while the section is in view, so the reader can move between families without
scrolling back up. Tab labels are the family name with the identifier as a
small classification badge (a label, never styled as code): "Exchange and
price discovery · F1". On mobile the strip scrolls horizontally with the active
tab kept in view. Below the strip, the six blocks in a fixed order with a
left-hand block index (What it is · Facet profile · Action targets · What goes
wrong · What Moriarty does · Sources) that highlights on scroll and jumps on
click.

**Headline (Build).**

> Eight tabs. Every DeFi category, the same six questions.

**Headline (Verify).**

> The conformance surface: seven families plus the cross-cutting targets that
> make them composable.

**The six blocks, and how this proposal renders each.**

1. **What this category is.** Two short paragraphs: Function, then Boundary,
   verbatim in substance from CATEGORY-TABS.
2. **Facet profile.** The eight facets as a two-column definition list; all
   eight always present, including "none" and "not applicable". The defining
   facet for the family (Collateral and solvency for F2, Custody for F5,
   Oracles for P) is set in the accent.
3. **Action targets.** One row per target: identifier as a badge, the action
   name, the semantic requirement in secondary ink, and the **distinguishing
   test** in full ink and heavier weight. This is the emphasized element, as
   the brief requires. Where a real line of source demonstrates the test, a
   line reference sits at the end of the row and opens the centrepiece at that
   line.
4. **What goes wrong.** One paragraph stated as a mechanism.
5. **What Moriarty does about it.** Prose plus a source block wherever real
   `.mori` exists. This proposal puts more weight here than the others: the
   tabs are where the source reader pays off, because each family's answer is
   a line the reader has already seen.
6. **Reference sources.** A short list, no links invented.

**Tab by tab: the source this proposal shows in block 5.**

- **F1 Exchange and price discovery** (DA01, DA02, DA03). The full
  `policy swap_output` block from `swap.mori` (lines 27 to 34) and the two
  guards on lines 80 and 81. The numbers: reserves 1,000,000 A and 2,000,000 B,
  997/1000 fee, 10,000 A in, 19,743 B out; requesting 19,744 produces a named
  slippage failure. A "Run it" link opens the centrepiece at `swap:80` with
  the run panel focused.
- **F2 Credit and collateralized debt** (DA04 to DA10, seven targets, the
  largest tab). The `payInterest` action from the partial payment (lines 9 to
  14), then the `Transfer` and `Repay` emissions from the funded file (lines 9
  to 16), then the closure guard from `loan.mori` line 108. Prose names the
  three allocation rules AccrualFirst, PrincipalFirst and ProRata and the
  kernel's rejections: `DUST` for a positive nominal converting to zero
  settlement; converted settlement, not nominal quantity, checked against the
  transfer's funding; ProRata checks `nominal × principal` fits UInt128 before
  dividing.
- **F3 Derivatives** (DA11, DA12 shared with P). `lifetime 8;` and
  `horizon 2000000000;` from `swap.mori` lines 2 and 3 as the demonstration
  that bounds are declared on the agreement, plus the sentence that time and
  duration are distinct types from amounts so an exercise date cannot be
  compared against a quantity. The tab is short and says why: DA11 has no
  repository source yet, and the site does not invent one.
- **F4 Consensus-position claims** (DA14, DA15). No `.mori` exists for staking;
  the block shows the DA18 pending-lifecycle language from the spec in prose
  (Pending, Claimable, Claimed as distinct states with a residual amount) and
  states that DA15 carries an identified primary-lifecycle source gap.
- **F5 Tokenized off-chain claims** (DA16). The manifest fields for external
  capabilities as a definition list, the typed-observation field list (source,
  feed, unit, timestamp, freshness, sequence, bounds, fallback), and the
  residual risk beside it: the source can still lie. "External behavior cannot
  inherit Moriarty guarantees" as the block's last line.
- **F6 Delegated asset management** (DA17, DA18, DA19). The loan policy block's
  `rounding floor(...)` and `remainder "..."` lines as the demonstration that
  method-specific rounding is declared, with the note that ERC-4626 has four
  conversion methods and the correct direction differs between them.
- **P Prediction markets** (DA13, DA12 shared). The DA24 conservation sentence,
  "split partitions work and claims; join cannot duplicate resource", set as a
  displayed statement, and the security position "a valid oracle signature does
  not establish economic truth". Conditional-token split/merge is presented as
  architecture, not a shipped feature.
- **⊥ Cross-cutting** (DA20 to DA24). The five composition operators as a row of
  five: sequence, parallel, interleave, synchronize, message. The 5.14× result
  restated in one line with its numbers (1,830 pairs, 1,645 compose, 185 fail,
  182 cross-category, 2.10% versus 10.79%). Governance (DA22) and messaging
  (DA23) each get their paragraph from CATEGORY-TABS; the cross-chain material
  from CONTENT-SPEC §7 lives here, including the bridge-rollback challenge.

**Interaction.** Tabs are keyboard tabs (`role="tablist"`, arrow keys). Target
rows expand on click to show the semantic requirement in full where it was
truncated. Every line reference opens the centrepiece in place (the reader is
scrolled to `#language` with the file and line pinned, and a "Back to F2" link
appears in the reader's toolbar so the round trip is one click each way).

**Mode difference.** Build orders the blocks as listed. Verify moves Action
targets to the top of every tab, retitles the block "Conformance targets",
adds the `orthogonal` tag where the source matrix has it, and shows the facet
profile last. In Verify the F2 tab also names the K result: 16 frozen cases
matching independent expectations including exact rejection code and index;
Transfer-only with 15 guards and 18 presentation steps; repayment with 28
guards and 37 steps.

**Completeness checks rendered on the page.** A quiet footer under the tab
panel reads "24 action targets · DA12 shown in F3 and P · 8 facets in every
profile · 5 operators". Counts are computed from the data module at render
time, not typed, so the sentence cannot drift from the data.

---

## 4. Composition  `#composition`

**On screen.** One figure and one paragraph. The figure is a single horizontal
bar of 1,830 units drawn as inline SVG: 1,645 in the neutral, 185 in the
failure colour, and the 185 split into 182 cross-category and 3
within-category by a hairline. Under it, two rates set as large tabular
numbers: 2.10% and 10.79%, with 5.14× between them. Nothing animates; the bar
is a fact, not a race.

**Headline (Build).**

> Composition across categories is where DeFi breaks.

**Headline (Verify).**

> 182 of 185 composition failures are cross-category.

**Paragraph (both).**

> Over 1,830 eligible protocol pairs, 1,645 compose cleanly and 185 fail.
> Within a category the failure rate is 2.10%; across categories it is 10.79%,
> a 5.14× difference. That is the empirical argument for a type system and an
> operational semantics at the boundary between financial categories. An
> earlier informal "sixty times" figure was checked and is wrong.

**The five operators.** A row of five labelled cells: sequence, parallel,
interleave, synchronize, message. Each carries one line: "own authority rules,
duty propagation, conflict semantics and fan-in". Verify mode adds the DA24
distinguishing test under the row as a displayed statement.

---

## 5. The formalization model  `#formalization`

**On screen.** Three stacked figures, each a table or a list, none a diagram
for its own sake.

**Headline (Build).**

> What a `.mori` file is, in five layers.

**Headline (Verify).**

> Five specification layers, one pipeline, one Core.

**Figure A: the five layers.** The table from CONTENT-SPEC §3, three columns
(layer, method, what it fixes). Build mode shows only the first and third
columns by default with a "show methods" toggle; Verify shows all three and
adds the three "why each choice" notes beneath: EBNF over ABNF; K
configurations and rewrite rules with typing judgments defining admissible
programs; the control layer presented in Felleisen-Hieb reduction semantics.

**Figure B: the compilation pipeline.** Seven stages as a vertical list, each
a monospace label with one sentence, from `Moriarty source (.mori)` to
`Midnight ledger and wallet`. The deferral decision is drawn as a bypass arrow
from source straight to ZKIR 3, crossed out, with the reason beside it: ZKIR is
a typed straight-line circuit IR with guarded impacts and no source-level
financial concepts, ledger-coupled and evolving; generating Compact preserves a
reviewable backend artifact and reuses the supported compiler, source maps,
runtime bindings and ledger operations. Shown as a decision with its reason,
as the brief asks.

**Figure C: the Midnight realization.** The one architectural idea that
deserves a visualization: a long-lived agreement as a sequence of bounded
one-transition proofs. Drawn as a horizontal chain of small boxes labelled
`revision 0`, `revision 1`, … with `remaining` counting down from `lifetime`.
The prototype's swap file is the data: eight boxes for `lifetime 8`, the last
one labelled `close` because of `reserve swap for close;`. Hovering a box shows
the sealed fields (Core hash, version, parties and capabilities, token policy,
deadlines, resource limits) versus the mutable public state (phase, sequence
number, commitments, continuation roots). Under it, four short statements:
witness callbacks are unverified TypeScript, so every witness result is
constrained; `disclose()` is generated only from an explicit source visibility
transition; timeouts are permissionless exported transitions guarded by
block-time predicates and do not execute autonomously; every datum is
classified public, private, committed or revealed.

**Core versus surface.** A two-column list. Core retains: finite continuations,
explicit actions and waits, accounting, value conservation, explicit timeouts,
typed warnings and errors, a decreasing structural measure, visibility and
trust information. Surface adds: modules, named definitions, schedules,
token-indexed amounts, durations, records, packages, bounded compile-time
loops and folds, templates. One line under it: all of it must elaborate away
into Core. The manifest's recorded fields as a compact definition list.

**Mode difference.** Build reads A, B, C in that order with the pipeline
compressed to labels. Verify reads A with all notes, then C, then B, and
expands the manifest list.

---

## 6. The guarantees  `#guarantees`

**On screen.** The layered assurance vocabulary as a vertical chain of six
labelled rungs, then the property inventory, then the four mandatory claims,
then the threat table.

**Headline (Build).**

> What breaks without each layer.

**Headline (Verify).**

> Six obligations. Establishing one does not establish the next.

**The chain.** Six rungs: semantic validity → Moriarty-to-Compact translation
validity → Compact-to-ZKIR compiler correctness → circuit and proof
correctness → Midnight ledger feasibility → transaction construction and wallet
correctness. Each rung has a one-line Build gloss stating the failure it rules
out and a one-line Verify gloss stating what it does not establish about the
next rung. The chain is inline SVG with real text and no animation; the rungs
are visually separated so that the collapse the spec calls "the standard
marketing lie" is impossible to read into the figure.

**The property inventory.** Twelve rows, three columns, no collapsing:
property, the assumption that qualifies it, the Moriarty obligation. Rendered
as a table in its own horizontal-scroll container on mobile. Build mode shows
the property and obligation columns and reveals the assumption on tap; Verify
shows all three. The Marlowe/Isabelle lineage is credited with its pinned
commit `7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`.

**The four mandatory proof claims.** Four numbered statements in fixed order:
ContractInvariant, IntentRefinement, TransitionValidity, HistoryCompliance.
Under the fourth, the sentence that distinguishes the project: a valid proof of
a valid transition against an invalid history must not be accepted. In Build
mode this is phrased as the bug it kills: "each step looks locally fine".

**Security and trust boundaries.** The threat table with all thirteen rows,
three columns: threat, required control, residual risk. Residual risk is set in
full ink; the other two columns in secondary ink, because the residual column
is the point. Under the table, the audit-scope statement: a single "Moriarty
audit" is not an adequate claim, and the seven audit surfaces it splits into.

**Source tie-in.** Three rows carry a line reference into the reader: Token or
unit confusion → `swap.mori` line 14 (unit-indexed reserves); Authorization
replay → the `remaining` guard on line 67 and the horizon guard on line 66;
Compiler mistranslation → the independent value checks on `swap.mori` line 79
and `loan.mori` line 80.

---

## 7. The DSL  `#language`

**This is the centrepiece section.** The full Marginalia reader as specified in
`CENTREPIECE.md` and built in `prototype.html`, with the section's copy above
it and the declaration set and workflow below it.

**Headline (Build).**

> The guarantee is written in the source.

**Headline (Verify).**

> Two syntax profiles, one grammar family, and what each one establishes.

**Paragraph (Build).**

> Four real `.mori` files from the repository. Read a line and the margin tells
> you which bug it makes unwritable.

**Paragraph (Verify).**

> The same four files. The margin names the grammar production, the profile
> that admits it, and the obligation the semantics still owe.

**The reader.** Five tabs (partial-payment, funded-partial-payment, swap, loan,
profile diff), six lenses, the margin, the run panel on the swap file. The
reader arrives with the partial payment open and nothing pinned; if the reader
came here from a line reference elsewhere on the page, that file and line are
pinned and a "Back to …" link is in the toolbar.

**Two profiles, stated plainly.** Above the reader, a two-line strip:

> `moriarty-bounded-atomic/1`, the implemented profile: grammar, parser, type
> checker, canonical encoding, local evaluator and restricted Compact lowering.
> The loan and swap examples run under it.
>
> `moriarty-successor-syntax/0`, the provisional successor profile: lexical
> rules, EBNF grammar, bounded parser, canonical formatter and a read-only CLI
> (`check-syntax`, `format`). They are not interchangeable.

**The declaration set.** Below the reader, the eleven top-level declarations
as a row of monospace chips: `unit`, `party`, `const`, `state`, `observation`,
`settlement`, `status`, `policy`, `reserve`, `effect`, `action`; a second row
for the four action statements `guard`, `let`, `set`, `emit`; and a third for
the two bounds `lifetime` and `horizon`. Hovering a chip highlights every line
in the open file that uses it, which is the same dimming mechanism as a lens.
Chips for constructs the successor profile lacks (`policy`, `observation`,
`settlement`, `status`, `reserve`) show a small "atomic only" label when the
open file is a successor file, so the difference between profiles is visible
in the declaration set itself.

**The policy block, called out.** One paragraph beside the chips:

> Every policy names its unit, its derivation, its rounding direction, what
> happens to the remainder, how it is compared, and the proof obligation it
> discharges. Rounding is a declared, named, provable artifact rather than an
> accident of integer division.

**The developer workflow.** The nine steps from CONTENT-SPEC §5.4 as a
vertical list with the two parallel steps (inspect and simulate; analyze named
properties) drawn side by side. Two displayed rules beneath it:

> Simulation and static analysis are distinct. A successful example does not
> turn the property list green.
>
> A valid proof does not make a stale predecessor live.

**The workspace.** Three labelled columns in a single row, AGREEMENT ·
BEHAVIOR · TRANSACTION, each listing its contents from the spec. Text only; no
fake screenshot.

**Mode difference.** The reader's notes, status sentences and diff paragraphs
swap wholesale. Build opens on the partial payment; Verify opens on the profile
diff, because the researcher's first question is what the two grammars admit.
Verify also shows the successor profile's bounds beneath the two-profile strip:
65,536 source bytes, 64-character identifiers, 78-digit integers, 8,192 tokens,
nesting depth 64, and the stable error-code list.

**Scroll.** The reader is not pinned. It is tall enough that the reader scrolls
inside the page normally; the file list on desktop is `position: sticky` within
the reader so it stays in reach while the swap action scrolls.

---

## 8. Intents · multichain · safe DeFi  `#intents`

**On screen.** Three sub-sections in one anchor, separated by hairlines. The
first is a five-part figure, the second is a challenge statement, the third is
four numbered mechanisms each pointing at a line in the reader.

**Headline (Build).**

> What you sign, what a solver may do with it, and what the wallet shows you.

**Headline (Verify).**

> Five artifacts, two signing profiles, affine gross authority.

**The five artifacts.** Intent · Permission · Plan · Execution · Receipt as five
labelled cells in one row, with IntentIR, PlanIR and Receipt expanded beneath
as definition lists (principal + scope + authority + requirements + assumptions
+ lifecycle + validity/replay + semantic versions; intent identity + bounded
steps + adapter identities + dependencies; intent/plan identities + checked
effects + status + residual resources).

**Two signing profiles.** Two short paragraphs: exact-plan signing (the working
restricted profile) and outcome-intent signing (authority, goals, permitted
programs, validity and nonce fixed before a plan exists; solvers search outside
the finite checker; ranking cannot excuse invalid authority; no global
best-price claim).

**The authority rules.** Eight displayed statements, each one line, in the
spec's order: authority is affine; gross, not net, and a refund never restores
allowance; every recipient enumerated, including refunds; goals compare net
final-minus-initial credits and fees count against them; asset identity is the
entire `{domain, issuer, reference, kind}` tuple; validity is
`notBefore <= now < expiresAt` with a nonce and a domain separator; pending
progress has its own judgment; anti-vacuity requires trace inclusion and a
feasible positive witness. The wallet sentence closes: a deterministic semantic
signing summary, not a hex blob.

**Multichain.** The honest position first, as three quoted contradictions from
the register (atomic-with-refunds versus asynchronous external calls; "non-
custodial" versus contract-held balances and a proof-of-authority bridge;
confidentiality labels without normative leakage definitions). Then the three
rules: atomicity per layer and per route; a route-specific custody and
authority manifest before approval; confidentiality as a named adapter
profile. Then the challenge, set as a displayed statement:

> Construct a cross-chain swap trace in which the internal batch succeeds but
> the bridge withdrawal fails or is rolled back. A model that cannot represent
> that trace, and reject it, is not modelling cross-chain settlement.

Bridges split into message-verified and custodial trust, and the two are never
merged on the page.

**Safe DeFi: four mechanisms, each with a line.** Numbered one to four, each
with its sentence from CONTENT-SPEC §8 and a line reference into the reader:

1. Typed asset identity and units → `partial-payment.mori` line 7 (`Debt<USD>`)
   and `swap.mori` line 14.
2. Declared rounding and remainder policy → `loan.mori` lines 28 to 35.
3. Affine authority with gross accounting → `swap.mori` line 67 (`remaining`),
   with the prose noting that outcome-intent authority is specified in the
   intents design rather than in these files.
4. History compliance as a proof obligation → the HistoryCompliance claim in
   section 6; no source line yet, and the page says so.

Closing rules, displayed: ABI shape does not establish semantic compatibility;
a valid oracle signature does not establish economic truth; flash borrowing is
a capability with legitimate uses, not a vulnerability. The adversarial fixture
shape (actor capability, vulnerable layer, precondition, ordered effects,
violated predicate, loss outcome) is listed in Verify mode only.

---

## 9. Roadmap  `#roadmap`

**On screen.** The twelve sprints as a single table (sprint, deliverable,
decisive completion evidence), all twelve rows, in a horizontal-scroll container
on mobile. Above it, the three parallel tracks drawn as three short horizontal
sequences of sprint labels. Below it, the milestones reached, as plain
statements with their numbers.

**Headline (both).**

> Twelve sprints, three tracks, and what has actually been reached.

**Tracks.** SP01 → SP02 → SP03 (language). SP01 F0 → SP04 → SP06 (native
feasibility and proofs). Accepted atomic + loan/swap subset → SP05 (financial
Preview integration).

**Milestones reached** (the only place on the site that reports status, and it
reports only these):

- The bounded K definition executes Transfer-only and Transfer-then-Repay with
  AccrualFirst, PrincipalFirst and ProRata allocation and explicit
  none/floor/ceil conversion rounding. All 16 frozen cases match independent
  financial expectations, including exact rejection code and index.
  Transfer-only carries 15 guards and 18 presentation steps; repayment carries
  28 guards across its stages and 37 steps.
- The successor syntax profile has lexical rules, EBNF grammar, bounded parser,
  canonical formatter and read-only CLI.
- A hello-world Compact deployment and call were finalized on a local Midnight
  network with exact state readback and indexed blocks below the finalized
  head.

No progress bars, no percentages, no filled tracks. A sprint is a row; a
milestone is a sentence with its numbers.

**Mode difference.** Build shows the table with the deliverable column
emphasized; Verify emphasizes the completion-evidence column and adds a line
under the milestones naming the guard counts as the K result the guarantees
section referred to.

---

## Footer

Repository path, the spec files the site traces to (`CONTENT-SPEC.md`,
`CATEGORY-TABS.md`), and the pinned Marlowe commit. Nothing else.

---

## Section layout families (repetition check)

1. Opening: split text and live reader.
2. Category model: cross-grid figure.
3. Category tabs: sticky tab strip with block index.
4. Composition: single proportional bar with two numbers.
5. Formalization: stacked tables and a chain of boxes.
6. Guarantees: vertical rung chain, then wide tables.
7. The DSL: the full reader, chips, workflow list.
8. Intents: five-cell row, displayed statements, numbered list.
9. Roadmap: track sequences and one table.

No family repeats. Eyebrows are not used; section headlines carry the topic.
Motion appears in exactly three places, all explanatory: the margin card
following a line, the lens dimming, and the cross-grid filling on hover. All
three collapse to instant under `prefers-reduced-motion`.
