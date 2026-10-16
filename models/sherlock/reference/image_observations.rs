//! Fixed-input observations from the EXISTING private flat-ID A interpreter.
//! This standalone stdlib-only host diagnostic adds no evaluator or input API.
//! ProgramImageDraft tables and ImageMachine::step drive every execution below.
//! Materialized K states are a comparison projection, not a canonical image,
//! native encoding, Candidate B schema, admission policy or source commitment.

#[path = "../../../crates/mori-core/src/sherlock/reference_types.rs"]
mod reference_types;
#[path = "../../../crates/mori-core/src/sherlock/reference_display.rs"]
mod reference_display;
#[path = "../../../crates/mori-core/src/sherlock/program_image.rs"]
mod program_image;
#[path = "../../../crates/mori-core/src/sherlock/image_reference.rs"]
mod image_reference;
#[path = "../../../crates/mori-core/src/sherlock/normal_oracle_inputs.rs"]
mod normal_oracle_inputs;

use image_reference::ImageMachine;
use normal_oracle_inputs::normal_program;
use program_image::{prepare_program_image, ProgramImageDraft, TableSlice};
use reference_display::{environment_to_k, KDisplay};
use reference_types::*;
use std::io::{self, BufWriter, Write};
use std::sync::Arc;

// Fixed source selection and execution bounds from the independent normal
// input documents. No expected output, state, rule or work guides execution.
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

fn numbers(out: &mut impl Write, values: impl IntoIterator<Item = usize>) -> io::Result<()> {
    write!(out, "[")?;
    for (index, value) in values.into_iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{value}")?;
    }
    write!(out, "]")
}

fn table_slice(out: &mut impl Write, slice: TableSlice) -> io::Result<()> {
    write!(out, "{{\"start\":{},\"len\":{}}}", slice.start, slice.len)
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
        None => write!(out, "null")?,
    }
    write!(out, "}}")
}

/// Preserve runtime IDs/slices before structural Snapshot materialization can
/// erase occurrence identity. These strings are the actual private controls and
/// frames, not IDs inferred by comparing structurally equal source expressions.
struct RawState {
    control_origin: usize,
    frame_origins: Vec<usize>,
    control_debug: Option<String>,
    frames_top_first_debug: Vec<String>,
}

impl RawState {
    fn observe(machine: &ImageMachine) -> Self {
        let (control_debug, frames_top_first_debug) = machine.representation_diagnostics();
        Self {
            control_origin: machine.control_origin().0,
            frame_origins: machine.frame_origins().iter().map(|id| id.0).collect(),
            control_debug,
            frames_top_first_debug,
        }
    }

    fn write(&self, out: &mut impl Write) -> io::Result<()> {
        write!(out, "{{\"control_origin\":{},\"frame_origins\":", self.control_origin)?;
        numbers(out, self.frame_origins.iter().copied())?;
        write!(out, ",\"control_debug\":")?;
        match &self.control_debug {
            Some(control) => json_string(out, control)?,
            None => write!(out, "null")?,
        }
        write!(out, ",\"frames_top_first_debug\":[")?;
        for (index, frame) in self.frames_top_first_debug.iter().enumerate() {
            if index != 0 { write!(out, ",")?; }
            json_string(out, frame)?;
        }
        write!(out, "]}}")
    }
}

struct ObservedStep {
    record: StepRecord,
    raw_before: RawState,
    raw_after: RawState,
}

