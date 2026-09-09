# Centrepiece: the guarantee chain

## In two sentences

Six link-shaped obligations stand in a vertical column with a visible gap
between each pair labelled "does not establish", and the reader can collapse
them into a single bar to see the shape of the claim the field usually
publishes, then expand them again. Selecting an obligation opens a panel that
says what that layer establishes, what it leaves open (always the next layer,
by name), and which of the thirteen threat rows sit at that boundary; the four
proof claims, the twelve properties in three columns, the thirteen rows with
residual risk emphasized and the seven audit boundaries are arranged beneath.

## What it draws

### The chain

An ordered list of six elements, top to bottom, in the fixed order from
CONTENT-SPEC §4:

1. Semantic validity
2. Moriarty-to-Compact translation validity
3. Compact-to-ZKIR compiler correctness
4. Circuit and proof correctness
5. Midnight ledger feasibility
6. Transaction construction and wallet correctness

Each element is a button 64px tall with a mono kicker ("obligation 2 of 6") and
the layer name in the display weight. Between each pair is a 44px gap holding a
1px stem and the label "↓ does not establish" in the accent colour and the mono
face. The gaps are the diagram. The links are deliberately plain; everything
that carries meaning is in the spaces between them.

After the sixth element a dashed, unfilled seventh element reads "What no layer
establishes". It is not a seventh layer and is not drawn as one: it is
unfilled, dashed, and its kicker reads "residual" instead of "obligation n of
6". It exists because the residual-risk column is the part nobody publishes,
and the chain needs a place to end that is not a full stop.

### The panel

Right of the chain on desktop, below it on mobile. For a selected obligation it
renders, in order:

- kicker: `obligation n of 6`
- the layer name as the panel heading
- **What it settles** (Build) or **What it establishes** (Verify): one
  paragraph, mode-specific, traceable to §3, §4, §5 or §6 of the spec
- for obligation 4 only: the four mandatory proof claims as chips, with
  HistoryCompliance in the accent
- **Leaves open**: a block with an accent rule on its left, the line
  "does not establish ↓ <next layer name>", and one paragraph stating what
  the next obligation still has to do
- **Threat rows at this boundary**: chips linking to rows in the table below

For the terminal element the panel lists all thirteen residual risks, one per
line, with the threat name in the secondary colour and the residual risk in the
ink colour. For the collapsed state the panel carries the overclaim caption and
nothing else.

### The collapsed state

A control above the chain reads "Show it collapsed". Pressing it drives every
gap to zero height (scaleY on the gap, translateY on each element, transform
only), squares the corners of the inner elements so the six read as one
continuous bar, and switches the panel to:

> One bar, six obligations, no gaps. This is the shape the claim usually
> takes: a single word standing for six separate things, with nothing in it
> saying which of the six was established, under what assumption, or what
> remains.

The collapsed bar carries no word, no checkmark and no colour. It is not
labelled "verified" or "audited", because the rules forbid rendering a state
the repository does not support, even as a counter-example. The absence of a
label is the point: the bar is unreadable. Selecting any element while
collapsed expands the chain first.

### The four claims

A row of four equal cells under the chain, numbered 1 to 4, names in the mono
face, one sentence each from §4.2. HistoryCompliance has an accent border and
an accent name. Beneath the row, with an accent left rule: "The fourth is the
one that distinguishes Moriarty from contract-level verification: a valid proof
of a valid transition against an invalid history must not be accepted."

### The property table

