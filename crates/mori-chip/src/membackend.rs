//! MEM interface with swappable memory backends (experiment C1 of
//! `research/sherlock-final-design-20261111/NIGHT-PLAN.md`).
//!
//! The leaf's memory is split in two modules (`MODULARITY.md` items 1 and 3):
//!
//! - MEM ([`WindowCfg`], backend-neutral): the frontier decomposition of `len_pre`
//!   and `len_post` and the in-leaf window. The window is `8 (W + 1)` provider
//!   rows at addresses `base + i`, `base = 8 floor(len_pre / 8)`: rows below
//!   `len_pre` are the imported tail (`imp = 1`), rows below `len_post` are active
//!   (`act = 1`), the rest is padding. Active rows serve
//!   `(MEM, 8 addr + kind, a, b, c)` on the bus; padding serves the zero tuple.
//! - MEMORY BACKEND ([`MemBackend`]): authenticates the imported tail against the
//!   pre token, persists completed segments, returns the post token, and opens
//!   records below `base` (cold reads). The token is opaque to MEM: the dense
//!   backend uses `[root, tail_d]`; a write-once LogUp backend would put its
//!   obligation cells there.
//!
//! MEM guarantees to the backend (each is a gate of [`WindowCfg::window`], which
//! also decomposes both lengths; mutant names in `dense::tests`):
//! - `len_pre` and `len_post` are integers below `2^(TOP + 3)` (TOP + 3 boolean
//!   bits each); `q_pre = floor(len_pre / 8)` is the high part of `len_pre`;
//! - row `i` has address `8 q_pre + i` (one row per address);
//! - `act` and `imp` are boolean prefixes, `imp <= act`, the `imp` prefix ends at
//!   `len_pre` and the `act` prefix at `len_post`; the last row is inactive, so a
//!   leaf adds at most `8 W + 7 - r_pre` records and crosses at most `W`
//!   segment boundaries;
//! - a padding row has zero kind and data and puts the zero tuple on the bus;
//! - `kind` is in `0..=4`, and equal to the row's fixed `kfix` when `kfix != 0`
//!   (the four-arena variant pins an arena's kind); so `8 addr + kind` is
//!   injective and the bus key carries the kind tag that a reader checks.
use ff::{Field, PrimeField};
use midnight_proofs::{
    circuit::{AssignedCell, Region, Value},
    plonk::{Column, ConstraintSystem, Error, Expression, Fixed, VirtualCells},
    poly::Rotation,
};

use crate::{
    Cols, F,
    bus::{BusConfig, CHIP_MEM},
    poseidon::PoseidonCfg,
    vsel::{VSel, pack},
};

pub type Cell = AssignedCell<F, F>;

/// Records per segment.
pub const SEGC: usize = 8;
/// Record kinds (AUDIT finding 8: a read constrains the kind it expects).
pub const KIND_DATA: u64 = 1;
pub const KIND_FIELD: u64 = 2;
pub const KIND_ENV: u64 = 3;
pub const KIND_SLOT: u64 = 4;

/// Bus key of the record at `addr` with kind `kind`.
pub fn bus_key(addr: F, kind: F) -> F {
    addr * F::from(8u64) + kind
}

/// Named cells kept for negative tests (overwritten after synthesis).
pub type Names = Vec<(String, Cell)>;

/// One window row as seen by the backend.
#[derive(Clone, Debug)]
pub struct WinRow {
    pub addr: Cell,
    pub act: Cell,
    pub imp: Cell,
    /// `[kind, a, b, c]`.
    pub rec: [Cell; 4],
}

/// The window of one leaf.
#[derive(Clone, Debug)]
pub struct Window {
    pub len_pre: Cell,
    pub len_post: Cell,
    pub q_pre: Cell,
    /// Address of row 0 (`8 q_pre`).
    pub base: Cell,
    pub rows: Vec<WinRow>,
}

