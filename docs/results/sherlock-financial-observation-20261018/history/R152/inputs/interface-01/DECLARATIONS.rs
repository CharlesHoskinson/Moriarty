// BODY-FREE ordinary observation contract. Not a registered/compilable module.
// Production implementations will use these exact public declarations.
use crate::{AssetId, Effect, UInt};

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
