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
#[path = "sherlock/draft_source.rs"]
mod sherlock_draft;
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[cfg(test)]
#[path = "sherlock/draft_source_tests.rs"]
mod sherlock_draft_tests;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge.rs"]
mod sherlock_draft_image;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge_tests.rs"]
mod sherlock_draft_image_tests;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
// Read-only financial observations over the existing Core/2 public carrier.
#[path = "sherlock/financial_projection.rs"]
pub mod sherlock_financial_projection;
pub mod stage;
mod replay;
mod runtime;
mod source_sponsor;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;

// SDK-free typed host components; no lowering/circuit/native claim.
#[path = "sherlock/word_arithmetic.rs"]
pub mod sherlock_word_arithmetic;
