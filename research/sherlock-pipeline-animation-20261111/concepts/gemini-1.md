# Visual Concept 1: The Instrument

## 1. Concept in one paragraph

Each of the eight stations on the bench is a metrological instrument machined from matte alabaster ceramic, brushed titanium, and dark chain slate. Housings part along precision rails and lift their lids to expose internal working parts: rule keys, lookup channels, ceramic memory trays, hash compression dies, height calipers, and privacy shutters. Data never floats or glows as abstract light; every field element, digest, stack link, witness frame, proof slab, and accumulator ring is a solid object with beveled edges, registration pins, and physical stops that a hand can pick up. Most beats use square, long-lens Inspect views so the viewer reads mechanism through mechanical fit, coincidence, and interlock. The single image the viewer remembers is the open leaf instrument of Station 3: sixteen step slots driving a five-cube bus bar down a milled titanium channel while eight ceramic tiles lock into a segment tray beneath a depth-63 gold Merkle column.

---

## 2. Object sheet

### Viewport framing and stage lighting

The stage reuses the studio environment map, warm key light (`0xfff7ea`, single `2048` PCF shadow map), cool fill (`0xd5e3f8`), champagne rim (`0xffeed8`), and ACES tone mapping (`exposure = 1.06`) of the workbench. Stations 1 through 5 sit on a continuous `paper` (`0xf4f0e8`) bench with `paperWarm` (`0xebe5da`) inlaid instrument beds; Stations 7 and 8 sit on a `graphite` (`0x1c2331`) slab with `wellSlate` (`0x161d29`) recessed wells; Station 6 bridges the physical butt-joint between them. On wide viewports (`> 960 px`), `camera.setViewOffset` shifts the optical center into the left two-thirds of the canvas so the right third stays clear for the DOM narration panel. On narrow viewports (`<= 960 px`), the view offset clears, the canvas takes the top `56%` of the screen at full width with camera distance scaled by `1.22x`, and the narration panel docks underneath.

### Four universal material states

Every object and housing part communicates its state without text through four surface and positional states:

- **Idle:** Matte `ceramicAlabaster` (`0xebe5da`, `roughness = 0.82`, `metalness = 0.02`, mineral grain map) or brushed `titanium` (`0xd8dde6`, `roughness = 0.34`, `metalness = 0.78`, anisotropic streak map), `ink` (`0x14171f`) markings, zero emissive output, seated flush at rest elevation (`Δy = 0`).
- **Active:** The operating component steps up `0.04` units on a detent rail; its bevel collar takes `heat` (`0xea580c`) albedo with `heatBright` (`0xf97316`) emissive (`emissiveIntensity = 0.55`). Non-participating neighbors stay matte and still.
- **Passed:** A thin specular sweep bar crosses the object; its rim pulses `goldBright` (`0xfbbf24`, `emissiveIntensity = 0.70`) for `0.25 s`, then settles into a permanent locked state: specular `gold` (`0xd4af37`, `roughness = 0.24`, `metalness = 0.88`, zero emissive) with a physical brass-gold latch pin seated flush in its notch. In any still frame, a passed object shows a closed gold latch collar.
- **Refused:** Forward travel halts against a raised stop pin, steps back `0.04` units leaving a visible air gap, and drops a physical 45-degree diagonal cross-hatch grille (`InstancedMesh` of `0.008`-wide ribs in `errorRed` `0xe11d48`, `emissiveIntensity = 0.35`) across its top face. The gap and diagonal ribs hold in the still pose.

### Data objects

