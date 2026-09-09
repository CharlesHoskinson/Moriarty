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

This document records the decisions and the reasoning behind them, so that a
reader who picks it up later can tell which parts are load-bearing and argue
with them on the merits. Where a decision costs something, the cost is written
beside it.

## Framing decision

The site tells the story of the **target architecture** — proof-carrying
financial settlement as designed — rather than annotating every element with its
current maturity. Delivery status lives in **one consolidated roadmap section**
at the end (CONTENT-SPEC §9), which carries the sprints and the milestones
already reached.

This is a narrative decision, not a licence to overclaim. The rules in
CONTENT-SPEC §10 still bind: every number real, every assumption named beside
its claim, no invented verified badges.

If every element carried its maturity, every sentence would end in a status tag
and the reader would learn the delivery state of an idea before learning the
idea. A reader who wants to know what has shipped goes to the roadmap, where
each sprint carries its decisive completion evidence and the milestones reached
are stated with their own exact figures, and finds the whole answer in one
place rather than scattered across the page as caveats.

## Two modes

The site has a persistent mode switch. Both modes render the same section spine
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
  that qualifies it, and the Moriarty obligation, each in its own column and
  never collapsed.
- Felleisen–Hieb reduction semantics, the K configuration, the guard counts
  (15 / 28), the ZKIR deferral decision and its reasoning.
- The action targets are the conformance surface.

Implementation: mode is one piece of state at the app root, persisted in
`localStorage`, defaulting to Build. It changes copy, section ordering within a
section, and which depth layer of a shared visualization is expanded — it does
not fork the component tree into two apps.

### What differs between the modes, and what does not

A reader in either mode meets the same facts. The swap returns 19,743 B in
both. The partial payment preserves principal in both. The residual risk beside
the oracle row reads the same in both, and an assumption that qualifies a
property is never present in one mode and absent in the other. What the mode
changes is the door the reader comes in by, the words used at that door, and the
order in which the parts of a section are given weight.

The Build reader arrives at the failure and is walked back to the construct that
prevents it. The Verify reader arrives at the specification layer and is walked
forward to the failure that layer rules out. Inside the Fork the case index
emphasises the distinguishing test for one reader and the semantic requirement
for the other, and the `enforced by` line names a type or a guard for one and a
specification layer for the other. Inside the source reader the same line of
`.mori` carries a note for each reader, and switching mode swaps the note
without moving the pinned line. The guarantee chain swaps one paragraph in its
panel and nothing else. That is the whole extent of the difference.

### Why the site does not fork into two applications

The easy design is two applications behind one switch. It was rejected because
the facts would then live twice. Copies of the action targets drift, and the
drift is silent, because each copy is complete on its own terms and nothing
compares them. An assumption added beside a claim in the Verify copy is missing
from the Build copy, and the Build reader is the one more likely to be sold
something, so the copy that most needs the assumption is the one least likely
to carry it. The completeness test would have to run against both trees, and a
reader who switched mode in the middle of credit would land at the top of a
different page and lose the place they had.

One tree with one piece of state avoids all of that. Mode is a context at the
app root, read from `localStorage` under `moriarty.mode`, and a small helper
picks the Build or the Verify value where the copy differs. A component with
nothing mode-specific to say does not read the mode at all. Every visualization
reads the same data module whatever the mode, and the mode only chooses which
depth layer opens first and which sentence stands in the emphasised position.
If a future change makes the modes disagree about a fact, that is a bug in the
copy and not a feature of the mode.

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
8. **Intents · multichain · safe DeFi** — the artifacts the intents model keeps
   separate; affine gross authority; per-layer atomicity and the bridge-rollback
   challenge; the safety mechanisms.
9. **Roadmap** — the sprints, the tracks running in parallel, and the milestones
   already reached.

## Centrepiece — decided

Each lane receives this plan and the content spec and proposes a different
centrepiece visualization plus a full storyboard. The proposals land in
`site/storyboards/<lane>/`. One is then selected, or synthesized, before any
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

Each lane owns a distinct angle so the proposals are genuinely different rather
than variants of one idea:

