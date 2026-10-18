// R191 prospective external public-API test wiring only.
// The production lexer is linked as a normal rlib, never compiled with cfg(test).
pub use mori_lexer::*;
#[path = "tests/lexer_token_limit.rs"]
mod selected_lexer_tests;
