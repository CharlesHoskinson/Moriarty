//! Diagnostics for the Moriarty toolchain.
//!
//! Every phase reports problems through one open [`MoriDiagnostic`] type,
//! following oxc's `OxcDiagnostic`. Phases define small constructor functions
//! in their own `diagnostics.rs` instead of a type per error.
//!
//! The `fancy` feature enables the [`render`] module and miette's graphical
//! handler.

mod diagnostic;
#[cfg(any(feature = "fancy", test))]
pub mod render;
mod suggest;

pub use diagnostic::MoriDiagnostic;
pub use miette::{Diagnostic, LabeledSpan, NamedSource, Report, Severity, SourceSpan};
pub use suggest::{corrected_line, suggest};
