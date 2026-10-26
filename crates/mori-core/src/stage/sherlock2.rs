//! Profile `sherlock-helper-microstep/2` financial route over a staged
//! `moriarty/2` action (suspension design note §7, §8 L1).
//!
//! One XFin owner event per statement, through the same scalar statement
//! executor as Core/2 (`runtime::execute_statement`), so balances, liabilities,
//! effects and logical work agree with `stage::start`/`stage::resume` on the
//! same program and inputs (U018). The image keeps the committed snapshot and
//! effect log, the retained env and the work counters; tentative state lives
//! only inside one invocation, so no image exists with a Waiting control over
//! an uncommitted delta (§7.3) and SI-25 holds by construction.
//!
//! - `XFinAwaitEnter` commits the segment and publishes `Waiting{s}` in one event.
//! - `XFinResumeAccept` evaluates the guard over (captures, observation) and,
//!   only on `true`, starts segment `s` from the committed snapshot.
//! - `XFinLocalFailure` publishes `Rejected{code}`: the committed prefix is
//!   kept, the tentative effects become the discarded delta, and the failed
//!   segment's consumed work stays charged. Under `retain-guaranteed-fees/1`
//!   a sealed fee phase is committed first (§8 L1); a failing fee phase
//!   retains nothing.
//! - `XFinComplete` commits and publishes `Halted`.
//!
//! Host semantics only; the C6 `FinImageState` carrier and its circuits are
//! later work.
use super::{NamedValue, StageLogical, StagedProgram, cost, decode, digest, residual};
use crate::policy::Meter;
use crate::runtime;
use crate::{
    Effect, Error, LOGICAL_COST_PROFILE_2, Liability, Scenario, Snapshot, Statement, UInt, Value,
};
use serde::{Deserialize, Serialize};

pub const POLICY_ALL_OR_NOTHING: &str = "all-or-nothing/1";
pub const POLICY_RETAIN_FEES: &str = "retain-guaranteed-fees/1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum FinOutcome2 {
    Running,
    Waiting { site: u16 },
    Halted,
    Rejected { code: String, message: String },
    Exhausted { reason: String },
}

/// The complete /2 financial image carried by a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinImage2 {
    pub program_hash: String,
    pub plan_hash: String,
    pub action: String,
    pub policy: String,
    pub outcome: FinOutcome2,
    /// Index of the segment that runs next (or failed / last ran).
    pub segment: usize,
    /// Captures of the pending site (Waiting only), in site capture order.
    pub captures: Vec<NamedValue>,
    pub committed: Snapshot,
    pub committed_effects: Vec<Effect>,
    pub committed_logical: UInt,
    pub initial_fuel: UInt,
    pub remaining_work: UInt,
    pub consumed_work: UInt,
    /// Terminal-only diagnostics (SI-26): the discarded tentative delta and
    /// the fees a retain policy committed on local failure.
    pub discarded_effects: Vec<Effect>,
    pub retained_by_policy: Vec<Effect>,
}

/// Admission refusal (no image exists).
#[derive(Debug, Clone)]
pub struct FinAdmitError {
    pub error: Error,
}

fn err(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}

pub fn policy_of(selected: Option<&str>) -> Result<&'static str, Error> {
    match selected {
        None | Some(POLICY_ALL_OR_NOTHING) => Ok(POLICY_ALL_OR_NOTHING),
        Some(POLICY_RETAIN_FEES) => Ok(POLICY_RETAIN_FEES),
        Some(other) => Err(err(
            "phase_policy",
            format!("Unknown phase policy {other:?}."),
        )),
    }
}

/// Statement ranges `[start, end)` of each segment of `action`.
fn segments(sp: &StagedProgram, action: usize) -> Vec<(usize, usize)> {
    let plan = &sp.plan().actions[action];
    let n = sp.program().actions[action].statements.len();
    let mut out = Vec::new();
    let mut start = 0;
    for site in &plan.sites {
        let o = usize::from(site.ordinal);
        out.push((start, o));
        start = o + 1;
    }
    out.push((start, n));
    out
}

/// `retain-guaranteed-fees/1` admission: in every segment each `Fee`
/// precedes every effect statement other than `Let`/`Requires`.
pub fn check_policy(sp: &StagedProgram, action: usize, policy: &str) -> Result<(), Error> {
    if policy != POLICY_RETAIN_FEES {
        return Ok(());
    }
    let statements = &sp.program().actions[action].statements;
    for (start, end) in segments(sp, action) {
        let mut fallible = false;
        for (o, s) in statements[start..end].iter().enumerate() {
            match s {
                Statement::Fee { .. } if fallible => {
                    return Err(err(
                        "phase_policy",
                        format!(
                            "Fee at statement {} follows a fallible effect in its segment; \
                             retain-guaranteed-fees/1 requires fees first.",
                            start + o
                        ),
                    ));
                }
                Statement::Fee { .. } | Statement::Let { .. } | Statement::Requires { .. } => {}
                _ => fallible = true,
            }
        }
    }
    Ok(())
}

