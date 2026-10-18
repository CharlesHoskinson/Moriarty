//// Sharded settlement (PROPOSED, unreviewed, uncompiled, unrun). Same-transaction split of
//// the W1 settle gate into two withdraw-zero shards, consumed by the S2' thread validator:
////
////   C2 (no parameter) -> VI(c2_hash) -> VS(c2_hash, vi_hash) -> S2'(c2_hash, vi_hash, vs_hash)
////
//// VI (authorization shard) checks, per covered thread, `settle_authority_shape`,
//// `settle_intent_window`, `settle_intent_native`, renders every owner-intent message and
//// verifies every signature, and publishes the thread's intents digest in its redeemer.
//// VS (state shard) checks the other 16 W1 settle components, using the intents digest VI
//// published for the same outref to build the statement, and runs `global_outputs_distinct`
//// once per transaction. S2' delegates Settle to VS and keeps genesis, burn, revoke and
//// close.
////
//// Every condition is the unchanged W1 `pub fn` in `pvc2/thread`; this module only composes
//// them and adds the binding conjuncts (exact shard credential and purpose, one redeemer
//// per shard, exact coverage in transaction input order, one policy, per-outref digest
//// re-checked by its producer). No new proof relation, signed format or ABI v2 type
//// changes. No datum field or status flag is ever accepted as proof provenance.
////
//// Ledger premises (not established here): a script-credential withdrawal runs that
//// script with the Withdraw purpose; every script of a transaction must succeed; redeemers
//// are keyed uniquely by purpose; every script sees the same TxInfo. Splitting inside one
//// transaction shares one ExUnit budget: it fixes per-script size, not per-transaction cost.

use aiken/builtin
use aiken/collection/list
use aiken/collection/pairs
use cardano/address.{Credential, Script}
use cardano/assets.{PolicyId}
use cardano/transaction.{
  InlineDatum, Input, OutputReference, Spend, Transaction, Withdraw,
}
use pvc2/bytes
use pvc2/intent.{SigCheckV2}
use pvc2/thread
use pvc2/types.{CatalogueV2, Settle, SettleRedeemerV2, ThreadDatumV2}

// ## Shard redeemers (new types; Data constructor 0, fields positional)

/// One covered thread and the intents digest VI computed for it.
pub type ThreadDigestV1 {
  outref: OutputReference,
  intents_digest: ByteArray,
}

/// VI redeemer: the thread policy and every covered thread, in transaction input order.
pub type AuthShardV1 {
  thread_policy: ByteArray,
  threads: List<ThreadDigestV1>,
}

/// VS redeemer: the thread policy and every covered outref, in transaction input order.
pub type StateShardV1 {
  thread_policy: ByteArray,
  threads: List<OutputReference>,
}

/// A thread input whose spend redeemer is `Settle`, with its inline datum.
pub type CoveredV1 {
  input: Input,
  td: ThreadDatumV2,
  redeemer: SettleRedeemerV2,
}

/// What VI derives for one covered thread before signature validity.
pub type AuthorizedV1 {
  intents_digest: ByteArray,
  sigs: List<SigCheckV2>,
}

// ## Binding helpers

/// S2' path selection: `Settle` is constructor 0 of `SettleRedeemerV2`. A malformed
/// constructor-0 redeemer is delegated and then refused by the shards, which decode it.
pub fn is_settle(redeemer: Data) -> Bool {
  let Pair(ix, _) = builtin.un_constr_data(redeemer)
  ix == 0
}

/// The redeemer of the withdraw-zero of `Script(hash)`: `Some` only when the withdrawal is
/// present and exactly one `Withdraw(Script(hash))` redeemer exists.
pub fn shard_redeemer(hash: ByteArray, self: Transaction) -> Option<Data> {
  let cred = Script(hash)
  if pairs.has_key(self.withdrawals, cred) {
    let found =
      list.filter_map(
        self.redeemers,
        fn(p) {
          let Pair(purpose, d) = p
          if purpose == Withdraw(cred) {
            Some(d)
          } else {
            None
          }
        },
      )
    when found is {
      [d] -> Some(d)
      _ -> None
    }
  } else {
    None
  }
}

