//! SOURCE_ONLY / UNRUN: fixed trusted Core source-to-host diagnostics.
//!
//! Link to the actual existing mori_core library; never replace its parser,
//! validation or bounded scalar consumer with a standalone facsimile. This producer's
//! own code uses Std only, but the real Core library has external dependencies.
//! Compilation/execution requires separate exact-candidate review and the
//! existing caller-owned execution envelope. No arbitrary source, CLI argument,
//! network endpoint, effects interpreter or public compiler profile is added.
//! The private Sherlock draft frontend remains a separate unregistered profile.
//! Diagnostic K strings are not a canonical state codec or target evidence.

use mori_core::sherlock::reference_display::{KDisplay, environment_to_k};
use mori_core::sherlock::{Snapshot, StepRecord, Value, K_SOURCE_SHA256, PROFILE};
use mori_core::sherlock_bounded_source::{BoundedSourceObservation, HostObservationBounds,
                                        observe_scalar_action};
use mori_core::sherlock_source::ScalarActionReport;
use std::io::{self, BufWriter, Write};

struct Input {
    case_id: &'static str,
    source_path: &'static str,
    source: &'static str,
    arguments: &'static [u128],
    max_fuel: u128,
}

// Input selection and diagnostic collection limits, never expected outcomes.
// The existing source specimens remain the actual parser inputs.
const INPUTS: [Input; 2] = [
    Input {
        case_id: "S01",
        source_path: "crates/mori-core/src/sherlock/source_adapter_fixtures/weighted-total.mori",
        source: include_str!("../../../crates/mori-core/src/sherlock/source_adapter_fixtures/weighted-total.mori"),
        arguments: &[4, 9],
        max_fuel: 35,
    },
    Input {
        case_id: "S02",
        source_path: "crates/mori-core/src/sherlock/source_adapter_fixtures/threshold-check.mori",
        source: include_str!("../../../crates/mori-core/src/sherlock/source_adapter_fixtures/threshold-check.mori"),
        arguments: &[7, 10],
        max_fuel: 33,
    },
];

// Task-local operational choices matching the bounded consumer's normal
// specimens. These are neither a default policy nor another language profile.
// Their H1 qualification remains subject to separate exact-candidate reviews.
const BOUNDS: HostObservationBounds = HostObservationBounds {
    max_source_actions: 8,
    max_source_expressions: 64,
    max_source_statements: 16,
    max_scalar_arguments: 4,
    max_projection_occurrences: 64,
    max_ast_nesting: 16,
    stack_limit: 8,
    max_step_records: 64,
    max_retained_slots: 1 << 20,
};

struct Prepared {
    source: mori_core::Program,
    arguments: Vec<Value>,
    zero_fuel: BoundedSourceObservation,
}

fn prepare(input: &Input) -> io::Result<Prepared> {
    let source = mori_core::parse(input.source)
        .map_err(|error| io::Error::other(format!("{}: parse: {error:?}", input.case_id)))?;
    let arguments: Vec<_> = input.arguments.iter().copied().map(Value::UInt).collect();
    // The first actual bounded result supplies the program/preparation record
    // and is reused as the fuel-zero run. No hand-built Sherlock AST is used.
    let zero_fuel = observe(input.case_id, &source, &arguments, 0)?;
    Ok(Prepared { source, arguments, zero_fuel })
}

