//! The spike machine: the register file and three rule bodies, run natively.
//!
//! Rules (spike analogues; names say which C6 rule each stands for):
//! - `Push` (`KMachinePushAccept`): ALU `depth < limit`; APPEND the frame
//!   `(FRAME_RIGHT, ctl_a, fp)`; `fp := len`, `len += 1`, `depth += 1`,
//!   `r := ctl_b`, `ctl := (RET, 0, 0)`. Control rule: stack check and push.
//! - `Ret` (`KMachineReturnRight`): ALU `fp < len`; LOAD the frame at `fp`;
//!   `fp := link`, `depth -= 1`, `ctl := (ARITH, saved, r)`. Data rule: memory read.
//! - `Arith` (`BHelperScalarArithmetic`): ALU checked `ctl_a + ctl_b`; `r := z`;
//!   `ctl := (PUSH, z, ctl_b)`, or `(ERR, 0, 0)` on overflow. Arithmetic rule.
//!
//! From `(PUSH, 1, 1)` the run is Push, Ret, Arith repeated, each step one ALU
//! request and every Push and Ret one MEM request.
use crate::{
    F,
    alu::{AluOp, OP_ADD, OP_LT},
    fu,
};

pub const CTL_RET: u64 = 1;
pub const CTL_PUSH: u64 = 2;
pub const CTL_ARITH: u64 = 3;
pub const CTL_ERR: u64 = 4;

pub const RULE_PUSH: u64 = 1;
pub const RULE_ARITH: u64 = 2;
pub const RULE_RET: u64 = 3;

/// DEC rows `(class = control tag, rule)`.
pub const DEC_ROWS: [(u64, u64); 3] = [
    (CTL_PUSH, RULE_PUSH),
    (CTL_ARITH, RULE_ARITH),
    (CTL_RET, RULE_RET),
];

pub const FRAME_RIGHT: u64 = 7;

/// Register words per state.
pub const NREG: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Regs {
    pub ctl_tag: u64,
    pub ctl_a: u128,
    pub ctl_b: u128,
    pub fp: u128,
    pub depth: u128,
    pub limit: u128,
    pub len: u128,
    pub r: u128,
}

impl Regs {
    pub fn words(&self) -> [F; NREG] {
        [
            F::from(self.ctl_tag),
            fu(self.ctl_a),
            fu(self.ctl_b),
            fu(self.fp),
            fu(self.depth),
            fu(self.limit),
            fu(self.len),
            fu(self.r),
        ]
    }

    /// The machine start used by the experiments: `(PUSH, 1, 1)`, empty stack,
    /// stack limit 8, memory length 1 (address 0 is the null link).
    pub fn genesis() -> Self {
        Self {
            ctl_tag: CTL_PUSH,
            ctl_a: 1,
            ctl_b: 1,
            fp: 0,
            depth: 0,
            limit: 8,
            len: 1,
            r: 0,
        }
    }
}

/// Placeholder state digest: `sum lambda^i word_i`. Not collision resistant; it
/// stands in for the HASH chip (experiment E4) so the leaf has the node's statement.
pub fn digest(regs: &Regs) -> F {
    let lambda = F::from(1_000_003u64);
    let mut acc = F::from(0u64);
    for w in regs.words().iter().rev() {
        acc = acc * lambda + w;
    }
    acc
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Push,
    Arith,
    Ret,
}

impl Rule {
    pub fn id(self) -> u64 {
        match self {
            Rule::Push => RULE_PUSH,
            Rule::Arith => RULE_ARITH,
            Rule::Ret => RULE_RET,
        }
    }
}