/// A read of a record below `base`, enabled by `enable` (boolean, gated by the
/// requester): the backend proves `rec` is the record at `addr` in the pre token.
#[derive(Clone, Debug)]
pub struct ColdRead {
    pub enable: Cell,
    pub addr: Cell,
    pub rec: [Cell; 4],
}

/// A memory backend behind MEM. `pre` and the returned post token are the
/// backend's boundary cells; MEM never reads inside them.
pub trait MemBackend {
    /// Per-leaf auxiliary witness (dense: walk siblings).
    type LeafW;
    /// Per-cold-read auxiliary witness (dense: the opening path).
    type ReadW;

    /// Authenticate the imported tail of `win` against `pre`, persist the
    /// completed segments, and return the post token.
    #[allow(clippy::too_many_arguments)]
    fn persist(
        &self,
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        win: &Window,
        pre: &[Cell],
        w: &Self::LeafW,
        names: &mut Names,
    ) -> Result<Vec<Cell>, Error>;

    /// When `rd.enable = 1`, prove that `rd.rec` is the record at `rd.addr`
    /// under `pre`.
    #[allow(clippy::too_many_arguments)]
    fn cold_read(
        &self,
        pc: &PoseidonCfg,
        r: &mut Region<'_, F>,
        o: &mut usize,
        pre: &[Cell],
        rd: &ColdRead,
        w: &Self::ReadW,
        names: &mut Names,
    ) -> Result<(), Error>;
}

/// Witness of one window row.
#[derive(Clone, Copy, Debug, Default)]
pub struct WinRowW {
    pub kind: u64,
    pub d: [F; 3],
    pub act: bool,
    pub imp: bool,
}

/// Witness of a window.
#[derive(Clone, Debug)]
pub struct WinW {
    pub q_pre: F,
    pub rows: Vec<WinRowW>,
}

/// MEM: frontier bits, window rows and an increment row.
///
/// Columns of a window row: 0..5 the provider tuple, 5 `addr`, 6 `kind`, 7 `act`,
/// 8 `imp`, 9 `len_post`, 10 `len_pre`, 11 `q_pre` (row 0 only). A bits row:
/// 7 `b`, 8 `acc_in`, 9 `acc_out`. An increment row: 5 `x`, 6 `x + 1`.
#[derive(Clone, Debug)]
pub struct WindowCfg {
    pub cols: Cols,
    pub bus: BusConfig,
    /// Expected kind of a row (0: any of `1..=4`); also used by requesters.
    pub kfix: Column<Fixed>,
    /// `row`, `first`, `chain`, `last`, `bits step`, `bits last`, `inc`.
    pub q: [VSel; 7],
}

const W_ROW: usize = 0;
const W_FIRST: usize = 1;
const W_CHAIN: usize = 2;
const W_LAST: usize = 3;
const B_STEP: usize = 4;
const B_LAST: usize = 5;
const INC: usize = 6;

fn k(v: u64) -> Expression<F> {
    Expression::Constant(F::from(v))
}

/// The lowest bit of the canonical integer of `v`.
pub fn low_bit(v: F) -> F {
    if bool::from(v.is_odd()) {
        F::ONE
    } else {
        F::ZERO
    }
}

/// The `acc_in` values of a bits decomposition of `v`: `v`, then `(acc - b) / 2`.
/// The last `acc_out` is zero exactly when `v < 2^nb` as an integer.
pub fn bit_accs(v: F, nb: usize) -> (Vec<F>, Vec<F>) {
    let half = F::from(2u64).invert().unwrap();
    let mut acc = vec![v];
    let mut bits = Vec::new();
    for i in 0..nb {
        let b = low_bit(acc[i]);
        bits.push(b);
        acc.push((acc[i] - b) * half);
    }
    (acc, bits)
}

