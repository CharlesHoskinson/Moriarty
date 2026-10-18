//// Signed-intent bridge (P3-7; ABI-V2.md section 7). P3S1, PROPOSED, uncompiled. W1
//// renders the signer-independent message fragments once per settle step; the signed
//// bytes and the format are unchanged.
////
//// S2 never parses a signed document. It rebuilds the exact Core `moriarty-owner-intent/2`
//// signing message from authenticated facts plus the redeemer's limits and window, then
//// verifies the secp256k1 signature the way Core's k256 verifier does (both schemes sign
//// SHA-256 of the message). Canonical JSON = keys sorted bytewise, compact separators,
//// UInt as decimal strings; only strings in the escape-free subset are emitted.

use aiken/builtin
use aiken/collection/list
use aiken/interval.{Finite}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/bytes.{jarray, jobject, jstr, juint}
use pvc2/types.{
  AccountV2, AssetIdV2, BalanceRowV2, CatalogueV2, CellRefV2, DeclV2, DutyRowV2,
  EcdsaKey, IntentKeyV2, LimitV2, SchnorrKey, SignedIntentV2, VBool, VUInt,
  ValueV2,
}

pub const intent_profile: ByteArray = "moriarty-owner-intent/2"

pub const semantics: ByteArray = "moriarty-core/1"

/// Domain mapping identity (documentation; not part of any message). P3S1-R1 moves from
/// `/1` (target_hash = predicate digest) to `/2` (M5-a): target_hash names the predicate,
/// verifier-key and shape digests of the action entry. The two versions never produce the
/// same message: `/2` target strings are 194 bytes with two colons, `/1` strings are 64
/// hex bytes, and the R1 S2 script (anchor) differs from the P3S1 S2 script.
pub const domain_profile: ByteArray = "moriarty-cardano-intent-domain/2"

// ## JSON fragments

pub fn asset_json(a: AssetIdV2) -> ByteArray {
  expect a.scale >= 0 && a.scale < 256
  jobject(
    [
      ("issuer", jstr(a.issuer)), ("network", jstr(a.network)),
      ("reference", jstr(a.reference)), ("scale", bytes.decimal(a.scale)),
    ],
  )
}

pub fn value_json(v: ValueV2) -> ByteArray {
  when v is {
    VUInt(n) -> jobject([("type", jstr("UInt")), ("value", juint(n))])
    VBool(b) ->
      jobject(
        [
          ("type", jstr("Bool")),
          (
            "value",
            if b {
              "true"
            } else {
              "false"
            },
          ),
        ],
      )
  }
}

fn named_values(decls: List<DeclV2>, values: List<ValueV2>) -> ByteArray {
  expect list.length(decls) == list.length(values)
  let sorted =
    list.sort(
      list.zip(decls, values),
      fn(a, b) {
        if bytes.bytes_lt(a.1st.name, b.1st.name) {
          Less
        } else if a.1st.name == b.1st.name {
          Equal
        } else {
          Greater
        }
      },
    )
  expect bytes.strictly_ascending(list.map(sorted, fn(p) { p.1st.name }), bytes.bytes_lt)
  jobject(list.map(sorted, fn(p) { (p.1st.name, value_json(p.2nd)) }))
}

fn asset_id(cat: CatalogueV2, index: Int) -> AssetIdV2 {
  expect Some(m) = list.at(cat.assets, index)
  m.id
}

fn balance_json(cat: CatalogueV2, account: ByteArray, asset: Int, amount: Int) -> ByteArray {
  jobject(
    [
      ("account", jstr(account)), ("amount", juint(amount)),
      ("asset", asset_json(asset_id(cat, asset))),
    ],
  )
}

/// Canonical Core `Scenario`: datum state; step inputs; custody rows in datum order then
/// external premise rows in plan cell order; datum liabilities.
pub fn scenario_json(
  cat: CatalogueV2,
  input_decls: List<DeclV2>,
  inputs: List<ValueV2>,
  state: List<ValueV2>,
  rows: List<BalanceRowV2>,
  premises: List<(CellRefV2, Int)>,
  liabilities: List<DutyRowV2>,
) -> ByteArray {
  jobject(
    [
      (
        "balances",
        jarray(
          list.concat(
            list.map(rows, fn(r) { balance_json(cat, r.account, r.asset, r.amount) }),
            list.map(premises, fn(p) { balance_json(cat, p.1st.account, p.1st.asset, p.2nd) }),
          ),
        ),
      ),
      ("inputs", named_values(input_decls, inputs)),
      (
        "liabilities",
        jarray(
          list.map(
            liabilities,
            fn(d) {
              jobject(
                [
                  ("amount", juint(d.amount)),
                  ("asset", asset_json(asset_id(cat, d.asset))),
                  ("creditor", jstr(d.creditor)), ("debtor", jstr(d.debtor)),
                  ("name", jstr(d.name)),
                ],
              )
            },
          ),
        ),
      ),
      ("state", named_values(cat.states, state)),
    ],
  )
}

