//! DEC: the decoder ROM, a fixed table `(class, rule)`.
//!
//! In this spike the class is the control tag; the full design packs the trap bits,
//! tags, phase and comparator flags into the class (study 2.2). Row `(0, 0)` serves
//! disabled slots. One lookup argument; the table is part of the key.
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::{ConstraintSystem, Error, Expression, TableColumn, VirtualCells},
};

use crate::F;

#[derive(Clone, Copy, Debug)]
pub struct DecConfig {
    pub class: TableColumn,
    pub rule: TableColumn,
}

impl DecConfig {
    /// `input` gives the `(class, rule)` expressions the core looks up.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        input: impl FnOnce(&mut VirtualCells<'_, F>) -> (Expression<F>, Expression<F>),
    ) -> Self {
        let class = meta.lookup_table_column();
        let rule = meta.lookup_table_column();
        meta.lookup("dec", |m| {
            let (c, r) = input(m);
            vec![(c, class), (r, rule)]
        });
        Self { class, rule }
    }

    pub fn load(&self, l: &mut impl Layouter<F>, rows: &[(u64, u64)]) -> Result<(), Error> {
        l.assign_table(
            || "dec",
            |mut t| {
                for (i, (c, r)) in std::iter::once(&(0, 0)).chain(rows).enumerate() {
                    t.assign_cell(|| "class", self.class, i, || Value::known(F::from(*c)))?;
                    t.assign_cell(|| "rule", self.rule, i, || Value::known(F::from(*r)))?;
                }
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use midnight_proofs::{
        circuit::SimpleFloorPlanner,
        dev::MockProver,
        plonk::{Circuit, Selector},
        poly::Rotation,
    };

    use super::*;
    use crate::Cols;

    #[derive(Clone)]
    struct DecOnly {
        pairs: Vec<(u64, u64)>,
    }

    impl Circuit<F> for DecOnly {
        type Config = (Cols, Selector, DecConfig);
        type FloorPlanner = SimpleFloorPlanner;
        type Params = ();
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let cols = Cols::configure(meta);
            let q = meta.complex_selector();
            let dec = DecConfig::configure(meta, |m| {
                let q = m.query_selector(q);
                let c = m.query_advice(cols.a[0], Rotation::cur());
                let r = m.query_advice(cols.a[1], Rotation::cur());
                (q.clone() * c, q * r)
            });
            (cols, q, dec)
        }
        fn synthesize(
            &self,
            (cols, q, dec): Self::Config,
            mut l: impl Layouter<F>,
        ) -> Result<(), Error> {
            dec.load(&mut l, &[(2, 1), (3, 2), (1, 3)])?;
            l.assign_region(
                || "dec",
                |mut r| {
                    for (i, (c, rule)) in self.pairs.iter().enumerate() {
                        q.enable(&mut r, i)?;
                        r.assign_advice(|| "c", cols.a[0], i, || Value::known(F::from(*c)))?;
                        r.assign_advice(|| "r", cols.a[1], i, || Value::known(F::from(*rule)))?;
                    }
                    Ok(())
                },
            )
        }
    }

    fn ok(c: &DecOnly) -> bool {
        MockProver::run(c, vec![])
            .map(|p| p.verify().is_ok())
            .unwrap_or(false)
    }

    #[test]
    fn table_rows_pass() {
        assert!(ok(&DecOnly {
            pairs: vec![(2, 1), (3, 2), (1, 3), (0, 0)]
        }));
    }

    #[test]
    fn wrong_rule_for_class_rejected() {
        assert!(!ok(&DecOnly {
            pairs: vec![(2, 2)]
        }));
        assert!(!ok(&DecOnly {
            pairs: vec![(4, 1)]
        }));
    }
}
