//! CORE: the register file over S step slots and the micro-op requests.
//!
//! A slot is `H_CORE = 2` rows:
//! - REG row: registers `ctl_tag, ctl_a, ctl_b, fp, depth, limit, len, r` in
//!   columns 0..8, rule flags `push, arith, ret` in 8..11, the running step count
//!   (steps before this slot) in 11. `en = push + arith + ret` is not stored;
//! - REQ row: the ALU request in bus lane 0 (columns 0..5), the MEM request (APPEND
//!   for push, LOAD for ret) in bus lane 1 (columns 5..10), the carry in 10, the
//!   pre-state digest in 11 (slot 0 only).
//!
//! The next slot's REG row holds the post-state. After the last slot come the final
//! REG row and one row with the post-state digest in column 0.
//!
//! Gates sit on the REQ row and reach the slot's REG row at rotation -1 and the next
//! REG row at +1 (the in-circuit verifier supports rotations -1, 0, +1 only): flags
//! boolean with `en` boolean; `en` is a prefix; a disabled slot keeps every register
//! and makes no request; each request tuple and each next register is the rule's
//! expression; the count advances by `en`. The DEC lookup sits on the REG row and
//! ties `(en * ctl_tag, rule)` to the decoder table. A register a rule does not name
//! keeps its value (frame condition) by the same expressions.
use midnight_proofs::{
    circuit::{AssignedCell, Region, Value},
    plonk::{ConstraintSystem, Error, Expression, Selector, VirtualCells},
    poly::Rotation,
};

use crate::{
    F,
    alu::{FLAG_SHIFT, OP_ADD, OP_LT},
    bus::{BusConfig, CHIP_ALU, CHIP_MEM, Tuple, WIDTH},
    dec::DecConfig,
    trace::{CTL_ARITH, CTL_ERR, CTL_PUSH, CTL_RET, DEC_ROWS, FRAME_RIGHT, NREG, Regs},
};

pub const H_CORE: usize = 2;
/// REG row columns.
pub const C_P: usize = 8;
pub const C_A: usize = 9;
pub const C_T: usize = 10;
pub const C_CNT: usize = 11;
/// REQ row columns.
pub const C_OVF: usize = 10;
pub const C_PRE: usize = 11;
/// Register column of `len` (copied to MEM's first address).
pub const C_LEN: usize = 6;
/// Columns with copy constraints: post digest 0, MEM address 1, `len`, and the
/// count and pre digest (both column 11).
pub const EQ_COLS: [usize; 4] = [0, 1, C_LEN, C_CNT];

/// Placeholder digest weight (see [`crate::trace::digest`]).
fn lambda() -> F {
    F::from(1_000_003u64)
}

#[derive(Clone, Debug)]
pub struct CoreConfig {
    pub bus: BusConfig,
    pub dec: DecConfig,
    pub q_step: Selector,
    pub q_dec: Selector,
    pub q_cnt0: Selector,
    pub q_pre: Selector,
    pub q_fin: Selector,
}

/// One slot's witness: pre registers, flags `(push, arith, ret)`, carry, and the
/// two request tuples (zero when inactive).
#[derive(Clone, Copy, Debug)]
pub struct SlotWitness {
    pub pre: Regs,
    pub flags: [u64; 3],
    pub ovf: u64,
    pub u1: Tuple,
    pub u2: Tuple,
}

/// Cells the leaf ties to the statement and to MEM.
pub struct CoreCells {
    pub len0: AssignedCell<F, F>,
    pub pre_d: AssignedCell<F, F>,
    pub post_d: AssignedCell<F, F>,
    pub steps: AssignedCell<F, F>,
}

fn digest_expr(m: &mut VirtualCells<'_, F>, bus: &BusConfig, rot: i32) -> Expression<F> {
    let mut acc = Expression::Constant(F::from(0u64));
    for i in (0..NREG).rev() {
        acc = acc * Expression::Constant(lambda()) + m.query_advice(bus.cols.a[i], Rotation(rot));
    }
    acc
}

