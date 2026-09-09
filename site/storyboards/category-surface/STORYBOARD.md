# Storyboard — the category surface

Proposal 1 of 5. Centrepiece: **the Register** (see `CENTREPIECE.md`). This
storyboard walks the nine-section spine of PLAN.md and states, for each
section, what is on screen, what the copy says, what interaction does, and how
Build and Verify differ. All copy below is the copy; it is written to be used.

Conventions used throughout:

- **Build** is the default mode. **Verify** is the second. The switch is a
  two-segment control in the site header (`Build · Verify`), persisted, and
  never moves.
- Family identifiers (`F1`, `⊥`) and target identifiers (`DA06`) are set in a
  small-caps label style, never in the code face, so they never read as source
  syntax.
- Every number is one from CONTENT-SPEC or CATEGORY-TABS. There are no others.
- Page-level structure: a 64 px fixed header (wordmark · section index · mode
  switch · theme), then nine sections with real `id` anchors, then a short
  colophon. The section index in the header is nine small numerals that fill
  as the reader passes each section — the same ink-mark language as the
  Register.

---

## Global surfaces

### Header

Left: `Moriarty` wordmark, set in the display face at 18 px, no logo mark.
Centre: nine numerals `1 2 3 4 5 6 7 8 9`, current one underlined, passed ones
filled. Right: `Build | Verify` segmented control; a theme control.

Changing mode does not scroll. Whatever is on screen re-renders in place with
the other mode's copy and ordering. Scroll position is preserved by anchoring
to the nearest section heading before the swap.

### Type and colour

- Display and body: one humanist serif with true small caps for identifiers
  and a monospace face for `.mori` and for numbers in tables. (Self-hosted;
  the prototype uses the system stacks as stand-ins.)
- Palette: paper (light) or slate (dark) ground, one ink, one muted ink, one
  rule colour, and eight low-saturation column hues used only for the standing
  column and the tab body's left rule. Accent for failure text (`"minimum
  output not met"`) is the ink itself in the code face, never red — a named
  failure is a correct result, not an alarm.
- Numbers keep their thousands separators exactly as the brief writes them:
  `1,000,000 A`, `19,743 B`, `1,830`, `5.14×`.

### Scroll behaviour

Sections are not snapped. Each section opens with a full-width heading band;
the band's eyebrow gives the section number and title. Scroll-linked motion is
limited to the section-index numerals and, in section 3, the Register's pinned
spine.

---

## 1 · Opening

**On screen.** Full viewport. Top third: the eyebrow `Moriarty · bounded
financial contracts on Midnight`. Centre: the one-line claim, set large:

> **Financial meaning that survives compilation, proof and settlement.**

Beneath it, at reading size, the problem statement and the ancestry line.
Bottom of the viewport: a slim preview of the Register — the eight column
headers and the twenty-four target chips, unmarked, at 40% ink. It is a hint of
the object to come, not a second copy. Scrolling brings the reader to it.

**Copy — Build.**

Compact and general smart-contract languages give you circuits and state. They
do not tell you which asset an amount denotes, how interest rounds, when a
payment becomes due, what a participant authorized, or which obligations
survive a transaction. Those are the questions that decide whether a repayment
discharged the right debt to the right creditor without erasing the remaining
principal. Moriarty makes them part of the type system and the operational
semantics instead of part of the audit.

A developer describes financial state, permitted actions, payment obligations
and authorization rules in a domain-specific language. Those descriptions
compile to Compact, and every transaction carries a proof that its execution —
and the contract history it extends — satisfies the agreement.

**Copy — Verify.** Same two paragraphs in the other order: the pipeline
sentence first, the questions second. Then one more line:

Moriarty is a new bounded financial-agreement language. It is not a renamed
Marlowe and not a general-purpose Compact dialect.

**Ancestry.** Three short columns under the heading *Three influences*, the
same in both modes:

- **ACTUS.** Reference behaviour for events, state transitions and cash flows:
  interest, principal repayment, maturity. Moriarty must reproduce the
  financial behaviour including dates and rounding. Scope: 277 fixtures, 18
  executable types, 32 source-backed taxonomy dispositions.
