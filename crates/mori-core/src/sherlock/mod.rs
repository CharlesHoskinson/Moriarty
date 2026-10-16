//! Sherlock's `sherlock-k-a/1` host reference component.
//!
//! This typed API represents the valid-UInt128, nonnegative-budget common
//! domain of the frozen K reference. It does not change Moriarty source `/1`
//! or `/2`, implement the proposed B helper schedule, or produce constraints.
//! Pure helpers and host allocation remain unmetered in this A profile.

pub mod compare;
mod reference;
pub mod reference_display;
mod reference_types;

pub use reference::Machine;
pub use reference_types::*;

#[cfg(test)]
mod reference_tests;

#[cfg(test)]
mod reference_bounded_tests;

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
#[cfg(test)]
mod reference_slice;

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

// Internal L4 host preparation source proposal; H1, no network/untrusted API.
// Compilation and all normal tests remain UNRUN pending separate admission.
#[cfg(test)]
mod layout_a;
