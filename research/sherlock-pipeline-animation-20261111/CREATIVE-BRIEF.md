# Creative brief: the Sherlock pipeline animation

This file sets the vision, the visual system and the pacing. `BRIEF.md` holds the facts and the technical contract; where the two touch, `BRIEF.md` wins on facts and this file wins on look, motion and tone. Every lane reads both.

## 1. What the piece is

A mechanism-first explainer. A viewer who watches all eight scenes can explain, in their own words, how a Moriarty program becomes an accepted claim on Midnight: what each stage receives, what it checks, what it hands on and why the next stage can trust it.

It is not a promo film. There is no logo reveal, no mood shot and no beat that exists only to look good. Every object on screen is a named thing in the system. Every motion is a named operation. If a shot does not show a mechanism, it is cut.

The narration is complete prose in a DOM panel beside the canvas. The canvas shows the mechanism; the panel says what moves, why it moves and what it proves. There is no voice-over, so every beat lasts as long as its paragraph takes to read.

## 2. The metaphor: the proof line

One straight workbench runs left to right. It has eight stations, one per step. Work enters at the left as a sheet of source text and leaves at the right as a record on the chain. Nothing teleports: every object that a later station uses is carried there along the bench, in view.

Three ideas carry all eight scenes.

1. **Follow the two tokens.** Scene 1 mints two digest tokens, `prog_d` and `input_d`. They stay on screen, in a fixed carrier at the front edge of the bench, through every station. In scene 5 `outcome_d` and `n_steps` join them. In scene 8 the same four objects sit in the accepted claim. The viewer can always point at them.
2. **Long becomes small, and small stays the same size.** The run is a long tape. Leaves cut it into slices and turn each slice into a proof slab. Nodes join two slabs into one slab of the same size. The decider turns the last slab into one seal. The tape can grow to the range of a 64-bit counter; the seal never grows.
3. **Paper to slate.** Stations 1 to 5 are the prover's side: a light paper bench with ceramic and titanium tools. Stations 7 and 8 are the chain: dark slate. Station 6 is the crossing. The change of ground tells the viewer who does the work and who checks it.

The bench is a tool for showing mechanism and not a factory fantasy. No conveyor belts with decorative gears, no smoke, no workers. Stations open up into exploded diagrams when the narration goes inside them.

## 3. Visual system

### 3.1 Palette

Canvas colours come from `PALETTE` in `visualization/src/designers/theme.js`. DOM colours (panel, controls, labels) come from the CSS variables of `visualization/src/styles.css`. The two files hold different values for some names (gold, heat); the canvas uses the `theme.js` value and the DOM uses the CSS value, and no lane mixes them.

Colour carries meaning. One meaning per hue, the same in all eight scenes.

| Meaning | Canvas (`PALETTE`) | DOM |
|---|---|---|
| Ground, prover side | `paper` `0xf4f0e8`, `paperWarm` `0xebe5da` | `--paper` |
| Ground, chain side | `graphite` `0x1c2331`, `wellSlate` `0x161d29` | `--graphite` |
| Structure: housings, rails, frames | `titanium` `0xd8dde6`, `ceramicAlabaster` `0xebe5da`, `ceramicEdge` `0xd5cec0` | `--paper-border` |
| Text and line work | `ink` `0x14171f`, `inkSoft` `0x333c4d` | `--ink`, `--ink-soft` |
| Program: source, program image, ROM, `prog_d` | `gateViolet` `0x818cf8` | `--gate-violet` |
| Input value and `input_d` | `steel` `0x0284c7`, `steelBright` `0x38bdf8` | `--env-scope` |
| Outcome value and `outcome_d` | `resolve` `0x0d9488`, `resolveBright` `0x2dd4bf` | `--resolve` |
| The step in progress: the firing rule, the moving request | `heat` `0xea580c`, `heatBright` `0xf97316` | `--heat` |
| Hashing, digests of state, proofs, keys, the accumulator | `gold` `0xd4af37`; `goldBright` `0xfbbf24` only for the instant a check passes | `--gold` |
| A check that fails, a refused transaction | `errorRed` `0xe11d48` | `--error` |

Memory records are neutral ceramic tiles with ink marks. They take the heat colour only while a step touches them. No other hue is used. Gold on paper has low contrast, so gold never carries text: labels are ink, with a small colour chip.

