//! MEM: the write-once append log of one leaf.
//!
//! Provider rows `(MEM, addr, a, b, c)` with consecutive addresses: the first row's
//! address is copy-constrained to the core's `len` at the start of the leaf
//! (`base`), and each next row's address is the previous one plus one. An address
//! therefore has exactly one row and one value: the chip is a function of its key.
//!
//! The core appends with `(MEM, len, x)` and then `len := len + 1`, and reads only
//! below `len` (an ALU less-than request). So every row whose address is below
//! the final `len` was written by exactly one append, and every other row is
//! unreachable: a chip row with no request can never be read. No row count,
//! activity flag or reverse lookup is needed. Rows below `base` (cells of earlier
//! leaves) are not here; they are PAGE's job (not in this spike).
use midnight_proofs::{
    circuit::{AssignedCell, Region, Value},
    plonk::{ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

use crate::{
    F,
    bus::{BusConfig, CHIP_MEM, tuple},
};

/// One memory cell: three field elements.
pub type Cell3 = [F; 3];

#[derive(Clone, Debug)]
pub struct MemConfig {
    pub q_row: Selector,
    pub q_chain: Selector,
    pub bus: BusConfig,
}

impl MemConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig) -> Self {
        let q_row = meta.selector();
        let q_chain = meta.selector();
        let a = bus.cols.a;
        meta.create_gate("mem row", |m| {
            let q = m.query_selector(q_row);
            let chip = m.query_advice(a[0], Rotation::cur());
            crate::gate(vec![q * (chip - Expression::Constant(F::from(CHIP_MEM)))])
        });
        meta.create_gate("mem chain", |m| {
            let q = m.query_selector(q_chain);
            let addr = m.query_advice(a[1], Rotation::cur());
            let next = m.query_advice(a[1], Rotation::next());
            crate::gate(vec![
                q * (next - addr - Expression::Constant(F::from(1u64))),
            ])
        });
        Self {
            q_row,
            q_chain,
            bus,
        }
    }

    /// `n` rows from `offset`: `cells[i]` (zero when absent) at address `base + i`.
    /// Returns the first row's address cell for the caller's copy constraint.
    pub fn assign(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        n: usize,
        base: Value<F>,
        cells: &[Value<Cell3>],
    ) -> Result<AssignedCell<F, F>, Error> {
        assert!(n >= 1 && cells.len() <= n);
        let mut first = None;
        for i in 0..n {
            self.q_row.enable(region, offset + i)?;
            if i + 1 < n {
                self.q_chain.enable(region, offset + i)?;
            }
            let cell = cells
                .get(i)
                .copied()
                .unwrap_or(Value::known([F::from(0u64); 3]));
            let t = base
                .zip(cell)
                .map(|(b, c)| tuple(CHIP_MEM, b + F::from(i as u64), c[0], c[1], c[2]));
            let assigned = self.bus.provide(region, offset + i, t)?;
            if i == 0 {
                first = Some(assigned[1].clone());
            }
        }
        Ok(first.expect("n >= 1"))
    }
}

#[cfg(test)]
mod tests {
    //! MEM with the bus; requests are raw rows and the base is a constant.
    use midnight_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        dev::MockProver,
        plonk::Circuit,
    };

    use super::*;
    use crate::{Cols, bus::Tuple};

    #[derive(Clone)]
    struct MemOnly {
        base: u64,
        cells: Vec<Cell3>,
        requests: Vec<Tuple>,
        /// Overwrite the address of row `i` (breaks the chain).
        bad_addr: Option<(usize, u64)>,
    }

    impl Circuit<F> for MemOnly {
        type Config = MemConfig;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> MemConfig {
            let cols = Cols::configure(meta);
            let bus = BusConfig::configure(meta, cols);
            MemConfig::configure(meta, bus)
        }
        fn synthesize(&self, cfg: MemConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
            l.assign_region(
                || "mem",
                |mut r| {
                    let cells: Vec<_> = self.cells.iter().map(|c| Value::known(*c)).collect();
                    let n = cells.len().max(1);
                    cfg.assign(&mut r, 0, n, Value::known(F::from(self.base)), &cells)?;
                    if let Some((i, a)) = self.bad_addr {
                        r.assign_advice(|| "x", cfg.bus.cols.a[1], i, || Value::known(F::from(a)))?;
                    }
                    for (j, t) in self.requests.iter().enumerate() {
                        cfg.bus.request(&mut r, n + j, Value::known(*t))?;
                    }
                    Ok(())
                },
            )
        }
    }

    fn c3(a: u64, b: u64, c: u64) -> Cell3 {
        [F::from(a), F::from(b), F::from(c)]
    }

    fn req(addr: u64, c: Cell3) -> Tuple {
        tuple(CHIP_MEM, F::from(addr), c[0], c[1], c[2])
    }

    fn ok(c: &MemOnly) -> bool {
        MockProver::run(c, vec![])
            .map(|p| p.verify().is_ok())
            .unwrap_or(false)
    }

    #[test]
    fn appends_and_reads_at_consecutive_addresses() {
        let cells = vec![c3(7, 1, 0), c3(7, 2, 10), c3(7, 3, 11)];
        let requests = vec![req(10, cells[0]), req(12, cells[2]), req(11, cells[1])];
        assert!(ok(&MemOnly {
            base: 10,
            cells,
            requests,
            bad_addr: None
        }));
    }

    #[test]
    fn wrong_read_value_rejected() {
        let cells = vec![c3(7, 1, 0), c3(7, 2, 10)];
        let requests = vec![req(11, c3(7, 9, 10))];
        assert!(!ok(&MemOnly {
            base: 10,
            cells,
            requests,
            bad_addr: None
        }));
    }

    #[test]
    fn duplicate_address_rejected() {
        // Row 1 claims address 10 again (a second value for one address).
        let cells = vec![c3(7, 1, 0), c3(7, 2, 10)];
        let requests = vec![req(10, cells[1])];
        assert!(!ok(&MemOnly {
            base: 10,
            cells,
            requests,
            bad_addr: Some((1, 10))
        }));
    }
}
