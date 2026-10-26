//! One v3 root-history append per header, including empty headers.
use super::i1::{
    fp::{self, F, dom},
    types::StateVec,
};
use ff::{Field, PrimeField};
use midnight_circuits::{instructions::*, types::AssignedNative};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};
use std::io;

type A = AssignedNative<F>;

#[derive(Clone, Debug)]
pub struct CloseWitness {
    pub before: StateVec,
    pub after: StateVec,
    pub height: u64,
    pub path: Vec<F>,
}

impl CloseWitness {
    pub fn instance(&self) -> Vec<F> {
        vec![
            self.before.digest(),
            self.after.digest(),
            F::from(self.height),
        ]
    }
}

pub fn assign_state(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    state: Value<StateVec>,
) -> Result<Vec<A>, Error> {
    let cells = s.assign_many(l, &state.map(|v| v.fields().to_vec()).transpose_vec(12))?;
    for i in [1, 3, 5, 7] {
        range(s, l, &cells[i], 32)?;
    }
    for i in [8, 9, 10, 11] {
        range(s, l, &cells[i], 64)?;
    }
    Ok(cells)
}

pub fn hash(s: &ZkStdLib, l: &mut impl Layouter<F>, tag: F, fields: &[A]) -> Result<A, Error> {
    let d: A = s.assign_fixed(l, tag)?;
    s.poseidon(l, &[vec![d], fields.to_vec()].concat())
}

pub fn path_root(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    leaf: &A,
    index: &A,
    path: &[A],
) -> Result<A, Error> {
    let bits = s.assigned_to_le_bits(l, index, Some(path.len()), true)?;
    let mut node = leaf.clone();
    for (bit, sibling) in bits.iter().zip(path) {
        let left = s.select(l, bit, sibling, &node)?;
        let right = s.select(l, bit, &node, sibling)?;
        node = hash(s, l, dom::node(), &[left, right])?;
    }
    Ok(node)
}

/// No witness-selected branches: all state lanes except RH and height are copied.
pub fn close_header(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    before: &[A],
    height: &A,
    path: Value<Vec<F>>,
) -> Result<Vec<A>, Error> {
    let n: A = s.assign(l, path.as_ref().map(|v| F::from(v.len() as u64)))?;
    s.assert_equal_to_fixed(l, &n, F::from(fp::RH_DEPTH as u64))?;
    let path = s.assign_many(l, &path.transpose_vec(fp::RH_DEPTH))?;
    let zero: A = s.assign_fixed(l, F::ZERO)?;
    let old = path_root(s, l, &zero, &before[5], &path)?;
    s.assert_equal(l, &old, &before[4])?;
    let entry = hash(s, l, dom::rh(), &[zero, before[0].clone()])?;
    let root = path_root(s, l, &entry, &before[5], &path)?;
    let count = s.add_constant(l, &before[5], F::ONE)?;
    range(s, l, &count, 32)?;
    let mut after = before.to_vec();
    after[4] = root;
    after[5] = count;
    after[8] = height.clone();
    Ok(after)
}

#[derive(Clone, Debug)]
pub struct CloseRelation;
impl Relation for CloseRelation {
    type Instance = Vec<F>;
    type Witness = CloseWitness;
    type Error = Error;
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            nr_pow2range_cols: 4,
            ..Default::default()
        }
    }
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _: Value<Vec<F>>,
        w: Value<CloseWitness>,
    ) -> Result<(), Error> {
        let before = assign_state(s, l, w.as_ref().map(|w| w.before.clone()))?;
        let claimed = assign_state(s, l, w.as_ref().map(|w| w.after.clone()))?;
        let height: A = s.assign(l, w.as_ref().map(|w| F::from(w.height)))?;
        range(s, l, &height, 64)?;
        // Immediately before close, height is h-1 (empty) or h (nonempty).
        let delta = s.sub(l, &height, &before[8])?;
        range(s, l, &delta, 1)?;
        let expected_count = s.add_constant(l, &height, F::ONE)?;
        s.assert_equal(l, &expected_count, &claimed[5])?;
        let after = close_header(s, l, &before, &height, w.map(|w| w.path))?;
        for (a, b) in after.iter().zip(&claimed) {
            s.assert_equal(l, a, b)?;
        }
        let pre = hash(s, l, dom::state(), &before)?;
        let post = hash(s, l, dom::state(), &after)?;
        for cell in [pre, post, height] {
            s.constrain_as_public_input(l, &cell)?;
        }
        Ok(())
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"FSYNC001")
    }
    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut bytes = Vec::new();
        r.read_to_end(&mut bytes)?;
        if bytes != b"FSYNC001" {
            return Err(io::Error::other("invalid v3 close relation"));
        }
        Ok(Self)
    }
}

pub fn range(s: &ZkStdLib, l: &mut impl Layouter<F>, x: &A, bits: usize) -> Result<(), Error> {
    // Decompose a safely truncated witness, then constrain it to the source.
    // The stock decomposition CPU helper panics on an out-of-range value.
    let safe: A = s.assign(
        l,
        x.value().map(|value| {
            let mut bytes = value.to_repr();
            for i in bits..256 {
                bytes.as_mut()[i / 8] &= !(1 << (i % 8));
            }
            F::from_repr(bytes).unwrap()
        }),
    )?;
    s.assert_equal(l, x, &safe)?;
    s.assigned_to_le_bits(l, &safe, Some(bits), true)?;
    Ok(())
}