/// End (exclusive) of the leading guaranteed fee phase of `[start, end)`.
fn fee_end(statements: &[Statement], start: usize, end: usize) -> usize {
    statements[start..end]
        .iter()
        .rposition(|s| matches!(s, Statement::Fee { .. }))
        .map_or(start, |i| start + i + 1)
}

pub struct FinStart {
    pub image: FinImage2,
    /// Logical work attempted by this invocation (host diagnostic).
    pub attempted: UInt,
}

/// Genesis plus segment 0: `admit_fin` analogue followed by the XFin run.
pub fn start(
    sp: &StagedProgram,
    action: &str,
    scenario: Scenario,
    policy: &str,
    fuel: u128,
) -> Result<FinStart, FinAdmitError> {
    let refuse = |error: Error| FinAdmitError { error };
    let index = sp
        .action_index(action)
        .ok_or_else(|| refuse(err("action", format!("Unknown action {action:?}."))))?;
    check_policy(sp, index, policy).map_err(refuse)?;
    let selected = &sp.program().actions[index];
    let plan = &sp.plan().actions[index];
    let pre = runtime::scenario_snapshot(sp.program(), selected, &scenario).map_err(refuse)?;
    let mut inputs = Vec::with_capacity(selected.inputs.len());
    for input in &selected.inputs {
        let value = scenario.inputs.get(&input.name).cloned().ok_or_else(|| {
            refuse(err(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ))
        })?;
        inputs.push(NamedValue {
            name: input.name.clone(),
            value,
        });
    }
    if !plan.sites.is_empty() {
        decode::canonical_boundary(sp, index, &inputs, &pre).map_err(refuse)?;
    }
    let mut image = FinImage2 {
        program_hash: digest::hex(&sp.program_hash()),
        plan_hash: digest::hex(&sp.plan_hash()),
        action: action.into(),
        policy: policy.into(),
        outcome: FinOutcome2::Running,
        segment: 0,
        captures: Vec::new(),
        committed: pre.clone(),
        committed_effects: Vec::new(),
        committed_logical: UInt(0),
        initial_fuel: UInt(fuel),
        remaining_work: UInt(fuel),
        consumed_work: UInt(0),
        discarded_effects: Vec::new(),
        retained_by_policy: Vec::new(),
    };
    let mut env: std::collections::BTreeMap<String, Value> = pre.state.clone();
    env.extend(inputs.iter().map(|v| (v.name.clone(), v.value.clone())));
    let limit = *plan
        .segment_cost
        .first()
        .ok_or_else(|| refuse(err("stage_logical", "Stage plan has no segment costs.")))?;
    let mut meter = cost::segment_meter(limit);
    run_segment(sp, index, &mut image, env, 0, &mut meter);
    Ok(FinStart {
        attempted: meter.used().logical,
        image,
    })
}

fn charge(image: &mut FinImage2) -> bool {
    if image.remaining_work.0 == 0 {
        return false;
    }
    image.remaining_work.0 -= 1;
    image.consumed_work.0 += 1;
    true
}

/// Publish a local failure or exhaustion of the running segment (§7.5, §8).
fn fail(
    image: &mut FinImage2,
    outcome: FinOutcome2,
    fee_stage: Option<(Snapshot, Vec<Effect>)>,
    fee_effects: Vec<Effect>,
    fallible: Vec<Effect>,
) {
    match fee_stage {
        Some((snapshot, fees)) if image.policy == POLICY_RETAIN_FEES => {
            image.committed = snapshot;
            image.committed_effects.extend(fees.iter().cloned());
            image.retained_by_policy = fees;
            image.discarded_effects = fallible;
        }
        _ => {
            let mut discarded = fee_effects;
            discarded.extend(fallible);
            image.discarded_effects = discarded;
        }
    }
    image.captures.clear();
    image.outcome = outcome;
}

