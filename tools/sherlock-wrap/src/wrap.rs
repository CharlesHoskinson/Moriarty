//! I6: the wrap relation. The whole `L_Fr` verifier (`part1` to `part4`) on the gadget back
//! end; the statement is 1,026 words. The `L_Fr` shape (params, presence, heights, DAGs,
//! layouts, WHIR rounds, preprocessed commitments, vk pre-hash) is pinned in the circuit as
//! constants, so it is fixed by the key.
//!
//! `PublicMap` (LEAD, PLAN section 4, to be replaced by the E3 map): the first `n` words are
//! the `L_Fr` public values in AIR order, the other `1,026 - n` words are constrained zero.

use std::io;

use midnight_circuits::instructions::PublicInputInstructions;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    alg::{Alg, Gadget},
    native::F,
    smoke::{NPI, wrap_arch},
    stacked::{StackShape, part3},
    transcript::Transcript,
    verifier::{ProofData, Shape, commit_cells, part1, part2},
    whir::{WhirShape, part4},
    wrapin::WrapIn,
};

#[derive(Clone, Debug)]
pub struct WrapRelation {
    pub shape: Shape,
    pub stack: StackShape,
    pub whir: WhirShape,
    pub nr_cols: u8,
}

impl WrapRelation {
    pub fn from_wrapin(w: &WrapIn, nr_cols: u8) -> Self {
        let shape = Shape::from_wrapin(w);
        let stack = StackShape::from_wrapin(w, &shape);
        let whir = WhirShape::from_wrapin(w, &stack.widths);
        let n: usize = shape.airs.iter().map(|a| if a.log_height.is_some() { a.num_public_values } else { 0 }).sum();
        assert!(n <= NPI, "PublicMap: {n} public values exceed {NPI} words");
        WrapRelation { shape, stack, whir, nr_cols }
    }

    /// The statement of a proof under `PublicMap`.
    pub fn instance(&self, p: &ProofData) -> Vec<F> {
        let mut v: Vec<F> = p.public_values.iter().flatten().copied().collect();
        v.resize(NPI, F::from(0u64));
        v
    }
}

impl Relation for WrapRelation {
    type Instance = Vec<F>;
    type Witness = ProofData;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        witness: Value<ProofData>,
    ) -> Result<(), Error> {
        let mut cells = vec![];
        {
            let mut a = Gadget::new(std, &mut *l);
            let mut t = Transcript::new(&mut a, self.shape.u_transcript)?;
            let pv = witness.as_ref();
            let p1 = part1(&mut a, &mut t, &self.shape, &pv)?;
            let op = part2(&mut a, &mut t, &self.shape, &pv, &p1)?;
            let (u, q) = part3(&mut a, &mut t, &self.shape, &self.stack, &pv.map(|p| &p.stack), &p1.rs, &op)?;
            let commits = commit_cells(&mut a, &self.shape, &p1)?;
            part4(&mut a, &mut t, &self.whir, &pv.map(|p| &p.whir), &q, &commits, &u, self.shape.l_skip)?;
            cells.extend(p1.public_values.iter().flatten().cloned());
            let zero = a.cst(F::from(0u64))?;
            cells.resize(NPI, zero);
        }
        for c in &cells {
            std.constrain_as_public_input(l, c)?;
        }
        Ok(())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        wrap_arch(self.nr_cols)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        // The relation is rebuilt from the pinned `L_Fr` key, not from these bytes.
        w.write_all(b"sherlock-wrap-v1")
    }

    fn read_relation<R: io::Read>(_r: &mut R) -> io::Result<Self> {
        Err(io::Error::other("rebuild WrapRelation from the pinned L_Fr key"))
    }
}
