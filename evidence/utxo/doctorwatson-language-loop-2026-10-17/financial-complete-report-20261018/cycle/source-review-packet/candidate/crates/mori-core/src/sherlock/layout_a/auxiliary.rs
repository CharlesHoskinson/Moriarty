//! Internal host packing of supplied sherlock-k-a-aux/1 records.
//! This is an AuxBlock component, not packStepWitnessA or a trace validator.
//! No program evaluation, enable derivation, acceptance bit or native binding.
use super::limits::Locator;
use super::rows::{Capacities, Slice, TableId, ValueRef};
use super::state::{EnvRoot, StateCapacities, StateFailure, StateResult};

/// Source profile identity only: not serialized or bound to a native statement.
pub(super) const AUX_PROFILE: &str = "sherlock-k-a-aux/1";

/// Caller supplied, no numeric default or adequacy claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct AuxDimensions { pub q_eq: u128 }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub(super) enum EnvTerm { #[default] None = 0, Found = 1, Absent = 2 }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub(super) enum EqTerm { #[default] None = 0, Mismatch = 1, Empty = 2 }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub(super) enum NodeKind { #[default] UInt = 0, Bool = 1, Constructor = 2 }

// Defaults below are zero *rows*, not dimension/resource defaults or trace builders.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct EnvGetRow {
    pub act: bool, pub cur: EnvRoot, pub name_id: usize, pub value: ValueRef,
    pub parent: EnvRoot, pub matches: bool, pub term: EnvTerm,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EnvGetScan {
    pub res_found: bool, pub res_value: ValueRef, pub rows: Vec<EnvGetRow>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct FindRow {
    pub act: bool, pub name_id: usize, pub matches: bool, pub seen: bool, pub first: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FindScan { pub out_present: bool, pub out_id: usize, pub rows: Vec<FindRow> }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct BindHeader {
    pub parameters_start: usize, pub k: usize, pub return_type: usize, pub body: usize,
    pub values_start: usize, pub values_len: usize, pub len_ok: bool, pub ok: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct BindRow {
    pub reached: bool, pub name_id: usize, pub parameter_type: usize,
    pub argument: ValueRef, pub argument_type: usize, pub type_ok: bool,
    pub duplicate: bool, pub pass: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BindPreflight {
    pub header: BindHeader,
    pub rows: Vec<BindRow>,
    /// Strict upper triangle: (0,1)..(0,P-1),(1,2)..(P-2,P-1).
    /// No diagonal/lower triangle, pass_P, diagnostic or result column.
    pub eq_name: Vec<bool>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct RefPair { pub left: ValueRef, pub right: ValueRef }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct AuxNode {
    pub kind: NodeKind, pub type_id: usize, pub ctor_id: usize,
    pub uint_bytes: [u8; 16], pub bool_value: bool, pub fields: Slice,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EqRow {
    pub act: bool, pub sp: usize, pub stack: Vec<RefPair>,
    pub left: AuxNode, pub right: AuxNode, pub hd: bool,
    pub fields_left: [ValueRef; 2], pub fields_right: [ValueRef; 2], pub term: EqTerm,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EqScratch { pub rows: Vec<EqRow> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct AuxBlock {
    pub env_get: EnvGetScan, pub find: FindScan, pub bind: BindPreflight, pub eq: EqScratch,
}

fn host(field: &'static str) -> StateFailure {
    StateFailure::HostUnrepresentable { field, locator: Locator::field(field) }
}
fn shape(table: &'static str, row: usize, column: &'static str,
    definition: &'static str) -> StateFailure {
    StateFailure::WrongRepresentation { block: "AuxBlock", table, row, column, definition }
}
fn length(actual: usize, expected: u128, table: &'static str,
    row: usize, column: &'static str) -> StateResult<()> {
    if actual as u128 != expected {
        return Err(shape(table, row, column, "exact caller-dimension fixed shape"));
    }
    Ok(())
}

// Same width and fallible byte-append routines as state_encoding.rs. Kept local
// so no existing codec visibility/behavior is changed. These are representation
// widths, not active-index, slice-validity or semantic checks.
fn count_bits(capacity: u128) -> u32 { (128 - capacity.leading_zeros()).max(1) }
fn id_bits(capacity: u128) -> u32 { count_bits(capacity.saturating_sub(1)) }
struct Writer { bytes: Vec<u8>, location: Locator }
impl Writer {
    fn at(&mut self, table: &'static str, row: usize) {
        self.location = Locator { expression: None, function: None, field: table, item: row };
    }
    fn host(&self) -> StateFailure {
        StateFailure::HostUnrepresentable { field: self.location.field, locator: self.location }
    }
    fn raw(&mut self, bytes: &[u8]) -> StateResult<()> {
        self.bytes.try_reserve(bytes.len()).map_err(|_| self.host())?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
    fn unsigned(&mut self, value: u128, bits: u32) -> StateResult<()> {
        if bits < 128 && value >> bits != 0 { return Err(self.host()); }
        let width = ((bits + 7) / 8) as usize;
        self.raw(&value.to_le_bytes()[..width])
    }
    fn boolean(&mut self, value: bool) -> StateResult<()> { self.raw(&[u8::from(value)]) }
    fn id(&mut self, value: usize, capacity: u128) -> StateResult<()> {
        self.unsigned(value as u128, id_bits(capacity))
    }
    fn count(&mut self, value: usize, capacity: u128) -> StateResult<()> {
        self.unsigned(value as u128, count_bits(capacity))
    }
    fn value_ref(&mut self, value: ValueRef, capacity: u128) -> StateResult<()> {
        self.boolean(value.space)?;
        self.id(value.index, capacity)
    }
    fn environment(&mut self, value: EnvRoot, capacity: u128) -> StateResult<()> {
        self.boolean(value.present)?;
        self.id(value.index, capacity)
    }
    fn node(&mut self, n: &AuxNode, types: u128, constructors: u128,
        fields: u128) -> StateResult<()> {
        self.raw(&[n.kind as u8])?;
        self.id(n.type_id, types)?;
        self.id(n.ctor_id, constructors)?;
        self.raw(&n.uint_bytes)?;
        self.boolean(n.bool_value)?;
        self.count(n.fields.start, fields)?;
        self.count(n.fields.len, fields)
    }
}

impl AuxBlock {
    /// Expand all supplied records in the exact P3 section 3 order.
    /// Success means shape/width packing only. It is not evidence for E/F/B/Q
    /// equations, disabled-record zeros, index guards, typed nodes, StepInv,
    /// canonicality, an adequate Q_eq, or an accepted machine transition.
    /// No lengths, dimensions, profile tag or derived enables are prepended.
    pub(super) fn ordered_bytes(&self, program: &Capacities,
        state: &StateCapacities, dimensions: &AuxDimensions) -> StateResult<Vec<u8>> {
        let cap = |id: TableId| program.tables[id as usize];
        if program.data != state.data {
            return Err(shape("dimensions", 0, "D", "Program and State D must agree"));
        }
        if dimensions.q_eq == 0 {
            return Err(shape("dimensions", 0, "Q_eq", "Q_eq >= 1"));
        }
        let width = state.data.checked_add(cap(TableId::Constants))
            .and_then(|v| v.checked_add(1)).ok_or_else(|| host("EqScratch.stack"))?;
        let parameters = cap(TableId::Parameters);
        // Divide the even factor before multiplication; no P+1 or unchecked product.
        let previous = parameters.saturating_sub(1);
        let triangle = if parameters % 2 == 0 {
            (parameters / 2).checked_mul(previous)
        } else {
            parameters.checked_mul(previous / 2)
        }.ok_or_else(|| host("BindPreflight.eqName"))?;
        length(self.env_get.rows.len(), state.env_cells.max(1), "EnvGetScan", 0, "rows")?;
        length(self.find.rows.len(), cap(TableId::Functions), "FindScan", 0, "rows")?;
        length(self.bind.rows.len(), parameters, "BindPreflight", 0, "rows")?;
        length(self.bind.eq_name.len(), triangle, "BindPreflight", 0, "eqName")?;
        length(self.eq.rows.len(), dimensions.q_eq, "EqScratch", 0, "rows")?;
        for (i, row) in self.eq.rows.iter().enumerate() {
            length(row.stack.len(), width, "EqScratch", i, "st")?;
        }

        let values = cap(TableId::Constants).max(state.data);
        let fields = cap(TableId::ConstantFields).max(state.data_fields);
        let mut w = Writer { bytes: Vec::new(), location: Locator::field("EnvGetScan.header") };
        w.boolean(self.env_get.res_found)?;
        w.value_ref(self.env_get.res_value, values)?;
        for (i, r) in self.env_get.rows.iter().enumerate() {
            w.at("EnvGetScan", i);
            w.boolean(r.act)?;
            w.environment(r.cur, state.env_cells)?;
            w.id(r.name_id, cap(TableId::Names))?;
            w.value_ref(r.value, values)?;
            w.environment(r.parent, state.env_cells)?;
            w.boolean(r.matches)?;
            w.raw(&[r.term as u8])?;
        }
        w.at("FindScan.header", 0);
        w.boolean(self.find.out_present)?;
        w.id(self.find.out_id, cap(TableId::Functions))?;
        for (i, r) in self.find.rows.iter().enumerate() {
            w.at("FindScan", i);
            w.boolean(r.act)?;
            w.id(r.name_id, cap(TableId::Names))?;
            w.boolean(r.matches)?;
            // R2-7: seen may remain true beyond activeFunctions. Never zero by act.
            w.boolean(r.seen)?;
            w.boolean(r.first)?;
        }
        w.at("BindPreflight.header", 0);
        let h = &self.bind.header;
        w.count(h.parameters_start, parameters)?;
        w.count(h.k, parameters)?;
        w.id(h.return_type, cap(TableId::Types))?;
        w.id(h.body, cap(TableId::Expressions))?;
        w.count(h.values_start, state.value_slots)?;
        w.count(h.values_len, state.value_slots)?;
        w.boolean(h.len_ok)?;
        w.boolean(h.ok)?;
        for (i, r) in self.bind.rows.iter().enumerate() {
            w.at("BindPreflight", i);
            w.boolean(r.reached)?;
            w.id(r.name_id, cap(TableId::Names))?;
            w.id(r.parameter_type, cap(TableId::Types))?;
            w.value_ref(r.argument, values)?;
            w.id(r.argument_type, cap(TableId::Types))?;
            w.boolean(r.type_ok)?;
            w.boolean(r.duplicate)?;
            w.boolean(r.pass)?;
        }
        // The supplied flat vector is defined in strict upper-triangle row-major
        // order by BindPreflight.eq_name; no diagonal or boundary pass_P is stored.
        for (i, bit) in self.bind.eq_name.iter().enumerate() {
            w.at("BindPreflight.eqName", i);
            w.boolean(*bit)?;
        }
        // EqScratch has no header and no result/acceptance flag.
        for (i, r) in self.eq.rows.iter().enumerate() {
            w.at("EqScratch", i);
            w.boolean(r.act)?;
            w.count(r.sp, width)?;
            for pair in &r.stack {
                w.value_ref(pair.left, values)?;
                w.value_ref(pair.right, values)?;
            }
            w.node(&r.left, cap(TableId::Types), cap(TableId::Constructors), fields)?;
            w.node(&r.right, cap(TableId::Types), cap(TableId::Constructors), fields)?;
            w.boolean(r.hd)?;
            for value in r.fields_left { w.value_ref(value, values)?; }
            for value in r.fields_right { w.value_ref(value, values)?; }
            w.raw(&[r.term as u8])?;
        }
        Ok(w.bytes)
    }
}
