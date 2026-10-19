//! Bounded composable Moriarty language and deterministic local semantics.
pub mod intent;
mod ir;
pub mod json;
mod logical_cost;
mod parser;
pub mod policy;
pub mod sherlock;

// Unregistered draft source capability: Core-only ordinary host test proposal.
// These lexer-dependent modules stay outside Sherlock's standalone Std graph.
mod replay;
mod runtime;
#[cfg(test)]
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
#[cfg(test)]
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
mod source_sponsor;
pub mod stage;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, language_v3, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;
