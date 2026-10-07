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

/// Checks syntax and names. Values, types and declaration rules come later.
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
    let diagnostics = match mori_parser::parse(&source) {
        Err(error) => vec![error],
        Ok(ast) => {
            shell.verbose_status(
                "Parsed",
                format!("{} tokens, {} nodes", ast.tokens.len(), ast.nodes.len()),
            );
            mori_checker::check(&ast).diagnostics
        }
    };

    if !diagnostics.is_empty() {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(&name, source.clone())));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
        return Ok(ExitCode::FAILURE);
    }

    shell.status(
        "Finished",
        format!("check in {:.2}s", started.elapsed().as_secs_f64()),
    );
    shell.note("declaration, call and intent rules are not checked yet");
    Ok(ExitCode::SUCCESS)
}
