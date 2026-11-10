//! The full-width register file (experiment E5): 32 register words, the cached
//! frame and task protocol of design 2.7.8 D7, and six spike rules.
//!
//! The word layout follows `study.md` 1.3 (about 32 words): `ctl` 5, `st` 8, `env`
//! 1, `fp`/`tp`/`len` 3, `top` 2, `task` 2, `h` 6, `r` 2, `v0..v2` 3. A real top
//! frame has 13 lanes; here `top` and `task` are two words each, so a stored entry
//! `(top0, top1, fp)` fits one bus tuple. The state digest covers words 0..29 and
//! leaves out the scratch registers `v0..v2`.
//!
//! Rules (D7, cached): every enabled step adds one to `work`, takes one from
//! `fuel`, and writes `v0 := alu.z`, `v1 := mem.a`, `v2 := mem.b`.
//! - `Push`: ALU `fcount < limit`; APPEND `(top0, top1, fp)` at `len`; `fp := len`,
//!   `len += 1`, `top := (ctl_a, ctl_b)`, `fcount += 1`; next `Rewrite`.
//! - `Rewrite`: no bus request; `top0 := ctl_b`; next `TPush`.
//! - `TPush`: ALU `tcount < limit`; APPEND `(task0, task1, tp)` at `len`; `tp := len`,
//!   `len += 1`, `task := (ctl_a, ctl_c)`, `tcount += 1`; next `Arith`.
//! - `Arith`: ALU checked `ctl_a + ctl_b`; `r0 := z`, `ctl_a := z`; next `TPop`, or
//!   `ERR` on overflow.
//! - `TPop`: ALU `tp < len`; LOAD `mem[tp] = (a, b, link)`; `h := (task0, task1)`,
//!   `task := (a, b)`, `tp := link`, `tcount -= 1`; next `Pop`.
//! - `Pop`: ALU `fp < len`; LOAD `mem[fp] = (a, b, link)`; `r1 := top0`,
//!   `top := (a, b)`, `fp := link`, `fcount -= 1`; next `Push`.
//!
//! These are spike rules for sizing the register file; they are not C6 rules. The
//! `frame_count = 0` clauses of D7 and `NIL != 0` (the spike uses `NIL = 0` with
//! address 0 reserved, as the 8-register spike does) are not modelled here.
use crate::{
    alu::{AluOp, OP_ADD, OP_LT},
    trace::Cell,
};

pub const NREG: usize = 32;
/// Words covered by the state digest (all but `v0..v2`).
pub const NDIG: usize = 29;

pub const CTL_TAG: usize = 0;
pub const CTL_A: usize = 1;
pub const CTL_B: usize = 2;
pub const CTL_C: usize = 3;
pub const CTL_ORIGIN: usize = 4;
pub const OUTCOME: usize = 5;
pub const FUEL0: usize = 6;
pub const FUEL: usize = 7;
pub const WORK: usize = 8;
pub const HCTR: usize = 9;
pub const LIMIT: usize = 10;
pub const FCOUNT: usize = 11;
pub const TCOUNT: usize = 12;
pub const ENV: usize = 13;
pub const FP: usize = 14;
pub const TP: usize = 15;
pub const LEN: usize = 16;
pub const TOP0: usize = 17;
pub const TOP1: usize = 18;
pub const TASK0: usize = 19;
pub const TASK1: usize = 20;
pub const H0: usize = 21;
pub const H1: usize = 22;
pub const H2: usize = 23;
pub const H3: usize = 24;
pub const H4: usize = 25;
pub const H5: usize = 26;
pub const R0: usize = 27;
pub const R1: usize = 28;
pub const V0: usize = 29;
pub const V1: usize = 30;
pub const V2: usize = 31;

