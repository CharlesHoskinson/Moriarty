//! families::adt_helper -- proposed source for all 12 join14 ADT helper rows.
//! The owner-produced SelectedAdt routes pure payload construction. The owner
//! retains admission, selection, guards, StepInv, charge, caps, Apply and events.
//! Exact interface: shared_adt.proposed.rs, sha256
//! 8dc9687741fbdac180eaff65dcc674caafbc3e3cb52e2e70fd376bc3e09b7f6b.
//! Source preparation only: uncompiled, uninstalled and unadopted.
use crate::sherlock::helper_machine::shared::{
    ControlC, EnvCellC, HostViolation, HostViolationKind, PlanDelta, RefC, SliceC,
    TriStatus, Write,
};
use crate::sherlock::helper_machine::shared::adt::{
    adt_pure, AdtEffectPlan, AdtFamilyError, AdtGuardEvaluator, AdtGuardId,
    AdtHelperC, AdtHelperCommon, AdtHelperFamilyPlans, AdtHelperResultC,
    AdtHelperRuleC, AdtHelperSite, AdtPlannedWrites, AdtPreImageView, AdtRowFault,
    AdtRowFaultKind, BuildHelperC, BuildPhase, ControlExtC, PlanHelperC,
    PlanNodeRowC, PlanPhase, SelectedAdt,
};

pub struct AdtHelperFamilies;

fn foreign_rule() -> AdtFamilyError {
    AdtFamilyError::Host(HostViolation {
        kind: HostViolationKind::WrongVariant,
        clause: None,
        rule: None,
        locator: Some("foreign rule for family".to_string()),
    })
}

impl AdtHelperFamilyPlans for AdtHelperFamilies {
    fn plan_adt_build(
        &self,
        sel: &SelectedAdt,
        pre: &dyn AdtPreImageView,
        g: &dyn AdtGuardEvaluator,
    ) -> Result<AdtEffectPlan, AdtFamilyError> {
        match sel.rule {
            AdtHelperRuleC::BuildEntry => build_entry(pre),
            AdtHelperRuleC::BuildCell => build_cell(pre),
            AdtHelperRuleC::BuildField => build_field(pre, g),
            AdtHelperRuleC::BuildExit => build_exit(pre),
            AdtHelperRuleC::PlanEntry
            | AdtHelperRuleC::PlanWalk
            | AdtHelperRuleC::PlanSwitch
            | AdtHelperRuleC::PlanBind
            | AdtHelperRuleC::PlanSelect
            | AdtHelperRuleC::PlanNoMatch
            | AdtHelperRuleC::PlanExit
            | AdtHelperRuleC::PlanFailure => Err(foreign_rule()),
        }
    }

    fn plan_adt_plan(
        &self,
        sel: &SelectedAdt,
        pre: &dyn AdtPreImageView,
    ) -> Result<AdtEffectPlan, AdtFamilyError> {
        match sel.rule {
            AdtHelperRuleC::PlanEntry => plan_entry(pre),
            AdtHelperRuleC::PlanWalk => plan_walk(pre),
            AdtHelperRuleC::PlanSwitch => plan_switch(pre),
            AdtHelperRuleC::PlanBind => plan_bind(pre),
            AdtHelperRuleC::PlanSelect => plan_select(pre),
            AdtHelperRuleC::PlanNoMatch => plan_nomatch(pre),
            AdtHelperRuleC::PlanExit => plan_exit(pre),
            AdtHelperRuleC::PlanFailure => plan_failure(pre),
            AdtHelperRuleC::BuildEntry
            | AdtHelperRuleC::BuildCell
            | AdtHelperRuleC::BuildField
            | AdtHelperRuleC::BuildExit => Err(foreign_rule()),
        }
    }
}

fn row_fault(kind: AdtRowFaultKind, at: &'static str) -> AdtFamilyError {
    AdtFamilyError::Fault(AdtRowFault { kind, at })
}

fn build_helper(pre: &dyn AdtPreImageView) -> Result<BuildHelperC, AdtFamilyError> {
    match pre.adt_helper() {
        Some(AdtHelperC::Build(h)) => Ok(*h),
        Some(AdtHelperC::Plan(_)) => Err(row_fault(AdtRowFaultKind::InactiveVariant, "h")),
        None => Err(row_fault(AdtRowFaultKind::UndefinedRoot, "h")),
    }
}

fn plan_helper(pre: &dyn AdtPreImageView) -> Result<PlanHelperC, AdtFamilyError> {
    match pre.adt_helper() {
        Some(AdtHelperC::Plan(h)) => Ok(*h),
        Some(AdtHelperC::Build(_)) => Err(row_fault(AdtRowFaultKind::InactiveVariant, "h")),
        None => Err(row_fault(AdtRowFaultKind::UndefinedRoot, "h")),
    }
}

