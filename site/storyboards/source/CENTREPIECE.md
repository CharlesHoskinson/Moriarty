# Centrepiece: Marginalia

The source reader. Real `.mori` files are the primary visual element of the
site; the margin next to them carries the meaning. A reader who has never seen
Moriarty reads `ensures post.principal == pre.principal;` and understands the
guarantee before any prose explains it, because the guarantee is the line.

Prototype: `prototype.html` in this directory. It runs standalone, with no
network requests, and is the reference for everything below.

## 1. What it draws

A single reading surface, laid out as a page of annotated code:

```
+-----------+--------------------------------------------+------------------+
| files     | path · profile           [lens chips]      |                  |
|           +--------------------------------------------+   margin         |
| partial-  |  1 │ profile "moriarty-successor-syntax/0";|                  |
| payment   |  2 │                                       |  ENSURES  line 13|
| funded-   |  3 │ agreement PartialPayment {            |  The thesis.     |
| partial-  |  4 │   unit USD;                           |  Paying interest |
| payment   | .. │   ...                                 |  must not touch  |
| swap      | 13 │   ensures post.principal == pre.pr.. |  principal ...   |
| loan      | 14 │   }                                   |                  |
| profile   | 15 │ }                                     |                  |
|   diff    +--------------------------------------------+                  |
|           | [run panel, swap.mori only]                |                  |
+-----------+--------------------------------------------+------------------+
```

Three columns on desktop: a file list (200px), the code pane (fluid), and the
margin (300px). Above the code, the file's repository path, its profile string,
and six lens chips.

The five files are:

| Tab | Source | Profile | Shown |
|---|---|---|---|
| partial-payment.mori | `experiments/moriarty-language/spec/successor/examples/partial-payment.mori` | `moriarty-successor-syntax/0` | all 15 lines |
| funded-partial-payment.mori | `.../spec/successor/examples/funded-partial-payment.mori` | `moriarty-successor-syntax/0` | all 18 lines |
| swap.mori | `.../spec/examples/swap.mori` | `moriarty-bounded-atomic/1` | lines 1 to 3, 9 to 11, 14 to 16, 26, 65 to 90 |
| loan.mori | `.../spec/examples/loan.mori` | `moriarty-bounded-atomic/1` | lines 1 to 4, 15, 28 to 35, 73 to 84, 108 to 109 |
| profile diff | partial-payment.mori against loan.mori, plus both grammars | both | five construct rows |

Every line is verbatim. Line numbers are the file's own. Elided ranges are
shown as a quiet row ("… 38 lines not shown (27 to 64)") so the reader can see
that something was cut and exactly how much. Nothing is paraphrased,
reformatted or shortened inside a line.

### Syntax highlighting that carries meaning

The highlighter has six token classes and each one is a semantic claim, not a
decoration:

- **keyword** (accent, bold): `guard`, `requires`, `ensures`, `policy`,
  `lifetime`, `horizon`, `emit`, `set`, `next`, the literal constructors
  `uint`, `text`, `amount`, `debt`, and `floor_div`. The reader's eye lands on
  the words that make promises.
- **type or unit** (ink, underlined when it takes a type argument): `Debt`,
  `Amount`, `UInt128`, `USD`, `AssetB_quantum`, `Transfer`, `Payer`. The
  underline under `Debt<USD>` and `Amount<Cash>` is the visual carrier of
  "these are distinct types".
- **namespace** (muted): `pre.`, `post.`, `next.`, `state.`, `arg.`, `obs.`,
  `const.`. Muting them makes the field name read first while keeping the
  provenance visible: which state, before or after.
- **guard message** (failure colour): the string in a `guard` statement. Every
  named failure is red at the point it is written, so a reader scanning the
  swap action sees fourteen named rejections before reading one condition.
- **string** (secondary ink): all other strings, including the six policy
  fields.
- **number** (ink, tabular): exact integers, never grouped inside code.

