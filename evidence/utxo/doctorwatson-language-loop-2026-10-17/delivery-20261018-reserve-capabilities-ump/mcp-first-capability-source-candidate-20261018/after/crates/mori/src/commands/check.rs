use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use super::check_source::{CheckAnalysis, analyze_source};
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
    let source = super::core_io::read_source(&args.file)?;
    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    let analysis = analyze_source(&name, &source);
    let accepted = analysis.is_accepted();

    if args.json {
        match &analysis {
            // Serialize the original struct so its public field order stays intact.
            CheckAnalysis::Beta { report, .. } => super::core_io::print(report)?,
            CheckAnalysis::Core { .. } => {
                let report = analysis
                    .cli_report(&name, &source)
                    .map_err(super::core_io::operation)?;
                super::core_io::print(&report)?;
            }
        }
    } else {
        match analysis {
            CheckAnalysis::Core { result, .. } => match result {
                Ok(_) => shell.status("Finished", "bounded core check"),
                Err(error) => shell.error(error),
            },
            CheckAnalysis::Beta {
                diagnostics,
                parsed,
                ..
            } => {
                if let Some((tokens, nodes)) = parsed {
                    shell.verbose_status("Parsed", format!("{tokens} tokens, {nodes} nodes"));
                }
                if finish(shell, &name, &source, diagnostics) {
                    shell.status(
                        "Finished",
                        format!("check in {:.2}s", started.elapsed().as_secs_f64()),
                    );
                }
            }
        }
    }
    Ok(if accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Renders human diagnostics. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    let count = diagnostics.len();
    for diagnostic in diagnostics {
        shell.report(&diagnostic.with_source_code(NamedSource::new(
            crate::display::text(name),
            source.to_owned(),
        )));
    }
    let plural = if count == 1 { "" } else { "s" };
    shell.error(format!(
        "could not check `{name}` due to {count} previous error{plural}"
    ));
    false
}
