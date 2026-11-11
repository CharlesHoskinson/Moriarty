# Visual Concept 2: The Drawing

## 1. Concept in one paragraph

The workbench becomes an architectural drafting table where all eight stations sit as precision 2.5D plates in shallow bas-relief: recessed rule lines, L-shaped corner registration marks, calibrated scale bars, and flat hardware trays at a uniform `0.06`-unit relief height. Real vertical elevation (`+Y`) is spent only where height means quantity: depth `63` of the memory tree, stack chain links, and recursion rank. Most beats are square-on `Inspect` views aligned to a three-column reading grid (entry at left, mechanism at center, output at right, persistent tokens on the front datum rail, pinned keys on the rear rail) so every end-of-beat pause reads in five seconds like a figure in a technical monograph. Side-by-side comparisons share one split plate with aligned hairlines rather than sequential cuts. The one image a viewer remembers is the scale lock at the center of the bench: a `64`-bit logarithmic ruler broken by architectural zig-zag marks (`16` steps against `2^64`) feeding a stack of `k = 20` titanium proof slabs whose caliper jaws hold the exact same width at height `16` and at height `2^64 - 1`, while the four public tokens on the front rail never shift out of registration.

---

## 2. Object sheet

All world units treat `1.0` unit as `10 cm` on the drafting table. Station centers sit at `X_s = (s - 1) * 11.0` along `+X`, depth `z` in `[-4.2, +4.2]`, bench surface at `y = 0.00`. Bevels are `0.008`-unit 45-degree chamfers so top-left directional light (`pos (6.0, 10.5, 5.4)`) catches crisp drafting edges without specular glare. Materials use `MeshStandardNodeMaterial` from `three/webgpu` with procedural grain maps from `theme.js`.

### 2.1 Canonical data objects

- **Field element / register / cell.** Rounded cube (`BoxGeometry(0.18, 0.08, 0.18)` with `0.02` corner bevel). Matte `ceramicAlabaster` (`0xebe5da`) with `inkSoft` (`0x333c4d`) top hairline border. Instanced per station in one `InstancedMesh` (up to `256` instances) with per-instance color. *Idle*: flat `ceramicAlabaster`, emissive `0x000000`. *Active*: lifts `+0.03` in `Y`, rim tinted `heat` (`0xea580c`), emissive `heatBright` (`0xf97316`) at `0.22` intensity. *Passed*: `0.25 s` flash of `goldBright` (`0xfbbf24`) settling to `ceramicAlabaster` with a `gold` (`0xd4af37`) corner pip. *Refused*: halts, shifts `-0.04` back, overlaid with a `45°` `errorRed` (`0xe11d48`) TSL stripe hatch.
- **Digest token and `5 x 5` dot face.** Flat regular hexagonal prism (`CylinderGeometry(0.28, 0.28, 0.08, 6)`, rotated `30°` so two edges lie parallel to the `X` rail) with a `0.03`-wide raised bezel in the family hue: `gateViolet` (`0x818cf8`) for `prog_d` and `prog_root`; `steel` (`0x0284c7`) for `input_d`; `resolve` (`0x0d9488`) for `outcome_d`; `gold` (`0xd4af37`) for `pre_d`, `post_d`, `body_d`, `rules_d`, `deps_d`, `claim_d`, `set_root`, `reg_root`, and `shape_d`. The recessed face (`ceramicAlabaster` on paper stations, `titaniumSilver` `0xe2e7f0` on slate stations) holds a `5 x 5` matrix of circular pips (`CylinderGeometry(0.024, 0.024, 0.012, 12)`, spacing `0.062`) driven by one shared `InstancedMesh`. A deterministic 32-bit FNV-1a hash of the content tag (for example `"prog:A"`, `"state:leaf0:post"`) sets the `25` bits, normalized to between `9` and `16` active `ink` (`0x14171f`) pips; inactive positions show shallow `ceramicEdge` (`0xd5cec0`) dimples, and a micro-notch on the top-left bezel fixes orientation. **Screen comparison of equal faces**: two tokens stop `0.80` units apart inside a twin caliper frame crossed by five `inkSoft` registration lines through rows `1` to `5`, then slide into physical coincidence at the midpoint; identical pips land dead-on without double edges, and the merged bezel flashes `goldBright`.
- **Poseidon hash block.** Rectangular machined block (`BoxGeometry(0.76, 0.14, 0.52)`), brushed `titanium` (`0xd8dde6`) body with a recessed `gold` (`0xd4af37`) throat channel from `-X` to `+X` and three engraved permutation ribs across the top. One `InstancedMesh` where multiple sponges appear in parallel. *Idle*: matte `titanium` with `gold` throat. *Active*: throat ribs sweep from `-X` to `+X` in `goldBright` (`0xfbbf24`, emissive `0.25`). *Passed*: exit collar holds `gold` rim. *Refused*: exit gate closes and hashes `errorRed` across the top plate.
- **Bus tuple.** Monolithic 5-slot carrier bar (`BoxGeometry(1.04, 0.09, 0.24)`, `titaniumSilver`) holding five `0.18` cubes `(chip, key, a, b, c)`. Slot `0` (`chip`) carries `1` to `4` engraved `ink` bars on its top face to identify `1` ALU, `2` MEM, `3` HASH, `4` ROM without texture text. *Active*: carrier rim glows `heat` (`0xea580c`) while in transit along the bus groove; returns to `titanium` once matched.
- **Memory record and 8-cell segment tray.** One record is a square `ceramicAlabaster` tile (`BoxGeometry(0.22, 0.05, 0.22)`) with four engraved `inkSoft` grooves representing `(kind, a, b, c)` and `1` to `4` corner pips for `kind`. Eight tiles sit in a recessed `ceramicEdge` linear tray (`BoxGeometry(1.96, 0.06, 0.30)`) with `8` numbered division ticks. Padding slots in an unfilled tail show flat `paperWarm` (`0xebe5da`) blank plugs with diagonal recess lines. *Active*: touched tiles tint `heat` (`0xea580c`).
- **Memory tree (`TOP = 63`).** Vertical column (`x` width `1.96`, `y` height `2.80`) rising in `+Y` directly behind the segment tray. Shows levels `0`, `1`, `2` as `gold`-rimmed node bars above the `8`-cell `Tree8` bracket, then an architectural double zig-zag break bar (`y = 1.15` to `1.45`), then levels `61`, `62`, and the `root` token socket at `TOP = 63` (`y = 2.65`), flanked on the left by a vertical `ink` depth ruler ticked at `0`, `1`, `2`, `—╱╲—`, `62`, `63`.
- **Machine step frame and witness tape.** Each step frame is a flat `paperWarm` plate (`BoxGeometry(0.46, 0.04, 0.92)`) bordered in `ceramicEdge`, divided into three horizontal bands: top rule-id tag (`0.16` z), middle `pre` register strip (`0.34` z), bottom `post` register strip (`0.34` z). Sixteen frames butt edge-to-edge along `+X` (`7.36` units per slice) on a recessed rail to form the witness tape. *Active*: the firing frame outlines in `heatBright`.
- **Stack (`FRAMES`, `TASKS`).** Horizontal stepped track (`1.80 x 0.36`) holding a raised `ceramicAlabaster` cached-top tray at the front and a linked chain of overlapping `0.20`-radius `gold` hexagonal tokens receding by `+0.03` steps in `Y`, ending at an engraved `EMPTY` anchor pin.
- **Proof slab.** Precision brushed `titanium` plate (`BoxGeometry(1.60, 0.12, 1.05)`) with a `0.02` `gold` perimeter inlay. Every leaf slab, node slab, and decider seal uses the exact `1.60 x 1.05` footprint. Public lanes protrude along the `+Z` front edge as `0.08 x 0.14` color-coded `titaniumSilver` contact pins (`7` on leaf, `5` on node, `4` on decider seal). *Passed*: sweep line leaves a `0.3 s` `goldBright` perimeter. *Refused*: `errorRed` cross-hatch across the upper face.
- **Accumulator ring.** Lathed `gold` torus (`TorusGeometry(0.24, 0.032, 12, 48)`) seated in a circular recess on the upper-right quadrant of a node slab (`y = 0.13`). Each folded deferred pairing check adds one radial `ink` tick mark (`InstancedMesh` of `BoxGeometry(0.015, 0.02, 0.07)`) around the ring circumference; a second ring never appears. In Scene 5, two semicircular jaws clamp the ring into a solid `gold` disc to show the single closing pairing.
- **Verifier key rack.** Continuous rear drafting ledge (`z = -3.65`, `y = 0.16`) carrying three flat `gold`-rimmed `titanium` cards (`BoxGeometry(0.68, 0.05, 0.42)`): `leaf` ( notched with `7` micro-cuts), `node` (`5` micro-cuts + ring glyph), `decider` (`4` micro-cuts). Fixed in geometry across every program.
- **Claim and `runs` cabinet.** `claim_d` is a `gold`-rimmed digest token. The contract's `runs` map is a shallow frontal index cabinet on the slate station (`BoxGeometry(2.60, 1.80, 0.32)`, `graphiteLight` `0x2a3446` frame over `wellSlate` `0x161d29` recesses) with `16` visible horizontal drawer fronts. Each occupied drawer carries its `claim_d` `5 x 5` face on the left tab and two recessed slots inside (`outcome_d` token and `n_steps` 8-byte bar), plus a ruled empty square stamped with an `inkSoft` diagonal cross to show that no state digest is stored.
- **Transaction envelope.** Flat `graphiteLight` (`0x2a3446`) folio (`BoxGeometry(2.30, 0.10, 1.48)`) with `gold` call-binding circuit traces etched between four token sockets, a `1.60 x 1.05` decider-seal pocket, a `DUST` fee slot, and a `SherlockRun.acceptRun` header tab.

### 2.2 Changes to the system