/// A memory cell `(tag, payload, link)`.
pub type Cell = [u128; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemOp {
    Append(u128, Cell),
    Load(u128, Cell),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step {
    pub rule: Rule,
    pub pre: Regs,
    pub post: Regs,
    pub alu: AluOp,
    pub mem: Option<MemOp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub pre: Regs,
    pub steps: Vec<Step>,
    /// Cells appended in this leaf, at addresses `pre.len ..`.
    pub appended: Vec<Cell>,
}

impl Trace {
    pub fn post(&self) -> Regs {
        self.steps.last().map_or(self.pre, |s| s.post)
    }
}

/// One step from `x` over the leaf's appended cells (base `base`), or `None` when
/// no spike rule applies (halt, error, refusal, or a read of an older cell).
pub fn step(x: &Regs, base: u128, appended: &[Cell]) -> Option<Step> {
    let mut y = *x;
    let (rule, alu, mem) = match x.ctl_tag {
        CTL_PUSH => {
            let alu = AluOp::eval(OP_LT, x.depth, x.limit);
            if alu.z != 1 {
                return None;
            }
            let cell = [FRAME_RIGHT as u128, x.ctl_a, x.fp];
            y.fp = x.len;
            y.len = x.len + 1;
            y.depth = x.depth + 1;
            y.r = x.ctl_b;
            (y.ctl_tag, y.ctl_a, y.ctl_b) = (CTL_RET, 0, 0);
            (Rule::Push, alu, Some(MemOp::Append(x.len, cell)))
        }
        CTL_RET => {
            let alu = AluOp::eval(OP_LT, x.fp, x.len);
            if alu.z != 1 || x.fp < base {
                return None;
            }
            let cell = *appended.get((x.fp - base) as usize)?;
            if cell[0] != FRAME_RIGHT as u128 {
                return None;
            }
            y.fp = cell[2];
            y.depth = x.depth.checked_sub(1)?;
            (y.ctl_tag, y.ctl_a, y.ctl_b) = (CTL_ARITH, cell[1], x.r);
            (Rule::Ret, alu, Some(MemOp::Load(x.fp, cell)))
        }
        CTL_ARITH => {
            let alu = AluOp::eval(OP_ADD, x.ctl_a, x.ctl_b);
            y.r = alu.z;
            if alu.flag == 1 {
                (y.ctl_tag, y.ctl_a, y.ctl_b) = (CTL_ERR, 0, 0);
            } else {
                (y.ctl_tag, y.ctl_a, y.ctl_b) = (CTL_PUSH, alu.z, x.ctl_b);
            }
            (Rule::Arith, alu, None)
        }
        _ => return None,
    };
    Some(Step {
        rule,
        pre: *x,
        post: y,
        alu,
        mem,
    })
}

/// Run at most `n` steps from `pre`.
pub fn run(pre: Regs, n: usize) -> Trace {
    let mut t = Trace {
        pre,
        steps: Vec::new(),
        appended: Vec::new(),
    };
    let mut x = pre;
    while t.steps.len() < n {
        let Some(s) = step(&x, pre.len, &t.appended) else {
            break;
        };
        if let Some(MemOp::Append(_, c)) = s.mem {
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
    fn genesis_runs_push_ret_arith() {
        let t = run(Regs::genesis(), 9);
        assert_eq!(t.steps.len(), 9);
        let rules: Vec<_> = t.steps.iter().map(|s| s.rule).collect();
        assert_eq!(&rules[..3], &[Rule::Push, Rule::Ret, Rule::Arith]);
        // r: 1, then 1+1 = 2, 2+1 = 3, 3+1 = 4.
        assert_eq!(t.post().r, 4);
        assert_eq!(t.appended.len(), 3);
        assert_eq!(t.post().depth, 0);
    }

    #[test]
    fn overflow_publishes_error_and_halts() {
        let x = Regs {
            ctl_tag: CTL_ARITH,
            ctl_a: u128::MAX,
            ctl_b: 1,
            ..Regs::genesis()
        };
        let t = run(x, 4);
        assert_eq!(t.steps.len(), 1);
        assert_eq!(t.post().ctl_tag, CTL_ERR);
    }
}
