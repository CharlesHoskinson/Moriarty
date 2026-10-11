use super::core_io;
use crate::{error::CliError, shell::Shell};
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    /// Exact target profile JSON with network, anchor and compiler limits
    #[arg(long)]
    pub target: PathBuf,
    /// New output directory; refuses to overwrite existing artifacts
    #[arg(long)]
    pub out: PathBuf,
}
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let (_, program) = core_io::program(&args.file)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let artifact = if target.signed_policy.is_some() {
        mori_backend::compile_signed(&program, &target)
    } else {
        mori_backend::compile(&program, &target)
    }
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    for (index, circuit) in artifact.circuits.iter().enumerate() {
        let path = args.out.join(format!("{index:03}-{}.zkir", circuit.action));
        std::fs::write(&path, &circuit.zkir).map_err(|source| CliError::Write { path, source })?;
    }
    let path = args.out.join("artifact.json");
    std::fs::write(&path, manifest).map_err(|source| CliError::Write { path, source })?;
    shell.status(
        "Compiled",
        format!(
            "{} circuits into {}",
            artifact.circuits.len(),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}
