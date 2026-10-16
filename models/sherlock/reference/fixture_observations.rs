//! SOURCE_ONLY / UNRUN: the eight original fixed K fixtures on the existing
//! Rust A reference. Proposed location: models/sherlock/reference/fixture_observations.rs.
//!
//! The typed tables below transcribe source inputs, not oracle outputs. There
//! is no source parser, compiler, evaluator, new admission rule or arbitrary
//! input path here. All transitions are existing Machine transitions. The
//! original fixture-budget runs and their observed minimal fuel-prefix runs
//! remain separate. No independent expectation is read or linked.
//!
//! H1 is unchanged: this is fixed trusted host diagnostic preparation only.
//! Rust/compiler/model/native execution requires separate exact-candidate
//! classification and reviews. Counts are not host memory, time, byte or stack
//! guarantees. Raw prefix diagnostics accumulate in a Vec, as in the existing
//! image producer; no memory/disk cap or new guard is claimed. K strings and
//! Rust Debug text are diagnostics, not a canonical ABI or commitment.

#[path = "../../../crates/mori-core/src/sherlock/reference_types.rs"]
mod reference_types;
#[path = "../../../crates/mori-core/src/sherlock/reference.rs"]
mod reference;
#[path = "../../../crates/mori-core/src/sherlock/reference_display.rs"]
mod reference_display;
use reference::Machine;
use reference_display::{KDisplay, environment_to_k};
use reference_types::*;
use std::io::{self, BufWriter, Write};
use std::sync::Arc;

struct Fixture {
    case_id: &'static str,
    source_path: &'static str,
    declared_source_sha256: &'static str,
    source_text: &'static str,
    original_fuel: u128,
    stack_limit: usize,
    program: Arc<Program>,
}

fn e(expression: Expr) -> ExprRef { Arc::new(expression) }
fn u(value: u128) -> ExprRef { e(Expr::Literal(Value::UInt(value))) }
fn b(value: bool) -> ExprRef { e(Expr::Literal(Value::Bool(value))) }
fn var(name: &str) -> ExprRef { e(Expr::Var(name.into())) }
fn binary(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    e(Expr::Binary(op, left, right))
}
fn call(name: &str, argument: ExprRef) -> ExprRef {
    e(Expr::Call(name.into(), vec![argument]))
}
fn parameter(name: &str, ty: Type) -> Parameter { Parameter { name: name.into(), ty } }
fn program(functions: Vec<Function>, entry: ExprRef) -> Arc<Program> {
    Arc::new(Program { functions, entry })
}

// The source's two distinct self-recursive tables remain distinct. This is
// constructor notation only: no expression is evaluated while building a table.
fn factorial3() -> Arc<Program> {
    program(vec![Function {
        name: "1".into(), parameters: vec![parameter("0", Type::UInt)],
        return_type: Type::UInt,
        body: e(Expr::If(
            binary(BinaryOp::Eq, var("0"), u(0)),
            u(1),
            binary(BinaryOp::Mul, var("0"),
                call("1", binary(BinaryOp::Sub, var("0"), u(1)))),
        )),
    }], call("1", u(3)))
}

fn sum3() -> Arc<Program> {
    program(vec![Function {
        name: "2".into(), parameters: vec![parameter("0", Type::UInt)],
        return_type: Type::UInt,
        body: e(Expr::If(
            binary(BinaryOp::Eq, var("0"), u(0)),
            u(0),
            binary(BinaryOp::Add, var("0"),
                call("2", binary(BinaryOp::Sub, var("0"), u(1)))),
        )),
    }], call("2", u(3)))
}

fn listsum123() -> Arc<Program> {
    program(vec![Function {
        name: "3".into(), parameters: vec![parameter("0", Type::List)],
        return_type: Type::UInt,
        body: e(Expr::Match(var("0"), vec![
            Arm { pattern: Pattern::Nil, body: u(0) },
            Arm { pattern: Pattern::Cons("1".into(), "2".into()),
                  body: binary(BinaryOp::Add, var("1"), call("3", var("2"))) },
        ])),
    }], call("3", e(Expr::Literal(Value::List(vec![1, 2, 3])))))
}