1. **Field element, register, cell:** Rounded cube (`0.12 x 0.12 x 0.12` units, `0.018` bevel), matte `ceramicAlabaster` with `ceramicEdge` (`0xd5cec0`) seams and one `ink` top pip. Instanced via `InstancedMesh` per module register bank. Turns `heat` on write; returns to matte ceramic when latched.
2. **Digest token and 5x5 dot face:** Regular hexagonal prism (circumradius `0.22` units, thickness `0.06` units, `0.01` top chamfer). The outer rim carries the family colour: `gateViolet` (`0x818cf8`) for `prog_d`, `steel` (`0x0284c7`) for `input_d`, `resolve` (`0x0d9488`) for `outcome_d`, and `gold` (`0xd4af37`) for state, stack, tree, set, registry, and `claim_d` digests. The recessed center disc (`paperWarm` on prover side, `wellSlate` on chain side) carries a `5 x 5` grid of `25` instanced cylindrical pips (`pitch = 0.048`, radius `0.016`). A deterministic 32-bit FNV-1a hash of the canonical content identifier sets each of the `25` positions: `1` raises a `0.012`-tall `ink` pip (`gold` pip on slate), `0` leaves a `0.004`-deep recessed dimple, clamped to `7..18` raised pips so no face is blank. **On-screen comparison:** To compare two digests, the upper token translates over the lower token at `Δy = +0.08` in an Inspect view, holds `0.15 s` so every raised pip visibly aligns coaxially over its twin below, then drops to `Δy = 0` so both hexagonal rims coincide into one outline. If two faces differ, mismatched raised pips strike solid rim flats, holding the upper token `0.03` units proud with an `errorRed` hatch.
3. **Split boundary token (`pre_d`, `post_d`) `[Change to the system]`:** State digests keep the exact `0.22`-radius gold-rimmed hexagonal footprint and `5 x 5` face of every digest token, machined in two horizontal halves: a `0.025`-thick upper header cap (with five underside micro-pins for `outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`) seated onto a `0.035`-thick lower `body_d` base plug. *Reason:* Makes selective header opening in Scene 5 physically self-explanatory without exposing body registers or memory roots.
4. **Poseidon hash block:** Heavy rectangular compression die (`0.48 x 0.28 x 0.32` units, `0.02` bevel) in brushed `titanium` with an inlaid `gold` intake slot on the left and a hexagonal `gold` ejection port on the right. One shared geometry across all scenes.
5. **Bus tuple:** Rigid titanium carrier bar (`0.68 x 0.14 x 0.14` units) holding five rounded cubes `(chip, key, a, b, c)`. Cube 0 (`chip`) carries deep engraved tally notches on its top face—1 notch (`ALU`), 2 notches (`MEM`), 3 notches (`HASH`), 4 notches (`ROM`)—tinted `heat` while moving on the bus.
6. **Memory record and 8-cell segment tray:** One record is a flat square `ceramicAlabaster` tile (`0.14 x 0.03 x 0.14` units, `0.006` chamfer) with `inkSoft` (`0x333c4d`) kind-groove marks (`1..4` edge notches; padding tiles smooth and ungrooved). Eight tiles sit side-by-side in a milled `ceramicEdge` tray (`1.24 x 0.06 x 0.20` units): one segment.
7. **Memory tree column:** Vertical square frame (`0.52 x 1.90 x 0.36` units) rising above the backend tray bay. Draws the bottom `Tree8` compression block, two lower gold hash levels, a machined `ceramicEdge` zig-zag break jaw at mid-height, two top gold hash levels, the `root` token socket at the crown, and a vertical `ink`-graduated depth ruler (`0` at segment base to `63` at crown).
8. **Machine step frame and witness tape:** Each step frame is a `0.26 x 0.03 x 0.44` `paperWarm` card bordered in `ceramicEdge`, holding three inlaid parts front-to-back: a narrow `ink` rule-id plate, a pre-state cube strip, and a post-state cube strip. Connected edge-to-edge along `+X`, frames form the witness tape; every `16` frames carry a scored shear groove.
9. **Stack (`FRAMES`, `TASKS`):** Each stack rack (`0.42 x 0.10 x 0.70` units) holds one exposed register strip for the cached top record at the front and a vertical chute of linked `gold` hexagonal tokens behind it (`frames_below`, `tasks_below`), joined by hinged `titanium` shackle bars.
10. **Proof slab and keyed pins `[Change to the system]`:** Every leaf proof, node proof, and decider seal is a brushed `titanium` slab of identical footprint (`1.10 x 0.12 x 0.74` units, `0.015` gold chamfered perimeter). Public values project from the front (`+Z`) edge as machined registration pins (`7` on a leaf slab, `5` on a node slab, `4` on the decider seal). *Refinement:* Instead of uniform cylinders, digest pins carry hexagonal collars in their family hue (`gateViolet` for `prog_d`, `steel` for `input_d`, `resolve` for `outcome_d`, `gold` for `binding`, `pre_d`, `post_d`, `rules_d`), while integer height and step pins (`h_pre`, `steps`, `h_lo`, `h_hi`, `n_steps`) are rectangular Vernier bars with `ink` graduations. *Reason:* In any label-free frame, pin shape and collar colour distinguish every public value and make segment stitching self-evident.
11. **Accumulator ring:** Lathe-turned `gold` torus (outer radius `0.20`, tube radius `0.026`) seated in a circular recess on the top face of every node slab. Each folded obligation adds one radial `ink` score notch around the upper bevel (`InstancedMesh` of score ticks); a second ring never appears. When paired in Scene 5, a central `gold` plug locks inside the torus bore, turning the ring into a flush solid seal disc.
12. **Verifier key rack and optical gate bridge `[Change to the system]`:** Mounted on the back wall rail (`z = -1.65, y = +1.35`) above the bench sit three flat brushed `gold` key cards (`0.38 x 0.54 x 0.03` units) with distinct milled bit-notches along their bottom edges: `leaf`, `node`, `decider`. *Refinement:* When a station proves or verifies under a key, a sliding titanium optical reader bridge drops a narrow `gold` alignment bar onto that station without unseating the card. *Reason:* Shows visually across every scene that the three keys stay fixed on the wall when programs and inputs change.
13. **Token carrier:** Owned by the shell at `z = +1.65, y = +0.08`, sliding on a front `titanium` rail in sync with the camera. Holds four recessed sockets: two hexagonal pockets on the left (`prog_d`, `input_d`, populated in Scene 1), one hexagonal pocket (`outcome_d`) and one 8-byte Vernier slot (`n_steps`) on the right (populated in Scene 5).
14. **Claim drawer and `runs` cabinet:** Dark `graphite` apothecary cabinet (`1.60 x 1.30 x 0.65` units) with `wellSlate` drawers; each drawer faceplate has a hexagonal `gold` socket for `claim_d` on the front and two interior pockets (`outcome_d`, `n_steps`) inside.
15. **Transaction envelope:** Flat `graphite` case (`1.45 x 0.16 x 0.96` units, `0.015` bevel) with `wellSlate` interior cutouts for the decider seal slab, the four public value tokens, a call-target plate, a side `DUST` wafer slot, and a sliding titanium **Transcript Locking Bar** (`[Change to the system]`, *reason:* mechanically couples the four value pockets to the seal's `binding` notch so tampering visibly jams the lock).

### Station housings (1 to 8)

- **Station 1 (Compiler & Digest Press):** Split `ceramicAlabaster` bed (`4.4 x 0.24 x 2.4` units): left source platen and ROM table rack (`gateViolet` guides), right dual Poseidon press (`prog_d` and `input_d` under `sherlock-value/1`).
- **Station 2 (Reference Machine `sherlock-k-a/1`):** Hinged `ceramicAlabaster` instrument case (`4.8 x 0.42 x 2.6` units) whose upper cowl retracts backward (`-Z`) by `0.9` units to reveal six state bays, a `12 x 10` matrix of `120` ceramic rule keys, a `64`-bit Vernier step drum, and a right-hand witness tape tractor feed.
- **Station 3 (Leaf Circuit):** Heavy `ceramicAlabaster` chip substrate (`5.4 x 0.30 x 3.2` units) under a brushed `titanium` lid (`k <= 16`) that lifts `1.6` units upward and fades out during cutaway, revealing physical module blocks around a recessed `titanium` lookup channel (`BUS`), an `8`-tile tail tray, cache line bays, and the `1.90`-unit vertical Merkle column.
- **Station 4 (Recursion Node):** Twin-jawed `titanium` and `ceramicAlabaster` merge press (`4.6 x 0.36 x 2.6` units, `k = 20`) with left/right child slab carriages that converge toward `x = 0`, an overhead accumulator folding die, and interchangeable `MERGE` / `JOIN` mode plates.
- **Station 5 (Run Decider):** Two-chamber `ceramicAlabaster` and `titanium` sealing housing (`4.6 x 0.48 x 2.5` units, `2^20` rows): an internal inspection bay (genesis zero-pin, header extractor claw, pairing caliper) separated from the right output plate by a vertical brushed `titanium` **Privacy Shutter** (`[Change to the system]`, *reason:* gives physical form to hiding state digests, roots, lengths, and addresses) pierced only by four value apertures and the seal slot.
- **Station 6 (Crossing Gate):** Half-`paper`, half-`graphite` bridge table (`4.2 x 0.20 x 2.2` units) split cleanly at `x = 0.0`, carrying the envelope assembly jig on the left and a titanium rail crossing onto dark slate on the right.
- **Station 7 (`SherlockRun` Contract):** Monolithic `graphite` verification desk (`4.8 x 0.50 x 2.8` units) housing a verifier sweep bridge, a Poseidon `claim_d` recompute die, the `runs` drawer cabinet, and a side `reg_root` key-registry column.
- **Station 8 (Midnight Network):** Curved `graphite` ledger plinth (`5.2 x 0.30 x 3.0` units) supporting five instanced `graphiteLight` (`0x2a3446`) validator monoliths arranged in a shallow arc around a central settled-ledger spine, plus downstream contract and prover reader rails.

---

## 3. Motion sheet

Every motion is a pure deterministic function of scene time `t` using `easeInOutCubic(u)` where `u = clamp((t - t0) / dur, 0, 1)`. All non-participating parts hold still at zero velocity.

1. **Poseidon hash (`dur = 0.90 s`):**
   - `0.00–0.30 s`: Ordered input cubes/tokens step into a straight horizontal rank `0.45` units left of the gold Poseidon block (`Δz = 0`, `0.14` pitch).
   - `0.30–0.65 s`: The rank translates along `+X` (`easeInOutCubic`) into the left intake slot while the block's gold seam pulses `goldBright`.
   - `0.65–0.90 s`: One hexagonal digest token emerges `0.38` units from the right ejection port with its deterministic `5 x 5` dot face engraved and stops on its detent pad. Everything else on the station holds still.
2. **Equality (`dur = 0.75 s`):**
   - `0.00–0.40 s`: Token B lifts `0.08` units and translates (`easeInOutCubic`) directly above Token A (`Δx = 0, Δz = 0, Δy = +0.08`).
   - `0.40–0.55 s`: Holds coaxial alignment for `0.15 s` so the two identical `5 x 5` pip grids read clearly.
   - `0.55–0.75 s`: Lowers to `Δy = 0`; the two rims merge into one outline and flash `goldBright` before settling into locked `gold`.
3. **Verification (`dur = 0.40 s`):**
   - `0.00–0.28 s`: A `0.015`-wide `goldBright` optical sweep bar translates linearly (`-X` to `+X`) across the top surface of the slab or token while the overhead key reader bridge illuminates its matching wall key card.
   - `0.28–0.40 s`: The object's perimeter bevel flashes `goldBright`, then seats its brass-gold latch pin flush.
4. **Refusal (`dur = 0.55 s`):**
   - `0.00–0.22 s`: Object advances along `+X` until it contacts a raised `ink` stop pin and halts abruptly (clamped linear ramp, zero bounce).
   - `0.22–0.38 s`: Steps backward `-0.04` units (`easeInOutCubic`).
   - `0.38–0.55 s`: The `errorRed` (`0xe11d48`) 45-degree diagonal hatch grille lowers `0.01` units onto the object's face and stays locked through the hold pose.
5. **Station handoff (`dur = 1.80 s`):**
   - Scene `N` ends with its output object seated at local `(+2.8, 0.12, 0.0)`. In Scene `N+1` beat 1 (`Follow` shot), the camera and front token carrier translate `+5.6` global `X` units (`easeInOutCubic` over `1.80 s`) while the handoff plate carries the identical object from local `(-2.8, 0.12, 0.0)` into Station `N+1`'s intake dock at `(-1.9, 0.12, 0.0)`.
6. **Housing split / cutaway (`dur = 1.00 s`):**
   - Upper ceramic cowls slide along `-Z` (`0.9` units) or titanium lids lift along `+Y` (`1.6` units, `easeInOutCubic`) before any internal beat motion starts, and stay parked on mechanical stanchions until the closing shot.
7. **Bus request and response (`dur = 1.00 s`):**
   - `0.00–0.45 s`: Five-cube `(chip, key, a, b, c)` bar leaves `CORE` port along request lane 0 or 1 (`easeInOutCubic`), tinted `heat`, and docks against the matching provider row socket.
   - `0.45–0.65 s`: Coincidence flash (`goldBright`) against the provider row (`LookupSound` membership check).
   - `0.65–1.00 s`: Authenticated result cell copies slide back along the return groove into `CORE`'s post-register bank while the bus bar clears to matte ceramic.

---

## 4. Scene by scene, 1 to 8

*(Coordinates `(x, y, z)` are station-local: station center `(0, 0, 0)`, bench top `y = 0`, `+X` right, `+Y` up, `+Z` toward viewer. Token carrier sits at `z = +1.65`; wall key rack at `(0, 1.35, -1.65)`.)*

### Scene 1: The program (target 80 s)

**Station layout:** Source sheet platen at `(-1.8, 0.14, -0.2)`; compiler press at `(-0.8, 0.22, -0.2)`; `gateViolet`-trimmed ROM table rack at `(0.2, 0.24, -0.3)` holding stacked 8-row code tables; `prog_d` Poseidon block at `(1.2, 0.20, -0.4)`; input value tray (`steel` trim) at `(0.2, 0.16, 0.55)` feeding the `sherlock-value/1` Poseidon block at `(1.2, 0.20, 0.55)`; right handoff plate at `(2.8, 0.12, 0.0)`.

- **Shot 1.1 — Establish** | Cam `(0.0, 2.5, 4.4)` -> `(0.0, 0.2, 0.0)`
  - *Objects:* Paper sheet of Moriarty source, compiler press, empty ROM rack, overhead key rack (`leaf`, `node`, `decider`), empty front token carrier.
  - *Key motion:* Source sheet feeds (`+X`, `0.9 s`) through the compiler press, stamping stacked `gateViolet`-edged code table cards into the ROM rack.
  - *Mechanism:* Moriarty source compiles to a Sherlock program image (code tables in ROM).
  - *Still pose:* ROM rack filled with indexed code table plates; source sheet parked left.
  - *Labels:* `Moriarty source`, `Program image (ROM)`, `leaf · node · decider`.
- **Shot 1.2 — Inspect** | Cam `(0.8, 2.2, 0.9)` -> `(0.8, 0.18, 0.1)`
  - *Objects:* ROM table rack, `prog_d` Poseidon block, input value cubes, `sherlock-value/1` Poseidon block, front token carrier.
  - *Key motion:* ROM table rows hash through the upper gold Poseidon block into violet-rimmed `prog_d`; then input cubes hash through the lower `sherlock-value/1` block into blue-rimmed `input_d`; both tokens seat into the front token carrier (`1.1 s`).
  - *Mechanism:* Program image hashes to `prog_d`; input value digests under `sherlock-value/1` to `input_d`.
  - *Still pose:* `prog_d` and `input_d` seated in left two sockets of the front carrier with distinct `5 x 5` dot faces; ROM rack holds a duplicate `prog_d` pin.
  - *Labels:* `prog_d`, `input_d`, `sherlock-value/1`, `Poseidon digest`.
- **Shot 1.3 — Inspect** | Cam `(0.2, 1.3, 2.8)` -> `(0.2, 0.75, -0.6)`
  - *Objects:* ROM rack, `prog_d` and `input_d` sockets, overhead wall rack with three gold key cards (`leaf`, `node`, `decider`) and optical gate bridge.
  - *Key motion:* A second program's ROM rack slides across the bed (`0.9 s`), swapping `prog_d` to a new `5 x 5` dot pattern while the optical bridge sweeps across the three wall key cards without moving or changing a single notch on `leaf`, `node`, or `decider`; then the first program rack returns.
  - *Mechanism:* One verifier key set serves every program because the program is ROM data, not circuit gates.
  - *Still pose:* Original ROM rack and `prog_d` restored; three gold key cards locked and unchanged in the wall rack with passed gold latch pins.
  - *Labels:* `Program is data`, `One key set`, `prog_d`, `leaf · node · decider`.
- **Shot 1.4 — Establish (Handoff 1 -> 2)** | Cam `(0.8, 2.5, 4.4)` -> `(1.4, 0.2, 0.0)`
  - *Objects:* Whole Station 1, ROM rack, `prog_d` and `input_d` in carrier, right handoff plate.
  - *Key motion:* ROM rack and paired `prog_d`, `input_d` reference tokens slide (`1.0 s`) onto the right handoff plate at `(2.8, 0.12, 0.0)`.
  - *Mechanism:* Program image and digests hand off to execution.
  - *Still pose:* ROM rack and tokens resting on handoff plate `(2.8, 0.12, 0.0)`; `prog_d` and `input_d` locked in front carrier.
  - *Labels:* `Program image (ROM)`, `prog_d`, `input_d`.

*Transition 1 -> 2:* `Follow` rail move (`1.8 s`) tracking the ROM rack and tokens across the seam into Station 2's left dock while the front carrier glides alongside.

---

### Scene 2: The run (target 110 s)

**Station layout:** Left ROM/input dock at `(-2.3, 0.16, -0.3)`; six-bay machine bed (`sherlock-k-a/1`) centered at `(-0.4, 0.18, -0.2)` containing `control` dial, `environment` slot, `frame stack` chute, `task stack` chute, `memory` tile bed, and `fuel` counter bar; `12 x 10` matrix of `120` ceramic rule keys at `(-0.4, 0.14, 0.65)`; `64`-bit Vernier step counter drum at `(0.9, 0.24, -0.55)`; witness tape tractor feed running `(1.0, 0.16, 0.1)` to `(2.8, 0.12, 0.0)`.

- **Shot 2.1 — Establish** | Cam `(0.0, 2.6, 4.5)` -> `(0.0, 0.2, 0.0)`
  - *Objects:* Closed `sherlock-k-a/1` ceramic housing, ROM rack on left dock, `prog_d` and `input_d` in carrier.
  - *Key motion:* ROM rack locks into left dock (`0.6 s`) and the ceramic cowl slides back `-0.9` units (`1.0 s`) to expose the six interior state bays and the `120`-key rule matrix.
  - *Mechanism:* The reference machine initializes from the program and input and exposes its six explicit state components.
  - *Still pose:* Open six-bay machine bed and matte `120`-rule matrix visible from above.
  - *Labels:* `sherlock-k-a/1`, `control · environment`, `frame · task stacks`, `memory · fuel`.
- **Shot 2.2 — Inspect** | Cam `(-0.4, 2.3, 0.8)` -> `(-0.4, 0.15, 0.2)`
  - *Objects:* Six state bays, `12 x 10` rule matrix (`120` keys), first frame on witness tape.
  - *Key motion:* On one machine step, key `#47` in the `120`-rule matrix steps up `0.04` units in `heatBright` while the other `119` keys stay flat and matte (`0.4 s`); `control` and `fuel` cells update, and one witness frame (`rule id`, `state before`, `state after`) stamps onto the witness tape (`0.7 s`).
  - *Mechanism:* Each step fires exactly one of `120` enabled rules and writes one witness frame.
  - *Still pose:* One lit `heat` rule key raised out of `120`; stamped witness frame resting on tape showing `rule id`, pre-state strip, post-state strip.
  - *Labels:* `1 of 120 rules`, `Witness frame`, `state before · after`.
- **Shot 2.3 — Inspect** | Cam `(0.9, 1.8, 0.6)` -> `(0.9, 0.22, -0.45)`
  - *Objects:* `64`-bit Vernier step counter drum (`8` byte wheels with break mark to `2^64 - 1`), advancing witness tape.
  - *Key motion:* Three rapid steps advance the tape three frames (`0.9 s`) while the low wheel of the `64`-bit counter clicks forward `3` ticks beside an engraved scale bar marked `0 .. 2^64 - 1`.
  - *Mechanism:* Steps are counted by a `64`-bit counter (`2`<sup>`64`</sup> is counter range, not circuit size).
  - *Still pose:* Counter wheel locked on current step count beside the break-marked `2`<sup>`64`</sup> `- 1` range scale.
  - *Labels:* `64-bit step counter`, `Up to 2⁶⁴ − 1 steps`, `Counter range`.
- **Shot 2.4 — Inspect** | Cam `(1.5, 2.1, 0.9)` -> `(1.5, 0.16, 0.1)`
  - *Objects:* Terminal exit gate with three side-by-side reference exit stamps above the tape end: `Halted` (`resolve` teal value cube), `Rejected` (`errorRed`-trimmed code plate stamped `10`), `Exhausted` (`amber`-trimmed kind plate beside zeroed fuel bar).
  - *Key motion:* Selector carriage steps across the three terminal stamps (`1.1 s`), demonstrating `Rejected` code `10` (arithmetic overflow) and `Exhausted` (fuel empty) as valid recorded terminal frames before locking onto the active run's `Halted` value frame at the tape end.
  - *Mechanism:* Every run terminates in one of three provable outcomes: `Halted` (value), `Rejected` (code `10` overflow), or `Exhausted` (kind).
  - *Still pose:* Three terminal stamp dies visible in comparison rack; `Halted` outcome frame locked onto the end of the witness tape.
  - *Labels:* `Halted (value)`, `Rejected (code 10)`, `Exhausted (kind)`.
- **Shot 2.5 — Establish (Handoff 2 -> 3)** | Cam `(0.8, 2.6, 4.5)` -> `(1.4, 0.2, 0.0)`
  - *Objects:* Station 2 bed, coiled/extended witness tape ending in its terminal outcome frame, right handoff plate.
  - *Key motion:* Witness tape advances (`1.0 s`) onto the right handoff plate `(2.8, 0.12, 0.0)`.
  - *Mechanism:* Complete witness stream hands off to leaf proving.
  - *Still pose:* Witness tape with terminal frame resting on `(2.8, 0.12, 0.0)`.
  - *Labels:* `Witness stream`, `prog_d`, `input_d`.

*Transition 2 -> 3:* `Follow` move (`1.8 s`) tracking the witness tape across the gap into Station 3's slicing shear.

---

### Scene 3: The leaf (target 240 s)

**Station layout (floor plan on `5.4 x 3.2` substrate, modeling `leafChip.js` in 3D physical relief):**
- Tape shear & `16`-slot step ribbon: `(-2.5, 0.22, -1.1)` to `(-0.6, 0.22, -1.1)`.
- `INIT` genesis caliper: `(-2.3, 0.20, -0.35)` (adjacent to entry `pre_d` press).
- `CORE` block: `(-1.6, 0.20, -0.15)`, size `1.5 x 0.18 x 1.5`, holding pre/post register pairs, read port, and append chain bar.
- `STACKS` (`FRAMES`, `TASKS` hash-chain racks): `(-1.6, 0.20, +0.95)`, two parallel linked-token chutes.
- `BUS` channel: `(-0.55, 0.12, 0.0)`, a milled `titanium` trench (`0.32` wide, `2.3` long along `Z`) with `2` recessed request tracks and `1` provider socket lip.
- Tagged fixed-table rail (`DEC` + `RANGE`): `(-0.55, 0.14, -1.25)` running to `(+0.65, 0.14, -1.25)`.
- Provider column (`x = +0.45`): `DEC` (`z = -0.95`), `PAGE/ROM` block cache (`z = -0.45`, holding unnumbered heap line trays and violet-edged ROM line trays), `ALU` + `RANGE` (`z = +0.05`, with hinged overflow flag latch and `12`-bit limb splitter), `MEM window` (`z = +0.55`, `8`-cell imported tail + new tile slots), `HASH` + `VALUE DIGEST` (`z = +1.05`, gold Poseidon sponge blocks).
- `MEMORY BACKEND` & depth-`63` tree column: `(+1.65, 0.20..2.10, +0.45)`, holding `w_flush = 1` segment flush chute, scalar `cold_read` opening track, and the `1.90`-tall break-marked gold Merkle column.
- `BOUNDARY COMMITMENT` presses: entry `pre_d` at `(-2.3, 0.20, +0.35)` and exit `post_d` at `(+1.85, 0.20, -0.65)` (each with a body sponge die and a 5-pin header sponge die).
- Front `7`-pin Leaf Slab press: `(+2.2, 0.16, +1.15)` leading to handoff plate `(+2.8, 0.12, 0.0)`.

- **Shot 3.1 — Establish** | Cam `(0.0, 2.9, 4.8)` -> `(0.0, 0.3, 0.0)`
  - *Objects:* Witness tape entering left titanium shear, closed `k <= 16` leaf housing, overhead `leaf` gold key card lit by optical bridge.
  - *Key motion:* Shear blade indexes and cuts `S = 16` step frames (`0.5 s`); the `16`-frame slice slides into the housing while the brushed titanium lid lifts `1.6` units upward (`1.0 s`) to reveal the 3D module floor plan.
  - *Mechanism:* One leaf circuit (`2`<sup>`16`</sup> rows max, one `leaf` key) proves a `16`-step slice.
  - *Still pose:* Cutaway floor plan open; `16` step slots loaded along the `CORE` top ribbon; `leaf` key card illuminated above.
  - *Labels:* `S = 16 steps`, `One leaf key`, `k ≤ 16 (2¹⁶ rows)`.
- **Shot 3.2 — Inspect (INIT at `h_pre = 0`)** | Cam `(-2.1, 1.9, 0.5)` -> `(-2.1, 0.18, 0.0)`
  - *Objects:* `h_pre = 0` Vernier pin, `INIT` genesis caliper, `prog_d` and `input_d` copies, empty memory constants, entry `pre_d` block.
  - *Key motion:* Because `h_pre` reads `0`, the `INIT` caliper engages (`0.6 s`), opening `prog_d` (locking `prog_root` register) and `input_d` (`sherlock-value/1`, fuel, stack limit) and pinning empty stacks and empty genesis memory into the first pre-state; a ghost step to `h_pre = 16` shows the caliper retracting flush.
  - *Mechanism:* `INIT` runs inside the leaf only when `h_pre = 0`, establishing `Init(prog_d, input_d)` with empty genesis memory.
  - *Still pose:* `INIT` caliper engaged on `h_pre = 0`, `prog_root` and `input_d` latched into `CORE` entry registers.
  - *Labels:* `INIT (h_pre = 0)`, `prog_d`, `input_d`, `Empty genesis memory`.
- **Shot 3.3 — Inspect (DEC & Tagged Fixed Table)** | Cam `(-0.5, 2.0, -0.3)` -> `(-0.5, 0.16, -0.95)`
  - *Objects:* `CORE` slot 1, tagged fixed-table rail, `DEC` block (`120` rule class rows + idle row).
  - *Key motion:* `CORE` slides its dispatch key across the `DEC` port (`0.8 s`); one row of `DEC`'s fixed table steps up in `heatBright` and returns a bounded control-word bar into `CORE` slot 1.
  - *Mechanism:* `DEC` decodes the firing rule's control word from the fixed `120`-rule table.
  - *Still pose:* Selected `DEC` row raised; control-word bar seated in `CORE` slot 1.
  - *Labels:* `DEC fixed table`, `Control word`, `CORE`.
- **Shot 3.4 — Follow -> Inspect (BUS, ALU overflow, 12-bit RANGE)** | Cam `(-0.2, 1.75, 0.65)` -> `(-0.05, 0.16, 0.05)`
  - *Objects:* `CORE` request port, `BUS` 2-lane trench, 5-cube tuple bar `(1, key, a, b, c)`, `ALU` block with overflow flag latch, `RANGE` `12`-bit limb splitter.
  - *Key motion:* `CORE` pushes a 5-cube bar (`chip = 1`) down `BUS` lane 0 into `ALU` (`0.6 s`); inside `ALU`, a `128`-bit operand cube splits into `12`-bit limb wafers that drop into `RANGE` table slots (`0.4 s`), and on an overflowing sum the hinged `flag` latch flips up (`flag = 1`, `heatBright`) and returns to `CORE`.
  - *Mechanism:* `CORE` is the sole requester on `BUS` `(chip, key, a, b, c)`; `RANGE` checks `12`-bit limbs; `ALU` proves both exact ops and the overflow flag for `Rejected(10)`.
  - *Still pose:* `(1, key, a, b, c)` bar docked at `ALU` (`gold` passed collar); `12`-bit limb wafers seated in `RANGE`; overflow flag latch raised.
  - *Labels:* `(chip, key, a, b, c)`, `chip 1 · ALU`, `12-bit RANGE limbs`, `Overflow flag`.
- **Shot 3.5 — Inspect (MEM appends & 8-cell tail flush into depth-63 tree)** | Cam `(1.05, 2.1, 1.45)` -> `(1.05, 0.55, 0.50)`
  - *Objects:* `CORE` append chain, `BUS` tuple (`chip = 2`), `MEM window` `8`-tile tail tray (`6` tiles already seated), `w_flush = 1` backend chute, depth-`63` Merkle column with break jaws and `0..63` ruler.
  - *Key motion:* One step places `2` new ceramic tiles (at most `3` per step) via `MEM` bus tuples, filling slots `7` and `8` of the `8`-cell tail tray (`0.5 s`); the full `8`-tile tray locks as a segment, compresses through `Tree8`, and walks up the break-marked depth-`63` column (`0.9 s`), updating the crown `root` token while a fresh empty tail tray indexes underneath.
  - *Mechanism:* Steps append at most `3` cells; when the `8`-cell tail fills (`w_flush = 1`, `8,821` rows), it flushes as one segment into the depth-`63` Poseidon Merkle tree.
  - *Still pose:* Flushed segment digest locked at crown `root` atop the `0..63` break-marked column; fresh tail tray below ready for remaining appends.
  - *Labels:* `8-cell tail tray`, `Flush · 8,821 rows`, `Depth 63 tree`, `At most 3 appends`.
- **Shot 3.6 — Inspect (Old reads: scalar cold read vs 8-record cache line & ROM line)** | Cam `(0.85, 2.15, 0.55)` -> `(0.85, 0.40, -0.05)`
  - *Objects:* Pre-state `pre.root` socket on tree column, scalar `cold_read` vertical spine, `PAGE/ROM` bay holding fixed-capacity (unnumbered) `8`-tile heap cache trays and violet-edged `8`-row `ROM` trays (`prog_root`), `BUS` channel.
  - *Key motion:* First, one old ceramic tile climbs the `cold_read` Merkle path (`0.6 s`) to coincide with `pre.root` (`4,761` rows) and passes straight to `CORE`'s read port; second, an `8`-record heap cache tray opens against `pre.root` and parks `8` provider tuples (`chip = 2`) on `BUS` while a `ROM` tray opens `8` code rows against `prog_root` onto `BUS` (`chip = 4`, `0.6 s`).
  - *Mechanism:* Isolated old reads open by scalar Merkle walk against `pre.root` (`4,761` rows); repeated locality uses `8`-record heap cache lines (`chip 2`) and `8`-row `ROM` lines (`chip 4`) on `BUS`.
  - *Still pose:* Scalar opening latched to `pre.root`; one `8`-tile heap line and one `8`-row violet `ROM` line docked on the `BUS` provider lip.
  - *Labels:* `Cold read · 4,761 rows`, `8-record cache line`, `chip 4 · ROM`, `pre-state root`.
- **Shot 3.7 — Inspect (STACKS hash chains & VALUE DIGEST)** | Cam `(-0.6, 1.85, 1.75)` -> `(-0.6, 0.20, 0.98)`
  - *Objects:* `FRAMES` and `TASKS` linked gold-token chutes, `HASH` Poseidon sponge slot, `VALUE DIGEST` block receiving `AuthRec` cells from `CORE`.
  - *Key motion:* On `FRAMES`, a push slides the old cached top record and head token into a gold Poseidon block (`0.6 s`), minting a new linked head token while a pop on `TASKS` opens its top link back into the cached top strip; beside it, `VALUE DIGEST` hashes an authenticated record into a `sherlock-value/1` cell (`0.5 s`).
  - *Mechanism:* `FRAMES` and `TASKS` are Poseidon hash chains (push hashes, pop opens); `VALUE DIGEST` computes `sherlock-value/1` inside the circuit.
  - *Still pose:* `FRAMES` chain extended by one gold shackle link; `TASKS` popped record seated in top register strip; `VALUE DIGEST` output cell latched.
  - *Labels:* `FRAMES hash chain`, `TASKS hash chain`, `Push · Pop`, `VALUE DIGEST`.
- **Shot 3.8 — Inspect (Header-and-body Boundary Commitment `pre_d` / `post_d`)** | Cam `(1.75, 1.85, -0.05)` -> `(1.75, 0.20, -0.65)`
  - *Objects:* `BOUNDARY COMMITMENT` exit press: body sponge block (receiving registers, stack chain heads, and `(len, root, tail_d)`) and 5-input header sponge block (`outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`).
  - *Key motion:* Registers, stack heads, and `(len, root, tail_d)` compress through the body Poseidon block into the `body_d` base plug (`0.5 s`); then the four header words and `body_d` compress through the header Poseidon block, locking the hexagonal header cap onto `body_d` to form `post_d` (`0.5 s`).
  - *Mechanism:* Boundary digests `pre_d` and `post_d` commit state in two-part Poseidon header-and-body form.
  - *Still pose:* Completed split-hexagonal `post_d` token showing its upper 5-pin header seam mated onto its `body_d` base.
  - *Labels:* `body_d (regs, stacks, mem)`, `5-field header`, `pre_d`, `post_d`.
- **Shot 3.9 — Inspect (7 Public Lanes on the Leaf Slab)** | Cam `(2.1, 1.5, 2.2)` -> `(2.1, 0.18, 1.1)`
  - *Objects:* Titanium lid closing over leaf board, freshly pressed brushed-titanium Leaf Slab with `7` front registration pins.
  - *Key motion:* Titanium lid lowers onto the board (`0.6 s`) and ejects one Leaf Slab forward (`0.5 s`), seating its `7` keyed front pins: gold `binding`, violet `prog_d`, split-gold `pre_d`, split-gold `post_d`, gold `rules_d`, and two Vernier bars `h_pre` and `steps`.
  - *Mechanism:* Each leaf publishes exactly seven lanes `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`.
  - *Still pose:* Frontal three-quarter close-up of the Leaf Slab with all `7` profile-keyed pins clearly distinguishable without text.
  - *Labels:* `[binding, prog_d, pre_d, post_d,`, `rules_d, h_pre, steps]`.
- **Shot 3.10 — Inspect (Partition Independence)** | Cam `(0.4, 2.7, 1.2)` -> `(0.4, 0.20, 0.2)`
  - *Objects:* Dual comparison rail on bench bed: Track A cuts a `24`-step run into `16 + 8` steps; Track B cuts the same `24`-step run into `10 + 7 + 7` steps; final `(len, root, tail_d)` trays and terminal `post_d` tokens at right end of both tracks.
  - *Key motion:* Track B's terminal `(len, root, tail_d)` tray and `post_d` token translate (`0.75 s`) onto Track A's terminal tray and token in the fixed **Equality** coincidence motion, merging identical `5 x 5` dot faces and tile counts.
  - *Mechanism:* Partition independence: any valid cut of the run into leaves yields the exact same final memory state and claim.
  - *Still pose:* Coincident terminal memory trays and merged `post_d` outline glowing locked `gold` beside the two differently sliced tapes behind them.
  - *Labels:* `Cut A (16 + 8)`, `Cut B (10 + 7 + 7)`, `Same memory state`.
- **Shot 3.11 — Establish (Handoff 3 -> 4)** | Cam `(0.8, 2.8, 4.6)` -> `(1.5, 0.2, 0.0)`
  - *Objects:* Closed leaf station, stack of proven `7`-pin Leaf Slabs sliding onto right handoff plate `(2.8, 0.12, 0.0)`.
  - *Key motion:* Pair of adjacent `7`-pin Leaf Slabs translates (`0.9 s`) onto `(2.8, 0.12, 0.0)`.
  - *Mechanism:* Leaf slabs hand off to recursive aggregation.
  - *Still pose:* Leaf slabs resting on `(2.8, 0.12, 0.0)` with `prog_d` and `input_d` in the front carrier.
  - *Labels:* `Leaf proof slabs`, `prog_d`, `input_d`.

*Transition 3 -> 4:* `Follow` move (`1.8 s`) carrying the Leaf Slabs into Station 4's left and right merge carriages.

---

### Scene 4: The node (target 130 s)

**Station layout:** Left child carriage `L` at `(-1.3, 0.18, 0.0)` and right child carriage `R` at `(+1.3, 0.18, 0.0)` converging on the central `MERGE` / `JOIN` node press (`k = 20`) at `(0.0, 0.24, 0.0)`; overhead accumulator folding die at `(0.0, 0.55, -0.1)`; `64`-bit height Vernier bar at `(0.0, 0.22, 0.65)`; linear-work stepped gauge rail at `(-1.6, 0.14, -0.95)`; two parallel companion node presses at `z = -0.85` and `z = +0.85` (for parallel proving beat); handoff plate at `(2.8, 0.12, 0.0)`.

- **Shot 4.1 — Establish -> Inspect (MERGE & Three Equalities)** | Cam `(0.0, 2.1, 1.8)` -> `(0.0, 0.20, 0.15)`
  - *Objects:* Child slabs `L` and `R` in carriages, `node` gold key card lit on wall rack, central `MERGE` die.
  - *Key motion:* `L` and `R` slide inward (`0.5 s`); three **Equality** coincidences snap in `0.25 s` steps: violet `L.prog_d` and `R.prog_d` pins align, split-gold `L.post_d` and `R.pre_d` pins interlock into one outline, and rectangular `L.h_hi` and `R.h_lo` Vernier bars butt-joint flush.
  - *Mechanism:* `MERGE` verifies both children under `k = 20` and enforces same `prog_d`, `L.post_d == R.pre_d`, and `L.h_hi == R.h_lo`.
  - *Still pose:* Interlocked `L` and `R` pins locked in `gold` at the center seam.
  - *Labels:* `same prog_d`, `L.post_d == R.pre_d`, `L.h_hi == R.h_lo`, `k = 20 node`.
- **Shot 4.2 — Inspect (Same-Size Slab & Single Accumulator Ring)** | Cam `(0.0, 2.0, 1.1)` -> `(0.0, 0.26, 0.0)`
  - *Objects:* Newly pressed parent Node Slab (`1.10 x 0.12 x 0.74` units—identical footprint to `L` and `R`), `5` front pins `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`, gold accumulator ring in top recess.
  - *Key motion:* Child accumulator rings and the node's deferred pairing check slide into the folding die (`0.8 s`) and press into **one** gold torus on the parent slab, adding two radial score notches without changing ring or slab dimensions.
  - *Mechanism:* Deferred pairing checks fold into one accumulator, so a node slab at any height keeps the exact same size.
  - *Still pose:* Single Node Slab framed inside fixed titanium caliper jaws, carrying `5` front pins and `1` multi-scored gold ring.
  - *Labels:* `Same slab size`, `One accumulator ring`, `(prog_d, pre_d, post_d, h_lo, h_hi)`.
- **Shot 4.3 — Inspect (Height Below 2^64 & Linear Work)** | Cam `(-0.4, 2.1, 1.3)` -> `(-0.4, 0.22, -0.35)`
  - *Objects:* `h_hi` Vernier bar against break-marked `0 .. 2^64` counter rail; linear-work bar gauge behind (`1`, `2`, `4` slices matched by `1`, `2`, `4` uniform work blocks along a straight `45°` titanium straightedge).
  - *Key motion:* As two more slabs merge, `h_hi` advances along the `< 2^64` counter rail while one uniform work cube appends under the straightedge (`0.8 s`), and the output slab footprint remains unchanged.
  - *Mechanism:* Heights below `2`<sup>`64`</sup> bound a counter, not circuit size; total prover work grows linearly with steps (`65 s` per node at `20` threads, `17 GB`).
  - *Still pose:* Straight diagonal edge resting cleanly across the linear work blocks; `h_hi < 2^64` latched; constant-size slab in front.
  - *Labels:* `Heights < 2⁶⁴`, `Linear prover work`, `65 s · 20 threads`.
- **Shot 4.4 —Inspect (Parallel Provers, Trees, DAGs & JOIN)** | Cam `(0.0, 3.1, 1.6)` -> `(0.0, 0.20, 0.0)`
  - *Objects:* Three parallel node presses closing at once (`1.73x` at `3` provers), feeding an Inspect relief diagram of a binary merge tree and a diamond DAG where two sealed run `SET` tokens enter a `JOIN` mode plate (`H(SHK_SET_JOIN, l, r)` -> `set_root`).
  - *Key motion:* Three parallel presses cycle together (`0.8 s`), then two `SET` tokens slide through the `JOIN` Poseidon block into one gold `set_root` token (`0.7 s`).
  - *Mechanism:* Independent provers merge branches in parallel across any tree or DAG; `JOIN` combines sealed runs into a set (`set_root`) with no claim counter.
  - *Still pose:* `3` parallel presses latched; DAG relief showing `MERGE` along segments and `JOIN` producing `set_root`.
  - *Labels:* `3 provers · 1.73x`, `MERGE · JOIN`, `set_root`, `Tree or DAG`.
- **Shot 4.5 — Establish (Handoff 4 -> 5)** | Cam `(0.8, 2.6, 4.5)` -> `(1.5, 0.2, 0.0)`
  - *Objects:* Node station closing on the single root Node Slab carrying its multi-scored gold accumulator ring.
  - *Key motion:* Root slab slides (`0.9 s`) onto right handoff plate `(2.8, 0.12, 0.0)`.
  - *Mechanism:* Root segment slab and single accumulator hand off to the decider.
  - *Still pose:* Root slab with scored gold ring at `(2.8, 0.12, 0.0)`.
  - *Labels:* `Root proof slab`, `Accumulator ring`, `prog_d`, `input_d`.

*Transition 4 -> 5:* `Follow` move (`1.8 s`) carrying the root slab into Station 5's sealing cradle.

---

### Scene 5: The decider (target 110 s)

**Station layout:** Root slab cradle & `h_lo == 0` zero-stop pin at `(-1.6, 0.20, 0.0)`; `post_d` header extractor claw & terminal interlock bar at `(-0.6, 0.24, -0.25)`; `outcome_d` (`sherlock-value/1`) and `claim_d` (`SHK_RUN`) Poseidon blocks at `(0.2, 0.20, -0.35)`; two-jawed accumulator pairing caliper at `(0.2, 0.34, 0.45)`; vertical brushed-titanium **Privacy Shutter** across `x = +1.15`; four-aperture output dock and `6.4 KB` seal tray at `(+1.9, 0.16, 0.0)`; handoff plate at `(2.8, 0.12, 0.0)`.

- **Shot 5.1 — Establish -> Inspect (Genesis Pin, Header Opening & Running Refusal)** | Cam `(-0.8, 2.0, 1.5)` -> `(-0.8, 0.22, -0.1)`
  - *Objects:* Root slab in cradle, `decider` key card lit on wall (`2`<sup>`20`</sup> rows), `h_lo == 0` pin, split-hexagonal `post_d` token, terminal interlock bar.
  - *Key motion:* Zero-stop pin drops into `h_lo == 0` (`0.3 s`); extractor claw lifts **only** the upper header cap of `post_d` (`0.5 s`) exposing `5` header objects (`outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `body_d`) while `body_d` stays shut. A test `Running` tag hits the interlock and triggers **Refusal** (`errorRed` hatch, `0.5 s`), then swaps to the true `Halted` tag which clears the gate.
  - *Mechanism:* `SEAL` requires `h_lo == 0`, opens only the `5`-field header of `post_d`, and refuses a `Running` state.
  - *Still pose:* `h_lo == 0` pin seated; `post_d` header cap lifted above `5` header fields (`body_d` sealed); `Halted` tag latched past interlock (refused `Running` sample parked aside with red hatch).
  - *Labels:* `h_lo == 0`, `Open post_d header`, `Running refused`, `2²⁰ rows`.
- **Shot 5.2 — Inspect (`outcome_d`, `claim_d` & Closing the Accumulator)** | Cam `(0.2, 2.0, 1.3)` -> `(0.2, 0.24, 0.05)`
  - *Objects:* Opened `Halted` tag + payload, `sherlock-value/1` Poseidon block, `SHK_RUN` Poseidon block, `8`-byte `n_steps` decomposition bar, gold accumulator ring in pairing caliper.
  - *Key motion:* `outcome_tag` and payload hash into teal-rimmed `outcome_d` (`0.6 s`), which seats in the front token carrier alongside `8`-byte-checked `n_steps`; copies of `(prog_d, input_d, outcome_d)` hash under `SHK_RUN` into gold-rimmed `claim_d` (`0.6 s`); then the pairing caliper jaws close once on the accumulator ring (`0.4 s` **Verification** sweep -> solid sealed gold disc).
  - *Mechanism:* Decider computes `outcome_d` and `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`, range-checks `n_steps` (8 bytes), and closes the accumulator in one pairing.
  - *Still pose:* Solid paired gold disc seated in the Decider Seal slab; `outcome_d` and `n_steps` seated in front carrier; `claim_d` resting beside them.
  - *Labels:* `outcome_d`, `claim_d`, `n_steps (8 bytes)`, `One pairing`.
- **Shot 5.3 — Inspect (Privacy Shutter: Four Values Leave)** | Cam `(1.15, 1.7, 2.1)` -> `(1.15, 0.25, 0.0)`
  - *Objects:* Internal bay (`pre_d`, `post_d`, `body_d`, `root`, `tail_d`, `len`, tile addresses), brushed-titanium Privacy Shutter, `4`-pin Decider Seal and `[prog_d, input_d, outcome_d, n_steps]`.
  - *Key motion:* The heavy titanium Privacy Shutter drops (`0.7 s`) between `x = +1.15` and the bench bed, walling off every internal state token, root, tail, length, and address while passing **only** the `4`-pin Decider Seal and `[prog_d, input_d, outcome_d, n_steps]` through precision cutouts to `(+1.9, 0.16, 0.0)`.
  - *Mechanism:* No state digest, memory root, tail digest, length, or address is public; only `[prog_d, input_d, outcome_d, n_steps]` leave.
  - *Still pose:* Closed titanium shutter blocking the left chamber; only the Decider Seal and four colour-coded tokens visible on the right output bed.
  - *Labels:* `State stays hidden`, `[prog_d, input_d,`, `outcome_d, n_steps]`.
- **Shot 5.4 — Inspect (Address-Hiding Value Equality)** | Cam `(1.9, 2.0, 1.0)` -> `(1.9, 0.18, 0.2)`
  - *Objects:* Two small comparison trays behind glassless cutaway cutouts—Tray 1 holds a value at tile address `2` with shared link, Tray 2 holds the equal value unfolded across addresses `5` and `6`—and their two teal `outcome_d` tokens.
  - *Key motion:* Both trays hash under `sherlock-value/1` into teal `outcome_d` tokens that perform the **Equality** coincidence motion (`0.75 s`), merging identical `5 x 5` dot faces.
  - *Mechanism:* `outcome_d` hides memory addresses and sharing: two runs computing the same value publish identical `outcome_d`.
  - *Still pose:* Different tile layouts behind; single merged `outcome_d` face in front.
  - *Labels:* `Different addresses`, `Same outcome_d`.
- **Shot 5.5 — Establish (Handoff 5 -> 6)** | Cam `(0.8, 2.6, 4.5)` -> `(1.6, 0.2, 0.0)`
  - *Objects:* Closed Decider shutter, `6.4 KB` Decider Seal slab and `[prog_d, input_d, outcome_d, n_steps]` sliding onto `(2.8, 0.12, 0.0)`.
  - *Key motion:* Seal and four values slide (`0.8 s`) onto the right handoff plate.
  - *Mechanism:* Address-free seal (`6.4 KB`) and four values hand off to transaction assembly.
  - *Still pose:* Decider seal and four tokens on `(2.8, 0.12, 0.0)`; all four carrier sockets filled in front.
  - *Labels:* `Decider seal · 6.4 KB`, `prog_d · input_d`, `outcome_d · n_steps`.

*Transition 5 -> 6:* `Follow` move (`1.8 s`) carrying the seal and four tokens onto the left (`paper`) half of Station 6.

---

### Scene 6: The transaction (target 60 s)

**Station layout:** Bench ground splits at `x = 0.0` from `paper` (`0xf4f0e8`) on `x < 0` to `graphite` (`0x1c2331`) on `x > 0`. Envelope assembly cradle at `(-1.2, 0.16, 0.0)` holding the `graphite` envelope (`6.4 KB` seal pocket, `4` value pockets, `SherlockRun.acceptRun` call plate, Transcript Locking Bar, `DUST` fee slot); slate handoff plate at `(+2.8, 0.12, 0.0)`.

- **Shot 6.1 — Establish -> Inspect (Envelope & Call Binding)** | Cam `(-1.1, 2.1, 1.5)` -> `(-1.1, 0.18, 0.0)`
  - *Objects:* Open `graphite` envelope on `paper` side, `6.4 KB` seal slab, `[prog_d, input_d, outcome_d, n_steps]`, call plate, Transcript Locking Bar.
  - *Key motion:* Seal and four tokens seat into their milled `wellSlate` pockets (`0.6 s`) and the titanium Transcript Locking Bar slides across (`0.4 s`), engaging four teeth with the four tokens and one key tooth into the seal's `binding` pin.
  - *Mechanism:* Transaction wraps the `6.4 KB` proof, the four public values, and the call to `SherlockRun.acceptRun` under one transcript binding.
  - *Still pose:* Seated seal and four tokens coupled by the engaged Transcript Locking Bar.
  - *Labels:* `SherlockRun.acceptRun`, `6.4 KB proof`, `Transcript binding`.
- **Shot 6.2 — Inspect (Tamper Refusal & DUST Fee)** | Cam `(-1.1, 1.9, 1.1)` -> `(-1.1, 0.18, 0.05)`
  - *Objects:* Envelope pockets, swapped altered `outcome_d` token, Transcript Locking Bar, `DUST` gold-rimmed fee wafer.
  - *Key motion:* `outcome_d` swaps for a different `5 x 5` dot token; the locking bar tooth jams against the seal's `binding` notch, triggering **Refusal** (`errorRed` hatch on seal, `0.55 s`). True `outcome_d` returns, locking bar seats `gold`, and the `DUST` wafer drops into the side fee slot (`0.5 s`), closing the slate cover.
  - *Mechanism:* Changing any of the four values breaks transcript binding and invalidates the proof; fees are paid in `DUST`.
  - *Still pose:* Closed slate envelope with `DUST` wafer latched flush in its side slot (tampered token rejected aside).
  - *Labels:* `Changed value fails`, `DUST fee`.
- **Shot 6.3 — Follow (Crossing Paper to Slate, Handoff 6 -> 7)** | Cam `(-0.8 -> +1.6, 2.3, 4.0)` -> `(-0.4 -> +2.0, 0.16, 0.0)`
  - *Objects:* Closed slate envelope crossing `x = 0.0` seam from `paper` bench to `graphite` chain slab; front token carrier tracking alongside.
  - *Key motion:* Envelope glides (`1.2 s`, `easeInOutCubic`) from `(-1.1, 0.16, 0.0)` across the `x = 0.0` material seam onto `(+2.8, 0.12, 0.0)` while `scene.background` and `FogExp2` color interpolate deterministically (`u = smoothstep(-0.6, 1.2, envX)`) from `paper` `0xf4f0e8` to `graphite` `0x1c2331`.
  - *Mechanism:* Prover work ends on paper; chain verification begins on dark slate.
  - *Still pose:* Slate envelope resting on `graphite` handoff plate `(2.8, 0.12, 0.0)` against dark slate fog; four tokens clear in front carrier.
  - *Labels:* `Prover · paper`, `Chain · Midnight`, `SherlockRun.acceptRun`.

*Transition 6 -> 7:* `Follow` move (`1.8 s`) over dark `graphite` into Station 7's intake dock.

---

### Scene 7: The contract (target 100 s)

**Station layout:** Envelope verifier dock at `(-1.7, 0.20, 0.0)` under pinned `decider` key reader; `claim_d` Poseidon recompute block at `(-0.5, 0.20, -0.2)`; `runs` drawer cabinet at `(+0.9, 0.55, -0.25)`; consumer-seal & `reg_root` foreign-registry sub-stage at `(+0.6, 0.22, +0.75)`; handoff plate at `(2.8, 0.12, 0.0)`.

- **Shot 7.1 — Establish -> Inspect (Verify, Recompute `claim_d`, Store Record)** | Cam `(0.0, 2.2, 2.2)` -> `(0.1, 0.30, -0.1)`
  - *Objects:* Slate envelope open at verifier dock, `decider` key card lit on wall, `claim_d` Poseidon block, `runs` cabinet.
  - *Key motion:* **Verification** sweep crosses the decider seal (`0.4 s`, `goldBright`); `(prog_d, input_d, outcome_d)` hash through the Poseidon block into `claim_d` (`0.5 s`); a matching unoccupied drawer in `runs` unlatches, receives **only** `outcome_d` and `n_steps` inside (`0.5 s`), and locks `claim_d` on its faceplate.
  - *Mechanism:* `acceptRun` verifies the proof, recomputes `claim_d`, confirms absence in `runs`, and stores `outcome_d` and `n_steps` (no state digest).
  - *Still pose:* `runs[claim_d]` drawer latched with `gold` collar holding `outcome_d` and `n_steps` visible in cutaway slot.
  - *Labels:* `Verify proof`, `Recompute claim_d`, `runs[claim_d]`, `outcome_d · n_steps`.
- **Shot 7.2 — Inspect (Replay Refusal Across Partitions)** | Cam `(0.2, 1.9, 1.4)` -> `(0.4, 0.32, -0.15)`
  - *Objects:* Second envelope (proved from the same run under a different leaf cut, carrying a subtle two-cut badge on its tray), `claim_d` Poseidon block, already-latched `runs[claim_d]` drawer.
  - *Key motion:* Second envelope verifies and recomputes `claim_d` with the **identical** `5 x 5` dot face (`0.5 s`), advances toward `runs[claim_d]`, strikes the closed drawer stop pin, and triggers **Refusal** (`errorRed` diagonal hatch across the replay envelope, `0.55 s`).
  - *Mechanism:* `runs` refuses any duplicate `claim_d`, including the same run proved again under another leaf partition.
  - *Still pose:* Latched `runs[claim_d]` drawer intact; second-cut envelope stopped `0.04` units short under `errorRed` diagonal hatch ribs.
  - *Labels:* `Same claim_d`, `Another leaf cut`, `Replay refused`.
- **Shot 7.3 — Inspect (Consuming an Earlier Claim & Foreign Proof via Registry)** | Cam `(0.6, 2.1, 1.9)` -> `(0.6, 0.24, 0.55)`
  - *Objects:* Left side: later run's `SEAL-WITH-DEPS` cradle pulling a copy of `claim_d` and matching its `Halted` `outcome_d` value to the consumer's `input_d`. Right side: `reg_root` registry column, chamfered foreign circuit slab + key card, adapter block, node `FOREIGN` child slot.
  - *Key motion:* Consumer seal coincides `outcome_d` value with `input_d` (`0.5 s`); then the foreign key card proves a Merkle path up to `reg_root` (`0.5 s`), passes through the adapter into a `FOREIGN` claim token, and locks into child slot 1 of a node beside a Sherlock run in slot 0.
  - *Mechanism:* Later runs consume verified claims at seal (`deps_d` / input link); foreign proofs enter through `reg_root` and an authenticated adapter as `FOREIGN` node children.
  - *Still pose:* Consumer `DEP` seal latched on left; foreign slab verified under `reg_root` and seated in node child slot 1 on right.
  - *Labels:* `Consume at seal`, `reg_root registry`, `FOREIGN child`.
- **Shot 7.4 — Establish (Handoff 7 -> 8)** | Cam `(0.9, 2.5, 4.4)` -> `(1.5, 0.25, 0.0)`
  - *Objects:* `SherlockRun` desk, latched `claim_d` drawer carriage sliding onto `(2.8, 0.12, 0.0)`.
  - *Key motion:* Verified `claim_d` record plate (`claim_d`, `outcome_d`, `n_steps`) slides (`0.9 s`) onto `(2.8, 0.12, 0.0)`.
  - *Mechanism:* Accepted contract record hands off to network-wide consensus.
  - *Still pose:* `claim_d` record plate at `(2.8, 0.12, 0.0)`.
  - *Labels:* `claim_d`, `outcome_d`, `n_steps`.

*Transition 7 -> 8:* `Follow` move (`1.8 s`) carrying the accepted claim record into Station 8's ledger spine.

---

### Scene 8: The network (target 80 s)

**Station layout:** Central ledger `9.1.0.1` spine at `(0.0, 0.22, 0.0)`; five instanced `graphiteLight` Midnight validator monoliths in an arc at radius `1.55` (`z < 0`), each holding an identical miniature `decider` gold key card; collection settlement dock `[reg_root, set_root, shape_d]` at `(-1.3, 0.20, 0.65)`; downstream contract and prover reader rails at `(+1.5, 0.20, 0.55)`.

- **Shot 8.1 — Establish -> Inspect (Nodes Verify Under One Key)** | Cam `(0.0, 2.4, 3.2)` -> `(0.0, 0.35, -0.4)`
  - *Objects:* Five Midnight validator monoliths, identical gold key cards on each monolith, central ledger spine.
  - *Key motion:* Parallel motion: all five Midnight nodes simultaneously sweep a `goldBright` **Verification** bar across the transaction (`0.4 s`) under identical copies of the single verifier key, then lock the `claim_d` drawer into the central ledger `9.1.0.1` spine (`0.6 s`).
  - *Mechanism:* Midnight nodes on ledger `9.1.0.1` verify transactions for every program with one fixed verifier key.
  - *Still pose:* Five validator monoliths showing locked `gold` collars; `claim_d` seated in central ledger spine.
  - *Labels:* `Ledger 9.1.0.1`, `Same verifier key`, `claim_d`.
- **Shot 8.2 — Inspect (On-Chain Fact & Collection Statement `[reg_root, set_root, shape_d]`)** | Cam `(0.1, 2.1, 1.8)` -> `(0.1, 0.22, 0.45)`
  - *Objects:* Locked `claim_d` fact (`prog_d`, `input_d`, `outcome_d`, `n_steps`), two downstream reader rails (contract module and next prover seal), and the three-pin collection plate `[reg_root, set_root, shape_d]`.
  - *Key motion:* Downstream contract and prover pins dock onto `claim_d` (`0.5 s`); beside it, a batch seal seats the three-token collection statement `[reg_root, set_root, shape_d]` (`0.6 s`, with zero count pin) and opens one member claim path from `set_root`.
  - *Mechanism:* Any contract or later proof relies on the settled claim as an on-chain fact; run collections settle as `[reg_root, set_root, shape_d]` with no claim counter.
  - *Still pose:* Downstream readers locked to `claim_d`; `[reg_root, set_root, shape_d]` plate seated flush in the ledger spine.
  - *Labels:* `[reg_root, set_root, shape_d]`, `On-chain fact`, `prog_d · input_d`, `outcome_d · n_steps`.
- **Shot 8.3 — Establish (Final Wide Shot of All Eight Stations)** | Cam `(global center, 7.8, 22.5)` -> `(global center, 0.3, 0.0)`
  - *Objects:* Entire `8`-station bench in one frame: Stations 1–5 on `paper` left, crossing at Station 6, Stations 7–8 on `graphite` slate right, the `3` gold key cards on the back wall, and the `4` tokens `[prog_d, input_d, outcome_d, n_steps]` in the front carrier aligned with the settled claim in Station 8.
  - *Key motion:* Camera pulls back along the rail (`2.4 s`, `easeInOutCubic`) as fog density relaxes (`0.016 -> 0.003`) so all eight instruments read sharply end-to-end; then everything holds still.
  - *Mechanism:* Complete proof line from source text to accepted Midnight claim: long execution compressed to a constant-size address-free proof under one key set.
  - *Still pose:* Full `8`-station bench in crisp side-elevation three-quarter view; four carrier tokens (`gateViolet`, `steel`, `resolve`, Vernier `n_steps`) gleaming in foreground.
  - *Labels:* `1–5 · Prover (paper)`, `6 · Transaction`, `7–8 · Chain (slate)`, `[prog_d, input_d, outcome_d, n_steps]`.

---

## 5. Scale devices

Large bounds are drawn with metrological gauges, break jaws, and counters—never by multiplying meshes:

1. **`16` steps against `2`<sup>`64`</sup>:** Station 2 and Station 4 mount an `8`-wheel Vernier counter bar beside an engraved titanium scale strip whose left end shows `16` physical tick notches (matching the `16`-slot leaf slice) followed by a milled zig-zag break jaw and the upper stop engraved `2`<sup>`64`</sup>` − 1`. The viewer sees `16` as one physical caliper bite along a counter range, not a `2`<sup>`64`</sup>-gate circuit.
2. **`8` cells against `2`<sup>`66`</sup>:** Station 3 pairs one literal `8`-tile ceramic segment tray (`1.24` units wide) against an address frontier bar split by a break jaw between `0..7` and `2`<sup>`66`</sup>` − 1`, paired with the `3` appends/step stop (`3 × 2`<sup>`64`</sup>` < 2`<sup>`66`</sup>) showing why memory address space never exhausts before the `64`-bit step counter.
3. **Depth `63`:** The Merkle tree column in Station 3 draws `Tree8` at the base, levels `1` and `2`, a physical `ceramicEdge` zig-zag break coupling, levels `62` and `63`, and `root` at the top, flanked by a vertical brass-and-ink depth ruler marked `0`, `1`, `2`, break, `62`, `63`.
4. **`120` rules:** Drawn literally once in Station 2 as a single `InstancedMesh` of `12 x 10 = 120` ceramic rule keys (`0.09` pitch). One active key in `heatBright` against `119` matte keys makes `1 of 120` immediate at a glance.
5. **"Same size at any height":** Station 4 mounts two fixed titanium caliper jaws (`1.10` units apart) over the output cradle. Whether merging `2` leaves (`32` steps) or two deep segment trees, the ejected slab touches the exact same left and right caliper jaws and carries one `0.20`-radius gold accumulator ring.
6. **"Linear work":** Station 4 places a `45°` machined titanium straightedge behind the press; `1`, `2`, and `4` slice runs stack `1`, `2`, and `4` identical ceramic work blocks whose top corners kiss the straight line.

---

## 6. Build notes

- **Renderer and materials (`THREE.WebGPURenderer` + `?webgl`):** Use `MeshStandardNodeMaterial` from `three/webgpu` with shared cached textures from `theme.js` (`createCeramicGrainTexture`, `createBrushedMetalTexture`, `createStudioEnvironmentMap`). Avoid `ShaderMaterial`, compute shaders, and renderer `clippingPlanes`. Every housing cutaway is a pure `Object3D.position` translation of rigid lid/cowl groups, guaranteeing pixel-identical structure and shadows under both WebGPU and `forceWebGL: true`.
- **Sweep and refusal effects:** Build the Verification sweep bar as a thin unlit `MeshBasicNodeMaterial` box (`goldBright`) translated across the slab's bounding box in `seek(t)`, and the Refusal cross-hatch as a pre-built `InstancedMesh` (`12` diagonal `errorRed` ribs) toggled via `visible` and `position.y`. Zero custom shader risk on `?webgl`.
- **Deterministic `seek(t)` and scene isolation:** Each `sceneN.js` owns a self-contained root `THREE.Group` and pure pose table `poseAt(t)`. Token `5 x 5` dot grids (`25` cylinder instances per token) and slab pin profiles are generated by one shared helper module (`objects.js`) imported by every builder. Scene `N` end-handoff and Scene `N+1` start-handoff call the same factory with identical local plate transforms.
- **Expected draw calls and instance budgets (60 fps on integrated GPU):**
  - *Shared materials:* `9` total (`ceramic`, `titanium`, `gold`, `slate`, `ink`, `violet`, `steel`, `resolve`, `stateEmissive`).
  - *Lights:* `1` shadow-casting directional key (`autoUpdate = false`, updated only when `seek(t)` moves geometry), `2` shadowless directional fill/rim lights, `1` hemisphere light. Zero point lights, zero post-processing.
  - *Draw calls per active scene:* `18` to `34` draw calls (`< 42` in Scene 8 wide shot using LOD simplified station shells).
  - *Peak `InstancedMesh` counts:* Scene 2: `120` rule keys + `64` tape cubes (`4` instanced calls). Scene 3: `64` register cubes + `48` memory/cache/ROM tiles + `16` step slots + `175` digest dots (`7` tokens × `25` pips) = `~310` total instances across `7` `InstancedMesh` calls (`< 35,000` triangles total).
- **First cuts if frame time rises above 12 ms:** (1) Drop shadow map resolution from `2048` to `1024` and freeze shadow updates during Inspect holds; (2) replace bevelled `RoundedBoxGeometry` (`segments = 2`) on register cubes and `5 x 5` pips with 1-segment flat-chamfered low-poly buffers (`68%` triangle reduction, zero loss of silhouette); (3) render non-active stations in Scene 8's final wide shot with merged static hull geometries (`1` draw call per distant station).

---

## 7. Three risks

1. **Scene 3 visual crowding across fourteen leaf modules.**
   - *Failure mode:* Showing `CORE`, `BUS`, `DEC`, `ALU`, `RANGE`, `MEM`, `MEMORY BACKEND`, `PAGE/ROM`, `STACKS`, `VALUE DIGEST`, `BOUNDARY COMMITMENT`, and `INIT` on one board can flatten into an unreadable grey circuit board.
   - *Repair:* Separate modules by physical elevation and material contrast (`titanium` trench sunken at `y = 0.08`, `ceramic` blocks at `y = 0.20`, `gold` Merkle column rising to `y = 2.10`), keep inactive blocks completely matte and still, and frame Shots `3.2`–`3.8` as tight regional Inspect views covering `2`–`3` interacting blocks at a time with at most `4` DOM labels visible.
2. **Confusing the `5 x 5` digest dot face with raw state data.**
   - *Failure mode:* A viewer could mistake the `25` dots on a hexagonal token for `25` memory cells rather than an opaque cryptographic digest.
   - *Repair:* Establish the Poseidon hash motion twice in Scene 1 before any complex state exists—many rectangular/cube inputs enter the gold block, one sealed hexagonal coin with `25` micro-pips comes out—and keep memory cells strictly square `ceramicAlabaster` tiles while digests are strictly colour-rimmed hexagonal coins.
3. **Losing continuity of the four public values during cutaway Inspect shots.**
   - *Failure mode:* Tight top-down Inspect cameras in Scenes 3, 4, and 5 could crop out the front-edge Token Carrier, breaking "follow the two (then four) tokens."
   - *Repair:* Lock the Token Carrier onto a camera-relative lower-foreground shelf rail (`bottom 11%` of the canvas frustum) during Inspect shots so `[prog_d, input_d, outcome_d, n_steps]` remain physically anchored in view across every beat without colliding with the board plan above.
