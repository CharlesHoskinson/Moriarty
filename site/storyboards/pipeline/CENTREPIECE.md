# Centrepiece: The Trace

One transaction, followed from `.mori` source to Midnight ledger acceptance.
At every stage the reader sees two things: **what the value has become** (its
representation) and **which guarantee is fixed here and which is still owed**.
A second strip beneath shows that the transaction is one of a sequence of
bounded one-transition proofs over the agreement's lifetime.

Prototype: [`prototype.html`](prototype.html). Storyboard placement: Section 5.

---

## 1. What it draws

### 1.1 The rail

Seven stations on a horizontal line, in the order fixed by the brief's
compilation pipeline:

| # | Station | Representation shown | Guarantee fixed here |
|---|---------|----------------------|----------------------|
| 1 | Source `.mori` | the `swap` action: guards, `let` arithmetic, `floor_div`, two `emit Transfer` lines; `amount(10000, AssetA_quantum)` typed `Amount<AssetA_quantum>` | none yet; what the developer stated |
| 2 | Typed elaboration and finite resource/lifetime certificate | the typing judgment; the exact intermediate integers; the certificate fields (maximum lifetime, maximum transition count, horizon `2000000000`, per-entry-point resource estimates) | admissibility: units, resource use, bounds |
| 3 | Canonical Moriarty Core | the action as explicit guards, accounting writes and effects; every datum classified `public / private / committed / revealed`; the decreasing structural measure; the local evaluator's verdict | semantic validity (Core step relation) |
| 4 | Generated Compact and correspondence manifest | the eleven manifest fields; the rule that `disclose()` is generated only from an explicit source visibility transition; translation validation | Moriarty-to-Compact translation validity |
| 5 | compactc: ZKIR 3, generated TypeScript, proving and verifier artifacts | the deferral decision; the witness rule (every witness result constrained); backend artifact hashes recorded | Compact-to-ZKIR compiler correctness; circuit/proof correctness (pinned toolchain) |
| 6 | Proof of one transition | the four mandatory claims by name and order; sequence number; predecessor binding | ContractInvariant, IntentRefinement, TransitionValidity, HistoryCompliance |
| 7 | Midnight ledger and wallet | sealed fields; mutable public state (phase, sequence number, commitments, continuation roots); the semantic signing summary (gross debit, net receipt, recipients, validity) | ledger feasibility; transaction construction and wallet correctness |

A **value chip** sits on the rail at the selected station. It always reads
`in 10,000 A · out 19,743 B` for the swap (or `interest 10 USD · principal
100 USD → 100 USD` for the repayment) and never changes its number: the point
of the component is that the number does not change while its representation
does.

### 1.2 The stage panel

Below the rail, one panel for the selected station with three regions laid
out as 7 / 5 columns on desktop and stacked on narrow screens:

- **Representation** (left): monospace block, real source or real field names.
  No invented Compact or ZKIR code appears anywhere: the brief has none, so
  stages 4 and 5 show the manifest fields and the artifact list, not code.
- **Assurance ladder** (right): the six layers of the layered assurance
  vocabulary as a vertical list. Each row is in one of three states:
  `owed` (outlined, muted), `fixed here` (accent, filled marker), `fixed
  earlier` (ink, filled marker). Below the ladder, a **residual** line:
  the residual risk from the security table that survives this stage even
  when its layer is fixed (for example at stage 5: *compiler and validator bugs
  until proof coverage is complete*; at stage 7: *a compromised wallet UI can
  still deceive the user*).
- **Named failures here** (below both): the guards that can reject at this
  stage, quoted by their exact message. Stage 3 for the swap lists
  `"minimum output not met"`, `"output would empty reserve"`, `"insufficient
  synthetic balance"`, `"horizon expired"`, `"trader authority required"`,
  `"final allowance is reserved for closure"`. Stage 7 lists the wallet's
  recomputation checks (Core hash, entry point, public effects, destinations,
  fees, versions, artifact hashes).

### 1.3 The exact arithmetic (stage 2)

This is the block that carries the brand. It is integer arithmetic on the
brief's stated formula and numbers, shown in full:

```
effective_input   = 10,000 × 997                       =              9,970,000
numerator         = 9,970,000 × 2,000,000              =     19,940,000,000,000
denominator       = 1,000,000 × 1,000 + 9,970,000      =          1,009,970,000
output_calculated = floor_div(numerator, denominator)  =                 19,743
remainder         = 19,940,000,000,000 − 19,743 × 1,009,970,000 = 162,290,000
                    "unpaid output remains in reserve_b"
reserve_a after   = 1,000,000 + 10,000                 =              1,010,000
reserve_b after   = 2,000,000 − 19,743                 =              1,980,257
```

