//! ALU: 128-bit checked add and less-than, one bus provider row per operation.
//!
//! Tuple `(ALU, key, x, y, z)` with `key = op + 8 * flag`. The tuple and the
//! auxiliary columns 5 = op, 6 = flag sit on the centre row of the decomposition. One 128-bit decomposition `w` per operation:
//! - ADD: `x + y = z + flag * 2^128`, `w = z` (so `z < 2^128`, flag = carry);
//! - LT:  `flag = 0`, `z` boolean, `w = z (y - x - 1) + (1 - z)(x - y)`.
//!
//! For operands below 2^128 (the requester's invariant) `z` and `flag` are unique,
//! so the chip is a function of `(op, x, y)`.
use midnight_proofs::{
    circuit::{Region, Value},
    plonk::{ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

use crate::{
    F,
    bus::{BusConfig, CHIP_ALU, tuple},
    fu,
    range::RangeConfig,
};

pub const OP_ADD: u64 = 1;
pub const OP_LT: u64 = 2;
pub const FLAG_SHIFT: u64 = 8;

/// One ALU operation and its result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AluOp {
    pub op: u64,
    pub x: u128,
    pub y: u128,
    pub flag: u64,
    pub z: u128,
}

impl AluOp {
    /// The honest result of `op(x, y)` for `x, y < 2^128`.
    pub fn eval(op: u64, x: u128, y: u128) -> Self {
        match op {
            OP_ADD => {
                let (z, carry) = x.overflowing_add(y);
                Self {
                    op,
                    x,
                    y,
                    flag: carry as u64,
                    z,
                }
            }
            OP_LT => Self {
                op,
                x,
                y,
                flag: 0,
                z: (x < y) as u128,
            },
            _ => panic!("unknown ALU op {op}"),
        }
    }

    pub fn key(&self) -> F {
        F::from(self.op + FLAG_SHIFT * self.flag)
    }

    /// The decomposed witness `w`.
    pub fn w(&self) -> u128 {
        match self.op {
            OP_ADD => self.z,
            _ if self.z == 1 => self.y - self.x - 1,
            _ => self.x - self.y,
        }
    }

    pub fn tuple(&self) -> crate::bus::Tuple {
        tuple(CHIP_ALU, self.key(), fu(self.x), fu(self.y), fu(self.z))
    }
}

#[derive(Clone, Debug)]
pub struct AluConfig {
    pub q_alu: Selector,
    pub bus: BusConfig,
    pub range: RangeConfig,
}

impl AluConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig, range: RangeConfig) -> Self {
        let q_alu = meta.selector();
        let a = bus.cols.a;
        let r2 = range.clone();
        meta.create_gate("alu", |m| {
            let q = m.query_selector(q_alu);
            let c = |m: &mut midnight_proofs::plonk::VirtualCells<'_, F>, i: usize| {
                m.query_advice(a[i], Rotation::cur())
            };
            let (chip, key, x, y, z, op, flag) = (
                c(m, 0),
                c(m, 1),
                c(m, 2),
                c(m, 3),
                c(m, 4),
                c(m, 5),
                c(m, 6),
            );
            let k = |v: u64| Expression::Constant(F::from(v));
            let one = k(1);
            let two128 = Expression::Constant(fu(u128::MAX) + F::from(1u64));
            let is_add = k(2) - op.clone();
            let is_lt = op.clone() - one.clone();
            let w = is_add.clone() * z.clone()
                + is_lt.clone()
                    * (z.clone() * (y.clone() - x.clone() - one.clone())
                        + (one.clone() - z.clone()) * (x.clone() - y.clone()));
            let (sum, pad) = r2.recompose(m);
            crate::gate(vec![
                q.clone() * (chip - k(CHIP_ALU)),
                q.clone() * (key - op.clone() - flag.clone() * k(FLAG_SHIFT)),
                q.clone() * (op.clone() - k(OP_ADD)) * (op - k(OP_LT)),
                q.clone() * flag.clone() * (one.clone() - flag.clone()),
                q.clone() * is_add * (x + y - z.clone() - flag.clone() * two128),
                q.clone() * is_lt.clone() * flag,
                q.clone() * is_lt * z.clone() * (one - z),
                q.clone() * (w - sum),
                q * pad,
            ])
        });
        Self { q_alu, bus, range }
    }

    pub fn rows_per_op(&self) -> usize {
        self.range.rows()
    }

    /// Assign one operation as a provider at `offset`; returns the rows used.
    pub fn assign(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        op: Value<AluOp>,
    ) -> Result<usize, Error> {
        let c = offset + self.range.center();
        self.q_alu.enable(region, c)?;
        self.bus.provide(region, c, op.map(|o| o.tuple()))?;
        let a = self.bus.cols.a;
        region.assign_advice(|| "op", a[5], c, || op.map(|o| F::from(o.op)))?;
        region.assign_advice(|| "flag", a[6], c, || op.map(|o| F::from(o.flag)))?;
        self.range.assign(region, offset, op.map(|o| o.w()))?;
        Ok(self.rows_per_op())
    }

    /// As [`Self::assign`] with a forged result `z` (negative tests): the
    /// decomposition is of the forged `w`, so only the arithmetic gate can catch it.
    pub fn assign_forged(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        op: AluOp,
    ) -> Result<usize, Error> {
        let c = offset + self.range.center();
        self.q_alu.enable(region, c)?;
        self.bus.provide(region, c, Value::known(op.tuple()))?;
        let a = self.bus.cols.a;
        region.assign_advice(|| "op", a[5], c, || Value::known(F::from(op.op)))?;
        region.assign_advice(|| "flag", a[6], c, || Value::known(F::from(op.flag)))?;
        let w = if op.op == OP_ADD { op.z } else { 0 };
        self.range.assign(region, offset, Value::known(w))?;
        Ok(self.rows_per_op())
    }
}