Thesis lines (`ensures` in the partial payment, `policy` and the closure guard
in the loan, `action pay(...)` in the funded file, the slippage guard in the
swap) are set in a heavier weight. That is the only weight change in the pane.

## 2. The data it reads

A single data module (`site/src/data/sources.ts` in the application; the
`FILES` object in the prototype) with, per file:

- `path`, `profile`, and a `status` sentence per mode stating what that profile
  actually establishes (the successor profile parses and formats; the atomic
  profile runs);
- `segments`: `{ start, lines[] }`, verbatim text with real start line numbers;
- `thesis`: line numbers set in the heavier weight;
- `notes`: keyed by line number, each with `lens`, `title`, optional `calc`
  (a small monospace arithmetic block), and a `build` and a `verify` text.

The swap file additionally declares `run: true`, which mounts the run panel.
The evaluation reads the file's own constants and initial state (fee 997/1000,
reserves 1,000,000 A and 2,000,000 B, trader balance 100,000 A, expected output
19,743, expected reserves 1,010,000 and 1,980,257, lifetime 8, horizon
2,000,000,000) and evaluates the action's statements in source order with
BigInt arithmetic.

Forty-three annotated lines across the four files (10, 4, 16 and 13); every
note traces to
CONTENT-SPEC, CATEGORY-TABS, the two grammars, `lexical.md`,
`syntax-profile.json`, `semantics.md`, `funded-source.md` or
`repayment-kernel.md`.

## 3. Lenses

Six chips: All, Types, Guards, Policy, Bounds, Pre / Post. Each chip shows the
count of annotated lines it selects in the current file, so the reader sees at a
glance that the partial payment has five type lines and zero policy lines, and
the loan has eight policy lines. Selecting a lens dims every line it does not
select to 42% opacity and puts the lens's explanatory paragraph in the margin.
The lens is progressive disclosure done with the source itself: the same file,
read five ways, and the code never moves.

The lens definitions are the five ideas the brief asked the centrepiece to
carry: `Debt<USD>` as a type distinct from `Amount<USD>` (Types); named failures
rather than silent adjustments (Guards); rounding as a declared artifact
(Policy); `lifetime` and `horizon` as bounds on the agreement itself (Bounds);
and `ensures post.principal == pre.principal` (Pre / Post).

## 4. The margin

The margin column holds one card at a time:

- **Nothing hovered or pinned, lens All**: "What this profile establishes",
  with the profile string and the honest status sentence for the current mode.
  This is where the site says, on the hero surface, that the successor profile
  does not execute a payment yet.
- **A lens active**: the lens paragraph and the count.
- **A line hovered, focused or pinned**: that line's note, positioned level with
  the line. The card's `top` is set from the line's offset; on hover it slides
  (180ms) between lines, and under `prefers-reduced-motion` it jumps.

Each note has a small-caps title, the line reference in the accent colour, an
optional arithmetic block, and one paragraph. The arithmetic blocks are where
the surface earns trust: the loan's remainder line says "discard 54/73
micro-USD" and the note shows 1,240,000,000,000 / 36,500 = 33,972,602 remainder
27,000, and 27,000 / 36,500 = 54/73. The policy's free-text string turns out to
be exactly checkable, and the reader checks it.

## 5. The run panel (swap.mori only)

Three inputs, `obs.now`, `arg.amount_in` and `arg.min_out`, prefilled with
1700000000, 10000 and 19743. Everything else is fixed by the file and the panel
says so. On every keystroke the action is re-evaluated in source order:

- guard lines show `holds` in the gutter, or `REJECT` on the first false guard,
  and `not reached` below it;
- `let` and `set` lines show the exact value they bound or wrote (9,970,000;
  19,940,000,000,000; 1,009,970,000; 19,743; then 90,000; 1,010,000;
  1,980,257; 19,743);
- `emit` lines show the transfer amount and asset.

The result strip reads either

> Complete. Output **19,743** ASSET_B for **10,000** ASSET_A. Reserves 1,010,000
> A and 1,980,257 B. `remaining` 8 → 7.