fn treesum45() -> Arc<Program> {
    program(vec![Function {
        name: "4".into(), parameters: vec![parameter("0", Type::Tree)],
        return_type: Type::UInt,
        body: e(Expr::Match(var("0"), vec![
            Arm { pattern: Pattern::Leaf("1".into()), body: var("1") },
            Arm { pattern: Pattern::Branch("1".into(), "2".into()),
                  body: binary(BinaryOp::Add, call("4", var("1")), call("4", var("2"))) },
        ])),
    }], call("4", e(Expr::Literal(Value::Tree(Tree::Branch(
        Arc::new(Tree::Leaf(4)), Arc::new(Tree::Leaf(5)),
    ))))))
}

fn construct_list() -> Arc<Program> {
    // Keep the two strict constructor expressions; do not fold to a literal.
    program(vec![], binary(BinaryOp::Cons, u(1),
        binary(BinaryOp::Cons, u(2), e(Expr::Literal(Value::List(vec![]))))))
}

fn bool_let_order() -> Arc<Program> {
    program(vec![], e(Expr::Let("0".into(), u(5), e(Expr::If(
        e(Expr::Unary(UnaryOp::Not, b(false))),
        binary(BinaryOp::Add, var("0"), u(2)),
        u(99),
    )))))
}

fn caller_local() -> Arc<Program> {
    program(vec![Function {
        name: "5".into(), parameters: vec![parameter("0", Type::UInt)],
        return_type: Type::UInt, body: var("0"),
    }], e(Expr::Let("0".into(), u(10),
        binary(BinaryOp::Add, call("5", u(2)), var("0")))))
}

fn mutual_even4() -> Arc<Program> {
    // Original order is function 6, then function 7. Calls remain named data.
    program(vec![
        Function {
            name: "6".into(), parameters: vec![parameter("0", Type::UInt)],
            return_type: Type::Bool,
            body: e(Expr::If(binary(BinaryOp::Eq, var("0"), u(0)), b(true),
                call("7", binary(BinaryOp::Sub, var("0"), u(1))))),
        },
        Function {
            name: "7".into(), parameters: vec![parameter("0", Type::UInt)],
            return_type: Type::Bool,
            body: e(Expr::If(binary(BinaryOp::Eq, var("0"), u(0)), b(false),
                call("6", binary(BinaryOp::Sub, var("0"), u(1))))),
        },
    ], call("6", u(4)))
}

// Membership and budgets come only from the pinned original input terms.
// Hashes are declarations; the future caller must verify the actual file bytes.
fn fixtures() -> [Fixture; 8] {
    [
        Fixture { case_id: "F01", source_path: "models/sherlock/k/fixtures/uiFactorial3.ui",
            declared_source_sha256: "0622e01b0b7ba1ee588ff648a9299840fd97e64375fd5aad3d6393ed349b6592",
            source_text: include_str!("../k/fixtures/uiFactorial3.ui"),
            original_fuel: 1000, stack_limit: 64, program: factorial3() },
        Fixture { case_id: "F02", source_path: "models/sherlock/k/fixtures/uiSum3.ui",
            declared_source_sha256: "661bd0d819885de654e5605635e10231bd587c4113547db696f4aaabd67ec351",
            source_text: include_str!("../k/fixtures/uiSum3.ui"),
            original_fuel: 1000, stack_limit: 64, program: sum3() },
        Fixture { case_id: "F03", source_path: "models/sherlock/k/fixtures/uiListsum123.ui",
            declared_source_sha256: "e08da8ed9afb166899de72c8dccf85f63f67325936e83ad805d58dedff9bf440",
            source_text: include_str!("../k/fixtures/uiListsum123.ui"),
            original_fuel: 1000, stack_limit: 64, program: listsum123() },
        Fixture { case_id: "F04", source_path: "models/sherlock/k/fixtures/uiTreesum45.ui",
            declared_source_sha256: "addb55d31d554e626f6c6f52ea7225d69b94ee48d75656495ef6e5eaf2c0b4b3",
            source_text: include_str!("../k/fixtures/uiTreesum45.ui"),
            original_fuel: 1000, stack_limit: 64, program: treesum45() },
        Fixture { case_id: "F05", source_path: "models/sherlock/k/fixtures/uiConstructList.ui",
            declared_source_sha256: "e9391d748b80e8b47109b3d8b2198ae1b6fc258127a81d854a1de9730feffc64",
            source_text: include_str!("../k/fixtures/uiConstructList.ui"),
            original_fuel: 1000, stack_limit: 64, program: construct_list() },
        Fixture { case_id: "F06", source_path: "models/sherlock/k/fixtures/uiBoolLetOrder.ui",
            declared_source_sha256: "ecb5743d4047b89fbc01f1b5ed8b73717953eacc5aa7ff1a4d776fe1845cbd7a",
            source_text: include_str!("../k/fixtures/uiBoolLetOrder.ui"),
            original_fuel: 1000, stack_limit: 64, program: bool_let_order() },
        Fixture { case_id: "F07", source_path: "models/sherlock/k/fixtures/uiCallerLocal.ui",
            declared_source_sha256: "074815b24122a529fe76f4e278798cbf0cfb4b6fb03738b946fbc6ca5f812dc3",
            source_text: include_str!("../k/fixtures/uiCallerLocal.ui"),
            original_fuel: 1000, stack_limit: 64, program: caller_local() },
        Fixture { case_id: "F08", source_path: "models/sherlock/k/fixtures/uiMutualEven4.ui",
            declared_source_sha256: "4de8f5a9b05cf6a19ec07497513555c8b333f02b26572a1f752595a6756bfe16",
            source_text: include_str!("../k/fixtures/uiMutualEven4.ui"),
            original_fuel: 1000, stack_limit: 64, program: mutual_even4() },
    ]
}

