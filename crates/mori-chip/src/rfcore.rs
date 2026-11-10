//! E5: the 32-word register file of [`crate::rf`] in three row layouts, with the
//! ALU, MEM, RANGE and DEC chips of the spike, a state digest region and an
//! optional HASH chip (E6, [`crate::hashchip`]).
//!
//! The in-circuit verifier of the node evaluates queries at rotations -1, 0 and +1
//! only (`midnight-circuits` 7.3.0 `verifier_gadget.rs:609`), so a gate sees at most
//! three consecutive rows. A layout says where each logical cell of a step slot
//! lives (row in the slot, advice column) and on which slot rows gates sit. Every
//! constraint is written once over logical cells; [`Layout::host`] puts it on the
//! first gate row from which all its cells are visible, and configuration panics if
//! no such row exists (no constraint is ever dropped).
//!
//! Layouts:
//! - [`Arch::Wide`] (A): one row per slot holding every register as its own column,
//!   the two bus lanes, the flags and the count; the next state is the next row.
//! - [`Arch::Narrow`] (B): 12 columns. A copy constraint is unconditional, so it
//!   can only link a slot's post cell to the next slot's pre cell; the frame
//!   condition "unchanged unless named" still needs a gate over a pre/post pair.
//!   So each register is a `(pre, post)` pair in a pair row, `post` is copied to the
//!   next slot's `pre`, and every pair row sits between a lane row and a flag row
//!   (replicas copied from the canonical rows). Operands from other pair rows are
//!   routed by copies into spare cells.
//! - [`Arch::Staggered`] (C): two rows per slot, a `lo` row of registers and a `hi`
//!   row of the lanes, flags, count and the registers whose next state needs only
//!   `hi` cells. The gate on the `hi` row checks the `lo` registers (it sees `lo`,
//!   `hi` and the next `lo`); the gate on the next `lo` row checks the `hi`
//!   registers (it sees `hi`, the next `lo` and the next `hi`).
//!
//! The state digest is the placeholder linear digest over words 0..29 (Horner in
//! `lambda`). Its words are copied into a 3-row digest region for every layout, as
//! a Poseidon digest would need; [`Arch::WideInline`] is A with the digest computed
//! by one gate on the register row instead (a lower bound: Poseidon cannot be one
//! gate).
use std::collections::{BTreeMap, BTreeSet};

use ff::Field;

use midnight_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Instance, Selector},
    poly::Rotation,
};

use crate::{
    Cols, F, NCOL,
    alu::{AluConfig, AluOp, FLAG_SHIFT, OP_ADD, OP_LT},
    bus::{BusConfig, CHIP_ALU, CHIP_MEM, Tuple, tuple},
    dec::DecConfig,
    fu,
    hashchip::{HashConfig, SlimHashShape, hash_cells, hash_free, hash_tuple},
    m3::{M3W, WalkConfig},
    mem::{MemConfig, SEG, SegMemConfig},
    poseidon::{HIn, PoseidonCfg, Tamper},
    range::{RangeConfig, rows_per_value},
    rf::*,
    trace::Cell as MemCell,
    vsel::{VSel, pack},
};

/// Limb columns of RANGE (as in the spike).
pub const RC: usize = 4;

/// A logical cell of a step slot. `Post`, `NextFlag` and `NextCnt` resolve to the
/// next slot's `Pre`, `Flag` and `Cnt` unless the layout places `Post` itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    Pre(usize),
    Post(usize),
    Flag(usize),
    NextFlag(usize),
    L0(usize),
    L1(usize),
    Ovf,
    Cnt,
    NextCnt,
    Dig,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Arch {
    #[default]
    Wide,
    WideInline,
    Narrow,
    Staggered,
}

/// Registers of the `hi` row in [`Arch::Staggered`]: their next state needs only
/// `hi` cells (lanes, flags, own value).
pub const HI: [usize; 7] = [V0, V1, V2, WORK, FUEL, FCOUNT, TCOUNT];

#[derive(Clone, Debug)]
pub struct Layout {
    pub arch: Arch,
    /// Rows per slot.
    pub h: usize,
    /// Advice columns.
    pub w: usize,
    /// The row of the bus request lanes (canonical `L0`, `L1` at columns 0..10).
    pub req_row: usize,
    /// `(cell, row, column)`; the first placement of a cell is canonical, later
    /// ones are copy-constrained to it.
    pub place: Vec<(Cell, usize, usize)>,
    /// Slot rows that carry a gate (may be `h`, the next slot's first row).
    pub gates: Vec<usize>,
}

fn lanes(place: &mut Vec<(Cell, usize, usize)>, row: usize) {
    for i in 0..5 {
        place.push((Cell::L0(i), row, i));
        place.push((Cell::L1(i), row, 5 + i));
    }
}

impl Layout {
    pub fn new(arch: Arch) -> Self {
        match arch {
            Arch::Wide | Arch::WideInline => {
                let mut p = Vec::new();
                lanes(&mut p, 0);
                p.push((Cell::Ovf, 0, 10));
                p.push((Cell::Cnt, 0, 11));
                for r in 0..NRULE {
                    p.push((Cell::Flag(r), 0, 12 + r));
                }
                for j in 0..NREG {
                    p.push((Cell::Pre(j), 0, 18 + j));
                }
                let mut w = 18 + NREG;
                if arch == Arch::WideInline {
                    p.push((Cell::Dig, 0, w));
                    w += 1;
                }
                Self {
                    arch,
                    h: 1,
                    w,
                    req_row: 0,
                    place: p,
                    gates: vec![0],
                }
            }
            Arch::Staggered => {
                let mut p = Vec::new();
                lanes(&mut p, 1);
                p.push((Cell::Ovf, 1, 10));
                p.push((Cell::Cnt, 1, 11));
                for r in 0..NRULE {
                    p.push((Cell::Flag(r), 1, 12 + r));
                }
                for (i, &j) in HI.iter().enumerate() {
                    p.push((Cell::Pre(j), 1, 18 + i));
                }
                let lo: Vec<usize> = (0..NREG).filter(|j| !HI.contains(j)).collect();
                for (i, &j) in lo.iter().enumerate() {
                    p.push((Cell::Pre(j), 0, i));
                }
                let w = (18 + HI.len()).max(lo.len());
                Self {
                    arch,
                    h: 2,
                    w,
                    req_row: 1,
                    place: p,
                    gates: vec![1, 2],
                }
            }
            Arch::Narrow => {
                // Rows: 0 F, 1 P0, 2 L, 3 P1, 4 F, 5 P2, 6 L, 7 P3, 8 F, 9 P4, 10 L,
                // 11 P5, 12 F. Canonical F is row 0, canonical L (requests) row 2.
                let f_row = |p: &mut Vec<(Cell, usize, usize)>, row: usize| {
                    for r in 0..NRULE {
                        p.push((Cell::Flag(r), row, r));
                    }
                    p.push((Cell::Ovf, row, 6));
                    p.push((Cell::Cnt, row, 7));
                };
                let mut p = Vec::new();
                f_row(&mut p, 0);
                lanes(&mut p, 2);
                let pairs: [(usize, &[usize]); 6] = [
                    (1, &[CTL_TAG, FCOUNT, FP, TCOUNT, TP, LEN]),
                    (3, &[TOP0, TOP1, TASK0, TASK1, CTL_A, CTL_B]),
                    (5, &[H0, H1, R1, R0, V0, V1]),
                    (7, &[V2, CTL_C, CTL_ORIGIN, OUTCOME, FUEL0, FUEL]),
                    (9, &[WORK, HCTR, LIMIT, ENV, H2, H3]),
                    (11, &[H4, H5]),
                ];
                for (row, regs) in pairs {
                    for (m, &j) in regs.iter().enumerate() {
                        p.push((Cell::Pre(j), row, 2 * m));
                        p.push((Cell::Post(j), row, 2 * m + 1));
                    }
                }
                for row in [4, 8, 12] {
                    f_row(&mut p, row);
                }
                for row in [6, 10] {
                    lanes(&mut p, row);
                }
                // Routed operands (copies of pre cells into spare cells).
                for (j, row, col) in [
                    (CTL_A, 0, 8),
                    (CTL_B, 0, 9),
                    (LIMIT, 0, 10),
                    (CTL_C, 4, 8),
                    (TASK0, 4, 9),
                    (TASK1, 4, 10),
                    (TOP0, 4, 11),
                ] {
                    p.push((Cell::Pre(j), row, col));
                }
                Self {
                    arch,
                    h: 13,
                    w: NCOL,
                    req_row: 2,
                    place: p,
                    gates: vec![1, 3, 5, 7, 9, 11, 12],
                }
            }
        }
    }