or

> Rejected at line 80: `GUARD_FAILED` "minimum output not met"

with the rejected line tinted in the failure colour. Type 19744 and the
slippage guard fires. Type 10001 and line 79 fires with "independent output
value mismatch", which teaches something the prose cannot: the example file is
a frozen worked instance whose author pinned the answer, not a general pool.
Set `obs.now` to 2000000000 and the horizon guard fires.

Inputs are validated as canonical unsigned decimal tokens (no sign, no leading
zero, no decimal point), which is the language's own integer rule.

The panel is labelled as the site's own reading of the source in source order,
not the repository evaluator. Verify mode says so in the status sentence.

## 6. The profile diff

The fifth tab replaces the code pane with five construct rows, successor on the
left and atomic on the right, each row a real line with its file and line
number:

| Row | successor | atomic |
|---|---|---|
| Precondition | `requires payment <= pre.interest;` (line 11) | `guard state.borrower_cash >= arg.amount_due, "insufficient synthetic balance";` (loan line 96) |
| State write | `next.interest = pre.interest - payment;` (12) | `set interest_due = interest_calculated;` (loan 82) |
| Postcondition | `ensures post.principal == pre.principal;` (13) | `guard state.notional == const.expected_outstanding_notional, "episode closure cannot discharge remaining notional";` (loan 108) |
| Nominal debt | `state principal: Debt<USD> = debt(100, USD);` (7) | no `Debt` type; `stored_type = "UInt128" \| "Text" \| "Amount", "<", identifier, ">"` |
| Rounding policy | not a production of this profile (grammar note 9) | the full `policy accrued_interest` block (loan 28 to 35) |

The diff is a construct correspondence, not a translation, and the margin says
so. It carries two facts the site must not blur: the successor profile is where
the language is going and the atomic profile is what runs today; and the
direction of "missing" is not uniform, since `policy` exists only in the
implemented profile.

## 7. States and transitions

| State | Entered by | Visible result |
|---|---|---|
| file selected | tab click, `#swap` hash | code pane re-renders, lens resets to All, margin shows status |
| lens active | chip click | non-lens lines dim; margin shows lens paragraph; arrow keys walk only lens lines |
| line previewed | hover or keyboard focus on a marked line | margin card moves level with the line and shows the note |
| line pinned | click, Enter or Space on a marked line; `#swap:80` hash | line gets the accent rail and wash; note stays when the pointer leaves; `aria-expanded="true"` |
| run evaluated | any input change | gutter statuses and the result strip update; rejected line tints |
| mode switched | Build / Verify | every note, status sentence, lens paragraph and diff paragraph swaps text; layout does not move |
| theme | auto / light / dark | tokens swap; persisted in `localStorage` |

Mode is one piece of state, persisted in `localStorage` under `moriarty-mode`,
exactly as PLAN.md prescribes. Switching mode never changes which line is
pinned or which file is open.

## 8. Interaction model, keyboard and screen readers

- Marked lines are real `<button>` elements; unmarked lines are inert spans.
  Tab moves through the marked lines in document order.
- Up and Down arrows move between marked lines, respecting the active lens.
  Enter or Space pins; Escape unpins.
- Each button carries `aria-label="Line 13. ensures post.principal ==
  pre.principal;. ensures"` and `aria-describedby` pointing to a visually hidden
  copy of the note for the current mode, so a screen reader hears the code, the
  title and the explanation without the margin card being announced twice. The
  margin itself is `aria-hidden`.
- `aria-expanded` reflects pinning. The file list is a `tablist`; lens chips are
  `aria-pressed` toggles; the mode switch is a labelled group of `aria-pressed`
  buttons.
- The run panel inputs have visible labels above them naming the source
  argument (`arg.min_out`) and its unit. The result strip is `aria-live="polite"`.
- Focus rings are 2px accent outlines with offset; nothing relies on colour
  alone: the rejected guard line says `REJECT` in text, and the result strip
  says "Rejected at line 80".
