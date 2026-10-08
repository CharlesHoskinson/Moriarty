use std::path::{Path, PathBuf};
use std::process::ExitCode;

use mori_diagnostics::{MoriDiagnostic, NamedSource, Report};
use rayon::prelude::*;

use crate::error::CliError;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Files or directories to format; the current directory when omitted
    pub paths: Vec<PathBuf>,

    /// Write nothing; show a diff and fail when a file is not formatted
    #[arg(long)]
    pub check: bool,
}

/// What happened to one file.
enum Outcome {
    Unchanged,
    /// Formatted and written back.
    Formatted,
    /// Under `--check`: the file would change.
    NotFormatted {
        source: String,
        formatted: String,
    },
    /// The file does not parse.
    Invalid {
        source: String,
        error: MoriDiagnostic,
    },
    Failed(CliError),
}

/// Formats every `.mori` file under `paths`, in parallel.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let roots = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths
    };
    let files = collect_files(&roots)?;
    let outcomes: Vec<Outcome> = files
        .par_iter()
        .map(|path| format_file(path, args.check))
        .collect();

    let (mut formatted, mut not_formatted, mut failed) = (0, 0, 0);
    for (path, outcome) in files.iter().zip(outcomes) {
        let name = path.display().to_string();
        match outcome {
            Outcome::Unchanged => {}
            Outcome::Formatted => {
                shell.verbose_status("Formatted", &name);
                formatted += 1;
            }
            Outcome::NotFormatted { source, formatted } => {
                shell.report(&Report::new(mori_fmt::not_formatted(&name)));
                shell.diff(&name, &mori_fmt::diff(&source, &formatted));
                not_formatted += 1;
            }
            Outcome::Invalid { source, error } => {
                shell.report(&error.with_source_code(NamedSource::new(&name, source)));
                failed += 1;
            }
            Outcome::Failed(error) => {
                shell.report(&Report::new(error));
                failed += 1;
            }
        }
    }

    let total = files.len();
    if failed > 0 {
        shell.error(format!(
            "could not format {failed} {} due to errors",
            plural(failed, "file", "files")
        ));
    }
    if not_formatted > 0 {
        let (count, verb) = match not_formatted {
            1 => ("1 file".to_owned(), "is"),
            n => (format!("{n} files"), "are"),
        };
        shell.error(format!(
            "{count} {verb} not formatted; run `mori format` to fix {}",
            plural(not_formatted, "it", "them")
        ));
    } else if args.check && failed == 0 {
        shell.status(
            "Checked",
            format!("{total} {}, all formatted", plural(total, "file", "files")),
        );
    } else if !args.check {
        shell.status(
            "Formatted",
            format!("{formatted} of {total} {}", plural(total, "file", "files")),
        );
    }
    Ok(if failed > 0 || not_formatted > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

fn format_file(path: &Path, check: bool) -> Outcome {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(source) => {
            let path = path.to_owned();
            return Outcome::Failed(CliError::Read { path, source });
        }
    };
    let formatted = match mori_fmt::format(&source) {
        Ok(formatted) => formatted,
        Err(error) => return Outcome::Invalid { source, error },
    };
    if formatted == source {
        return Outcome::Unchanged;
    }
    if check {
        return Outcome::NotFormatted { source, formatted };
    }
    match std::fs::write(path, formatted) {
        Ok(()) => Outcome::Formatted,
        Err(source) => Outcome::Failed(CliError::Write {
            path: path.to_owned(),
            source,
        }),
    }
}

/// The files named, and every `.mori` file under the directories named,
/// skipping hidden and ignored files. Sorted, without repeats.
fn collect_files(roots: &[PathBuf]) -> Result<Vec<PathBuf>, CliError> {
    let mut files = Vec::new();
    for root in roots {
        if root.is_file() {
            files.push(root.clone());
            continue;
        }
        if !root.exists() {
            return Err(CliError::NotFound { path: root.clone() });
        }
        for entry in ignore::WalkBuilder::new(root).build() {
            let entry = entry.map_err(|source| CliError::Walk {
                path: root.clone(),
                source,
            })?;
            // `./amm.mori` reads better as `amm.mori`.
            let path = entry.path();
            let path = path.strip_prefix(".").unwrap_or(path);
            if path.is_file()
                && path
                    .extension()
                    .is_some_and(|extension| extension == "mori")
            {
                files.push(path.to_owned());
            }
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn plural<'a>(count: usize, one: &'a str, many: &'a str) -> &'a str {
    if count == 1 { one } else { many }
}
