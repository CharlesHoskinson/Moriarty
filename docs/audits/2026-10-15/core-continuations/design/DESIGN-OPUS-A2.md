# Opus 5.5 amendment A1.1: final correction to A1 (baseline `fe01bc8b…`)

Design only; nothing executed. This amends A6 and A7. Everything else in A0–A11 is unchanged.

## C1. Zero-await pending representation (resolves Clarification §2.4 vs A6/A7)

**Type (A6).** In `StagedCircuit`, replace the field with:

```rust
#[serde(deserialize_with = "Option::deserialize")] pub pending_positions: Option<PendingPositions>,
```

- It is always serialized. N = 0 is written as `null`.
- `deserialize_with` makes the key required, so an absent key is rejected and there is no second encoding.
- `PendingPositions` itself is unchanged and has no sentinel values.

**Decoder (A6 rule 4 replaced).** Per action, N = segment count − 1.

| Case | Requirements |
|---|---|
| N = 0 | exactly one circuit; `segment == 0`, `entry_site == 0`, `exit_site == 0`; `capture_width == 0`; `capture_in`, `capture_out` and `event` empty; `pending_positions == None`; no public binding with any of the five pending roles |
| N ≥ 1 | every circuit has `pending_positions == Some(p)`; `capture_width == L`; `p` satisfies former rule 4 verbatim |
| any mismatch | existing artifact decode error identity |

Former rule 4 is: in range, pairwise distinct, disjoint from other positions, roles/names/types/constants per A7, lengths L and B, and segment 0 has no event entries.

**Frame helpers (A7 replaced).**

```rust
pub(crate) fn emit_pre(b: &mut Builder, l: &SegmentLayout) -> Result<Option<PreFrame>, Error>;
pub(crate) fn emit_post(b: &mut Builder, l: &SegmentLayout, pre: Option<&PreFrame>,
    live: &[(&str, Wire)]) -> Result<Option<PendingPositions>, Error>;
```

- **New `PreFrame` fields.** It gains `pre_site_pos: u32`, `pre_capture_pos: Vec<u32>` and `event_pos: Vec<u32>`. These are module-private to `frame.rs`. Each records the position `Builder::public` assigns (U-4).
- **Zero-await test.** `zero := l.entry_site == 0 && l.exit_site == 0`. This holds iff N = 0, because advance segments have `entry_site ≥ 1`.
- **`emit_pre` branches.**
  - If `zero`: require `l.slots`, `len_in`, `len_out` and `event` all empty, else `artifact`. Emit nothing and return `Ok(None)`.
  - Otherwise: emit the pre-pending block per the A7 table and return `Ok(Some(..))`.
- **`emit_post` branches.**
  - `(zero, None)`: require `live` empty. Emit nothing and return `Ok(None)`.
  - `(!zero, Some(pre))`: run the existing A7 `live` checks, emit the post block, and return `Ok(Some(PendingPositions{..}))` built from `pre`'s positions plus the post positions.
  - Any other combination: `artifact`.
- **Lowerer.** It calls both helpers unconditionally and stores the result in `pending_positions`. The branch is owned solely by `frame.rs`.
- **Unchanged.** H27 and the A8 preimage are unchanged; with N = 0 the projection simply has no pending entries. Zero-await support is retained. No pending roles are ever emitted for N = 0.

## C2. Builder borrow correction

**Confirmed.** Dropping the redundant `bindings: &mut Vec<PublicBinding>` parameter is an allowed routine interface correction under identical constraints:

- All pending roles are emitted only through `Builder::public`, which appends to `b.bindings`.
- They are emitted only from `frame.rs` (H27).
- Positions, order and insertion points are as in A7.
- No other `Builder` API is added, and `Builder` fields are not touched from `frame.rs`. Reading `b.bindings.len()` is permitted only if `public` returns no position.

The same latitude applies to Opus-authored code: any signature change limited to satisfying the borrow checker, with no change to bytes, constraints, roles or order, needs no design round. It must be reported in the code review.

## C3. Status

- A0–A11 and the A12 sign-off conditions are unchanged.
- Native authority remains 0, and every U/M exit is retained.
- **No further Opus design round is needed** if:
  - Sol adopts A1 plus this A1.1 verbatim, and
  - Muse approves the final design.
- Source implementation then begins. All remaining scrutiny is code/test review and the pre-execution closure classification, not more design.

No execution is approved by this document.
