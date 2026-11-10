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
    mem::MemConfig,
    range::{RangeConfig, rows_per_value},
    rf::*,
    trace::Cell as MemCell,
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
    pub mem: MemConfig,
    pub dec: DecConfig,
    pub gate_sels: Vec<Option<Selector>>,
    pub q_dec: Selector,
    pub dec_off: usize,
    pub q_cnt0: Selector,
    /// Inline digest gate (Wide-inline only) and its slot row.
    pub q_dig_inline: Option<(Selector, usize)>,
    pub dig: Option<DigConfig>,
    pub hash: Option<HashConfig>,
    pub slim: Option<SlimHashShape>,
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
        }
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
        vec![
            F::from(0u64),
            F::from(0u64),
            rf_digest(&self.slots[0].pre),
            rf_digest(&self.slots[self.s()].pre),
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
        assert!(self.params.hash || (n2 == 0 && extra.is_empty()));
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
        l.h * (s + 1) + s * rows_per_value(RC) + s + dig + n_hash2
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
        let range = RangeConfig::configure(meta, c12, RC);
        let alu = AluConfig::configure(meta, bus, range.clone());
        let mem = MemConfig::configure(meta, bus);
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
        let mut gate_sels = Vec::new();
        for (gi, idx) in by_gate.iter().enumerate() {
            if idx.is_empty() {
                gate_sels.push(None);
                continue;
            }
            let sel = meta.selector();
            let o = layout.gates[gi];
            let (lay, idx, colv) = (layout.clone(), idx.clone(), cols.clone());
            meta.create_gate("rf core", move |m| {
                let cons = core_constraints();
                let q = m.query_selector(sel);
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

        // DEC: `(en * ctl_tag, rule id)` on the first gate row that sees them.
        let dec_ex = ex(|g| {
            let f = flags(g)?;
            Some(g(Cell::Pre(CTL_TAG))? * sum(&f))
        });
        let dgi = layout.host(&dec_ex).expect("DEC host");
        let dec_off = layout.gates[dgi];
        let q_dec = meta.complex_selector();
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

        let q_cnt0 = meta.selector();
        let (cnt_col, _) = layout
            .find(Cell::Cnt, layout.place[layout.canon(Cell::Cnt)].1)
            .unwrap();
        meta.create_gate("rf count0", |m| {
            let q = m.query_selector(q_cnt0);
            crate::gate(vec![q * m.query_advice(cols[cnt_col], Rotation::cur())])
        });

        // Digest.
        let mut q_dig_inline = None;
        let mut dig = None;
        if params.arch == Arch::WideInline {
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

        RfConfig {
            params,
            layout,
            cols,
            bus,
            range,
            alu,
            mem,
            dec,
            gate_sels,
            q_dec,
            dec_off,
            q_cnt0,
            q_dig_inline,
            dig,
            hash,
            slim,
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
                let mem: Vec<_> = match w {
                    Some(w) => w
                        .mem
                        .iter()
                        .map(|c| Value::known([fu(c[0]), fu(c[1]), fu(c[2])]))
                        .collect(),
                    None => vec![Value::unknown(); s],
                };
                let first = cfg.mem.assign(&mut r, o, s, base, &mem)?;
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
            for (hash, slim_hash) in [(false, false), (true, false), (false, true)] {
                for r in rotations(RfParams {
                    arch,
                    hash,
                    pad: 1,
                    slim_hash,
                    extra_lookups: 0,
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
            },
            4,
            Some(w),
        )
        .with_hashes(3, vec![4]);
        rf_mock(&c).unwrap();
    }
}
