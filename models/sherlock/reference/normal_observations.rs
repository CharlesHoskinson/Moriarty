//! Standalone fixed-input observation producer for the existing host Machine.
//! No Cargo, SDK, model engine, source parser or public input endpoint is used.
//! Diagnostic K strings below are not a canonical state codec or public ABI.

#[path = "../../../crates/mori-core/src/sherlock/reference_types.rs"]
mod reference_types;
#[path = "../../../crates/mori-core/src/sherlock/reference.rs"]
mod reference;
#[path = "../../../crates/mori-core/src/sherlock/reference_display.rs"]
mod reference_display;
#[path = "../../../crates/mori-core/src/sherlock/compare.rs"]
mod compare;
#[path = "../../../crates/mori-core/src/sherlock/normal_oracle_inputs.rs"]
mod normal_oracle_inputs;

#[cfg(test)]
#[path = "../../../crates/mori-core/src/sherlock/reference_bounded_tests.rs"]
mod reference_bounded_tests;

use compare::{TraceIdentity, TraceView};
use normal_oracle_inputs::normal_program;
use reference::Machine;
use reference_display::{KDisplay, environment_to_k};
use reference_types::*;
use std::io::{self, BufWriter, Write};

// Immutable input selection from the committed independent program documents.
// These are budgets/stack limits, not calculated expected work or output values.
const INPUTS: [(&str, u128, usize); 8] = [
    ("N01", 1, 8), ("N02", 7, 8), ("N03", 9, 8), ("N04", 17, 8),
    ("N05", 27, 8), ("N06", 19, 8), ("N07", 38, 8), ("N08", 82, 8),
];

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
    match state.checked_charged_steps() {
        Some(work) => write!(out, "{work}")?,
        None => write!(out, "null")?, // Preserve invalid counters as a diagnosable mismatch.
    }
    write!(out, "}}")
}

fn observation(out: &mut impl Write, case_id: &str, trace: TraceView<'_>,
               terminal: &Snapshot, terminal_has_next: bool) -> io::Result<()> {
    // TraceView borrows the actual Machine-produced typed observations. Identity
    // declarations do not authenticate the separately pinned execution artifact.
    debug_assert_eq!(trace.identity.profile, PROFILE);
    debug_assert_eq!(trace.identity.source_sha256, K_SOURCE_SHA256);
    write!(out, "{{")?;
    string_field(out, "case_id", case_id)?;
    write!(out, ",\"initial\":")?;
    snapshot(out, trace.initial)?;
    write!(out, ",\"records\":[")?;
    for (index, record) in trace.records.iter().enumerate() {
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
    // Fixed trusted typed ASTs only. The underlying A machine remains unmetered
    // for arbitrary inputs; this producer supplies no general input interface.
    let mut out = BufWriter::new(io::stdout().lock());
    write!(out, "{{")?;
    string_field(&mut out, "producer", "sherlock-host-reference")?;
    write!(out, ",")?;
    string_field(&mut out, "profile", PROFILE)?;
    write!(out, ",")?;
    string_field(&mut out, "source_sha256", K_SOURCE_SHA256)?;
    write!(out, ",")?;
    string_field(&mut out, "boundary", "post-startup")?;
    write!(out, ",\"programs\":[")?;
    for (index, (case_id, _, _)) in INPUTS.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        let program = normal_program(case_id);
        write!(out, "{{")?;
        string_field(&mut out, "case_id", case_id)?;
        write!(out, ",")?;
        string_field(&mut out, "code_k", &program.to_k())?;
        write!(out, ",")?;
        string_field(&mut out, "entry_k", &program.entry.to_k())?;
        write!(out, "}}")?;
    }
    write!(out, "],\"runs\":[")?;
    let mut first = true;
    for (case_id, completion_budget, stack_limit) in INPUTS {
        let program = normal_program(case_id);
        for fuel in 0..=completion_budget + 1 {
            let mut machine = Machine::new(program.clone(), fuel, stack_limit);
            let initial = machine.snapshot();
            // Exercise the actual bounded execution API: at most fuel charged
            // events plus free publication. No expected states guide execution.
            let event_limit = usize::try_from(fuel + 1).expect("fixed normal budget fits usize");
            let records = machine.run_steps(event_limit);
            let terminal = machine.snapshot();
            let terminal_has_next = machine.step().is_some();
            if !first { write!(out, ",")?; }
            first = false;
            let trace = TraceView {
                identity: TraceIdentity { profile: PROFILE, source_sha256: K_SOURCE_SHA256 },
                initial: &initial,
                records: &records,
            };
            observation(&mut out, case_id, trace, &terminal, terminal_has_next)?;
        }
    }
    writeln!(out, "]}}")?;
    out.flush()
}
