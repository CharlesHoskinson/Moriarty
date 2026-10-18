//! `moriarty/2` admission and the staged lowering plan (not lowered in this slice).
//!
//! A `/2` source is admitted by the actual `mori_core::stage::load` (parse_v2,
//! require_core2, validate_v2, plan, plan invariants, cost fill, cost invariants,
//! program/plan hashes; `stage/mod.rs:71-88`). [`StagedAdmission::lowering_plan`]
//! derives, from that real [`mori_core::stage::StagePlan`], the per-segment relation
//! shapes a staged lowering must produce. `compile_source` then refuses with
//! `CompileError::StagedNotLowered`: no staged relation exists yet and none is
//! approximated by the `/1` relation.
//!
//! Each future staged relation is one (program, plan, action, segment) and must bind,
//! in addition to the `/1` statement parts: the program **and** plan hashes; the pre
//! frame (Idle, or Pending{site, captures} in the site's exact capture layout); the
//! event binders of the entry site; the resumed guard evaluated as its own constrained
//! Step against the unchanged boundary; the segment's statements with an environment
//! seeded from captures and event (states read from the boundary); the exit (await
//! entry charge 1 and captures out in the next site's layout, or Complete with an
//! empty frame); `committed_before`/`committed_after` from the plan; the stored cap as
//! an explicit public slot (D-13 is split, see [`StoredCapParameter`]); the R7 boundary
//! canonicality predicates over the represented cells; and residual duties
//! (present with amount > 0) derivable from the post duty slots, present-zero kept
//! distinct from absent. A false guard (`NotAdvanced`) is a separate "not a
//! transaction" relation with attempted work `dispatch_cost` and an unchanged
//! continuation. See STAGED-LOWERING-PLAN.md.
use crate::error::CompileError;
use crate::identity::Digest32;
use crate::plan;
use mori_core::stage::{CaptureSlot, StageBinder, StagedProgram};
use mori_core::{Statement, UInt};
use serde::Serialize;

/// A `/2` program admitted by the actual stage loader. Only `frontend` constructs one.
#[derive(Debug, Clone)]
pub struct StagedAdmission {
    staged: StagedProgram,
    source_sha256: Digest32,
}

