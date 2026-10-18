# Audit 06 — Apple HIG: Typography, Contrast, Ring Gauge, Caption, Transport, DOM Updates

Auditor: Grok 4.7 xhigh #6 (Apple HIG UI, typography, and inspector hierarchy)
Date: 2026-10-18
Scope: `index.html`, `src/styles.css`, `src/app.js`
Cross-checks: `src/designers/theme.js` (palette drift), `src/engine/sherlockEngine.js` (frame labels and the substitution equation)

Method: source inspection plus WCAG 2.1 relative-luminance contrast computed from the authored hex values. Slot tints are composited over `--paper-elevated` (`#fbf8f2`). Button heights are computed from padding, a 1.2 line box, and 1px borders. This pass did not include a rendered browser measurement.

Apple HIG lens used here: Clarity (legible type, controls that look like their job), Deference (the die and the live state stay in front of the chrome), Depth (one primary surface, quiet separators, an obvious current selection), and Accessibility (44×44pt targets, contrast, keyboard, Dynamic Type, Reduce Motion, sentence case).

## Verdict

The page already has the right skeleton for an Apple-style instrument: a quiet canvas, a trailing inspector, labeled transport, and a step gate so the DOM does not rebuild every animation frame. The masthead and the prose deck are the only regions that meet HIG type size and reading measure.

The instrument itself is set below the HIG floor. Primary ring readouts are 9.5px. The chrome color `--muted` (`#8c887f`) is about 3.2:1 on the paper cards. Idle slots are dimmed further to about 2.3:1. Transport and camera controls compute to roughly 25–29px tall. The “6 split-arc rings” are a 3-column card grid, so the radial story told in the prose is not the picture in the inspector. `syncStepUI` rebuilds that grid, the frame list, the call chips, and the line-number column through `innerHTML` on every step, and the scrubber calls that path on every fractional input.

Next iteration: keep the paper editorial voice. Raise type to 12px minimum, retokenize muted and heat, turn the gauge into six stacked split tracks plus a visible control core, merge the caption into one toolbar, and patch DOM text instead of rebuilding it.

## What already matches the HIG

- Three regions with honest jobs: Program, the die, Inspector. DOM order matches the stacked layout at `max-width: 1360px`.
- The viewport card is free of overlaid HUD chrome. The caption sits under the canvas (`index.html` around the `#stageCaptionBar` block).
- HTML copy is sentence case. Uppercase is a CSS `text-transform`, so the source strings can stay as labels.
- Masthead scale is sound: 12px brand, `clamp(24px, 2.4vw, 32px)` title at weight 500, 16.5px deck in `--ink-soft` (about 6.2:1).
- Prose is the best reading block on the page: 20px titles, 14.5px / 1.55 body, `--ink-soft` on `--paper-card` at about 6.4:1.
- Telemetry uses a real `dl`. Sliders and the speed select have `label for`. The preset group and the camera group have accessible names. Presets expose `aria-pressed`.
- `:focus-visible` draws a 2px `--heat` ring. Heat on paper is about 4.1:1, which clears the 3:1 focus-indicator bar.
- Status is a word (Evaluating, Done, Rejected, Exhausted), and most ring readouts encode kind in the string (`RET:`, `SCOPE`, `IF`, `MATCH`, `LET`).
- Dynamic strings go through `escapeHtml` before `innerHTML`.
- Playback DOM writes are intended to run on integer step changes (`state.lastRenderedStepIdx` in `animate`). That is the correct boundary.

## Typography

HIG text styles bottom out at Caption 2 (11pt). Anything that carries meaning sits at 11pt or above, in sentence case, with weight and size doing the hierarchy. Tracked-out all-caps is reserved for a single brand mark.

### Current ramp

