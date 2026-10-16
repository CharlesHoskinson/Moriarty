//! Fixed setup-mode matrices of one compiled relation, and the row-by-row decision of any full
//! assignment against them. K4H successor source; never compiled or run by its author.
//!
//! A Groth16 adversary never runs this driver's synthesis. Soundness rests on the matrices the
//! setup reduced, so a hostile claim must be judged by those rows, not by a re-synthesis that
//! recomputes every wire honestly. [`FixedMatrices::of`] synthesizes the relation's own
//! `circuit_v2(None, None, _)` in setup mode (as the compile probe and the ark-groth16 generator
//! do) and refuses unless the `mori-r1cs-matrices/1` digest equals the descriptor's
//! `r1csSha256`. [`FixedMatrices::evaluate`] then decides `(A z) * (B z) = C z` for every row of
//! any vector `z = [1, x1, x2, w...]`, honest or hand-mutated.
//!
//! [`prove_assignment`] produces the vector for a prepared claim. It is a proposal tool: its
//! verdict is never read; only the fixed rows decide. A [`Tamper`] map replaces single labelled
//! witness values and nothing else, which tests use to locate witnesses by label.
use crate::claim::PreparedClaim;
use crate::error::DriverError;
use crate::relation::{self, SelectedRelation};
use ark_bls12_381::Fr;
use ark_ff::{One, Zero};
use ark_relations::r1cs::{ConstraintMatrices, ConstraintSystem, OptimizationGoal, SynthesisMode};
use mori_r1cs::Tamper;

/// The finalized A/B/C matrices of one relation, checked against its descriptor.
#[derive(Debug, Clone)]
pub struct FixedMatrices {
    matrices: ConstraintMatrices<Fr>,
    digest_hex: String,
}

/// The decision of the fixed rows on one full assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluation {
    pub constraints: usize,
    pub violated: usize,
    /// The first (at most 16) violated rows, ascending.
    pub first_violated: Vec<usize>,
}

impl Evaluation {
    pub fn satisfied(&self) -> bool {
        self.violated == 0
    }
}

impl FixedMatrices {
    /// Setup-mode synthesis of the relation's own circuit; refuses unless the matrix digest is
    /// the descriptor's `r1csSha256` and every row index is inside the assignment width.
    pub fn of(selected: &SelectedRelation) -> Result<Self, DriverError> {
        let circuit = selected.relation().circuit_v2::<Fr>(None, None, Tamper::none()).map_err(DriverError::Compile)?;
        let cs = ConstraintSystem::<Fr>::new_ref();
        cs.set_optimization_goal(OptimizationGoal::Constraints);
        cs.set_mode(SynthesisMode::Setup);
        circuit.synthesize_into(cs.clone()).map_err(|e| DriverError::Backend(format!("setup-mode synthesis: {e}")))?;
        cs.finalize();
        let matrices = cs.to_matrices().ok_or_else(|| DriverError::Backend("setup-mode matrices were not constructed".into()))?;
        let digest_hex = relation::matrix_digest(&matrices).hex();
        let expected = &selected.descriptor().r1cs_sha256;
        if &digest_hex != expected {
            return Err(DriverError::mismatch("r1cs matrices (setup synthesis vs descriptor)", expected.clone(), digest_hex));
        }
        let width = matrices.num_instance_variables + matrices.num_witness_variables;
        for m in [&matrices.a, &matrices.b, &matrices.c] {
            if m.len() != matrices.num_constraints || m.iter().flatten().any(|(_, j)| *j >= width) {
                return Err(DriverError::Backend("setup-mode matrices are malformed".into()));
            }
        }
        Ok(Self { matrices, digest_hex })
    }

    pub fn matrices(&self) -> &ConstraintMatrices<Fr> {
        &self.matrices
    }

    pub fn digest_hex(&self) -> &str {
        &self.digest_hex
    }

