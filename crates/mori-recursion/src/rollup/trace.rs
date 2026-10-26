//! Deterministic one-hot binding of operations to adjacent committed trace states.
//! Unique edge indices plus one operation per edge enforce complete coverage.
use super::{close::range, user::A};
use crate::util::F;
use ff::Field;
use midnight_circuits::{instructions::*, types::AssignedBit};
use midnight_proofs::{circuit::Layouter, plonk::Error};
use midnight_zk_stdlib::ZkStdLib;

pub fn bind_event(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    states: &[A],
    index: &A,
    before: &A,
    after: &A,
) -> Result<(), Error> {
    if states.len() < 2 {
        return Err(Error::Synthesis("trace has no edges".into()));
    }
    let mut count: A = s.assign_fixed(l, F::ZERO)?;
    for i in 0..states.len() - 1 {
        let selected: AssignedBit<F> = s.is_equal_to_fixed(l, index, F::from(i as u64))?;
        let bit: A = s.convert(l, &selected)?;
        count = s.add(l, &count, &bit)?;
        s.cond_assert_equal(l, &selected, before, &states[i])?;
        s.cond_assert_equal(l, &selected, after, &states[i + 1])?;
    }
    s.assert_equal_to_fixed(l, &count, F::ONE)
}

pub fn unique_indices(s: &ZkStdLib, l: &mut impl Layouter<F>, indices: &[A]) -> Result<(), Error> {
    for (i, a) in indices.iter().enumerate() {
        for b in &indices[..i] {
            let equal = s.is_equal(l, a, b)?;
            s.assert_false(l, &equal)?;
        }
    }
    Ok(())
}

pub fn ordered_indices(s: &ZkStdLib, l: &mut impl Layouter<F>, indices: &[A]) -> Result<(), Error> {
    for pair in indices.windows(2) {
        let delta = s.sub(l, &pair[1], &pair[0])?;
        let delta = s.add_constant(l, &delta, -F::ONE)?;
        range(s, l, &delta, 7)?;
    }
    Ok(())
}
