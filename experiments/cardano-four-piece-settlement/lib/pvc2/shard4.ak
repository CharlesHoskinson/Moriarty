//// Four-piece state shard (PROPOSED, unreviewed, uncompiled, unrun). The single VS state
//// shard of `pvc2/shard` is replaced by four withdraw-zero pieces. Each runs once per
//// settle transaction, and the S2' Settle path requires all four:
////
////   C2 -> VI(c2_hash) -> VS3(c2_hash, vi_hash)
////   VS1, VS2, VS4 (no parameter)
////   S2'(c2_hash, vi_hash, vs1_hash, vs2_hash, vs3_hash, vs4_hash)
////
//// The 15 per-thread W1 settle components VS checked, and the once-per-transaction global
//// allocation, are partitioned so that each runs in exactly one piece:
////
////   VS1 structure   shape_structure, shape_cells, own_mint, topup, reserve_floor,
////                   global_outputs (once per transaction)
////   VS2 accounting  recomputed, bounds, intent_effects
////   VS3 identity    claim_script, shape_digest, claim, statement_digest
////   VS4 custody     custody_pre, continuation, payouts
////
//// VI (`pvc2/shard`, unchanged) keeps authority_shape, intent_window, intent_native and
//// every signature. Every condition is still the unchanged W1 `pub fn` in `pvc2/thread`.
//// Two values cross pieces, and neither is a new published value. VS3 consumes VI's
//// per-outref intents digest exactly as VS did. VS4 builds the next datum from the claim
//// envelope's statement digest, and VS3 checks that digest equals the recomputed one.
////
//// Each piece checks its own tag and exact coverage of the named policy. S2' checks, per
//// Settle input, that every piece's withdrawal is present with exactly one redeemer of the
//// right tag naming this policy and listing this outref, and that the six identities and
//// the policy are pairwise distinct. No datum field, status flag or reference input is
//// accepted as provenance.

use aiken/collection/list
use cardano/address.{Credential, Script}
use cardano/assets.{PolicyId}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/bytes
use pvc2/shard.{AuthShardV1, CoveredV1}
use pvc2/thread.{SettleBaseV2, SettleSnapshotV2}
use pvc2/types.{CatalogueV2, ThreadDatumV2}

// ## Piece redeemer (new type; Data constructor 0, fields positional)

/// Redeemer of piece `piece` (1..4): the thread policy and every covered outref, in
/// transaction input order.
pub type VsPieceV1 {
  piece: Int,
  thread_policy: ByteArray,
  threads: List<OutputReference>,
}

/// The settle base of VS1, VS2 and VS4, none of whose components reads `c2_hash`.
const no_c2: ByteArray = #""

fn base(c2_hash: ByteArray, c: CoveredV1, self: Transaction) -> SettleBaseV2 {
  thread.settle_base(c2_hash, c.td, c.input.output_reference, c.redeemer, self)
}

/// Exact coverage of piece `k`: the redeemer carries tag `k` and a 28-byte policy, and its
/// thread list equals the non-empty covered set of that policy in transaction input order.
pub fn coverage(
  k: Int,
  r: VsPieceV1,
  self: Transaction,
) -> Option<List<CoveredV1>> {
  if r.piece == k && bytes.len(r.thread_policy) == 28 {
    let cov = shard.covered(r.thread_policy, self)
    let exact =
      and {
        !list.is_empty(cov),
        list.map(cov, fn(c) { c.input.output_reference }) == r.threads,
      }
    if exact {
      Some(cov)
    } else {
      None
    }
  } else {
    None
  }
}

// ## VS1: structure

/// VS1's part of one thread: structural shape, own-policy mint, top-up and reserve floor.
pub fn structure_thread(c: CoveredV1, self: Transaction) -> Bool {
  let b = base(no_c2, c, self)
  and {
    thread.settle_shape_structure(b),
    thread.settle_shape_cells(b),
    thread.settle_own_mint(b),
    thread.settle_topup(b),
    thread.settle_reserve_floor(b),
  }
}

/// The VS1 validator body: exact coverage, global output allocation once per transaction,
/// then every covered thread's VS1 part.
pub fn vs1_structure(r: VsPieceV1, self: Transaction) -> Bool {
  when coverage(1, r, self) is {
    Some(cov) -> and {
        thread.global_outputs_distinct(r.thread_policy, self),
        list.all(cov, fn(c) { structure_thread(c, self) }),
      }
    None -> False
  }
}

// ## VS2: accounting

/// VS2's part of one thread: recomputed post values, Core bounds, and every signer's
/// gross, fee, net and liability limits on the complete next snapshot.
pub fn accounting_thread(c: CoveredV1, self: Transaction) -> Bool {
  let b = base(no_c2, c, self)
  let e = thread.settle_effects(b)
  if thread.settle_recomputed(b, e) && thread.settle_bounds(b, e) {
    thread.settle_intent_effects(b, e, thread.settle_snapshot(b, e))
  } else {
    False
  }
}

/// The VS2 validator body.
pub fn vs2_accounting(r: VsPieceV1, self: Transaction) -> Bool {
  when coverage(2, r, self) is {
    Some(cov) -> list.all(cov, fn(c) { accounting_thread(c, self) })
    None -> False
  }
}

// ## VS3: identity

/// VS3's part of one thread: the reviewed claim script, the pinned shape digest, the
/// claim envelope, and the statement digest built with the intents digest VI published
/// for this outref.
pub fn identity_thread(
  c2_hash: ByteArray,
  c: CoveredV1,
  intents_digest: ByteArray,
  self: Transaction,
) -> Bool {
  let b = base(c2_hash, c, self)
  if thread.settle_claim_script(b) {
    let shape_digest = thread.settle_shape_digest_of(b)
    if thread.settle_shape_digest(b, shape_digest) {
      let e = thread.settle_effects(b)
      let cl = thread.settle_claim_input(b)
      if thread.settle_claim(b, cl) {
        thread.settle_statement_digest(
          cl,
          thread.settle_statement(b, e, cl, shape_digest, intents_digest),
        )
      } else {
        False
      }
    } else {
      False
    }
  } else {
    False
  }
}

