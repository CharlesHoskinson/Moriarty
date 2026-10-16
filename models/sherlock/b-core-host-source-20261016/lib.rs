// SDK-free source composition proposal, not an executable interpreter.
// Raw carriers and row fragments are exact snapshots; owner bodies remain absent.
#![forbid(unsafe_code)]
#[path = "inputs/raw.rs"]
pub mod raw;
#[path = "inputs/shared.rs"]
pub mod shared;
mod families;
pub use families::HostFamilyPlans;
