//! Thin layer over `ark_relations::r1cs` (arkworks 0.4 line, `=0.4.0`).
//!
//! All arkworks API use outside `circuit.rs` is concentrated here:
//! `ConstraintSystemRef::{new_input_variable, new_witness_variable, enforce_constraint,
//! num_constraints, is_in_setup_mode}`, `LinearCombination`'s public tuple field,
//! `Variable::One` (and its `Ord`, used to merge terms) and `SynthesisError`. These are
//! recalled from ark-relations 0.4 and from the frozen prototype that built against
//! `=0.4.0`; they are UNVERIFIED here because nothing was compiled.
//!
//! Every witness allocation carries a deterministic label `scope/role#n`. A
//! [`Tamper`] map can replace the value assigned to any labelled witness, so tests can
//! play an adversarial prover without editing the lowering.
use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSystemRef, LinearCombination, SynthesisError, Variable};
use std::collections::BTreeMap;

pub(crate) fn fe<F: PrimeField>(v: u128) -> F {
    F::from(v)
}

pub(crate) fn fe_bool<F: PrimeField>(b: bool) -> F {
    if b { F::from(1u64) } else { F::from(0u64) }
}

/// 2^64 in the field.
pub(crate) fn two64<F: PrimeField>() -> F {
    F::from(1u128 << 64)
}

/// 2^128 in the field.
pub(crate) fn two128<F: PrimeField>() -> F {
    two64::<F>() * two64::<F>()
}

/// An owned linear combination. Converted to arkworks form with duplicate variables
/// merged and zero coefficients dropped.
#[derive(Clone, Debug)]
pub struct Lc<F: PrimeField> {
    terms: Vec<(F, Variable)>,
}

impl<F: PrimeField> Lc<F> {
    pub fn zero() -> Self {
        Self { terms: Vec::new() }
    }
    pub fn constant(c: F) -> Self {
        Self { terms: vec![(c, Variable::One)] }
    }
    pub fn one() -> Self {
        Self::constant(F::from(1u64))
    }
    pub fn var(v: Variable) -> Self {
        Self { terms: vec![(F::from(1u64), v)] }
    }
    pub fn term(mut self, k: F, v: Variable) -> Self {
        self.terms.push((k, v));
        self
    }
    pub fn plus(mut self, other: &Lc<F>) -> Self {
        self.terms.extend(other.terms.iter().cloned());
        self
    }
    pub fn minus(mut self, other: &Lc<F>) -> Self {
        self.terms.extend(other.terms.iter().map(|(k, v)| (-*k, *v)));
        self
    }
    pub fn scaled(&self, k: F) -> Self {
        Self { terms: self.terms.iter().map(|(c, v)| (*c * k, *v)).collect() }
    }
    pub(crate) fn into_ark(self) -> LinearCombination<F> {
        let mut merged: BTreeMap<Variable, F> = BTreeMap::new();
        for (k, v) in self.terms {
            let entry = merged.entry(v).or_insert_with(|| F::from(0u64));
            *entry += k;
        }
        let zero = F::from(0u64);
        LinearCombination(merged.into_iter().filter(|(_, k)| *k != zero).map(|(v, k)| (k, v)).collect())
    }
}

/// Test hook: replaces the assigned value of labelled witnesses. Keys are the exact
/// labels `scope/role#n` produced by [`Cs::witness`]. Public inputs are not tampered
/// here: a test mutates the claimed public vector of an `Assignment` instead.
#[derive(Clone, Debug)]
pub struct Tamper<F: PrimeField> {
    overrides: BTreeMap<String, F>,
}

impl<F: PrimeField> Tamper<F> {
    pub fn none() -> Self {
        Self { overrides: BTreeMap::new() }
    }
    pub fn set(mut self, label: impl Into<String>, value: F) -> Self {
        self.overrides.insert(label.into(), value);
        self
    }
    pub fn is_empty(&self) -> bool {
        self.overrides.is_empty()
    }
}

/// Constraint count of one labelled region (entry, a statement, cap, accounting or
/// binding block). Filled only by an actual synthesis; no count is recorded in source.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct RegionCount {
    pub label: String,
    pub constraints: usize,
}

pub(crate) struct Cs<F: PrimeField> {
    cs: ConstraintSystemRef<F>,
    tamper: Tamper<F>,
    scope: Vec<String>,
    seen: BTreeMap<String, u32>,
    used_overrides: usize,
    pub(crate) regions: Vec<RegionCount>,
}

impl<F: PrimeField> Cs<F> {
    pub(crate) fn new(cs: ConstraintSystemRef<F>, tamper: Tamper<F>) -> Self {
        Self { cs, tamper, scope: Vec::new(), seen: BTreeMap::new(), used_overrides: 0, regions: Vec::new() }
    }

    pub(crate) fn push(&mut self, s: String) {
        self.scope.push(s);
    }
    pub(crate) fn pop(&mut self) {
        self.scope.pop();
    }

    fn label(&mut self, role: &str) -> String {
        let base = if self.scope.is_empty() { role.to_string() } else { format!("{}/{}", self.scope.join("/"), role) };
        let n = self.seen.entry(base.clone()).or_insert(0);
        let label = format!("{base}#{n}");
        *n += 1;
        label
    }

    /// Allocates a private witness. `value` is the honest proposal; a tamper entry for
    /// this label replaces it. In setup mode arkworks never evaluates the closure.
    pub(crate) fn witness(&mut self, role: &str, value: Option<F>) -> Result<Variable, SynthesisError> {
        let label = self.label(role);
        let value = match self.tamper.overrides.get(&label) {
            Some(v) => {
                self.used_overrides += 1;
                Some(*v)
            }
            None => value,
        };
        self.cs.new_witness_variable(|| value.ok_or(SynthesisError::AssignmentMissing))
    }

    /// Allocates the next public input. The adapter allocates public inputs in slot
    /// order, so instance order equals the encoded statement order.
    pub(crate) fn input(&mut self, value: Option<F>) -> Result<Variable, SynthesisError> {
        self.cs.new_input_variable(|| value.ok_or(SynthesisError::AssignmentMissing))
    }

    pub(crate) fn enforce(&mut self, a: Lc<F>, b: Lc<F>, c: Lc<F>) -> Result<(), SynthesisError> {
        self.cs.enforce_constraint(a.into_ark(), b.into_ark(), c.into_ark())
    }

    /// `lc * 1 = 0`.
    pub(crate) fn enforce_zero(&mut self, lc: Lc<F>) -> Result<(), SynthesisError> {
        self.enforce(lc, Lc::one(), Lc::zero())
    }

    /// `(a - b) * 1 = 0`.
    pub(crate) fn enforce_eq(&mut self, a: &Lc<F>, b: &Lc<F>) -> Result<(), SynthesisError> {
        self.enforce_zero(a.clone().minus(b))
    }

    pub(crate) fn num_constraints(&self) -> usize {
        self.cs.num_constraints()
    }

    pub(crate) fn is_setup(&self) -> bool {
        self.cs.is_in_setup_mode()
    }

    pub(crate) fn record(&mut self, label: String, start: usize) {
        let constraints = self.num_constraints().saturating_sub(start);
        self.regions.push(RegionCount { label, constraints });
    }

    /// Number of tamper entries that matched an allocated label (tests use this to
    /// prove that a hostile mutation actually reached a witness).
    pub(crate) fn overrides_used(&self) -> usize {
        self.used_overrides
    }
}