// extension-rows.json#/adt_helper_rows/0: x.adt.helper.Build.entry.
fn build_entry(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let (type_id, ctor_id, src) = if let ControlExtC::Materialize {
        type_id, ctor_id, fields,
    } = pre.control_ext() {
        (*type_id, *ctor_id, *fields)
    } else {
        return Err(row_fault(AdtRowFaultKind::InactiveVariant, "c"));
    };
    let origin_reference_cursor = pre.reference_cursor();
    let field_start = pre.active().data_fields;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Build(BuildHelperC {
                common: AdtHelperCommon {
                    site: AdtHelperSite::Materialize,
                    origin_reference_cursor,
                    parent_resume: None,
                },
                type_id,
                ctor_id,
                src,
                src_cursor: 0,
                field_start,
                staged: None,
                node: None,
                phase: BuildPhase::Copy,
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/1: x.adt.helper.Build.cell.
fn build_cell(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = build_helper(pre)?;
    let value = pre.vslot(&h.src, &h.src_cursor)?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Build(BuildHelperC {
                staged: Some(value),
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/2: x.adt.helper.Build.field.
fn build_field(
    pre: &dyn AdtPreImageView,
    g: &dyn AdtGuardEvaluator,
) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = build_helper(pre)?;
    let staged = h.staged.ok_or_else(|| {
        row_fault(AdtRowFaultKind::UnwrapAbsent, "unwrap(h.staged)")
    })?;
    let next_cursor = h.src_cursor.checked_add(1).ok_or_else(|| {
        row_fault(AdtRowFaultKind::HostIndexOverflow, "h.srcCursor + 1")
    })?;
    // One owner result supplies both the delta add and the identical effect branch.
    let last = g.eval_adt_guard(&AdtGuardId::BuildLast, pre)?;
    let active = pre.active();
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta {
            alloc_data_fields: 1,
            alloc_data_nodes: if last { 1 } else { 0 },
            ..PlanDelta::default()
        },
        writes: AdtPlannedWrites {
            alloc_field: Some(staged),
            alloc_constructor: if last {
                Some(crate::sherlock::helper_machine::shared::adt::ConstructorNodeWrite {
                    type_id: h.type_id,
                    ctor_id: h.ctor_id,
                    fields: SliceC::normalized(h.field_start, h.src.len),
                })
            } else {
                None
            },
            helper: Write::Set(Some(AdtHelperC::Build(BuildHelperC {
                src_cursor: next_cursor,
                staged: None,
                node: if last { Some(RefC::Data(active.data)) } else { h.node },
                phase: if last { BuildPhase::Done } else { h.phase },
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/3: x.adt.helper.Build.exit.
fn build_exit(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = build_helper(pre)?;
    let node = h.node.ok_or_else(|| {
        row_fault(AdtRowFaultKind::UnwrapAbsent, "unwrap(h.node)")
    })?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(None),
            result: Some(AdtHelperResultC::BuildDone { value: node }),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/4: x.adt.helper.Plan.entry.
fn plan_entry(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let (value, saved_env) = if let ControlExtC::Core(ControlC::Cases {
        value, saved_env, ..
    }) = pre.control_ext() {
        (*value, *saved_env)
    } else {
        return Err(row_fault(AdtRowFaultKind::InactiveVariant, "c"));
    };
    let origin = pre.control_origin();
    let n0 = pre.match_plan_root(&origin)?.ok_or_else(|| {
        row_fault(
            AdtRowFaultKind::UnwrapAbsent,
            "unwrap(expr(pre.controlOrigin).plan)",
        )
    })?;
    let nd0 = *pre.plan_node(&n0)?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Plan(PlanHelperC {
                common: AdtHelperCommon {
                    site: AdtHelperSite::Cases,
                    origin_reference_cursor: pre.reference_cursor(),
                    parent_resume: None,
                },
                match_expr: origin,
                scrutinee: value,
                saved_env,
                partial: saved_env,
                node: n0,
                phase: adt_pure::load_phase(&nd0),
                after_walk: adt_pure::load_after(&nd0),
                cur: value,
                path: adt_pure::load_path(&nd0),
                path_cursor: 0,
                arm: adt_pure::load_arm(&nd0),
                status: TriStatus::Pending,
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/5: x.adt.helper.Plan.walk.
fn plan_walk(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = plan_helper(pre)?;
    let pos = h.path.start.checked_add(h.path_cursor).ok_or_else(|| {
        row_fault(AdtRowFaultKind::HostIndexOverflow, "h.path.start + h.pathCursor")
    })?;
    let idx = pre.field_path_cell(&pos)?;
    let cur = pre.field_ext(&h.cur, &idx)?;
    let next_cursor = h.path_cursor.checked_add(1).ok_or_else(|| {
        row_fault(AdtRowFaultKind::HostIndexOverflow, "h.pathCursor + 1")
    })?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Plan(PlanHelperC {
                cur,
                path_cursor: next_cursor,
                phase: if next_cursor == h.path.len {
                    adt_pure::after_walk_phase(h.after_walk)
                } else {
                    h.phase
                },
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/6: x.adt.helper.Plan.switch.
fn plan_switch(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = plan_helper(pre)?;
    let nd = *pre.plan_node(&h.node)?;
    let kind = pre.kind_ext(&h.cur)?;
    let (type_id, default_node) = if let PlanNodeRowC::Switch {
        type_id, default_node, ..
    } = nd {
        (type_id, default_node)
    } else {
        return Err(row_fault(AdtRowFaultKind::InactiveVariant, "nd.typeId"));
    };
    // This type-row fetch is unconditional in reads[2]. The case read is guarded.
    let ty = *pre.type_row(&type_id)?;
    let hit = if adt_pure::ctor_class(&kind)
        && adt_pure::type_of_kind(&kind) == Some(type_id)
    {
        let slot = adt_pure::case_slot(&nd, &kind, &ty)?;
        pre.plan_case(&slot)?
    } else {
        None
    };
    let next = hit.unwrap_or(default_node);
    let ndx = *pre.plan_node(&next)?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Plan(PlanHelperC {
                node: next,
                cur: h.scrutinee,
                path: adt_pure::load_path(&ndx),
                path_cursor: 0,
                after_walk: adt_pure::load_after(&ndx),
                arm: adt_pure::load_arm(&ndx),
                phase: adt_pure::load_phase(&ndx),
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/7: x.adt.helper.Plan.bind.
fn plan_bind(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = plan_helper(pre)?;
    let nd = *pre.plan_node(&h.node)?;
    let (binder, next) = if let PlanNodeRowC::Bind { binder, next, .. } = nd {
        (binder, next)
    } else {
        return Err(row_fault(AdtRowFaultKind::InactiveVariant, "nd.binder"));
    };
    let ndn = *pre.plan_node(&next)?;
    let cell = pre.active().env;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta {
            alloc_environment_cells: 1,
            ..PlanDelta::default()
        },
        writes: AdtPlannedWrites {
            alloc_env: Some(EnvCellC {
                binder,
                value: h.cur,
                previous: h.partial,
            }),
            helper: Write::Set(Some(AdtHelperC::Plan(PlanHelperC {
                partial: Some(cell),
                node: next,
                cur: h.scrutinee,
                path: adt_pure::load_path(&ndn),
                path_cursor: 0,
                after_walk: adt_pure::load_after(&ndn),
                arm: adt_pure::load_arm(&ndn),
                phase: adt_pure::load_phase(&ndn),
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/8: x.adt.helper.Plan.select.
fn plan_select(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = plan_helper(pre)?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Plan(PlanHelperC {
                status: TriStatus::Positive,
                phase: PlanPhase::Done,
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/9: x.adt.helper.Plan.nomatch.
fn plan_nomatch(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = plan_helper(pre)?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(Some(AdtHelperC::Plan(PlanHelperC {
                status: TriStatus::Negative,
                phase: PlanPhase::Done,
                ..h
            }))),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/10: x.adt.helper.Plan.exit.
fn plan_exit(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let h = plan_helper(pre)?;
    let arm = h.arm.ok_or_else(|| {
        row_fault(AdtRowFaultKind::UnwrapAbsent, "unwrap(h.arm)")
    })?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(None),
            result: Some(AdtHelperResultC::PlanMatched { arm, env: h.partial }),
            ..AdtPlannedWrites::default()
        },
    })
}

// extension-rows.json#/adt_helper_rows/11: x.adt.helper.Plan.failure.
fn plan_failure(pre: &dyn AdtPreImageView) -> Result<AdtEffectPlan, AdtFamilyError> {
    let _h = plan_helper(pre)?;
    Ok(AdtEffectPlan {
        delta_assert: PlanDelta::default(),
        writes: AdtPlannedWrites {
            helper: Write::Set(None),
            result: Some(AdtHelperResultC::PlanNoMatch),
            ..AdtPlannedWrites::default()
        },
    })
}
