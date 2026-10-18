# Quineling 04 — Material Design 3 UI, Code Editor & Visual Noise

Auditor: Grok 4.7 xhigh, Quineling Tester 4 of 5
Date: 2026-10-18
Evidence: `quineling_interior_factorial.png` (Inside Chip, Factorial, step around Rule A135), plus `index.html`, `src/styles.css`, `src/app.js`, with plaque placement checked in `src/designers/designer8_quineling.js` and `src/designers/theme.js`.
No application source was edited.

## Verdict

| Request | Result |
| --- | --- |
| Remove the `WebGPU` box | Done. No such node in the DOM. The still has a clean masthead. |
| Remove the `Outcome done true` box | Done. No such node. Status is a word in the caption and the Inspector. |
| Replace the white `#sourceEditor` scrollbar with an M3 code surface | Done in CSS. Thin outline thumb, transparent track, no arrow buttons. The still shows no white bar. |
| Align `.line-numbers` with `#sourceEditor` | Idle rows match (same font, size, line-height, 12px top padding; scroll is synced). The active-line rule will shove every number below the highlighted row. |
| Drop leftover 3D billboards | Not done. `plaqueMesh` on the Quineling core is still a large tilted card, clipped by the viewport and mirrored on orbit. The Inspector already shows this text. |

## 1. WebGPU box — removed

`index.html` has no masthead pills. The only `WebGPU` strings in the page are the import map (`three/webgpu`) and the renderer in `SherlockStage.js`. Those are not on screen.

The still’s masthead is type only: brand, title, deck. No renderer badge.

Dead CSS can bring the pill back if someone remounts it: `.masthead-badges` and `.status-pill` in `src/styles.css` (around lines 275–304). Delete those rules when touching this file. Do not add a renderer chip.

## 2. Outcome box — removed

There is no outcome card in `index.html`. The stage caption uses `#stageStatusBadge` (`Evaluating` / `Done` / `Rejected` / `Exhausted`). The Inspector `Status` cell repeats that word, and on rule line 80 it becomes `Done (${outcome.summary})` (`src/app.js`, `syncStepUI`).

The still shows `Evaluating` in the caption and under `STATUS`. It does not show `Outcome done true`.

`formatOutcomePill` in `src/app.js` (around line 285) is never called. Delete it so the old pill copy cannot be wired back up. `.qdl-stage-pill` in CSS is also unused. Leave the caption status badge and the Inspector `Status` cell; those are the real readout.

## 3. Editor scrollbar — replaced

`#sourceEditor` (`src/styles.css`, around lines 648–688):

- `scrollbar-width: thin`
- `scrollbar-color: rgba(120, 113, 102, 0.36) transparent` (M3 `--md-sys-color-outline` at 36%)
- WebKit thumb 6px, full corner radius, transparent track
- `::-webkit-scrollbar-button` forced to 0×0

The field sits in an outlined M3 card: `--md-sys-color-surface-container-lowest` page, `--md-sys-color-surface-container-low` gutter, `--md-sys-color-outline-variant` hairline, 12px corner, amber focus ring on `.editor-container:focus-within`.

The factorial still crops the editor before a long scroll, so the thumb itself is not in frame. No default white track or arrow buttons are visible on the rows that are in frame.

Leave this styling. One leftover in the same block: `.compile-error` uses `var(--error-bg)`, and `--error-bg` is never defined, so an error banner has no fill. Point it at a light error container (for example `rgba(196, 48, 43, 0.10)`).

## 4. Line numbers — aligned until the active row

Shared metrics, which is why lines 1–6 in the still sit on the code:

- Gutter and textarea: JetBrains Mono, `12.25px`, `line-height: 1.58`, padding-top `12px`
- Grid: `40px` gutter + `1fr` (` .code-editor-wrap`)
- `src/app.js` copies `scrollTop` from `#sourceEditor` onto `#editorLineNumbers`

The break is `.line-numbers .active-line`:

```css
display: inline-block;
width: 100%;
padding-right: 4px;
border-left: 2px solid var(--active-amber);
font-weight: 700;
```

An inline-block with the default baseline alignment makes that gutter row taller than a textarea row. The still’s toolbar says `Line 8`, and the crop stops at line 6, so the drift is below the frame. Every number after the active line will step down once that row is on screen.

Fix only the active-line rule:

