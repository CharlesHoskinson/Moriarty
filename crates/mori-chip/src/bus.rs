//! The bus: one tuple format `(chip, key, a, b, c)`. Providers hold their tuple in
//! shared columns 0..5; a request row holds two request lanes, columns 0..5 and
//! 5..10.
//!
//! A request row sets the fixed column `q_req`; a provider row (a chip row that
//! serves requests) sets `q_prov`. One `lookup_any` per lane asserts that the lane's
//! tuple, scaled by `q_req`, is among the tuples of all rows scaled by `q_prov`. Two
//! lanes let the core issue both requests of a step from one row, so every core
//! gate reaches its neighbours at rotations -1, 0 and +1 only (the in-circuit
//! verifier supports no other rotation).
//!
//! Soundness rules the users of the bus keep:
//! - a provider row is never a request row, so a request cannot match itself;
//! - every provider gates its `chip` column to its own nonzero chip id;
//! - an active request gates its `chip` column to the target chip id. An inactive
//!   request has `chip = 0`; the lookup then forces the whole tuple to zero, the
//!   only provider-side tuple with `chip = 0` (the rows with `q_prov = 0`);
//! - every chip is a function of its key: two provider rows with the same
//!   `(chip, key, a, b)` have the same `c` (and for MEM the same `(a, b, c)`), so set
//!   inclusion is enough and no multiplicity is needed (study 1.4).
use midnight_proofs::{
    circuit::{AssignedCell, Region, Value},
    plonk::{Column, ConstraintSystem, Error, Fixed},
    poly::Rotation,
};

use crate::{Cols, F};

/// Chip ids on the bus. Zero is reserved for "no request".
pub const CHIP_ALU: u64 = 1;
pub const CHIP_MEM: u64 = 2;

/// Number of bus columns.
pub const WIDTH: usize = 5;

/// One bus tuple.
pub type Tuple = [F; WIDTH];

#[derive(Clone, Copy, Debug)]
pub struct BusConfig {
    pub cols: Cols,
    pub q_req: Column<Fixed>,
    pub q_prov: Column<Fixed>,
}

impl BusConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols) -> Self {
        let q_req = meta.fixed_column();
        let q_prov = meta.fixed_column();
        for (name, lane) in [("bus", 0), ("bus lane 1", 1)] {
            meta.lookup_any(name, |m| {
                let qr = m.query_fixed(q_req, Rotation::cur());
                let qp = m.query_fixed(q_prov, Rotation::cur());
                (0..WIDTH)
                    .map(|i| {
                        let r = m.query_advice(cols.a[lane * WIDTH + i], Rotation::cur());
                        let p = m.query_advice(cols.a[i], Rotation::cur());
                        (qr.clone() * r, qp.clone() * p)
                    })
                    .collect()
            });
        }
        Self {
            cols,
            q_req,
            q_prov,
        }
    }

    fn put(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        q: Column<Fixed>,
        t: Value<Tuple>,
    ) -> Result<Vec<AssignedCell<F, F>>, Error> {
        region.assign_fixed(|| "bus q", q, offset, || Value::known(F::from(1u64)))?;
        (0..WIDTH)
            .map(|i| {
                region.assign_advice(|| "bus tuple", self.cols.a[i], offset, || t.map(|t| t[i]))
            })
            .collect()
    }

    /// A request row at `offset` with tuple `t` in lane 0 and lane 1 zero.
    pub fn request(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        t: Value<Tuple>,
    ) -> Result<Vec<AssignedCell<F, F>>, Error> {
        self.request_pair(region, offset, t, Value::known([F::from(0u64); WIDTH]))
    }

    /// A request row at `offset` with `t0` in lane 0 and `t1` in lane 1 (a lane is
    /// all zero when inactive). Returns the lane 0 cells.
    pub fn request_pair(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        t0: Value<Tuple>,
        t1: Value<Tuple>,
    ) -> Result<Vec<AssignedCell<F, F>>, Error> {
        for i in 0..WIDTH {
            region.assign_advice(
                || "bus lane 1",
                self.cols.a[WIDTH + i],
                offset,
                || t1.map(|t| t[i]),
            )?;
        }
        self.put(region, offset, self.q_req, t0)
    }

    /// A provider row at `offset` with tuple `t`.
    pub fn provide(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        t: Value<Tuple>,
    ) -> Result<Vec<AssignedCell<F, F>>, Error> {
        self.put(region, offset, self.q_prov, t)
    }
}

