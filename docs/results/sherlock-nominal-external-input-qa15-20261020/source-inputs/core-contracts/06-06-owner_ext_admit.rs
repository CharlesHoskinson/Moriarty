pub fn admit_ext_program(
    program: ExtProgram,
    domain: ExtDomain,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    locator: Option<LocatorC>,
) -> Result<(Admitted, PreparationReportC), ExtTypeBuildError> {
    // AD-0a: layout.bounds == bounds.
    if layout.bounds != *bounds {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
            reason: AdmissionReason::LayoutBoundsMismatch,
            detail: "layout.bounds != bounds".to_string(),
            locator: field_loc("layout.bounds"),
        }));
    }
    // AD-1: stepCap >= 1.
    if bounds.step_cap < 1 {
        return Err(ExtTypeBuildError::CoreTooling(
            Tooling::CapacityNotAdmitted {
                kind: CapacityKind::StepCap,
                needed: NeededNatC::Exact(1),
                limit: bounds.step_cap,
                locator: field_loc("bounds.stepCap"),
            },
        ));
    }
    let ext_caps = ExtCapacitiesC::effective_for_extended_domain(layout.ext_capacities);
    // AD-2a & AD-2b: widths.
    if let Some(w) = &layout.widths {
        let checks: [(CounterWidthField, u128, &str); 3] = [
            (
                CounterWidthField::WorkCounter,
                bitlen_count(bounds.max_reference_work),
                "WorkCounter",
            ),
            (
                CounterWidthField::HelperCounter,
                bitlen_count(bounds.max_helper_work),
                "HelperCounter",
            ),
            (
                CounterWidthField::ReferenceCursor,
                bitlen_count(bounds.max_reference_work),
                "ReferenceCursor",
            ),
        ];
        for (field, needed, nm) in checks {
            let limit = w.counter_width(&field) as u128;
            if limit < needed {
                return Err(ExtTypeBuildError::CoreTooling(
                    Tooling::CapacityNotAdmitted {
                        kind: CapacityKind::CounterWidth { field },
                        needed: NeededNatC::Exact(needed),
                        limit,
                        locator: field_loc(&format!("layout.widths.counterWidths.{nm}")),
                    },
                ));
            }
        }
        let idx: [(&str, u128); 29] = [
            ("ExprId", bitlen_id(bounds.code_rows as u128)),
            ("ChildPos", bitlen_count(bounds.code_child_slots as u128)),
            ("FunctionRowId", bitlen_id(bounds.function_rows as u128)),
            ("FunctionCursor", bitlen_count(bounds.function_rows as u128)),
            ("ParamPos", bitlen_count(bounds.parameter_slots as u128)),
            ("ArmId", bitlen_id(bounds.match_arm_rows as u128)),
            ("ArmPos", bitlen_count(bounds.match_arm_rows as u128)),
            ("PatternRowId", bitlen_id(bounds.pattern_rows as u128)),
            ("ConstId", bitlen_id(bounds.constant_nodes as u128)),
            ("ConstFieldPos", bitlen_count(bounds.constant_field_slots as u128)),
            ("DataId", bitlen_id(bounds.data_nodes as u128)),
            ("FieldPos", bitlen_count(bounds.data_field_slots as u128)),
            ("EnvId", bitlen_id(bounds.environment_cells as u128)),
            ("NameId", bitlen_id(layout.capacities.name_rows as u128)),
            (
                "OccPathPos",
                bitlen_count(layout.capacities.occurrence_path_slots as u128),
            ),
            (
                "ChildPosition",
                bitlen_count(
                    (bounds.max_arity as u128).max(bounds.match_arm_rows as u128 + 1),
                ),
            ),
            ("ValueSlotPos", bitlen_count(layout.capacities.value_slots as u128)),
            ("ArityCount", bitlen_count(bounds.max_arity as u128)),
            ("Depth", bitlen_count(bounds.frames as u128)),
            ("TaskCount", bitlen_count(bounds.helper_tasks as u128)),
            ("TypeId", bitlen_id(bounds.type_rows as u128)),
            ("CtorId", bitlen_id(bounds.constructor_rows as u128)),
            (
                "CtorFieldPos",
                bitlen_count(bounds.constructor_field_slots as u128),
            ),
            ("PlanNodeId", bitlen_id(bounds.match_plan_rows as u128)),
            ("PlanCasePos", bitlen_count(ext_caps.plan_case_slots as u128)),
            ("FieldPathPos", bitlen_count(ext_caps.field_path_slots as u128)),
            (
                "PatternChildPos",
                bitlen_count(ext_caps.pattern_child_slots as u128),
            ),
            ("DeclIdx", bitlen_id(bounds.type_rows as u128)),
            ("TypeArgPos", bitlen_count(bounds.type_rows as u128)),
        ];
        for (nm, needed) in idx {
            let limit = w.index_width(nm) as u128;
            if limit < needed {
                return Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
                    reason: AdmissionReason::LayoutWidthTooSmall,
                    detail: nm.to_string(),
                    locator: field_loc(&format!("layout.widths.indexWidths.{nm}")),
                }));
            }
        }
        return Err(ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
            reason: AdmissionReason::LayoutWidthTooSmall,
            detail: "recordWidths".to_string(),
            locator: field_loc("layout.widths.recordWidths"),
        }));
    }
    // AD-3: maxArity >= 2.
    if bounds.max_arity < 2 {
        return Err(ExtTypeBuildError::CoreTooling(
            Tooling::CapacityNotAdmitted {
                kind: CapacityKind::MaxArity,
                needed: NeededNatC::Exact(2),
                limit: bounds.max_arity as u128,
                locator: field_loc("bounds.maxArity"),
            },
        ));
    }
    // prepareExt: Pass B -> Pass D -> Pass T -> Pass C1..C6 -> Pass P.
    let (view, report) = prepare_ext_program(program, domain, bounds, layout, locator)?;
    // AD-5 (G-11): check_program_view_ext (V1..V17).
    check_program_view_ext(&view, Some(&report), domain, bounds, layout)?;
    // AD-6: boundary capacities must be absent.
    if layout.boundary_capacities_present {
        return Err(ExtTypeBuildError::CoreTooling(Tooling::UnsupportedFeature {
            feature: FeatureKind::BoundarySection,
            locator: field_loc("layout.boundaryCapacities"),
        }));
    }
    let identity = IdentityC {
        profile_id: 2,
        grammar_domain: domain.grammar_domain,
        finance: domain.finance,
        layout_version: layout.layout_version.clone(),
        layout_digest: DigestC::Unselected,
        bounds_digest: DigestC::Unselected,
        field_id: layout.field_id.clone(),
        program_membership_digest: DigestC::Unselected,
    };
    let admitted = Admitted {
        body: Arc::new(AdmittedBody {
            identity,
            bounds: *bounds,
            view,
            layout: layout.clone(),
        }),
    };
    Ok((admitted, report))
}