/// Copy `c` to `(col, row)`, or assign the value.
pub fn put(
    r: &mut Region<'_, F>,
    col: Column<midnight_proofs::plonk::Advice>,
    row: usize,
    v: &Src,
) -> Result<Cell, Error> {
    match v {
        Src::Cell(c) => c.copy_advice(|| "copy", r, col, row),
        Src::Val(x) => r.assign_advice(|| "val", col, row, || *x),
    }
}

/// A cell source: a copy of a cell, or a free value.
#[derive(Clone, Debug)]
pub enum Src {
    Cell(Cell),
    Val(Value<F>),
}

impl Src {
    pub fn value(&self) -> Value<F> {
        match self {
            Src::Cell(c) => c.value().copied(),
            Src::Val(v) => *v,
        }
    }
    pub fn known(v: F) -> Self {
        Src::Val(Value::known(v))
    }
}

/// The value of an assigned cell (tests: always known).
pub fn val(c: &Cell) -> F {
    let mut out = F::ZERO;
    c.value().map(|v| out = *v);
    out
}

impl WindowCfg {
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig) -> Self {
        let cols = bus.cols;
        let a = cols.a;
        for c in a {
            meta.enable_equality(c);
        }
        let kfix = meta.fixed_column();
        let qv = pack(meta, &[5, 3, 3, 1, 2, 2, 1]);
        let q: [VSel; 7] = std::array::from_fn(|i| qv[i]);
        let cur = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::cur());
        let nxt = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::next());
        let sq = q;
        meta.create_gate("mem win row", move |m| {
            let s = sq[W_ROW].expr(m);
            let (chip, key) = (cur(m, 0), cur(m, 1));
            let (addr, kind, act, imp) = (cur(m, 5), cur(m, 6), cur(m, 7), cur(m, 8));
            let kf = m.query_fixed(kfix, Rotation::cur());
            let na = k(1) - act.clone();
            let mut v = vec![
                act.clone() * na.clone(),
                imp.clone() * (k(1) - imp.clone()),
                imp * na.clone(),
                chip - act.clone() * k(CHIP_MEM),
                key - act.clone() * (addr * k(8) + kind.clone()),
                na.clone() * kind.clone(),
            ];
            for i in 2..5 {
                v.push(na.clone() * cur(m, i));
            }
            v.push((1..=4).fold(kind.clone(), |e, j| e * (kind.clone() - k(j))));
            v.push(act * kf.clone() * (kind - kf));
            crate::gate(v.into_iter().map(|e| s.clone() * e).collect())
        });
        let chain = |m: &mut VirtualCells<'_, F>| {
            let (addr, act, imp, lp, lq) = (cur(m, 5), cur(m, 7), cur(m, 8), cur(m, 9), cur(m, 10));
            let (addr2, act2, imp2) = (nxt(m, 5), nxt(m, 7), nxt(m, 8));
            vec![
                addr2 - addr.clone() - k(1),
                nxt(m, 9) - lp.clone(),
                nxt(m, 10) - lq.clone(),
                act2.clone() * (k(1) - act.clone()),
                imp2.clone() * (k(1) - imp.clone()),
                act * (k(1) - act2) * (addr.clone() + k(1) - lp),
                imp * (k(1) - imp2) * (addr + k(1) - lq),
            ]
        };
        meta.create_gate("mem win first", move |m| {
            let s = sq[W_FIRST].expr(m);
            let (addr, act, imp, lp, lq, qp) = (
                cur(m, 5),
                cur(m, 7),
                cur(m, 8),
                cur(m, 9),
                cur(m, 10),
                cur(m, 11),
            );
            let mut v = chain(m);
            v.push((k(1) - act) * (addr.clone() - lp));
            v.push((k(1) - imp) * (addr.clone() - lq));
            v.push(addr - qp * k(8));
            crate::gate(v.into_iter().map(|e| s.clone() * e).collect())
        });
        meta.create_gate("mem win chain", move |m| {
            let s = sq[W_CHAIN].expr(m);
            crate::gate(chain(m).into_iter().map(|e| s.clone() * e).collect())
        });
        meta.create_gate("mem win last", move |m| {
            let s = sq[W_LAST].expr(m);
            crate::gate(vec![s * cur(m, 7)])
        });
        let bits = |m: &mut VirtualCells<'_, F>| {
            let (b, ai, ao) = (cur(m, 7), cur(m, 8), cur(m, 9));
            vec![b.clone() * (k(1) - b.clone()), ai - ao * k(2) - b]
        };
        meta.create_gate("mem bits step", move |m| {
            let s = sq[B_STEP].expr(m);
            let mut v = bits(m);
            v.push(nxt(m, 8) - cur(m, 9));
            crate::gate(v.into_iter().map(|e| s.clone() * e).collect())
        });
        meta.create_gate("mem bits last", move |m| {
            let s = sq[B_LAST].expr(m);
            let mut v = bits(m);
            v.push(cur(m, 9));
            crate::gate(v.into_iter().map(|e| s.clone() * e).collect())
        });
        meta.create_gate("mem inc", move |m| {
            let s = sq[INC].expr(m);
            crate::gate(vec![s * (cur(m, 6) - cur(m, 5) - k(1))])
        });
        Self { cols, bus, kfix, q }
    }

    /// Decompose `v` into `nb` bits from row `*o`. Returns the `acc_in` cells:
    /// `[0]` is `v`, `[i]` is `floor(v / 2^i)`.
    pub fn bits(
        &self,
        r: &mut Region<'_, F>,
        o: &mut usize,
        v: &Src,
        nb: usize,
    ) -> Result<Vec<Cell>, Error> {
        let a = self.cols.a;
        let mut x = F::ZERO;
        v.value().map(|y| x = y);
        let (acc, bits) = bit_accs(x, nb);
        let mut out = Vec::new();
        for i in 0..nb {
            let row = *o;
            self.q[if i + 1 == nb { B_LAST } else { B_STEP }].enable(r, row)?;
            let ai = if i == 0 {
                put(r, a[8], row, v)?
            } else {
                r.assign_advice(|| "acc_in", a[8], row, || Value::known(acc[i]))?
            };
            r.assign_advice(|| "b", a[7], row, || Value::known(bits[i]))?;
            r.assign_advice(|| "acc_out", a[9], row, || Value::known(acc[i + 1]))?;
            out.push(ai);
            *o += 1;
        }
        Ok(out)
    }

    /// An increment row: returns the cell `x + 1`.
    pub fn inc(&self, r: &mut Region<'_, F>, o: &mut usize, x: &Cell) -> Result<Cell, Error> {
        let a = self.cols.a;
        let row = *o;
        self.q[INC].enable(r, row)?;
        x.copy_advice(|| "x", r, a[5], row)?;
        let y = r.assign_advice(|| "x+1", a[6], row, || x.value().map(|v| *v + F::ONE))?;
        *o += 1;
        Ok(y)
    }

    /// The frontier and the window rows from `*o`. `len_pre` and `len_post` (from
    /// the boundary and from CORE) are decomposed here into `top + 3` bits each,
    /// so every user of MEM gets the range checks; the returned [`Window`]
    /// carries the decomposed cells, which the caller copies to the boundary.
    #[allow(clippy::too_many_arguments)]
    pub fn window(
        &self,
        r: &mut Region<'_, F>,
        o: &mut usize,
        len_pre: &Src,
        len_post: &Src,
        top: usize,
        kfix: u64,
        w: &WinW,
        names: &mut Names,
        tag: &str,
    ) -> Result<Window, Error> {
        let start = *o;
        let lp = self.bits(r, o, len_pre, top + 3)?;
        let lq = self.bits(r, o, len_post, top + 3)?;
        log_rows("mem.frontier", *o - start);
        let (len_pre, q_pre, len_post) = (&lp[0], &lp[3], &lq[0]);
        let a = self.cols.a;
        let nw = w.rows.len();
        assert!(nw >= 2 && nw.is_multiple_of(SEGC));
        let mut rows = Vec::new();
        let mut base = None;
        for (i, rw) in w.rows.iter().enumerate() {
            let row = *o;
            self.q[W_ROW].enable(r, row)?;
            let code = if i == 0 {
                W_FIRST
            } else if i + 1 == nw {
                W_LAST
            } else {
                W_CHAIN
            };
            self.q[code].enable(r, row)?;
            r.assign_fixed(|| "kfix", self.kfix, row, || Value::known(F::from(kfix)))?;
            let addr = w.q_pre * F::from(8u64) + F::from(i as u64);
            let actf = F::from(rw.act as u64);
            let kind = F::from(rw.kind);
            let t = [
                actf * F::from(CHIP_MEM),
                actf * bus_key(addr, kind),
                rw.d[0],
                rw.d[1],
                rw.d[2],
            ];
            let tc = self.bus.provide(r, row, Value::known(t))?;
            let ad = r.assign_advice(|| "addr", a[5], row, || Value::known(addr))?;
            let kc = r.assign_advice(|| "kind", a[6], row, || Value::known(kind))?;
            let ac = r.assign_advice(|| "act", a[7], row, || Value::known(actf))?;
            let ic =
                r.assign_advice(|| "imp", a[8], row, || Value::known(F::from(rw.imp as u64)))?;
            if i == 0 {
                len_post.copy_advice(|| "len_post", r, a[9], row)?;
                len_pre.copy_advice(|| "len_pre", r, a[10], row)?;
                q_pre.copy_advice(|| "q_pre", r, a[11], row)?;
                base = Some(ad.clone());
            } else {
                r.assign_advice(|| "len_post", a[9], row, || len_post.value().copied())?;
                r.assign_advice(|| "len_pre", a[10], row, || len_pre.value().copied())?;
                r.assign_advice(|| "q0", a[11], row, || Value::known(F::ZERO))?;
            }
            for (nm, c) in [
                ("chip", &tc[0]),
                ("key", &tc[1]),
                ("addr", &ad),
                ("kind", &kc),
                ("act", &ac),
            ] {
                names.push((format!("{tag}.win{i}.{nm}"), c.clone()));
            }
            rows.push(WinRow {
                addr: ad,
                act: ac,
                imp: ic,
                rec: [kc, tc[2].clone(), tc[3].clone(), tc[4].clone()],
            });
            *o += 1;
        }
        Ok(Window {
            len_pre: len_pre.clone(),
            len_post: len_post.clone(),
            q_pre: q_pre.clone(),
            base: base.unwrap(),
            rows,
        })
    }
}

