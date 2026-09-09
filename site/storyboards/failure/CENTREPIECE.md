# Centrepiece: the Fork

One pre-state. One action. Two post-states. The left post-state is what a
plausible, balance-based implementation produces. The right is what the
distinguishing test requires: a preserved value, or a named rejection. The
line where the two columns differ is the guarantee, and it is the only line
drawn in colour.

The Fork renders all 24 action targets, grouped by the eight category tabs,
from one data module. Two of the 24 cases are live: DA01 computes the real
`19,743` from the real formula with the reader's `min_out`, and DA06 runs the
real `PartialPayment` action with the reader's payment. The other 22 are
structural: the pre-state and the two outcomes are written as state lines,
not as numbers, because the brief gives no numbers for them and none will be
invented.

A second view of the same component, the **field**, draws the composition
result: `1,830` marks, `185` in rust, partitioned within / across categories.
It appears at the foot of the ⊥ tab and stands alone in section 4.

---

## Why this form

The most persuasive material the project has is a list of 24 sentences of the
form "X must not do Y". A list of sentences is not a visualization, and the
obvious alternatives all lose something:

- **A grid of 24 cards** flattens the tests to captions. The reader sees
  breadth and no depth; nothing distinguishes DA06 from DA03.
- **An exploit timeline** (pre-state, attacker step, loss) makes the site a
  security-scare page and would require inventing attacks the brief does not
  contain.
- **A checklist with green ticks** is exactly the "invented verified badge"
  CONTENT-SPEC §10 forbids.
- **A single hero calculator** (the swap alone) is honest but covers one of
  24 targets and says nothing about obligations, which is the thesis.

The Fork keeps what the tests actually are: a *pair of answers* to the same
question. Showing both answers side by side, with the pre-state shared, makes
the reader do the comparison rather than be told the conclusion. It also
scales: 22 structural cases and 2 numeric cases share one layout, so the
section reads as one instrument rather than 24 illustrations. And it is a
shape the reader recognises from a ledger or a diff: left and right, one
hairline apart.

The plausible column is set in mid-grey, never red. The wrong answer is not
dangerous-looking; it is *less defined*. That is the actual claim: the
balance-based model does not have a place to write the allocation, the claim
identity, the pending state or the exercised duty, so those lines are shown
as `undefined` or `not represented`, in grey. The rejection ink (rust) is used
only on the required side, for a named guard that fires.

---

## What it draws

```
┌ tab strip (8) ───────────────────────────────────────────────────────┐
│ F1 3 · F2 7 · F3 2 · F4 2 · F5 1 · F6 3 · P 1+1 · ⊥ 5                │
├─ case index ─────────┬─ fork ────────────────────────────────────────┤
│ DA04 supply/redeem   │ DA06  borrow / accrue / repay                 │
│   insufficient …     │ requirement: nominal debt distinct from …     │
│ DA05 post/release    │ test: partial repayment preserves principal … │
│ ▸ DA06 borrow/repay  │                                               │
│   partial repayment… │ PRE-STATE      PLAUSIBLE        REQUIRED      │
│ DA07 liquidate       │ principal 100  balance 110-p    interest 10-p │
│ …                    │ interest  10   allocation: ?    principal 100 │
│                      │ payment   [4]                   ensures …     │
│                      │                                               │
│                      │ enforced by  Debt<USD> ≠ Amount<USD>; ensures │
└──────────────────────┴───────────────────────────────────────────────┘
```

**Tab strip.** Eight tabs, identifier + short name + count. `P` reads
`1 + 1`; a computed line beneath the strip reads
`24 action targets · DA12 shared by F3 and P`.

**Case index.** The tab's action targets as a vertical radiogroup. Each item:
identifier in the monospace, the action, and the distinguishing test in
emphasis (Build) or the semantic requirement in emphasis (Verify). DA12
carries a `shared` badge in both F3 and P.

**Fork panel.** Header: identifier, action, family tags. Two rows:
`requirement` and `test`, verbatim from the matrix. Then three columns:

- **PRE-STATE.** State lines. For DA01 and DA06, one line is an input.
- **PLAUSIBLE.** What a balance-based implementation writes after the action.
  Lines that the model cannot represent are set as `undefined` or
  `not represented`, in grey.
- **REQUIRED.** What the distinguishing test demands. Ends in an outcome
  line: either `preserved: <what>` in the accent, or `rejects: "<named
  failure>"` in rust.