    /// Length of every full assignment: the constant one, the two public inputs, the witnesses.
    pub fn width(&self) -> usize {
        self.matrices.num_instance_variables + self.matrices.num_witness_variables
    }

    /// `(A z, B z, C z)` of row `r`. `z` must have [`Self::width`] entries.
    pub fn row_values(&self, r: usize, z: &[Fr]) -> (Fr, Fr, Fr) {
        let dot = |row: &[(Fr, usize)]| row.iter().fold(Fr::zero(), |acc, (k, j)| acc + *k * z[*j]);
        (dot(&self.matrices.a[r]), dot(&self.matrices.b[r]), dot(&self.matrices.c[r]))
    }

    pub fn row_satisfied(&self, r: usize, z: &[Fr]) -> bool {
        let (a, b, c) = self.row_values(r, z);
        a * b == c
    }

    /// Every row against `z`. Refuses a vector of the wrong width or without the constant one.
    pub fn evaluate(&self, z: &[Fr]) -> Result<Evaluation, DriverError> {
        if z.len() != self.width() {
            return Err(DriverError::mismatch("full assignment length", self.width().to_string(), z.len().to_string()));
        }
        if z[0] != Fr::one() {
            return Err(DriverError::Backend("assignment entry 0 is not the constant one".into()));
        }
        let mut violated = 0;
        let mut first_violated = Vec::new();
        for r in 0..self.matrices.num_constraints {
            if !self.row_satisfied(r, z) {
                violated += 1;
                if first_violated.len() < 16 {
                    first_violated.push(r);
                }
            }
        }
        Ok(Evaluation { constraints: self.matrices.num_constraints, violated, first_violated })
    }
}

/// One prove-mode synthesis of a prepared claim.
#[derive(Debug, Clone)]
pub struct ProveAssignment {
    /// `[1, x1, x2, w...]`.
    pub full_assignment: Vec<Fr>,
    /// Tamper entries that reached an allocated witness.
    pub overrides_used: usize,
    /// Digest of the prove-mode matrices, when they were constructed.
    pub matrix_digest_hex: Option<String>,
}

/// Synthesizes `claim` in prove mode. The public inputs are the claim's own digest halves; a
/// caller that wants other public inputs edits entries 1 and 2 of the returned vector.
pub fn prove_assignment(
    selected: &SelectedRelation,
    claim: &PreparedClaim,
    tamper: Tamper<Fr>,
    construct_matrices: bool,
) -> Result<ProveAssignment, DriverError> {
    let relation = selected.relation();
    let assignment = relation.assignment_v2::<Fr>(claim.values.clone(), &claim.context).map_err(DriverError::Proposal)?;
    if assignment.public != vec![Fr::from(claim.public_inputs[0]), Fr::from(claim.public_inputs[1])] {
        return Err(DriverError::Backend("assignment public inputs differ from the statement digest halves".into()));
    }
    let circuit = relation
        .circuit_v2::<Fr>(Some(assignment), Some(claim.context.clone()), tamper)
        .map_err(DriverError::Compile)?;
    let cs = ConstraintSystem::<Fr>::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    cs.set_mode(SynthesisMode::Prove { construct_matrices });
    let report = circuit.synthesize_into(cs.clone()).map_err(|e| DriverError::Backend(format!("prove-mode synthesis: {e}")))?;
    let matrix_digest_hex = if construct_matrices {
        cs.finalize();
        let matrices = cs.to_matrices().ok_or_else(|| DriverError::Backend("prove-mode matrices were not constructed".into()))?;
        Some(relation::matrix_digest(&matrices).hex())
    } else {
        None
    };
    let full_assignment = {
        let inner = cs.borrow().ok_or_else(|| DriverError::Backend("constraint system missing".into()))?;
        inner.instance_assignment.iter().chain(inner.witness_assignment.iter()).copied().collect::<Vec<Fr>>()
    };
    Ok(ProveAssignment { full_assignment, overrides_used: report.overrides_used, matrix_digest_hex })
}