pub fn scheme_and_key(k: IntentKeyV2) -> (ByteArray, ByteArray) {
  when k is {
    SchnorrKey(x) -> ("schnorr_bip340", x)
    EcdsaKey(x) -> ("ecdsa_secp256k1_sha256", x)
  }
}

fn limit_json(cat: CatalogueV2, l: LimitV2) -> ByteArray {
  expect bytes.core_string(l.beneficiary)
  jobject(
    [
      ("asset", asset_json(asset_id(cat, l.asset))),
      ("beneficiary", jstr(l.beneficiary)), ("fee_cap", juint(l.fee_cap)),
      ("gross_cap", juint(l.gross_cap)), ("liability_cap", juint(l.liability_cap)),
      ("net_floor", juint(l.net_floor)),
    ],
  )
}

/// Everything an intent binds besides the signer's own fields
/// (`moriarty-cardano-intent-domain/2`).
pub type IntentDomainV2 {
  action: ByteArray,
  settle_hash: ByteArray,
  thread_id: ByteArray,
  predicate_digest: ByteArray,
  vk_digest: ByteArray,
  shape_digest: ByteArray,
  predecessor: OutputReference,
  seq: Int,
  scenario_hash: ByteArray,
}

/// Domain `/2` `authority.target_hash`:
/// hex(predicate_digest) ":" hex(vk_digest) ":" hex(shape_digest) of the action entry
/// (M5-a). Every digest must be 32 bytes, so the string is exactly 194 bytes.
pub fn target_hash_v2(dom: IntentDomainV2) -> ByteArray {
  expect and {
      bytes.len(dom.predicate_digest) == 32,
      bytes.len(dom.vk_digest) == 32,
      bytes.len(dom.shape_digest) == 32,
    }
  bytes.cat(
    [
      bytes.hex(dom.predicate_digest), ":", bytes.hex(dom.vk_digest), ":",
      bytes.hex(dom.shape_digest),
    ],
  )
}

/// Core `prepare_intent` checks that apply to the limits (unique asset, program asset on
/// the authority network, non-empty beneficiary, at most 256 limits).
pub fn limits_ok(cat: CatalogueV2, limits: List<LimitV2>) -> Bool {
  let assets = list.map(limits, fn(l) { l.asset })
  and {
    list.length(limits) <= 256,
    list.length(list.unique(assets)) == list.length(assets),
    list.all(
      limits,
      fn(l) {
        and {
          l.asset >= 0,
          l.asset < list.length(cat.assets),
          asset_id(cat, l.asset).network == cat.moriarty_network,
          bytes.core_string(l.beneficiary),
          bytes.is_u128(l.gross_cap),
          bytes.is_u128(l.fee_cap),
          bytes.is_u128(l.net_floor),
          bytes.is_u128(l.liability_cap),
        }
      },
    ),
  }
}

/// The signer-independent JSON values of every OwnerIntent of one settle step (W1). Each
/// field is the exact fragment `owner_intent_json` emitted per signer before W1; it is now
/// rendered and checked once per step.
pub type IntentSharedV2 {
  action: ByteArray,
  assets: ByteArray,
  anchor: ByteArray,
  network: ByteArray,
  nonce: ByteArray,
  predecessor: ByteArray,
  program_address: ByteArray,
  target_hash: ByteArray,
  program_hash: ByteArray,
  scenario_hash: ByteArray,
  source_hash: ByteArray,
}