impl CoreConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig) -> Self {
        let a = bus.cols.a;
        let q_step = meta.selector();
        let q_dec = meta.complex_selector();
        let q_cnt0 = meta.selector();
        let q_pre = meta.selector();
        let q_fin = meta.selector();
        let k = |v: u64| Expression::Constant(F::from(v));
        let dec = DecConfig::configure(meta, |m| {
            let q = m.query_selector(q_dec);
            let tag = m.query_advice(a[0], Rotation::cur());
            let p = m.query_advice(a[C_P], Rotation::cur());
            let ar = m.query_advice(a[C_A], Rotation::cur());
            let t = m.query_advice(a[C_T], Rotation::cur());
            let en = p.clone() + ar.clone() + t.clone();
            let rule = p + ar * k(2) + t * k(3);
            (q.clone() * en * tag, q * rule)
        });
        meta.create_gate("core step", |m| {
            let q = m.query_selector(q_step);
            let at = |m: &mut VirtualCells<'_, F>, i: usize, rot: i32| {
                m.query_advice(a[i], Rotation(rot))
            };
            // This slot's REG row (-1), the REQ row (0), the next REG row (+1).
            let (tag, ca, cb, fp, depth, limit, len, r) = (
                at(m, 0, -1),
                at(m, 1, -1),
                at(m, 2, -1),
                at(m, 3, -1),
                at(m, 4, -1),
                at(m, 5, -1),
                at(m, 6, -1),
                at(m, 7, -1),
            );
            let (p, ar, t, cnt) = (
                at(m, C_P, -1),
                at(m, C_A, -1),
                at(m, C_T, -1),
                at(m, C_CNT, -1),
            );
            let en = p.clone() + ar.clone() + t.clone();
            let en_next = at(m, C_P, 1) + at(m, C_A, 1) + at(m, C_T, 1);
            let cnt_next = at(m, C_CNT, 1);
            let u1: Vec<_> = (0..WIDTH).map(|i| at(m, i, 0)).collect();
            let u2: Vec<_> = (0..WIDTH).map(|i| at(m, WIDTH + i, 0)).collect();
            let ovf = at(m, C_OVF, 0);
            let nx: Vec<_> = (0..NREG).map(|i| at(m, i, 1)).collect();
            let one = k(1);
            let off = one.clone() - en.clone();
            let pt = p.clone() + t.clone();
            let z = u1[4].clone();
            let (saved, link) = (u2[3].clone(), u2[4].clone());
            let no_ovf = one.clone() - ovf.clone();
            let bool_ = |x: Expression<F>| x.clone() * (one.clone() - x);
            let mut c = vec![
                bool_(p.clone()),
                bool_(ar.clone()),
                bool_(t.clone()),
                bool_(en.clone()),
                en_next * off.clone(),
                cnt_next - cnt - en.clone(),
                // Lane 0: ALU.
                u1[0].clone() - en.clone() * k(CHIP_ALU),
                u1[1].clone()
                    - pt.clone() * k(OP_LT)
                    - ar.clone() * (k(OP_ADD) + ovf.clone() * k(FLAG_SHIFT)),
                u1[2].clone()
                    - p.clone() * depth.clone()
                    - ar.clone() * ca.clone()
                    - t.clone() * fp.clone(),
                u1[3].clone()
                    - p.clone() * limit.clone()
                    - ar.clone() * cb.clone()
                    - t.clone() * len.clone(),
                pt.clone() * (z.clone() - one.clone()),
                bool_(ovf.clone()),
                (one.clone() - ar.clone()) * ovf.clone(),
                // Lane 1: MEM.
                u2[0].clone() - pt.clone() * k(CHIP_MEM),
                u2[1].clone() - p.clone() * len.clone() - t.clone() * fp.clone(),
                pt.clone() * (u2[2].clone() - k(FRAME_RIGHT)),
                p.clone() * (u2[3].clone() - ca.clone()),
                p.clone() * (u2[4].clone() - fp.clone()),
            ];
            // Next state.
            let next = [
                off.clone() * tag
                    + p.clone() * k(CTL_RET)
                    + ar.clone() * (k(CTL_PUSH) + ovf.clone() * (k(CTL_ERR) - k(CTL_PUSH)))
                    + t.clone() * k(CTL_ARITH),
                off.clone() * ca + ar.clone() * no_ovf.clone() * z.clone() + t.clone() * saved,
                off.clone() * cb.clone() + ar.clone() * no_ovf * cb.clone() + t.clone() * r.clone(),
                off.clone() * fp.clone()
                    + p.clone() * len.clone()
                    + ar.clone() * fp
                    + t.clone() * link,
                depth + p.clone() - t.clone(),
                limit,
                len + p.clone(),
                off * r.clone() + p * cb + ar * z + t * r,
            ];
            for (n, e) in nx.into_iter().zip(next) {
                c.push(n - e);
            }
            crate::gate(c.into_iter().map(|e| q.clone() * e).collect())
        });
        meta.create_gate("core count0", |m| {
            let q = m.query_selector(q_cnt0);
            let cnt = m.query_advice(a[C_CNT], Rotation::cur());
            crate::gate(vec![q * cnt])
        });
        meta.create_gate("core pre digest", |m| {
            let q = m.query_selector(q_pre);
            let d = m.query_advice(a[C_PRE], Rotation::cur());
            crate::gate(vec![q * (d - digest_expr(m, &bus, -1))])
        });
        meta.create_gate("core post digest", |m| {
            let q = m.query_selector(q_fin);
            let d = m.query_advice(a[0], Rotation::cur());
            crate::gate(vec![q * (d - digest_expr(m, &bus, -1))])
        });
        Self {
            bus,
            dec,
            q_step,
            q_dec,
            q_cnt0,
            q_pre,
            q_fin,
        }
    }

    pub fn load(&self, l: &mut impl midnight_proofs::circuit::Layouter<F>) -> Result<(), Error> {
        self.dec.load(l, &DEC_ROWS)
    }

    /// Rows the core region takes for `s` slots.
    pub fn rows(s: usize) -> usize {
        H_CORE * s + 2
    }

    /// A REG row: registers, flags and the running count.
    fn reg_row(
        &self,
        region: &mut Region<'_, F>,
        o: usize,
        x: Value<Regs>,
        flags: Value<[u64; 3]>,
        cnt: Value<F>,
    ) -> Result<(Vec<AssignedCell<F, F>>, AssignedCell<F, F>), Error> {
        let a = self.bus.cols.a;
        let cells = (0..NREG)
            .map(|i| region.assign_advice(|| "reg", a[i], o, || x.map(|x| x.words()[i])))
            .collect::<Result<Vec<_>, _>>()?;
        for (j, col) in [C_P, C_A, C_T].into_iter().enumerate() {
            region.assign_advice(|| "flag", a[col], o, || flags.map(|f| F::from(f[j])))?;
        }
        let c = region.assign_advice(|| "cnt", a[C_CNT], o, || cnt)?;
        Ok((cells, c))
    }

    /// Assign the core at `offset`: `slots[i]` for each slot and the final state.
    pub fn assign(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        slots: &[Value<SlotWitness>],
        post: Value<Regs>,
    ) -> Result<CoreCells, Error> {
        let a = self.bus.cols.a;
        let s = slots.len();
        let mut len0 = None;
        let mut pre_d = None;
        let mut cnt = Value::known(F::from(0u64));
        for (i, w) in slots.iter().enumerate() {
            let o = offset + H_CORE * i;
            self.q_dec.enable(region, o)?;
            self.q_step.enable(region, o + 1)?;
            if i == 0 {
                self.q_cnt0.enable(region, o)?;
                self.q_pre.enable(region, o + 1)?;
            }
            let (cells, _) = self.reg_row(region, o, w.map(|w| w.pre), w.map(|w| w.flags), cnt)?;
            if i == 0 {
                len0 = Some(cells[C_LEN].clone());
            }
            self.bus
                .request_pair(region, o + 1, w.map(|w| w.u1), w.map(|w| w.u2))?;
            region.assign_advice(|| "ovf", a[C_OVF], o + 1, || w.map(|w| F::from(w.ovf)))?;
            if i == 0 {
                let d = w.map(|w| crate::trace::digest(&w.pre));
                pre_d = Some(region.assign_advice(|| "pre_d", a[C_PRE], o + 1, || d)?);
            }
            cnt = cnt + w.map(|w| F::from(w.flags.iter().sum::<u64>()));
        }
        let o = offset + H_CORE * s;
        let (_, steps) = self.reg_row(region, o, post, Value::known([0; 3]), cnt)?;
        self.q_fin.enable(region, o + 1)?;
        let post_d = region.assign_advice(
            || "post_d",
            a[0],
            o + 1,
            || post.map(|x| crate::trace::digest(&x)),
        )?;
        Ok(CoreCells {
            len0: len0.expect("s >= 1"),
            pre_d: pre_d.expect("s >= 1"),
            post_d,
            steps,
        })
    }
}