/// The single spend redeemer of an input (halts unless exactly one exists).
fn spend_redeemer(self: Transaction, out_ref: OutputReference) -> Data {
  expect [d] =
    list.filter_map(
      self.redeemers,
      fn(p) {
        let Pair(purpose, v) = p
        if purpose == Spend(out_ref) {
          Some(v)
        } else {
          None
        }
      },
    )
  d
}

/// The covered set: every input locked by `Script(policy)` whose spend redeemer is
/// `Settle`, in transaction input order, with its inline thread datum. Revoke and Close
/// inputs are not covered (S2' checks them itself). Halts on a non-inline datum or a
/// malformed redeemer.
pub fn covered(policy: ByteArray, self: Transaction) -> List<CoveredV1> {
  let lock = Script(policy)
  list.filter_map(
    self.inputs,
    fn(i) {
      if i.output.address.payment_credential == lock {
        expect r: SettleRedeemerV2 = spend_redeemer(self, i.output_reference)
        when r is {
          Settle { .. } -> {
            expect InlineDatum(raw) = i.output.datum
            expect td: ThreadDatumV2 = raw
            Some(CoveredV1 { input: i, td, redeemer: r })
          }
          _ -> None
        }
      } else {
        None
      }
    },
  )
}

pub fn covered_refs(policy: ByteArray, self: Transaction) -> List<OutputReference> {
  list.map(covered(policy, self), fn(c) { c.input.output_reference })
}

// ## VI: authorization shard

/// VI's part of one thread's settle (W1 components 9, 10, 12 and stages B, E, A, SC, M).
/// `Some` carries the intents digest and the signature checks still to verify.
pub fn thread_authorization(
  c2_hash: ByteArray,
  c: CoveredV1,
  self: Transaction,
) -> Option<AuthorizedV1> {
  let b =
    thread.settle_base(c2_hash, c.td, c.input.output_reference, c.redeemer, self)
  let e = thread.settle_effects(b)
  if thread.settle_authority_shape(b, e) && thread.settle_intent_window(b) {
    let signers = thread.settle_signers(b)
    if thread.settle_intent_native(b, signers) {
      let m =
        thread.settle_messages(b, signers, thread.settle_scenario_hash(b, e))
      Some(
        AuthorizedV1 {
          intents_digest: m.intents_digest,
          sigs: list.map(m.signed, fn(s) { s.sig }),
        },
      )
    } else {
      None
    }
  } else {
    None
  }
}

fn authorize_threads(
  c2_hash: ByteArray,
  rows: List<(CoveredV1, ThreadDigestV1)>,
  self: Transaction,
  acc: List<SigCheckV2>,
) -> Option<List<SigCheckV2>> {
  when rows is {
    [] -> Some(acc)
    [p, ..rest] -> {
      let (c, t) = p
      when thread_authorization(c2_hash, c, self) is {
        Some(a) ->
          if a.intents_digest == t.intents_digest {
            authorize_threads(c2_hash, rest, self, list.concat(acc, a.sigs))
          } else {
            None
          }
        None -> None
      }
    }
  }
}

/// Every VI check except signature validity: exact non-empty coverage, then each covered
/// thread's part, with the published digest equal to the recomputed one.
pub fn authorize_gate(
  c2_hash: ByteArray,
  r: AuthShardV1,
  self: Transaction,
) -> Option<List<SigCheckV2>> {
  let cov = covered(r.thread_policy, self)
  let coverage_ok =
    and {
      bytes.len(r.thread_policy) == 28,
      !list.is_empty(cov),
      list.map(cov, fn(c) { c.input.output_reference }) == list.map(
        r.threads,
        fn(t) { t.outref },
      ),
    }
  if coverage_ok {
    authorize_threads(c2_hash, list.zip(cov, r.threads), self, [])
  } else {
    None
  }
}

/// The VI validator body: every check, then every signature of every covered thread.
pub fn authorize(c2_hash: ByteArray, r: AuthShardV1, self: Transaction) -> Bool {
  when authorize_gate(c2_hash, r, self) is {
    Some(sigs) -> list.all(sigs, intent.verify)
    None -> False
  }
}

// ## VS: state shard

