//! Proposed output DTOs and abstract interface; no implementation.
use mori_core::{
    Liability, Program, Scenario, Snapshot, UInt, Value,
    stage::{Continuation, PendingFrame, StageEvent, StagePlan},
};
use serde::Serialize;
use serde_json::Value as Json;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityPolicy { AllObserved }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SuccessPolicy { AllComplete }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FramePolicy { DisjointUnion }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Availability { NotObserved, Observed }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParentSuccess { Pending, Complete }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Phase { ParentsPending, ReadyToEnter, SuccessorPending, Complete }

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProgramRef {
    pub module: String,
    pub program_hash: String,
    pub plan_hash: String,
    pub action: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ParentBinding {
    pub label: String,
    #[serde(flatten)]
    pub program: ProgramRef,
}
#[derive(Debug, Clone, Serialize)]
pub struct ModuleView {
    pub alias: String,
    pub source_key: String,
    pub program: Program,
    pub stage_plan: StagePlan,
}
#[derive(Debug, Clone, Serialize)]
pub struct JoinPlanView {
    pub format: String,
    pub name: String,
    pub modules: Vec<ModuleView>,
    pub parents: [ParentBinding; 2],
    pub successor: ProgramRef,
    pub availability_policy: AvailabilityPolicy,
    pub success_policy: SuccessPolicy,
    pub frame_policy: FramePolicy,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ControlIdentity {
    pub format: String,
    pub join_name: String,
    pub parents: [ParentBinding; 2],
    pub successor: ProgramRef,
    pub availability_policy: AvailabilityPolicy,
    pub success_policy: SuccessPolicy,
    pub frame_policy: FramePolicy,
    pub instance_tag: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ParentFrame {
    pub label: String,
    pub frame: PendingFrame,
    pub boundary: Snapshot,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SuccessorFrame { pub frame: PendingFrame, pub boundary: Snapshot }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct JointFrame {
    pub parents: [ParentFrame; 2],
    pub successor: Option<SuccessorFrame>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LocalCaps {
    pub parents: [Option<UInt>; 2],
    pub successor: Option<UInt>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Overhead { Unmeasured }
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct JoinWork {
    pub aggregation: String,
    pub imported_attempted: [UInt; 2],
    pub post_attach_attempted: [UInt; 2],
    pub current_committed: [UInt; 2],
    pub successor_attempted: UInt,
    pub successor_committed: UInt,
    pub core_attempted_total: UInt,
    pub core_committed_total: UInt,
    pub this_call_core_attempted: UInt,
    pub orchestration_overhead: Overhead,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation { Attach, ResumeParent, Enter, ResumeSuccessor }
#[derive(Debug, Clone, Serialize)]
pub struct Invocation {
    pub ordinal: UInt,
    pub operation: Operation,
    pub parent: Option<String>,
    pub core_report: Option<Json>,
}
#[derive(Debug, Clone, Serialize)]
pub struct JoinView {
    pub phase: Phase,
    pub control: ControlIdentity,
    pub current_frame: JointFrame,
    pub availability: [Availability; 2],
    pub success: [ParentSuccess; 2],
    pub availability_satisfied: bool,
    pub success_satisfied: bool,
    pub union_boundary: Snapshot,
    pub handoff_boundary: Option<Snapshot>,
    pub effective_boundary: Snapshot,
    pub residual: Vec<Liability>,
    pub caps: LocalCaps,
    pub work: JoinWork,
    pub imported_parent_reports: [Json; 2],
    pub after_attach: Vec<Invocation>,
}
#[derive(Debug, Clone, Serialize)]
pub struct JoinReport {
    pub format: String,
    pub qualification: String,
    pub authentication: String,
    pub consumption: String,
    #[serde(flatten)]
    pub view: JoinView,
}

pub enum Outcome<C, Done> {
    ParentsPending { continuation: C },
    ReadyToEnter { continuation: C },
    SuccessorPending { continuation: C },
    Complete { completion: Done },
}

/// Future free functions in stage::join_draft use these signatures.
/// Program, PendingParent, JoinContinuation and JoinCompletion are opaque owned
/// types with private fields. Reports/views are Serialize-only output.
/// No decoder, default constructor or alternate evaluator is supplied here.
pub trait ProposedAllOfJoinApi {
    type Program;
    type PendingParent;
    type Continuation;
    type Completion;
    type Error;
    fn load(source: &str, modules: &[(&str, &str)]) -> Result<Self::Program, Self::Error>;
    fn plan_report(program: &Self::Program) -> Json;
    fn prepare_parent(program: &Self::Program, label: &str, scenario: Scenario, cap: Option<UInt>)
        -> Result<Self::PendingParent, Self::Error>;
    fn parent_report(parent: &Self::PendingParent) -> Json;
    fn attach(program: &Self::Program, parents: [Self::PendingParent; 2],
        successor_inputs: BTreeMap<String, Value>, successor_cap: Option<UInt>, instance_tag: &str)
        -> Result<Outcome<Self::Continuation, Self::Completion>, Self::Error>;
    fn resume_parent(program: &Self::Program, continuation: Self::Continuation,
        label: &str, event: StageEvent)
        -> Result<Outcome<Self::Continuation, Self::Completion>, Self::Error>;
    fn enter(program: &Self::Program, continuation: Self::Continuation)
        -> Result<Outcome<Self::Continuation, Self::Completion>, Self::Error>;
    fn resume_successor(program: &Self::Program, continuation: Self::Continuation, event: StageEvent)
        -> Result<Outcome<Self::Continuation, Self::Completion>, Self::Error>;
    fn outcome_report(outcome: &Outcome<Self::Continuation, Self::Completion>) -> Json;
    fn view(continuation: &Self::Continuation) -> JoinView;
    fn completion_view(completion: &Self::Completion) -> JoinView;
    fn parent_continuation<'a>(continuation: &'a Self::Continuation, label: &str) -> Option<&'a Continuation>;
}
