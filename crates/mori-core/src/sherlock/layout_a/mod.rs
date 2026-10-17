//! SDK-free host L4 preparation and typed observation codec. Trusted diagnostic use only.
//! PreparedA remains a mutable diagnostic record, not an authenticated capability.
//! No byte parser, transition acceptance or native ABI; no Machine-state importer.
//! decode_observed_a is a public typed-row decoder under the trust preconditions.
//!
//! Unchecked preconditions: every `&PreparedA` is the unmodified value returned by
//! `prepare_a`, and every decoded `StateBlock` came from `encode_observed_a` over a
//! trusted Snapshot with the same PreparedA and capacities. Otherwise calls may panic
//! (direct table indexing), run without practical bound, abort on native stack overflow
//! when decoded recursive values are dropped or compared, or return Ok with program,
//! name or expression data never bound to the ProgramBlock. Err is a located refusal,
//! not an exhaustive validator. Do not pass constructed, mutated or untrusted records.
//!
//! Capacities are caller-trusted with no ceiling. prepare_a, encode_observed_a and
//! observed_state_bytes_a allocate and zero-fill every table to capacity; a successful
//! reservation does not guarantee the host can commit it. Encoder lookups are superlinear.
//!
//! DecodeMode::Step accepts noncanonical in-step rows (Constant-space references,
//! shared subtrees, non-least IDs). Neither mode's Ok is canonical admission,
//! reachability, StepInv, transition acceptance, F4/G4 correctness or native acceptance.
//! A decoded Snapshot is not Machine-produced and cannot be loaded into a Machine.
mod rows;
mod limits;
mod constants;
mod prepare;
mod encoding;
mod state;
mod state_encode;
mod state_decode;
mod state_encoding;
#[cfg(test)]
mod observation;
pub use rows::*;
pub use limits::{Locator, PrepareFailure};
pub use prepare::prepare_a;
pub use state::*;
pub use state_encode::encode_observed_a;
pub use state_decode::decode_observed_a;
pub use state_encoding::observed_state_bytes_a;
#[cfg(test)]
mod normal_tests;
#[cfg(test)]
mod state_encode_normal_tests;
#[cfg(test)]
mod state_decode_normal_tests;
#[cfg(test)]
mod state_encoding_normal_tests;
#[cfg(test)]
mod observation_normal_tests;

// Internal supplied AuxBlock byte component; no complete step-witness profile.
#[cfg(test)]
mod auxiliary;
#[cfg(test)]
mod aux_normal_tests;

// Private decoded A program/state consumer for aux/2 numeric host columns.
#[cfg(test)]
mod arithmetic_fuel;
#[cfg(test)]
mod arithmetic_fuel_normal_tests;
