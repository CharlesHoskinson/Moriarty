//! Genuine status renderer shared by Shell and SDK-free selected consumers.
use crate::styles;
use anstyle::Style;
use std::{fmt::Display, io::Write};

/// Width of the right-aligned verb column, matching cargo.
const VERB_WIDTH: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

pub fn status(err: &mut impl Write, verbosity: Verbosity, verb: &str, message: impl Display) {
    if verbosity != Verbosity::Quiet {
        aligned(err, verb, styles::HEADER, message);
    }
}

pub(crate) fn aligned(err: &mut impl Write, verb: &str, style: Style, message: impl Display) {
    let message = crate::display::text(&message.to_string());
    let _ = writeln!(err, "{style}{verb:>VERB_WIDTH$}{style:#} {message}");
}