#[cfg(test)]
mod tests {
    //! ALU with RANGE and the bus; requests are raw rows.
    use midnight_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        dev::MockProver,
        plonk::Circuit,
    };

    use super::*;
    use crate::{Cols, bus::Tuple};

    /// `RUN`: RANGE in running-sum mode.
    #[derive(Clone)]
    struct AluOnly<const RUN: bool> {
        ops: Vec<AluOp>,
        forged: Vec<AluOp>,
        requests: Vec<Tuple>,
    }

    impl<const RUN: bool> Circuit<F> for AluOnly<RUN> {
        type Config = AluConfig;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> AluConfig {
            let cols = Cols::configure(meta);
            let bus = BusConfig::configure(meta, cols);
            let range = if RUN {
                RangeConfig::configure_running(meta, cols)
            } else {
                RangeConfig::configure(meta, cols, 4)
            };
            AluConfig::configure(meta, bus, range)
        }
        fn synthesize(&self, cfg: AluConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
            cfg.range.load(&mut l)?;
            l.assign_region(
                || "alu",
                |mut r| {
                    let mut o = 0;
                    for op in &self.ops {
                        o += cfg.assign(&mut r, o, Value::known(*op))?;
                    }
                    for op in &self.forged {
                        o += cfg.assign_forged(&mut r, o, *op)?;
                    }
                    for t in &self.requests {
                        cfg.bus.request(&mut r, o, Value::known(*t))?;
                        o += 1;
                    }
                    Ok(())
                },
            )
        }
    }

    fn ok_mode<const RUN: bool>(c: &AluOnly<RUN>) -> bool {
        MockProver::run(c, vec![])
            .map(|p| p.verify().is_ok())
            .unwrap_or(false)
    }

    /// The same verdict with 4 limb columns and with the running sum.
    fn ok(c: &AluOnly<false>) -> bool {
        let r = AluOnly::<true> {
            ops: c.ops.clone(),
            forged: c.forged.clone(),
            requests: c.requests.clone(),
        };
        let (a, b) = (ok_mode(c), ok_mode(&r));
        assert_eq!(a, b, "RANGE modes disagree");
        a
    }

    #[test]
    fn honest_ops_serve_requests() {
        let ops = vec![
            AluOp::eval(OP_ADD, 3, 4),
            AluOp::eval(OP_ADD, u128::MAX, 2),
            AluOp::eval(OP_LT, 3, 4),
            AluOp::eval(OP_LT, 4, 4),
            AluOp::eval(OP_LT, u128::MAX, 0),
        ];
        let requests = ops.iter().map(|o| o.tuple()).collect();
        assert!(ok(&AluOnly {
            ops,
            forged: vec![],
            requests
        }));
    }

    #[test]
    fn wrong_sum_rejected() {
        let mut bad = AluOp::eval(OP_ADD, 3, 4);
        bad.z = 8;
        assert!(!ok(&AluOnly {
            ops: vec![],
            forged: vec![bad],
            requests: vec![bad.tuple()]
        }));
    }

    #[test]
    fn wrong_comparison_rejected() {
        let mut bad = AluOp::eval(OP_LT, 5, 4);
        bad.z = 1;
        assert!(!ok(&AluOnly {
            ops: vec![],
            forged: vec![bad],
            requests: vec![bad.tuple()]
        }));
    }

    #[test]
    fn request_without_matching_op_rejected() {
        let good = AluOp::eval(OP_ADD, 3, 4);
        let mut req = good;
        req.z = 8;
        assert!(!ok(&AluOnly {
            ops: vec![good],
            forged: vec![],
            requests: vec![req.tuple()]
        }));
    }
}