The line at which the two columns first differ is marked with a hairline
bracket spanning both columns and is the only coloured element besides the
outcome line.

Footer row: `enforced by`, naming the construct (Build: type, guard, policy,
proof claim; Verify: the specification layer that carries it).

**Live cases.**

- *DA01.* Pre-state carries `reserve_a 1,000,000`, `reserve_b 2,000,000`,
  `amount_in 10,000`, `fee 997/1000`, and `min_out` as an input (default
  `19,744`, the brief's own failing request). Required column computes
  `output_calculated = floor((10,000·997·2,000,000) / (1,000,000·1,000 +
  10,000·997)) = floor(19,940,000,000,000 / 1,009,970,000) = 19,743`,
  shows the discarded remainder `162,290,000 / 1,009,970,000` with the policy
  text "unpaid output remains in reserve_b", then evaluates the two guards by
  name. `min_out ≤ 19,743` passes; otherwise `rejects: "minimum output not
  met"`. `19,743 < 2,000,000` passes the reserve guard. The plausible column
  shows the same formula with the rounding direction unstated and the reserve
  guard absent, and for an unmet minimum reads `delivered: adjusted to
  output` in grey. Arithmetic is exact (BigInt).
- *DA06.* Pre-state carries `principal 100`, `interest 10` and `payment` as
  an input (default `4`). Required column evaluates `requires payment >
  debt(0, USD)` and `requires payment <= pre.interest` by name; if both hold
  it writes `next.interest = 10 - p` and checks `ensures post.principal ==
  pre.principal` (always `100`). If `p > 10` the outcome is `rejects:
  "requires payment <= pre.interest"`. The plausible column writes `balance
  110 - p` and `principal: undefined (a residue of subtraction order)`; for
  `p > 10` it continues happily and reads `principal silently reduced by
  p - 10`, which is the bug.

**The field.** An SVG of `1,830` 1-unit squares in two blocks. View *by
category*: block A `143` within-category pairs with `3` failures; block B
`1,687` cross-category pairs with `182` failures. View *by result*: block A
`1,645` clean; block B `185` failed, with the `3` within-category failures
distinguishable by a lighter rust. Captions carry the exact counts and rates:
`2.10%`, `10.79%`, `5.14×`. A derivation line states that `143` and `1,687`
are implied by `3 / 2.10%` and `182 / 10.79%` and sum to `1,830`, and that
marks are placed by count, not by pair identity. Failures are spread at even
intervals so that no two adjacent marks suggest a real pair.

---

## Data it reads

- `actionTargets[24]`: `id`, `families[]`, `action`, `requirement`, `test`.
- `forkCases[24]`: `pre[]`, `plausible[]`, `required[]`, `outcome {kind:
  preserve | reject, text}`, `enforced`, `layer`, `live?: "swap" | "loan"`.
  Every string is either verbatim from the three brief files or a state line
  derived from the target's own test sentence; the module carries a `source`
  field per case naming the CATEGORY-TABS block it derives from.
- `tabs[8]`: identifier, name, `targetIds[]`; DA12 appears in two.
- `compositionResult`: `{eligible: 1830, clean: 1645, failed: 185, within:
  {pairs: 143, failed: 3, rate: "2.10%"}, cross: {pairs: 1687, failed: 182,
  rate: "10.79%"}, ratio: "5.14×"}`.
- `swapInstance`: `{reserveA: 1000000n, reserveB: 2000000n, amountIn: 10000n,
  feeNum: 997n, feeDen: 1000n}`.
- `loanInstance`: `{principal: 100, interest: 10}`.

The counts `24`, `8`, `5` and `4` are asserted at module load; a mismatch
throws.

---

## States and transitions

1. **Idle.** A tab is active and its first case is selected. The Fork shows
   that case fully resolved. There is no empty state: the component never
   renders without a selected case.
2. **Case change** (index click, arrow key, or an inbound link from
   section 8). The pre-state column updates immediately. The two post-state
   columns clear and refill: 240ms, opacity and a 6px rise, plausible first,
   required 80ms later. The ordering is the explanation: the reader sees the
   naive answer land, then the required one. Under reduced motion both
   columns replace instantly.
3. **Tab change.** Tab strip updates, case index re-renders, the tab's first
   case is selected and state 2 runs. Scroll position is held.
4. **Live input.** Every keystroke re-evaluates. The outcome line changes
   colour with a 120ms transition on `color` only; no layout shift. Invalid
   or empty input is treated as `0` and shown as such, which for DA06 fires
   `requires payment > debt(0, USD)` by name rather than showing an error
   toast.
5. **Mode change** (Build / Verify, from the app root). The case index swaps
   its emphasised line; the Fork adds or removes the `obligation` row;
   `enforced by` re-words. 120ms cross-fade; instant under reduced motion.
6. **Field entry.** On first intersection (30% visible) the failures draw in
   over 900ms with a per-mark delay. Once. Under reduced motion they are
   present on load.

---

## Interaction model

- Tab strip: pointer, or keyboard within a `role="tablist"`.
- Case index: pointer, or keyboard within a `role="radiogroup"`.
- Live inputs: standard `<input type="number">` with `inputmode="numeric"`,
  labelled, `step="1"`, `min="0"`.
- Field toggle: two-segment control, `by category` / `by result`.
- Inbound deep link: `#tests/DA21` selects the ⊥ tab and the DA21 case; the
  four mechanisms in section 8 link this way.
- Hover on any `undefined` / `not represented` line shows one sentence: "A
  balance-based model has no field for this." No other tooltips.

## Keyboard and screen reader

- Tab strip: roving `tabindex`; `ArrowLeft` / `ArrowRight` move and activate
  (automatic activation, since the panel change is cheap); `Home` / `End`.
  Each tab has `aria-controls` pointing at the case index and `aria-selected`.
- Case index: roving `tabindex`; `ArrowUp` / `ArrowDown` move and select;
  `Home` / `End`. Each item is `role="radio"` with `aria-checked`. The
  emphasised test line is part of the accessible name, so a screen reader
  hears "DA06, borrow / accrue / repay, partial repayment preserves principal
  and interest allocation, 3 of 7".
- Fork panel: `role="region"`, `aria-labelledby` the case header, and
  `aria-live="polite"` on the outcome line only, so a live input announces
  "rejects: minimum output not met" without re-reading the whole panel.
- The three columns are real `<dl>` lists under `<h4>` headings, in DOM order
  PRE-STATE, PLAUSIBLE, REQUIRED, so linear reading matches visual reading.
  The differing-line bracket is decorative and `aria-hidden`; the same fact
  is conveyed by the outcome line text.
- The field: the SVG has `role="img"` and an `aria-label` carrying all eleven
  numbers in a sentence; the marks are `aria-hidden`. The toggle is a
  `radiogroup`.
- Focus rings are 2px, the accent, offset 2px, on every interactive element,
  in both themes.

## At 400px

- The tab strip becomes a single horizontally scrolling row with
  `scroll-snap-type: x proximity`; the active tab scrolls into view on
  selection. Identifier and count stay visible; the short name may truncate
  with an ellipsis but never the identifier.
- Case index and Fork stack: index above, Fork below. The index collapses to
  a single-line-per-case list; the emphasised test line wraps.
- The three Fork columns stack vertically in DOM order, each with its heading,
  separated by hairlines. The differing-line bracket becomes a 2px left rule
  on the differing lines of both columns.
- Live inputs are full-width, 44px tall.
- The field's SVG scales with `viewBox`; below 480px the two blocks stack
  vertically inside the SVG so marks stay at least 3px. Captions move under
  the figure. The page body never scrolls sideways; only the tab strip does.

## `prefers-reduced-motion`

All transitions are declared once under `@media (prefers-reduced-motion:
no-preference)`. Outside that query: no column refill animation (columns
replace), no colour transition on the outcome line, no field draw-in
(failures present on load), no tab cross-fade. Nothing on the page loops in
either case, so there is nothing to pause.

## Theme

Tokens on `:root` for light; redefined under `prefers-color-scheme: dark`
guarded by `:root:not([data-theme="light"])`, and again under
`:root[data-theme="dark"]` so an explicit choice wins. The accent green and
the rust ink are re-tuned for dark so both keep at least 4.5:1 against the
panel background. The grey used for `undefined` lines is checked to 4.5:1 in
both themes: "less defined" must still be legible.

## Failure modes of the design, named

- If a future case has no honest "plausible" column (the naive model gives
  the same answer), the case must say so: `same as required` in grey, with
  the test then carried by the pre-state alone. Do not invent a wrong
  answer to fill the column.
- If the counts drift from `24 / 8 / 5 / 4`, the component throws at load.
  The completeness line under the tab strip is computed, never typed.
- The field must never be captioned with a rounded ratio. `5.14×`, and the
  four exact counts, or nothing.