- **Change to the system 1 — Caliper, Break-Mark, and Row-Budget Kit.** Adds three drafting-instrument primitives in `inkSoft` (`0x333c4d`) and `titanium`: (a) `FixedCaliper`, a rigid `U`-bracket (`1.60` inner span) that straddles proof slabs across scenes to prove constant size visually without numbers; (b) `BreakRule`, a ruled axis interrupted by a `—╱╲—` double zig-zag cut for logarithmic spans (`2^64`, `2^66`, depth `63`); and (c) `RowBudgetBar`, a `2.40 x 0.16` linear gauge (`2^16` in Scene 3, `2^20` in Scenes 4 and 5) whose fill blocks show exact row shares (`8,821` flush, `4,761` cold read) without counting rows. *Reason*: Replaces object counts with standard engineering diagram conventions legible in a still frame.
- **Change to the system 2 — 8-Byte Counter Bar for `64`-bit heights and `n_steps`.** Represents `h_pre`, `h_lo`, `h_hi`, and `n_steps` as eight contiguous `0.09` rounded cubes locked inside one `0.80 x 0.12` `titanium` collar (`8 bytes = 64 bits`). *Reason*: Distinguishes a `64`-bit counter value from a single unstructured field cube, shows directly why `2^64` is a counter's range rather than a circuit size, and makes Scene 5's `8`-byte decomposition of `n_steps` self-evident when the collar opens by `0.03` per byte for range-checking.
- **Change to the system 3 — Opaque Privacy Shutter.** Adds a sliding `titanium` drafting cover (`BoxGeometry(2.80, 0.04, 1.60)`) scored with `45°` `inkSoft` architectural section lines. *Reason*: In Scene 5, sliding a physical shutter over `post_d`, `body_d`, `root`, `tail_d`, `len`, and internal addresses—while the four public pins protrude clear of its front edge—shows cryptographic hiding accurately, whereas fading objects out misleads the viewer into thinking state never existed.
- **Change to the system 4 — Split-Track Comparison Plate.** Divides comparison stations along `z = 0` into Track A (`z = -1.35`) and Track B (`z = +1.15`) linked by a shared right-hand coincidence socket or rear key rail. *Reason*: Keeps two programs (Scene 1), two tape partitions (Scene 3), two equal-value runs (Scene 5), and replay cuts (Scene 7) simultaneously visible in one `Inspect` still pose.

### 2.3 Station housings and shared rails

Stations `1` to `5` sit on `paper` (`0xf4f0e8`) plates framed by `0.02`-wide `ceramicEdge` (`0xd5cec0`) score lines and `0.35`-arm `inkSoft` corner registration marks at `(±5.1, 0.005, ±3.3)`. Station `6` splits down `x = 0.0`: `paper` on `[-5.5, 0.0]`, `graphite` (`0x1c2331`) on `[0.0, +5.5]` separated by a `0.04` `titanium` threshold bar. Stations `7` and `8` sit on `graphite` (`0x1c2331`) plates with `wellSlate` (`0x161d29`) recessed wells and `titanium` (`0xd8dde6`) registration marks. The shell owns two continuous rails across all scenes: the **Token Carrier** at `z = +3.65, y = 0.08` (four sockets holding `prog_d`, `input_d`, and from Scene 5 onward `outcome_d` and `n_steps`) and the **Handoff Plates** (`2.20 x 1.60` `ceramicAlabaster` or `graphiteLight` pads) centered at local `x = ±5.50, z = 0.0` (world boundary between adjacent `11.0`-unit stations).

---

## 3. Motion sheet

Every motion is evaluated statelessly in `seek(t)` via normalized progress `u = clamp((t - t0) / dur, 0, 1)` and `e = easeInOutCubic(u)`. Outside `[t0, t0 + dur]`, every object sits in its exact pre-pose (`u = 0`) or post-pose (`u = 1`). Every beat holds its `u = 1` pose completely still for at least the final `2.0 s`.

- **Poseidon hash (`dur = 1.00 s`).**
  - `0.00–0.30 s`: Ordered input cells lift `+0.03` in `Y` and align along a straight `-X` feeder line (`0.22` pitch) with `heat` rims; everything else holds still.
  - `0.30–0.75 s`: Input line translates `easeInOutCubic` along `+X` into the `gold` throat of the Poseidon block; as each cell crosses the throat plane `x_in`, it scales `Y` to `0.01` inside the block while the three top permutation ribs light sequentially in `goldBright`.
  - `0.75–1.00 s`: One hexagonal digest token translates `0.55` units out the `+X` exit onto its output socket, flashing `goldBright` for `0.15 s` before settling to its family rim hue with its deterministic `5 x 5` dot face visible.
- **Equality by physical coincidence (`dur = 0.80 s`).**
  - `0.00–0.25 s`: Five `inkSoft` horizontal registration hairlines extend between the two tokens (`0.80` units apart); calipers frame both faces so the viewer compares the identical `5 x 5` dot patterns.
  - `0.25–0.65 s`: Both tokens translate symmetrically (`easeInOutCubic`) along the hairlines to the exact midpoint `(x_mid, y + 0.002, z_mid)`. Identical pips coincide cleanly into one sharp `5 x 5` grid.
  - `0.65–0.80 s`: The combined bezel pulses `goldBright` (`emissiveIntensity = 0.45 * (1 - u_tail)`) and locks into a single crisp outline. All surrounding plates hold still.
- **Verification (`dur = 0.40 s`).**
  - `0.00–0.28 s`: A `0.03`-wide `goldBright` scan bar translates at `easeInOutCubic` across the object from `-X` to `+X` (`y_top + 0.005`), connected by a thin `gold` hairline to the active key card on the rear rack.
  - `0.28–0.40 s`: Scan bar vanishes at `+X`; the verified object's perimeter bezel lights `goldBright`, decaying linearly to steady `gold` at `0.40 s`.
- **Refusal (`dur = 0.45 s`).**
  - `0.00–0.22 s`: Moving object or scan bar advances to the constraint gate at `u = 0.65` of its path and halts abruptly (`clamp` cut, zero bounce).
  - `0.22–0.45 s`: Object recoils `-0.12` units along its incoming axis (`easeInOutCubic`); a `45°` `errorRed` (`0xe11d48`) stripe hatch steps to opacity `1.0` across its top face and gate line. Remains frozen in that recoiled hatched state for the rest of the beat.
- **Station handoff (`dur = 1.60 s`).**
  - Scene `N` ends with its output seated at local `(+5.50, y, 0.00)`—identical in world coordinates and pose to Scene `N+1` local `(-5.50, y, 0.00)`. At the start of Scene `N+1`, a `Follow` camera (`dur = 1.60 s`, `easeInOutCubic`) tracks parallel along `+X` as the handoff object slides on a recessed rail from `x = -5.50` into the entry bay at `x = -3.60` while the Token Carrier glides alongside at `z = +3.65`.
- **Added diagram motions (Changes to the system):**
  - **Caliper lock (`dur = 0.50 s`)**: Two `inkSoft` caliper jaws slide inward along `X` until they kiss the `±0.80` edges of a `1.60`-unit proof slab and lock with two `gold` alignment ticks.
  - **Privacy shutter close (`dur = 0.70 s`)**: The hatched `titanium` cover translates in `+Z` (`easeInOutCubic`) over private witness bays, halting cleanly against a datum bar behind the public output pins.
  - **Row-bar step (`dur = 0.50 s`)**: A rectangular fill block inside `RowBudgetBar` scales along `+X` (`easeInOutCubic`) from `0` to its exact row-fraction width (`rows / 2^k`) and locks against a tick line.

---

## 4. Scene by scene, 1 to 8

**Viewport framing rule (all scenes):** On wide viewports (`>= 1024 px`), the DOM narration panel occupies the right `34%` of the window; `shell.js` shifts the camera frustum center into the clear left `66%` via `camera.setViewOffset`. On narrow viewports (`< 1024 px`), the panel moves below the canvas (`42%` height), `clearViewOffset()` centers the frame, and camera distance scales by `1.18x`. All `Inspect` plan shots use pitch `85°` (`z_cam = z_target + 0.85`, `y_cam = 10.2`, `fov = 16°`) so `camera.up` stays `(0, 1, 0)` without gimbal lock while foreshortening stays below `1.5%` and top-left light catches every relief bevel. Coordinates below are station-local `(x, y, z)`.

---

### Scene 1: The program (Station 1, world `X = 0.0`, paper ground)

**Station layout:**
- Rear Key Rack: `x in [-1.8, +1.8], z = -3.65, y = 0.16` holding the three `gold` cards (`leaf`, `node`, `decider`).
- Track A (upper, primary program): source sheet at `(-3.8, 0.04, -1.3)`, compiler channel to ROM table tray (code, type, constant tables) at `(-1.2, 0.06, -1.3)`, Poseidon block `H_prog` at `(+1.2, 0.07, -1.3)`, output socket `prog_d` at `(+2.6, 0.06, -1.3)`.
- Input lane (center): structured input tile group at `(-2.8, 0.05, +0.2)`, `sherlock-value/1` Poseidon block at `(+0.4, 0.07, +0.2)`, output socket `input_d` at `(+2.6, 0.06, +0.2)`.
- Track B (lower, comparison program, revealed in Beat 1.4): second source sheet and ROM tray at `z = +1.7`, hashing to a distinct `prog_d'` at `(+2.6, 0.06, +1.7)`.
- Right Handoff Plate: `(+5.5, 0.02, 0.0)`. Front Token Carrier: `z = +3.65`.

**Shot list:**
1. **Beat 1.1 — Source compiles to ROM image (`Establish`).**
   - *Camera*: `pos (0.0, 6.8, 8.4)` -> `target (0.0, 0.1, -0.2)`, `fov = 25°`.
   - *Objects*: Source sheet (`paperWarm` with ruled `gateViolet` code bars), compiler die, ROM code-table rack, rear 3-key rack, empty front Token Carrier.
   - *Key motion*: From `t = 0.8 s`, `dur = 1.1 s`, the source sheet's ruled bars extrude and slide `+X` into three partitioned `gateViolet`-bordered ROM code tables; unused table tail slots lock canonical blank plugs.
   - *Mechanism*: Moriarty source compiles to the Sherlock machine language: a fixed program image of code tables in ROM.
   - *Still pose*: Source sheet on left linked by a straight arrow rule to the filled three-table ROM rack at `(-1.2, 0.06, -1.3)`.
   - *Labels*: `Moriarty source`, `Program image (ROM)`, `leaf · node · decider`.