/// The VS3 validator body. VI must run in this transaction (its withdrawal and exactly one
/// redeemer), name the same policy and cover exactly the same threads; VS3's own coverage
/// is exact; every covered thread's VS3 part holds with VI's digest for that thread.
pub fn vs3_identity(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  r: VsPieceV1,
  self: Transaction,
) -> Bool {
  when shard.shard_redeemer(vi_hash, self) is {
    None -> False
    Some(raw) -> {
      expect auth: AuthShardV1 = raw
      when coverage(3, r, self) is {
        Some(cov) -> and {
            auth.thread_policy == r.thread_policy,
            list.map(auth.threads, fn(t) { t.outref }) == r.threads,
            list.all(
              list.zip(cov, auth.threads),
              fn(p) {
                identity_thread(c2_hash, p.1st, p.2nd.intents_digest, self)
              },
            ),
          }
        None -> False
      }
    }
  }
}

// ## VS4: custody

/// The W1 `settle_next` with the statement digest of the thread's claim envelope. VS3
/// accepts only if that digest equals the recomputed statement digest, so under the
/// conjunction this is the monolithic next datum.
pub fn claimed_next(b: SettleBaseV2, snap: SettleSnapshotV2) -> ThreadDatumV2 {
  let cl = thread.settle_claim_input(b)
  thread.settle_next(b, snap, cl.env.statement_digest)
}

/// VS4's part of one thread: exact custody before, the complete continuation (datum and
/// custody value) and the tagged external payouts.
pub fn custody_thread(c: CoveredV1, self: Transaction) -> Bool {
  let b = base(no_c2, c, self)
  if thread.settle_custody_pre(b) {
    let e = thread.settle_effects(b)
    let snap = thread.settle_snapshot(b, e)
    and {
      thread.settle_continuation(b, snap, claimed_next(b, snap)),
      thread.settle_payouts(b, e),
    }
  } else {
    False
  }
}

/// The VS4 validator body.
pub fn vs4_custody(r: VsPieceV1, self: Transaction) -> Bool {
  when coverage(4, r, self) is {
    Some(cov) -> list.all(cov, fn(c) { custody_thread(c, self) })
    None -> False
  }
}

// ## S2': delegation and genesis guard

/// The C2, VI and four piece hashes and the S2' policy are seven distinct 28-byte
/// identities, with exactly four pieces. An aliased parameter would let one script stand
/// for two pieces and drop the other's checks.
pub fn identities_distinct(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  pieces: List<ByteArray>,
  policy: ByteArray,
) -> Bool {
  let ids = [c2_hash, vi_hash, policy, ..pieces]
  and {
    list.length(pieces) == 4,
    list.all(ids, fn(h) { bytes.len(h) == 28 }),
    list.length(list.unique(ids)) == list.length(ids),
  }
}

/// Piece `k` runs in this transaction (withdrawal plus exactly one redeemer), carries tag
/// `k`, names this policy and lists this outref.
fn piece_lists(
  k: Int,
  hash: ByteArray,
  own_hash: ByteArray,
  own_ref: OutputReference,
  self: Transaction,
) -> Bool {
  when shard.shard_redeemer(hash, self) is {
    Some(raw) -> {
      expect p: VsPieceV1 = raw
      and {
        p.piece == k,
        p.thread_policy == own_hash,
        list.has(p.threads, own_ref),
      }
    }
    None -> False
  }
}

/// S2' Settle path: distinct identities, and each of the four pieces (`pieces` in piece
/// order 1..4) lists this input.
pub fn delegated(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  pieces: List<ByteArray>,
  own_ref: OutputReference,
  self: Transaction,
) -> Bool {
  expect Some(own) =
    list.find(self.inputs, fn(i) { i.output_reference == own_ref })
  expect Script(own_hash) = own.output.address.payment_credential
  and {
    identities_distinct(c2_hash, vi_hash, pieces, own_hash),
    list.all(
      list.indexed_map(pieces, fn(i, h) { (i + 1, h) }),
      fn(kh) { piece_lists(kh.1st, kh.2nd, own_hash, own_ref, self) },
    ),
  }
}

/// No catalogue account's native co-authority and no reserve owner may be a shard
/// credential: every shard withdrawal is present in every settle transaction, so
/// `catalogue.native_ok` would otherwise grant that authority to anyone.
pub fn credentials_free(cat: CatalogueV2, shards: List<ByteArray>) -> Bool {
  let is_shard =
    fn(c: Credential) { list.any(shards, fn(h) { c == Script(h) }) }
  and {
    !is_shard(cat.reserve_owner.payment_credential),
    list.all(
      cat.accounts,
      fn(a) {
        when a.native is {
          Some(c) -> !is_shard(c)
          None -> True
        }
      },
    ),
  }
}

/// S2' genesis: the unchanged W1 genesis gate, distinct identities, and the
/// shard-credential guard over VI and the four pieces.
pub fn genesis(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  pieces: List<ByteArray>,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> Bool {
  let g = thread.genesis_base(c2_hash, policy_id, seed, self)
  and {
    thread.genesis(c2_hash, policy_id, seed, self),
    identities_distinct(c2_hash, vi_hash, pieces, policy_id),
    credentials_free(g.td.catalogue, [vi_hash, ..pieces]),
  }
}
