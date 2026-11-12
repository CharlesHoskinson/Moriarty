# Storyboard: the Sherlock pipeline animation

This file is the build instruction for the eight scenes. `BRIEF.md` holds the facts, `CREATIVE-BRIEF.md` the vision, `visualization/src/pipeline/SCENE-API.md` the shell interface and `SCENE-CONTRACT.md` the rules of the build. Where this file and `CREATIVE-BRIEF.md` differ, this file wins: it is later and it follows the Material 3 dark scheme.

The title and the narration text under each beat are final. Copy it exactly. Do not rewrite it, shorten it or add to it.

## 0. Rules for every scene

### 0.1 Running time

| Scene | Title | Beats | Length |
|---:|---|---:|---:|
| 1 | The program | 6 | 142 s (2:22) |
| 2 | The run | 10 | 233 s (3:53) |
| 3 | The leaf | 35 | 857 s (14:17) |
| 4 | The node | 17 | 370 s (6:10) |
| 5 | The decider | 14 | 311 s (5:11) |
| 6 | The transaction | 4 | 93 s (1:33) |
| 7 | The contract | 21 | 530 s (8:50) |
| 8 | The network | 8 | 202 s (3:22) |
| | Total | 115 | 2738 s (45:38) |

Exhaustive detail wins over the first target lengths. Every beat lasts its narration word count divided by 2.5, rounded up, with a minimum of 12 s. The durations below are computed from the final text; use them as written.

### 0.2 Space, time and notation

- Coordinates are station-local, in scene units: x from -16 to 16 (left to right), z from -6 to 6 (back to front, the viewer looks from +z), bench top at y = 0, y up. The handoff plates are at `ctx.plates.in` = (-20, 0.2, 0) and `ctx.plates.out` = (20, 0.2, 0); each is 4 by 4 units. Keep every other object inside x -16 to 16 and z -6 to 6.
- A position is written (x, y, z). A size is written w by h by d (x by y by z).
- Times in "Objects and motion" are seconds into the beat. "2.0 to 3.2: A moves from P to Q" means `lerp(P, Q, phase(b.t, 2.0, 3.2))`. Before the start time the object is at P, after the end time at Q. That is the whole rule of determinism: every property is a function of the beat index and `b.t`.
- An object that is "parked" (a lid, a cowl, a wall lifted out of the way) is hidden once it reaches its parked height: it scales to 0 over the last 0.4 s of its lift, and it appears the same way before it descends. A parked object never stands between a camera and the bench.
- State carries across beats by definition, not by memory: in beat k every object is in the end pose of beat k-1 unless the beat text moves it. Write `seek` as: compute the end pose of all earlier beats (a table per object), then apply the motions of the current beat.
- The beat timeline: 0 to 1.5 the camera move, if the beat has one. The key motion starts at 2.0 unless the beat says otherwise and ends at least 2 s before the beat ends. The last 2 s hold the still pose. The still pose must explain the beat without motion: it is the frame of the stepped mode and of the captures.

### 0.3 Colour and material (Material 3, dark only)

The shell draws the ground, the bench slabs, the rail and the plates. A scene never draws ground. Scenes take every colour from `ctx.theme.PALETTE`, `ctx.theme.FAMILY` and `ctx.theme.materials`, and never write a hex literal.

| Use | Material call | PALETTE key |
|---|---|---|
| Housings, tiles, trays, tape, token caps, cubes at rest | `materials.ceramic()` | `ceramic` |
| Proof slabs, rails, frames, rims, rulers | `materials.titanium()` | `titanium` |
| Hash blocks, key cards, the accumulator ring, slab edges, state digests | `materials.gold()` | `gold`; `goldBright` as emissive only while a check passes |
| Marks, pips and dots on ceramic | `materials.ink()` | `ink` |
| Lines drawn on the dark bench: routes, connectors, ruler ticks, break marks | unlit line or thin box, colour `PALETTE.onSurface` | `onSurface` |
| Program objects: source sheet, ROM rack, ROM lines | `materials.family('program')` | `gateViolet` |
| Input objects | `materials.family('input')` | `steel`, lit `steelBright` |
| Outcome objects | `materials.family('outcome')` | `resolve`, lit `resolveBright` |
| The step in progress: firing rule, moving request, touched tile | `materials.family('step', { lit })` | `heat`, lit `heatBright` |
| Refusal hatch | `materials.family('error')` | `errorRed` |
| Chain-side objects (stations 6 right half, 7, 8): cabinet, envelope body, node trays | `materials.slate()` with a titanium rim | `graphite` |

Emissive light means state and nothing else: the active rule, the request in flight, a passing check (0.3 s), a refusal hatch. Idle objects are unlit and still. No bloom, no transparency sorting tricks, no particles. Dimming an object means lerping its colour toward `PALETTE.paperWarm` (prover side) or `PALETTE.graphite` (chain side) by 0.6, not lowering opacity.

Lines on the bench are thin boxes (0.04 high, 0.06 wide) or `THREE.Line` with `LineBasicMaterial`, both of which work on WebGL2. A connector is a line from object to object that appears by growing from its source end.

### 0.4 The shared kit

Every scene builds its own copies of these objects, to these sizes, so that the stations match. Sizes are in scene units.

| Object | Geometry and size | Material |
|---|---|---|
| Cell (field element, register) | Plain box, 0.4 cube (the vendored build has no rounded box). Cells in a row have pitch 0.5. | ceramic; family colour when the cell belongs to a family |
| Digest token | `makeDigestToken({ family, content, size })`. Sizes: 0.8 on a slab or in a row, 1.2 standing alone, 2.0 as the subject of an inspect shot. Laid face up on the bench (rotate -π/2 about x) in top views, upright in front views. | from the helper |
| Hash block (Poseidon) | Box 2.4 by 1.0 by 1.2, with an input slot on its -x face and an output slot on its +x face (inset boxes 0.1 deep, ink). | gold |
| Bus tuple | Five cells in a row along x, pitch 0.45, total length 2.2. Cell 1 is the chip cell: it carries 1 to 4 ink pips on its top face (1 ALU, 2 MEM, 3 HASH, 4 ROM). | chip cell `family('step')` while in flight, else ceramic; the other four ceramic |
| Memory record | Tile 0.8 by 0.12 by 0.8 with one ink bar 0.5 by 0.02 by 0.1 on top. | ceramic |
| Segment tray | Open tray 7.4 by 0.2 by 1.1 holding 8 tiles at pitch 0.9. | titanium rim, ceramic tiles |
| Memory tree column | Gold hash nodes (boxes 0.5 by 0.3 by 0.5) on a vertical line: three levels drawn above the trays, a break mark (two parallel `onSurface` strokes at 20 degrees), then two levels and the root token (size 1.2). A depth ruler stands beside it. Never 63 literal levels. | gold, `onSurface` |
| Witness frame | Plate 0.9 by 0.06 by 1.2. On it: a rule plate 0.3 by 0.08 by 0.5 at the centre, three 0.18 cubes on the -x side (state before), three on the +x side (state after). Frames on the tape have pitch 1.0. | ceramic plate, rule plate `family('step')`, cubes ceramic |
| Stack link | A token of size 0.6 (family `state`) joined to the link below by a gold bar 0.1 by 0.1 by 0.5. Chains grow along -z or downward in front views. | gold |
| Proof slab | Box 3.0 by 0.3 by 2.0 on a gold edge plate 3.14 by 0.1 by 2.14 that sits under it. Pins: boxes 0.22 by 0.22 by 0.3 on the front face (+z), evenly spaced: 7 on a leaf slab, 5 on a node slab, 4 on the decider seal. A pin takes its family colour when the beat names it. | titanium, gold edge |
| Accumulator ring | Torus, radius 0.7, tube 0.08, lying flat on the slab top, with a gap of 30 degrees (the unclosed joint) toward +z. A fold mark is a box 0.1 by 0.12 by 0.2 on the ring. | gold |
| Key card | Box 1.6 by 1.0 by 0.08, upright, with an ink notch pattern (3 notches for leaf, 2 for node, 1 for decider) along its bottom edge. | gold |
| Key rack | Titanium rail 7 by 0.15 by 0.15 at y = 4.2, z = -5.4, on two posts; three cards hang at x offsets -2.2, 0 and 2.2 (leaf, node, decider). | titanium |
| Ruler | Titanium strip, 0.1 high and 0.3 deep, with `onSurface` ticks. A logarithmic ruler has equal tick spacing per power of two; a break mark shows omitted range. | titanium |
| Inset tray | A rimmed tray, 9 by 0.1 by 4.5, that holds a comparison or a second example. It sits behind or beside the main line, and rises 0.1 from the bench (0 to 0.6 s) when its beat begins. Objects of the main line never enter it. | titanium rim, `paperWarm` or `graphite` floor |
| Refusal hatch | Three parallel bars, 0.08 by 0.02, at 45 degrees across the top face of the refused object. | `family('error')`, emissive 0.6 |

### 0.5 The fixed motions

These have one form in every scene. Durations are exact.

- **Hash** (3.0 s). 0 to 0.6: the input objects move into a single file on the -x side of the block, in the stated order, the first input nearest the block. 0.6 to 1.8: they slide into the input slot one after the other (each takes 0.3 s; scale to 0 at the slot). 1.8 to 2.2: the block's emissive rises to `goldBright` 0.8 and falls to 0. 2.2 to 3.0: the output token moves 1.6 units out of the +x slot and scales from 0.2 to its size. Inputs that the narration keeps (a token that stays in the carrier, a field that stays readable) are copies: the original stays where it is and a copy of it travels.
- **Equality** (1.2 s). The right-hand object moves in a straight line onto the left-hand object. In the last 0.2 s the two outlines are one: hide the moving object and show a gold outline ring (a torus or box frame 0.05 thick, 10 percent larger than the object, emissive `goldBright` 0.5 for 0.3 s, then plain gold). Equal digest tokens have equal `content` strings, so their faces match before they meet.
- **Verification** (0.7 s). A bar of `goldBright` (0.08 wide, emissive 1.0) crosses the object's top face from -x to +x in 0.4 s. Then the object's gold edge is emissive `goldBright` 0.8 for 0.3 s and returns to gold.
- **Refusal** (0.8 s). The object moves toward its goal, stops 0.5 units short (0 to 0.3 s), returns 0.4 units (0.3 to 0.6 s), and the refusal hatch appears (0.6 to 0.8 s) and stays.
- **Handoff slide.** A straight move along x at bench level, 1.6 to 2.4 s, cubic in and out. The object ends centred on the out plate. The first beat of the next scene draws the same object, centred on its in plate, in the same orientation.
- **Appear and leave.** An object that is not carried in appears by scaling from 0 to 1 over 0.4 s at its place. It leaves the same way.

### 0.6 Camera

Use only `ctx.camera.shots.establish`, `inspect` and `follow`. Times in a beat's Camera field are seconds into the beat; add the beat's scene start time (given in each beat heading) to get the `t` of the key. Each scene names its poses once (the table at the head of the scene) and each beat names one pose. Declare all keys in one `ctx.camera.track([...])` call in `build`: one key at the start time of each beat whose pose differs from the beat before, `dur: 1.5`. The first key of a scene has `dur: 2` and arrives from the last pose of the scene before. A beat whose text says "cut" sets `cut: true`. The camera never moves during a key motion; the only exceptions are the follow shots, which this file marks.

Framing: the stage is about 1.6 to 1 on a wide screen and on a phone. Keep the subject of a beat inside the middle 80 percent of the frame width and the middle 70 percent of its height, because labels need the margin.

### 0.7 Labels

- The label text is final. Text in backticks is an identifier: write it with `ident()` from `labels.js` (it becomes `<code>`). Powers use `<sup>`.
- The `kind` sets the colour chip: `program`, `input`, `outcome`, `step`, `state`, `error`, `neutral`. Where a label line gives no kind, use `state` for digests, proofs, keys and hashes, `step` for the firing rule and requests, `program`, `input` and `outcome` for their families, `error` for refusals and `neutral` for everything else.
- "on:" names the object the label is anchored to. Put the label above the object (`dy` 0.6 to 1.2) so that it does not cover it.
- A label appears with its object and leaves with it. Give `from` when the object appears after the start of the beat.
- The shell labels the four carrier tokens. Where a label below is "on: the carrier", the scene does not create it: the shell's label is that label. Such labels do not count toward the six.

### 0.8 Carrier and handoff

The token carrier belongs to the shell. A scene only calls `ctx.carrier.show(name, amount, content)` in `seek`. The content strings are fixed for the whole piece, so that faces match across scenes:

| Token | `content` string | Family |
|---|---|---|
| `prog_d` | `'prog:main'` | program |
| `input_d` | `'input:main'` | input |
| `outcome_d` | `'outcome:main'` | outcome |
| `n_steps` | `'n_steps:main'` | state |
| `claim_d` of the main run | `'claim:main'` | state |
| `val_d` of the main input | `'val:input'` | input |
| Boundary digest between slice i and slice i+1 | `'state:' + i` (the genesis state is `'state:init'`, the final state `'state:final'`) | state |
| A second program, a second run | `'prog:other'`, `'outcome:other'` and so on | as above |

Any other digest uses a content string of the form `'<name>:<what it commits to>'`, and two tokens that the narration calls equal use the same string.

Handoff objects are drawn by both neighbours: the last pose of scene N and the first pose of scene N+1 show the same object, centred on the shared plate, in the same orientation and size.


## Scene 1: The program

File `scenes/scene1.js`. 6 beats, 142 s (2:22). Scene starts at 0:00 of the piece.

Scene id `program`. Arrives: nothing (the piece starts here). Leaves: the ROM rack on the out plate, with `prog_d` and `input_d` in the carrier.

**Ideas taken.** The press that turns the sheet into shelved code tables is from concepts 1 and 3. The two programs under one key rack in a single still is from concept 2 (the split comparison); it is the clearest picture of "the program is data, not circuit". The empty third and fourth carrier sockets, which set up scene 5, are from concept 3. The 8-cube counter collar for a 64-bit quantity is from concept 2; this scene uses it for the fuel halves and the stack limit, and scenes 2, 4 and 5 use the same object for step counts and heights.

**Set.** All objects are built in `build`. Positions are centres.

| Name | Object | Position | Size and material |
|---|---|---|---|
| `sheet` | Source sheet with 11 rule bars (the 11 lines of the example: 2 for `lift`, a gap, 8 for `main`; bar lengths 1.2 to 3.2) | (-12.5, 0.04, -0.5) | Plate 4.0 by 0.08 by 5.4, ceramic; bars 0.06 high, `family('program')` |
| `inp` | Input value cube | (-12.5, 0.3, 4.4) | Box 0.6, `family('input')` |
| `compiler` | Press: a base and an upper platen with a slot between them | (-7.5, 0.9, -0.5) | Base 3.0 by 0.5 by 5.6 at y 0.25; platen 3.0 by 0.8 by 5.6 at y 1.3 (rest), ceramic, titanium rim |
| `rom` | ROM rack: frame with 5 shelves; from the top: expressions, functions, argument references, types, constants; each shelf carries 6 cells | (-2.5, 1.3, -1.5) | Frame 3.0 by 2.6 by 1.6 titanium; shelves 2.6 by 0.08 by 1.2 `family('program')`; cells 0.25, one `InstancedMesh` |
| `hashP` | Hash block for the program image | (3.5, 0.5, -1.5) | kit |
| `tokP` | `prog_d`, content `'prog:main'` | (5.9, 0.7, -1.5) upright | token size 1.2, program |
| `hashV` | Hash block for the value digest | (3.5, 0.5, 2.5) | kit |
| `tokV` | `val_d`, content `'val:input'` | (5.9, 0.7, 2.5) upright | token size 1.2, input |
| `collars` | Three 8-cube collars: `fuel_lo`, `fuel_hi`, `stack_limit` | (7.0, 0.3, 4.6), (9.2, 0.3, 4.6), (11.4, 0.3, 4.6) | 8 cubes of 0.2 at pitch 0.22 in a titanium channel 1.9 by 0.12 by 0.3 |
| `shkIn` | Domain cell `SHK_IN` | (5.0, 0.3, 4.6) | cell, gold |
| `hashI` | Hash block for the input commitment | (9.5, 0.5, 2.5) | kit |
| `tokI` | `input_d`, content `'input:main'` | (11.9, 0.7, 2.5) upright | token size 1.2, input |
| `inset` | Inset tray | (11.0, 0.05, -3.2) | kit; x 6.5 to 15.5, z -5.45 to -0.95 |
| `rack` | Key rack, rail 14 long | rail centre (4, 4.2, -5.4); cards at x 1.8, 4.0, 6.2 | kit |
| `rom2`, `tokP2` | Second program image (the same rack at scale 0.6) and its token, content `'prog:other'`, size 0.8 | on `inset`: (9.0, 0.85, -3.2) and (12.5, 0.5, -3.2) | as `rom`, `tokP` |

**Shots.**

| Name | Pose |
|---|---|
| `IN` | `establish([-10, 1, 0], 30)` |
| `PRESS` | `establish([-6, 1, -0.5], 20)` |
| `HASHP` | `establish([2, 1, -1.5], 16)` |
| `VALUE` | `establish([6, 1, 0.5], 22)` |
| `COMMIT` | `establish([8.5, 1, 3], 14)` |
| `ALL` | `establish([2, 1.5, 0], 36)` |
| `OUT` | `establish([10, 1, 0], 30)` |


### Beat 1.1: `source-and-input` (index 0, 22 s, scene time 0 to 22)

**Title.** Source and input

**Camera.** `IN`, first key at scene time 0, `cut: true` (the piece opens on it).

**Objects and motion.** 2.0 to 3.2: `sheet` slides in from (-15.5, 0.04, -0.5) to its place and `inp` appears beside it. 5.0 to 8.0: the two `lift` bars are emissive (program bright, 0.5), then return. 9.0 to 12.0: the three bars of the conditional lines (the fifth and the last two bars of `main`) are emissive, then return. Nothing else is visible except `compiler`, unlit, and the empty `rack`-less bench: hide `rom`, the hash blocks, the tokens, the collars and the inset. Still pose: sheet and input cube side by side.

**Labels.**

- Moriarty source | on: the source sheet
- Input value | on: the blue cube
- Named call | on: the lift definition
- Conditional | on: the selected expression

**Carrier and handoff.** Carrier: `prog_d` 0, `input_d` 0, `outcome_d` 0, `n_steps` 0 (the four sockets are empty). In plate empty.

**Narration.**

> Moriarty source enters with an input value named input. The program saves that value, calls lift to add increment and compares the result with ceiling. A conditional selects the next value. Further named calls create intermediate values, and the selected branch then adds the saved input. The main run passes every arithmetic and resource check.

### Beat 1.2: `source-to-rom` (index 1, 22 s, scene time 22 to 44)

**Title.** Source to ROM

**Camera.** `PRESS`.

**Objects and motion.** 2.0 to 3.4: a copy of `sheet` (scale 0.6) slides along x into the slot of `compiler`; the original stays. 3.4 to 3.8 the platen lowers 0.25; 3.8 to 4.2 it rises. 4.4 to 7.0: `rom` appears (frame first, 0.4 s) and the five shelves leave the +x face of the press and slide to their rack positions, top shelf first, each 0.5 s, staggered 0.5 s. 8.0: one verification sweep across `rom`. Still pose: sheet, press, filled rack in a row.

**Labels.**

- Compiler | on: the compiler housing
- Program image | on: the ROM rack
- Code tables | on: the expression and function shelves
- Types and constants | on: the supporting shelves

**Carrier and handoff.** Carrier unchanged (all 0).

**Narration.**

> The compiler turns the source into a program image: the machine language, stored in ROM (read-only memory). Its tables hold expressions, function entries, argument references, types, constructors and constants. A rule reads the table and row that its instruction names. Each row is authenticated against this image, so nobody can substitute another operation or constant.

### Beat 1.3: `program-digest` (index 2, 22 s, scene time 44 to 66)

**Title.** Program digest

**Camera.** `HASHP`.

**Objects and motion.** 2.0 to 5.0: the hash motion at `hashP`. The inputs are five copies of the shelves, each scaled to a cell 0.4, in shelf order from the top; the rack itself stays filled. The output is `tokP`. 6.0 to 6.8: a connector grows from `rom` to `tokP`. 8.0 to 10.0: a copy of `tokP` travels to the front edge at (0, 0.4, 5.8) and scales to 0.3 while `ctx.carrier.show('prog_d', phase(b.t, 8, 10), 'prog:main')` rises. `tokP` and its connector stay beside the rack. Still pose: rack, connector, token; `prog_d` in the carrier.

**Labels.**

- Poseidon | on: the hash block
- Program image | on: the ROM rack
- `prog_d` | on: the violet-rimmed token

**Carrier and handoff.** Carrier: `prog_d` rises 0 to 1 at 8.0 to 10.0. Others 0.

**Narration.**

> Poseidon, the hash function of Sherlock, digests the program image into `prog_d`. The image holds the code tables and their entry information in a fixed encoding. Changing any encoded operation, constant, function body or entry changes what the digest commits to. Later program reads prove membership under this commitment, so substituted code cannot run.

### Beat 1.4: `canonical-input` (index 3, 22 s, scene time 66 to 88)

**Title.** Canonical input

**Camera.** `VALUE`.

**Objects and motion.** 2.0 to 3.0: `inp` moves to (-1, 0.3, 2.5). 3.0 to 3.5: it becomes four cells in a row (type, value form, two ordered contents; family `input`), pitch 0.5. 3.5 to 6.5: the hash motion at `hashV`; the output is `tokV`. 7.0: `inset` rises. On it, left: a shared structure, three tiles with two connectors that end on the same tile, and a token (size 0.8, content `'val:struct'`) at (8.5, 0.5, -2.0). Right: the same value stored separately, four tiles with one connector each, and a second token with the same content at (13.5, 0.5, -2.0). 9.0 to 10.2: the equality motion, right token onto left token. Still pose: `tokV` at the hash output; one outline on the inset over two different tile arrangements.

**Labels.**

- Canonical value | on: the ordered input cells
- `sherlock-value/1` | on: the value hashing port
- `val_d` | on: the blue-rimmed value token
- Equal value | on: the coincident inset tokens

**Carrier and handoff.** Carrier: `prog_d` 1. `input_d` stays 0: `val_d` is not a carrier token.

**Narration.**

> The input value becomes `val_d` under `sherlock-value/1`, the canonical value digest. Canonical means that equal values hash from one encoding, fixed by type, value form and ordered contents. A reference contributes the digest of its target, so moving records or sharing equal substructures keeps the digest. A different value gives a different `val_d`.

### Beat 1.5: `input-commitment` (index 4, 29 s, scene time 88 to 117)

**Title.** Input commitment

**Camera.** `COMMIT`.

**Objects and motion.** The inset and its objects are hidden from the start of the beat. 2.0 to 2.6: `shkIn` and the three `collars` appear on the lane at z 4.6. 3.0 to 5.4: under each collar a ruler 1.9 long appears with a break mark and an end tick, and one verification sweep crosses each collar in turn (starts 3.0, 3.8, 4.6). 6.0 to 9.0: the hash motion at `hashI`. Inputs in order: `shkIn`, a copy of `tokV`, the `fuel_lo` collar, the `fuel_hi` collar, the `stack_limit` collar (each collar travels as one object). `tokV` itself stays. The output is `tokI`. 10.0 to 12.0: a copy of `tokI` travels to the front edge and into the carrier as in beat 1.3. Still pose: the five inputs' places marked by the three rulers, `tokI` at the output, `input_d` in the carrier.

**Labels.**

- `SHK_IN` | on: `shkIn` | kind: state
- `val_d` | on: `tokV` | kind: input
- `fuel_lo` | on: first collar | kind: input
- `fuel_hi` | on: second collar | kind: input
- `stack_limit` | on: third collar | kind: input
- `input_d` | on: `tokI`, from 9.0 | kind: input

**Carrier and handoff.** Carrier: `input_d` rises 0 to 1 at 10.0 to 12.0. `prog_d` 1.

**Narration.**

> `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`. The digest binds the input value to the resource policy of the run: the 128-bit fuel, written as two 64-bit halves, and the stack limit. The two halves and the stack limit are each range-checked below 2<sup>64</sup>. The same value under a different fuel budget or stack limit gives a different `input_d`, so a claim names its resource policy together with its input value.

### Beat 1.6: `fixed-key-handoff` (index 5, 25 s, scene time 117 to 142)