struct ObservedRun {
    initial: Snapshot,
    records: Vec<StepRecord>,
    terminal: Snapshot,
    terminal_has_next: bool,
}

fn observe(fixture: &Fixture, fuel: u128) -> io::Result<ObservedRun> {
    let events = fuel.checked_add(1).and_then(|events| usize::try_from(events).ok())
        .ok_or_else(|| io::Error::other("fixed diagnostic event count does not fit usize"))?;
    let mut machine = Machine::new(fixture.program.clone(), fuel, fixture.stack_limit);
    let initial = machine.snapshot();
    let records = machine.run_steps(events);
    let terminal = machine.snapshot();
    let terminal_has_next = machine.step().is_some();
    Ok(ObservedRun { initial, records, terminal, terminal_has_next })
}

fn json_string(out: &mut impl Write, value: &str) -> io::Result<()> {
    write!(out, "\"")?;
    for ch in value.chars() {
        match ch {
            '"' => write!(out, "\\\"")?,
            '\\' => write!(out, "\\\\")?,
            '\n' => write!(out, "\\n")?,
            '\r' => write!(out, "\\r")?,
            '\t' => write!(out, "\\t")?,
            ch if ch < ' ' => write!(out, "\\u{:04x}", ch as u32)?,
            ch => write!(out, "{ch}")?,
        }
    }
    write!(out, "\"")
}

fn string_field(out: &mut impl Write, key: &str, value: &str) -> io::Result<()> {
    json_string(out, key)?;
    write!(out, ":")?;
    json_string(out, value)
}

fn optional_count(out: &mut impl Write, value: Option<u128>) -> io::Result<()> {
    match value { Some(value) => write!(out, "{value}"), None => write!(out, "null") }
}

fn snapshot(out: &mut impl Write, state: &Snapshot) -> io::Result<()> {
    write!(out, "{{")?;
    string_field(out, "code_k", &state.program.to_k())?;
    write!(out, ",")?;
    string_field(out, "entry_k", &state.program.entry.to_k())?;
    write!(out, ",")?;
    let control = state.control.as_ref().map(KDisplay::to_k).unwrap_or_else(|| ".K".into());
    string_field(out, "control_k", &control)?;
    write!(out, ",")?;
    string_field(out, "environment_k", &environment_to_k(&state.environment))?;
    write!(out, ",\"frames_top_first_k\":[")?;
    for (index, frame) in state.frames.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        json_string(out, &frame.to_k())?;
    }
    write!(out, "],\"depth\":{},\"stack_limit\":{},\"remaining_fuel\":{},\"initial_fuel\":{},",
           state.depth(), state.stack_limit, state.remaining_fuel, state.initial_fuel)?;
    string_field(out, "output_k", &state.outcome.to_k())?;
    write!(out, ",\"charged_steps\":")?;
    optional_count(out, state.checked_charged_steps())?;
    write!(out, "}}")
}