### 3.2 Materials and light

Reuse the look of the existing stage: paper background `0xf4f0e8` with light fog of the same colour, a warm key light, a cool fill, a soft rim, ACES tone mapping. Matte ceramic for housings and trays. Brushed titanium for proof slabs and rails. Gold only as thin edges, engraved faces and the accumulator ring. Graphite slate for the chain, lit by the same lights, with fog that changes to slate colour during the crossing. No bloom haze, no lens flare, no glass, no particles as decoration. Emissive colour is used for state (the active rule, a passing check) and for nothing else.

### 3.3 Typography

All text is DOM text positioned over the canvas from 3D anchor points. No text is baked into textures. The fonts are the site fonts: Newsreader (serif) for scene titles and the narration paragraphs, Plus Jakarta Sans for label words, JetBrains Mono for identifiers, tuples and numbers.

- An in-scene label has at most four words or one identifier with one value. It never wraps.
- Identifiers are written exactly as in `BRIEF.md`: `prog_d`, `input_d`, `outcome_d`, `n_steps`, `claim_d`, `pre_d`, `post_d`, `rules_d`, `h_pre`, `(chip, key, a, b, c)`.
- Powers are written with a superscript in the DOM: 2<sup>64</sup>, 2<sup>66</sup>, 2<sup>16</sup>, 2<sup>20</sup>.
- At most six labels are visible at one time. A label appears with the object it names and leaves with it.
- No label carries a status word, a hedge or a file path.

### 3.4 Data as objects

Each kind of data has one shape, and keeps it in every scene.

| Data | Object |
|---|---|
| Field element, register, cell | A small rounded cube. |
| Digest | A hexagonal token with an engraved face: a 5 by 5 dot pattern computed from the content it stands for. Equal digests have equal faces. Different digests have different faces. The pattern replaces hex digits, so no digest value is ever invented. The rim colour gives the family (violet program, blue input, teal outcome, gold state or claim). |
| Poseidon hash | One fixed motion in every scene: the input cells line up in their stated order, slide into a short gold block, and one token comes out. Same block, same timing, every time. |
| Bus tuple | A bar of five cubes `(chip, key, a, b, c)`. The first cube shows the chip id: 1 ALU, 2 MEM, 3 HASH, 4 ROM. |
| Memory record | A ceramic tile, one cell of memory. Eight tiles fill a tray: one segment. |
| Memory tree | A column of gold hash blocks above the trays, depth 63. It is drawn with a break mark and a depth ruler, never as 63 literal levels of geometry at once. |
| Machine step | One frame on the witness tape: a rule id plate, the state before, the state after. |
| Witness stream | The tape. Sixteen frames make one slice. |
| Stack (FRAMES, TASKS) | A chain of linked gold tokens. A push hashes the old top into a new link. A pop opens the top link. |
| Proof | A titanium slab with a gold edge. Its public values are pins on the front edge: seven pins on a leaf slab, five values on a node slab, four on the decider seal. Every slab has the same footprint. |
| Accumulator | One gold ring carried on a node slab. Folding adds a mark to the ring and never a second ring. The decider closes it once. |
| Key | A flat gold key card in a wall rack above the bench: leaf, node, decider. The rack never changes when the program changes. |
| Claim | A gold-rimmed token `claim_d`, and a drawer in the contract's `runs` cabinet. |
| Transaction | A flat slate envelope that holds the seal, the four values and the call name. |

Three motions are also fixed for the whole piece.

- **Equality** is physical coincidence. Two tokens that must be equal slide onto the same spot; equal faces merge into one outline. This shows `L.post_d == R.pre_d`, the same `prog_d` in two children, and two runs with the same `outcome_d`.
- **Verification** is a single sweep line across the object, then a short bright-gold edge.
- **Refusal** is a stop, a small return movement and a red hatch on the object. Nothing explodes.

### 3.5 How data travels between scenes

Each scene ends with its output object resting on a handoff plate at the right edge of its station. The next scene begins with the same object on the same plate. The camera follows the object across the gap.

