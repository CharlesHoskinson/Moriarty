//! Ordered local stage admission around the single shared evaluator.
//!
//! Admission is a pure function of its arguments. No consumption, nonce or
//! lock state exists here (P3): a copied continuation resumes again, and a
//! fabricated idle scenario starts again. Every rejection publishes no
//! segment, effects or continuation; the evaluator only ever sees a clone of
//! the boundary, so the caller's input stays valid for a corrected retry (C10).
use super::{
    CONTINUATION_FORMAT, Continuation, NamedValue, PendingFrame, STAGE_EVENT_FORMAT,
    StageEvent, StageLogical, StageOutcome, StageRejection, StageSegment, StagedProgram, cost,
    decode, digest, profile, residual,
};
use crate::runtime::{self, SegmentEntry, SegmentRun};
use crate::{Error, LOGICAL_COST_PROFILE_2, Scenario, UInt};

fn rejected(error: Error, attempted: UInt, committed: UInt) -> StageRejection {
    StageRejection {
        error,
        attempted,
        committed,
    }
}

/// Values must match the slot layout exactly in length, order, name and type.
fn layout_matches<'a, T: 'a>(
    values: &[NamedValue],
    slots: impl ExactSizeIterator<Item = &'a T>,
    shape: impl Fn(&'a T) -> (&'a str, crate::Type),
) -> bool {
    values.len() == slots.len()
        && values.iter().zip(slots).all(|(value, slot)| {
            let (name, ty) = shape(slot);
            value.name == name && value.value.ty() == ty
        })
}

/// A legitimate segment never exceeds its static cost, so a meter refusal
/// under that limit is the same H26 premise violation.
fn premise(error: Error) -> Error {
    if error.code == "mori::core::meter" {
        let mut violated = Error::new("stage_logical", "static cost premise violated");
        violated.step = error.step;
        violated
    } else {
        error
    }
}

/// H25: a Waiting continuation must decode back to itself through the actual
/// resume decoder and pass R7, or it is not emitted.
fn emit_checked(sp: &StagedProgram, action: usize, c: &Continuation) -> Result<(), Error> {
    let unresumable = || {
        Error::new(
            "bound",
            "Continuation exceeds the resume decoder limits or cannot be resumed.",
        )
    };
    let text = serde_json::to_string(c).map_err(|_| unresumable())?;
    if text.len() > decode::MAX_CONTINUATION_BYTES {
        return Err(unresumable());
    }
    let decoded = decode::continuation(&text).map_err(|_| unresumable())?;
    if decoded != *c {
        return Err(unresumable());
    }
    decode::canonical_boundary(sp, action, &decoded.captures, &decoded.boundary)
        .map_err(|_| unresumable())
}

/// An admitted segment: `entry` is its entry site (0 = start) and therefore
/// its segment index.
struct Admitted<'a> {
    sp: &'a StagedProgram,
    action: usize,
    entry: u16,
    /// Committed work reported on rejection: the unchanged input's value.
    committed: UInt,
    stored_cap: Option<UInt>,
    frame_pre: PendingFrame,
}

