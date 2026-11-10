//! RANGE: a fixed table of 12-bit values and the 128-bit decomposition shape.
//!
//! A 128-bit value is 11 limbs of 12 bits (132 bits) whose top limb is below 2^8,
//! checked through a twelfth cell `pad = 16 * limb10` that is also in the table.
//! The 12 cells are laid row-major over `rc` limb columns (the last `rc` shared
//! advice columns), so one value takes `ceil(12 / rc)` rows. Each limb column has
//! its own lookup argument into the table; `rc` trades rows for lookup arguments.
//! The user's gate sits on the centre row of the value and reaches the limbs at
//! rotations -1, 0 and +1 (the in-circuit verifier supports no other rotation), so
//! a value takes at most 3 rows: `rc` is 4 or 5.
//!
//! Running-sum mode ([`RangeConfig::configure_running`], experiment E7): one limb
//! column (column 11) and one lookup argument. A value takes 12 rows: row 0 holds
//! the pad cell, rows 1..=11 the limbs from the top (`limb10`) down to `limb0`,
//! and column 10 a Horner running sum `acc_1 = limb10`,
//! `acc_t = 4096 acc_(t-1) + limb_(11-t)`, with `pad = 16 limb10` checked on row 1.
//! The value is `acc_11` on row 11, which carries the user's gate. A lookup
//! argument costs the node about 22,600 rows and the 12 rows per value cost it
//! nothing (`experiments-2.md` 3.3).
use midnight_proofs::{
    circuit::{Layouter, Region, Value},
    plonk::{
        Advice, Column, ConstraintSystem, Error, Expression, Selector, TableColumn, VirtualCells,
    },
    poly::Rotation,
};

use crate::{Cols, F, NCOL};

pub const LIMB_BITS: u32 = 12;
pub const LIMBS: usize = 11;
/// Limb cells per value: 11 limbs and the top-limb pad.
pub const CELLS: usize = 12;
pub const TABLE_ROWS: usize = 1 << LIMB_BITS;

#[derive(Clone, Debug)]
pub struct RangeConfig {
    pub table: TableColumn,
    pub q_rng: Selector,
    pub limb_cols: Vec<Column<Advice>>,
    /// Running-sum mode.
    pub run: Option<RunCfg>,
}

/// Running-sum mode: the accumulator column and its two gates.
#[derive(Clone, Debug)]
pub struct RunCfg {
    pub acc: Column<Advice>,
    pub q_first: Selector,
    pub q_step: Selector,
}

/// Rows one 128-bit value takes with `rc` limb columns.
pub fn rows_per_value(rc: usize) -> usize {
    CELLS.div_ceil(rc)
}

/// The 12 cells of `v < 2^128`.
pub fn cells(v: u128) -> [u64; CELLS] {
    let mut out = [0u64; CELLS];
    for (i, c) in out.iter_mut().take(LIMBS).enumerate() {
        *c = ((v >> (LIMB_BITS as usize * i)) & 0xfff) as u64;
    }
    out[LIMBS] = 16 * out[LIMBS - 1];
    out
}

