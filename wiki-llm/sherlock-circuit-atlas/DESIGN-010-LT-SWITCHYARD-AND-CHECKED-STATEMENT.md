# DESIGN010 — Less-than switchyard and the checked statement

- **Date:** 2026-10-19 UTC
- **Kind:** Creative concept note for the future Sherlock circuit site. It adds no circuit iteration, website code, build, release or deployment. It runs alongside implementation and blocks nothing.
- **Author:** Opus 5.5 (`claude-opus-5-5`), the requested Opus HIGH concept author for this entry.
- **Launch condition:** Unchanged. The site goes live only when all of Sherlock is finished.

## Continuity

This note keeps the DESIGN008/009 board: ink ground, a flat unlit Moriarty wordmark, copper accent `#e8724a`, a measured 4px grid, and tabular monospace labels that never reflow. Lt mounts as a new bay south of the Sub ladder.

Lt rhymes with Sub, with sixteen byte lanes and a borrow ladder. Its keyed sockets still read `Lt`. Its result is one Bool rail, not a value. It has no overflow gate: its only error socket is 11. Add, Sub, Mul and Lt stay distinct carriers, and Eq is not drawn in this bay.

## Recommendation: three linked boards on one spine

**Board 1: Comparator** (`b_scalar_less_than.rs`, `emit_b_scalar_arithmetic_lt`)
- **Operand rack (A01-A08).** Two shelves, Const and Data, are chosen by `operand_b.value.space`. One `index` probe pulls a row.
  - A tag gauge accepts kinds 0..=5 only.
  - `b_is_uint` lights the UInt key.
  - On a non-UInt row, all 16 `read_n` bytes are shown pinned to zero.
- **Borrow ladder (A11).** Sixteen rungs pair `reg_a[i]` (left) with `read_n[i]` (right), little-endian, with byte 0 at the west end.
  - Each rung forms `need = read_n[i] + borrow`, asks `reg_a[i] < need`, and passes `borrow_chain[i]` east.
  - The last rung is the switch point, `is_less`. In `emit_public_b_scalar_arithmetic_lt` its output role is `Underflow`, and the label shows both names.
- **Staging switch (A12-A13).** UInt goes to `BoolValue(is_less)`. A valid non-UInt goes to `PrimError(11)`. The staged `u128_value` bytes are held at zero.
- **Conduits and ticker (A14-A17).** Helper lanes run as straight, via-free pre→post conduits. The counter ticker reads `[-1,+1,+1,+1]`.

**Board 2: Bool dock** (`b_scalar_less_than_completion.rs`)
- **Exit (`emit_b_scalar_exit_lt_bool`).**
  - When applied, an append mark writes `DataRow::Bool` at physical `pre.active.data`, chosen by one-hot `write_selectors`.
  - The logical `alloc_data_nodes` counter sits beside the physical index and is never merged with it.
  - On cap refusal, a held pad shows `Exhausted(DataNodes)` and the helper is retained.
- **Failure (`emit_b_scalar_failure_lt`).** The socket accepts code 11 only and writes no Data row.
- **Typed adapters.** These are drawn as via-free pass-throughs: `lt_helper_to_common`, `BLtPublishState::to_admin_publish_state`, `to_legacy_handoff`, and the inverse `BLtLegacyHandoff::to_admin_publish_state`. The inverse refuses fewer than seven rest lanes. `mode` keeps Lt ordinal 3 throughout.
- **Shield.** `emit_b_lt_completion_complement` is a glass shield over site, control, every frame lane and the cursor bytes.

**Board 3: Return yard** (`b_return_right_less_than.rs`)
- **Entry.** `emit_b_lt_return_right_typed_block` takes the dock's `BLtPublishState`, crosses via `to_legacy_handoff`, and drives `emit_k_machine_return_right_lt_block`.
- **Frame sidings.**
  - The top siding must be `RightF` with `right_op == BINARY_OP_LT` (4).
  - Only that siding lifts off, and its `origin` becomes `post.control_origin`.
  - Sidings below are locked, slots above are zeroed, and `depth` drops by 1.
- **Result transfer.** A Data-space `ScalarValue` or a `ScalarError(11)` becomes `PrimResult`, `post.site = Prim`, and the helper result is cleared.
- **Shield handoff.** The yard's own shield, `lt_rr_complement_operands`, covers the outcome payload and Data rows but not frames. The dock freezes frames and the yard releases exactly one. The source states why: reusing the completion complement here "would incorrectly freeze frames."
- **Ticker.** T-LEGACY reads `[-1,+1,0,0]`, the cursor moves +1, and `task_live = 0` when enabled.

**Outer chassis.** The three boards sit inside an outline chassis with labelled sockets:
- caller dispatch and a mux of exactly one enabled row;
- comparator disabled-row stutter;
- global admission, StepInv and SI-21 freshness;
- per-IrSource ownership of `%{scope}.local.temp.`, `%{scope}__` and `%{scope}_typed__`;
- arena and physical packing;
- untouched-image framing.