/// A tuple from small integers.
pub fn tuple(chip: u64, key: F, a: F, b: F, c: F) -> Tuple {
    [F::from(chip), key, a, b, c]
}

#[cfg(test)]
mod tests {
    //! The bus alone: raw provider rows and raw request rows, no chip gates.
    use midnight_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        dev::MockProver,
        plonk::Circuit,
    };

    use super::*;

    #[derive(Clone, Default)]
    struct BusOnly {
        providers: Vec<Tuple>,
        requests: Vec<Tuple>,
        pairs: Vec<(Tuple, Tuple)>,
    }

    impl Circuit<F> for BusOnly {
        type Config = BusConfig;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> BusConfig {
            let cols = Cols::configure(meta);
            BusConfig::configure(meta, cols)
        }
        fn synthesize(&self, cfg: BusConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
            l.assign_region(
                || "bus",
                |mut r| {
                    let mut o = 0;
                    for t in &self.providers {
                        cfg.provide(&mut r, o, Value::known(*t))?;
                        o += 1;
                    }
                    for t in &self.requests {
                        cfg.request(&mut r, o, Value::known(*t))?;
                        o += 1;
                    }
                    for (t0, t1) in &self.pairs {
                        cfg.request_pair(&mut r, o, Value::known(*t0), Value::known(*t1))?;
                        o += 1;
                    }
                    Ok(())
                },
            )
        }
    }

    fn t(chip: u64, k: u64, a: u64, b: u64, c: u64) -> Tuple {
        tuple(chip, F::from(k), F::from(a), F::from(b), F::from(c))
    }

    fn ok(c: &BusOnly) -> bool {
        MockProver::run(c, vec![])
            .map(|p| p.verify().is_ok())
            .unwrap_or(false)
    }

    #[test]
    fn served_requests_pass_repeats_allowed() {
        let c = BusOnly {
            providers: vec![t(1, 2, 3, 4, 7), t(2, 9, 1, 1, 1)],
            requests: vec![
                t(1, 2, 3, 4, 7),
                t(1, 2, 3, 4, 7),
                t(2, 9, 1, 1, 1),
                [F::from(0u64); 5],
            ],
            pairs: vec![],
        };
        assert!(ok(&c));
    }

    #[test]
    fn forged_tuple_rejected() {
        let c = BusOnly {
            providers: vec![t(1, 2, 3, 4, 7)],
            requests: vec![t(1, 2, 3, 4, 8)],
            pairs: vec![],
        };
        assert!(!ok(&c));
    }

    #[test]
    fn requests_do_not_serve_each_other() {
        // Two identical forged requests and no provider row for them.
        let c = BusOnly {
            providers: vec![t(1, 2, 3, 4, 7)],
            requests: vec![t(1, 5, 5, 5, 5), t(1, 5, 5, 5, 5)],
            pairs: vec![],
        };
        assert!(!ok(&c));
    }

    #[test]
    fn lane_one_is_checked() {
        let c = BusOnly {
            providers: vec![t(1, 2, 3, 4, 7)],
            requests: vec![],
            pairs: vec![(t(1, 2, 3, 4, 7), t(1, 2, 3, 4, 7))],
        };
        assert!(ok(&c));
        let c = BusOnly {
            providers: vec![t(1, 2, 3, 4, 7)],
            requests: vec![],
            pairs: vec![(t(1, 2, 3, 4, 7), t(1, 2, 3, 4, 8))],
        };
        assert!(!ok(&c));
    }

    #[test]
    fn zero_chip_with_payload_rejected() {
        let c = BusOnly {
            providers: vec![t(1, 2, 3, 4, 7)],
            requests: vec![t(0, 2, 3, 4, 7)],
            pairs: vec![],
        };
        assert!(!ok(&c));
    }
}