fn observation(out: &mut impl Write, raw_out: &mut impl Write, case_id: &str,
               machine: &mut ImageMachine, fuel: u128) -> io::Result<()> {
    let initial = machine.snapshot();
    let raw_initial = RawState::observe(machine);
    let mut records = Vec::new();
    // Frozen A scheduling permits at most fuel charged transitions plus one
    // free publication. The bound neither assumes a successful terminal value
    // nor hides an extra event: terminal_has_next observes that event separately.
    let event_limit = usize::try_from(fuel + 1).expect("fixed normal fuel fits usize");
    for _ in 0..event_limit {
        let raw_before = RawState::observe(machine);
        match machine.step() {
            Some(record) => records.push(ObservedStep {
                record, raw_before, raw_after: RawState::observe(machine),
            }),
            None => break,
        }
    }
    let terminal = machine.snapshot();
    let raw_terminal = RawState::observe(machine);
    let terminal_has_next = machine.step().is_some();
    write!(out, "{{")?;
    string_field(out, "case_id", case_id)?;
    write!(out, ",\"initial\":")?;
    snapshot(out, &initial)?;
    write!(out, ",\"records\":[")?;
    for (index, observed) in records.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        let record = &observed.record;
        write!(out, "{{\"rule_start_line\":{},\"charge\":{},\"before\":",
               record.rule_start_line, record.charge)?;
        snapshot(out, &record.before)?;
        write!(out, ",\"after\":")?;
        snapshot(out, &record.after)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"terminal\":")?;
    snapshot(out, &terminal)?;
    write!(out, ",\"terminal_has_next\":{terminal_has_next}}}")?;

    write!(raw_out, "{{")?;
    string_field(raw_out, "case_id", case_id)?;
    write!(raw_out, ",\"initial_fuel\":{fuel},\"initial\":")?;
    raw_initial.write(raw_out)?;
    write!(raw_out, ",\"records\":[")?;
    for (index, observed) in records.iter().enumerate() {
        if index != 0 { write!(raw_out, ",")?; }
        write!(raw_out, "{{\"rule_start_line\":{},\"charge\":{},\"before\":",
               observed.record.rule_start_line, observed.record.charge)?;
        observed.raw_before.write(raw_out)?;
        write!(raw_out, ",\"after\":")?;
        observed.raw_after.write(raw_out)?;
        write!(raw_out, "}}")?;
    }
    write!(raw_out, "],\"terminal\":")?;
    raw_terminal.write(raw_out)?;
    write!(raw_out, ",\"terminal_has_next\":{terminal_has_next}}}")
}

/// All actual builder table fields, in original row order. Debug payloads are
/// explicitly diagnostic Rust representations; ordered ID cells stay numeric.
fn program_image(out: &mut impl Write, image: &ProgramImageDraft) -> io::Result<()> {
    write!(out, "{{")?;
    string_field(out, "profile_id", image.profile_id)?;
    write!(out, ",\"entry\":{},\"expressions\":[", image.entry.0)?;
    for (index, row) in image.expressions.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(out, "payload_debug", &format!("{:?}", row.payload))?;
        write!(out, ",\"children\":")?;
        table_slice(out, row.children)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"ordered_children\":")?;
    numbers(out, image.ordered_children.iter().map(|id| id.0))?;
    write!(out, ",\"functions\":[")?;
    for (index, row) in image.functions.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{\"source_table_index\":{},", row.source_table_index)?;
        string_field(out, "name", &row.name)?;
        write!(out, ",\"parameters\":")?;
        table_slice(out, row.parameters)?;
        write!(out, ",")?;
        string_field(out, "return_type_k", &row.return_type.to_k())?;
        write!(out, ",\"body\":{}}}", row.body.0)?;
    }
    write!(out, "],\"ordered_parameters\":[")?;
    for (index, parameter) in image.ordered_parameters.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(out, "name", &parameter.name)?;
        write!(out, ",")?;
        string_field(out, "type_k", &parameter.ty.to_k())?;
        write!(out, "}}")?;
    }
    write!(out, "],\"match_arms\":[")?;
    for (index, row) in image.match_arms.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(out, "pattern_k", &row.pattern.to_k())?;
        write!(out, ",\"body\":{}}}", row.body.0)?;
    }
    write!(out, "],\"constants\":[")?;
    for (index, row) in image.constants.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(out, "payload_debug", &format!("{:?}", row.payload))?;
        write!(out, ",\"fields\":")?;
        table_slice(out, row.fields)?;
        write!(out, ",\"source\":{{\"expression\":{},\"field_path\":", row.source.expression.0)?;
        numbers(out, row.source.field_path.iter().copied())?;
        write!(out, "}}}}")?;
    }
    write!(out, "],\"constant_fields\":")?;
    numbers(out, image.constant_fields.iter().map(|id| id.0))?;
    write!(out, ",\"source_map\":[")?;
    for (index, locator) in image.source_map.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(out, "owner_debug", &format!("{:?}", locator.owner))?;
        write!(out, ",\"ast_path\":")?;
        numbers(out, locator.ast_path.iter().copied())?;
        write!(out, "}}")?;
    }
    write!(out, "]}}")
}

