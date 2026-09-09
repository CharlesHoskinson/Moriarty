# Storyboard: the failure

Proposal for the Moriarty single-page application, built around the
distinguishing tests. Every guarantee on the site is introduced by the bug it
catches. The centrepiece is **the Fork**: one pre-state, one action, two
post-states. The left post-state is what a plausible balance-based
implementation produces; the right is what the distinguishing test requires,
either a preserved value or a named rejection. It is specified in
[`CENTREPIECE.md`](CENTREPIECE.md) and implemented in
[`prototype.html`](prototype.html).

Everything below traces to `site/PLAN.md`, `site/CONTENT-SPEC.md` and
`site/CATEGORY-TABS.md`. Where a section needs material that is not in those
three files it says so in a **Gap** note rather than inventing it.

---

## Visual system

**Read.** A technical argument for two expert audiences. Not a security-scare
page, not a launch page: a conformance sheet with a thesis. The page should
feel like a well-set financial statement: ruled, tabular, exact.

**Dials.** Variance 5, motion 3, density 5. Layouts are left-aligned on a
12-column grid with a wide text measure (65ch) and a right-hand column reserved
for source, figures and the Fork. Sections are separated by hairlines and
space, not by cards or background flips.

**Type.** Two families, both self-hosted. A grotesk with tabular figures for
prose and headlines (candidates: Geist, Söhne, PP Neue Montreal; tabular
lining figures are the requirement, the face is not). A monospace with slashed
zero for every number that is a quantity, every identifier, every `.mori`
fragment (candidates: Geist Mono, JetBrains Mono, Berkeley Mono). Rule:
**if it is a quantity, it is monospace and tabular.** 19,743 is always set
`19,743`, never rounded, never in the prose face.

**Colour.** One neutral ramp (cool, off-white `#f6f7f8` to off-black `#15171b`)
and one accent, deep green `#1e6b58`, used only for the "required" side of a
fork, the selected tab, and links. Two semantic inks: rust `#a8452a` for a
named rejection, and a mid-grey for the "plausible" side so that the wrong
answer is never coloured as danger, only as *less defined*. No glows, no
gradients, no red-alert vocabulary. Dark theme inverts the neutrals and
brightens both inks by one step; the accent stays recognisable.

**Shape.** Radius 2px everywhere. Chips, inputs, tabs and buttons share it.

**Motion.** Three motivated motions only: (1) a fork resolving, where the two
post-state columns fill from the pre-state after a case is chosen; (2) the
composition field drawing its 185 failures on first entry to section 4; (3)
the pipeline rail in section 5 advancing one stage at a time on
keyboard/scroll. All three collapse to instant under
`prefers-reduced-motion`. Nothing loops.

**Eyebrows.** Three on the whole page: sections 1, 3 and 6. Every other
section headline stands alone.

---

## Global chrome

**Header, 64px, sticky.** Left: the wordmark `Moriarty` in the monospace.
Centre: nine section anchors (1 Opening · 2 Model · 3 Categories · 4
Composition · 5 Formalization · 6 Guarantees · 7 DSL · 8 Intents · 9 Roadmap),
condensed under a "Contents" disclosure below 1024px. Right: the mode switch
and the theme toggle.

**Mode switch.** A two-segment control, `Build` | `Verify`, default `Build`,
persisted in `localStorage` under one key. Switching does not navigate: the
current section stays in view, block order inside the section re-sorts with a
120ms cross-fade (instant under reduced motion), copy variants swap, and the
Fork's expanded depth layer changes. The switch has `role="radiogroup"` and is
reachable from the first Tab press.

**Scroll.** Native scrolling only. No hijack, no pinning of whole sections.
Two sticky elements exist: the header, and, inside section 3 only, the
eight-tab strip, which docks beneath the header while section 3 is in view and
releases when it leaves. Section anchors update the URL hash on scroll via an
`IntersectionObserver`.

**Footer.** One line: the one-line claim, and the pinned Marlowe source commit
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af` as the provenance of the property
inventory. No version footer, no badges.

---

## 1. Opening

**Sees.** A split hero. Left: headline, dek, one primary action. Right: the
`PartialPayment` source, real, with the `ensures` line highlighted in the
accent, and beneath it a static miniature Fork for DA06. Below the fold line,
the ancestry row: three named influences, each with its real numbers.

**Build copy.**

> *eyebrow* Bounded financial contracts on Midnight
>
> # Paying interest must not touch principal.
>
> Moriarty is a language and toolchain for bounded financial contracts. The
> questions an audit asks become types and operational semantics, and every
> transaction carries a proof that its history satisfies the agreement.
>
> [ Read the distinguishing tests ]

Standing under the header, in the monospace, both modes: `Financial meaning
that survives compilation, proof and settlement.`

**Verify copy.**

> *eyebrow* A bounded financial-agreement language for Midnight
>
> # Financial meaning that survives compilation, proof and settlement.
>
> Lexical rules, an EBNF grammar, typing judgments, an executable K semantics
> and correctness claims stated over those semantics. Not a Marlowe rename,
> not a Compact dialect.
>
> [ Read the distinguishing tests ]

**Right column, both modes.**

```mori
profile "moriarty-successor-syntax/0";

