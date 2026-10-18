//! Local Source/6 presentation for the closed beta transfer slice.
//!
//! This crate emits a proposal. Its scenario cells and declared hashes remain
//! claims; it performs no Core/5 preparation, signing, proof or ledger action.
//! The generated text must be consumed by the existing Source/6 lowerer before
//! any claim of Source/6 formation or Core/5 preparation.

#[cfg(feature = "source6-transfer-slice")]
mod diagnostics;
#[cfg(feature = "source6-transfer-slice")]
mod transfer;

#[cfg(feature = "source6-transfer-slice")]
pub use transfer::{
    Allowance, Balance, LocalTransferScenario, Replay, TransferExpansion,
    expand_transfer,
};
