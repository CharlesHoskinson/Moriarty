//! I0 ledger smoke relation: the `ZkStdLibArch` of the wrap (`poseidon`, pow2range), the
//! bare permutation through the vendored patch, an `L_Fr` transcript (u = 7) and 1,026
//! public inputs. Public inputs: `w[0..NPI-1]` (observed in order) and the first sample.

use std::io;

use midnight_circuits::instructions::PublicInputInstructions;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    alg::{Alg, Gadget},
    native::{F, NativeTranscript, U_LFR_TRANSCRIPT},
    transcript::Transcript,
};

pub const NPI: usize = 1026;

pub fn wrap_arch(nr_cols: u8) -> ZkStdLibArch {
    ZkStdLibArch { poseidon: true, nr_pow2range_cols: nr_cols, ..ZkStdLibArch::default() }
}

#[derive(Clone, Debug)]
pub struct SmokeRelation {
    pub nr_cols: u8,
    /// PoW bits checked on the witness `pow` after the observes (0 = none).
    pub pow_bits: usize,
}

#[derive(Clone, Debug)]
pub struct SmokeWitness {
    pub w: Vec<F>,
    pub pow: F,
}

impl SmokeRelation {
    /// The public inputs of a witness (native transcript).
    pub fn instance(&self, wit: &SmokeWitness) -> Vec<F> {
        let mut t = NativeTranscript::new(U_LFR_TRANSCRIPT);
        for x in &wit.w {
            t.observe(*x);
        }
        let s = t.sample();
        let mut pi = wit.w.clone();
        pi.push(s);
        pi
    }

    /// A PoW witness for `w` (brute force, native).
    pub fn grind(&self, w: &[F]) -> F {
        let mut t = NativeTranscript::new(U_LFR_TRANSCRIPT);
        for x in w {
            t.observe(*x);
        }
        t.sample();
        (0u64..).map(F::from).find(|c| t.clone().check_witness(self.pow_bits, *c)).unwrap()
    }
}

impl Relation for SmokeRelation {
    type Instance = Vec<F>;
    type Witness = SmokeWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        witness: Value<SmokeWitness>,
    ) -> Result<(), Error> {
        let w = witness.as_ref().map(|w| w.w.clone()).transpose_vec(NPI - 1);
        let mut cells = Vec::with_capacity(NPI);
        {
            let mut a = Gadget::new(std, &mut *l);
            let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT)?;
            for v in w {
                let c = a.wit(v)?;
                t.observe(&mut a, &c)?;
                cells.push(c);
            }
            cells.push(t.sample(&mut a)?);
            if self.pow_bits > 0 {
                let pow = a.wit(witness.as_ref().map(|w| w.pow))?;
                t.check_witness(&mut a, self.pow_bits, &pow)?;
            }
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
        w.write_all(&[self.nr_cols, self.pow_bits as u8])
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut b = [0u8; 2];
        r.read_exact(&mut b)?;
        Ok(Self { nr_cols: b[0], pow_bits: b[1] as usize })
    }
}
