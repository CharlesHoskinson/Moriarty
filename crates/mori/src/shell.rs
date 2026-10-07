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
/// Command results go to stdout and never pass through the shell. Output is
/// best effort: if stderr cannot be written, there is nowhere to report it.
pub struct Shell {
    err: AutoStream<Stderr>,
    verbosity: Verbosity,
}

impl Shell {
    pub fn new(verbosity: Verbosity, color: ColorChoice) -> Self {
        Self {
            err: AutoStream::new(io::stderr(), color),
            verbosity,
        }
    }

    /// Prints a right-aligned green verb followed by a message.
    pub fn status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    /// Like [`Shell::status`], but only under `--verbose`.
    pub fn verbose_status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity == Verbosity::Verbose {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    pub fn note(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("note", styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command emits warnings")]
    pub fn warn(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("warning", styles::WARN, message);
        }
    }

    fn aligned(&mut self, verb: &str, style: Style, message: impl Display) {
        let _ = writeln!(self.err, "{style}{verb:>VERB_WIDTH$}{style:#} {message}");
    }

    fn prefixed(&mut self, label: &str, style: Style, message: impl Display) {
        let _ = writeln!(self.err, "{style}{label}:{style:#} {message}");
    }
}
