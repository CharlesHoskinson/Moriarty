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
/// Executes with a separately trusted, unsigned local logical-work cap.
/// A refusal reports charged attempts and exposes no partial execution.
// The refusal carries its charged cost report by design.
#[allow(clippy::result_large_err)]
pub fn simulate_metered(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    cap: UInt,
) -> Result<MeteredExecution, MeteredExecutionError> {
    let mut meter = Meter::new(Cost {
        logical: cap,
        ..Cost::default()
    });
    let result = crate::runtime::simulate_with_meter(program, action, scenario, Some(&mut meter));
    let cost = CostReport {
        profile: LOGICAL_COST_PROFILE.into(),
        cap,
        used: meter.used().logical,
    };
    match result {
        Ok(execution) => Ok(MeteredExecution { execution, cost }),
        Err(error) => Err(MeteredExecutionError { error, cost }),
    }
}
/// Reconstructs once under the trusted cap, then uses existing first-divergence
/// comparison. Comparison and serialization work are outside this cost profile.
// The refusal carries its charged cost report by design.
#[allow(clippy::result_large_err)]
pub fn replay_detailed_metered(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    candidate: &Execution,
    cap: UInt,
) -> Result<CostReport, MeteredReplayError> {
    let metered =
        simulate_metered(program, action, scenario, cap).map_err(|refused| MeteredReplayError {
            error: DetailedReplayError::Execution(refused.error),
            cost: refused.cost,
        })?;
    compare_execution(&metered.execution, candidate).map_err(|difference| MeteredReplayError {
        error: DetailedReplayError::Divergence(difference),
        cost: metered.cost.clone(),
    })?;
    Ok(metered.cost)
}