/// Run segment `image.segment` from `begin` with `env` until it awaits,
/// completes, fails or exhausts. `image` must be Running.
fn run_segment(
    sp: &StagedProgram,
    action: usize,
    image: &mut FinImage2,
    mut env: std::collections::BTreeMap<String, Value>,
    begin: usize,
    meter: &mut Meter,
) {
    let program = sp.program();
    let plan = &sp.plan().actions[action];
    let statements = &program.actions[action].statements;
    let segment = image.segment;
    let (_, end) = segments(sp, action)[segment];
    let seal_at = if image.policy == POLICY_RETAIN_FEES {
        fee_end(statements, begin, end)
    } else {
        begin
    };
    let mut tentative = image.committed.clone();
    let mut fee_effects: Vec<Effect> = Vec::new();
    let mut fallible: Vec<Effect> = Vec::new();
    let mut fee_stage: Option<(Snapshot, Vec<Effect>)> =
        (seal_at == begin).then(|| (tentative.clone(), Vec::new()));
    let mut meter = Some(meter);
    for ordinal in begin..end {
        if fee_stage.is_none() && ordinal == seal_at {
            fee_stage = Some((tentative.clone(), fee_effects.clone()));
        }
        if !charge(image) {
            let reason = FinOutcome2::Exhausted {
                reason: "Fuel".into(),
            };
            return fail(image, reason, fee_stage, fee_effects, fallible);
        }
        let mut effects = Vec::new();
        let result = runtime::execute_statement(
            program,
            &statements[ordinal],
            &mut env,
            &mut tentative,
            &mut meter,
            &mut effects,
        );
        if let Err(error) = result {
            let outcome = FinOutcome2::Rejected {
                code: error.code.clone(),
                message: error.message.clone(),
            };
            if fee_stage.is_none() {
                fee_effects.extend(effects);
            } else {
                fallible.extend(effects);
            }
            return fail(image, outcome, fee_stage, fee_effects, fallible);
        }
        if fee_stage.is_none() {
            fee_effects.extend(effects);
        } else {
            fallible.extend(effects);
        }
    }
    if fee_stage.is_none() {
        fee_stage = Some((tentative.clone(), fee_effects.clone()));
    }
    // Commit point: XFinAwaitEnter or XFinComplete, one event.
    if !charge(image) {
        let reason = FinOutcome2::Exhausted {
            reason: "Fuel".into(),
        };
        return fail(image, reason, fee_stage, fee_effects, fallible);
    }
    if end < statements.len() && image.remaining_work.0 == 0 {
        // SI-29 (fin): a Waiting image keeps the fuel of its resume charge.
        let reason = FinOutcome2::Exhausted {
            reason: "Fuel".into(),
        };
        return fail(image, reason, fee_stage, fee_effects, fallible);
    }
    if end < statements.len() {
        // Await entry dispatch is attempted now; its guard belongs to resume.
        let entry = meter
            .as_mut()
            .map(|m| m.charge(crate::policy::CostPlane::Logical, UInt(1)))
            .transpose();
        if entry.is_err() {
            return fail(
                image,
                FinOutcome2::Exhausted {
                    reason: "LogicalCap".into(),
                },
                fee_stage,
                fee_effects,
                fallible,
            );
        }
    }
    let used = meter.as_ref().map(|m| m.used().logical);
    let planned = plan.segment_cost.get(segment).copied();
    let committed = cost::committed_after(plan, segment);
    let (Some(used), Some(planned), Ok(committed)) = (used, planned, committed) else {
        return fail(
            image,
            FinOutcome2::Rejected {
                code: "stage_logical".into(),
                message: "Segment is absent from the stage plan.".into(),
            },
            None,
            fee_effects,
            fallible,
        );
    };
    if used != planned {
        return fail(
            image,
            FinOutcome2::Rejected {
                code: "stage_logical".into(),
                message: "static cost premise violated".into(),
            },
            None,
            fee_effects,
            fallible,
        );
    }
    image.committed = tentative;
    image.committed_effects.extend(fee_effects);
    image.committed_effects.extend(fallible);
    image.committed_logical = committed;
    if end == statements.len() {
        image.captures.clear();
        image.outcome = FinOutcome2::Halted;
        return;
    }
    let site = &plan.sites[segment];
    let mut captures = Vec::with_capacity(site.captures.len());
    for slot in &site.captures {
        match env.get(&slot.name) {
            Some(value) => captures.push(NamedValue {
                name: slot.name.clone(),
                value: value.clone(),
            }),
            None => {
                image.outcome = FinOutcome2::Rejected {
                    code: "name".into(),
                    message: format!("Unknown name {:?}.", slot.name),
                };
                return;
            }
        }
    }
    image.captures = captures;
    image.segment = segment + 1;
    image.outcome = FinOutcome2::Waiting { site: site.site };
}

/// R2: the checkpoint's admission identity against the supplied program.
pub fn same_admission(sp: &StagedProgram, image: &FinImage2, policy: &str) -> bool {
    digest::matches(&image.program_hash, &sp.program_hash())
        && digest::matches(&image.plan_hash, &sp.plan_hash())
        && image.policy == policy
        && sp.action_index(&image.action).is_some()
}

