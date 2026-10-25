// Adapted from the pinned Fenrir B2 oracle; see ../sources.json.
//! Sequential reference state machine (the oracle, `straw-ref`).
//!
//! Applies one transaction at a time in global order, recomputing
//! every value from the public record and the witness and checking
//! the rules of STRAWMAN section 2.4 in a fixed order. The check
//! order decides which reason code a bad transaction gets; it is
//! part of the interface and is listed in SPEC.md. On error the
//! machine state is unspecified and callers must discard it (the
//! same convention as an unsatisfiable circuit witness).

use super::fp::{
    CM_DEPTH, F, FLAT_FEE, NF_DEPTH, RH_DEPTH, RH_RETENTION_MS, T0, TOK_USD, W_RH, WD_DEPTH, dom,
    f_u64, hash, is_reserved_nullifier, jfr_from_hex,
};
use super::keys;
use super::trees::{IndexedTree, NullifierLeaf, SparseTree, nullifier_leaf_hash, root_from_path};
use super::types::*;
use ff::{Field, PrimeField};
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    BadShape,
    BadKind,
    HeightRegress,
    Expired,
    BadMsg,
    BadRecordHash,
    UnknownRoot,
    RootExpired,
    BadDeposit,
    BadCommitment,
    BadMembership,
    BadNullifier,
    MixedOwner,
    BadSig,
    Unbalanced,
    NullifierDup,
    NullifierSpent,
    ReservedNullifier,
    BadNfWitness,
    BadCmWitness,
    BadRhWitness,
    BadWdWitness,
    WrongPreviousRoot,
    WrongPreviousState,
    BadBatchChain,
    /// An output commitment already in the commitment tree, or repeated
    /// in the same transaction (NF-046, DUP-001; interim until the re-key).
    CommitmentAlreadyPresent,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::BadShape => "BAD_SHAPE",
            ErrorCode::BadKind => "BAD_KIND",
            ErrorCode::HeightRegress => "HEIGHT_REGRESS",
            ErrorCode::Expired => "EXPIRED",
            ErrorCode::BadMsg => "BAD_MSG",
            ErrorCode::BadRecordHash => "BAD_RECORD_HASH",
            ErrorCode::UnknownRoot => "UNKNOWN_ROOT",
            ErrorCode::RootExpired => "ROOT_EXPIRED",
            ErrorCode::BadDeposit => "BAD_DEPOSIT",
            ErrorCode::BadCommitment => "BAD_CM",
            ErrorCode::BadMembership => "BAD_MEMBERSHIP",
            ErrorCode::BadNullifier => "BAD_NF",
            ErrorCode::MixedOwner => "MIXED_OWNER",
            ErrorCode::BadSig => "BAD_SIG",
            ErrorCode::Unbalanced => "UNBALANCED",
            ErrorCode::NullifierDup => "NULLIFIER_DUP",
            ErrorCode::NullifierSpent => "NULLIFIER_SPENT",
            ErrorCode::ReservedNullifier => "RESERVED_NULLIFIER",
            ErrorCode::BadNfWitness => "BAD_NF_WITNESS",
            ErrorCode::BadCmWitness => "BAD_CM_WITNESS",
            ErrorCode::BadRhWitness => "BAD_RH_WITNESS",
            ErrorCode::BadWdWitness => "BAD_WD_WITNESS",
            ErrorCode::WrongPreviousRoot => "WRONG_PREVIOUS_ROOT",
            ErrorCode::WrongPreviousState => "WRONG_PREVIOUS_STATE",
            ErrorCode::BadBatchChain => "BAD_BATCH_CHAIN",
            ErrorCode::CommitmentAlreadyPresent => "COMMITMENT_ALREADY_PRESENT",
        }
    }
    pub fn from_str_code(s: &str) -> Option<Self> {
        [
            ErrorCode::BadShape,
            ErrorCode::BadKind,
            ErrorCode::HeightRegress,
            ErrorCode::Expired,
            ErrorCode::BadMsg,
            ErrorCode::BadRecordHash,
            ErrorCode::UnknownRoot,
            ErrorCode::RootExpired,
            ErrorCode::BadDeposit,
            ErrorCode::BadCommitment,
            ErrorCode::BadMembership,
            ErrorCode::BadNullifier,
            ErrorCode::MixedOwner,
            ErrorCode::BadSig,
            ErrorCode::Unbalanced,
            ErrorCode::NullifierDup,
            ErrorCode::NullifierSpent,
            ErrorCode::ReservedNullifier,
            ErrorCode::BadNfWitness,
            ErrorCode::BadCmWitness,
            ErrorCode::BadRhWitness,
            ErrorCode::BadWdWitness,
            ErrorCode::WrongPreviousRoot,
            ErrorCode::WrongPreviousState,
            ErrorCode::BadBatchChain,
            ErrorCode::CommitmentAlreadyPresent,
        ]
        .into_iter()
        .find(|c| c.as_str() == s)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub type Result<T> = std::result::Result<T, ErrorCode>;

fn err<T>(c: ErrorCode) -> Result<T> {
    Err(c)
}

#[derive(Clone)]
pub struct Oracle {
    pub cm: SparseTree,
    pub nf: IndexedTree,
    pub rh: SparseTree,
    pub wd: SparseTree,
    pub state: StateVec,
    pub dep_acc: F,
    pub deposit_queue: Vec<DepositRecord>,
    /// Interim commitment set of P1 (NF-046): every commitment in `cm`,
    /// by its 32-byte encoding. Not part of the state digest.
    pub cm_set: HashSet<[u8; 32]>,
    /// Per root-history entry: (era, authenticated time in ms of the header
    /// that recorded it). Index-aligned with the root-history tree; not part
    /// of the state digest.
    pub rh_log: Vec<(u64, u64)>,
    /// The current commitment era (0 until the FX42 era roll).
    pub era: u64,
    /// Authenticated time (ms) of the header being applied; anchors are
    /// judged against it (OD-16 (a)).
    pub now_ms: u64,
}

/// One root-history entry: `H(DOM_RH, era, root)` (NF-048).
pub fn rh_entry(era: u64, root: F) -> F {
    hash(&[dom::rh(), f_u64(era), root])
}

fn cm_key(x: &F) -> [u8; 32] {
    let mut b = [0u8; 32];
    b.copy_from_slice(x.to_repr().as_ref());
    b
}

impl Oracle {
    /// Genesis: append the genesis notes, record the genesis
    /// commitment root at root-history index 0, and fold the
    /// deposit queue into the Anchor side accumulator records.
    /// The returned genesis value carries the openings; the
    /// accumulator stored here starts at zero and steps per
    /// consumed deposit (STRAWMAN section 2.4).
    pub fn from_genesis(genesis: &Genesis) -> Result<Self> {
        let mut cm = SparseTree::new(CM_DEPTH);
        let mut cm_set = HashSet::new();
        for (i, gn) in genesis.notes.iter().enumerate() {
            if gn.leaf_index != i as u32 || gn.cm != gn.note.commitment() || gn.cm == F::ZERO {
                return err(ErrorCode::BadCommitment);
            }
            if !cm_set.insert(cm_key(&gn.cm)) {
                return err(ErrorCode::CommitmentAlreadyPresent);
            }
            cm.set(gn.leaf_index, gn.cm);
        }
        let mut rh = SparseTree::new(RH_DEPTH);
        rh.set(0, rh_entry(0, cm.root()));
        let state = StateVec {
            cm_root: cm.root(),
            cm_count: genesis.notes.len() as u32,
            nf_root: IndexedTree::new(NF_DEPTH).root(),
            nf_count: 1,
            rh_root: rh.root(),
            rh_count: 1,
            wd_root: SparseTree::new(WD_DEPTH).root(),
            wd_count: 0,
            height: 0,
            tx_count: 0,
            dep_consumed: 0,
            fee_total: 0,
        };
        if state != genesis.state || state.digest() != genesis.s_digest {
            return err(ErrorCode::WrongPreviousState);
        }
        // The genesis dep_acc is the fold of the whole queue from
        // zero; consumption steps must reproduce it at the end.
        let mut acc = F::ZERO;
        for rec in &genesis.deposit_queue {
            if rec.cm != rec.note.commitment() {
                return err(ErrorCode::BadDeposit);
            }
            acc = dep_step(acc, rec);
        }
        if acc != genesis.dep_acc {
            return err(ErrorCode::BadDeposit);
        }
        Ok(Self {
            cm,
            nf: IndexedTree::new(NF_DEPTH),
            rh,
            wd: SparseTree::new(WD_DEPTH),
            state,
            dep_acc: F::ZERO,
            deposit_queue: genesis.deposit_queue.clone(),
            cm_set,
            rh_log: vec![(0, T0 * 1000)],
            era: 0,
            now_ms: T0 * 1000,
        })
    }

    /// Test setup for the ROOT_EXPIRED case: append `n` dummy entries
    /// `H(DOM_PAD, 1_000_000 + i)` to the root-history tree, recorded at the
    /// current time.
    pub fn advance_root_history(&mut self, n: u32) {
        for i in 0..n {
            let dummy = hash(&[dom::pad(), f_u64(1_000_000 + i as u64)]);
            self.rh.set(self.state.rh_count, dummy);
            self.rh_log.push((self.era, self.now_ms));
            self.state.rh_count += 1;
        }
        self.state.rh_root = self.rh.root();
    }

    fn check_slot_shape(&self, p: &TxPublic, w: &TxWitness) -> Result<()> {
        if p.nullifiers.len() != 2
            || p.commitments.len() != 2
            || w.inputs.len() != 2
            || w.outputs.len() != 2
        {
            return err(ErrorCode::BadShape);
        }
        for (s, iw) in p.nullifiers.iter().zip(&w.inputs) {
            if s.enabled != iw.enabled {
                return err(ErrorCode::BadShape);
            }
            if !s.enabled && s.value != F::ZERO {
                return err(ErrorCode::BadShape);
            }
        }
        for (s, ow) in p.commitments.iter().zip(&w.outputs) {
            if s.enabled != ow.enabled {
                return err(ErrorCode::BadShape);
            }
            if !s.enabled && s.value != F::ZERO {
                return err(ErrorCode::BadShape);
            }
            if s.enabled && s.value == F::ZERO {
                return err(ErrorCode::BadShape);
            }
        }
        if p.compute_active_flags() != p.active_flags {
            return err(ErrorCode::BadShape);
        }
        Ok(())
    }

    fn check_kind(&self, p: &TxPublic, w: &TxWitness) -> Result<()> {
        let n_in = p.nullifiers.iter().filter(|s| s.enabled).count();
        let n_out = p.commitments.iter().filter(|s| s.enabled).count();
        match p.kind {
            TxKind::Deposit => {
                if n_in != 0
                    || n_out != 1
                    || p.fee != 0
                    || p.wd_dest.is_some()
                    || p.spender.is_some()
                    || p.rk.is_some()
                    || p.signature.is_some()
                    || w.deposit_record.is_none()
                    || w.withdrawal.is_some()
                    || p.pub_amount == 0
                {
                    return err(ErrorCode::BadKind);
                }
            }
            TxKind::Withdrawal => {
                if n_in == 0
                    || p.wd_dest.is_none()
                    || p.pub_amount == 0
                    || w.withdrawal.is_none()
                    || w.deposit_record.is_some()
                    || p.spender.is_none()
                    || p.signature.is_none()
                {
                    return err(ErrorCode::BadKind);
                }
            }
            _ => {
                if n_in == 0
                    || n_out == 0
                    || p.pub_amount != 0
                    || p.wd_dest.is_some()
                    || w.deposit_record.is_some()
                    || w.withdrawal.is_some()
                    || p.spender.is_none()
                    || p.signature.is_none()
                {
                    return err(ErrorCode::BadKind);
                }
            }
        }
        if p.pub_token != TOK_USD {
            return err(ErrorCode::BadKind);
        }
        // Per kind slot counts of the mix (STRAWMAN section 2.3).
        let shape_ok = match p.kind {
            TxKind::CheckoutChange | TxKind::Sweep => n_in == 2 && n_out >= 1,
            TxKind::CheckoutExact => n_in == 1 && n_out == 1,
            TxKind::P2p => (1..=2).contains(&n_in) && n_out >= 1,
            TxKind::Deposit => true,
            TxKind::Withdrawal => (1..=2).contains(&n_in),
        };
        if !shape_ok {
            return err(ErrorCode::BadKind);
        }
        Ok(())
    }

    pub fn apply_tx(&mut self, tx: &Tx) -> Result<()> {
        let p = &tx.public;
        let w = &tx.witness;
        // a. shape and kind
        self.check_slot_shape(p, w)?;
        self.check_kind(p, w)?;
        // b. height and expiry
        if p.height < self.state.height || p.height == 0 {
            return err(ErrorCode::HeightRegress);
        }
        if tx.state_before != self.state {
            if tx.state_before.cm_root != self.state.cm_root
                || tx.state_before.nf_root != self.state.nf_root
            {
                return err(ErrorCode::WrongPreviousRoot);
            }
            return err(ErrorCode::WrongPreviousState);
        }
        // Expiry against the applying header's authenticated time, the same
        // time the anchor window uses (seconds in the record, ms here).
        if p.expiry.saturating_mul(1000) < self.now_ms {
            return err(ErrorCode::Expired);
        }
        // c. message and record hash recomputed from the record
        if p.compute_msg() != p.msg {
            return err(ErrorCode::BadMsg);
        }
        if p.compute_record_hash() != p.record_hash {
            return err(ErrorCode::BadRecordHash);
        }
        // d. root history: membership of the entry H(DOM_RH, era, root),
        //    then the index window (exactly W_RH usable), then the time window
        if w.rh_root_before != self.state.rh_root || w.rh_count_before != self.state.rh_count {
            return err(ErrorCode::BadRhWitness);
        }
        if p.anchor_rh_index >= self.state.rh_count
            || w.rh_path.len() != RH_DEPTH
            || root_from_path(
                rh_entry(w.anchor_era, p.anchor_root),
                p.anchor_rh_index,
                &w.rh_path,
            ) != self.state.rh_root
        {
            return err(ErrorCode::UnknownRoot);
        }
        if (p.anchor_rh_index as u64) + (W_RH as u64) < self.state.rh_count as u64 {
            return err(ErrorCode::RootExpired);
        }
        let recorded_ms = self
            .rh_log
            .get(p.anchor_rh_index as usize)
            .map(|e| e.1)
            .unwrap_or(0);
        if self.now_ms.saturating_sub(recorded_ms) > RH_RETENTION_MS {
            return err(ErrorCode::RootExpired);
        }
        // e. deposit record must be the next queued record
        if let Some(rec) = &w.deposit_record {
            let next = self.state.dep_consumed + 1;
            let queued = self.deposit_queue.iter().find(|r| r.seq == next);
            match queued {
                Some(q)
                    if q.seq == rec.seq
                        && q.cm == rec.cm
                        && q.token == rec.token
                        && q.amount == rec.amount
                        && rec.cm == rec.note.commitment()
                        && rec.amount == p.pub_amount
                        && rec.token == p.pub_token => {}
                _ => return err(ErrorCode::BadDeposit),
            }
            if w.dep_acc_before != self.dep_acc || w.dep_acc_after != dep_step(self.dep_acc, rec) {
                return err(ErrorCode::BadDeposit);
            }
        } else if w.dep_acc_before != self.dep_acc || w.dep_acc_after != self.dep_acc {
            return err(ErrorCode::BadDeposit);
        }
        // f. inputs: openings, membership under the anchor, nullifiers
        let mut owner_pk: Option<Point> = None;
        let mut sum_in: u128 = 0;
        let mut nfs: Vec<F> = Vec::new();
        for (i, iw) in w.inputs.iter().enumerate() {
            if !iw.enabled {
                continue;
            }
            let cm = iw.note.commitment();
            if iw.membership_path.len() != CM_DEPTH
                || root_from_path(cm, iw.leaf_index, &iw.membership_path) != p.anchor_root
            {
                return err(ErrorCode::BadMembership);
            }
            let nf = iw.note.nullifier(w.spender_nk);
            if nf != p.nullifiers[i].value {
                return err(ErrorCode::BadNullifier);
            }
            if is_reserved_nullifier(nf) {
                return err(ErrorCode::ReservedNullifier);
            }
            if iw.note.token != p.pub_token
                || iw.note.value == 0
                || !keys::is_valid_point(iw.note.owner_public_key)
            {
                return err(ErrorCode::BadCommitment);
            }
            match owner_pk {
                None => owner_pk = Some(iw.note.owner_public_key),
                Some(pk) if pk != iw.note.owner_public_key => return err(ErrorCode::MixedOwner),
                _ => {}
            }
            if Some(iw.note.owner) != p.spender && !p.kind.is_deposit() {
                return err(ErrorCode::MixedOwner);
            }
            sum_in += iw.note.value as u128;
            nfs.push(nf);
        }
        // g. authorization: rk binding and the Schnorr equation
        if !p.kind.is_deposit() {
            let (Some(rk), Some(sig), Some(pk), Some(alpha_hex), Some(spk)) = (
                p.rk,
                &p.signature,
                owner_pk,
                &w.spender_alpha_jfr,
                w.spender_public_key,
            ) else {
                return err(ErrorCode::BadSig);
            };
            let sig = sig.clone();
            if spk != pk {
                return err(ErrorCode::BadSig);
            }
            let Some(alpha) = jfr_from_hex(alpha_hex) else {
                return err(ErrorCode::BadSig);
            };
            // The spend secret must bind both the owner key and
            // the nullifier key, or the same note could be spent
            // twice under two invented nullifier keys.
            let Some(sk_hex) = &w.spender_secret_key_jfr else {
                return err(ErrorCode::BadSig);
            };
            let Some(sk) = jfr_from_hex(sk_hex) else {
                return err(ErrorCode::BadSig);
            };
            if keys::public_key_of(sk) != pk || keys::nk_of(sk) != w.spender_nk {
                return err(ErrorCode::BadSig);
            }
            if !keys::rk_binding_ok(pk, alpha, rk) {
                return err(ErrorCode::BadSig);
            }
            if !keys::verify(rk, p.msg, &sig) {
                return err(ErrorCode::BadSig);
            }
        }
        // h. outputs: openings
        let mut sum_out: u128 = 0;
        for (i, ow) in w.outputs.iter().enumerate() {
            if !ow.enabled {
                continue;
            }
            if ow.note.commitment() != p.commitments[i].value {
                return err(ErrorCode::BadCommitment);
            }
            if ow.note.token != p.pub_token
                || ow.note.value == 0
                || !keys::is_valid_point(ow.note.owner_public_key)
            {
                return err(ErrorCode::BadCommitment);
            }
            sum_out += ow.note.value as u128;
        }
        if p.kind.is_deposit() {
            let rec = w.deposit_record.as_ref().unwrap();
            let ow = w.outputs.iter().find(|o| o.enabled).unwrap();
            if ow.note.commitment() != rec.cm || ow.note.value != rec.amount {
                return err(ErrorCode::BadDeposit);
            }
        }
        // i. value balance with the flat fee
        if !p.kind.is_deposit() && p.fee != FLAT_FEE {
            return err(ErrorCode::Unbalanced);
        }
        let wd_amount = if p.kind.is_withdrawal() {
            p.pub_amount as u128
        } else {
            0
        };
        let dep_amount = if p.kind.is_deposit() {
            p.pub_amount as u128
        } else {
            0
        };
        if sum_in + dep_amount != sum_out + wd_amount + p.fee as u128 {
            return err(ErrorCode::Unbalanced);
        }
        // j. nullifiers: duplicate inside the tx, spent, then insert
        for a in 0..nfs.len() {
            for b in a + 1..nfs.len() {
                if nfs[a] == nfs[b] {
                    return err(ErrorCode::NullifierDup);
                }
            }
        }
        for nf in &nfs {
            if self.nf.contains(*nf) {
                return err(ErrorCode::NullifierSpent);
            }
        }
        // j'. P1 (NF-046, DUP-001): no output commitment already in the
        // commitment tree or repeated in this transaction. Same position
        // as fenrir-core `L2State::apply_tx` (after the nullifier checks,
        // before any write).
        let outs: Vec<F> = w
            .outputs
            .iter()
            .filter(|o| o.enabled)
            .map(|o| o.note.commitment())
            .collect();
        for (i, c) in outs.iter().enumerate() {
            if outs[..i].contains(c) || self.cm_set.contains(&cm_key(c)) {
                return err(ErrorCode::CommitmentAlreadyPresent);
            }
        }
        for iw in w.inputs.iter().filter(|i| i.enabled) {
            let nw = &iw.nullifier;
            if nw.nf_root_before != self.nf.root() || nw.nf_count_before != self.nf.count {
                return err(ErrorCode::BadNfWitness);
            }
            let nf_value = iw.note.nullifier(w.spender_nk);
            let root_before = self.nf.root();
            let ins = match self.nf.insert(nf_value) {
                Ok(v) => v,
                Err("double_spend") => return err(ErrorCode::NullifierSpent),
                Err("reserved") => return err(ErrorCode::ReservedNullifier),
                Err(_) => return err(ErrorCode::BadNfWitness),
            };
            let pred_ok = ins.predecessor_index == nw.predecessor_index
                && ins.predecessor.value == nw.predecessor_value
                && ins.predecessor.next_index == nw.predecessor_next_index
                && ins.predecessor.next_value == nw.predecessor_next_value
                && ins.predecessor_path == nw.predecessor_path
                && ins.append_path == nw.append_path;
            if !pred_ok {
                return err(ErrorCode::BadNfWitness);
            }
            // Independently re-derive the two step roots (low-leaf
            // update, then append) so the witness cannot smuggle a
            // different intermediate tree past the insert compare.
            let pred_leaf = NullifierLeaf {
                value: nw.predecessor_value,
                next_index: nw.predecessor_next_index,
                next_value: nw.predecessor_next_value,
            };
            if root_from_path(
                nullifier_leaf_hash(&pred_leaf),
                nw.predecessor_index,
                &nw.predecessor_path,
            ) != root_before
            {
                return err(ErrorCode::BadNfWitness);
            }
            if nw.nf_root_after != self.nf.root() {
                return err(ErrorCode::BadNfWitness);
            }
            self.state.nf_root = self.nf.root();
            self.state.nf_count = self.nf.count;
        }
        // k. commitment appends
        for ow in w.outputs.iter().filter(|o| o.enabled) {
            let cmv = ow.note.commitment();
            if ow.cm_root_before != self.cm.root()
                || ow.cm_count_before != self.state.cm_count
                || ow.append_index != self.state.cm_count
                || ow.append_path.len() != CM_DEPTH
                || ow.append_path != self.cm.path(ow.append_index)
                || root_from_path(F::ZERO, ow.append_index, &ow.append_path) != self.cm.root()
            {
                return err(ErrorCode::BadCmWitness);
            }
            self.cm.set(ow.append_index, cmv);
            self.cm_set.insert(cm_key(&cmv));
            if ow.cm_root_after != self.cm.root() {
                return err(ErrorCode::BadCmWitness);
            }
            self.state.cm_root = self.cm.root();
            self.state.cm_count += 1;
        }
        // withdrawal append
        if let Some(ww) = &w.withdrawal {
            let leaf = withdrawal_leaf(ww.wd_id, p.wd_dest.unwrap(), p.pub_token, p.pub_amount);
            if leaf != ww.leaf
                || ww.wd_id != self.state.wd_count
                || ww.append_index != self.state.wd_count
                || ww.wd_root_before != self.state.wd_root
                || ww.append_path.len() != WD_DEPTH
                || ww.append_path != self.wd.path(ww.append_index)
            {
                return err(ErrorCode::BadWdWitness);
            }
            self.wd.set(ww.append_index, leaf);
            if ww.wd_root_after != self.wd.root() {
                return err(ErrorCode::BadWdWitness);
            }
            self.state.wd_root = self.wd.root();
            self.state.wd_count += 1;
        }
        // counters
        self.state.height = p.height;
        self.state.tx_count += 1;
        self.state.fee_total += p.fee;
        if p.kind.is_deposit() {
            self.state.dep_consumed += 1;
            self.dep_acc = w.dep_acc_after;
        }
        if tx.state_after != self.state {
            if tx.state_after.cm_root != self.state.cm_root
                || tx.state_after.nf_root != self.state.nf_root
            {
                return err(ErrorCode::WrongPreviousRoot);
            }
            return err(ErrorCode::WrongPreviousState);
        }
        if tx.pre_digest != tx.state_before.digest() || tx.post_digest != self.state.digest() {
            return err(ErrorCode::WrongPreviousState);
        }
        Ok(())
    }

    /// Apply one header: every transaction in order, then the
    /// height advances even for an empty header.
    pub fn apply_header(&mut self, header: &Header) -> Result<Expected> {
        if header.state_before != self.state {
            if header.state_before.cm_root != self.state.cm_root
                || header.state_before.nf_root != self.state.nf_root
            {
                return err(ErrorCode::WrongPreviousRoot);
            }
            return err(ErrorCode::WrongPreviousState);
        }
        if header.height != self.state.height + 1 {
            return err(ErrorCode::HeightRegress);
        }
        // The header's authenticated time: anchors of its transactions are
        // judged against it, and its root-history entry records it.
        let time_ms = header
            .time
            .checked_mul(1000)
            .ok_or(ErrorCode::HeightRegress)?;
        if time_ms < self.rh_log.last().map(|e| e.1).unwrap_or(0) {
            return err(ErrorCode::HeightRegress);
        }
        self.now_ms = time_ms;
        let before = self.state.clone();
        let pre_digest = before.digest();
        let mut ids = Vec::new();
        for tx in &header.txs {
            if tx.public.height != header.height {
                return err(ErrorCode::BadShape);
            }
            self.apply_tx(tx)?;
            ids.push(tx.public.id.clone());
        }
        let records: Vec<F> = header.txs.iter().map(|t| t.public.record_hash).collect();
        if records != header.record_hashes {
            return err(ErrorCode::BadRecordHash);
        }
        // Root history: exactly one entry H(DOM_RH, era, cm_root) per
        // committed header, after its last transaction (R6-AW-001).
        let w = &header.rh_append_path;
        if w.len() != RH_DEPTH
            || *w != self.rh.path(self.state.rh_count)
            || root_from_path(F::ZERO, self.state.rh_count, w) != self.state.rh_root
        {
            return err(ErrorCode::BadRhWitness);
        }
        self.rh
            .set(self.state.rh_count, rh_entry(self.era, self.state.cm_root));
        self.rh_log.push((self.era, time_ms));
        self.state.rh_root = self.rh.root();
        self.state.rh_count += 1;
        self.state.height = header.height;
        Ok(Expected {
            height: header.height,
            time: header.time,
            batch: header.batch,
            tx_ids: ids,
            state_before: before,
            state_after: self.state.clone(),
            pre_digest,
            post_digest: self.state.digest(),
            dep_acc_after: self.dep_acc,
        })
    }
}

/// Validate the batch chain: batches are consecutive, each pre
/// digest equals the previous post digest, heights tile the
/// window, `in_root` recomputes over the record hashes, and the
/// deposit range and accumulator chain continuously from the
/// pre and post states. The fee and amount sums are recomputed
/// from the headers by the dataset verifier, not here. A batch
/// built from an older state fails here (the Anchor rejects the
/// same condition in settleValidity).
pub fn validate_batches(batches: &[BatchStatement]) -> Result<()> {
    for (i, b) in batches.iter().enumerate() {
        if b.batch_no != i as u64 + 1
            || b.pre != b.state_pre.digest()
            || b.post != b.state_post.digest()
        {
            return err(ErrorCode::BadBatchChain);
        }
        if b.in_root != batch_in_root(&b.record_hashes) {
            return err(ErrorCode::BadBatchChain);
        }
        if b.h_first == 0
            || b.h_last < b.h_first
            || b.n_tx != b.record_hashes.len()
            || b.first_pos > b.last_pos
            || b.dep_from != b.state_pre.dep_consumed
            || b.dep_to != b.state_post.dep_consumed
            || b.dep_from > b.dep_to
        {
            return err(ErrorCode::BadBatchChain);
        }
        if i > 0 {
            let prev = &batches[i - 1];
            if b.pre != prev.post
                || b.h_first != prev.h_last + 1
                || b.dep_from != prev.dep_to
                || b.dep_acc_from != prev.dep_acc_to
            {
                return err(ErrorCode::BadBatchChain);
            }
        }
    }
    Ok(())
}

/// Replay the honest context an invalid case is set in: the
/// first `context_after_tx + 1` transactions in global order (no
/// transactions when it is `None`), followed by any headers with
/// no unapplied transactions up to `state_before_height`, so
/// empty headers still advance the height. Pass the invalid
/// transaction's recorded `state_before.height` as
/// `state_before_height`; the result then reproduces exactly
/// that `state_before`.
pub fn replay_context(
    genesis: &Genesis,
    headers: &[Header],
    context_after_tx: Option<usize>,
    state_before_height: u64,
) -> Result<Oracle> {
    let limit = context_after_tx.map(|u| u + 1).unwrap_or(0);
    let mut oracle = Oracle::from_genesis(genesis)?;
    let mut applied = 0usize;
    for header in headers {
        if applied >= limit && header.height > state_before_height {
            break;
        }
        if applied + header.txs.len() <= limit {
            oracle.apply_header(header)?;
            applied += header.txs.len();
            continue;
        }
        if applied < limit {
            oracle.now_ms = header.time * 1000;
            for tx in &header.txs {
                if applied >= limit {
                    break;
                }
                oracle.apply_tx(tx)?;
                applied += 1;
            }
            break;
        }
        break;
    }
    debug_assert_eq!(applied, limit);
    Ok(oracle)
}

/// Dummy root-history value for setup and for witness building.
pub fn dummy_rh_value(i: u32) -> F {
    hash(&[dom::pad(), f_u64(1_000_000 + i as u64)])
}
