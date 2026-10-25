// Adapted from the pinned Fenrir B2 oracle; see ../sources.json.
//! On-disk data types (interface I1). Field elements serialize as
//! 32-byte little-endian hex of the BLS12-381 scalar field; Jubjub
//! scalars serialize the same way over their own field and are
//! named `*_jfr` in JSON through the serde types below.

use super::fp::{F, JFr, dom, f_u64, hash, network_tag, serde_f, serde_f_vec, serde_jfr, trunc248};
use ff::Field;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Shopper,
    Merchant,
    Processor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TxKind {
    CheckoutChange,
    CheckoutExact,
    P2p,
    Sweep,
    Deposit,
    Withdrawal,
}

impl TxKind {
    pub fn code(&self) -> u64 {
        match self {
            TxKind::CheckoutChange => 0,
            TxKind::CheckoutExact => 1,
            TxKind::P2p => 2,
            TxKind::Sweep => 3,
            TxKind::Deposit => 4,
            TxKind::Withdrawal => 5,
        }
    }
    pub fn is_deposit(&self) -> bool {
        matches!(self, TxKind::Deposit)
    }
    pub fn is_withdrawal(&self) -> bool {
        matches!(self, TxKind::Withdrawal)
    }
}

/// A Jubjub point in affine coordinates. `x` is the u coordinate and
/// `y` is the v coordinate in midnight-curves terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    #[serde(with = "serde_f")]
    pub x: F,
    #[serde(with = "serde_f")]
    pub y: F,
}

