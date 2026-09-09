# Centrepiece — The Register

Angle: **the category surface.** The DeFi category model itself is the hero:
seven economic families plus the cross-cutting tab, eight mandatory facets, and
all twenty-four action targets DA01–DA24, drawn as one object a reader can
stand on, move across, and measure.

The object is called **the Register**. It is a ledger-shaped table: eight
columns (F1 · F2 · F3 · F4 · F5 · F6 · P · ⊥, never reordered), a top rail
holding the twenty-four action targets, and eight facet rows beneath. Every
tab, every target and every facet placement has one fixed position on it, and
the reader's own reading marks fill it in as they go. The eight category tabs
are not a separate widget — they are the Register's columns, and the six-block
tab body renders directly beneath the Register in the column's colour.

---

## 1. What it draws

```
              F1        F2 (7)              F3     F4     F5   F6       P      ⊥ (5)
targets      ▢▢▢    ▢▢▢▢▢▢▢          ▢▢     ▢▢     ▢    ▢▢▢     ▢◌     ▢▢▢▢▢
Execution     ·         ·                ·      ·      ·     ·       ·      DA21
Settlement    ·         ·                ·      ·      ·     ·       ·      DA23
Custody       ·         ·                ·      ·      ·     ·       ·        –
Legal dep.    ·         ·                ·      ·      ·     ·       ·        –
Collateral    ·         ·                ·      ·      ·     ·       ·        –
Oracles       ·         ·                ·      ·      ·     ·       ·      DA20
Authorization ·         ·                ·      ·      ·     ·       ·    DA21 DA22
Price disc.   ·         ·                ·      ·      ·     ·       ·      DA20
foot         targets read 0 / 24 · tabs opened 0 / 8 · facets compared 0 / 8
```

**Columns.** One per tab, in the fixed order F1, F2, F3, F4, F5, F6, P, ⊥.
Column width is proportional to the number of action targets the tab owns
(3 · 7 · 2 · 2 · 1 · 3 · 2 · 5, where P's 2 is one own plus one shared), with a
minimum width so F5's single-target column stays readable. The width *is*
information: the reader sees at a glance that credit is the largest surface and
that the cross-cutting tab is the second largest, before reading a word.

**The target rail (row 0).** Twenty-five chips for twenty-four targets. Each
chip carries its identifier (`DA06`) and, on hover, focus or narrow layouts, its
action (`borrow / accrue / repay`). DA12 is drawn twice — solid in F3, outlined
and labelled *shared* in P — and the two chips light together on hover or
focus. The coverage count treats DA12 as one target so the foot always reads
out of 24.

**The facet grid (rows 1–8).** Eight rows in the fixed facet order: Execution,
Settlement, Custody, Legal dependence, Collateral and solvency, Oracles,
Authorization and mandate, Price discovery. Each of the 64 cells carries the
short form of that family's facet placement from CATEGORY-TABS (for example
F5 × Legal dependence: *maximal*; F1 × Oracles: *usually none — the pool is the
price*). The full sentence lives in the tab body's facet-profile block and in
the facet lens panel; the cell is the index, not the prose.

The ⊥ column's facet cells are different in kind and are drawn that way. The
cross-cutting targets do not *sit on* facets — they *instrument* them. The
brief fixes two placements explicitly (intents are an execution facet; bridges
and messaging are a settlement facet) and the ⊥ tab fixes DA20 as observation
and DA21/DA22 as authority and governance. So ⊥ × Execution reads `DA21`,
⊥ × Settlement reads `DA23`, ⊥ × Oracles and ⊥ × Price discovery read `DA20`,
⊥ × Authorization reads `DA21 DA22`, and the remaining three cells read
*instantiated by the seven families*. Nothing is invented: those cells state
that the honest answer is "not applicable to a cross-cutting tab", which the
completeness check explicitly allows.

**The foot.** Three exact counters: `targets read n / 24`, `tabs opened n / 8`,
`facets compared n / 8`. They are integers, never percentages and never a bar
that could be read as a verification state.

## 2. Data it reads

The Register is rendered from the data modules PLAN.md prescribes, and nothing
else:

- `families` — id, label, targets owned, function, boundary, what-goes-wrong,
  what-Moriarty-does, code samples, reference sources (CATEGORY-TABS, one
  record per tab).