#[cfg(test)]
mod tests {
    //! CORE with the bus and DEC; every request is served by a stub provider row
    //! that copies it, so these tests exercise the core's own gates only.
    use midnight_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        dev::MockProver,
        plonk::Circuit,
    };

    use super::*;
    use crate::{
        Cols,
        leaf::LeafWitness,
        trace::{Regs, run},
    };

    #[derive(Clone)]
    struct CoreOnly {
        w: LeafWitness,
    }

    impl Circuit<F> for CoreOnly {
        type Config = CoreConfig;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> CoreConfig {
            let cols = Cols::configure(meta);
            let bus = BusConfig::configure(meta, cols);
            CoreConfig::configure(meta, bus)
        }
        fn synthesize(&self, cfg: CoreConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
            cfg.load(&mut l)?;
            l.assign_region(
                || "core",
                |mut r| {
                    let slots: Vec<_> = self.w.slots.iter().map(|s| Value::known(*s)).collect();
                    cfg.assign(&mut r, 0, &slots, Value::known(self.w.post))?;
                    let mut o = CoreConfig::rows(slots.len());
                    for s in &self.w.slots {
                        for t in [s.u1, s.u2] {
                            if t[0] != F::from(0u64) {
                                cfg.bus.provide(&mut r, o, Value::known(t))?;
                                o += 1;
                            }
                        }
                    }
                    Ok(())
                },
            )
        }
    }

    fn ok(w: &LeafWitness) -> bool {
        MockProver::run(&CoreOnly { w: w.clone() }, vec![])
            .map(|p| p.verify().is_ok())
            .unwrap_or(false)
    }

    fn honest(s: usize) -> LeafWitness {
        LeafWitness::from_trace(&run(Regs::genesis(), s), s)
    }

    #[test]
    fn honest_core_passes() {
        assert!(ok(&honest(6)));
        assert!(ok(&LeafWitness::from_trace(&run(Regs::genesis(), 2), 4)));
    }

    #[test]
    fn decoder_refuses_wrong_rule() {
        // The push slot claims the arithmetic rule.
        let mut w = honest(1);
        w.slots[0].flags = [0, 1, 0];
        assert!(!ok(&w));
    }

    #[test]
    fn frame_condition_holds() {
        // An arithmetic step must not move fp.
        let mut w = honest(3);
        w.post.fp += 1;
        assert!(!ok(&w));
        // A disabled slot must not change a register.
        let mut w = LeafWitness::from_trace(&run(Regs::genesis(), 1), 2);
        w.post.depth += 1;
        assert!(!ok(&w));
    }

    #[test]
    fn enable_is_a_prefix() {
        let mut w = LeafWitness::from_trace(&run(Regs::genesis(), 1), 2);
        // Disable slot 0 and enable slot 1 with the same step.
        let s0 = w.slots[0];
        w.slots[1] = s0;
        w.slots[0] = SlotWitness {
            pre: s0.pre,
            flags: [0, 0, 0],
            ovf: 0,
            u1: [F::from(0u64); 5],
            u2: [F::from(0u64); 5],
        };
        assert!(!ok(&w));
    }
}
