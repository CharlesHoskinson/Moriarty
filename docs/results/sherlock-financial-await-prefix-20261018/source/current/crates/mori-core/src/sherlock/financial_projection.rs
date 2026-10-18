//! Owning invocation observations of the existing public Core/2 stage carrier.
//! `start_observed` and `resume_observed` call the existing stage producer once,
//! retain its exact arguments/result, and publish a complete borrowed observation.
//! No transition, financial policy or target correspondence is changed.
//! The lower-level diagnostic projectors below perform no evaluation.
//! `project_invocation` retains the full actual program, captured input and raw
//! result alongside complete borrowed state/steps/effects/duties/work. It does
//! not establish that those arguments produced that result or passed admission.
//! Rejection publishes no tentative payload; none is synthesized here.
//! Accounting is diagnostic metadata over one published successful segment's
//! ordered effects, with explicit checked representability, separate from work.
//! No cumulative prefix history, native fee, DUST or ledger phase is invented.

use crate::stage::{
    Continuation, NamedValue, PendingFrame, StageEvent, StageLogical, StageOutcome, StageRejection,
    StageSegment,
};
use crate::{AssetId, Effect, Error, Execution, Liability, Scenario, Snapshot, Step, UInt};
use crate::stage::StagedProgram;

/// Every field of one existing execution, borrowed without reduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct ExecutionObservation<'a> {
    pub semantics: &'a str,
    pub action: &'a str,
    pub pre: &'a Snapshot,
    pub post: &'a Snapshot,
    pub steps: &'a [Step],
    pub effects: &'a [Effect],
}

/// One published successful segment, including both pending frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SegmentObservation<'a> {
    pub action: &'a str,
    pub frame_pre: &'a PendingFrame,
    pub frame_post: &'a PendingFrame,
    pub execution: ExecutionObservation<'a>,
}

/// The three cases already represented by the public stage result type.
///
/// No absent segment is replaced by an empty execution. In particular a false
/// await guard retains `segment: None`, while a successful zero-effect segment
/// remains `Some` and keeps its steps, frames and complete state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum FinancialObservation<'a> {
    Waiting {
        advanced: bool,
        segment: Option<SegmentObservation<'a>>,
        continuation: &'a Continuation,
        attempted: UInt,
        residual: &'a [Liability],
    },
    Complete {
        segment: SegmentObservation<'a>,
        committed: UInt,
        attempted: UInt,
        residual: &'a [Liability],
    },
    Rejected {
        error: &'a Error,
        attempted: UInt,
        committed: UInt,
    },
}

/// Ordinary field/control view of the actual returned Core/2 await boundary.
///
/// `Waiting` borrows one returned continuation's control, committed snapshot and
/// logical counters, together with that outcome's residual duties and optional
/// published segment. It is not a universal-machine image or persistent codec.
/// The complete continuation envelope remains in `FinancialObservation`.
///
/// `advanced: false` and `published_segment: None` remain distinct from a
/// successful segment with no effects. No earlier effects are reconstructed.
/// Completion and rejection do not acquire a synthetic await checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum AwaitPrefixObservation<'a> {
    Waiting {
        advanced: bool,
        action: &'a str,
        site: u16,
        captures: &'a [NamedValue],
        logical: &'a StageLogical,
        committed_boundary: &'a Snapshot,
        residual: &'a [Liability],
        published_segment: Option<SegmentObservation<'a>>,
        attempted: UInt,
    },
    Complete,
    RejectedNoReturnedCheckpoint,
}

// No evaluation, admission, checkpoint construction or input/result pairing.
// Only fields of the same returned outcome feed this consumed publication view.
fn project_await_prefix(
    result: &Result<StageOutcome, StageRejection>,
) -> AwaitPrefixObservation<'_> {
    match result {
        Ok(StageOutcome::Waiting {
            advanced, segment, continuation, attempted, residual,
        }) => {
            let Continuation {
                format: _, program_hash: _, plan_hash: _, action, site, captures,
                logical, boundary,
            } = continuation;
            AwaitPrefixObservation::Waiting {
                advanced: *advanced,
                action,
                site: *site,
                captures,
                logical,
                committed_boundary: boundary,
                residual,
                published_segment: segment.as_ref().map(project_segment),
                attempted: *attempted,
            }
        }
        Ok(StageOutcome::Complete {
            segment: _, committed: _, attempted: _, residual: _,
        }) => AwaitPrefixObservation::Complete,
        Err(StageRejection { error: _, attempted: _, committed: _ }) => {
            AwaitPrefixObservation::RejectedNoReturnedCheckpoint
        }
    }
}