**Title.** Fixed key handoff

**Camera.** `ALL` at the start of the beat; `OUT` at 13.0.

**Objects and motion.** 2.0: `rack` appears with its three cards. 2.6: `inset` rises with `rom2` and `tokP2`. 3.5 to 5.6: one verification sweep crosses each key card in turn (leaf, node, decider). 6.0 to 7.0: thin connectors drop from the three cards to the bench line at z -5.4 and run to both `rom` and `rom2`: the same three cards serve both images. `tokP` and `tokP2` show different faces. Hold to 13.0. 15.0 to 17.2: handoff slide of `rom` to `ctx.plates.out`, base on the plate (rack centre at (20, 1.5, 0)). Still pose: rack on the out plate, key rack and second program behind.

**Labels.**

- Leaf key | on: leaf card | kind: state
- Node key | on: node card | kind: state
- Decider key | on: decider card | kind: state
- Program image | on: `rom` | kind: program
- `prog_d` | on: the carrier | kind: program
- `input_d` | on: the carrier | kind: input

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, the other two 0. Out plate: `rom`, upright, long side along x.

**Narration.**

> The ROM rack and the tokens `prog_d` and `input_d` identify the run. Three key cards serve it: leaf, node and decider. A key fixes a proof circuit, the constraints that check its witness. Every program and input uses this key set, because the circuit reads authenticated program data from ROM. A second program changes the program token and leaves the keys unchanged.

## Scene 2: The run

File `scenes/scene2.js`. 10 beats, 233 s (3:53). Scene starts at 2:22 of the piece.

Scene id `run`. Arrives: the ROM rack on the in plate. Leaves: the witness tape on the out plate, its Halted frame at the right end.

**Ideas taken.** One device per state part, under a cowl that lifts, is from concept 1; the ruled two-by-three arrangement of the six parts is from concept 2. The raised rule key (height shows "exactly one") is from concept 1, and the connector from that key to the new tape frame is from concept 2. The three outcomes shown with their causes (an overflow, a fuel shortfall) and not only their results are from concept 3. The 8-cube collar on a broken ruler for the 64-bit counter is from concept 2.

**Set.**

| Name | Object | Position | Size and material |
|---|---|---|---|
| `rom` | ROM rack as in scene 1 | starts on the in plate (-20, 1.5, 0); socket at (-14, 1.5, -1.5) | as scene 1 |
| `refPlate` | Reference plate of the machine | (-14, 0.1, 1.2) | Plate 2.4 by 0.1 by 1.0, titanium |
| `cowl` | Machine housing | rest (-6, 1.3, 0); parked (-6, 6.5, 0), hidden when parked | Box 12 by 2.4 by 6.4, ceramic, titanium rim |
| `bed` | Six bays in two rows, each a rimmed floor 3.6 by 0.1 by 2.6 | back row z -1.5: control x -9.8, environment x -6, fuel x -2.2; front row z 1.5: frame stack x -9.8, task stack x -6, memory x -2.2 | titanium rims, `paperWarm` floors |
| control bay | Two rows of 4 cells: current state (back), next state (front); one ROM row (4 small program cells) at its left edge | in the bay | cells ceramic; ROM row `family('program')` |
| environment bay | 3 cells, one with a connector to a memory tile | in the bay | ceramic |
| fuel bay | Three bar gauges (initial budget, remaining fuel, consumed work): frames 0.4 by 0.1 by 2.0 with a fill bar; and a group of 2 limit cells | in the bay | titanium frames, ceramic fills |
| frame bay, task bay | Stack chains: 3 links and 2 links, the cached top at the front | in the bays | kit |
| memory bay | One tray with 5 tiles filled | in the bay | kit |
| `rules` | Rule table: 12 by 10 keycaps | centre (3.4, 0.1, -2.5); pitch 0.6 in x, 0.5 in z | keycap 0.5 by 0.12 by 0.4, ceramic, one `InstancedMesh` of 120 |
| `ruleHot` | The firing rule: the keycap in column 7 from the left, row 4 from the back | in `rules` | raised 0.15, `family('step', { lit: 0.6 })` |
| `head` | Tape head | (1.5, 0.6, 3.6) | Box 1.0 by 0.8 by 1.4, titanium |
| `tape` | Witness tape: frames along x from 2.5 to 13.5 at z 3.6, pitch 1.0 | lane z 3.6 | kit frames, one `InstancedMesh` per part |
| `counter` | 64-bit step counter: 8-cube collar | (10.5, 0.3, -4.6) | as scene 1 collars, cubes ceramic |
| `ruler` | Counter range ruler with a break mark at two thirds and an end tick | from (8.5, 0.05, -3.6) to (15.0, 0.05, -3.6) | kit |
| `nPlate` | `n_steps` plate at the tape end | (14.6, 0.2, 3.6) | cell, gold |
| `insetA`, `insetB` | Two half trays | (9.3, 0.05, -0.6) and (13.6, 0.05, -0.6) | each 4.2 by 0.1 by 4.0 |
| `sheetMini` | The source sheet at scale 0.5 | (-14.2, 0.04, 4.2) | as scene 1 |

**Shots.**

| Name | Pose |
|---|---|
| `IN` | `establish([-10, 1, 0], 30)` |
| `BED` | `establish([-6, 0.5, 0], 18)` |
| `RULE` | `establish([-1.5, 0.5, -1.5], 24)` |
| `CALL` | `establish([-9.5, 0.5, 1.5], 16)` |
| `FUEL` | `establish([-2.2, 0.5, -1.5], 9)` |
| `TAPE` | `establish([6, 0.5, 3.2], 14)` |
| `COUNT` | `establish([11, 0.5, -1], 18)` |
| `INSET` | `establish([11.5, 0.5, -0.6], 13)` |
| `ALL` | `establish([0, 1, 0], 36)` |
| `OUT` | `establish([10, 1, 0], 30)` |


### Beat 2.1: `machine-arrival` (index 0, 23 s, scene time 0 to 23)

**Title.** Machine arrival

**Camera.** `IN`, first key at scene time 0.

**Objects and motion.** First pose: `rom` on the in plate as scene 1 left it; `cowl` at rest; the tape lane empty (the `head` and an `onSurface` lane line only). 2.0 to 4.0: `rom` slides to its socket at (-14, 1.5, -1.5). 5.0: one verification sweep across `refPlate`. Still pose: rack in the socket, closed machine, empty lane.

**Labels.**

- `mori simulate` | on: the machine housing
- Program image | on: the ROM socket
- `sherlock-k-a/1` | on: the machine’s reference plate
- Witness stream | on: the empty tape path

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1 (shell default). In plate: `rom` in the first pose, then empty.

**Narration.**

> `mori simulate` receives the ROM program, the input value and the resource policy that `input_d` binds. It runs a CEK-style machine, which keeps the current computation, the variable bindings and the saved continuations explicit. The reference machine `sherlock-k-a/1` defines every transition. The run produces the detailed evidence that the leaf checks. The simulator's own record certifies nothing.

### Beat 2.2: `machine-state` (index 1, 24 s, scene time 23 to 47)

**Title.** Machine state

**Camera.** `BED`.

**Objects and motion.** 2.0 to 3.5: `cowl` lifts to its parked height, is hidden there and stays hidden for the rest of the scene. 3.5 to 6.0: the contents of the six bays appear bay by bay in the order control, environment, frame stack, task stack, memory, fuel (0.4 s each). Still pose: the six bays in two ruled rows, nothing moving.

**Labels.**

- Control | on: the control compartment
- Environment | on: the environment compartment
- Frame stack | on: the continuation chain
- Task stack | on: the helper chain
- Memory | on: the ceramic record trays
- Fuel counters | on: the resource register compartment

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> The machine state has six parts. Control names the expression, return or helper action in progress. The environment maps variable names to values. The frame stack holds suspended computation and return context, and the task stack holds unfinished helper work. Memory stores immutable records. Fuel counters track available and consumed work, beside the logical resource counters that execution updates.

### Beat 2.3: `enabled-rule` (index 2, 24 s, scene time 47 to 71)

**Title.** Enabled rule

**Camera.** `RULE`.

**Objects and motion.** 2.0: one verification sweep crosses the current-state row of the control bay and its ROM row. 3.0 to 3.4: `ruleHot` rises 0.15 and lights; the other 119 keycaps stay flat and unlit. 4.0 to 5.0: a connector grows from `ruleHot` to the next-state row. 5.0 to 5.6: the next-state cells take the `step` colour. Still pose: one raised key, one connector, the two state rows.

**Labels.**

- 120 rules | on: the rule table
- Current state | on: the incoming register bank
- Instruction | on: the ROM row
- Selected rule | on: the active rule position
- Next state | on: the outgoing register bank

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> Each step fires exactly one rule out of 120. A rule is enabled when its conditions hold for the current state and authenticated instruction, under the reference priority. Exactly one means that the selected rule fixes every permitted update and other state stays unchanged. A claimed transition cannot mix writes from two rules or pick a convenient branch.

### Beat 2.4: `named-call-context` (index 3, 24 s, scene time 71 to 95)

**Title.** Named call context

**Camera.** `CALL`.