- `display: block`
- `line-height: inherit`
- `padding: 0 4px 0 0` (no vertical padding)
- `margin: 0`
- keep the amber border and the secondary-container background
- do not change `font-weight` if the bold face changes the em box; color and the border are enough

Also set `min-height: 0` on both grid children and `overflow: hidden` on `.code-editor-wrap`. The textarea’s default `min-height: auto` can fight `max-height: 416px` and desync the two scrollports. Keep the existing `scroll` listener.

## 5. Visual noise — remove the Quineling billboard

The Inspector `Quineling Core` cell already reads, in the still: `Nautilus Arithmeticus · nautilus (logarithmic-mantle) · UNFURLING ORGAN #7 · DEPTH 0`. That card is the right surface.

The 3D stage still draws a second copy. In `src/designers/designer8_quineling.js` (`plaqueMesh`, around lines 1073–1182):

- `PlaneGeometry(0.74, 0.185)` parented to `coreRoot`
- local position `(0, -0.49, 0.34)`, `rotation.x = -0.18`
- `coreRoot` sits at `y = -2.05` (`QUINELING_CORE_Y`), so the card hangs near `y ≈ -2.54`, about 0.84 above the vault floor (`y = -3.38`), tilted toward the lens
- `MeshBasicMaterial`, `transparent`, `opacity: 0.95`, `depthWrite: false`, `side: THREE.DoubleSide`
- texture rebuilt from `createPlaqueTexture` with badge `` `${nodeCount} NODES` ``, title `NAUTILUS ARITHMETICUS`, subtitle `logarithmic-mantle · …`

That is the dark card cut off along the bottom of the canvas in the still (`…NODES · NAUTILUS ARITHMETICUS` / `logarithmic-mantle`). `.stage-viewport-card` is `overflow: hidden`, so the lower edge of the plane is clipped. `DoubleSide` shows the same UVs on the back face, so orbiting past it mirrors the type. The glossy floor repeats it again.

The refresh loop makes this worse: every few steps it calls `plaqueMat.map.dispose()` on a texture that `createPlaqueTexture` (`src/designers/theme.js`) may have just returned from the shared LRU cache. Disposing a cached map blanks anyone else still holding that entry.

Do this:

1. Delete `plaqueMesh`, its material, `refreshInspectMetadata`’s plaque tag, and the plaque-key block inside `update`.
2. Do not dispose textures that came from `createPlaqueTexture`.
3. Do not put a replacement billboard in the chamber. The Inspector cell is the species readout. A name on the die, if one is still wanted, should be a small floor engraving (well under 0.3 world units, `FrontSide` only, not a 512×128 HUD card).

Keep these. They are not the slop card:

- Floor hub `SHERLOCK · K-A/1 · 39-RULE ROM` in `designer2_well.js` (row 6 of the 39-module grid). It is a socket label, and the prose documents it.
- The 39 chiplet names (`A135 PushFrame`, and the rest). They are the rule modules.
- Designer 3 gallery badges (`0.40 × 0.096` at `BADGE_RADIUS = 1.01` in `designer3_frames.js`). In the still they are the small `R1·A` / `fact(4)` tags on the rings. They are a second `DoubleSide` plaque family, so they will also mirror if the camera swings behind them. Leave them for this pass unless a later shot shows them colliding with the organism. If they are edited, switch them to `FrontSide` and keep them on the collar. Do not enlarge them.
- The colored squares around the shell are the genome spores, not labels.

`createPlaqueTexture` ignores `textColor`, `mutedColor`, and `borderColor` (it reads `fg` / `textHex`, `muted`, and `accent`). That only matters for callers that remain after the Quineling card is gone. No need to restyle the deleted card.

## Fix order

1. Remove Designer 8 `plaqueMesh` and its per-step texture rebuild. Do not dispose cached plaque textures.
2. Change `.active-line` to a block row with no extra vertical padding, and lock both editor columns with `min-height: 0`.
3. Delete unused `.masthead-badges`, `.status-pill`, `.qdl-stage-pill`, and `formatOutcomePill`.
4. Define a background for `.compile-error`.

Re-shoot Inside Chip on Factorial after step 1. The lower canvas edge should show floor modules and the ROM hub only, with no tilted species card and no mirrored copy while orbiting. Then scroll the editor to the active line and confirm the gutter and the source stay on the same baselines.