/// Captured continuation, event and CLI flags, separate from the returned result.
///
/// `input` is the caller-supplied continuation. This view does not establish
/// that `outcome` was produced from it. Callers capture the actual arguments
/// represented here and the returned result at the existing `stage::resume`
/// call site. The staged program argument and its actual identity are omitted;
/// the continuation's claimed hashes do not substitute for that argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct ResumeFinancialObservation<'a> {
    pub input: &'a Continuation,
    pub event: &'a StageEvent,
    pub cli_action: Option<&'a str>,
    pub cli_cap: Option<UInt>,
    pub outcome: FinancialObservation<'a>,
}

/// Projects an execution without recomputing state, effects or step values.
pub fn project_execution(execution: &Execution) -> ExecutionObservation<'_> {
    let Execution { semantics, action, pre, post, steps, effects } = execution;
    ExecutionObservation { semantics, action, pre, post, steps, effects }
}

/// Projects both frames and the complete segment execution.
pub fn project_segment(segment: &StageSegment) -> SegmentObservation<'_> {
    let StageSegment { action, frame_pre, frame_post, execution } = segment;
    SegmentObservation {
        action,
        frame_pre,
        frame_post,
        execution: project_execution(execution),
    }
}

/// Projects an existing start or resume result. No evaluator is invoked.
///
/// Exhaustive field patterns make additions to these result structures require
/// an explicit mapping update when this module is compiled with them.
pub fn project_stage_result(
    result: &Result<StageOutcome, StageRejection>,
) -> FinancialObservation<'_> {
    match result {
        Ok(StageOutcome::Waiting {
            advanced, segment, continuation, attempted, residual,
        }) => FinancialObservation::Waiting {
            advanced: *advanced,
            segment: segment.as_ref().map(project_segment),
            continuation,
            attempted: *attempted,
            residual,
        },
        Ok(StageOutcome::Complete { segment, committed, attempted, residual }) => {
            FinancialObservation::Complete {
                segment: project_segment(segment),
                committed: *committed,
                attempted: *attempted,
                residual,
            }
        }
        Err(StageRejection { error, attempted, committed }) => {
            FinancialObservation::Rejected {
                error,
                attempted: *attempted,
                committed: *committed,
            }
        }
    }
}

/// Retains the captured continuation, event and CLI flags with the full result.
pub fn project_resume_result<'a>(
    input: &'a Continuation,
    event: &'a StageEvent,
    cli_action: Option<&'a str>,
    cli_cap: Option<UInt>,
    result: &'a Result<StageOutcome, StageRejection>,
) -> ResumeFinancialObservation<'a> {
    ResumeFinancialObservation {
        input, event, cli_action, cli_cap, outcome: project_stage_result(result),
    }
}

impl<'a> FinancialObservation<'a> {
    /// The successful invocation's published boundary, if one was returned.
    /// A rejection has no returned financial boundary.
    pub fn committed_boundary(&self) -> Option<&'a Snapshot> {
        match self {
            Self::Waiting { continuation, .. } => Some(&continuation.boundary),
            Self::Complete { segment, .. } => Some(segment.execution.post),
            Self::Rejected { .. } => None,
        }
    }

    /// Only this invocation's published segment; never a prefix effect history.
    pub fn successful_segment(&self) -> Option<SegmentObservation<'a>> {
        match self {
            Self::Waiting { segment, .. } => *segment,
            Self::Complete { segment, .. } => Some(*segment),
            Self::Rejected { .. } => None,
        }
    }

    /// The exact `committed` logical work counter reported by Core/2.
    ///
    /// On early resume refusals this can be an unvalidated input echo; start
    /// refusals and CLI failures before work can report zero. This projection
    /// does not establish admission or independently validate the counter.
    pub fn committed_logical_work(&self) -> UInt {
        match self {
            Self::Waiting { continuation, .. } => continuation.logical.committed,
            Self::Complete { committed, .. } | Self::Rejected { committed, .. } => *committed,
        }
    }
}

