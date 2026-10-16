//// W1 production-gate tests (PROPOSED; written by hand, never compiled or run by their
//// author). They call the unnamed production gates (`thread.settle_gate`,
//// `thread.genesis`, `thread.revoke`, `thread.close`, `claim.release`) on the frozen R1
//// fixtures. Expectations come from EXPECTED-CASES-R1.json (`r1.evidence`, `falseParts`),
//// which predates W1:
//// - BOOLEAN case: the production gate refuses WITHOUT halting (`None`/False). This is
////   predicted from the source argument in `pvc2/diag` (diagnostics evaluate without
////   halting on these cases, and the production gate evaluates a subset of the same
////   expressions). A halt here fails the test and falsifies that argument.
//// - ERROR-PATH case: `fail` test; it passes on a halt or on refusal, so it is refusal
////   evidence only. No isolation and no ERROR-to-False conversion is claimed.
//// Positive cases are covered by the unchanged `settle_r1_test` `accepted` tests, which
//// require the named diagnostics to hold AND the production `settle_checks` to return
//// one signature check per intent.

use cardano/transaction.{OutputReference, Transaction}
use pvc2/claim
use pvc2/fixtures_r1 as fx
use pvc2/intent.{SigCheckV2}
use pvc2/test_ctx
use pvc2/thread
use pvc2/types.{
  Close, EnvV2, Genesis, Revoke, S2Mint, SettleRedeemerV2, ThreadDatumV2,
}

fn settle_parts(
  ctx: ByteArray,
) -> (ThreadDatumV2, OutputReference, SettleRedeemerV2, Transaction) {
  let (own_ref, raw, rd, tx) = test_ctx.spending_of(ctx)
  expect td: ThreadDatumV2 = raw
  expect r: SettleRedeemerV2 = rd
  (td, own_ref, r, tx)
}

fn gate_of(c2: ByteArray, ctx: ByteArray) -> Option<List<SigCheckV2>> {
  let (td, own_ref, r, tx) = settle_parts(ctx)
  thread.settle_gate(c2, td, own_ref, r, tx)
}

/// BOOLEAN settle case: the production gate returns `None` (no halt).
fn gate_none(c2: ByteArray, ctx: ByteArray) -> Bool {
  when gate_of(c2, ctx) is {
    None -> True
    Some(_) -> False
  }
}

/// ERROR-PATH settle case: used only in `fail` tests (halt or False both pass).
fn gate_some(ctx: ByteArray) -> Bool {
  when gate_of(fx.c2_hash, ctx) is {
    Some(_) -> True
    None -> False
  }
}

fn genesis_ok(ctx: ByteArray) -> Bool {
  let (policy, rd, tx) = test_ctx.minting_of(ctx)
  expect m: S2Mint = rd
  expect Genesis { seed } = m
  thread.genesis(fx.c2_hash, policy, seed, tx)
}

fn revoke_ok(ctx: ByteArray) -> Bool {
  let (own_ref, raw, rd, tx) = test_ctx.spending_of(ctx)
  expect td: ThreadDatumV2 = raw
  expect r: SettleRedeemerV2 = rd
  expect Revoke { signer, signature, continuation } = r
  thread.revoke(td, own_ref, signer, signature, continuation, tx)
}

fn close_ok(ctx: ByteArray) -> Bool {
  let (own_ref, raw, rd, tx) = test_ctx.spending_of(ctx)
  expect td: ThreadDatumV2 = raw
  expect r: SettleRedeemerV2 = rd
  expect Close { reserve_output } = r
  thread.close(td, own_ref, reserve_output, tx)
}

fn release_ok(ctx: ByteArray) -> Bool {
  let (own_ref, raw, _, tx) = test_ctx.spending_of(ctx)
  expect env: EnvV2 = raw
  claim.release(own_ref, raw, env, tx)
}

// ## Settle BOOLEAN cases: production gate refuses with None

