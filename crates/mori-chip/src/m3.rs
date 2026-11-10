//! M3 (experiment E8): the leaf's aligned memory segment, its update of
//! `mem_root`, and the Poseidon state digest.
//!
//! Memory is a depth-20 tree of cells. A leaf owns the aligned segment of
//! [`SEG`] = 8 cells at `base .. base + 8` (`base` a multiple of 8). The segment
//! is a static depth-3 subtree: its root hashes all 8 MEM rows, padding rows
//! included. Its root is a leaf of the depth-17 top tree, at index `base / 8`.
//!
//! - Record of MEM row `i`: `H(act, a, b, c)` (2 permutations), its inputs copied
//!   from the MEM row. `act` is the tag word (1 on written records).
//! - Segment root: 7 compressions over the 8 records.
//! - Top tree: an empty slot is 0, an empty node of level `l + 1` is
//!   `H(E_l, E_l)`. The old walk starts at 0 and the new walk at the segment root;
//!   both use the same sibling and bit cells (copies), so the old walk proves
//!   slot `base / 8` was empty in `pre.mem_root` and the new walk gives
//!   `post.mem_root` with only that slot changed (17 + 17 compressions).
//! - The walker gate sits on the absorb row of each top-tree compression. Codes of
//!   one fixed column ([`crate::vsel`]): 1 every level, 2 the old walk's level 0,
//!   3 the old walk's level 16.
//!   - `m3 walk` (every code): `b` boolean, `(in0, in1) = b ? (s, x) : (x, s)`,
//!     `acc_in = 2 acc_out + b`.
//!   - `m3 walk first` (code 2): `x = 0` (the empty slot), `8 acc_in = base`,
//!     `len_post = base + 8`.
//!   - `m3 walk last` (code 3): `acc_out = 0`.
//!
//!   Level `l + 1` copies `acc_out` of level `l` into its `acc_in`, so
//!   `base / 8 = sum b_l 2^l < 2^17`: `base` is aligned and in range (verdict C3),
//!   and the bits that steer both walks are the bits of `base / 8`.
//! - State digest: `H(w_0, .., w_28, mem_root)`, 30 inputs (15 permutations).
//!   `pre_d` takes slot 0's register cells and the old root. `post_d` takes slot
//!   S's register cells, except LEN, which is the close cell `len_post = base + 8`
//!   of the walker (verdict C4), and the new root.
//!
//! Columns of a walker row: 2 `in0`, 3 `in1` (the sponge's), 5 `x`, 6 `s`,
//! 7 `b`, 8 `acc_in`, 9 `acc_out`, 10 `base`, 11 `len_post` (first row only).
use ff::Field;
use midnight_proofs::{
    circuit::{AssignedCell, Region, Value},
    plonk::{Column, ConstraintSystem, Error, Expression, Fixed},
    poly::Rotation,
};

use crate::{
    Cols, F,
    mem::{SEG, SegCells},
    poseidon::{HIn, PoseidonCfg, Tamper, sponge_native},
    vsel::{VSel, code_column},
};

/// Depth of the top tree (20 - 3).
pub const TOP: usize = 17;
/// Rows of the M3 hash work of one leaf: 8 records (2 permutations each), 7 + 34
/// compressions, 2 digests of 30 inputs (15 permutations each).
pub const PERMS: usize = 8 * 2 + 7 + 2 * TOP + 2 * 15;

pub fn h2(x: F, y: F) -> F {
    sponge_native(&[x, y])
}

pub fn record_native(act: F, c: [F; 3]) -> F {
    sponge_native(&[act, c[0], c[1], c[2]])
}

pub fn seg_root_native(recs: &[F]) -> F {
    let mut v = recs.to_vec();
    while v.len() > 1 {
        v = v.chunks(2).map(|p| h2(p[0], p[1])).collect();
    }
    v[0]
}

/// Empty top-tree node of level `l` (0: an empty segment slot).
pub fn empty_top(l: usize) -> F {
    (0..l).fold(F::ZERO, |e, _| h2(e, e))
}

/// Root over `x` at index `idx` with siblings `sibs` (bottom up).
pub fn walk_native(x: F, sibs: &[F], idx: u64) -> F {
    sibs.iter().enumerate().fold(x, |x, (l, &s)| {
        if (idx >> l) & 1 == 1 {
            h2(s, x)
        } else {
            h2(x, s)
        }
    })
}

/// The state digest: the 29 digested register words, then `mem_root`.
pub fn state_digest(words: &[F], mem_root: F) -> F {
    let mut v = words.to_vec();
    v.push(mem_root);
    sponge_native(&v)
}

