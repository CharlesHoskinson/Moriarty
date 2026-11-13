//! I7 unit costs (cost model only): rows of `N` range checks `< 2^30`, `N` checks `< P`
//! (BabyBear, not a power of two) and `N` witnessed bits, at `nr_pow2range_cols` 1 and 4.
use std::io;

use midnight_circuits::instructions::{AssignmentInstructions, RangeCheckInstructions};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch, cost_model};
use num_bigint::BigUint;
use sherlock_wrap::{native::F, smoke::wrap_arch};

#[derive(Clone, Debug)]
struct Unit {
    kind: u8,
    n: usize,
    cols: u8,
}

impl Relation for Unit {
    type Instance = Vec<F>;
    type Witness = ();
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(&self, std: &ZkStdLib, l: &mut impl Layouter<F>, _: Value<Vec<F>>, _: Value<()>) -> Result<(), Error> {
        for i in 0..self.n {
            match self.kind {
                0 => {
                    let x = std.assign(l, Value::known(F::from(i as u64)))?;
                    std.native_gadget().assert_lower_than_fixed(l, &x, &(BigUint::from(1u64) << 30))?;
                }
                1 => {
                    let x = std.assign(l, Value::known(F::from(i as u64)))?;
                    std.native_gadget().assert_lower_than_fixed(l, &x, &BigUint::from(0x7800_0001u64))?;
                }
                _ => {
                    let _b: midnight_circuits::types::AssignedBit<F> = std.assign(l, Value::known(i % 2 == 0))?;
                }
            }
        }
        Ok(())
    }
    fn used_chips(&self) -> ZkStdLibArch {
        wrap_arch(self.cols)
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"unit")
    }
    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("unit"))
    }
}

#[test]
fn i7_unit_cost() {
    for cols in [1u8, 4] {
        let base = cost_model(&Unit { kind: 2, n: 0, cols }, Some(17)).rows;
        for (kind, name) in [(0u8, "range30"), (1, "lt_p"), (2, "bit")] {
            let n = 4000;
            let rows = cost_model(&Unit { kind, n, cols }, Some(17)).rows;
            println!("E5UNIT cols {cols} {name}: {:.3} rows each (n {n}, rows {rows}, base {base})", (rows - base) as f64 / n as f64);
        }
    }
}
