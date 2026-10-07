//! Cargo-style status output on stderr.

use std::fmt::Display;
use std::io::{self, Stderr, Write};

use anstream::{AutoStream, ColorChoice};
use anstyle::Style;

use crate::styles;

/// Width of the right-aligned verb column, matching cargo.
const VERB_WIDTH: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// Writes status lines such as `    Checking invoice.mori` to stderr.
///
/// Command results go to stdout and never pass through the shell.
pub struct Shell {
    err: AutoStream<Stderr>,
    verbosity: Verbosity,
}

#[expect(dead_code, reason = "used once command handlers are filled in")]
impl Shell {
    pub fn new(verbosity: Verbosity, color: ColorChoice) -> Self {
        Self {
            err: AutoStream::new(io::stderr(), color),
            verbosity,
        }
    }

    pub fn verbosity(&self) -> Verbosity {
        self.verbosity
    }

    /// Silences status lines and warnings, for example under `--json`.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    /// Prints a right-aligned green verb followed by a message.
    pub fn status(&mut self, verb: &str, message: impl Display) -> io::Result<()> {
        if self.verbosity == Verbosity::Quiet {
            return Ok(());
        }
        self.line(verb, styles::HEADER, message, true)
    }

    /// Like [`Shell::status`], but only under `--verbose`.
    pub fn verbose_status(&mut self, verb: &str, message: impl Display) -> io::Result<()> {
        if self.verbosity != Verbosity::Verbose {
            return Ok(());
        }
        self.line(verb, styles::HEADER, message, true)
    }

    pub fn warn(&mut self, message: impl Display) -> io::Result<()> {
        if self.verbosity == Verbosity::Quiet {
            return Ok(());
        }
        self.line("warning", styles::WARN, message, false)
    }

    /// Prints an error line. Errors are shown even under `--quiet`.
    pub fn error(&mut self, message: impl Display) -> io::Result<()> {
        self.line("error", styles::ERROR, message, false)
    }

    fn line(
        &mut self,
        label: &str,
        style: Style,
        message: impl Display,
        aligned: bool,
    ) -> io::Result<()> {
        if aligned {
            writeln!(self.err, "{style}{label:>VERB_WIDTH$}{style:#} {message}")
        } else {
            writeln!(self.err, "{style}{label}:{style:#} {message}")
        }
    }
}
