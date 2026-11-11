# Scene contract: the build rules for `scenes/sceneN.js`

One builder builds one scene. This file says what the builder owns, what it may import, how the scene must behave and how it is accepted. The content of each scene is in `STORYBOARD.md`. The shell interface is in `visualization/src/pipeline/SCENE-API.md`. The facts are in `BRIEF.md`. The design system is in `visualization/DESIGN.md` and `visualization/PRODUCT.md`.

Paths below are relative to the build worktree. `P/` is `visualization/src/pipeline/`.

## 1. Ownership

| Builder | Owns (may create and edit) | Scene |
|---|---|---|
| Scene 1 | `P/scenes/scene1.js`, `P/scenes/scene1/**` | The program |
| Scene 2 | `P/scenes/scene2.js`, `P/scenes/scene2/**` | The run |
| Scene 3 | `P/scenes/scene3.js`, `P/scenes/scene3/**` | The leaf |
| Scene 4 | `P/scenes/scene4.js`, `P/scenes/scene4/**` | The node |
| Scene 5 | `P/scenes/scene5.js`, `P/scenes/scene5/**` | The decider |
| Scene 6 | `P/scenes/scene6.js`, `P/scenes/scene6/**` | The transaction |
| Scene 7 | `P/scenes/scene7.js`, `P/scenes/scene7/**` | The contract |
| Scene 8 | `P/scenes/scene8.js`, `P/scenes/scene8/**` | The network |

- A builder edits only its own file and its own folder. It replaces the placeholder `sceneN.js` that the shell ships.
- A builder never edits `P/shell.js`, `P/theme.js`, `P/labels.js`, `P/m3palette.js`, `P/m3.css`, `P/SCENE-API.md`, `visualization/pipeline.html`, another scene, or anything under `visualization/vendor/` or `visualization/src/leaf/`.
- A builder that needs a shell change does not make it. It reports the need in its final message, with the exact member it wants and why, and builds against the shell as it is.
- No new dependency, no npm install, no file outside `visualization/`.
- No commits to `main`, no push. Rebase the worktree on `main` before the first edit.

## 2. Imports

A scene file may import only these:

| Module | What to take |
|---|---|
| `three/webgpu` | `THREE` classes: `Group`, `Mesh`, `InstancedMesh`, `BoxGeometry`, `CylinderGeometry`, `TorusGeometry`, `PlaneGeometry`, `BufferGeometry`, `Line`, `LineSegments`, `LineBasicMaterial`, `MeshStandardMaterial`, `MeshBasicMaterial`, `Color`, `Vector3`, `Matrix4`, `Quaternion`, `Object3D`. |
| `three/tsl` | Only when a node material is needed and the same frame renders under `?webgl`. A scene that can do without it does without it. |
| `../theme.js` | `PALETTE`, `FAMILY`, `materials`, `makeDigestToken`, `digestPattern`, `makeCube`, `phase`, `ramp`, `lerp`, `easeInOut`, `clamp01`. The same members are on `ctx.theme`. |
| `../labels.js` | `sup`, `pow2`, `ident`, `MAX_SCENE_LABELS`. |
| `../m3palette.js` | Read only, and only for a role that `theme.js` does not map. Prefer `theme.js`. |
| `./sceneN/*.js` | The builder's own helper modules. |

Not allowed: raw GLSL `ShaderMaterial` or `RawShaderMaterial`, compute shaders, post-processing passes, textures loaded from the network, canvas textures that hold text, `Math.random` without a seeded generator, `Date.now` or `performance.now` inside `seek`, timers, event listeners on `window` or `document`, DOM writes outside `ctx.labels`.

### Scene 3 and the earlier leaf chip view

`visualization/src/leaf/leafChip.js` is an SVG view of an older leaf model. Scene 3 does not import it: the module mounts DOM, and its rule set, register file and genesis are not the facts of `BRIEF.md`. Scene 3 reuses these parts by copying them into `P/scenes/scene3/`:

- `polyLen(pts)` and `pointAt(pts, f)` (the two pure helpers that place a packet at fraction `f` of a polyline route). Use them, extended to 3D points, for every packet that travels the bus.
- The floor-plan arrangement of `build(layoutKey)` and the `LAYOUTS` table: the 16 step slots as a ribbon along the top, CORE as the large block, the provider chips around one bus trunk with a branch to each chip, and a route table `routes[name]` from the CORE port to each chip. `STORYBOARD.md` scene 3 gives the 3D coordinates.
- The 8-cell tail tray that fills from the left and flushes when full.

Scene 3 takes no number, no register name and no rule name from that file.

## 3. Behaviour every scene must have

1. **Default export.** `{ id, title, beats, build(ctx), seek(t), dispose() }` as in `SCENE-API.md`. `id` and `title` are those of `STORYBOARD.md`. Each beat has `id` (the slug), `title` (the Title line of the storyboard beat), `duration`, `narration` and `labels`.
2. **Narration.** The text of each beat is copied from `STORYBOARD.md` without change. Paragraphs are split by a blank line. Text in backticks becomes `<code>`. `<sup>` and `&lt;` stay as written.
3. **Deterministic seek.** `seek(t)` sets every position, rotation, scale, visibility, colour, emissive value, instance matrix, label and carrier amount from `t` alone. It keeps no state between calls. Calling `seek(a)`, then `seek(b)`, then `seek(a)` gives the same frame as the first call. `build` creates every object that any beat uses; `seek` creates nothing and disposes nothing.
4. **Structure of `seek`.** Find the beat with `ctx.beatAt(t)`. Set every object to its pose at the end of the beat before (a pure function of the beat index), then apply the motions of the current beat from `b.t`. An object that a beat does not use is hidden or dimmed as the storyboard says.
5. **Still pose.** At `b.t >= duration - 2` every motion of the beat is finished. In stepped mode the shell seeks to the end of each beat, so the end pose must show the whole idea of the beat.
6. **Camera.** One `ctx.camera.track(keys)` call in `build`, with the poses that the storyboard names. No camera access anywhere else. The camera holds still during a key motion. The storyboard marks the three exceptions: the follow shots of beats 3.6 and 6.4 and the pull-back of beat 8.8.
7. **Labels.** At most six scene labels at once, at most four words each or one identifier with one value. Static labels go in `beat.labels` with `anchor` names registered through `ctx.labels.anchor`. A label that follows a moving object uses `ctx.labels.show` inside `seek`.
8. **Carrier and plates.** Call `ctx.carrier.show` only as the storyboard says, with the content strings of storyboard section 0.8. Draw the handoff object on `ctx.plates.in` in the first pose and on `ctx.plates.out` in the last pose, in the size and orientation the storyboard gives.
9. **Colour.** No hex literal, no `new Color('#...')`, no CSS colour name. Every colour comes from `PALETTE`, `FAMILY` or `materials`. A dimmed object lerps toward the ground colour; it does not use opacity.
10. **Bounds.** Every object stays inside x -16 to 16 and z -6 to 6, except the handoff object on its plate. Nothing is below y = 0. The one other exception is the gold trace line of scene 8, beat 8.8, which runs the length of the bench.
11. **Dispose.** `dispose()` disposes every geometry and material the scene created and clears its references.
12. **Page register.** Every string a viewer can read follows `SLOP-RULES.md` and `BRIEF.md` section 1: present tense, the system as built, no status word, no account of how the work was made, no number that is not in `BRIEF.md` sections 2 and 3. The builder adds no viewer text of its own.

## 4. Performance budget

Measured on the WebGL2 path (`?webgl`), at 1600 by 900, with the scene alone on screen. Read the counts from `ctx.renderer.info.render` after a frame.

| Measure | Budget per scene | Scene 3 | Scene 8, wide shot only |
|---|---:|---:|---:|
| Draw calls per frame | 120 | 180 | 260 |
| Triangles per frame | 120,000 | 200,000 | 300,000 |
| Geometries created in `build` | 60 | 90 | 60 |
| Materials created in `build` | 40 | 60 | 40 |
| Time of one `seek(t)` call (median over 600 calls spread over the scene) | 1.5 ms | 2.5 ms | 1.5 ms |
| Frame time while playing, median | 16.7 ms on an integrated GPU; on the headless software renderer of this host, at most 50 ms |