- **The DeFi corpus.** The project's catalogue of decentralized-finance
  behaviours: swaps, liquidity, lending, composition. 72 protocol rows across
  the historical 12-category set. It supplies conformance requirements; it is
  not a runtime service.
- **Marlowe.** A financial-contract DSL built so contract behaviour can be
  analysed before execution. Moriarty adopts that goal and builds its own
  authoring, proof and settlement architecture for Midnight.

**Interaction.** None beyond the mode switch. The Register preview is not
interactive here; it is a still.

---

## 2 · The category model

**On screen.** Two-level diagram, static, full width. Left: the seven families
as a vertical list, `F1` to `P`, each with its label and its examples line.
Right: the eight facets as a vertical list. Between them a single sentence
set in the margin: *Every instance carries values on all eight.* Below the
diagram, two callouts on a rule.

**Copy — heading.**

> **Seven kinds of financial thing. Eight properties every one of them has.**

**Copy — body.**

Two levels. Economic families say what kind of financial thing something is.
Facets are the orthogonal properties every instance has. Product labels are not
categories: the same protocol carries several facets and can appear in more
than one family.

The identifiers organise packages and evidence. They are not Moriarty source
syntax; there is no `F2` keyword.

**The two callouts** (both modes, verbatim):

- **Intents are an execution facet, not a family.** An intent is a way of
  authorizing an action, not a kind of financial product.
- **Bridges are a settlement facet, and they split by message-verified versus
  custodial trust.** Those are different trust assumptions. Collapsing them is
  a category error.

