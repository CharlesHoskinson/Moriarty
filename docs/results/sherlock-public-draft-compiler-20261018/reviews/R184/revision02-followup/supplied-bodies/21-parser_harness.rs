// R191 prospective external test wiring only; not a production module change.
pub use mori_core::sherlock;
#[path = "candidate/draft_source.rs"]
mod candidate_parser;
use candidate_parser::*;
use crate::sherlock::{Arm, BinaryOp, Expr, ExprRef, Function, Name, Parameter,
    Pattern, Program, Type, UnaryOp, Value};
use std::sync::Arc;
#[path = "tests/bounded_parser.rs"]
mod selected_parser_tests;