| Role | CSS | Size / case | HIG reading |
| --- | --- | --- | --- |
| Brandmark | `.brandmark` | 12px, upper, tracking 0.22em | Fine as the one small-caps mark |
| Screen title | `.subtitle` | 24–32px, weight 500 | Matches Title 2 / Title 1 |
| Deck | `.deck` | 16.5px `--ink-soft` | One step under Body (17pt). Acceptable |
| Panel title | `.panel-header h2` | 11px upper, tracking 0.14em, `--muted` | Below the floor, and the color fails contrast |
| Buttons | `.btn-primary`, `.btn-icon` | 11px upper, tracking 0.06em | Below the floor |
| Preset subtitle | `.preset-btn small` | 9.5px | Smallest supporting text |
| Editor | `#sourceEditor`, `.line-numbers` | 11.5px mono | At the floor. Gutter color fails (below) |
| Slider labels | `.hw-control-top` | 10px upper `--muted` | Below the floor |
| Caption | `.stage-caption` | 11.5px | At the floor |
| Status chip | `.status-badge` | 10px upper, tracking 0.12em | Below the floor |
| Caption hint | `.stage-caption-meta` | 10.5px `--muted` | Below the floor |
| Scrubber meta | `.scrubber-top` | 10px upper `--muted` | Below the floor |
| Camera | `.lens-buttons button` | 10.5px | Below the floor |
| Telemetry labels | `.telemetry-dl dt` | 9.5px upper, tracking 0.12em, `--muted` | Smallest label on the page |
| Telemetry values | `.telemetry-dl dd` | 12px weight 600 | First inspector value that clears 12px |
| Subpanel title | `.subpanel-header` | 10px upper, tracking 0.12em, `--muted` | The rings title is set as a caption |
| Legend and ring cards | `.ring-legend`, `.ring-pill` | 9.5px | The primary instrument is the smallest text |
| Frame kind tag | `.frame-kind-tag` | 9.5px upper `--muted` | Below the floor |
| Prose | `.prose-card p` | 14.5px / 1.55 | Sound. Table headers fall back to 10px upper `--muted` |

Hierarchy is inverted. A reader can read the essay under the workbench more easily than the twelve slots they are meant to watch.

`button, select, input, textarea { font: inherit }` starts from Newsreader, then almost every control overrides to JetBrains Mono and uppercase. The UI speaks in two voices at once: a literary masthead and a 10px instrument stencil. HIG would keep Newsreader (or the system serif) for the masthead and the prose, and a 12–13px system mono for values only. Labels would be the proportional face in sentence case.

There is no `rem` ramp and no type that responds to the user text size. HIG Dynamic Type is unmet. A practical next step on the web is a root size with `clamp` and tokens in `rem`, leaving the editor mono at a fixed size so the code column does not reflow.

Optical sizing on Newsreader is loaded (`opsz`) and unused. Italic is loaded and unused. Dropping those axes shortens the font request.

### Concrete type ramp for the next pass

Keep the masthead. Change the instrument:

| Token | Spec |
| --- | --- |
| `--text-section` | 13px / 600 / sentence case / `--ink`. Panel titles and subpanel titles |
| `--text-label` | 12px / 500 / sentence case / `--ink-soft`. Telemetry names, slider names, legend |
| `--text-value` | 13px / 550 / `--ink`. Ring readouts, telemetry values, caption control |
| `--text-caption` | 12px / 400 / `--ink-soft`. Hints, slot indices, kind tags |
| `--text-mono` | JetBrains Mono, `ui-monospace`, `SFMono-Regular`, Menlo. Values and code only |

Remove `text-transform: uppercase` and tracking above 0.02em from buttons, `dt`, subpanel headers, the scrubber, the status chip, and the rings header. Leave tracking on `.brandmark` only.

Set `.prose-card p` to 17px if the essay is meant to match HIG Body. 15px is the floor if density matters more.

## Color and contrast

Ratios are WCAG 2.1. Text under 18px needs 4.5:1. A boundary or swatch that is the only edge of a control needs 3:1.

### Tokens that fail