impl Point {
    pub fn identity() -> Self {
        Self {
            x: F::ZERO,
            y: F::ONE,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Party {
    pub id: u32,
    pub role: Role,
    pub public_key: Point,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainEntry {
    pub name: String,
    #[serde(with = "serde_f")]
    pub value: F,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HashVector {
    pub name: String,
    #[serde(with = "serde_f_vec")]
    pub inputs: Vec<F>,
    #[serde(with = "serde_f")]
    pub output: F,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub seed: u64,
    pub seed_hex: String,
    pub headers: u64,
    pub batch_headers: u64,
    pub batches: u64,
    pub tx_per_batch: usize,
    pub valid_tx_total: usize,
    pub shoppers: usize,
    pub merchants: usize,
    pub processors: usize,
    pub parties: Vec<Party>,
    pub cm_depth: usize,
    pub nf_depth: usize,
    pub rh_depth: usize,
    pub wd_depth: usize,
    pub rh_retention_ms: u64,
    pub w_rh: u32,
    pub flat_fee_micro: u64,
    pub token_usd: u64,
    pub token_night: u64,
    pub t0: u64,
    pub header_secs: u64,
    #[serde(with = "serde_f")]
    pub network_tag: F,
    pub domains: Vec<DomainEntry>,
    pub hash_vectors: Vec<HashVector>,
    pub mix: MixCounts,
    pub hash_function: String,
    pub field_encoding: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MixCounts {
    pub checkout_change: usize,
    pub checkout_exact: usize,
    pub p2p: usize,
    pub sweep: usize,
    pub deposit: usize,
    pub withdrawal: usize,
}

impl MixCounts {
    pub fn total(&self) -> usize {
        self.checkout_change
            + self.checkout_exact
            + self.p2p
            + self.sweep
            + self.deposit
            + self.withdrawal
    }
    pub fn add(&mut self, kind: TxKind) {
        match kind {
            TxKind::CheckoutChange => self.checkout_change += 1,
            TxKind::CheckoutExact => self.checkout_exact += 1,
            TxKind::P2p => self.p2p += 1,
            TxKind::Sweep => self.sweep += 1,
            TxKind::Deposit => self.deposit += 1,
            TxKind::Withdrawal => self.withdrawal += 1,
        }
    }
}

/// The 12-field L2 state vector S (STRAWMAN section 2.5).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateVec {
    #[serde(with = "serde_f")]
    pub cm_root: F,
    pub cm_count: u32,
    #[serde(with = "serde_f")]
    pub nf_root: F,
    pub nf_count: u32,
    #[serde(with = "serde_f")]
    pub rh_root: F,
    pub rh_count: u32,
    #[serde(with = "serde_f")]
    pub wd_root: F,
    pub wd_count: u32,
    pub height: u64,
    pub tx_count: u64,
    pub dep_consumed: u64,
    pub fee_total: u64,
}

impl StateVec {
    pub fn fields(&self) -> [F; 12] {
        [
            self.cm_root,
            f_u64(self.cm_count as u64),
            self.nf_root,
            f_u64(self.nf_count as u64),
            self.rh_root,
            f_u64(self.rh_count as u64),
            self.wd_root,
            f_u64(self.wd_count as u64),
            f_u64(self.height),
            f_u64(self.tx_count),
            f_u64(self.dep_consumed),
            f_u64(self.fee_total),
        ]
    }
    pub fn digest(&self) -> F {
        let mut input = vec![dom::state()];
        input.extend(self.fields());
        hash(&input)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteOpening {
    pub owner: u32,
    pub owner_public_key: Point,
    pub token: u64,
    pub value: u64,
    #[serde(with = "serde_f")]
    pub rho: F,
    #[serde(with = "serde_f")]
    pub r: F,
}

impl NoteOpening {
    /// cm = H(DOM_CM, token, value, pk.x, pk.y, rho, r)
    pub fn commitment(&self) -> F {
        hash(&[
            dom::cm(),
            f_u64(self.token),
            f_u64(self.value),
            self.owner_public_key.x,
            self.owner_public_key.y,
            self.rho,
            self.r,
        ])
    }
    /// nf = trunc248(H(DOM_NF, nk, rho))
    pub fn nullifier(&self, nk: F) -> F {
        trunc248(hash(&[dom::nf(), nk, self.rho]))
    }
    pub fn zero() -> Self {
        Self {
            owner: 0,
            owner_public_key: Point::identity(),
            token: 0,
            value: 0,
            rho: F::ZERO,
            r: F::ZERO,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepositRecord {
    pub seq: u64,
    pub token: u64,
    pub amount: u64,
    #[serde(with = "serde_f")]
    pub cm: F,
    pub note: NoteOpening,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genesis {
    pub state: StateVec,
    #[serde(with = "serde_f")]
    pub s_digest: F,
    #[serde(with = "serde_f")]
    pub dep_acc: F,
    pub notes: Vec<GenesisNote>,
    pub deposit_queue: Vec<DepositRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenesisNote {
    pub leaf_index: u32,
    #[serde(with = "serde_f")]
    pub cm: F,
    pub note: NoteOpening,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Slot {
    pub enabled: bool,
    #[serde(with = "serde_f")]
    pub value: F,
}

impl Slot {
    pub fn off() -> Self {
        Self {
            enabled: false,
            value: F::ZERO,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    pub r_point: Point,
    #[serde(with = "serde_jfr")]
    pub s: JFr,
}

/// Public transaction record. The field list hashed for the record
/// hash and the signed message is fixed in SPEC.md.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxPublic {
    pub id: String,
    pub batch: u64,
    pub height: u64,
    pub slot: u64,
    pub kind: TxKind,
    #[serde(with = "serde_f")]
    pub anchor_root: F,
    pub anchor_rh_index: u32,
    pub nullifiers: Vec<Slot>,
    pub commitments: Vec<Slot>,
    pub spender: Option<u32>,
    pub rk: Option<Point>,
    pub signature: Option<Signature>,
    pub fee: u64,
    pub pub_token: u64,
    pub pub_amount: u64,
    pub wd_dest: Option<u32>,
    #[serde(with = "serde_f")]
    pub ct_digest: F,
    pub expiry: u64,
    #[serde(with = "serde_f")]
    pub msg: F,
    #[serde(with = "serde_f")]
    pub record_hash: F,
    pub active_flags: u64,
}

impl TxPublic {
    /// msg = H(DOM_TXMSG, network_tag, kind, nf0, nf1, cm0, cm1,
    /// fee, pub_amount, wd_dest, ct_digest, expiry). Disabled
    /// slots contribute zero; wd_dest is the party id, or zero
    /// when there is no withdrawal. Slot enablement is bound by
    /// the record hash and by active_flags in the L0 instance.
    pub fn compute_msg(&self) -> F {
        let slot = |i: usize, v: &[Slot]| v.get(i).map(|s| s.value).unwrap_or(F::ZERO);
        hash(&[
            dom::txmsg(),
            network_tag(),
            f_u64(self.kind.code()),
            slot(0, &self.nullifiers),
            slot(1, &self.nullifiers),
            slot(0, &self.commitments),
            slot(1, &self.commitments),
            f_u64(self.fee),
            f_u64(self.pub_amount),
            f_u64(self.wd_dest.map(|d| d as u64).unwrap_or(0)),
            self.ct_digest,
            f_u64(self.expiry),
        ])
    }

    /// record_hash = H(DOM_TX, network_tag, kind, height, slot,
    /// anchor_root, nf flags and values, cm flags and values,
    /// rk, fee, pub_token, pub_amount, wd_dest_present, wd_dest,
    /// ct_digest, expiry). The batch in_root combines these.
    pub fn compute_record_hash(&self) -> F {
        let slot = |i: usize, v: &[Slot]| {
            v.get(i)
                .map(|s| (s.enabled, s.value))
                .unwrap_or((false, F::ZERO))
        };
        let (nf0e, nf0) = slot(0, &self.nullifiers);
        let (nf1e, nf1) = slot(1, &self.nullifiers);
        let (cm0e, cm0) = slot(0, &self.commitments);
        let (cm1e, cm1) = slot(1, &self.commitments);
        let (rkx, rky) = self.rk.map(|p| (p.x, p.y)).unwrap_or((F::ZERO, F::ZERO));
        hash(&[
            dom::tx(),
            network_tag(),
            f_u64(self.kind.code()),
            f_u64(self.height),
            f_u64(self.slot),
            self.anchor_root,
            f_u64(nf0e as u64),
            nf0,
            f_u64(nf1e as u64),
            nf1,
            f_u64(cm0e as u64),
            cm0,
            f_u64(cm1e as u64),
            cm1,
            rkx,
            rky,
            f_u64(self.fee),
            f_u64(self.pub_token),
            f_u64(self.pub_amount),
            f_u64(self.wd_dest.is_some() as u64),
            f_u64(self.wd_dest.map(|d| d as u64).unwrap_or(0)),
            self.ct_digest,
            f_u64(self.expiry),
        ])
    }

    pub fn compute_active_flags(&self) -> u64 {
        let en = |i: usize, v: &[Slot]| v.get(i).map(|s| s.enabled).unwrap_or(false) as u64;
        en(0, &self.nullifiers)
            | (en(1, &self.nullifiers) << 1)
            | (en(0, &self.commitments) << 2)
            | (en(1, &self.commitments) << 3)
            | ((self.signature.is_some() as u64) << 4)
            | ((self.wd_dest.is_some() as u64) << 5)
    }

    /// Position in the batch order: height * 2^16 + slot.
    pub fn pos(&self) -> u64 {
        self.height * 65536 + self.slot
    }

    /// The L0 user-proof instance in the frozen order
    /// `[network_tag, kind, anchor_root, nf0, nf1, cm0, cm1,
    /// rk0.x, rk0.y, rk1.x, rk1.y, fee, pub_token, pub_amount,
    /// wd_dest, ct_digest, active_flags]` (STRAWMAN I-L0). This
    /// history has one spender per transaction, so both key slots
    /// carry the same randomized key (zero for a deposit).
    /// Disabled nullifier and commitment slots contribute zero,
    /// and `wd_dest` is zero when there is no withdrawal.
    pub fn l0_instance(&self) -> [F; 17] {
        let slot = |i: usize, v: &[Slot]| v.get(i).map(|s| s.value).unwrap_or(F::ZERO);
        let (rkx, rky) = self.rk.map(|p| (p.x, p.y)).unwrap_or((F::ZERO, F::ZERO));
        [
            network_tag(),
            f_u64(self.kind.code()),
            self.anchor_root,
            slot(0, &self.nullifiers),
            slot(1, &self.nullifiers),
            slot(0, &self.commitments),
            slot(1, &self.commitments),
            rkx,
            rky,
            rkx,
            rky,
            f_u64(self.fee),
            f_u64(self.pub_token),
            f_u64(self.pub_amount),
            f_u64(self.wd_dest.map(|d| d as u64).unwrap_or(0)),
            self.ct_digest,
            f_u64(self.active_flags),
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NullifierWitness {
    pub predecessor_index: u32,
    #[serde(with = "serde_f")]
    pub predecessor_value: F,
    pub predecessor_next_index: u32,
    #[serde(with = "serde_f")]
    pub predecessor_next_value: F,
    #[serde(with = "serde_f_vec")]
    pub predecessor_path: Vec<F>,
    #[serde(with = "serde_f_vec")]
    pub append_path: Vec<F>,
    #[serde(with = "serde_f")]
    pub nf_root_before: F,
    #[serde(with = "serde_f")]
    pub nf_root_after: F,
    pub nf_count_before: u32,
}

impl NullifierWitness {
    pub fn zero(depth: usize) -> Self {
        Self {
            predecessor_index: 0,
            predecessor_value: F::ZERO,
            predecessor_next_index: 0,
            predecessor_next_value: F::ZERO,
            predecessor_path: vec![F::ZERO; depth],
            append_path: vec![F::ZERO; depth],
            nf_root_before: F::ZERO,
            nf_root_after: F::ZERO,
            nf_count_before: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputWitness {
    pub enabled: bool,
    pub note: NoteOpening,
    pub leaf_index: u32,
    #[serde(with = "serde_f_vec")]
    pub membership_path: Vec<F>,
    pub nullifier: NullifierWitness,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputWitness {
    pub enabled: bool,
    pub note: NoteOpening,
    pub append_index: u32,
    #[serde(with = "serde_f_vec")]
    pub append_path: Vec<F>,
    #[serde(with = "serde_f")]
    pub cm_root_before: F,
    #[serde(with = "serde_f")]
    pub cm_root_after: F,
    pub cm_count_before: u32,
}

impl OutputWitness {
    pub fn zero(depth: usize) -> Self {
        Self {
            enabled: false,
            note: NoteOpening::zero(),
            append_index: 0,
            append_path: vec![F::ZERO; depth],
            cm_root_before: F::ZERO,
            cm_root_after: F::ZERO,
            cm_count_before: 0,
        }
    }
}

/// Full leaf witness. Secrets repeat here for circuit convenience;
/// every party secret is also re-derivable from the seed in
/// params.json by the formula in SPEC.md. Synthetic data only.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxWitness {
    pub spender_secret_key_jfr: Option<String>,
    #[serde(with = "serde_f")]
    pub spender_nk: F,
    pub spender_alpha_jfr: Option<String>,
    pub spender_public_key: Option<Point>,
    pub inputs: Vec<InputWitness>,
    pub outputs: Vec<OutputWitness>,
    #[serde(with = "serde_f_vec")]
    pub rh_path: Vec<F>,
    pub rh_count_before: u32,
    #[serde(with = "serde_f")]
    pub rh_root_before: F,
    /// Era of the anchor entry: the root-history leaf at `anchor_rh_index`
    /// is `H(DOM_RH, anchor_era, anchor_root)` (NF-048).
    #[serde(default)]
    pub anchor_era: u64,
    pub deposit_record: Option<DepositRecord>,
    #[serde(with = "serde_f")]
    pub dep_acc_before: F,
    #[serde(with = "serde_f")]
    pub dep_acc_after: F,
    pub withdrawal: Option<WithdrawalWitness>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalWitness {
    pub wd_id: u32,
    #[serde(with = "serde_f")]
    pub leaf: F,
    pub append_index: u32,
    #[serde(with = "serde_f_vec")]
    pub append_path: Vec<F>,
    #[serde(with = "serde_f")]
    pub wd_root_before: F,
    #[serde(with = "serde_f")]
    pub wd_root_after: F,
}

/// Withdrawal leaf: H(DOM_WD, wd_id, dest, token, amount).
pub fn withdrawal_leaf(wd_id: u32, dest: u32, token: u64, amount: u64) -> F {
    hash(&[
        dom::wd(),
        f_u64(wd_id as u64),
        f_u64(dest as u64),
        f_u64(token),
        f_u64(amount),
    ])
}

/// Deposit accumulator step: H(DOM_DEP, acc, seq, token, amount, cm).
pub fn dep_step(acc: F, rec: &DepositRecord) -> F {
    hash(&[
        dom::dep(),
        acc,
        f_u64(rec.seq),
        f_u64(rec.token),
        f_u64(rec.amount),
        rec.cm,
    ])
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub public: TxPublic,
    pub witness: TxWitness,
    pub state_before: StateVec,
    pub state_after: StateVec,
    #[serde(with = "serde_f")]
    pub pre_digest: F,
    #[serde(with = "serde_f")]
    pub post_digest: F,
}

/// One header (brief I1 calls these blocks). Empty headers carry no
/// transactions and advance only the height.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub height: u64,
    /// Authenticated header time, unix seconds.
    pub time: u64,
    pub batch: u64,
    pub state_before: StateVec,
    pub txs: Vec<Tx>,
    #[serde(with = "serde_f_vec")]
    pub record_hashes: Vec<F>,
    /// Depth 16 append path for this header's root-history entry at index
    /// `rh_count` after the last transaction (one entry per header, R6-AW-001).
    #[serde(with = "serde_f_vec")]
    pub rh_append_path: Vec<F>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Expected {
    pub height: u64,
    pub time: u64,
    pub batch: u64,
    pub tx_ids: Vec<String>,
    pub state_before: StateVec,
    pub state_after: StateVec,
    #[serde(with = "serde_f")]
    pub pre_digest: F,
    #[serde(with = "serde_f")]
    pub post_digest: F,
    #[serde(with = "serde_f")]
    pub dep_acc_after: F,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchStatement {
    pub batch_no: u64,
    pub h_first: u64,
    pub h_last: u64,
    pub n_tx: usize,
    pub state_pre: StateVec,
    pub state_post: StateVec,
    #[serde(with = "serde_f")]
    pub pre: F,
    #[serde(with = "serde_f")]
    pub post: F,
    #[serde(with = "serde_f")]
    pub in_root: F,
    #[serde(with = "serde_f_vec")]
    pub record_hashes: Vec<F>,
    pub first_pos: u64,
    pub last_pos: u64,
    pub dep_from: u64,
    pub dep_to: u64,
    #[serde(with = "serde_f")]
    pub dep_acc_from: F,
    #[serde(with = "serde_f")]
    pub dep_acc_to: F,
    pub fee_sum: u64,
    pub wd_sum_usd: u64,
    pub wd_sum_night: u64,
    pub dep_sum_usd: u64,
    pub dep_sum_night: u64,
}

impl BatchStatement {
    /// stmt with a caller supplied instance id (deployment id).
    /// hist cannot know the deployment, so files carry the fields;
    /// this helper is the frozen formula from STRAWMAN section 3.2.
    pub fn stmt(&self, instance_id: F) -> F {
        hash(&[
            dom::stmt(),
            instance_id,
            f_u64(self.batch_no),
            self.pre,
            self.post,
            f_u64(self.h_first),
            f_u64(self.h_last),
            self.in_root,
            f_u64(self.dep_from),
            f_u64(self.dep_to),
            self.dep_acc_to,
            f_u64(self.fee_sum),
            f_u64(self.wd_sum_usd),
            f_u64(self.wd_sum_night),
            f_u64(self.dep_sum_usd),
            f_u64(self.dep_sum_night),
        ])
    }
}

/// Batch in_root over 16 record hashes: two arity-4 levels.
/// in4 node: H(DOM_IN4, c0, c1, c2, c3). Missing slots use
/// H(DOM_PAD) computed by `pad_leaf`.
pub fn pad_leaf() -> F {
    hash(&[dom::pad()])
}

pub fn batch_in_root(records: &[F]) -> F {
    assert!(records.len() <= 16);
    let mut leaves: Vec<F> = records.to_vec();
    while leaves.len() < 16 {
        leaves.push(pad_leaf());
    }
    let lvl1: Vec<F> = (0..4)
        .map(|i| {
            hash(&[
                dom::in4(),
                leaves[4 * i],
                leaves[4 * i + 1],
                leaves[4 * i + 2],
                leaves[4 * i + 3],
            ])
        })
        .collect();
    hash(&[dom::in4(), lvl1[0], lvl1[1], lvl1[2], lvl1[3]])
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvalidCase {
    pub case: String,
    pub level: String,
    pub reason_code: String,
    pub expected_failure_place: String,
    pub description: String,
    /// Global index of the last honest transaction applied before
    /// this case, or null for the genesis state. Batch level cases
    /// use `batch_ref` instead and carry no transaction.
    pub context_after_tx: Option<usize>,
    pub setup_advance_root_history: Option<u32>,
    pub tx: Option<Tx>,
    pub batch_ref: Option<u64>,
    #[serde(with = "serde_f")]
    pub claimed_pre_digest: F,
}
