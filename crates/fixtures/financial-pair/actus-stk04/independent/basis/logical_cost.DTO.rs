//! Local primitive interpretation cost, separate from execution and replay schemas.
//! Dust, service and external costs are unmeasured by this profile.
use crate::policy::{Cost, Meter};
use crate::{DetailedReplayError, Error, Execution, Program, Scenario, UInt, compare_execution};
use serde::Serialize;

pub const LOGICAL_COST_PROFILE: &str = "moriarty-reference-logical-cost/1";
pub const LOGICAL_COST_PROFILE_2: &str = "moriarty-reference-logical-cost/2";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CostReport {
    pub profile: String,
    pub cap: UInt,
    pub used: UInt,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MeteredExecution {
    pub execution: Execution,
    pub cost: CostReport,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq, thiserror::Error)]
#[error("{error}")]
pub struct MeteredExecutionError {
    pub error: Error,
    pub cost: CostReport,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq, thiserror::Error)]
#[error("{error}")]
pub struct MeteredReplayError {
    pub error: DetailedReplayError,
    pub cost: CostReport,
}
