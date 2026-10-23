//! Sherlock's `sherlock-k-a/1` host reference component.
//!
//! This typed API represents the valid-UInt128, nonnegative-budget common
//! domain of the frozen K reference. It does not change Moriarty source `/1`
//! or `/2`, implement the proposed B helper schedule, or produce constraints.
//! The separate B helper machine lives in the `helper_machine` child module;
//! it does not alter this A profile or its execution paths.
//! Pure helpers and host allocation remain unmetered in this A profile.

pub mod compare;
pub mod helper_machine;
mod reference;
mod reference_stream;
pub mod reference_display;
mod reference_types;

pub use reference::Machine;
pub use reference_stream::{VisitResult, VisitStop};
pub use reference_types::*;

#[cfg(test)]
mod reference_tests;

#[cfg(test)]
mod reference_bounded_tests;

#[cfg(test)]
mod reference_bounded_extra_tests;

#[cfg(test)]
mod normal_oracle_inputs;

#[cfg(test)]
mod reference_oracles_tests;

#[cfg(test)]
mod resource_tests;

#[cfg(test)]
mod compare_tests;

// Host-only image proposal: exercised here without exporting a public runtime API.
#[cfg(test)]
pub(crate) mod program_image;

// Private flat-table consumer proposal, normal-oracle comparisons only.
#[cfg(test)]
pub(crate) mod image_reference;

// Host-only event grouping; every internal reference record stays observable.
pub mod reference_slice;

// Unadopted host snapshot rows; normal materialization checks only.
#[cfg(test)]
mod machine_image;

// Unadopted finite call-helper proposal; clock and handle adapters stay separate.
#[cfg(test)]
mod helper_schedule;

// Host projection of helper completions to the unchanged A reference boundary.
#[cfg(test)]
mod helper_call_adapter;

// Causal occurrence metadata on the private image execution path.
#[cfg(test)]
mod source_origin_normal_tests;

// SDK-free host L4 preparation and typed observation codec.
// No native ABI, machine-state importer or transition acceptance is provided.
pub mod layout_a;

pub mod nominal_external_inputs;

#[cfg(test)]
mod reference_stream_tests;

// Private host arithmetic/fuel materialization; no constraint or wire ABI.
#[cfg(test)]
mod arithmetic_witness;
#[cfg(test)]
mod arithmetic_witness_normal_tests;

// Typed Rust nominal values; source parsing/JSON wire bounds stay unchanged.
pub mod nominal_ast_inputs;
