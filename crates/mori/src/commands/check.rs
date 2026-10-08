use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::error::CliError;
use crate::json::CheckReport;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = std::fs::read_to_string(&args.file).map_err(|source| CliError::Read {
        path: args.file.clone(),
        source,
    })?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    let (accepted, report) = match mori_parser::parse(&source) {
        Err(error) => {
            let diagnostics = vec![error];
            let report = args
                .json
                .then(|| CheckReport::new(name.clone(), &source, None, &diagnostics));
            (
                finish(shell, &name, &source, diagnostics, args.json),
                report,
            )
        }
        Ok(ast) => {
            if !args.json {
                shell.verbose_status(
                    "Parsed",
                    format!("{} tokens, {} nodes", ast.tokens.len(), ast.nodes.len()),
                );
            }
            let result = mori_checker::check(&ast);
            let report = args.json.then(|| {
                CheckReport::new(
                    name.clone(),
                    &source,
                    Some(&result.checked),
                    &result.diagnostics,
                )
            });
            (
                finish(shell, &name, &source, result.diagnostics, args.json),
                report,
            )
        }
    };

    if let Some(report) = report {
        let json = serde_json::to_string_pretty(&report).expect("the report serializes");
        println!("{json}");
    } else if accepted {
        shell.status(
            "Finished",
            format!("check in {:.2}s", started.elapsed().as_secs_f64()),
        );
    }
    Ok(if accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(name, source.to_owned())));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}