agreement PartialPayment {
  unit USD;
  party borrower;
  party lender;
  state principal: Debt<USD> = debt(100, USD);
  state interest:  Debt<USD> = debt(10, USD);
  action payInterest(payment: Debt<USD>) {
    requires payment > debt(0, USD);
    requires payment <= pre.interest;
    next.interest = pre.interest - payment;
    ensures post.principal == pre.principal;
  }
}
```

Caption under the source, Build: "The last line is the whole thesis. A
balance-based model has nowhere to write it." Verify: "`Debt<USD>` is a type
distinct from `Amount<USD>`; `pre` and `post` are explicit; the `ensures` is a
proof obligation, not a comment."

**Miniature Fork (static).** Pre-state `principal 100 · interest 10`.
Left, grey: `balance 110 - payment`, sub-line "allocation: undefined".
Right, accent: `interest 10 - payment · principal 100`, sub-line
"`ensures post.principal == pre.principal`". It does not animate here; it is
the first appearance of the shape the reader will use in section 3.

**Ancestry row.** Three columns under a hairline, no cards.

- **ACTUS.** Reference behaviour for events, transitions and cash flows.
  Moriarty must reproduce it including dates and rounding. `277` fixtures ·
  `18` executable types · `32` source-backed dispositions.
- **The DeFi corpus.** Swaps, liquidity, lending, composition. `72` protocol
  rows across the historical `12`-category set. Conformance requirements, not a
  runtime service.
- **Marlowe.** A financial-contract DSL built so behaviour can be analyzed
  before execution. Moriarty adopts the goal and builds its own authoring,
  proof and settlement architecture for Midnight.

**Interaction.** The primary action scrolls to section 3 and focuses the first
tab. Hovering the highlighted `ensures` line shows a one-line tooltip: "Paying
interest may not touch principal."

**Mode difference.** Headline and dek swap; the source moves from right column
(Build) to below the ancestry row at a smaller scale (Verify), and a compact
five-layer strip (Lexical · Syntax · Static · Dynamic · Claims, each with its
method) takes the right column in Verify.

---

## 2. The category model

**Sees.** A two-part section. Part one: the seven families as a ruled list
(identifier, name, what lands there). Part two: the eight mandatory facets as a
horizontal band of eight labelled cells; each cell carries a one-line
definition. Two callouts sit between them.

**Headline.** Build: **Product labels are not categories.** Verify: **Two
levels: families and facets.**

**Body, both modes.**

> A protocol carries several facets and can appear in more than one family.
> The families are the map; the facets are what make it a classification
> rather than a list.

**Families.** F1 Exchange and price discovery · F2 Credit and collateralized
debt · F3 Derivatives · F4 Consensus-position claims · F5 Tokenized off-chain
claims · F6 Delegated asset management · P Prediction markets, each with the
examples column from the spec. Under the list, in the monospace: `F2 is a
classification label, not a keyword. Moriarty source has no F2.`

**Facets band.** Execution · Settlement · Custody · Legal dependence ·
Collateral and solvency · Oracles · Authorization and mandate · Price discovery.
Always eight, always in this order.

**Callouts.**

> **Intents are an execution facet, not a family.** An intent is a way of
> authorizing an action, not a kind of financial product.

> **Bridges are a settlement facet, and they split by trust.** Message-verified
> and custodial bridges are different trust assumptions. Collapsing them is a
> category error.

**Interaction.** Hovering or focusing a facet cell highlights that facet's
line in every tab's facet profile preview (a thin strip of eight cells is
mirrored at the top of section 3). This is the reader's first evidence that
the same eight questions are asked of every family.

**Mode difference.** Build orders families then facets; Verify orders facets
then families and adds one sentence: "The facet values are what the semantic
requirement of each action target constrains; the family is only the index."

---

## 3. The category tabs, carrying the Fork

This section carries the weight and is where the centrepiece lives.

**Sees.** Under the section headline, a sticky strip of eight tabs with their
action-target counts:

`F1 Exchange 3 · F2 Credit 7 · F3 Derivatives 2 · F4 Consensus 2 · F5 Off-chain
1 · F6 Delegated 3 · P Prediction 1+1 · ⊥ Cross-cutting 5`

Inside the active tab, six blocks. Blocks 3 (action targets) and 4 (what goes
wrong) are rendered together as the Fork: a case index of that tab's action
targets on the left, and the Fork panel on the right showing the selected
case's pre-state, plausible outcome and required outcome. Blocks 1, 2, 5 and 6
are prose, a facet strip, real source, and a reference list.

**Headline.**

> *eyebrow* Eight categories, twenty-four actions
>
> # Every action has one case that tells a correct implementation from a plausible one.
>
> The distinguishing test is the case where a balance-based model and an
> obligation-based model give different answers. Pick a category; pick an
> action; the Fork shows both answers.

**Block order.** Build: 1 What it is → 3+4 Fork → 5 What Moriarty does → 2
Facet profile → 6 Sources. Verify: 1 → 2 → 3+4 Fork → 5 → 6. Reordering is a
cross-fade; nothing is hidden in either mode.

**The Fork inside a tab.** Full specification in `CENTREPIECE.md`. Summary:
the case index lists the tab's action targets (identifier, action, and the
distinguishing test in emphasis). Selecting one fills the Fork: PRE-STATE ·
PLAUSIBLE · REQUIRED, with the differing line highlighted and an "Enforced by"
row naming the construct. Two cases are live calculators: DA01 (the reader
sets `min_out`; the page computes `19,743` from the real formula and fires the
named guard) and DA06 (the reader sets the payment; the `requires` lines fire
by name and `post.principal == pre.principal` is checked). Build mode expands
the *test* layer; Verify mode expands the *requirement* layer and shows the
test as a conformance vector.

**Per-tab content.** All copy below is from `CATEGORY-TABS.md`; the storyboard
notes only what each tab does that the others do not.

### F1 Exchange and price discovery

- **What it is.** Convert one asset into another and, in doing so, produce a
  price. Boundary: the exchange function is separate from the mechanism;
  routing and aggregation are execution-facet properties layered on top.
- **Facet profile.** Execution atomic or intent-mediated · Settlement
  same-domain, immediate · Custody pool-held during the swap · Legal
  dependence none · Collateral: reserves are the backing · Oracles usually
  none for CFMMs, the pool is the price · Authorization per-transaction with
  slippage bounds · Price discovery: this family produces it.
- **Fork cases.** DA01 (live calculator), DA02, DA03.
- **What goes wrong** (rendered as the plausible column of the Fork and as a
  paragraph): integer division decides who keeps the remainder; an unguarded
  zero-supply case transfers value silently and legally; an output that
  empties a reserve satisfies the constant-product identity and destroys the
  pool.
- **What Moriarty does.** The `policy swap_output` block, verbatim, with the
  two guards `"minimum output not met"` and `"output would empty reserve"`.
  The concrete instance: reserves `1,000,000` A and `2,000,000` B, fee
  `997/1000`, input `10,000` A, output exactly `19,743` B; requesting
  `19,744` produces the named slippage failure.
- **Sources.** AMM literature §§2.3.2, 3.1, 3.3, 4; Uniswap V4 core.

### F2 Credit and collateralized debt

- **What it is.** Create an obligation to repay and manage it through accrual,
  partial performance, default and discharge. Boundary, set large: **Nominal
  debt is not a transfer.** The family Moriarty is architecturally organized
  around.
- **Facet profile.** The eight lines from the spec, with "Collateral and
  solvency: the defining facet" and "borrower authority to draw, liquidator
  authority to seize, and they are different mandates" emphasised.
- **Fork cases.** DA04, DA05, DA06 (live calculator, the flagship case),
  DA07, DA08, DA09, DA10. Seven cases; the index scrolls internally above 5.
- **What goes wrong.** A repayment reduces a balance without discharging the
  right obligation. Interest and principal are allocated by whichever
  subtraction the code performed first. A withdrawal that cannot be funded
  erases the claim instead of rejecting. A refinance issues new debt without
  discharging old.
- **What Moriarty does.** `Debt<T>` distinct from `Amount<T>`; allocation
  policy explicit (AccrualFirst, PrincipalFirst, ProRata) with none/floor/ceil
  rounding. The `payInterest` action; the `emit Transfer` / `emit Repay` pair
  with `TransferId("T1")`, `AllocationId("Alloc1")`, `ObligationId("Due100")`;
  the K rejections named by code: `DUST` for a positive nominal payment
  converting to zero settlement; converted settlement checked against the
  preceding transfer's funding; ProRata overflow checked against UInt128
  before division; the closure guard `"episode closure cannot discharge
  remaining notional"`.
