use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

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
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    if mori_lexer::first_token_text(&source).is_ok_and(|token| token == "language") {
        let result = mori_core::parse(&source);
        if args.json {
            let report = match &result {
                Ok(program) => {
                    serde_json::json!({"status":"accepted", "file":name,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
                }
                Err(error) => {
                    serde_json::json!({"status":"rejected","file":name,"diagnostics":[error]})
                }
            };
            super::core_io::print(&report)?;
        } else {
            match &result {
                Ok(_) => shell.status("Finished", "bounded core check"),
                Err(error) => shell.error(error),
            }
        }
        return Ok(if result.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        });
    }
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
        super::core_io::print(&report)?;
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