- Deep links: `#swap:80` opens swap.mori with line 80 pinned. The application
  will use the same scheme for section anchors so that every category tab can
  link into a specific line of a specific file.

## 9. At 400px

- The file list becomes a horizontally scrollable row of tabs above the code;
  the subtitle line under each tab is dropped.
- The margin column is removed. A pinned note renders inline under its line
  as a bordered block with the accent rail, so the code and its explanation
  stay adjacent. Hover preview does not exist on touch; tap pins.
- Code wraps with a hanging indent instead of scrolling sideways, so the note
  and the line remain in one column and the body never scrolls horizontally.
  Wrapping changes nothing in the text.
- The evaluation gutter narrows to 118px at 10.5px; the widest value,
  19,940,000,000,000, still fits without truncation. Numbers are never
  ellipsised.
- The run panel stacks its inputs one per row; the diff rows stack successor
  above atomic.
- Verified in the prototype at 400px in both themes.

## 10. Under `prefers-reduced-motion`

The only motion on the surface is the margin card sliding between lines on
hover (180ms, `top` transition) and the lens dimming (opacity). Under reduced
motion the card jumps and the dimming is instant. Nothing on the surface
animates on load, on scroll or on a timer. The run panel never animates; a
rejection is a colour and a word, not a shake.

## 11. Why this form

**Why code and not a diagram.** The site's claim is that the language is the
product. A diagram of the pipeline or the guarantee chain proves the site can
draw; twelve lines of source with a margin prove that a developer can read the
guarantee. The centrepiece has to be the thing the developer will actually
touch, and that thing is a `.mori` file.

**Why marginalia and not tooltips.** Tooltips hide the explanation behind a
hover and cannot hold a three-line arithmetic block. A margin is how annotated
texts have been read for a very long time: the source stays whole and in order,
the commentary sits beside it at the same height, and the reader controls the
pace. It also gives Build and Verify a natural place to differ: same code, two
commentaries.

**Why lenses and not a slideshow.** A staged reveal (types first, then guards,
then policy) would force one reading order. Lenses let the F2 engineer go
straight to Guards and the researcher straight to Bounds, and the counts on the
chips are themselves content: zero policy lines in the successor profile is a
true statement about the language's current state.

**Why an evaluator on one file and not all.** Only the atomic profile runs.
Simulating the successor profile would invent semantics the repository says do
not exist yet. The swap action is the file where the brief's numbers live
(19,743; 19,744 rejects), so it is the one file where interaction is honest and
teaches something the text cannot.

**Why a construct diff and not a side-by-side of two whole files.** Whole files
invite the reader to assume the profiles are two spellings of one thing. The
construct rows show correspondence where it exists, absence where it does not,
and that the absence runs both ways.

**What was rejected.** A typewriter reveal of the source (motion as decoration);
an editor with a live parser (the successor parser is a Node CLI and the site
ships no backend, and a fake "syntax OK" badge would be an invented verified
state); a graph of guards to failure codes (moves the reader off the source);
a single-file hero with only the partial payment (fifteen lines are the thesis
but the policy block and the run panel are the proof that the thesis is
implemented somewhere).

## 12. Implementation notes for the application

- Hand-authored DOM, not SVG: the surface is text and must be selectable,
  wrappable and screen-readable. No charting or code-editor dependency.
- The highlighter is a forty-line tokenizer over the two keyword sets. It must
  not be replaced by a general-purpose grammar package, which would colour
  `pre` and `post` as keywords (they are identifiers) and miss the guard-message
  class.
- Sources are stored as verbatim arrays with real start lines. A build-time
  check should compare each segment to the file at the recorded path and fail
  the build on drift, so the site can keep saying "nothing on this surface is
  paraphrased".
- The run panel's evaluator is 40 lines of BigInt over the file's constants.
  It is labelled as the site's own reading in source order; when the repository
  evaluator becomes callable from the browser, the label changes, not the UI.