impl<'a> ResumeFinancialObservation<'a> {
    /// The caller-supplied, unvalidated boundary, including on rejection.
    ///
    /// It is unauthenticated input and represents a locally retained prefix
    /// only when admission passed. This projection cannot infer that passage
    /// and does not substitute this input for a missing returned boundary.
    pub fn input_boundary(&self) -> &'a Snapshot {
        &self.input.boundary
    }
}

/// Full ordered diagnostic comparison against an independently frozen view.
///
/// For an independent oracle comparison, the expected view must be authored
/// independently before candidate execution. This includes statement ExprIds
/// and continuation hashes: their independent literal IR/plan derivation must
/// be recorded separately, without parser/evaluator/candidate output. Existing
/// Core/1 Step/Execution JSON decoding cannot bind an Await step; a separate
/// test-only typed literal binding remains required for Core/2 oracle fixtures.
/// Equality compares complete steps/effects/snapshots, frames, continuation
/// fields, residual rows, work counters and errors, including present zero
/// liabilities. Equality alone does not establish expected-value independence.
/// This helper does not derive or certify any expectation. The present normal
/// oracle task makes no independently specified Error-case comparison claim.
pub fn financial_observation_matches(
    actual: &Result<StageOutcome, StageRejection>,
    expected: &FinancialObservation<'_>,
) -> bool {
    project_stage_result(actual) == *expected
}