- **Sources.** Lending literature §§3.2–3.4; Kotzer §§III-B, III-C, V-C;
  Werner §3.2; Gogol §IV-A3; ERC-3156.

### F3 Derivatives

- **What it is.** Payoffs that reference something else rather than conveying
  ownership of it. "Synthetic" used narrowly.
- **Facet profile.** Continuous margin maintenance; funding and terminal
  settlement on different clocks; oracles mandatory and adversarial;
  position-holder mandate distinct from liquidator mandate.
- **Fork cases.** DA11, DA12 (badge: *shared with P*).
- **What goes wrong.** Funding sign inversion. Settlement convention
  ambiguity: mark, index or last trade, and at which timestamp. An expiry
  sweep that cancels obligations already validly exercised. Insolvency
  recognized after the loss rather than as a state.
- **What Moriarty does.** Time and duration are distinct types from amounts;
  an exercised claim is a `Debt`-typed duty that expiry cannot erase without
  violating an `ensures`; observations carry feed, unit, timestamp, sequence
  (DA20); `lifetime` and `horizon` bound the agreement itself.
- **Sources.** Werner §3.5; Gogol §IV-B3.

### F4 Consensus-position claims

- **What it is.** Staking, liquid staking, restaking, shared security.
  Boundary: slashable allocation is not leveraged lending or a token-lock
  reward.
