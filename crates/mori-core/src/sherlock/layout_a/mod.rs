//! Internal cfg(test), host-only L4 preparation and typed state codec source.
//! Compilation and tests are UNRUN. No evaluator, network importer or native ABI.
mod rows;
mod limits;
mod constants;
mod prepare;
mod encoding;
mod state;
mod state_encode;
mod state_decode;
mod state_encoding;
mod observation;
pub(super) use rows::*;
pub(super) use limits::{Locator, PrepareFailure};
pub(super) use prepare::prepare_a;
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
mod auxiliary;
#[cfg(test)]
mod aux_normal_tests;