The remainder line links to the `policy swap_output` block, which is the
declaration that made `floor` a named decision rather than an accident.

### 1.4 The failure path

An **input control** above the rail sets `min_out` to `19,743` (default) or
`19,744`. With `19,744`, the trace still passes stages 1 and 2 (the program is
admissible; the arithmetic is the same) and stops at stage 3 with the named
rejection `"minimum output not met"`. Stations 4 to 7 change to the state
`not reached`, and the panel for each reads: *No proof is generated for a
rejected transition and nothing is submitted. The guard is part of Core, so it
is also part of what the generated Compact constrains.* The value chip shows
`rejected at guard`.

For the repayment example the control sets `payment` to `10 USD` (accepted)
or `11 USD` (rejected by `requires payment <= pre.interest`). The accepted
path shows `ensures post.principal == pre.principal` holding at stage 3 with
principal `100 USD` before and after.

### 1.5 The lifetime strip: one agreement, many proofs

Under the stage panel, a horizontal strip of small boxes representing the
agreement's transitions in sequence. Each box is one bounded transition with
its own proof; a thin arrow labelled `HistoryCompliance` connects each box to
its predecessor. The swap being traced is the highlighted box. The final box
is `closure`, tied to the guard `"final allowance is reserved for closure"`.
A caption:

> A long-lived agreement is a sequence of bounded one-transition proofs, not
> one circuit that executes an entire lifetime. Each transition proves its own
> four claims and that the history it extends is itself compliant. A valid
> proof against an invalid predecessor is rejected.

The strip is drawn from the agreement's declared `lifetime` in the full site;
the prototype draws the traced transition, a predecessor, an elided run of
further bounded transitions and closure, since the brief fixes the bound's
existence but not a number for it. No transition is ever drawn as "proved" in
a green or ticked state; boxes carry the claim names only.

---

## 2. Data it reads

From the typed data modules (`site/src/data/`), one record per station:

```ts
type Stage = {
  id: 'source'|'elaboration'|'core'|'compact'|'zkir'|'proof'|'ledger';
  title: string;                // both modes
  claim: string;                // one sentence
  representation: { build: string; verify: string };   // monospace text
  fixed: AssuranceLayer[];      // layers fixed at this stage
  residual: string;             // from the security table
  failures: string[];           // exact guard messages that can fire here
  note?: { deferred: string; chosen: string };         // stage 5 only
};
type AssuranceLayer =
  'semantic' | 'translation' | 'compiler' | 'circuit' | 'ledger' | 'wallet';
type Example = {
  id: 'swap'|'repayment';
  chip: string;                 // "in 10,000 A · out 19,743 B"
  inputs: { label: string; accepted: string; rejected: string; guard: string; rejectAt: Stage['id'] };
  arithmetic: string[];         // stage-2 lines, integers only
  lifetime: number;             // transition cap
};
```

Every string in these records is a quotation from `CONTENT-SPEC.md` or
`CATEGORY-TABS.md` or integer arithmetic on numbers quoted there. The data
module is the single source; the SVG rail, the stage panel and the lifetime
strip all render from it.

---

## 3. States and transitions

| State | Meaning | Visual |
|---|---|---|
| `resting` | page loaded, no stage selected by scroll or click | station 1 selected, chip at station 1, panel for station 1 |
| `selected(k)` | station k is current | chip at k, rail segments 1..k in ink, k+1..7 in hairline, panel for k |
| `rejected(k)` | input is the rejecting value and k ≥ rejectAt | chip reads `rejected at guard`; stations after rejectAt shown `not reached` |
| `running` | the Run control is active | advances `selected(k)` by one station every 1,100 ms until 7 or until the rejecting station |
| `example switched` | example changed | chip text, arithmetic, failures and lifetime strip re-render; stage stays |

Transitions:

- `selected(k) → selected(k±1)`: chip slides along the rail (transform only,
  380 ms, `cubic-bezier(0.16, 1, 0.3, 1)`); panel cross-fades 160 ms; ladder
  rows that change state animate their marker fill 200 ms.
- `running`: Space or the Run button starts it; any user selection, Escape,
  or losing focus stops it.
- In the full site the section is pinned and `selected(k)` is driven by scroll
  progress through the pinned range (an `IntersectionObserver` over seven
  sentinel elements, never a scroll listener). Direct selection by click or
  key overrides scroll until the next scroll event.

---

## 4. Interaction model

- **Rail stations** are a `role="tablist"` with seven `role="tab"` buttons.
  Click selects. The stage panel is `role="tabpanel"`, `aria-labelledby` the
  active tab.
