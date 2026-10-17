//! External, non-test consumer of the real mori_core production API.
//! Fixed trusted diagnostic input only. This does not construct a native step witness.
use std::sync::Arc;
use mori_core::sherlock::{Expr, Machine, Program, Snapshot, Value, PROFILE};
use mori_core::sherlock::layout_a::{
    prepare_a, encode_observed_a, decode_observed_a, observed_state_bytes_a,
    AImage, Capacities, DecodeMode, PreparedA, StateCapacities,
};

fn roundtrip(prepared: &PreparedA, capacities: &StateCapacities,
    snapshot: &Snapshot) -> Result<usize, String> {
    let state = encode_observed_a(prepared, capacities, snapshot)
        .map_err(|e| format!("encode: {e:?}"))?;
    let image = AImage { profile: PROFILE, full_program: snapshot.program.as_ref(),
        program: &prepared.block, state: &state };
    let decoded = decode_observed_a(prepared, capacities, &image, DecodeMode::Canonical)
        .map_err(|e| format!("decode: {e:?}"))?;
    if &decoded != snapshot { return Err("canonical snapshot roundtrip differed".into()); }
    Ok(observed_state_bytes_a(prepared, capacities, snapshot)
        .map_err(|e| format!("state bytes: {e:?}"))?.len())
}

fn main() -> Result<(), String> {
    // Explicit dimensions for this diagnostic input; no default/admission policy.
    let program = Arc::new(Program { functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(11))) });
    let prepared = prepare_a(program.clone(), Capacities {
        tables: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0], data: 1,
    }).map_err(|e| format!("prepare: {e:?}"))?;
    let capacities = StateCapacities { data: 1, data_fields: 0, env_cells: 0,
        value_slots: 0, frame_slots: 0, control_rows: 1, stack_limit: 0 };
    let mut machine = Machine::new(program, 1, 0);
    let initial_bytes = roundtrip(&prepared, &capacities, &machine.snapshot())?;
    let record = machine.step().ok_or("reference machine produced no first record")?;
    let after_bytes = roundtrip(&prepared, &capacities, &record.after)?;
    let program_bytes = prepared.program_bytes().map_err(|e| format!("program bytes: {e:?}"))?;
    println!("canonical roundtrips=2 program_bytes={} initial_bytes={} after_bytes={}",
        program_bytes.len(), initial_bytes, after_bytes);
    Ok(())
}
