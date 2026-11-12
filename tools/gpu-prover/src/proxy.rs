// SPDX-License-Identifier: Apache-2.0
//! A k = 19 stand-in with the decider's constraint system. The run decider does not
//! fit k = 19 (929,170 rows), so the k = 19 row of the measurement uses this relation:
//! the same `ZkStdLibArch` as `RunDeciderRelation` (BLS12-381 and Poseidon chips,
//! 4 pow2range columns), hence the same gates, columns, lookups and degree, filled
//! with a chain of `iters` Poseidon hashes whose output is the one public input.
//! Prover cost is a function of (constraint system, domain); the witness content is
//! not the decider's. Not a wrap circuit and never keyed for the ledger.
use std::io;

use midnight_circuits::{
    instructions::{AssignmentInstructions, PublicInputInstructions},
    types::AssignedNative,
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};
use mori_recursion::{parent::poseidon, util::F};

#[derive(Clone, Debug)]
pub struct PoseidonChain {
    pub iters: u32,
}

impl PoseidonChain {
    pub fn output(&self, x0: F) -> F {
        (0..self.iters).fold(x0, |x, _| poseidon(&[x, x]))
    }
}

impl Relation for PoseidonChain {
    type Instance = Vec<F>;
    type Witness = F;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        witness: Value<F>,
    ) -> Result<(), Error> {
        let mut x: AssignedNative<F> = std.assign(l, witness)?;
        for _ in 0..self.iters {
            x = std.poseidon(l, &[x.clone(), x])?;
        }
        std.constrain_as_public_input(l, &x)
    }

    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            bls12_381: true,
            poseidon: true,
            nr_pow2range_cols: 4,
            ..ZkStdLibArch::default()
        }
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"GWPCHAIN")?;
        w.write_all(&self.iters.to_le_bytes())
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut m = [0u8; 8];
        r.read_exact(&mut m)?;
        if &m != b"GWPCHAIN" {
            return Err(io::Error::other("not a PoseidonChain relation"));
        }
        let mut b = [0u8; 4];
        r.read_exact(&mut b)?;
        Ok(PoseidonChain {
            iters: u32::from_le_bytes(b),
        })
    }
}
