//// P3S1 unit tests (written after FREEZE-1). Expected values are spec constants from
//// ABI-V2.md and Core `intent.rs`/`runtime.rs` semantics as read by the author; they are
//// author derivations, not an independent oracle.

use pvc2/bytes
use pvc2/core.{CDuty, CMove, CState, MoveV2, StatementCtxV2}
use pvc2/groth16
use pvc2/types.{DutyRowV2, DutyValV2, LimitV2, VBool, VUInt}

// ## Encoders

test hex_lowercase() {
  bytes.hex(#"00ff10ab") == "00ff10ab"
}

test decimal_bounds() {
  and {
    bytes.decimal(0) == "0",
    bytes.decimal(10) == "10",
    bytes.decimal(340282366920938463463374607431768211455) == "340282366920938463463374607431768211455",
  }
}

test juint_refuses_2_pow_128() fail {
  bytes.juint(340282366920938463463374607431768211456) == ""
}

test jstr_refuses_quote() fail {
  bytes.jstr(#"22") == ""
}

test jstr_refuses_backslash() fail {
  bytes.jstr(#"5c") == ""
}

test json_safe_refuses_controls_and_del() {
  and {
    !bytes.json_safe(#"1f"),
    !bytes.json_safe(#"7f"),
    bytes.json_safe("cardano-thread:ab#0"),
  }
}

test core_string_limits() {
  and {
    !bytes.core_string(""),
    bytes.core_string("a"),
  }
}

test wide_is_18_bytes() {
  bytes.len(bytes.wide(22300745198530623141535718272648361505980415)) == 18
}

test wide_refuses_2_pow_144() fail {
  bytes.len(bytes.wide(22300745198530623141535718272648361505980416)) == 18
}

test u16_refuses_65536() fail {
  bytes.u16(65536) == #"0000"
}

test json_object_and_array() {
  and {
    // {"a":1,"b":[]}
    bytes.jobject([("a", "1"), ("b", bytes.jarray([]))]) == #"7b2261223a312c2262223a5b5d7d",
    bytes.jarray(["1", "2"]) == "[1,2]",
  }
}

test scalars_split_big_endian() {
  let d =
    #"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
  groth16.scalars_of_digest(d) == (
    5233100606242806050955395731361295,
    21356283574076891493948969979685445151,
  )
}

test groth16_refuses_short_vk() fail {
  groth16.verify_digest(#"00", #"00", bytes.zero32)
}

// ## Core check_effects port

fn mv(fee: Bool, from: ByteArray, to: ByteArray, amount: Int) -> core.CoreEffectV2 {
  CMove(MoveV2 { fee, asset: 0, from, to, from_cell: 0, to_cell: 1, amount })
}

fn limit(beneficiary: ByteArray, gross: Int, fee: Int, net: Int, liab: Int) -> LimitV2 {
  LimitV2 {
    asset: 0,
    beneficiary,
    gross_cap: gross,
    fee_cap: fee,
    net_floor: net,
    liability_cap: liab,
  }
}

/// Core: a Fee paid by the beneficiary when the beneficiary is not the signer is a
/// beneficiary debit (net = credit - debit - signer fees).
test check_effects_beneficiary_fee_is_debit() {
  let effects = [mv(False, "x", "b", 10), mv(True, "b", "f", 3)]
  and {
    core.check_effects("a", [limit("b", 0, 0, 7, 0)], effects, []),
    !core.check_effects("a", [limit("b", 0, 0, 8, 0)], effects, []),
  }
}

test check_effects_signer_debit_needs_asset_limit() {
  !core.check_effects("a", [], [mv(False, "a", "b", 1)], [])
}

test check_effects_closing_duty_needs_limit() {
  let closing =
    [DutyRowV2 { name: "d", asset: 0, debtor: "a", creditor: "c", amount: 5 }]
  and {
    !core.check_effects("a", [], [], closing),
    core.check_effects("a", [limit("a", 0, 0, 0, 5)], [], closing),
    !core.check_effects("a", [limit("a", 0, 0, 0, 4)], [], closing),
  }
}

test check_effects_net_negative_refused() {
  !core.check_effects(
    "a",
    [limit("a", 10, 0, 0, 0)],
    [mv(False, "a", "b", 10)],
    [],
  )
}

test check_effects_duplicate_limit_refused() {
  !core.check_effects("a", [limit("b", 0, 0, 0, 0), limit("c", 0, 0, 0, 0)], [], [])
}

test check_effects_overflow_errors() fail {
  core.check_effects(
    "a",
    [limit("b", 0, 0, 0, 0)],
    [
      mv(False, "x", "b", 340282366920938463463374607431768211455),
      mv(False, "y", "b", 1),
    ],
    [],
  )
}

// ## Recomputation

test self_transfer_leaves_cell_unchanged() {
  let ms = [MoveV2 { fee: False, asset: 0, from: "a", to: "a", from_cell: 0, to_cell: 0, amount: 4 }]
  core.cells_recomputed([9], [9], ms)
}

test cell_inflation_refused() {
  let ms = [MoveV2 { fee: False, asset: 0, from: "a", to: "b", from_cell: 0, to_cell: 1, amount: 4 }]
  and {
    core.cells_recomputed([9, 0], [5, 4], ms),
    !core.cells_recomputed([9, 0], [5, 5], ms),
  }
}

fn duty(before: Int, after: Int, discharge: Bool) -> core.CoreEffectV2 {
  CDuty {
    duty: 0,
    name: "d",
    asset: 0,
    debtor: "a",
    creditor: "c",
    before,
    after,
    discharge,
  }
}

test accrual_makes_present() {
  core.duty_recomputed(
    0,
    DutyValV2 { present: False, amount: 0 },
    DutyValV2 { present: True, amount: 7 },
    [duty(0, 7, False)],
  )
}

test present_zero_survives_full_discharge() {
  core.duty_recomputed(
    0,
    DutyValV2 { present: True, amount: 7 },
    DutyValV2 { present: True, amount: 0 },
    [duty(7, 0, True)],
  )
}

test discharge_of_absent_duty_refused() fail {
  core.duty_recomputed(
    0,
    DutyValV2 { present: False, amount: 0 },
    DutyValV2 { present: False, amount: 0 },
    [duty(0, 0, True)],
  )
}

test erasing_present_row_refused() {
  !core.duty_recomputed(
    0,
    DutyValV2 { present: True, amount: 3 },
    DutyValV2 { present: False, amount: 0 },
    [],
  )
}

test state_chain_rebinding() {
  core.state_recomputed(
    0,
    VUInt(1),
    VUInt(4),
    [
      CState { state: 0, before: VUInt(1), after: VUInt(2) },
      CState { state: 0, before: VUInt(2), after: VUInt(4) },
      CState { state: 1, before: VBool(False), after: VBool(True) },
    ],
  )
}

test statement_refuses_short_claimant() fail {
  let ctx =
    StatementCtxV2 {
      predicate_digest: bytes.zero32,
      shape_digest: bytes.zero32,
      vk_digest: bytes.zero32,
      claimant_vkh: #"00",
      network_id: 0,
      settle_hash: #"00000000000000000000000000000000000000000000000000000000",
      thread_id: bytes.zero32,
      consumed_txid: bytes.zero32,
      consumed_ix: 0,
      seq: 0,
      catalogue_digest: bytes.zero32,
      intents_digest: bytes.zero32,
    }
  bytes.len(core.statement(ctx, #"")) > 0
}
