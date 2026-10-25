#![cfg(feature = "sherlock-rollup-prefix")]
//! Every operation must occupy exactly one edge in the ordered trace.
use midnight_circuits::{instructions::*, types::AssignedNative};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::MockProver,
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightCircuit, Relation, ZkStdLib, ZkStdLibArch};
use mori_recursion::{
    rollup::trace::{bind_event, ordered_indices, unique_indices},
    util::F,
};
use std::io;
#[derive(Clone, Debug)]
struct Trace;
#[derive(Clone, Debug)]
struct Witness {
    states: Vec<F>,
    before: [F; 2],
    indices: [u64; 2],
}
impl Relation for Trace {
    type Instance = Vec<F>;
    type Witness = Witness;
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            nr_pow2range_cols: 1,
            ..Default::default()
        }
    }
    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _: Value<Vec<F>>,
        w: Value<Witness>,
    ) -> Result<(), Error> {
        let states: Vec<AssignedNative<F>> =
            s.assign_many(l, &w.as_ref().map(|w| w.states.clone()).transpose_vec(3))?;
        let mut indices = Vec::new();
        for i in 0..2 {
            let index = s.assign(l, w.as_ref().map(|w| F::from(w.indices[i])))?;
            let before = s.assign(l, w.as_ref().map(|w| w.before[i]))?;
            let after = s.add_constant(l, &before, F::from(1))?;
            bind_event(s, l, &states, &index, &before, &after)?;
            indices.push(index);
        }
        unique_indices(s, l, &indices)?;
        ordered_indices(s, l, &indices)?;
        s.constrain_as_public_input(l, &states[0])?;
        s.constrain_as_public_input(l, &states[2])?;
        Ok(())
    }
    fn write_relation<W: io::Write>(&self, _: &mut W) -> io::Result<()> {
        Ok(())
    }
    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Ok(Self)
    }
}
fn accepts(w: Witness) -> bool {
    let r = Trace;
    let instance = vec![F::from(0), F::from(2)];
    let c = MidnightCircuit::new(&r, Value::known(instance.clone()), Value::known(w), Some(9));
    MockProver::run(&c, vec![vec![], instance])
        .unwrap()
        .verify()
        .is_ok()
}
#[test]
fn all_edges_are_covered_once_and_in_order() {
    let base = Witness {
        states: vec![F::from(0), F::from(1), F::from(2)],
        before: [F::from(0), F::from(1)],
        indices: [0, 1],
    };
    assert!(accepts(base.clone()));
    for (indices, before) in [
        ([0, 0], [0, 0]),
        ([1, 1], [1, 1]),
        ([0, 2], [0, 1]),
        ([1, 0], [1, 0]),
    ] {
        let mut changed = base.clone();
        changed.indices = indices;
        changed.before = before.map(F::from);
        assert!(!accepts(changed), "invalid event indices {indices:?}");
    }
    let mut changed = base;
    changed.states[1] = F::from(9);
    assert!(!accepts(changed));
}