    /// Placements of `c` as `(row relative to the slot, column)`.
    fn cands(&self, c: Cell) -> Vec<(isize, usize)> {
        let h = self.h as isize;
        let of = |b: Cell, shift: isize| -> Vec<(isize, usize)> {
            self.place
                .iter()
                .filter(|p| p.0 == b)
                .map(|p| (p.1 as isize + shift, p.2))
                .collect()
        };
        match c {
            Cell::Post(j) => {
                let e = of(Cell::Post(j), 0);
                if e.is_empty() { of(Cell::Pre(j), h) } else { e }
            }
            Cell::NextFlag(r) => of(Cell::Flag(r), h),
            Cell::NextCnt => of(Cell::Cnt, h),
            c => of(c, 0),
        }
    }

    /// Column and rotation of `c` from a gate on slot row `o`, if visible.
    pub fn find(&self, c: Cell, o: usize) -> Option<(usize, i32)> {
        self.cands(c)
            .into_iter()
            .map(|(r, col)| (col, (r - o as isize) as i32))
            .find(|(_, rot)| (-1..=1).contains(rot))
    }

    /// Index of the first gate that sees every cell of `e`.
    pub fn host(&self, e: &Ex) -> Option<usize> {
        self.gates.iter().position(|&o| {
            let mut dry = |c: Cell| self.find(c, o).map(|_| k(0));
            e(&mut dry).is_some()
        })
    }

    /// Index in `place` of the canonical placement of `c`.
    fn canon(&self, c: Cell) -> usize {
        self.place.iter().position(|p| p.0 == c).expect("placed")
    }
}

/// A constraint over logical cells; `None` when a cell is not visible.
pub type Ex = Box<dyn Fn(&mut dyn FnMut(Cell) -> Option<Expression<F>>) -> Option<Expression<F>>>;

fn k(v: u64) -> Expression<F> {
    Expression::Constant(F::from(v))
}

fn ex(
    f: impl Fn(&mut dyn FnMut(Cell) -> Option<Expression<F>>) -> Option<Expression<F>> + 'static,
) -> Ex {
    Box::new(f)
}

type G<'a> = dyn FnMut(Cell) -> Option<Expression<F>> + 'a;

fn flags(g: &mut G<'_>) -> Option<Vec<Expression<F>>> {
    (0..NRULE).map(|r| g(Cell::Flag(r))).collect()
}

fn sum(v: &[Expression<F>]) -> Expression<F> {
    v.iter().cloned().fold(k(0), |a, b| a + b)
}

fn lambda() -> F {
    F::from(1_000_003u64)
}

/// Placeholder state digest: Horner in `lambda` over words `0..NDIG`.
pub fn rf_digest(x: &Regs32) -> F {
    let mut acc = F::from(0u64);
    for &w in x.iter().take(NDIG) {
        acc = acc * lambda() + fu(w);
    }
    acc
}

/// The next-state, request and flag constraints of the core.
pub fn core_constraints() -> Vec<(String, Ex)> {
    let mut v: Vec<(String, Ex)> = Vec::new();
    for r in 0..NRULE {
        v.push((
            format!("flag {r} bool"),
            ex(move |g| {
                let f = g(Cell::Flag(r))?;
                Some(f.clone() * (k(1) - f))
            }),
        ));
    }
    v.push((
        "en bool".into(),
        ex(|g| {
            let en = sum(&flags(g)?);
            Some(en.clone() * (k(1) - en))
        }),
    ));
    v.push((
        "en prefix".into(),
        ex(|g| {
            let en = sum(&flags(g)?);
            let nf: Option<Vec<_>> = (0..NRULE).map(|r| g(Cell::NextFlag(r))).collect();
            Some(sum(&nf?) * (k(1) - en))
        }),
    ));
    v.push((
        "count".into(),
        ex(|g| {
            let en = sum(&flags(g)?);
            Some(g(Cell::NextCnt)? - g(Cell::Cnt)? - en)
        }),
    ));
    v.push((
        "ovf".into(),
        ex(|g| {
            let o = g(Cell::Ovf)?;
            Some(o.clone() * (k(1) - o))
        }),
    ));
    v.push((
        "ovf arith only".into(),
        ex(|g| {
            let f = flags(g)?;
            Some((k(1) - f[ARITH].clone()) * g(Cell::Ovf)?)
        }),
    ));
    // Lane 0: ALU.
    v.push((
        "l0 chip".into(),
        ex(|g| {
            let f = flags(g)?;
            let act = sum(&f) - f[REWRITE].clone();
            Some(g(Cell::L0(0))? - act * k(CHIP_ALU))
        }),
    ));
    v.push((
        "l0 op".into(),
        ex(|g| {
            let f = flags(g)?;
            let lt = f[PUSH].clone() + f[POP].clone() + f[TPUSH].clone() + f[TPOP].clone();
            let ovf = g(Cell::Ovf)?;
            Some(
                g(Cell::L0(1))?
                    - lt * k(OP_LT)
                    - f[ARITH].clone() * (k(OP_ADD) + ovf * k(FLAG_SHIFT)),
            )
        }),
    ));
    v.push((
        "l0 x".into(),
        ex(|g| {
            let f = flags(g)?;
            Some(
                g(Cell::L0(2))?
                    - f[PUSH].clone() * g(Cell::Pre(FCOUNT))?
                    - f[POP].clone() * g(Cell::Pre(FP))?
                    - f[ARITH].clone() * g(Cell::Pre(CTL_A))?
                    - f[TPUSH].clone() * g(Cell::Pre(TCOUNT))?
                    - f[TPOP].clone() * g(Cell::Pre(TP))?,
            )
        }),
    ));
    v.push((
        "l0 y".into(),
        ex(|g| {
            let f = flags(g)?;
            let limit = g(Cell::Pre(LIMIT))?;
            let len = g(Cell::Pre(LEN))?;
            Some(
                g(Cell::L0(3))?
                    - (f[PUSH].clone() + f[TPUSH].clone()) * limit
                    - (f[POP].clone() + f[TPOP].clone()) * len
                    - f[ARITH].clone() * g(Cell::Pre(CTL_B))?,
            )
        }),
    ));
    v.push((
        "l0 z".into(),
        ex(|g| {
            let f = flags(g)?;
            let lt = f[PUSH].clone() + f[POP].clone() + f[TPUSH].clone() + f[TPOP].clone();
            Some(lt * (g(Cell::L0(4))? - k(1)))
        }),
    ));
    // Lane 1: MEM.
    v.push((
        "l1 chip".into(),
        ex(|g| {
            let f = flags(g)?;
            let m = f[PUSH].clone() + f[POP].clone() + f[TPUSH].clone() + f[TPOP].clone();
            Some(g(Cell::L1(0))? - m * k(CHIP_MEM))
        }),
    ));
    v.push((
        "l1 key".into(),
        ex(|g| {
            let f = flags(g)?;
            let len = g(Cell::Pre(LEN))?;
            Some(
                g(Cell::L1(1))?
                    - (f[PUSH].clone() + f[TPUSH].clone()) * len
                    - f[POP].clone() * g(Cell::Pre(FP))?
                    - f[TPOP].clone() * g(Cell::Pre(TP))?,
            )
        }),
    ));
    for (lane, a, b) in [(2, TOP0, TASK0), (3, TOP1, TASK1), (4, FP, TP)] {
        v.push((
            format!("l1 stored {lane}"),
            ex(move |g| {
                let f = flags(g)?;
                let x = g(Cell::L1(lane))?;
                Some(
                    f[PUSH].clone() * (x.clone() - g(Cell::Pre(a))?)
                        + f[TPUSH].clone() * (x - g(Cell::Pre(b))?),
                )
            }),
        ));
    }
    for j in 0..NREG {
        v.push((format!("next {j}"), ex(move |g| next_state(g, j))));
    }
    v
}

/// Body degree of each core gate row (0 for a row with no constraint), from a
/// scratch constraint system.
pub fn core_degrees(layout: &Layout, by_gate: &[Vec<usize>]) -> Vec<usize> {
    let mut cs = ConstraintSystem::<F>::default();
    let cols: Vec<Column<Advice>> = (0..layout.w).map(|_| cs.advice_column()).collect();
    let mut out = Vec::new();
    for (gi, idx) in by_gate.iter().enumerate() {
        if idx.is_empty() {
            out.push(0);
            continue;
        }
        let o = layout.gates[gi];
        let mut d = 0;
        cs.create_gate("deg", |m| {
            let cons = core_constraints();
            let mut v = Vec::new();
            for &ci in idx {
                let mut get = |c: Cell| {
                    layout
                        .find(c, o)
                        .map(|(col, rot)| m.query_advice(cols[col], Rotation(rot)))
                };
                let e = (cons[ci].1)(&mut get).expect("hosted");
                d = d.max(e.degree());
                v.push(e);
            }
            crate::gate(v)
        });
        out.push(d);
    }
    out
}

