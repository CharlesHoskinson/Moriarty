//// C2: ABI v2 claim issuer (mint) and holder (spend). P3S1-R1 W0/W1 successor, PROPOSED,
//// uncompiled. R1 named the release components (RP-3); W1 keeps each as one function and
//// moves the named list to the test-only `pvc2/diag` module.
////
//// Reused starting material: the C1 structure of `claim-utxo-abi-v1-combined-repair-
//// candidate-20261015/source/aiken/lib/pvc/claim1.ak` (R6, R7, D9, N1'), re-versioned for
//// `EnvV2`. A claim certifies only "a Groth16 proof verifies for this statement digest
//// under this VK". It confers no authority: S2 recomputes the statement from
//// authenticated facts, pins the VK through the thread catalogue and separately demands
//// signed intents. Issuance stays permissionless.

use aiken/builtin
use aiken/collection/dict
use aiken/collection/list
use aiken/interval.{Finite}
use cardano/address.{Script}
use cardano/assets.{PolicyId}
use cardano/transaction.{InlineDatum, Input, Output, OutputReference, Transaction}
use pvc2/bytes
use pvc2/core
use pvc2/groth16
use pvc2/types.{EnvV2}

pub fn envelope_name(raw: Data) -> ByteArray {
  builtin.blake2b_256(builtin.serialise_data(raw))
}

pub fn holds_policy(o: Output, policy: PolicyId) -> Bool {
  !dict.is_empty(assets.tokens(o.value, policy))
}

pub fn well_formed(env: EnvV2) -> Bool {
  and {
    env.version == 2,
    env.claim_kind == 0,
    env.family_id == core.family_v2,
    bytes.len(env.predicate_digest) == 32,
    bytes.len(env.vk_digest) == 32,
    bytes.len(env.claimant_vkh) == 28,
    bytes.len(env.settle_outref.transaction_id) == 32,
    env.settle_outref.output_index >= 0,
    env.settle_outref.output_index < 65536,
    bytes.len(env.statement_digest) == 32,
  }
}

/// Issue: exactly one claim token named H(envelope) minted into exactly one C2 output,
/// after the pairing check on the envelope's statement digest (checked last).
pub fn issue(
  policy_id: PolicyId,
  vk: ByteArray,
  proof: ByteArray,
  self: Transaction,
) -> Bool {
  expect [Pair(name, qty)] = dict.to_pairs(assets.tokens(self.mint, policy_id))
  expect [claim] = list.filter(self.outputs, fn(o) { holds_policy(o, policy_id) })
  expect Script(holder) = claim.address.payment_credential
  expect InlineDatum(raw) = claim.datum
  expect env: EnvV2 = raw
  and {
    qty == 1,
    holder == policy_id,
    assets.quantity_of(claim.value, policy_id, name) == 1,
    dict.size(assets.tokens(claim.value, policy_id)) == 1,
    claim.reference_script == None,
    name == envelope_name(raw),
    well_formed(env),
    env.vk_digest == builtin.blake2b_256(vk),
    groth16.verify_digest(vk, proof, env.statement_digest),
  }
}

/// Burn: only negative quantities of this policy.
pub fn burn_only(policy_id: PolicyId, self: Transaction) -> Bool {
  list.all(dict.values(assets.tokens(self.mint, policy_id)), fn(q) { q < 0 })
}

fn after_expiry(env: EnvV2, self: Transaction) -> Bool {
  when self.validity_range.lower_bound.bound_type is {
    Finite(t) -> t > env.expiry
    _ -> False
  }
}

/// Parsed release: the spent claim input, its script hash and its envelope token name.
pub type ReleaseBaseV2 {
  env: EnvV2,
  tx: Transaction,
  own: Input,
  own_hash: ByteArray,
  name: ByteArray,
}

pub fn release_base(
  own_ref: OutputReference,
  raw: Data,
  env: EnvV2,
  self: Transaction,
) -> ReleaseBaseV2 {
  expect Some(own) = list.find(self.inputs, fn(i) { i.output_reference == own_ref })
  expect Script(own_hash) = own.output.address.payment_credential
  ReleaseBaseV2 { env, tx: self, own, own_hash, name: envelope_name(raw) }
}

// Release components (P3S1-R1, RP-3): the claimant signs every path (N1'); exactly one
// own token named by the envelope; no output keeps this policy (so the token is burned);
// and the named thread outref is spent in this transaction, or the validity lower bound
// is after expiry. W1: each component is defined once here; `release` conjoins them and
// `diag.release_parts` names the same calls.

pub fn release_own_token(v: ReleaseBaseV2) -> Bool {
  assets.quantity_of(v.own.output.value, v.own_hash, v.name) == 1
}

pub fn release_single_token(v: ReleaseBaseV2) -> Bool {
  dict.size(assets.tokens(v.own.output.value, v.own_hash)) == 1
}

pub fn release_burned(v: ReleaseBaseV2) -> Bool {
  !list.any(v.tx.outputs, fn(o) { holds_policy(o, v.own_hash) })
}

pub fn release_claimant_signs(v: ReleaseBaseV2) -> Bool {
  list.has(v.tx.extra_signatories, v.env.claimant_vkh)
}

pub fn release_spent_or_expired(v: ReleaseBaseV2) -> Bool {
  or {
    list.any(v.tx.inputs, fn(i) { i.output_reference == v.env.settle_outref }),
    after_expiry(v.env, v.tx),
  }
}

/// Production release gate.
pub fn release(
  own_ref: OutputReference,
  raw: Data,
  env: EnvV2,
  self: Transaction,
) -> Bool {
  let v = release_base(own_ref, raw, env, self)
  and {
    release_own_token(v),
    release_single_token(v),
    release_burned(v),
    release_claimant_signs(v),
    release_spent_or_expired(v),
  }
}
