//! Internal L4 ordered StateBlock bytes; no parser, digest or native embedding.
use crate::sherlock::Snapshot;
use super::limits::Locator;
use super::rows::{Capacities, PreparedA, Slice, TableId, ValueRef};
use super::state::{ControlRow, EnvRoot, FrameRow, StateBlock, StateCapacities,
    StateFailure, StateResult};
use super::state_encode::encode_observed_a;

// Mathematical count(Z) includes Z, so its width is bit_length(Z), at least 1.
// id(Z) uses bit_length(Z - 1), with zero/one capacities still one bit wide.
// This avoids forming Z + 1 at the u128 boundary.
fn count_bits(capacity: u128) -> u32 {
    (128 - capacity.leading_zeros()).max(1)
}
fn id_bits(capacity: u128) -> u32 {
    count_bits(capacity.saturating_sub(1))
}

struct Writer {
    bytes: Vec<u8>,
    location: Locator,
}

impl Writer {
    fn at(&mut self, table: &'static str, row: usize) {
        self.location = Locator { expression: None, function: None, field: table, item: row };
    }

    fn host(&self) -> StateFailure {
        StateFailure::HostUnrepresentable {
            field: self.location.field, locator: self.location,
        }
    }

    fn raw(&mut self, bytes: &[u8]) -> StateResult<()> {
        self.bytes.try_reserve(bytes.len()).map_err(|_| self.host())?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    fn unsigned(&mut self, value: u128, bits: u32) -> StateResult<()> {
        if bits < 128 && value >> bits != 0 {
            return Err(self.host());
        }
        let width = ((bits + 7) / 8) as usize;
        self.raw(&value.to_le_bytes()[..width])
    }

    fn byte(&mut self, value: u8) -> StateResult<()> {
        self.raw(&[value])
    }

    fn boolean(&mut self, value: bool) -> StateResult<()> {
        self.byte(u8::from(value))
    }

    fn id(&mut self, value: usize, capacity: u128) -> StateResult<()> {
        self.unsigned(value as u128, id_bits(capacity))
    }

    fn count(&mut self, value: usize, capacity: u128) -> StateResult<()> {
        self.unsigned(value as u128, count_bits(capacity))
    }

    fn slice(&mut self, value: Slice, capacity: u128) -> StateResult<()> {
        self.count(value.start, capacity)?;
        self.count(value.len, capacity)
    }

    fn value_ref(&mut self, value: ValueRef, capacity: u128) -> StateResult<()> {
        self.boolean(value.space)?;
        self.id(value.index, capacity)
    }

    fn environment(&mut self, value: EnvRoot, capacity: u128) -> StateResult<()> {
        self.boolean(value.present)?;
        self.id(value.index, capacity)
    }

    fn frame(&mut self, row: &FrameRow, program: &Capacities,
        state: &StateCapacities) -> StateResult<()> {
        let cap = |id: TableId| program.tables[id as usize];
        let values = cap(TableId::Constants).max(state.data);
        self.byte(row.tag)?;
        self.id(row.name_id, cap(TableId::Names))?;
        self.byte(row.binary_op)?;
        self.byte(row.unary_op)?;
        self.id(row.type_id as usize, cap(TableId::Types))?;
        self.id(row.expr0, cap(TableId::Expressions))?;
        self.id(row.expr1, cap(TableId::Expressions))?;
        self.environment(row.saved_env, state.env_cells)?;
        self.value_ref(row.value, values)?;
        self.slice(row.completed, state.value_slots)?;
        self.slice(row.remaining_exprs, cap(TableId::ExprChildren))?;
        self.slice(row.arms, cap(TableId::Arms))
    }

    fn control(&mut self, row: &ControlRow, program: &Capacities,
        state: &StateCapacities) -> StateResult<()> {
        let cap = |id: TableId| program.tables[id as usize];
        let values = cap(TableId::Constants).max(state.data);
        self.byte(row.tag)?;
        self.id(row.expr_id, cap(TableId::Expressions))?;
        self.value_ref(row.value, values)?;
        self.frame(&row.pending_frame, program, state)?;
        self.environment(row.environment, state.env_cells)?;
        self.id(row.name_id, cap(TableId::Names))?;
        self.slice(row.values, state.value_slots)?;
        self.boolean(row.function_present)?;
        self.id(row.function_id, cap(TableId::Functions))?;
        self.boolean(row.bindings_present)?;
        self.environment(row.bindings, state.env_cells)?;
        self.id(row.type_id as usize, cap(TableId::Types))?;
        self.slice(row.arms, cap(TableId::Arms))?;
        self.boolean(row.primitive_is_ok)?;
        self.byte(row.error_code)
    }
}

// This helper is deliberately private. Only the real canonical encoder supplies
// its padded block. It is not a raw-image acceptance or validation interface.
fn expand_state(block: &StateBlock, program: &Capacities,
    state: &StateCapacities) -> StateResult<Vec<u8>> {
    let cap = |id: TableId| program.tables[id as usize];
    let values = cap(TableId::Constants).max(state.data);
    let mut w = Writer { bytes: Vec::new(), location: Locator::field("Root") };
    let r = &block.root;
    w.count(r.active_data, state.data)?;
    w.count(r.active_data_fields, state.data_fields)?;
    w.count(r.active_env_cells, state.env_cells)?;
    w.count(r.active_value_slots, state.value_slots)?;
    w.count(r.depth, state.frame_slots)?;
    w.count(r.control_length, state.control_rows)?;
    w.environment(r.environment, state.env_cells)?;
    w.unsigned(r.stack_limit, count_bits(state.stack_limit))?;
    w.raw(&r.remaining_fuel)?;
    w.raw(&r.initial_fuel)?;
    w.byte(r.outcome_tag)?;
    w.value_ref(r.outcome_value, values)?;
    w.byte(r.outcome_error)?;

    // Entire fixed-capacity tables are expanded, including inactive zero rows.
    // Inactive zero id lanes are not dereferenced against an empty active table.
    for (i, r) in block.data.iter().enumerate() {
        w.at("Data", i);
        w.byte(r.kind)?;
        w.id(r.type_id as usize, cap(TableId::Types))?;
        w.id(r.ctor_id, cap(TableId::Constructors))?;
        w.raw(&r.uint_bytes)?;
        w.boolean(r.bool_value)?;
        w.slice(r.fields, state.data_fields)?;
    }
    for (i, r) in block.data_fields.iter().enumerate() {
        w.at("DataFields", i);
        w.value_ref(*r, values)?;
    }
    for (i, r) in block.env_cells.iter().enumerate() {
        w.at("EnvCells", i);
        w.id(r.name_id, cap(TableId::Names))?;
        w.value_ref(r.value, values)?;
        w.environment(r.parent, state.env_cells)?;
    }
    for (i, r) in block.value_slots.iter().enumerate() {
        w.at("ValueSlots", i);
        w.value_ref(*r, values)?;
    }
    for (i, r) in block.frame_slots.iter().enumerate() {
        w.at("FrameSlots", i);
        w.frame(r, program, state)?;
    }
    for (i, r) in block.control_rows.iter().enumerate() {
        w.at("ControlRows", i);
        w.control(r, program, state)?;
    }
    Ok(w.bytes)
}

/// Expand an observed current-A Snapshot after actual canonical encoding.
/// No byte parser, statement binding or native ABI is exposed.
/// No Machine-state importer; decode_observed_a is a public typed-row decoder
/// under the trust preconditions documented by layout_a.
pub fn observed_state_bytes_a(prepared: &PreparedA,
    capacities: &StateCapacities, snapshot: &Snapshot) -> StateResult<Vec<u8>> {
    let block = encode_observed_a(prepared, capacities, snapshot)?;
    expand_state(&block, &prepared.capacities, capacities)
}