impl RangeConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols, rc: usize) -> Self {
        assert!(
            (4..=5).contains(&rc),
            "4 or 5 limb columns: at most 3 rows per value, columns 0..7 free"
        );
        let table = meta.lookup_table_column();
        let q_rng = meta.complex_selector();
        let limb_cols: Vec<_> = cols.a[NCOL - rc..].to_vec();
        for &col in &limb_cols {
            meta.lookup("range", |m| {
                let q = m.query_selector(q_rng);
                let v = m.query_advice(col, Rotation::cur());
                vec![(q * v, table)]
            });
        }
        Self {
            table,
            q_rng,
            limb_cols,
            run: None,
        }
    }

    /// Running-sum mode: limbs in column 11, accumulator in column 10.
    pub fn configure_running(meta: &mut ConstraintSystem<F>, cols: Cols) -> Self {
        let table = meta.lookup_table_column();
        let q_rng = meta.complex_selector();
        let limb = cols.a[NCOL - 1];
        let acc = cols.a[NCOL - 2];
        meta.lookup("range", |m| {
            let q = m.query_selector(q_rng);
            let v = m.query_advice(limb, Rotation::cur());
            vec![(q * v, table)]
        });
        let q_first = meta.selector();
        let q_step = meta.selector();
        meta.create_gate("range run", |m| {
            let qf = m.query_selector(q_first);
            let qs = m.query_selector(q_step);
            let l = m.query_advice(limb, Rotation::cur());
            let pad = m.query_advice(limb, Rotation::prev());
            let a = m.query_advice(acc, Rotation::cur());
            let ap = m.query_advice(acc, Rotation::prev());
            let k = |v: u64| Expression::Constant(F::from(v));
            crate::gate(vec![
                qf.clone() * (a.clone() - l.clone()),
                qf * (pad - l.clone() * k(16)),
                qs * (a - ap * k(1 << LIMB_BITS) - l),
            ])
        });
        Self {
            table,
            q_rng,
            limb_cols: vec![limb],
            run: Some(RunCfg {
                acc,
                q_first,
                q_step,
            }),
        }
    }

    pub fn rc(&self) -> usize {
        self.limb_cols.len()
    }

    /// Rows one value takes.
    pub fn rows(&self) -> usize {
        if self.run.is_some() {
            CELLS
        } else {
            rows_per_value(self.rc())
        }
    }

    /// Row of a value (from its first row) that carries the user's gate.
    pub fn center(&self) -> usize {
        if self.run.is_some() {
            CELLS - 1
        } else {
            (rows_per_value(self.rc()) - 1) / 2
        }
    }

    pub fn load(&self, l: &mut impl Layouter<F>) -> Result<(), Error> {
        l.assign_table(
            || "range table",
            |mut t| {
                for i in 0..TABLE_ROWS {
                    t.assign_cell(|| "r", self.table, i, || Value::known(F::from(i as u64)))?;
                }
                Ok(())
            },
        )
    }

    /// Place cell `i` of a value starting at row `offset`.
    pub fn place(&self, i: usize) -> (usize, Column<Advice>) {
        (i / self.rc(), self.limb_cols[i % self.rc()])
    }

    /// Assign the 12 cells of a value at rows `offset..offset + rows_per_value`.
    pub fn assign(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        v: Value<u128>,
    ) -> Result<(), Error> {
        if self.run.is_some() {
            return self.assign_running(region, offset, v.map(cells));
        }
        for r in 0..rows_per_value(self.rc()) {
            self.q_rng.enable(region, offset + r)?;
        }
        for i in 0..CELLS {
            let (dr, col) = self.place(i);
            region.assign_advice(
                || "limb",
                col,
                offset + dr,
                || v.map(|v| F::from(cells(v)[i])),
            )?;
        }
        // Unused trailing cells of the last row are zero.
        for i in CELLS..rows_per_value(self.rc()) * self.rc() {
            let (dr, col) = self.place(i);
            region.assign_advice(
                || "limb pad",
                col,
                offset + dr,
                || Value::known(F::from(0u64)),
            )?;
        }
        Ok(())
    }

    /// Running-sum mode: the 12 cells `c` (honest or forged) from row `offset`,
    /// with the accumulator computed from them.
    pub fn assign_running(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        c: Value<[u64; CELLS]>,
    ) -> Result<(), Error> {
        let run = self.run.as_ref().expect("running mode");
        let limb = self.limb_cols[0];
        let mut acc = c.map(|_| F::from(0u64));
        for t in 0..CELLS {
            self.q_rng.enable(region, offset + t)?;
            // Row 0: pad; row t >= 1: limb 11 - t.
            let x = c.map(|c| F::from(if t == 0 { c[LIMBS] } else { c[CELLS - 1 - t] }));
            region.assign_advice(|| "limb", limb, offset + t, || x)?;
            if t >= 1 {
                acc = acc.zip(x).map(|(a, x)| a * F::from(1u64 << LIMB_BITS) + x);
            }
            if t == 1 {
                run.q_first.enable(region, offset + t)?;
            } else if t >= 2 {
                run.q_step.enable(region, offset + t)?;
            }
            region.assign_advice(|| "acc", run.acc, offset + t, || acc)?;
        }
        Ok(())
    }

    /// `(sum of limbs * 2^(12 i)) , pad - 16 * limb10` over a value whose centre
    /// row is the current row: the gate of the user constrains the first to the
    /// value and the second to zero. Running-sum mode: `(acc, 0)` (the pad is
    /// checked by the RANGE gate).
    pub fn recompose(&self, m: &mut VirtualCells<'_, F>) -> (Expression<F>, Expression<F>) {
        if let Some(run) = &self.run {
            return (
                m.query_advice(run.acc, Rotation::cur()),
                Expression::Constant(F::from(0u64)),
            );
        }
        let cell = |m: &mut VirtualCells<'_, F>, i: usize| {
            let (dr, col) = self.place(i);
            m.query_advice(col, Rotation(dr as i32 - self.center() as i32))
        };
        let mut sum = Expression::Constant(F::from(0u64));
        let mut pow = F::from(1u64);
        for i in 0..LIMBS {
            sum = sum + cell(m, i) * Expression::Constant(pow);
            pow *= F::from(1u64 << LIMB_BITS);
        }
        let pad = cell(m, LIMBS) - cell(m, LIMBS - 1) * Expression::Constant(F::from(16u64));
        (sum, pad)
    }
}

