//! Explicit ordinary reference API for the opt-in unsigned floor-division profile.
use crate::policy::{Cost, Meter};
use crate::{CostReport, DetailedReplayError, Error, Execution, MeteredExecution,
    MeteredExecutionError, Program, Scenario, UInt};

pub const LANGUAGE: &str = "moriarty/proposed-uint-floor-div/1";
pub const SEMANTICS: &str = "moriarty-core/proposed-uint-floor-div/1";
pub const LOGICAL_COST_PROFILE: &str = "moriarty-proposed-reference-logical-cost-floor-div/1";

pub fn parse(source: &str) -> Result<Program, Error> {
    crate::parser::parse_floor_div_draft(source)
}
pub fn validate(program: &Program) -> Result<(), Error> {
    crate::runtime::validate_floor_div_draft(program)
}
pub fn simulate(program: &Program, action: &str, scenario: &Scenario) -> Result<Execution, Error> {
    crate::runtime::simulate_floor_div_draft_with_meter(program, action, scenario, None)
}
pub fn simulate_metered(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    cap: UInt,
) -> Result<MeteredExecution, MeteredExecutionError> {
    let mut meter = Meter::new(Cost { logical: cap, ..Cost::default() });
    let result = crate::runtime::simulate_floor_div_draft_with_meter(
        program, action, scenario, Some(&mut meter),
    );
    let cost = CostReport { profile: LOGICAL_COST_PROFILE.into(), cap, used: meter.used().logical };
    match result {
        Ok(execution) => Ok(MeteredExecution { execution, cost }),
        Err(error) => Err(MeteredExecutionError { error, cost }),
    }
}
pub fn replay_detailed(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    candidate: &Execution,
) -> Result<(), DetailedReplayError> {
    let actual = simulate(program, action, scenario).map_err(DetailedReplayError::Execution)?;
    crate::compare_execution(&actual, candidate).map_err(DetailedReplayError::Divergence)
}