2. **Beat 1.2 — Program image hashes to `prog_d` (`Inspect`).**
   - *Camera*: `pos (0.2, 10.2, -0.45)` -> `target (0.2, 0.06, -1.3)`, `fov = 16°`.
   - *Objects*: ROM rack, Poseidon block `H_prog`, `prog_d` socket, Token Carrier socket 1.
   - *Key motion*: `t = 0.6 s`, `dur = 1.0 s` Poseidon hash motion: ROM root and metadata cells feed `H_prog`, minting the `gateViolet`-rimmed `prog_d` token (`5 x 5` pattern A), which duplicates into Token Carrier socket 1 (`t = 1.8 s`, `dur = 0.8 s`).
   - *Mechanism*: `prog_d` is the Poseidon digest of the program image.
   - *Still pose*: ROM rack connected through `H_prog` to `prog_d`, with a twin `prog_d` seated in the front Token Carrier.
   - *Labels*: `Program image (ROM)`, `Poseidon hash`, `prog_d`.
3. **Beat 1.3 — Input value digests to `input_d` (`Inspect`).**
   - *Camera*: `pos (0.0, 10.2, 1.05)` -> `target (0.0, 0.06, 0.2)`, `fov = 16°`.
   - *Objects*: `steel`-tinted input value cells + resource-limit cubes (fuel, stack limit), `sherlock-value/1` Poseidon sponge, `input_d` socket, Token Carrier socket 2.
   - *Key motion*: `t = 0.6 s`, `dur = 1.0 s` Poseidon hash motion digests the input value under `sherlock-value/1` and packs `input_d` (`steel` rim, `5 x 5` pattern B), seating a copy in Token Carrier socket 2 (`t = 1.8 s`, `dur = 0.8 s`).
   - *Mechanism*: `input_d` commits the input value under canonical value digest `sherlock-value/1`.
   - *Still pose*: Input cells at left, `sherlock-value/1` block at center, `input_d` aligned on both the plate and the front Token Carrier beside `prog_d`.
   - *Labels*: `Input value`, `sherlock-value/1`, `input_d`, `prog_d`.
4. **Beat 1.4 — Program is data, not circuit (`Inspect`).**
   - *Camera*: `pos (0.0, 10.8, 0.85)` -> `target (0.0, 0.08, 0.0)`, `fov = 18°` (frames rear Key Rack, Track A, and Track B together).
   - *Objects*: Rear Key Rack (`leaf`, `node`, `decider`), Track A (`prog_d` face A), Track B (`prog_d'` with a visibly different `5 x 5` dot pattern C).
   - *Key motion*: `t = 0.6 s`, `dur = 1.0 s`: Track B hashes its different ROM image into `prog_d'` (face C) while three vertical `gold` datum lines drop from the unchanged `leaf`, `node`, `decider` key cards across both Track A and Track B (`0.4 s` verification sweep across the static rack).
   - *Mechanism*: Changing the program changes `prog_d` data tokens while the three circuit keys stay identical; one key set serves every program.
   - *Still pose*: Split plate with Program A (`prog_d` face A) above and Program B (`prog_d'` face C) below, both spanned by one bracket pointing to the three unchanged `gold` key cards.
   - *Labels*: `Program A: prog_d`, `Program B: new prog_d`, `Same three keys`, `Program is data`.
5. **Transition 1 → 2 (`Establish` then `Follow`).** Track A's ROM rack slides to Handoff Plate `(+5.5, 0.04, 0.0)` beside `prog_d` and `input_d`; camera follows `+11.0` units in `+X` into Station 2 (`dur = 1.6 s`).

---

### Scene 2: The run (Station 2, world `X = 11.0`, paper ground)

**Station layout:**
- Left Machine Plate (`x in [-4.6, -0.8], z in [-2.6, +1.8]`): shallow `ceramicAlabaster` relief chassis split into six labeled compartments on a `2 x 3` grid: `Control` `(-3.9, -1.8)`, `Environment` `(-2.1, -1.8)`, `Frame stack` `(-3.9, -0.4)`, `Task stack` `(-2.1, -0.4)`, `Memory` `(-3.9, +1.0)`, `Fuel counters` `(-2.1, +1.0)`.
- Center `120`-Rule Matrix (`x in [-0.4, +1.8], z in [-2.6, +0.4]`): crisp `12 x 10` instanced grid of `0.15 x 0.18` `ceramicAlabaster` rule tiles (`120` tiles total, zero padding).
- Upper Scale Strip (`x in [-0.4, +4.8], z = -3.1`): `8`-byte `64`-bit step counter collar paired with a broken logarithmic ruler from `0` to `2^64 - 1`.
- Right Witness Tape & Outcome Fork (`x in [+2.0, +5.2], z in [-1.8, +2.2]`): witness stream rail feeding out along `+X`, plus a 3-prong terminal gate (`Halted`, `Rejected`, `Exhausted`).

**Shot list:**
1. **Beat 2.1 — Six-part machine state (`Inspect`).**
   - *Camera*: `pos (-2.7, 9.6, 0.45)` -> `target (-2.7, 0.06, -0.4)`, `fov = 16°`.
   - *Objects*: Handoff ROM rack docked at `-4.8`, the six state bays (`Control`, `Environment`, `Frame stack`, `Task stack`, `Memory`, `Fuel counters`).
   - *Key motion*: `t = 0.5 s`, `dur = 0.9 s`: `prog_d` and `input_d` seed the six bays—`Control` lights `Eval`, `Frame stack` and `Task stack` seat `EMPTY` anchors, `Memory` shows `0` tiles, `Fuel counters` fill their bar.
   - *Mechanism*: `mori simulate` runs `sherlock-k-a/1` over six explicit state parts.
   - *Still pose*: Plan view of the `2 x 3` machine chassis with all six compartments initialized and ruled apart.
   - *Labels*: `Control`, `Environment`, `Frame stack`, `Task stack`, `Memory`, `Fuel counters`.
2. **Beat 2.2 — One of 120 rules fires and writes one witness frame (`Inspect`).**
   - *Camera*: `pos (0.8, 10.2, 0.35)` -> `target (0.8, 0.06, -0.5)`, `fov = 17°`.
   - *Objects*: Machine chassis edge, `12 x 10` matrix of `120` rule tiles, witness tape print head at `x = +2.4`.
   - *Key motion*: `t = 0.5 s`, `dur = 1.1 s`: guard hairlines from `Control` and `Fuel counters` intersect tile `(row 4, col 7)` of the `120`-tile grid, lifting that single tile `+0.04` in `heatBright` (`119` tiles remain flat matte); a step frame stamps onto the witness tape carrying the active rule badge at top, `state before` in the middle, and `state after` at bottom, then advances one pitch (`+0.46`) right.
   - *Mechanism*: Each step fires exactly one of `120` enabled rules and records `(rule id, state before, state after)` on the witness stream.
   - *Still pose*: Tile `(4, 7)` lit in `heatBright` inside the `12 x 10` grid; three completed frames visible on the witness tape with the newest frame's `rule id / before / after` bands aligned to the chassis.
   - *Labels*: `120 enabled rules`, `One rule per step`, `Witness frame`, `before -> after`.
3. **Beat 2.3 — The 64-bit step counter (`Inspect`).**
   - *Camera*: `pos (2.2, 8.8, -2.25)` -> `target (2.2, 0.06, -3.1)`, `fov = 15°`.
   - *Objects*: `8`-byte counter collar (`64` bits) and the `0` to `2^64 - 1` `BreakRule` scale bar.
   - *Key motion*: `t = 0.5 s`, `dur = 0.9 s`: four rapid step ticks advance the low byte cube of the `8`-byte collar while a sliding pointer marks current position near the left origin of the broken `2^64 - 1` ruler; a caliper bracket spans the `8` byte cubes.
   - *Mechanism*: Steps are counted by an `8`-byte (`64`-bit) counter up to `2^64 - 1`; `2^64` is the counter's numerical range, not a circuit size.
   - *Still pose*: `8`-byte counter bar above the zig-zag broken ruler ending at `2^64 - 1`.
   - *Labels*: `64-bit step counter`, `Max 2^64 - 1 steps`, `Counter range only`.
4. **Beat 2.4 — The three terminal outcomes (`Inspect`).**
   - *Camera*: `pos (3.5, 9.8, 1.05)` -> `target (3.5, 0.06, 0.2)`, `fov = 16°`.
   - *Objects*: Three parallel end-of-tape small-multiple tracks stacked in `Z`: Track 1 (`z = -1.1`, `Halted` with `resolve` value cell), Track 2 (`z = +0.2`, `Rejected` with `errorRed`-bordered code tile `10` for arithmetic overflow), Track 3 (`z = +1.5`, `Exhausted` with `opAmber`-bordered kind tile).
   - *Key motion*: `t = 0.5 s`, `dur = 1.0 s`: three terminal frames stamp simultaneously into their respective exit gates (`Halted` lights `resolve`, `Rejected` stamps code `10`, `Exhausted` stamps fuel-kind marker), locking each tape end against further step frames.
   - *Mechanism*: Every run ends in one of three terminal outcomes: `Halted` (value), `Rejected` (code, such as `10` for overflow), or `Exhausted` (kind).
   - *Still pose*: Clean three-row small-multiple diagram comparing the three terminal tape end-caps side by side.
   - *Labels*: `Halted (value)`, `Rejected (code 10)`, `Exhausted (kind)`, `Witness stream`.
5. **Transition 2 → 3 (`Establish` then `Follow`).** The primary `Halted` witness tape slides onto Handoff Plate `(+5.5, 0.04, 0.0)`; camera follows `+X` into Station 3 (`dur = 1.6 s`).