How to stay inside it:

- Every repeated object is one `InstancedMesh`: cells, tiles, tape frames, pins, rule positions, ruler ticks, token dots. Set instance matrices and instance colours in `seek`.
- Share geometry and material instances. Create a material copy only for an object whose colour or emissive value changes on its own.
- Hide what a beat does not show (`visible = false`), so it costs no draw call.
- Lights belong to the shell. A scene adds no light and no shadow.
- No allocation inside `seek`: reuse module-level `Vector3`, `Matrix4`, `Quaternion` and `Color` scratch objects.

The builder reports the four measured numbers for its scene in its final message.

## 5. Acceptance checks

The site server runs at `http://127.0.0.1:4377/` and serves the build worktree's `visualization/`. The capture tool is:

```
/home/hoskinson/.local/share/uv/tools/scrapling/bin/python /home/hoskinson/.cache/pipeline-anim/tools/shot.py "URL" OUT.png W H
```

It prints the page title and the last console lines. Run captures one at a time. Write them to `/home/hoskinson/.cache/pipeline-anim/shots/sceneN/`.

A scene is accepted when all of these hold:

1. **Every beat, wide.** For each beat index B from 0 to the last, capture the still pose at 1600 by 900: `http://127.0.0.1:4377/pipeline.html?scene=N&beat=B&t=T&webgl`, with T = the beat's duration minus 1. File name `bBB-end.png`.
2. **Every beat, mid-motion.** For each beat, one capture at T = half the duration (`bBB-mid.png`), to show that the key motion is under way or finished and that nothing is misplaced between poses.
3. **Every beat, phone.** For each beat, capture the still pose once at 390 by 844 (`bBB-phone.png`). The subject is in frame, no label is cut by the edge of the stage, and labels do not cover each other.
4. **Zero console errors.** No capture prints a line that starts with `error:`, `pageerror:` or `timeout`. Warnings from the browser about the software renderer are allowed; warnings from Three.js about the scene's own materials or geometries are not.
5. **Byte-identical repeat.** Capture the still pose of the first beat, the middle beat and the last beat a second time. `cmp` reports no difference from the first capture of each.
6. **Seek order.** In the browser console or a small test page, `seek(a); seek(b); seek(a)` for ten pairs `(a, b)` spread over the scene gives the same object matrices for `a` both times (compare `matrixWorld` of every mesh, tolerance 1e-9).
7. **Stepped mode.** `...?scene=N&beat=B&webgl&stepped` shows the same still pose as check 1 for three beats of the builder's choice.
8. **Handoff match.** The first pose of the scene shows the handoff object on the in plate as the storyboard describes it. The last pose shows the handoff object on the out plate. The lead compares it with the neighbour scene at integration.
9. **Storyboard match.** For each beat: the named objects are present at the stated sizes and positions (the builder may move an object by up to 1 unit to clear a collision or a covered label, and lists each such move in its final message), the key motion is the stated one, the labels are the stated text on the stated objects, and the narration is byte-equal to the storyboard text (after the backtick to `<code>` conversion).
10. **Colour check.** `grep -nE "#[0-9a-fA-F]{3,8}\b|0x[0-9a-fA-F]{6}\b|rgb\(" P/scenes/sceneN.js P/scenes/sceneN/` prints nothing.
11. **Design check.** `~/.claude/skills/impeccable/scripts/impeccable detect --json P/scenes/sceneN.js P/scenes/sceneN/` on the builder's files reports zero findings. The builder loads the `impeccable` skill and reads its `reference/craft-floor.md` before the first edit.
12. **Text check.** The checks of `SLOP-RULES.md` section 7, run on the scene's beat titles, narration and labels, print nothing that the rules forbid.
13. **Budget.** The numbers of section 4 are inside the budget, measured at the beat with the most objects in view.

The builder's final message lists: the files written, the result of each check, the four budget numbers, the capture folder, and every point where it could not follow the storyboard, with the reason.