#[derive(Clone, Debug)]
pub struct WalkConfig {
    pub col: Column<Fixed>,
    /// Codes 1 (every level), 2 (old walk level 0), 3 (old walk level 16).
    pub q: [VSel; 3],
    pub cols: Cols,
}

impl WalkConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols) -> Self {
        let qv = code_column(meta, 3);
        let q = [qv[0], qv[1], qv[2]];
        let col = match q[0] {
            VSel::Code { col, .. } => col,
            VSel::Simple(_) => unreachable!(),
        };
        let a = cols.a;
        let k = |v: u64| Expression::Constant(F::from(v));
        meta.create_gate("m3 walk", |m| {
            let any = m.query_fixed(col, Rotation::cur());
            let mut c = |i: usize| m.query_advice(a[i], Rotation::cur());
            let (in0, in1, x, s, b, ai, ao) = (c(2), c(3), c(5), c(6), c(7), c(8), c(9));
            crate::gate(vec![
                any.clone() * b.clone() * (k(1) - b.clone()),
                any.clone() * (in0 - x.clone() - b.clone() * (s.clone() - x.clone())),
                any.clone() * (in1 - s.clone() - b.clone() * (x - s)),
                any * (ai - ao * k(2) - b),
            ])
        });
        let (q2, q3) = (q[1], q[2]);
        meta.create_gate("m3 walk first", |m| {
            let s = q2.expr(m);
            let mut c = |i: usize| m.query_advice(a[i], Rotation::cur());
            let (x, ai, base, lp) = (c(5), c(8), c(10), c(11));
            crate::gate(vec![
                s.clone() * x,
                s.clone() * (ai * k(8) - base.clone()),
                s * (lp - base - k(8)),
            ])
        });
        meta.create_gate("m3 walk last", |m| {
            let s = q3.expr(m);
            let ao = m.query_advice(a[9], Rotation::cur());
            crate::gate(vec![s * ao])
        });
        for i in [2, 3, 5, 6, 7, 8, 9, 10, 11] {
            meta.enable_equality(a[i]);
        }
        Self { col, q, cols }
    }
}

/// Witness of the M3 part of a leaf and its negative-test knobs.
#[derive(Clone, Debug)]
pub struct M3W {
    /// The 8 MEM rows `(a, b, c)` and their active flags.
    pub cells: [[F; 3]; SEG],
    pub act: [F; SEG],
    /// The core's running `len` after the last slot, as witnessed on MEM rows.
    pub len_end: F,
    /// Siblings of the old walk, and of the new walk (equal when honest).
    pub sibs: [F; TOP],
    pub sibs_new: [F; TOP],
    /// The segment index whose bits steer the walks (`base / 8` when honest).
    pub idx: u64,
    /// `mem_root` of the statement's `pre_d`.
    pub pre_root: F,
    /// The close cell `len_post` (`base + 8` when honest).
    pub len_post: F,
    /// `(digest, input, delta)`: digest 0 (`pre_d`) or 1 (`post_d`) hashes input
    /// `input` as its cell plus `delta` (the copy constraint stays).
    pub dig_in: Option<(usize, usize, F)>,
    /// Negative tests: the walk bits as field elements (the bits of `idx` when
    /// `None`); the accumulators follow `acc_out = (acc_in - b) / 2`.
    pub bits: Option<[F; TOP]>,
    /// Negative tests: `(walk, level, delta)` adds `delta` to `in0` of that
    /// compression (walk 0 old, 1 new); the hash is of the forged input.
    pub forge_in0: Option<(usize, usize, F)>,
}

/// A walk with field bits and an optional forged `in0` at one level.
pub fn walk_field(x: F, sibs: &[F], bits: &[F], forge: Option<(usize, F)>) -> F {
    sibs.iter().enumerate().fold(x, |x, (l, &s)| {
        let b = bits[l];
        let mut in0 = x + b * (s - x);
        if let Some((fl, d)) = forge {
            if fl == l {
                in0 += d;
            }
        }
        h2(in0, s + b * (x - s))
    })
}

impl M3W {
    /// An honest M3 witness for a leaf at `base` (a multiple of 8) whose appended
    /// cells are `appended`, with old-tree siblings `sibs`.
    pub fn honest(base: u128, len_end: u128, appended: &[[F; 3]], sibs: [F; TOP]) -> Self {
        assert!(appended.len() <= SEG, "a leaf appends at most 8 cells");
        let mut cells = [[F::ZERO; 3]; SEG];
        let mut act = [F::ZERO; SEG];
        for (i, c) in appended.iter().enumerate() {
            cells[i] = *c;
            act[i] = F::ONE;
        }
        let idx = (base / SEG as u128) as u64;
        Self {
            cells,
            act,
            len_end: crate::fu(len_end),
            sibs,
            sibs_new: sibs,
            idx,
            pre_root: walk_native(F::ZERO, &sibs, idx),
            len_post: crate::fu(base + SEG as u128),
            dig_in: None,
            bits: None,
            forge_in0: None,
        }
    }