---

### Scene 3: The leaf (Station 3, world `X = 22.0`, paper ground)

**Station layout (3D relief adaptation of `leafChip.js` wide floor plan):**
- Top Slice & Budget Strip (`z = -3.1`): incoming witness tape cut by `16`-step guillotine brackets (`S = 16` slot ribbon, `x in [-4.8, -0.8]`) and `RowBudgetBar` (`2^16` rows, `k <= 16`, `x in [+0.4, +4.8]`). Rear Key Rack highlights only the `leaf` key card.
- West Column (`x in [-4.8, -1.5], z in [-2.3, +2.4]`):
  - `INIT` gate bar at `(-3.15, 0.07, -2.1)` gated by `h_pre = 0`.
  - `CORE` plate (`-3.15, 0.06, -0.4`, `3.1 x 2.6`): persistent register grid (pre/post pairs, unnumbered count), `16`-slot rule lane strip, read port, and sequential append chain (`len_pre -> len_post`).
  - `STACKS` bay (`-3.15, 0.06, +1.5`, `3.1 x 0.9`): `FRAMES` and `TASKS` cached tops + Poseidon hash-chain tracks.
- Central Spine (`x = -0.95, z in [-2.0, +2.2]`): `BUS` dual-lane recessed channel (`2` request grooves, CORE as sole requester port) + tagged fixed-table rail (`DEC` and `RANGE` tags) at `z = -2.1`.
- Middle Provider Column (`x in [-0.3, +2.2]`):
  - `DEC` (`z = -1.9`), `ALU` (`z = -1.0`), `RANGE` (`z = -0.2`, `12`-bit limb tray), `PAGE/ROM` block cache (`z = +0.7`, unnumbered fixed heap-line rack and unnumbered fixed ROM-line rack), `MEM` window (`z = +1.6`, `8`-cell imported/new tail tray), `HASH` & `VALUE DIGEST` (`z = +2.4`, Poseidon whole-sponge bays + `2`-input bus provider).
- East Memory & Boundary Column (`x in [+2.5, +5.0]`):
  - `MEMORY BACKEND` (`+3.5, z = +0.9`): `w_flush = 1` flush unit, scalar `cold_read` walk track, and rising `TOP = 63` broken-axis Merkle tree (`y` up to `2.65`), plus a `2^66`-cell broken address ruler.
  - `BOUNDARY COMMITMENT` (`+3.6, z = -1.5`): body sponge (`registers + stack hashes + (len, root, tail_d) -> body_d`) feeding header sponge (`(outcome_tag, outcome_payload, input_d, deps_d, body_d) -> pre_d / post_d`).
  - Output Bay (`+4.6, z = +2.3`): leaf titanium slab with `7` front pins `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`.

**Shot list:**
1. **Beat 3.1 — Cut into 16-step slices; one uniform leaf circuit (`Establish`).**
   - *Camera*: `pos (0.0, 7.6, 8.8)` -> `target (0.0, 0.2, 0.0)`, `fov = 26°`.
   - *Objects*: Witness tape on left handoff plate, `S = 16` cutter dividers, full leaf floor plan, single lit `leaf` key card on rear rack, `2^16`-row (`k <= 16`) budget bar.
   - *Key motion*: `t = 0.6 s`, `dur = 1.0 s`: `ink` dividers drop every `16` frames along the witness tape; the first `16`-step slice slides into the top slot ribbon while a `gold` line links the whole floor plan to the single `leaf` key card.
   - *Mechanism*: The witness tape is cut into `S = 16` step slices; one uniform circuit with one key (`k <= 16`, `2^16` rows) proves any slice.
   - *Still pose*: Full relief floor plan of all leaf modules under one lit `leaf` key card, with `16` step slots loaded along the top ribbon.
   - *Labels*: `S = 16 steps`, `One leaf key`, `k <= 16 (2^16 rows)`, `CORE`, `BUS`.
2. **Beat 3.2 — `INIT` at `h_pre = 0` (`Inspect`).**
   - *Camera*: `pos (-2.4, 9.2, -1.25)` -> `target (-2.4, 0.07, -2.0)`, `fov = 15°`.
   - *Objects*: `h_pre` 8-byte counter reading `0`, `INIT` gate bar, `prog_d` and `input_d` opening sponges, genesis empty memory token `(len = 0, root, tail_d)`.
   - *Key motion*: `t = 0.5 s`, `dur = 1.0 s`: `h_pre = 0` closes the `INIT` gate switch, opening `prog_d` (fixing `prog_root` for ROM) and `input_d` (`sherlock-value/1` input by digest plus fuel and stack limits) into `CORE`'s first pre-state with empty genesis memory.
   - *Mechanism*: At `h_pre = 0`, `INIT` proves `Init(prog_d, input_d)` with empty genesis memory; on later leaves (`h_pre > 0`), `INIT` stays gated off.
   - *Still pose*: `h_pre = 0` locked to the closed `INIT` bar, connecting opened `prog_d` and `input_d` cells to `CORE` pre-state and empty genesis memory.
   - *Labels*: `h_pre = 0`, `INIT active`, `prog_d -> prog_root`, `Empty genesis memory`.
3. **Beat 3.3 — `DEC` control word and `BUS` tuple round-trip (`Follow` then `Inspect`).**
   - *Camera*: `pos (-1.2, 8.6, -0.55)` -> `target (-1.2, 0.07, -1.3)`, `fov = 15°`.
   - *Objects*: `CORE` request port, tagged fixed table (`DEC` row), `BUS` two-lane channel, `ALU` (`1`), `MEM` (`2`), `HASH` (`3`), `ROM` (`4`) provider ports.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: `CORE` queries `DEC` on the fixed-table rail (`DEC` lights one control-word bar and returns micro-op flags), then launches a `5`-cube bus bar `(1, key, a, b, c)` down lane `1` of `BUS` into `ALU`, where it coincides with `ALU`'s provider row.
   - *Mechanism*: `DEC` decodes the firing rule's control word from a fixed table; `CORE` alone issues `(chip, key, a, b, c)` tuples on the two-lane `BUS` (`1` ALU, `2` MEM, `3` HASH, `4` ROM).
   - *Still pose*: `DEC` control word lit in `heat`, and bus bar `(1, key, a, b, c)` seated in coincidence on `ALU`'s provider socket with chip ids `1..4` visible along `BUS`.
   - *Labels*: `DEC fixed table`, `(chip, key, a, b, c)`, `1 ALU · 2 MEM`, `3 HASH · 4 ROM`, `Two request lanes`.
4. **Beat 3.4 — `ALU` overflow flag and `RANGE` `12`-bit limbs (`Inspect`).**
   - *Camera*: `pos (0.8, 8.4, -0.05)` -> `target (0.8, 0.07, -0.8)`, `fov = 14°`.
   - *Objects*: `ALU` plate with `ADD` / `LT` rows and `flag` bit cube (`0` vs `1`), `RANGE` tray slicing a `128`-bit operand bar into `12`-bit limb cubes checked against the fixed table.
   - *Key motion*: `t = 0.5 s`, `dur = 1.0 s`: operand bar splits into `12`-bit limb cubes that sweep-verify against the `RANGE` fixed table; in a side-by-side `ALU` sub-row, an overflowing `128`-bit `ADD` sets `flag = 1` (`heatBright`, valid provider tuple, not a circuit failure), routing `Rejected(10)` back to `CORE`.
   - *Mechanism*: `RANGE` bounds integers via `12`-bit limb lookups; `ALU` proves both normal ops (`flag = 0`) and exact `128`-bit overflow (`flag = 1` -> `Rejected(10)`).
   - *Still pose*: Decomposed `12`-bit limb cubes locked in `RANGE`, beside `ALU` showing both `flag = 0` and `flag = 1` (`Rejected` code `10`) valid rows.
   - *Labels*: `12-bit range limbs`, `ALU overflow flag = 1`, `Proves Rejected (10)`.
5. **Beat 3.5 — Appends (at most `3`/step), `8`-cell tail, and depth-`63` flush (`Inspect`).**
   - *Camera*: `pos (2.1, 4.2, 6.4)` -> `target (2.1, 1.1, 1.1)`, `fov = 20°` (slight frontal elevation to show the `TOP = 63` vertical tree ruler and `RowBudgetBar`).
   - *Objects*: `CORE` append chain, `MEM` `8`-cell tail tray (5 imported tiles + 3 new tiles), `w_flush = 1` flush walk, `TOP = 63` broken Merkle column, `3 x 2^64 < 2^66` broken scale ruler, `RowBudgetBar` (`2^16`).
   - *Key motion*: `t = 0.4 s`, `dur = 1.2 s`: one step appends `3` `heat`-rimmed tiles into slots `5, 6, 7`, filling the `8`-cell tail; the `8` tiles hash (`Tree8`) into one segment token that walks up the `TOP = 63` column (`w_flush = 1`) from `EMPTY_SEG` to update `root`, while an `8,821`-row block fills inside the `2^16` row bar.
   - *Mechanism*: Steps append at most `3` cells (`3 x 2^64 < 2^66`, so `2^66` memory never runs out before the `64`-bit step counter); when the `8`-cell tail fills, `w_flush = 1` hashes the segment into the depth-`63` tree (`8,821` rows).
   - *Still pose*: Filled `8`-cell segment compressed into the `TOP = 63` tree root, `3 x 2^64 < 2^66` ruler aligned below, and `8,821 rows` marked on the `2^16` bar.
   - *Labels*: `At most 3 appends/step`, `8-cell tail -> segment`, `TOP = 63 (2^66 cells)`, `w_flush = 1: 8,821 rows`.