/// The pending site of a Waiting image: (label, binder layout).
pub fn waiting_site(
    sp: &StagedProgram,
    image: &FinImage2,
) -> Option<(u16, String, Vec<(String, crate::Type)>)> {
    let FinOutcome2::Waiting { site } = image.outcome else {
        return None;
    };
    let index = sp.action_index(&image.action)?;
    let s = sp.plan().actions[index]
        .sites
        .get(usize::from(site).checked_sub(1)?)?;
    Some((
        site,
        s.label.clone(),
        s.binders.iter().map(|b| (b.name.clone(), b.ty)).collect(),
    ))
}

pub enum FinResume {
    /// The guard did not return true; nothing changed. `guard_error` names
    /// the evaluation error code when the guard failed rather than returned.
    NotReady {
        attempted: UInt,
        guard_error: Option<String>,
    },
    Advanced {
        image: FinImage2,
        attempted: UInt,
    },
}

/// R6, R7, then `XFinResumeAccept` and the next segment.
pub fn resume(
    sp: &StagedProgram,
    image: &FinImage2,
    values: Vec<NamedValue>,
) -> Result<FinResume, (&'static str, Error)> {
    let FinOutcome2::Waiting { site } = image.outcome else {
        return Err(("R1", err("stage_site", "Checkpoint is not Waiting.")));
    };
    let index = sp
        .action_index(&image.action)
        .ok_or(("R2", err("stage_site", "Unknown action.")))?;
    let plan = &sp.plan().actions[index];
    let logical = StageLogical {
        profile: LOGICAL_COST_PROFILE_2.into(),
        cap: None,
        committed: image.committed_logical,
    };
    if image.remaining_work.0 == 0 {
        // SI-29: refused as malformed before the guard runs.
        return Err((
            "R0",
            err(
                "stage_site",
                "Waiting image violates the resumability invariant SI-29.",
            ),
        ));
    }
    let limit = cost::resume_preflight(plan, site, &logical, None).map_err(|e| ("R6", e))?;
    decode::canonical_boundary(sp, index, &image.captures, &image.committed)
        .map_err(|e| ("R7", e))?;
    let program = sp.program();
    let ordinal = usize::from(plan.sites[usize::from(site) - 1].ordinal);
    let mut env = image.committed.state.clone();
    env.extend(
        image
            .captures
            .iter()
            .chain(&values)
            .map(|v| (v.name.clone(), v.value.clone())),
    );
    let mut meter = cost::segment_meter(limit);
    let mut scratch = image.committed.clone();
    let mut effects = Vec::new();
    let guard = runtime::execute_statement(
        program,
        &program.actions[index].statements[ordinal],
        &mut env,
        &mut scratch,
        &mut Some(&mut meter),
        &mut effects,
    );
    match guard {
        Ok(Some(Value::Bool(true))) => {}
        Ok(_) => {
            return Ok(FinResume::NotReady {
                attempted: meter.used().logical,
                guard_error: None,
            });
        }
        Err(e) => {
            return Ok(FinResume::NotReady {
                attempted: meter.used().logical,
                guard_error: Some(e.code),
            });
        }
    }
    let mut next = image.clone();
    if !charge(&mut next) {
        // SI-29 analogue: unreachable for a Waiting image the entry admitted.
        next.outcome = FinOutcome2::Exhausted {
            reason: "Fuel".into(),
        };
        return Ok(FinResume::Advanced {
            image: next,
            attempted: meter.used().logical,
        });
    }
    next.outcome = FinOutcome2::Running;
    next.captures.clear();
    run_segment(sp, index, &mut next, env, ordinal + 1, &mut meter);
    Ok(FinResume::Advanced {
        attempted: meter.used().logical,
        image: next,
    })
}

/// Positive residual duties of the committed snapshot.
pub fn residual_duties(image: &FinImage2) -> Vec<Liability> {
    residual(&image.committed)
}

/// Accounting over the committed log only: gross = payments + fees.
/// `None` when a sum exceeds UInt128 (reported as an error, never saturated).
pub fn accounting(effects: &[Effect]) -> Option<(u128, u128, u128)> {
    let mut payments = 0u128;
    let mut fees = 0u128;
    for e in effects {
        match e {
            Effect::Transfer { amount, .. } => payments = payments.checked_add(amount.0)?,
            Effect::Fee { amount, .. } => fees = fees.checked_add(amount.0)?,
            _ => {}
        }
    }
    Some((payments.checked_add(fees)?, fees, payments))
}