/// Rules, by flag index; the rule id is index + 1 and equals the control tag
/// that selects it.
pub const NRULE: usize = 6;
pub const PUSH: usize = 0;
pub const POP: usize = 1;
pub const REWRITE: usize = 2;
pub const ARITH: usize = 3;
pub const TPUSH: usize = 4;
pub const TPOP: usize = 5;
/// Control tags (`tag = rule + 1`); `T_ERR` has no rule.
pub const T_PUSH: u64 = 1;
pub const T_POP: u64 = 2;
pub const T_REWRITE: u64 = 3;
pub const T_ARITH: u64 = 4;
pub const T_TPUSH: u64 = 5;
pub const T_TPOP: u64 = 6;
pub const T_ERR: u64 = 7;

/// DEC rows `(control tag, rule id)`.
pub const DEC_ROWS: [(u64, u64); NRULE] = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

pub type Regs32 = [u128; NREG];

/// The experiment start: `ctl = (PUSH, 1, 1, 5, 0)`, fuel 10^6, stack limit 8,
/// empty frame and task stacks (`fp = tp = 0`), memory length 1.
pub fn genesis() -> Regs32 {
    let mut x = [0u128; NREG];
    x[CTL_TAG] = T_PUSH as u128;
    x[CTL_A] = 1;
    x[CTL_B] = 1;
    x[CTL_C] = 5;
    x[FUEL0] = 1_000_000;
    x[FUEL] = 1_000_000;
    x[LIMIT] = 8;
    x[LEN] = 1;
    x
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemReq {
    Append(u128, Cell),
    Load(u128, Cell),
}

impl MemReq {
    pub fn parts(&self) -> (u128, Cell) {
        match *self {
            MemReq::Append(a, c) | MemReq::Load(a, c) => (a, c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step32 {
    pub rule: usize,
    pub pre: Regs32,
    pub post: Regs32,
    pub alu: Option<AluOp>,
    pub mem: Option<MemReq>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace32 {
    pub pre: Regs32,
    pub steps: Vec<Step32>,
    /// Cells appended in this leaf, at addresses `pre[LEN] ..`.
    pub appended: Vec<Cell>,
}

impl Trace32 {
    pub fn post(&self) -> Regs32 {
        self.steps.last().map_or(self.pre, |s| s.post)
    }
}

fn load(x: u128, base: u128, appended: &[Cell]) -> Option<Cell> {
    if x < base {
        return None;
    }
    appended.get((x - base) as usize).copied()
}

/// One step from `x`, or `None` when no rule applies (error tag, refusal, or a read
/// of a cell of an earlier leaf).
pub fn step32(x: &Regs32, base: u128, appended: &[Cell]) -> Option<Step32> {
    let mut y = *x;
    let tag = x[CTL_TAG] as u64;
    let (rule, alu, mem) = match tag {
        T_PUSH => {
            let alu = AluOp::eval(OP_LT, x[FCOUNT], x[LIMIT]);
            if alu.z != 1 {
                return None;
            }
            let cell = [x[TOP0], x[TOP1], x[FP]];
            y[FP] = x[LEN];
            y[LEN] = x[LEN] + 1;
            y[TOP0] = x[CTL_A];
            y[TOP1] = x[CTL_B];
            y[FCOUNT] = x[FCOUNT] + 1;
            y[CTL_TAG] = T_REWRITE as u128;
            (PUSH, Some(alu), Some(MemReq::Append(x[LEN], cell)))
        }
        T_REWRITE => {
            y[TOP0] = x[CTL_B];
            y[CTL_TAG] = T_TPUSH as u128;
            (REWRITE, None, None)
        }
        T_TPUSH => {
            let alu = AluOp::eval(OP_LT, x[TCOUNT], x[LIMIT]);
            if alu.z != 1 {
                return None;
            }
            let cell = [x[TASK0], x[TASK1], x[TP]];
            y[TP] = x[LEN];
            y[LEN] = x[LEN] + 1;
            y[TASK0] = x[CTL_A];
            y[TASK1] = x[CTL_C];
            y[TCOUNT] = x[TCOUNT] + 1;
            y[CTL_TAG] = T_ARITH as u128;
            (TPUSH, Some(alu), Some(MemReq::Append(x[LEN], cell)))
        }
        T_ARITH => {
            let alu = AluOp::eval(OP_ADD, x[CTL_A], x[CTL_B]);
            y[R0] = alu.z;
            if alu.flag == 1 {
                y[CTL_TAG] = T_ERR as u128;
            } else {
                y[CTL_A] = alu.z;
                y[CTL_TAG] = T_TPOP as u128;
            }
            (ARITH, Some(alu), None)
        }
        T_TPOP => {
            let alu = AluOp::eval(OP_LT, x[TP], x[LEN]);
            if alu.z != 1 || x[TCOUNT] == 0 {
                return None;
            }
            let cell = load(x[TP], base, appended)?;
            y[H0] = x[TASK0];
            y[H1] = x[TASK1];
            y[TASK0] = cell[0];
            y[TASK1] = cell[1];
            y[TP] = cell[2];
            y[TCOUNT] = x[TCOUNT] - 1;
            y[CTL_TAG] = T_POP as u128;
            (TPOP, Some(alu), Some(MemReq::Load(x[TP], cell)))
        }
        T_POP => {
            let alu = AluOp::eval(OP_LT, x[FP], x[LEN]);
            if alu.z != 1 || x[FCOUNT] == 0 {
                return None;
            }
            let cell = load(x[FP], base, appended)?;
            y[R1] = x[TOP0];
            y[TOP0] = cell[0];
            y[TOP1] = cell[1];
            y[FP] = cell[2];
            y[FCOUNT] = x[FCOUNT] - 1;
            y[CTL_TAG] = T_PUSH as u128;
            (POP, Some(alu), Some(MemReq::Load(x[FP], cell)))
        }
        _ => return None,
    };
    y[WORK] = x[WORK] + 1;
    y[FUEL] = x[FUEL].checked_sub(1)?;
    y[V0] = alu.map_or(0, |a| a.z);
    let (va, vb) = mem.map_or((0, 0), |m| {
        let c = m.parts().1;
        (c[0], c[1])
    });
    y[V1] = va;
    y[V2] = vb;
    Some(Step32 {
        rule,
        pre: *x,
        post: y,
        alu,
        mem,
    })
}

/// Run at most `n` steps from `pre`.
pub fn run32(pre: Regs32, n: usize) -> Trace32 {
    let mut t = Trace32 {
        pre,
        steps: Vec::new(),
        appended: Vec::new(),
    };
    let mut x = pre;
    while t.steps.len() < n {
        let Some(s) = step32(&x, pre[LEN], &t.appended) else {
            break;
        };
        if let Some(MemReq::Append(_, c)) = s.mem {
            t.appended.push(c);
        }
        x = s.post;
        t.steps.push(s);
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_of_six_rules() {
        let t = run32(genesis(), 12);
        assert_eq!(t.steps.len(), 12);
        let rules: Vec<_> = t.steps.iter().map(|s| s.rule).collect();
        assert_eq!(&rules[..6], &[PUSH, REWRITE, TPUSH, ARITH, TPOP, POP]);
        assert_eq!(t.appended.len(), 4);
        let y = t.post();
        assert_eq!((y[FCOUNT], y[TCOUNT], y[WORK]), (0, 0, 12));
        // D7: after a push and a pop the cached top is the old top again.
        assert_eq!((y[TOP0], y[TOP1], y[FP]), (0, 0, 0));
    }

    #[test]
    fn every_step_writes_a_few_registers() {
        let t = run32(genesis(), 6);
        for s in &t.steps {
            let n = (0..NREG).filter(|&j| s.pre[j] != s.post[j]).count();
            assert!((3..=10).contains(&n), "rule {} wrote {n}", s.rule);
        }
    }
}
