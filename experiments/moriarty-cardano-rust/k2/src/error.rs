//! Driver refusals.
//!
//! A `DriverError` means nothing was set up, proved, exported or accepted. Only
//! [`DriverError::Unsatisfied`] and [`DriverError::ProofRejected`] are verdicts, and both are
//! refusals: the first comes from the compiled relation, the second from the pairing check.
use std::fmt;

#[derive(Debug)]
pub enum DriverError {
    /// Bounded reading or strict JSON parsing refused the bytes.
    Input(String),
    /// A selector value outside the supported SettlementHashV2 profile.
    Unsupported(String),
    /// The actual compiler refused (Core parse/validate, plan, adapter, bound, staged `/2`).
    Compile(mori_r1cs::CompileError),
    /// No claim was proposed (reference refusal, record shape or frame change).
    Proposal(mori_r1cs::MappingError),
    /// Two identities that must be equal differ.
    IdentityMismatch { what: String, expected: String, actual: String },
    /// The compiled relation is not satisfied by the claimed transition in this context.
    Unsatisfied(String),
    /// Bytes are not a canonical, valid encoding of the profile.
    Encoding(String),
    /// Synthesis, the Groth16 backend or an internal invariant failed.
    Backend(String),
    /// A Groth16 pairing check returned false.
    ProofRejected(String),
    /// A file-system effect was refused (for example an output path that already exists).
    Io(String),
}

impl DriverError {
    pub(crate) fn mismatch(what: impl Into<String>, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::IdentityMismatch { what: what.into(), expected: expected.into(), actual: actual.into() }
    }
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(m) => write!(f, "input: {m}"),
            Self::Unsupported(m) => write!(f, "unsupported: {m}"),
            Self::Compile(e) => write!(f, "compile: {e}"),
            Self::Proposal(e) => write!(f, "proposal: {e}"),
            Self::IdentityMismatch { what, expected, actual } => {
                write!(f, "identity mismatch on {what}: expected {expected}, got {actual}")
            }
            Self::Unsatisfied(m) => write!(f, "relation unsatisfied: {m}"),
            Self::Encoding(m) => write!(f, "encoding: {m}"),
            Self::Backend(m) => write!(f, "backend: {m}"),
            Self::ProofRejected(m) => write!(f, "proof rejected: {m}"),
            Self::Io(m) => write!(f, "io: {m}"),
        }
    }
}

impl std::error::Error for DriverError {}
