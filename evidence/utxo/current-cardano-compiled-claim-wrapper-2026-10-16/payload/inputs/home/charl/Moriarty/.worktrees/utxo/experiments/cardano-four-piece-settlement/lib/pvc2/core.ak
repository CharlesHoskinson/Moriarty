//// Core `moriarty-core/1` semantics needed on-chain for ABI v2 (P3S1-R1 successor,
//// PROPOSED, uncompiled): shape bytes, structural shape validation, the typed effect
//// view, defence-in-depth recomputation of the represented snapshot, Core
//// `intent::check_effects`, and the statement bytes.
////
//// Nothing here re-executes a Moriarty program: expressions, requires/ensures and amount
//// formulas are the relation's job (proof). These functions re-derive every quantity that
//// moves value or debt from the bound effects. Together with the structural shape checks
//// (cells strictly ascending, hence distinct; review RR-01) custody value and debt change
//// only through effects the required signers authorized, independently of proof
//// soundness. Program-rule conformance still rests on proof soundness and setup honesty.

use aiken/builtin
use aiken/collection/list
use aiken/primitive/int
use pvc2/bytes
use pvc2/catalogue
use pvc2/types.{
  ActionEntryV2, BalanceRowV2, CatalogueV2, CellRefV2, DeclV2, DutyRefV2,
  DutyRowV2, DutyValV2, EFee, ELiability, EState, ETransfer, EffectShapeV2,
  EffectValV2, EvAmount, EvDuty, EvState, LimitV2, ShapeV2, VBool, VUInt,
  ValueV2,
}

pub const stmt_tag: ByteArray = "MRG16-PVC-STMT-2"

pub const family_v2: ByteArray = "MRG16-PVC-FAM2-G16-BLS-ARK04-HL2"

pub const shape_tag: ByteArray = "MRG16-PVC-SHP-02"

// ## Shape