**Build differences.** The examples lines are the loud part ("AMMs, order
books, aggregators, routing"). The facet list shows a one-line gloss each,
e.g. *Custody — who holds the asset during the operation*.

**Verify differences.** The family list is quieter; the facet list is the loud
part and the diagram adds the phrase *classification, not list* as the caption.
A third line under the callouts: *The facets are what make the model a
classification rather than a taxonomy; the cross-cutting tab in the next
section carries the machinery that instantiates them.*

**Interaction.** Hovering or focusing a family in this section highlights that
family's column in the Register preview left over from section 1 if it is
still in view; otherwise nothing. This is deliberately a reading section.

---

## 3 · The category tabs — the Register

The section that carries the weight. Fully specified in `CENTREPIECE.md`;
this entry describes the page around it.

**On screen, top of section.** Heading band:

> **Eight tabs. Twenty-four action targets. Nothing elided.**

Sub-line (Build): *One tab per DeFi category. Each tab says what the category
is, where it sits on the eight facets, which actions the language must execute
and the bug that tells a correct implementation from a plausible one, what goes
wrong, what Moriarty does about it, and where the requirements come from.*

Sub-line (Verify): *One tab per economic family plus the cross-cutting tab.
The twenty-four action targets are the conformance surface; every tab renders
the same six blocks; DA12 appears in two tabs and is counted once.*

**Then the Register**, full width. Foot reads `targets read 0 / 24 · tabs
opened 0 / 8 · facets compared 0 / 8` on first visit, and whatever the reader
has reached on return.

**Then the tablist** — eight tabs in the fixed order, label and count each
(`F2 · Credit and collateralized debt · 7`), synced with the Register's
standing column. F1 is open on arrival.

**Then the tab body** — six blocks, always in this order, always all six:

1. **What this category is** — function paragraph, then *Boundary* paragraph
   set as a labelled aside.
2. **Facet profile** — eight rows, facet name left, full sentence right, in the
   fixed facet order. The row the reader last lifted in the facet lens is
   marked with the column's rule colour.
3. **Action targets** — one entry per target. Each entry: identifier and
   action as the heading; requirement; distinguishing test. The test is the
   emphasised element: set in the ink at body size while the requirement is
   set in the muted ink one step smaller.
   - Build: the entry heading is the action; under it, *The bug this catches*
     precedes the test, and *What it must mean* precedes the requirement.
   - Verify: the entry heading is the identifier; *Semantic requirement* comes
     first, then *Distinguishing test*. The wording of both is identical across
     modes — only order and labels change.
4. **What goes wrong** — the failure-mechanism paragraph.
5. **What Moriarty does about it** — the construct, type, policy or authority
   rule, with the `.mori` where the brief supplies it (F1, F2). For the other
   tabs, prose only; no placeholder code, no "example coming".
6. **Reference sources** — the pinned standards and papers, as a list.

**Tab copy.** All six blocks for all eight tabs are taken verbatim from
CATEGORY-TABS. The prototype carries the complete set; it is the content
module. Two places the storyboard adds emphasis only:

- On F2 block 3, DA06 carries a small label *flagship case*, because the
  brief names it so.
- On the ⊥ tab, block 1 is headed *What makes the other seven composable* and
  block 5 carries the measured result: *Over 1,830 eligible protocol pairs,
  1,645 compose cleanly and 185 fail. 182 of the 185 failures are
  cross-category. Within-category failure rate 2.10%; cross-category 10.79% — a
  5.14× difference.* This is stated once here and once in section 4; the two
  are cross-linked.

**Scroll behaviour.** When the Register's bottom edge passes the top of the
viewport the spine pins beneath the header. Scrolling within a long tab (F2,
⊥) keeps the spine's cursor on the target entry currently at the top of the
viewport, so the reader always knows which of the seven they are on. Leaving
section 3 unpins the spine.

**Interaction.** Column header → tab; chip → target entry; facet row → facet
lens (the panel replaces the tab body with the eight-family comparison for that
facet, and the tablist stays); facet cell → tab plus highlighted facet row;
Escape → back. Keyboard and screen-reader behaviour per `CENTREPIECE.md` §5.

**Coverage perception.** Three signals, always exact: the foot counters, the
filled chips and cells on the Register, and the section-index numeral for
section 3, which fills only when `targets read` reaches 24. A reader can leave
and return; the marks persist locally. There is one *clear marks* control at
the foot.

**Mode differences beyond block 3.** Build opens on F1 with the sub-line above
and the tablist labels; Verify opens on ⊥, because the researcher's entry is
the machinery, and its sub-line names DA12 explicitly. The Register itself does
not change between modes.

---

## 4 · Composition

**On screen.** A single numeric figure, drawn as two proportional rules — not a
bar chart, two lines whose lengths are the rates — and the five operators as
five terms on a rule.

**Copy — heading.**

> **Composition across categories is where DeFi breaks — 5.14× more often.**

**Copy — body (both modes).**

From the DeFiFormal audit, reproduced independently: over 1,830 eligible
protocol pairs, 1,645 compose cleanly and 185 fail. Of those failures, 182 are
cross-category and 3 are within-category. Within-category failure rate 2.10%;
cross-category 10.79%. That is a 5.14× difference.

Composition across financial categories is where DeFi breaks, and that is
exactly what a type system and an operational semantics can police.

**The figure.** Two horizontal rules labelled at the right with the exact
rates: `within-category · 2.10%` and `cross-category · 10.79%`; the lengths are
proportional; the multiplier `5.14×` set between them in the display face. The
counts `3 of 185` and `182 of 185` set under each. No axis, no ticks.

**The five operators.** `sequence · parallel · interleave · synchronize ·
message`, always five, each with one line taken from DA24: *operator-specific
authority, duties, conflicts and fan-in*. Below: *Distinguishing test: split
partitions work and claims; join cannot duplicate resource.*

**Build.** The operator strip links each operator to a place it is used:
sequence → F2 DA09 refinance; parallel and synchronize → DA08 flash borrow;
message → DA23; split/join → P DA13. Clicking scrolls to the Register and
focuses that chip.

**Verify.** A note under the figure: *An earlier informal claim of "sixty
times" was checked and is wrong; do not use it.* — presented as a correction
because the brief presents it as one. The operators are followed by the
sentence *Each has its own authority rules, duty propagation, conflict
semantics and fan-in behaviour.*

**Interaction.** Hovering a rule shows the underlying integers. Operators link
into the Register in Build. No motion other than the rules drawing to their
length on first entry (240 ms; none under reduced motion).

---

## 5 · The formalization model

**On screen.** Three stacked drawings, each full width: the five layers as a
table; the compilation pipeline as a vertical chain; the Midnight realization
as a row of bounded transition boxes.

**Copy — heading.**

> **Five layers say what a program looks like and what it means.**

**The five-layer table** (both modes, verbatim from CONTENT-SPEC §3): Lexical
structure · Syntax · Static semantics · Dynamic semantics · Correctness claims,
with specification method and what it fixes. The file extension `.mori` is
stated in the lead sentence.

**The pipeline.** The seven-stage chain drawn top to bottom in the code face:

```
Moriarty source (.mori)
  → typed elaboration + finite resource/lifetime certificate
  → canonical Moriarty Core
  → readable generated Compact + correspondence manifest
  → compactc
  → ZKIR 3 + generated TypeScript + proving/verifier artifacts
  → Midnight ledger and wallet
```

Beside the Compact stage, a labelled decision marker: **Decision — generate
Compact, not ZKIR directly.** Expanding it: *ZKIR is a typed straight-line
circuit IR with guarded impacts and no source-level financial concepts, and it
is ledger-coupled and evolving. Generating Compact preserves a reviewable
backend artifact and reuses the supported compiler, source maps, runtime
bindings and ledger operations.*

**Core / surface.** Two columns. Core retains: finite continuations, explicit
actions and waits, accounting, value conservation, explicit timeouts, typed
warnings and errors, a decreasing structural measure — and every datum is
classified `public`, `private`, `committed` or `revealed`, and every oracle or
external effect has a named capability and assurance boundary. Surface adds:
modules, named definitions, schedules, token-indexed amounts, durations,
records, packages, bounded compile-time loops and folds, templates. *All of it
must elaborate away into Core.* The manifest's eleven recorded fields are
listed in full.

**Midnight realization.** Heading: *A long-lived agreement is a sequence of
bounded one-transition proofs, not one circuit that executes an entire
lifetime.* The drawing: a row of five identical boxes labelled `transition n`,
`n+1`, …, each with *sealed* fields above and *mutable* fields below, and a
proof glyph between boxes. The five bullets of §3.3 sit under it, including
the `disclose()` rule and the timeout rule, verbatim.

**Build.** The layer table is collapsed to its first two columns by default
and expands; the pipeline is the loud element. The realization drawing's
caption: *Each transaction proves one step and that the step extends a
compliant history. Nothing proves a whole lifetime at once.*

**Verify.** The layer table is fully open. Under it, the three method notes:
EBNF (ISO/IEC 14977) chosen over ABNF (RFC 5234); K configurations and rewrite
rules for execution, typing judgments for admissibility, Hoare-style assertions
for what must be proved, denotational models as supporting analyses only; the
control layer presented in Felleisen–Hieb reduction semantics.

**Interaction.** The decision marker expands in place. The realization boxes
are focusable; each announces its sealed and mutable field names.

---

## 6 · The guarantees

**On screen.** The assurance chain drawn as six stacked rungs; the twelve-row
property inventory as a three-column table in its own horizontal scroll
container; the four mandatory claims as four numbered stones; the threat table.

**Copy — heading.**

> **Six separate obligations. Establishing one does not establish the next.**

**The chain**, verbatim from §4: semantic validity → Moriarty-to-Compact
translation validity → Compact-to-ZKIR compiler correctness → circuit / proof
correctness → Midnight ledger feasibility → transaction construction and
wallet correctness. Caption: *Collapsing this chain is the standard marketing
lie in this space.*

**The property inventory.** All twelve rows, three columns — Property ·
Assumption that qualifies it · Moriarty obligation — verbatim. The source line
under the table names the pinned Marlowe/Isabelle commit
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`. No row is collapsed in either mode.

**The four mandatory proof claims**, fixed names and order:
1. ContractInvariant — the agreement's own rules hold.
2. IntentRefinement — what executed refines what the principal authorized.
3. TransitionValidity — this state transition is legal.
4. HistoryCompliance — the predecessor history this extends is itself compliant.

Under the fourth: *A valid proof of a valid transition against an invalid
history must not be accepted.*

**Security and residual risk.** The nine high-value threat rows from §4.3 as
Threat · Required control · Residual risk, with the residual-risk column
emphasised. The line *A single "Moriarty audit" is not an adequate claim*
followed by the seven audit scopes.

**Build.** Ordering: four claims first, then the chain, then the threat table,
then the property inventory. Each of the four claims carries a one-line *what
breaks without it* gloss derived from §8's four mechanisms (e.g. HistoryCompliance
→ *kills the class where each step looks locally fine*).

**Verify.** Ordering: chain first, property inventory second (the hero), four
claims, threats. The inventory's assumption column is emphasised.

**Interaction.** Hovering a rung of the chain dims the rungs above it, showing
what the rung does not establish. The inventory scrolls sideways inside its
container at narrow widths; the body never does.

---

## 7 · The DSL

**On screen.** Two real `.mori` samples set large, with a numbered margin; the
`policy` block set as the centrepiece of the section; the declaration set as a
row of eleven terms; the workflow as a nine-step chain.

**Copy — heading.**

> **`ensures post.principal == pre.principal` is the whole thesis in one line.**

**Two profiles**, stated before any code: `moriarty-bounded-atomic/1` is the
implemented profile (grammar, parser, type checker, canonical encoding, local
evaluator, restricted Compact lowering; the loan and swap examples run under
it). `moriarty-successor-syntax/0` is the provisional successor (lexical
rules, EBNF grammar, bounded parser, canonical formatter, read-only CLI:
`check-syntax`, `format`). *They are not interchangeable.*

**Sample 1 — successor profile, PartialPayment**, verbatim from §5.2, with a
margin note on the `ensures` line: *paying interest must not silently touch
principal* and on the `Debt<USD>` type: *distinct from a transferable
`Amount<USD>`*.

**Sample 2 — atomic profile, swap**, verbatim from §5.2. Under it the concrete
instance in exact integers: *Reserves 1,000,000 A and 2,000,000 B; a 10,000 A
input returns 19,743 B under the integer formula with a 997/1000 fee. Ask for
19,744 and you get a named slippage failure, not a silent adjustment.*

**The `policy` block**, verbatim, with the six clauses annotated in the
margin: unit · derivation · rounding · remainder · comparison · proof. Heading
above it: *Rounding is a declared, named, provable artifact — not an accident
of integer division.*

**The declaration set.** `unit · party · const · state · observation ·
settlement · status · policy · reserve · effect · action`; inside an action
`guard` (with a named failure message) · `let` · `set` · `emit`; on the
agreement `lifetime` and `horizon`.

**The workflow.** The nine steps of §5.4 as a chain, with the two enforced
rules set beneath as a pair: *Simulation and static analysis are distinct — a
successful example does not turn the property list green.* · *A valid proof
does not make a stale predecessor live.* The three-column workspace
(AGREEMENT · BEHAVIOR · TRANSACTION) is described in one line each.

**Build.** Sample order: swap first (the one that runs), PartialPayment second.
The swap's failure line is the emphasised element: `"minimum output not met"`.

**Verify.** Sample order: PartialPayment first (the invariant), swap second.
The `requires / next / ensures` shape is called out as pre- and post-state
over an explicit transition.

**Interaction.** Margin notes reveal on hover or focus of the annotated line.
The two samples are real text, selectable, in `pre` containers that scroll
sideways if needed.

---

## 8 · Intents · multichain · safe DeFi

**On screen.** Three sub-sections under one band, each with a small drawing:
five artifacts on a line; the per-layer atomicity ladder; four mechanisms in a
two-by-two.

**Copy — heading.**

> **Authority is affine. A refund never restores allowance.**

**Intents.** The five artifacts `Intent · Permission · Plan · Execution ·
Receipt` on one rule. IntentIR, PlanIR and Receipt defined verbatim. The two
signing profiles as a pair: *exact-plan signing* (the working restricted
profile) and *outcome-intent signing*. Then the authority rules as a list,
verbatim: affine authority; gross not net, a refund never restores allowance,
fees have their own cap and still consume gross authority; every recipient
enumerated including refunds; goals compare net; asset identity is the entire
`{domain, issuer, reference, kind}` tuple, no ticker aliases, a receipt token
is not the delivered asset; validity `notBefore <= now < expiresAt` with nonce
and domain separator; pending progress has its own judgment; anti-vacuity.
Closing line: *Wallet rendering is a deterministic semantic signing summary,
not a hex blob.*

**Multichain.** Heading: *Atomicity is defined per layer and per route.* The
contradiction register's three findings, then Moriarty's three rules, then the
challenge set as a box:

> Construct a cross-chain swap trace in which the internal batch succeeds but
> the bridge withdrawal fails or is rolled back. Any model that cannot
> represent that trace — and reject it — is not modelling cross-chain
> settlement.

Under it: *Operator, relay, treasury and bridge attestations cannot discharge
a proof obligation.* And the bridge split, again: message-verified versus
custodial trust are different security stories.

**Safe DeFi.** The four mechanisms, numbered, verbatim from §8, each with the
class of failure it kills. Then the adversarial-modelling requirement (actor
capability · vulnerable layer · precondition · ordered effects · violated
predicate · loss outcome) and the two rules: *ABI shape does not establish
semantic compatibility* · *a valid oracle signature does not establish economic
truth.* And: *Flash borrowing is a capability with legitimate uses, not a
vulnerability.*

**Build.** Order: safe DeFi first (the four mechanisms are what the engineer
came for), intents second, multichain third. Each mechanism links to the
Register: 1 → DA20/DA21 typed observation and asset identity; 2 → F1 DA01 and
F6 DA17; 3 → DA21; 4 → HistoryCompliance in section 6.

**Verify.** Order: intents, multichain, safe DeFi. The IntentIR/PlanIR/Receipt
definitions are set as records; the validity predicate is set in the code face.

**Interaction.** The atomicity ladder has three rungs (internal ledger batch ·
route · bridge fulfilment); hovering one shows the sentence *never lift
this layer's atomicity to the one above*.

---

## 9 · Roadmap

**On screen.** Twelve rows, a three-track diagram, and three milestone stones.
This is the only section that speaks about status.

**Copy — heading.**

> **Twelve sprints. Three tracks running now. What has been reached.**

**The twelve rows**, SP01–SP12, Deliverable · Decisive completion evidence,
verbatim. No status column, no ticks, no colour: the brief supplies evidence
criteria, not completion states, and the site shows exactly that.

**Three tracks**, drawn as three parallel rules with their sprint nodes:
`SP01 → SP02 → SP03` (language) · `SP01 F0 → SP04 → SP06` (native feasibility
and proofs) · `accepted atomic + loan/swap subset → SP05` (financial Preview
integration).

**Milestones reached**, three stones, verbatim:

- The bounded K definition executes Transfer-only and Transfer-then-Repay with
  AccrualFirst / PrincipalFirst / ProRata allocation and explicit
  none/floor/ceil conversion rounding. All 16 frozen cases match independent
  financial expectations, including exact rejection code and index.
  Transfer-only carries 15 guards and 18 presentation steps; repayment carries
  28 guards across its stages and 37 steps.
- The successor syntax profile has lexical rules, EBNF grammar, bounded
  parser, canonical formatter and read-only CLI.
- A hello-world Compact deployment and call were finalized on a local Midnight
  network with exact state readback and indexed blocks below the finalized
  head.

**Build.** Milestones first, then tracks, then the twelve rows.

**Verify.** The twelve rows first, tracks, then milestones; the guard counts
15 / 28 are set in the display face beside the first milestone.

**Interaction.** None beyond row focus. This section has no marks to fill; the
section-index numeral fills on reaching it.

---

## Colophon

One line: *Every number on this page traces to the repository's content
specification.* No credits, no badges.

---

## How the Register threads the whole page

The Register is section 3's object, but its language — fixed rows, exact
counts, the reader's own ink — is the page's language:

- Section 1 previews it, still.
- Section 4's operators, section 6's fourth claim and section 8's mechanisms
  link back into it and focus the relevant chip, so the surface is the map the
  rest of the site points at.
- The header's section index fills the same way its cells do.
- Nothing anywhere on the page fills green, shows a checkmark, or claims a
  state the repository does not support.