| Lane | Angle |
|---|---|
| category-surface | **The category surface** — the tabs as one navigable object; how a reader moves across family, facet and action target without losing place |
| pipeline | **The pipeline** — one transaction traced from `.mori` through Core, Compact, ZKIR, proof, ledger |
| guarantee-chain | **The guarantee chain** — the layered assurance vocabulary and what each layer does and does not establish |
| source | **The source** — the DSL itself as the centrepiece; code as the primary visual element |
| failure | **The failure** — the distinguishing tests; each guarantee introduced by the bug it catches |

### What each lane was best at

The category surface proposed the Register. Family, facet and target are all on
one screen with fixed positions, and the category tabs are not a second widget
beside the surface but the surface's own columns. Its facet lens is the single
move on the site that shows the model is a classification rather than a list.
Its weakness is that it has no depth inside a target, and that its foot carried
counters.

The pipeline proposed the Trace. One value is carried across every
representation from source to ledger and the number never changes while the
type, the encoding and the guarantee change around it, which is the tagline
made testable, and its failure path lets the reader ask for 19,744 and watch
the trace stop at Core before any proof is attempted. Its weakness is that it
is one transaction deep, that the brief has no Compact or ZKIR source so those
stations show manifest fields rather than code, and that it explains the
pipeline better than the category model the site is organised around.

The guarantee chain proposed the chain with the gaps labelled. With the links
plain and the gaps carrying "does not establish", the eye reads refusals rather
than achievements, and the collapse control lets the reader produce the
overclaim themselves without the site ever rendering a badge. Its weakness as a
centrepiece is that it is about assurance and not about finance, and that its
content is a short list of names and relations, which makes a section and not a
spine.

The source proposed Marginalia. The code is verbatim with the file's own line
numbers, the margin holds an arithmetic block where the policy's free-text
remainder turns out to be exactly checkable, and the run panel evaluates the
swap in source order so a reader can type 10001 and learn that the example is a
frozen worked instance rather than a general pool. Its weakness is that only the
atomic profile runs, that its lens chips carried counts, and that a reader who
does not yet know why `Debt<USD>` matters is shown the answer before the
question.

The failure proposed the Fork. It keeps what the distinguishing tests actually
are, a pair of answers to one question, and shows both so the reader does the
comparison rather than being told the conclusion. Setting the plausible column
in grey is the correct claim: the balance-based implementation is not
dangerous-looking, it is less defined, and it has no field in which to write the
allocation or the pending state. Its weakness as a centrepiece is that on its
own it needs a tab strip and a case index, which is a second map of the
territory the Register already draws, and its tab strip carried counts too.

### What the Register does that the others cannot

Only the Register puts family, facet and target on one screen together. The
Trace runs along pipeline order and nothing else. The chain runs along
assurance order. The source reader has files and lines. The Fork has targets
grouped by tab and no facets at all. The brief says the category section carries
the weight and that the facets are what make the model a classification, and
the Register is the only proposal in which a reader can stand on a target, look
across a facet and open a family without leaving the surface. It also reads as
a ledger, with a fixed set of rows and columns and every cell filled, and that
is the brand.

### Why the Fork sits inside a target

The Register answers where a reader is. The Fork answers what a target requires.
Those are different questions and neither proposal answered the other's.
Standing alone the Fork needed its own tab strip and case index, and a site with
both would carry a second navigation of the same targets, which a reader would
have to learn twice. The distinguishing test belongs to its target, so the Fork
renders inside the target's entry in the tab body, reached by activating the
chip on the rail. Nothing else in the Fork's design changes. The grey plausible
column, the one coloured line where the columns part, the live evaluation for
DA01 and DA06 and the structural state lines for the rest all survive, and only
its navigation is removed, because the Register supplies a better one.

### What the choice gives up

The Trace's carried value is the strongest argument on the site for a protocol
engineer, and it is now in the formalization section rather than on the hero. A
reader who leaves before section 5 does not see 19,743 stay 19,743 across the
pipeline. The chain's collapse control is in section 6. The source reader is in
section 7, so the language, which Build mode calls the hero, is not the first
thing a Build reader sees. The Register is dense at first sight, and a reader
who does not care for classifications must scroll past one to reach the code.
The Fork's ability to scan every target across every tab in one index is lost,
because inside the Register the reader reaches one target at a time. These costs
were accepted because the alternative was a site whose hero explained the
pipeline or the code while its category section was a tab bar, and a tab bar
makes completeness invisible.

