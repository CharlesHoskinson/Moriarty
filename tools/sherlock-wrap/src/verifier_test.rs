//! I3 test relation: `part1` on the gadget back end, with every sample asserted equal to the
//! recorded stock sample (witness cells), so the `MockProver` checks agreement.

use std::io;

use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    alg::{Alg, Gadget},
    native::F,
    smoke::wrap_arch,
    transcript::Transcript,
    stacked::{StackShape, part3},
    verifier::{ProofData, Shape, commit_cells, part1, part2},
    whir::{WhirShape, part4},
};

#[derive(Clone, Debug)]
pub struct Part1Relation {
    pub shape: Shape,
    pub n_samples: usize,
    /// I4: also run `part2` and take one more sample (the first stacked-reduction sample).
    pub part2: bool,
    /// I4b: also run the stacked reduction (implies `part2`, no extra sample).
    pub stack: Option<StackShape>,
    /// I5: also run WHIR after the stacked reduction (needs `stack`).
    pub whir: Option<WhirShape>,
}

impl Relation for Part1Relation {
    type Instance = ();
    type Witness = (ProofData, Vec<F>);
    type Error = Error;

    fn format_instance(_: &()) -> Result<Vec<F>, Error> {
        Ok(vec![])
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<()>,
        witness: Value<(ProofData, Vec<F>)>,
    ) -> Result<(), Error> {
        let mut a = Gadget::new(std, &mut *l);
        let mut t = Transcript::new(&mut a, self.shape.u_transcript)?;
        let pv = witness.as_ref().map(|w| &w.0);
        let p1 = part1(&mut a, &mut t, &self.shape, &pv)?;
        if let Some(ss) = &self.stack {
            let op = part2(&mut a, &mut t, &self.shape, &pv, &p1)?;
            let sv = pv.map(|p| &p.stack);
            let (u, q) = part3(&mut a, &mut t, &self.shape, ss, &sv, &p1.rs, &op)?;
            if let Some(ws) = &self.whir {
                let commits = commit_cells(&mut a, &self.shape, &p1)?;
                let wv = pv.map(|p| &p.whir);
                part4(&mut a, &mut t, ws, &wv, &q, &commits, &u, self.shape.l_skip)?;
            }
        } else if self.part2 {
            part2(&mut a, &mut t, &self.shape, &pv, &p1)?;
            t.sample(&mut a)?;
        }
        assert_eq!(t.samples.len(), self.n_samples);
        for (i, s) in t.samples.clone().iter().enumerate() {
            let r = a.wit(witness.as_ref().map(|w| w.1[i]))?;
            a.assert_eq(s, &r, "stock sample")?;
        }
        Ok(())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        wrap_arch(1)
    }

    fn write_relation<W: io::Write>(&self, _w: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn read_relation<R: io::Read>(_r: &mut R) -> io::Result<Self> {
        Err(io::Error::other("test relation"))
    }
}