fn observe(case_id: &str, source: &mori_core::Program, arguments: &[Value], fuel: u128)
    -> io::Result<BoundedSourceObservation>
{
    observe_scalar_action(source, 0, arguments, fuel, BOUNDS)
        .map_err(|error| io::Error::other(format!("{case_id}: bounded observation: {error:?}")))
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

fn optional_number<T: std::fmt::Display>(out: &mut impl Write, value: Option<T>) -> io::Result<()> {
    match value { Some(value) => write!(out, "{value}"), None => write!(out, "null") }
}

fn preparation(out: &mut impl Write, report: &ScalarActionReport) -> io::Result<()> {
    write!(out, "{{\"action_index\":{},", report.action_index)?;
    string_field(out, "action", &report.action)?;
    write!(out, ",\"inputs\":[")?;
    for (index, input) in report.inputs.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"index\":{},", input.index)?;
        string_field(out, "name", &input.name)?;
        write!(out, ",")?;
        string_field(out, "type_debug", &format!("{:?}", input.ty))?;
        write!(out, "}}")?;
    }
    write!(out, "],\"lets\":[")?;
    for (index, site) in report.lets.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"statement_index\":{},\"expression\":{},", site.statement_index, site.expression)?;
        string_field(out, "name", &site.name)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"emit\":{{\"statement_index\":{},\"expression\":{},",
           report.emit.statement_index, report.emit.expression)?;
    string_field(out, "name", &report.emit.name)?;
    write!(out, ",")?;
    string_field(out, "role", &report.emit.role)?;
    write!(out, "}},\"occurrences\":[")?;
    for (index, site) in report.occurrences.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"action_index\":{},\"statement_index\":{},\"expression\":",
               site.action_index, site.statement_index)?;
        optional_number(out, site.expression)?;
        write!(out, ",\"body_path\":[")?;
        for (index, position) in site.body_path.iter().enumerate() {
            if index != 0 { write!(out, ",")?; }
            write!(out, "{position}")?;
        }
        write!(out, "],")?;
        string_field(out, "node_kind", &site.node_kind)?;
        write!(out, ",\"source_expression_debug\":")?;
        match &site.source_expression {
            Some(expression) => json_string(out, &format!("{expression:?}"))?,
            None => write!(out, "null")?,
        }
        write!(out, "}}")?;
    }
    write!(out, "],")?;
    string_field(out, "return_type_debug", &format!("{:?}", report.return_type))?;
    write!(out, ",")?;
    string_field(out, "generated_function_name", &report.generated_function_name)?;
    write!(out, ",")?;
    string_field(out, "symbol_domain", &report.symbol_domain)?;
    write!(out, "}}")
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
    optional_number(out, state.checked_charged_steps())?;
    write!(out, "}}")
}

fn observation(out: &mut impl Write, case_id: &str, initial: &Snapshot,
               records: &[StepRecord], terminal: &Snapshot, terminal_has_next: bool) -> io::Result<()> {
    write!(out, "{{")?;
    string_field(out, "case_id", case_id)?;
    write!(out, ",\"initial\":")?;
    snapshot(out, initial)?;
    write!(out, ",\"records\":[")?;
    for (index, record) in records.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"rule_start_line\":{},\"charge\":{},\"before\":",
               record.rule_start_line, record.charge)?;
        snapshot(out, &record.before)?;
        write!(out, ",\"after\":")?;
        snapshot(out, &record.after)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"terminal\":")?;
    snapshot(out, terminal)?;
    write!(out, ",\"terminal_has_next\":{terminal_has_next}}}")
}

fn main() -> io::Result<()> {
    // Parse both real sources and retain their actual bounded fuel-zero results
    // before emitting any observation document. There is no arbitrary input.
    let prepared = INPUTS.iter().map(prepare).collect::<io::Result<Vec<_>>>()?;
    let mut out = BufWriter::new(io::stdout().lock());
    write!(out, "{{")?;
    string_field(&mut out, "producer", "sherlock-core-source-reference")?;
    write!(out, ",")?;
    string_field(&mut out, "profile", PROFILE)?;
    write!(out, ",")?;
    string_field(&mut out, "source_sha256", K_SOURCE_SHA256)?;
    write!(out, ",")?;
    string_field(&mut out, "boundary", "post-startup")?;
    write!(out, ",\"programs\":[")?;
    for (index, (input, actual)) in INPUTS.iter().zip(&prepared).enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(&mut out, "case_id", input.case_id)?;
        write!(out, ",")?;
        string_field(&mut out, "source_path", input.source_path)?;
        write!(out, ",")?;
        string_field(&mut out, "source_text", input.source)?;
        write!(out, ",")?;
        string_field(&mut out, "source_language", &actual.source.language)?;
        write!(out, ",")?;
        string_field(&mut out, "source_semantics", &actual.source.semantics)?;
        write!(out, ",")?;
        string_field(&mut out, "code_k", &actual.zero_fuel.program.to_k())?;
        write!(out, ",")?;
        string_field(&mut out, "entry_k", &actual.zero_fuel.program.entry.to_k())?;
        write!(out, ",\"preparation\":")?;
        preparation(&mut out, &actual.zero_fuel.report)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"runs\":[")?;
    let mut first = true;
    for (input, actual) in INPUTS.iter().zip(&prepared) {
        for fuel in 0..=input.max_fuel {
            let next_observation;
            let observed = if fuel == 0 {
                &actual.zero_fuel
            } else {
                next_observation = observe(input.case_id, &actual.source, &actual.arguments, fuel)?;
                &next_observation
            };
            if !first { write!(out, ",")?; }
            first = false;
            // Retain the unchanged common diagnostic shape. Extent and
            // qualification are typed API metadata, not part of this codec.
            observation(&mut out, input.case_id, &observed.initial, &observed.records,
                        &observed.terminal, observed.terminal_has_next)?;
        }
    }
    writeln!(out, "]}}")?;
    out.flush()
}
