# Moriarty website — plan

Branch: `website`. Worktree: `.worktrees/website`. Output: `site/`.

## Goal

A single-page React application that explains, to two audiences at once, what
Moriarty is. It covers the DeFi category model Moriarty targets, the
formalization model it uses, the guarantees that model produces, and the DSL a
developer writes.

Substance is fixed by [`CONTENT-SPEC.md`](CONTENT-SPEC.md) and, for the category
section, [`CATEGORY-TABS.md`](CATEGORY-TABS.md). Nothing goes on the site that is
not traceable to them. Register and diction are fixed by [`VOICE.md`](VOICE.md),
which governs every line of copy and forbids the tallies.

This document records the decisions and the reasoning behind them, so that a
later reader can argue with them on the merits. Where a decision costs
something, the cost is written beside it.

## Framing decision

The site tells the story of the target architecture, proof-carrying financial
settlement as designed, rather than annotating every element with its current
maturity. Delivery status lives in one roadmap section at the end
(CONTENT-SPEC §9).

This is a narrative decision, not a licence to overclaim, and the rules in
CONTENT-SPEC §10 still bind: every number real, every assumption named beside
its claim, no invented verified badges.

If every element carried its maturity, every sentence would end in a status tag
and the reader would learn the delivery state of an idea before learning the
idea. A reader who wants to know what has shipped goes to the roadmap, where
each sprint carries its completion evidence, and finds the whole answer in one
place.

## Two modes

The site has a persistent mode switch. Both modes render the same section spine
and the same underlying data; they differ in entry path, emphasis, depth and
vocabulary. A reader can switch at any point and stay in place.

### Build mode, for protocol engineers

Build is the default. The reader knows AMMs, lending, liquidations, ERC-4626
and MEV, and does not necessarily know K, EBNF, operational semantics or ZK.

- The entry is the failure they recognize: a partial repayment that quietly
  touches principal, or a rounding direction that drains a vault.
- The DSL is the hero, and real `.mori` source appears from the first screen.
- Formal machinery is explained in their terms: "the type system will not let
  you add `Debt<USD>` to `Amount<USD>`" before "typing judgment `Γ ⊢ e : τ`".
- The action targets are the map of what you can build, and each distinguishing
  test is the bug that target catches.
- Guarantees are framed as what breaks without them.

### Verify mode, for researchers

The reader knows semantics, and DeFi is the motivating application.

- The entry is the specification stack: lexical rules, EBNF, typing judgments, K
  operational semantics, correctness claims over those semantics.
- The layered assurance vocabulary (CONTENT-SPEC §4) is the hero, with the
  chain from semantic validity through to wallet correctness drawn explicitly.
- The property inventory shows the Marlowe/Isabelle evidence, the assumption
  that qualifies it, and the Moriarty obligation, each in its own column and
  never collapsed.
- The action targets are the conformance surface.

### What differs between the modes, and what does not

A reader in either mode meets the same facts. The swap returns 19,743 B in
both, and an assumption that qualifies a property is never present in one mode
and absent in the other. The Build reader arrives at the failure and is walked
back to the construct that prevents it. The Verify reader arrives at the
specification layer and is walked forward to the failure that layer rules out.
Inside the source reader the same line of `.mori` carries a note for each
reader, and switching mode swaps the note without moving the pinned line.

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
app root, read from `localStorage` under `moriarty.mode` and defaulting to
Build. If a future change makes the modes disagree about a fact, that is a
bug in the copy and not a feature of the mode.

## Section spine (both modes)

1. Opening: what Moriarty is, the one-line claim, and the ancestry (ACTUS, the
   DeFi corpus, Marlowe).
2. The category model: the economic families and the facets each must answer.
3. The category tabs, the section that carries the weight. One tab per DeFi
   category and a final tab for the cross-cutting targets. Every tab renders
   the same blocks in the same order.
4. Composition: the measured cross-category failure result, and the operators
   DA24 names.
5. The formalization model: the specification layers, the compilation pipeline,
   Core against the surface language, and the Midnight realization.
6. The guarantees: the assurance layers, the qualified properties, the proof
   claims and the trust boundaries.
7. The DSL: profiles, real source, the `policy` block, the declaration set, the
   developer workflow.
8. Intents, multichain and safe DeFi: the intent artifacts, per-layer atomicity
   and the safety mechanisms.
9. Roadmap: the sprints, the tracks running in parallel, and the milestones
   already reached.

## Centrepiece

Each lane's proposal is in `site/storyboards/<lane>/`, and the decision is the
Register, with the Fork inside it.

The Register is the centrepiece and the navigation spine. Its columns are the
category tabs, its rows are the facets, and the action targets sit in the columns
they belong to, so family, facet and target are all reachable without leaving the
surface. A facet row header lifts that one facet across every family at once.
That is the move that shows the model is a classification.

