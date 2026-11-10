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
        }
    }

    pub fn rc(&self) -> usize {
        self.limb_cols.len()
    }

    /// Row of a value (from its first row) that carries the user's gate.
    pub fn center(&self) -> usize {
        (rows_per_value(self.rc()) - 1) / 2
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

    /// `(sum of limbs * 2^(12 i)) , pad - 16 * limb10` over a value whose centre
    /// row is the current row: the gate of the user constrains the first to the
    /// value and the second to zero.
    pub fn recompose(&self, m: &mut VirtualCells<'_, F>) -> (Expression<F>, Expression<F>) {
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
    //! RANGE alone: a value column and its decomposition.
    use midnight_proofs::{circuit::SimpleFloorPlanner, dev::MockProver, plonk::Circuit};

    use super::*;
    use crate::fu;

    #[derive(Clone)]
    struct RangeOnly {
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

    impl Circuit<F> for RangeOnly {
        type Config = Cfg;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> Cfg {
            let cols = Cols::configure(meta);
            let range = RangeConfig::configure(meta, cols, 4);
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

    fn ok(c: &RangeOnly) -> bool {
        MockProver::run(c, vec![])
            .map(|p| p.verify().is_ok())
            .unwrap_or(false)
    }

    #[test]
    fn values_below_2_128_pass() {
        for v in [0u128, 1, 4095, 4096, u64::MAX as u128, u128::MAX] {
            assert!(
                ok(&RangeOnly {
                    value: v,
                    forged: None,
                    claimed: fu(v)
                }),
                "{v}"
            );
        }
    }

    #[test]
    fn wrong_value_rejected() {
        assert!(!ok(&RangeOnly {
            value: 5,
            forged: None,
            claimed: fu(6)
        }));
    }

    #[test]
    fn value_at_2_128_rejected() {
        // 2^128 = limb10 = 256, outside the 8-bit top limb; pad = 4096 is off table.
        let mut c = [0u64; CELLS];
        c[10] = 256;
        c[11] = 4096;
        let claimed = fu(u128::MAX) + F::from(1u64);
        assert!(!ok(&RangeOnly {
            value: 0,
            forged: Some(c),
            claimed
        }));
    }

    #[test]
    fn limb_out_of_table_rejected() {
        // 4096 = 4096 * 2^0 written as one limb.
        let mut c = [0u64; CELLS];
        c[0] = 4096;
        assert!(!ok(&RangeOnly {
            value: 0,
            forged: Some(c),
            claimed: fu(4096)
        }));
    }
}
