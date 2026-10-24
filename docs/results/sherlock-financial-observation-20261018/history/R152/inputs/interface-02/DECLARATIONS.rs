// BODY-FREE ordinary observation contract. Not a registered/compilable module.
// Production implementations will use these exact public declarations.
use crate::{AssetId, Effect, UInt, Scenario};
use crate::stage::{Continuation, StageEvent, StageOutcome, StageRejection, StagedProgram};

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
// Public function signatures (documentation declarations, no implementation):
// pub fn account_effects(effects: &[Effect]) -> AccountingReport<'_>;
// pub fn project_invocation<'a>(program: &'a crate::stage::StagedProgram,
//   input: InvocationInput<'a>,
//   result: &'a Result<crate::stage::StageOutcome,crate::stage::StageRejection>)
//   -> InvocationObservation<'a>;

// Invocation views retain complete original inputs and outputs, not claims
// about their provenance/admission or a manufactured returned continuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvocationInput<'a> {
    Start { action: &'a str, scenario: &'a Scenario, cap: Option<UInt> },
    Resume { continuation: &'a Continuation, event: &'a StageEvent,
        cli_action: Option<&'a str>, cli_cap: Option<UInt> },
}
#[derive(Debug, Clone)]
pub struct InvocationObservation<'a> {
    pub program: &'a StagedProgram,
    pub input: InvocationInput<'a>,
    pub result: &'a Result<StageOutcome, StageRejection>,
    pub outcome: FinancialObservation<'a>,
    pub accounting: AccountingAvailability<'a>,
    pub tentative_publication: TentativePublication,
}
// FinancialObservation, SegmentObservation and ExecutionObservation are exactly
// the pre-existing borrowed field declarations reproduced in PRIOR-DECLARATIONS.rs.
