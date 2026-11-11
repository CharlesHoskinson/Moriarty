//! Dense tail-buffered Merkle memory backend (experiment C1; design of
//! `research/sherlock-runlength-consensus-20261111/CONSENSUS.md` 1.2.3 item 2
//! with the invariants of `AUDIT.md` finding 8).
//!
//! State of one arena: `(len, root, tail_d)`. `len = 8 q + r` (`q < 2^TOP`,
//! `r < 8`); `root` is a fixed-depth binary tree of `2^TOP` segment slots holding
//! exactly the completed segments `0 .. q`; `tail_d` commits the `r` records of
//! segment `q`, padded canonically.
//!
//! Hashes (every one domain-tagged in the sponge capacity, `poseidon::sponge_iv`;
//! tags are fixed by the circuit, so no permutation is spent on them):
//! - record `H_REC(kind, a, b, c)`, 4 inputs, 2 permutations; `kind` in `1..=4`;
//! - pad record `PAD = H_PAD(0, 0)` (a constant), for every slot `>= r` of a
//!   tail, so `tail_d` is canonical (AUDIT finding 8, first rule);
//! - node of height `h` `H_NODE_h(x, y)`, one permutation; heights 1..3 inside a
//!   segment, `4 .. TOP + 3` in the top tree, so a node is bound to its height;
//! - empty segment slot `EMPTY_SEG = H_EMPTY(0, 0) != 0` (a constant), empty node
//!   of height `h + 1` = `H_NODE_(h+1)(E_h, E_h)`.
//!
//! Per leaf ([`DenseCfg::persist`], `W` flush units):
//! - every leaf opens the tail: `Tree8(imp ? rec_i : PAD)` over window rows
//!   0..7 equals `pre.tail_d` (so a leaf that neither reads nor extends the tail
//!   passes it through: the post tail is recomputed from the same cells);
//! - `T_j = Tree8(act ? rec_i : PAD)` over window segment `j`;
//! - flush unit `j` is on iff `f_j = act` of row `8 j + 7`, i.e. iff
//!   `floor(len_post / 8) > q_pre + j`. Its old walk starts at `EMPTY_SEG` and
//!   must reach the running root when `f_j = 1`; its new walk from `T_j` uses
//!   the same siblings and bits and gives the next root. With `f_j = 0` the root
//!   is unchanged (`root' = root + f (new - root)`). The walk index is
//!   `idx_0 = q_pre`, `idx_j = idx_(j-1) + f_j`, decomposed into TOP bits by the
//!   walk's accumulator;
//! - the post tail is `T_k` with `k` the number of flushes (a mux chain on `f`).
//!
//! Cold read ([`DenseCfg::cold_read`]): the record hash, then `TOP + 3` levels
//! whose bits are the bits of `addr`, against `pre.root`, gated by `enable`.
use std::{collections::HashMap, sync::OnceLock};

use ff::Field;
use midnight_proofs::{
    circuit::{Region, Value},
    plonk::{ConstraintSystem, Error, Expression, VirtualCells},
    poly::Rotation,
};

use crate::{
    Cols, F,
    membackend::{
        Cell, ColdRead, MemBackend, Names, SEGC, Src, WinRowW, WinW, Window, bit_accs, put,
    },
    poseidon::{HIn, PoseidonCfg, Tamper, sponge_native_iv},
    vsel::{VSel, pack},
};

pub const DOM_REC: u64 = 1;
pub const DOM_PAD: u64 = 2;
pub const DOM_EMPTY: u64 = 3;
pub const DOM_NODE: u64 = 4;

/// Capacity tag of domain `dom`, sub-tag `sub` (a height for nodes).
pub fn iv(dom: u64, sub: u64) -> F {
    F::from(((dom << 16) | sub) << 32)
}

/// A record: `[kind, a, b, c]`.
pub type Rec = [F; 4];

pub fn rec_d(r: &Rec) -> F {
    sponge_native_iv(r, iv(DOM_REC, 0))
}

