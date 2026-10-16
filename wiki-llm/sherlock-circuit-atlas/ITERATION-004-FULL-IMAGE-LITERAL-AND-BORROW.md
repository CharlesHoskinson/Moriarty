# Iteration 004: full-image literal and borrow

Proposed corrected materialization. The raw Gemini author reply stays immutable. This proposal performs no wiki transaction or publication.

## Authorship and provenance

- **Initial concepts:** Gemini 4 Argon (High) via AGY, drafted under the user's explicit fallback while Claude Opus 5.5 was temporarily unavailable. Raw reply `circuit-atlas-004-argon-author-r1/RESULT.json` `975cb29561fc0b0bc94bd673a2ef465bbc726fc3aa2746495569229b9d98e1a2` (26997 bytes).
  - Requested: `gemini-4-argon-high`, mode `plan`, effort `high`.
  - Returned: selection `gemini-4-argon-high` and `permission_mode: always-proceed`. Separate canonical ID and returned effort are both `NOT_REPORTED`.
  - No plan-mode enforcement or mechanical tool absence is claimed.
- **Source-accuracy revisions:** Claude Opus 5.5 (`claude-opus-5-5`, requested effort high). Captured Muse normal findings (`circuit-atlas-004-muse-review-r1/LAUNCH.stdout` `496f840969365740a143a24d34140e3e384fbb1f7af29b6e5ef37e37f1fb9ed3`; qualification `74b34f3b3331eaf8832aae3620807b933fb52df18e74dbd623a14eee05a60e39`) were checked independently against the complete frozen source. Muse's reply lacked its required outer delivery marker, so it is normal advice, not an approved audit.
- Iterations 000–003 keep their original Claude Opus 5.5 authorship.

## Status

- This is a creative, source-only concept note. It is not an audit, a compiler check, or an acceptance decision.
- At authoring time, the A102 source was pending fresh owner review.
- Two scoped source reviews came later, both `APPROVED_SOURCE_WITH_LIMITS`:
  - Muse normal `f3ba9ef52a43124b0fc9d450d621e4ca1553e93f252133d7f00954ae2633346e`
  - Gemini source-owner `09c4a6c2375721fbdb6984ff2ff8bb253edf93b5297a8f43a809aa22cf45bbd8`
  
  They cover the source fragment only. They do not approve this note, a website, execution, proof, cost, or acceptance.
- At its freeze, the original source was unregistered and uncompiled, all five tests were UNRUN, and `native_authority = 0`. A later registration source now in preparation is excluded from this iteration's freeze.
- No cost, `k`, row, proof, runtime, or stagenet claim is made.

## Source pins

Manifest `662bc7748f2a98e2186372970951c08ebed0328b04c574f6f7e4f5eab42e7f85`. Envelope/carrier clarification `d808e46cc9c1b871f285722639511c5e45e0b01e4e9067078d5fa5a40216f3cc` continues scoped P1 `ebea84ac1ce065d508d6a0ff17bafa39a190007214142075205fb97278c59213`.

| Path | SHA256 | Bytes |
|---|---|---:|
| `crates/mori-backend/src/sherlock_machine/literal_step.rs` | `b5d2ca73b0e6b561f554902c3f4e50ec7743cc7ca04b9e7e9acb0c3aca10b839` | 19788 |
| `crates/mori-backend/src/sherlock_machine/literal_step_normal_tests.rs` | `255408fdafb36b2ed771206c366232dea2b61d7345d79537121e556f38163ef6` | 8079 |
| `models/sherlock/literal-step/MAPPING.md` | `72411a42e8886f18286192d10aff885bc8435347e424c9170c1f1f4dc467eaad` | 9041 |

## Source features

1. **Logical schema** (`literal_schema`, `check_image`).
   - Rejects with `MinimumShape` when `x == 0 || k == 0 || control == 0 || t < 4 || q < 4 || qf < 5`.
   - Builds a 16-lane program header and 15 program tables with lanes `[2,1,4,1,11,1,6,2,3,2,3,4,1,22,2]`.
   - Each state image gets a 45-lane Root and six tables with lanes `[22,2,5,2,17,36]`.
   - These are logical operand images (`LiteralImage`), not serialized bytes or canonical L4.
2. **Pre guards.** When `enable` is set:
   - the pre outcome is Pending with zero payload and error;
   - `controlLength == 1`;
   - ControlRows row 0 is zero except `exprId`, which makes it Eval;
   - later rows are zero.
3. **Real G3 fetch** (`emit_enabled_fetch_block`).
   - Index is `pre.tables[5][0].exprId`, rows are `program.tables[4]`, and the active count is `program.root[5]`.
   - The selected row must have `tag == 0` and `valueRef.space == 0`, with inactive columns and slices zero.
   - It must also satisfy `occurrenceId == exprId` and `valueRef.index < program.root[14]`.