6. **Beat 3.6 — Old reads (scalar cold read & 8-record cache line) and ROM lines (`Inspect`).**
   - *Camera*: `pos (1.6, 9.6, 1.65)` -> `target (1.6, 0.15, 0.8)`, `fov = 16°`.
   - *Objects*: Pre-state `pre.root` socket, scalar `cold_read` Merkle path, `PAGE/ROM` unnumbered fixed heap-line rack (`8` tiles per line), `PAGE/ROM` unnumbered fixed ROM-line rack (`8` rows per line opened against `prog_root`), `BUS`, `RowBudgetBar`.
   - *Key motion*: `t = 0.5 s`, `dur = 1.1 s`: (1) an isolated old read opens a Merkle path directly against `pre.root` (`4,761` rows added on the `2^16` bar); (2) an `8`-record heap cache line opens against `pre.root` and serves `(2, ...)` tuples on `BUS`; (3) an `8`-row ROM line opens against `prog_root` (`gateViolet`) and serves `(4, ...)` tuples on `BUS`.
   - *Mechanism*: Old heap reads authenticate against `pre.root` either as a scalar cold read (`4,761` rows) or as an `8`-record cache line on `BUS`; program reads open `8`-row ROM lines against `prog_root` on `BUS`.
   - *Still pose*: Both openings anchored strictly to `pre.root` (never the post-flush root) and `prog_root`, with `(2, ...)` and `(4, ...)` provider rows seated on `BUS`.
   - *Labels*: `Cold read: 4,761 rows`, `8-record heap line`, `8-row ROM line`, `Opened at pre-state root`.
7. **Beat 3.7 — `FRAMES` and `TASKS` hash-chain stacks (`Inspect`).**
   - *Camera*: `pos (-3.1, 8.2, 2.25)` -> `target (-3.1, 0.08, 1.5)`, `fov = 14°`.
   - *Objects*: `FRAMES` track and `TASKS` track, each with cached-top register tray, Poseidon sponge, and linked `gold` chain tokens.
   - *Key motion*: `t = 0.5 s`, `dur = 1.0 s`: on `FRAMES`, a `push` slides the old top through the Poseidon block to mint a new `gold` chain link while seating the new top; on `TASKS`, a `pop` opens the head `gold` link through the sponge to restore the prior top and predecessor link.
   - *Mechanism*: `FRAMES` and `TASKS` are Poseidon hash chains outside random-access memory: push hashes the old top into the chain; pop opens it.
   - *Still pose*: `FRAMES` showing `+1` chain link after push, `TASKS` showing opened link and restored top after pop.
   - *Labels*: `FRAMES hash chain`, `TASKS hash chain`, `Push hashes top`, `Pop opens link`.
8. **Beat 3.8 — Header-and-body boundary commitment and the seven public lanes (`Inspect`).**
   - *Camera*: `pos (3.6, 9.4, 0.85)` -> `target (3.6, 0.08, 0.0)`, `fov = 16°`.
   - *Objects*: `BOUNDARY COMMITMENT` dual-sponge plate (`body_d` sponge + `header` sponge), `rules_d` 16-slot sponge, leaf titanium slab with `7` pins.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: registers, stack hashes, and `(len, root, tail_d)` hash into `body_d`; then `(outcome_tag, outcome_payload, input_d, deps_d, body_d)` hash into `pre_d` and `post_d`, which lock onto pins `3` and `4` of the `7`-pin leaf slab `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`.
   - *Mechanism*: Boundary state commits in header-and-body Poseidon form (`pre_d`, `post_d`), and the leaf publishes exactly seven public lanes.
   - *Still pose*: Two-stage diagram (`body -> body_d -> header -> post_d`) wired directly to the `7` labeled pins along the leaf slab's front edge.
   - *Labels*: `Body -> body_d`, `Header -> pre_d, post_d`, `7 public leaf lanes`, `rules_d · h_pre · steps`.
9. **Beat 3.9 — Partition independence (`Inspect`).**
   - *Camera*: `pos (0.5, 10.6, 0.85)` -> `target (0.5, 0.08, 0.0)`, `fov = 18°`.
   - *Objects*: Split comparison plate: Track A (`z = -1.3`) cuts a `20`-step tape at `16 + 4`; Track B (`z = +1.3`) cuts the same `20`-step tape at `9 + 11` (early close); shared right coincidence socket at `(+4.0, 0.08, 0.0)`.
   - *Key motion*: `t = 0.5 s`, `dur = 0.9 s` Equality motion: Track A's final `(len, root, tail_d)` and `post_d` token and Track B's final `(len, root, tail_d)` and `post_d` token slide along registration lines into physical coincidence at `(+4.0, 0.08, 0.0)`, flashing `goldBright`.
   - *Mechanism*: Any valid partition of one run into leaves produces the exact same final memory state and `post_d`, so the claim never depends on where leaves cut.
   - *Still pose*: Two differently sliced tapes above and below `z = 0` converging onto one identical `5 x 5` `post_d` face at right.
   - *Labels*: `Cut A: 16 + 4`, `Cut B: 9 + 11`, `Same memory state`, `Partition independence`.
10. **Transition 3 → 4 (`Establish` then `Follow`).** Paired leaf slabs with their `7` front pins rest on Handoff Plate `(+5.5, 0.06, 0.0)`; camera follows `+X` into Station 4 (`dur = 1.6 s`).

---

### Scene 4: The node (Station 4, world `X = 33.0`, paper ground)

**Station layout:**
- Rear Key Rack highlights the `node` key card (`k = 20`, `2^20` rows).
- Left Merge Cradle (`x in [-4.6, -0.8], z in [-1.8, +1.8]`): Left child slot `L` and Right child slot `R` with three central equality tracks between their pins.
- Center Output & Caliper Bay (`x in [-0.4, +2.0], z = 0.0`): Parent node slab (`1.60 x 1.05`, `5` pins `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`, single `gold` accumulator ring) straddled by `FixedCaliper` and paired with `RowBudgetBar` (`2^20` rows) and the `2^64` height-counter scale.
- Right Parallel / Shape / `JOIN` Plate (`x in [+2.2, +5.0], z in [-2.6, +2.4]`): linear work plot, `3`-prover parallel bay (`1.73x` bar, `65 s · 20 threads · 17 GB` host gauge), DAG/tree wiring diagram, and `JOIN` hash block (`set_root = H(SHK_SET_JOIN, l, r)`).

**Shot list:**
1. **Beat 4.1 — `MERGE` and its three equalities (`Inspect`).**
   - *Camera*: `pos (-2.4, 9.6, 0.80)` -> `target (-2.4, 0.08, 0.0)`, `fov = 16°`.
   - *Objects*: Child slabs `L` and `R` in the merge cradle, `node` key card, three coincidence sockets (`prog_d`, `L.post_d == R.pre_d`, `L.h_hi == R.h_lo`).
   - *Key motion*: `t = 0.5 s`, `dur = 0.9 s`: verification sweeps run across `L` and `R`, then the three pin pairs slide into physical coincidence simultaneously (`prog_d` violet tokens merge, `L.post_d` and `R.pre_d` gold tokens merge, `L.h_hi` and `R.h_lo` 8-byte bars lock end-to-end).
   - *Mechanism*: `MERGE` verifies two children and enforces equal `prog_d`, `L.post_d == R.pre_d`, and `L.h_hi == R.h_lo`.
   - *Still pose*: `L` and `R` joined by three locked `gold` equality clasps, exporting the outer span `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`.
   - *Labels*: `Same prog_d`, `L.post_d == R.pre_d`, `L.h_hi == R.h_lo`, `k = 20 node`.
2. **Beat 4.2 — Constant slab size and the single accumulator (`Inspect`).**
   - *Camera*: `pos (0.8, 8.8, 0.75)` -> `target (0.8, 0.10, 0.0)`, `fov = 15°`.
   - *Objects*: Output parent slab inside `FixedCaliper` (`1.60` span), child accumulator rings folding into the parent's one `gold` ring, `8`-byte height bar against the `2^64` broken ruler, `2^20` row bar.
   - *Key motion*: `t = 0.5 s`, `dur = 0.9 s`: both children's `gold` rings slide into the parent slab's single ring recess, adding two radial `ink` tick marks without growing the ring or slab; `FixedCaliper` jaws clamp `1.60` width while the `8`-byte `R.h_hi` counter ticks toward `2^64`.
   - *Mechanism*: Deferred pairing checks fold into one accumulator, so a node slab has the exact same size at any height below `2^64`; `2^64` is counter range, while circuit rows stay at `k = 20` (`2^20`).
   - *Still pose*: Parent slab locked inside `FixedCaliper` with one multi-tick `gold` ring, `5` front pins, `2^20` circuit bar, and `h < 2^64` counter ruler.
   - *Labels*: `Same slab size`, `One accumulator ring`, `Heights < 2^64`, `(prog_d, pre_d, post_d, h_lo, h_hi)`.
