//! Shared production argument definitions for the public CLI and direct consumers.
//! No command handlers, backend/ledger imports, file access or runtime dispatch.
use clap::ValueEnum;

/// Signed native owner-intent profile selected explicitly by name.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SignedProfile {
    #[value(name = "moriarty-jubjub-owner-intent/1")]
    JubjubOwnerIntent,
}

#[path = "cli_args/replay.rs"]
pub mod replay;
#[path = "cli_args/simulate.rs"]
pub mod simulate;
