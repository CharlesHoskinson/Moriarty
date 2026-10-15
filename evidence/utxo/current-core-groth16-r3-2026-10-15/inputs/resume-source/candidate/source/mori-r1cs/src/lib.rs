//! UNADOPTED candidate compiler from an admitted current-Core `moriarty/1` typed
//! `Program` to an arkworks 0.4 R1CS relation, one relation per (program, action).
//!
//! Source only: nothing in this crate has been compiled, tested or run.
//!
//! Pipeline, the same for every admitted program:
//!
//! 1. [`frontend`]: actual `mori_core::language` routing, then `mori_core::parse` +
//!    `validate` + `stage::require_core1` for `/1`, or `mori_core::stage::load` for
//!    `/2`. `/2` is admitted and then refused by [`compile_source`] with a
//!    [`staged::StagedLoweringPlan`] built from the real stage plan.
//! 2. [`plan`]: a static analysis of the selected action over the typed IR: name
//!    environment, represented balance cells, the program duty registry, ordered effect
//!    shapes and the logical-work constant `W`. Unsupported constructs refuse here.
//! 3. [`statement`]: the typed statement layout (header constants and body slots) and
//!    its exact field encoding.
//! 4. `lower`: constraint lowering of every statement and every expression node with
//!    the exact-integer gadgets. Nothing is recognised by program name or hash.
//! 5. [`circuit`]: entry wires, lowering, cap and accounting constraints, and binding of
//!    every slot through the UNADOPTED [`adapter`] boundary.
//!
//! Separation of witness proposal and acceptance: [`witness`] may call the current
//! Core reference (`simulate`/`simulate_metered`) to *propose* the claimed transition.
//! The circuit recomputes every internal wire from the claimed pre-state and inputs and
//! constrains it against the claimed post-state and effects; no reference result is a
//! Boolean the circuit trusts. A wrong proposal yields an unsatisfied constraint system,
//! not an accepted claim.
#![forbid(unsafe_code)]

pub mod adapter;
pub mod circuit;
pub mod coverage;
pub mod error;
pub mod frontend;
mod gadgets;
#[cfg(feature = "groth16")]
pub mod groth16;
pub mod identity;
mod lower;
pub mod plan;
pub mod r1cs;
pub mod staged;
pub mod statement;
pub mod witness;

pub use adapter::{AdapterError, StatementAdapter, initial_ledger_target_admits};
pub use circuit::{Assignment, CompiledRelation, ProbeReport, SynthesisReport, TransitionCircuit, compile, compile_source};
pub use error::{CompileError, MappingError};
pub use frontend::{Admitted, AdmittedCore1, admit_program, admit_source};
pub use identity::{CompilerBuild, Digest32, ProgramIdentity, RelationIdentity};
pub use plan::{AccountingBinding, ActionPlan, CapBinding, CellKey, CellRef, CompileOptions, DutyKey, EffectShape, FrameCells};
pub use r1cs::Tamper;
pub use statement::{BodyValues, DutyValue, EffectValues, Slot, SlotEncoding, SlotKind, StatementLayout, TransitionValues, Wide};

/// Version of the transition relation family produced by this compiler.
pub const RELATION_VERSION: u64 = 1;
/// Human-readable relation family; part of the profile digest.
pub const RELATION_FORMAT: &str = "mori-r1cs-transition/1-unadopted";
/// The smallest scalar-field bit size accepted. Every exact-integer argument in the
/// gadgets needs `r > 2^195`; a 198-bit modulus guarantees `r >= 2^197`.
pub const MIN_MODULUS_BITS: u32 = 198;