Inside a target, the Fork renders the distinguishing test: one pre-state, one
action, two post-states. The left is what a plausible balance-based
implementation writes. The right is what the test requires. The line where they
part is the only coloured line on the page, and what the plausible model cannot
hold is set in grey rather than in red, because this is a precision argument and
not a scare page.

The guarantee chain takes the guarantees section, with residual risk set as the
loudest column, the pipeline trace takes the formalization section and the
annotated source reader takes the language section. The Register's reading
counters are cut, for reasons given below.

### What each lane was best at

The lanes were `category-surface`, `pipeline`, `guarantee-chain`, `source` and
`failure`.

The category surface proposed the Register. Family, facet and target are all on
one screen with fixed positions, and the category tabs are the surface's own
columns, with no second widget beside it. It has no depth inside a target, and
its foot carried counters.

The pipeline proposed the Trace, which carries one value across every
representation from source to ledger while the type, the encoding and the
guarantee change around it and the number never does, and whose failure path
lets the reader ask for 19,744 and watch the trace stop at Core before any
proof is attempted. It is one transaction deep, and it explains the pipeline
better than the category model the site is organised around.

The guarantee chain's proposal draws the chain with its gaps labelled, so the
eye reads refusals rather than achievements. As a centrepiece it is about
assurance, not finance, and its content is a short list of names and relations.

Marginalia came from the source lane, with the code verbatim under the file's
own line numbers. Only the atomic profile runs, its lens chips carried counts,
and a reader who does not yet know why `Debt<USD>` matters is shown the answer
before the question.

A distinguishing test is a pair of answers to one question, and the Fork, the
failure lane's proposal, keeps that shape and shows both answers so the reader
does the comparison. Its grey column is the correct claim, because the
balance-based implementation is not dangerous-looking, it is less defined, and
it has no field in which to write the allocation or the pending state. Its tab
strip carried counts too.

### What the Register does that the others cannot

Only the Register puts family, facet and target on one screen together. The
Trace runs along pipeline order and nothing else, the chain runs along
assurance order, the source reader has files and lines, and the Fork has
targets grouped by tab and no facets at all. The category section carries the
weight, and the facets are what make the model a classification.

### Why the Fork sits inside a target

The Register answers where a reader is. The Fork answers what a target
requires. The Fork alone needed its own tab strip and case index, and a site
with both would carry a second navigation of the same targets, which a reader
would have to learn twice. The distinguishing test belongs to its
target, so the Fork renders inside the target's entry in the tab body. The grey
plausible column, the one coloured line where the columns part, the live
evaluation for DA01 and DA06 and the structural state lines for the rest all
survive, and only its navigation is removed, because the Register supplies a
better one.

### What the choice gives up

The Trace's carried value is the strongest argument on the site for a protocol
engineer, and it is now in the formalization section rather than on the hero. A
reader who leaves before section 5 does not see 19,743 stay 19,743 across the
pipeline. The chain's collapse control is in section 6, and the source reader
is in section 7, so the language, which Build mode calls the hero, is not the
first thing a Build reader sees. The Register is dense at first sight, and a
reader who does not care for classifications must scroll past one to reach the
code. The Fork's ability to scan every target across every tab in one index is
lost, because inside the Register the reader reaches one target at a time.
These costs were accepted because the alternative was a site whose hero
explained the pipeline or the code while its category section was a tab bar,
and a tab bar makes completeness invisible.

### Why the reading counters were cut

The Register's foot read out targets read, tabs opened and facets compared, as
exact integers. The proposal defended them carefully: integers rather than
percentages, no bar that could be read as a verification state, marks in
neutral ink, and a legend saying the marks were the reader's own. All of that
was right, and the counters are still cut.

Prose that announces how many things it is about to describe is counting
instead of describing, and the site forbids it. An interface that reports how
much of itself the reader has covered is the same habit moved into the chrome.
The reader learns that they have opened some tabs and not others, which is a
fact about the reader rather than about Moriarty, and the readout invites them
to finish the grid rather than to read the target in front of them. The same
cut applies to the lens chips in the source reader, to the computed line under
the Fork's tab strip, and to any screen-reader name that ends in a position
within a tab.

Coverage is shown by the surface. The Register has a
fixed extent and every cell is filled. Column width is proportional to the
targets a family owns, so credit is visibly the largest surface before a word
is read. A cell the reader has opened takes a neutral ink fill with no number
beside it, and the pinned spine carries the same marks at the same positions.
A reader deep in credit can see where they have been and where they have not
without being told a fraction.

## Technical shape

- Single-page React application, built with Vite, React 19 and TypeScript.
- No backend. Routing is in-page section navigation with real anchors and
  scroll sync.
- Responsive down to 400px. Dense tables get their own horizontal scroll
  container, and the page body never scrolls sideways.

### The data layer

The content ships as typed modules under `site/src/data/`, and `index.ts`
re-exports them so a component imports one path. An action target is a record
with an identifier, the families it belongs to, its action, its semantic
requirement, its distinguishing test and its roadmap disposition, and DA12 is
one record that lists both its families.