Twelve rows, three columns: Property · Assumption that qualifies it · Moriarty
obligation, verbatim from §4.1. The three columns never merge. Below 720px each
row becomes a card with the three column names as field labels, so the middle
column keeps its name even when it has no column. In Verify mode the
sub-heading carries the pinned Isabelle source commit
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`.

### The threat table

Thirteen rows, three columns: Threat · Required control · Residual risk. The
residual-risk header is in the accent, each residual cell carries a small
accent mark, and the residual text is in the ink colour while the control text
is in the secondary colour: residual risk is visually the loudest column, which
is the inversion of every other security table. Rows whose required control
sits at the selected obligation are lifted (accent rule on the left); the others
are dimmed to 38% opacity, never hidden. A note below the table states how many
rows sit at the selected boundary and that the placement is editorial.

Four rows (Invalid initial state, Backend version skew, Resource exhaustion,
Governance capture) are not reproduced in CONTENT-SPEC §4.3, which names
thirteen and lists nine; they are taken from `wiki/security.md`, the file the
spec cites, and each carries a small mono source line saying so. The storyboard
asks for these four to be added to the spec before implementation.

### The audit boundaries

A two-column numbered list of the seven boundaries from §4.3 under the heading
"A single 'Moriarty audit' is not an adequate claim."

## The data it reads

From the site's typed data modules (PLAN.md, Technical shape):

- `assurance.layers`: six records `{ n, name, build, verify, open }`
- `assurance.claims`: four records `{ name, text, distinguishing: boolean }`
- `assurance.properties`: twelve records `{ property, assumption, obligation }`
- `security.threats`: thirteen records
  `{ id, name, control, residual, at: number[], source: 'spec' | 'wiki' }`
- `security.audits`: seven strings

The `at` field on a threat is an editorial cross-reference (which obligation's
control the row's control belongs to). It drives only the highlight and the
"at this boundary" chips. The page never states it as a fact, every row is
always visible, and the table note says the placement is editorial. If the
content owner would rather have no cross-reference, emptying every `at` array
removes the highlight without touching the chain.

## States and transitions

| State | Chain | Panel | Threat table |
|---|---|---|---|
| Entering viewport (motion allowed) | drawn as one bar, then separates over 700ms after 420ms | selected obligation | rows at selected boundary lifted |
| Entering viewport (reduced motion) | drawn separated | selected obligation | same |
| Obligation n selected | link n pressed | obligation n | rows with `at` containing n lifted, others dimmed |
| Terminal selected | dashed element pressed | all thirteen residual risks | rows with `at` containing 7 lifted |
| Collapsed | gaps at zero, corners squared | overclaim caption | nothing lifted, nothing dimmed |
| Mode changed | unchanged | paragraph swaps between Build and Verify text | unchanged |

Transitions: expand and collapse use `transform` and `opacity` only, 700ms
with `cubic-bezier(0.22, 1, 0.36, 1)`; selection changes are instant except a
200ms border colour change; the panel does not animate, it re-renders. Under
`prefers-reduced-motion: reduce` every transition is removed and the entry
state is the expanded chain.

Deep link: the selected element is written to the hash as `#guarantees/<n>`
with `replaceState`, and read back on load; `n` from 1 to 7.

## Interaction model

- Pointer: click an element to select; click the collapse control to toggle;
  click a threat chip to jump to and highlight the row (`:target` tint); click
  a claim chip to jump to the claims row.
- Keyboard: every element is a native `button` in tab order. Within the chain,
  Up/Down (and Left/Right) move focus and selection to the neighbouring
  element, Home/End to the first and last. Enter and Space select. The collapse
  control is a toggle button with `aria-pressed`.
- Mode switch: a `radiogroup` labelled "Reading mode", persisted in
  `localStorage` under `moriarty.mode`; switching re-renders the panel
  paragraph and the properties sub-heading only.

## Screen-reader behaviour

- The chain is an `ol` labelled "The assurance chain"; each item is a button
  whose accessible name is "obligation n of 6, <layer name>"; `aria-pressed`
  marks the selection. The gap labels are `aria-hidden`, because the
  relationship they draw is spoken by the panel's "Leaves open: does not
  establish <next layer>" line, which is real text.
- A visually hidden `aria-live="polite"` region announces "Obligation n of 6,
  <name>, selected", "Chain collapsed into one bar" and "Chain expanded into
  six obligations". The panel itself is not live, so a selection change is one
  sentence, not a paragraph.