3. **Beat 4.3 — Linear work and parallel provers (`Inspect`).**
   - *Camera*: `pos (3.5, 9.8, -0.45)` -> `target (3.5, 0.08, -1.3)`, `fov = 16°`.
   - *Objects*: Straight-line step-vs-work ruled graph (`N` steps -> proportional leaf+node count), host spec bar (`65 s · 20 threads · 17 GB`), and three parallel node cradles firing simultaneously (`1.73x` throughput bar).
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s` (authorized parallel-work exception): three adjacent node cradles sweep-verify and emit slabs in parallel while the throughput bar extends from `1.00x` to `1.73x` alongside the linear work diagonal.
   - *Mechanism*: Total proving work grows linearly with step count; one node takes about `65 s` (`20` threads, `17 GB` RAM), and `3` parallel provers yield `1.73x` throughput.
   - *Still pose*: Three completed node slabs in parallel lanes beside the straight `1:1` linear-work line and `1.73x` bar.
   - *Labels*: `Linear work in steps`, `1 node: 65 s, 17 GB`, `20 threads`, `3 provers: 1.73x`.
4. **Beat 4.4 — Trees, DAGs, and `JOIN` into a set (`Inspect`).**
   - *Camera*: `pos (3.5, 9.8, 1.85)` -> `target (3.5, 0.08, 1.0)`, `fov = 16°`.
   - *Objects*: Relief topology plate showing segment `MERGE` (left-to-right tree) alongside whole-run `JOIN` (`Poseidon` block combining two sealed set tokens `l` and `r` into `set_root = H(SHK_SET_JOIN, l, r)` in a shared-node DAG).
   - *Key motion*: `t = 0.5 s`, `dur = 1.0 s`: two sealed run tokens `l` and `r` feed the `SHK_SET_JOIN` Poseidon block to mint `set_root`, while a shared DAG ancestor feeds two branches without duplicating its proof slab.
   - *Mechanism*: Segments merge left to right (`MERGE`), and whole runs join into sets (`set_root = H(SHK_SET_JOIN, l, r)`) in any tree or DAG.
   - *Still pose*: Left-to-right `MERGE` tree paired with a `JOIN` DAG converging on `set_root`.
   - *Labels*: `MERGE segments`, `JOIN whole runs`, `H(SHK_SET_JOIN, l, r)`, `Any tree or DAG`.
5. **Transition 4 → 5 (`Establish` then `Follow`).** The root segment slab with its multi-tick `gold` accumulator ring rests on Handoff Plate `(+5.5, 0.06, 0.0)`; camera follows `+X` into Station 5 (`dur = 1.6 s`).

---

### Scene 5: The decider (Station 5, world `X = 44.0`, paper ground)

**Station layout:**
- Rear Key Rack highlights the `decider` key card (`2^20` rows, `RowBudgetBar` at `z = -3.1`).
- Left Root & Genesis Check (`x in [-4.5, -1.8], z in [-1.5, +1.0]`): root slab socket, `h_lo == 0` zero-lock notch, and accumulator pairing clamp.
- Center Header Opening & Shutter Bay (`x in [-1.5, +1.8], z in [-2.2, +1.8]`): `post_d` header-opening sponge revealing `5` cells (`outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`), `Running` refusal gate vs terminal pass gate, `sherlock-value/1` `outcome_d` sponge, `claim_d` sponge, `8`-byte `n_steps` decomposition bracket, and the sliding `titanium` **Privacy Shutter**.
- Right Public Seal & Value-Equality Split (`x in [+2.0, +5.0]`): `4`-pin decider seal `[prog_d, input_d, outcome_d, n_steps]`, plus Track A / Track B value-hiding comparison bay (`z = ±1.4`).

**Shot list:**
1. **Beat 5.1 — `h_lo == 0` and opening the `post_d` header; `Running` refused (`Inspect`).**
   - *Camera*: `pos (-0.8, 9.8, 0.55)` -> `target (-0.8, 0.08, -0.3)`, `fov = 16°`.
   - *Objects*: Root slab verified under pinned keys, `h_lo` 8-byte bar locked into `== 0` notch, `post_d` opening into `5` header cells (`body_d` stays a closed `gold` token), side-by-side `Running` tag vs `Halted` tag gates.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: `h_lo == 0` flashes `goldBright`; `post_d` opens its header to the `5` cells; in the upper test slot a `Running` tag recoils `-0.12` with an `errorRed` hatch (refusal), while in the main slot `Halted` sweeps `goldBright` and passes.
   - *Mechanism*: `SEAL` checks `h_lo == 0` (proving genesis `INIT` ran), opens the `5`-word header of `post_d` without opening `body_d`, and refuses a `Running` state.
   - *Still pose*: `h_lo == 0` locked, `5` header cells exposed from `post_d`, `Running` hatched in `errorRed` beside passing `Halted`.
   - *Labels*: `h_lo == 0`, `Open post_d header`, `Running refused`, `body_d stays closed`.
2. **Beat 5.2 — `outcome_d`, `claim_d`, and one pairing closes the accumulator (`Inspect`).**
   - *Camera*: `pos (0.4, 9.4, 0.95)` -> `target (0.4, 0.08, 0.1)`, `fov = 16°`.
   - *Objects*: Terminal kind + payload feeding `sherlock-value/1` -> `outcome_d` (`resolve` teal rim), `Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)` -> `claim_d` (`gold` rim), accumulator ring in its pairing clamp.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: `outcome_d` mints, then joins `SHK_RUN`, `1`, `prog_d`, `input_d` to mint `claim_d`, while the two semicircular pairing jaws clamp shut on the multi-tick `gold` accumulator ring (`0.4 s` `goldBright` seal).
   - *Mechanism*: Decider computes `outcome_d` under `sherlock-value/1`, computes `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`, and closes all accumulated checks in one pairing.
   - *Still pose*: Closed solid `gold` pairing disc at left, minted `outcome_d` (teal) and `claim_d` (gold) tokens at right.
   - *Labels*: `outcome_d (sherlock-value/1)`, `claim_d`, `One pairing check`, `Decider <= 2^20 rows`.
3. **Beat 5.3 — Four values leave; privacy shutter hides everything else (`Inspect`).**
   - *Camera*: `pos (1.2, 9.6, 1.35)` -> `target (1.2, 0.10, 0.5)`, `fov = 16°`.
   - *Objects*: `n_steps` 8-byte collar checked in its `8`-slot decomposition bracket, internal witness bay (`post_d`, `body_d`, `root`, `tail_d`, `len`, addresses), `titanium` Privacy Shutter, `4`-pin decider seal, front Token Carrier.
   - *Key motion*: `t = 0.5 s`, `dur = 0.9 s`: `n_steps` passes its `8`-byte check; the `titanium` Privacy Shutter slides `+Z` (`0.7 s`) completely covering `post_d`, `body_d`, `root`, `tail_d`, `len`, and addresses, leaving only `[prog_d, input_d, outcome_d, n_steps]` protruding on the decider seal—and `outcome_d` + `n_steps` seat into the front Token Carrier alongside `prog_d` and `input_d`.
   - *Mechanism*: `n_steps` is range-checked by `8`-byte decomposition; only `[prog_d, input_d, outcome_d, n_steps]` are public; no state digest, root, tail, length, or address leaves.
   - *Still pose*: Hatched `titanium` shutter locked over all internal state, with only the four color-coded public tokens in front and mirrored in the Token Carrier.
   - *Labels*: `prog_d · input_d`, `outcome_d · n_steps`, `8-byte range check`, `State & addresses hidden`.
4. **Beat 5.4 — Two runs with the same value publish the same `outcome_d` (`Inspect`).**
   - *Camera*: `pos (3.3, 9.6, 0.85)` -> `target (3.3, 0.08, 0.0)`, `fov = 16°`.
   - *Objects*: Split plate: Run A (`z = -1.3`, scattered heap tiles at high addresses) and Run B (`z = +1.3`, shared/compact heap tiles at low addresses), each behind a half-shutter, feeding `sherlock-value/1` into a central coincidence socket.
   - *Key motion*: `t = 0.5 s`, `dur = 0.8 s` Equality motion: Run A's `outcome_d` and Run B's `outcome_d` slide into physical coincidence, matching `5 x 5` dot for dot.
   - *Mechanism*: `outcome_d` hides memory addresses and sharing: two runs that compute the same value publish the exact same `outcome_d`.
   - *Still pose*: Different memory layouts behind shutters on left, one merged `resolve` `outcome_d` face on right.
   - *Labels*: `Different memory addresses`, `Shared vs unfolded`, `Identical outcome_d`.
5. **Transition 5 → 6 (`Establish` then `Follow`).** The `1.60 x 1.05` decider seal and the four tokens rest on Handoff Plate `(+5.5, 0.06, 0.0)`; camera follows `+X` toward the paper-to-slate border (`dur = 1.6 s`).

---

### Scene 6: The transaction (Station 6, world `X = 55.0`, paper-to-slate crossing)

**Station layout:**
- `x in [-5.5, 0.0]`: `paper` (`0xf4f0e8`) ground; `x = 0.0`: `0.04` `titanium` threshold rule; `x in [0.0, +5.5]`: `graphite` (`0x1c2331`) chain ground. Scene background and `FogExp2` color interpolate deterministically in `seek(t)` from `0xf4f0e8` to `0x1c2331` during Beat 6.3.
- Assembly Cradle (`x = -2.2, z = 0.0`): `graphiteLight` transaction envelope (`2.30 x 1.48`) with `6.4 KB` caliper scale next to the decider seal pocket, four transcript-bound value sockets, `SherlockRun.acceptRun` call header, and `DUST` fee recess.

**Shot list:**
1. **Beat 6.1 — Packing the envelope and `DUST` fee (`Inspect`).**
   - *Camera*: `pos (-2.2, 9.4, 0.85)` -> `target (-2.2, 0.08, 0.0)`, `fov = 16°`.
   - *Objects*: Decider seal (`~6.4 KB` caliper bar), `[prog_d, input_d, outcome_d, n_steps]`, `DUST` fee tile, open `graphiteLight` envelope addressed to `SherlockRun.acceptRun`.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: the decider seal, the four public tokens, and the `DUST` fee tile drop into their machined pockets in the envelope; `gold` call-transcript traces light, linking all four sockets to the seal's binding edge.
   - *Mechanism*: The Midnight transaction wraps the `~6.4 KB` decider proof, the four public values, the `SherlockRun.acceptRun` call, and `DUST` fees.
   - *Still pose*: Packed slate envelope on the paper bench edge with `gold` transcript traces locking the four tokens to the `6.4 KB` seal.
   - *Labels*: `SherlockRun.acceptRun`, `Proof ~6.4 KB`, `4 bound values`, `DUST fee`.
2. **Beat 6.2 — Call transcript binding refuses a tampered value (`Inspect`).**
   - *Camera*: `pos (-2.2, 8.6, 0.80)` -> `target (-2.2, 0.08, 0.0)`, `fov = 15°`.
   - *Objects*: Envelope with `gold` transcript circuit linking `[prog_d, input_d, outcome_d, n_steps]` to the seal.
   - *Key motion*: `t = 0.4 s`, `dur = 1.2 s`: an alternate `outcome_d'` token (`5 x 5` face mismatch) swaps into socket `3` (`0.4 s`), breaking the `gold` transcript trace; a verification sweep halts mid-seal in `Refusal` (`0.45 s`, recoil + `errorRed` hatch across envelope); at `t = 1.7 s` (`0.35 s`), original `outcome_d` restores the `gold` trace.
   - *Mechanism*: The call transcript binds all four public values into the proof; changing any value invalidates verification.
   - *Still pose*: Split inset at top-right freezes the `errorRed`-hatched tampered state beside the restored `gold`-linked valid envelope.
   - *Labels*: `Transcript binding`, `Changed value refused`.