    /// The walk bits.
    pub fn bit_vals(&self) -> [F; TOP] {
        self.bits
            .unwrap_or_else(|| std::array::from_fn(|l| F::from((self.idx >> l) & 1)))
    }

    /// `acc_in` of each level: `idx`, then `(acc_in - b) / 2`.
    pub fn accs(&self) -> [F; TOP + 1] {
        let b = self.bit_vals();
        let half = F::from(2u64).invert().unwrap();
        let mut a = [F::ZERO; TOP + 1];
        a[0] = F::from(self.idx);
        for l in 0..TOP {
            a[l + 1] = (a[l] - b[l]) * half;
        }
        a
    }

    fn forge(&self, walk: usize) -> Option<(usize, F)> {
        self.forge_in0
            .and_then(|(wk, l, d)| (wk == walk).then_some((l, d)))
    }

    /// `mem_root` before the leaf, as the circuit's old walk computes it.
    pub fn old_root(&self) -> F {
        walk_field(F::ZERO, &self.sibs, &self.bit_vals(), self.forge(0))
    }

    pub fn seg_root(&self) -> F {
        let recs: Vec<F> = (0..SEG)
            .map(|i| record_native(self.act[i], self.cells[i]))
            .collect();
        seg_root_native(&recs)
    }

    /// `mem_root` after the leaf, as the circuit computes it.
    pub fn new_root(&self) -> F {
        walk_field(
            self.seg_root(),
            &self.sibs_new,
            &self.bit_vals(),
            self.forge(1),
        )
    }

    /// The digest inputs: `words` then the root; `side` 0 is `pre_d`, 1 `post_d`.
    pub fn digest(&self, side: usize, words: &[F], root: F) -> F {
        let mut v = words.to_vec();
        v.push(root);
        if let Some((sd, i, d)) = self.dig_in {
            if sd == side {
                v[i] += d;
            }
        }
        sponge_native(&v)
    }
}

/// Cells of the digests' inputs and the roots, for [`assign`].
pub struct M3Out {
    pub pre_d: AssignedCell<F, F>,
    pub post_d: AssignedCell<F, F>,
}

fn known(v: Option<F>) -> Value<F> {
    v.map_or(Value::unknown(), Value::known)
}

