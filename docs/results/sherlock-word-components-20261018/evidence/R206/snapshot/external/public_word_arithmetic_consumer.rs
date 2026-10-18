// R206 compile-only public API consumer; no selected test execution.
pub use mori_core::sherlock_word_arithmetic::*;
#[path = "word_arithmetic_independent_normal_tests.rs"]
mod independent_normal_tests;