- The panel is an `aside` labelled by its heading. The tables are real tables
  with `th` headers; below 720px they are linearised with `data-label` field
  names, so the three column names are still spoken per cell.
- Dimmed threat rows remain in the accessibility tree at full text; dimming is
  opacity only.

## At 400px

- The stage becomes a single column: chain first, panel directly beneath. The
  chain elements keep their 64px height and full-width labels; the gap labels
  stay readable at 11.5px mono.
- The collapse control and its heading sit in one row above the chain.
- The claims row stacks to four full-width cells.
- Both tables become stacked cards: one card per row, each field led by its
  column name, the residual field led by "Residual risk" in the accent. The
  page body never scrolls sideways; there is no horizontal scroll container,
  because the column labels matter more than the row shape.
- The audit list becomes a single numbered column.
- The lede paragraph and all body copy are capped at 64ch; the heading scales
  to 30px.

## Under `prefers-reduced-motion`

The chain never starts collapsed; the entry observer is not attached. The
collapse control still works but the state change is instant. No other motion
exists in the section. All meaning is carried by static text and position,
so the reduced-motion reader loses nothing but the one explanatory animation,
whose content (the six separate from the one) is also stated in the lede.

## Why this form

**Why a vertical list of buttons and not an SVG diagram.** PLAN.md prefers
hand-authored SVG, and the other visualizations on the site (the failure
matrix, the one-transition strip, the pipeline branch) should use it. The chain
is different: its content is six names and five relations, all text, and the
reader must be able to select each one. A semantic ordered list of native
buttons gives correct tab order, real text at every width, and mobile
reflow for free; an SVG version would need `foreignObject` or a parallel
accessible structure to achieve the same. The diagram is the spacing, and CSS
draws spacing better than SVG does.

**Why the gaps carry the labels and not the links.** The thesis is that
establishing one layer does not establish the next. If the links carried
"establishes" text, the eye would read six achievements. With the gaps labelled
and the links plain, the eye reads five refusals. That is the correct reading.

**Why a collapse control and not a before/after pair.** A side-by-side "what
they say / what is true" pair would need a label on the collapsed bar, and
every candidate label ("verified", "audited", a checkmark) is forbidden by the
rules and would be the first thing screenshotted. Letting the reader collapse
the chain themselves, and giving the collapsed bar no label at all, makes the
overclaim visible without ever rendering it.

**Why the panel says "leaves open" by name.** "Does not establish the next"
is abstract. The panel makes it concrete by naming the next obligation and
stating in one paragraph what that obligation still has to do, so the reader
always sees the chain as pairs, not as a list.

**Why the threat rows are attached to the chain.** The residual-risk column is
the part nobody publishes; on its own it is a thirteen-row table. Attached to
the chain it becomes the answer to "and what does this layer still not
handle", which is the question the chain raises. The attachment is highlight
only, the rows are never hidden, and the note says the placement is editorial,
so the site gains the connection without asserting a mapping the spec does not
make.

**Why residual risk is the loudest column.** Every security table on every
other site makes the control column the loud one. Inverting that is the
cheapest possible way to say what this project is.

**Alternatives rejected.** A horizontal pipeline with the chain drawn as a
progress bar: reads as progress, and progress implies completion. A radial or
nested-ring diagram (each layer enclosing the next): implies that the outer
layer contains the inner guarantees, which is exactly the collapse the section
argues against. A matrix of layers against properties: would require mapping
each of the twelve properties to a layer, which the spec does not do and the
rules forbid inventing.

## What must be true before implementation

- The four threat rows not in CONTENT-SPEC §4.3 are added to the spec, or the
  `source: 'wiki'` marker stays on the page.
- The `at` cross-references are reviewed by the content owner or emptied.
- `wiki/formal-assurance.md` draws the chain with two further arrows (private
  witness correctness and availability; participant and oracle liveness).
  The site follows the spec's six. If the spec adopts eight, the chain takes
  two more elements from the data module and the terminal element's copy
  changes; nothing else moves.