fn common_run(out: &mut impl Write, case_id: &str, observed: &ObservedRun) -> io::Result<()> {
    write!(out, "{{")?;
    string_field(out, "case_id", case_id)?;
    write!(out, ",\"initial\":")?;
    snapshot(out, &observed.initial)?;
    write!(out, ",\"records\":[")?;
    for (index, record) in observed.records.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"rule_start_line\":{},\"charge\":{},\"before\":",
               record.rule_start_line, record.charge)?;
        snapshot(out, &record.before)?;
        write!(out, ",\"after\":")?;
        snapshot(out, &record.after)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"terminal\":")?;
    snapshot(out, &observed.terminal)?;
    write!(out, ",\"terminal_has_next\":{}}}", observed.terminal_has_next)
}

// Debug formatting observes the actual typed states, without parsing K text or
// deriving runtime controls/frames by replay. Complete program data is emitted
// once per case in projection.inputs[].program_debug; common snapshots retain
// complete code_k/entry_k at every boundary as in the existing host producer.
fn raw_state(out: &mut impl Write, state: &Snapshot) -> io::Result<()> {
    write!(out, "{{\"control_debug\":")?;
    match &state.control {
        Some(control) => json_string(out, &format!("{control:?}"))?,
        None => write!(out, "null")?,
    }
    write!(out, ",")?;
    string_field(out, "environment_debug", &format!("{:?}", state.environment))?;
    write!(out, ",\"frames_top_first_debug\":[")?;
    for (index, frame) in state.frames.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        json_string(out, &format!("{frame:?}"))?;
    }
    write!(out, "],\"depth\":{},\"stack_limit\":{},\"remaining_fuel\":{},\"initial_fuel\":{},",
           state.depth(), state.stack_limit, state.remaining_fuel, state.initial_fuel)?;
    string_field(out, "outcome_debug", &format!("{:?}", state.outcome))?;
    write!(out, ",\"charged_steps\":")?;
    optional_count(out, state.checked_charged_steps())?;
    write!(out, "}}")
}

fn raw_run(out: &mut impl Write, case_id: &str, observed: &ObservedRun) -> io::Result<()> {
    write!(out, "{{")?;
    string_field(out, "case_id", case_id)?;
    write!(out, ",\"initial_fuel\":{},\"initial\":", observed.initial.initial_fuel)?;
    raw_state(out, &observed.initial)?;
    write!(out, ",\"records\":[")?;
    for (index, record) in observed.records.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"rule_start_line\":{},\"charge\":{},\"before\":",
               record.rule_start_line, record.charge)?;
        raw_state(out, &record.before)?;
        write!(out, ",\"after\":")?;
        raw_state(out, &record.after)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"terminal\":")?;
    raw_state(out, &observed.terminal)?;
    write!(out, ",\"terminal_has_next\":{}}}", observed.terminal_has_next)
}

