//// Catalogue (P3-1 asset map, P3-2 account map) validation, lookups, custody value and
//// native co-authority. P3S1, PROPOSED, uncompiled.

use aiken/builtin
use aiken/collection/list
use aiken/collection/pairs
use cardano/address.{Credential, Inline, Pointer, Script, VerificationKey}
use cardano/assets.{Value}
use cardano/transaction.{Transaction}
use pvc2/bytes
use pvc2/types.{
  AccountV2, ActionEntryV2, AssetMapV2, BalanceRowV2, CardanoUnit, CatalogueV2,
  Custody, DeclV2, EcdsaKey, External, IntentKeyV2, Lovelace, SchnorrKey, Token,
}

pub fn find_account(cat: CatalogueV2, name: ByteArray) -> Option<AccountV2> {
  list.find(cat.accounts, fn(a) { a.name == name })
}

pub fn is_custody(acc: AccountV2) -> Bool {
  when acc.kind is {
    Custody -> True
    External { .. } -> False
  }
}

pub fn asset_map(cat: CatalogueV2, index: Int) -> AssetMapV2 {
  expect Some(m) = list.at(cat.assets, index)
  m
}

/// Catalogue digest bound in the statement: blake2b_256(serialise_data(catalogue)).
pub fn digest(cat: CatalogueV2) -> ByteArray {
  let d: Data = cat
  builtin.blake2b_256(builtin.serialise_data(d))
}

fn key_ok(k: IntentKeyV2) -> Bool {
  when k is {
    SchnorrKey(x) -> bytes.len(x) == 32
    EcdsaKey(x) -> and {
        bytes.len(x) == 33,
        or {
          builtin.index_bytearray(x, 0) == 2,
          builtin.index_bytearray(x, 0) == 3,
        },
      }
  }
}

fn credential_ok(c: Credential) -> Bool {
  when c is {
    VerificationKey(h) -> bytes.len(h) == 28
    Script(h) -> bytes.len(h) == 28
  }
}

fn account_ok(acc: AccountV2) -> Bool {
  and {
    bytes.core_string(acc.name),
    key_ok(acc.key),
    when acc.native is {
      None -> True
      Some(c) -> credential_ok(c)
    },
    when acc.kind is {
      Custody -> True
      External { address, .. } -> and {
          credential_ok(address.payment_credential),
          when address.stake_credential is {
            None -> True
            Some(Inline(c)) -> credential_ok(c)
            Some(Pointer { .. }) -> False
          },
        }
    },
  }
}

fn unit_ok(u: CardanoUnit) -> Bool {
  when u is {
    Lovelace -> True
    Token { policy, name } -> and {
        bytes.len(policy) == 28,
        bytes.len(name) <= 32,
      }
  }
}

fn asset_ok(m: AssetMapV2) -> Bool {
  and {
    bytes.core_string(m.id.network),
    bytes.json_safe(m.id.issuer),
    bytes.json_safe(m.id.reference),
    m.id.scale >= 0,
    m.id.scale < 256,
    m.exp >= 0,
    m.exp <= 38,
    when m.unit is {
      None -> True
      Some(u) -> unit_ok(u)
    },
  }
}

/// P3-1: no two Moriarty assets map to the same Cardano unit.
fn units_injective(assets: List<AssetMapV2>) -> Bool {
  let units = list.filter_map(assets, fn(m) { m.unit })
  list.length(list.unique(units)) == list.length(units)
}

fn decl_ok(d: DeclV2) -> Bool {
  and {
    bytes.core_string(d.name),
    or {
      d.ty == 0,
      d.ty == 1,
    },
  }
}

fn action_ok(cat: CatalogueV2, a: ActionEntryV2) -> Bool {
  and {
    bytes.core_string(a.name),
    a.index >= 0,
    a.index < 65536,
    bytes.len(a.predicate_digest) == 32,
    bytes.len(a.vk_digest) == 32,
    bytes.len(a.shape_digest) == 32,
    list.all(a.advancers, fn(n) { find_account(cat, n) != None }),
  }
}

/// Complete catalogue well-formedness, checked once at genesis; the catalogue is then
/// immutable (every continuation datum carries it unchanged).
pub fn well_formed(cat: CatalogueV2) -> Bool {
  and {
    or {
      cat.network_id == 0,
      cat.network_id == 1,
    },
    bytes.core_string(cat.moriarty_network),
    bytes.len(cat.program_hash) == 32,
    bytes.len(cat.source_hash) == 32,
    !list.is_empty(cat.assets),
    list.length(cat.assets) < 65536,
    list.all(cat.assets, asset_ok),
    bytes.strictly_ascending(
      list.map(cat.assets, fn(m) { m.id }),
      bytes.asset_lt,
    ),
    units_injective(cat.assets),
    list.all(cat.accounts, account_ok),
    bytes.strictly_ascending(
      list.map(cat.accounts, fn(a) { a.name }),
      bytes.bytes_lt,
    ),
    list.all(cat.states, decl_ok),
    bytes.strictly_ascending(list.map(cat.states, fn(d) { d.name }), bytes.bytes_lt),
    !list.is_empty(cat.actions),
    list.all(cat.actions, fn(a) { action_ok(cat, a) }),
    bytes.strictly_ascending(list.map(cat.actions, fn(a) { a.index }), fn(x, y) { x < y }),
    credential_ok(cat.reserve_owner.payment_credential),
    cat.reserve_floor >= 0,
  }
}

/// Native co-authority present in this transaction: a key signature, or a script
/// credential exercised by a withdrawal or by spending one of its inputs.
pub fn native_ok(c: Credential, self: Transaction) -> Bool {
  when c is {
    VerificationKey(h) -> list.has(self.extra_signatories, h)
    Script(_) -> or {
        pairs.has_key(self.withdrawals, c),
        list.any(self.inputs, fn(i) { i.output.address.payment_credential == c }),
      }
  }
}

/// The Value of `quantity` base units of a Moriarty asset under the catalogue map.
/// Fails for an unmapped asset with a nonzero quantity, or outside [0, 2^63).
pub fn unit_value(m: AssetMapV2, amount: Int) -> Value {
  if amount == 0 {
    assets.zero
  } else {
    expect Some(u) = m.unit
    let q = amount * bytes.pow10(m.exp)
    expect amount > 0 && q < bytes.two_63
    when u is {
      Lovelace -> assets.from_lovelace(q)
      Token { policy, name } -> assets.from_asset(policy, name, q)
    }
  }
}

/// P3-5 exact custody value: thread token, every custody row under its mapped unit, and
/// the reserve lovelace. Nothing else may sit in custody.
pub fn custody_value(
  cat: CatalogueV2,
  rows: List<BalanceRowV2>,
  reserve: Int,
  policy: ByteArray,
  thread_id: ByteArray,
) -> Value {
  expect reserve >= 0 && reserve < bytes.two_63
  let base =
    assets.from_lovelace(reserve)
      |> assets.add(policy, thread_id, 1)
  let v =
    list.foldl(
      rows,
      base,
      fn(r, acc) { assets.merge(acc, unit_value(asset_map(cat, r.asset), r.amount)) },
    )
  expect
    list.all(assets.flatten(v), fn(t) { t.3rd > 0 && t.3rd < bytes.two_63 })
  v
}