impl Admitted<'_> {
    fn finish(
        self,
        run: Result<SegmentRun, Error>,
        meter: &crate::policy::Meter,
        unchanged: Option<Continuation>,
    ) -> Result<StageOutcome, StageRejection> {
        let Admitted {
            sp,
            action: index,
            entry,
            committed: kept,
            stored_cap,
            frame_pre,
        } = self;
        let attempted = meter.used().logical;
        let fail = |error: Error| rejected(error, attempted, kept);
        let plan = &sp.plan().actions[index];
        let action = &sp.program().actions[index].name;
        let segment = usize::from(entry);
        match run.map_err(|error| fail(premise(error)))? {
            SegmentRun::NotAdvanced => {
                // Only a resume evaluates a guard; a false one changes nothing.
                let Some(c) = unchanged else {
                    return Err(fail(Error::new(
                        "stage_site",
                        "A start segment has no guard to observe.",
                    )));
                };
                let dispatch = cost::dispatch_cost(sp.program(), plan, entry).map_err(fail)?;
                cost::require_static(meter, dispatch).map_err(fail)?;
                emit_checked(sp, index, &c).map_err(fail)?;
                let owed = residual(&c.boundary);
                Ok(StageOutcome::Waiting {
                    advanced: false,
                    segment: None,
                    continuation: c,
                    attempted,
                    residual: owed,
                })
            }
            SegmentRun::Ran {
                execution,
                exit_site,
                captures_out,
            } => {
                let planned = *plan.segment_cost.get(segment).ok_or_else(|| {
                    fail(Error::new("stage_logical", "Segment is absent from the stage plan."))
                })?;
                cost::require_static(meter, planned).map_err(fail)?;
                let committed = cost::committed_after(plan, segment).map_err(fail)?;
                let expected_exit = if segment == plan.sites.len() {
                    0
                } else {
                    entry + 1
                };
                if exit_site != expected_exit {
                    return Err(fail(Error::new(
                        "stage_site",
                        "Segment exit differs from the stage plan.",
                    )));
                }
                let frame_post = PendingFrame {
                    site: exit_site,
                    captures: captures_out,
                };
                if exit_site == 0 {
                    if !frame_post.captures.is_empty() || committed != plan.plan_total {
                        return Err(fail(Error::new(
                            "stage_logical",
                            "Completed work differs from the action plan total.",
                        )));
                    }
                    let owed = residual(&execution.post);
                    return Ok(StageOutcome::Complete {
                        segment: StageSegment {
                            action: action.clone(),
                            frame_pre,
                            frame_post,
                            execution,
                        },
                        committed,
                        attempted,
                        residual: owed,
                    });
                }
                let site = &plan.sites[usize::from(exit_site) - 1];
                if !layout_matches(&frame_post.captures, site.captures.iter(), |s| {
                    (s.name.as_str(), s.ty)
                }) {
                    return Err(fail(Error::new(
                        "stage_capture",
                        "Segment captures differ from the next site layout.",
                    )));
                }
                if plan.committed_before.get(usize::from(exit_site)) != Some(&committed) {
                    return Err(fail(Error::new(
                        "stage_logical",
                        "Committed work differs from the stage plan.",
                    )));
                }
                let continuation = Continuation {
                    format: CONTINUATION_FORMAT.into(),
                    program_hash: digest::hex(&sp.program_hash()),
                    plan_hash: digest::hex(&sp.plan_hash()),
                    action: action.clone(),
                    site: exit_site,
                    captures: frame_post.captures.clone(),
                    logical: StageLogical {
                        profile: LOGICAL_COST_PROFILE_2.into(),
                        cap: stored_cap,
                        committed,
                    },
                    boundary: execution.post.clone(),
                };
                emit_checked(sp, index, &continuation).map_err(fail)?;
                let owed = residual(&execution.post);
                Ok(StageOutcome::Waiting {
                    advanced: true,
                    segment: Some(StageSegment {
                        action: action.clone(),
                        frame_pre,
                        frame_post,
                        execution,
                    }),
                    continuation,
                    attempted,
                    residual: owed,
                })
            }
        }
    }
}

