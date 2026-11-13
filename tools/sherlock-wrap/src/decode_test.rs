//! Test relation of the decoder alone: the `L_Fr` public words as witness cells, observed into
//! a fresh `L_Fr` transcript, then `decode`; the statement lanes are the public inputs.
use std::io;

use midnight_circuits::instructions::{AssignmentInstructions, PublicInputInstructions};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    alg::Gadget,
    decode::{DecodeShape, decode},
    native::{F, U_LFR_TRANSCRIPT},
    smoke::wrap_arch,
    transcript::Transcript,
};

#[derive(Clone, Debug)]
pub struct DecodeRelation {
    pub sh: DecodeShape,
}

impl Relation for DecodeRelation {
    type Instance = Vec<F>;
    type Witness = Vec<F>;
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(&self, std: &ZkStdLib, l: &mut impl Layouter<F>, _: Value<Vec<F>>, w: Value<Vec<F>>) -> Result<(), Error> {
        let n = self.sh.d.n_public();
        let mut pw = vec![];
        for i in 0..n {
            pw.push(std.assign(l, w.as_ref().map(|w| w[i]))?);
        }
        let s = {
            let mut a = Gadget::new(std, &mut *l);
            let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT)?;
            t.observe_all(&mut a, &pw)?;
            decode(&mut a, &mut t, &self.sh, &pw)?
        };
        for c in &s {
            std.constrain_as_public_input(l, c)?;
        }
        Ok(())
    }
    fn used_chips(&self) -> ZkStdLibArch {
        wrap_arch(1)
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"decode")
    }
    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("decode"))
    }
}

