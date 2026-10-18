// Reexport only the existing public shared carrier module; no private engine.
pub use mori_core::sherlock::helper_machine::shared;
#[path = "../candidate/source_input_bridge.rs"]
mod source_input_bridge;