**Objects and motion.** `sheetMini` appears at the start. 2.0 to 3.2: a cell (the caller's continuation) leaves the control bay and becomes the new cached top of the frame chain; the old top moves down one link. 4.0 to 5.5: three connectors grow from `sheetMini`: from the call bar to the new frame, from the addition bar to a result cell in the control bay, from the comparison bar to a Boolean cell beside it. 6.0: the memory tile that holds the saved input gets a `step` outline and its connector from the environment bay lights. Still pose: pushed frame, three connectors, the referenced tile.

**Labels.**

- Named call | on: the active call expression
- Saved continuation | on: the pushed frame
- Raised value | on: the addition result cube
- Branch condition | on: the Boolean cube
- Saved input | on: the referenced memory record

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> The call to lift saves the caller's continuation in a frame, so returning restores the environment and next expression. The addition inside lift produces the raised value. The comparison produces the Boolean that selects the conditional branch, and named calls continue from that choice. The saved input stays reachable through its environment reference until the final addition reads it.

### Beat 2.5: `resource-accounting` (index 4, 24 s, scene time 95 to 119)

**Title.** Resource accounting

**Camera.** `FUEL`.

**Objects and motion.** 2.0 to 3.0: a `step` cube (the rule's charge) drops from above to the side of the remaining-fuel gauge. 3.5: one verification sweep across charge and gauge. 4.2 to 5.0: the remaining-fuel fill lowers by one eighth of the frame and the consumed-work fill rises by the same amount; the initial-budget gauge stays full. The two limit cells get a gold outline for 0.3 s. Still pose: three gauges at three levels, the charge cube, the limit cells.

**Labels.**

- Initial budget | on: the initial-fuel register
- Required charge | on: the rule’s charge cube
- Remaining fuel | on: the remaining-fuel register
- Consumed work | on: the work register
- Resource limits | on: the logical-limit register group

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> Fuel starts at the budget that `input_d` binds. Each rule applies its specified charge to remaining fuel and to consumed work. Helper counters track traversal and construction, and resource limits bound stack growth and allocation. Each check compares the requested work with the remaining allowance, so no free computation, hidden allocation or counter wrap can extend the authorized run.

### Beat 2.6: `witness-frame` (index 5, 23 s, scene time 119 to 142)

**Title.** Witness frame

**Camera.** `TAPE`.

**Objects and motion.** The tape holds two frames at x 2.5 and 3.5. 2.0 to 2.4: `head` lowers 0.2 and rises. 2.4 to 3.2: a new frame leaves the head and the tape advances one pitch, so the frames are at 2.5, 3.5, 4.5 with the newest at 2.5. 3.6 to 4.8: the equality motion between the three state-after cubes of the frame at 3.5 and the three state-before cubes of the frame at 2.5 (the before cubes move 0.3 toward the after cubes; one outline). A connector runs from `ruleHot` to the rule plate of the new frame. Still pose: three frames, one shared boundary outline.

**Labels.**

- Witness stream | on: the tape
- Rule id | on: the rule plate
- State before | on: the incoming state cells
- State after | on: the outgoing state cells
- Equal boundary | on: the coincident cells

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> Each fired rule records one frame on the witness stream, the private execution tape. A frame holds the rule id, the state before and the state after. Each frame begins in the state where the previous frame ended. An unexplained change between recorded steps breaks that equality. The leaf checks this continuity and the rule's actual transition.

### Beat 2.7: `step-counter` (index 6, 23 s, scene time 142 to 165)

**Title.** Step counter

**Camera.** `COUNT`.

**Objects and motion.** 2.0 to 6.2: three more frames are printed (at 2.0, 3.4, 4.8; each as in beat 2.6 without the equality). With each frame the lowest cube of `counter` lights `step` for 0.4 s. The tape now runs to x 8.5; a break mark sits on the lane at x 9.5 and the lane continues to `nPlate`. 7.0: `ruler` appears with its break mark and end tick. Still pose: tape with break mark, `nPlate` at its end, collar and ruler behind.

**Labels.**

- 64-bit counter | on: the step counter
- `n_steps` | on: the tape’s total-count plate
- 2<sup>64</sup> - 1 steps | on: the counter limit
- Counter range | on: the range ruler

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> A 64-bit counter counts the fired steps, terminal publication included. A run has up to 2<sup>64</sup> - 1 steps, and its total is `n_steps`. The value 2<sup>64</sup> is the counter's range, not the size of a circuit. Counter checks stop a long execution from wrapping into a smaller claimed count. Fuel charges and step counts stay separate.

### Beat 2.8: `arithmetic-rejection` (index 7, 21 s, scene time 165 to 186)

**Title.** Arithmetic rejection

**Camera.** `INSET`.

**Objects and motion.** `insetA` rises. On it: two operand cells and a result cell in a row at z -1.2, and an empty frame plate at z 0.4. 2.0 to 2.8: the refusal motion on the result cell as it moves toward the frame plate. 3.4: a terminal frame appears on the plate: its rule plate is `family('error')`, with one payload cell beside it. The main tape does not change. Still pose: hatched result, Rejected frame.

**Labels.**

- Arithmetic overflow | on: the overflow check
- Rejected | on: the terminal tag
- Code 10 | on: the terminal payload

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> An addition that overflows ends the run as Rejected with code 10, the payload for arithmetic overflow. The proof checks that the arithmetic really overflows, and the wrapped result never becomes a successful return value. This ending uses the same rule system and the same proof key as the successful one.

### Beat 2.9: `fuel-exhaustion` (index 8, 23 s, scene time 186 to 209)

**Title.** Fuel exhaustion

**Camera.** `INSET` (no move).

**Objects and motion.** `insetB` rises; `insetA` stays as it ended. On `insetB`: a `step` charge cube and a short remaining-fuel gauge (fill at one tenth) at z -1.2, an empty frame plate at z 0.4. 2.0 to 2.8: the charge cube moves toward the gauge with the refusal motion. 3.4: a terminal frame appears with an `error` rule plate and one payload cell. Still pose: both half trays, each with its cause and its terminal frame.

**Labels.**

- Required charge | on: the charge cube
- Remaining fuel | on: the allowance cube
- Exhausted | on: the terminal tag
- Fuel | on: the terminal payload

**Carrier and handoff.** Carrier unchanged.

**Narration.**

> When the next required work charge exceeds the remaining fuel, the run ends as Exhausted with the kind Fuel. The payload names the resource. The check compares the required charge with the remaining allowance, so no prover can fake exhaustion. A leaf that fills its proving capacity only moves the tape cut and never produces this outcome.

### Beat 2.10: `halted-tape-handoff` (index 9, 24 s, scene time 209 to 233)

**Title.** Halted tape handoff

**Camera.** `ALL` at the start; `OUT` at 10.0.

**Objects and motion.** Both half trays lower and hide (0 to 0.6). 2.0 to 3.2: the last frame is printed: its rule plate is `family('outcome')` and a teal result cube (0.4, `family('outcome')`) sits on its state-after side. 4.0 to 6.0: the tape closes up into its handoff form: four frames at pitch 1.0 with the break mark between the second and third, the Halted frame last, `nPlate` attached at the right end (total length 5.2). 12.0 to 14.2: handoff slide of that group to `ctx.plates.out`, long side along x. Still pose: tape on the out plate, machine open behind.

**Labels.**

- Witness stream | on: the tape | kind: neutral
- Halted | on: rule plate of the last frame | kind: outcome
- Result value | on: the teal cube | kind: outcome
- `n_steps` | on: `nPlate` | kind: state
- `prog_d` | on: the carrier | kind: program
- `input_d` | on: the carrier | kind: input

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1. Out plate: the tape group (4 frames, break mark, `nPlate`), Halted frame at +x.

**Narration.**

> The main run reads the saved input, completes its final addition and returns the selected value. Each return through the frame stack restores a suspended context. At the outer return, publication records Halted with the result value as its payload. The witness tape ends at that outcome frame and carries `n_steps`, the run's step count, into leaf proving.

## Scene 3: The leaf

File `scenes/scene3.js`. 35 beats, 857 s (14:17). Scene starts at 6:15 of the piece.

Scene id `leaf`. Arrives: the witness tape on the in plate. Leaves: leaf proof slabs on the out plate, each with seven pins.

This is the longest scene. It is one floor plan that the camera visits region by region. Build the whole plan once; each beat lights one region and dims the rest (colour lerp 0.6 toward `paperWarm`, section 0.3).

**Ideas taken.** The blade that cuts the tape every 16 frames, the dedicated top-down reveal of the plan, the follow shot on a bus request, the idle zero tuples that lie flat in the two request lanes, the segment token that climbs the tree, the sibling tokens that join a cold read, and the cache line that answers two reads from one opening are from concept 3. The layering by height (bus trench low, modules as low blocks, the tree as the only tall object), the DEC row that steps up and returns a control word, the overflow flag as a latch, the operand that splits into limbs, the two-stage boundary press that assembles a split token, the keyed pins (hexagonal collars for digests, flat bars for integers) and the uneven second cut in the partition comparison are from concept 1. The row budget bar that stays in view and takes the flush and old-read shares, the broken rulers for depth 63 and 2<sup>66</sup>, and the split comparison tray are from concept 2. The route helpers and the plan arrangement are from the earlier leaf chip view (see `SCENE-CONTRACT.md` section 2).

**Change to the kit.** A boundary digest (`pre_d`, `post_d`) is a split token: a gold base disc (the body, `body_d`) with a gold cap ring in five segments (the header fields). Build it as a `makeDigestToken` of family `state` with a ring of five arc segments (torus arcs of 66 degrees with 6 degree gaps, tube 0.06) around its rim. Scene 5 opens it. In this scene it is closed except in beats 3.26 to 3.28.

**Set.** Heights: bus lines at y 0.05, module housings 0.3 high (top at y 0.3), objects on a module at y 0.3 plus half their height. A housing is a ceramic box with a titanium rim line.

| Name | Object | Position (centre) | Size |
|---|---|---|---|
| `lid` | Leaf housing lid | rest (0, 1.4, 0.4); parked (0, 7, 0.4), hidden when parked | 27 by 0.2 by 9, titanium |
| `lane` | Tape lane line | z -5.3, x -16 to 8.5 | `onSurface` line |
| `ribbon` | 16 step slots | x = -7.5 + i for i 0 to 15, z -5.3 | slot floor 0.9 by 0.05 by 1.2, `paperWarm`, titanium rim |
| `cutter` | Blade on two posts | (-8.2, 1.2, -5.3) raised; blade edge at y 0.1 when down | blade 0.1 by 1.2 by 1.5, titanium |
| `wait` | Waiting area for slices not in the ribbon, drawn at scale 0.4 | x -15.6 to -9.2, z -5.3 | |
| `rack` | Key rack; the leaf card is the lit one | rail centre (11, 4.2, -5.4) | kit |
| `CORE` | Housing | (-8, 0.15, 0) | 7 by 0.3 by 5 |
| `regs` | Persistent registers: before row and after row, 8 cells each | x -10.75 to -7.25 (pitch 0.5); z -1.6 (before), z -1.0 (after) | cells |
| `slots` | 16 rule slots on CORE | x -11 to -5 (pitch 0.4), z -2.2 | cells 0.3 |
| `chain` | Append chain: `len_pre` cube, three event cubes, `len_post` cube, joined by a line | z 1.9; x -11, -9.5, -8, -6.5, -5 | cells; line `onSurface` |
| `readPort` | Read port: address cell, kind cell, running-length marker | (-4.9, 0.5, 0.8), cells along z | cells |
| `busPort` | CORE bus port | (-4.7, 0.5, -0.6) | box 0.4 by 0.4 by 1.0, titanium |
| `BUS` | Two request lanes and the provider lip: lines along z from -4.3 to 4.6 at x -3.3, x -2.7 (lanes) and x -2.1 (lip); a branch line from x -2.1 to x -1 at each provider's z | | lines |
| `zero0`, `zero1` | Idle zero tuples, flat in the lanes | (-3.3, 0.04, -0.6), (-2.7, 0.04, -0.6) | tuple bars 0.08 high, ceramic dimmed |
| `DEC` | Housing with a table of 8 visible rows of 5 cells, a break mark, and one idle row | (0.5, 0.15, -3.6) | 3 by 0.3 by 1.5 |
| `table` | Tagged fixed table rail with two tag cells at its ends (`TAG_DEC` at x -0.8, `TAG_RANGE` at x 5.6) | (2.4, 0.1, -4.7) | 7 by 0.1 by 0.4, titanium |
| `RANGE` | Housing with a column of limb slots | (4.3, 0.15, -3.6) | 3 by 0.3 by 1.5 |
| `ALU` | Housing with two operand cells, a result cell and a flag latch (a plate 0.4 by 0.05 by 0.4 hinged on its back edge) | (0.5, 0.15, -1.7) | 3 by 0.3 by 1.5 |
| `aluSide` | Small pad for the overflow case | (4.0, 0.1, -1.6) | 2.6 by 0.1 by 1.6 |
| `MEM` | MEM window housing with a tray at scale 0.6 (`win`) | (1.5, 0.15, 0.2) | 5 by 0.3 by 1.5 |
| `HASH` | Housing with a kit hash block at scale 0.7 (`hashH`) | (0.5, 0.15, 2.1) | 3 by 0.3 by 1.5 |
| `VD` | VALUE DIGEST housing with a format plate | (4.3, 0.15, 2.1) | 3 by 0.3 by 1.5 |
| `PAGE` | PAGE/ROM block cache housing with two line trays at scale 0.4 | (1.5, 0.15, 4.2) | 5 by 0.3 by 1.6 |
| `BACKEND` | MEMORY BACKEND housing; on it the tail tray `tail` at (9.5, 0.4, 0.9), a `len` cell at (6.4, 0.5, -0.3), the `root` token at (12.6, 0.6, -0.3) and the `tail_d` token at (11.2, 0.6, -0.3), tokens size 0.8 | (9.5, 0.15, 0.4) | 7.8 by 0.3 by 2.2 |
| `segs` | Completed segments: two trays at scale 0.5 with a break mark between | (7.0, 0.15, -2.3), (12.0, 0.15, -2.3) | |
| `tree` | Memory tree column, the only tall object. It is hidden whenever `lid` is at rest (beats 3.1 and 3.32 to 3.35) and shown from beat 3.2 to 3.31; nodes at y 1.0, 1.7, 2.4, break at 3.1, nodes at 3.8, 4.5, root token (size 1.2) at y 5.4; sibling tokens (size 0.5) 0.8 to the right of each node; depth ruler at x 11.2 | x 9.5, z -3.6 | kit |
| `BND` | BOUNDARY COMMITMENT housing with two hash blocks at scale 0.7: `hashBody` at (7.6, 0.6, 4.2) and `hashHead` at (11.4, 0.6, 4.2) | (9.5, 0.15, 4.2) | 7.8 by 0.3 by 2 |
| `INIT` | Housing | (-10.2, 0.15, 4.2) | 2.6 by 0.3 by 1.5 |
| `STACKS` | Housing with a hash block at scale 0.6 (`hashS`) in its middle; the FRAMES chain rises at x -7.6 and the TASKS chain at x -5.0, links stacked upward at pitch 0.7, the cached top as a cell on a small titanium shelf above each chain | (-6.3, 0.15, 4.3) | 4 by 0.3 by 1.8 |
| `dock` | Slab dock | (14.2, 0.35, -4.2) | a leaf slab stands here |
| `budget` | Row budget bar: frame with two fills that grow from the left: flush share 2.15 long, old-read share 1.16 long (their true shares of the bar) | frame from x -8 to 8 at z 5.75 | 16 by 0.12 by 0.3, titanium; fills gold |
| `inset` | Raised comparison tray; while it is up, everything under it is dimmed | (3, 1.6, 0.6) | 12 by 0.1 by 5.5 |

Three slices exist as tape groups of 16 frames: `G` (the genesis slice), `A` (a middle slice, all 16 active) and `T` (the terminal slice: 10 active frames, the tenth with an `outcome` rule plate, then 6 inactive frames whose before and after cubes are equal and whose rule plates are ceramic). One of them is in the ribbon; the others wait at scale 0.4. A slice swap (1.2 s) moves the ribbon slice to `wait` and the named slice into the ribbon.

**Shots.**

| Name | Pose | | Name | Pose |
|---|---|---|---|---|
| `IN` | `establish([-10, 1, -2], 30)` | | `TREE` | `establish([9.5, 2.8, -1.5], 13)` |
| `PLAN` | `inspect([0, 0, 0.3], 30, 'top')` | | `READ` | `inspect([2.5, 0, 0.5], 19, 'top')` |
| `CORE` | `inspect([-8, 0, 0], 10, 'top')` | | `PAGE` | `inspect([4, 0, 2.2], 17, 'top')` |
| `DEC` | `inspect([-1.5, 0, -2.6], 13, 'top')` | | `STACK` | `establish([-6.3, 1.6, 4.3], 8)` |
| `BUSV` | `inspect([-2, 0, -0.6], 12, 'top')` | | `VD` | `inspect([3, 0, 2.2], 11, 'top')` |
| `ALU` | `inspect([1.5, 0, -1.7], 9, 'top')` | | `BND` | `inspect([9.5, 0, 3.6], 12, 'top')` |
| `RANGE` | `inspect([3, 0, -4.0], 9, 'top')` | | `INIT` | `inspect([-8.5, 0, 2.6], 12, 'top')` |
| `HASH` | `inspect([1, 0, 2.1], 9, 'top')` | | `RIB` | `inspect([0.5, 0, -4.9], 19, 'top')` |
| `CHAIN` | `inspect([-4.5, 0, 1.2], 14, 'top')` | | `DOCK` | `establish([13.2, 1, -3.4], 10)` |
| `MEMV` | `inspect([5.5, 0, 0.5], 17, 'top')` | | `INSET` | `inspect([3, 1.6, 0.6], 14, 'top')` |
| `COST` | `establish([2, 1, 2.5], 30)` | | `OUT` | `establish([10, 1, 0], 30)` |


### Beat 3.1: `tape-to-slices` (index 0, 23 s, scene time 0 to 23)

**Title.** Tape to slices

**Camera.** `IN`, first key at scene time 0.

**Objects and motion.** First pose: the tape group of scene 2 on the in plate; `lid` at rest over the plan; `rack` with the leaf card lit (gold emissive 0.3). 2.0 to 3.6: the tape slides in a straight line from the plate to the lane start at (-13, 0.1, -5.3) and becomes the long tape: slice `A` at full scale ahead of the blade position, slice `T` behind it at scale 0.4 in `wait`. 4.0 to 4.4: the blade drops between `A` and `T` and rises. 4.6 to 6.2: `A` slides into the 16 ribbon slots. A ruler with 16 ticks lies along the ribbon at z -4.5. In `T`, the 6 inactive frames are dimmed. Still pose: slice in the ribbon under the leaf key, terminal slice waiting.

**Labels.**

- Witness stream | on: the incoming tape
- `S = 16` | on: the slot ruler
- Leaf key | on: the key card
- Active slice | on: the inserted tape segment
- Inactive suffix | on: the unused terminal positions

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1 (shell default). In plate: the tape in the first pose, then empty.

**Narration.**

> The witness tape arrives with its Halted outcome frame attached. The leaf cuts the tape into slices of `S = 16` step slots, each entering the same circuit under one leaf key. A full slice uses every slot. A shorter slice keeps its unused suffix inactive, so state holds still and the proof covers the recorded steps.

### Beat 3.2: `core-register-chain` (index 1, 23 s, scene time 23 to 46)

**Title.** CORE register chain

**Camera.** Stays on `IN` for the lift; `PLAN` at 4.0; `CORE` at 9.5.

**Objects and motion.** 2.0 to 3.5: `lid` rises to its parked height, is hidden there (section 0.2) and stays hidden until beat 3.32. As the lid clears it, `tree` appears (0.4 s). From 4.0 the whole plan is lit, seen from above, to 9.5: this is the only full view of all modules. From 9.5 the plan outside CORE is dimmed. 11.5 to 12.7: the row of 8 after-cells of rule slot 3 (shown as a copy of the `regs` after row, lifted 0.3) moves onto the before row of slot 4 with the equality motion. 13.5: in `regs`, the six cells outside the write set get a thin `onSurface` outline; the two written cells are `step` coloured. Still pose: CORE with one shared boundary outline.

**Labels.**

- CORE | on: the core housing
- Persistent registers | on: the register bank
- Rule slot | on: the inspected slot
- Equal state | on: the coincident boundary cells
- Unchanged registers | on: the cells outside the write set

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> CORE, the module that checks machine transitions, holds the persistent registers and the rule slots. Its registers encode control, environment reference, cached stack tops, helper work, result, outcome and resource counters. Each slot's final register cells equal the next slot's initial cells. Registers outside the selected rule's write set keep their values and hide no change.

### Beat 3.3: `decoder-control-word` (index 2, 25 s, scene time 46 to 71)

**Title.** Decoder control word

**Camera.** `DEC`.

**Objects and motion.** 2.0 to 3.0: three dispatch cells (copies of three `regs` before cells) slide from CORE along the line at z -3.6 to the left edge of `DEC`. 3.4: one verification sweep down the DEC table. 4.0 to 4.4: one table row (the third) steps up 0.15 and takes the `step` colour; the other rows dim. 5.0 to 6.4: a copy of that row, a bar of 5 cells, returns along the same line to CORE and settles on slot 3 of `slots`; two connectors run from it to the two written cells of `regs`. Still pose: raised row, returned control word, connectors.

**Labels.**

- DEC | on: the decoder housing
- Dispatch facts | on: the incoming state cells
- Fixed table | on: the table rack
- Control word | on: the selected row
- Write set | on: CORE’s permitted-write cells

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> DEC, the decoder, derives a rule control word from a fixed table. It checks the current control and outcome, the authenticated instruction, stack tags and comparison results against each rule's conditions, and reference priority selects exactly one matching rule. The returned word fixes the rule id, operation order and write set. Bounded fields stop any other word passing for that entry.

### Beat 3.4: `active-prefix` (index 3, 22 s, scene time 71 to 93)

**Title.** Active prefix

**Camera.** `RIB`.

**Objects and motion.** 0 to 1.2: slice swap, `T` into the ribbon. 2.0 to 4.4: one slow verification sweep (2.4 s for this beat only) crosses slots 1 to 10 and stops at slot 10, whose rule plate is `outcome` coloured. Slots 11 to 16 stay dimmed: in each, the three before cubes and the three after cubes have equal faces, and a small empty socket (an `onSurface` square outline 0.3) lies beside the rule plate. A bracket line under slots 1 to 10 and a second under 11 to 16 mark prefix and suffix. Still pose: lit prefix ending in the terminal slot, dim suffix.

**Labels.**

- Active prefix | on: the enabled slots
- Terminal publication | on: the final active slot
- Inactive suffix | on: the remaining slots
- State preserved | on: the suffix register cells

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Active slots form one continuous prefix. Each active slot counts as one step, and its internal operations add no steps. An inactive slot keeps every persistent register and makes no request. Terminal publication ends the active prefix. A prover cannot skip a transition, pad the step count or keep executing after Halted, Rejected or Exhausted.

### Beat 3.5: `bus-tuple` (index 4, 24 s, scene time 93 to 117)

**Title.** BUS tuple

**Camera.** `BUSV`.

**Objects and motion.** 0 to 1.2: slice swap, `A` back into the ribbon. `zero0` and `zero1` lie flat in the lanes. 2.0 to 3.2: a tuple bar rises at `busPort` (chip cell with 1 pip, `step` colour) and moves into lane 0 at (-3.3, 0.3, -0.6); `zero0` is hidden while the bar is in the lane. On each provider's branch line a chip cell with that provider's pips is shown: 1 at ALU, 2 at MEM, 3 at HASH, 4 at PAGE. Still pose: the tuple stopped at the junction with the ALU branch at (-3.3, 0.3, -1.7), five cells readable, four pip cells on four branches.

**Labels.**

- BUS | on: the shared route
- `(chip, key, a, b, c)` | on: the packet bar
- ALU 1 | on: the ALU provider
- MEM 2 | on: the MEM provider
- HASH 3 | on: the HASH provider
- ROM 4 | on: the ROM provider

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> BUS connects CORE requests to checked provider tuples `(chip, key, a, b, c)`. The chip names the operation family, the key selects the operation or record, and the other cells carry the data. Chip ids are 1 for ALU, 2 for MEM, 3 for HASH and 4 for ROM. CORE alone issues requests, through the two request lanes.

### Beat 3.6: `request-meets-provider` (index 5, 24 s, scene time 117 to 141)

**Title.** Request meets provider

**Camera.** Follow shot (marked exception). Key at the start: `follow([-3.3, 0.3, -1.7], [-2, 9, 7])`. Key at 2.0 with `dur: 1.4`: `follow([-0.6, 0.3, -1.7], [-2, 9, 7])`. Key at 6.0 with `dur: 1.6`: `follow([-4.7, 0.3, -0.6], [-2, 9, 7])`.

**Objects and motion.** 2.0 to 3.4: the tuple travels the route lane 0, ALU branch, to the provider row at (-0.6, 0.45, -1.7) (use `pointAt` on the route). On the ALU housing a provider tuple of five ceramic cells lies at that spot. 3.6 to 4.8: the equality motion, request onto provider tuple. 6.0 to 7.6: the checked tuple (gold outline) returns along the same route to `busPort` and stops beside the `regs` after row. Still pose: checked tuple at CORE.

**Labels.**

- CORE request | on: the outgoing packet
- ADD | on: its operation cell
- Checked provider | on: the ALU row
- Equal tuple | on: the matched packet outlines

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> CORE puts the addition's operation, operands and claimed result into a BUS request. The whole tuple must equal a checked ALU provider tuple. Every provider satisfies its operation constraints, even when no request uses it. Several requests can match the same provider, but requests cannot authenticate each other. An invented answer therefore has no valid computation to match.

### Beat 3.7: `checked-addition` (index 6, 29 s, scene time 141 to 170)

**Title.** Checked addition

**Camera.** `ALU`.

**Objects and motion.** 2.0: on `ALU`, two operand cells and a result cell are lit. 2.6: one verification sweep across them. 3.4 to 4.0: the flag latch stays flat (flag clear); a connector grows from it to the CORE after row. 6.0: `aluSide` rises with a second latch standing upright (rotated 80 degrees about its back edge: flag set), a connector from it to a payload cell with the refusal hatch. Still pose: flat latch wired to CORE, upright latch wired to the hatched payload.

**Labels.**

- ALU | on: the arithmetic housing
- ADD | on: the active operation
- Operands | on: the operand cubes
- Result | on: the output cube
- Overflow flag | on: the flag cube
- Code 10 | on: the rejection payload

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> ALU, the arithmetic module, proves the exact unsigned addition of the value and increment. It constrains both operands, the result and the overflow flag, and a carry past the permitted integer width sets the flag. CORE connects a set flag to Rejected with code 10. In the main run the flag is clear, and the ordinary result passes unwrapped. The same ALU checks subtraction, multiplication and equality under their machine rules.

### Beat 3.8: `checked-comparison` (index 7, 24 s, scene time 170 to 194)

**Title.** Checked comparison

**Camera.** `ALU` (no move).

**Objects and motion.** `aluSide` lowers and hides. 2.0 to 3.2: a second tuple bar (chip cell 1 pip) arrives on lane 1 and its branch to `ALU`; its `a` and `b` cells settle on the two operand cells. 3.6: one verification sweep across operands and result. 4.4 to 5.6: a copy of the result cell travels back to CORE and meets the conditional-control cell (the first `regs` after cell) with the equality motion. The camera stays; CORE's edge is in frame at the left. Still pose: Boolean result cell and its twin outline at CORE.

**Labels.**

- LT | on: the comparison operation
- Raised value | on: the first operand
- Ceiling | on: the comparison bound
- Boolean result | on: the output cube
- Branch condition | on: CORE’s matching control cell

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The LT request asks ALU whether the raised value is less than ceiling. ALU constrains its Boolean result to that comparison of the authenticated operands, and CORE uses the same result cell to choose the conditional branch. A Boolean changed without a change to the operands fails the comparison check, so the prover cannot pick the branch it prefers.

### Beat 3.9: `integer-range-checks` (index 8, 31 s, scene time 194 to 225)

**Title.** Integer range checks

**Camera.** `RANGE`.

**Objects and motion.** 2.0 to 3.0: a checked cell on `ALU` sends a copy to the left edge of `RANGE`. 3.0 to 4.2: the copy splits into 11 limb cells in a column (z pitch 0.12, scale 0.25), the top one narrower (scale 0.12 in x). 4.6 to 6.4: a verification sweep moves down the column; each limb gets a connector to the `table` rail as the sweep passes. 7.0 to 8.2: the limbs close up into one cell again and that cell moves back onto the original with the equality motion. `TAG_RANGE` and `TAG_DEC` are lit at the two ends of `table`. Still pose: limb column, table rail with two tags, one outline on the ALU cell.

**Labels.**

- RANGE | on: the range-check housing | kind: neutral
- 12-bit limbs | on: the limb column | kind: neutral
- Exact top width | on: the narrow top limb | kind: neutral
- Equal reconstruction | on: the outline on the ALU cell | kind: state
- Fixed table tags | on: the two tag cells | kind: neutral

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> RANGE proves that integer cells fit their required widths. It reconstructs each checked cell from 12-bit limbs, small integer pieces checked against a fixed table, and the highest piece also obeys the exact width of the original integer. The reconstruction equals the caller's cell. Oversized operands and modular arithmetic tricks that imitate valid integer equations fail here. Fixed table tags keep RANGE queries apart from DEC queries, so neither check can borrow the other's table entries.

### Beat 3.10: `poseidon-relations` (index 9, 34 s, scene time 225 to 259)

**Title.** Poseidon relations

**Camera.** `HASH`.

**Objects and motion.** 2.0 to 5.0: the hash motion at `hashH` with four inputs: a gold domain cell first, then three ceramic cells that arrive from CORE over three direct connector lines (not over the bus). Output: a state token, size 0.8. The three connector lines stay. 6.0: on the HASH branch of the bus a provider tuple with a 3-pip chip cell is shown, separate from the direct lines. Still pose: block, output token, three direct lines, one bus socket.

**Labels.**

- HASH | on: the hash housing
- Ordered inputs | on: the incoming cells
- Domain tag | on: the fixed domain cell
- Poseidon | on: the gold block
- Digest | on: the output token
- Direct port | on: the longer input connection

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> HASH computes Poseidon over the caller's ordered input cells. It constrains the initial state, absorption, permutation rounds and final output. Fixed domain tags tell memory records, tree levels, stacks and state bodies apart, so a stack link cannot pass for a memory node. Public digest formats keep their specified encoding. Each input copies the caller's cell, so substituted data cannot be hashed. Longer hashes use direct ports with copied input cells in a fixed order. Chained short BUS hashes differ from the whole-message hash.

### Beat 3.11: `append-address-chain` (index 10, 25 s, scene time 259 to 284)

**Title.** Append address chain

**Camera.** `CHAIN`.

**Objects and motion.** 2.0 to 5.0: a verification sweep runs along `chain` from `len_pre` to `len_post`. As it passes each of the three event cubes, the cube lights `step` and a tuple bar (chip cell 2 pips) leaves it, travels by lane 0 and the MEM branch, and lands on a tile position of `win` (three tuples, staggered 1.0 s, 0.9 s each). A connector from each event cube to its tile stays. Still pose: five chain cubes, three tuples resting on three window tiles.

**Labels.**

- Append chain | on: CORE’s event line
- Incoming length | on: the initial length cube
- Append address | on: the inspected event
- MEM request | on: its packet
- Outgoing length | on: the final length cube

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> CORE owns the append chain, the ordered sequence of new memory addresses. The first append starts at the incoming length, each active append advances the length to the next address, and the final length equals the outgoing memory length. Every appended record also matches a MEM provider through BUS. No address is skipped or allocated twice, and no record's contents are swapped.

### Beat 3.12: `memory-window` (index 11, 24 s, scene time 284 to 308)

**Title.** Memory window

**Camera.** `MEMV`.

**Objects and motion.** `win` holds, from the left: 3 tiles with a titanium outline (the imported tail), then the 3 tiles of beat 3.11, then 2 padding positions (flat `paperWarm` plugs with no ink bar, no branch line to the bus). 2.0 to 3.0: one more tile slides from the MEM branch into the next free position. Two marker cells (`len_pre`, `len_post`) sit under the tray at the boundary after the imported tiles and after the last new tile; connectors join them to the `chain` ends. Still pose: three groups in one tray, two length markers.

**Labels.**

- MEM window | on: the window housing
- Imported tail | on: the inherited tiles
- New records | on: the appended tiles
- Padding | on: the unused positions
- At most 3 cells | on: the per-step append port

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The MEM window holds the imported partial tail and this leaf's new records, in a single arena. Each logical record occupies one cell. Active records form a continuous prefix, and the imported records end exactly at the incoming length. Rules append at most 3 cells per step. Padding serves no read. Unused capacity therefore holds no invented program data.

### Beat 3.13: `authenticated-tail` (index 12, 24 s, scene time 308 to 332)

**Title.** Authenticated tail

**Camera.** `MEMV` (no move).

**Objects and motion.** 2.0 to 3.4: the window tiles copy to `tail` on `BACKEND` (tiles travel as one group and grow to full scale), so `tail` holds 8 tiles: 3 imported, 5 new. 4.0 to 7.0: the hash motion at a kit hash block that appears at (6.6, 0.8, 1.3) on `BACKEND`: inputs are copies of the 3 imported tiles and 5 fixed padding cells (ceramic, no ink); output a state token, content `'tail:pre'`. 7.4 to 8.6: that token meets `tail_d` (same content) with the equality motion. Still pose: full tail tray, one outline on `tail_d`.

**Labels.**

- MEMORY BACKEND | on: the backend housing
- 8-cell tail | on: the full tray
- Imported records | on: the inherited portion
- Fixed padding | on: the tail-opening padding cells
- Equal tail digest | on: the coincident tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The named calls and intermediate bindings append records in execution order until the 8-cell tail is full. The dense MEMORY BACKEND authenticates the imported tail: it recomputes the tail's digest and matches the incoming tail token. Unused tail positions hold fixed padding. At a leaf boundary, inherited records cannot be replaced and a partial tail cannot take a second encoding.

### Beat 3.14: `segment-flush` (index 13, 24 s, scene time 332 to 356)

**Title.** Segment flush

**Camera.** `TREE`.

**Objects and motion.** The lowest node of `tree` has an empty socket (an `onSurface` outline) on its left. 2.0 to 3.0: the 8 tiles of `tail` close up into one segment token (size 0.8, content `'seg:new'`) that rises to the socket's height. 3.2 to 4.2: a `step` cursor climbs the column from the empty socket through three nodes, crosses the break and reaches the root: the path of the empty position; each sibling token on the path lights for 0.3 s. 4.6 to 5.6: the segment token enters the socket and the cursor climbs the same nodes again with the same siblings; the root token changes content from `'root:pre'` to `'root:post'`. `tail` is now empty. The flush fill of `budget` grows to 2.15 (5.6 to 6.2). Still pose: segment in place, lit path with siblings, new root.

**Labels.**

- Segment flush | on: the flush unit
- Empty position | on: the old segment position
- Same address path | on: the shared path
- Shared siblings | on: the sibling tokens
- New root | on: the outgoing root token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The dense MEMORY BACKEND turns the full 8-cell tail into a segment. Its flush proves that the next segment position is empty under the current tree root. The new segment follows the same address path and sibling hashes to compute the new root. The matching paths stop the flush from overwriting an occupied segment or placing records elsewhere.

### Beat 3.15: `memory-outlasts-counter` (index 14, 22 s, scene time 356 to 378)

**Title.** Memory outlasts counter

**Camera.** `TREE` (no move).

**Objects and motion.** 2.0: the depth ruler beside the tree lights its end ticks. 3.0: two rulers appear in front of the tree at y 0.05, z -1.2, from x 5.5 to 13.5, one above the other (z -1.2 and z -0.6): the step-counter ruler ends at three quarters of the length with an end tick; the memory-capacity ruler runs the full length; each has a break mark. A third short bar, three times the counter mark in the drawing's logarithmic scale, ends between the two end ticks. 4.0: one verification sweep across the three. An address cell beside the root gets a gold outline. Still pose: tree with depth ruler; counter ruler shorter than capacity ruler.

**Labels.**

- Depth 63 | on: the tree ruler
- 2<sup>66</sup> cells | on: the memory-capacity ruler
- 64-bit counter | on: the step-counter ruler
- Capacity inequality | on: the arithmetic inset
- Address bound | on: the checked address cube

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The memory tree has depth 63 over segments, a capacity of 2<sup>66</sup> cells. Rules append at most 3 cells per step, and 3 x 2<sup>64</sup> &lt; 2<sup>66</sup>, so memory outlasts the step counter even at the maximum append rate. Address checks keep each record inside that space, so field arithmetic cannot disguise an outside address.

### Beat 3.16: `capacity-closes-slice` (index 15, 22 s, scene time 378 to 400)

**Title.** Capacity closes slice

**Camera.** `MEMV`.

**Objects and motion.** `tail` shows 6 tiles and the flush unit (a titanium bracket 1.0 wide at the right end of `BACKEND`) with a gold fill: used. A small copy of the tape (8 frames at scale 0.5) lies along z 2.6 from x 2 to 6, with a cut marker (a titanium blade icon, 0.1 by 0.6 by 0.8) over it. 2.0 to 3.2: the cut marker moves one frame to the left, to the boundary before the frame whose three append cells would not fit. That frame gets a `step` outline and moves 0.4 to the right, across the marker. Two state tokens (same content, size 0.6) sit on both sides of the marker. Still pose: marker before the waiting frame, equal tokens across the cut.

**Labels.**

- Flush unit | on: the leaf’s flush housing
- Remaining capacity | on: the unoccupied window
- Slice boundary | on: the cut marker
- Next step | on: the waiting frame
- State preserved | on: the matching state tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> A leaf has one flush unit. Its remaining capacity depends on the imported tail and the work already in the slice. When the next step exceeds that capacity, the leaf ends before it, and execution resumes in the next leaf with no extra charge. The unused suffix stays inactive and preserves the machine state.

### Beat 3.17: `read-address-and-time` (index 16, 24 s, scene time 400 to 424)

**Title.** Read address and time

**Camera.** `READ`.

**Objects and motion.** 2.0: at `readPort` the address cell, the kind cell and the running-length marker light. A connector joins the address cell to a `regs` cell; a second joins the marker to the second event cube of `chain`. 3.0: one verification sweep down the three port cells. 4.0: in `win`, the tile at the requested address gets a gold outline; a line across the tray at the marker's position separates it from the two later tiles, which get the `step` colour (appended later in the slice). Still pose: port, permitted tile behind the marker line, later tiles beyond it.

**Labels.**

- Read port | on: CORE’s read interface
- Requested address | on: the address cube
- Expected kind | on: the kind cube
- Running length | on: the append-chain marker
- Later append | on: a record beyond that marker

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Every read goes through CORE's authenticated read port. The requested address equals the address in the requesting register or preceding operation, and the record kind equals the kind the rule requires. The address must lie below the running memory length at that exact read point. A read cannot reach a later append, take another reference or confuse record kinds.

### Beat 3.18: `scalar-old-read` (index 17, 24 s, scene time 424 to 448)

**Title.** Scalar old read

**Camera.** `TREE`.

**Objects and motion.** In the left `segs` tray one tile has a `step` outline (the saved input). 2.0 to 3.0: a copy of the tile rises to the lowest tree node as a token (size 0.6). 3.0 to 5.4: it climbs: at each node the sibling token slides in from the right and the pair merge into the node (0.4 s per level, a short hash pulse on the node), through the break, to a computed root token at y 5.4, offset 1.4 to the left of the root. 5.8 to 7.0: the computed root meets the root token (content `'root:pre'`) with the equality motion. 7.4: a direct connector grows from the tile to `readPort`; it does not touch the bus. The old-read fill of `budget` grows to 1.16. Still pose: lit path with siblings, one outline at the root, connector to the read port.

**Labels.**

- Saved input | on: the old record
- Scalar cold read | on: the direct read interface
- Merkle opening | on: the sibling path
- Computed root | on: the reconstructed token
- Pre-state root | on: the matching incoming root

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The last use of the saved input requests an old record from a completed segment. A scalar cold read, one isolated authenticated memory read, hashes the record and follows its Merkle opening: the sibling hashes along the address path. The computed root must equal the leaf's pre-state root, which rules out a forged payload, wrong kind or substituted address.

### Beat 3.19: `authenticated-cache-line` (index 18, 24 s, scene time 448 to 472)

**Title.** Authenticated cache line

**Camera.** `PAGE`.

**Objects and motion.** 2.0: on `PAGE` the first line tray holds 8 tiles (an old segment's records). A marker on the `segs` rail shows its segment position, left of the imported tail. 2.6: one verification sweep across the 8 tiles; a connector climbs from the tray to the root token (`'root:pre'`). 4.0 to 5.0: the 8 tiles each get a short branch to the bus lip, with a 2-pip chip cell on the shared PAGE branch. 6.0 to 8.4: two tuple bars leave CORE one after the other (lane 0, then lane 1) and each lands on a different tile of the line. The second line tray stays empty and dim. Still pose: one opened line, eight sockets, two answered requests.

**Labels.**

- PAGE/ROM | on: the cache housing
- 8-record cache line | on: the line tray
- Pre-state root | on: the authentication target
- Old segment | on: the segment-position marker
- MEM providers | on: the line’s BUS sockets

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> PAGE/ROM also serves old memory as an 8-record cache line. The dense MEMORY BACKEND hashes the line and checks its path against the same pre-state root. The line's segment must lie before the imported tail. Its checked record cells become MEM providers on BUS. Repeated reads are authenticated, and the cache can neither invent contents nor overlap the window.

### Beat 3.20: `authenticated-rom-line` (index 19, 24 s, scene time 472 to 496)

**Title.** Authenticated ROM line

**Camera.** `PAGE` (no move).

**Objects and motion.** The second line tray fills with 8 `program` coloured tiles (a ROM line). 2.0 to 3.2: a connector grows from the tray toward the left edge of the frame and ends on a `program` token (size 0.8, content `'prog:main'`) at (-1.5, 0.6, 5.2): the program root. 2.6: one verification sweep across the 8 tiles. 4.0 to 5.4: a tuple bar with a 4-pip chip cell leaves CORE by lane 0, takes the PAGE branch and lands on one tile with the equality motion. Still pose: ROM line opened against the program root, one answered request.

**Labels.**

- ROM line | on: the program-row tray
- Table and row | on: the requested key
- Authenticated path | on: the ROM path
- Program root | on: the root token
- `prog_d` | on: the violet token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1. The program root token in the scene and `prog_d` in the carrier show the same face.

**Narration.**

> ROM lines authenticate program table rows against the program root that `prog_d` binds. CORE fixes the requested table and range-checks the row index. PAGE/ROM derives each provider key from the authenticated path and row position, and the served cells equal the hashed cells. A row from another table or another program cannot supply an instruction, type or constant.

### Beat 3.21: `frame-chain-push` (index 20, 24 s, scene time 496 to 520)

**Title.** Frame chain push

**Camera.** `STACK`.

**Objects and motion.** FRAMES has two links and a cached top cell; TASKS has one link and a cached top cell, with a different domain cell (gold, one notch against two) at the base of each chain. 2.0 to 5.0: the hash motion at `hashS`: inputs are the FRAMES cached top cell, a depth cell, and a copy of the top link token; the output is a new link token that rises onto the top of the FRAMES chain. 5.4 to 6.2: a new cell (the frame of the named call) drops onto the FRAMES shelf as the cached top. TASKS does not move. Still pose: FRAMES one link taller with a new top; TASKS unchanged.

**Labels.**

- STACKS | on: the stack housing
- FRAMES | on: the continuation chain
- TASKS | on: the helper chain
- Previous top | on: the spilled frame
- Stack depth | on: the depth cube
- Cached top | on: the newly installed frame

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> STACKS keeps FRAMES and TASKS as Poseidon hash chains with cached top records. FRAMES stores continuations and TASKS stores helper work. Pushing during the named call hashes the previous top, its depth and the preceding chain into a new link, and the new frame becomes the cached top. Separate hash domains stop a task record posing as a frame.

### Beat 3.22: `frame-chain-pop` (index 21, 24 s, scene time 520 to 544)

**Title.** Frame chain pop

**Camera.** `STACK` (no move).

**Objects and motion.** 2.0: a restored frame cell, a depth cell and a predecessor token (a copy of the second FRAMES link) appear in front of `hashS`. 2.4 to 5.4: the hash motion; the output token rises beside the top FRAMES link. 5.8 to 7.0: the equality motion, output onto the top link. 7.4 to 8.2: the top link lowers out of the chain and hides, the restored frame cell rises to the shelf as the cached top, and the predecessor is now the top link. Still pose: FRAMES back to two links, restored frame on the shelf.

**Labels.**

- Frame pop | on: the active stack operation
- Restored frame | on: the restored record
- Predecessor | on: the preceding chain token
- Stored link | on: the authentication target
- Equal link | on: the coincident tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Returning from lift pops FRAMES. STACKS recomputes the current chain link from the restored frame supplied, its depth and its predecessor digest. That hash must equal the stored link before the frame becomes the cached top. Counts and empty-stack constants fix the endpoints. A forged return context, wrong predecessor or frame restored at another depth fails this equality.

### Beat 3.23: `task-rewrite-before-push` (index 22, 24 s, scene time 544 to 568)

**Title.** Task rewrite before push

**Camera.** `STACK` (no move).

**Objects and motion.** The TASKS chain is lit; FRAMES is dimmed. Beside the TASKS shelf, a superseded cursor cell lies dimmed with one ink stroke across it, and the top cell shows the updated cursor (`step` colour). 2.0 to 5.0: the hash motion at `hashS`: the updated parent cell, a depth cell and a copy of the TASKS link; output a new TASKS link. 5.4 to 6.2: a child cell drops onto the shelf as the cached top. A connector runs from both changes to one frame of a three-frame tape strip at scale 0.5 that lies in front of the housing at z 5.5. Still pose: updated parent linked beneath the child, both tied to one frame.

**Labels.**

- TASKS | on: the helper chain
- Updated parent | on: the rewritten task
- Child task | on: the new cached top
- Cursor | on: the updated progress cube
- Same machine step | on: the associated tape frame

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Helper work uses the same checked chain protocol on TASKS. When a helper advances a parent cursor and pushes a child, the top rewrite precedes the push, so the chain commits to the updated parent. Popping the child restores that updated record. CORE checks this order as one machine event, so no work repeats and no cursor update is lost.

### Beat 3.24: `value-digest-at-creation` (index 23, 23 s, scene time 568 to 591)

**Title.** Value digest at creation

**Camera.** `VD`.

**Objects and motion.** 2.0 to 3.0: four cells (a form cell and three content cells) travel from `readPort` along a direct connector to the left of `hashH`. 3.0 to 6.0: the hash motion at `hashH`; output a value token (family `state`, size 0.8, content `'vd:new'`). 6.4 to 7.6: the token travels to `win` and settles on the digest position of the newest tile (it lies on the tile, scale 0.5). The format plate on `VD` is lit. Still pose: value token on the new record, direct connector from the read port.

**Labels.**

- VALUE DIGEST | on: the value-digest housing
- `sherlock-value/1` | on: its format plate
- Authenticated contents | on: the incoming cells
- Value digest | on: the output token
- New record | on: the appended tile

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> VALUE DIGEST computes `sherlock-value/1` whenever a rule creates a value. The value form, scalar contents and authenticated child digests determine the hash. CORE supplies the child records and ROM data through its checked read ports. The resulting digest equals the digest stored with the new record, so a correct-looking record cannot carry an invented semantic value.

### Beat 3.25: `structured-value-checks` (index 24, 24 s, scene time 591 to 615)

**Title.** Structured value checks

**Camera.** `INSET`.

**Objects and motion.** `inset` rises; the plan below dims. Left half: a constructor record (one tile with a small metadata cell) above a row of three child digest tokens (size 0.6) and a group of three progress cells. 2.0 to 4.4: one verification sweep crosses the metadata cell, then the three children in order (each lights as it passes), then a final comparison cell that gets a gold outline. Right half: two tile arrangements of the same value, one with a shared tile (two connectors to one tile), one relocated (the tiles at other positions), each with a value token (content `'vd:same'`). 5.0 to 6.2: the equality motion between the two tokens. Still pose: checked fold on the left, one outline over two arrangements on the right.

**Labels.**

- Constructor identity | on: the constructor metadata
- Ordered fields | on: the child digest row
- Fold state | on: the progress register group
- Completion check | on: the final comparison
- Equal value digest | on: the matching value tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> VALUE DIGEST checks scalar bounds, Boolean values, constructor identity, field order and field count. A structured value combines its children's digests, and constructor processing carries checked fold state across steps. Completion confirms that every field contributes before the value is used. A reference keeps its target's digest. Sharing and relocation keep value identity, and a missing or substituted field fails.

### Beat 3.26: `boundary-body` (index 25, 24 s, scene time 615 to 639)

**Title.** Boundary body

**Camera.** `BND`.

**Objects and motion.** `inset` lowers and hides. 2.0 to 3.4: three groups of cells travel to the left of `hashBody` along three connectors that stay: 8 cells from `regs` (from the left edge of the frame), 2 link tokens from `STACKS`, and the `len` cell, `root` token and `tail_d` token from `BACKEND` (copies; the originals stay). 3.4 to 6.4: the hash motion at `hashBody`; the output is the base disc of a split token (content `'body:A'`), at (9.5, 0.6, 4.2). Still pose: three connectors into the body press, `body_d` disc at its output.

**Labels.**

- BOUNDARY COMMITMENT | on: the boundary housing
- Persistent registers | on: CORE’s body contribution
- Stack state | on: STACKS’ contribution
- `(len, root, tail_d)` | on: the memory contribution
- `body_d` | on: the output token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> BOUNDARY COMMITMENT binds the state that execution needs to continue. Its body hashes the persistent registers, constructor progress, stack hashes and counts, and memory `(len, root, tail_d)`. Here len is the allocated length, root commits completed segments and tail_d commits the partial tail. Each input equals its owning module's checked cell, so the commitment cannot describe another state.

### Beat 3.27: `boundary-header` (index 26, 24 s, scene time 639 to 663)

**Title.** Boundary header

**Camera.** `BND` (no move).

**Objects and motion.** 2.0 to 2.8: above the `body_d` disc, four header objects appear in a row along x at z 3.2: an `outcome_tag` cell, an `outcome_payload` cell, an `input` token (size 0.6, `'input:main'`), a `deps_d` token (size 0.6, state). 3.0 to 6.0: the hash motion at `hashHead`: the four header objects in that order, then the `body_d` disc last. Copies travel; the five originals stay readable. The output is the complete split token (five cap segments on the base, content `'state:A'`) at (13.2, 0.6, 4.2). Still pose: five header fields in order, the state token beside them.

**Labels.**

- `outcome_tag` | on: the header’s outcome-tag cube
- `outcome_payload` | on: the header’s payload cell
- `input_d` | on: the header’s input token
- `deps_d` | on: the dependency token
- `body_d` | on: the body token
- State digest | on: the header hash output

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The boundary header holds `outcome_tag`, `outcome_payload`, `input_d`, `deps_d` and `body_d`. The outcome fields carry the execution status and its value digest, rejection code or exhaustion kind. The dependency digest binds consumed claims. Poseidon hashes the header in this fixed order. Its digest binds the readable outcome fields to the complete body, and checking the header leaves the registers hidden.

### Beat 3.28: `endpoint-digests` (index 27, 24 s, scene time 663 to 687)

**Title.** Endpoint digests

**Camera.** `BND` (no move).

**Objects and motion.** Two split tokens are shown: the entry state (content `'state:pre'`) at (6.2, 0.6, 5.4) and the exit state at (13.2, 0.6, 4.2). Two pin sockets (hexagonal collars 0.3, gold) appear at (6.2, 0.4, 3.0) and (13.2, 0.4, 3.0), each holding a public token of the same content. 2.0 to 3.2 and 3.6 to 4.8: each computed token meets its public token with the equality motion. 6.0: a small pad rises at (9.5, 0.1, 5.6) with three scratch cells joined by two short arrows inside one slot outline; no line leaves the outline. Still pose: two outlines at the two pins, the closed scratch slot.

**Labels.**

- `pre_d` | on: the entry digest pin
- `post_d` | on: the exit digest pin
- Computed digest | on: the matching hash output
- Within-slot scratch | on: the temporary-cell inset
- Producer connection | on: the scratch dependency line

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The entry header digest becomes `pre_d` and the exit header digest becomes `post_d`. Each public token equals its computed hash output. By Poseidon's collision resistance, equal boundary tokens bind equal committed states. Temporary scratch stays outside the boundary because each use depends on a value produced earlier in the same slot. Incoming scratch is unconstrained and cannot change persistent execution.

### Beat 3.29: `genesis-initialization` (index 28, 25 s, scene time 687 to 712)

**Title.** Genesis initialization

**Camera.** `INIT`.

**Objects and motion.** 0 to 1.2: slice swap, `G` into the ribbon (out of frame; the swap still happens). A height cell with an 8-cube collar, all cubes unlit, sits at the left edge of `INIT`; a connector joins it to a selector cell on `INIT` that is lit gold. 2.0 to 4.4: one slow verification sweep (2.4 s) crosses, in order, the `regs` before row, an outcome cell on `INIT`, the three fuel cells, the two empty chain bases on `STACKS` (no links, a base pin only) and an empty tray at scale 0.4 on `INIT`. 5.0: a split token (content `'state:init'`) appears on `INIT`, with connectors to the swept objects and to two small tokens, `'prog:main'` and `'input:main'`. Still pose: INIT wired to an empty state and to the two commitments.

**Labels.**

- INIT | on: the initialization housing
- `h_pre = 0` | on: the first slice’s height cell
- Running | on: its initial outcome tag
- Empty genesis memory | on: the empty tray
- Empty stacks | on: the initial chain endpoints
- Initial state | on: the checked entry registers

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> INIT checks the initial state only when `h_pre = 0`. It proves `Init(prog_d, input_d)`: the entry control, Running outcome, resource counters, empty stacks and empty genesis memory match their prescribed initial values. It also authenticates the program and input commitments. The height check drives the INIT selector, so a prover cannot switch initialization off and start from a convenient intermediate state.

### Beat 3.30: `input-by-digest` (index 29, 25 s, scene time 712 to 737)

**Title.** Input by digest

**Camera.** `INIT` (no move).

**Objects and motion.** The empty tray and the `'input:main'` token stay. A value token (content `'val:input'`, size 0.6, family `input`) lies in a `regs` cell position as a reference. 2.0: one input node appears in front of `INIT` at (-8.5, 0.5, 5.4): four cells. 2.4 to 5.4: the hash motion at `hashS` (the nearest block): the four cells in order; output a token of content `'val:input'`. 5.8 to 7.0: the equality motion, output onto the reference token. 7.4 to 8.4: the node's cells travel to the first `chain` event cube. Still pose: one outline on the reference, the node at the append chain.

**Labels.**

- Input by digest | on: the initial input reference
- Input node | on: the inspected node
- Expected digest | on: the authentication target
- Equal value digest | on: the coincident tokens
- Append port | on: CORE’s record-entry port

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Input admission keeps the value as the digest `val_d` in that empty genesis state. When execution inspects an input node, VALUE DIGEST recomputes its canonical digest and matches it to the expected digest before use. Records enter memory through the checked append chain. The input enters without loading its whole structure at genesis, and unauthenticated input contents cannot influence any transition.

### Beat 3.31: `rules-and-step-interval` (index 30, 24 s, scene time 737 to 761)

**Title.** Rules and step interval

**Camera.** `RIB`.

**Objects and motion.** 0 to 1.2: slice swap, `T` into the ribbon. 2.0 to 2.6: the 16 rule plates lift 0.3 out of their frames; the 6 inactive ones are ceramic. 2.6 to 5.6: the hash motion at a kit hash block that appears at (11.6, 0.5, -5.0): the 16 plates enter in slot order (0.075 s each in the slide phase); output `rules_d` (state, size 0.8). Beside it at (14.6, 0.4, -3.0) two integer bars appear (flat bars 0.9 by 0.15 by 0.3 with ink graduations): `h_pre` and `steps`. 6.4: one verification sweep across the two bars. Still pose: `rules_d` and the two bars in a row at the dock side.

**Labels.**

- Ordered rule ids | on: the slot plates
- Inactive entries | on: the suffix plates
- `rules_d` | on: the rule digest pin
- `h_pre` | on: the starting-height pin
- `steps` | on: the active-count pin

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> CORE computes `rules_d` by hashing the selected rule identifiers in slot order, fixed inactive entries included. It also constrains `steps` to the active slot count and `h_pre` to the slice's starting height, and the resulting height stays within the step counter's range. With these bindings a prover cannot claim another rule sequence or an inconsistent interval for the slice.

### Beat 3.32: `seven-public-pins` (index 31, 24 s, scene time 761 to 785)

**Title.** Seven public pins

**Camera.** `DOCK`.

**Objects and motion.** 0 to 0.4: `tree` hides. 2.0 to 3.4: `lid` appears at its parked height and descends to rest. 3.4 to 4.4: a leaf slab rises at `dock`. Its seven pins, from the left: `binding` (gold hexagonal collar), `prog_d` (collar, `program`), `pre_d` and `post_d` (collars with the five-segment cap ring), `rules_d` (gold collar), `h_pre` and `steps` (flat bars). The `rules_d` token and the two bars of beat 3.31 move onto their pins. 5.0: one verification sweep across the slab. Still pose: slab with seven pins of three shapes.

**Labels.**

- Seven public pins | on: the pin row | kind: state
- `binding` | on: pin 1 | kind: state
- `prog_d` | on: pin 2 | kind: program
- `pre_d` | on: pin 3 | kind: state
- `post_d` | on: pin 4 | kind: state

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The seven public pins are `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`. The binding value connects the proof to its transcript, the data the proof protocol authenticates. The program digest fixes the executed image, and the boundary digests fix the endpoints. The rule digest fixes each slot's choice, and the height and step count place the slice in the run.

### Beat 3.33: `partition-independence` (index 32, 24 s, scene time 785 to 809)

**Title.** Partition independence

**Camera.** `INSET`.

**Objects and motion.** `inset` rises over the closed lid. Two tracks, each a tape of 24 frames at scale 0.4 (9.6 long): track A at z -0.6 with one cut marker after frame 16; track B at z 1.4 with cut markers after frames 10 and 17. At the right end of each track: a tray at scale 0.4 with the same 5 tiles, the same two segment tokens, and a state token (content `'mem:final'`). 2.0 to 3.2: the equality motion, B's token onto A's (B's token travels along z). 4.0: on each track, a fuel gauge icon (as scene 2, scale 0.4) at the same level. Still pose: different cut markers, one outline at the right end.

**Labels.**

- Same execution | on: the matching tapes
- Different cuts | on: the differing slice markers
- Same memory | on: the matching segment and tail arrangements
- Equal final digest | on: the coincident tokens
- No closing charge | on: the unchanged resource registers

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Changing the cuts leaves memory unchanged after the same executed steps. Only real appends advance its length, full segments enter the tree at their sequential positions, and partial tails carry forward unchanged. Closing a leaf adds no record or charge. Cache choices change no state. Every valid partition reaches the same final memory and supports the same run claim.

### Beat 3.34: `bounded-leaf-cost` (index 33, 24 s, scene time 809 to 833)

**Title.** Bounded leaf cost

**Camera.** `COST`.

**Objects and motion.** `inset` lowers and hides. `budget` is the subject: 2.0 to 2.8 its flush fill and 3.2 to 4.0 its old-read fill each get one verification sweep, and two cost plates (titanium, 1.4 by 0.1 by 0.6) rise above the two fills. 5.0: a bracket line spans the whole bar. Along z -5.3 the tape lies in the lane with a break mark. `lid` is at rest: the leaf has one fixed footprint. A flat copy of the depth ruler lies on the lid at (9.5, 1.55, -2.0), 4 long, with its break mark. Still pose: bar with two shares, closed leaf, broken tape.

**Labels.**

- Depth 63 | on: the memory-tree ruler
- 8,821 rows | on: the flush-cost plate
- 4,761 rows | on: the old-read cost plate
- 2<sup>16</sup> rows | on: the complete leaf’s capacity ruler
- Leaf circuit | on: the fixed housing

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> At depth 63 a flush costs 8,821 rows and an old-memory read costs 4,761 rows. Rows are positions in the circuit's constraint layout, and the complete leaf fits 2<sup>16</sup> rows. That fixed capacity covers one slice's checks, and successive leaves cover the long run. Neither the tape's length nor the memory tree's capacity makes a single leaf circuit larger.

### Beat 3.35: `leaf-slabs-handoff` (index 34, 24 s, scene time 833 to 857)

**Title.** Leaf slabs handoff

**Camera.** `COST` at the start; `OUT` at 8.0.

**Objects and motion.** 2.0 to 4.0: two more leaf slabs rise onto the slab at `dock`, stacked with the handoff offset (0.8 in x, 0.4 in y, -0.9 in z per slab), so three slabs stand there, the genesis slab lowest. Above each slab its two boundary tokens (`pre_d` at the left end, `post_d` at the right end; contents `'state:init'`/`'state:0'`, `'state:0'`/`'state:1'`, `'state:1'`/`'state:final'`, size 0.5) and its two integer bars. 10.0 to 12.4: handoff slide. The three slabs move to `ctx.plates.out` and stack with an offset: centres (19.2, 0.35, 0.9), (20, 0.75, 0), (20.8, 1.15, -0.9), pins toward +z, genesis slab lowest. Still pose: the stack on the out plate.

**Labels.**

- Leaf proofs | on: the slab stack | kind: state
- Seven public pins | on: the pin row of the top slab | kind: state
- Boundary digests | on: the tokens over the top slab | kind: state
- Step intervals | on: the integer bars of the top slab | kind: neutral
- `prog_d` | on: the carrier | kind: program
- `input_d` | on: the carrier | kind: input

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1. Out plate: three leaf slabs stacked with an offset, pins toward +z, genesis slab lowest.

**Narration.**

> The checked slices leave as leaf proof slabs, each with its seven public pins. Each proof binds the executed rules, authenticated data and initial and final state of its slice. Neighboring slabs carry the endpoint digests and step intervals that linking needs. The handoff plate takes the slabs to the node, and `prog_d` and `input_d` stay with the run.

## Scene 4: The node

File `scenes/scene4.js`. 17 beats, 370 s (6:10). Scene starts at 20:32 of the piece.

Scene id `node`. Arrives: three leaf slabs stacked on the in plate. Leaves: the root slab with its accumulator ring on the out plate.

**Ideas taken.** The three equalities as three different fits (collar on collar for `prog_d`, split token on split token for the state, bar against bar for the height) are from concept 1, as is the block that the child obligations enter before the ring takes a mark. The fixed caliper that every output slab fits, here and again at the decider, is from concepts 2 and 3. The counter that advances while the slab in the caliper stays still is from concept 3. The paired bars for linear work (leaves below, merges above, ends aligned) are from concept 3, and the throughput bar for three provers is from concept 2. The tree whose slabs re-link into a DAG, in place of a separate diagram, is from concept 3.

**Set.**

| Name | Object | Position (centre) | Size |
|---|---|---|---|
| `rack` | Key rack; leaf and node cards lit | rail centre (-6, 4.2, -5.4) | kit |
| `node` | Node housing | (-6, 0.15, 0) | 9 by 0.3 by 7, ceramic, titanium rim |
| `sockL`, `sockR` | Child sockets (recessed outlines, slab footprint) | (-8, 0.45, 0), (-4, 0.45, 0) | 3.1 by 0.05 by 2.1 |
| `sockW` | Third socket: the work socket; in a plain MERGE it holds a flat neutral plate | (-6, 0.32, 2.6) | 4 by 0.05 by 1.2; neutral plate 3.6 by 0.06 by 0.8, ceramic dimmed |
| `leafG` | Genesis leaf slab (7 pins) | starts in the in-plate stack | kit |
| `nodeR` | A node slab (5 pins, ring with 1 mark) that covers the slices after the genesis slice, up to the state `'state:1'` | `sockR` | kit |
| tokens over a child | Above each child slab, at y 1.3: a `program` token (size 0.6) over the middle, the entry boundary token over the left end and the exit boundary token over the right end (split tokens, size 0.6). Contents: `leafG` `'state:init'` and `'state:0'`; `nodeR` `'state:0'` and `'state:1'` | | |
| `rail` | Interval rail with integer bars standing on it: `L.h_lo`, `L.h_hi`, `R.h_lo`, `R.h_hi` (flat bars 0.9 by 0.15 by 0.3) | line z 3.9, x -10 to -2; bars at x -9.4, -6.5, -5.5, -2.6 | titanium line, bars titanium with ink graduations |
| `hcol`, `hruler` | Height counter: 8-cube collar, and its range ruler with a break mark and an end tick | collar (-9.5, 0.3, 5.3); ruler x -8 to 2, z 5.3 | kit |
| `fold` | Fold block | (-1.6, 0.6, 0) | Box 1.0 by 0.9 by 1.0, gold, with two input notches on -x and one on +x |
| `out` | Output position inside a fixed caliper (two posts) | slab at (1.4, 0.45, 0); posts at x -0.25 and 3.05 | posts 0.2 by 1.0 by 2.4, titanium |
| `nodeOut` | Output node slab (5 pins) with its ring | appears at `out` | kit |
| `treeT` | Proof tree drawn with slabs at scale 0.35: four leaves at z 0.8, x 6.5, 9, 11.5, 14; two nodes at z -1.2, x 7.75, 12.75; root at z -3.2, x 10.25; connectors between levels; a break mark under the leaf row | | each mini slab keeps its ring |
| `ratio` | Three bars with a common left end at x 5.5: machine steps (z 2.4), leaf proofs (z 3.0), node proofs (z 3.6); each grows to x 15 | | titanium frames 9.5 by 0.1 by 0.3 with gold fills |
| `prov` | Three prover housings | (6.5, 0.6, 5.0), (10.25, 0.6, 5.0), (14, 0.6, 5.0) | 2.6 by 1.2 by 1.6, ceramic |
| `side` | Raised side tray over the tree area | (10.25, 1.6, -1.2) | 10 by 0.1 by 5.4 |
| `store` | Checkpoint recess and tray `ckpt` | recess (-13.4, 0.3, 4.0); tray starts at (-13.4, 0.5, 0.5) | recess 3.6 by 0.6 by 2.6 open box, titanium; tray 3.2 by 0.1 by 2.2 carrying 4 cells, 3 tiles, 2 link tokens and two tokens `'prog:main'`, `'input:main'` (size 0.5) |

**Shots.**

| Name | Pose | | Name | Pose |
|---|---|---|---|---|
| `IN` | `establish([-10, 1, 0], 30)` | | `TREE` | `inspect([10.25, 0, 0.6], 13, 'top')` |
| `NODE` | `inspect([-6, 0, 0.6], 12, 'top')` | | `PROV` | `establish([10.25, 0.8, 2.5], 14)` |
| `PINS` | `establish([-6, 0.9, 1.0], 10)` | | `SIDE` | `inspect([10.25, 1.6, -1.2], 12, 'top')` |
| `RAIL` | `inspect([-5, 0, 4.4], 13, 'top')` | | `CKPT` | `establish([-9.5, 0.8, 1.5], 15)` |
| `OUTV` | `establish([-1.5, 0.8, 0.3], 13)` | | `OUT` | `establish([10, 1, 0], 30)` |


### Beat 4.1: `leaf-arrival` (index 0, 22 s, scene time 0 to 22)

**Title.** Leaf arrival

**Camera.** `IN`, first key at scene time 0.

**Objects and motion.** First pose: the three-slab stack on the in plate as scene 3 left it; `nodeR` in `sockR`; `rack` lit. 2.0 to 3.6: `leafG` (the lowest slab) slides to `sockL`. 3.6 to 5.0: the other two leaf slabs slide to the tree area and become the two right-hand leaves of `treeT` (scale to 0.35 on the way). Still pose: a leaf slab and a node slab side by side in the two sockets, the same footprint, under the rack.

**Labels.**

- Leaf proof | on: the arriving leaf slab
- Node proof | on: the stationary node slab
- Pinned keys | on: the wall rack
- `k = 20` | on: the node housing

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1 (shell default). In plate: the stack in the first pose, then empty.

**Narration.**

> Leaf proofs arrive with their authenticated public values. The node treats each proof as a segment, a consecutive interval of the run. Its self-recursive circuit verifies leaf proofs and proofs of its own circuit under pinned keys: keys that the verifier fixes. The circuit size setting is `k = 20` throughout the tree.

### Beat 4.2: `authenticate-children` (index 1, 24 s, scene time 22 to 46)

**Title.** Authenticated children

**Camera.** `NODE`.

**Objects and motion.** 2.0 to 3.0: two connectors rise from the slabs to the rack: `leafG` to the leaf card, `nodeR` to the node card. The pins of both slabs take their family colours. 3.4: one verification sweep that crosses `sockL`, then `sockR`, in one pass (0.8 s for this beat). Both gold edges flash. `sockW` holds its neutral plate, unlit. Still pose: two authenticated children wired to two cards, one neutral socket.

**Labels.**

- Child proof | on: the active slab
- Public values | on: its front-edge pins
- Key selector | on: the connection to its key card
- Neutral encoding | on: the unused socket

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Child verification binds each proof to the exact values it publishes. A key selector chooses the permitted proof type and its matching key together. A prover cannot attach altered boundaries to a valid proof or swap in a circuit that has weaker checks. Unused child slots and the initial carry use neutral encodings, which add no verification obligation.

### Beat 4.3: `bound-the-heights` (index 2, 22 s, scene time 46 to 68)

**Title.** Bounded heights

**Camera.** `RAIL`.

**Objects and motion.** 2.0 to 3.2: copies of the `h_pre` and `steps` pins of `leafG` (flat bars) leave the slab and settle on `rail` as `L.h_lo` and, 0.3 s later, `L.h_hi`; a connector from the `steps` bar spans the two. 3.6: one verification sweep along the rail from `L.h_lo` to `L.h_hi`. 4.4: `hcol` and `hruler` appear; a connector joins `L.h_hi` to the collar. Still pose: two bars on the rail, collar and broken ruler in front.

**Labels.**

- `h_pre` | on: the leaf’s starting-height cube
- `steps` | on: the leaf’s step-count cube
- `h_lo` | on: the segment’s starting-height cube
- `h_hi` | on: the segment’s ending-height cube
- 64-bit counter | on: the counter housing
- Below 2<sup>64</sup> | on: the height ruler

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> A leaf enters with `h_lo = h_pre` and `h_hi = h_pre + steps`. Heights count completed machine steps. The node checks that `steps` is positive and fits the leaf, and that both heights stay below 2<sup>64</sup>. The 64-bit counter cannot wrap, so no invalid interval passes as valid. Its range is not a circuit size.

### Beat 4.4: `match-the-program` (index 3, 16 s, scene time 68 to 84)

**Title.** Program match

**Camera.** `PINS`.

**Objects and motion.** The three tokens over each child are shown. 2.0 to 3.2: the equality motion: the `program` token over `nodeR` moves to the `program` token over `leafG`. The single outline then moves to the midpoint between the slabs (3.4 to 4.0) and stays there. Still pose: one program token over two slabs.

**Labels.**

- MERGE | on: the node housing
- Left segment | on: the left slab
- Right segment | on: the right slab
- `prog_d` | on: the coincident program tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> MERGE combines adjacent segments. It first requires the left child's `prog_d` to equal the right child's `prog_d`, so both proofs describe the same program. Execution spliced from different programs cannot form a segment that claims one program identity.

### Beat 4.5: `match-the-state` (index 4, 20 s, scene time 84 to 104)

**Title.** State match

**Camera.** `PINS` (no move).

**Objects and motion.** 2.0 to 3.2: the equality motion: the entry token of `nodeR` (`'state:0'`) moves onto the exit token of `leafG` (`'state:0'`). 3.6: a connector line runs from the entry token of `leafG` through the shared outline to the exit token of `nodeR`. Still pose: one outline at the inner boundary, one continuous line across both slabs.

**Labels.**

- `L.post_d` | on: the left slab’s ending-state attachment
- `R.pre_d` | on: the right slab’s starting-state attachment
- Equal boundary | on: the coincident tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> MERGE requires `L.post_d == R.pre_d`: the state where the left segment ends equals the state where the right segment begins. The boundary digest binds the entire header and body. Registers, stacks or memory replaced between valid segments fail this check, so disconnected executions cannot pass as one run.

### Beat 4.6: `match-the-height` (index 5, 19 s, scene time 104 to 123)

**Title.** Height match

**Camera.** `RAIL`.

**Objects and motion.** `R.h_lo` and `R.h_hi` stand on the rail. 2.0 to 3.2: the equality motion: `R.h_lo` moves onto `L.h_hi`; the bars butt end to end and show one outline. The two interval lines on the rail now meet at that point. Still pose: three bar positions on one unbroken rail.

**Labels.**

- `L.h_hi` | on: the left interval’s ending-height attachment
- `R.h_lo` | on: the right interval’s starting-height attachment
- Equal height | on: the coincident cubes

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> MERGE also requires `L.h_hi == R.h_lo`: the left segment's ending height equals the right segment's starting height. State equality alone does not fix this position in the run. With height equality, no skipped or overlapping interval can leave a gap or repeated steps in the claimed coverage.

### Beat 4.7: `retain-the-outer-boundaries` (index 6, 22 s, scene time 123 to 145)

**Title.** Outer boundaries

**Camera.** `OUTV`.

**Objects and motion.** 2.0 to 2.6: the two child slabs and the inner outlines dim. 2.6 to 4.0: `nodeOut` appears between the sockets and slides to `out`, between the caliper posts, which it touches on both sides. 4.4 to 5.6: its five pins take their values: copies of the program token, the entry token of `leafG`, the exit token of `nodeR` (collars), and the `L.h_lo` and `R.h_hi` bars travel from their places onto the pins. Still pose: output slab in the caliper with five keyed pins; children dim behind.

**Labels.**

- `prog_d` | on: pin 1 | kind: program
- `L.pre_d` | on: pin 2 | kind: state
- `R.post_d` | on: pin 3 | kind: state
- `L.h_lo` | on: pin 4 | kind: neutral
- `R.h_hi` | on: pin 5 | kind: neutral

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The merged segment publishes `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`. Its first state and height come from the left child. Its last state and height come from the right child. The matching inner digests leave the output, because the node proof has already established their equality. The next merge needs only the outer boundaries.

### Beat 4.8: `fold-the-obligations` (index 7, 23 s, scene time 145 to 168)

**Title.** Folded obligations

**Camera.** `OUTV` (no move).

**Objects and motion.** The ring of `nodeOut` is shown with its gap toward +z and no mark. 2.0 to 3.4: three gold cells leave: one from `sockL`, one from `sockR` (with it, the one mark that `nodeR` already carried, as a second cell), and enter the notches of `fold`. 3.4 to 3.8: `fold` pulses. 3.8 to 4.8: one cell leaves `fold` and lands on the ring as a fold mark. The ring's radius does not change and no second ring appears. The connectors from both sockets to `fold` stay. Still pose: one ring, marked, gap open.

**Labels.**

- Child checks | on: the incoming connections
- Fold | on: their meeting point
- Accumulator | on: the gold ring
- Deferred pairing | on: the ring’s unclosed joint

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Verifying a child proof leaves a pairing check, a cryptographic equation that completes proof verification. The node defers that pairing and folds the new check and every carried child check into one accumulator, a fixed-size encoding of deferred checks. Dropping a child's accumulator discards its verification obligation, so the circuit binds every enabled carry into the fold.

### Beat 4.9: `fixed-node-linear-work` (index 8, 22 s, scene time 168 to 190)

**Title.** Fixed node, linear work

**Camera.** `TREE`.

**Objects and motion.** `treeT` and `ratio` are shown. A thin caliper outline (the slab footprint at scale 0.35) is drawn at the node position (7.75, -1.2) and at the root position. 2.0 to 3.6: a mini node slab moves from (7.75, -1.2) to the root and fits the outline there exactly; its ring keeps its radius and gains a mark. 4.4 to 6.0: the three `ratio` fills grow together to the same right end. Still pose: tree with break mark, slab in the root outline, three bars of equal length.

**Labels.**

- Fixed node size | on: the slab’s footprint outline
- Fixed accumulator size | on: the ring outline
- Machine steps | on: the execution ruler
- Leaf proofs | on: the leaf-work ruler
- Node proofs | on: the node-work ruler
- Linear work | on: the aligned rulers

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The accumulator keeps the same size as more checks enter it. A node at any tree height has the same circuit size and the same per-node proving cost. Longer runs require more leaf proofs and more node proofs. Their numbers grow in proportion to the number of machine steps, so total proving work grows linearly.

### Beat 4.10: `node-proving-cost` (index 9, 18 s, scene time 190 to 208)

**Title.** Node proving cost

**Camera.** `PROV`.

**Objects and motion.** Only the middle prover housing is lit. 2.0 to 2.8: its front panel (a ceramic plate) lowers to show a node slab inside. 3.2: one verification sweep across the slab. Still pose: one prover, one finished slab. The three figures are labels.

**Labels.**

- Reference host | on: the prover housing
- About 65 s | on: the proof-time annotation
- 20 threads | on: the thread annotation
- 17 GB | on: the memory annotation

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> On the reference host, one node proof takes about 65 s at 20 threads and uses 17 GB of memory. That cost is per node proof: a node near the root verifies accumulated work through the same circuit as a node near the leaves.

### Beat 4.11: `parallel-merge-trees` (index 10, 20 s, scene time 208 to 228)

**Title.** Parallel merge trees

**Camera.** `PROV` (no move).

**Objects and motion.** All three housings are lit, panels down. Connectors join each to a pair of ready children in `treeT`. 2.0: three verification sweeps run at the same time, one per housing (parallel work is the subject of this beat). 3.0 to 4.4: the three finished mini slabs move to their merge positions in the tree: the two nodes and the root. An arrow line along z 1.6 from x 5.5 to 15 marks the execution order. A throughput bar at (10.25, 0.1, 6.2), 4 long, fills to 1.73 of a unit mark drawn at one third of its length. Still pose: three slabs placed, arrow, bar.

**Labels.**

- Parallel provers | on: the prover housings
- Ready children | on: the incoming slabs
- Left to right | on: the execution-order rail
- 1.73x throughput | on: the reference-host annotation

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> Machines prove independent leaves and ready merges at the same time. Every merge preserves execution order from left to right, while the tree can group adjacent segments in any shape. Work proceeds whenever the required child proofs are ready. On the reference host, three provers in parallel give 1.73x throughput.

### Beat 4.12: `include-new-work` (index 11, 27 s, scene time 228 to 255)

**Title.** New work in a node

**Camera.** `NODE`.

**Objects and motion.** The children are lit again; `nodeOut` waits dimmed at `out`. 2.0 to 3.4: a tape slice of 4 frames (scale 0.6) slides from the front edge into `sockW`, replacing the neutral plate. 4.0 to 5.2: the entry token of the slice (content `'state:1'`, shown at its left end; its exit token, `'state:final'`, at its right end) meets the exit token of `nodeR` with the equality motion (the slice token travels along z). 5.6: a connector joins the slice to the program token; a bar for its start height meets `R.h_hi` on the rail, and a new bar appears to the right of it, joined to the height pin of `nodeOut`, whose exit pin token changes to `'state:final'`. Still pose: two children and one work slice, three connections.

**Labels.**

- MERGE-WORK | on: the node housing
- Prior segments | on: the child slabs
- New steps | on: the witness-tape slice
- `prog_d` | on: the shared program connection
- `post_d` | on: the preceding segment’s ending-state token
- Below 2<sup>64</sup> | on: the output-height ruler

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> MERGE-WORK combines prior segment proofs with new machine steps inside the same node. The new work uses the leaf's execution checks. Its program must equal `prog_d`, its starting state must equal the right child's `post_d`, and its starting height must equal that child's ending height. The output height includes exactly the new steps and stays below 2<sup>64</sup>. Detached work and uncounted execution cannot enter the segment.

### Beat 4.13: `join-complete-runs` (index 12, 25 s, scene time 255 to 280)

**Title.** Join of complete runs

**Camera.** `SIDE`.

**Objects and motion.** `side` rises. On it, left: two mini node slabs (scale 0.5) at (7, 1.9, -0.2) and (7, 1.9, -2.2), each with a set-root token (state, size 0.6; contents `'set:l'`, `'set:r'`). 2.0 to 5.0: the hash motion at a kit hash block at (10.25, 2.1, -1.2): a gold domain cell, the `l` token, the `r` token; output `set_root` (content `'set:lr'`, size 1.2) at (12.7, 2.3, -1.2), on a third mini slab. The main sockets below stay as they were. Still pose: two child sets, one joined set.

**Labels.**

- JOIN | on: the side-tray operation housing
- `SHK_SET_JOIN` | on: the hash’s domain-tag cell
- `l` | on: the left child set-root token
- `r` | on: the right child set-root token
- `set_root` | on: the emitted token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> JOIN combines sets of claims, statements of complete runs. It computes `set_root = H(SHK_SET_JOIN, l, r)`, where `l` and `r` are the child set roots and `H` is Poseidon. The tag `SHK_SET_JOIN` identifies this hash as a set join. The ordered hash binds both child roots. JOIN needs no execution boundary between independent runs, as MERGE does, and carries no claim counter.

### Beat 4.14: `share-claims-in-a-dag` (index 13, 23 s, scene time 280 to 303)

**Title.** Shared claims in a DAG

**Camera.** `SIDE` (no move).

**Objects and motion.** The JOIN objects slide to the left third of the tray. On the right two thirds: a claim token (state, size 0.8, content `'claim:shared'`) at (10, 1.9, 0.6); two consumer slabs (scale 0.4) at (12, 1.9, -0.4) and (12, 1.9, 1.6), each joined to the claim token by a connector; a join slab at (14, 1.9, 0.6) fed by both. 2.0 to 3.6: one verification sweep travels the graph from the claim token to the join slab along both branches. 4.2: a shape token (state, size 0.8, content `'shape:dag'`) appears at (14, 1.9, -1.8). Still pose: a diamond with one shared claim, and its shape token.

**Labels.**

- Shared claim | on: the claim token
- Consumers | on: the receiving proof slabs
- DAG | on: the graph’s inspection tray
- `shape_d` | on: the shape token
- Same keys | on: the fixed key rack

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> A DAG, a directed acyclic graph, lets a proved claim feed multiple consumers without creating a cycle. Reusing a claim never licenses a duplicated run interval inside MERGE. The address-free shape digest `shape_d` binds the proof graph's connections and operation kinds. Different graph shapes preserve the claims they contain, and the same keys serve every depth.

### Beat 4.15: `retain-a-checkpoint` (index 14, 24 s, scene time 303 to 327)

**Title.** Checkpoint

**Camera.** `CKPT`.

**Objects and motion.** `side` lowers and hides. `leafG` is lit in `sockL` as the proved prefix, with its exit token and its ring. `ckpt` lies in front of it; a connector joins the tray to the exit token of the prefix. 2.0 to 3.8: `ckpt` slides to `store` and lowers into the recess until its top is level with the rim. A thin connector runs from `store` to the rack. Still pose: prefix slab with ring and exit token; tray in the recess.

**Labels.**

- CONTINUATION | on: the checkpoint housing
- Checkpoint | on: the stored tray
- Proved prefix | on: the prefix slab
- Retained witness | on: the resource tray
- `post_d` | on: the prefix’s ending-state token
- Key set | on: the checkpoint’s key connection

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> CONTINUATION saves a checkpoint, the proved prefix and the witness data needed to continue it. The checkpoint retains the prefix proofs and accumulators, its ending height and `post_d`, registers, memory records, stack contents, program and input. Restoring it checks the key set and recomputes the committed state. Altered checkpoint data cannot give the suffix a different starting state.

### Beat 4.16: `resume-the-run` (index 15, 23 s, scene time 327 to 350)

**Title.** Resumed run

**Camera.** `CKPT` (no move).

**Objects and motion.** 2.0 to 3.0: `ckpt` rises out of `store` to rim height. A suffix slab (7 pins) appears at (-4, 0.45, 2.8) with its entry token (`'state:0'`) and a start-height bar. 3.4 to 4.6: the suffix slides into `sockR` (in place of `nodeR`, which is hidden in this beat) and its entry token meets the exit token of the prefix with the equality motion; its start-height bar meets `L.h_hi` on the rail. On the tray, the fuel cell and the counter cell keep their colour: no change. Still pose: prefix and suffix joined, tray beside them.

**Labels.**

- Proved prefix | on: the prefix slab
- Resumed suffix | on: the suffix slab
- `post_d` | on: the prefix’s ending-state attachment
- `pre_d` | on: the suffix’s starting-state attachment
- Saved height | on: the common height position
- Same statement | on: the joined output slab

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1.

**Narration.**

> The resumed suffix starts at the saved height, and its `pre_d` equals the prefix's `post_d`. MERGE checks these equalities exactly as it checks an uninterrupted run. Pausing and resuming execute no machine step and change no fuel or logical counter. Proving the run in parts gives the same final statement as proving it in one go.

### Beat 4.17: `root-handoff` (index 16, 20 s, scene time 350 to 370)

**Title.** Root handoff

**Camera.** `OUTV` at the start; `OUT` at 6.0.

**Objects and motion.** `nodeR` is back in `sockR`. `nodeOut` at `out` is lit: the root slab, five pins (exit pin `'state:final'`), ring with marks and its gap open. 8.0 to 10.4: handoff slide of `nodeOut` to `ctx.plates.out`, centre (20, 0.45, 0), pins toward +z. Still pose: root slab with open ring on the out plate.

**Labels.**

- Root proof | on: the root slab
- Outer boundaries | on: the root’s front-edge pins
- Accumulator | on: the attached gold ring
- Deferred pairing | on: the ring’s unclosed joint

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1. Out plate: the root slab (5 pins toward +z) with its marked, open ring.

**Narration.**

> The root proof carries the complete segment's outer boundaries and one accumulator ring. Every merge check is inside that proof. The accumulator still carries the deferred pairing checks. The decider receives this root so it can establish genesis, authenticate the terminal outcome and settle the accumulated verification obligations.

## Scene 5: The decider

File `scenes/scene5.js`. 14 beats, 311 s (5:11). Scene starts at 26:42 of the piece.

Scene id `decider`. Arrives: the root slab with its open ring on the in plate. Leaves: the decider seal on the out plate, with `prog_d`, `input_d`, `outcome_d` and `n_steps` in the carrier.

**Ideas taken.** The height bar that drops into a zero notch on a gauge, the five header fields that fan out in order beside a closed body disc, and the claim hash with every input as an object in formula order are from concept 3. The plug that closes the ring into a solid disc, the wall with apertures that only the seal and the four values pass, and the two storage layouts of one value are from concept 1. The 8-cube strip for the byte decomposition of `n_steps` and the row budget bar are from concept 2; the hatched cover lines on the wall, which read as "covered" and not as "deleted", are also from concept 2.

**Set.**

| Name | Object | Position (centre) | Size |
|---|---|---|---|
| `rack` | Key rack; node and decider cards lit | rail centre (-6, 4.2, -5.4) | kit |
| `base` | Decider housing floor | (-2.5, 0.15, 0) | 15 by 0.3 by 8, ceramic, titanium rim |
| `cradle` | Root slab position inside the fixed caliper of scene 4 | slab at (-8, 0.45, 0); posts at x -9.65 and -6.35 | as scene 4 |
| `root` | Root slab (5 pins) with ring | arrives on the in plate | kit |
| `gauge` | Height gauge: a ruler with a notch at its left end (the zero position), a break mark and an end tick | x -9.5 to -5, z 2.6 | kit ruler; notch 0.3 wide |
| `post` | The final `post_d`: split token, size 2.0, content `'state:final'`, face up | (-3.5, 0.5, -2.2) | cap ring of five segments |
| `fields` | The five header objects, in order along x at z -3.9: `outcome_tag` cell, `outcome_payload` cell, `input_d` token (size 0.8, `'input:main'`), `deps_d` token (size 0.8, state, `'deps:empty'`), `body_d` disc (size 0.8, state) | x -5.5, -4.5, -3.5, -2.5, -1.5 | |
| `hashD` | Hash block used for every hash of this scene | (1.0, 0.8, -2.2); output at (3.4, 1.0, -2.2) | kit |
| `gate` | Terminal gate: two posts and a lintel | (-1.0, 0.9, 1.6); opening along x | posts 0.2 by 1.2 by 0.2, 1.4 apart; titanium |
| `nstrip` | `n_steps` collar of 8 cubes and, under it, a strip of 8 separate byte cells | collar (-4.5, 0.5, 3.4); strip z 4.2 | as scene 1 collars |
| `nruler` | Range ruler for the count | x -6 to -1, z 5.0 | kit |
| `claim` | Private socket for `claim_d` | (3.4, 0.4, 0.8) | hexagonal recess, titanium |
| `wall` | Shutter wall with four apertures (three hexagonal, one rectangular) at y 0.9 and a slot at bench level for the seal; its -x face carries diagonal ink lines | rest (5.0, 1.6, 0); parked (5.0, 8, 0), hidden when parked | 0.2 by 3.2 by 8, titanium |
| `sealDock` | Seal position on the public side | (8.5, 0.45, 0) | |
| `seal` | Decider seal: slab with 4 pins and a closed ring (a solid gold disc, radius 0.7) | appears in beat 5.12 | kit |
| `pub` | Four public objects on a titanium bar: three tokens (size 0.8) and the `n_steps` collar | bar (8.5, 0.3, 2.8); objects at x 7.0, 8.0, 9.0, 10.2 | bar 4.6 by 0.1 by 0.6 |
| `budget` | Row budget bar | frame x 6.5 to 14.5, z 5.6 | 8 by 0.12 by 0.3, titanium, unfilled |
| `inset` | Inset tray on the right half of the bench (free until beat 5.13) | (10.5, 0.05, -1.5) | 9 by 0.1 by 4.5 |

**Shots.**

| Name | Pose | | Name | Pose |
|---|---|---|---|---|
| `IN` | `establish([-10, 1, 0], 30)` | | `INSET` | `establish([10.5, 0.6, -1.5], 12)` |
| `GAUGE` | `inspect([-7.3, 0, 1.4], 9, 'top')` | | `COUNT` | `inspect([-4, 0, 3.6], 9, 'top')` |
| `HEAD` | `inspect([-3, 0, -3.0], 10, 'top')` | | `RING` | `establish([-8, 0.6, 0], 7)` |
| `HASHV` | `inspect([-0.5, 0, -2.4], 13, 'top')` | | `WALL` | `establish([4.5, 1, 0.5], 22)` |
| `GATE` | `establish([-1, 0.8, 1.6], 8)` | | `OUT` | `establish([10, 1, 0], 30)` |


### Beat 5.1: `root-arrival` (index 0, 21 s, scene time 0 to 21)

**Title.** Root arrival

**Camera.** `IN`, first key at scene time 0.

**Objects and motion.** First pose: `root` on the in plate as scene 4 left it; `wall` parked; the right half of the bench empty. 2.0 to 4.0: `root` slides into `cradle`; it touches both caliper posts. 4.6: a connector drops from the node card to the slab, then one verification sweep across the slab. Still pose: authenticated root slab in the caliper, open ring on top.

**Labels.**

- DECIDER | on: the decider housing
- Root proof | on: the arriving slab
- Pinned node key | on: the key card
- Accumulator | on: the root’s ring

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, others 0. Call `ctx.carrier.show('outcome_d', 0)` and `ctx.carrier.show('n_steps', 0)` through beat 5.12. In plate: `root` in the first pose, then empty.

**Narration.**

> The root slab and its accumulator enter the decider, the circuit that completes verification and publishes the run statement. It verifies the root proof under pinned node keys and authenticates its boundary values and carry. With pinned keys, no prover can swap in a node circuit that accepts an invalid run.

### Beat 5.2: `require-genesis` (index 1, 22 s, scene time 21 to 43)

**Title.** Genesis requirement

**Camera.** `GAUGE`.

**Objects and motion.** 2.0 to 2.8: a copy of the `L.h_lo` pin of `root` (a flat bar) leaves the slab and hovers over the right part of `gauge`. 3.0 to 4.2: the bar slides to the left end and drops into the zero notch; bar and notch show one outline (equality). 5.0 to 6.4: a connector grows from the notch back to the slab and on to the left edge of the frame, where a mini leaf slab (scale 0.35, 7 pins) stands at (-11.3, 0.3, 2.6): the genesis leaf. Still pose: bar in the zero notch, line back to the genesis leaf.

**Labels.**

- SEAL | on: `base`, at the cradle | kind: state
- `h_lo = 0` | on: the bar in the notch | kind: neutral
- Genesis leaf | on: the mini leaf slab | kind: state
- INIT | on: the mini leaf slab, `dy` 1.4 | kind: neutral

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> SEAL, the operation that turns a complete segment into a claim, requires `h_lo == 0`. The authenticated segment then starts at the genesis leaf, where INIT proves `Init(prog_d, input_d)`, so the continuous segment starts from the required initial state. A valid suffix cannot pass as a complete run with everything before it omitted.

### Beat 5.3: `expose-the-header` (index 2, 24 s, scene time 43 to 67)

**Title.** Opened header

**Camera.** `HEAD`.

**Objects and motion.** 2.0 to 3.0: a copy of the `R.post_d` pin token rises from `root` and lands at the `post` position as the size 2.0 split token (the copy travels in from the left edge of the frame). 3.6 to 5.4: the five cap segments leave the rim one after the other (0.3 s each, staggered 0.3 s) and become the five `fields` in their row, in order from the left; the base disc of `post` stays closed and its fifth field, the `body_d` disc, is a small copy of it. Thin connectors join each field to the gap it left. Still pose: capless `post` disc, five fields in a row behind it.

**Labels.**

- `post_d` | on: the authenticated final-state token
- `outcome_tag` | on: the outcome-kind cell
- `outcome_payload` | on: the payload cell
- `input_d` | on: the header’s input token
- `deps_d` | on: the dependency token
- `body_d` | on: the body token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> The final `post_d` opens to five header fields. `outcome_tag` names the outcome kind. `outcome_payload` carries the result digest, rejection code or exhaustion kind. `input_d` binds the input value and the resource policy. `deps_d` commits to the dependency list, and `body_d` commits to the remaining machine state. The body stays behind its digest while the seal reads the header.

### Beat 5.4: `authenticate-the-header` (index 3, 22 s, scene time 67 to 89)

**Title.** Authenticated header

**Camera.** `HASHV`.

**Objects and motion.** 2.0 to 5.0: the hash motion at `hashD`: a gold domain cell (it appears at the head of the file), then copies of the five `fields` in order; the originals stay. Output: a split token of size 1.2, content `'state:final'`. 5.4 to 6.6: the equality motion: the output travels back to `post` and lands on it; `post` shows its five cap segments again inside one outline. Five connectors from the fields to `post` stay. Still pose: one outline on `post`, fields wired to it.

**Labels.**

- BOUNDARY COMMITMENT | on: the header-check assembly
- State domain | on: the fixed domain-tag cell
- Header fields | on: the ordered input tray
- Recomputed digest | on: the hash output attachment
- `post_d` | on: the coincident state tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> BOUNDARY COMMITMENT rehashes the supplied header fields with the fixed state domain tag, the marker that identifies a state hash. The result must equal the root proof's authenticated `post_d`. This equality binds every opened field to the proved final state. A prover cannot replace the input, outcome or dependency list at the seal.

### Beat 5.5: `refuse-running` (index 4, 19 s, scene time 89 to 108)

**Title.** Running state refused

**Camera.** `GATE`.

**Objects and motion.** A slab at scale 0.5 with an open ring (a checkpoint segment) stands at (-3.2, 0.4, 2.8). 2.0: an outcome cell in the `step` colour (a Running state) appears at (-3.0, 0.5, 1.6), left of `gate`. 2.4 to 3.2: the refusal motion toward the gate opening. The copy of the real `outcome_tag` cell (`outcome` colour) waits behind it and does not move in this beat. Still pose: hatched Running cell outside the gate, checkpoint slab beside it.

**Labels.**

- Running | on: the refused outcome cell
- Terminal required | on: the seal gate
- Checkpoint | on: the stationary segment slab

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> SEAL requires the authenticated `outcome_tag` to name a terminal outcome, a state with no successor machine step. Running fails this check. Unfinished execution, halted at an intermediate state, cannot be labeled a completed claim. A checkpoint remains a segment until the run reaches a terminal outcome.

### Beat 5.6: `seal-every-outcome` (index 5, 22 s, scene time 108 to 130)

**Title.** Seal for every outcome

**Camera.** `INSET`.

**Objects and motion.** `inset` rises. Three rows on it at z -2.8, -1.5, -0.2: each row has a terminal cell at x 7.5 (rule-plate shape; `outcome` colour in row 1, `error` colour in rows 2 and 3), a payload cell at x 8.5, a mini seal (scale 0.4) at x 11 and an `outcome` token (size 0.8) at x 13.5 with contents `'outcome:main'`, `'outcome:rejected'`, `'outcome:exhausted'`. 2.0: one verification sweep travels down the three terminal cells; each gets a gold edge. Still pose: three rows, three different outcome faces.

**Labels.**

- Halted | on: the value-result example
- Rejected | on: the rejection-code example
- Exhausted | on: the exhaustion-kind example
- `outcome_d` | on: the Halted example’s token
- `outcome_d` | on: the Rejected example’s token
- `outcome_d` | on: the Exhausted example’s token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> Halted, Rejected and Exhausted each seal as a valid claim. Halted carries a value digest. Rejected carries a rejection code. Exhausted carries an exhaustion kind. Each kind gives its own `outcome_d`. The proof certifies the machine's outcome, including rejection and exhaustion, so successful proof verification does not imply that the program returns a value.

### Beat 5.7: `compute-the-outcome` (index 6, 24 s, scene time 130 to 154)

**Title.** Outcome digest

**Camera.** `HASHV`.

**Objects and motion.** `inset` lowers and hides. 1.6 to 2.4: the real `outcome_tag` cell passes through `gate` (a copy travels from `fields` through the gate opening to the file position). 2.6 to 5.6: the hash motion at `hashD`: a gold domain cell, a version cell, the tag cell, the payload cell; output `outcome_d` (family `outcome`, size 1.2, content `'outcome:main'`). 6.0 to 7.0: the token moves to (3.4, 0.6, -0.6) and lies face up beside two small copies of `prog_d` and `input_d` (size 0.8) that appear at (1.4, 0.6, -0.6) and (2.4, 0.6, -0.6). Still pose: three tokens in a row, no cell attached to them.

**Labels.**

- VALUE DIGEST | on: the outcome-hash assembly
- `sherlock-value/1` | on: the encoding plate
- Outcome kind | on: the authenticated kind cell
- Authenticated payload | on: the payload attachment
- `outcome_d` | on: the emitted teal token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> VALUE DIGEST applies `sherlock-value/1`, the canonical encoding of values and outcomes. The outcome hash binds the outcome domain, version, kind and authenticated payload in their fixed order. For Halted, the payload is the value digest already established by execution. For Rejected and Exhausted, it is the checked code or kind. The kind inside `outcome_d` stops one outcome passing as another.

### Beat 5.8: `equal-values-equal-faces` (index 7, 20 s, scene time 154 to 174)

**Title.** Equal outcome digests

**Camera.** `INSET`.

**Objects and motion.** `inset` rises with a new set. Left: three tiles, two connectors ending on one tile (shared subvalues), under a mini seal at (8, 0.4, -2.8). Right: four tiles with one connector each (separate copies), under a mini seal at (13, 0.4, -2.8). Each seal has an `outcome` token (size 0.8, content `'outcome:same'`) in front of it at z -1.6. 2.0 to 3.2: the equality motion, right token onto left token. Still pose: two seals, two storage drawings, one outcome outline.

**Labels.**

- Shared subvalues | on: the shared storage diagram
- Separate copies | on: the duplicated storage diagram
- Equal value | on: the comparison tray
- `outcome_d` | on: the coincident outcome tokens

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> The value digest follows value content and ordered fields. A reference contributes the digest of its target, so memory addresses and sharing do not enter `outcome_d`. Two Halted runs that compute the same value publish the same `outcome_d`, even when one shares equal subvalues and the other stores separate copies.

### Beat 5.9: `compute-the-claim` (index 8, 24 s, scene time 174 to 198)

**Title.** Claim digest

**Camera.** `HASHV`.

**Objects and motion.** `inset` lowers and hides. 2.0 to 5.0: the hash motion at `hashD`: a gold domain cell, a version cell with one ink pip, then copies of the three tokens of beat 5.7 in order: program, input, outcome. Output `claim_d` (state, size 1.2, content `'claim:main'`). 5.4 to 6.4: `claim_d` moves into the `claim` socket. The `n_steps` collar at `nstrip` is in frame at the lower edge and takes no part. Still pose: five inputs' sources in a row, `claim_d` in its private socket.

**Labels.**

- `SHK_RUN` | on: the domain-tag cell
- Version 1 | on: the version cell
- `prog_d` | on: the program input token
- `input_d` | on: the input token
- `outcome_d` | on: the outcome token
- `claim_d` | on: the emitted claim token

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> The seal computes `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`. `SHK_RUN` identifies a run claim, and `1` identifies its version. The program, input and outcome come from the authenticated run. This digest identifies the claim for replay checks. Neither `n_steps` nor a physical state field enters it, so changing proof cuts does not create a different claim identity.

### Beat 5.10: `authenticate-dependencies` (index 9, 22 s, scene time 198 to 220)

**Title.** Authenticated dependencies

**Camera.** `HEAD`.

**Objects and motion.** The `deps_d` field in `fields` is lit. Beside it at (0.5, 0.5, -3.9) a second route is drawn: a small proof slab (scale 0.4) with three claim tokens (size 0.5, state) attached by connectors, and a list token (size 0.8, state, content `'deps:list'`). A copy of `deps_d` with content `'deps:list'` sits at (-0.6, 0.5, -3.9). 2.0 to 3.2: the equality motion, list token onto that copy. The original `deps_d` field, content `'deps:empty'`, stays in its row with a small empty tray beside it. Still pose: the route with dependencies on the right, the empty-dependency field in the row.

**Labels.**

- SEAL-WITH-DEPS | on: the dependency-enabled seal route
- Earlier claims | on: the attached claim tokens
- Proved dependency list | on: the dependency proof
- `deps_d` | on: the coincident commitment tokens
- Empty dependencies | on: the ordinary SEAL route

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> A run consumes earlier claims at its seal through SEAL-WITH-DEPS. This mode checks the proved dependency list against the opened `deps_d`, so a prover cannot replace or omit the claims the run consumes. Ordinary SEAL requires `deps_d` to equal the empty dependency commitment. A nonempty list therefore requires dependency verification before the run seals.

### Beat 5.11: `bind-the-step-count` (index 10, 24 s, scene time 220 to 244)

**Title.** Step count

**Camera.** `COUNT`.

**Objects and motion.** 2.0 to 3.0: a copy of the `R.h_hi` pin bar of `root` arrives from the left and lands on the `nstrip` collar; a connector joins them. 3.4 to 4.6: the 8 cubes of the collar drop to the strip below, one position each, and eight short connectors show the recombination back to the collar. 5.0: one verification sweep along the strip, then `nruler` appears with its break mark and end tick. No cube shows a value. Still pose: collar, byte strip, ruler.

**Labels.**

- `h_hi` | on: the root’s ending-height cube
- `n_steps` | on: the public counter cube
- 8-byte decomposition | on: the byte strip
- Equal count | on: the recombination connection
- Below 2<sup>64</sup> | on: the counter’s range ruler

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> The decider takes `n_steps` from the root's ending height because the segment begins at genesis. It requires a positive count. It range-checks the count with an 8-byte decomposition: each byte is valid, and recombining the bytes must equal the authenticated count. The count stays below 2<sup>64</sup>. An arbitrary field element or a wrapped count cannot become the reported run length.

### Beat 5.12: `close-the-accumulator` (index 11, 23 s, scene time 244 to 267)

**Title.** Closed accumulator

**Camera.** `RING`.

**Objects and motion.** The ring on `root` has its marks and its open gap. A connector from the node card ends on the ring (the root check). 2.0 to 3.0: a gold arc segment closes the gap. 3.0 to 3.6: a gold plug (cylinder, radius 0.62, height 0.16) lowers into the ring: one solid disc. 3.6: the disc edge flashes `goldBright` for 0.3 s. 5.0 to 6.4: `seal` appears over `root` (same footprint, 4 pins, the closed disc on top) and `root` dims beneath it. Still pose: the seal on the cradle with a closed disc.

**Labels.**

- Root verification | on: the incoming root-check connection
- Accumulator | on: the gold ring
- One pairing | on: the ring’s closing joint
- Checks settled | on: the closed ring

**Carrier and handoff.** Carrier: `prog_d` 1, `input_d` 1, `outcome_d` 0, `n_steps` 0.

**Narration.**

> The decider folds the root verification obligation together with the root's carried accumulator and performs the pairing once. This settles every deferred check in the proof tree. The pairing tests the accumulated equation, including all authenticated child carries. Skipping the pairing or dropping the root carry leaves proof verification incomplete, and no valid decider proof results.

### Beat 5.13: `publish-only-the-statement` (index 12, 24 s, scene time 267 to 291)

**Title.** Public statement

**Camera.** `WALL`.

**Objects and motion.** 2.0 to 3.0: `pub` is set up on the private side at x 2.6: copies of the three tokens of beat 5.7 and the `n_steps` collar move onto the bar. 3.4 to 4.6: `wall` appears at its parked height and lowers to rest. 5.0 to 6.6: the bar's four objects pass through the four apertures and `seal` passes through the slot, to `pub` at (8.5, 0.3, 2.8) and `sealDock`. Everything else stays on the -x side behind the wall: `post`, `fields`, `claim` with `claim_d`, `gauge`, the dimmed `root`. 7.0 to 8.0: `ctx.carrier.show('outcome_d', k, 'outcome:main')` and `ctx.carrier.show('n_steps', k, 'n_steps:main')` rise from 0 to 1. Still pose: wall; seal and four objects on the public side.

**Labels.**

- `prog_d` | on: the public program token
- `input_d` | on: the public input token
- `outcome_d` | on: the public outcome token
- `n_steps` | on: the public counter cube
- Private state | on: the closed shutter

**Carrier and handoff.** Carrier: `outcome_d` and `n_steps` rise 0 to 1 at 7.0 to 8.0; all four are 1 from here on.

**Narration.**

> The decider publishes exactly `[prog_d, input_d, outcome_d, n_steps]`. No state digest, memory root, tail digest, memory length or address becomes public. Those fields describe physical execution boundaries and storage. Leaf cuts choose which boundaries appear inside the proof. With those fields outside the public statement, the claim is independent of those cuts and of the value's storage layout.

### Beat 5.14: `seal-handoff` (index 13, 20 s, scene time 291 to 311)

**Title.** Seal handoff

**Camera.** `WALL` at the start; `OUT` at 6.0.

**Objects and motion.** `budget` appears with a bracket over its whole length. The seal's four pins take their colours: collar `program`, collar `input`, collar `outcome`, flat bar. `pub` and its objects lower and hide (2.0 to 2.6): the carrier holds them. 8.0 to 10.2: handoff slide of `seal` to `ctx.plates.out`, centre (20, 0.45, 0), pins toward +z. Still pose: seal on the out plate, wall behind.

**Labels.**

- Decider seal | on: `seal` | kind: state
- 2<sup>20</sup> rows | on: `budget` | kind: neutral
- `prog_d` | on: the carrier | kind: program
- `input_d` | on: the carrier | kind: input
- `outcome_d` | on: the carrier | kind: outcome
- `n_steps` | on: the carrier | kind: state

**Carrier and handoff.** Carrier: all four 1. Out plate: the decider seal (4 pins toward +z, closed disc on top).

**Narration.**

> The decider fits in 2<sup>20</sup> rows and produces the seal carrying the four public values. The seal certifies a run from `Init(prog_d, input_d)` to the authenticated terminal outcome in `n_steps` steps. The transaction receives the seal with `prog_d`, `input_d`, `outcome_d` and `n_steps`, ready to bind them to its call.

## Scene 6: The transaction

File `scenes/scene6.js`. 4 beats, 93 s (1:33). Scene starts at 31:53 of the piece.

Scene id `transaction`. Arrives: the decider seal on the in plate; the four public values in the carrier. Leaves: the envelope on the out plate.

The shell draws this station's bench in two halves: the prover-side tone for x below 0 and the chain-side tone for x above 0. The scene adds only the threshold bar on the seam. There is no colour blend and no fog change.

**Ideas taken.** The locking bar that couples the four value sockets to the seal's binding pin, and jams when a value is swapped, is from concept 1. The honest envelope and the tampered one side by side in one still is from concept 2, and the failure shown at the one socket that changed is from concept 3. The fee as its own object with its own slot is from concept 3.

**Set.**

| Name | Object | Position (centre) | Size |
|---|---|---|---|
| `seal` | Decider seal | arrives on the in plate | as scene 5 |
| `env` | Envelope: an open case. Pockets: seal pocket (3.2 by 2.2) at local (-0.7, -0.3); four sockets on the front edge at local x -1.9, -0.9, 0.1, 1.2 (three hexagonal, one rectangular), z 1.3; address tab on the back edge; call-data position at local (2.0, 0.4); fee recess at local (2.0, -0.9) | (-9, 0.15, 0) | 5.2 by 0.3 by 3.6, `materials.slate()` with a titanium rim |
| `lock` | Locking bar with four teeth toward the sockets and one tooth toward the seal pocket | on `env` at local z 0.85; it slides 0.3 along x to engage | 4.4 by 0.12 by 0.2, titanium; teeth 0.2 cubes |
| `tab` | Address tab | back edge of `env` | 2.4 by 0.1 by 0.5, gold |
| `env2`, `lock2` | Comparison envelope with the same seal and a different outcome token (content `'outcome:other'`) | (-9, 0.15, 4.0) | as `env` |
| `rack` | Key rack, all three cards | rail centre (3, 4.2, -5.4) | kit |
| `door` | Contract entrance on the chain side: two posts and a lintel | (13, 1.1, 0); opening along x, 4.2 wide in z | posts 0.3 by 2.2 by 0.3, slate with titanium rim |
| `bar` | Threshold bar on the seam | (0, 0.06, 0) | 0.3 by 0.12 by 13, titanium |
| `dust` | Fee object: a disc with a gold edge | starts at (-5.5, 1.6, -2.4) | cylinder radius 0.4, height 0.12, ceramic; gold rim torus |

**Shots.**

| Name | Pose |
|---|---|
| `IN` | `establish([-10, 1, 0], 30)` |
| `ENV` | `inspect([-9, 0, 0], 9, 'top')` |
| `CMP` | `inspect([-9, 0, 2.0], 12, 'top')` |
| `ALL` | `establish([1, 1, 0], 36)` |


### Beat 6.1: `pack-the-call` (index 0, 30 s, scene time 0 to 30)

**Title.** The call

**Camera.** `IN`, first key at scene time 0; `ENV` at 9.0.

**Objects and motion.** First pose: `seal` on the in plate; `env` open and empty at its place; `bar` on the seam. 2.0 to 4.0: `seal` slides into the seal pocket. 4.4 to 6.4: four copies leave the carrier position at the front edge (start at (-9, 0.4, 5.8)) and seat in the four sockets in order: `'prog:main'`, `'input:main'`, `'outcome:main'` (tokens, size 0.8) and the `n_steps` collar. From 9.0 (top view): 11.0 to 11.6 `lock` slides 0.3 and its five teeth engage; one verification sweep along the bar. `tab` is lit. Still pose: open envelope from above: seal, four values, engaged bar, tab.

**Labels.**

- Decider proof | on: the titanium seal
- `prog_d` | on: the violet digest token
- `input_d` | on: the blue digest token
- `outcome_d` | on: the teal digest token
- `n_steps` | on: the step-count position in the carrier
- `SherlockRun.acceptRun` | on: the envelope address

**Carrier and handoff.** Carrier: all four 1. The envelope holds copies. In plate: `seal` in the first pose, then empty.

**Narration.**

> The prover places the decider proof and its four public values in a Midnight transaction, an envelope submitted for ledger validation. `prog_d` is the program image digest, and `input_d` binds the input value digest, the fuel and the stack limit. `outcome_d` is the outcome digest under `sherlock-value/1`, the canonical value encoding. `n_steps` gives the machine step count. The envelope calls `SherlockRun.acceptRun`. Its proof is about 6.4 KB, whatever the length of the run.

### Beat 6.2: `refuse-a-swapped-value` (index 1, 23 s, scene time 30 to 53)

**Title.** Swapped value refused

**Camera.** `CMP`.

**Objects and motion.** `env2` appears in front of `env` with its seal and four values seated; its third socket holds the token with the other face. 2.0 to 2.8: `lock2` starts its 0.3 slide and performs the refusal motion: the third tooth stops against the changed token. 2.8 to 3.4: the refusal hatch appears on the seal of `env2`. A short connector marks the third socket. `env` and `lock` do not move. Still pose: engaged bar above, jammed bar and hatched seal below, the two outcome faces visibly different.

**Labels.**

- Call transcript | on: the original envelope’s public-value strip
- Same proof | on: the comparison seal
- Changed outcome | on: the substituted token
- Refused | on: the red-hatched comparison envelope

**Carrier and handoff.** Carrier: all four 1 (shell default from this scene on).

**Narration.**

> The call transcript, the ordered data bound to the proof, includes these exact public values. When a sender replaces `outcome_d` and keeps the proof, verification against the altered transcript fails and the transaction is refused. A changed `prog_d`, `input_d` or `n_steps` meets the same refusal. A valid proof therefore authorizes only its own claim and step count.

### Beat 6.3: `keep-the-contract` (index 2, 20 s, scene time 53 to 73)

**Title.** One contract

**Camera.** `ALL`.

**Objects and motion.** `env2` lowers and hides (0 to 0.6). `rack` and `door` are lit. 2.0 to 3.2: on `env`, the program token moves from its socket to the call-data position; a thin tooth of `lock` extends with it, so it stays coupled. 4.0 to 5.0: a connector grows from the call-data position across `bar` to `door`, and three connectors drop from the rack cards to `door`. Still pose: envelope on the prover side, entrance on the chain side, one program token as call data, one key rack over both.

**Labels.**

- `prog_d` | on: the program token in the call-data position
- Call data | on: the envelope’s input strip
- Shared key set | on: the gold key rack
- `SherlockRun` | on: the contract entrance

**Carrier and handoff.** Carrier: all four 1 (shell default from this scene on).

**Narration.**

> The contract receives `prog_d` as call data, a value carried by the invocation. One contract and one key set serve every program and every input, so the program identity selects no contract and no verifier key. The proof binds the supplied program digest to the execution it proves.

### Beat 6.4: `cross-to-slate` (index 3, 20 s, scene time 73 to 93)

**Title.** Crossing to the chain

**Camera.** Follow shot (marked exception). Key at the start: `follow([-9, 0.3, 0], [-5, 8, 15])`. Key at 6.0 with `dur: 4.0`: `follow([20, 0.3, 0], [-5, 8, 15])`.

**Objects and motion.** 2.0 to 3.2: `dust` moves down into the fee recess and stays visible in it. 3.6 to 4.2: a lid plate (5.2 by 0.08 by 3.6, slate, with cut-outs over the four sockets, the fee recess and the tab) lowers onto `env`. 6.0 to 10.0: the envelope slides along x from -9 to `ctx.plates.out`, across `bar` and through `door`; the camera moves with it. It ends centred at (20, 0.35, 0), long side along x, sockets toward +z. Still pose: closed envelope on the out plate, values and fee visible through the cut-outs.

**Labels.**

- DUST | on: the transaction fee entry
- Decider proof | on: the seal inside the envelope
- `SherlockRun.acceptRun` | on: the envelope address
- Transaction | on: the outgoing envelope

**Carrier and handoff.** Carrier: all four 1. Out plate: the closed envelope, long side along x, sockets toward +z.

**Narration.**

> The prover pays the transaction fee in DUST, Midnight's fee resource. The envelope carries the decider proof, the public values and the call to `SherlockRun.acceptRun` across to the chain. Its contents give the contract everything it needs to check the run claim. The witness tape stays with the prover.

## Scene 7: The contract

File `scenes/scene7.js`. 21 beats, 530 s (8:50). Scene starts at 33:26 of the piece.

Scene id `contract`. Arrives: the envelope on the in plate. Leaves: the stored claim record on the out plate.

The station has two grounds. The contract (the deck, the `runs` cabinet) stands on the chain-side bench that the shell draws. Proving work that happens off chain (a later run that consumes the claim, a foreign proof on its way into a node) is shown on `ptray`, a wide tray with a `paperWarm` floor: the prover-side tone. The viewer always sees which side a mechanism is on.

**Ideas taken.** Verification and the claim hash as separate beats with every hash input as an object are from concept 3. The drawer with exactly two sockets and a struck-out square where a state digest would be is from concept 2. The second envelope that carries a badge of two different cuts, which ties the refusal back to scene 3, is from concept 1, as are the adapter and the node that holds a Sherlock child and a foreign child side by side. The copy that leaves the drawer for the later run's seal, so that the stored claim stays in place, is from concept 3.

**Set.**

| Name | Object | Position (centre) | Size |
|---|---|---|---|
| `env` | The envelope of scene 6; its lid lifts off in beat 7.1 | arrives on the in plate | as scene 6 |
| `deck` | Contract deck | (-11, 0.15, -1.0) | 7 by 0.3 by 5, `materials.slate()`, titanium rim |
| `vk` | Verifier key card on a post over the deck | (-11, 3.0, -3.2) | kit card at scale 1.4 |
| `hashC` | Hash block of the contract | (-4.6, 0.5, -1.4); output at (-2.2, 0.7, -1.4) | kit |
| `cab` | `runs` cabinet: 3 columns by 3 rows of drawer fronts, each 3.0 by 1.0, facing +z. Three fronts carry claim tokens with other faces (`'claim:a'`, `'claim:b'`, `'claim:c'`); a break mark on the right side shows that the cabinet continues | (3, 1.75, -4.6) | 9.6 by 3.5 by 2.0, slate, titanium rim |
| `drw` | The hero drawer: middle column, middle row. Front with a hexagonal socket; inside, a hexagonal socket, a rectangular socket and a square outline with one ink stroke across it (no state digest) | front at (3, 1.75, -3.6) when closed; pulls out 1.8 toward +z | 3.0 by 1.0 by 2.0 |
| `index` | Index rail in front of the cabinet | line from (-2.2, 0.7, -3.2) to (7.6, 0.7, -3.2) | titanium |
| `ptray` | Prover-side tray | (5, 0.1, 3.0) | 20 by 0.1 by 5.4 (x -5 to 15, z 0.3 to 5.7), `paperWarm` floor, titanium rim |

Objects on `ptray` are given in tray coordinates (u, v): u = x - 5, v = z - 3, at y 0.2 plus half their height. Each beat from 7.7 to 7.20 lists what stands on the tray; what a beat does not list is hidden, unless the beat says it stays. A "path" is a column of 3 small gold nodes (0.4 by 0.25 by 0.4) along v with a sibling token (size 0.4) beside each, ending in a root token.

**Shots.**

| Name | Pose | | Name | Pose |
|---|---|---|---|---|
| `IN` | `establish([-10, 1, 0], 30)` | | `TL` | `inspect([0.5, 0.1, 3], 12, 'top')` |
| `DECK` | `inspect([-11, 0, -1], 9, 'top')` | | `TC` | `inspect([5, 0.1, 3], 13, 'top')` |
| `HASHC` | `inspect([-6, 0, -1.2], 13, 'top')` | | `TR` | `inspect([10, 0.1, 3], 12, 'top')` |
| `CAB` | `establish([3, 1.6, -2.6], 13)` | | `BOTH` | `establish([4, 1, 0], 26)` |
| `DRW` | `establish([3, 1.5, -2.2], 7)` | | `OUT` | `establish([10, 1, 0], 30)` |


### Beat 7.1: `check-the-installed-key` (index 0, 25 s, scene time 0 to 25)

**Title.** Verifier key check

**Camera.** `IN`, first key at scene time 0; `DECK` at 9.0.

**Objects and motion.** First pose: the closed `env` on the in plate. 2.0 to 4.0: `env` slides onto `deck`. 4.4 to 5.2: its lid lifts and hides: seal, four values and tab are exposed. From 9.0 (top view): 11.0: one verification sweep that crosses the seal and then the four sockets (0.8 s); the seal's gold edge flashes, and `vk` flashes with it. `cab` stands at the back, closed. Still pose: verified open envelope on the deck under the key card.

**Labels.**

- `acceptRun` | on: the contract entrance
- Verifier key | on: the fixed gold key card
- Decider proof | on: the seal
- Call transcript | on: the public-value strip
- `runs` | on: the cabinet

**Carrier and handoff.** Carrier: all four 1. In plate: `env` in the first pose, then empty.

**Narration.**

> The transaction reaches `acceptRun`, the contract operation that accepts a proved run. It checks the decider proof against the contract's verifier key, the public key that defines the accepted proof relation, and the call transcript. A failed check refuses the call, so a sender cannot pass off a proof of an unrelated relation. The same verifier key serves every program and input.

### Beat 7.2: `derive-the-claim` (index 1, 27 s, scene time 25 to 52)

**Title.** Claim identity

**Camera.** `HASHC`.

**Objects and motion.** 2.0 to 5.0: the hash motion at `hashC`: a gold domain cell, a version cell with one ink pip, then copies of the three tokens from the envelope sockets in order (program, input, outcome). The originals stay in the envelope, and the `n_steps` collar stays in its socket with a thin `onSurface` outline. Output `claim_d` (state, size 1.2, content `'claim:main'`). Still pose: envelope at the left, block, `claim_d` at the output, the collar still in its socket.

**Labels.**

- `SHK_RUN` | on: the domain marker
- Claim version | on: the version cell
- Poseidon | on: the gold hash block
- `claim_d` | on: the emerging gold-rimmed token
- `n_steps` | on: the stationary step-count position

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The contract recomputes `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`. Poseidon is the digest function. `SHK_RUN` marks the run-claim domain, separating this digest from other kinds of digest, and `1` is the claim version. The remaining inputs come from the verified call.
>
> With the identity derived from these values, no sender can give an existing result a fresh identity. The step count stays outside this identity.

### Beat 7.3: `look-up-the-claim` (index 2, 21 s, scene time 52 to 73)

**Title.** Claim lookup

**Camera.** `CAB`.

**Objects and motion.** 2.0 to 4.4: `claim_d` travels from the hash output to the left end of `index` and along it. It passes the three occupied fronts; at each it pauses 0.3 s beside the stored token: the faces differ. It stops in front of `drw`, whose front socket is empty. Still pose: the token beside the empty socket; three other faces on three other drawers.

**Labels.**

- `runs` | on: the cabinet
- `claim_d` | on: the lookup token
- Claim absent | on: the unused drawer position

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The contract looks up the recomputed `claim_d` in `runs`, its record of accepted run claims. It compares this digest with the stored claim keys. An absent key passes the freshness check. An existing key refuses the call. The lookup stops a valid proof from creating a second acceptance of the same claim.

### Beat 7.4: `store-the-outcome` (index 3, 22 s, scene time 73 to 95)

**Title.** Stored outcome

**Camera.** `DRW`.

**Objects and motion.** 2.0 to 2.8: `drw` pulls out 1.8 and `claim_d` seats in the front socket. 3.2 to 4.8: a copy of the outcome token and a copy of the `n_steps` collar travel from the left edge of the frame (from the envelope) into the two inner sockets. The struck-out square stays empty. The drawer stays pulled out, so the contents show. Still pose: `claim_d` on the front, two stored objects, one empty struck-out square.

**Labels.**

- `claim_d` | on: the drawer front
- `outcome_d` | on: the stored teal token
- `n_steps` | on: the stored step-count object
- `runs` | on: the cabinet

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> For a fresh `claim_d`, the contract stores `outcome_d` and `n_steps` under that key. These are the values that the verified proof binds, so the record preserves both the result identity and the authenticated step count. The contract stores no state digest. Memory roots, tail digests, memory lengths and addresses remain inside the proof.

### Beat 7.5: `refuse-the-same-proof` (index 4, 22 s, scene time 95 to 117)

**Title.** Same proof refused

**Camera.** `CAB`.

**Objects and motion.** A second envelope (open, same seal, same three faces) stands at (-3.6, 0.3, -1.8), at the left end of `index`, with its own `claim_d` copy (`'claim:main'`) above it. 2.0 to 3.4: the copy travels along `index` to `drw` and performs the refusal motion at the occupied front socket: the two faces are equal. 3.4 to 4.0: the refusal hatch appears on the second envelope. `drw` and its contents do not move. Still pose: hatched envelope, token stopped short of an occupied socket.

**Labels.**

- Same proof | on: the repeated seal
- `claim_d` | on: the occupied drawer
- Claim present | on: the cabinet lookup position
- Replay refused | on: the repeated envelope

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> A sender submits the same proof again with the same public values. Proof verification succeeds again, the contract derives the same `claim_d`, and its lookup finds the existing record and refuses the repeated call. The stored outcome and step count remain unchanged. A valid proof lets its claim be accepted once, never repeatedly.

### Beat 7.6: `refuse-another-partition` (index 5, 22 s, scene time 117 to 139)

**Title.** Another partition refused

**Camera.** `CAB` (no move).

**Objects and motion.** The second envelope is replaced by a third at the same place. Its seal carries a small proof token (state, size 0.5, content `'proof:cutB'`; the first seal's is `'proof:cutA'`, shown on `env` in beat 7.1 from this beat's start as well). Behind it, two tape strips of 8 frames at scale 0.3 stand one above the other with cut markers at different places. Its three value faces and its `claim_d` copy equal the stored ones. 2.0 to 3.4: the copy travels along `index` and performs the refusal motion at `drw`. 3.4 to 4.0: hatch on the third envelope. Still pose: different cuts and a different proof token behind an equal claim face, refused.

**Labels.**

- Different leaf boundaries | on: the comparison tape strips
- Different proof | on: the comparison seal
- Same claim | on: the matching engraved claim faces
- Replay refused | on: the comparison envelope

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The sender proves the same run again with different boundaries between leaves. The proof changes, while `prog_d`, `input_d`, `outcome_d` and `n_steps` stay equal. The contract recomputes the same `claim_d` and refuses the call. Leaf boundaries, proof randomness and aggregation shape never enter this identity, so changing the proof's construction cannot bypass replay refusal.

### Beat 7.7: `authenticate-without-counting` (index 6, 24 s, scene time 139 to 163)

**Title.** Authentication without a count

**Camera.** `TL`.

**Objects and motion.** `ptray` rises (it stays up to beat 7.20). On it: a collection slab (node slab, 5 pins) at (-5, 0); three mini slabs (scale 0.35) at (-7.5, -1.2), (-7.5, 0), (-7.5, 1.2) joined to it by connectors; a path from the slab at (-3, 0) that ends, at its foot, on a claim token (`'claim:main'`, size 0.8) at (-3, 1.8). 2.0: one verification sweep across the slab; its edge stays bright for 0.3 s. No counter cell and no collar is on the tray. Still pose: slab, three children, one opening path to the claim.

**Labels.**

- Verified collection | on: the collection slab
- Membership opening | on: the claim-opening path
- `claim_d` | on: the accepted claim token

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The public statement carries no count of claims. A count says nothing about whether a particular claim is valid or present. Proof checks establish validity, digest openings establish membership, and `claim_d` controls replay refusal. Collections combine authenticated children without adding a claim counter, so shared dependencies require no count that grows with every path through the proof graph.

### Beat 7.8: `open-a-sealed-dependency` (index 7, 29 s, scene time 163 to 192)

**Title.** Sealed dependency opened

**Camera.** `TL` (no move).

**Objects and motion.** The slab of beat 7.7 stays and now carries a `set_root` token (state, size 0.8, content `'set:prod'`) on its top. The path is drawn larger: its three nodes at (-2.5, -1.4), (-2.5, -0.2), (-2.5, 1.0), the claim token at (-2.5, 2.0). 2.0 to 5.0: at the middle node, the hash motion on a hash block at scale 0.5 placed at (-0.6, -0.2): the node below and its sibling enter, and the output token moves onto the node above with the equality motion (5.0 to 6.2). The other two nodes light in turn (6.4, 6.8), up to `set_root`. Still pose: lit path from the claim to `set_root`. Across the frame's top edge, `drw` is still visible, pulled out.

**Labels.**

- Verified SET | on: the producer’s collection slab
- `set_root` | on: its root token
- Membership opening | on: the selected path link
- Sealed claim | on: the authenticated claim token

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> A later run consumes this claim at its own seal. Consume means authenticate a claim for use as a dependency. The producer supplies a verified SET, a digest commitment to sealed claims. Hash openings connect the requested claim to that verified root, through joined collections or dependency coverage. Every opening must reproduce its parent digest. An unrelated claim, an unfinished segment or a state digest offered as a result fails here.

### Beat 7.9: `match-the-dependency-list` (index 8, 26 s, scene time 192 to 218)

**Title.** Dependency list match

**Camera.** `TC`.

**Objects and motion.** On the tray: the five header fields of a later run in a row at v -1.2, u -2 to 2 (as scene 5, size 0.6), the `deps_d` token fourth; a discharge chain at v 1.0: three claim tokens (size 0.5) joined in a chain by gold bars, ending in a chain token (state, size 0.8) at (1, 1.0); the producer's path of beat 7.8 at small scale, attached to the first chain link by a connector. The `deps_d` token and the chain token have the same content, `'deps:chain'`. 2.0 to 3.2: the equality motion, `deps_d` onto the chain token (a copy travels; the field stays in the row). Still pose: one outline on the chain token, membership evidence attached.

**Labels.**

- SEAL-WITH-DEPS | on: the later seal’s mode plate
- `deps_d` | on: the final-header token
- Discharge chain | on: the authenticated dependency-chain token
- Equal dependencies | on: the merged outline

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The later run uses SEAL-WITH-DEPS, the seal mode that checks dependencies. Its `deps_d` commits to the ordered dependency list, including repeated uses. The proof constructs a discharge chain from authenticated claims and requires that chain to equal the `deps_d` opened from the run's final state header. Missing, added or reordered entries and removed repetitions fail this equality. Plain SEAL accepts only an empty dependency list.

### Beat 7.10: `link-the-input-value` (index 9, 31 s, scene time 218 to 249)

**Title.** Input value link

**Camera.** `TC` (no move).

**Objects and motion.** On the tray, left group at u -3: the producer's claim token (`'claim:main'`) at (-3, -1.4); below it its `outcome_d` (outcome, size 0.8) opened into a terminal cell (`outcome` colour) and a value token (input family, size 0.8, content `'val:link'`) at (-3, 1.0). Right group at u 3: the consumer's `input_d` (input, size 0.8, content `'input:consumer'`) at (3, -1.4), opened below into a gold domain cell, a value token (`'val:link'`) at (3, 1.0) and three collars (as scene 1) to its right. 2.0 to 3.2: the equality motion: the consumer's value token travels to the midpoint (0, 1.0) and the producer's joins it there (both move). Still pose: two openings, one value outline between them.

**Labels.**

- Producer claim | on: the authenticated producer claim
- `outcome_d` | on: the producer’s outcome token
- Halted | on: the outcome wrapper
- `input_d` | on: the consumer’s input token
- Equal value digests | on: the merged value-digest outline
- SEAL-WITH-DEPS | on: the consumer’s seal mode plate

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> When the earlier result supplies the later input, SEAL-WITH-DEPS recomputes the producer's claim identity and opens its `outcome_d` under `sherlock-value/1`. It requires Halted and compares the resulting value digest with the value digest `val_d` bound in the consumer's `input_d`. A mismatch fails. Rejected and Exhausted outcomes remain usable as claims, but never as input values.
>
> This input link requires SEAL-WITH-DEPS even with an empty dependency list. The later run uses the result without executing the producer again.

### Beat 7.11: `retain-dependency-evidence` (index 10, 24 s, scene time 249 to 273)

**Title.** Retained dependency evidence

**Camera.** `BOTH`.

**Objects and motion.** On the tray: a consumer proof slab (seal, 4 pins) at (-6, 0) with a claim token (`'claim:consumer'`, size 0.8) and a coverage token (state, size 0.6, `'deps:chain'`) on it, and a connector (the input link) back to the value outline of beat 7.10, which stays at (0, 1.0). 2.0 to 4.0: the slab slides along the tray from u -6 to u 6. `drw` stays pulled out in the cabinet with its contents. Still pose: new proof on the prover-side tray, unchanged record in the cabinet.

**Labels.**

- Consumer proof | on: the completed proof slab
- Consumer claim | on: its claim token
- Dependency coverage | on: its coverage token
- Input link | on: its authenticated input-link connection
- `runs` | on: the unchanged producer record cabinet

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The later seal commits its own claim and step count together with the verified dependency coverage and any input link. The resulting proof retains the evidence for its dependencies. Consuming a claim does not remove it from `runs`. The same producer can support other consumers, and repeated uses remain recorded in the dependency list. Each consumer proves its own execution.

### Beat 7.12: `authenticate-foreign-keys` (index 11, 26 s, scene time 273 to 299)

**Title.** Foreign keys authenticated

**Camera.** `TR`.

**Objects and motion.** On the tray: a foreign proof slab at (2, 1.4): a slab with a titanium edge plate (not gold) and no ring. 2.0 to 3.2: it slides in from u -4. Two key cards lie flat at (4, -1.4) and (6, -1.4): the foreign verifier key and the adapter key. From each a path runs toward +u and both end on one `reg_root` token (state, size 1.2, content `'reg:root'`) at (8.5, -1.4). 3.6: one verification sweep across both paths. Still pose: foreign slab, two cards, two paths, one registry root.

**Labels.**

- Foreign proof | on: the incoming slab
- Foreign key | on: the foreign verifier key card
- Adapter key | on: the adapter key card
- Registry membership | on: the opening paths
- `reg_root` | on: the registry-root token

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> A foreign proof, a proof from another supported proof system, enters through an authenticated registry of verifier keys. `reg_root` is the digest committing to that registry. The node checks membership openings for both the foreign key and the adapter key against this root. An adapter is the checked relation that translates verified public inputs into a claim. An unregistered key or adapter fails membership.

### Beat 7.13: `bind-the-verification-rules` (index 12, 25 s, scene time 299 to 324)

**Title.** Verification rules bound

**Camera.** `TR` (no move).

**Objects and motion.** The foreign slab, the foreign key card and `reg_root` stay. Below the card, a registry entry is laid out as a row of four cells at v 0.4, u 3 to 6 (pitch 1.0); a second row of four cells at v 1.2 holds the supplied interpretation. A connector joins the entry row to `reg_root`. 2.0: one verification sweep crosses both rows from the left; as it passes, each pair of cells gets one gold outline. Still pose: two aligned rows, four outlines, the line to `reg_root`.

**Labels.**

- Proof system | on: the proof-system field
- Curve | on: the curve field
- Commitment scheme | on: the commitment-scheme field
- Setup parameters | on: the setup-parameter field
- Registry entry | on: the authenticated entry tray
- `reg_root` | on: the registry-root token

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The registry entry fixes the proof system, the curve used for group arithmetic and the commitment scheme that binds the proof's algebraic data. It also fixes setup parameters, the shared cryptographic parameters used by those commitments. The node checks the supplied verification interpretation against this authenticated entry. Substituted arithmetic or commitment rules fail this check, even under a matching key digest.

### Beat 7.14: `bind-the-input-interpretation` (index 13, 25 s, scene time 324 to 349)

**Title.** Input interpretation bound

**Camera.** `TR` (no move).

**Objects and motion.** The entry row is replaced by its remaining six fields: cells at v 0.4, u 2.5 to 7.5 (pitch 1.0), the fourth a token (state, size 0.6, the adapter digest). The supplied row at v 1.2 has six matching objects. 2.0: one verification sweep crosses both rows; each pair gets one outline. No cell shows a value. Still pose: six matched pairs, `reg_root` above.

**Labels.**

- Transcript rules | on: the transcript field
- Input schema | on: the public-input schema field
- Input count | on: the count field
- Adapter digest | on: the adapter token
- Format version | on: the format field
- Admission policy | on: the policy field

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The entry also binds the transcript rules, public-input schema and exact input count, adapter digest, format version and admission policy. The schema specifies how the public inputs are interpreted. The foreign verifier checks these supplied details against the registered entry. A sender who keeps the registered key still cannot change the input order, omit an input or substitute a different translation.

### Beat 7.15: `parse-the-foreign-proof` (index 14, 25 s, scene time 349 to 374)

**Title.** Foreign proof parsed

**Camera.** `TC`.

**Objects and motion.** On the tray: the foreign slab at (-4, 0) with a strip pulled out of it toward +u: 5 small tokens (size 0.4, state; curve points), then 4 cells (scalars), then a titanium end stop at u 2.4. A format card (a key card shape in titanium) lies at (-1, -1.6). 2.0 to 3.6: one slow verification sweep along the strip to the end stop (1.6 s); each element takes a gold edge as it passes. Nothing lies beyond the stop. Still pose: parsed strip with gold edges, end stop, format card.

**Labels.**

- Proof parser | on: the parsing tray
- Registered format | on: the format card
- Curve points | on: the point-encoding strip
- Canonical scalars | on: the scalar-encoding strip
- Proof boundary | on: the end boundary

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The proof parser decodes the foreign proof under the registered format. It checks curve points against the curve equation and required subgroup, the allowed group of points. It checks scalars, the field numbers used in proof arithmetic, against their canonical encodings. It refuses missing or trailing proof data. Malformed encodings and unconsumed data never acquire the meaning of a valid proof.

### Beat 7.16: `reproduce-the-foreign-transcript` (index 15, 22 s, scene time 374 to 396)

**Title.** Foreign transcript reproduced

**Camera.** `TC` (no move).

**Objects and motion.** The strip stays. 2.0 to 3.0: three public-input cells (input family) join the row after the end stop, at u 3.0 to 4.0. A transcript-rule card (titanium) lies at (0.5, -1.6) with a connector to the row. 3.4: one verification sweep along the whole row. 4.2 to 5.2: three challenge cells (gold) appear at the end of the row, u 5.0 to 6.0, each joined to the row by a connector. Still pose: ordered row, three challenge cells derived at its end.

**Labels.**

- Registered transcript | on: the transcript-rule card
- Proof elements | on: the ordered element row
- Public inputs | on: the input cells
- Derived challenges | on: the resulting challenge cells

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The foreign transcript verifier processes the proof elements and public inputs in the registered order. It derives challenges, the field values that select the proof's verification equations, from that transcript. The arithmetic checks use those derived challenges. A prover-supplied replacement challenge fails the relation. The sender cannot choose favorable equations after seeing the proof data.

### Beat 7.17: `check-the-foreign-equations` (index 16, 26 s, scene time 396 to 422)

**Title.** Foreign equations checked

**Camera.** `TC` (no move).

**Objects and motion.** The row shrinks to scale 0.6 at v -1.6. Below it, the arithmetic tray: from the three challenge cells, connectors run to a group of four evaluation cells at (-3, 0.6) and to a group of three point tokens at (0, 0.6) joined by a bracket (a sum). Two result objects stand at (3.5, 0.2) and (3.5, 1.0): the two sides of the opening equation. 2.0: one verification sweep across evaluations, sum and both sides. 3.0 to 4.2: the equality motion between the two sides. Still pose: challenges wired to the arithmetic, one outline on the equation.

**Labels.**

- Verifier arithmetic | on: the arithmetic tray
- Constraint evaluations | on: the evaluated constraint cells
- Curve-point sum | on: the accumulated group calculation
- Opening equation | on: the final equation
- Derived challenges | on: the connected challenge cells

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The foreign arithmetic verifier evaluates the circuit constraints and consistency equations at the transcript's challenge values. It combines scalar multiples of curve points to form the commitment opening equation. That equation checks claimed evaluations against committed polynomials, the algebraic expressions encoding the circuit constraints. The accumulated arithmetic must match this equation. A valid encoding alone cannot establish that the foreign circuit's constraints hold.

### Beat 7.18: `continue-the-same-verification` (index 17, 28 s, scene time 422 to 450)

**Title.** Continued verification

**Camera.** `TC` (no move).

**Objects and motion.** On the tray: two proof slabs (node slabs, scale 0.6) at (-3.5, -0.8) and (0.5, -0.8), each with a state token over its end: the committed state over the right end of the first (content `'fv:state1'`), the required start state over the left end of the second (same content). An inspection row at v 1.4 shows the state's named fields as four objects: an entry cell, a cursor cell, an arithmetic-state cell and a commitment token (size 0.5). An empty hexagonal socket at (5, -0.8) is the place for the foreign claim. 2.0 to 3.2: the equality motion between the two state tokens. Still pose: two linked work units, the four fields, an empty claim socket.

**Labels.**

- Verification work unit | on: the successor slab
- Committed verification state | on: the merged state-token outline
- Registry entry | on: the bound entry field
- Proof position | on: the cursor field
- Partial arithmetic | on: the arithmetic-state field
- Consumed proof elements | on: the consumed-data commitment

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> Foreign verification proceeds in bounded work units, each proving a portion of the registered verifier. Every unit authenticates its predecessor's committed state before continuing. That state binds the registry entry, phase, proof position, transcript, partial arithmetic and consumed proof elements. A mismatched continuation fails. The schedule admits no skipped, repeated or replaced element and releases no claim before every phase completes. Verification work adds no machine steps to the run.

### Beat 7.19: `finish-the-foreign-check` (index 18, 26 s, scene time 450 to 476)

**Title.** Final foreign check

**Camera.** `TC` (no move).

**Objects and motion.** On the tray: an obligation cell (gold) at (-4, 0); a setup-parameter card (titanium) at (-4, -1.6) with a second, equal card at (-1.5, -1.6) (Sherlock's parameters); two destinations: an accumulator ring (kit, on a mini slab) at (2, -0.8) and a closed check block (a gold box 1.0 by 0.5 by 1.0 with a gold plug on top) at (2, 1.2). Connectors run from the obligation cell to both. 2.0 to 3.2: the equality motion between the two cards. 3.6: one verification sweep along the connector to the ring, which takes a fold mark; the connector to the check block stays unlit with its block dimmed. Still pose: matched cards, lit route to the ring, the other route drawn and dim.

**Labels.**

- Final opening check | on: the final verifier tray
- Registered setup parameters | on: the authenticated parameter card
- Shared accumulator | on: the existing gold ring
- Completed in circuit | on: the foreign verification completion position

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The final verifier checks the commitment opening obligation using the registered setup parameters. When those parameters match Sherlock's, the obligation joins the accumulator, the carried commitment to deferred checks, for the decider's final pairing. A pairing is the cryptographic operation that closes those checks. With different setup parameters, the foreign verifier completes the opening check inside its bounded work units. It never combines incompatible obligations.

### Beat 7.20: `admit-the-foreign-child` (index 19, 29 s, scene time 476 to 505)

**Title.** Foreign child admitted

**Camera.** `TR`.

**Objects and motion.** On the tray: a mini node housing (scale 0.5 of scene 4's) at (6, 0) with a Sherlock child slab in its first socket and its second socket empty. The foreign slab stands at (1, 0) with, on it, an adapter card (titanium), three verified input cells and a foreign claim token (state, size 0.8, content `'claim:foreign'`, inside a square titanium frame, where run claims have none). 2.0 to 3.6: the foreign slab slides into the second socket; two connectors (adapter and foreign verification) stay attached and end on the housing's fold block. Still pose: one node, two children of two kinds.

**Labels.**

- Authenticated adapter | on: the adapter relation
- Verified public inputs | on: the foreign input cells
- Foreign claim | on: the foreign claim token
- FOREIGN | on: the node’s child position
- Adapter obligation | on: the adapter evidence connection
- Foreign obligation | on: the foreign proof evidence connection

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> The authenticated adapter consumes the verified foreign public inputs and derives the foreign claim. The claim binds the foreign key, adapter and translated output in a domain distinct from run claims. An arbitrary foreign field cannot pass for a Sherlock value digest. The proof enters a node as a FOREIGN child, the child type for authenticated foreign claims. The node retains both the adapter's verification obligation and the foreign proof's verification obligation.

### Beat 7.21: `hand-off-the-record` (index 20, 25 s, scene time 505 to 530)

**Title.** Record handoff

**Camera.** `BOTH` at the start; `OUT` at 8.0.

**Objects and motion.** `ptray` lowers and hides (0 to 0.8). 2.0 to 3.4: a copy of `drw` (the record: front with `claim_d`, two stored objects) leaves the cabinet position and comes forward to (3, 0.5, 0); the drawer in the cabinet closes, its front still showing `claim_d`. 10.0 to 12.2: handoff slide of the record to `ctx.plates.out`, centre (20, 0.7, 0), front toward +z, top open. Still pose: the record on the out plate; the cabinet closed behind.

**Labels.**

- `claim_d` | on: the record front | kind: state
- `outcome_d` | on: the stored token | kind: outcome
- `n_steps` | on: the stored collar | kind: state
- `prog_d` | on: the carrier | kind: program
- `input_d` | on: the carrier | kind: input
- Stored claim | on: the record | kind: state

**Carrier and handoff.** Carrier: all four 1. Out plate: the record (drawer copy), front with `claim_d` toward +z, `outcome_d` and the `n_steps` collar visible inside.

**Narration.**

> The contract's accepted state transition leaves the stored claim under `claim_d`, holding `outcome_d` and `n_steps`. The program and input remain bound through the claim identity and verified call. This record is the result the network checks and records. Later proofs authenticate dependencies through their own proof checks, so the stored record and the sealed proof serve distinct parts of the same claim.

## Scene 8: The network

File `scenes/scene8.js`. 8 beats, 202 s (3:22). Scene starts at 42:16 of the piece.

Scene id `network`. Arrives: the stored claim record on the in plate. Leaves: the accepted claim, which is the end of the piece.

**Ideas taken.** The same key card over nodes that hold different programs restates scene 1 and is from concepts 2 and 3. The four carried tokens that leave the carrier and dock at the claim, and the gold line traced along the whole bench in the last shot, are from concept 3. The member path opened from `set_root` is from concept 1, and the collection as a slab with exactly three pins is from concept 3.

**Set.**

| Name | Object | Position (centre) | Size |
|---|---|---|---|
| `record` | The stored claim record: a drawer with `claim_d` on its front (toward +z) and `outcome_d` and the `n_steps` collar in its two sockets | arrives on the in plate | as scene 7 |
| `nodes` | Six node plinths in a shallow arc at the back: x = -12 + 3.6 i for i 0 to 5; z = -3.6 for the outer two, -4.0 for the next two, -4.2 for the middle two. On each: a mini envelope (scale 0.3) lying flat, and behind it a key card (scale 0.7) on a post | | plinth 2.6 by 0.8 by 1.8, `materials.slate()`, titanium rim |
| `plinth` | Claim plinth with four sockets on its front edge (three hexagonal, one rectangular) | (-3, 0.3, 2.4) | 7 by 0.6 by 2.6, slate, titanium rim |
| `ports` | Two reader posts: contract use at (3.2, 0.6, 1.4), later proof at (3.2, 0.6, 3.4) | | posts 0.4 by 1.2 by 0.4, titanium, each with a gold cap |
| `tapeFar` | A strip of the witness tape with a break mark, on a small tray with a `paperWarm` floor | (-13, 0.1, 5.2) | tray 5 by 0.1 by 1.5; 3 frames at scale 0.6 |
| `col` | Collection area: hash block at (10.5, 0.5, 0.4); node slab at (10.5, 0.45, 2.6); collection seal at (13.2, 0.45, -2.4) | x 7 to 15.5 | kit |
| `graph` | Proof graph drawn with mini slabs (scale 0.3) and connectors: two leaves joined, two more joined, the two joins joined, and one claim token used by two of them; break marks on two connectors | x 7.5 to 14.5, z -4.8 to -1.2 | |
| `trace` | Gold line along the whole bench (the one object of the piece that leaves its station) | from local (-300, 0.03, 5.6) to (20, 0.03, 5.6) | box 0.12 by 0.04, gold, length animated |

**Shots.**

| Name | Pose |
|---|---|
| `IN` | `establish([-10, 1, 0], 30)` |
| `NODES` | `establish([-3, 0.8, -2.5], 24)` |
| `ONE` | `establish([-4.8, 0.8, -4.0], 7)` |
| `CLAIM` | `establish([-1, 0.6, 2.4], 13)` |
| `JOIN` | `inspect([10.5, 0, 1.4], 11, 'top')` |
| `GRAPH` | `inspect([11, 0, -2.0], 12, 'top')` |
| `SETTLE` | `establish([11.5, 0.6, -0.5], 13)` |
| `WIDE` | `establish([-140, 0, 0], 330)` |


### Beat 8.1: `check-at-each-node` (index 0, 23 s, scene time 0 to 23)

**Title.** Check at each node

**Camera.** `IN`, first key at scene time 0; `NODES` at 6.0.

**Objects and motion.** First pose: `record` on the in plate. 2.0 to 4.0: `record` slides to `plinth` and stands on it. From 6.0: on the six `nodes`, the mini envelopes are lit. 8.0: six verification sweeps, one per plinth, start at 8.0, 8.15, 8.3, 8.1, 8.25, 8.05 (fixed offsets, so the nodes are visibly independent). After each sweep the key card behind it flashes `goldBright` for 0.3 s and the envelope's rim turns gold. All six key cards are the same card. Still pose: six accepted envelopes under six equal cards; the record on the plinth in front.

**Labels.**

- Midnight nodes | on: the validation-tray group
- Ledger 9.1.0.1 | on: the network station
- Shared verifier key | on: the matching key cards
- Independent verification | on: the validation tracks
- Stored claim | on: the incoming drawer

**Carrier and handoff.** Carrier: all four 1. In plate: `record` in the first pose, then empty.

**Narration.**

> The stored claim reaches network validation as the result of the transaction's contract state transition. Midnight nodes on ledger 9.1.0.1 each verify the transaction independently. Each uses the contract's verifier key and checks the bound public values and contract operations against ledger state. Each node rejects an invalid proof or a replay on its own checks.

### Beat 8.2: `keep-verification-fixed` (index 1, 24 s, scene time 23 to 47)

**Title.** Fixed verification cost

**Camera.** `ONE`.

**Objects and motion.** The third node plinth fills the frame. Its mini envelope is replaced by an open one at scale 0.6 that shows the seal (4 pins, closed disc) and the four value sockets. 2.0: a thin caliper (two posts) closes on the seal: the same footprint as every slab. 3.0: one verification sweep crosses the seal and the four sockets only. `tapeFar` is lit at the far left of the bench; the camera does not go there, and a connector with a break mark runs from it toward the node and stops short of it. Still pose: seal in the caliper under the key card.

**Labels.**

- Decider proof | on: the final seal
- About 6.4 KB | on: the seal’s size annotation
- Shared verifier key | on: the node’s key card
- Witness tape | on: the stationary paper-side tape

**Carrier and handoff.** Carrier: all four 1 (shell default).

**Narration.**

> Each node uses the same verifier key for every program. It checks the final decider proof. It does not execute the witness tape or check each leaf. The proof remains about 6.4 KB. The cost of checking that proof does not depend on the length of the run. Longer execution adds prover work and leaves the node's run-verification work unchanged.

### Beat 8.3: `accept-the-run-fact` (index 2, 22 s, scene time 47 to 69)

**Title.** Accepted run fact

**Camera.** `CLAIM`.

**Objects and motion.** 2.0 to 3.0: `ctx.carrier.show(name, 1 - k)` lowers all four carrier tokens to 0 while four scene tokens with the same contents rise at the front edge at (-3, 0.4, 5.6). 3.0 to 4.6: the program and input tokens seat in the first two sockets of `plinth`; the outcome token and the collar travel to the drawer and merge with the two already stored there (equality motion, both at once). 5.2: one verification sweep across plinth and drawer. 6.0 to 7.0: two gold connectors grow from the drawer front to the two `ports`, whose caps light. Still pose: record with `claim_d` on its front, `prog_d` and `input_d` beside it, `outcome_d` and `n_steps` inside, two read lines.

**Labels.**

- `prog_d` | on: the original program token
- `input_d` | on: the original input token
- `outcome_d` | on: the stored outcome token
- `n_steps` | on: the stored step-count object
- `claim_d` | on: the drawer front
- Accepted claim | on: the complete arrangement

**Carrier and handoff.** Carrier: all four go from 1 to 0 at 2.0 to 3.0 and stay 0 to the end of the piece: the tokens are now at the claim.

**Narration.**

> After acceptance, the claim is a fact on chain: program `prog_d` on input `input_d` produced `outcome_d` in `n_steps` steps. Any contract or later proof can rely on that statement. A contract identifies it through `claim_d` and the stored record. A later proof checks the sealed claim and any required value opening before using its result.

### Beat 8.4: `join-complete-runs` (index 3, 25 s, scene time 69 to 94)

**Title.** Join of complete runs

**Camera.** `JOIN`.

**Objects and motion.** On the node slab at (10.5, 0.45, 2.6) lie two set-root tokens (state, size 0.8, contents `'set:l'`, `'set:r'`), each with a short opening path below it (two small nodes and a claim token, size 0.5). 2.0 to 5.0: the hash motion at the hash block: a gold domain cell, then copies of the left and right roots; output `set_root` (size 1.2, content `'set:lr'`), which settles on the slab between the two. The two paths stay. Still pose: one new root over two child roots and their paths.

**Labels.**

- JOIN | on: the node’s relation plate
- `SHK_SET_JOIN` | on: the domain marker
- Left collection | on: the left child root
- Right collection | on: the right child root
- `set_root` | on: the emerging collection token
- Membership openings | on: the retained child paths

**Carrier and handoff.** Carrier: all 0.

**Narration.**

> Collections combine sealed runs with JOIN, the relation that joins claim sets. It computes `set_root = H(SHK_SET_JOIN, l, r)`, where `H` is Poseidon and `l` and `r` are the authenticated child roots. The domain tag identifies a set join. Recomputing this digest binds both children in order and rejects a substituted child. Membership openings recover each included claim from the resulting root.

### Beat 8.5: `bind-the-proof-graph` (index 4, 25 s, scene time 94 to 119)

**Title.** Proof graph binding

**Camera.** `GRAPH`.

**Objects and motion.** `graph` is lit. 2.0 to 3.0: descriptor cells rise from the graph, one per mini slab (5 cells), and line up at a second hash block that appears at (11, 0.5, -0.4). 3.0 to 6.0: the hash motion; output `shape_d` (state, size 1.2, content `'shape:graph'`) at (13.4, 0.6, -0.4). The graph stays lit behind it; the claim token that two slabs share has two connectors. Still pose: graph, shape token.

**Labels.**

- Proof graph | on: the static graph
- Shared dependency | on: the reused claim token
- Authenticated descriptors | on: the hash-input tray
- `shape_d` | on: the resulting shape token

**Carrier and handoff.** Carrier: all 0.

**Narration.**

> Runs join in any tree or DAG, a directed graph without cycles that permits shared dependencies. `shape_d` commits to the proof graph through relation types, enabled child positions, key selections, child shapes and authenticated work descriptors. Each node derives these fields from its checked children and work. A proof cannot claim another graph. The shape contains no physical state commitment or address.

### Beat 8.6: `settle-the-collection` (index 5, 27 s, scene time 119 to 146)

**Title.** Collection settlement

**Camera.** `SETTLE`.

**Objects and motion.** The collection seal stands at (13.2, 0.45, -2.4) with three pins (gold collars). 2.0 to 3.4: three tokens move onto the pins from the left: `reg_root` (content `'reg:root'`), `set_root`, `shape_d`. A second `reg_root` token with the same content stands on a small slate post at (9.4, 0.8, -2.4): the contract's registry root. 4.0 to 5.2: the equality motion between a copy of the pin's `reg_root` and the post's token. 5.8: one verification sweep across the seal. Under the seal, the two opening paths of beat 8.4 are drawn again at small scale. Still pose: three-pin seal, registry match, paths below.

**Labels.**

- Collection statement | on: the collection seal’s public-value strip
- `reg_root` | on: the registry token
- `set_root` | on: the collection-root token
- `shape_d` | on: the shape token
- Contract registry | on: the registry token read from contract state

**Carrier and handoff.** Carrier: all 0.

**Narration.**

> A collection settles as `[reg_root, set_root, shape_d]`. `reg_root` commits to admitted verifier keys and their interpretation. `set_root` commits to the authenticated claims and their dependency coverage. `shape_d` commits to the graph that combines them. Verification binds these values and requires the registry root to match contract state. A stale registry fails. The final proof size and verification cost stay independent of graph depth and claim count.

### Beat 8.7: `preserve-claim-identity` (index 6, 24 s, scene time 146 to 170)

**Title.** Claim identity preserved

**Camera.** `CLAIM` at the start, with a wider span: `establish([2, 0.6, 1.5], 22)`.

**Objects and motion.** Two small graphs (three mini slabs each, different shapes: a chain and a fork) stand at (5.5, 0.3, -0.6) and (5.5, 0.3, 4.4), each with its own collection-root token and shape token (four different faces) and one member claim token (size 0.8, content `'claim:main'` on both). 2.0 to 3.4: both member tokens travel to the drawer front on `plinth` and land on the `claim_d` there with the equality motion. The collection-root and shape tokens stay with their graphs. The drawer does not change. Still pose: two graphs, four distinct tokens, one claim outline on the drawer.

**Labels.**

- Collection roots | on: the separate collection-root tokens
- Shape digests | on: the separate shape tokens
- `claim_d` | on: the merged claim-key outline
- Shared claim | on: the unchanged accepted drawer

**Carrier and handoff.** Carrier: all 0.

**Narration.**

> Different groupings of the same claims can produce different collection roots and shape digests. These commitments describe the collection. Each member keeps its `claim_d`. Acceptance, replay refusal and consumption use claim identity. A shared claim can feed several branches, and a second path grants no second settlement. Collection verification creates no repeated per-claim effect keyed by collection shape.

### Beat 8.8: `follow-the-whole-path` (index 7, 32 s, scene time 170 to 202)

**Title.** The whole path

**Camera.** Starts on `CLAIM`. Key at 3.0 with `dur: 5.0`: `WIDE` (marked exception: the only wide shot; in stepped mode it is a cut).

**Objects and motion.** `ctx.showAll(true)` from 3.0. Every station stands in its last pose with its handoff object. 8.0 to 14.0: `trace` grows from station 1 to the claim plinth, at constant speed. As its head passes each station boundary, the handoff object there flashes `goldBright` for 0.3 s (the scene knows the eight times; the shell draws the objects, so the flash is a small gold ring that this scene draws above each plate position at local x = 20 - 40 k for k 0 to 7). The six labels sit on the claim and are legible only before the pull-back; from 5.0 show a single label on the claim instead. Still pose: the whole bench, a gold line from end to end, the claim at the right.

**Labels.**

- `prog_d` | on: first plinth socket, to 5.0 | kind: program
- `input_d` | on: second plinth socket, to 5.0 | kind: input
- `outcome_d` | on: drawer socket, to 5.0 | kind: outcome
- `n_steps` | on: drawer collar, to 5.0 | kind: state
- `claim_d` | on: drawer front, to 5.0 | kind: state
- Accepted claim | on: `plinth`, whole beat | kind: state

**Carrier and handoff.** Carrier: all 0. `ctx.showAll(true)` from 3.0. No out handoff: the piece ends here.

**Narration.**

> Moriarty source becomes the ROM program image with `prog_d` and `input_d`. Execution hands over the witness tape and its outcome frame. Leaf proving turns slices into slabs with their public pins. Nodes join the slabs into a root slab carrying its accumulator ring. The decider hands over a seal and `[prog_d, input_d, outcome_d, n_steps]`. The transaction carries them in an envelope. The contract produces the stored claim drawer. Midnight nodes accept the claim, with the same public values bound throughout.