- **Facet profile.** Two-phase execution with a protocol-imposed delay;
  delegated custody with persisting operator authority; slashing is loss
  allocation, not liquidation.
- **Fork cases.** DA14, DA15.
- **What goes wrong.** A rebasing balance treated as a fixed claim, or a fixed
  share balance treated as if quantity were entitlement. Exit modelled as
  instantaneous, so the pending state (still slashable, no longer earning,
  not yet transferable) has no representation.
- **What Moriarty does.** The pending state is an object with an identity;
  same machinery as DA18; affine authority means a partially completed exit
  consumes its authorization and the residual cannot grow; slashing must
  produce a compliant successor of a compliant history.
- **Sources.** Gogol §IV-A1; pending-workflow analysis. Note shown in the
  tab: DA15 carries an identified primary-lifecycle source gap.

### F5 Tokenized off-chain claims

- **What it is.** Claims ultimately enforced off-chain. A receipt is evidence
  of a deposit; the redemption right is a separate specification. ERC-3643 and
  ERC-7943 describe transfer policy, not legal conclusions.
- **Facet profile.** Legal dependence maximal; custody: an off-chain custodian
  holds the underlying; attestation is the oracle and the attestor can lie;
  valuation-time / publication-time gap.
- **Fork cases.** DA16. One case; the shortest tab, at full structural depth.
- **What goes wrong.** The on-chain leg completes and is treated as
  settlement. "The token was burned and the wire has not arrived" has no
  representation, so it is represented as done.
- **What Moriarty does.** Every oracle and external effect has a named
  capability and an assurance boundary recorded in the manifest; a typed
  observation carries source, feed, unit, timestamp, freshness, sequence,
  bounds and fallback, and the residual risk is printed beside it: **the
  source can still lie.** External contract calls excluded from V0; later a
  capability manifest, allowlist and effect summary; **external behavior
  cannot inherit Moriarty guarantees.**
- **Sources.** Gogol §§III-B, IV-A2; ERC-3643; ERC-7943; ERC-8330; Centrifuge
  protocol source.

### F6 Delegated asset management

- **What it is.** Someone else allocates your capital. "Vault" is the most
  overloaded word in the vocabulary; always qualify it.
- **Facet profile.** Synchronous or asynchronous execution; nested exposure;
  valuation is the oracle problem in disguise; the depositor authorized a
  policy, not each trade; NAV is a valuation, not a realizable price.
- **Fork cases.** DA17, DA18, DA19.
- **What goes wrong.** The donation attack; rounding in the depositor's favour
  on one method and the vault's on another; an asynchronous claim processed
  twice because the request had no residual amount; an unwind that reports the
  position closed while its debt persists.
- **What Moriarty does.** The `policy` block requires each conversion to state
  rounding direction and remainder disposition; Pending, Claimable, Claimed
  are distinct states with a residual request amount; conversion, preview,
  limit and execution semantics are distinct so a preview cannot be mistaken
  for an authorization; each delegation is a named capability.
- **Sources.** ERC-4626; ERC-7540 `requestRedeem`; ERC-7575; yield literature
  §§III-A Fig. 2, III-B, V-B; Morpho MetaMorpho source.

### P Prediction markets and event-contingent claims

- **What it is.** Claims indexed by an event, resolved by evidence. Held apart
  from F3 because resolution is evidentiary: not "what is the number" but
  "what happened, and who says so".
- **Facet profile.** Collateral held against the full outcome set; **the
  resolution source is the entire trust model**; who may resolve and under
  what evidence is the critical mandate; price discovery produces probability
  estimates.
- **Fork cases.** DA13, and DA12 with the *shared with F3* badge. The tab
  count reads `1 + 1` so the global total still reads 24.
- **What goes wrong.** Split and merge treated as token operations rather than
  conservation-preserving transformations, so a merge can duplicate value.
  Resolution accepted from a source whose authority was never modelled. No
  rejection path for an invalid resolution, so the first answer wins.
- **What Moriarty does.** Split and merge are the DA24 operators under a
  conservation property: **split partitions work and claims; join cannot
  duplicate resource.** Resolution is a typed observation with a named
  capability. **A valid oracle signature does not establish economic truth.**
  Conditional-token split/merge is presented as architecture belonging to a
  later composition layer, not as a shipped feature.
- **Sources.** DeFi report contingent-claim targets; Werner §3.5. Shown: DA13
  carries an identified primary-lifecycle-source gap.

