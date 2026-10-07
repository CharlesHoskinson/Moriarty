use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::NamedSource;

use crate::error::CliError;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax only, until name resolution and type checking exist.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    if args.json {
        todo!("mori check --json");
    }
    let name = args.file.display().to_string();
    let source = std::fs::read_to_string(&args.file).map_err(|source| CliError::Read {
        path: args.file.clone(),
        source,
    })?;

    shell.status("Checking", &name);
    let started = Instant::now();
    let ast = mori_parser::parse(&source)
        .map_err(|error| error.with_source_code(NamedSource::new(&name, source.clone())))?;
    shell.verbose_status(
        "Parsed",
        format!("{} tokens, {} nodes", ast.tokens.len(), ast.nodes.len()),
    );
    shell.status(
        "Finished",
        format!("syntax check in {:.2}s", started.elapsed().as_secs_f64()),
    );
    shell.note("names, types and values are not checked yet");
    Ok(ExitCode::SUCCESS)
}
