//! Bounded composable Moriarty language and deterministic local semantics.
pub mod intent;
mod ir;
pub mod json;
mod logical_cost;
mod parser;
pub mod policy;
pub mod sherlock;
// Explicit selected HostOnly source preparation/report; no native export.
#[path = "sherlock/compile_artifact.rs"]
pub mod sherlock_compile_artifact;

// Explicit dedicated-draft host preparation; ordinary Core grammar is unchanged.
// These lexer-dependent modules stay outside Sherlock's standalone Std graph.
mod replay;
mod runtime;
#[path = "sherlock/draft_source.rs"]
mod sherlock_draft;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge.rs"]
mod sherlock_draft_image;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge_tests.rs"]
mod sherlock_draft_image_tests;
#[cfg(test)]
#[path = "sherlock/draft_source_tests.rs"]
mod sherlock_draft_tests;
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
// Read-only financial observations over the existing Core/2 public carrier.
#[path = "sherlock/financial_projection.rs"]
pub mod sherlock_financial_projection;
mod source_sponsor;
pub mod stage;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;

// SDK-free typed host components; no lowering/circuit/native claim.
#[path = "sherlock/word_arithmetic.rs"]
pub mod sherlock_word_arithmetic;

// Explicit typed host arithmetic lowering used by the selected Scalar family.
#[path = "sherlock/word_lowering.rs"]
pub mod sherlock_word_lowering;

// Explicit nominal text source -> existing owner type preparation, no evaluation.
#[path = "sherlock/nominal_source.rs"]
pub mod sherlock_nominal_source;

// Additive owner-prepared nominal body typing proposal; no public path adoption.
#[path = "sherlock/nominal_body_typecheck.rs"]
pub mod sherlock_nominal_body_types;

// Pure typed selected-ledger source adapter; no native or financial execution.
#[path = "sherlock/selected_ledger.rs"]
pub mod sherlock_selected_ledger;