/// D-13 (canonical reconstruction cap) is split between two recorded votes:
/// `Some(plan_total[a])` (pr54-stage-owner-tests-and-design DESIGN-VOTES D-13) and
/// `None` (pr54-owner-suite independent Opus review §9). This compiler selects
/// neither: a future staged relation carries the stored cap as a public, caller-chosen
/// slot and never fabricates a reconstruction default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum StoredCapParameter {
    /// The continuation's stored cap (`Option<UInt>`) is a public statement slot:
    /// a presence Bool and a UInt, absent exactly `(false, 0)`.
    ExplicitPublicSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SegmentEntry {
    /// Segment 0: inputs seed the environment; the pre frame is locally Idle.
    Start { inputs: Vec<CaptureSlot> },
    /// Segment `site`: captures and event binders seed the environment; the guard at
    /// `guard_ordinal` must evaluate to true as its own Step.
    Resume { site: u16, label: String, guard_ordinal: u16, captures: Vec<CaptureSlot>, binders: Vec<StageBinder> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SegmentExit {
    /// The next await is reached: entry charge 1, captures in that site's layout.
    Await { site: u16, label: String, ordinal: u16, captures: Vec<CaptureSlot> },
    /// The action completes; the post frame is Idle with no captures.
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StagedSegmentShape {
    pub segment: u16,
    pub entry: SegmentEntry,
    /// Half-open source ordinals executed by this segment (a resume starts at its
    /// guard; the exit await's ordinal is excluded).
    pub statement_start: u16,
    pub statement_end: u16,
    pub exit: SegmentExit,
    /// From the actual stage plan (`stage/cost.rs:70-145`).
    pub segment_cost: UInt,
    pub committed_before: UInt,
    pub committed_after: UInt,
    /// Attempted work of a false guard at this resume site (NotAdvanced relation).
    pub not_advanced_attempted: Option<UInt>,
    /// Effect kinds in statement order, including Emit (never dropped).
    pub effect_kinds: Vec<String>,
    pub has_emit: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StagedActionPlan {
    pub action: String,
    pub action_index: u16,
    pub statement_count: u16,
    pub plan_total: UInt,
    pub segments: Vec<StagedSegmentShape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StagedLoweringPlan {
    pub language: String,
    pub semantics: String,
    pub logical_cost_profile: String,
    pub stage_plan_format: String,
    pub continuation_format: String,
    pub program_hash: Digest32,
    pub plan_hash: Digest32,
    pub source_sha256: Digest32,
    pub stored_cap: StoredCapParameter,
    pub actions: Vec<StagedActionPlan>,
    /// Required before any staged relation exists; none is waived.
    pub remaining: Vec<String>,
}

/// Remaining staged obligations. Listing them waives none.
pub const STAGED_REMAINING: &[&str] = &[
    "segment relations for every (action, segment) with program and plan hash bound in-circuit",
    "pre/post frame slots (Idle | Pending{site, captures}) in the exact capture layouts, captures prefix-monotone",
    "event binder slots and the resumed guard as a constrained Step against the unchanged boundary",
    "await entry charge and committed_before/committed_after/plan_total binding (logical-cost/2)",
    "stored cap as an explicit public slot; D-13 canonical reconstruction value stays split",
    "R7 canonical boundary predicates (states exact, balances strictly ascending, registry identities, trace bound)",
    "residual duties and present-zero versus absent across segments",
    "NotAdvanced (false guard) as a separate non-transaction relation with attempted work",
    "Emit and every effect kind in staged segments (the staged UNIT codec admits Emit)",
    "staged continuation codec correspondence (production codec 3b21... frozen; FREEZE-4 r2 result unadopted)",
    "authentication and one-time consumption of continuations and events (STAGE_LABELS: none/untracked)",
    "independent staged differential oracles and hostile witness mutations before any execution",
];

fn effect_kind(statement: &Statement) -> Option<String> {
    Some(match statement {
        Statement::Next { state, .. } => format!("State:{state}"),
        Statement::Emit { name, .. } => format!("Emit:{name}"),
        Statement::Transfer { from, to, .. } => format!("Transfer:{from}->{to}"),
        Statement::Fee { from, to, .. } => format!("Fee:{from}->{to}"),
        Statement::Liability { name, .. } => format!("Liability:{name}"),
        Statement::Discharge { name, .. } => format!("Liability(discharge):{name}"),
        Statement::Await { .. } | Statement::Let { .. } | Statement::Requires { .. } | Statement::Ensures { .. } => return None,
    })
}

impl StagedAdmission {
    pub(crate) fn new(staged: StagedProgram, source_sha256: Digest32) -> Self {
        Self { staged, source_sha256 }
    }

    pub fn staged(&self) -> &StagedProgram {
        &self.staged
    }

    pub fn source_sha256(&self) -> Digest32 {
        self.source_sha256
    }

    /// Derives every segment shape from the actual stage plan. No cost is recomputed
    /// here except the NotAdvanced dispatch charge (1 + guard nodes, `stage/cost.rs:149-162`).
    pub fn lowering_plan(&self) -> Result<StagedLoweringPlan, CompileError> {
        let program = self.staged.program();
        let stage_plan = self.staged.plan();
        let mut actions = Vec::with_capacity(stage_plan.actions.len());
        for (index, (action, ap)) in program.actions.iter().zip(&stage_plan.actions).enumerate() {
            if ap.action != action.name || ap.segment_cost.len() != ap.sites.len() + 1 || ap.committed_before.len() != ap.sites.len() + 1 {
                return Err(CompileError::Bound(format!("stage plan for {:?} is not the admitted plan", action.name)));
            }
            let action_index = u16::try_from(index).map_err(|_| CompileError::Bound("action index exceeds u16".into()))?;
            let inputs: Vec<CaptureSlot> = action
                .inputs
                .iter()
                .enumerate()
                .map(|(i, input)| CaptureSlot { name: input.name.clone(), ty: input.ty, origin: mori_core::stage::Origin::Input, ordinal: i as u16 })
                .collect();
            let mut segments = Vec::with_capacity(ap.sites.len() + 1);
            for segment in 0..=ap.sites.len() {
                let (entry, start) = if segment == 0 {
                    (SegmentEntry::Start { inputs: inputs.clone() }, 0u16)
                } else {
                    let site = &ap.sites[segment - 1];
                    (
                        SegmentEntry::Resume {
                            site: site.site,
                            label: site.label.clone(),
                            guard_ordinal: site.ordinal,
                            captures: site.captures.clone(),
                            binders: site.binders.clone(),
                        },
                        site.ordinal,
                    )
                };
                let (exit, end) = match ap.sites.get(segment) {
                    Some(next) => (
                        SegmentExit::Await { site: next.site, label: next.label.clone(), ordinal: next.ordinal, captures: next.captures.clone() },
                        next.ordinal,
                    ),
                    None => (SegmentExit::Complete, ap.statement_count),
                };
                let statements = action
                    .statements
                    .get(usize::from(start)..usize::from(end))
                    .ok_or_else(|| CompileError::Bound("segment range outside the action".into()))?;
                let effect_kinds: Vec<String> = statements.iter().filter_map(effect_kind).collect();
                let has_emit = statements.iter().any(|s| matches!(s, Statement::Emit { .. }));
                let not_advanced_attempted = match statements.first() {
                    Some(guard @ Statement::Await { .. }) if segment > 0 => Some(UInt(plan::statement_cost(program, guard)?)),
                    _ => None,
                };
                let before = ap.committed_before[segment];
                let cost = ap.segment_cost[segment];
                let after = before.0.checked_add(cost.0).map(UInt).ok_or_else(|| CompileError::Bound("committed work exceeds u128".into()))?;
                segments.push(StagedSegmentShape {
                    segment: segment as u16,
                    entry,
                    statement_start: start,
                    statement_end: end,
                    exit,
                    segment_cost: cost,
                    committed_before: before,
                    committed_after: after,
                    not_advanced_attempted,
                    effect_kinds,
                    has_emit,
                });
            }
            actions.push(StagedActionPlan {
                action: action.name.clone(),
                action_index,
                statement_count: ap.statement_count,
                plan_total: ap.plan_total,
                segments,
            });
        }
        Ok(StagedLoweringPlan {
            language: program.language.clone(),
            semantics: program.semantics.clone(),
            logical_cost_profile: mori_core::stage::LOGICAL_COST_PROFILE_2.into(),
            stage_plan_format: mori_core::stage::STAGE_PLAN_FORMAT.into(),
            continuation_format: mori_core::stage::CONTINUATION_FORMAT.into(),
            program_hash: Digest32(self.staged.program_hash()),
            plan_hash: Digest32(self.staged.plan_hash()),
            source_sha256: self.source_sha256,
            stored_cap: StoredCapParameter::ExplicitPublicSlot,
            actions,
            remaining: STAGED_REMAINING.iter().map(|s| (*s).to_string()).collect(),
        })
    }
}