## 6. Capture URL lists

Beat indices are 0-based. The still pose of beat B is at `t = duration - 1`. The lists below give the URL pattern and the number of beats; the durations are in `STORYBOARD.md` at each beat heading and in the table below, which includes the changes of `STORYBOARD-AMEND-1.md`.

```
BASE=http://127.0.0.1:4377/pipeline.html
wide:   $BASE?scene=N&beat=B&t=<duration-1>&webgl          1600 900
mid:    $BASE?scene=N&beat=B&t=<duration/2>&webgl          1600 900
phone:  $BASE?scene=N&beat=B&t=<duration-1>&webgl           390 844
step:   $BASE?scene=N&beat=B&webgl&stepped                 1600 900
```

| Scene | `id` | Beats | Beat indices | Durations in beat order (s) | `where` per beat | Captures (wide + mid + phone) |
|---:|---|---:|---|---|---|---:|
| 1 | `program` | 6 | 0 to 5 | 29 22 22 22 29 25 | all `off` | 18 |
| 2 | `run` | 10 | 0 to 9 | 28 24 24 24 24 23 23 21 23 24 | all `off` | 30 |
| 3 | `leaf` | 35 | 0 to 34 | 34 23 25 22 24 24 29 24 31 34 25 24 24 24 22 22 24 24 24 24 24 24 24 23 24 24 24 24 25 25 24 24 24 24 29 | all `off` | 105 |
| 4 | `node` | 17 | 0 to 16 | 27 24 22 16 20 19 22 23 22 18 20 27 25 23 24 23 20 | all `off` | 51 |
| 5 | `decider` | 14 | 0 to 13 | 24 22 24 22 19 22 24 20 24 22 24 23 29 20 | all `off` | 42 |
| 6 | `transaction` | 4 | 0 to 3 | 32 23 27 30 | `off` `off` `off` `cross` | 12 |
| 7 | `contract` | 21 | 0 to 20 | 33 27 21 29 22 22 24 29 26 31 24 26 25 25 25 22 26 28 26 29 25 | beats 0 to 5 `on`, 6 to 19 `off`, 20 `on` | 63 |
| 8 | `network` | 9 | 0 to 8 | 26 26 22 25 25 27 24 32 56 | beats 0 to 2 `on`, 3 and 4 `off`, 5 to 8 `on` | 27 |

`STORYBOARD-AMEND-1.md` wins over `STORYBOARD.md`: 116 beats, 2,874 s (47:54). Scene 8 has nine beats; the ninth, `what-lives-where` (index 8, 56 s), carries the inventory card and holds the 8.8 wide pose. The captures of 8.8 and 8.9 are taken with the gate and both rail colours in frame; the chip must read the value in the `where` column, and the zone labels show whenever the gate is in frame. Check 4 adds: the chip text of a capture equals the text of its `where` value (`Off-chain: your machine`, `Crossing: the transaction`, `On-chain: Midnight`), and no scene draws a threshold bar of its own.

A loop that makes the wide set for one scene, with the durations taken from the scene module itself:

```
PY=/home/hoskinson/.local/share/uv/tools/scrapling/bin/python
SHOT=/home/hoskinson/.cache/pipeline-anim/tools/shot.py
N=4; OUT=/home/hoskinson/.cache/pipeline-anim/shots/scene$N; mkdir -p $OUT
node -e "import('./visualization/src/pipeline/scenes/scene$N.js').then(m=>console.log(m.default.beats.map(b=>b.duration).join(' ')))" 2>/dev/null \
  || echo "read the durations from STORYBOARD.md"
B=0; for D in <the durations of scene N from the table above>; do
  $PY $SHOT "http://127.0.0.1:4377/pipeline.html?scene=$N&beat=$B&t=$((D-1))&webgl" $OUT/b$(printf %02d $B)-end.png 1600 900
  B=$((B+1)); done
```

The scene module imports `three/webgpu` through the page's import map, so the `node -e` line works only where that map is set up; otherwise copy the durations from the storyboard.