| Pair | Ratio | Where |
| --- | --- | --- |
| `--muted #8c887f` on `--paper #f3efe6` | 3.08:1 | Page background labels |
| `--muted` on `--paper-card #f7f3ea` | 3.19:1 | Panel titles, table headers |
| `--muted` on `--paper-elevated #fbf8f2` | 3.33:1 | Telemetry `dt`, ring headers, hints, kind tags |
| Idle slot text, opacity 0.72, composited on the pill | about 2.26:1 | `.ring-slot-row.slot-idle` |
| `--heat #c84b31` on `--paper-elevated` | 4.40:1 | “Evaluating” chip, 10px |
| `--paper` on `--heat` | 4.06:1 | Rejected and Exhausted chips |
| `#656872` on `#15161a` | 3.25:1 | Editor line numbers |
| `#1f7a6c` on env tint `#e1e9e2` | 4.18:1 | Slot key “A:” / “B:” in ENV rows |
| `#9e6314` on op tint `#f4e7d5` | 4.06:1 | Slot key in OP rows |
| Swatch `#c88224` on `--paper-elevated` | 2.97:1 | OP legend chip, as a graphical object |
| Right edge `#b8842d` on its gold tint | 2.74:1 | `.slot-op-right` 3px bar |
| `--paper-border #d5cfc2` on card / elevated / paper | 1.35–1.46:1 | Every control whose edge is this hairline |
| Elevated fill `#fbf8f2` on card `#f7f3ea` | 1.04:1 | Buttons and pills have no fill separation |

`--ink` (about 15.5:1), `--ink-soft` (about 6.2–6.7:1), `--resolve #2b5b4c` (about 7:1), slot *values* (`#16594f`, `#7c4d0d`, `#7d3910`, `#6e480b`, `#4a3561`), editor toolbar badges, and the compile-error pink on graphite all clear 4.5:1. The failure is concentrated in the chrome gray, the idle treatment, the heat chips, the gutter, and the A:/B: keys.

`.ring-slot-row.slot-idle` sets `opacity: 0.72` on the whole row. That fades text that was already under 4.5:1. Idle should be full-strength `--ink-soft` on a dashed track.

### Heat is doing the wrong semantic job

HIG uses red for destructive, orange for caution, and a neutral or tint for “in progress.” Evaluating, the steady state, is drawn in the same hue as Rejected and Exhausted:

- Evaluating: heat text, heat border (`.status-badge.status-evaluating`)
- Rejected and Exhausted: identical filled heat chips (the two classes share one rule)

A running trace looks like a warning. A fuel stop and a rule rejection look like the same chip. `.telemetry-dl dd.is-stopped` also uses `--heat` for both stops.

Next pass:

- Evaluating: `--ink` text, 1px `--ink` border, paper fill. No red.
- Rejected: text `#6e241c` on a 16% heat tint (computed about 8.3:1), or paper text on `#7a2a20` (about 8.4:1).
- Exhausted: text `#7a4e08` on a warm gold tint. Fuel running out is a limit, and `theme.js` already calls that state `FUEL`.
- Done: keep `--resolve`.

`--heat` can stay as the slider `accent-color` and the focus ring. It should leave the steady-state badge.

### Borders

Apple separators are allowed to be quiet. Control edges are not. `.btn-icon`, `.preset-btn`, `.lens-buttons button`, `.ring-pill`, and the scrubber track are identified by `#d5cfc2` on a near-matching fill (about 1.4:1).

Next pass: keep `#d5cfc2` for section rules inside a card. Give controls a border of `#6f6a60` (about 4.9:1 on the card) or a fill of `#e7e0d2` plus an ink label. The active ring can stay `--ink`. Add a `:active` fill so press feedback exists without hover. Touch has no hover, and the file has hover styles only.

### Two palettes

`styles.css` and `src/designers/theme.js` `CSS_COLORS` / `FRAME_KIND_META` disagree:

| Role | CSS chrome | theme.js |
| --- | --- | --- |
| Paper | `#f3efe6` | `#f4f0e8` |
| Heat | `#c84b31` | `#ea580c` |
| Resolve | `#2b5b4c` | `#0d9488` |
| Gold | `#b8842d` | `#d4af37` |
| Muted | `#8c887f` | `#64748b` |
| ENV | swatch `#1f7a6c` | die `#2dd4bf`, text css `#0d9488` |
| OP | swatch `#c88224` | die `#f59e0b` / `#fbbf24` |
| GATE | swatch `#6e548a` | die `#818cf8`, text css `#4f46e5` |

The well is dark, so the die can use bright cyan, amber, and indigo. The paper inspector correctly uses darker inks. The hues still need to be the same families. GATE is the drift that will confuse a glance from die to legend: periwinkle on the wafer versus dusty plum in the chip. Point both at one token module. Use `FRAME_KIND_META.css` as the paper-safe ink and the die hex as the well ink.