/// Assign the M3 hash work from row `*o` of `region`: records of `seg`, the
/// segment root, the two walks and the two digests. `pre_words` / `post_words`
/// are the register cells of slot 0 and slot S (29 each; `post_words[len_idx]` is
/// replaced by the close cell), `base` is slot 0's LEN cell.
#[allow(clippy::too_many_arguments)]
pub fn assign(
    pc: &PoseidonCfg,
    wk: &WalkConfig,
    region: &mut Region<'_, F>,
    o: &mut usize,
    seg: &SegCells,
    base: &AssignedCell<F, F>,
    pre_words: &[AssignedCell<F, F>],
    post_words: &[AssignedCell<F, F>],
    len_idx: usize,
    w: Option<&M3W>,
) -> Result<M3Out, Error> {
    let tm = Tamper::default();
    let mut perm = 0;
    let a = wk.cols.a;
    // Records.
    let mut level: Vec<AssignedCell<F, F>> = Vec::new();
    for r in &seg.rec {
        let ins: Vec<HIn> = r.iter().map(|c| HIn::Cell(c.clone())).collect();
        let (out, n) = pc.sponge(region, *o, &ins, false, &tm, &mut perm)?;
        *o += n;
        level.push(out);
    }
    // Segment root.
    while level.len() > 1 {
        let mut next = Vec::new();
        for p in level.chunks(2) {
            let ins = [HIn::Cell(p[0].clone()), HIn::Cell(p[1].clone())];
            let (out, n) = pc.sponge(region, *o, &ins, false, &tm, &mut perm)?;
            *o += n;
            next.push(out);
        }
        level = next;
    }
    let seg_root = level.pop().unwrap();

    // Walks. Old walk first; the new walk copies its s, b, acc cells.
    type Row = [AssignedCell<F, F>; 4]; // s, b, acc_in, acc_out
    let mut old_rows: Vec<Row> = Vec::new();
    let mut len_post = None;
    let mut roots = Vec::new();
    for walk in 0..2 {
        let mut x: Option<AssignedCell<F, F>> = None;
        for l in 0..TOP {
            let r = *o;
            let bit = w.map(|w| w.bit_vals()[l]);
            let s_val = w.map(|w| if walk == 0 { w.sibs[l] } else { w.sibs_new[l] });
            // x
            let xc = match (&x, walk) {
                (None, 0) => region.assign_advice(|| "x", a[5], r, || known(w.map(|_| F::ZERO)))?,
                (None, _) => seg_root.copy_advice(|| "x", region, a[5], r)?,
                (Some(c), _) => c.copy_advice(|| "x", region, a[5], r)?,
            };
            let (sc, bc, ai, ao) = if walk == 0 {
                let sc = region.assign_advice(|| "s", a[6], r, || known(s_val))?;
                let bc = region.assign_advice(|| "b", a[7], r, || known(bit))?;
                let ai = match old_rows.last() {
                    None => region.assign_advice(
                        || "acc_in",
                        a[8],
                        r,
                        || known(w.map(|w| F::from(w.idx))),
                    )?,
                    Some(p) => p[3].copy_advice(|| "acc_in", region, a[8], r)?,
                };
                let ao = region.assign_advice(
                    || "acc_out",
                    a[9],
                    r,
                    || known(w.map(|w| w.accs()[l + 1])),
                )?;
                (sc, bc, ai, ao)
            } else {
                let p = &old_rows[l];
                let sc = p[0].copy_advice(|| "s", region, a[6], r)?;
                if let Some(v) = s_val {
                    // The new walk's sibling as witnessed (forged in a negative).
                    region.assign_advice(|| "s", a[6], r, || Value::known(v))?;
                }
                (
                    sc,
                    p[1].copy_advice(|| "b", region, a[7], r)?,
                    p[2].copy_advice(|| "acc_in", region, a[8], r)?,
                    p[3].copy_advice(|| "acc_out", region, a[9], r)?,
                )
            };
            let code = if walk == 0 && l == 0 {
                1
            } else if walk == 0 && l == TOP - 1 {
                2
            } else {
                0
            };
            wk.q[code].enable(region, r)?;
            if code == 1 {
                base.copy_advice(|| "base", region, a[10], r)?;
                len_post = Some(region.assign_advice(
                    || "len_post",
                    a[11],
                    r,
                    || known(w.map(|w| w.len_post)),
                )?);
            }
            let xv = xc.value().copied();
            let sv = known(s_val);
            let bv = known(bit);
            let fd = w
                .and_then(|w| w.forge_in0)
                .and_then(|(wk, fl, d)| (wk == walk && fl == l).then_some(d))
                .unwrap_or(F::ZERO);
            let in0 = xv.zip(sv).zip(bv).map(|((x, s), b)| x + b * (s - x) + fd);
            let in1 = xv.zip(sv).zip(bv).map(|((x, s), b)| s + b * (x - s));
            let (out, n) = pc.sponge(
                region,
                r,
                &[HIn::Val(in0), HIn::Val(in1)],
                false,
                &tm,
                &mut perm,
            )?;
            *o += n;
            if walk == 0 {
                old_rows.push([sc, bc, ai, ao]);
            }
            x = Some(out);
        }
        roots.push(x.unwrap());
    }
    let len_post = len_post.unwrap();

    // Digests.
    let mut dig = Vec::new();
    for side in 0..2 {
        let words = if side == 0 { pre_words } else { post_words };
        let mut cells: Vec<AssignedCell<F, F>> = words.to_vec();
        if side == 1 {
            cells[len_idx] = len_post.clone();
        }
        cells.push(roots[side].clone());
        let ins: Vec<HIn> = cells
            .into_iter()
            .enumerate()
            .map(|(i, c)| match w.and_then(|w| w.dig_in) {
                Some((sd, j, d)) if sd == side && j == i => {
                    let v = c.value().map(|v| *v + d);
                    HIn::CellAs(c, v)
                }
                _ => HIn::Cell(c),
            })
            .collect();
        let (out, n) = pc.sponge(region, *o, &ins, false, &tm, &mut perm)?;
        *o += n;
        dig.push(out);
    }
    debug_assert_eq!(perm, PERMS);
    Ok(M3Out {
        pre_d: dig[0].clone(),
        post_d: dig[1].clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_native_matches_a_dense_tree() {
        // Depth-3 top tree with leaves v_i; the walk from leaf 5 with its
        // siblings gives the dense root.
        let v: Vec<F> = (0..8).map(|i| F::from(i as u64 + 11)).collect();
        let l1: Vec<F> = v.chunks(2).map(|p| h2(p[0], p[1])).collect();
        let l2: Vec<F> = l1.chunks(2).map(|p| h2(p[0], p[1])).collect();
        let root = h2(l2[0], l2[1]);
        let sibs = [v[4], l1[3], l2[0]];
        assert_eq!(walk_native(v[5], &sibs, 5), root);
        assert_eq!(empty_top(1), h2(F::ZERO, F::ZERO));
    }
}
