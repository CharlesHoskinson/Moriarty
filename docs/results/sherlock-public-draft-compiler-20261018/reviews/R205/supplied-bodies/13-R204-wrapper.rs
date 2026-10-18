// R204 prospective external type-checker test wiring only.
pub use mori_core::sherlock;
#[path = "candidate/draft_source.rs"]
pub mod sherlock_draft;
#[path = "candidate/draft_typecheck.rs"]
pub mod sherlock_draft_types;