There is no `prefers-color-scheme` and no `prefers-contrast`. HIG expects both appearances and a higher-contrast increase. The paper brand can stay the default. Ship the dark appearance by aliasing the same tokens (`--paper` becomes the well slate, `--ink` becomes titanium) rather than a second stylesheet of hex.

## Layout and inspector hierarchy

### Workbench

`.workbench-grid` is `340px | minmax(520px, 1fr) | 390px` with a 16px gap. The fixed columns plus padding need about 1330px, and the stack breakpoint is 1360px. A 13-inch window drops the die below the whole editor. Before stacking, narrow the rails (300 / fluid / 340) and let the stage keep the middle down to about 1180px.

The stage column is four stacked cards of equal visual weight: viewport, caption, transport, camera. HIG deference asks for one primary surface and one toolbar. Put the caption, transport, and camera inside `.stage-viewport-card` as a footer toolbar, or group them in a single bar under the canvas. The viewport keeps `min-height: 460px` and `height: min(68vh, 640px)`, which is the right priority.

Spacing is off the 8pt grid (5, 6, 7, 9, 10, 11, 14). Snap padding and gaps to 4 / 8 / 12 / 16. Corner radius is 0 everywhere except the status dot. Sharp corners fit the editorial masthead. If the next pass stays editorial, keep the corners and fix type, contrast, and targets. Continuous corners (12px cards, 8px controls, capsule chips) are the HIG material alternative. Pick one and use it on every card.

Safe areas are unset. Add `padding-left/right: max(24px, env(safe-area-inset-*))` on `.workbench-shell` before this is used on a phone with a home indicator.

### Inspector order

An Apple inspector leads with the object, then disclosure groups, one fact each, values aligned so numbers scan.

Current order:

1. Title “Inspector” plus a rule pill.
2. Six-field telemetry grid that repeats the caption, the scrubber, and the rule pill.
3. Rings, under a 46-character uppercase title.
4. Saved frames, top of stack first.
5. Active calls, a third drawing of the same stack.

Facts repeated on one screen:

| Fact | Copies |
| --- | --- |
| Control expression | Caption, telemetry “Current control”, rings badge `CTRL:` |
| Status | Caption chip, telemetry Status |
| Step index | Scrubber, telemetry Transition |
| Rule id | Header pill, Transition `<small>` |
| Fuel | Telemetry, rings badge |
| Frame / slot count | Telemetry “Saved frames”, rings badge, frame-list count |

The rings are the object this inspector exists to explain. They should be the first subpanel. Telemetry that remains belongs in one disclosure, “Step details,” closed by default, in a single column so `Eval(...)` can use the full 390px. The rule pill can be the disclosure’s value. The scrubber already owns the step index.

“Saved frames, top first” and the gauge use opposite orders. The gauge’s R1 is chronological outer (`chronoFrames`, `app.js` in `syncStepUI`). The frame list walks `frames` top-first and then tags each row with `R${ringNum}·${slotChar}`. The top card is the inner ring. A person matching a card to a slot has to reverse the list in their head. Use one order in both places. Recommendation: chronological outer-to-inner, with the stack top marked `aria-current="true"`, because that is the order of the die. The words “top first” can move to a secondary caption if the semantic stack order still needs a name.

Subpanels do not collapse. On a laptop the rings sit under a telemetry block and often below the fold. Make each `.subpanel` a `<details>` (or a button plus `aria-expanded`) with the rings open and the frame list and call list closed until asked. Headers at 13px sentence case, values at 13px.

The telemetry grid is two columns with a left rule. “Current control” wraps beside a short neighbor, so the column of values does not scan. One column, label above value, full width for the control string.

Call chips (`.rec-call-node`) are the right model for “jump to this call,” and they are real buttons. They are about 23px tall and they repeat the frame story. Keep them, inside a closed disclosure, at 44px hit height.

## 12-slot split-arc ring gauge

The data model in `syncStepUI` is the right one:

- `chronoFrames = [...frames].reverse()` so index 0 is the oldest frame.
- Ring `r` owns slots `2r` and `2r+1` (West A, East B).
- Six rings, twelve slots.
- The active ring is the innermost occupied ring.
- Past 12 frames, R6 receives `.overflow` and the telemetry line adds `+N`.

