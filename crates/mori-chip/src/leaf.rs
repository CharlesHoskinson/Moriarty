//! The spike leaf: CORE, ALU with RANGE, MEM and DEC in one circuit over the
//! shared columns, with the node's leaf statement.
//!
//! Layout (one region): core `2 S + 2` rows, then `S` ALU operations of
//! `ceil(12 / RC)` rows, then `S` MEM rows. Tables: RANGE (4,096 rows) and DEC.
//!
//! Statement: two instance columns as the stdlib leaves have them (committed first,
//! unused; then the node's 7 leaf lanes `[binding, prog_d, pre_d, post_d, rule,
//! h_pre, steps]`). `pre_d`, `post_d` and `steps` are copy-constrained; `pre_d` and
//! `post_d` are the placeholder digest of [`crate::trace::digest`] (the HASH chip is
//! E4). The other lanes are not constrained by this spike.
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, Column, ConstraintSystem, Error, Instance},
};

use crate::{
    Cols, F,
    alu::{AluConfig, AluOp, OP_ADD},
    bus::{BusConfig, CHIP_MEM, Tuple, tuple},
    cpu::{CoreConfig, SlotWitness},
    fu,
    mem::MemConfig,
    range::{RangeConfig, rows_per_value},
    trace::{MemOp, Regs, Trace, digest},
};

/// Leaf statement lanes (the node's `LEAF_LANES`).
pub const LEAF_LANES: usize = 7;

#[derive(Clone, Debug)]
pub struct LeafConfig {
    pub cols: Cols,
    pub bus: BusConfig,
    pub range: RangeConfig,
    pub alu: AluConfig,
    pub mem: MemConfig,
    pub core: CoreConfig,
    pub committed: Column<Instance>,
    pub instance: Column<Instance>,
}

/// Everything the prover writes. Built from a [`Trace`]; negative tests edit it.
#[derive(Clone, Debug)]
pub struct LeafWitness {
    pub slots: Vec<SlotWitness>,
    pub post: Regs,
    /// One ALU provider per slot; `true` writes it with a forged decomposition.
    pub alu: Vec<(AluOp, bool)>,
    pub mem_base: u128,
    /// MEM provider rows at `mem_base ..` (absent rows are zero).
    pub mem: Vec<[u128; 3]>,
}

fn zero() -> Tuple {
    [F::from(0u64); 5]
}

impl LeafWitness {
    /// The witness of `trace` padded to `s` slots (disabled slots keep the post-state).
    pub fn from_trace(t: &Trace, s: usize) -> Self {
        assert!(t.steps.len() <= s, "trace longer than the leaf");
        let mut slots = Vec::with_capacity(s);
        let mut alu = Vec::with_capacity(s);
        for st in &t.steps {
            let flags = match st.rule {
                crate::trace::Rule::Push => [1, 0, 0],
                crate::trace::Rule::Arith => [0, 1, 0],
                crate::trace::Rule::Ret => [0, 0, 1],
            };
            let u2 = match st.mem {
                Some(MemOp::Append(addr, c)) | Some(MemOp::Load(addr, c)) => {
                    tuple(CHIP_MEM, fu(addr), fu(c[0]), fu(c[1]), fu(c[2]))
                }
                None => zero(),
            };
            slots.push(SlotWitness {
                pre: st.pre,
                flags,
                ovf: st.alu.flag,
                u1: st.alu.tuple(),
                u2,
            });
            alu.push((st.alu, false));
        }
        let post = t.post();
        while slots.len() < s {
            slots.push(SlotWitness {
                pre: post,
                flags: [0, 0, 0],
                ovf: 0,
                u1: zero(),
                u2: zero(),
            });
            alu.push((AluOp::eval(OP_ADD, 0, 0), false));
        }
        Self {
            slots,
            post,
            alu,
            mem_base: t.pre.len,
            mem: t.appended.clone(),
        }
    }

    pub fn steps(&self) -> u64 {
        self.slots.iter().map(|s| s.flags.iter().sum::<u64>()).sum()
    }

