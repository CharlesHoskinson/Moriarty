# Iteration 000: scalar literal step

- Date: 2026-10-16 UTC
- Author: Claude Opus 5.5 (`claude-opus-5-5`). This is a planning note for this log. The attribution belongs to the note, not to any proposed website copy.
- Source authorship: the Astra high implementation lane wrote the emitter. Root hand-derived the normal fixture before the emitter existed.

## Source pins

Paths are relative to this folder; `../../` is the repository root. The files are an uncommitted candidate in the implementation worktree and are not on `main`. They are therefore shown as code rather than links. Root will record the branch and commit at publication, and links can follow.

| Path | SHA256 |
|---|---|
| `../../crates/mori-backend/src/sherlock_step/mod.rs` | `7cefbcfce5f36b1fef1363a5979922e31185c3771cb1b888a9769e4ea5839771` |
| `../../crates/mori-backend/src/sherlock_step/normal_tests.rs` | `6800ea0761baccf63791ce601647cc71a1018db8c2c6c3e30e86c10e9303e813` |
| `../../models/sherlock/step-slice/README.md` | `853e431755b496df69b7f0803455b09d6fa4b858681a3eb3777f177cad80aef5` |
| `../../models/sherlock/step-slice/independent-normal-expectations.json` | `358e34422f4e2487832a5afd5768cb4849a0f0247052a3163a0a204a6ada7990` |

Other pins:

- Profile: `sherlock-k-a/1`
- Layout: `sherlock-scalar-literal-step/1`
- K source: `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`
- ZKIR API source: midnight-ledger `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, ZKIR `3.0.0-rc.2`

## Status at this iteration

Construction source exists, but the module is unregistered: `crates/mori-backend/src/lib.rs` does not yet declare `pub mod sherlock_step;`.

The following are UNRUN:

- compilation
- the five normal tests
- SDK load, typecheck and constraint checks
- native checks
- K and Quint comparison
- Clean comparison
- target acceptance

The two fixture cases are hand-derived normal expectations, not circuit verification. They are UInt 7 with fuel 1 and stack limit 0, and Bool false with fuel 3 and stack limit 4. Security review is pending, and the shared B machine format is not frozen.

## What the source gives a board

These observations come from reading `emit_scalar_literal_step`; no tool produced them.

- Every public field follows one motif from `Emitter::public`: an unguarded Native `PublicInput`, then a `ConstrainBits` at the field's width, then an `Impact` with guard 1. The 30 fields keep transcript order: 0–7 program header, 8–18 before state, 19–29 after state.
- The header is one immutable program row. `ConstrainEq` pins seven of its fields to immediates. `program.literal` is the only free header field and the single source of the literal value.
- Each side computes `Add(remaining_fuel, consumed_work)` into `accounted_fuel` and equates it with `initial_fuel`. Each side also pins environment length, frame count and depth to 0. Five after = before framing equalities follow, and the before outcome is pinned to Pending.
- `EvalLiteral` (K line 102, charge 1):
  - control tag goes 0 → 1
  - the literal is copied into `after.control_payload`
  - consumed work goes 0 → 1
  - `Add(after.remaining_fuel, 1)` produces `fuel_before`, which is equated with `before.remaining_fuel`; that `Add` is the charge
- `PublishReturn` (K line 80, charge 0):
  - control goes 1 → 2
  - the literal is carried from `before.control_payload` to `after.outcome_payload`
  - outcome goes 0 → 1
  - fuel and work are copied unchanged, with no charge part
  - the UInt 7 case publishes at remaining fuel 0
- `Output` returns the eleven after fields.
- `ScalarKind` and `StepRule` are static choices, so there are four distinct boards. Payload pads are 128 bits for UInt and 1 bit for Bool.
- UInt 7 and UInt 9 produce identical IR (`contrasting_uint_programs_use_identical_constraint_source`). The value sits on a pad and is never part of the copper.
- By hand count, `EvalLiteral` emits 125 instructions and `PublishReturn` emits 124. This count only sizes a scene and is not tool output.

## Approaches

**A. Layered board.** Each relation becomes one PCB with a stack-up: header strip, before plane, after plane, rule-constraint layer, and a transcript plane underneath that receives the `Impact` vias. Every instruction gets a physical place, and peeling layers is natural. The risks are density, and the program row reading as just another region unless it is designed as a header.

**B. State slabs.** One fixture case's three states become slabs, and the rule boards are bridges between them. This tells the normal trace at a glance. However, it duplicates the program row across slabs, which contradicts the source's single immutable row, and it pushes instruction detail off-stage.

**C. Ink schematic.** An orthographic 2.5D drawing, with light only on selection. It is readable and closest to the current site, but it does not answer the luminous PCB brief. It is the right fallback.

## Recommendation: the ledger board

Proposed: A as the scene, B's state sequence as a camera path for stepping, and C as the static rendering of the same data.

**Hero.** The ground is deep ink, viewed with a low raking camera. Two boards of the same scalar kind sit side by side: `EvalLiteral` on the left, `PublishReturn` on the right. Most of the copper is identical, so the eye goes to where they differ. On the left, a small two-input part sits on the fuel rail in warm copper, and that part is the charge. On the right, the same rail runs straight across with nothing on it. The wordmark is a flat page element and never appears in the scene.

**Camera.** Overview uses perspective with slight pointer parallax. Detail dollies into an orthographic view, so labels stay square and figures stay tabular. Bookmarked framings: overview, program row, rule region.

**Materials.**

- Substrate: near-black, with a faint woven-fibre normal map.
- Traces: brushed copper with low emission.
- Silkscreen: small monospace caps in ink `#f0ede4`.
- Pads: edge fingers numbered 0–29 in transcript order.
- Vias: drilled rings that show through to the transcript plane.