The drawing does not match that model.

### Geometry

`.six-rings-bar` is `repeat(3, 1fr)`. Visual order is R1 R2 R3 / R4 R5 R6. R2 sits beside R1 at the same depth. Nothing in the card says West or East except the `title` tooltip, which is unavailable on touch and easy to miss with a pointer. Nothing draws a control core. The heading “6 Split-Arc Rings (Slots A & B) & Control Core” promises a stator the badge reduces to `CTRL: EVAL`.

`.ring-pill.ring-idle` is applied and has no rule. Idle is only the slot row.

`.slot-val` is `nowrap` plus ellipsis. Card math at the 390px rail: content width 362px, three columns, about 74px for the value, about 13 mono characters at 9.5px. From real labels in `sherlockEngine.js`:

| Readout | Fits in 74px |
| --- | --- |
| `RET:UInt`, `SCOPE(3)`, `4 * [?]`, `IF [?]`, `LET n=[?]` | Yes |
| `[?] * fact(3)`, `LET cube=[?]` | On the edge |
| `sum_list(2/3 args)`, `is_even(1/1 args)` | Truncated |
| A longer Left/Right expression | Truncated |

`getFrameSlotMeta` does not run `formatAlgebraicText`. The frame list shows `4 × [?]`. The ring shows `4 * [?]`. One notation.

Left and Right share the algebraic string. The amber-to-gold shift the prose describes is a border color (`#c86428` vs `#b8842d`) with no word. That is color used as the only channel. The legend has a single OP swatch, so it cannot teach the shift.

The active treatment is a 1px ink border on the whole ring. Both halves look current when only the stack-top half was just written. Occupied (`#b5ae9e`) versus idle (the failing gray border) is a small step. The current selection should be unmistakable at a glance: ink border plus a 13px value on that half only.

Overflow paints all of R6 as an error, including the legitimate slots 11 and 12. Extra frames exist only in the frame list. With the frame-limit slider at 20 and the die at 12, overflow is a normal setting, not a fault.

The header and the badge are a `space-between` row at 10px with 0.12em tracking. The title is 46 characters (about 330px) and a live badge such as `8/12 SLOTS · FUEL 100% · CTRL: EVAL` is about 220px. The row’s content width is 362px, so the pair wraps into a stack of tracked caps above the actual data.

There is no media query for the gauge. At the stacked breakpoint the inspector is full width and still three columns, so the values get slightly more room on a phone and still truncate. One column of rings is the layout that has room for the readout (about 50 characters).

Keyboard: the pills are `div`s. The active ring is not `aria-current`. The legend swatches are empty `<i>` elements, which is fine only while the adjacent words remain.

### Next gauge

Replace the 3-column grid with a vertical stack. R1 on top, labeled “Outer.” R6 at the bottom, labeled “Inner.” Each row is one split track:

```text
Outer
R1  01–02   [ A  West  RET:UInt     ENV ][ B  East  4 × [?]  Right  OP ]
R2  03–04   [ A  — ][ B  — ]
...
Inner
R6  11–12   [ A  — ][ B  — ]
Control     EVAL  PUSH  RET  INVOKE  ENTER  BOUND  CASES  MATCH  PRIM  ERR
```

Implementation sketch:

- Each half is a real `<button type="button">` (or a focusable grid cell) with `aria-current="true"` on the stack-top half only.
- Visible text includes side, readout, family, and for OP the side of the operator: `Left` or `Right`. Color repeats that word. It does not replace it.
- Readout uses `formatAlgebraicText`, wraps to two lines, and keeps the full string in `aria-label` rather than only in `title`.
- Legend gains four chips: ENV, OP Left, OP Right, GATE, plus a hollow Idle. Swatch minimum `#9a6414` for amber (about 4.7:1 on elevated) so the chip clears 3:1.
- Family inks, checked against their own tints: ENV key `#0f6e62` on `#e7f3f0` (about 5.4:1), OP key `#7a4e08` on `#f8edd9` (about 6.2:1), GATE key `#5c4a78` on `#efeaf4` (about 6.6:1). Align those hues with `FRAME_KIND_META`.
- The ten control sectors are a segmented strip under the stack. The active sector uses the ink fill. The badge drops `CTRL:` and fuel. Fuel already has a telemetry row.
- Overflow is a trailing row, “+N frames past the die,” in `--ink`. R6 keeps its real slots and loses the red wash.
- Header becomes “Rings” with a value “8 of 12 slots.” The essay below can keep the long name.