    /// The 7 leaf lanes; unconstrained lanes are zero.
    pub fn instance(&self) -> Vec<F> {
        let pre = self.slots.first().map_or(self.post, |s| s.pre);
        vec![
            F::from(0u64),
            F::from(0u64),
            digest(&pre),
            digest(&self.post),
            F::from(0u64),
            F::from(0u64),
            F::from(self.steps()),
        ]
    }
}

/// The leaf circuit with `s` slots and `RC` range-checked limb columns.
#[derive(Clone, Debug)]
pub struct Leaf<const RC: usize> {
    pub s: usize,
    pub w: Option<LeafWitness>,
}

impl<const RC: usize> Leaf<RC> {
    pub fn new(s: usize, w: Option<LeafWitness>) -> Self {
        assert!(s >= 1);
        if let Some(w) = &w {
            assert_eq!(w.slots.len(), s);
            assert_eq!(w.alu.len(), s);
            assert!(w.mem.len() <= s);
        }
        Self { s, w }
    }

    /// Rows used by the region (tables excluded).
    pub fn region_rows(s: usize) -> usize {
        CoreConfig::rows(s) + s * rows_per_value(RC) + s
    }
}

impl<const RC: usize> Circuit<F> for Leaf<RC> {
    type Config = LeafConfig;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = ();

