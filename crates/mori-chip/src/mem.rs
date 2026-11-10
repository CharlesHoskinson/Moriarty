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
    plonk::{ConstraintSystem, Error, Expression, Selector, VirtualCells},
    poly::Rotation,
};

use crate::{
    F,
    bus::{BusConfig, CHIP_MEM, tuple},
    vsel::{VSel, code_column},
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

/// M3 MEM (experiment E8): the leaf's aligned segment of [`SEG`] cells at
/// addresses `base .. base + SEG`, with an active prefix.
///
/// Row `i` holds `(chip, key, a, b, c)` in columns 0..5, its address in column
/// 5, `len_end` (the core's running `len` after the last slot) in column 6 and
/// the active flag `act` in column 7. One code column: code 1 on row 0, code 2
/// on rows 1..SEG-1, code 3 on row SEG-1 ([`crate::vsel`]).
///
/// - `mem row` (every row): `act` is boolean; `chip = act * MEM` and
///   `key = act * addr`; `(1 - act) * x = 0` for the three data words. A padding
///   row therefore puts the zero tuple on the table side (D8), so no request
///   (whose chip is non-zero) can read it.
/// - `mem chain` (rows 0..SEG-2): `addr' = addr + 1`, `len_end' = len_end`,
///   `act' <= act` (prefix), and where the prefix ends, `addr + 1 = len_end`.
/// - `mem first`: if row 0 is padding, `addr = len_end` (no append).
/// - `mem last`: if row SEG-1 is active, `addr + 1 = len_end` (segment full).
///
/// So the active rows are exactly the addresses below `len_end`: an active row
/// above it, or a padding row below it, fails a gate (Gemini Quint verdict C4).
/// The committed record of row `i` is `H(act, a, b, c)`: `act` is the record's
/// tag word, 1 on every written record, so no written record (D7's sentinel
/// spill `(0, 0, NIL)` included) hashes like padding (verdict C2).
#[derive(Clone, Debug)]
pub struct SegMemConfig {
    pub q: [VSel; 3],
    pub bus: BusConfig,
}

/// Cells per segment.
pub const SEG: usize = 8;

/// Assigned cells of the segment region.
pub struct SegCells {
    pub addr0: AssignedCell<F, F>,
    pub len_end: AssignedCell<F, F>,
    /// Per row: `[act, a, b, c]`.
    pub rec: Vec<[AssignedCell<F, F>; 4]>,
}

impl SegMemConfig {
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig) -> Self {
        let qv = code_column(meta, 3);
        let q = [qv[0], qv[1], qv[2]];
        let col = match q[0] {
            VSel::Code { col, .. } => col,
            VSel::Simple(_) => unreachable!(),
        };
        let a = bus.cols.a;
        let k = |v: u64| Expression::Constant(F::from(v));
        let cur = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::cur());
        let nxt = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::next());
        meta.create_gate("mem row", |m| {
            // Non-zero on every code: `q` itself.
            let any = m.query_fixed(col, Rotation::cur());
            let act = cur(m, 7);
            let addr = cur(m, 5);
            let mut v = vec![
                any.clone() * act.clone() * (k(1) - act.clone()),
                any.clone() * (cur(m, 0) - act.clone() * k(CHIP_MEM)),
                any.clone() * (cur(m, 1) - act.clone() * addr),
            ];
            for i in 2..5 {
                v.push(any.clone() * (k(1) - act.clone()) * cur(m, i));
            }
            crate::gate(v)
        });
        meta.create_gate("mem chain", |m| {
            // Non-zero on codes 1 and 2, zero on 0 and 3.
            let qq = m.query_fixed(col, Rotation::cur());
            let on = qq.clone() * (qq - k(3));
            let (act, act2) = (cur(m, 7), nxt(m, 7));
            let (addr, addr2) = (cur(m, 5), nxt(m, 5));
            let (len, len2) = (cur(m, 6), nxt(m, 6));
            crate::gate(vec![
                on.clone() * (addr2 - addr.clone() - k(1)),
                on.clone() * (len2 - len.clone()),
                on.clone() * act2.clone() * (k(1) - act.clone()),
                on * act * (k(1) - act2) * (addr + k(1) - len),
            ])
        });
        let (q1, q3) = (q[0], q[2]);
        meta.create_gate("mem first", move |m| {
            let s = q1.expr(m);
            let (act, addr, len) = (cur(m, 7), cur(m, 5), cur(m, 6));
            crate::gate(vec![s * (k(1) - act) * (addr - len)])
        });
        meta.create_gate("mem last", move |m| {
            let s = q3.expr(m);
            let (act, addr, len) = (cur(m, 7), cur(m, 5), cur(m, 6));
            crate::gate(vec![s * act * (addr + k(1) - len)])
        });
        for i in 0..8 {
            meta.enable_equality(a[i]);
        }
        Self { q, bus }
    }

    /// The [`SEG`] rows from `offset`: data `cells[i]`, flags `act[i]`.
    pub fn assign(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        base: Value<F>,
        len_end: Value<F>,
        cells: &[Value<Cell3>],
        act: &[Value<F>],
    ) -> Result<SegCells, Error> {
        assert_eq!(cells.len(), SEG);
        assert_eq!(act.len(), SEG);
        let a = self.bus.cols.a;
        let mut addr0 = None;
        let mut len0 = None;
        let mut rec = Vec::new();
        for i in 0..SEG {
            let r = offset + i;
            let code = if i == 0 {
                0
            } else if i + 1 == SEG {
                2
            } else {
                1
            };
            self.q[code].enable(region, r)?;
            let addr = base.map(|b| b + F::from(i as u64));
            let t = addr
                .zip(cells[i])
                .zip(act[i])
                .map(|((ad, c), f)| tuple(CHIP_MEM, f * ad, c[0], c[1], c[2]));
            let t = t.zip(act[i]).map(|(mut t, f)| {
                t[0] *= f;
                t
            });
            let tc = self.bus.provide(region, r, t)?;
            let ad = region.assign_advice(|| "mem addr", a[5], r, || addr)?;
            let ln = region.assign_advice(|| "mem len_end", a[6], r, || len_end)?;
            let ac = region.assign_advice(|| "mem act", a[7], r, || act[i])?;
            if i == 0 {
                addr0 = Some(ad);
                len0 = Some(ln);
            }
            rec.push([ac, tc[2].clone(), tc[3].clone(), tc[4].clone()]);
        }
        Ok(SegCells {
            addr0: addr0.unwrap(),
            len_end: len0.unwrap(),
            rec,
        })
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

    /// The M3 segment alone: `used` active rows at `base = 8`, `len_end` free.
    #[derive(Clone)]
    struct SegOnly {
        cells: Vec<Cell3>,
        used: usize,
        len_end: u64,
        requests: Vec<Tuple>,
        /// Padding row that keeps `chip = MEM`, `key = addr` on the bus.
        forge_pad: Option<usize>,
    }

    impl Circuit<F> for SegOnly {
        type Config = SegMemConfig;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> SegMemConfig {
            let cols = Cols::configure(meta);
            let bus = BusConfig::configure(meta, cols);
            SegMemConfig::configure(meta, bus)
        }
        fn synthesize(&self, cfg: SegMemConfig, mut l: impl Layouter<F>) -> Result<(), Error> {
            l.assign_region(
                || "seg",
                |mut r| {
                    let cells: Vec<_> = (0..SEG)
                        .map(|i| {
                            Value::known(self.cells.get(i).copied().unwrap_or([F::from(0u64); 3]))
                        })
                        .collect();
                    let act: Vec<_> = (0..SEG)
                        .map(|i| Value::known(F::from((i < self.used) as u64)))
                        .collect();
                    cfg.assign(
                        &mut r,
                        0,
                        Value::known(F::from(8u64)),
                        Value::known(F::from(self.len_end)),
                        &cells,
                        &act,
                    )?;
                    if let Some(row) = self.forge_pad {
                        let a = cfg.bus.cols.a;
                        let ad = F::from(8 + row as u64);
                        r.assign_advice(|| "chip", a[0], row, || Value::known(F::from(CHIP_MEM)))?;
                        r.assign_advice(|| "key", a[1], row, || Value::known(ad))?;
                    }
                    for (j, t) in self.requests.iter().enumerate() {
                        cfg.bus.request(&mut r, SEG + j, Value::known(*t))?;
                    }
                    Ok(())
                },
            )
        }
    }

    fn seg_err(c: &SegOnly) -> Option<String> {
        let p = MockProver::run(c, vec![]).unwrap();
        p.verify().err().map(|e| {
            e.iter()
                .map(|f| format!("{f}"))
                .collect::<Vec<_>>()
                .join(" | ")
        })
    }

    /// Reads of written cells pass, including a written zero tuple; a read of a
    /// padding cell (as the zero tuple or anything else) is refused by the bus,
    /// because padding rows put the zero tuple on the table side.
    #[test]
    fn seg_padding_cell_is_unreadable() {
        let cells = vec![c3(0, 0, 0), c3(7, 2, 10), c3(7, 3, 11)];
        let ok_reqs = vec![req(8, cells[0]), req(10, cells[2])];
        let honest = SegOnly {
            cells: cells.clone(),
            used: 3,
            len_end: 11,
            requests: ok_reqs,
            forge_pad: None,
        };
        assert_eq!(seg_err(&honest), None);
        for (addr, c) in [(11, c3(0, 0, 0)), (15, c3(0, 0, 0)), (12, c3(7, 1, 1))] {
            let c = SegOnly {
                requests: vec![req(addr, c)],
                ..honest.clone()
            };
            let e = seg_err(&c).expect("padding read");
            assert!(e.contains("ookup"), "{e}");
        }
        // Full and empty segments.
        let full = SegOnly {
            cells: (0..8).map(|i| c3(1, i, 0)).collect(),
            used: 8,
            len_end: 16,
            requests: vec![req(15, c3(1, 7, 0))],
            forge_pad: None,
        };
        assert_eq!(seg_err(&full), None);
        let empty = SegOnly {
            cells: vec![],
            used: 0,
            len_end: 8,
            requests: vec![],
            forge_pad: None,
        };
        assert_eq!(seg_err(&empty), None);
        // A padding row that still offers `(MEM, addr, 0, 0, 0)` on the bus.
        let c = SegOnly {
            forge_pad: Some(5),
            requests: vec![req(13, c3(0, 0, 0))],
            ..honest.clone()
        };
        let e = seg_err(&c).expect("padding on the bus");
        assert!(e.contains("mem row") && !e.contains("ookup"), "{e}");
        // `len_end` off by one at either end of the prefix.
        for (used, len_end, gate) in [
            (0, 9, "mem first"),
            (8, 15, "mem last"),
            (3, 12, "mem chain"),
        ] {
            let c = SegOnly {
                cells: (0..used).map(|i| c3(1, i as u64, 0)).collect(),
                used,
                len_end,
                requests: vec![],
                forge_pad: None,
            };
            let e = seg_err(&c).expect("desync");
            assert!(e.contains(gate), "{used} {len_end}: {e}");
        }
    }
}
