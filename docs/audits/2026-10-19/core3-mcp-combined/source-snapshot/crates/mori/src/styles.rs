//! Cargo's color palette, shared by clap's help output and the shell.

use anstyle::{AnsiColor, Effects, Style};
use clap::builder::Styles;

pub const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub const WARN: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
pub const REMOVED: Style = AnsiColor::Red.on_default();
pub const ADDED: Style = AnsiColor::Green.on_default();

pub const CLAP: Styles = Styles::styled()
    .header(HEADER)
    .usage(HEADER)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(LITERAL)
    .invalid(WARN);