#[cfg(test)]
mod tests {
    //! RANGE alone: a value column and its decomposition, in both modes.
    use midnight_proofs::{circuit::SimpleFloorPlanner, dev::MockProver, plonk::Circuit};

    use super::*;
    use crate::fu;

    /// `RUN`: running-sum mode (1 limb column) instead of 4 limb columns.
    #[derive(Clone)]
    struct RangeOnly<const RUN: bool> {
        value: u128,
        /// Cells to write instead of the honest decomposition.
        forged: Option<[u64; CELLS]>,
        claimed: F,
    }

    #[derive(Clone)]
    struct Cfg {
        cols: Cols,
        range: RangeConfig,
        q: Selector,
    }

    impl<const RUN: bool> Circuit<F> for RangeOnly<RUN> {
        type Config = Cfg;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> Cfg {
            let cols = Cols::configure(meta);
            let range = if RUN {
                RangeConfig::configure_running(meta, cols)
            } else {
                RangeConfig::configure(meta, cols, 4)
            };
            let q = meta.selector();
            let r2 = range.clone();
            meta.create_gate("value", |m| {
                let q = m.query_selector(q);
                let v = m.query_advice(cols.a[0], Rotation::cur());
                let (sum, pad) = r2.recompose(m);
                crate::gate(vec![q.clone() * (v - sum), q * pad])
            });
            Cfg { cols, range, q }
        }
        fn synthesize(&self, cfg: Cfg, mut l: impl Layouter<F>) -> Result<(), Error> {
            cfg.range.load(&mut l)?;
            l.assign_region(
                || "v",
                |mut r| {
                    let c = cfg.range.center();
                    cfg.q.enable(&mut r, c)?;
                    r.assign_advice(|| "v", cfg.cols.a[0], c, || Value::known(self.claimed))?;
                    match self.forged {
                        None => cfg.range.assign(&mut r, 0, Value::known(self.value)),
                        Some(c) if RUN => cfg.range.assign_running(&mut r, 0, Value::known(c)),
                        Some(c) => {
                            for r_ in 0..3 {
                                cfg.range.q_rng.enable(&mut r, r_)?;
                            }
                            for (i, x) in c.iter().enumerate() {
                                let (dr, col) = cfg.range.place(i);
                                r.assign_advice(|| "l", col, dr, || Value::known(F::from(*x)))?;
                            }
                            Ok(())
                        }
                    }
                },
            )
        }
    }

    fn ok_mode<const RUN: bool>(value: u128, forged: Option<[u64; CELLS]>, claimed: F) -> bool {
        MockProver::run(
            &RangeOnly::<RUN> {
                value,
                forged,
                claimed,
            },
            vec![],
        )
        .map(|p| p.verify().is_ok())
        .unwrap_or(false)
    }

    /// The same verdict in both modes.
    fn ok(value: u128, forged: Option<[u64; CELLS]>, claimed: F) -> bool {
        let a = ok_mode::<false>(value, forged, claimed);
        let b = ok_mode::<true>(value, forged, claimed);
        assert_eq!(a, b, "modes disagree on {value} {forged:?}");
        a
    }

    #[test]
    fn values_below_2_128_pass() {
        for v in [0u128, 1, 4095, 4096, u64::MAX as u128, u128::MAX] {
            assert!(ok(v, None, fu(v)), "{v}");
        }
    }

    #[test]
    fn wrong_value_rejected() {
        assert!(!ok(5, None, fu(6)));
    }

    #[test]
    fn value_at_2_128_rejected() {
        // 2^128 = limb10 = 256, outside the 8-bit top limb; pad = 4096 is off table.
        let mut c = [0u64; CELLS];
        c[10] = 256;
        c[11] = 4096;
        let claimed = fu(u128::MAX) + F::from(1u64);
        assert!(!ok(0, Some(c), claimed));
    }

    #[test]
    fn limb_out_of_table_rejected() {
        // 4096 = 4096 * 2^0 written as one limb.
        let mut c = [0u64; CELLS];
        c[0] = 4096;
        assert!(!ok(0, Some(c), fu(4096)));
    }

    /// A pad cell that is in the table but not `16 * limb10`.
    #[test]
    fn wrong_pad_rejected() {
        let mut c = cells(5u128 << 120);
        assert!(ok(0, Some(c), fu(5u128 << 120)));
        c[11] += 1;
        assert!(!ok(0, Some(c), fu(5u128 << 120)));
    }

    /// Running mode: rows and the gate row.
    #[test]
    fn running_shape() {
        let mut cs = ConstraintSystem::<F>::default();
        let cols = Cols::configure(&mut cs);
        let r = RangeConfig::configure_running(&mut cs, cols);
        assert_eq!((r.rows(), r.center(), r.rc()), (12, 11, 1));
        assert_eq!(cs.lookups().len(), 1);
    }
}
