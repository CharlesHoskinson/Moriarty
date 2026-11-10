//! HASH (experiment E6): the Poseidon chip of `midnight-circuits` 7.3.0 (the one
//! `midnight-zk-stdlib` 2.4.0 composes) inside the chip leaf, behind the bus.
//!
//! Configuration: `NativeChip` on shared advice columns 0..5 and `PoseidonChip` on
//! shared advice columns 0..8, both over `NB_ARITH_FIXED_COLS` (9) new fixed
//! columns, with the leaf's two instance columns. This is the stdlib composition,
//! not a slimmed HASH chip: a hash is the stdlib sponge (`init` with the input
//! length in the capacity, one native `add` per absorbed input, one permutation per
//! `RATE = 2` inputs).
//!
//! A bus provider row `(HASH, id, in0, in1, out)` serves a 2-input hash: `in0` and
//! `in1` are the sponge inputs (copied from the provider row) and `out` is
//! copy-constrained to the sponge output. The domain tag `id` is not absorbed in
//! this measurement; absorbing it makes a 3-input hash (two permutations).
use midnight_circuits::{
    ComposableChip,
    field::{NativeChip, NativeConfig, native::NB_ARITH_COLS, native::NB_ARITH_FIXED_COLS},
    hash::poseidon::{NB_POSEIDON_ADVICE_COLS, PoseidonChip, PoseidonConfig},
    instructions::hash::HashInstructions,
};
use midnight_proofs::{
    circuit::{AssignedCell, Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed, Instance},
};

use crate::{
    F,
    bus::{BusConfig, Tuple},
};

/// Chip id of HASH on the bus.
pub const CHIP_HASH: u64 = 3;

/// Key-shape stand-in for a slim HASH chip: the `PoseidonChip` gates on shared
/// advice columns 0..8 with their own 8 round-constant columns, and no
/// `NativeChip` (no coefficient columns, no fixed-constant column, no extra
/// equality columns: the permutation's input and output cells would be copied from
/// the bus provider row, whose columns already allow equality). It cannot hash:
/// `PoseidonChip::permutation` needs the native chip, and its selectors are private,
/// so only the round-constant columns are filled. It measures what such a chip
/// costs the node, not its rows.
#[derive(Clone, Debug)]
pub struct SlimHashShape {
    pub poseidon: PoseidonConfig<F>,
    pub fixed: Vec<Column<Fixed>>,
    pub advice: Vec<Column<Advice>>,
}

impl SlimHashShape {
    pub fn configure(meta: &mut ConstraintSystem<F>, advice: &[Column<Advice>]) -> Self {
        let nf = midnight_circuits::hash::poseidon::NB_POSEIDON_FIXED_COLS;
        let fixed: Vec<Column<Fixed>> = (0..nf).map(|_| meta.fixed_column()).collect();
        let poseidon = PoseidonChip::configure(
            meta,
            &(
                advice[..NB_POSEIDON_ADVICE_COLS].try_into().unwrap(),
                fixed[..].try_into().unwrap(),
            ),
        );
        Self {
            poseidon,
            fixed,
            advice: advice[..NB_POSEIDON_ADVICE_COLS].to_vec(),
        }
    }

    /// Fill `rows` rows of the round-constant columns with nonzero constants (20
    /// rows per permutation in the stdlib layout).
    pub fn fill(&self, l: &mut impl Layouter<F>, rows: usize) -> Result<(), Error> {
        l.assign_region(
            || "slim hash shape",
            |mut r| {
                for i in 0..rows {
                    for (j, c) in self.fixed.iter().enumerate() {
                        let v = F::from((i * 31 + j * 7 + 1) as u64);
                        r.assign_fixed(|| "rc", *c, i, || Value::known(v))?;
                    }
                    for a in &self.advice {
                        r.assign_advice(|| "reg", *a, i, || Value::known(F::from(0u64)))?;
                    }
                }
                Ok(())
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct HashConfig {
    pub native: NativeConfig,
    pub poseidon: PoseidonConfig<F>,
    pub fixed: Vec<Column<Fixed>>,
}

impl HashConfig {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: &[Column<Advice>],
        instances: [Column<Instance>; 2],
    ) -> Self {
        assert!(advice.len() >= NB_POSEIDON_ADVICE_COLS.max(NB_ARITH_COLS));
        let fixed: Vec<Column<Fixed>> = (0..NB_ARITH_FIXED_COLS)
            .map(|_| meta.fixed_column())
            .collect();
        let native = NativeChip::configure(
            meta,
            &(
                advice[..NB_ARITH_COLS].try_into().unwrap(),
                fixed[..NB_ARITH_FIXED_COLS].try_into().unwrap(),
                instances,
            ),
        );
        let poseidon = PoseidonChip::configure(
            meta,
            &(
                advice[..NB_POSEIDON_ADVICE_COLS].try_into().unwrap(),
                fixed[..midnight_circuits::hash::poseidon::NB_POSEIDON_FIXED_COLS]
                    .try_into()
                    .unwrap(),
            ),
        );
        Self {
            native,
            poseidon,
            fixed,
        }
    }

    pub fn chip(&self) -> PoseidonChip<F> {
        let native = NativeChip::new(&self.native, &());
        PoseidonChip::new(&self.poseidon, &native)
    }
}

/// The native value of the 2-input hash a provider row serves.
pub fn hash_native(inputs: &[F]) -> F {
    <PoseidonChip<F> as midnight_circuits::instructions::hash::HashCPU<F, F>>::hash(inputs)
}

/// A provider tuple for `out = H(in0, in1)`.
pub fn hash_tuple(id: u64, in0: F, in1: F) -> Tuple {
    crate::bus::tuple(CHIP_HASH, F::from(id), in0, in1, hash_native(&[in0, in1]))
}

/// Hash `inputs` (cells assigned elsewhere) in their own regions; returns the
/// output cell.
pub fn hash_cells(
    cfg: &HashConfig,
    l: &mut impl Layouter<F>,
    inputs: &[AssignedCell<F, F>],
) -> Result<AssignedCell<F, F>, Error> {
    cfg.chip().hash(l, inputs)
}

/// Assign `n` free input cells in one region of column 0 and hash them.
pub fn hash_free(
    cfg: &HashConfig,
    bus: &BusConfig,
    l: &mut impl Layouter<F>,
    vals: &[Value<F>],
) -> Result<AssignedCell<F, F>, Error> {
    let cells = l.assign_region(
        || "hash inputs",
        |mut r| {
            vals.iter()
                .enumerate()
                .map(|(i, v)| r.assign_advice(|| "in", bus.cols.a[i % 2], i / 2, || *v))
                .collect::<Result<Vec<_>, _>>()
        },
    )?;
    hash_cells(cfg, l, &cells)
}