/// VS's part of one thread's settle: the 15 per-thread W1 components not in VI, in W1
/// gate order, with the statement built from the intents digest VI published.
pub fn thread_state(
  c2_hash: ByteArray,
  c: CoveredV1,
  intents_digest: ByteArray,
  self: Transaction,
) -> Bool {
  let b =
    thread.settle_base(c2_hash, c.td, c.input.output_reference, c.redeemer, self)
  let base_ok =
    and {
      thread.settle_custody_pre(b),
      thread.settle_claim_script(b),
      thread.settle_shape_structure(b),
      thread.settle_shape_cells(b),
    }
  if base_ok {
    let shape_digest = thread.settle_shape_digest_of(b)
    if thread.settle_shape_digest(b, shape_digest) {
      let e = thread.settle_effects(b)
      if thread.settle_recomputed(b, e) && thread.settle_bounds(b, e) {
        let cl = thread.settle_claim_input(b)
        if thread.settle_claim(b, cl) {
          let snap = thread.settle_snapshot(b, e)
          if thread.settle_intent_effects(b, e, snap) {
            let st =
              thread.settle_statement(b, e, cl, shape_digest, intents_digest)
            and {
              thread.settle_statement_digest(cl, st),
              thread.settle_continuation(
                b,
                snap,
                thread.settle_next(b, snap, st.statement_digest),
              ),
              thread.settle_payouts(b, e),
              thread.settle_own_mint(b),
              thread.settle_topup(b),
              thread.settle_reserve_floor(b),
            }
          } else {
            False
          }
        } else {
          False
        }
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

/// The VS validator body. VI must run in this transaction (its withdrawal and exactly one
/// redeemer), name the same policy and cover exactly the same threads; VS's own coverage
/// is exact; global output allocation runs once; every covered thread's VS part holds
/// with VI's digest for that thread.
pub fn state(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  r: StateShardV1,
  self: Transaction,
) -> Bool {
  when shard_redeemer(vi_hash, self) is {
    None -> False
    Some(raw) -> {
      expect auth: AuthShardV1 = raw
      let cov = covered(r.thread_policy, self)
      let refs = list.map(cov, fn(c) { c.input.output_reference })
      and {
        bytes.len(r.thread_policy) == 28,
        !list.is_empty(cov),
        refs == r.threads,
        auth.thread_policy == r.thread_policy,
        list.map(auth.threads, fn(t) { t.outref }) == refs,
        thread.global_outputs_distinct(r.thread_policy, self),
        list.all(
          list.zip(cov, auth.threads),
          fn(p) { thread_state(c2_hash, p.1st, p.2nd.intents_digest, self) },
        ),
      }
    }
  }
}

// ## S2': delegation and genesis guard

/// S2' Settle path: VS runs in this transaction (withdrawal plus exactly one redeemer),
/// names this input's own script hash and lists this outref.
pub fn delegated(
  vs_hash: ByteArray,
  own_ref: OutputReference,
  self: Transaction,
) -> Bool {
  expect Some(own) =
    list.find(self.inputs, fn(i) { i.output_reference == own_ref })
  expect Script(own_hash) = own.output.address.payment_credential
  when shard_redeemer(vs_hash, self) is {
    Some(raw) -> {
      expect st: StateShardV1 = raw
      st.thread_policy == own_hash && list.has(st.threads, own_ref)
    }
    None -> False
  }
}

/// No catalogue account's native co-authority and no reserve owner may be a shard
/// credential: a shard withdrawal is present in every settle transaction, so
/// `catalogue.native_ok` would otherwise grant that authority to anyone.
pub fn credentials_free(
  cat: CatalogueV2,
  vi_hash: ByteArray,
  vs_hash: ByteArray,
) -> Bool {
  let is_shard = fn(c: Credential) { c == Script(vi_hash) || c == Script(vs_hash) }
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

/// S2' genesis: the unchanged W1 genesis gate and the shard-credential guard.
pub fn genesis(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  vs_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> Bool {
  let g = thread.genesis_base(c2_hash, policy_id, seed, self)
  and {
    thread.genesis(c2_hash, policy_id, seed, self),
    credentials_free(g.td.catalogue, vi_hash, vs_hash),
  }
}