test w1_h01_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h01)
}

test w1_h02_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h02)
}

test w1_h03_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h03)
}

test w1_h04_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h04)
}

test w1_h05_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h05)
}

test w1_h06_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h06)
}

test w1_h07_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h07)
}

test w1_h09_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h09)
}

test w1_h11_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h11)
}

test w1_h12_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h12)
}

test w1_h13_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h13)
}

test w1_h20_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h20)
}

test w1_h21_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h21)
}

test w1_h22_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h22)
}

test w1_h23_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h23)
}

test w1_h24_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h24)
}

test w1_h25_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h25)
}

test w1_h26_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h26)
}

test w1_h27_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h27)
}

test w1_h28_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h28)
}

test w1_h29_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h29)
}

test w1_h30_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h30)
}

test w1_h31_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h31)
}

test w1_h32_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h32)
}

test w1_h34_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h34)
}

test w1_h35_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h35)
}

test w1_h36_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h36)
}

test w1_h37_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h37)
}

test w1_h38_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h38)
}

test w1_h39_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h39)
}

test w1_h42_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h42)
}

test w1_h43_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h43)
}

test w1_h44_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h44)
}

test w1_h45_0_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h45_0)
}

test w1_h45_1_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h45_1)
}

test w1_h56_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h56)
}

test w1_h57_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h57)
}

test w1_h60_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h60)
}

test w1_h61_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h61)
}

test w1_h62_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h62)
}

test w1_h65_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h65)
}

test w1_h66_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h66)
}

test w1_h69_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h69)
}

test w1_h70_0_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h70_0)
}

test w1_h70_1_gate_none() {
  gate_none(fx.c2_hash, fx.ctx_h70_1)
}

/// H59: the P02 context under an S2 parameterized by another C2 hash.
test w1_h59_gate_none() {
  gate_none(fx.c2_hash_foreign, fx.ctx_p02)
}

// ## Settle ERROR-PATH cases: refusal only (halt or None)

test w1_h08_gate_refuses() fail {
  gate_some(fx.ctx_h08)
}

test w1_h10_gate_refuses() fail {
  gate_some(fx.ctx_h10)
}

test w1_h14_gate_refuses() fail {
  gate_some(fx.ctx_h14)
}

test w1_h33_gate_refuses() fail {
  gate_some(fx.ctx_h33)
}

test w1_h40_gate_refuses() fail {
  gate_some(fx.ctx_h40)
}

test w1_h41_gate_refuses() fail {
  gate_some(fx.ctx_h41)
}

test w1_h64_gate_refuses() fail {
  gate_some(fx.ctx_h64)
}

test w1_h67_gate_refuses() fail {
  gate_some(fx.ctx_h67)
}

test w1_h68_gate_refuses() fail {
  gate_some(fx.ctx_h68)
}

// ## Genesis, revoke, close and release BOOLEAN cases: production gate is False (no halt)

test w1_h46_genesis_false() {
  !genesis_ok(fx.ctx_h46)
}

test w1_h47_genesis_false() {
  !genesis_ok(fx.ctx_h47)
}

test w1_h48_genesis_false() {
  !genesis_ok(fx.ctx_h48)
}

test w1_h58_genesis_false() {
  !genesis_ok(fx.ctx_h58)
}

test w1_h49_revoke_false() {
  !revoke_ok(fx.ctx_h49)
}

test w1_h63_revoke_false() {
  !revoke_ok(fx.ctx_h63)
}

test w1_h51_close_false() {
  !close_ok(fx.ctx_h51)
}

test w1_h52_close_false() {
  !close_ok(fx.ctx_h52)
}

test w1_h53_release_false() {
  !release_ok(fx.ctx_h53)
}

test w1_h54_release_false() {
  !release_ok(fx.ctx_h54)
}

test w1_h55_release_false() {
  !release_ok(fx.ctx_h55)
}