### ⊥ Cross-cutting

- **What it is.** Not a leftover tab. The five targets every other category
  depends on.
- **Facet profile.** Rendered as the honest answer for each of the eight
  facets: these targets *constrain* facet values rather than taking them.
  (Execution: defines intents and exact-plan authority · Settlement: defines
  pending messages and refund duty · Custody: n/a, inherited · Legal
  dependence: n/a · Collateral: n/a · Oracles: defines typed observation ·
  Authorization: defines affine gross authority · Price discovery: defines
  observation provenance.) The "n/a" cells are shown, not dropped.
- **Fork cases.** DA20, DA21, DA22, DA23, DA24. DA24's Fork carries the
  cross-chain trace as its plausible/required pair: internal batch succeeds,
  bridge withdrawal rolled back; a model that cannot represent and reject that
  trace is not modelling cross-chain settlement.
- **What goes wrong.** Governance treated as outside the financial semantics,
  so positions get changed retroactively; messaging modelled without pending
  commitments, so delayed and duplicate delivery have no refund duty.
- **What Moriarty does.** The five composition operators, always listed as
  five: **sequence, parallel, interleave, synchronize, message**, each with
  its own authority rules, duty propagation, conflict semantics and fan-in.
  The composition result stated here in full and drawn in section 4.
- **Sources.** As per the spec: DA20–DA24 from the action-target matrix; the
  cross-chain material of CONTENT-SPEC §7.

**Completeness surface.** A small line under the tab strip, in the monospace,
reads `24 action targets · DA12 shared by F3 and P` and is computed from the
data module, not typed. If the count is not 24 the build fails.

**Keyboard.** Tab strip is `role="tablist"` with arrow-key movement; the case
index is a `radiogroup`; the Fork's live inputs are ordinary number inputs.
Full behaviour in `CENTREPIECE.md`.