| From | To | Object on the plate |
|---|---|---|
| 1 | 2 | The program image (ROM rack) and the two tokens `prog_d`, `input_d` |
| 2 | 3 | The witness tape, with its outcome frame at the end |
| 3 | 4 | Leaf slabs, each with seven pins `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]` |
| 4 | 5 | The root slab with its accumulator ring |
| 5 | 6 | The decider seal and the four tokens `[prog_d, input_d, outcome_d, n_steps]` |
| 6 | 7 | The envelope |
| 7 | 8 | The stored claim: drawer `claim_d` holding `outcome_d` and `n_steps` |

The token carrier at the front edge of the bench moves with the camera through all scenes. The shell owns the token carrier and the handoff plates. A scene can be rendered alone, so the last pose of scene N and the first pose of scene N+1 each draw the handoff object themselves, at the same plate position and in the same pose.

### 3.6 Motion rules

1. Every pose is a pure function of time. `seek(t)` rebuilds the scene state from `t` alone. No physics, no accumulating particles, no random values without a fixed seed, no effect that needs earlier frames.
2. One focus at a time. One object moves while the rest hold still. The exception is a beat whose subject is parallel work (several provers, several network nodes); then the parallel motion is the subject.
3. Easing is cubic in and out. A travel takes 0.6 to 1.2 s. A check takes 0.4 s. Nothing bounces, wobbles or overshoots.
4. Idle objects are still. No ambient spin, no pulsing glow.
5. Every beat ends in a hold of at least 2 s on a legible still pose. That pose is the frame used by the stepped reduced-motion mode and by screenshots, so it must explain the beat without motion.
6. Large quantities are never shown by object count. 2<sup>64</sup> steps, 2<sup>66</sup> cells and depth 63 are shown with rulers, counters and break marks. Instanced objects are for small exact counts: 16 steps, 8 tiles, 120 rules, 7 pins.
7. Every effect has a WebGL2 path. The page runs with `?webgl` and looks the same in structure; no shot depends on a WebGPU-only feature.

### 3.7 Camera grammar

One camera on a rail parallel to the bench. Three shot types and no others.

- **Establish.** A three-quarter view from above the front edge of the bench. It shows a whole station and its handoff plates. Each scene opens and closes on it.
- **Inspect.** A close, nearly orthographic view, square to the object: top-down for floor plans (the leaf, the memory trays), frontal for racks and cabinets. All diagram beats use it. Text is largest here.
- **Follow.** The camera tracks one object along the bench. It is used for handoffs between stations and for a request on the bus.

A move between two shots takes 1.5 to 2.5 s and never happens while a paragraph's key motion plays. No orbit for its own sake, no shake, no depth-of-field pulls, no speed ramps. In the stepped mode every move is a cut.

The last shot of scene 8 is the only wide shot of the whole bench: all eight stations in one frame, with the four tokens in the claim.

## 4. The eight stations

Lengths are targets. The narration decides the final beat lengths (section 5).

| # | Station | Target | What the viewer must see happen |
|---|---|---:|---|
| 1 | The program | 80 s | Source compiles to a program image: code tables in ROM. The image is hashed to `prog_d`. The input value is digested under `sherlock-value/1` to `input_d`. A second program passes the same key rack: the tokens change, the three keys do not. The program is data, not circuit. |
| 2 | The run | 110 s | The machine opened into its six parts: control, environment, frame stack, task stack, memory, fuel counters. A table of 120 rules; each step lights exactly one. Each step prints one frame on the tape. The 64-bit step counter. The three exits: Halted with a value, Rejected with a code, Exhausted with a kind. |
| 3 | The leaf | 240 s | The tape is cut into 16-step slices. One slice enters the leaf: one circuit, one key, a floor plan of modules. CORE asks, the bus carries a tuple, a provider answers. Appends fill the 8-cell tail; a full tail becomes a segment and is hashed into the depth-63 tree. Old reads open against the pre-state root. Stacks are hash chains. The boundary digest in header and body form gives `pre_d` and `post_d`. The genesis leaf proves Init at height 0. Seven lanes come out. Two different cuts of one tape end in the same memory state. |
| 4 | The node | 130 s | Two slabs enter one node. Three equalities snap: `prog_d`, `L.post_d == R.pre_d`, `L.h_hi == R.h_lo`. One slab of the same size leaves, with one accumulator ring. The height counter and its range. Work that grows in a straight line with the number of steps. Many provers at once; trees and DAGs of any shape; MERGE and JOIN. |
| 5 | The decider | 110 s | The root slab is sealed once. Height starts at 0. The header of the final `post_d` opens to five fields. A running state cannot seal. `outcome_d` is computed; `claim_d` is computed. The ring closes in one pairing. Exactly four values leave; everything else stays behind a closed shutter. Two runs with the same value show the same `outcome_d` face. |
| 6 | The transaction | 60 s | The seal and the four values go into an envelope addressed to `SherlockRun.acceptRun`. The call transcript ties the four values to the proof: change one and the proof no longer verifies. The fee is paid in DUST. The envelope crosses from paper to slate. |
| 7 | The contract | 100 s | The contract verifies the seal with its verifier key, recomputes `claim_d`, looks in `runs`, refuses a claim that is already there (also when the run was proved under another cut), and stores `outcome_d` and `n_steps` under `claim_d`. A later run consumes the claim at its own seal. A proof from another system enters through the registry. |
| 8 | The network | 80 s | Midnight nodes verify the transaction, each with the same key for every program. The claim becomes a fact that contracts and later proofs rely on. Collections of runs settle as one statement. The camera pulls back over the whole bench. |

