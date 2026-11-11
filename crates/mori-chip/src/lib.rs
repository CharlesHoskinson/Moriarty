//! Chiplet leaf spike for the Sherlock VM (experiment E3 of
//! `research/sherlock-processor-analogy-20261110/study.md`).
//!
//! One PLONK circuit, one set of shared advice columns, one bus. A CORE carries a
//! small register file across S step slots and issues at most two bus requests per
//! step. Chips serve the requests from their own row regions in the same columns:
//!
//! - [`bus`]: the tuple `(chip, key, a, b, c)` in columns 0..5, tied by one
//!   `lookup_any` of the request rows into the provider rows;
//! - [`range`]: 12-bit limb table and the 128-bit decomposition shape;
//! - [`alu`]: 128-bit checked add and less-than, one decomposition per operation;
//! - [`mem`]: write-once append log with fixed consecutive addresses;
//! - [`dec`]: fixed decoder table, control tag to rule;
//! - [`cpu`]: CORE, the register file, request rows and the step transition;
//! - [`leaf`]: the composed circuit with the node's leaf statement.
//!
//! The rule bodies are spike analogues of `KMachinePushAccept`,
//! `KMachineReturnRight` and `BHelperScalarArithmetic` ([`trace`]); they are not
//! the C6 rules and carry no claim of equivalence with them.
pub mod alu;
pub mod bus;
pub mod cpu;
pub mod dec;
pub mod dense;
pub mod hashchip;
pub mod leaf;
pub mod m3;
pub mod mem;
pub mod membackend;
pub mod poseidon;
pub mod range;
pub mod rf;
pub mod rfcore;
pub mod trace;
pub mod vsel;

use ff::PrimeField;
use midnight_proofs::plonk::{Advice, Column, ConstraintSystem, Constraints, Expression};

/// Scalar field of the BLS12-381 proof system the node verifies.
pub type F = midnight_curves::Fq;

/// Shared advice columns. Every chip region and the core use these columns.
pub const NCOL: usize = 12;

/// The shared advice column set.
#[derive(Clone, Copy, Debug)]
pub struct Cols {
    pub a: [Column<Advice>; NCOL],
}

impl Cols {
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        Self {
            a: std::array::from_fn(|_| meta.advice_column()),
        }
    }
}

/// `F` from a `u128`.
pub fn fu(v: u128) -> F {
    F::from_u128(v)
}

/// Gate body: the constraints already carry their selector factor.
pub fn gate(c: Vec<Expression<F>>) -> Constraints<F> {
    Constraints::without_selector(c)
}