pub fn pad_d() -> F {
    static V: OnceLock<F> = OnceLock::new();
    *V.get_or_init(|| sponge_native_iv(&[F::ZERO, F::ZERO], iv(DOM_PAD, 0)))
}

pub fn empty_seg() -> F {
    static V: OnceLock<F> = OnceLock::new();
    *V.get_or_init(|| sponge_native_iv(&[F::ZERO, F::ZERO], iv(DOM_EMPTY, 0)))
}

pub fn node(h: usize, x: F, y: F) -> F {
    sponge_native_iv(&[x, y], iv(DOM_NODE, h as u64))
}

/// Empty node of height `h >= 3` (3: an empty segment slot).
pub fn empty_at(h: usize) -> F {
    static V: OnceLock<Vec<F>> = OnceLock::new();
    let v = V.get_or_init(|| {
        let mut v = vec![F::ZERO, F::ZERO, F::ZERO, empty_seg()];
        for hh in 4..=80 {
            let e = v[hh - 1];
            v.push(node(hh, e, e));
        }
        v
    });
    assert!(h >= 3);
    v[h]
}

/// Root of 8 leaves (heights 1..3).
pub fn tree8(l: &[F]) -> F {
    assert_eq!(l.len(), SEGC);
    let mut v = l.to_vec();
    let mut h = 1;
    while v.len() > 1 {
        v = v.chunks(2).map(|p| node(h, p[0], p[1])).collect();
        h += 1;
    }
    v[0]
}

/// Canonical tail commitment of `recs` (`recs.len() < 8`).
pub fn tail_d(recs: &[Rec]) -> F {
    assert!(recs.len() < SEGC);
    let l: Vec<F> = (0..SEGC)
        .map(|i| recs.get(i).map_or(pad_d(), rec_d))
        .collect();
    tree8(&l)
}

/// The boundary state of one arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pub len: F,
    pub root: F,
    pub tail: F,
}

/// Native model of one dense arena with a sparse top tree.
#[derive(Clone, Debug)]
pub struct DenseModel {
    pub top: usize,
    pub len: u128,
    pub tail: Vec<Rec>,
    pub segs: HashMap<u128, [Rec; SEGC]>,
    nodes: HashMap<(usize, u128), F>,
}

/// Auxiliary witness of one flush unit.
#[derive(Clone, Debug)]
pub struct UnitW {
    pub sibs: Vec<F>,
    /// Negative tests: start the old walk here instead of `EMPTY_SEG`.
    pub x0: Option<F>,
}

/// Auxiliary witness of one leaf of the dense backend.
#[derive(Clone, Debug)]
pub struct DenseLeafW {
    pub top: usize,
    pub units: Vec<UnitW>,
}

/// Auxiliary witness of one cold read: the `TOP + 3` siblings.
#[derive(Clone, Debug)]
pub struct DenseReadW {
    pub top: usize,
    pub sibs: Vec<F>,
}

