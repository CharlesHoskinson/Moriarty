//! External test consumer of the FULL ordinary mori-core rlib.
//! Compile this file with rustc --test only after a separately authorized normal
//! library build. No Core cfg(test), private implementation include or SDK dev
//! dependency is required. The original private unit-test module is unchanged.
pub use mori_core::sherlock_nominal_body_types::*;
pub use mori_core::{sherlock, sherlock_nominal_source};
use sherlock::helper_machine::shared::CoreValue;
use sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtValue};
use sherlock::helper_machine::shared::owner_ext_type_builder::TypeExprSrc;

// These exact source bytes also define the internal module's functional cases.
// The public aliases above satisfy only its import paths; checker behavior is
// reached through mori_core::sherlock_nominal_body_types in the linked rlib.
#[path = "../../src/sherlock/nominal_body_typecheck_tests.rs"]
mod same_functional_expectations;
