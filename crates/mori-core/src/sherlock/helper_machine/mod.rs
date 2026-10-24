#![forbid(unsafe_code)]
//! Qualified B helper machine (`sherlock-helper-microstep/1`): DedicatedSorts,
//! finance Absent, HostOnly, genesis and one uncut segment.
//! Source integration only; no native or full-B acceptance, and no change to
//! the parent A machine or existing financial execution.

pub mod raw;
pub mod shared;
mod families;
pub mod source_input_bridge;

// Complete owned observations around the existing fixed host owner step.
pub mod complete_observation;
pub mod c6_phys;
pub mod c6_phys_program;
pub mod c6_phys_data;
pub mod c6_phys_helper;
pub mod c6_phys_state;
pub mod c6_phys_host;
// Actual one-step saved-boundary codec adapter; no trace/proof authority.
pub mod c6_complete_step_capture;
pub mod witness_plan;
pub mod complete_comparison;

// Source-only Core scalar consumer and diagnostic projections. No native
// or whole-language acceptance is established by these registrations.
mod core_action_consumer_security;
pub mod core_action_consumer;
pub mod resource_report;
pub mod resource_event_accounting;

#[cfg(test)]
mod core_action_consumer_tests;

#[cfg(test)]
mod core_action_consumer_security_operator_tests;

#[cfg(test)]
mod core_action_resource_integration_tests;

#[cfg(test)]
mod core_action_operator_pipeline_tests;

// Read-only diagnostic reference prefixes from saved actual B observations.
pub mod reference_projection;

// Bounded lossless actual C6 storage, not proof or source authority.
pub mod c6_stream_event;
pub mod c6_range_stream;
pub mod c6_range_stream_read;
// Raw HostSnapshot value archive; diagnostic inverse, no owner authority.
pub mod c6_raw_snapshot_archive;