On a wide inspector, a concentric SVG of six split arcs is the picture that matches the die. Put the twelve readouts beside it, not inside 70px wedges. Do that only after the vertical split tracks are legible. A tiny diagram that truncates is the current failure mode.

## Stage caption bar

`#stageCaptionBar` is a full bordered card: status chip, `#hudControlSummary`, a middot, `#recursionEquationText`, and `#stageInspectText`. `aria-live="polite"` is on the whole bar.

The equation is worth showing. `sherlockEngine.js` builds a substitution such as `4 × (3 × [ fact(2) ])`, which is not the same string as `Eval(fact(2))`. When the stack is empty the equation collapses to `[ fact(4) ]` beside `Eval(fact(4))`. Those two spans then say the same thing. Hide `#recursionEquationText` when it is only the bracketed control atom.

The bar grows with `overflow-wrap: anywhere` and `flex-wrap: wrap`. A long control string changes the height of the column and shoves the transport down mid-playback. Give the sentence one line, `text-overflow: ellipsis`, and the full string in a tooltip or the inspector.

“Drag to orbit · Scroll to zoom” is 10.5px `--muted` (about 3.2:1). It disappears on inspect, which is right, and it never names a touch gesture. Scroll-to-zoom also fights page scroll once the layout stacks. The hint belongs in the caption footnote at 12px `--ink-soft`, and the touch hint is “Drag to orbit. Pinch to zoom.”

`aria-live="polite"` on a region rewritten at about 3.2 steps per second (see performance) makes VoiceOver queue a new announcement per step, including hover-inspect changes from `SherlockStage`’s `onInspect`. HIG live regions announce a change in outcome, not a telemetry stream.

Next caption, as the title row of the stage toolbar:

- A 12px status word. Live region is a visually hidden node that updates only when the status label changes.
- One 13px sentence: the equation if it adds information, otherwise the control text.
- A 12px inspect footnote. Writes to it do not use a live region.
- Fixed one-line height.

## Transport controls

The set is right: play/pause, previous, next, reset, a position slider, a speed menu. The presentation is a second card of equal weight, with targets under the HIG 44×44pt minimum.

Computed control heights from `styles.css`:

| Control | Computed height |
| --- | --- |
| `.btn-primary` | about 29px |
| `.btn-icon` (Play, Step, Reset) | about 27px |
| `.lens-buttons button` | about 25px |
| `.preset-btn` | about 43px |
| `.rec-call-node` | about 23px |

HIG allows a smaller glyph inside a 44pt hit box. Add `min-height: 44px` and horizontal padding of at least 12px on those controls. The play control is the primary one: ink fill, paper label, centered between previous and next. Reset is a trailing text button, “Start,” with the title “Go to step 1.” It currently has the same weight as Play.

Order is Play, Previous, Next, Reset. Playback toolbars put the play control between the step controls.

Ends are silent. Previous at step 0 and Next at the last step still look enabled. Set `disabled` and a muted style at the bounds.

Play starts as `state.playing = true`, so the trace runs on load and loops after `maxStep + 3.5`. Autoplay plus a live region is the accessibility failure. Start paused. Honor `prefers-reduced-motion: reduce` by staying paused and by skipping the loop.

The scrubber is `<input type="range" step="0.01">` over an integer trace (`app.js`, `#stepScrubber`). Keyboard users who focus it need on the order of 100 arrow presses to move one step. `aria-valuetext` is unset, so the accessible value is a fraction. Set `step="1"`, and on each integer change set `aria-valuetext` to `Step 12 of 45, Evaluating, Rule A102`.

The hint “Space plays/pauses · ← → step” repeats the button titles. One hint is enough, in the caption footnote, at 12px.

### Keyboard conflicts

The window `keydown` handler ignores `TEXTAREA`, `INPUT`, and `SELECT` only.

