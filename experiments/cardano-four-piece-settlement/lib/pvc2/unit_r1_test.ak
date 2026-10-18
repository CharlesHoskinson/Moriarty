//// P3S1-R1 unit tests (AUTH: expected values are the R1 author's derivations from the
//// R1 source and ABI-V2.md; not an independent oracle; never compiled or run).

use cardano/transaction.{OutputReference}
use pvc2/bytes
use pvc2/core
use pvc2/diag
use pvc2/intent.{IntentDomainV2}
use pvc2/types.{CellRefV2, ETransfer, ShapeV2}

fn cell(asset: Int, account: ByteArray) -> CellRefV2 {
  CellRefV2 { asset, account }
}

fn shape_with(cells: List<CellRefV2>) -> ShapeV2 {
  ShapeV2 {
    inputs: [],
    cells,
    duties: [],
    effects: [ETransfer { from_cell: 0, to_cell: 1 }],
    debit_cells: [0],
    statement_count: 1,
    liability_statements: 0,
  }
}

// ## RR-01: structural cell order and distinctness

test cells_ordered_accepts_plan_order() {
  and {
    core.cells_ordered(shape_with([cell(0, "escrow"), cell(0, "fees"), cell(0, "seller")])),
    core.cells_ordered(
      shape_with([cell(0, "alpha"), cell(0, "vault"), cell(1, "alpha"), cell(1, "vault")]),
    ),
    core.cells_ordered(shape_with([])),
    core.cells_ordered(shape_with([cell(3, "x")])),
  }
}

test cells_ordered_refuses_duplicate_cell() {
  !core.cells_ordered(shape_with([cell(0, "escrow"), cell(0, "escrow")]))
}

test cells_ordered_refuses_descending_accounts() {
  !core.cells_ordered(shape_with([cell(0, "seller"), cell(0, "escrow")]))
}

test cells_ordered_refuses_descending_assets() {
  !core.cells_ordered(shape_with([cell(1, "alpha"), cell(0, "vault")]))
}

/// Shorter prefix sorts first, as Rust `String` order does ("ab" < "abc").
test cell_lt_is_bytewise_with_prefix_first() {
  and {
    core.cell_lt(cell(0, "ab"), cell(0, "abc")),
    !core.cell_lt(cell(0, "abc"), cell(0, "ab")),
    !core.cell_lt(cell(0, "ab"), cell(0, "ab")),
  }
}

// ## Named gate components helpers

test false_parts_lists_false_components_in_order() {
  and {
    diag.false_parts([("a", True), ("b", False), ("c", True), ("d", False)]) == [
      "b", "d",
    ],
    diag.false_parts([]) == [],
    diag.all_parts([("a", True)]),
    !diag.all_parts([("a", True), ("b", False)]),
  }
}

// ## M5-a: domain /2 target_hash

fn dom(p: ByteArray, v: ByteArray, s: ByteArray) -> IntentDomainV2 {
  IntentDomainV2 {
    action: "partial",
    settle_hash: #"a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2",
    thread_id: bytes.zero32,
    predicate_digest: p,
    vk_digest: v,
    shape_digest: s,
    predecessor: OutputReference { transaction_id: bytes.zero32, output_index: 0 },
    seq: 1,
    scenario_hash: bytes.zero32,
  }
}

test target_hash_v2_names_three_digests() {
  let p = #"0101010101010101010101010101010101010101010101010101010101010101"
  let v = #"0202020202020202020202020202020202020202020202020202020202020202"
  let s = #"0303030303030303030303030303030303030303030303030303030303030303"
  let t = intent.target_hash_v2(dom(p, v, s))
  and {
    bytes.len(t) == 194,
    t == bytes.cat([bytes.hex(p), ":", bytes.hex(v), ":", bytes.hex(s)]),
    bytes.core_string(t),
  }
}

test target_hash_v2_refuses_short_digest() fail {
  bytes.len(intent.target_hash_v2(dom(#"01", bytes.zero32, bytes.zero32))) > 0
}

test domain_profile_is_v2() {
  intent.domain_profile == "moriarty-cardano-intent-domain/2"
}
