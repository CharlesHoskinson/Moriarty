//! Shared test helpers. Source only: none of these tests has been compiled or run.
//!
//! Expectation attribution used throughout the tests:
//! - [ORACLE] values come unchanged from the frozen non-author `/1` oracle packet
//!   (fixtures/reused, GPT-6.1 Sol high, pending its own review) or the frozen staged
//!   positive oracle (fixtures/staged);
//! - [AUTHOR] values come from the source author's frozen packet
//!   (EXPECTED-PACKET/AUTHOR-EXPECTATIONS.json, ADDENDUM-1.json): an extra derivation,
//!   NOT an independent oracle;
//! - [HOSTILE] marks a claim built to look plausible (wrapping arithmetic,
//!   short-circuit evaluation, non-canonical encodings, tampered witnesses) that the
//!   relation must refuse.
//! No expected value is obtained by running the compiler or the evaluator, except in
//! `propose.rs`, which compares the reference proposal against oracle values.
#![allow(dead_code)]

use ark_bls12_381::Fr;
use ark_relations::r1cs::ConstraintSystem;
use mori_core::{Effect, ExprId, Execution, Scenario, Snapshot, Statement, UInt, Value};
use mori_r1cs::*;
use serde_json::Value as Json;

macro_rules! packet {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../", $path))
    };
}

pub const INVOICE: &str = packet!("fixtures/reused/installment-invoice.mori");
pub const RESERVE: &str = packet!("fixtures/reused/conditional-reserve.mori");
pub const INVOICE_PROGRAM: &str = packet!("fixtures/reused/installment-invoice.expected-program.json");
pub const RESERVE_PROGRAM: &str = packet!("fixtures/reused/conditional-reserve.expected-program.json");
pub const PAYROLL: &str = packet!("EXPECTED-PACKET/fresh/payroll-ledger.mori");
pub const ARITH: &str = packet!("EXPECTED-PACKET/fresh/arith-boundary.mori");
pub const SELF_MOVE: &str = packet!("EXPECTED-PACKET/fresh/self-move.mori");
pub const DELIVERY2: &str = packet!("fixtures/staged/delivery.mori");
pub const RESERVE2: &str = packet!("fixtures/staged/reserve.mori");
pub const DELIVERY2_PLAN: &str = packet!("fixtures/staged/delivery.expected-plan.json");
pub const RESERVE2_PLAN: &str = packet!("fixtures/staged/reserve.expected-plan.json");
pub const AUTHOR: &str = packet!("EXPECTED-PACKET/AUTHOR-EXPECTATIONS.json");
pub const ADDENDUM: &str = packet!("EXPECTED-PACKET/ADDENDUM-1.json");

pub fn json(text: &str) -> Json {
    serde_json::from_str(text).expect("frozen JSON parses")
}

/// [ORACLE] frozen `/1` lifecycle vectors.
pub fn fixture(name: &str) -> Json {
    json(match name {
        "invoice-open" => packet!("fixtures/reused/invoice-open.json"),
        "invoice-partial" => packet!("fixtures/reused/invoice-partial.json"),
        "invoice-finish" => packet!("fixtures/reused/invoice-finish.json"),
        "reserve-allocate" => packet!("fixtures/reused/reserve-allocate.json"),
        "reserve-rebate4" => packet!("fixtures/reused/reserve-rebate4.json"),
        "reserve-rebate2" => packet!("fixtures/reused/reserve-rebate2.json"),
        other => panic!("no fixture {other}"),
    })
}

/// TEST-ONLY placeholder build digests. A real run supplies the frozen
/// BUILD-MANIFEST digest and CORE-CLOSURE-PINS `closure_sha256`.
pub fn build() -> CompilerBuild {
    CompilerBuild { compiler_source_manifest_sha256: Digest32([0x5a; 32]), core_closure_sha256: Digest32([0xc0; 32]) }
}

pub fn options(action: &str, cap: CapBinding) -> CompileOptions {
    CompileOptions {
        action: action.into(),
        cap,
        frame: FrameCells::TouchedOnly,
        accounting: AccountingBinding::NotBound,
        compiler: build(),
    }
}

pub fn compile(source: &str, options: CompileOptions) -> CompiledRelation {
    compile_source::<Fr>(source, options, StatementAdapter::DirectFieldV0).unwrap_or_else(|e| panic!("compile refused: {e}"))
}

pub fn try_compile(source: &str, options: CompileOptions) -> Result<CompiledRelation, CompileError> {
    compile_source::<Fr>(source, options, StatementAdapter::DirectFieldV0)
}

pub struct Outcome {
    pub satisfied: bool,
    pub overrides_used: usize,
}

pub fn run(rel: &CompiledRelation, assignment: Assignment<Fr>, tamper: Tamper<Fr>) -> Outcome {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let report = rel.circuit(Some(assignment), tamper).expect("same field").synthesize_into(cs.clone()).expect("synthesis");
    Outcome { satisfied: cs.is_satisfied().expect("prove-mode system"), overrides_used: report.overrides_used }
}