/// Full comparison of the captured resume view, including both CLI flags.
/// The staged program argument is not part of this view. Expected-value
/// independence and Core/2 typed fixture binding have the requirements above.
pub fn resume_financial_observation_matches(
    actual: &ResumeFinancialObservation<'_>,
    expected: &ResumeFinancialObservation<'_>,
) -> bool {
    actual == expected
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum AmountAggregate { Exact(UInt), AboveU128 }
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum NetFlow { Balanced, Inflow(UInt), Outflow(UInt), Unavailable }
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AccountFlow<'a> {
    pub asset: &'a AssetId,
    pub account: &'a str,
    pub transfer_debit: AmountAggregate,
    pub transfer_receipts: AmountAggregate,
    pub fee_debit: AmountAggregate,
    pub fee_receipts: AmountAggregate,
    pub gross_debit: AmountAggregate,
    pub gross_receipts: AmountAggregate,
    pub net_flow: NetFlow,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AccountingReport<'a> {
    pub effects: &'a [Effect],
    pub accounts: Vec<AccountFlow<'a>>,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum AccountingAvailability<'a> {
    PublishedSegment(AccountingReport<'a>),
    NoPublishedSegment,
    RejectedNoPublishedDelta,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TentativePublication {
    PublishedSuccessfulSegment,
    NoAdvancedSegment,
    UnavailableFromStageRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum InvocationInput<'a> {
    Start { action: &'a str, scenario: &'a Scenario, cap: Option<UInt> },
    Resume { continuation: &'a Continuation, event: &'a StageEvent,
        cli_action: Option<&'a str>, cli_cap: Option<UInt> },
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct InvocationObservation<'a> {
    #[serde(serialize_with = "serialize_staged_program")]
    pub program: &'a StagedProgram,
    pub input: InvocationInput<'a>,
    pub result: &'a Result<StageOutcome, StageRejection>,
    pub outcome: FinancialObservation<'a>,
    pub accounting: AccountingAvailability<'a>,
    pub tentative_publication: TentativePublication,
}

impl<'a> InvocationObservation<'a> {
    /// Borrows the returned await control and committed prefix from this result.
    /// The actual owning producer retains the same-call binding; diagnostic
    /// observations retain their existing caller-supplied relationship limits.
    pub fn await_prefix(&self) -> AwaitPrefixObservation<'a> {
        project_await_prefix(self.result)
    }
}

impl AmountAggregate {
    fn add(self, amount: UInt) -> Self {
        match self {
            Self::Exact(value) => match value.0.checked_add(amount.0) {
                Some(sum) => Self::Exact(UInt(sum)), None => Self::AboveU128,
            },
            Self::AboveU128 => Self::AboveU128,
        }
    }
}

fn account_index<'a>(accounts: &mut Vec<AccountFlow<'a>>, asset: &'a AssetId,
    account: &'a str) -> usize
{
    if let Some(index) = accounts.iter().position(|row| row.asset == asset && row.account == account) {
        return index;
    }
    let index = accounts.len();
    let zero = AmountAggregate::Exact(UInt(0));
    accounts.push(AccountFlow { asset, account, transfer_debit: zero, transfer_receipts: zero,
        fee_debit: zero, fee_receipts: zero, gross_debit: zero, gross_receipts: zero,
        net_flow: NetFlow::Balanced });
    index
}

/// Group exact existing transfer/fee identities in first encounter order.
/// Original full effects remain borrowed and ordered. Fees are part of gross,
/// never added to gross a second time; reverse transfers never erase debit.
/// Accounts with no monetary effect have no aggregate row, while their complete
/// balances/state/duties remain in the original invocation's snapshots.
pub fn account_effects(effects: &[Effect]) -> AccountingReport<'_> {
    let mut accounts = Vec::new();
    for effect in effects {
        let (asset, from, to, amount, fee) = match effect {
            Effect::Transfer { asset, from, to, amount } => (asset, from.as_str(), to.as_str(), *amount, false),
            Effect::Fee { asset, from, to, amount } => (asset, from.as_str(), to.as_str(), *amount, true),
            Effect::State { .. } | Effect::Liability { .. } | Effect::Emit { .. } => continue,
        };
        let debit = account_index(&mut accounts, asset, from);
        {
            let row = &mut accounts[debit];
            row.gross_debit = row.gross_debit.add(amount);
            if fee { row.fee_debit = row.fee_debit.add(amount); }
            else { row.transfer_debit = row.transfer_debit.add(amount); }
        }
        let credit = account_index(&mut accounts, asset, to);
        {
            let row = &mut accounts[credit];
            row.gross_receipts = row.gross_receipts.add(amount);
            if fee { row.fee_receipts = row.fee_receipts.add(amount); }
            else { row.transfer_receipts = row.transfer_receipts.add(amount); }
        }
    }
    for row in &mut accounts {
        row.net_flow = match (row.gross_receipts, row.gross_debit) {
            (AmountAggregate::Exact(receipts), AmountAggregate::Exact(debit)) => {
                if receipts.0 > debit.0 { NetFlow::Inflow(UInt(receipts.0 - debit.0)) }
                else if debit.0 > receipts.0 { NetFlow::Outflow(UInt(debit.0 - receipts.0)) }
                else { NetFlow::Balanced }
            }
            _ => NetFlow::Unavailable,
        };
    }
    AccountingReport { effects, accounts }
}

/// Read-only projection of one already returned actual stage result and its
/// captured input. No stage::start/resume, parser, policy or evaluator is called.
/// The program reference exposes the complete original Program/StagePlan and
/// their actual hashes through the existing getters. Callers retain the existing
/// stage::STAGE_LABELS on publication; this typed view adds no output wrapper.
/// A captured input boundary is not a returned rollback continuation.
pub fn project_invocation<'a>(
    program: &'a StagedProgram,
    input: InvocationInput<'a>,
    result: &'a Result<StageOutcome, StageRejection>,
) -> InvocationObservation<'a> {
    let (accounting, tentative_publication) = match result {
        Ok(StageOutcome::Waiting { segment: Some(segment), .. }) |
        Ok(StageOutcome::Complete { segment, .. }) => (
            AccountingAvailability::PublishedSegment(account_effects(&segment.execution.effects)),
            TentativePublication::PublishedSuccessfulSegment,
        ),
        Ok(StageOutcome::Waiting { segment: None, .. }) => (
            AccountingAvailability::NoPublishedSegment, TentativePublication::NoAdvancedSegment,
        ),
        Err(_) => (AccountingAvailability::RejectedNoPublishedDelta,
            TentativePublication::UnavailableFromStageRejection),
    };
    InvocationObservation { program, input, result, outcome: project_stage_result(result),
        accounting, tentative_publication }
}


/// Serialize every public component of the actual staged program context.
/// The program is already admitted by `stage::load`; hashes are identifiers,
/// not authentication or an independent correspondence certificate.
fn serialize_staged_program<S>(program: &&StagedProgram, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    #[derive(serde::Serialize)]
    struct Context<'a> {
        program: &'a crate::Program,
        plan: &'a crate::stage::StagePlan,
        program_hash: String,
        plan_hash: String,
    }
    fn hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
    serde::Serialize::serialize(&Context {
        program: program.program(),
        plan: program.plan(),
        program_hash: hex(program.program_hash()),
        plan_hash: hex(program.plan_hash()),
    }, serializer)
}