### Why the reading counters were cut

The Register's foot read out targets read, tabs opened and facets compared, as
exact integers, and its screen-reader region announced them when they changed.
The proposal defended them carefully: integers rather than percentages, no bar
that could be read as a verification state, marks in neutral ink, and a legend
saying the marks were the reader's own. All of that was right, and the counters
are still cut.

Prose that announces how many things it is about to describe is counting
instead of describing, and the site forbids it. An interface that reports how
much of itself the reader has covered is the same habit with the numbers moved
off the page and into the chrome. The reader learns that they have opened some
tabs and not others, which is a fact about the reader rather than about
Moriarty, and the readout invites them to finish the grid rather than to read
the target in front of them. The same cut applies to the lens chips in the
source reader, to the computed line under the Fork's tab strip, and to any
screen-reader name that ends in a position within a tab.

Coverage is still shown, and it is shown by the surface. The Register has a
fixed extent and every cell is filled, so a reader sees at once how much there
is. Column width is proportional to the targets a family owns, so credit is
visibly the largest surface before a word is read. A cell the reader has opened
takes a neutral ink fill with no number beside it, and the pinned spine carries
the same marks at the same positions, so a reader deep in credit can see where
they have been and where they have not without being told a fraction. A grid
whose cells are all filled says more than a line reporting that all of them are.

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

### The data layer

The content ships as typed modules under `site/src/data/`: the families and
facets, the action targets, the assurance layers with the properties, the proof
claims and the threats, the language's specification layers and samples, the
intent artifacts and the sprints. `index.ts` re-exports them so a component
imports one path. An action target is a record with an identifier, the families
it belongs to, its action, its semantic requirement, its distinguishing test and
its roadmap disposition, and DA12 is one record that lists both its families
rather than a record per family.

Several visualizations read the same target and cannot disagree about it. The
Register's chip, the tab body's entry, the Fork's case, the pinned spine's dot
and the roadmap's disposition all come from the same record, and a corrected
sentence is corrected everywhere at once. The type checker also sees the
content, so a target without a distinguishing test is a compile error and a
facet name misspelled in one family's profile is a compile error, where in
markup both would be a paragraph that silently reads wrong. And the content can
be tested, which is the subject of the completeness section below.

The cost is that copy lives in TypeScript, which is a worse place to write
prose than a document. The source lane asked for verbatim segments with real
line numbers and a build-time check that each segment matches the repository
file, which is the same idea applied to code samples, and that check is cheaper
to write against a module than against a page.

### Hand-authored SVG

A charting dependency assumes the content is a numeric series. The site's
content is text: guard messages, facet sentences, claim names, manifest fields.
There is no ordinal value in a facet placement to encode as colour or height,
and the category surface lane rejected a heatmap for exactly that reason. A
force-directed layout would give arbitrary positions that differed between
visits and could not be read by a screen reader, and no proposal needed one.
Hand-authored SVG driven from the data modules gives real text that is
selectable and read aloud, positions that are fixed and meaningful, and no
runtime that fetches, animates or resizes on its own terms. Where the content
is a list of selectable text, as in the chain, the same rule chooses DOM over
SVG, because CSS draws spacing better than SVG does and native buttons give tab
order for free.

The cost is that everything is drawn by hand. The Register transposes rather
than shrinks at 400px, and the transposition is written rather than configured.
There are no free tooltips, legends or axes, and every responsive case is a
decision that someone has to make and maintain.

### Motion that explains

Motion on the site has to say something about position or order. The Fork's
plausible column lands first and the required column lands a beat later, and
the order is the explanation. The facet lens draws a rule along the lifted row
from the header to the last column, and the rule draws the reading direction.
The chain enters as one bar and separates into its obligations, and the
separation is the thesis. The site does not reveal source with a typewriter,
count a number up from zero, pulse a mark when it fills or slide tabs as if they
were a carousel, because a counting-up number implies approximation and a slide
implies the tabs are a sequence when they are columns of a fixed table.