3. **Beat 6.3 — Crossing from paper to slate (`Follow` then `Establish`).**
   - *Camera*: `pos (-2.0 -> +2.2, 6.2, 7.6)` -> `target (-2.0 -> +2.2, 0.1, 0.0)`, `fov = 24°`.
   - *Objects*: Closed envelope, `x = 0.0` `titanium` threshold bar between `paper` (`0xf4f0e8`) and `graphite` (`0x1c2331`), front Token Carrier.
   - *Key motion*: `t = 0.4 s`, `dur = 1.4 s`: the envelope translates across `x = 0.0` from prover paper onto chain slate while `scene.background` and `scene.fog.color` lerp smoothly (`easeInOutCubic`) from `0xf4f0e8` to `0x1c2331`, ending on Handoff Plate `(+5.5, 0.06, 0.0)`.
   - *Mechanism*: Work leaves the prover's host and enters on-chain verification on Midnight.
   - *Still pose*: Envelope seated on dark `graphite` slate right of the visible `paper | slate` threshold line, with the four tokens bright in the front carrier.
   - *Labels*: `Prover (paper)`, `Chain (slate)`, `Transaction submitted`.
4. **Transition 6 → 7 (`Follow`).** Camera tracks the envelope from `(+5.5, 0.06, 0.0)` into Station 7 (`dur = 1.6 s`).

---

### Scene 7: The contract (Station 7, world `X = 66.0`, slate ground)