- `facets` — the eight names in order (CONTENT-SPEC §2.2).
- `facetProfile[family][facet]` — short cell text and full sentence
  (CATEGORY-TABS facet-profile paragraphs, split by the `·` separators).
- `actionTargets` — DA01–DA24 with family tags, action, semantic requirement,
  distinguishing test (CONTENT-SPEC §2.3); DA12 carries `F3;P`.

The prototype embeds these as one JavaScript object. In the application they
become `site/src/data/*.ts` and the Register is hand-authored DOM/SVG driven
from them — no charting dependency and no force layout are needed.

## 3. States

The Register has one **lens** and one **cursor**, plus the reader's **marks**.

| State | Values | Meaning |
|---|---|---|
| `lens` | `family` (default) · `facet` | Which axis the panel beneath is organised by |
| `column` | F1 … ⊥ | The open tab; the standing column |
| `facet` | 1 … 8 or none | The lifted facet row, when `lens = facet` |
| `target` | DA01 … DA24 or none | The focused chip; the entry the tab body is scrolled to |
| `marks.read` | set of target ids | Targets whose entry the reader has actually read |
| `marks.opened` | set of tab ids | Tabs whose body has been rendered on screen |
| `marks.compared` | set of facet ids | Facet rows the reader has opened in the facet lens |
| `mode` | `build` · `verify` | Site-wide reader mode (persisted, from the app root) |
| `pinned` | boolean | Whether the Register has scrolled off and the spine is showing |

**Family lens.** The selected column stands at full contrast; the other seven
recede to about 55% but remain legible and clickable. Beneath the Register, the
tab body renders that column's six blocks. The column's target chips are
rendered again as anchors at the top of block 3.

**Facet lens.** A facet row header has been activated. The row lifts across all
eight columns (full contrast; the rest recede) and the panel beneath the
Register becomes the **facet lens**: the eight full facet sentences, one per
family, in column order, each with an *open tab* action. The tablist remains
visible and active; activating any tab returns to the family lens with that
column standing. This is the only way to read one facet across the whole
surface, and it is the move that shows the model is a classification and not a
list.

**Target focus.** Activating a chip opens its family (switching lens to
`family` if needed), scrolls the tab body to that target's entry, and sets the
cursor. For DA12 from the P column the P tab opens, with the entry marked
*shared with F3* and a link to the F3 entry.

**Marks.** A target is *read* when its entry in block 3 has been on screen for
600 ms or was reached by activation. A tab is *opened* when its body renders. A
facet is *compared* when its facet-lens panel renders. Marks persist in
`localStorage` inside a try/catch and there is one *clear marks* control.
Marked chips and cells take a neutral ink fill (the text colour at low alpha),
not a colour and not a checkmark, and the legend beneath the foot says: *Marks
are yours. They record what you have read, not what has been verified.* This
keeps the coverage instrument on the right side of CONTENT-SPEC §10's rule
against invented verified states.

**Pinned spine.** When the Register scrolls above the viewport, a 44 px spine
sticks under the site header: the eight column segments with their target dots,
the standing column and cursor highlighted, the three counters at the right.
It is the reader's place-keeper while they are deep in F2's seven targets.
Activating the spine scrolls back to the full Register; the spine's dots are
themselves activatable and do exactly what the chips do.

## 4. Transitions

Motion is used only where it says something about position.

- **Column change (family lens):** the receding/standing contrast crossfades in
  160 ms; the tab body swaps with no slide. A slide would imply the tabs are a
  carousel; they are columns of a fixed table.
- **Row lift (facet lens):** the row's cells raise contrast in 160 ms and a
  1 px rule extends along the row from the header to the ⊥ column in 240 ms.
  The rule is the one explanatory motion: it draws the reading direction.
- **Target focus:** the tab body scrolls to the entry with the browser's
  native smooth scroll; the chip and the entry share a 2 px ink outline while
  the cursor is on them.
- **Marking:** a chip fills in 120 ms after the read threshold. No pulse.
- **Spine:** appears and disappears instantly on the intersection boundary.
- **DA12 pair:** hovering either chip draws a thin arc between the two over the
  rail, static, in 120 ms.

Under `prefers-reduced-motion: reduce` every duration above is 0 ms, scrolling
uses `auto` behaviour, and the DA12 arc is drawn without animation. States
still change; nothing moves to get there.

## 5. Interaction model

