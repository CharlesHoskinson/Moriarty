//! Recursive verifier parent for a v3 batch child. The child key is circuit-fixed.
//! It carries the collapsed child accumulator into the existing Sherlock decider.
use crate::{
    bls_arch,
    inner::{acc_pis, children_acc_offcircuit, verify_child_incircuit, vk_name},
    util::{F, read_bytes, vk_bytes, vk_from_bytes, write_bytes},
};
use midnight_circuits::{instructions::*, types::AssignedNative};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch};
use std::io;

pub const STATEMENT_FIELDS: usize = 17;

#[derive(Clone, Debug)]
pub struct RollupParent {
    pub child_vk: MidnightVK,
}

#[derive(Clone, Debug)]
pub struct ParentWitness {
    pub instance: Vec<F>,
    pub proof: Vec<u8>,
}

impl RollupParent {
    fn name(&self) -> String {
        vk_name("fsyn", &self.child_vk)
    }
    pub fn instance(&self, w: &ParentWitness) -> Result<Vec<F>, String> {
        if w.instance.len() != STATEMENT_FIELDS {
            return Err("batch child must have 17 statement lanes".into());
        }
        let acc = children_acc_offcircuit(
            &self.child_vk,
            &self.name(),
            std::slice::from_ref(&w.instance),
            std::slice::from_ref(&w.proof),
        )?;
        Ok([w.instance.clone(), acc_pis(&acc)].concat())
    }
}

impl Relation for RollupParent {
    type Instance = Vec<F>;
    type Witness = ParentWitness;
    type Error = Error;
    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(4)
    }
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _: Value<Vec<F>>,
        w: Value<ParentWitness>,
    ) -> Result<(), Error> {
        w.error_if_known_and(|w| {
            w.instance.len() != STATEMENT_FIELDS || w.proof.is_empty() || w.proof.len() > 262144
        })?;
        let cells: Vec<AssignedNative<F>> = s.assign_many(
            l,
            &w.as_ref()
                .map(|w| w.instance.clone())
                .transpose_vec(STATEMENT_FIELDS),
        )?;
        let acc = verify_child_incircuit(
            s,
            l,
            &self.child_vk,
            &self.name(),
            &cells,
            w.map(|w| w.proof),
            false,
        )?;
        for c in &cells {
            s.constrain_as_public_input(l, c)?;
        }
        s.verifier().constrain_as_public_input(l, &acc)
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"FSYNP001")?;
        write_bytes(w, &vk_bytes(&self.child_vk))
    }
    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0; 8];
        r.read_exact(&mut magic)?;
        if &magic != b"FSYNP001" {
            return Err(io::Error::other("invalid rollup parent"));
        }
        let child_vk = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        let mut rest = Vec::new();
        r.read_to_end(&mut rest)?;
        if !rest.is_empty() {
            return Err(io::Error::other("trailing parent relation bytes"));
        }
        Ok(Self { child_vk })
    }
}