**Station layout (`graphite` `0x1c2331` plate, `wellSlate` `0x161d29` wells):**
- Left Verifier & Hash Cradle (`x in [-4.6, -1.4], z in [-1.4, +1.4]`): single contract verifier key card (`gold`), envelope unpacking bay, and `claim_d` recompute Poseidon block.
- Center `runs` Cabinet (`x in [-0.8, +2.0], z = -1.2`): `16`-drawer shallow relief index cabinet (`claim_d` face tab on each drawer; inside: `outcome_d` + `n_steps` slots, plus crossed-out empty state-digest box).
- Right Hooks Plate (`x in [+2.3, +5.0], z in [-2.4, +2.2]`): upper bay for later-run consumption at seal (`SEAL-WITH-DEPS`, `deps_d`); lower bay for foreign proof entry (`reg_root` registry opening into a node's `FOREIGN` child slot).

**Shot list:**
1. **Beat 7.1 — Verify proof, recompute `claim_d`, lookup `runs`, and store (`Inspect`).**
   - *Camera*: `pos (-0.8, 9.6, 0.65)` -> `target (-0.8, 0.12, -0.2)`, `fov = 16°`.
   - *Objects*: Unpacked seal + `4` values, single contract verifier key, `claim_d` Poseidon block, `runs` cabinet with drawer `7` empty.
   - *Key motion*: `t = 0.3 s`, `dur = 1.3 s`: `0.4 s` verification sweep passes the seal against the single verifier key; `[prog_d, input_d, outcome_d]` hash into `claim_d`; an index cursor scans `runs` (no match), slides drawer `7` out `+0.35` in `+Z`, seats `claim_d` on the tab and `(outcome_d, n_steps)` inside, leaving the crossed-out state-digest square empty.
   - *Mechanism*: `acceptRun` verifies the proof, recomputes `claim_d`, checks `runs` for absence, and stores only `outcome_d` and `n_steps` under `claim_d` (no state digest).
   - *Still pose*: Verified seal at left, `claim_d` tab on open drawer `7` holding `outcome_d` and `n_steps` beside the empty crossed-out state box.
   - *Labels*: `Verify seal`, `Recompute claim_d`, `runs[claim_d]`, `No state digest stored`.
2. **Beat 7.2 — Replay refused, even under another leaf partition (`Inspect`).**
   - *Camera*: `pos (-0.4, 9.2, 0.85)` -> `target (-0.4, 0.12, 0.0)`, `fov = 16°`.
   - *Objects*: Occupied drawer `7` (`claim_d` face D), and a second incoming envelope labeled from Cut B (`9 + 11` slice badge on its proof, same `[prog_d, input_d, outcome_d]`).
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: Cut B's envelope hashes `[prog_d, input_d, outcome_d]` to the exact same `claim_d` face D, slides to drawer `7`, collides with the existing `claim_d` tab, and halts in `Refusal` (`-0.12` recoil + `errorRed` hatch).
   - *Mechanism*: `runs` refuses any duplicate `claim_d`, including the same run proved again under a different leaf partition.
   - *Still pose*: Occupied drawer `7` untouched in `gold`; Cut B's replay `claim_d` recoiled and hatched in `errorRed` directly in front of it.
   - *Labels*: `Same run, another cut`, `Same claim_d`, `Replay refused`.
3. **Beat 7.3 — Later run consumes claim at seal; foreign proof enters via `reg_root` (`Inspect`).**
   - *Camera*: `pos (3.5, 9.8, 0.75)` -> `target (3.5, 0.10, -0.1)`, `fov = 16°`.
   - *Objects*: Upper bay: Run 2 seal plate (`SEAL-WITH-DEPS`) linked to drawer `7`'s `claim_d` via `deps_d` chain token; Lower bay: `reg_root` registry Merkle bar authenticating a foreign verifier key + adapter into the `FOREIGN` child slot of a Sherlock node slab.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: above, a copy of `claim_d` slides from `runs` into Run 2's `deps_d` header check at its seal; below, a foreign proof slab sweeps against an entry opened under `reg_root` and locks into a node's `FOREIGN` child slot.
   - *Mechanism*: A later run consumes an earlier claim at its seal (`SEAL-WITH-DEPS`, `deps_d`), and a foreign proof enters through the verifier-key registry (`reg_root`) as a `FOREIGN` child of a node.
   - *Still pose*: Two clean horizontal sub-diagrams on the right plate: `runs -> deps_d -> SEAL-WITH-DEPS` above, and `reg_root -> FOREIGN child -> node` below.
   - *Labels*: `Consume at seal (deps_d)`, `Registry root: reg_root`, `FOREIGN node child`.
4. **Transition 7 → 8 (`Establish` then `Follow`).** Drawer `claim_d` holding `(outcome_d, n_steps)` clones onto Handoff Plate `(+5.5, 0.06, 0.0)`; camera follows `+X` into Station 8 (`dur = 1.6 s`).

---

### Scene 8: The network (Station 8, world `X = 77.0`, slate ground)

**Station layout (`graphite` `0x1c2331` plate):**
- Left Validator Grid (`x in [-4.8, -1.2], z in [-2.2, +2.0]`): `2 x 3` small-multiple array of six Midnight ledger `9.1.0.1` node plates, each carrying an identical `gold` verifier key card and checking claims with distinct `prog_d` `5 x 5` dot faces.
- Center On-Chain Fact & Consumer Fan-out (`x in [-0.8, +2.0], z = 0.0`): settled `claim_d` plinth holding `[prog_d, input_d, outcome_d, n_steps]` with rigid `gold` read traces fanning out to a downstream contract plate and a later proof's seal plate.
- Right Collection Settlement Plaque (`x in [+2.4, +5.0], z = 0.0`): DAG of joined claims settling into one `3`-pin titanium plaque `[reg_root, set_root, shape_d]` (explicitly three pins, no counter).

**Shot list:**
1. **Beat 8.1 — Midnight nodes verify every program with one key (`Inspect`).**
   - *Camera*: `pos (-3.0, 9.6, 0.75)` -> `target (-3.0, 0.10, -0.1)`, `fov = 16°`.
   - *Objects*: Six ledger `9.1.0.1` node plates, each holding the identical `gold` key card and verifying transactions with distinct `prog_d` dot faces.
   - *Key motion*: `t = 0.4 s`, `dur = 0.8 s` (parallel-work exception): simultaneous `0.4 s` `goldBright` verification sweeps cross all six Midnight nodes against their identical `gold` key cards.
   - *Mechanism*: Midnight nodes on ledger `9.1.0.1` verify every transaction with the exact same verifier key for every program.
   - *Still pose*: `2 x 3` small-multiple grid of slate validator plates lit with `gold` borders, identical key cards aligned along top hairlines over varied `prog_d` faces.
   - *Labels*: `Midnight ledger 9.1.0.1`, `Same verifier key`, `Every program`.
2. **Beat 8.2 — The claim as an on-chain fact; collection statement `[reg_root, set_root, shape_d]` (`Inspect`).**
   - *Camera*: `pos (1.8, 9.8, 0.85)` -> `target (1.8, 0.10, 0.0)`, `fov = 17°`.
   - *Objects*: Settled `claim_d` plinth (`prog_d`, `input_d`, `outcome_d`, `n_steps`) wired to a contract consumer and a later proof consumer; beside it, a multi-run DAG settling into the `3`-pin collection slab `[reg_root, set_root, shape_d]`.
   - *Key motion*: `t = 0.4 s`, `dur = 1.1 s`: the four tokens rise from the front Token Carrier into the settled `claim_d` plinth (`gold` read lines light to both consumers) while a `3`-run DAG folds into the `[reg_root, set_root, shape_d]` plaque.
   - *Mechanism*: Any contract or later proof relies on the settled fact `(prog_d, input_d, outcome_d, n_steps)`; collections of runs settle as `[reg_root, set_root, shape_d]` with no claim count.
   - *Still pose*: Settled `4`-token run fact feeding two consumers on left; `3`-pin `[reg_root, set_root, shape_d]` collection plaque on right.
   - *Labels*: `Accepted claim fact`, `Contracts & proofs read`, `[reg_root, set_root, shape_d]`.
3. **Beat 8.3 — The whole proof line (`Establish`, final wide bench shot).**
   - *Camera*: `pos (38.5, 22.0, 44.0)` -> `target (38.5, 0.0, 0.0)`, `fov = 24°` (reduces `scene.fog.density` deterministically from `0.016` to `0.002` so all `88` world units read cleanly without haze).
   - *Objects*: All eight stations (`X = 0` to `X = 77`) in one straight left-to-right drafting bench—five light `paper` stations on the left, the split `paper | slate` crossing at Station 6, two dark `graphite` stations on the right, rear Key Rack unchanged, and `[prog_d, input_d, outcome_d, n_steps]` seated in the accepted claim at Station 8.
   - *Key motion*: `t = 0.0 s`, `dur = 2.4 s`: smooth camera pull-back from Station 8 to the full-bench vantage; then a `0.9 s` datum pulse traces left-to-right from Station 1 source sheet to Station 8 settled claim (`t = 2.5 s` to `3.4 s`), holding completely still to the end.
   - *Mechanism*: Shows the complete invariants of the pipeline in one diagram: source to claim, `2^64` tape compressed to a constant `6.4 KB` seal, prover paper to chain slate, four tokens preserved end-to-end.
   - *Still pose*: Textbook elevation-perspective panorama of all `8` stations with the four color-coded tokens glowing gently in Station 8.
   - *Labels*: `1–5 Prover (paper)`, `6 Transaction (~6.4 KB)`, `7–8 Midnight (slate)`, `[prog_d, input_d, outcome_d, n_steps]`.

---

## 5. Scale devices

Large bounds never spawn object clouds. Six reusable drafting instruments make every scale invariant readable in five seconds:

1. **`16` steps against `2^64` (Scenes 2, 3, 4).** A horizontal `BreakRule` (`4.4` units long) shows `16` literal tick marks (`0.06` pitch = `0.96` units) at its left origin inside a caliper bracket (`S = 16`), followed by logarithmic decade ticks (`2^8`, `2^16`, `2^32`), a crisp `—╱╲—` double zig-zag break cut, and the terminal stop at `2^64 - 1` paired with the `8`-byte counter collar. The viewer sees both the exact `16`-step slice and the `64`-bit counter ceiling on one continuous datum line.
2. **`8` cells against `2^66` (Scene 3).** Directly beneath the `8`-tile `MEM` tray (`1.96` units wide), an address `BreakRule` aligns tick `0..7` (`1` segment = `8` cells) on the left, cuts through a `—╱╲—` break mark, and marks `3 x 2^64` (the maximum cells appended if every one of `2^64 - 1` steps appended `3` cells) clearly to the left of the hard stop `2^66` (`2^63` segments x `8` cells). That single geometric gap proves visually why memory never overflows before the step counter.
3. **Depth `63` (`TOP = 63`, Scene 3).** Drawn as a `2.80`-unit vertical elevation column behind the `8`-cell tray: levels `0..2` (`Tree8` over `8` tiles) at the base, level `1` and `2` of the segment tree, a horizontal `—╱╲—` break collar across the shaft, and levels `61`, `62`, `63` (`root`) at the top beside an `ink` vertical scale bar. A flush (`w_flush = 1`) animates a pulse through the bottom levels, jumps the break mark in `0.15 s`, and emerges into levels `61..63`.
4. **`120` rules (Scene 2).** Shown literally once as a flat `12 x 10` instanced tile matrix (`1.80 x 1.80` units, `1` `InstancedMesh` of `120` rounded rectangles). Because `120` is small and exact, lighting `1` `heatBright` tile against `119` still `ceramicAlabaster` tiles makes `1-of-120` dispatch immediate without counting.
5. **"Same size at any height" (Scenes 3, 4, 5, 6).** Every leaf slab, height-`32` node slab, height-`2^40` node slab, and decider seal uses the exact `1.60 x 1.05` `BoxGeometry`, framed by the rigid `1.60`-span `FixedCaliper` and paired with `RowBudgetBar` (`2^20` rows) and the `~6.4 KB` envelope caliper. Only the `8`-byte height counter bar and radial tick marks on the single `gold` accumulator ring advance.
6. **"Linear work" (Scene 4).** Drawn as a flat Cartesian rule plate (`2.0 x 1.4` units): horizontal axis `steps`, vertical axis `node proofs`, with `4`, `8`, and `12` small-multiple mini-slab ticks falling strictly on a ruled `45°` `ink` diagonal, paired with the `3`-prover parallel bar (`1.73x`).

---

## 6. Build notes

- **Renderer and dual-backend materials (`three/webgpu` r175 + `?webgl`).** Every scene builds with `MeshStandardNodeMaterial` and `MeshBasicNodeMaterial` imported from `three/webgpu`, reusing cached `createCeramicGrainTexture()`, `createBrushedMetalTexture()`, and `createStudioEnvironmentMap()` from `visualization/src/designers/theme.js`. Two custom surface states—the `goldBright` verification scan band and the `45°` `errorRed` refusal hatch—are written as `three/tsl` fragment color mix nodes driven by `uniform(float)` values (`uScanX`, `uRefuseMix`) updated inside `seek(t)`. TSL compiles cleanly to WGSL on WebGPU and GLSL 300 es under `?webgl` (`forceWebGL: true`).
  - *Fallback if a TSL node hits a backend quirk*: Replace the scan band with a `0.03`-wide `MeshBasicNodeMaterial` bar mesh translated in `X`, and replace the TSL hatch with a cached `64 x 64` diagonal-stripe `CanvasTexture` overlaid on a `+0.002 Y` decal quad (`visible = uRefuseMix > 0`). Zero shader risk.
- **Deterministic `seek(t)` architecture.** Each `sceneN.js` precomputes a pure JSON-like beat table of fixed start/end transforms and deterministic `5 x 5` dot bitmasks at `build(ctx)` time. `seek(t)` resolves `(beatIndex, localT)` by binary search, resets all instance matrices and colors from static base arrays, applies `easeInOutCubic` interpolations for the active beat, updates `DOM` label anchors via `Vector3.project(camera)`, and touches zero frame-to-frame state. Under `prefers-reduced-motion`, `shell.js` calls `seek(beat.endTime - 0.1)` to jump directly between end-of-beat still poses.
- **Draw-call and instance budget (integrated GPU 60 fps target, 1 shadow-casting directional light at `2048²` with `shadow.autoUpdate = false` refreshed only when objects move):**

| Scene | Draw calls | Max instances (`InstancedMesh`) | Heaviest geometry |
|---|---:|---:|---|
| Shell (rails + carrier + keys) | 6 | 112 (4x25 carrier pips + 12 pins) | Continuous bench plane + 3 key cards |
| 1. The program | 14 | 160 (ROM cells + 3x25 token pips) | Two ROM trays + 2 Poseidon blocks |
| 2. The run | 16 | 240 (`120` rule tiles + tape cells) | `12 x 10` rule matrix + 3 exit tracks |
| 3. The leaf | 28 | 380 (registers, limbs, tray tiles, pips) | Floor plan housings + `TOP = 63` broken column |
| 4. The node | 18 | 190 (slabs, pins, ring ticks, DAG nodes) | 3 parallel cradles + DAG relief lines |
| 5. The decider | 16 | 175 (header cells, shutter, 2x25 pips) | Shutter assembly + split value plate |
| 6. The transaction | 12 | 110 (envelope traces + token pips) | Slate/paper split ground + envelope |
| 7. The contract | 18 | 260 (`16` cabinet drawers + pips) | `runs` cabinet + 2 GP hook sub-plates |
| 8. The network (incl. wide shot) | 34 | 320 (`6` validator plates + simplified bench) | 6 node plates + low-LOD bench proxy |

  *Note on Scene 8 wide shot*: Station `8` owns its own low-detail proxy silhouette of Stations `1–7` (`1` merged `BufferGeometry` per material, `6` draw calls total) for Beat 8.3, respecting the strict rule that no scene reaches into another scene's objects.
- **What to cut first if a scene runs slow:**
  1. Drop `devicePixelRatio` cap from `2.0` to `1.35` during camera transitions (`shell.js` adaptive DPR).
  2. Freeze `keyLight.shadow.needsUpdate` to once at beat end (`u = 1`) rather than updating during `0.8 s` slides; shallow `0.06` bas-relief reads primarily from directional normal shading and AO edge contrast rather than cast shadows.
  3. Merge all static `inkSoft` ruled grid lines and registration marks per station into a single `LineSegments` draw call.

---

## 7. Three risks

1. **Risk: Shallow `0.06`-unit 2.5D relief looks washed out on light `paper` (`0xf4f0e8`) under near-top-down `Inspect` shots.**
   - *Repair*: Outline every `ceramicAlabaster` housing plate with a `0.015` recessed `ceramicEdge` (`0xd5cec0`) moat and an `inkSoft` (`0x333c4d`) `LineSegments` perimeter crease, and keep the key light at grazing incidence (`pos (6.0, 10.5, 5.4)`) so every north and west bevel stays crisp even at `85°` pitch.
2. **Risk: Scene 3's thirteen modules overwhelm the five-second reading rule when shown together.**
   - *Repair*: Keep inactive modules at low-contrast `paperWarm` / `ceramicEdge` line-drawing state (`0` labels, `0` saturated color) during `Inspect` beats; only the `2` or `3` modules participating in the active beat take `titanium` / `heat` / `gold` contrast and DOM labels. Every beat isolates one localized sub-circuit inside the unchanging floor plan.
3. **Risk: Viewers read the `—╱╲—` break mark on the `2^64`, `2^66`, and `TOP = 63` rulers as a literal short bar of ~`10` units rather than a vast power-of-two gulf.**
   - *Repair*: Precede every break mark with logarithmic decade ticks (`16`, `2^8`, `2^16`, `2^32`) whose spacing visibly compresses before the cut, and pair the far side of the cut with the `8`-byte (`64`-bit) segmented collar so both logarithmic compression and bit-width reinforce the jump.
