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
mod reference_oracles_tests;

#[cfg(test)]
mod resource_tests;

#[cfg(test)]
mod compare_tests;