/// `post_j - next_j(pre, lanes, flags)`.
fn next_state(g: &mut G<'_>, j: usize) -> Option<Expression<F>> {
    let f = flags(g)?;
    let en = sum(&f);
    let off = k(1) - en.clone();
    let p = g(Cell::Pre(j))?;
    let q = g(Cell::Post(j))?;
    let fc = |r: usize| f[r].clone();
    let keep = |rs: &[usize]| k(1) - rs.iter().map(|&r| fc(r)).fold(k(0), |a, b| a + b);
    let next = match j {
        CTL_TAG => {
            let ovf = g(Cell::Ovf)?;
            off * p
                + fc(PUSH) * k(T_REWRITE)
                + fc(POP) * k(T_PUSH)
                + fc(REWRITE) * k(T_TPUSH)
                + fc(ARITH) * (k(T_TPOP) + ovf * (k(T_ERR) - k(T_TPOP)))
                + fc(TPUSH) * k(T_ARITH)
                + fc(TPOP) * k(T_POP)
        }
        CTL_A => {
            let ovf = g(Cell::Ovf)?;
            keep(&[ARITH]) * p.clone()
                + fc(ARITH) * ((k(1) - ovf.clone()) * g(Cell::L0(4))? + ovf * p)
        }
        FUEL => p - en,
        WORK => p + en,
        FCOUNT => p + fc(PUSH) - fc(POP),
        TCOUNT => p + fc(TPUSH) - fc(TPOP),
        LEN => p + fc(PUSH) + fc(TPUSH),
        FP => keep(&[PUSH, POP]) * p + fc(PUSH) * g(Cell::L1(1))? + fc(POP) * g(Cell::L1(4))?,
        TP => keep(&[TPUSH, TPOP]) * p + fc(TPUSH) * g(Cell::L1(1))? + fc(TPOP) * g(Cell::L1(4))?,
        TOP0 => {
            keep(&[PUSH, POP, REWRITE]) * p
                + fc(PUSH) * g(Cell::Pre(CTL_A))?
                + fc(POP) * g(Cell::L1(2))?
                + fc(REWRITE) * g(Cell::Pre(CTL_B))?
        }
        TOP1 => {
            keep(&[PUSH, POP]) * p + fc(PUSH) * g(Cell::Pre(CTL_B))? + fc(POP) * g(Cell::L1(3))?
        }
        TASK0 => {
            keep(&[TPUSH, TPOP]) * p + fc(TPUSH) * g(Cell::Pre(CTL_A))? + fc(TPOP) * g(Cell::L1(2))?
        }
        TASK1 => {
            keep(&[TPUSH, TPOP]) * p + fc(TPUSH) * g(Cell::Pre(CTL_C))? + fc(TPOP) * g(Cell::L1(3))?
        }
        H0 => keep(&[TPOP]) * p + fc(TPOP) * g(Cell::Pre(TASK0))?,
        H1 => keep(&[TPOP]) * p + fc(TPOP) * g(Cell::Pre(TASK1))?,
        R0 => keep(&[ARITH]) * p + fc(ARITH) * g(Cell::L0(4))?,
        R1 => keep(&[POP]) * p + fc(POP) * g(Cell::Pre(TOP0))?,
        V0 => off * p + en * g(Cell::L0(4))?,
        V1 => off * p + en * g(Cell::L1(2))?,
        V2 => off * p + en * g(Cell::L1(3))?,
        _ => p,
    };
    Some(q - next)
}

/// Configuration parameters of [`RfLeaf`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RfParams {
    pub arch: Arch,
    /// Configure the HASH chip.
    pub hash: bool,
    /// Sensitivity runs: emit every core constraint `pad` times (scaled copies);
    /// 0 and 1 mean once.
    pub pad: usize,
    /// Key-shape stand-in for a slim HASH chip ([`crate::hashchip::SlimHashShape`]);
    /// node measurement only, not satisfiable as a hash.
    pub slim_hash: bool,
    /// Node-cost probe: this many extra lookup arguments (key shape only).
    pub extra_lookups: usize,
    /// The Poseidon-only HASH chip of [`crate::poseidon`] (E7) instead of the
    /// stdlib composition.
    pub pos_hash: bool,
    /// RANGE in running-sum mode: one limb lookup instead of four (E7).
    pub range1: bool,
    /// Node-cost probe: this many extra selector columns, column `i` enabled on
    /// core rows `i mod 7 + 7 t` with a trivially true gate (key shape only).
    pub extra_selectors: usize,
    /// E8 cut 1: the HASH chip's three absorb selectors in one code column.
    pub one_abs: bool,
    /// E8 cut 2: the core gate selectors and `rf count0` packed into code
    /// columns ([`crate::vsel::pack`]); the gate on the DEC row uses `q_dec`.
    pub pack_core: bool,
    /// E8: M3 wired for real ([`crate::m3`]): the aligned MEM segment with its
    /// active prefix, records, segment root, the two top-tree walks and the
    /// Poseidon state digests over the register cells and `mem_root`. Needs
    /// `pos_hash`; replaces the placeholder digest and the spike MEM.
    pub m3: bool,
}

#[derive(Clone, Debug)]
pub struct DigConfig {
    pub q: [Selector; 3],
}

#[derive(Clone, Debug)]
pub struct RfConfig {
    pub params: RfParams,
    pub layout: Layout,
    pub cols: Vec<Column<Advice>>,
    pub bus: BusConfig,
    pub range: RangeConfig,
    pub alu: AluConfig,
    pub mem: Option<MemConfig>,
    pub seg: Option<SegMemConfig>,
    pub walk: Option<WalkConfig>,
    pub dec: DecConfig,
    pub gate_sels: Vec<Option<VSel>>,
    pub q_dec: Selector,
    pub dec_off: usize,
    pub q_cnt0: VSel,
    /// Inline digest gate (Wide-inline only) and its slot row.
    pub q_dig_inline: Option<(Selector, usize)>,
    pub dig: Option<DigConfig>,
    pub hash: Option<HashConfig>,
    pub slim: Option<SlimHashShape>,
    pub pos: Option<PoseidonCfg>,
    pub probe_sels: Vec<Selector>,
    pub committed: Column<Instance>,
    pub instance: Column<Instance>,
}

/// One slot's witness. Slot `s` (the last) holds the post-state with no flags.
#[derive(Clone, Copy, Debug)]
pub struct SlotW {
    pub pre: Regs32,
    pub post: Regs32,
    pub flags: [u64; NRULE],
    pub ovf: u64,
    pub l0: Tuple,
    pub l1: Tuple,
}

#[derive(Clone, Debug)]
pub struct RfWitness {
    /// `s + 1` slots.
    pub slots: Vec<SlotW>,
    pub alu: Vec<AluOp>,
    pub mem_base: u128,
    pub mem: Vec<MemCell>,
    /// Negative tests: `(slot, row, column, value)` written over the honest cell.
    pub overrides: Vec<(usize, usize, usize, F)>,
    /// The M3 part (leaves with `params.m3`).
    pub m3: Option<M3W>,
}

fn zero() -> Tuple {
    [F::from(0u64); 5]
}

impl RfWitness {
    pub fn from_trace(t: &Trace32, s: usize) -> Self {
        assert!(t.steps.len() <= s);
        let mut slots = Vec::with_capacity(s + 1);
        let mut alu = Vec::with_capacity(s);
        for st in &t.steps {
            let mut flags = [0; NRULE];
            flags[st.rule] = 1;
            let l1 = st.mem.map_or(zero(), |m| {
                let (a, c) = m.parts();
                tuple(CHIP_MEM, fu(a), fu(c[0]), fu(c[1]), fu(c[2]))
            });
            slots.push(SlotW {
                pre: st.pre,
                post: st.post,
                flags,
                ovf: st
                    .alu
                    .map_or(0, |a| if st.rule == ARITH { a.flag } else { 0 }),
                l0: st.alu.map_or(zero(), |a| a.tuple()),
                l1,
            });
            alu.push(st.alu.unwrap_or(AluOp::eval(OP_ADD, 0, 0)));
        }
        let post = t.post();
        while slots.len() <= s {
            slots.push(SlotW {
                pre: post,
                post,
                flags: [0; NRULE],
                ovf: 0,
                l0: zero(),
                l1: zero(),
            });
        }
        while alu.len() < s {
            alu.push(AluOp::eval(OP_ADD, 0, 0));
        }
        Self {
            slots,
            alu,
            mem_base: t.pre[LEN],
            mem: t.appended.clone(),
            overrides: Vec::new(),
            m3: None,
        }
    }

    /// As [`Self::from_trace`] with an honest M3 witness: the old tree has
    /// siblings `sibs` on the path of segment `pre.len / 8`.
    pub fn from_trace_m3(t: &Trace32, s: usize, sibs: [F; crate::m3::TOP]) -> Self {
        let mut w = Self::from_trace(t, s);
        let cells: Vec<[F; 3]> = t
            .appended
            .iter()
            .map(|c| [fu(c[0]), fu(c[1]), fu(c[2])])
            .collect();
        w.m3 = Some(M3W::honest(t.pre[LEN], t.post()[LEN], &cells, sibs));
        w
    }

