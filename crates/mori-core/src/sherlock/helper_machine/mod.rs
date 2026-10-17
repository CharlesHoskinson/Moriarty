#![forbid(unsafe_code)]
//! Qualified B helper machine (`sherlock-helper-microstep/1`): DedicatedSorts,
//! finance Absent, HostOnly, genesis and one uncut segment.
//! Source integration only; no native or full-B acceptance, and no change to
//! the parent A machine or existing financial execution.

pub mod raw;
pub mod shared;
mod families;
pub mod source_input_bridge;
