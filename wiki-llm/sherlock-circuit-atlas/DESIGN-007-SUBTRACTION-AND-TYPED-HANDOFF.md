# DESIGN-007: Subtraction and typed handoff

- **Date:** 2026-10-18 UTC
- **Author:** Claude Opus 5.5 (`claude-opus-5-5`), high effort
- **Kind:** Design companion. It adds no circuit iteration.

## Source pins

| File | SHA256 | Anchors |
|---|---|---|
| `b_scalar_subtraction.rs` | `3d3b80476c5d47f7396822b176b1adae0fe6e6fcff8c57a47cee1088d0c0e8e7` | `emit_b_scalar_arithmetic_sub` A04–A14 |
| `b_scalar_subtraction_completion.rs` | `c507e620edd88d0477672b3b459eac5ee290fefc3db3a47de835cad6a818951f` | `emit_b_scalar_exit_sub_u128` E02–E08, `emit_b_scalar_failure_sub` F02–F05, `BSubPublishState`, `emit_b_sub_completion_complement` C01–C02 |
| `b_scalar_subtraction_three_tests.rs` | `58122f212b4878e9f7dd292f2756d66909d01b1789649822f0d4bdc040e92c77` | Pinned by the freeze; body not forwarded, so not observed |

Freeze manifests: `99b0a7b6…f28056` and `c4d96eba…8951f`.

## Internal provenance (not visitor copy)

- The Sub emitter and its three tests are authored and materialized.
- Scalar Sub completion and the typed handoff are authored and materialized.
- Both are unregistered. All SDK tests are UNRUN.
- No Sub return-right or P1 integration exists yet. The source records that return-right still requires `BINARY_OP_ADD`.
- The Namespace RR/PV/PE correction is a separate source review, APPROVE_WITH_LIMITS, with 43 tests UNRUN.
- This note is not a security or source approval.
- It claims no emitted execution, proof, deployment or complete Sherlock.

## Source observations

1. **Fetch (A04–A08).** Operand B is fetched from the Const or Data space through `space_read_any_kind` → `emit_enabled_fetch_block`. Its tag must be below 6. Non-UInt rows force their bytes to zero.
2. **Borrow chain (A11).** For each little-endian byte, `need = read_n + borrow` is ranged to 9 bits, and `borrow_out = LessThan(eff_reg_a, need)`. The final borrow is `underflow`.
3. **Staging (A12–A13).** `post + need = reg_a + 256·borrow_out` holds under `is_val`. The block stages `U128Value`, `PrimError(10)` on underflow, or `PrimError(11)` when operand B is not UInt.
4. **Exit (E02–E08).** `applied + cap_refused = enable`. A one-hot write selector appends `DataRow::U128`. On refusal the outcome is `Exhausted(DataNodes)` and the helper is retained.
5. **Failure (F02–F05).** The code must be in {10, 11}. The result becomes `ScalarError`, the helper is cleared and the data rows are preserved.
6. **Typed handoff.** The handoff adapters (`to_admin_publish_state`, `to_legacy_handoff` and the inverse) move every lane unchanged and emit no instructions. The complement preserves every lane the completion emitters never write and binds `reference_cursor_bytes`.
7. **Disabled rows.** When disabled, post lanes are left free.

## Options (proposals)

- **A. Ripple ladder.** Sixteen byte lanes run as rungs. Each rung carries a `need` pad and a LessThan comparator, with the borrow trace stepping to the next rung.
- **B. Twin board.** Sub sits as a mirror layer over the iteration-006 Add board, with a diff overlay.
- **C. Edge connector.** The focus is the handoff: a ribbon from Sub carriers into the existing publication sockets.

**Recommendation: A, with C's connector as the board's exit.** A is closest to the actual relation. B suggests shared Rust types where the source says the carriers are only structural copies.

## Cumulative board: borrow lane to typed handoff

Layers enter the assembly history in freeze order:

1. **Fetch.** The reused enable bus from iteration 003.
2. **Ladder.** The sixteen-rung borrow ladder, read from byte 0 to byte 15.
3. **Staging socket.** Three pads: value, 10 and 11.
4. **Completion fork.** Exit splits into Applied (the arena write selector) or the cap-refusal pad. Failure is a separate branch.
5. **Handoff ribbon.** Via-free and unlit, because it emits no instructions. A guard ring around it represents the complement.

## Visitor copy (finished release)

> Subtraction runs across sixteen byte lanes. Each lane asks one question: is this byte smaller than what it owes? It passes the answer to the next lane. Follow the borrow to its end. A clean difference is written into the data arena. An underflow or a wrong sort becomes a typed error. Either way, the result leaves the board through a connector that changes nothing it carries.

## Accessibility, motion and clickthrough

- **DOM table.** It mirrors the ladder, with one row per byte, plus labelled outcome pads. Keyboard order follows the lane index.
- **Not colour alone.** Borrow is shown with a notch glyph and text. The palette uses the existing copper only, with no new hue.
- **Reduced motion.** There is no ripple. A selected rung highlights statically, and layer changes cut rather than animate.
- **Clickthrough.** Every glyph opens its path, symbol and comment anchor (A11, E05, C01) at the pinned SHA256.
- **Light channel.** Disabled rows are never animated. Light stays selection only until a measured run exists.