    /// Digested words of a register file.
    pub fn words(x: &Regs32) -> Vec<F> {
        x.iter().take(NDIG).map(|&v| fu(v)).collect()
    }

    pub fn s(&self) -> usize {
        self.slots.len() - 1
    }

    pub fn steps(&self) -> u64 {
        self.slots[..self.s()]
            .iter()
            .map(|x| x.flags.iter().sum::<u64>())
            .sum()
    }

    pub fn instance(&self) -> Vec<F> {
        let (pre_d, post_d) = match &self.m3 {
            None => (
                rf_digest(&self.slots[0].pre),
                rf_digest(&self.slots[self.s()].pre),
            ),
            Some(m) => {
                let pre = Self::words(&self.slots[0].pre);
                let mut post = Self::words(&self.slots[self.s()].pre);
                post[LEN] = m.len_post;
                (
                    m.digest(0, &pre, m.pre_root),
                    m.digest(1, &post, m.new_root()),
                )
            }
        };
        vec![
            F::from(0u64),
            F::from(0u64),
            pre_d,
            post_d,
            F::from(0u64),
            F::from(0u64),
            F::from(self.steps()),
        ]
    }

    /// Set register `j` of the state after step `i` (both its post cell and the
    /// next slot's pre cell, so no copy constraint is broken).
    pub fn set_post(&mut self, i: usize, j: usize, v: u128) {
        self.slots[i].post[j] = v;
        self.slots[i + 1].pre[j] = v;
        if i + 1 == self.s() {
            self.slots[i + 1].post[j] = v;
        }
    }
}

/// The E5/E6 leaf: `s` slots, `n_hash2` bus-served 2-input hashes and one extra
/// hash per entry of `hash_extra` (its input count).
#[derive(Clone, Debug)]
pub struct RfLeaf {
    pub params: RfParams,
    pub s: usize,
    pub n_hash2: usize,
    pub hash_extra: Vec<usize>,
    /// Permutations of round constants to fill when `params.slim_hash`.
    pub slim_perms: usize,
    pub w: Option<RfWitness>,
}

impl RfLeaf {
    pub fn new(params: RfParams, s: usize, w: Option<RfWitness>) -> Self {
        assert!(s >= 1);
        if let Some(w) = &w {
            assert_eq!(w.s(), s);
        }
        Self {
            params,
            s,
            n_hash2: 0,
            hash_extra: Vec::new(),
            slim_perms: 0,
            w,
        }
    }

    pub fn with_hashes(mut self, n2: usize, extra: Vec<usize>) -> Self {
        assert!(self.params.hash || self.params.pos_hash || (n2 == 0 && extra.is_empty()));
        self.n_hash2 = n2;
        self.hash_extra = extra;
        self
    }

    /// Rows of the main region (tables and HASH regions excluded).
    pub fn region_rows(params: RfParams, s: usize, n_hash2: usize) -> usize {
        let l = Layout::new(params.arch);
        let dig = if params.arch == Arch::WideInline {
            0
        } else {
            6
        };
        let alu = if params.range1 {
            crate::range::CELLS
        } else {
            rows_per_value(RC)
        };
        let hash = if params.pos_hash {
            n_hash2 * crate::poseidon::ROWS_PER_PERM
        } else {
            n_hash2
        };
        l.h * (s + 1) + s * alu + s + dig + hash
    }
}

impl Circuit<F> for RfLeaf {
    type Config = RfConfig;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = RfParams;

    fn without_witnesses(&self) -> Self {
        Self {
            w: None,
            ..self.clone()
        }
    }

    fn params(&self) -> RfParams {
        self.params
    }

    fn configure(_meta: &mut ConstraintSystem<F>) -> RfConfig {
        unreachable!("RfLeaf is configured with parameters")
    }

    fn configure_with_params(meta: &mut ConstraintSystem<F>, params: RfParams) -> RfConfig {
        let layout = Layout::new(params.arch);
        let cols: Vec<Column<Advice>> = (0..layout.w).map(|_| meta.advice_column()).collect();
        let committed = meta.instance_column();
        let instance = meta.instance_column();
        let c12 = Cols {
            a: std::array::from_fn(|i| cols[i]),
        };
        let bus = BusConfig::configure(meta, c12);
        let range = if params.range1 {
            RangeConfig::configure_running(meta, c12)
        } else {
            RangeConfig::configure(meta, c12, RC)
        };
        let alu = AluConfig::configure(meta, bus, range.clone());
        assert!(!params.m3 || (params.pos_hash && params.arch != Arch::WideInline));
        let (mem, seg) = if params.m3 {
            (None, Some(SegMemConfig::configure(meta, bus)))
        } else {
            (Some(MemConfig::configure(meta, bus)), None)
        };
        // Node-cost probe: extra lookup arguments into the RANGE table on the ALU rows
        // (columns 0.. hold small tuple fields there).
        for i in 0..params.extra_lookups {
            let (q, t, a) = (range.q_rng, range.table, cols[i]);
            meta.lookup("extra probe", |m| {
                let q = m.query_selector(q);
                vec![(q * m.query_advice(a, Rotation::cur()), t)]
            });
        }

        // Core gates: host every constraint, then build one gate per gate row.
        let cons = core_constraints();
        let mut by_gate: Vec<Vec<usize>> = vec![Vec::new(); layout.gates.len()];
        for (ci, (name, e)) in cons.iter().enumerate() {
            let gi = layout.host(e).unwrap_or_else(|| {
                panic!("constraint `{name}` has no host gate in {:?}", layout.arch)
            });
            by_gate[gi].push(ci);
        }
        let pad = params.pad.max(1);
        // DEC: `(en * ctl_tag, rule id)` on the first gate row that sees them.
        let dec_ex = ex(|g| {
            let f = flags(g)?;
            Some(g(Cell::Pre(CTL_TAG))? * sum(&f))
        });
        let dgi = layout.host(&dec_ex).expect("DEC host");
        let dec_off = layout.gates[dgi];
        let q_dec = meta.complex_selector();
        // Selectors of the core gates and of `rf count0` (the last entry).
        let mut vs: Vec<Option<VSel>> = Vec::new();
        if params.pack_core {
            let degs = core_degrees(&layout, &by_gate);
            let mut items = Vec::new();
            for (gi, idx) in by_gate.iter().enumerate() {
                if !idx.is_empty() && gi != dgi {
                    items.push((gi, degs[gi]));
                }
            }
            let mut d: Vec<usize> = items.iter().map(|x| x.1).collect();
            d.push(1);
            let packed = pack(meta, &d);
            vs = vec![None; by_gate.len() + 1];
            for (k, (gi, _)) in items.iter().enumerate() {
                vs[*gi] = Some(packed[k]);
            }
            if !by_gate[dgi].is_empty() {
                vs[dgi] = Some(VSel::Simple(q_dec));
            }
            vs[by_gate.len()] = Some(packed[items.len()]);
        } else {
            for idx in &by_gate {
                vs.push((!idx.is_empty()).then(|| VSel::Simple(meta.selector())));
            }
            vs.push(Some(VSel::Simple(meta.selector())));
        }
        let q_cnt0 = vs.pop().unwrap().unwrap();
        let mut gate_sels = Vec::new();
        for (gi, idx) in by_gate.iter().enumerate() {
            let Some(sel) = vs[gi] else {
                gate_sels.push(None);
                continue;
            };
            let o = layout.gates[gi];
            let (lay, idx, colv) = (layout.clone(), idx.clone(), cols.clone());
            meta.create_gate("rf core", move |m| {
                let cons = core_constraints();
                let q = sel.expr(m);
                let mut out = Vec::new();
                for &ci in &idx {
                    let mut get = |c: Cell| {
                        lay.find(c, o)
                            .map(|(col, rot)| m.query_advice(colv[col], Rotation(rot)))
                    };
                    let e = (cons[ci].1)(&mut get).expect("hosted");
                    for p in 0..pad {
                        out.push(q.clone() * e.clone() * k(p as u64 + 1));
                    }
                }
                crate::gate(out)
            });
            gate_sels.push(Some(sel));
        }

        let lay = layout.clone();
        let dec = DecConfig::configure(meta, |m| {
            let q = m.query_selector(q_dec);
            let mut get = |c: Cell| {
                lay.find(c, dec_off)
                    .map(|(col, rot)| m.query_advice(cols[col], Rotation(rot)))
            };
            let f = flags(&mut get).unwrap();
            let tag = get(Cell::Pre(CTL_TAG)).unwrap();
            let rule = f
                .iter()
                .enumerate()
                .fold(k(0), |a, (r, x)| a + x.clone() * k(r as u64 + 1));
            (q.clone() * sum(&f) * tag, q * rule)
        });

        let (cnt_col, _) = layout
            .find(Cell::Cnt, layout.place[layout.canon(Cell::Cnt)].1)
            .unwrap();
        meta.create_gate("rf count0", |m| {
            let q = q_cnt0.expr(m);
            crate::gate(vec![q * m.query_advice(cols[cnt_col], Rotation::cur())])
        });

        // Digest.
        let mut q_dig_inline = None;
        let mut dig = None;
        if params.m3 {
            // The Poseidon digests of `crate::m3` replace the placeholder.
        } else if params.arch == Arch::WideInline {
            let sel = meta.selector();
            let o = layout.place[layout.canon(Cell::Dig)].1;
            let (lay, colv) = (layout.clone(), cols.clone());
            meta.create_gate("rf digest inline", move |m| {
                let q = m.query_selector(sel);
                let mut get = |c: Cell| {
                    lay.find(c, o)
                        .map(|(col, rot)| m.query_advice(colv[col], Rotation(rot)))
                };
                let mut acc = k(0);
                for j in 0..NDIG {
                    acc = acc * Expression::Constant(lambda()) + get(Cell::Pre(j)).unwrap();
                }
                crate::gate(vec![q * (get(Cell::Dig).unwrap() - acc)])
            });
            q_dig_inline = Some((sel, o));
        } else {
            let q: [Selector; 3] = std::array::from_fn(|_| meta.selector());
            let a = c12.a;
            let lam = |e: usize| Expression::Constant(lambda().pow_vartime([e as u64]));
            meta.create_gate("rf digest", move |m| {
                let mut out = Vec::new();
                for (t, &sel) in q.iter().enumerate() {
                    let qq = m.query_selector(sel);
                    let n = if t == 2 { 9 } else { 10 };
                    let mut acc = if t == 0 {
                        k(0)
                    } else {
                        m.query_advice(a[10], Rotation::prev()) * lam(n)
                    };
                    for i in 0..n {
                        acc = acc + m.query_advice(a[i], Rotation::cur()) * lam(n - 1 - i);
                    }
                    out.push(qq * (m.query_advice(a[10], Rotation::cur()) - acc));
                }
                crate::gate(out)
            });
            dig = Some(DigConfig { q });
        }

        // Equality.
        let mut eq: BTreeSet<usize> = BTreeSet::new();
        eq.insert(1); // MEM address
        eq.insert(cnt_col);
        let mut seen: BTreeMap<Cell, usize> = BTreeMap::new();
        for (cell, _, col) in &layout.place {
            if let Some(&c0) = seen.get(cell) {
                eq.insert(c0);
                eq.insert(*col);
            } else {
                seen.insert(*cell, *col);
            }
            match cell {
                Cell::Pre(j) if *j == LEN => {
                    eq.insert(*col);
                }
                Cell::Pre(j) if params.arch != Arch::WideInline && *j < NDIG => {
                    eq.insert(*col);
                }
                Cell::Post(_) | Cell::Dig => {
                    eq.insert(*col);
                }
                _ => {}
            }
        }
        if layout.arch == Arch::Narrow {
            for (cell, _, col) in &layout.place {
                if let Cell::Pre(_) = cell {
                    eq.insert(*col);
                }
            }
        }
        if dig.is_some() {
            eq.extend(0..=10);
        }
        for c in eq {
            meta.enable_equality(cols[c]);
        }
        meta.enable_equality(instance);

        let hash = params
            .hash
            .then(|| HashConfig::configure(meta, &cols, [committed, instance]));
        let slim = params
            .slim_hash
            .then(|| SlimHashShape::configure(meta, &cols));
        let pos = params
            .pos_hash
            .then(|| PoseidonCfg::configure(meta, bus, params.one_abs));
        let walk = params.m3.then(|| WalkConfig::configure(meta, c12));
        let probe_sels: Vec<Selector> = (0..params.extra_selectors)
            .map(|_| {
                let q = meta.selector();
                let c0 = cols[0];
                meta.create_gate("selector probe", move |m| {
                    let q = m.query_selector(q);
                    let x = m.query_advice(c0, Rotation::cur());
                    crate::gate(vec![q * (x.clone() - x)])
                });
                q
            })
            .collect();

        RfConfig {
            params,
            layout,
            cols,
            bus,
            range,
            alu,
            mem,
            seg,
            walk,
            dec,
            gate_sels,
            q_dec,
            dec_off,
            q_cnt0,
            q_dig_inline,
            dig,
            hash,
            slim,
            pos,
            probe_sels,
            committed,
            instance,
        }
    }

