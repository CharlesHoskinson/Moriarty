//! Production compile arguments shared by the public CLI and direct consumers.
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    /// Exact target profile JSON with network, anchor and compiler limits
    #[arg(long)]
    pub target: PathBuf,
    /// New output directory; refuses to overwrite existing artifacts
    #[arg(long)]
    pub out: PathBuf,
    /// Explicit Sherlock execution profile; omission preserves ordinary Core compilation
    #[arg(long)]
    pub execution_profile: Option<String>,
    /// Explicit source dialect on the selected Sherlock route; no autodetection
    #[arg(long, requires = "execution_profile")]
    pub source_profile: Option<String>,
    /// Select the draft program's sole closed entry; no external inputs
    #[arg(long, requires = "execution_profile", conflicts_with = "action")]
    pub entry: bool,
    /// Exact parameter-free source action for selected Sherlock preparation
    #[arg(long, requires = "execution_profile")]
    pub action: Option<String>,
    /// Exact configuration for the selected source profile; no numerical defaults
    #[arg(long, requires = "execution_profile")]
    pub execution_config: Option<PathBuf>,
    /// Explicit host observation after actual preparation/admission
    #[arg(long, requires = "execution_profile")]
    pub observe: bool,
}
