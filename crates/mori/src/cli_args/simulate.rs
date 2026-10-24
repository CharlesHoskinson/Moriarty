//! The production simulate arguments shared with the public CLI.
//! Runtime handlers remain in commands::simulate and commands::simulate::sherlock.
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME", required_unless_present_any = ["resume", "execution_profile"])]
    pub action: Option<String>,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE", required_unless_present_any = ["resume", "execution_profile"])]
    pub scenario: Option<PathBuf>,
    /// Resume a moriarty/2 continuation (local, unauthenticated, consumption untracked)
    #[arg(
        long,
        value_name = "FILE",
        requires = "event",
        conflicts_with_all = ["scenario", "trace", "target", "profile", "authority", "at", "signature"]
    )]
    pub resume: Option<PathBuf>,
    /// Stage event JSON for --resume (at most 64 KiB)
    #[arg(long, value_name = "FILE", requires = "resume")]
    pub event: Option<PathBuf>,
    /// Generate a complete public replay sidecar against a trusted Target
    #[arg(long)]
    pub trace: bool,
    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with = "trace")]
    pub logical_cap: Option<mori_core::UInt>,
    /// Target JSON: trusted Target for --trace, explicit descriptor for Sherlock
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    #[command(flatten)]
    pub authorization: super::replay::AuthorizationArgs,
    /// Explicit Sherlock execution profile; omission preserves legacy simulation
    #[arg(long, conflicts_with_all = ["action", "scenario", "resume", "event", "trace", "logical_cap", "profile", "authority", "at", "signature"])]
    pub execution_profile: Option<String>,
    /// Explicit source dialect on the selected Sherlock route; no autodetection
    #[arg(long, requires = "execution_profile")]
    pub source_profile: Option<String>,
    /// Select the dedicated draft or body-typed nominal program's closed entry
    #[arg(long, requires = "execution_profile", conflicts_with_all = ["action", "scenario"])]
    pub entry: bool,
    /// Exact caller configuration; no numerical defaults
    #[arg(long, requires = "execution_profile")]
    pub execution_config: Option<PathBuf>,
}
fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}
#[path = "simulate_selection.rs"]
mod selected;
pub use selected::selection;
