// Adapted from pinned recl9 segment composition.
//! mono: the in-circuit segment composition and endpoint openings of
//! rec/src/segment_tree.rs (`range`, `validate_assigned`, `compose_assigned`,
//! `constrain_endpoint_states`), unchanged. The verifier halves of
//! `SegmentNode` and `BatchWrapper` are not copied.
use super::{
    i1::fp::{F, dom},
    segment::err,
};
use ff::Field;
use midnight_circuits::{instructions::*, types::AssignedNative};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk,
};
use midnight_zk_stdlib::ZkStdLib;
type A = AssignedNative<F>;
type TreeError = plonk::Error;

pub fn range(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    x: &A,
    bits: usize,
) -> Result<(), plonk::Error> {
    s.assigned_to_le_chunks(l, x, 8, Some(bits / 8))?;
    Ok(())
}
pub fn validate_assigned(s: &ZkStdLib, l: &mut impl Layouter<F>, f: &[A]) -> Result<(), TreeError> {
    if f.len() != 16 {
        return Err(err("segment shape"));
    }
    let pad = s.is_equal_to_fixed(l, &f[0], F::ONE)?;
    let real = s.is_equal_to_fixed(l, &f[0], F::ZERO)?;
    let ok = s.or(l, &[pad.clone(), real])?;
    s.assert_true(l, &ok)?;
    let zero: A = s.assign_fixed(l, F::ZERO)?;
    for x in &f[1..] {
        s.cond_assert_equal(l, &pad, x, &zero)?;
    }
    s.assert_equal_to_fixed(l, &f[15], F::ZERO)?;
    for i in [3, 4] {
        range(s, l, &f[i], 80)?;
    }
    for i in [6, 7, 10, 11, 12, 13, 14] {
        range(s, l, &f[i], 64)?;
    }
    let order = s.sub(l, &f[4], &f[3])?;
    range(s, l, &order, 80)?;
    let order = s.sub(l, &f[7], &f[6])?;
    range(s, l, &order, 64)?;
    Ok(())
}
pub fn compose_assigned(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    cs: &[Vec<A>],
) -> Result<Vec<A>, TreeError> {
    if !(2..=4).contains(&cs.len()) {
        return Err(err("assigned node arity"));
    }
    let zero: A = s.assign_fixed(l, F::ZERO)?;
    let mut output = cs[0].clone();
    let mut previous = cs[0].clone();
    let mut all_pad = s.is_equal_to_fixed(l, &cs[0][0], F::ONE)?;
    let mut saw_pad = all_pad.clone();
    let domain = if cs.len() == 4 {
        dom::in4()
    } else if cs.len() == 2 {
        dom::in2()
    } else {
        super::i1::fp::field_from_tag("DOM_IN3")
    };
    let mut hashes = vec![s.assign_fixed(l, domain)?];
    let pad_hash: A = s.assign_fixed(l, super::i1::types::pad_leaf())?;
    for (i, c) in cs.iter().enumerate() {
        validate_assigned(s, l, c)?;
        let pad = s.is_equal_to_fixed(l, &c[0], F::ONE)?;
        let real = s.not(l, &pad)?;
        let illegal = s.and(l, &[saw_pad.clone(), real.clone()])?;
        s.assert_false(l, &illegal)?;
        hashes.push(s.select(l, &pad, &pad_hash, &c[5])?);
        if i > 0 {
            for (a, b) in [(2, 1), (7, 6), (9, 8)] {
                s.cond_assert_equal(l, &real, &previous[a], &c[b])?;
            }
            let diff = s.sub(l, &c[3], &previous[4])?;
            let diff = s.add_constant(l, &diff, -F::ONE)?;
            let checked = s.select(l, &real, &diff, &zero)?;
            range(s, l, &checked, 80)?;
            for j in [2, 4, 7, 9] {
                output[j] = s.select(l, &real, &c[j], &output[j])?;
            }
            for j in 10..15 {
                output[j] = s.add(l, &output[j], &c[j])?;
                range(s, l, &output[j], 64)?;
            }
            previous = c.clone();
            all_pad = s.and(l, &[all_pad, pad.clone()])?;
        }
        saw_pad = s.or(l, &[saw_pad, pad])?;
    }
    let digest = s.poseidon(l, &hashes)?;
    output[5] = s.select(l, &all_pad, &zero, &digest)?;
    let one: A = s.assign_fixed(l, F::ONE)?;
    output[0] = s.select(l, &all_pad, &one, &zero)?;
    Ok(output)
}
/// Open the segment endpoint digests and bind their exact batch heights.
pub fn constrain_endpoint_states(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    f: &[A],
    pre_height: &A,
    post_height: &A,
    states: Value<(super::i1::types::StateVec, super::i1::types::StateVec)>,
) -> Result<(), TreeError> {
    for (pre, index, height) in [(true, 1, pre_height), (false, 2, post_height)] {
        let fields = states.as_ref().map(|states| {
            if pre {
                states.0.fields().to_vec()
            } else {
                states.1.fields().to_vec()
            }
        });
        let assigned: Vec<A> = s.assign_many(l, &fields.transpose_vec(12))?;
        s.assert_equal(l, &assigned[8], height)?;
        let mut preimage = vec![s.assign_fixed(l, dom::state())?];
        preimage.extend(assigned);
        let digest = s.poseidon(l, &preimage)?;
        s.assert_equal(l, &digest, &f[index])?;
    }
    Ok(())
}