4. **Constant alias.**
   - The selected `valueRef.{space,index}` is aliased into post ControlRows row 0.
   - The supplied `Constants` [13] and `ConstantFields` [14] rows are range-checked.
   - No fetch, unfold, or `Data` materialization occurs.
5. **Root and tables.**
   - All 45 Root lanes are constrained: 16 `remainingFuel` lanes by borrow equations, `controlLength == 1`, and the other 28 lanes (including every `initialFuel` byte) by `post == pre`.
   - Five whole tables, indices 0–4, are preserved cell by cell.
   - `ControlRows` (index 5) is constrained separately. Row 0 has `tag == 1` (Return) plus the aliased reference, and its other 33 columns are zero. Rows ≥ 1 are zero.
6. **Borrow.**
   - `zero_i = TestEq(pre_i, 0)`, and the all-zero fold is gated to 0.
   - `borrow_0 = enable` and `next = CondSelect(borrow_i, zero_i, 0)`.
   - Each byte satisfies the gated equation `pre_i + 256*next == post_i + borrow_i`, with 9-bit intermediates.
   - A final unconditional `ConstrainEq(borrow_16, 0)` closes the chain.
7. **Ranges and guarding.**
   - Every supplied lane gets a bit-width or Boolean check unconditionally.
   - Only lanes that carry a `maximum` also get `LessThan(maximum, val) == 0`.
   - `gated_eq` emits `ConstrainEq(CondSelect(enable, a, b), b)`. A disabled branch therefore imposes no A102 transition equation on the shared post state, while range checks still apply.
8. **Envelope.** `LiteralEnvelope` sits outside both blocks. When enabled, it constrains `rule_tag == 3` (enum(39)) and `charge == 1`. Host `RecordEnvelopeA.rule_start_line = 102` is unchanged and is not a circuit operand.

## Proposed Three.js PCB scenes

1. **Triple-plane logical-image carrier.**
   - Program, pre, and post planes carry the caption `logical operand images — not serialized bytes, canonical L4, or a running machine`.
   - Standoffs bridge the 28 preserved Root pads and the five preserved table banks.
   - ControlRows gets its own Return-row and zero-bar footprint.
   - Constants banks show populated pads with range terminations and no fetch traces.
   - Fuel is drawn as two solid 16-trace byte buses.
   - Non-fuel carriers end in dashed sockets labeled `OBL.G4@A102 / StepInv: ceil(w/8) LE byte bridge — caller obligation`.
2. **Sixteen-stage borrow daughterboard.**
   - A zero-fold rail runs above the borrow chain, which is grounded at `borrow_16 == 0`.
   - An optional `256 -> 255` stepper (`pre = [0,1,0..]`, `post = [255,0,0..]`) runs under the fixed banner **`arithmetic illustration; not an executed circuit`**.
3. **Guarded composition backplane.**
   - An unconditional bit/Boolean pad grid, with maximum pads only on lanes that carry a maximum.
   - A `%dispatch.a102` enable bus feeds the `CondSelect` trapezoids.
   - Dashed perimeter sockets are labeled `caller prerequisite — not implemented by A102`.

## Accessibility and truth labels

- Badge: `SOURCE-ONLY | UNREGISTERED AT FREEZE | 5 TESTS UNRUN | COMPILER/SDK UNRUN | NATIVE AUTHORITY 0 | COST UNMEASURED`.
- DOM netlist and static SVG fallback, with unabbreviated `%` identifiers.
- Keys `B`/`P`/`O` step through borrow stages, preservation rails, and open sockets.
- Under `prefers-reduced-motion`, transitions cut without interpolation.

## Corrections versus the raw draft

- `6-Table Preservation` now reads as five preserved tables plus separately constrained ControlRows.
- The maximum check is limited to lanes that carry a maximum.
- Minimum shape is stated as a rejection condition.
- Author-time pending review is separated from the later scoped approvals.
- The plan-mode request and the always-proceed return are both recorded. The ambiguous bare `effort` field is removed.
- Planes are captioned as logical images, and Constants are shown populated rather than as keep-outs.
- Opus additions:
  - removed the `RULE_LINES` bijection, which is absent from the supplied source;
  - replaced `post unconstrained` with `no A102 transition equation`;
  - added the pre guards.

## Open gates (unwaived)

- Non-fuel byte bridge, `StepInv`/O1–O4, and `uiValid`
- `F4` canonicality
- Exact-one 39-rule dispatch and free publication
- Remaining 38 events, unwritten P1 rows, and P2–P6
- Statement, trace, and public-private binding
- Resource admission
- Candidate B, general ADTs, and financial gates
- Proofs and stagenet