- **Example** and **input** controls are radio groups with visible labels.
- **Run** is a toggle button (`aria-pressed`).
- **Previous / Next** buttons flank the rail for pointer users who prefer
  stepping.
- **Ladder rows** are focusable; each has a `title` giving the layer's
  long name and the residual risk.
- **Lifetime boxes** are focusable and announce `transition n of lifetime;
  four claims; predecessor bound by HistoryCompliance`.
- Links out: stage 1 links to Section 7 (the source); stage 6 links to
  Section 6 (the four claims); the remainder line links to the `policy`
  block.

## 5. Keyboard and screen reader

- `Tab` moves between: example radios, input radios, Run, the rail (one tab
  stop; roving `tabindex`), Previous, Next, the panel's focusable rows, the
  lifetime strip.
- In the rail: `←/→` move a station, `Home/End` jump to first/last, `Enter` or
  `Space` on a focused station selects it (selection also follows focus, so
  arrows alone are enough).
- `Escape` stops a running trace.
- An `aria-live="polite"` region announces on every change: *Stage 3 of 7,
  Canonical Moriarty Core. Fixed here: semantic validity. Still owed: five
  layers.* On rejection: *Rejected at stage 3 by guard "minimum output not
  met". Stages 4 to 7 not reached.*
- All rail text is real SVG or HTML text, not paths. Every marker state is
  also carried by a visible word (`fixed here`, `owed`, `not reached`), never
  by colour alone.
- Contrast: ink on paper and paper on ink both exceed 12:1; the accent on
  either background exceeds 4.5:1; muted text is held at 4.6:1 or better.

## 6. At 400px

- The rail becomes a horizontally scrolling row of seven compact station chips
  inside its own `overflow-x: auto` container with scroll snapping; the page
  body never scrolls sideways. The selected chip is scrolled into view when
  selection changes.
- The value chip moves from the rail into the panel header so the number stays
  visible.
- The stage panel stacks: representation, then ladder, then failures. The
  arithmetic block gets its own horizontal scroll container so the
  fourteen-digit numerator is never wrapped or truncated.
- The lifetime strip wraps to two rows; arrows become short vertical ticks.
- Previous / Next move beneath the rail as two full-width buttons.
- No pinning; the section flows normally.

## 7. `prefers-reduced-motion`

- The chip does not slide; it is placed. Panels swap without cross-fade.
  Marker fills change instantly.
- Run remains available but performs a single jump to the terminal station
  (7, or the rejecting station) rather than a timed walk, so no content
  changes without a user action.
- No scroll pinning; the section is a normal block, and stations are selected
  only by tap, click or key.

## 8. Why this form

**Why a rail and not a flowchart.** A flowchart of the pipeline already exists
in the brief as a code block. Redrawing it as boxes and arrows adds nothing.
The rail exists to carry one value across seven representations so the reader
can check for themselves that 19,743 stays 19,743 while the type, the encoding
and the guarantee change around it. That is the argument in the tagline
(*meaning that survives compilation, proof and settlement*) made testable.

**Why the assurance ladder is inside the trace.** The brief calls collapsing
the layers "the standard marketing lie in this space". The most convincing
way to not collapse them is to show, at each stage, the five that are not yet
established. A separate guarantees diagram (Section 6) summarizes it; the
trace is where the reader feels it.

**Why the failure path is a first-class input.** Asking for 19,744 and
getting a named rejection instead of a silent adjustment is the single most
persuasive fact in the brief for a protocol engineer. Making it a toggle
rather than a footnote lets the reader produce the rejection and see where in
the pipeline it happens (Core, before any proof is attempted) and why it
cannot be bypassed (the guard is part of what is compiled).

**Why the lifetime strip is a second element rather than a mode of the
rail.** The rail is one transition deep; the architectural idea is that an
agreement is many such transitions, each separately proved and chained by
HistoryCompliance. Putting the sequence beneath the rail, with the traced
swap highlighted as one box among several, shows both scales at once without
zooming the reader in and out.

**Why not animate the number.** Counting-up numbers imply approximation.
The value is exact and it is fixed; the only things that move are the chip's
position and the ladder's state.

**Why not a 3D or isometric stack.** It would look like a product hero and
it would hide the text, and the text (guard messages, manifest fields, claim
names) is the content.

**What was rejected.** A vertical scrollytelling column with prose on the
left and a changing figure on the right (too much reading before the number
appears); a single pinned SVG that morphs source into circuit (the brief has
no ZKIR sample to morph into, and inventing one is forbidden); a comparison of
Moriarty against Compact-only development (no measured numbers exist for it).
