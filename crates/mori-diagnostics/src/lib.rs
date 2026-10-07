//! Diagnostics for the Moriarty toolchain.
//!
//! Every phase reports problems through one open [`MoriDiagnostic`] type,
//! following oxc's `OxcDiagnostic`. Phases define small constructor functions
//! in their own `diagnostics.rs` instead of a type per error.
//!
//! This is the only crate that depends on miette. The `fancy` feature enables
//! the [`render`] module and miette's graphical handler.

mod diagnostic;
#[cfg(any(feature = "fancy", test))]
pub mod render;

pub use diagnostic::MoriDiagnostic;
pub use miette::{LabeledSpan, NamedSource, Report, Severity, SourceSpan};
