//// Test support for the four-piece settlement tests (TEST-ONLY; PROPOSED, uncompiled,
//// unrun). No validator imports this module. It reuses `shard_harness` to decode the frozen
//// R1 contexts and derives a four-piece transaction by adding only the VI and VS1..VS4 zero
//// withdrawals and their Withdraw redeemers; nothing else in the R1 transaction changes.
////
//// `vs1_t`..`vs4_t` are test identifiers standing for script hashes. They are not genuine
//// hashes of the candidate scripts (those exist only after compilation).

use aiken/builtin
use aiken/collection/list
use cardano/address.{Script}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Transaction,
}
use pvc2/bytes
use pvc2/shard
use pvc2/shard4.{VsPieceV1}
use pvc2/shard_harness as h
use pvc2/thread.{SettleBaseV2, SettleSnapshotV2}
use pvc2/types.{EnvV2}

pub const vs1_t: ByteArray =
  #"c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1"

pub const vs2_t: ByteArray =
  #"d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2"

pub const vs3_t: ByteArray =
  #"c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3"

pub const vs4_t: ByteArray =
  #"c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c4"

/// The four piece identifiers in piece order.
pub fn pieces_t() -> List<ByteArray> {
  [vs1_t, vs2_t, vs3_t, vs4_t]
}

/// VI and the four pieces: the shard credentials the genesis guard must exclude.
pub fn shards_t() -> List<ByteArray> {
  [h.vi_t, ..pieces_t()]
}

// ## Piece redeemers and the four-piece transaction

pub fn piece(k: Int, policy: ByteArray, refs: List<OutputReference>) -> VsPieceV1 {
  VsPieceV1 { piece: k, thread_policy: policy, threads: refs }
}

/// The four honest piece redeemers under their identifiers.
pub fn honest_pieces(
  policy: ByteArray,
  refs: List<OutputReference>,
) -> List<(ByteArray, VsPieceV1)> {
  [
    (vs1_t, piece(1, policy, refs)),
    (vs2_t, piece(2, policy, refs)),
    (vs3_t, piece(3, policy, refs)),
    (vs4_t, piece(4, policy, refs)),
  ]
}