/// Runs segment 0 of `action`. The scenario passes the existing
/// `scenario_snapshot`, then the plan-total cap preflight (C3, attempted 0).
/// The pre frame is locally assumed idle (P3); `cap` becomes the stored cap.
pub fn start(
    sp: &StagedProgram,
    action: &str,
    scenario: Scenario,
    cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection> {
    let refuse = |error: Error| rejected(error, UInt(0), UInt(0));
    let index = sp
        .action_index(action)
        .ok_or_else(|| refuse(Error::new("action", format!("Unknown action {action:?}."))))?;
    let selected = &sp.program().actions[index];
    let plan = sp
        .plan()
        .actions
        .get(index)
        .filter(|plan| plan.action == action)
        .ok_or_else(|| refuse(Error::new("stage_site", "Action is absent from the stage plan.")))?;
    let pre = runtime::scenario_snapshot(sp.program(), selected, &scenario).map_err(refuse)?;
    let limit = cost::start_preflight(plan, cap).map_err(refuse)?;
    let mut inputs = Vec::with_capacity(selected.inputs.len());
    for input in &selected.inputs {
        let value = scenario.inputs.get(&input.name).cloned().ok_or_else(|| {
            refuse(Error::new(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ))
        })?;
        inputs.push(NamedValue {
            name: input.name.clone(),
            value,
        });
    }
    // A boundary that R7 would refuse must not start an awaiting action:
    // its continuation could never be resumed (H25).
    if !plan.sites.is_empty() {
        decode::canonical_boundary(sp, index, &inputs, &pre).map_err(refuse)?;
    }
    let mut meter = cost::segment_meter(limit);
    let run = runtime::run_segment(
        sp,
        index,
        SegmentEntry::Start { inputs: &inputs },
        pre,
        &mut meter,
    );
    Admitted {
        sp,
        action: index,
        entry: 0,
        committed: UInt(0),
        stored_cap: cap,
        frame_pre: PendingFrame::idle(),
    }
    .finish(run, &meter, None)
}

/// Resumes `c` with event `e`, applying R0 then R1-R7 in order. The first
/// failure rejects with attempted 0 and the input's committed value. A false
/// guard returns the input continuation unchanged, with its stored cap; the
/// CLI cap only checks this invocation.
pub fn resume(
    sp: &StagedProgram,
    c: Continuation,
    e: StageEvent,
    cli_action: Option<&str>,
    cli_cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection> {
    // Echoes the unchanged input; it is unverified when R1-R6 refuse.
    let kept = c.logical.committed;
    let refuse = |code: &str, message: &str| rejected(Error::new(code, message), UInt(0), kept);
    let refused = |error: Error| rejected(error, UInt(0), kept);
    // R0: an explicitly selected action must be the continuation's action.
    if cli_action.is_some_and(|selected| selected != c.action) {
        return Err(refuse(
            "stage_site",
            "Selected action differs from the continuation action.",
        ));
    }
    // R1
    if c.format != CONTINUATION_FORMAT || c.logical.profile != LOGICAL_COST_PROFILE_2 {
        return Err(refuse(
            "stage_format",
            "Unsupported continuation format or logical profile.",
        ));
    }
    // R2
    profile::require_core2(sp.program()).map_err(refused)?;
    if !digest::matches(&c.program_hash, &sp.program_hash())
        || !digest::matches(&c.plan_hash, &sp.plan_hash())
    {
        return Err(refuse(
            "stage_program",
            "Continuation does not belong to this program and stage plan.",
        ));
    }
    // R3
    let Some(index) = sp.action_index(&c.action) else {
        return Err(refuse("stage_site", "Continuation action does not exist."));
    };
    let Some(plan) = sp.plan().actions.get(index).filter(|plan| plan.action == c.action) else {
        return Err(refuse("stage_site", "Action is absent from the stage plan."));
    };
    if c.site == 0 || usize::from(c.site) > plan.sites.len() {
        return Err(refuse(
            "stage_site",
            "Continuation site is not a pending await of this action.",
        ));
    }
    let site = &plan.sites[usize::from(c.site) - 1];
    // R4
    if !layout_matches(&c.captures, site.captures.iter(), |s| (s.name.as_str(), s.ty)) {
        return Err(refuse(
            "stage_capture",
            "Captures differ from the pending site layout.",
        ));
    }
    // R5
    if e.format != STAGE_EVENT_FORMAT
        || e.action != c.action
        || e.label != site.label
        || !layout_matches(&e.values, site.binders.iter(), |b| (b.name.as_str(), b.ty))
    {
        return Err(refuse(
            "stage_event",
            "Event does not match the pending await.",
        ));
    }
    // R6
    let limit = cost::resume_preflight(plan, c.site, &c.logical, cli_cap).map_err(refused)?;
    // R7
    decode::canonical_boundary(sp, index, &c.captures, &c.boundary).map_err(refused)?;
    let mut meter = cost::segment_meter(limit);
    let run = runtime::run_segment(
        sp,
        index,
        SegmentEntry::Resume {
            site: c.site,
            captures: &c.captures,
            event: &e.values,
        },
        c.boundary.clone(),
        &mut meter,
    );
    Admitted {
        sp,
        action: index,
        entry: c.site,
        committed: kept,
        stored_cap: c.logical.cap,
        frame_pre: PendingFrame {
            site: c.site,
            captures: c.captures.clone(),
        },
    }
    .finish(run, &meter, Some(c))
}
