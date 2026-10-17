// SDK-free joined source for metadata diagnosis; no runtime/adoption claim.
#![forbid(unsafe_code)]
#[path = "inputs/raw.rs"]
pub mod raw;
#[path = "inputs/shared.rs"]
pub mod shared;
mod families;
pub use families::HostFamilyPlans;
pub mod typed_inputs;
pub mod observation_json;
pub mod execution_capture;

// Source-only registration against exact existing public A type declarations.
#[path = "inputs/source_a_types.rs"]
pub mod sherlock;
pub mod source_input_bridge;