    fn without_witnesses(&self) -> Self {
        Self { s: self.s, w: None }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> LeafConfig {
        let cols = Cols::configure(meta);
        let committed = meta.instance_column();
        let instance = meta.instance_column();
        let bus = BusConfig::configure(meta, cols);
        let range = RangeConfig::configure(meta, cols, RC);
        let alu = AluConfig::configure(meta, bus, range.clone());
        let mem = MemConfig::configure(meta, bus);
        let core = CoreConfig::configure(meta, bus);
        meta.enable_equality(instance);
        for i in crate::cpu::EQ_COLS {
            meta.enable_equality(cols.a[i]);
        }
        LeafConfig {
            cols,
            bus,
            range,
            alu,
            mem,
            core,
            committed,
            instance,
        }
    }

    fn synthesize(&self, cfg: LeafConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
        cfg.range.load(&mut l)?;
        cfg.core.load(&mut l)?;
        let s = self.s;
        let w = self.w.as_ref();
        let cells = l.assign_region(
            || "leaf",
            |mut r| {
                let slots: Vec<Value<SlotWitness>> = (0..s)
                    .map(|i| w.map_or(Value::unknown(), |w| Value::known(w.slots[i])))
                    .collect();
                let post = w.map_or(Value::unknown(), |w| Value::known(w.post));
                let cells = cfg.core.assign(&mut r, 0, &slots, post)?;
                let mut o = CoreConfig::rows(s);
                for i in 0..s {
                    match w.map(|w| w.alu[i]) {
                        Some((op, true)) => o += cfg.alu.assign_forged(&mut r, o, op)?,
                        Some((op, false)) => o += cfg.alu.assign(&mut r, o, Value::known(op))?,
                        None => o += cfg.alu.assign(&mut r, o, Value::unknown())?,
                    }
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
                r.constrain_equal(first.cell(), cells.len0.cell())?;
                Ok(cells)
            },
        )?;
        l.constrain_instance(cells.pre_d.cell(), cfg.instance, 2)?;
        l.constrain_instance(cells.post_d.cell(), cfg.instance, 3)?;
        l.constrain_instance(cells.steps.cell(), cfg.instance, 6)?;
        Ok(())
    }
}

/// MockProver result of a leaf on its witness's own statement.
pub fn mock_ok<const RC: usize>(s: usize, w: &LeafWitness) -> Result<(), String> {
    let c = Leaf::<RC>::new(s, Some(w.clone()));
    let p = midnight_proofs::dev::MockProver::run(&c, vec![vec![], w.instance()])
        .map_err(|e| format!("{e:?}"))?;
    p.verify().map_err(|e| {
        let mut v: Vec<String> = e.iter().take(4).map(|f| format!("{f}")).collect();
        v.sort();
        v.join(" | ")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        alu::OP_LT,
        trace::{CTL_RET, FRAME_RIGHT, Regs, run},
    };

    const RC: usize = 4;

    fn honest(s: usize) -> LeafWitness {
        LeafWitness::from_trace(&run(Regs::genesis(), s), s)
    }

    /// The in-circuit verifier evaluates queries at rotations -1, 0 and +1 only
    /// (midnight-circuits 7.3.0 `verifier_gadget.rs:609`).
    #[test]
    fn rotations_are_unit() {
        let mut cs = ConstraintSystem::<F>::default();
        Leaf::<RC>::configure(&mut cs);
        let rots = cs
            .advice_queries()
            .iter()
            .map(|q| q.1.0)
            .chain(cs.fixed_queries().iter().map(|q| q.1.0))
            .chain(cs.instance_queries().iter().map(|q| q.1.0));
        for r in rots {
            assert!((-1..=1).contains(&r), "rotation {r}");
        }
    }

    #[test]
    fn honest_leaves_pass() {
        for s in [1, 2, 4, 7] {
            let w = honest(s);
            assert_eq!(w.steps(), s as u64);
            mock_ok::<RC>(s, &w).unwrap_or_else(|e| panic!("s={s}: {e}"));
        }
        // Partially enabled leaf: 2 steps in 4 slots.
        let t = run(Regs::genesis(), 2);
        mock_ok::<RC>(4, &LeafWitness::from_trace(&t, 4)).unwrap();
    }

    #[test]
    fn wrong_statement_rejected() {
        let w = honest(3);
        let c = Leaf::<RC>::new(3, Some(w.clone()));
        let mut inst = w.instance();
        inst[6] = F::from(2u64);
        let p = midnight_proofs::dev::MockProver::run(&c, vec![vec![], inst]).unwrap();
        assert!(p.verify().is_err());
    }

    /// N1: a forged bus tuple. The push's APPEND request names a different cell
    /// than MEM holds; nothing else changes.
    #[test]
    fn n1_forged_bus_tuple_rejected() {
        let mut w = honest(3);
        w.slots[0].u2[3] += F::from(1u64);
        let e = mock_ok::<RC>(3, &w).unwrap_err();
        assert!(
            e.contains("bus") || e.contains("Lookup") || e.contains("lookup"),
            "{e}"
        );
    }

    /// N2: a wrong ALU result. The arithmetic step claims `2 + 1 = 4`: the request,
    /// the ALU provider row (with a decomposition of 4) and the next registers all
    /// agree, so only the ALU gate can refuse it.
    #[test]
    fn n2_wrong_alu_result_rejected() {
        let mut w = honest(6);
        let i = 5; // Arith of (2, 1)
        assert_eq!(w.slots[i].flags, [0, 1, 0]);
        let mut op = w.alu[i].0;
        op.z += 1;
        w.alu[i] = (op, true);
        w.slots[i].u1 = op.tuple();
        w.post.r += 1;
        w.post.ctl_a += 1;
        let e = mock_ok::<RC>(6, &w).unwrap_err();
        assert!(e.contains("alu"), "{e}");
    }

    /// N3: a wrong memory read. The return step loads a frame with another saved
    /// value; the request and the next registers agree, MEM does not.
    #[test]
    fn n3_wrong_memory_read_rejected() {
        let mut w = honest(3);
        let i = 1; // Ret
        assert_eq!(w.slots[i].flags, [0, 0, 1]);
        w.slots[i].u2[3] += F::from(5u64);
        w.slots[i + 1].pre.ctl_a += 5;
        // Step 2 (Arith) then adds the forged operand; recompute it honestly.
        let x = w.slots[2].pre;
        let op = AluOp::eval(OP_ADD, x.ctl_a, x.ctl_b);
        w.alu[2] = (op, false);
        w.slots[2].u1 = op.tuple();
        w.post.r = op.z;
        w.post.ctl_a = op.z;
        let e = mock_ok::<RC>(3, &w).unwrap_err();
        assert!(e.contains("bus") || e.contains("ookup"), "{e}");
    }

    /// N4: a wrong next pc. The push's successor control is `(ARITH, ..)` instead of
    /// `(RET, ..)`; everything else is honest.
    #[test]
    fn n4_wrong_next_control_rejected() {
        let mut w = honest(1);
        w.post.ctl_tag = crate::trace::CTL_ARITH;
        let e = mock_ok::<RC>(1, &w).unwrap_err();
        assert!(e.contains("core step"), "{e}");
        // And a wrong successor payload (the "pc" operand) after an arithmetic step.
        let mut w = honest(3);
        w.post.ctl_a += 1;
        let e = mock_ok::<RC>(3, &w).unwrap_err();
        assert!(e.contains("core step"), "{e}");
    }

    /// N5: a chip row with no request. MEM holds a frame at address 2 that no
    /// append wrote (the leaf appends nothing before it); a return reads it with
    /// `fp = 2 >= len = 1`, so the ALU `fp < len` request has no honest provider and
    /// a forged one fails the ALU gate.
    #[test]
    fn n5_unrequested_chip_row_rejected() {
        let pre = Regs {
            ctl_tag: CTL_RET,
            fp: 2,
            depth: 1,
            ..Regs::genesis()
        };
        let lt = AluOp {
            op: OP_LT,
            x: 2,
            y: 1,
            flag: 0,
            z: 1,
        };
        let frame = [FRAME_RIGHT as u128, 9, 0];
        let post = Regs {
            ctl_tag: crate::trace::CTL_ARITH,
            ctl_a: 9,
            ctl_b: pre.r,
            fp: 0,
            depth: 0,
            ..pre
        };
        let w = LeafWitness {
            slots: vec![SlotWitness {
                pre,
                flags: [0, 0, 1],
                ovf: 0,
                u1: lt.tuple(),
                u2: tuple(CHIP_MEM, fu(2), fu(frame[0]), fu(frame[1]), fu(frame[2])),
            }],
            post,
            alu: vec![(lt, true)],
            mem_base: 1,
            // Row 0 = address 1 (never appended), row 1 = address 2: the forged frame.
            mem: vec![[0, 0, 0], frame],
        };
        // Two MEM rows need two slots' worth of rows: use s = 2 with an idle slot.
        let mut w2 = w.clone();
        w2.slots.push(SlotWitness {
            pre: post,
            flags: [0, 0, 0],
            ovf: 0,
            u1: zero(),
            u2: zero(),
        });
        w2.alu.push((AluOp::eval(OP_ADD, 0, 0), false));
        let e = mock_ok::<RC>(2, &w2).unwrap_err();
        assert!(e.contains("alu"), "{e}");
        // Without the forged ALU row the request finds no provider at all.
        let mut w3 = w2.clone();
        w3.alu[0] = (AluOp::eval(OP_ADD, 0, 0), false);
        let e = mock_ok::<RC>(2, &w3).unwrap_err();
        assert!(e.contains("ookup") || e.contains("bus"), "{e}");
    }

    /// Two identical forged requests with no provider do not serve each other.
    #[test]
    fn n6_duplicate_forged_requests_rejected() {
        let mut w = honest(2);
        // Both the push and the return request the same forged cell at address 1.
        let forged = tuple(CHIP_MEM, fu(1), fu(FRAME_RIGHT as u128), fu(99), fu(0));
        w.slots[0].u2 = forged;
        w.slots[1].u2 = forged;
        w.slots[0].pre.ctl_a = 99;
        w.post.ctl_a = 99;
        let e = mock_ok::<RC>(2, &w).unwrap_err();
        assert!(e.contains("ookup") || e.contains("bus"), "{e}");
        assert!(!e.contains("core step"), "{e}");
    }

    /// A disabled slot that makes a request is refused (inactive tuples are zero).
    #[test]
    fn n7_disabled_slot_request_rejected() {
        let t = run(Regs::genesis(), 1);
        let mut w = LeafWitness::from_trace(&t, 2);
        w.slots[1].u1 = AluOp::eval(OP_ADD, 0, 0).tuple();
        assert!(mock_ok::<RC>(2, &w).is_err());
    }
}