pub fn assignment(rel: &CompiledRelation, values: TransitionValues) -> Assignment<Fr> {
    rel.assignment::<Fr>(values).expect("claim encodes")
}

pub fn accepts(rel: &CompiledRelation, values: TransitionValues) -> bool {
    run(rel, assignment(rel, values), Tamper::none()).satisfied
}

/// Encodes `values`, then lets a hostile claimant edit the public vector.
pub fn accepts_public(rel: &CompiledRelation, values: TransitionValues, mutate: impl FnOnce(&mut Vec<Fr>)) -> bool {
    let mut a = assignment(rel, values);
    mutate(&mut a.public);
    run(rel, a, Tamper::none()).satisfied
}

pub fn accepts_tampered(rel: &CompiledRelation, values: TransitionValues, tamper: Tamper<Fr>) -> Outcome {
    run(rel, assignment(rel, values), tamper)
}

pub fn offset(rel: &CompiledRelation, label: &str) -> usize {
    rel.layout().slot(label).unwrap_or_else(|| panic!("no slot {label}")).offset
}

pub fn fr(v: u128) -> Fr {
    Fr::from(v)
}

pub fn u(n: u128) -> Value {
    Value::UInt(UInt(n))
}

pub fn b(x: bool) -> Value {
    Value::Bool(x)
}

pub fn uint(text: &str) -> u128 {
    text.parse().expect("canonical decimal")
}

/// [AUTHOR] named constant from AUTHOR-EXPECTATIONS.json.
pub fn constant(name: &str) -> u128 {
    uint(json(AUTHOR)["constants"][name].as_str().expect("constant"))
}

/// [AUTHOR] vector by id.
pub fn vector(id: &str) -> Json {
    json(AUTHOR)["vectors"].as_array().expect("vectors").iter().find(|v| v["id"] == id).cloned().unwrap_or_else(|| panic!("no vector {id}"))
}

pub struct OracleCase {
    pub scenario: Scenario,
    pub execution: Execution,
    pub cap: UInt,
    pub used: UInt,
    pub schedule: Vec<UInt>,
}

/// [ORACLE] scenario, complete expected execution and logical-cost record.
pub fn oracle_case(name: &str) -> OracleCase {
    let v = fixture(name);
    let schedule = v["logical_schedule"]
        .as_array()
        .expect("schedule")
        .iter()
        .map(|s| UInt(uint(s["total"].as_str().expect("total"))))
        .collect();
    OracleCase {
        scenario: serde_json::from_value(v["scenario"].clone()).expect("Scenario"),
        execution: serde_json::from_value(v["expected"]["execution"].clone()).expect("Execution"),
        cap: serde_json::from_value(v["expected"]["cost"]["cap"].clone()).expect("cap"),
        used: serde_json::from_value(v["expected"]["cost"]["used"].clone()).expect("used"),
        schedule,
    }
}

/// Maps a frozen oracle record (never a fresh evaluation) into a claim.
pub fn oracle_values(rel: &CompiledRelation, case: &OracleCase, cap: Option<UInt>) -> TransitionValues {
    witness::from_execution(rel, &case.scenario, &case.execution, cap).expect("oracle record maps onto the relation")
}

/// Maps Core-format records transcribed from a frozen packet.
pub fn record_values(rel: &CompiledRelation, scenario: &Scenario, post: &Snapshot, effects: &[Effect], cap: Option<UInt>) -> TransitionValues {
    let pre = Snapshot { state: scenario.state.clone(), balances: scenario.balances.clone(), liabilities: scenario.liabilities.clone() };
    witness::from_records(rel.plan(), &scenario.inputs, &pre, post, effects, cap).expect("packet record maps onto the relation")
}

/// Root expression of statement `i` of the relation's action, read from the admitted
/// typed IR (labels of in-circuit witnesses are `s{i}/e{root}/...`).
pub fn root(rel: &CompiledRelation, i: usize) -> ExprId {
    let action = &rel.program().actions[usize::from(rel.plan().action_index)];
    match &action.statements[i] {
        Statement::Let { value, .. } | Statement::Next { value, .. } | Statement::Emit { value, .. } => *value,
        Statement::Requires { predicate } | Statement::Ensures { predicate } => *predicate,
        Statement::Transfer { amount, .. }
        | Statement::Fee { amount, .. }
        | Statement::Liability { amount, .. }
        | Statement::Discharge { amount, .. } => *amount,
        Statement::Await { guard, .. } => *guard,
    }
}

/// Sets a UInt slot `[lo, hi]` in a public vector.
pub fn set_uint(public: &mut [Fr], at: usize, v: u128) {
    public[at] = fr(v & u128::from(u64::MAX));
    public[at + 1] = fr(v >> 64);
}

/// Tamper entries writing `v` into the 128 bit witnesses of a UInt allocation.
pub fn uint_bits(mut tamper: Tamper<Fr>, prefix: &str, v: u128) -> Tamper<Fr> {
    for i in 0..128 {
        tamper = tamper.set(format!("{prefix}.bit{i}#0"), fr((v >> i) & 1));
    }
    tamper
}
