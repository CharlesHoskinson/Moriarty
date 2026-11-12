//! I2 test relation and fixture extraction: the WHIR Merkle openings of a `wrapin` proof,
//! each with the transcript sample that gives its query index.

use std::io;

use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    alg::{Alg, Gadget},
    merkle::{hash_slice, merkle_verify, tree_compress},
    native::{self, F},
    smoke::wrap_arch,
    wrapin::{Ev, WrapIn, digest, frs},
};

/// One query opening: `rows` (2^k_whir rows of `w` values), the path, the index sample, the root.
#[derive(Clone, Debug)]
pub struct Opening {
    pub rows: Vec<Vec<F>>,
    pub siblings: Vec<F>,
    pub sample: F,
    pub root: F,
}

fn native_digest(u_leaf: u128, u_node: u128, rows: &[Vec<F>]) -> F {
    let mut h: Vec<F> = rows.iter().map(|r| native::hash_slice(u_leaf, r)).collect();
    while h.len() > 1 {
        h = h.chunks_exact(2).map(|p| native::compress(u_node, p[0], p[1])).collect();
    }
    h[0]
}

fn native_walk(u_node: u128, mut cur: F, idx: u64, sib: &[F]) -> F {
    for (j, s) in sib.iter().enumerate() {
        cur = if (idx >> j) & 1 == 0 { native::compress(u_node, cur, *s) } else { native::compress(u_node, *s, cur) };
    }
    cur
}

/// All WHIR openings of the proof (round 0 for each commitment, then the codeword rounds).
/// The index sample of each opening is found among the recorded samples: the one whose low
/// bits walk the path to a known root (test-only search; the circuit takes it from the
/// transcript position).
pub fn openings(w: &WrapIn) -> Vec<Opening> {
    let (ul, un) = (w.u_leaf(), w.u_node());
    let p = w.proof();
    let samples: Vec<F> = w.log().into_iter().filter_map(|e| if let Ev::Sample(x) = e { Some(x) } else { None }).collect();
    let mut roots = vec![digest(&p["common_main_commit"])];
    for a in w.doc["vk"]["inner"]["per_air"].as_array().unwrap() {
        if !a["preprocessed_data"].is_null() {
            roots.push(digest(&a["preprocessed_data"]["commit"]));
        }
    }
    for t in p["trace_vdata"].as_array().unwrap() {
        if !t.is_null() {
            roots.extend(t["cached_commitments"].as_array().unwrap().iter().map(digest));
        }
    }
    let whir = &p["whir_proof"];
    let codeword_roots: Vec<F> = whir["codeword_commits"].as_array().unwrap().iter().map(digest).collect();
    let mut out = vec![];
    let mut find = |rows: Vec<Vec<F>>, siblings: Vec<F>, cands: &[F]| {
        let d = native_digest(ul, un, &rows);
        for s in &samples {
            let idx = native::low_u64(s) & ((1u64 << siblings.len()) - 1);
            let r = native_walk(un, d, idx, &siblings);
            if cands.contains(&r) {
                out.push(Opening { rows, siblings, sample: *s, root: r });
                return;
            }
        }
        panic!("no sample opens this path");
    };
    let rows0 = whir["initial_round_opened_rows"].as_array().unwrap();
    let mp0 = whir["initial_round_merkle_proofs"].as_array().unwrap();
    for (c, mp) in rows0.iter().zip(mp0) {
        for (q, path) in c.as_array().unwrap().iter().zip(mp.as_array().unwrap()) {
            let rows: Vec<Vec<F>> = q.as_array().unwrap().iter().map(frs).collect();
            let sib: Vec<F> = path.as_array().unwrap().iter().map(digest).collect();
            find(rows, sib, &roots);
        }
    }
    let vals = whir["codeword_opened_values"].as_array().unwrap();
    let mps = whir["codeword_merkle_proofs"].as_array().unwrap();
    for (r, (rv, rp)) in vals.iter().zip(mps).enumerate() {
        for (q, path) in rv.as_array().unwrap().iter().zip(rp.as_array().unwrap()) {
            let rows: Vec<Vec<F>> = frs(q).into_iter().map(|x| vec![x]).collect();
            let sib: Vec<F> = path.as_array().unwrap().iter().map(digest).collect();
            find(rows, sib, &codeword_roots[r..r + 1]);
        }
    }
    out
}

/// Shape of one opening: rows, width, depth.
#[derive(Clone, Debug)]
pub struct MerkleRelation {
    pub u_leaf: u128,
    pub u_node: u128,
    pub shapes: Vec<(usize, usize, usize)>,
}

impl MerkleRelation {
    pub fn new(w: &WrapIn, ops: &[Opening]) -> Self {
        Self {
            u_leaf: w.u_leaf(),
            u_node: w.u_node(),
            shapes: ops.iter().map(|o| (o.rows.len(), o.rows[0].len(), o.siblings.len())).collect(),
        }
    }

    /// The verifier lines on either back end.
    pub fn run<A: Alg>(&self, a: &mut A, ops: &Value<Vec<Opening>>) -> Result<(), Error> {
        for (i, &(nr, wd, depth)) in self.shapes.iter().enumerate() {
            let o = ops.as_ref().map(|v| v[i].clone());
            let mut leaves = vec![];
            for r in 0..nr {
                let vals = (0..wd)
                    .map(|c| a.wit(o.as_ref().map(|o| o.rows[r][c])))
                    .collect::<Result<Vec<_>, _>>()?;
                leaves.push(hash_slice(a, self.u_leaf, &vals)?);
            }
            let leaf = tree_compress(a, self.u_node, leaves)?;
            let sib = (0..depth)
                .map(|j| a.wit(o.as_ref().map(|o| o.siblings[j])))
                .collect::<Result<Vec<_>, _>>()?;
            let s = a.wit(o.as_ref().map(|o| o.sample))?;
            let bits = a.low_bits(&s, depth)?;
            let root = a.wit(o.as_ref().map(|o| o.root))?;
            merkle_verify(a, self.u_node, &root, &bits, &leaf, &sib)?;
        }
        Ok(())
    }
}

impl Relation for MerkleRelation {
    type Instance = ();
    type Witness = Vec<Opening>;
    type Error = Error;

    fn format_instance(_: &()) -> Result<Vec<F>, Error> {
        Ok(vec![])
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<()>,
        witness: Value<Vec<Opening>>,
    ) -> Result<(), Error> {
        let mut a = Gadget::new(std, &mut *l);
        self.run(&mut a, &witness)
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
