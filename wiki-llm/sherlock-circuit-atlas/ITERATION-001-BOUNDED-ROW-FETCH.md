# Iteration 001: bounded row fetch

- Date: 2026-10-16 UTC
- Author: Claude Opus 5.5 (`claude-opus-5-5`). The attribution belongs to this planning note, not to any proposed website copy.
- Source authorship: the Astra high implementation lane wrote the module, tests and mapping. Root wrote `expected.json` independently; its `author` field records that Root had not read the circuit source.
- Iteration 000 stays frozen; this note extends it by reference.

## Source pins

Paths are relative to this folder; `../../` is the repository root. The files are uncommitted additions in the Astra worktree `.worktrees/sherlock-astra-step-20261015` on base head `1fb794d1598e19fe34b9735f90996fc316b743fa`. The forwarded manifest names no branch. Nothing is on `main`, so paths are shown as code, not links.

| Path | SHA256 |
|---|---|
| `../../crates/mori-backend/src/sherlock_machine/mod.rs` | `56e58bc06a104a433d6f9d323a0b9dbb651df96709f20831bc5efb71eab33015` |
| `../../crates/mori-backend/src/sherlock_machine/fetch.rs` | `df1d55b458f7d388c4c1178c88b6899d6111af34f8de51ae4533ab099bbadd62` |
| `../../crates/mori-backend/src/sherlock_machine/normal_tests.rs` | `e4262f82395bb10e2c9b6433305cd82ea05b800d7f0f6e5cf6defe1824dab49f` |
| `../../models/sherlock/machine-circuit/MAPPING.md` | `40ece6255e592595b379a148708906b4a4099ac0560764838e9af06131b5d296` |
| `../../models/sherlock/machine-circuit/expected.json` | `1c0319aa6c447599eadbfb2020a60d66effc01001282470429d610e3d5867ee7` |

