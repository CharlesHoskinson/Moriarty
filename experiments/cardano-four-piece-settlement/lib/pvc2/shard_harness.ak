//// Test support for the sharded settlement tests (TEST-ONLY; PROPOSED, uncompiled, unrun).
//// No validator imports this module. It decodes the frozen R1 contexts through `test_ctx`
//// and derives a sharded transaction by adding only the VI/VS zero withdrawals and their
//// Withdraw redeemers; nothing else in the R1 transaction changes.
////
//// `vi_t`, `vs_t` and `foreign` are test identifiers standing for script hashes. They are
//// not genuine hashes of the candidate scripts (those exist only after compilation).
//// `honest_digest` recomputes a thread's intents digest with the unchanged W1 stage
//// functions; it is self-derived plumbing, not an independent oracle.

use aiken/collection/list
use cardano/address.{Script}
use cardano/transaction.{OutputReference, Transaction, Withdraw}
use pvc2/shard.{AuthShardV1, CoveredV1, StateShardV1, ThreadDigestV1}
use pvc2/test_ctx
use pvc2/thread
use pvc2/types.{CatalogueV2, Genesis, S2Mint, SettleRedeemerV2, ThreadDatumV2}

pub const vi_t: ByteArray =
  #"b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1"

pub const vs_t: ByteArray =
  #"b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2"

pub const foreign: ByteArray =
  #"b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3b3"

// ## Decoding

pub fn spending(
  ctx: ByteArray,
) -> (ThreadDatumV2, OutputReference, SettleRedeemerV2, Transaction) {
  let (own_ref, raw, rd, tx) = test_ctx.spending_of(ctx)
  expect td: ThreadDatumV2 = raw
  expect r: SettleRedeemerV2 = rd
  (td, own_ref, r, tx)
}

pub fn policy_of(tx: Transaction, own_ref: OutputReference) -> ByteArray {
  expect Some(i) = list.find(tx.inputs, fn(x) { x.output_reference == own_ref })
  expect Script(p) = i.output.address.payment_credential
  p
}

/// (own outref, thread policy, transaction) of a spending context.
pub fn single(ctx: ByteArray) -> (OutputReference, ByteArray, Transaction) {
  let (_, own_ref, _, tx) = spending(ctx)
  (own_ref, policy_of(tx, own_ref), tx)
}

/// The intents digest of the context's own thread, from the unchanged W1 stages.
pub fn honest_digest(c2: ByteArray, ctx: ByteArray) -> ByteArray {
  let (td, own_ref, r, tx) = spending(ctx)
  let b = thread.settle_base(c2, td, own_ref, r, tx)
  let e = thread.settle_effects(b)
  let m =
    thread.settle_messages(b, thread.settle_signers(b), thread.settle_scenario_hash(b, e))
  m.intents_digest
}

// ## Shard redeemers and the sharded transaction