fn effect_shape_bytes(e: EffectShapeV2) -> ByteArray {
  when e is {
    EState { state } -> bytes.cat([#"00", bytes.u16(state)])
    ETransfer { from_cell, to_cell } ->
      bytes.cat([#"01", bytes.u16(from_cell), bytes.u16(to_cell)])
    EFee { from_cell, to_cell } ->
      bytes.cat([#"02", bytes.u16(from_cell), bytes.u16(to_cell)])
    ELiability { duty, discharge } ->
      bytes.cat([#"03", bytes.u16(duty), bytes.bool_byte(discharge)])
  }
}

fn decl_bytes(d: DeclV2) -> ByteArray {
  bytes.cat([bytes.lp(d.name), bytes.u8(d.ty)])
}

fn counted(xs: List<a>, f: fn(a) -> ByteArray) -> ByteArray {
  bytes.cat([bytes.u16(list.length(xs)), ..list.map(xs, f)])
}

/// `MRG16-PVC-SHP-02` bytes (ABI-V2.md section 4): catalogue assets and states, then the
/// redeemer shape, then the action name and program index from the catalogue entry.
pub fn shape_bytes(
  cat: CatalogueV2,
  entry: ActionEntryV2,
  shape: ShapeV2,
) -> ByteArray {
  bytes.cat(
    [
      shape_tag,
      counted(
        cat.assets,
        fn(m) {
          bytes.cat(
            [
              bytes.lp(m.id.network), bytes.lp(m.id.issuer),
              bytes.lp(m.id.reference), bytes.u8(m.id.scale),
            ],
          )
        },
      ),
      counted(cat.states, decl_bytes),
      counted(shape.inputs, decl_bytes),
      counted(
        shape.cells,
        fn(c) { bytes.cat([bytes.u16(c.asset), bytes.lp(c.account)]) },
      ),
      counted(
        shape.duties,
        fn(d) {
          bytes.cat(
            [
              bytes.lp(d.name), bytes.u16(d.asset), bytes.lp(d.debtor),
              bytes.lp(d.creditor),
            ],
          )
        },
      ),
      counted(shape.effects, effect_shape_bytes),
      counted(shape.debit_cells, bytes.u16),
      bytes.u16(shape.statement_count),
      bytes.u16(shape.liability_statements),
      bytes.lp(entry.name),
      bytes.u16(entry.index),
    ],
  )
}

fn in_range(i: Int, xs: List<a>) -> Bool {
  i >= 0 && i < list.length(xs)
}

fn cell_asset(shape: ShapeV2, i: Int) -> Int {
  expect Some(c) = list.at(shape.cells, i)
  c.asset
}

/// Debit cells recomputed from the effects: ascending distinct `from_cell`s.
pub fn debit_cells_of(shape: ShapeV2) -> List<Int> {
  list.filter_map(
    shape.effects,
    fn(e) {
      when e is {
        ETransfer { from_cell, .. } -> Some(from_cell)
        EFee { from_cell, .. } -> Some(from_cell)
        _ -> None
      }
    },
  )
    |> list.unique
    |> list.sort(int.compare)
}

/// Plan cell order: (asset index, account), accounts compared bytewise. Catalogue assets
/// are strictly ascending by Core `AssetId` (`catalogue.well_formed`), so asset index
/// order equals `AssetId` order, and the compiler's cells (a `BTreeMap` keyed by
/// `(AssetId, account)`, `plan.rs`) satisfy this order.
pub fn cell_lt(a: CellRefV2, b: CellRefV2) -> Bool {
  or {
    a.asset < b.asset,
    and {
      a.asset == b.asset,
      bytes.bytes_lt(a.account, b.account),
    },
  }
}

/// RR-01 (P3S1-R1): represented cells are strictly ascending in plan order and therefore
/// pairwise distinct. A catalogue-pinned shape that repeats an `(asset, account)` would
/// let a net-zero "self-transfer" between two indices of one custody row pass every
/// signer's `check_effects` while `cells_recomputed` moves value between the indices and
/// the continuation keeps only the first index: custody value would leave with no payout.
/// Unsorted distinct cells are refused too, so the accepted shape language is exactly the
/// compiled one.
pub fn cells_ordered(shape: ShapeV2) -> Bool {
  bytes.strictly_ascending(shape.cells, cell_lt)
}

/// The complete structural shape gate: `shape_structure_ok` and `cells_ordered`.
/// S2 evaluates the two components separately (named gate parts) so that tests can
/// observe which one refuses.
pub fn shape_ok(cat: CatalogueV2, shape: ShapeV2) -> Bool {
  shape_structure_ok(cat, shape) && cells_ordered(shape)
}

/// Structural consistency of a shape with the catalogue (indices in range, Transfer/Fee
/// cells share an asset, duties strictly ascending by name, debit cells recomputed). The
/// digest pin binds the shape to the catalogue entry; this guards the interpretation
/// against malformed catalogue pins (unchanged P3S1 rule set).
pub fn shape_structure_ok(cat: CatalogueV2, shape: ShapeV2) -> Bool {
  and {
    list.all(shape.inputs, fn(d) { bytes.core_string(d.name) && (d.ty == 0 || d.ty == 1) }),
    list.all(shape.cells, fn(c) { in_range(c.asset, cat.assets) && bytes.core_string(c.account) }),
    list.all(
      shape.duties,
      fn(d) {
        and {
          in_range(d.asset, cat.assets),
          bytes.core_string(d.name),
          bytes.core_string(d.debtor),
          bytes.core_string(d.creditor),
        }
      },
    ),
    bytes.strictly_ascending(list.map(shape.duties, fn(d) { d.name }), bytes.bytes_lt),
    list.all(
      shape.effects,
      fn(e) {
        when e is {
          EState { state } -> in_range(state, cat.states)
          ETransfer { from_cell, to_cell } -> and {
              in_range(from_cell, shape.cells),
              in_range(to_cell, shape.cells),
              cell_asset(shape, from_cell) == cell_asset(shape, to_cell),
            }
          EFee { from_cell, to_cell } -> and {
              in_range(from_cell, shape.cells),
              in_range(to_cell, shape.cells),
              cell_asset(shape, from_cell) == cell_asset(shape, to_cell),
            }
          ELiability { duty, .. } -> in_range(duty, shape.duties)
        }
      },
    ),
    shape.debit_cells == debit_cells_of(shape),
    shape.statement_count > 0,
    shape.statement_count < 65536,
    shape.liability_statements >= 0,
  }
}

// ## Typed effect view

pub type MoveV2 {
  fee: Bool,
  asset: Int,
  from: ByteArray,
  to: ByteArray,
  from_cell: Int,
  to_cell: Int,
  amount: Int,
}

pub type CoreEffectV2 {
  CMove(MoveV2)
  CDuty {
    duty: Int,
    name: ByteArray,
    asset: Int,
    debtor: ByteArray,
    creditor: ByteArray,
    before: Int,
    after: Int,
    discharge: Bool,
  }
  CState { state: Int, before: ValueV2, after: ValueV2 }
}

pub fn value_ty(v: ValueV2) -> Int {
  when v is {
    VUInt(n) -> {
      expect bytes.is_u128(n)
      0
    }
    VBool(_) -> 1
  }
}

pub fn typed_bytes(v: ValueV2, ty: Int) -> ByteArray {
  expect value_ty(v) == ty
  when v is {
    VUInt(n) -> bytes.u128(n)
    VBool(b) -> bytes.bool_byte(b)
  }
}

/// Zips shape effects with redeemer effect values; fails on any kind mismatch or an
/// amount outside [0, 2^128).
pub fn effect_view(
  cat: CatalogueV2,
  shape: ShapeV2,
  values: List<EffectValV2>,
) -> List<CoreEffectV2> {
  expect list.length(values) == list.length(shape.effects)
  list.map(
    list.zip(shape.effects, values),
    fn(p) {
      let (s, v) = p
      when (s, v) is {
        (EState { state }, EvState { before, after }) -> {
          expect Some(d) = list.at(cat.states, state)
          expect value_ty(before) == d.ty && value_ty(after) == d.ty
          CState { state, before, after }
        }
        (ETransfer { from_cell, to_cell }, EvAmount { amount }) ->
          CMove(mv(shape, False, from_cell, to_cell, amount))
        (EFee { from_cell, to_cell }, EvAmount { amount }) ->
          CMove(mv(shape, True, from_cell, to_cell, amount))
        (ELiability { duty, discharge }, EvDuty { before, after }) -> {
          expect Some(d) = list.at(shape.duties, duty)
          expect bytes.is_u128(before) && bytes.is_u128(after)
          CDuty {
            duty,
            name: d.name,
            asset: d.asset,
            debtor: d.debtor,
            creditor: d.creditor,
            before,
            after,
            discharge,
          }
        }
        _ -> fail @"effect value kind differs from the shape"
      }
    },
  )
}

fn mv(shape: ShapeV2, fee: Bool, from_cell: Int, to_cell: Int, amount: Int) -> MoveV2 {
  expect bytes.is_u128(amount)
  expect Some(f) = list.at(shape.cells, from_cell)
  expect Some(t) = list.at(shape.cells, to_cell)
  MoveV2 { fee, asset: f.asset, from: f.account, to: t.account, from_cell, to_cell, amount }
}

pub fn moves(effects: List<CoreEffectV2>) -> List<MoveV2> {
  list.filter_map(
    effects,
    fn(e) {
      when e is {
        CMove(m) -> Some(m)
        _ -> None
      }
    },
  )
}

/// Gross debit of a represented cell: every Transfer/Fee naming it as `from`.
pub fn gross_of_cell(ms: List<MoveV2>, cell: Int) -> Int {
  list.foldl(ms, 0, fn(m, acc) { if m.from_cell == cell { acc + m.amount } else { acc } })
}

pub fn fees_of_cell(ms: List<MoveV2>, cell: Int) -> Int {
  list.foldl(ms, 0, fn(m, acc) { if m.from_cell == cell && m.fee { acc + m.amount } else { acc } })
}

pub fn credits_of_cell(ms: List<MoveV2>, cell: Int) -> Int {
  list.foldl(ms, 0, fn(m, acc) { if m.to_cell == cell { acc + m.amount } else { acc } })
}

// ## Defence-in-depth recomputation

/// post = pre + credits - debits for every represented cell, each in [0, 2^128).
/// Self-transfers cancel (Core leaves the row unchanged).
pub fn cells_recomputed(pre: List<Int>, post: List<Int>, ms: List<MoveV2>) -> Bool {
  list.length(pre) == list.length(post) && list.all(
    list.indexed_map(list.zip(pre, post), fn(i, p) { (i, p) }),
    fn(ip) {
      let (i, p) = ip
      let (a, b) = p
      bytes.is_u128(b) && b == a + credits_of_cell(ms, i) - gross_of_cell(ms, i)
    },
  )
}

/// Chains the Liability effects of duty `j` from its pre cell: the first `before`
/// equals the pre amount (0 when absent), accrual never decreases, discharge never
/// increases and needs a present row, and the result is the post cell. Present-zero is
/// distinct from absent: a row becomes present only by an accrual and never disappears.
pub fn duty_recomputed(
  j: Int,
  pre: DutyValV2,
  post: DutyValV2,
  effects: List<CoreEffectV2>,
) -> Bool {
  let (present, amount) =
    list.foldl(
      effects,
      (pre.present, pre.amount),
      fn(e, acc) {
        when e is {
          CDuty { duty, before, after, discharge, .. } ->
            if duty == j {
              let (p, a) = acc
              expect before == a
              if discharge {
                expect p && after <= before
                (p, after)
              } else {
                expect after >= before
                (True, after)
              }
            } else {
              acc
            }
          _ -> acc
        }
      },
    )
  and {
    pre.present || pre.amount == 0,
    post.present || post.amount == 0,
    post.present == present,
    post.amount == amount,
  }
}

/// Chains the State effects of state `k`; untouched states keep their value.
pub fn state_recomputed(
  k: Int,
  pre: ValueV2,
  post: ValueV2,
  effects: List<CoreEffectV2>,
) -> Bool {
  let last =
    list.foldl(
      effects,
      pre,
      fn(e, cur) {
        when e is {
          CState { state, before, after } ->
            if state == k {
              expect before == cur
              after
            } else {
              cur
            }
          _ -> cur
        }
      },
    )
  last == post
}

// ## Core intent::check_effects (exact port; u128 overflow is an error, as in Core)

fn add128(a: Int, b: Int) -> Int {
  let c = a + b
  expect c < bytes.two_128
  c
}

/// `closing` is the complete post liability list of the snapshot.
pub fn check_effects(
  signer: ByteArray,
  limits: List<LimitV2>,
  effects: List<CoreEffectV2>,
  closing: List<DutyRowV2>,
) -> Bool {
  let limit_assets = list.map(limits, fn(l) { l.asset })
  let has_limit = fn(asset) { list.has(limit_assets, asset) }
  let ms = moves(effects)
  and {
    list.length(list.unique(limit_assets)) == list.length(limit_assets),
    list.all(
      effects,
      fn(e) {
        when e is {
          CMove(m) -> m.from != signer || has_limit(m.asset)
          CDuty { asset, debtor, .. } -> debtor != signer || has_limit(asset)
          CState { .. } -> True
        }
      },
    ),
    list.all(closing, fn(d) { d.debtor != signer || has_limit(d.asset) }),
    list.all(
      limits,
      fn(l) {
        let (gross, fees, credit, debit) =
          list.foldl(
            ms,
            (0, 0, 0, 0),
            fn(m, acc) {
              let (g, f, c, d) = acc
              if m.asset != l.asset {
                acc
              } else if m.fee {
                (
                  if m.from == signer { add128(g, m.amount) } else { g },
                  if m.from == signer { add128(f, m.amount) } else { f },
                  c,
                  if m.from == l.beneficiary && m.from != signer {
                    add128(d, m.amount)
                  } else {
                    d
                  },
                )
              } else {
                (
                  if m.from == signer { add128(g, m.amount) } else { g },
                  f,
                  if m.to == l.beneficiary { add128(c, m.amount) } else { c },
                  if m.from == l.beneficiary { add128(d, m.amount) } else { d },
                )
              }
            },
          )
        let obligations =
          list.foldl(
            closing,
            0,
            fn(d, acc) {
              if d.asset == l.asset && d.debtor == signer {
                add128(acc, d.amount)
              } else {
                acc
              }
            },
          )
        let net = credit - debit - fees
        and {
          gross <= l.gross_cap,
          fees <= l.fee_cap,
          obligations <= l.liability_cap,
          net >= 0,
          net >= l.net_floor,
        }
      },
    ),
  }
}

// ## Statement

pub type StatementCtxV2 {
  predicate_digest: ByteArray,
  shape_digest: ByteArray,
  vk_digest: ByteArray,
  claimant_vkh: ByteArray,
  network_id: Int,
  settle_hash: ByteArray,
  thread_id: ByteArray,
  consumed_txid: ByteArray,
  consumed_ix: Int,
  seq: Int,
  catalogue_digest: ByteArray,
  intents_digest: ByteArray,
}

fn duty_bytes(d: DutyValV2) -> ByteArray {
  bytes.cat([bytes.bool_byte(d.present), bytes.u128(d.amount)])
}

fn effect_bytes(cat: CatalogueV2, e: CoreEffectV2) -> ByteArray {
  when e is {
    CState { state, before, after } -> {
      expect Some(d) = list.at(cat.states, state)
      bytes.cat([typed_bytes(before, d.ty), typed_bytes(after, d.ty)])
    }
    CMove(m) -> bytes.u128(m.amount)
    CDuty { before, after, .. } -> bytes.cat([bytes.u128(before), bytes.u128(after)])
  }
}

/// Statement body (ABI-V2.md section 3): the StatementLayout slot order without header
/// slots. Accounting is recomputed here from the effect amounts.
pub fn body(
  cat: CatalogueV2,
  shape: ShapeV2,
  inputs: List<ValueV2>,
  pre_states: List<ValueV2>,
  pre_cells: List<Int>,
  pre_duties: List<DutyValV2>,
  post_states: List<ValueV2>,
  post_cells: List<Int>,
  post_duties: List<DutyValV2>,
  effects: List<CoreEffectV2>,
) -> ByteArray {
  expect list.length(inputs) == list.length(shape.inputs)
  expect list.length(pre_states) == list.length(cat.states)
  expect list.length(post_states) == list.length(cat.states)
  expect list.length(pre_cells) == list.length(shape.cells)
  expect list.length(post_cells) == list.length(shape.cells)
  expect list.length(pre_duties) == list.length(shape.duties)
  expect list.length(post_duties) == list.length(shape.duties)
  let states_bytes =
    fn(vs) {
      bytes.cat(
        list.map(list.zip(cat.states, vs), fn(p) { typed_bytes(p.2nd, p.1st.ty) }),
      )
    }
  let ms = moves(effects)
  bytes.cat(
    [
      bytes.cat(
        list.map(list.zip(shape.inputs, inputs), fn(p) { typed_bytes(p.2nd, p.1st.ty) }),
      ),
      states_bytes(pre_states),
      bytes.cat(list.map(pre_cells, bytes.u128)),
      bytes.cat(list.map(pre_duties, duty_bytes)),
      states_bytes(post_states),
      bytes.cat(list.map(post_cells, bytes.u128)),
      bytes.cat(list.map(post_duties, duty_bytes)),
      bytes.cat(list.map(effects, fn(e) { effect_bytes(cat, e) })),
      bytes.cat(
        list.map(
          shape.debit_cells,
          fn(c) {
            bytes.cat([bytes.wide(gross_of_cell(ms, c)), bytes.wide(fees_of_cell(ms, c))])
          },
        ),
      ),
    ],
  )
}

/// `MRG16-PVC-STMT-2` bytes.
pub fn statement(ctx: StatementCtxV2, body_bytes: ByteArray) -> ByteArray {
  expect and {
      bytes.len(ctx.predicate_digest) == 32,
      bytes.len(ctx.shape_digest) == 32,
      bytes.len(ctx.vk_digest) == 32,
      bytes.len(ctx.claimant_vkh) == 28,
      bytes.len(ctx.settle_hash) == 28,
      bytes.len(ctx.thread_id) == 32,
      bytes.len(ctx.consumed_txid) == 32,
      bytes.len(ctx.catalogue_digest) == 32,
      bytes.len(ctx.intents_digest) == 32,
    }
  bytes.cat(
    [
      stmt_tag, family_v2, ctx.predicate_digest, ctx.shape_digest, ctx.vk_digest,
      ctx.claimant_vkh, bytes.u8(ctx.network_id), ctx.settle_hash, ctx.thread_id,
      ctx.consumed_txid, bytes.u16(ctx.consumed_ix), bytes.u64(ctx.seq),
      ctx.catalogue_digest, ctx.intents_digest, bytes.u32(bytes.len(body_bytes)),
      body_bytes,
    ],
  )
}

/// Pre cell of a represented cell: a custody row that must exist (F-04), or the
/// external premise (its gross debit in this step).
pub fn pre_cell(
  cat: CatalogueV2,
  rows: List<BalanceRowV2>,
  ms: List<MoveV2>,
  i: Int,
  c: CellRefV2,
) -> Int {
  expect Some(acc) = catalogue.find_account(cat, c.account)
  if catalogue.is_custody(acc) {
    expect Some(row) =
      list.find(rows, fn(r) { r.asset == c.asset && r.account == c.account })
    row.amount
  } else {
    gross_of_cell(ms, i)
  }
}

/// Pre duty cell from the datum rows; a present row must carry the registry identity.
pub fn pre_duty(rows: List<DutyRowV2>, d: DutyRefV2) -> DutyValV2 {
  when list.find(rows, fn(r) { r.name == d.name }) is {
    Some(r) -> {
      expect and {
          r.asset == d.asset,
          r.debtor == d.debtor,
          r.creditor == d.creditor,
        }
      DutyValV2 { present: True, amount: r.amount }
    }
    None -> DutyValV2 { present: False, amount: 0 }
  }
}

pub fn blake2b(b: ByteArray) -> ByteArray {
  builtin.blake2b_256(b)
}
