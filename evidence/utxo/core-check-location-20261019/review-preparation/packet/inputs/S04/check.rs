use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

use super::check_source::{self, SourceCheckConsumer};

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
    let outcome = check_source::dispatch(
        &source,
        CliConsumer {
            name: &name,
            source: &source,
            json: args.json,
            shell,
        },
    )?;
    if outcome.beta {
        if let Some(report) = outcome.report {
            super::core_io::print(&report)?;
        } else if outcome.accepted {
            shell.status(
                "Finished",
                format!("check in {:.2}s", started.elapsed().as_secs_f64()),
            );
        }
    }
    Ok(if outcome.accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

struct CliOutcome {
    accepted: bool,
    beta: bool,
    report: Option<CheckReport>,
}

struct CliConsumer<'a> {
    name: &'a str,
    source: &'a str,
    json: bool,
    shell: &'a mut Shell,
}

impl SourceCheckConsumer for CliConsumer<'_> {
    type Output = super::Result<CliOutcome>;

    fn parsed(&mut self, tokens: usize, nodes: usize) {
        if !self.json {
            self.shell
                .verbose_status("Parsed", format!("{tokens} tokens, {nodes} nodes"));
        }
    }

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        let report = self.json.then(|| {
            check_source::beta_report(self.name.to_owned(), self.source, checked, &diagnostics)
        });
        let accepted = finish(self.shell, self.name, self.source, diagnostics, self.json);
        Ok(CliOutcome {
            accepted,
            beta: true,
            report,
        })
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        if self.json {
            let report = check_source::core_report(self.name, self.source, &result, staged);
            super::core_io::print(&report)?;
        } else {
            match &result {
                Ok(_) => self.shell.status("Finished", "bounded core check"),
                Err(error) => {
                    if let Some(offset) = error
                        .offset
                        .filter(|offset| self.source.is_char_boundary(*offset as usize))
                    {
                        let diagnostic = MoriDiagnostic::error(error.message.clone())
                            .with_code(error.code.clone())
                            .with_label(mori_span::Span::empty(offset).primary_label("here"));
                        self.shell.report(&diagnostic.with_source_code(NamedSource::new(
                            crate::display::text(self.name),
                            self.source.to_owned(),
                        )));
                    } else {
                        self.shell.error(error);
                    }
                }
            }
        }
        Ok(CliOutcome {
            accepted: result.is_ok(),
            beta: false,
            report: None,
        })
    }
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
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}
