//! A stand-in leaf with the frozen 5-lane leaf statement, for rehearsing the fold,
//! decider and ledger path before the C6 universal leaf exists. It proves nothing
//! about the Sherlock machine: post_d = state_digest(tag, value, Poseidon(prog_d,
//! pre_d, rule, h_pre)) with private (tag, value). Never deployed as a qualification.
use std::io;

use midnight_circuits::{
    instructions::{AssertionInstructions, AssignmentInstructions, PublicInputInstructions},
    types::AssignedNative,
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    ivc::DOM_STATE_D,
    parent::{dom, poseidon},
    util::F,
};

#[derive(Clone, Debug)]
pub struct StandInLeaf;

/// Witness: the 5 statement lanes, then tag and value.
pub type StandInWitness = Vec<F>;

pub fn standin_body(prog_d: F, pre_d: F, rule: F, h_pre: F) -> F {
    poseidon(&[prog_d, pre_d, rule, h_pre])
}

impl Relation for StandInLeaf {
    type Instance = Vec<F>;
    type Witness = StandInWitness;
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _i: Value<Vec<F>>,
        w: Value<Vec<F>>,
    ) -> Result<(), Error> {
        let mut v: Vec<AssignedNative<F>> = Vec::new();
        for i in 0..5 {
            let x: AssignedNative<F> = std.assign(l, w.as_ref().map(|w| w[i]))?;
            std.constrain_as_public_input(l, &x)?;
            v.push(x);
        }
        let body = std.poseidon(l, &[v[0].clone(), v[1].clone(), v[3].clone(), v[4].clone()])?;
        let d: AssignedNative<F> = std.assign_fixed(l, dom(DOM_STATE_D))?;
        let tag: AssignedNative<F> = std.assign(l, w.as_ref().map(|w| w[5]))?;
        let val: AssignedNative<F> = std.assign(l, w.as_ref().map(|w| w[6]))?;
        let post = std.poseidon(l, &[d, tag, val, body])?;
        std.assert_equal(l, &post, &v[2])
    }
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            ..ZkStdLibArch::default()
        }
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"SHKSTND1")
    }
    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut b = [0u8; 8];
        r.read_exact(&mut b)?;
        if &b != b"SHKSTND1" {
            return Err(io::Error::other("not the stand-in leaf"));
        }
        Ok(StandInLeaf)
    }
}