Under `prefers-reduced-motion` every duration is zero and every state is still
reached, so the meaning of every animation has to exist in text or position as
well. That is the cost, and it is more work rather than less, because a site
that explains with motion must explain the same thing without it. The other
cost is that the page looks still to a visitor used to product heroes.

### Accessibility as a requirement

The argument of the site is in its text. A diagram a screen reader cannot read
has withheld the argument from that reader, and a keyboard user who cannot
enter the grid has been given a picture of a classification rather than a
classification. So the Register is a grid with a roving tab index and arrow
keys that cross column boundaries, the Fork's columns are real definition lists
in the order a sighted reader sees them, the chain's gap labels are hidden from
assistive technology because the panel speaks the same relation in real text,
and every state carried by colour is also carried by a word. Contrast is held
in both themes, and the grey used for a line the plausible model cannot
represent is checked to 4.5:1, because less defined must still be legible.

Moriarty's discipline is that nothing is claimed beyond what is established,
and a site that claimed to explain the language while part of its audience
could not read it would be overclaiming in the way the assurance chain warns
against. The cost is real: a keyboard model for a two-axis grid is a design
task in itself, the narrow layout is a second structure rather than a scaled
one, and every coloured state needs a matching word. It is the same cost as
being honest.

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

The test exists as a test rather than as a review checklist because a checklist
is read once, by a person, at review, and the content changes after review.
`src/data/completeness.test.mjs` runs with `npm test`. It checks that the family
identifiers are exactly the fixed set in the fixed order, that every family
answers every facet, that every target from DA01 to DA24 appears once and in
order with a requirement and a distinguishing test, that the operators DA24
names are all present, that the composition figures add up and the stated ratio
agrees with the rates, and that the assurance layers, the properties, the proof
claims and the threat rows are the sets the spec fixes. A build that trims a
threat row fails. A build that rounds the ratio fails.

The guarantee chain lane found, while storyboarding, that the content spec
named its count of threat rows and listed fewer, and that the missing rows were
in `wiki/security.md`. A review had already passed that document. A test of the
data layer notices when the words and the facts part company, and it notices
every time and not once.

## Constraints

- No tooling attribution anywhere. The site, its source, its comments, its
  assets and its commit messages carry no credit to any tool or vendor that
  touched them.
- No third-party logos, no fabricated testimonials, no invented metrics.
- The fixed sets are fixed. Nothing is rounded off, extended or dropped, and the
  test at `src/data/completeness.test.mjs` enforces this against the data layer.

### Why the honesty rules matter commercially

The rules in CONTENT-SPEC §10 read as ethics and they are also the sales
argument. The project's differentiator is that it states the assumption beside
the claim. Every property carries the assumption that qualifies it, every threat
carries the residual risk that survives the control, and the layered assurance
vocabulary is drawn as a chain because collapsing it is the standard marketing
lie in this space. The contradiction register in §7 is the competing pitch:
overviews that claim atomic cross-chain execution with automatic refunds while
the underlying verifier documentation says external calls complete
asynchronously and simulation excludes them. Moriarty's answer is to define
atomicity per layer and per route and to say so before approval.

A site that overclaimed would destroy the one thing that distinguishes the
project. The audience is protocol engineers and formal-methods researchers, and
both read the assumption first. An engineer who finds one invented number stops
trusting 19,743. A researcher who finds one collapsed layer stops reading the
chain. A green badge the repository does not support would be the first thing
screenshotted and the last thing forgiven. So there is no checkmark, no audited,
no proven, no rounded ratio and no logo, and residual risk is set as the loudest
column, because on this site the admission is the pitch.

## Sequence

1. Knowledge graph over the repository — done, `graphify-out/`.
2. Plan and content spec — this document and `CONTENT-SPEC.md`.
3. Storyboard proposals, one per lane — `site/storyboards/`.
4. Selection or synthesis of the centrepiece and the storyboard.
5. Implementation of the single-page application.