fn main() -> io::Result<()> {
    // Prepare each shared typed input once. Every fuel prefix executes the same
    // immutable tables; calls remain named references, never source unrolling.
    let inputs: Vec<_> = INPUTS.into_iter().map(|(case_id, budget, stack_limit)| {
        let program = normal_program(case_id);
        let image = Arc::new(prepare_program_image(&program));
        (case_id, budget, stack_limit, program, image)
    }).collect();
    let mut out = BufWriter::new(io::stdout().lock());
    write!(out, "{{")?;
    string_field(&mut out, "format", "sherlock-image-observations/1")?;
    write!(out, ",\"observations\":{{")?;
    string_field(&mut out, "producer", "sherlock-image-reference")?;
    write!(out, ",")?;
    string_field(&mut out, "profile", PROFILE)?;
    write!(out, ",")?;
    string_field(&mut out, "source_sha256", K_SOURCE_SHA256)?;
    write!(out, ",")?;
    string_field(&mut out, "boundary", "post-startup")?;
    write!(out, ",\"programs\":[")?;
    for (index, (case_id, _, _, program, _)) in inputs.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(&mut out, "case_id", case_id)?;
        write!(out, ",")?;
        string_field(&mut out, "code_k", &program.to_k())?;
        write!(out, ",")?;
        string_field(&mut out, "entry_k", &program.entry.to_k())?;
        write!(out, "}}")?;
    }
    write!(out, "],\"runs\":[")?;
    let mut raw_runs = Vec::new();
    let mut first = true;
    for (case_id, budget, stack_limit, _, image) in &inputs {
        for fuel in 0..=*budget + 1 {
            let mut machine = ImageMachine::new(image.clone(), fuel, *stack_limit);
            // This is the exact Arc used for row fetches, not an encoded Snapshot.
            debug_assert!(Arc::ptr_eq(machine.program_image(), image));
            if !first { write!(out, ",")?; write!(&mut raw_runs, ",")?; }
            first = false;
            observation(&mut out, &mut raw_runs, case_id, &mut machine, fuel)?;
        }
    }
    write!(out, "]}},\"projection\":{{")?;
    string_field(&mut out, "format", "sherlock-host-flat-id-diagnostics/1")?;
    write!(out, ",")?;
    string_field(&mut out, "materialization", "Common K snapshots reconstruct source expressions and functions from the executed image; runtime values remain existing typed host values.")?;
    write!(out, ",")?;
    string_field(&mut out, "retained_representation", "Original ProgramImageDraft row order, IDs, slices and locators; actual private runtime control/frame Debug and causal origin IDs at every boundary.")?;
    write!(out, ",")?;
    string_field(&mut out, "identity_scope", "Host image-local positions only; Debug and K display are noncanonical diagnostics, not native tags, a Candidate B schema, source authentication or commitments.")?;
    write!(out, ",\"program_images\":[")?;
    for (index, (case_id, _, _, _, image)) in inputs.iter().enumerate() {
        if index != 0 { write!(out, ",")?; }
        write!(out, "{{")?;
        string_field(&mut out, "case_id", case_id)?;
        write!(out, ",\"image\":")?;
        program_image(&mut out, image)?;
        write!(out, "}}")?;
    }
    write!(out, "],\"runs\":[")?;
    out.write_all(&raw_runs)?;
    writeln!(out, "]}}}}")?;
    out.flush()
}
