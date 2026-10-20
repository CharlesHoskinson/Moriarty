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
    /// Explicit source dialect, including nominal-body-typed/1; no autodetection
    #[arg(long, requires = "execution_profile")]
    pub source_profile: Option<String>,
    /// Select the draft or nominal program's sole closed entry; no external inputs
    #[arg(long, requires = "execution_profile", conflicts_with = "action")]
    pub entry: bool,
    /// Exact source action; without --execution-inputs it must be parameter-free
    #[arg(long, requires = "execution_profile")]
    pub action: Option<String>,
    /// Exact configuration for the selected source profile; no numerical defaults
    #[arg(long, requires = "execution_profile")]
    pub execution_config: Option<PathBuf>,
    /// Typed input map for Core/1 action binding; values remain visible in the artifact
    #[arg(long, value_name = "FILE", requires_all = ["execution_profile", "action"], conflicts_with = "entry")]
    pub execution_inputs: Option<PathBuf>,
    /// Explicit host observation after actual preparation/admission
    #[arg(long, requires = "execution_profile")]
    pub observe: bool,
}