impl DenseModel {
    pub fn new(top: usize) -> Self {
        Self {
            top,
            len: 0,
            tail: Vec::new(),
            segs: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    /// A model at `floor(len / 8) * 8` with an empty tail and every segment slot
    /// left empty (tests of the counter edge, which never open those slots).
    pub fn at_len(top: usize, len: u128) -> Self {
        let mut m = Self::new(top);
        m.len = len - (len % SEGC as u128);
        m
    }

    pub fn node_at(&self, h: usize, idx: u128) -> F {
        *self.nodes.get(&(h, idx)).unwrap_or(&empty_at(h))
    }

    pub fn root(&self) -> F {
        self.node_at(self.top + 3, 0)
    }

    pub fn state(&self) -> State {
        State {
            len: crate::membackend::fr(self.len),
            root: self.root(),
            tail: tail_d(&self.tail),
        }
    }

    /// Siblings of segment slot `q`, bottom up (TOP of them).
    pub fn sibs_top(&self, q: u128) -> Vec<F> {
        (3..self.top + 3)
            .map(|h| self.node_at(h, (q >> (h - 3)) ^ 1))
            .collect()
    }

    /// Opening path of record `addr` (TOP + 3 siblings).
    pub fn sibs_cold(&self, addr: u128) -> Vec<F> {
        let q = addr >> 3;
        let i = (addr & 7) as usize;
        let seg = self.segs.get(&q).expect("cold read of a flushed segment");
        let l0: Vec<F> = seg.iter().map(rec_d).collect();
        let l1: Vec<F> = l0.chunks(2).map(|p| node(1, p[0], p[1])).collect();
        let l2: Vec<F> = l1.chunks(2).map(|p| node(2, p[0], p[1])).collect();
        let mut s = vec![l0[i ^ 1], l1[(i >> 1) ^ 1], l2[(i >> 2) ^ 1]];
        s.extend(self.sibs_top(q));
        s
    }

    pub fn insert_seg(&mut self, q: u128, seg: [Rec; SEGC]) {
        let l: Vec<F> = seg.iter().map(rec_d).collect();
        let mut x = tree8(&l);
        self.segs.insert(q, seg);
        self.nodes.insert((3, q), x);
        for h in 3..self.top + 3 {
            let i = q >> (h - 3);
            let s = self.node_at(h, i ^ 1);
            x = if i & 1 == 1 {
                node(h + 1, s, x)
            } else {
                node(h + 1, x, s)
            };
            self.nodes.insert((h + 1, i >> 1), x);
        }
    }

    pub fn append(&mut self, r: Rec) {
        self.tail.push(r);
        self.len += 1;
        if self.tail.len() == SEGC {
            let q = self.len / SEGC as u128 - 1;
            let seg: [Rec; SEGC] = std::array::from_fn(|i| self.tail[i]);
            self.insert_seg(q, seg);
            self.tail.clear();
        }
    }

    /// The witness of one leaf with `w` flush units that appends `recs`; the
    /// model advances to the post state.
    pub fn leaf(&mut self, recs: &[Rec], w: usize) -> (WinW, DenseLeafW) {
        let r_pre = self.tail.len();
        assert!(r_pre + recs.len() < SEGC * (w + 1), "window overflow");
        let q_pre = self.len / SEGC as u128;
        let mut rows = Vec::new();
        for i in 0..SEGC * (w + 1) {
            let (rec, act, imp) = if i < r_pre {
                (self.tail[i], true, true)
            } else if i < r_pre + recs.len() {
                (recs[i - r_pre], true, false)
            } else {
                ([F::ZERO; 4], false, false)
            };
            let mut kind = 0u64;
            for kk in 1..=4u64 {
                if rec[0] == F::from(kk) {
                    kind = kk;
                }
            }
            assert!(!act || kind != 0, "record kind in 1..=4");
            rows.push(WinRowW {
                kind,
                d: [rec[1], rec[2], rec[3]],
                act,
                imp,
            });
        }
        let total = r_pre + recs.len();
        let mut units = Vec::new();
        let mut idx = q_pre;
        let mut pending: Vec<Rec> = self.tail.clone();
        pending.extend_from_slice(recs);
        for j in 0..w {
            let f = total >= SEGC * (j + 1);
            if j > 0 && f {
                idx += 1;
            }
            units.push(UnitW {
                sibs: self.sibs_top(idx),
                x0: None,
            });
            if f {
                let seg: [Rec; SEGC] = std::array::from_fn(|i| pending[SEGC * j + i]);
                self.insert_seg(q_pre + j as u128, seg);
            }
        }
        let nflush = total / SEGC;
        self.len += recs.len() as u128;
        self.tail = pending[SEGC * nflush..].to_vec();
        (
            WinW {
                q_pre: crate::membackend::fr(q_pre),
                rows,
            },
            DenseLeafW {
                top: self.top,
                units,
            },
        )
    }
}

/// The dense backend: masks, muxes, adds, the flush row, a gated equality and
/// the Merkle walk.
///
/// Arithmetic row columns: 5 `s`, 6 `x`, 7 `y`, 8 `out`, 9 `aux`. Walk row (the
/// absorb row of each compression): 2 `in0`, 3 `in1`, 5 `x`, 6 `s`, 7 `b`,
/// 8 `acc_in`, 9 `acc_out`.
#[derive(Clone, Debug)]
pub struct DenseCfg {
    pub cols: Cols,
    /// `mask`, `mux`, `add`, `flush`, `geq`, `walk`, `walk empty`, `walk last`.
    pub q: [VSel; 8],
}

const MASK: usize = 0;
const MUX: usize = 1;
const ADD: usize = 2;
const FLUSH: usize = 3;
const GEQ: usize = 4;
const WALK: usize = 5;
const WALK_EMPTY: usize = 6;
const WALK_LAST: usize = 7;

fn k(v: u64) -> Expression<F> {
    Expression::Constant(F::from(v))
}

/// Walk mode of the first level.
#[derive(Clone, Copy, PartialEq, Eq)]
enum First {
    Plain,
    Empty,
}

/// Shared cells of one walk level: `s`, `b`, `acc_in`, `acc_out`.
type WRow = [Cell; 4];

impl DenseCfg {
    pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols) -> Self {
        let a = cols.a;
        for c in a {
            meta.enable_equality(c);
        }
        let qv = pack(meta, &[2, 2, 1, 2, 2, 2, 2, 2]);
        let q: [VSel; 8] = std::array::from_fn(|i| qv[i]);
        let cur = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::cur());
        let pad = Expression::Constant(pad_d());
        let empty = Expression::Constant(empty_seg());
        let sq = q;
        meta.create_gate("dense mask", |m| {
            let s = sq[MASK].expr(m);
            let (b, x, out) = (cur(m, 5), cur(m, 6), cur(m, 8));
            crate::gate(vec![
                s.clone() * b.clone() * (k(1) - b.clone()),
                s * (out - pad.clone() - b * (x - pad.clone())),
            ])
        });
        meta.create_gate("dense mux", |m| {
            let s = sq[MUX].expr(m);
            let (b, x, y, out) = (cur(m, 5), cur(m, 6), cur(m, 7), cur(m, 8));
            crate::gate(vec![
                s.clone() * b.clone() * (k(1) - b.clone()),
                s * (out - y.clone() - b * (x - y)),
            ])
        });
        meta.create_gate("dense add", |m| {
            let s = sq[ADD].expr(m);
            crate::gate(vec![s * (cur(m, 8) - cur(m, 6) - cur(m, 7))])
        });
        meta.create_gate("dense flush", |m| {
            let s = sq[FLUSH].expr(m);
            let (f, new, rin, out, old) = (cur(m, 5), cur(m, 6), cur(m, 7), cur(m, 8), cur(m, 9));
            crate::gate(vec![
                s.clone() * f.clone() * (k(1) - f.clone()),
                s.clone() * f.clone() * (old - rin.clone()),
                s * (out - rin.clone() - f * (new - rin)),
            ])
        });
        meta.create_gate("dense geq", |m| {
            let s = sq[GEQ].expr(m);
            let (b, x, y) = (cur(m, 5), cur(m, 6), cur(m, 7));
            crate::gate(vec![
                s.clone() * b.clone() * (k(1) - b.clone()),
                s * b * (x - y),
            ])
        });
        let walk = |m: &mut VirtualCells<'_, F>| {
            let (in0, in1, x, s, b, ai, ao) = (
                cur(m, 2),
                cur(m, 3),
                cur(m, 5),
                cur(m, 6),
                cur(m, 7),
                cur(m, 8),
                cur(m, 9),
            );
            vec![
                b.clone() * (k(1) - b.clone()),
                in0 - x.clone() - b.clone() * (s.clone() - x.clone()),
                in1 - s.clone() - b.clone() * (x - s),
                ai - ao * k(2) - b,
            ]
        };
        meta.create_gate("dense walk", |m| {
            let s = sq[WALK].expr(m);
            crate::gate(walk(m).into_iter().map(|e| s.clone() * e).collect())
        });
        meta.create_gate("dense walk empty", |m| {
            let s = sq[WALK_EMPTY].expr(m);
            let mut v = walk(m);
            v.push(cur(m, 5) - empty.clone());
            crate::gate(v.into_iter().map(|e| s.clone() * e).collect())
        });
        meta.create_gate("dense walk last", |m| {
            let s = sq[WALK_LAST].expr(m);
            let mut v = walk(m);
            v.push(cur(m, 9));
            crate::gate(v.into_iter().map(|e| s.clone() * e).collect())
        });
        Self { cols, q }
    }

    /// An arithmetic row of gate `g` with cells in columns 5..10 (`None` leaves
    /// a column at zero). Returns the `out` cell (column 8).
    fn arith(
        &self,
        r: &mut Region<'_, F>,
        o: &mut usize,
        g: usize,
        ins: [Option<Src>; 5],
        out: Value<F>,
    ) -> Result<Cell, Error> {
        let a = self.cols.a;
        let row = *o;
        self.q[g].enable(r, row)?;
        let mut oc = None;
        for (j, src) in ins.iter().enumerate() {
            let col = a[5 + j];
            if j == 3 {
                oc = Some(r.assign_advice(|| "out", col, row, || out)?);
                continue;
            }
            match src {
                Some(s) => {
                    put(r, col, row, s)?;
                }
                None => {
                    r.assign_advice(|| "zero", col, row, || Value::known(F::ZERO))?;
                }
            }
        }
        *o += 1;
        Ok(oc.unwrap())
    }

    fn mask(&self, r: &mut Region<'_, F>, o: &mut usize, s: &Cell, x: &Src) -> Result<Cell, Error> {
        let out = s
            .value()
            .copied()
            .zip(x.value())
            .map(|(s, x)| pad_d() + s * (x - pad_d()));
        self.arith(
            r,
            o,
            MASK,
            [
                Some(Src::Cell(s.clone())),
                Some(x.clone()),
                None,
                None,
                None,
            ],
            out,
        )
    }

    fn mux(
        &self,
        r: &mut Region<'_, F>,
        o: &mut usize,
        s: &Cell,
        x: &Cell,
        y: &Cell,
    ) -> Result<Cell, Error> {
        let out = s
            .value()
            .copied()
            .zip(x.value().copied())
            .zip(y.value().copied())
            .map(|((s, x), y)| y + s * (x - y));
        let c = |c: &Cell| Some(Src::Cell(c.clone()));
        self.arith(r, o, MUX, [c(s), c(x), c(y), None, None], out)
    }

    fn add(&self, r: &mut Region<'_, F>, o: &mut usize, x: &Cell, y: &Cell) -> Result<Cell, Error> {
        let out = x
            .value()
            .copied()
            .zip(y.value().copied())
            .map(|(x, y)| x + y);
        let c = |c: &Cell| Some(Src::Cell(c.clone()));
        self.arith(r, o, ADD, [None, c(x), c(y), None, None], out)
    }

    fn sponge(
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        ins: &[HIn],
        ivv: F,
    ) -> Result<Cell, Error> {
        let mut perm = 0;
        let (out, n) = pc.sponge_iv(r, *o, ins, ivv, false, &Tamper::default(), &mut perm)?;
        *o += n;
        Ok(out)
    }

    fn tree8(
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        l: &[Cell],
    ) -> Result<Cell, Error> {
        let mut v = l.to_vec();
        let mut h = 1u64;
        while v.len() > 1 {
            let mut nx = Vec::new();
            for p in v.chunks(2) {
                let ins = [HIn::Cell(p[0].clone()), HIn::Cell(p[1].clone())];
                nx.push(Self::sponge(pc, r, o, &ins, iv(DOM_NODE, h))?);
            }
            v = nx;
            h += 1;
        }
        Ok(v.pop().unwrap())
    }

    /// A walk of `sibs.len()` levels from `x0`, the first compression at height
    /// `h0`, its index `idx` (copied into the first `acc_in`). With `shared`, the
    /// `s`, `b` and accumulator cells are copies of another walk's.
    #[allow(clippy::too_many_arguments)]
    fn walk(
        &self,
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        x0: &Src,
        h0: usize,
        idx: &Cell,
        sibs: &[F],
        first: First,
        shared: Option<&[WRow]>,
    ) -> Result<(Cell, Vec<WRow>), Error> {
        let a = self.cols.a;
        let n = sibs.len();
        let (acc, bits) = bit_accs(crate::membackend::val(idx), n);
        let mut x: Option<Cell> = None;
        let mut rows: Vec<WRow> = Vec::new();
        for l in 0..n {
            let row = *o;
            let xc = match &x {
                None => put(r, a[5], row, x0)?,
                Some(c) => c.copy_advice(|| "x", r, a[5], row)?,
            };
            let wr: WRow = match shared {
                Some(sh) => {
                    let p = &sh[l];
                    [
                        p[0].copy_advice(|| "s", r, a[6], row)?,
                        p[1].copy_advice(|| "b", r, a[7], row)?,
                        p[2].copy_advice(|| "acc_in", r, a[8], row)?,
                        p[3].copy_advice(|| "acc_out", r, a[9], row)?,
                    ]
                }
                None => {
                    let sc = r.assign_advice(|| "s", a[6], row, || Value::known(sibs[l]))?;
                    let bc = r.assign_advice(|| "b", a[7], row, || Value::known(bits[l]))?;
                    let ai = match rows.last() {
                        None => idx.copy_advice(|| "acc_in", r, a[8], row)?,
                        Some(p) => p[3].copy_advice(|| "acc_in", r, a[8], row)?,
                    };
                    let ao =
                        r.assign_advice(|| "acc_out", a[9], row, || Value::known(acc[l + 1]))?;
                    [sc, bc, ai, ao]
                }
            };
            let g = if l + 1 == n {
                WALK_LAST
            } else if l == 0 && first == First::Empty {
                WALK_EMPTY
            } else {
                WALK
            };
            self.q[g].enable(r, row)?;
            let xv = xc.value().copied();
            let sv = wr[0].value().copied();
            let bv = wr[1].value().copied();
            let in0 = xv.zip(sv).zip(bv).map(|((x, s), b)| x + b * (s - x));
            let in1 = xv.zip(sv).zip(bv).map(|((x, s), b)| s + b * (x - s));
            let out = Self::sponge(
                pc,
                r,
                o,
                &[HIn::Val(in0), HIn::Val(in1)],
                iv(DOM_NODE, (h0 + l) as u64),
            )?;
            rows.push(wr);
            x = Some(out);
        }
        Ok((x.unwrap(), rows))
    }
}

