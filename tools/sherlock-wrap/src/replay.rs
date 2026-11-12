//! I1 test relation: replay a recorded stock `L_Fr` transcript in circuit. Every observed
//! value is a private witness cell; every sample is computed by the in-circuit duplex and
//! asserted equal to the recorded sample (assigned as a witness), so the `MockProver`
//! checks that the circuit transcript equals the stock one.

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
    wrapin::Ev,
};

#[derive(Clone, Debug)]
pub struct ReplayRelation {
    /// Event kinds (the shape): 0 observe, 1 commit, 2 sample.
    pub kinds: Vec<u8>,
    pub u: u128,
}

impl ReplayRelation {
    pub fn new(log: &[Ev], u: u128) -> Self {
        let kinds = log
            .iter()
            .map(|e| match e {
                Ev::Obs(_) => 0,
                Ev::Commit(_) => 1,
                Ev::Sample(_) => 2,
            })
            .collect();
        Self { kinds, u }
    }
}

impl Relation for ReplayRelation {
    type Instance = ();
    type Witness = Vec<F>;
    type Error = Error;

    fn format_instance(_: &()) -> Result<Vec<F>, Error> {
        Ok(vec![])
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<()>,
        witness: Value<Vec<F>>,
    ) -> Result<(), Error> {
        let vals = witness.transpose_vec(self.kinds.len());
        let mut a = Gadget::new(std, &mut *l);
        let mut t = Transcript::new(&mut a, self.u)?;
        for (k, v) in self.kinds.iter().zip(vals) {
            let c = a.wit(v)?;
            if *k == 2 {
                let s = t.sample(&mut a)?;
                a.assert_eq(&s, &c, "sample")?;
            } else {
                t.observe(&mut a, &c)?;
            }
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

/// The witness of a log (the recorded values in order).
pub fn log_values(log: &[Ev]) -> Vec<F> {
    log.iter()
        .map(|e| match e {
            Ev::Obs(x) | Ev::Commit(x) | Ev::Sample(x) => *x,
        })
        .collect()
}