The author report bundle was forwarded alongside: `WORK_REPORT.md` `99a737b7af0cbab3d8a6f614f146d9320084f84485bcbc10bfd6cba797440aeb`, `SOURCE-MANIFEST.json` `e9ea0c853c8b404689f4186f0df02b974c4e55a38644052a1421825067de731d` and `STATIC-CHECKS.json` `7769f83b08d87c415a6b11e0a6729c194b42a05145df3d4777aad664400bf966`. The report names a `SOURCE.patch` that was not forwarded and is not cited. The SDK pin is the workspace Midnight revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`.

Symbol anchors: `TableShape::new`, `emit_fetch_block`, `Builder::less_than`, `constrain`, `emit_public_fetch`, `EmittedFetch::pack_witness`, `FieldRole`, `FetchBlock::preserved`.

## Status at this iteration

- The module is unregistered: `crates/mori-backend/src/lib.rs` declares no `sherlock_machine`, and Root owns registration.
- UNRUN: compilation, the four normal construction tests, serialization, SDK load and check, evaluation, native checks and proving. Native authority is 0.
- Executed: text and metadata checks only, as listed in `STATIC-CHECKS.json`. They are not parsing, typechecking or SDK validation.
- The shared full machine format is pending review. All eight `Snapshot` fields sit outside this generic block, and no machine transition is claimed.
- Binding, security and resource admission are open.

## What the source gives a board

These observations come from reading `mod.rs` and `fetch.rs`; no tool produced them.

- **Shape.** The caller supplies capacity N and ordered columns C, both nonzero, with nonempty distinct names and kind UInt or Bool. Overflow checks are construction errors, not admitted bounds.
- **Inputs.** Row-major: 0 `active_rows`, 1 `index`, then `2 + r*C + c` per cell. Each is a Native `PublicInput` with guard None named `row_fetch.input.{position}`, then a range constraint, then `Impact` with guard 1. UInt uses `ConstrainBits` 128; Bool uses `ConstrainToBoolean`.
- **Repeated ranges.** The fragment constrains every input again, so standalone pads carry two range checks.
- **Guards.** `LessThan(N, active_rows)` is constrained Boolean and equated to 0. `LessThan(index, active_rows)` is constrained Boolean and asserted. Both use 128 bits.
- **Decoder.** For each row r, `TestEq(index, r)` is constrained Boolean and added into a running sum seeded at 0; the final sum is equated to 1.
- **Lanes.** Each column runs a `CondSelect` chain seeded at immediate 0 through rows 0 to N−1, with `bit` the row selector, `a` the cell and `b` the accumulator. Only the final value receives the column's range constraint. `MAPPING.md` states the seed is construction data, not a default row.
- **Outputs.** C selected values, then every input operand in input order: C + 2 + N·C Native outputs. Preserved outputs are the same operands, not copies.
- **Temporaries.** `{scope}.temp.{n}`, 2 + N(C + 2) of them. Reading the 3×2 wrapper: temp.0 and temp.1 are the guards, temp.2/4/6 selectors, temp.3/5/7 sums, temp.8–10 the amount lane, temp.11–13 the enabled lane. A hand count gives 57 instructions, for scene sizing only.
- **Fixture.** amount:UInt and enabled:Bool, rows (7, true), (9, false), (12, true), `active_rows` 3. Packed inputs are `[3, index, 7, 1, 9, 0, 12, 1]`, and a test asserts identical IR across cases.
- **Second shape.** A test builds the fragment alone over 2 rows of ready:Bool, left:UInt, right:UInt with caller-named operands such as `caller.count` and `a.ready`, and checks it emits no `PublicInput` or `Output`.

## Approaches

**A. Array board.** Selectors become word lines across rows; each column becomes a lane with a `CondSelect` at every crossing. The geometry is the instruction structure and grows predictably with N and C. The chain must read as a sequence rather than a tree, and dense shapes need distance tiers.

**B. Functional strata.** Guards, decoder, lanes and return bus float as separate plates joined by vias. Grouping is clear and explode is natural, but each selector is pulled away from the row it drives.

**C. Table-first schematic.** A flat register table with lanes drawn underneath in 2.5D. It is the most readable and closest to the ledger, and the least luminous.

## Recommendation: the array board

Proposed: A as the scene, B as its exploded state, and C as the static and narrow-screen rendering, all generated from one scene record.

**Working title and hero.** Working title: *Bounded row fetch*. Proposed hero line: Reading one row from a bounded table. A low raking camera crosses a dark board where three word lines meet two lanes, one wide (UInt128) and one narrow (Bool). The flat wordmark stays on the page.

**Layout.** The board lies in X–Z. Row r is a strip at depth r, back to front. Lane c runs forward at column c, with its seed fiducial behind row 0 and its selected pin on the front edge. The decoder sits at negative x: an index bus down the side, `TestEq` taps stamped 0, 1, 2, and a sum spine ending in a terminator stamped 1. The guard block sits at the back left beside the `active_rows` and `index` pads; one comparator carries a stamped N, the other ends in an Assert pad. Layers, top to bottom: public frame (fingers, `Impact` vias, output header), table pads, decoder and guards, lanes, return bus.

**Scene graph.** Nodes are keyed by SDK identifier (`row_fetch.input.4`, `row_fetch.temp.9`) or, for fiducials, by immediate value. Node kinds are input pad, immediate fiducial, range gauge, comparator, equality terminator, assert pad, `TestEq` tap, `Add` part, `CondSelect` part, seed fiducial, output pin and via. Edges carry the operand role from the instruction field (`a`, `b`, `bit`, `cond`, `val`, `inputs`, `vals`) plus `output` from part to net. Computed outputs are directed; `ConstrainEq` stays undirected, as in 000. A preserved output is one net with two terminals, so hovering either end lights both.

**Light and colour.** The channels from 000 carry over: ambient edge breath, cyan selection, example figures, and measured light reserved. New proposal: copper emission marks the obligations that make the selection unique, namely the capacity terminator stamped 0, the Assert pad and the sum terminator stamped 1. Lanes and word lines stay unlit copper until selected.

**False and zero.** `false`, `0` and index 0 render as ink figures at full weight, and a Bool cell shows its packed scalar beside it (`false · 0`). The seed is a diamond fiducial labelled seed 0, never a row-shaped pad, so a data zero cannot be mistaken for the seed.

**Result card.** A flat DOM card beneath the board gives each output position three rows: *Supplied* (the case's input pads), *Expected* (Root's independent hand-derived data, copied verbatim) and *Actual* (UNRUN). Selected positions cite `cases[].selected`, positions 2 and 3 cite the comparison rule in `MAPPING.md`, and positions 4–9 cite `preserved_input_rows`. The browser never picks a row or computes a selected value.

## Mapping

| Source construct | Board element | Anchor |
|---|---|---|
| `active_rows`, `index` (positions 0, 1) | Two wide pads in the guard block, UInt128 gauge | `emit_public_fetch`, `FieldRole` |
| `rows[r][c]` (position 2 + r*C + c) | Cell pad at row r, lane c, tabular figure | `FieldRole::Cell` |
| Repeated range constraint | Two gauge ticks per pad, each anchored | `constrain` in wrapper and fragment |
| `LessThan(N, active_rows)` then `= 0` | Comparator with stamped N, copper terminator 0 | `emit_fetch_block` |
| `LessThan(index, active_rows)` then `Assert` | Comparator ending in an Assert pad | `emit_fetch_block` |
| `TestEq(index, r)` | Decoder tap stamped r driving word line r | selector loop |
| `Add` sum and `= 1` | Sum spine, copper terminator 1 | selector loop |
| `CondSelect` | Three-terminal part at each crossing: `bit` from word line, `a` from cell, `b` from upstream | column loop |
| Seed `immediate(0)` | Diamond fiducial at the lane head | column loop |
| Final lane range | Gauge on the selected pin only | column loop |
| Selected outputs 0 to C−1 | Front header pins under each lane | `Output` |
| Preserved outputs | Same net continuing on the return layer to pins C onward | `FetchBlock::preserved` |
| `Impact(1, [operand])` | Via under each finger, captioned by instruction name only | `emit_public_fetch` |
| `pack_witness` | Case loader filling pad figures; board unchanged | `EmittedFetch::pack_witness` |

## Motion and interaction

- **Camera.** Bookmarks run overview, guards, decoder, lanes, return bus, result card. The last move eases into orthographic so figures stay square.
- **Hover.** Shows identifier, role, input and output positions, kind and symbol anchor, plus the supplied figure when a case is loaded.
- **Select.** Lights the operand set in cyan and opens the Rust lines, resolved from the pinned bytes.
- **Case.** Keys 1–3 load the first, middle and last row cases. Pads and the card change; copper never moves.
- **Scrub.** A slider labelled *Instruction order* walks instructions in emission order. It is source order, not execution time, and nothing travels along traces.
- **Explode.** Separates layers toward B; the return bus drops to show alias nets running under the board.
- **Shape.** Switches to the second test shape, fragment only, with no public frame.

Motion is limited to camera moves, explode and selection fades. Reduced motion cuts between states.

## Keyboard and fallback

Tab enters the board. Arrows walk the cell grid; Home and End jump to the guard block and the output header. Brackets scrub, 1–3 load cases, E explodes, Enter selects and Escape clears. Focus uses the copper `:focus-visible` ring. The static fallback is an SVG array plus the input map, output map and instruction listing, so the page reads completely with scripts off.

## Carryforward and next questions

- The PublicInput, range and Impact motif from 000 recurs, built inline in `emit_public_fetch`. Because Bool uses `ConstrainToBoolean`, its gauge needs a Bool glyph rather than a 1-bit label.
- `MAPPING.md` writes positions 0..2 and 4..10 as half-open ranges while `WORK_REPORT.md` lists cells 0 and 1 and 4 through 9. The UI should list explicit positions.
- A machine board embedding this fragment has no source. Any socket imagery stays labelled conceptual and draws no `Snapshot` field.
- Larger N×C needs distance tiers: lanes as single strokes far, parts mid-range, identifiers near.
- Copper obligation emission and the cyan from 000 both need contrast measurement and design review.
- When a recorded normal run arrives, *Actual* fills from that record and measured light gets its first rendering rule.