**Mode difference.** Build: the case index leads with the *test* line and the
Fork's "Enforced by" row names the construct in developer terms ("the type
system will not let you add `Debt<USD>` to `Amount<USD>`"). Verify: the case
index leads with the *semantic requirement*, the Fork adds a fourth thin row,
"Obligation", phrased as a claim over the semantics ("for all `p` with
`0 < p ≤ interest`, `post.principal = pre.principal`"), and the "Enforced by"
row names the layer (static semantics, K rule, policy, manifest).

---

## 4. Composition

**Sees.** A full-width field of `1,830` small marks, partitioned into two
blocks, `143` within-category pairs and `1,687` cross-category pairs, with the
`185` failures inked in rust: `3` in the first block, `182` in the second.
Beneath it, the five operators as five short definitions.

**Headline.** Build: **Composition across categories fails 5.14 times as
often.** Verify: **The five operators, and why they carry the proof burden.**

**Body.**

> From the DeFiFormal audit, independently reproduced locally: over `1,830`
> eligible protocol pairs, `1,645` compose cleanly and `185` fail. `182` of
> the `185` failures are cross-category and `3` are within-category. The
> within-category failure rate is `2.10%`; the cross-category rate is
> `10.79%`. That is a `5.14×` difference.

> Composition across financial categories is where DeFi breaks, and it is
> exactly what a type system and an operational semantics can police.

Under the field, in the monospace, the derivation the reader can check:
`143 within and 1,687 cross are implied by 3 / 2.10% and 182 / 10.79%; they sum
to 1,830. Marks are placed by count, not by pair identity.`

**Operators.** sequence · parallel · interleave · synchronize · message. One
line each: "Each has its own authority rules, duty propagation, conflict
semantics and fan-in behaviour." The line is repeated once, not five times;
the five names are set large.

**Interaction.** First entry draws the failures in over 900ms (instant under
reduced motion). A two-state toggle `by category | by result` regroups the
field between "within / cross" and "clean / failed" so the reader sees the
same 1,830 marks from both partitions. Hovering either block reads its exact
count and rate in a tooltip; the same text is in the block's `aria-label`.

**Mode difference.** Build leads with the number and the toggle. Verify leads
with the operators, and the field is captioned as "the empirical motivation
for DA24's proof burden" rather than as the argument itself.

---

## 5. The formalization model

**Sees.** Three figures in sequence. (a) The five-layer table: Layer · Method ·
What it fixes. (b) The compilation pipeline as a vertical rail of seven
stations, with a dashed branch off `Moriarty Core` labelled `direct to ZKIR:
deferred` and its reason beside it. (c) The Midnight realization: a horizontal
chain of small boxes, each one bounded transition with its own proof; the
sealed fields drawn as a fixed band above the chain and the mutable public
state as a band that changes box to box.

**Headline.** Build: **What the compiler emits, and why you can read it.**
Verify: **Five layers, one Core, a sequence of bounded proofs.**

**Layer table copy.** Lexical structure: separate token rules and regular
expressions · Syntax: EBNF (ISO/IEC 14977) · Static semantics: typing and
scoping judgments, `Γ ⊢ e : τ` · Dynamic semantics: executable operational
semantics in the K Framework · Correctness claims: explicit properties over
those semantics. Verify adds the "why": EBNF describes grammar and does not
decide surface style (ABNF, RFC 5234, is the protocol-spec sibling); K
describes execution through configurations and rewrite rules; denotational
models support particular analyses but do not replace the execution
definition; the control layer is presented as Felleisen–Hieb reduction
semantics.

**Pipeline rail.** `.mori` → typed elaboration + finite resource/lifetime
certificate → canonical Moriarty Core → readable generated Compact +
correspondence manifest → `compactc` → ZKIR 3 + generated TypeScript +
proving/verifier artifacts → Midnight ledger and wallet. The deferred branch
reads: "ZKIR is a typed straight-line circuit IR with guarded impacts and no
source-level financial concepts, and it is ledger-coupled and evolving.
Generating Compact preserves a reviewable backend artifact and reuses the
supported compiler, source maps, runtime bindings and ledger operations."

**Core / surface.** Two columns. Core retains: finite continuations, explicit
actions and waits, accounting, value conservation, explicit timeouts, typed
warnings and errors, a decreasing structural measure; plus visibility and
trust: every datum is `public`, `private`, `committed` or `revealed`, and every
oracle or external effect has a named capability and assurance boundary.
Surface adds: modules, named definitions, schedules, token-indexed amounts,
durations, records, packages, bounded compile-time loops/folds, templates; all
of it elaborates away. The manifest's fields listed in the monospace.

**Midnight realization.** Headline inside the figure: **A long-lived agreement
is a sequence of bounded one-transition proofs, not one circuit for a
lifetime.** Four notes beside it: sealed ledger fields; mutable public state;
private witness callbacks are unverified TypeScript so a generated circuit
constrains every witness result; `disclose()` authorizes a flow past the
privacy analysis and does not make it safe, so it is generated only from an
explicit source visibility transition; timeouts are permissionless exported
transitions guarded by block-time predicates and do not execute autonomously.

**Interaction.** The rail advances one station per arrow key or per scroll
step through the figure; each station reveals its one-line description. The
chain figure lets the reader step through boxes; the sealed band never
changes, the mutable band does. Under reduced motion both render fully.

**Mode difference.** Build shows (b) first, then (c), then (a). Verify shows
(a), (b), (c). Build describes the manifest as "what a reviewer diffs"; Verify
lists its fields.

---

## 6. The guarantees

**Sees.** (a) The assurance chain as six linked labels. (b) The property
inventory, twelve rows, three columns, in its own horizontal scroll container.
(c) The four mandatory proof claims as four numbered lines. (d) The threat
table, thirteen rows, in its own scroll container.

**Headline.**

> *eyebrow* The layered assurance vocabulary
>
> # Establishing one layer does not establish the next.
>
> semantic validity → Moriarty-to-Compact translation validity →
> Compact-to-ZKIR compiler correctness → circuit / proof correctness →
> Midnight ledger feasibility → transaction construction and wallet
> correctness

Build adds under the chain: "Each arrow is a separate obligation. A site that
draws one arrow is telling you a marketing story." Verify: "Each arrow is a
separate obligation with its own evidence and its own residual."

**Property inventory.** All twelve rows, three columns, never collapsed:
Property · Assumption that qualifies it · Moriarty obligation. Contents
verbatim from CONTENT-SPEC §4.1. Row headers are the property name in the
grotesk; assumptions and obligations in prose. Provenance line above the
table: `Marlowe/Isabelle lineage, pinned at marlowe-lang/marlowe
7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`.

**The four claims.** `1 ContractInvariant` the agreement's own rules hold ·
`2 IntentRefinement` what executed refines what the principal authorized ·
`3 TransitionValidity` this state transition is legal · `4 HistoryCompliance`
the predecessor history this extends is itself compliant. Beside claim 4,
in the accent: "A valid proof of a valid transition against an invalid history
must not be accepted."

**Threat table.** Thirteen rows: Threat · Required control · Residual risk.
The nine rows in the spec, verbatim: Malicious witness · Accidental
disclosure · Compiler mistranslation · Token or unit confusion · Authorization
replay · Oracle manipulation · Continuation loss · Runtime substitution ·
External contract call. **Gap.** The remaining four rows (Invalid initial
state, Backend version skew, Resource exhaustion, Governance capture) exist in
`wiki/security.md` and are not yet in `CONTENT-SPEC.md`; they must be lifted
into the spec verbatim before implementation, not paraphrased here. The
residual-risk column is set in the accent's ink weight, because it is the
column nobody publishes.

Closing line: "A single 'Moriarty audit' is not an adequate claim." followed
by the seven audit surfaces.

**Interaction.** Clicking a chain label scrolls the threat table to the rows
that concern that layer and highlights them. Each property row can be expanded
to show which Fork case exercises it (Conservation of value → DA24 join;
Authorization → DA21; Continuation integrity → DA23; Determinism → DA01
rounding), which is the angle's link back to section 3.

**Mode difference.** Build: each property row shows a fourth thin line, "what
breaks without it", drawn from the corresponding tab's "what goes wrong"
paragraph. Verify: the three-column table is the hero, the chain is drawn
above it, and the four claims are stated as judgments.

---

## 7. The DSL

**Sees.** The two syntax profiles as two labelled columns; then the swap
action, real, with the `19,743` calculator (the same live component as DA01 in
the Fork, re-mounted here with the full source visible); then the `policy`
block; then the declaration set as a single line of eleven tokens; then the
developer workflow as a numbered rail; then the three-column workspace.

**Headline.** Build: **Rounding is a declared, named, provable artifact.**
Verify: **Two profiles, eleven declarations, one policy form.**

**Profiles.** `moriarty-bounded-atomic/1`: the implemented profile; grammar,
parser, type checker, canonical encoding, local evaluator and restricted
Compact lowering; the loan and swap examples run under it.
`moriarty-successor-syntax/0`: provisional; separate lexical rules, EBNF
grammar, bounded parser, canonical formatter, read-only CLI (`check-syntax`,
`format`). They are not interchangeable.

**Swap source.** Verbatim from CONTENT-SPEC §5.2, all guards, with the two
`emit Transfer` lines. Caption: "Reserves `1,000,000` A and `2,000,000` B, a
`10,000` A input, `997/1000` fee: output `19,743` B. Ask for `19,744` and you
get a named slippage failure, not a silent adjustment."

**Policy block.** The `accrued_interest` policy verbatim. Caption: "Every
policy names its unit, its derivation, its rounding direction, what happens to
the remainder, how it is compared, and the proof obligation it discharges."

**Declarations.** `unit · party · const · state · observation · settlement ·
status · policy · reserve · effect · action`, then inside an action: `guard`
(with a named failure message) · `let` · `set` · `emit`; plus `lifetime` and
`horizon` on the agreement.

**Workflow.** Import or author agreement → Check types and bounds → Inspect
events and simulate / Analyze named properties (drawn as two parallel steps)
→ Prepare and verify plan → Review and sign intent → Prove authorized
transition → Verify statement and proof → Check live ledger state and submit.
Two rules set beside it: **simulation and static analysis are distinct; a
successful example does not turn the property list green.** **A valid proof
does not make a stale predecessor live.**

**Workspace.** AGREEMENT · BEHAVIOR · TRANSACTION, three columns with their
listed contents, drawn as a plain ruled layout, not a fake screenshot.

**Interaction.** The calculator; hovering a declaration token shows its
one-line role; the workflow rail highlights the parallel pair when either is
focused.

**Mode difference.** Build: swap and calculator first, then policy, then
profiles. Verify: profiles first, then declarations, then policy, then swap.

---

## 8. Intents, multichain, safe DeFi

**Sees.** Three sub-sections under one headline. (a) The five artifacts as five
labelled positions on a line: Intent · Permission · Plan · Execution · Receipt,
with IntentIR, PlanIR and Receipt defined beneath their positions. (b) The
authority rules as a ruled list of eight. (c) Multichain: the three rules and
the challenge trace, drawn as a two-layer timeline (internal batch, bridge)
where the bridge lane ends in a rollback. (d) The four safety mechanisms, each
linked to the Fork cases that motivate it.

**Headline.** Build: **A refund never restores allowance.** Verify: **Intent,
permission, plan, execution, receipt.**

**Artifacts.** IntentIR = principal + scope + authority + requirements +
assumptions + lifecycle + validity/replay + semantic versions: what outcome
you authorize, before any route. PlanIR = intent identity + bounded steps +
adapter identities + dependencies: one chosen route. Receipt = intent/plan
identities + checked effects + status + residual resources. Two signing
profiles: exact-plan (the working restricted profile) and outcome-intent
(authority, goals, permitted programs, validity and nonce fixed before a plan
exists; solvers search outside the finite checker; ranking cannot excuse
invalid authority or a failed goal; no global best-price claim).

**Authority rules, verbatim list.** Authority is affine; partial completion
consumes it; residual authority cannot grow or reset an epoch. Gross, not net:
sum gross outgoing transfers and fees by asset across all recipients; a refund
never restores allowance; fees have their own cap and still consume gross
authority. Every recipient on the permitted list, including refunded
movements. Goals compare net final-minus-initial credits; fees count against
the net goal. Asset identity is the entire `{domain, issuer, reference, kind}`
tuple; no ticker aliases; a receipt token is not the delivered asset. Validity
is `notBefore <= now < expiresAt` with a nonce and a domain separator; an
envelope's key cannot appoint itself authoritative. Pending progress has its
own judgment; a pending receipt can never show a terminal goal as settled.
Anti-vacuity: require trace inclusion and a feasible positive witness.
Wallet rendering is a deterministic semantic signing summary, not a hex blob.

**Multichain.** The contradiction register in three lines (marketing claims
atomic cross-chain execution with automatic refunds; the verifier
documentation says external calls complete asynchronously, simulation excludes
them, and some paths are detached, nonrefundable or manually recovered;
"non-custodial" coexists with contract-held balances, a trusted swapping
agent, and a treasury plus proof-of-authority bridge; confidentiality labels
can lack normative leakage definitions). The three rules: define atomicity per
layer and per route; publish a route-specific custody and authority manifest
before approval; model confidentiality as a named adapter profile. The
challenge, set as a challenge: **construct a cross-chain swap trace in which
the internal batch succeeds but the bridge withdrawal fails or is rolled back.
A model that cannot represent that trace, and reject it, is not modelling
cross-chain settlement.** Operator, relay, treasury and bridge attestations
cannot discharge a proof obligation. Bridges split into message-verified and
custodial trust; the figure draws them as two separate lanes, never one.

**Four mechanisms, each with its Fork cases.**

1. Typed asset identity and units → DA16, DA20, DA11. Kills unit-confusion and
   ticker-aliasing at the type level.
2. Declared rounding and remainder policy → DA01, DA17, DA02. Kills the
   rounding-direction class of vault and share-conversion exploits.
3. Affine authority with gross accounting → DA21, DA15, DA18. Kills the
   authorization-scope class.
4. History compliance as a proof obligation → DA14, DA22, DA19. Kills the
   class where each step looks locally fine.

Closing rules: adversarial fixtures carry separate actor capability,
vulnerable layer, precondition, ordered effects, violated predicate and loss
outcome. **ABI shape does not establish semantic compatibility. A valid
oracle signature does not establish economic truth.** Flash borrowing is a
capability with legitimate uses, not a vulnerability.

**Interaction.** Clicking a mechanism's case identifiers jumps to section 3
with that case selected in the Fork (the tab strip switches accordingly). The
bridge timeline can be stepped; the rollback step is the terminal state and
the caption reads "rejected: bridge withdrawal rolled back after internal batch
success".

**Mode difference.** Build: (b) rules first, (d) mechanisms second, (a) and (c)
after. Verify: (a) artifacts first, (c) multichain with the trace as a
proof-obligation statement, then (b), then (d).

---

## 9. Roadmap

**Sees.** Twelve sprints as a single vertical rail, `SP01` to `SP12`, each with
its deliverable and its decisive completion evidence. Three parallel tracks
drawn as three thin lines beside the rail. A short "reached" list at the end
in plain text.

**Headline.** **Twelve sprints. Three tracks run now.**

**Sprints.** Verbatim from CONTENT-SPEC §9, all twelve, no elision.

**Tracks.** `SP01 → SP02 → SP03` language · `SP01 F0 → SP04 → SP06` native
feasibility and proofs · `accepted atomic + loan/swap subset → SP05` financial
Preview integration.

**Reached.** Bounded K definition executes Transfer-only and
Transfer-then-Repay with AccrualFirst / PrincipalFirst / ProRata allocation
and explicit none/floor/ceil conversion rounding; all `16` frozen cases match
independent financial expectations including exact rejection code and index;
Transfer-only carries `15` guards and `18` presentation steps; repayment
carries `28` guards and `37` steps. The successor syntax profile has lexical
rules, EBNF grammar, bounded parser, canonical formatter and read-only CLI. A
hello-world Compact deployment and call finalized on a local Midnight network
with exact state readback and indexed blocks below the finalized head.

No checkmarks, no green states, no progress bars. "Reached" is a word in a
sentence with its evidence beside it.

**Mode difference.** Build: the reached list first, then the rail. Verify: the
rail first, with the guard counts `15 / 28` cross-linked to section 5.

---

## Data modules the storyboard depends on

`families` (7), `facets` (8), `actionTargets` (24, with `families[]` so DA12
carries two), `forkCases` (24, the pre/plausible/required triples used by the
centrepiece), `compositionResult` (the eleven numbers above), `properties`
(12), `proofClaims` (4), `threats` (13, four pending the spec gap), `sprints`
(12), `codeSamples` (PartialPayment, swap, swap_output policy,
accrued_interest policy, Transfer/Repay pair, closure guard).

## Acceptance against CATEGORY-TABS.md

- Eight tabs, six blocks each, all populated: yes; ⊥'s facet profile shows
  "n/a" cells rather than omitting them.
- DA01–DA24 each present; DA12 twice, badged shared: yes; count line computed.
- Requirement and test shown for every target: both in the case index and in
  the Fork.
- Seven family identifiers and eight facet names never abbreviated or
  reordered: tab labels carry the identifier and full name; the strip
  abbreviates the *name* only in the compact tab label and never the
  identifier; the facet band is always eight in order.
- Five operators always five: section 4 and ⊥ tab.
- Identifiers presented as classification labels: the `F2 is not a keyword`
  line in section 2.