Several visualizations read the same target and cannot disagree about it. The
type checker also sees the content, so a target without a distinguishing test
is a compile error and a facet name misspelled in one family's profile is a
compile error. In markup both would be a paragraph that silently reads wrong.

Copy lives in TypeScript, which is a worse place to write prose than a
document. Code samples are verbatim segments with real line numbers, checked at
build time against the repository file, and that check is cheaper to write
against a module than against a page.

### Hand-authored SVG

A charting dependency assumes the content is a numeric series. The site's
content is text: guard messages, facet sentences, claim names, manifest fields.
A facet placement has no ordinal value to encode as colour or height. A
force-directed layout would give arbitrary positions that differed between
visits and could not be read by a screen reader, and no proposal needed one.
Hand-authored SVG driven from the data modules gives real text that a screen
reader can speak and fixed positions that mean something. It brings no runtime
that fetches, animates or resizes on its own terms. Where the content is a list
of selectable text, as in the chain, the same rule chooses DOM over SVG,
because CSS draws spacing better than SVG does and native buttons give tab
order for free.

All of it is drawn by hand, and that is the price. The Register transposes at
400px instead of shrinking, and the transposition is written, not configured.
No tooltip, legend or axis comes free, and every responsive case is a decision
this project has to make and maintain.

### Motion that explains

Motion on the site has to say something about position or order. The Fork's
plausible column lands first and the required column lands a beat later, and
the order is the explanation. The chain enters as one bar and separates into
its obligations. The site does not reveal source with a typewriter, count a
number up from zero, pulse a mark when it fills or slide tabs as if they were a
carousel. A counting-up number implies approximation, and a slide implies the
tabs are a sequence when they are columns of a fixed table.

Under `prefers-reduced-motion` every duration is zero and every state is still
reached, so the meaning of every animation has to exist in text or position as
well. This is more work, because a site that explains with motion must explain
the same thing without it, and the page looks still to a visitor used to
product heroes.

### Accessibility as a requirement

The argument of the site is in its text. A diagram a screen reader cannot read
has withheld the argument from that reader, and a keyboard user who cannot
enter the grid has been given a picture of a classification. So the Register
is a grid with a roving tab index and arrow keys that cross column boundaries,
and the Fork's columns are real definition lists in the order a sighted reader
sees them. The chain's gap labels are hidden from assistive technology because
the panel speaks the same relation in real text, and every state carried by
colour is also carried by a word. Contrast is held in both themes, and the grey
used for a line the plausible model cannot represent is checked to 4.5:1,
because less defined must still be legible.

Moriarty's discipline is that nothing is claimed beyond what is established,
and a site that claimed to explain the language while part of its audience
could not read it would be overclaiming in the way the assurance chain warns
against.
Every coloured state needs a matching word, the narrow layout is a second
structure and not a scaled one, and a keyboard model for a two-axis grid is a
design task in itself.

## Completeness

Completeness is a requirement of this site. The category section covers every
DeFi category at the same structural depth, and a category with thinner
repository evidence gets a shorter tab, never a missing one and never a
placeholder. The same holds for the properties, the mandatory proof claims, the
threat rows and the sprints.

The test exists as a test because a checklist is read once, by a person, at
review, and the content changes after review. `src/data/completeness.test.mjs`
runs with `npm test`. It checks that the family identifiers are exactly the
fixed set in the fixed order and that every family answers every facet. It
checks that every target from DA01 to DA24 appears once, in order, with a
requirement and a distinguishing test. It checks that the operators DA24 names
are present, that the composition figures add up and that the stated ratio
agrees with the rates. It checks that the assurance layers, the properties, the
proof claims and the threat rows are the sets the spec fixes. A build that
trims a threat row fails. A build that rounds the ratio fails.

The guarantee chain lane found, while storyboarding, that the content spec
named its count of threat rows and listed fewer, and that the missing rows were
in `wiki/security.md`. A review had already passed that document. A test of the
data layer notices when the words and the facts part company, and it notices
every time and not once.

## Constraints

- No tooling attribution anywhere. The site, its source and its commit
  messages carry no credit to any tool or vendor that touched them.
- No third-party logos, no fabricated testimonials, no invented metrics.
- The fixed sets are fixed, and the completeness test enforces this.

### Why the honesty rules matter commercially

The rules in CONTENT-SPEC §10 read as ethics and they are also the sales
argument. The project's differentiator is that it states the assumption beside
the claim. Every property carries the assumption that qualifies it, every
threat carries the residual risk that survives the control, and the layered
assurance vocabulary is drawn as a chain because collapsing it is the standard
marketing lie in this space. The contradiction register in §7 is the competing
pitch: overviews that claim atomic cross-chain execution with automatic refunds
while the underlying verifier documentation says external calls complete
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

1. The survey of the repository is done.
2. The plan and content spec are this document and `CONTENT-SPEC.md`.
3. The storyboard proposals sit one per lane in `site/storyboards/`.
4. The centrepiece and the storyboard are decided above.
5. Implementation of the single-page application comes last.