// Only the owning functions construct this input/result pair. Public diagnostic
// project_invocation remains available, but cannot construct ProducedInvocation.
#[derive(Debug)]
enum CapturedInvocationInput {
    Start { action: String, scenario: Scenario, cap: Option<UInt> },
    Resume { continuation: Continuation, event: StageEvent,
        cli_action: Option<String>, cli_cap: Option<UInt> },
}

/// The exact program, captured arguments and raw result of one real stage call.
///
/// Private fields prevent replacing the result or arguments. Borrowing this
/// record establishes the same-call relationship, not authentication, event
/// provenance, continuation consumption or native/financial correspondence.
/// The profile remains the existing local Core/2 stage carrier.
#[derive(Debug)]
pub struct ProducedInvocation<'p> {
    program: &'p StagedProgram,
    input: CapturedInvocationInput,
    result: Result<StageOutcome, StageRejection>,
}

/// Calls existing Core/2 start exactly once, retaining the consumed input value.
/// Cloning the scenario for the call preserves the equal original in the record.
pub fn start_observed<'p>(
    program: &'p StagedProgram,
    action: String,
    scenario: Scenario,
    cap: Option<UInt>,
) -> ProducedInvocation<'p> {
    let result = crate::stage::start(program, &action, scenario.clone(), cap);
    ProducedInvocation {
        program, input: CapturedInvocationInput::Start { action, scenario, cap }, result,
    }
}

/// Calls existing Core/2 resume exactly once with the captured continuation,
/// event and CLI flags. The original raw rejection/work fields remain intact.
pub fn resume_observed<'p>(
    program: &'p StagedProgram,
    continuation: Continuation,
    event: StageEvent,
    cli_action: Option<String>,
    cli_cap: Option<UInt>,
) -> ProducedInvocation<'p> {
    let result = crate::stage::resume(program, continuation.clone(), event.clone(),
        cli_action.as_deref(), cli_cap);
    ProducedInvocation {
        program,
        input: CapturedInvocationInput::Resume { continuation, event, cli_action, cli_cap },
        result,
    }
}

impl ProducedInvocation<'_> {
    /// The unmodified result returned by this record's single stage call.
    pub fn raw_result(&self) -> &Result<StageOutcome, StageRejection> {
        &self.result
    }

    /// Borrows this same program, exact captured input and original raw result.
    /// No producer, parser or evaluator is called while building this view.
    pub fn observation(&self) -> InvocationObservation<'_> {
        let input = match &self.input {
            CapturedInvocationInput::Start { action, scenario, cap } => {
                InvocationInput::Start { action, scenario, cap: *cap }
            }
            CapturedInvocationInput::Resume { continuation, event, cli_action, cli_cap } => {
                InvocationInput::Resume { continuation, event,
                    cli_action: cli_action.as_deref(), cli_cap: *cli_cap }
            }
        };
        project_invocation(self.program, input, &self.result)
    }

    /// Complete JSON publication shared with the actual Core/2 simulate branch.
    /// Existing status/segment/continuation/work/residual fields and local labels
    /// remain, with the entire serialized invocation and its returned await
    /// field/control view added. This is per-call accounting, not cumulative
    /// history, a B runtime, persistent checkpoint or target acceptance.
    pub fn publication(&self) -> Result<serde_json::Value, serde_json::Error> {
        let mut report = match &self.result {
            Ok(StageOutcome::Waiting { advanced, segment, continuation, attempted, residual }) => {
                serde_json::json!({
                    "status": "waiting", "advanced": advanced, "segment": segment,
                    "continuation": continuation, "attempted": attempted,
                    "committed": continuation.logical.committed, "residual": residual,
                })
            }
            Ok(StageOutcome::Complete { segment, committed, attempted, residual }) => {
                serde_json::json!({
                    "status": "complete", "segment": segment, "attempted": attempted,
                    "committed": committed, "residual": residual,
                })
            }
            Err(rejected) => serde_json::json!({
                "status": "rejected", "error": rejected.error,
                "attempted": rejected.attempted, "committed": rejected.committed,
            }),
        };
        let invocation = self.observation();
        let await_prefix = serde_json::to_value(invocation.await_prefix())?;
        let observation = serde_json::to_value(invocation)?;
        let object = report.as_object_mut().expect("stage reports are JSON objects");
        object.insert("financial_observation".into(), observation);
        object.insert("await_prefix".into(), await_prefix);
        for (name, value) in crate::stage::STAGE_LABELS {
            object.insert(name.into(), serde_json::Value::String(value.into()));
        }
        Ok(report)
    }
}