    fn synthesize(&self, cfg: RfConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
        cfg.range.load(&mut l)?;
        cfg.dec.load(&mut l, &DEC_ROWS)?;
        let s = self.s;
        let lay = &cfg.layout;
        let h = lay.h;
        let w = self.w.as_ref();
        let kn = |f: F| w.map_or(Value::unknown(), |_| Value::known(f));
        type Cells = (
            AssignedCell<F, F>,
            AssignedCell<F, F>,
            AssignedCell<F, F>,
            Vec<[AssignedCell<F, F>; 3]>,
        );
        let (pre_d, post_d, steps, hash_rows): Cells = l.assign_region(
            || "rf leaf",
            |mut r| {
                let mut cells: BTreeMap<(usize, usize), AssignedCell<F, F>> = BTreeMap::new();
                let mut used: BTreeSet<(usize, usize)> = BTreeSet::new();
                let mut cnt = 0u64;
                for i in 0..=s {
                    let sw = w.map(|w| w.slots[i]);
                    for (pi, &(cell, row, col)) in lay.place.iter().enumerate() {
                        let v = sw.map(|x| match cell {
                            Cell::Pre(j) => fu(x.pre[j]),
                            Cell::Post(j) => fu(x.post[j]),
                            Cell::Flag(f) => F::from(x.flags[f]),
                            Cell::L0(t) => x.l0[t],
                            Cell::L1(t) => x.l1[t],
                            Cell::Ovf => F::from(x.ovf),
                            Cell::Cnt => F::from(cnt),
                            Cell::Dig if i == 0 || i == s => rf_digest(&x.pre),
                            Cell::Dig => F::from(0u64),
                            _ => unreachable!("not a placed cell"),
                        });
                        let v = w
                            .and_then(|w| {
                                w.overrides
                                    .iter()
                                    .find(|o| (o.0, o.1, o.2) == (i, row, col))
                                    .map(|o| o.3)
                            })
                            .or(v);
                        let rr = i * h + row;
                        let c = r.assign_advice(|| "rf", cfg.cols[col], rr, || kn_or(v, w))?;
                        cells.insert((i, pi), c);
                        used.insert((rr, col));
                    }
                    if i < s {
                        for (gi, sel) in cfg.gate_sels.iter().enumerate() {
                            if let Some(sel) = sel {
                                sel.enable(&mut r, i * h + lay.gates[gi])?;
                            }
                        }
                        cfg.q_dec.enable(&mut r, i * h + cfg.dec_off)?;
                        r.assign_fixed(
                            || "q_req",
                            cfg.bus.q_req,
                            i * h + lay.req_row,
                            || Value::known(F::from(1u64)),
                        )?;
                        cnt += sw.map_or(0, |x| x.flags.iter().sum::<u64>());
                    }
                }
                for rr in 0..h * (s + 1) {
                    for col in 0..lay.w {
                        if !used.contains(&(rr, col)) {
                            r.assign_advice(
                                || "rf pad",
                                cfg.cols[col],
                                rr,
                                || Value::known(F::from(0u64)),
                            )?;
                        }
                    }
                }
                // Copies: replicas to the canonical cell; post to the next pre.
                for i in 0..=s {
                    for (pi, &(cell, _, _)) in lay.place.iter().enumerate() {
                        let c0 = lay.canon(cell);
                        if c0 != pi {
                            r.constrain_equal(cells[&(i, pi)].cell(), cells[&(i, c0)].cell())?;
                        }
                        if let Cell::Post(j) = cell {
                            if c0 == pi && i < s {
                                let nx = lay.canon(Cell::Pre(j));
                                r.constrain_equal(
                                    cells[&(i, pi)].cell(),
                                    cells[&(i + 1, nx)].cell(),
                                )?;
                            }
                        }
                    }
                }
                for (pi, q) in cfg.probe_sels.iter().enumerate() {
                    // Distinct rows per probe column, so no two commitments are equal.
                    for rr in (pi % 7..h * (s + 1)).step_by(7) {
                        q.enable(&mut r, rr)?;
                    }
                }
                let cnt_row = lay.place[lay.canon(Cell::Cnt)].1;
                cfg.q_cnt0.enable(&mut r, cnt_row)?;
                let pre_cell = |i: usize, j: usize| cells[&(i, lay.canon(Cell::Pre(j)))].clone();
                let steps = cells[&(s, lay.canon(Cell::Cnt))].clone();

                let mut o = h * (s + 1);
                for i in 0..s {
                    let op = w.map_or(Value::unknown(), |w| Value::known(w.alu[i]));
                    o += cfg.alu.assign(&mut r, o, op)?;
                }
                let base = w.map_or(Value::unknown(), |w| Value::known(fu(w.mem_base)));
                if let (Some(sc), Some(wk), Some(pc)) = (&cfg.seg, &cfg.walk, &cfg.pos) {
                    // M3: the aligned segment, its records and root, the walks and
                    // the Poseidon digests.
                    let m = w.map(|w| w.m3.as_ref().expect("M3 witness"));
                    let cells: Vec<Value<[F; 3]>> = (0..SEG)
                        .map(|i| m.map_or(Value::unknown(), |m| Value::known(m.cells[i])))
                        .collect();
                    let act: Vec<Value<F>> = (0..SEG)
                        .map(|i| m.map_or(Value::unknown(), |m| Value::known(m.act[i])))
                        .collect();
                    let len_end = m.map_or(Value::unknown(), |m| Value::known(m.len_end));
                    let segc = sc.assign(&mut r, o, base, len_end, &cells, &act)?;
                    r.constrain_equal(segc.addr0.cell(), pre_cell(0, LEN).cell())?;
                    r.constrain_equal(segc.len_end.cell(), pre_cell(s, LEN).cell())?;
                    o += SEG;
                    let pre_w: Vec<_> = (0..NDIG).map(|j| pre_cell(0, j)).collect();
                    let post_w: Vec<_> = (0..NDIG).map(|j| pre_cell(s, j)).collect();
                    let out = crate::m3::assign(
                        pc,
                        wk,
                        &mut r,
                        &mut o,
                        &segc,
                        &pre_cell(0, LEN),
                        &pre_w,
                        &post_w,
                        LEN,
                        m,
                    )?;
                    return Ok((out.pre_d, out.post_d, steps, Vec::new()));
                }
                let mem: Vec<_> = match w {
                    Some(w) => w
                        .mem
                        .iter()
                        .map(|c| Value::known([fu(c[0]), fu(c[1]), fu(c[2])]))
                        .collect(),
                    None => vec![Value::unknown(); s],
                };
                let first = cfg.mem.as_ref().unwrap().assign(&mut r, o, s, base, &mem)?;
                r.constrain_equal(first.cell(), pre_cell(0, LEN).cell())?;
                o += s;

                let (pre_d, post_d) = if let Some((sel, row)) = cfg.q_dig_inline {
                    sel.enable(&mut r, row)?;
                    sel.enable(&mut r, s * h + row)?;
                    let d = lay.canon(Cell::Dig);
                    (cells[&(0, d)].clone(), cells[&(s, d)].clone())
                } else {
                    let dg = cfg.dig.as_ref().unwrap();
                    let a = &cfg.cols;
                    let mut out = Vec::new();
                    for i in [0, s] {
                        let mut acc = F::from(0u64);
                        let mut last = None;
                        for t in 0..3 {
                            dg.q[t].enable(&mut r, o + t)?;
                            let n = if t == 2 { 9 } else { 10 };
                            for c in 0..10 {
                                let rr = o + t;
                                if c < n {
                                    let j = 10 * t + c;
                                    let src = pre_cell(i, j);
                                    let x = src.copy_advice(|| "dig", &mut r, a[c], rr)?;
                                    acc = acc * lambda()
                                        + w.map_or(F::from(0u64), |w| fu(w.slots[i].pre[j]));
                                    let _ = x;
                                } else {
                                    r.assign_advice(
                                        || "dig pad",
                                        a[c],
                                        rr,
                                        || Value::known(F::from(0u64)),
                                    )?;
                                }
                            }
                            last = Some(r.assign_advice(|| "acc", a[10], o + t, || kn(acc))?);
                        }
                        out.push(last.unwrap());
                        o += 3;
                    }
                    (out[0].clone(), out[1].clone())
                };

                let mut hash_rows = Vec::new();
                if let Some(pc) = &cfg.pos {
                    // Poseidon-only HASH: bus-served 2-input hashes, then the extra
                    // sponges, all in this region.
                    let (tm, mut perm) = (Tamper::default(), 0);
                    for t in 0..self.n_hash2 {
                        let ins = [
                            HIn::Val(Value::known(F::from(t as u64 + 1))),
                            HIn::Val(Value::known(F::from(t as u64 + 2))),
                        ];
                        o += pc.sponge(&mut r, o, &ins, true, &tm, &mut perm)?.1;
                    }
                    for (t, &n) in self.hash_extra.iter().enumerate() {
                        let ins: Vec<HIn> = (0..n)
                            .map(|i| HIn::Val(Value::known(F::from((100 * t + i) as u64))))
                            .collect();
                        o += pc.sponge(&mut r, o, &ins, false, &tm, &mut perm)?.1;
                    }
                    return Ok((pre_d, post_d, steps, hash_rows));
                }
                for t in 0..self.n_hash2 {
                    let (x, y) = (F::from(t as u64 + 1), F::from(t as u64 + 2));
                    let tup = hash_tuple(0, x, y);
                    let c = cfg.bus.provide(&mut r, o, Value::known(tup))?;
                    hash_rows.push([c[2].clone(), c[3].clone(), c[4].clone()]);
                    o += 1;
                }
                Ok((pre_d, post_d, steps, hash_rows))
            },
        )?;
        if let Some(hc) = &cfg.hash {
            for [x, y, out] in &hash_rows {
                let o2 = hash_cells(hc, &mut l, &[x.clone(), y.clone()])?;
                l.assign_region(
                    || "hash tie",
                    |mut r| r.constrain_equal(o2.cell(), out.cell()),
                )?;
            }
            for (t, &n) in self.hash_extra.iter().enumerate() {
                let vals: Vec<Value<F>> = (0..n)
                    .map(|i| Value::known(F::from((100 * t + i) as u64)))
                    .collect();
                hash_free(hc, &cfg.bus, &mut l, &vals)?;
            }
        }
        if let Some(sh) = &cfg.slim {
            sh.fill(&mut l, 20 * self.slim_perms)?;
        }
        l.constrain_instance(pre_d.cell(), cfg.instance, 2)?;
        l.constrain_instance(post_d.cell(), cfg.instance, 3)?;
        l.constrain_instance(steps.cell(), cfg.instance, 6)?;
        Ok(())
    }
}

