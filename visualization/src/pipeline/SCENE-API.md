# Scene API

A scene is one ES module, `scenes/sceneN.js`, with a default export:

```js
export default { id, title, beats: [{ id, title?, duration, narration, labels }], build(ctx), seek(t), dispose() }
```

Scene 8 is the only one that draws the whole bench. Every other scene shows its own station.

## Rules

- `seek(t)` receives seconds from the start of the scene (0 to the sum of the beat durations). It sets the whole scene from `t` alone: no state kept from earlier frames, no random values without a fixed seed, no physics. The same `t` always gives the same frame.
- `build(ctx)` runs once, before any `seek`. Create meshes under `ctx.scene`. Call `ctx.camera.track(...)` once.
- Beat `duration` is reading time: words divided by 2.5, rounded up, at least 12. The key motion starts within 2 s and ends 2 s before the beat ends. The last 2 s or more hold a still pose.
- `narration` is full prose for the panel. A blank line splits paragraphs. Allowed markup: `<sup>`, `<code>`, `<em>`.
- Scene coordinates are local: the station spans x from -16 to 16 and z from -6 to 6, the bench top is y = 0, y is up, the viewer looks from +z. The shell places the group at the station.

## ctx

| Member | Meaning |
|---|---|
| `ctx.scene` | `THREE.Group` at this station. Add everything here. |
| `ctx.beatAt(t)` | `{ index, id, t, start, duration }`: the beat at scene time `t`, with `t` local to the beat. |
| `ctx.duration` | Scene length in seconds. |
| `ctx.plates` | `{ in, out }`: `Vector3` positions (scene-local) of the handoff plates at the left and right edge of the station. The out plate of scene N and the in plate of scene N+1 are the same spot on the bench. Draw the handoff object on them in your first and last pose. |
| `ctx.camera.shots` | Pose builders: `establish(target, span)`, `inspect(target, span, 'top' or 'front')`, `follow(target, offset)`. A pose is `{ pos, target, fov }` in scene-local coordinates. |
| `ctx.camera.track(keys)` | `keys = [{ t, pose, dur = 2, cut = false }]`. From scene time `t` the camera moves to `pose` over `dur` seconds, cubic in and out. Before the first key the camera holds the first pose; the first move starts from the last pose of the previous scene unless `cut: true`. Pure in time, so seek is exact. In reduced-motion mode every move is a cut. |
| `ctx.labels.show(id, html, anchor, opts)` | Call inside `seek`. Shows a short label at `anchor` (scene-local `[x,y,z]`, `Vector3` or `Object3D`). `opts`: `kind` (`program`, `input`, `outcome`, `step`, `state`, `error`, `neutral`: sets the colour chip), `dy` (world units above the anchor). At most six scene labels show at once. At most four words. |
| `ctx.labels.anchor(name, obj)` | Names an `Object3D` so `beat.labels` entries can use `anchor: name`. |
| `beat.labels` | Static labels for a beat: `{ id?, text, anchor or at, kind, dy?, from?, to? }`. `from` and `to` are seconds into the beat. |
| `ctx.carrier.show(name, amount, content?)` | Call inside `seek`. Token carrier at the front edge of the bench: `name` is `prog_d`, `input_d`, `outcome_d` or `n_steps`; `amount` 0 to 1 is presence. `content` is a string that sets the dot face. Without a call, the shell shows `prog_d` and `input_d` from scene 2 on, and all four from scene 6 on. Scene 1 and scene 5 bring in their own tokens. |
| `ctx.showAll(flag)` | Call inside `seek`. Draws every other station in its final state (the wide shot of scene 8). |
| `ctx.reducedMotion` | `true` in stepped mode. The shell then seeks to the end of each beat. |
| `ctx.theme` | `theme.js`: `PALETTE` (M3 dark values), `M3`, `FAMILY`, `materials`, `makeDigestToken`, `digestPattern`, `makeCube`, `phase`, `ramp`, `lerp`, `easeInOut`, `clamp01`. |

## Digest tokens

`makeDigestToken({ family, content, size })` returns a hexagonal token (face toward +z) whose 5 by 5 dot pattern comes from `content`. Equal strings give equal faces. `token.userData.setContent(str)` changes the face. Families: `program` (violet), `input` (blue), `outcome` (teal), `state` (gold). Rotate the group by `-Math.PI / 2` about x to lay it face up on the bench.

## URL

`pipeline.html?scene=N&beat=B&t=T` shows scene N (1 to 8), beat B (0-based), T seconds into the beat, paused, then sets `window.__ready = true`. `&webgl` forces the WebGL backend. `&stepped` forces the stepped mode.

## Colors: use the M3 roles, not raw hex

The page is Material 3, dark only. One seed, `#d4af37` (the gold), gives every tonal palette (`m3.css` holds the CSS roles, `m3palette.js` the canvas values). Scenes take colors from `theme.PALETTE` and `theme.FAMILY` and never write a hex literal. The mapping:

| Meaning | PALETTE key | M3 role (dark) |
|---|---|---|
| Ground, prover side (stations 1 to 5) | `paper` | surface-container-high |
| Bench slabs, prover side | `paperWarm` | surface-container-highest |
| Ground, chain side (stations 7 and 8) | `wellSlate` | surface-container-lowest |
| Bench slabs, chain side | `graphite` | surface-container-low |
| Ceramic: tiles, housings, token caps | `ceramic` | neutral tone 90 |
| Titanium: slabs, rails | `titanium` | neutral-variant tone 80 |
| Marks and dots drawn on ceramic | `ink` | neutral tone 10 |
| Line work on the dark ground | `onSurface` | on-surface |
| Hashing, digests of state, proofs, keys, accumulator | `gold`, lit `goldBright` | primary (tone 80), primary tone 90 |
| Program (`prog_d`, ROM) | `gateViolet` | custom `program` (harmonized), tone 80 |
| Input (`input_d`) | `steel`, lit `steelBright` | custom `input`, tone 80 and 90 |
| Outcome (`outcome_d`) | `resolve`, lit `resolveBright` | custom `outcome`, tone 80 and 90 |
| Running step | `heat`, lit `heatBright` | custom `step`, tone 80 and 90 |
| Refusal | `errorRed` | error (tone 80) |

Custom colors are harmonized toward the seed hue (at most 15 degrees) and take the same tones as the primary roles, so they sit in the scheme the way M3 custom colors do. DOM labels use `--md-custom-program`, `--md-custom-input`, `--md-custom-outcome`, `--md-custom-step`, `--md-sys-color-primary` and `--md-sys-color-error` for the `kind` chip. Label text is `on-surface` on `surface-container-highest`, and all of these pairs pass WCAG 2.2 AA (4.5:1 and above for text).