fn main() -> io::Result<()> {
    let fixtures = fixtures();
    // First observe each original source budget exactly once. These actual
    // results, not any expected work counts, select the later prefix extent.
    let original: Vec<_> = fixtures.iter().map(|fixture| observe(fixture, fixture.original_fuel))
        .collect::<io::Result<_>>()?;
    let completion: Vec<_> = original.iter().map(|run| {
        if matches!(&run.terminal.outcome, Outcome::Done(_)) {
            run.terminal.checked_charged_steps()
        } else { None }
    }).collect();
    let prefix_last: Vec<_> = completion.iter().map(|work| work.and_then(|work| work.checked_add(1))).collect();
    let mut prefix_counts = vec![0u128; fixtures.len()];
    let mut raw_prefix = Vec::new();
    let mut out = BufWriter::new(io::stdout().lock());
    write!(out, "{{")?;
    string_field(&mut out, "format", "sherlock-original-fixture-host-observations/1")?;
    write!(out, ",\"observations\":{{")?;
    string_field(&mut out, "producer", "sherlock-original-fixture-host-reference")?;
    write!(out, ",")?;
    string_field(&mut out, "profile", PROFILE)?;
    write!(out, ",")?;
    string_field(&mut out, "source_sha256", K_SOURCE_SHA256)?;
    write!(out, ",")?;
    string_field(&mut out, "boundary", "post-startup")?;
    write!(out, ",\"programs\":[")?;
    for (index, fixture) in fixtures.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(&mut out, "case_id", fixture.case_id)?;
        write!(out, ",")?;
        string_field(&mut out, "code_k", &fixture.program.to_k())?;
        write!(out, ",")?;
        string_field(&mut out, "entry_k", &fixture.program.entry.to_k())?;
        write!(out, "}}")?;
    }
    write!(out, "],\"runs\":[")?;
    let mut first = true;
    for (index, fixture) in fixtures.iter().enumerate() {
        if let Some(last) = prefix_last[index] {
            for fuel in 0..=last {
                let next;
                let observed = if fuel == fixture.original_fuel {
                    &original[index]
                } else {
                    next = observe(fixture, fuel)?;
                    &next
                };
                if !first { write!(out, ",")?; write!(raw_prefix, ",")?; }
                first = false;
                common_run(&mut out, fixture.case_id, observed)?;
                raw_run(&mut raw_prefix, fixture.case_id, observed)?;
                prefix_counts[index] += 1;
            }
        }
    }
    write!(out, "]}},\"original_fixture_runs\":[")?;
    for (index, (fixture, observed)) in fixtures.iter().zip(&original).enumerate() {
        if index != 0 { write!(out, ",")?; }
        common_run(&mut out, fixture.case_id, observed)?;
    }
    write!(out, "],\"projection\":{{")?;
    string_field(&mut out, "format", "sherlock-original-fixture-host-diagnostics/1")?;
    write!(out, ",\"inputs\":[")?;
    for (index, fixture) in fixtures.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(&mut out, "case_id", fixture.case_id)?;
        write!(out, ",")?;
        string_field(&mut out, "fixture_path", fixture.source_path)?;
        write!(out, ",")?;
        string_field(&mut out, "declared_fixture_sha256", fixture.declared_source_sha256)?;
        write!(out, ",")?;
        string_field(&mut out, "fixture_source", fixture.source_text)?;
        write!(out, ",\"original_fuel\":{},\"stack_limit\":{},", fixture.original_fuel, fixture.stack_limit)?;
        string_field(&mut out, "program_debug", &format!("{:?}", fixture.program))?;
        write!(out, "}}")?;
    }
    write!(out, "],\"prefix_selection\":[")?;
    for (index, fixture) in fixtures.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(&mut out, "case_id", fixture.case_id)?;
        write!(out, ",")?;
        string_field(&mut out, "observed_original_outcome_k", &original[index].terminal.outcome.to_k())?;
        write!(out, ",\"original_run_record_count\":{},\"observed_original_charged_steps\":", original[index].records.len())?;
        optional_count(&mut out, original[index].terminal.checked_charged_steps())?;
        write!(out, ",\"observed_completion_charged_steps\":")?;
        optional_count(&mut out, completion[index])?;
        write!(out, ",\"prefix_first_fuel\":0,\"prefix_last_fuel\":")?;
        optional_count(&mut out, prefix_last[index])?;
        write!(out, ",\"prefix_captures_emitted\":{},", prefix_counts[index])?;
        string_field(&mut out, "selection_status", if prefix_last[index].is_some() {
            "selected-from-observed-original-done-boundary"
        } else { "not-selected-no-valid-original-done-boundary" })?;
        write!(out, "}}")?;
    }
    write!(out, "],\"original_runs\":[")?;
    for (index, (fixture, observed)) in fixtures.iter().zip(&original).enumerate() {
        if index != 0 { write!(out, ",")?; }
        raw_run(&mut out, fixture.case_id, observed)?;
    }
    write!(out, "],\"runs\":[")?;
    out.write_all(&raw_prefix)?;
    writeln!(out, "]}}}}")?;
    out.flush()
}