/// Renders the shared fragments. Applies the signer-independent checks of
/// `owner_intent_json` (network, anchor, program address, target hash and predecessor
/// strings; `jstr`/`juint`/`target_hash_v2` ranges).
pub fn intent_shared(cat: CatalogueV2, dom: IntentDomainV2) -> IntentSharedV2 {
  let anchor = bytes.cat(["cardano-plutus-v3:", bytes.hex(dom.settle_hash)])
  let program_address = bytes.cat(["cardano-thread:", bytes.hex(dom.thread_id)])
  let target_hash = target_hash_v2(dom)
  let predecessor =
    bytes.cat(
      [
        bytes.hex(dom.predecessor.transaction_id), "#",
        bytes.decimal(dom.predecessor.output_index),
      ],
    )
  expect and {
      bytes.core_string(cat.moriarty_network),
      bytes.core_string(anchor),
      bytes.core_string(program_address),
      bytes.core_string(target_hash),
      bytes.core_string(predecessor),
    }
  IntentSharedV2 {
    action: jstr(dom.action),
    assets: jarray(list.map(cat.assets, fn(m) { asset_json(m.id) })),
    anchor: jstr(anchor),
    network: jstr(cat.moriarty_network),
    nonce: juint(dom.seq),
    predecessor: jstr(predecessor),
    program_address: jstr(program_address),
    target_hash: jstr(target_hash),
    program_hash: jstr(bytes.hex(cat.program_hash)),
    scenario_hash: jstr(bytes.hex(dom.scenario_hash)),
    source_hash: jstr(bytes.hex(cat.source_hash)),
  }
}

/// Canonical OwnerIntent JSON (keys in bytewise order at every level) from the shared
/// fragments plus the signer's own fields, with the signer-specific checks.
pub fn owner_intent_json_shared(
  cat: CatalogueV2,
  sh: IntentSharedV2,
  acc: AccountV2,
  si: SignedIntentV2,
) -> ByteArray {
  let (scheme, key) = scheme_and_key(acc.key)
  expect and {
      bytes.core_string(acc.name),
      si.not_before >= 0,
      si.not_before <= si.not_after,
      bytes.is_u128(si.not_after),
      limits_ok(cat, si.limits),
    }
  jobject(
    [
      ("action", sh.action),
      ("assets", sh.assets),
      (
        "authority",
        jobject(
          [
            ("anchor", sh.anchor),
            ("limits", jarray(list.map(si.limits, fn(l) { limit_json(cat, l) }))),
            ("network", sh.network), ("nonce", sh.nonce),
            ("not_after", juint(si.not_after)),
            ("not_before", juint(si.not_before)),
            ("predecessor", sh.predecessor),
            ("program_address", sh.program_address),
            ("public_key_hex", jstr(bytes.hex(key))), ("scheme", jstr(scheme)),
            ("signer", jstr(acc.name)), ("target_hash", sh.target_hash),
          ],
        ),
      ),
      ("profile", jstr(intent_profile)),
      ("program_hash", sh.program_hash),
      ("scenario_hash", sh.scenario_hash),
      ("semantics", jstr(semantics)),
      ("source_hash", sh.source_hash),
    ],
  )
}

/// Canonical OwnerIntent JSON for one signer (same bytes and checks as before W1).
pub fn owner_intent_json(
  cat: CatalogueV2,
  dom: IntentDomainV2,
  acc: AccountV2,
  si: SignedIntentV2,
) -> ByteArray {
  owner_intent_json_shared(cat, intent_shared(cat, dom), acc, si)
}

/// Core `signing_message`: tag, NUL, u32 big-endian payload length, payload (<= 64 KiB).
pub fn signing_message(payload: ByteArray) -> ByteArray {
  expect bytes.len(payload) <= 65536
  bytes.cat(["moriarty-owner-intent/2", #"00", bytes.u32(bytes.len(payload)), payload])
}

/// The whole transaction validity interval lies inside [not_before, not_after].
pub fn window_ok(si: SignedIntentV2, self: Transaction) -> Bool {
  let lo =
    when self.validity_range.lower_bound.bound_type is {
      Finite(t) ->
        if self.validity_range.lower_bound.is_inclusive {
          Some(t)
        } else {
          Some(t + 1)
        }
      _ -> None
    }
  let hi =
    when self.validity_range.upper_bound.bound_type is {
      Finite(t) ->
        if self.validity_range.upper_bound.is_inclusive {
          Some(t)
        } else {
          Some(t - 1)
        }
      _ -> None
    }
  when (lo, hi) is {
    (Some(l), Some(h)) -> l >= si.not_before && h <= si.not_after && l <= h
    _ -> False
  }
}

/// A signature to verify once every other check has passed.
pub type SigCheckV2 {
  key: IntentKeyV2,
  digest: ByteArray,
  signature: ByteArray,
}

/// Core k256 semantics: Schnorr BIP-340 and ECDSA (low-S) both over SHA-256(message).
pub fn verify(c: SigCheckV2) -> Bool {
  when c.key is {
    SchnorrKey(k) ->
      builtin.verify_schnorr_secp256k1_signature(k, c.digest, c.signature)
    EcdsaKey(k) ->
      builtin.verify_ecdsa_secp256k1_signature(k, c.digest, c.signature)
  }
}