fn kn_or(v: Option<F>, w: Option<&RfWitness>) -> Value<F> {
    match (v, w) {
        (Some(v), Some(_)) => Value::known(v),
        _ => Value::unknown(),
    }
}

/// MockProver result of a leaf on its witness's own statement.
pub fn rf_mock(leaf: &RfLeaf) -> Result<(), String> {
    let w = leaf.w.as_ref().expect("witness");
    let p = midnight_proofs::dev::MockProver::run(leaf, vec![vec![], w.instance()])
        .map_err(|e| format!("{e:?}"))?;
    p.verify().map_err(|e| {
        let mut v: Vec<String> = e.iter().take(4).map(|f| format!("{f}")).collect();
        v.sort();
        v.join(" | ")
    })
}

/// Constraint-system query rotations, for the unit-rotation guard.
pub fn rotations(params: RfParams) -> Vec<i32> {
    let mut cs = ConstraintSystem::<F>::default();
    RfLeaf::configure_with_params(&mut cs, params);
    cs.advice_queries()
        .iter()
        .map(|q| q.1.0)
        .chain(cs.fixed_queries().iter().map(|q| q.1.0))
        .chain(cs.instance_queries().iter().map(|q| q.1.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARCHS: [Arch; 4] = [Arch::Wide, Arch::WideInline, Arch::Narrow, Arch::Staggered];

    fn p(arch: Arch) -> RfParams {
        RfParams {
            arch,
            ..Default::default()
        }
    }

    fn honest(s: usize) -> RfWitness {
        RfWitness::from_trace(&run32(genesis(), s), s)
    }

    fn leaf(arch: Arch, w: RfWitness) -> RfLeaf {
        RfLeaf::new(p(arch), w.s(), Some(w))
    }

    #[test]
    fn rotations_are_unit() {
        for arch in ARCHS {
            for (hash, slim_hash, pos_hash, range1) in [
                (false, false, false, false),
                (true, false, false, false),
                (false, true, false, false),
                (false, false, true, false),
                (false, false, true, true),
                (false, false, false, true),
            ] {
                for r in rotations(RfParams {
                    arch,
                    hash,
                    pad: 1,
                    slim_hash,
                    extra_lookups: 0,
                    pos_hash,
                    range1,
                    extra_selectors: 0,
                    ..Default::default()
                }) {
                    assert!((-1..=1).contains(&r), "{arch:?} hash={hash}: rotation {r}");
                }
            }
        }
    }

    #[test]
    fn honest_leaves_pass() {
        for arch in ARCHS {
            for s in [1, 6, 8] {
                rf_mock(&leaf(arch, honest(s))).unwrap_or_else(|e| panic!("{arch:?} s={s}: {e}"));
            }
            // 4 steps in 7 slots.
            let w = RfWitness::from_trace(&run32(genesis(), 4), 7);
            rf_mock(&leaf(arch, w)).unwrap_or_else(|e| panic!("{arch:?} partial: {e}"));
        }
    }

    /// Frame condition and next state, word by word: the last step (a `Pop`) of a
    /// 6-step leaf writes a wrong value into register `j`; every layout refuses it
    /// for each of the 32 words.
    #[test]
    fn every_register_word_is_checked() {
        for arch in ARCHS {
            let base = honest(6);
            assert_eq!(base.slots[5].flags[POP], 1);
            for j in 0..NREG {
                let mut w = base.clone();
                let v = w.slots[5].post[j];
                w.set_post(5, j, v + 1);
                let e = rf_mock(&leaf(arch, w)).expect_err(&format!("{arch:?} word {j}"));
                assert!(e.contains("rf core"), "{arch:?} word {j}: {e}");
            }
        }
    }

    /// D7: a push must store the core's `fp` as the link, not a free value.
    #[test]
    fn push_link_is_the_core_fp() {
        for arch in ARCHS {
            // Steps 6.. start a second cycle: step 6 is a Push with fp = 0; use a leaf
            // where a pushed entry is later popped (steps 0 and 5).
            let mut w = honest(6);
            w.slots[0].l1[4] = F::from(9u64);
            w.mem[0][2] = 9;
            // The pop at step 5 then loads link 9 into fp.
            w.slots[5].l1[4] = F::from(9u64);
            w.set_post(5, FP, 9);
            assert!(rf_mock(&leaf(arch, w)).is_err(), "{arch:?}");
        }
    }

    #[test]
    fn enable_is_a_prefix() {
        for arch in ARCHS {
            let t = run32(genesis(), 1);
            let mut w = RfWitness::from_trace(&t, 2);
            let s0 = w.slots[0];
            w.slots[1] = SlotW { ..s0 };
            w.slots[0] = SlotW {
                post: s0.pre,
                flags: [0; NRULE],
                ovf: 0,
                l0: zero(),
                l1: zero(),
                ..s0
            };
            w.alu = vec![AluOp::eval(OP_ADD, 0, 0), w.alu[0]];
            assert!(rf_mock(&leaf(arch, w)).is_err(), "{arch:?}");
        }
    }

    fn cell_of(arch: Arch, c: Cell, n: usize) -> (usize, usize) {
        let l = Layout::new(arch);
        let v: Vec<_> = l.place.iter().filter(|p| p.0 == c).collect();
        (v[n].1, v[n].2)
    }

    /// B only: a broken post-to-pre link and a broken context replica are refused
    /// by the permutation argument.
    #[test]
    fn narrow_copies_are_checked() {
        // Post cell of slot 2 differs from the next pre cell (no gate is wrong).
        let mut w = honest(6);
        let (row, col) = cell_of(Arch::Narrow, Cell::Post(ENV), 0);
        w.overrides.push((2, row, col, F::from(77u64)));
        let e = rf_mock(&leaf(Arch::Narrow, w)).unwrap_err();
        assert!(e.contains("quality") || e.contains("ermutation"), "{e}");
        // A flag replica of slot 1 differs from the canonical flag row.
        let mut w = honest(6);
        let (row, col) = cell_of(Arch::Narrow, Cell::Flag(REWRITE), 2);
        w.overrides.push((1, row, col, F::from(0u64)));
        assert!(rf_mock(&leaf(Arch::Narrow, w)).is_err());
        // A routed operand replica differs from its register.
        let mut w = honest(6);
        let (row, col) = cell_of(Arch::Narrow, Cell::Pre(CTL_A), 1);
        w.overrides.push((0, row, col, F::from(5u64)));
        assert!(rf_mock(&leaf(Arch::Narrow, w)).is_err());
    }

    #[test]
    fn wrong_digest_statement_rejected() {
        for arch in ARCHS {
            let w = honest(3);
            let c = leaf(arch, w.clone());
            let mut inst = w.instance();
            inst[3] += F::from(1u64);
            let pr = midnight_proofs::dev::MockProver::run(&c, vec![vec![], inst]).unwrap();
            assert!(pr.verify().is_err(), "{arch:?}");
        }
    }

    /// E7: the Poseidon-only HASH chip in the leaf (bus-served 2-input hashes and
    /// 3- and 31-input sponges) and RANGE on one limb lookup.
    #[test]
    fn pos_hash_and_range1_leaves_pass() {
        for range1 in [false, true] {
            let params = RfParams {
                arch: Arch::Narrow,
                pos_hash: true,
                range1,
                ..Default::default()
            };
            let c = RfLeaf::new(params, 4, Some(honest(4))).with_hashes(3, vec![3, 31]);
            rf_mock(&c).unwrap_or_else(|e| panic!("range1={range1}: {e}"));
        }
        for arch in ARCHS {
            let params = RfParams {
                arch,
                range1: true,
                ..Default::default()
            };
            rf_mock(&RfLeaf::new(params, 6, Some(honest(6))))
                .unwrap_or_else(|e| panic!("{arch:?}: {e}"));
        }
    }

    #[test]
    fn hash_chip_serves_bus_rows() {
        let w = honest(4);
        let c = RfLeaf::new(
            RfParams {
                arch: Arch::Staggered,
                hash: true,
                pad: 1,
                slim_hash: false,
                extra_lookups: 0,
                ..Default::default()
            },
            4,
            Some(w),
        )
        .with_hashes(3, vec![4]);
        rf_mock(&c).unwrap();
    }

    // -----------------------------------------------------------------------
    // E8: the cuts and M3 wired for real.

    use crate::m3::{TOP, empty_top, h2, record_native, walk_native};

    fn m3_params() -> RfParams {
        RfParams {
            arch: Arch::Narrow,
            pos_hash: true,
            range1: true,
            one_abs: true,
            pack_core: true,
            m3: true,
            ..Default::default()
        }
    }

    /// Siblings of segment `idx` in an old tree whose segments below `idx` are
    /// filled (stand-in values) and whose segments from `idx` up are empty.
    fn sibs_for(idx: u64) -> [F; TOP] {
        std::array::from_fn(|l| {
            if (idx >> l) & 1 == 1 {
                F::from(1000 + l as u64)
            } else {
                empty_top(l)
            }
        })
    }

    fn m3_witness(base: u128, steps: usize, s: usize) -> RfWitness {
        let mut g = genesis();
        g[LEN] = base;
        let t = run32(g, steps);
        RfWitness::from_trace_m3(&t, s, sibs_for((base / 8) as u64))
    }

    /// MockProver with every failure listed.
    fn m3_mock(w: RfWitness) -> Result<(), String> {
        let leaf = RfLeaf::new(m3_params(), w.s(), Some(w.clone()));
        let p = midnight_proofs::dev::MockProver::run(&leaf, vec![vec![], w.instance()])
            .map_err(|e| format!("{e:?}"))?;
        p.verify().map_err(|e| {
            let all: Vec<String> = e.iter().map(|f| format!("{f}")).collect();
            // One line per refused witness: the distinct failing gates and arguments.
            let mut kinds: Vec<String> = all
                .iter()
                .map(|f| {
                    if let Some(i) = f.find("gate ") {
                        f[i..].split(')').next().unwrap_or("").to_string() + ")"
                    } else if f.contains("ookup") {
                        "lookup".into()
                    } else if f.contains("quality") || f.contains("ermutation") {
                        "permutation".into()
                    } else {
                        f.chars().take(40).collect()
                    }
                })
                .collect();
            kinds.sort();
            kinds.dedup();
            println!(
                "E8-NEG {}: {}",
                std::thread::current().name().unwrap_or("?"),
                kinds.join(", ")
            );
            all.join(" | ")
        })
    }

    fn is_perm(e: &str) -> bool {
        e.contains("quality") || e.contains("ermutation")
    }

    #[test]
    fn m3_honest_leaves_pass() {
        for (base, steps, s) in [(8, 16, 16), (40, 6, 8), (8, 0, 2), (1 << 19, 13, 16)] {
            let w = m3_witness(base, steps, s);
            m3_mock(w).unwrap_or_else(|e| panic!("base {base} steps {steps}: {e}"));
        }
        // The cuts alone, without M3, on every layout but A-inline.
        for arch in [Arch::Wide, Arch::Narrow, Arch::Staggered] {
            let params = RfParams {
                arch,
                pos_hash: true,
                range1: true,
                one_abs: true,
                pack_core: true,
                ..Default::default()
            };
            let c = RfLeaf::new(params, 6, Some(honest(6))).with_hashes(2, vec![3]);
            rf_mock(&c).unwrap_or_else(|e| panic!("{arch:?}: {e}"));
        }
    }

    /// Two leaves chain: the second leaf's `pre_d` is the first leaf's `post_d`
    /// (`len := base + 8`, the new root is the next old root).
    #[test]
    fn m3_leaves_chain() {
        // Genesis tree: every segment empty (segment 0 is reserved).
        let empty: [F; TOP] = std::array::from_fn(empty_top);
        let w1 = RfWitness::from_trace_m3(&run32(genesis(), 16), 16, empty);
        m3_mock(w1.clone()).unwrap();
        let m1 = w1.m3.as_ref().unwrap();
        let mut g = w1.slots[16].pre;
        g[LEN] = 16;
        // Segment 2: left sibling at level 1 is H(seg0, seg1); the rest empty.
        let mut sibs: [F; TOP] = std::array::from_fn(empty_top);
        sibs[1] = h2(F::ZERO, m1.seg_root());
        let t2 = run32(g, 16);
        let w2 = RfWitness::from_trace_m3(&t2, 16, sibs);
        assert_eq!(w2.m3.as_ref().unwrap().pre_root, m1.new_root());
        assert_eq!(w2.instance()[2], w1.instance()[3]);
        m3_mock(w2).unwrap();
    }

    /// C1: no core constraint but the scratch registers' own next state reads
    /// `Pre(v_k)` (`v0..v2` are outside the digest), and a free slot-0 value of
    /// them changes neither acceptance nor the statement.
    #[test]
    fn m3_scratch_registers_are_write_only() {
        for (name, e) in core_constraints() {
            for v in [V0, V1, V2] {
                if name == format!("next {v}") {
                    continue;
                }
                let mut reads = false;
                let mut g = |c: Cell| {
                    if c == Cell::Pre(v) {
                        reads = true;
                    }
                    Some(k(1))
                };
                let _ = e(&mut g);
                assert!(!reads, "`{name}` reads Pre({v})");
            }
        }
        let mut w = m3_witness(8, 16, 16);
        let inst = w.instance();
        for v in [V0, V1, V2] {
            w.slots[0].pre[v] = 12345;
        }
        assert_eq!(w.instance(), inst);
        m3_mock(w).unwrap();
    }

    /// C2: the honest trace writes the zero tuple (D7's sentinel spill
    /// `(top0, top1, fp) = (0, 0, NIL)`); its record carries tag 1 and differs from
    /// padding. Marking it padding (tag 0) is refused.
    #[test]
    fn m3_zero_record_is_tagged() {
        let w = m3_witness(8, 16, 16);
        let m = w.m3.as_ref().unwrap();
        assert_eq!(m.cells[0], [F::ZERO; 3]);
        assert_eq!(m.act[0], F::ONE);
        assert_ne!(
            record_native(F::ONE, [F::ZERO; 3]),
            record_native(F::ZERO, [F::ZERO; 3])
        );
        let mut w = w.clone();
        w.m3.as_mut().unwrap().act[0] = F::ZERO;
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("ookup"), "{e}");
    }

    #[test]
    fn m3_append_not_requested_rejected() {
        let mut w = m3_witness(8, 16, 16);
        let m = w.m3.as_mut().unwrap();
        assert_eq!(m.act, [1u64, 1, 1, 1, 1, 1, 0, 0].map(F::from));
        m.act[6] = F::ONE;
        m.cells[6] = [F::from(5u64); 3];
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("mem chain"), "{e}");
        assert!(!e.contains("ookup"), "{e}");
    }

    #[test]
    fn m3_append_skipped_rejected() {
        // The last append's row zeroed and marked padding: its request is not
        // served and the prefix no longer ends at `len_end`.
        let mut w = m3_witness(8, 16, 16);
        let m = w.m3.as_mut().unwrap();
        m.act[5] = F::ZERO;
        m.cells[5] = [F::ZERO; 3];
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("mem chain") && e.contains("ookup"), "{e}");
        // Marked padding with data: also the padding check. (The spike's
        // honest records are all the zero tuple: every spill is the sentinel.)
        let mut w = m3_witness(8, 16, 16);
        let m = w.m3.as_mut().unwrap();
        m.act[5] = F::ZERO;
        m.cells[5] = [F::from(3u64); 3];
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("mem row"), "{e}");
    }

    #[test]
    fn m3_nonzero_padding_rejected() {
        for col in 0..3 {
            let mut w = m3_witness(8, 16, 16);
            w.m3.as_mut().unwrap().cells[7][col] = F::ONE;
            let e = m3_mock(w).unwrap_err();
            assert!(e.contains("mem row"), "col {col}: {e}");
        }
    }

    #[test]
    fn m3_len_root_desync_rejected() {
        // The close cell is not base + 8.
        let mut w = m3_witness(8, 16, 16);
        w.m3.as_mut().unwrap().len_post = F::from(24u64);
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("m3 walk first"), "{e}");
        // An unaligned base (verdict C3): no bits give 8 idx = 9.
        let w = m3_witness(9, 16, 16);
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("m3 walk first"), "{e}");
        // Walk bits that are not the bits of base / 8 (the update lands in
        // another slot).
        let mut w = m3_witness(8, 16, 16);
        let m = w.m3.as_mut().unwrap();
        m.idx = 3;
        m.pre_root = walk_native(F::ZERO, &m.sibs, 3);
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("m3 walk first"), "{e}");
        // An index of 18 bits (a slot outside the depth-17 top tree).
        let mut w = m3_witness(8, 16, 16);
        let m = w.m3.as_mut().unwrap();
        m.idx = 1 + (1 << 17);
        m.pre_root = walk_native(F::ZERO, &m.sibs, m.idx);
        let e = m3_mock(w).unwrap_err();
        assert!(e.contains("m3 walk"), "{e}");
        // `len_end` on the MEM rows is not the core's final `len`.
        let mut w = m3_witness(8, 16, 16);
        let m = w.m3.as_mut().unwrap();
        m.len_end += F::ONE;
        let e = m3_mock(w).unwrap_err();
        assert!(is_perm(&e), "{e}");
    }

    #[test]
    fn m3_forged_sibling_rejected() {
        for l in [0, 1, 9, TOP - 1] {
            // New walk only: the shared sibling cell's copy breaks.
            let mut w = m3_witness(8, 16, 16);
            w.m3.as_mut().unwrap().sibs_new[l] += F::ONE;
            let e = m3_mock(w).unwrap_err();
            assert!(is_perm(&e), "level {l}: {e}");
            assert!(!e.contains("m3 walk"), "level {l}: {e}");
            // Both walks: the old root is no longer `pre_d`'s root.
            let mut w = m3_witness(8, 16, 16);
            let m = w.m3.as_mut().unwrap();
            m.sibs[l] += F::ONE;
            m.sibs_new[l] += F::ONE;
            let e = m3_mock(w).unwrap_err();
            assert!(is_perm(&e), "level {l}: {e}");
        }
    }

    /// Every digest input (29 words and `mem_root`, in `pre_d` and `post_d`) is a
    /// copy of its cell: a digest of anything else is refused.
    #[test]
    fn m3_digest_binds_every_word() {
        let base = m3_witness(8, 6, 6);
        for side in 0..2 {
            for i in 0..=NDIG {
                let mut w = base.clone();
                w.m3.as_mut().unwrap().dig_in = Some((side, i, F::ONE));
                let e = m3_mock(w).expect_err(&format!("side {side} input {i}"));
                assert!(is_perm(&e), "side {side} input {i}: {e}");
            }
        }
    }

    /// The degree stays 6 with the cuts and M3; cut 1 removes 2 fixed columns
    /// and cut 2 more.
    #[test]
    fn e8_degree_and_columns() {
        let fixed = |p: RfParams| {
            let mut cs = ConstraintSystem::<F>::default();
            RfLeaf::configure_with_params(&mut cs, p);
            (
                cs.degree(),
                cs.num_fixed_columns() + cs.num_selectors(),
                cs.lookups().len(),
            )
        };
        let base = RfParams {
            arch: Arch::Narrow,
            pos_hash: true,
            range1: true,
            ..Default::default()
        };
        let a1 = RfParams {
            one_abs: true,
            ..base
        };
        let pc = RfParams {
            pack_core: true,
            ..a1
        };
        let v: Vec<_> = [base, a1, pc, m3_params()].into_iter().map(fixed).collect();
        println!("E8 (degree, fixed + selectors, lookups): {v:?}");
        let lay = Layout::new(Arch::Narrow);
        let mut by_gate = vec![Vec::new(); lay.gates.len()];
        for (ci, (_, e)) in core_constraints().iter().enumerate() {
            by_gate[lay.host(e).unwrap()].push(ci);
        }
        let n: Vec<usize> = by_gate.iter().map(|g| g.len()).collect();
        println!(
            "E8 core gate rows {:?}: constraints {n:?}, body degrees {:?}",
            lay.gates,
            core_degrees(&lay, &by_gate)
        );
        for x in &v {
            assert_eq!(x.0, 6);
        }
        assert_eq!(v[1].1, v[0].1 - 2);
        assert!(v[2].1 < v[1].1);
    }
}
