// Adapted from pinned recl9 gadgets; v3 changes described in b3-circuit.md.
//! mono: the I1 transaction (L1) rules of rec/src/transaction.rs with the
//! in-circuit wallet-proof verifier removed. `transition` is the rec method
//! body unchanged, as a free function; deposits are always allowed (rec's
//! `self.filler.is_none()` guard is dropped, because mono always handles
//! deposits). Nothing else in the rules changed.
use super::{
    i1::{
        fp::{self, F, dom},
        types::{NullifierWitness, StateVec, Tx, WithdrawalWitness},
    },
    user::{A, Gadgets},
};
use ff::Field;
use group::Group;
use midnight_circuits::{
    ecc::native::AssignedScalarOfNativeCurve, instructions::*, types::AssignedBit,
};
use midnight_curves::{JubjubExtended as Jubjub, JubjubSubgroup};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk,
};
use midnight_zk_stdlib::ZkStdLib;

pub type TreeError = plonk::Error;

#[derive(Clone, Debug)]
pub struct TransactionWitness {
    pub tx: Tx,
    pub pre_height: u64,
    pub post_height: u64,
}

pub type AssignedTransition = (Vec<A>, Vec<A>, AssignedBit<F>, Vec<A>, Vec<A>);

pub fn transition(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    w: Value<TransactionWitness>,
) -> Result<AssignedTransition, TreeError> {
    let g = Gadgets { s };
    let p = g.public(l, w.as_ref().map(|w| w.tx.public.clone()))?;
    let spend = s.not(l, &p.deposit)?;
    s.assert_equal_to_fixed(l, &p.fields[9], F::from(fp::TOK_USD))?;
    for present in [&p.rk_present, &p.signature_present, &p.spender_present] {
        s.assert_equal(l, present, &spend)?;
    }
    let zero: A = s.assign_fixed(l, F::ZERO)?;
    let one: A = s.assign_fixed(l, F::ONE)?;
    for i in [5, 6] {
        s.cond_assert_equal(l, &p.deposit, &p.fields[i], &zero)?;
    }
    let before = assign_state(&g, l, w.as_ref().map(|w| w.tx.state_before.clone()))?;
    let after = assign_state(&g, l, w.as_ref().map(|w| w.tx.state_after.clone()))?;
    let height: A = s.assign(l, w.as_ref().map(|w| F::from(w.tx.public.height)))?;
    let slot: A = s.assign(l, w.as_ref().map(|w| F::from(w.tx.public.slot)))?;
    g.range(l, &height, 64)?;
    g.range(l, &slot, 16)?;
    let pre_height: A = s.assign(l, w.as_ref().map(|w| F::from(w.pre_height)))?;
    let post_height: A = s.assign(l, w.as_ref().map(|w| F::from(w.post_height)))?;
    g.range(l, &pre_height, 64)?;
    g.range(l, &post_height, 64)?;
    g.leq(l, &pre_height, &height, 64)?;
    g.leq(l, &height, &post_height, 64)?;
    g.leq(l, &before[8], &height, 64)?;
    s.assert_equal(l, &after[8], &height)?;
    s.assert_not_equal_to_fixed(l, &height, F::ZERO)?;
    let time = s.mul_by_constant(l, &height, F::from(fp::HEADER_SECS))?;
    let time = s.add_constant(l, &time, F::from(fp::T0))?;
    g.range(l, &time, 64)?;
    g.leq(l, &time, &p.fields[14], 64)?;
    let pos = s.mul_by_constant(l, &height, F::from(65536))?;
    let pos = s.add(l, &pos, &slot)?;
    g.range(l, &pos, 80)?;
    // Retained root membership and exact strict 256-root bound.
    let rh_root: A = s.assign(l, w.as_ref().map(|w| w.tx.witness.rh_root_before))?;
    let rh_count: A = s.assign(
        l,
        w.as_ref()
            .map(|w| F::from(w.tx.witness.rh_count_before as u64)),
    )?;
    s.assert_equal(l, &rh_root, &before[4])?;
    s.assert_equal(l, &rh_count, &before[5])?;
    let rh_index: A = s.assign(
        l,
        w.as_ref()
            .map(|w| F::from(w.tx.public.anchor_rh_index as u64)),
    )?;
    g.shape(
        l,
        w.as_ref().map(|w| w.tx.witness.rh_path.len()),
        fp::RH_DEPTH,
    )?;
    let rh_path = g.assign_path(
        l,
        w.as_ref().map(|w| w.tx.witness.rh_path.clone()),
        fp::RH_DEPTH,
    )?;
    let entry = g.hash(l, dom::rh(), &[zero.clone(), p.fields[0].clone()])?;
    let retained = g.path(l, &entry, &rh_index, &rh_path)?;
    s.assert_equal(l, &retained, &before[4])?;
    let yes: AssignedBit<F> = s.assign_fixed(l, true)?;
    g.strict_if(l, &yes, &rh_index, &before[5], 32)?;
    let age = s.sub(l, &before[5], &rh_index)?;
    g.range(l, &age, 10)?;
    let limit: A = s.assign_fixed(l, F::from(fp::W_RH as u64))?;
    g.leq(l, &age, &limit, 10)?;
    s.assert_equal(l, &rh_count, &height)?;
    let era: A = s.assign(l, w.as_ref().map(|w| F::from(w.tx.witness.anchor_era)))?;
    s.assert_equal_to_fixed(l, &era, F::ZERO)?;
    // Nullifier inserts use two authenticated updates with shared paths.
    g.shape(l, w.as_ref().map(|w| w.tx.witness.inputs.len()), 2)?;
    let mut nr = before[2].clone();
    let mut nc = before[3].clone();
    for i in 0..2 {
        let input = w.as_ref().map(|w| w.tx.witness.inputs.get(i).cloned());
        let enabled: AssignedBit<F> = s.assign(
            l,
            input
                .as_ref()
                .map(|v| v.as_ref().is_some_and(|v| v.enabled)),
        )?;
        s.assert_equal(l, &enabled, &p.nf_enabled[i])?;
        let nw = input.map(|v| {
            v.map(|v| v.nullifier)
                .unwrap_or_else(|| NullifierWitness::zero(fp::NF_DEPTH))
        });
        (nr, nc) = insert_nf(&g, l, &nr, &nc, &p.fields[1 + i], &enabled, nw)?;
    }
    s.assert_equal(l, &nr, &after[2])?;
    s.assert_equal(l, &nc, &after[3])?;
    // Commitment appends read only public commitment values and paths.
    g.shape(l, w.as_ref().map(|w| w.tx.witness.outputs.len()), 2)?;
    let mut cr = before[0].clone();
    let mut cc = before[1].clone();
    for i in 0..2 {
        let ow = w.as_ref().map(|w| w.tx.witness.outputs.get(i).cloned());
        let enabled: AssignedBit<F> = s.assign(
            l,
            ow.as_ref().map(|v| v.as_ref().is_some_and(|v| v.enabled)),
        )?;
        s.assert_equal(l, &enabled, &p.cm_enabled[i])?;
        let index: A = s.assign(
            l,
            ow.as_ref()
                .map(|v| F::from(v.as_ref().map(|v| v.append_index as u64).unwrap_or(0))),
        )?;
        s.cond_assert_equal(l, &enabled, &index, &cc)?;
        let claimed_before: A = s.assign(
            l,
            ow.as_ref()
                .map(|v| v.as_ref().map(|v| v.cm_root_before).unwrap_or(F::ZERO)),
        )?;
        let claimed_count: A = s.assign(
            l,
            ow.as_ref()
                .map(|v| F::from(v.as_ref().map(|v| v.cm_count_before as u64).unwrap_or(0))),
        )?;
        s.cond_assert_equal(l, &enabled, &claimed_before, &cr)?;
        s.cond_assert_equal(l, &enabled, &claimed_count, &cc)?;
        g.shape(
            l,
            ow.as_ref()
                .map(|v| v.as_ref().map(|v| v.append_path.len()).unwrap_or(0)),
            fp::CM_DEPTH,
        )?;
        let path = g.assign_path(
            l,
            ow.as_ref().map(|v| {
                v.as_ref()
                    .map(|v| v.append_path.clone())
                    .unwrap_or_default()
            }),
            fp::CM_DEPTH,
        )?;
        let old = g.path(l, &zero, &cc, &path)?;
        s.cond_assert_equal(l, &enabled, &old, &cr)?;
        let new = g.path(l, &p.fields[3 + i], &cc, &path)?;
        let claimed_after: A = s.assign(
            l,
            ow.as_ref()
                .map(|v| v.as_ref().map(|v| v.cm_root_after).unwrap_or(F::ZERO)),
        )?;
        s.cond_assert_equal(l, &enabled, &claimed_after, &new)?;
        cr = s.select(l, &enabled, &new, &cr)?;
        let en: A = s.convert(l, &enabled)?;
        cc = s.add(l, &cc, &en)?;
        g.range(l, &cc, 32)?;
    }
    s.assert_equal(l, &cr, &after[0])?;
    s.assert_equal(l, &cc, &after[1])?;
    // v3 records exactly one era-wrapped root at header close.
    s.assert_equal(l, &before[4], &after[4])?;
    s.assert_equal(l, &before[5], &after[5])?;
    // Withdrawal append, with an authenticated zero leaf at the count.
    let wd = w.as_ref().map(|w| w.tx.witness.withdrawal.clone());
    let wd_present: AssignedBit<F> = s.assign(l, wd.as_ref().map(|v| v.is_some()))?;
    s.assert_equal(l, &wd_present, &p.withdrawal)?;
    let wd = wd.map(|v| {
        v.unwrap_or(WithdrawalWitness {
            wd_id: 0,
            leaf: F::ZERO,
            append_index: 0,
            append_path: vec![F::ZERO; fp::WD_DEPTH],
            wd_root_before: F::ZERO,
            wd_root_after: F::ZERO,
        })
    });
    let wd_id: A = s.assign(l, wd.as_ref().map(|v| F::from(v.wd_id as u64)))?;
    let wd_index: A = s.assign(l, wd.as_ref().map(|v| F::from(v.append_index as u64)))?;
    s.cond_assert_equal(l, &p.withdrawal, &wd_id, &before[7])?;
    s.cond_assert_equal(l, &p.withdrawal, &wd_index, &before[7])?;
    let leaf = g.hash(
        l,
        dom::wd(),
        &[
            wd_id,
            p.fields[12].clone(),
            p.fields[9].clone(),
            p.fields[10].clone(),
        ],
    )?;
    let claim: A = s.assign(l, wd.as_ref().map(|v| v.leaf))?;
    s.cond_assert_equal(l, &p.withdrawal, &claim, &leaf)?;
    let path = g.assign_path(l, wd.as_ref().map(|v| v.append_path.clone()), fp::WD_DEPTH)?;
    g.shape(l, wd.as_ref().map(|v| v.append_path.len()), fp::WD_DEPTH)?;
    let old = g.path(l, &zero, &before[7], &path)?;
    s.cond_assert_equal(l, &p.withdrawal, &old, &before[6])?;
    let claim: A = s.assign(l, wd.as_ref().map(|v| v.wd_root_before))?;
    s.cond_assert_equal(l, &p.withdrawal, &claim, &before[6])?;
    let new = g.path(l, &leaf, &before[7], &path)?;
    let claim: A = s.assign(l, wd.as_ref().map(|v| v.wd_root_after))?;
    s.cond_assert_equal(l, &p.withdrawal, &claim, &new)?;
    let wr = s.select(l, &p.withdrawal, &new, &before[6])?;
    s.assert_equal(l, &wr, &after[6])?;
    let wd_en: A = s.convert(l, &p.withdrawal)?;
    let wc = s.add(l, &before[7], &wd_en)?;
    s.assert_equal(l, &wc, &after[7])?;
    // Deposit record is bound to exactly one output and the accumulator.
    let rec = w.as_ref().map(|w| w.tx.witness.deposit_record.clone());
    let present: AssignedBit<F> = s.assign(l, rec.as_ref().map(|v| v.is_some()))?;
    s.assert_equal(l, &present, &p.deposit)?;
    let seq: A = s.assign(
        l,
        rec.as_ref()
            .map(|v| F::from(v.as_ref().map(|v| v.seq).unwrap_or(0))),
    )?;
    let token: A = s.assign(
        l,
        rec.as_ref()
            .map(|v| F::from(v.as_ref().map(|v| v.token).unwrap_or(0))),
    )?;
    let amount: A = s.assign(
        l,
        rec.as_ref()
            .map(|v| F::from(v.as_ref().map(|v| v.amount).unwrap_or(0))),
    )?;
    let cm: A = s.assign(
        l,
        rec.as_ref()
            .map(|v| v.as_ref().map(|v| v.cm).unwrap_or(F::ZERO)),
    )?;
    g.range(l, &seq, 64)?;
    g.range(l, &token, 64)?;
    g.range(l, &amount, 64)?;
    let next = s.add(l, &before[10], &one)?;
    s.cond_assert_equal(l, &p.deposit, &seq, &next)?;
    s.cond_assert_equal(l, &p.deposit, &token, &p.fields[9])?;
    s.cond_assert_equal(l, &p.deposit, &amount, &p.fields[10])?;
    let opening = rec.as_ref().map(|v| {
        v.as_ref()
            .map(|v| v.note.clone())
            .unwrap_or_else(super::i1::types::NoteOpening::zero)
    });
    let (derived, nt, nv, _, _) = g.opening(l, opening, &p.deposit)?;
    s.cond_assert_equal(l, &p.deposit, &derived, &cm)?;
    s.cond_assert_equal(l, &p.deposit, &nt, &token)?;
    s.cond_assert_equal(l, &p.deposit, &nv, &amount)?;
    let output = s.add(l, &p.fields[3], &p.fields[4])?;
    s.cond_assert_equal(l, &p.deposit, &cm, &output)?;
    let dep_acc_before: A = s.assign(l, w.as_ref().map(|w| w.tx.witness.dep_acc_before))?;
    let dep_acc_after: A = s.assign(l, w.as_ref().map(|w| w.tx.witness.dep_acc_after))?;
    let step = g.hash(
        l,
        dom::dep(),
        &[dep_acc_before.clone(), seq, token, amount, cm],
    )?;
    let expected = s.select(l, &p.deposit, &step, &dep_acc_before)?;
    s.assert_equal(l, &expected, &dep_acc_after)?;
    let dep: A = s.convert(l, &p.deposit)?;
    let consumed = s.add(l, &before[10], &dep)?;
    s.assert_equal(l, &consumed, &after[10])?;
    let tx_count = s.add(l, &before[9], &one)?;
    s.assert_equal(l, &tx_count, &after[9])?;
    let fee_total = s.add(l, &before[11], &p.fields[8])?;
    s.assert_equal(l, &fee_total, &after[11])?;
    // Transaction signature uses the complete, canonical BLS field challenge.
    let msg_network: A = s.assign_fixed(l, fp::network_tag())?;
    let msg = g.hash(
        l,
        dom::txmsg(),
        &[
            msg_network,
            p.fields[7].clone(),
            p.fields[1].clone(),
            p.fields[2].clone(),
            p.fields[3].clone(),
            p.fields[4].clone(),
            p.fields[8].clone(),
            p.fields[10].clone(),
            p.fields[12].clone(),
            p.fields[13].clone(),
            p.fields[14].clone(),
        ],
    )?;
    let claimed_msg: A = s.assign(l, w.as_ref().map(|w| w.tx.public.msg))?;
    s.assert_equal(l, &msg, &claimed_msg)?;
    let fallback = super::i1::keys::generator_point();
    let (rk, rkc) = g.point(
        l,
        w.as_ref().map(|w| w.tx.public.rk.unwrap_or(fallback)),
        None,
    )?;
    for (j, coordinate) in rkc.iter().enumerate() {
        s.cond_assert_equal(l, &spend, coordinate, &p.fields[5 + j])?;
    }
    let (r, rc) = g.point(
        l,
        w.as_ref().map(|w| {
            w.tx.public
                .signature
                .as_ref()
                .map(|v| v.r_point)
                .unwrap_or(fallback)
        }),
        None,
    )?;
    let response: A = s.assign(
        l,
        w.as_ref().map(|w| {
            w.tx.public
                .signature
                .as_ref()
                .map(|v| fp::jfr_to_f(v.s))
                .unwrap_or(F::ONE)
        }),
    )?;
    g.scalar(l, &response)?;
    s.assert_not_equal_to_fixed(l, &response, F::ZERO)?;
    let challenge = g.hash(
        l,
        dom::sig(),
        &[
            rc[0].clone(),
            rc[1].clone(),
            rkc[0].clone(),
            rkc[1].clone(),
            msg,
        ],
    )?;
    let ss: AssignedScalarOfNativeCurve<Jubjub> = s.jubjub().convert(l, &response)?;
    let es: AssignedScalarOfNativeCurve<Jubjub> = s.jubjub().convert(l, &challenge)?;
    let generator = s.jubjub().assign_fixed(l, JubjubSubgroup::generator())?;
    let lhs = s.jubjub().msm(l, &[ss], &[generator])?;
    let e_rk = s.jubjub().msm(l, &[es], &[rk])?;
    let rhs = EccInstructions::add(s.jubjub(), l, &r, &e_rk)?;
    s.cond_assert_equal(
        l,
        &spend,
        &s.jubjub().x_coordinate(&lhs),
        &s.jubjub().x_coordinate(&rhs),
    )?;
    s.cond_assert_equal(
        l,
        &spend,
        &s.jubjub().y_coordinate(&lhs),
        &s.jubjub().y_coordinate(&rhs),
    )?;
    // Public record hash, independent of witness secrets and signature bytes.
    let network: A = s.assign_fixed(l, fp::network_tag())?;
    let mut record = vec![
        network,
        p.fields[7].clone(),
        height,
        slot,
        p.fields[0].clone(),
    ];
    for i in 0..2 {
        let en: A = s.convert(l, &p.nf_enabled[i])?;
        record.extend([en, p.fields[1 + i].clone()]);
    }
    for i in 0..2 {
        let en: A = s.convert(l, &p.cm_enabled[i])?;
        record.extend([en, p.fields[3 + i].clone()]);
    }
    record.extend([
        p.fields[5].clone(),
        p.fields[6].clone(),
        p.fields[8].clone(),
        p.fields[9].clone(),
        p.fields[10].clone(),
        p.fields[11].clone(),
        p.fields[12].clone(),
        p.fields[13].clone(),
        p.fields[14].clone(),
    ]);
    let record = g.hash(l, dom::tx(), &record)?;
    let claim: A = s.assign(l, w.as_ref().map(|w| w.tx.public.record_hash))?;
    s.assert_equal(l, &record, &claim)?;
    let raw_pre = g.hash(l, dom::state(), &before)?;
    let raw_post = g.hash(l, dom::state(), &after)?;
    let claim: A = s.assign(l, w.as_ref().map(|w| w.tx.pre_digest))?;
    s.assert_equal(l, &raw_pre, &claim)?;
    let claim: A = s.assign(l, w.as_ref().map(|w| w.tx.post_digest))?;
    s.assert_equal(l, &raw_post, &claim)?;
    s.assert_equal(l, &pre_height, &before[8])?;
    s.assert_equal(l, &post_height, &after[8])?;
    let mut normalized_pre = before.clone();
    normalized_pre[8] = pre_height;
    let mut normalized_post = after.clone();
    normalized_post[8] = post_height;
    let pre = g.hash(l, dom::state(), &normalized_pre)?;
    let post = g.hash(l, dom::state(), &normalized_post)?;
    let wd_amount = s.select(l, &p.withdrawal, &p.fields[10], &zero)?;
    let dep_amount = s.select(l, &p.deposit, &p.fields[10], &zero)?;
    let mut wd_sums = [zero.clone(), zero.clone()];
    let mut dep_sums = [zero.clone(), zero.clone()];
    g.add_token(l, &mut wd_sums, &p.fields[9], &wd_amount)?;
    g.add_token(l, &mut dep_sums, &p.fields[9], &dep_amount)?;
    let segment = vec![
        zero.clone(),
        pre,
        post,
        pos.clone(),
        pos,
        record,
        before[10].clone(),
        after[10].clone(),
        dep_acc_before,
        dep_acc_after,
        p.fields[8].clone(),
        wd_sums[0].clone(),
        wd_sums[1].clone(),
        dep_sums[0].clone(),
        dep_sums[1].clone(),
        zero,
    ];
    Ok((segment, p.fields, p.deposit, before, after))
}
fn assign_state(
    g: &Gadgets<'_>,
    l: &mut impl Layouter<F>,
    w: Value<StateVec>,
) -> Result<Vec<A>, plonk::Error> {
    let fields = (0..12)
        .map(|i| g.s.assign(l, w.as_ref().map(|s| s.fields()[i])))
        .collect::<Result<Vec<A>, _>>()?;
    for i in [1, 3, 5, 7] {
        g.range(l, &fields[i], 32)?;
    }
    for i in [8, 9, 10, 11] {
        g.range(l, &fields[i], 64)?;
    }
    Ok(fields)
}
fn insert_nf(
    g: &Gadgets<'_>,
    l: &mut impl Layouter<F>,
    root: &A,
    count: &A,
    value: &A,
    en: &AssignedBit<F>,
    w: Value<NullifierWitness>,
) -> Result<(A, A), plonk::Error> {
    let s = g.s;
    let index: A = s.assign(l, w.as_ref().map(|n| F::from(n.predecessor_index as u64)))?;
    let pred: A = s.assign(l, w.as_ref().map(|n| n.predecessor_value))?;
    let next_index: A = s.assign(
        l,
        w.as_ref().map(|n| F::from(n.predecessor_next_index as u64)),
    )?;
    let next_value: A = s.assign(l, w.as_ref().map(|n| n.predecessor_next_value))?;
    for x in [value, &pred, &next_value] {
        g.range(l, x, 248)?;
    }
    for x in [&index, &next_index] {
        g.range(l, x, 32)?;
        g.strict_if(l, en, x, count, 32)?;
    }
    g.strict_if(l, en, &pred, value, 248)?;
    g.strict_if(l, en, value, &next_value, 248)?;
    let claimed: A = s.assign(l, w.as_ref().map(|n| n.nf_root_before))?;
    s.cond_assert_equal(l, en, &claimed, root)?;
    let claimed: A = s.assign(l, w.as_ref().map(|n| F::from(n.nf_count_before as u64)))?;
    s.cond_assert_equal(l, en, &claimed, count)?;
    let pred_path = g.assign_path(
        l,
        w.as_ref().map(|n| n.predecessor_path.clone()),
        fp::NF_DEPTH,
    )?;
    g.shape(
        l,
        w.as_ref().map(|v| v.predecessor_path.len()),
        fp::NF_DEPTH,
    )?;
    g.shape(l, w.as_ref().map(|v| v.append_path.len()), fp::NF_DEPTH)?;
    let append_path = g.assign_path(l, w.as_ref().map(|n| n.append_path.clone()), fp::NF_DEPTH)?;
    let old = g.hash(
        l,
        dom::nf_leaf(),
        &[pred.clone(), next_index.clone(), next_value.clone()],
    )?;
    let old = g.path(l, &old, &index, &pred_path)?;
    s.cond_assert_equal(l, en, &old, root)?;
    let updated = g.hash(l, dom::nf_leaf(), &[pred, count.clone(), value.clone()])?;
    let intermediate = g.path(l, &updated, &index, &pred_path)?;
    let zero: A = s.assign_fixed(l, F::ZERO)?;
    let empty = g.path(l, &zero, count, &append_path)?;
    s.cond_assert_equal(l, en, &empty, &intermediate)?;
    let leaf = g.hash(l, dom::nf_leaf(), &[value.clone(), next_index, next_value])?;
    let new = g.path(l, &leaf, count, &append_path)?;
    let claim: A = s.assign(l, w.map(|n| n.nf_root_after))?;
    s.cond_assert_equal(l, en, &claim, &new)?;
    let new = s.select(l, en, &new, root)?;
    let en: A = s.convert(l, en)?;
    let count = s.add(l, count, &en)?;
    g.range(l, &count, 32)?;
    Ok((new, count))
}