impl MemBackend for DenseCfg {
    type LeafW = DenseLeafW;
    type ReadW = DenseReadW;

    fn persist(
        &self,
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        win: &Window,
        pre: &[Cell],
        w: &DenseLeafW,
        names: &mut Names,
    ) -> Result<Vec<Cell>, Error> {
        let nw = win.rows.len();
        let nunits = nw / SEGC - 1;
        assert_eq!(w.units.len(), nunits);
        let (pre_root, pre_tail) = (&pre[0], &pre[1]);
        // Record hashes; the last row is inactive (MEM), so it is not hashed.
        let mut mark = *o;
        let mut lap = |o: usize, label: &str| {
            crate::membackend::log_rows(label, o - mark);
            mark = o;
        };
        let mut recs: Vec<Src> = Vec::new();
        for (i, row) in win.rows.iter().enumerate() {
            if i + 1 == nw {
                recs.push(Src::known(F::ZERO));
                continue;
            }
            let ins: Vec<HIn> = row.rec.iter().map(|c| HIn::Cell(c.clone())).collect();
            recs.push(Src::Cell(Self::sponge(pc, r, o, &ins, iv(DOM_REC, 0))?));
        }
        // Pre tail: the imported prefix of segment 0, padded.
        lap(*o, "persist.records");
        let mut m_imp = Vec::new();
        for (row, rec) in win.rows.iter().zip(&recs).take(SEGC) {
            m_imp.push(self.mask(r, o, &row.imp, rec)?);
        }
        let t_pre = Self::tree8(pc, r, o, &m_imp)?;
        names.push(("tail_open".into(), t_pre.clone()));
        r.constrain_equal(t_pre.cell(), pre_tail.cell())?;
        // Window segments.
        lap(*o, "persist.tail_open");
        let mut m_act = Vec::new();
        for (row, rec) in win.rows.iter().zip(&recs) {
            m_act.push(self.mask(r, o, &row.act, rec)?);
        }
        let mut ts = Vec::new();
        for j in 0..=nunits {
            ts.push(Self::tree8(pc, r, o, &m_act[SEGC * j..SEGC * (j + 1)])?);
        }
        // Flush units.
        lap(*o, "persist.segments");
        let mut root = pre_root.clone();
        let mut idx = win.q_pre.clone();
        for (j, u) in w.units.iter().enumerate() {
            let f = &win.rows[SEGC * j + SEGC - 1].act;
            if j > 0 {
                idx = self.add(r, o, &idx, f)?;
            }
            let x0 = Src::known(u.x0.unwrap_or(empty_seg()));
            let (old, rows) = self.walk(pc, r, o, &x0, 4, &idx, &u.sibs, First::Empty, None)?;
            let (new, _) = self.walk(
                pc,
                r,
                o,
                &Src::Cell(ts[j].clone()),
                4,
                &idx,
                &u.sibs,
                First::Plain,
                Some(&rows),
            )?;
            let out = f
                .value()
                .copied()
                .zip(new.value().copied())
                .zip(root.value().copied())
                .map(|((f, nw), rt)| rt + f * (nw - rt));
            let c = |c: &Cell| Some(Src::Cell(c.clone()));
            root = self.arith(r, o, FLUSH, [c(f), c(&new), c(&root), None, c(&old)], out)?;
            names.push((format!("flush{j}.root"), root.clone()));
            lap(*o, "persist.flush_unit");
        }
        // Post tail: T_k, k = number of flushes.
        let mut t = ts[0].clone();
        for j in 0..nunits {
            let f = &win.rows[SEGC * j + SEGC - 1].act;
            t = self.mux(r, o, f, &ts[j + 1], &t)?;
        }
        names.push(("tail_post".into(), t.clone()));
        lap(*o, "persist.tail_close");
        Ok(vec![root, t])
    }

    fn cold_read(
        &self,
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        pre: &[Cell],
        rd: &ColdRead,
        w: &DenseReadW,
        names: &mut Names,
    ) -> Result<(), Error> {
        assert_eq!(w.sibs.len(), w.top + 3);
        let ins: Vec<HIn> = rd.rec.iter().map(|c| HIn::Cell(c.clone())).collect();
        let start = *o;
        let rec = Self::sponge(pc, r, o, &ins, iv(DOM_REC, 0))?;
        let (root, _) = self.walk(
            pc,
            r,
            o,
            &Src::Cell(rec),
            1,
            &rd.addr,
            &w.sibs,
            First::Plain,
            None,
        )?;
        names.push(("cold.root".into(), root.clone()));
        let c = |c: &Cell| Some(Src::Cell(c.clone()));
        self.arith(
            r,
            o,
            GEQ,
            [c(&rd.enable), c(&root), c(&pre[0]), None, None],
            Value::known(F::ZERO),
        )?;
        crate::membackend::log_rows("cold_read", *o - start);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