No board extends into the chassis, and no loop from the yard back to dispatch is drawn. The local wrapper is never presented as the whole machine.

## Alternatives

- **B. Ladder close-up.** The sixteen rungs dominate, with the dock and yard as thumbnails. It is legible, but it hides why frames matter. Keep it as a zoom level.
- **C. Frame tower.** A vertical stack of caller frames carries the comparator at its crown. It is striking, but it buries the operand rack.

**Choose A**, with C's siding stack inside the yard and B as the deepest zoom.

## Interactive explanations

**Why reading the operand matters ("Pick the shelf").** Flipping the space key moves the probe to the other shelf. The tag gauge, `b_is_uint` and the sixteen right-hand lanes re-route their highlighted paths. The toggle shows topology only and computes no values.

> The comparison is only as true as the number it reads. The circuit names the exact row, checks what kind of thing lives there, and refuses to treat a non-number as a number.

**Why preserving caller frames matters ("Return to sender").** This is a three-step walk-through:
1. Highlight the waiting frames.
2. Lift only the top `RightF`.
3. Show the locks on every frame below.

> The question was asked on someone's behalf. The answer goes back to that exact caller, with everything they were holding still in hand.

## History room: the checked statement (dated 2026-10-19)

This is a separate gallery with no wire to the Lt bay. Its subject is the A102 Literal fixed fixture.

- Eight cards lie on a light table. One genuine statement is accepted and seven corrupted variants are refused.
- Beneath the cards runs a 2077-lane strip (2 context lanes, 2075 non-context lanes). Beside it are the 707-lane program image, the 683-lane pre and post images, and 685 output IDs.
- An independent comparison matched every value, all lane metadata and each whole image, with 0 mismatches.
- The caption names this as statement checking of one fixture. No durations appear on the site.

## Motion, keyboard, reduced motion, mobile, fallback

- **Motion.** The camera dollies along the spine between boards (400 ms ease). Selection glow is static. Nothing pulses to imply activity. The ladder highlight steps rung by rung only when the user presses Step, never on autoplay.
- **Keyboard.** Tab follows assembly order:
  1. shelves and tag;
  2. rungs 0-15 (arrow keys move between rungs);
  3. switch;
  4. dock: exit, refusal, failure, adapters, shield;
  5. yard: sidings from the top, PrimResult, shield;
  6. chassis sockets.

  Enter opens the source panel with file, symbol and comment tag (A11, E05, F02, C01).
- **Reduced motion.** Cuts replace the dolly. The ladder is drawn whole with per-rung labels.
- **Mobile.** Boards stack vertically and the ladder becomes a 16-row table. Tap targets are at least 44px, and explicit zoom buttons are provided.
- **Static fallback.** Each board gets an SVG plate plus a DOM table that mirrors every glyph with source links. It needs no WebGL. Tone is never the only signal: notches, labels and position always carry the meaning too.

## Next source iterations

1. Parent registration mounts the module in the chassis outline.
2. Caller dispatch and mux add a selector rail that feeds exactly one enabled row.
3. StepInv/SI-21 provenance draws a thread from the dock's `allocated_index` to the yard's `PrimResult`.
4. The Bool publication/P1 chain adds a spur east of the yard.
5. Physical packing adds a layer that snaps logical lanes to columns.
6. Each actual compile, SDK or proof result adds a dated history card, never a glow on a board.
7. Eq arrives as its own bay.

## Visitor copy (finished release)

```text
Ask whether one number is smaller than another, and the switchyard reads the right-hand number from exactly the shelf it was named on, then walks sixteen bytes from lowest to highest, passing a borrow down the line until one switch answers yes or no. That answer is recorded as a fresh true-or-false — or, if the right-hand side was never a number, the board says so with its own error. Then the yard hands the answer back to the very computation that asked, lifting only its waiting frame and leaving every other exactly in place.
```

## INTERNAL: provenance and gap register (not visitor copy)

- **Source.** All three Lt modules are source-audited, with exactly three imports and N1 removed.
  - Draft PR 2885 at `a321beb`: 3 files, 139,011 B (35,147 + 55,759 + 48,105).
  - The modules are unregistered, uncompiled and untested.
  - The headers name Sol as source author. A source audit is not compilation, execution or a security verdict.
- **Composition.** The constrained Lt Bool-value-or-error-11 into typed op4 ReturnRight path depends on caller dispatch/SI, namespace, and arena/framing conditions. The chassis sockets record exactly this.
- **SDK.** On the current SDK cut, the first heavy A102 Literal check passed in 5.8227 s with 1 positive and 7 negative checks.
  - That was 1 of 120 harness tests, selected.
  - Independently, all 2077 values, lane metadata and whole images match.
  - This is not proof, not stagenet, not native, and not an Lt test.
  - The source/result auditor for that SDK cut is still pending.
- **Not claimed.** No execution of a recursive-program fixed fixture. No compiler-to-ledger soundness.
- **Scope of this note.** These are design ideas, not security verdicts. There is no author-guard approval and no new runtime or publishing pipeline. Any fallback author keeps their own identity in the provenance record.