pub fn vi_data(
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> Data {
  let a: Data = h.auth_of(policy, refs, digests)
  a
}

/// Adds the zero withdrawal of `hash` and its Withdraw redeemer carrying `p`.
pub fn with_piece(tx: Transaction, hash: ByteArray, p: VsPieceV1) -> Transaction {
  let d: Data = p
  h.with_shard(tx, hash, d)
}

/// The transaction with VI (when `auth` is given) and the listed pieces.
pub fn attach(
  tx: Transaction,
  auth: Option<Data>,
  ps: List<(ByteArray, VsPieceV1)>,
) -> Transaction {
  let t0 =
    when auth is {
      Some(ad) -> h.with_shard(tx, h.vi_t, ad)
      None -> tx
    }
  list.foldl(ps, t0, fn(hp, t) { with_piece(t, hp.1st, hp.2nd) })
}

/// VI and all four honest pieces, all naming `policy` and `refs`.
pub fn sharded4(
  tx: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> Transaction {
  attach(tx, Some(vi_data(policy, refs, digests)), honest_pieces(policy, refs))
}

/// VI and the honest pieces except piece `skip`.
pub fn without_piece(
  tx: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
  skip: Int,
) -> Transaction {
  attach(
    tx,
    Some(vi_data(policy, refs, digests)),
    list.filter(honest_pieces(policy, refs), fn(hp) { hp.2nd.piece != skip }),
  )
}

// ## Piece predicates

pub fn vs1_on(t: Transaction, policy: ByteArray, refs: List<OutputReference>) -> Bool {
  shard4.vs1_structure(piece(1, policy, refs), t)
}

pub fn vs2_on(t: Transaction, policy: ByteArray, refs: List<OutputReference>) -> Bool {
  shard4.vs2_accounting(piece(2, policy, refs), t)
}

pub fn vs3_on(
  c2: ByteArray,
  t: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
) -> Bool {
  shard4.vs3_identity(c2, h.vi_t, piece(3, policy, refs), t)
}

pub fn vs4_on(t: Transaction, policy: ByteArray, refs: List<OutputReference>) -> Bool {
  shard4.vs4_custody(piece(4, policy, refs), t)
}

pub fn s2p_on(c2: ByteArray, t: Transaction, own_ref: OutputReference) -> Bool {
  shard4.delegated(c2, h.vi_t, pieces_t(), own_ref, t)
}

/// [VI gate is Some, VS1, VS2, VS3, VS4] on a transaction.
pub fn verdicts(
  c2: ByteArray,
  t: Transaction,
  policy: ByteArray,
  refs: List<OutputReference>,
  digests: List<ByteArray>,
) -> List<Bool> {
  [
    h.vi_some(c2, t, policy, refs, digests),
    vs1_on(t, policy, refs),
    vs2_on(t, policy, refs),
    vs3_on(c2, t, policy, refs),
    vs4_on(t, policy, refs),
  ]
}

/// SHARD4-POSITIVE, one thread: coverage is the own outref, VI's gate returns `n` checks,
/// every piece holds and S2' accepts.
pub fn accepts1(ctx: ByteArray, c2: ByteArray, digest: ByteArray, n: Int) -> Bool {
  let (own_ref, p, tx) = h.single(ctx)
  let refs = [own_ref]
  let t = sharded4(tx, p, refs, [digest])
  and {
    shard.covered_refs(p, tx) == refs,
    h.vi_count(c2, t, p, refs, [digest]) == n,
    vs1_on(t, p, refs),
    vs2_on(t, p, refs),
    vs3_on(c2, t, p, refs),
    vs4_on(t, p, refs),
    s2p_on(c2, t, own_ref),
  }
}

/// SHARD4-SPLIT, one thread, with the honest digest.
pub fn split1(c2: ByteArray, ctx: ByteArray) -> List<Bool> {
  let (own_ref, p, tx) = h.single(ctx)
  let d = h.honest_digest(c2, ctx)
  let refs = [own_ref]
  verdicts(c2, sharded4(tx, p, refs, [d]), p, refs, [d])
}

/// SHARD4-SPLIT, two threads of one transaction.
pub fn split2(c2: ByteArray, ctx_a: ByteArray, ctx_b: ByteArray) -> List<Bool> {
  let (p, tx, refs, ds) = h.pair(c2, ctx_a, ctx_b)
  verdicts(c2, sharded4(tx, p, refs, ds), p, refs, ds)
}

/// VS1, VS2 and VS4 (no VI digest involved) on a single-thread context (refusal-only).
pub fn nonvi_pieces(ctx: ByteArray) -> Bool {
  let (own_ref, p, tx) = h.single(ctx)
  let refs = [own_ref]
  let t = sharded4(tx, p, refs, [bytes.zero32])
  vs1_on(t, p, refs) && vs2_on(t, p, refs) && vs4_on(t, p, refs)
}

// ## Genesis

pub fn genesis4(c2: ByteArray, ctx: ByteArray, pieces: List<ByteArray>) -> Bool {
  let (policy, seed, tx) = h.genesis_args(ctx)
  shard4.genesis(c2, h.vi_t, pieces, policy, seed, tx)
}

// ## Financial outcome (VS4's next datum against the R1 author fixtures)

fn vs4_stages(ctx: ByteArray) -> (SettleBaseV2, SettleSnapshotV2) {
  let (td, own_ref, r, tx) = h.spending(ctx)
  let b = thread.settle_base(#"", td, own_ref, r, tx)
  (b, thread.settle_snapshot(b, thread.settle_effects(b)))
}

/// serialise_data of the production `shard4.claimed_next` for the context's own thread.
pub fn vs4_next_bytes(ctx: ByteArray) -> ByteArray {
  let (b, snap) = vs4_stages(ctx)
  let next: Data = shard4.claimed_next(b, snap)
  builtin.serialise_data(next)
}

/// The statement digest of the context's claim envelope.
pub fn claim_digest(ctx: ByteArray) -> ByteArray {
  let (b, _) = vs4_stages(ctx)
  let cl = thread.settle_claim_input(b)
  cl.env.statement_digest
}

// ## Mutations

/// Every input at `Script(claim_hash)` with its envelope statement digest replaced by 32
/// zero bytes (the token name no longer matches the envelope).
pub fn forge_claim_digest(tx: Transaction, claim_hash: ByteArray) -> Transaction {
  let ins =
    list.map(
      tx.inputs,
      fn(i) {
        if i.output.address.payment_credential == Script(claim_hash) {
          expect InlineDatum(raw) = i.output.datum
          expect env: EnvV2 = raw
          let forged: Data = EnvV2 { ..env, statement_digest: bytes.zero32 }
          let o = i.output
          Input { ..i, output: Output { ..o, datum: InlineDatum(forged) } }
        } else {
          i
        }
      },
    )
  Transaction { ..tx, inputs: ins }
}

/// The inputs selected by `pick` moved from the spent inputs to the reference inputs.
pub fn to_reference(tx: Transaction, pick: fn(Input) -> Bool) -> Transaction {
  Transaction {
    ..tx,
    inputs: list.filter(tx.inputs, fn(i) { !pick(i) }),
    reference_inputs: list.concat(tx.reference_inputs, list.filter(tx.inputs, pick)),
  }
}
