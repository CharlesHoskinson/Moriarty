//! Unregistered host AST bridge for the current dedicated-sort source carriers.
//!
//! Root owns registration and the existing cfg(test)-only / no Stage-2 seam.
//! This copies AST shape, not execution state, profiles, budgets or provenance.
//! A and B have distinct helper/fuel schedules; no behavioral qualification is
//! implied. Legacy [`from_program`] remains available to its existing guarded
//! scalar and fixture callers. [`from_program_bounded`] enforces explicit
//! caller-owned [`SourceCopyBudget`] extents before every affected bridge
//! worklist, child slot, arm, string, list, or tree allocation.
//!
//! Security boundary: [`from_program_bounded`] bounds only the allocations
//! performed while constructing [`b::CoreProgram`] in this bridge. It does not
//! bound transient replacement-box/worklist allocations inside owner-provided
//! iterative `Drop` for `CoreProgram`/`CoreExpr`/`CoreTree`, recursive `Arc`
//! destruction of `a::Program`, owner `prepare_core`/`admit`/`admit_run`
//! allocations, downstream provenance/report/JSON serialization, or global
//! host heap/stack/time admission (H1).

use crate::sherlock as a;
use super::shared as b;
use b::i128_class::KIntC;