- Space on a focused button (Run, a preset, a camera, a call chip) is `preventDefault`’d and toggles playback. Space should activate the focused control. Ignore the shortcut when `document.activeElement` is a `button` or anything with `role="button"`, or only handle it when focus is on `body` or the canvas.
- Arrow keys while a camera button or a call chip is focused step the trace. Inside `.lens-buttons`, arrows should move the segmented selection. Trace stepping can stay on the window when focus is not in that group.
- `#sourceEditor` sets `outline: none` (specificity 1,0,0), which beats the global `:focus-visible` rule. Keyboard users inside the program lose the focus ring. Remove that outline and let `:focus-visible` show, or draw `.editor-container:focus-within { box-shadow: 0 0 0 2px var(--heat); }`.

Camera buttons toggle `.active` and never set `aria-pressed` or `aria-checked`. The group is a segmented control. Use `aria-pressed="true"` on the selected view the same way presets already do.

Reset, previous, and next do not update `lastRenderedStepIdx` themselves. The following section covers the double render that causes.

## DOM update performance

`animate` is a permanent `requestAnimationFrame` loop. That is justified for the WebGPU die. The DOM strategy is half-finished.

### What is already gated

```javascript
const currentIdx = Math.min(maxStep, Math.floor(state.playhead));
if (currentIdx !== state.lastRenderedStepIdx) {
  state.lastRenderedStepIdx = currentIdx;
  syncStepUI(currentIdx);
}
```

At 1×, `playhead += dt * 3.2 * speed` is about 3.2 integer steps per second. At 4× it is about 12.8. The inspector does not rebuild 60 times a second during playback. Keep this boundary.

### Where the gate leaks

`syncStepUI` does not check `lastRenderedStepIdx` itself.

| Caller | Behavior |
| --- | --- |
| `animate` | Sets the gate, then calls `syncStepUI` |
| Previous, Next, Reset | Call `syncStepUI` and do not set `lastRenderedStepIdx`, so the next frame calls it again |
| `#stepScrubber` `input` | Calls `syncStepUI` on every event, including many events that share one integer step. `step="0.01"` makes a drag across one step fire a long run of full rebuilds |
| `executeCurrentSource` | Sets the gate to `-1`, then syncs. One extra call. Fine |

Each unguarded call does all of the following:

1. `buildStepState(...)` with a hard-coded `dt: 0.016`. `animate` then calls `buildStepState` again that frame for `stage.render`. On a step boundary the trace is walked twice.
2. `updateEditorLineNumbers` rebuilds the entire gutter with `innerHTML` even when the active line is unchanged (`app.js`, `updateEditorLineNumbers`).
3. The six ring cards are one HTML string assigned to `#sixRingsGauge.innerHTML` (about 18 elements, plus the text nodes).
4. `#frameStackList.innerHTML` replaces every frame card.
5. `#recursionCallsTrack.innerHTML` replaces every call button and rebinds `click` listeners. Delegation on the track once would survive a rebuild.
6. Telemetry writes `innerHTML` for strings that include `<small>`, so the parser runs for values that are plain text split across two existing nodes.

While `state.playing`, `els.stepScrubber.value` is assigned every animation frame, including between integer steps. That moves the thumb smoothly and also dirties the accessibility tree at frame rate. With `step="1"`, assign the value only when `currentIdx` changes.

Slider commits are a second hot path. `#fuelSlider` and `#stackSlider` listen to `input` and call `executeCurrentSource`, which recompiles the program. A drag becomes a burst of full traces. HIG sliders update the visible number live and commit on release. Point `input` at the label and `change` at `executeCurrentSource`.

`renderPresets` rebuilds buttons on each preset choice. That is rare. Leave it.

No layout read is interleaved with these writes inside `syncStepUI`, so this is write amplification rather than forced synchronous layout. At 4×, a full inspector parse-and-style about 13 times a second still contends with the WebGPU frame.

### Next update path

1. Make `syncStepUI` return immediately when `stepIdx === state.lastRenderedStepIdx`, and set the gate inside the function. Every caller becomes safe.
2. In `animate`, build `stepState` once and pass it to both the DOM sync and `stage.render`.
3. Cache a signature per block (ring readouts + classes, frame labels, call labels + statuses, active line, status label). Assign `textContent` and `classList` only on the nodes whose signature changed.
4. Build the twelve slot nodes and the line-number spans once per trace. Playback only updates text and classes.
5. One delegated click listener on `#recursionCallsTrack`.
6. `aria-live` only for a status-label change, not for the caption container.
7. Scrubber `step="1"` and a value write only on integer change.
8. Fuel and frame-limit recompile on `change`.