**Pointer.** Column header → open tab. Chip → focus target. Facet row header →
facet lens for that row. Facet cell → open that column *and* highlight that
facet's row inside the tab's facet-profile block (both coordinates at once).
Spine segment or dot → same as header or chip. Foot counters → no action;
*clear marks* → resets marks after a confirm.

**Keyboard.** The Register is a `role="grid"` with roving `tabindex`. One Tab
stop enters the grid at the last cursor position. Arrow keys move the cursor:
Left/Right across chips within the rail and across cells within a facet row,
crossing column boundaries; Up/Down between the rail and the facet rows within
the same column (Up from the rail goes to the column header, Down from row 8
stays). Home/End go to the first/last cell in the row; Ctrl+Home to F1's
header. Enter or Space activates the cell exactly as a click would. Escape from
the facet lens returns to the family lens with the previous column standing.
The tablist below the Register is a standard `role="tablist"` with Left/Right
and Home/End, and it is the second, redundant way to change column so a reader
who never enters the grid still has full access.

**Screen reader.** Column headers are `columnheader` with the tab label and
target count ("F2, Credit and collateralized debt, 7 action targets"). Chips are
`gridcell` with an accessible name of the form "DA06, borrow / accrue / repay,
F2, read" or "…, not read"; the P copy of DA12 says "shared with F3". Facet
cells read "Custody, F5, off-chain custodian holds the underlying — defining
facet". The foot is a polite `aria-live` region that announces counts when they
change ("targets read 9 of 24"). The spine is `aria-hidden` because it repeats
the Register; its function is reachable through the Register itself.

## 6. At 400 px

The Register transposes rather than shrinks. The same DOM is laid out as eight
stacked family blocks in the fixed order. Each block is: the header (id, label,
count), its chips in one wrapping row, then its eight facet placements as an
eight-line list with the facet name in front of each value (the facet row
headers of the wide layout become inline labels via a data attribute). Facet
lens is still available from a select-style row of eight facet buttons above
the stack; choosing one collapses each block to its header plus that one facet
line, so the eight-entry comparison stays on one screen height.

The spine at narrow width is the 24-chip strip alone: 24 squares of 11 px with
2 px gaps is 310 px, which fits within a 400 px viewport with 16 px gutters
and the counters wrapped to a second line. Nothing on the page scrolls
sideways; the only horizontal scroll container on the site is the property
inventory table in section 6.

The tab body's six blocks stack in one column at every width; the code samples
scroll horizontally inside their own `pre` container.

## 7. Why this form

The obvious alternatives, and why each loses to the Register:

- **A plain tab bar with eight labels.** Eight labels cannot show 24. A reader
  looking at "F5 · Tokenized off-chain claims" has no way to know that tab owns
  one target and F2 owns seven, or that they have read eleven targets and have
  thirteen to go. Completeness is the brief; a tab bar makes completeness
  invisible.
- **A 24-row action-target table.** Shows every target, but flattens family
  into a column of text and loses facets entirely. It is a list, and the whole
  point of the model is that it is not a list.
- **A treemap or sunburst of families.** Shows proportion well and facets not
  at all, and it invites the reader to treat area as importance. Width in the
  Register carries the same proportion without the decoration and keeps a row
  axis free for facets.
- **A force-directed graph of targets and facets.** Positions would be
  arbitrary, unstable between visits, unreadable to a screen reader, and would
  need a dependency PLAN.md tells us to avoid.
- **A heatmap of facet × family with colour intensity.** Colour would have to
  encode something, and there is no ordinal value to encode. Facet placements
  are sentences, not scores. The Register's cells are text.

The Register is the only form I found in which all three axes are on one
screen, with exact integers, with the reader's position visible at all times,
and in which the tabs the user asked for are not an extra component but the
object's own columns. It reads as a ledger, which is the brand: a fixed set of
rows and columns, every entry filled, no line rounded away.

## 8. What it deliberately does not do

- It does not colour-code families beyond one hue per column at low
  saturation, used only for the standing column and its tab body's rules. The
  page is not a rainbow.
- It does not animate coverage as a progress bar, ring or percentage.
- It does not show any verified, audited, proven or green state. The only
  marks are the reader's own.
- It does not put `F2` or `DA06` in code font in a way that suggests source
  syntax. Identifiers are set in the small-caps label style; only `.mori`
  samples use the code face.