Total target: 910 s, about 15 minutes at normal play. The viewer can pause, step by beat and scrub.

Scene 3 builds on the earlier leaf chip view (`visualization/src/leaf/leafChip.js`): its floor plan of chips on one board, its packet that travels a route on the bus, its trays for the 8-cell tail. It reuses those layout ideas in 3D. It does not reuse that file's facts: its rule set, register file and genesis are an older model. The facts are those of `BRIEF.md`.

## 5. Pacing

- Reading speed sets time. A beat lasts its narration word count divided by 2.5, in seconds, rounded up, and never less than 12 s.
- A beat holds one idea and one key motion. Its narration is 30 to 90 words, so a beat lasts 12 to 36 s.
- Exhaustive detail wins over the length targets. When a mechanism needs more words, add a beat. Never compress two mechanisms into one beat, and never drop one to meet a length.
- The key motion of a beat starts within the first 2 s and ends before the last 2 s. The final hold is the still pose.
- Scenes follow one rhythm: establish (what arrives), inspect (how it works, most of the beats), establish (what leaves).
- Scene 3 is the longest and the densest. Scenes 6 and 8 are the shortest.

## 6. Page register

These rules apply to every word a viewer can read: narration, labels, titles, controls, tooltips, alt text.

- Present tense. The system is built and running. The page describes what it does.
- No status language: never "designed", "planned", "proposed", "estimate", "pending", "roadmap", "landed", "will", "would", "future", and never "open" as a status. The verb "open" (to open a digest, a Merkle opening) is a mechanism term and is used.
- No hedges: no "roughly speaking", "in principle", "should", "may". State what happens.
- No making-of language: no agents, lanes, councils, audits, experiments, findings, decision ids, profile names, file paths, crate names, line numbers or commit ids.
- Only the choices of `BRIEF.md` section 2 appear. No alternative backend, no alternative boundary, no comparison with a rejected option.
- Only the numbers of `BRIEF.md` sections 2 and 3 appear. Where they give no number, the quantity is shown without one ("a fixed number of cache lines per leaf") and is not flagged.
- Plain technical English. Short declarative sentences. Each paragraph says what moves, why it moves and what it proves. No slogans, no rhetorical questions, no second-person pep.
- A term is introduced once, with its meaning, and then used with the same spelling everywhere.

## Amendment: Material Design 3 (owner instruction, 2026-11-11)

The owner set Google Material Design 3 as the design system of the website. The page chrome follows M3 exactly (BRIEF.md section 6). The visual system above keeps its metaphor, camera grammar, objects and pacing; its palette maps onto the M3 tonal palettes generated from one seed (the main accent above), and the paper and slate stages map to M3 surface roles of the light and dark schemes. Motion in the chrome uses the M3 emphasized and standard easings. The mapping lives in `visualization/src/pipeline/SCENE-API.md`.

## Amendment: dark only and impeccable (owner decisions, 2026-11-11)

The page uses the Material 3 dark scheme only. The paper and slate stages become two dark surface tones: the prover side (stations 1 to 5) on a lighter dark surface (surface-container-high), the chain (stations 7 and 8) on the deepest (surface-container-lowest), with station 6 as the crossing between them. The page follows the `impeccable` skill: `visualization/PRODUCT.md` and `visualization/DESIGN.md` are the design authority.