A signature check alone, before any further restructuring, removes the scrubber’s fractional rebuilds and the double call from the step buttons. That is the first patch to land.

## Next iteration, in order

### P0 — Legibility, targets, and the update path

1. Replace `--muted` with `--ink-soft` (`#5a5852`, about 6.4:1 on the card) or `#5c5954` (about 6.6:1 on elevated). Delete `opacity: 0.72` on idle rows.
2. Apply the type ramp above. Nothing that is a label or a value stays under 12px. Drop uppercase and wide tracking outside `.brandmark`.
3. `min-height: 44px` on `.btn-primary`, `.btn-icon`, `.lens-buttons button`, `.preset-btn`, and `.rec-call-node`.
4. Evaluating leaves `--heat`. Rejected and Exhausted stop sharing one filled chip.
5. Control borders move to `#6f6a60`. Card section rules may keep `#d5cfc2`.
6. Put the `lastRenderedStepIdx` guard inside `syncStepUI`. Signature-skip the four `innerHTML` rebuilds. Recompile sliders on `change`. Scrubber `step="1"`.
7. Remove `aria-live` from `#stageCaptionBar`. Announce status transitions only. Start with `playing: false`. Respect `prefers-reduced-motion`.
8. Restore a visible focus ring on the editor. Do not steal Space from a focused button.

### P1 — Gauge, caption, inspector

1. Rebuild the gauge as six stacked split tracks, outer to inner, West and East labeled in the text, 13px readouts, two-line wrap, `formatAlgebraicText`.
2. Mark the stack-top half with `aria-current` and an ink edge. Add the ten-sector control strip under the stack.
3. Legend: ENV, OP Left, OP Right, GATE, Idle. Same hues as `FRAME_KIND_META`.
4. Overflow row instead of a red R6.
5. One chronological order for the gauge and the frame list.
6. Rings first. “Step details,” frames, and calls become disclosures. Delete the duplicated fuel and control strings from the rings badge.
7. One stage toolbar: status, one sentence, 44px transport with play in the center, integer scrubber, speed. Camera views as a segmented control with `aria-pressed`.
8. Hide the equation span when it only repeats the control atom. Ellipsis the sentence to one line.

### P2 — Materials and appearances

1. One token module shared by `styles.css` and `theme.js`.
2. `prefers-color-scheme: dark` by aliasing those tokens. The well slate is the dark paper.
3. `prefers-contrast: more` darkens borders and uses `--ink` for every label.
4. Root font size in `rem`.
5. Decide corners: stay sharp for the editorial brand, or 12px continuous cards if the next pass is meant to feel like a system inspector.
6. Optional concentric SVG once the stacked tracks are readable at 12px.

## Acceptance checks for the iteration

- Every label and value in the inspector, caption, and transport is at least 12px and at least 4.5:1 on its painted background, including ENV and OP slot keys on their tints.
- Idle slots stay above 4.5:1 with opacity at 1.
- Evaluating is not red. Rejected and Exhausted are distinguishable without reading the word, and each word still clears 4.5:1.
- Play, step, reset, presets, camera, and call chips have a 44×44px hit box. Previous is disabled on step 1. Next is disabled on the last step.
- The gauge shows twelve slots in outer-to-inner order, with West and East and Left and Right visible as text. A `sum_list(2/3 args)` readout is fully visible or wraps, and it uses the same `×` as the frame list.
- R6 is red only when its own slots are an error state. Extra frames read as “+N frames past the die.”
- The control sector strip shows which of the ten kinds is current.
- Dragging the scrubber across one step rebuilds inspector DOM once. Dragging the fuel slider updates the number live and compiles once on release.
- VoiceOver does not speak on every step during playback. It does speak when status becomes Done, Rejected, or Exhausted.
- Space on “Run program” runs the program. Tab into the editor shows a focus ring.
- `prefers-reduced-motion: reduce` leaves the trace paused on load.