**Proposed palette.**

| Role | Colour | Origin |
|---|---|---|
| Ground | `#07080a` | New |
| Substrate | `#14130f` | Existing dark paper |
| Fuel, work and charge | `#e8724a` / `#9a3412` | Existing copper accent |
| Literal and control routing, selection | Cyan near `#5ad1d8` | New |

The cyan's contrast against the substrate needs measurement before adoption. Copper stays the scarce, meaning-bearing hue.

**Light.** Light is split into channels with distinct meanings:

- Ambient light breathes along board edges. It carries no meaning and never rides a trace.
- Selection light is cyan on the operand set of the chosen instruction.
- Example light appears only when a fixture case is loaded. It shows that case's values as tabular figures on the pads, captioned as hand-derived normal expectations.
- Measured light, for proof or runtime activity, is reserved and has no rendering until a forwarded result exists.

`ConstrainEq` is symmetric, so its selection light rises from both ends and stands on the trace instead of flowing. Only `Add` shows direction: two operand traces in, one named output net out.

**Mood.** Wet ink under a desk lamp at night; copper leaf in raking light; the restraint of a single oscilloscope phosphor trace; gold edge fingers on a memory module; ledger rules seen through smoked glass.

## Mapping

| Source construct | Board element | Meaning | Anchor |
|---|---|---|---|
| `PublicInput` | Edge-finger pad at position 0–29 | Public transcript slot, Native | `Emitter::public` |
| `ConstrainBits` | Pad width with gauge label (1, 2, 32, 128) | Range | `Emitter::public` |
| `Impact` | Via to the transcript plane | The instruction with guard 1; no binding judgment drawn | `Emitter::public` |
| `ConstrainEq` to an immediate | Trace ending in a stamped constant fiducial | Field pinned | `Emitter::equal`, `constant` |
| `ConstrainEq` between wires | Undirected copper trace | Equality | `Emitter::equal`, `variable` |
| `Add` | Two-input part with a named output net | Sum wire | `Emitter::add` |
| `Output` | Eleven-pin header on the after edge | Returned after state | Final `Output` push |
| Program row | One silkscreened strip, drawn once | Immutable code header | First loop of `emit_scalar_literal_step` |
| Literal copy | Cyan trace from `program.literal` | A copy, not a computation | `match rule` arms |

## Interactions

- **Hover a pad:** shows name, offset, width and anchor, plus the value when a case is loaded.
- **Select an instruction:** lights its operands and opens the exact Rust lines alongside the K rule line from `StepRule::k_rule_start_line`.
- **Step:** walks a fixture case S0 → S1 → S2, moving from the `EvalLiteral` board to `PublishReturn`. Remaining fuel reads 1, 0, 0 for UInt 7 and 3, 2, 2 for Bool false, with charge 1 and then charge 0 written beside each transition.
- **Peel layers:** toggles for header, before, after, rule constraints and vias.
- **Kind toggle:** narrows payload pads from 128 to 1 bit and moves nothing else.
- **Literal field:** entering 9 in place of 7 changes the pad figure, leaves the board untouched, and cites the test.

## Readability and access

Detail mode keeps all text in a DOM overlay and never rotates it.

Keyboard:

- Tab enters the board.
- Arrows walk positions 0–29.
- Enter selects; Escape clears.
- Brackets step through the case.
- Focus uses the existing copper `:focus-visible` ring.

Reduced motion removes parallax and light travel; selection becomes static colour, and steps cut rather than animate.

The static fallback is an SVG schematic plus the field table, generated from the same scene data, so the page reads completely with scripts off. Charged and free transitions are always labelled, never shown by colour alone.

Layout is three columns at the existing page width: anchored source, board, and explanation. A register-style field table sits below, and status lives in its own strip as `VOICE.md` asks. Disclosure moves from overview to rule region to a single instruction to its source line.

## Scene data ideas

Generate the scene from the emitted `IrSource` rather than drawing it by hand. The scene would hold the instruction list, operand nets, public positions and widths, stamped with the source SHA256 and layout id. That extraction would be a reviewed later step; nothing ran here.

- The viewer compares the stamped hash with the page's pin and shows a stale board on mismatch.
- Instance pads, vias, fiducials and parts by kind, and batch traces per layer.
- Use three distance tiers: layer buses when far, nets in the middle, labels and figures when near. The tiers matter little at this size and more once row-table gadgets arrive.

No performance figure is claimed.

## Open notes

- On narrow screens, one board with a rule switch may read better than two side by side.
- Adding cyan next to the one-accent rule needs a design decision. Copper-only emission is the conservative alternative.
- `Impact` needs a caption that names the instruction without implying a transcript-binding judgment; that judgment remains with security review.
- The typed bounded-row fetch gadget will likely need a table-shaped component.
- Recursive calls have no source yet, so any nested-board imagery stays labelled conceptual.
