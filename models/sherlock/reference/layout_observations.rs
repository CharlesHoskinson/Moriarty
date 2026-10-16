//! Fixed N01–N08 complete-prefix diagnostic capture through the real L4 codec.
//! Register as a cfg(test) child of the private layout_a module. No public input
//! endpoint, byte parser, native operation or physical-codec theorem is provided.
//! Compilation/execution UNRUN until exact-closure Opus/Muse preflights.
use crate::sherlock::{Machine, Snapshot, PROFILE, K_SOURCE_SHA256};
use crate::sherlock::compare::{first_difference, TraceDifference, TraceIdentity, TraceView};
use crate::sherlock::normal_oracle_inputs::normal_program;
use crate::sherlock::reference_display::{KDisplay, environment_to_k};
use super::limits::PrepareFailure;
use super::observation::{materialize_trace_a, ObservationProgramA, ObservedRecordA, RecordEnvelopeA};
use super::prepare::prepare_a;
use super::rows::Capacities;
use super::state::{StateBlock, StateCapacities, StateFailure};
use super::state_encode::encode_observed_a;
use std::io::{self, Write};

#[derive(Debug)]
pub(crate) enum CaptureFailure {
    Io(io::Error),
    Preparation(PrepareFailure),
    State(StateFailure),
    MaterializationDifference(TraceDifference),
}
impl From<io::Error> for CaptureFailure {
    fn from(error: io::Error) -> Self { Self::Io(error) }
}
impl From<PrepareFailure> for CaptureFailure {
    fn from(error: PrepareFailure) -> Self { Self::Preparation(error) }
}
impl From<StateFailure> for CaptureFailure {
    fn from(error: StateFailure) -> Self { Self::State(error) }
}

// Exact fixed input fuel/stack limits from the existing independent normal
// corpus. Expected states, rule choices and results never guide execution.
const INPUTS: [(&str, u128, usize); 8] = [
    ("N01", 1, 8), ("N02", 7, 8), ("N03", 9, 8), ("N04", 17, 8),
    ("N05", 27, 8), ("N06", 19, 8), ("N07", 38, 8), ("N08", 82, 8),
];

// Diagnostic JSON formatting transcribed unchanged from normal_observations.rs.
// It is a full typed-state projection, not canonical physical L4 byte encoding.
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

/// Write all 216 normal fuel-prefix observations after real codec materialization.
/// The external consumer compares this artifact with frozen independent JSON.
/// The typed comparison below separately checks internal preservation only.
pub(crate) fn write_n01_n08(out: &mut impl Write) -> Result<(), CaptureFailure> {
    write!(out, "{{")?;
    string_field(out, "producer", "sherlock-layout-reference")?;
    write!(out, ",")?;
    string_field(out, "profile", PROFILE)?;
    write!(out, ",")?;
    string_field(out, "source_sha256", K_SOURCE_SHA256)?;
    write!(out, ",")?;
    string_field(out, "boundary", "post-startup")?;
    write!(out, ",\"programs\":[")?;
    for (index, (case_id, _, _)) in INPUTS.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        let program = normal_program(case_id);
        write!(out, "{{")?;
        string_field(out, "case_id", case_id)?;
        write!(out, ",")?;
        string_field(out, "code_k", &program.to_k())?;
        write!(out, ",")?;
        string_field(out, "entry_k", &program.entry.to_k())?;
        write!(out, "}}")?;
    }
    write!(out, "],\"runs\":[")?;
    let mut first = true;
    for (case_id, completion_budget, stack_limit) in INPUTS {
        let program = normal_program(case_id);
        // Unchanged explicit dimensions from state_encode_normal_tests.rs.
        // These fixed examples are not defaults or canonical-envelope admission.
        let prepared = prepare_a(program.clone(), Capacities { tables: [64; 15], data: 64 })?;
        let capacities = StateCapacities { data: 64, data_fields: 64, env_cells: 32,
            value_slots: 32, frame_slots: 16, control_rows: 16, stack_limit: 32 };
        let shared = ObservationProgramA { profile: PROFILE,
            full_program: &program, program: &prepared.block };
        let identity = TraceIdentity { profile: PROFILE, source_sha256: K_SOURCE_SHA256 };
        for fuel in 0..=completion_budget + 1 {
            let mut machine = Machine::new(program.clone(), fuel, stack_limit);
            let initial = machine.snapshot();
            let event_limit = usize::try_from(fuel + 1).expect("fixed normal budget fits usize");
            let records = machine.run_steps(event_limit);
            let terminal_has_next = machine.step().is_some();
            let initial_image = encode_observed_a(&prepared, &capacities, &initial)?;
            let images: Vec<(StateBlock, StateBlock)> = records.iter().map(|record| {
                Ok((encode_observed_a(&prepared, &capacities, &record.before)?,
                    encode_observed_a(&prepared, &capacities, &record.after)?))
            }).collect::<Result<_, StateFailure>>()?;
            let supplied: Vec<ObservedRecordA<'_>> = records.iter().zip(&images)
                .map(|(record, (before_image, after_image))| ObservedRecordA {
                    envelope: RecordEnvelopeA { rule_start_line: record.rule_start_line,
                        charge: record.charge }, before_image, after_image,
                }).collect();
            let materialized = materialize_trace_a(&prepared, &capacities, &shared,
                identity, &initial_image, &supplied)?;
            if let Some(difference) = first_difference(materialized.view(), TraceView {
                identity, initial: &initial, records: &records,
            }) {
                return Err(CaptureFailure::MaterializationDifference(difference));
            }
            let terminal = materialized.records.last().map(|record| &record.after)
                .unwrap_or(&materialized.initial);
            if !first { write!(out, ",")?; }
            first = false;
            observation(out, case_id, materialized.view(), terminal, terminal_has_next)?;
        }
    }
    writeln!(out, "]}}")?;
    Ok(())
}
