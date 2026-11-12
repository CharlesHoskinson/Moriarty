//! The stock overwrite duplex (`DuplexSponge<Fr, P, 3, 2>` from `(0, 0, cap_var(u))`) on
//! either back end. The indices are host values: the op order is fixed by the pinned shape.

use midnight_proofs::plonk::Error;

use crate::{
    alg::Alg,
    native::{RATE, WIDTH, cap_var},
};

#[derive(Clone, Debug)]
pub struct Transcript<V> {
    pub state: [V; WIDTH],
    pub absorb_idx: usize,
    pub sample_idx: usize,
}

impl<V: Clone> Transcript<V> {
    pub fn new<A: Alg<V = V>>(a: &mut A, u: u128) -> Result<Self, Error> {
        let z = a.cst(<crate::native::F as ff::Field>::ZERO)?;
        let c = a.cst(cap_var(u))?;
        Ok(Self { state: [z.clone(), z, c], absorb_idx: 0, sample_idx: 0 })
    }

    fn perm<A: Alg<V = V>>(&mut self, a: &mut A) -> Result<(), Error> {
        self.state = a.permute(&self.state)?;
        self.absorb_idx = 0;
        self.sample_idx = RATE;
        Ok(())
    }

    pub fn observe<A: Alg<V = V>>(&mut self, a: &mut A, x: &V) -> Result<(), Error> {
        self.state[self.absorb_idx] = x.clone();
        self.absorb_idx += 1;
        if self.absorb_idx == RATE {
            self.perm(a)?;
        }
        Ok(())
    }

    pub fn observe_all<A: Alg<V = V>>(&mut self, a: &mut A, xs: &[V]) -> Result<(), Error> {
        xs.iter().try_for_each(|x| self.observe(a, x))
    }

    pub fn sample<A: Alg<V = V>>(&mut self, a: &mut A) -> Result<V, Error> {
        if self.absorb_idx != 0 || self.sample_idx == 0 {
            self.perm(a)?;
        }
        self.sample_idx -= 1;
        Ok(self.state[self.sample_idx].clone())
    }

    /// `sample_bits`: the low `bits` bits (little-endian) of the canonical representative of
    /// the next sample, with the sample cell. Canonical, so the `x + r` alias is refused.
    pub fn sample_bits<A: Alg<V = V>>(
        &mut self,
        a: &mut A,
        bits: usize,
    ) -> Result<(V, Vec<A::B>), Error> {
        assert!(bits < 32);
        let x = self.sample(a)?;
        let b = a.low_bits(&x, bits)?;
        Ok((x, b))
    }

    /// `check_witness`: observe `w`, then the next `bits` sampled bits are all zero.
    pub fn check_witness<A: Alg<V = V>>(&mut self, a: &mut A, bits: usize, w: &V) -> Result<(), Error> {
        if bits == 0 {
            return Ok(());
        }
        self.observe(a, w)?;
        let (_, b) = self.sample_bits(a, bits)?;
        for x in &b {
            a.assert_bit_false(x, "pow")?;
        }
        Ok(())
    }
}