/// Overwrite an assigned advice cell (negative tests; copy constraints stay).
pub fn overwrite(r: &mut Region<'_, F>, c: &Cell, v: F) -> Result<(), Error> {
    let col =
        midnight_proofs::plonk::Column::<midnight_proofs::plonk::Advice>::try_from(c.cell().column)
            .expect("advice cell");
    r.assign_advice(|| "mutant", col, c.cell().row_offset, || Value::known(v))?;
    Ok(())
}

thread_local! {
    static ROWS: std::cell::RefCell<Vec<(String, usize)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Record that section `label` used `n` rows (measurement only). A synthesis
/// pass starts with [`rows_begin`]; [`rows_last`] returns the last pass.
pub fn log_rows(label: &str, n: usize) {
    ROWS.with(|r| r.borrow_mut().push((label.to_string(), n)));
}

pub fn rows_begin() {
    log_rows("BEGIN", 0);
}

/// The sections of the last synthesis pass.
pub fn rows_last() -> Vec<(String, usize)> {
    ROWS.with(|r| {
        let v = r.borrow();
        let start = v
            .iter()
            .rposition(|(l, _)| l == "BEGIN")
            .map_or(0, |i| i + 1);
        v[start..].to_vec()
    })
}

/// `F` from an integer below 2^128.
pub fn fr(v: u128) -> F {
    F::from_u128(v)
}
