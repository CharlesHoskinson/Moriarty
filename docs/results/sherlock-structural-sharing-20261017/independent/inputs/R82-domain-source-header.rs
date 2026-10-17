//! SOURCE ONLY: designated-owner exact closure classification precedes execution.
//! Original UIREQ003/004/005/006 positive acceptance examples, typed host A only.
//! Expected results/work/checkpoints are handwritten from the frozen K-A rules.
//! No actual interpreter output or helper evaluator supplies expected values.

use std::sync::Arc;
use mori_core::sherlock::{
    BinaryOp, Control, Environment, Expr, ExprRef, Frame, Function, Machine,
    Outcome, Parameter, Program, Snapshot, StepRecord, Type, Value, PROFILE, K_SOURCE_SHA256,
};

const FUEL: u128 = 512;
const STACK: usize = 32;
const MAX_EVENTS: usize = 513;
const GENERIC_RULES: &[usize] = &[
    80, 102, 108, 114, 117, 123, 129, 131, 135, 145, 149, 153,
    161, 165, 177, 181, 185, 195, 200, 204, 208, 223,
];
fn u(n: u128) -> ExprRef { Arc::new(Expr::Literal(Value::UInt(n))) }
fn b(value: bool) -> ExprRef { Arc::new(Expr::Literal(Value::Bool(value))) }