pub fn auth_of(
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> AuthShardV1 {
  AuthShardV1 {
    thread_policy: policy,
    threads: list.map(
      list.zip(refs, digests),
      fn(p) { ThreadDigestV1 { outref: p.1st, intents_digest: p.2nd } },
    ),
  }
}

pub fn state_of(policy: ByteArray, refs: List<OutputReference>) -> StateShardV1 {
  StateShardV1 { thread_policy: policy, threads: refs }
}

pub fn with_withdrawal(tx: Transaction, hash: ByteArray) -> Transaction {
  Transaction { ..tx, withdrawals: [Pair(Script(hash), 0), ..tx.withdrawals] }
}

pub fn with_redeemer(tx: Transaction, hash: ByteArray, d: Data) -> Transaction {
  Transaction {
    ..tx,
    redeemers: list.concat(tx.redeemers, [Pair(Withdraw(Script(hash)), d)]),
  }
}

pub fn with_shard(tx: Transaction, hash: ByteArray, d: Data) -> Transaction {
  with_redeemer(with_withdrawal(tx, hash), hash, d)
}

/// The transaction with VI (redeemer `a`) and VS (redeemer `s`) withdraw-zero shards.
pub fn attach(tx: Transaction, a: AuthShardV1, s: StateShardV1) -> Transaction {
  let ad: Data = a
  let sd: Data = s
  tx |> with_shard(vi_t, ad) |> with_shard(vs_t, sd)
}

pub fn sharded(
  tx: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> Transaction {
  attach(tx, auth_of(policy, refs, digests), state_of(policy, refs))
}

// ## Shard predicates

/// Number of signature checks VI's gate returns, or -1 when it refuses.
pub fn vi_count(
  c2: ByteArray,
  t: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> Int {
  when shard.authorize_gate(c2, auth_of(policy, refs, digests), t) is {
    Some(sigs) -> list.length(sigs)
    None -> -1
  }
}

pub fn vi_some(
  c2: ByteArray,
  t: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> Bool {
  vi_count(c2, t, policy, refs, digests) >= 0
}

pub fn vs_holds(
  c2: ByteArray,
  t: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
) -> Bool {
  shard.state(c2, vi_t, state_of(policy, refs), t)
}

/// SHARD-POSITIVE, one thread: coverage is the own outref, VI's gate returns `n` checks,
/// VS holds and S2' accepts.
pub fn accepts1(ctx: ByteArray, c2: ByteArray, digest: ByteArray, n: Int) -> Bool {
  let (own_ref, p, tx) = single(ctx)
  let refs = [own_ref]
  let t = sharded(tx, p, refs, [digest])
  and {
    shard.covered_refs(p, tx) == refs,
    vi_count(c2, t, p, refs, [digest]) == n,
    vs_holds(c2, t, p, refs),
    shard.delegated(vs_t, own_ref, t),
  }
}

/// SHARD-SPLIT, one thread: (VI gate is Some, VS holds) with the honest digest.
pub fn split1(c2: ByteArray, ctx: ByteArray) -> (Bool, Bool) {
  let (own_ref, p, tx) = single(ctx)
  let d = honest_digest(c2, ctx)
  let refs = [own_ref]
  let t = sharded(tx, p, refs, [d])
  (vi_some(c2, t, p, refs, [d]), vs_holds(c2, t, p, refs))
}

/// Two thread contexts of one transaction: the policy, the transaction (from the first
/// context), the covered outrefs in input order and each thread's honest digest aligned.
pub fn pair(
  c2: ByteArray,
  ctx_a: ByteArray,
  ctx_b: ByteArray,
) -> (ByteArray, Transaction, List<OutputReference>, List<ByteArray>) {
  let (ra, p, tx) = single(ctx_a)
  let da = honest_digest(c2, ctx_a)
  let db = honest_digest(c2, ctx_b)
  let refs = shard.covered_refs(p, tx)
  (
    p,
    tx,
    refs,
    list.map(
      refs,
      fn(x) {
        if x == ra {
          da
        } else {
          db
        }
      },
    ),
  )
}

/// SHARD-SPLIT, two threads of one transaction.
pub fn split2(c2: ByteArray, ctx_a: ByteArray, ctx_b: ByteArray) -> (Bool, Bool) {
  let (p, tx, refs, ds) = pair(c2, ctx_a, ctx_b)
  let t = sharded(tx, p, refs, ds)
  (vi_some(c2, t, p, refs, ds), vs_holds(c2, t, p, refs))
}

// ## Per-thread shard parts (refusal-only ERROR-PATH tests)

pub fn covered_own(ctx: ByteArray) -> (CoveredV1, Transaction) {
  let (own_ref, p, tx) = single(ctx)
  expect Some(c) =
    list.find(
      shard.covered(p, tx),
      fn(x) { x.input.output_reference == own_ref },
    )
  (c, tx)
}

pub fn thread_auth_some(c2: ByteArray, ctx: ByteArray) -> Bool {
  let (c, tx) = covered_own(ctx)
  when shard.thread_authorization(c2, c, tx) is {
    Some(_) -> True
    None -> False
  }
}

pub fn thread_state_holds(c2: ByteArray, ctx: ByteArray, digest: ByteArray) -> Bool {
  let (c, tx) = covered_own(ctx)
  shard.thread_state(c2, c, digest, tx)
}

// ## Genesis

pub fn genesis_args(ctx: ByteArray) -> (ByteArray, OutputReference, Transaction) {
  let (policy, rd, tx) = test_ctx.minting_of(ctx)
  expect m: S2Mint = rd
  expect Genesis { seed } = m
  (policy, seed, tx)
}

pub fn sharded_genesis(c2: ByteArray, ctx: ByteArray) -> Bool {
  let (policy, seed, tx) = genesis_args(ctx)
  shard.genesis(c2, vi_t, vs_t, policy, seed, tx)
}

pub fn genesis_catalogue(c2: ByteArray, ctx: ByteArray) -> CatalogueV2 {
  let (policy, seed, tx) = genesis_args(ctx)
  let g = thread.genesis_base(c2, policy, seed, tx)
  g.td.catalogue
}
