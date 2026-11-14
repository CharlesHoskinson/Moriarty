//! I6: the wrap relation. The whole `L_Fr` verifier (`part1` to `part4`) on the gadget back
//! end; the statement is 1,026 words. The `L_Fr` shape (params, presence, heights, DAGs,
//! layouts, WHIR rounds, preprocessed commitments, vk pre-hash) is pinned in the circuit as
//! constants, so it is fixed by the key.
//!
//! The statement map. A fixture with a `decode` section (the E3 `L_Fr`, openvm e3-iface
//! 2f09caf) uses the E3 map: the D-9 decoder (`decode.rs`) on the `L_Fr` public values, after
//! `part4` on the same transcript; the statement is the call vector (1,026 lanes). Other
//! fixtures keep `PublicMap` (LEAD, PLAN section 4): the first `n` words are the `L_Fr` public
//! values in AIR order, the other `1,026 - n` words are constrained zero.

use std::io;

use midnight_circuits::instructions::PublicInputInstructions;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    alg::{Alg, Gadget, Native},
    decode::{DecodeShape, decode_with},
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
    /// The E3 map (`None`: `PublicMap`).
    pub map: Option<DecodeShape>,
}

impl WrapRelation {
    pub fn from_wrapin(w: &WrapIn, nr_cols: u8) -> Self {
        let shape = Shape::from_wrapin(w);
        let stack = StackShape::from_wrapin(w, &shape);
        let whir = WhirShape::from_wrapin(w, &stack.widths);
        let n: usize = shape.airs.iter().map(|a| if a.log_height.is_some() { a.num_public_values } else { 0 }).sum();
        let map = w.doc.get("decode").filter(|d| !d.is_null()).map(|dd| {
            let p = ProofData::from_wrapin(w);
            let sh = DecodeShape::from_doc(dd, p.public_values.iter().flatten().copied().next().unwrap());
            assert_eq!(sh.d.lanes(), NPI, "E3 map: the call vector is the statement");
            assert_eq!(n, sh.d.n_public(), "E3 map: L_Fr public values");
            sh
        });
        if map.is_none() {
            assert!(n <= NPI, "PublicMap: {n} public values exceed {NPI} words");
        }
        WrapRelation { shape, stack, whir, nr_cols, map }
    }

    /// The statement of a proof: the native decoder (E3 map) or `PublicMap`.
    pub fn instance(&self, p: &ProofData) -> Vec<F> {
        let mut v: Vec<F> = p.public_values.iter().flatten().copied().collect();
        if let Some(sh) = &self.map {
            let mut a = Native::default();
            let mut t = Transcript::new(&mut a, self.shape.u_transcript).unwrap();
            let pv: Vec<Value<F>> = v.iter().map(|x| Value::known(*x)).collect();
            t.observe_all(&mut a, &pv).unwrap();
            let s = decode_with(&mut a, &mut t, sh, &pv, &Value::known(p.path_sibs.clone())).unwrap();
            assert!(a.failures.is_empty(), "decoder refused: {:?}", a.failures);
            let mut out = vec![];
            for x in s {
                x.map(|x| out.push(x));
            }
            return out;
        }
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
            let pw: Vec<_> = p1.public_values.iter().flatten().cloned().collect();
            if let Some(sh) = &self.map {
                let sibs = witness.as_ref().map(|p| p.path_sibs.clone());
                cells = decode_with(&mut a, &mut t, sh, &pw, &sibs)?;
            } else {
                cells.extend(pw);
                let zero = a.cst(F::from(0u64))?;
                cells.resize(NPI, zero);
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
        // The relation is rebuilt from the pinned `L_Fr` key, not from these bytes.
        w.write_all(b"sherlock-wrap-v1")
    }

    fn read_relation<R: io::Read>(_r: &mut R) -> io::Result<Self> {
        Err(io::Error::other("rebuild WrapRelation from the pinned L_Fr key"))
    }
}
