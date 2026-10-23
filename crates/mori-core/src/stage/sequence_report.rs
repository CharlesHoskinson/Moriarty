//! Complete output carriers for local sequential composition.
//!
//! These serialize-only records retain original local reports alongside whole
//! frames, history and accounting. They neither construct nor validate a
//! Sequence, project a frame, compute accounting, or define failure publication.

use super::{PendingFrame, StageEvent, StagePlan};
use crate::{AssetId, Effect, Liability, Program, Snapshot, UInt, Value};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RunFormat {
    #[serde(rename = "moriarty-local-sequential-proposed/1")]
    Sequential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Qualification {
    #[serde(rename = "local-reference-stage")]
    LocalReferenceStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompositionQualification {
    #[serde(rename = "proposed-local-sequential-composition")]
    ProposedLocalSequentialComposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Authentication {
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Consumption {
    #[serde(rename = "untracked")]
    Untracked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProgressStatus {
    Ready,
    Waiting,
    BetweenPrograms,
    Complete,
}

/// Complete catalog objects, including unused actions and expressions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CatalogEntry {
    pub key: String,
    pub program: Program,
    pub stage_plan: StagePlan,
    pub program_hash: String,
    pub plan_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvocationSpec {
    pub program: String,
    pub action: String,
    pub inputs: BTreeMap<String, Value>,
    pub state_bindings: BTreeMap<String, String>,
    pub duty_bindings: BTreeMap<String, String>,
    /// Mandatory key, including null when the original cap is absent.
    pub logical_cap: Option<UInt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SequenceRunReport {
    pub format: RunFormat,
    pub qualification: Qualification,
    pub composition_qualification: CompositionQualification,
    pub authentication: Authentication,
    pub consumption: Consumption,
    pub status: ProgressStatus,
    pub catalog: Vec<CatalogEntry>,
    pub plan: Vec<InvocationSpec>,
    pub opening: Snapshot,
    pub calls: Vec<CallObservation>,
    pub effects: Vec<EffectLink>,
    pub observations: Vec<PrefixObservation>,
    /// Array position, not an identity or a replacement for earlier observations.
    pub final_observation: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CallKind {
    Start,
    Resume,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CallObservation {
    pub index: usize,
    pub invocation: usize,
    pub kind: CallKind,
    /// Mandatory null for start, complete original event for resume.
    pub event: Option<StageEvent>,
    pub whole_pre: Snapshot,
    pub whole_post: Snapshot,
    pub projected_pre: Snapshot,
    pub projected_post: Snapshot,
    pub outside: Snapshot,
    /// Entire successful `stage::outcome_report` value, unchanged. Raw
    /// `StageOutcome` serialization has a different envelope. This carrier
    /// neither constructs nor validates the embedded report.
    pub outcome: JsonValue,
    pub framed_steps: Vec<FramedStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FramedStep {
    pub step_offset: usize,
    /// Original Step.index, distinct from its array offset.
    pub source_index: u16,
    pub whole_snapshot: Snapshot,
    pub effect_prefix_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EffectLink {
    pub index: usize,
    pub call_index: usize,
    pub step_offset: usize,
    pub step_effect_offset: usize,
    pub execution_effect_offset: usize,
    pub whole_effect: Effect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ObservationCause {
    Opening,
    Call { call_index: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Progress {
    pub status: ProgressStatus,
    pub completed_invocations: usize,
    pub active_invocation: Option<usize>,
    /// Available to begin/next now: null while waiting or complete.
    pub next_invocation: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvocationFrame {
    pub invocation: usize,
    pub frame: PendingFrame,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LatestCall {
    pub invocation: usize,
    pub call_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrefixObservation {
    pub index: usize,
    pub cause: ObservationCause,
    pub call_prefix_len: usize,
    pub effect_prefix_len: usize,
    pub head: Snapshot,
    pub progress: Progress,
    pub frames: Vec<InvocationFrame>,
    pub latest: Vec<LatestCall>,
    pub financial: FinancialSummary,
    pub logical: LogicalSummary,
    pub residual: Vec<Liability>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetDirection {
    Increase,
    Decrease,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetChange {
    pub direction: NetDirection,
    pub amount: UInt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BalanceAccounting {
    pub asset: AssetId,
    pub account: String,
    pub opening: UInt,
    pub closing: UInt,
    pub transfer_debits: UInt,
    pub transfer_credits: UInt,
    pub fees_paid: UInt,
    pub fees_received: UInt,
    pub gross_debits: UInt,
    pub gross_credits: UInt,
    pub net_change: NetChange,
}

/// Presence remains separate from amount; absent has null, present zero has "0".
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LiabilityPresence {
    pub present: bool,
    pub amount: Option<UInt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LiabilityAccounting {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub opening: LiabilityPresence,
    pub additions: UInt,
    pub discharges: UInt,
    pub closing: LiabilityPresence,
    pub effect_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AssetTotals {
    pub asset: AssetId,
    pub transfer_volume: UInt,
    pub fee_volume: UInt,
    pub gross_debit_volume: UInt,
    pub gross_credit_volume: UInt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FinancialSummary {
    pub balances: Vec<BalanceAccounting>,
    pub liabilities: Vec<LiabilityAccounting>,
    pub asset_totals: Vec<AssetTotals>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogicalAllocation {
    pub ordinary: UInt,
    pub closure_reserve: UInt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvocationLogical {
    pub invocation: usize,
    pub cap: Option<UInt>,
    pub attempted: UInt,
    pub committed: UInt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceBasis {
    #[serde(rename = "reference-logical-accounting-only")]
    ReferenceLogicalAccountingOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogicalSummary {
    /// Original logical-cost profile, retained without rewriting local reports.
    pub profile: String,
    pub resource_basis: ResourceBasis,
    pub allocation: LogicalAllocation,
    pub per_invocation: Vec<InvocationLogical>,
    pub attempted_total: UInt,
    pub committed_total: UInt,
    pub available_ordinary: UInt,
    pub remaining_closure_reserve: UInt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ExpandedPrefixFormat {
    #[serde(rename = "moriarty-local-sequential-expanded-prefix-proposed/1")]
    SequentialExpandedPrefix,
}

/// Complete run prefix: full catalog/plan/opening and all earlier observations.
/// This is output/expected data, not a deserializable Sequence or handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExpandedPrefixFixture {
    pub format: ExpandedPrefixFormat,
    pub run: SequenceRunReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrefixComparison {
    pub index: usize,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WholeReportComparison {
    pub passed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CaseReportFormat {
    #[serde(rename = "moriarty-sequential-case-observations-proposed/1")]
    SequentialCaseObservations,
}

/// Successful operations, including ordinary expectation mismatches. This is
/// not an error envelope and performs no execution or expectation comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SuccessfulSequenceCaseReport {
    pub format: CaseReportFormat,
    pub name: String,
    pub passed: bool,
    pub actual: SequenceRunReport,
    pub observations: Vec<PrefixComparison>,
    pub report: WholeReportComparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregateStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SuccessfulSequenceTestAggregate {
    pub status: AggregateStatus,
    pub cases: Vec<SuccessfulSequenceCaseReport>,
}
