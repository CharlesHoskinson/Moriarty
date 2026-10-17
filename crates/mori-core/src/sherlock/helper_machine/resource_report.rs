//! Typed diagnostic projection of actual owner observations, SOURCE_ONLY/UNRUN.
//!
//! B consumed/remaining work includes this profile's helper charges. Preserve
//! helper counters and reference_cursor separately; this is no A/B cost match.
//! This copies observations, never validates/adopts an image or estimates an
//! unobserved cost. In particular final depth is not maximum depth, slice len
//! is not allocator capacity, and allocation counters are not physical bytes.
use serde::{Serialize, Serializer};

use super::shared::owner_host::HostObservation;
use super::shared::{BoundsC, ExhaustReason, FinanceInput, GrammarDomain, OutcomeC,
    RefC, RunIntC, RunSummary};

pub const FORMAT: &str = "moriarty-sherlock-b-resource-report/1";

/// Exact owner u128 counter; decimal JSON strings avoid precision truncation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counter(pub u128);
impl Serialize for Counter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ValueReference { Const { index: usize }, Data { index: usize } }
impl From<RefC> for ValueReference {
    fn from(value: RefC) -> Self {
        match value { RefC::Const(index) => Self::Const { index },
            RefC::Data(index) => Self::Data { index } }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Exhaustion {
    Fuel, DataNodes, DataFields, EnvironmentCells, HelperTasks, HelperWork, SerializationBytes,
}
impl From<ExhaustReason> for Exhaustion {
    fn from(reason: ExhaustReason) -> Self {
        match reason {
            ExhaustReason::Fuel => Self::Fuel, ExhaustReason::DataNodes => Self::DataNodes,
            ExhaustReason::DataFields => Self::DataFields,
            ExhaustReason::EnvironmentCells => Self::EnvironmentCells,
            ExhaustReason::HelperTasks => Self::HelperTasks,
            ExhaustReason::HelperWork => Self::HelperWork,
            ExhaustReason::SerializationBytes => Self::SerializationBytes,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ReportOutcome {
    Running, Halted { value: ValueReference }, Rejected { code: u8 },
    Exhausted { reason: Exhaustion },
}
impl From<OutcomeC> for ReportOutcome {
    fn from(outcome: OutcomeC) -> Self {
        match outcome {
            OutcomeC::Running => Self::Running,
            OutcomeC::Halted { value } => Self::Halted { value: value.into() },
            OutcomeC::Rejected { code } => Self::Rejected { code },
            OutcomeC::Exhausted { reason } => Self::Exhausted { reason: reason.into() },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum RunInput { Negative, Nat { value: Counter }, NatAboveU128 }
impl From<RunIntC> for RunInput {
    fn from(input: RunIntC) -> Self {
        match input { RunIntC::Negative => Self::Negative,
            RunIntC::Nat(value) => Self::Nat { value: Counter(value) },
            RunIntC::NatAboveU128 => Self::NatAboveU128 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportGrammar { DedicatedSorts, GenericNominalExtensionReserved }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportFinance { Absent, TypedExtensionReserved }

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Profile {
    pub profile_id: u8,
    pub grammar: ReportGrammar,
    pub finance: ReportFinance,
    pub layout_version: (String, String),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct RunInputs { pub fuel: RunInput, pub stack: RunInput }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Observed {
    pub outcome: ReportOutcome,
    pub initial_fuel: Counter,
    pub consumed_work: Counter,
    pub remaining_work: Counter,
    pub helper_work: Counter,
    pub helper_visits: Counter,
    pub reference_cursor: Counter,
    pub logical_depth: usize,
    pub helper_tasks: usize,
    pub stack_limit: usize,
    pub helper_present: bool,
    pub result_present: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Summary {
    pub outcome: ReportOutcome,
    pub events: Counter,
    pub consumed_work: Counter,
    pub helper_work: Counter,
    pub helper_visits: Counter,
    pub reference_cursor: Counter,
}
impl From<&RunSummary> for Summary {
    fn from(summary: &RunSummary) -> Self {
        Self { outcome: summary.outcome.into(), events: Counter(summary.events),
            consumed_work: Counter(summary.consumed_work), helper_work: Counter(summary.helper_work),
            helper_visits: Counter(summary.helper_visits), reference_cursor: Counter(summary.reference_cursor) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct StoredActive { pub data: usize, pub data_fields: usize, pub env: usize, pub value_slots: usize }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct CurrentSlices {
    pub data: usize, pub data_fields: usize, pub env: usize,
    pub frames: usize, pub tasks: usize, pub value_slots: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct AllocationCounters { pub data_nodes: usize, pub data_fields: usize, pub environment_cells: usize }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct ProgramActive {
    pub expressions: usize, pub occurrences: usize, pub occurrence_paths: usize,
    pub children: usize, pub functions: usize, pub parameters: usize,
    pub arms: usize, pub patterns: usize, pub constants: usize,
    pub constant_fields: usize, pub names: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct SuppliedBounds {
    pub code_rows: usize, pub function_rows: usize, pub code_child_slots: usize,
    pub parameter_slots: usize, pub pattern_rows: usize, pub match_plan_rows: usize,
    pub match_arm_rows: usize, pub type_rows: usize, pub constructor_rows: usize,
    pub constructor_field_slots: usize, pub max_arity: usize, pub constant_nodes: usize,
    pub constant_field_slots: usize, pub data_nodes: usize, pub data_field_slots: usize,
    pub environment_cells: usize, pub frames: usize, pub helper_tasks: usize,
    pub serialization_bytes: usize, pub max_reference_work: Counter,
    pub max_helper_work: Counter, pub step_cap: Counter,
}
impl From<&BoundsC> for SuppliedBounds {
    fn from(bounds: &BoundsC) -> Self {
        Self { code_rows: bounds.code_rows, function_rows: bounds.function_rows,
            code_child_slots: bounds.code_child_slots, parameter_slots: bounds.parameter_slots,
            pattern_rows: bounds.pattern_rows, match_plan_rows: bounds.match_plan_rows,
            match_arm_rows: bounds.match_arm_rows, type_rows: bounds.type_rows,
            constructor_rows: bounds.constructor_rows, constructor_field_slots: bounds.constructor_field_slots,
            max_arity: bounds.max_arity, constant_nodes: bounds.constant_nodes,
            constant_field_slots: bounds.constant_field_slots, data_nodes: bounds.data_nodes,
            data_field_slots: bounds.data_field_slots, environment_cells: bounds.environment_cells,
            frames: bounds.frames, helper_tasks: bounds.helper_tasks,
            serialization_bytes: bounds.serialization_bytes, max_reference_work: Counter(bounds.max_reference_work),
            max_helper_work: Counter(bounds.max_helper_work), step_cap: Counter(bounds.step_cap) }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct LayoutCapacities {
    pub name_rows: usize, pub name_bytes: usize,
    pub occurrence_path_slots: usize, pub value_slots: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Supplied {
    pub admission_bounds: SuppliedBounds,
    pub layout_bounds: SuppliedBounds,
    pub layout_capacities: LayoutCapacities,
    pub boundary_capacities_present: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnmeasuredStatus { Unmeasured }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct UnmeasuredGroup {
    pub status: UnmeasuredStatus,
    pub reason: &'static str,
    pub metrics: &'static [&'static str],
}
fn gap(reason: &'static str, metrics: &'static [&'static str]) -> UnmeasuredGroup {
    UnmeasuredGroup { status: UnmeasuredStatus::Unmeasured, reason, metrics }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Unmeasured {
    pub peaks: UnmeasuredGroup,
    pub physical: UnmeasuredGroup,
    pub circuit: UnmeasuredGroup,
    pub proof: UnmeasuredGroup,
    pub host: UnmeasuredGroup,
    pub overhead: UnmeasuredGroup,
    pub financial_target: UnmeasuredGroup,
    pub event_reconciliation: UnmeasuredGroup,
    pub remaining_nonhelper_reference_work: UnmeasuredGroup,
}
impl Unmeasured {
    pub fn from_snapshot_only() -> Self {
        Self {
            peaks: gap("final-snapshot-is-not-a-history",
                &["maximum-logical-depth", "peak-live-helper-tasks", "peak-storage-bytes"]),
            physical: gap("no-physical-accounting-observation",
                &["physical-dispatches", "frame-bytes", "environment-bytes", "node-bytes", "allocator-reserved-bytes"]),
            circuit: gap("no-circuit-observation",
                &["active-rows", "padded-rows", "active-cells", "padded-cells", "degree", "table-bytes", "code-bytes"]),
            proof: gap("no-native-proof-observation",
                &["key-bytes", "proof-bytes", "witness-time", "witness-memory", "proving-time", "proving-memory", "verification-time", "verification-memory"]),
            host: gap("no-host-measurement", &["wall-time", "peak-memory"]),
            overhead: gap("no-overhead-measurement",
                &["preparation", "wrappers", "checkpoint-io", "conversion", "report-serialization", "end-to-end-total"]),
            financial_target: gap("no-financial-or-target-observation",
                &["target-identity", "dust", "financial-fees", "service-fees", "reserve-recovery-accounting"]),
            event_reconciliation: gap("final-snapshot-and-summary-are-not-an-event-stream",
                &["per-rule-counts", "per-event-allocation-ledger"]),
            remaining_nonhelper_reference_work: gap("no-total-nonhelper-reference-budget",
                &["remaining-nonhelper-reference-work"]),
        }
    }
}

/// An owned, serializable diagnostic report, not a validated state or receipt.
/// Snapshot/summary values remain distinct even if a caller supplies a stale
/// summary. No check, subtraction, clamping, hidden total or peak is added.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ResourceReport {
    pub format: &'static str,
    pub profile: Profile,
    pub run_inputs: RunInputs,
    pub observed: Observed,
    pub run_summary: Summary,
    pub stored_active: StoredActive,
    pub current_slices: CurrentSlices,
    pub allocation_counters: AllocationCounters,
    pub program_active: ProgramActive,
    pub supplied: Supplied,
    pub unmeasured: Unmeasured,
}
impl ResourceReport {
    pub fn from_owner(observation: &HostObservation<'_>, summary: &RunSummary) -> Self {
        let admission = observation.admission;
        let image = observation.image;
        let view = admission.view;
        let identity = admission.identity;
        let capacities = admission.layout.capacities;
        Self {
            format: FORMAT,
            profile: Profile { profile_id: identity.profile_id,
                grammar: match identity.grammar_domain {
                    GrammarDomain::DedicatedSorts => ReportGrammar::DedicatedSorts,
                    GrammarDomain::GenericNominalExtensionReserved => ReportGrammar::GenericNominalExtensionReserved,
                },
                finance: match identity.finance {
                    FinanceInput::Absent => ReportFinance::Absent,
                    FinanceInput::TypedExtensionReserved => ReportFinance::TypedExtensionReserved,
                },
                layout_version: identity.layout_version.clone() },
            run_inputs: RunInputs { fuel: image.run_id.fuel_input.into(), stack: image.run_id.stack_limit_input.into() },
            observed: Observed { outcome: (*image.outcome).into(),
                initial_fuel: Counter(image.initial_fuel), consumed_work: Counter(image.consumed_work),
                remaining_work: Counter(image.remaining_work), helper_work: Counter(image.helper_work),
                helper_visits: Counter(image.helper_visits), reference_cursor: Counter(image.reference_cursor),
                logical_depth: image.depth(), helper_tasks: image.task_live(), stack_limit: image.stack_limit,
                helper_present: image.helper.is_some(), result_present: image.result.is_some() },
            run_summary: summary.into(),
            stored_active: StoredActive { data: image.active.data, data_fields: image.active.data_fields,
                env: image.active.env, value_slots: image.active.value_slots },
            current_slices: CurrentSlices { data: image.data.len(), data_fields: image.data_fields.len(),
                env: image.env.len(), frames: image.frames.len(), tasks: image.tasks.len(), value_slots: image.value_slots.len() },
            allocation_counters: AllocationCounters { data_nodes: image.alloc.data_nodes,
                data_fields: image.alloc.data_fields, environment_cells: image.alloc.environment_cells },
            program_active: ProgramActive { expressions: view.expressions.len(), occurrences: view.occurrences.len(),
                occurrence_paths: view.occurrence_paths.len(), children: view.children.len(), functions: view.functions.len(),
                parameters: view.parameters.len(), arms: view.arms.len(), patterns: view.patterns.len(),
                constants: view.constants.len(), constant_fields: view.constant_fields.len(), names: view.names.len() },
            supplied: Supplied { admission_bounds: admission.bounds.into(), layout_bounds: admission.layout.bounds.into(),
                layout_capacities: LayoutCapacities { name_rows: capacities.name_rows, name_bytes: capacities.name_bytes,
                    occurrence_path_slots: capacities.occurrence_path_slots, value_slots: capacities.value_slots },
                boundary_capacities_present: admission.layout.boundary_capacities_present },
            unmeasured: Unmeasured::from_snapshot_only(),
        }
    }
}

#[cfg(test)]
#[path = "resource_report_tests.rs"]
mod tests;
