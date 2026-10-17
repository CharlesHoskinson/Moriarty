// PROVISIONAL private free-function source fragment: all six R4 Scalar rows.
// Root supplies pinned shared v3a names in eventual scope and delegates from
// its sole FamilyPlans implementation; placement and imports remain Root-owned.
// Selected is authoritative. All effect predicates are the literal R4 row
// branches; the three guarded allocation deltas call the owner evaluator.
// No selection, admission, charge, cap, plan validation or machine mutation.
// Compilation, Rust tests and native execution are UNRUN.

fn scalar_payload_error(kind: HostViolationKind, locator: &str) -> HostViolation {
    HostViolation { kind, clause: None, rule: None, locator: Some(locator.to_owned()) }
}

fn scalar_helper(pre: &dyn PreImageView) -> Result<HelperControlC, HostViolation> {
    match pre.helper() {
        Some(h @ HelperControlC::Scalar { .. }) => Ok(*h),
        _ => Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar helper payload")),
    }
}

fn scalar_operand_b(operand_b: Option<RefC>) -> Result<RefC, HostViolation> {
    operand_b.ok_or_else(|| scalar_payload_error(HostViolationKind::WrongVariant, "Scalar operandB payload"))
}

fn plan_scalar(sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator)
    -> Result<EffectPlan, HostViolation>
{
    let mut writes = PlannedWrites::default();
    let mut delta = PlanDelta::default();
    match &sel.rule {
        // ROW b.helper.Scalar.entry
        RuleIdC::BHelperScalarEntry => {
            let site = sel.site.ok_or_else(|| scalar_payload_error(
                HostViolationKind::WrongVariant, "Scalar selected site payload"))?;
            let value = match pre.control() { ControlC::Return { value } => *value,
                _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar Return payload")) };
            let (mode, operand_a, operand_b, expected) = match site {
                SiteTag::RetRight => {
                    let (op, left) = match pre.top_frame() { Some(FrameRowC::RightF { op, left, .. }) => (*op, *left),
                        _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar RightF payload")) };
                    let mode = match op {
                        BinaryOp::Add => ScalarMode::Add, BinaryOp::Sub => ScalarMode::Sub,
                        BinaryOp::Mul => ScalarMode::Mul, BinaryOp::Lt => ScalarMode::Lt,
                        BinaryOp::Cons => ScalarMode::Cons, BinaryOp::Branch => ScalarMode::Branch,
                        BinaryOp::Eq => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar binary mode payload")),
                    };
                    (mode, left, Some(value), None)
                }
                SiteTag::RetUnary => {
                    let op = match pre.top_frame() { Some(FrameRowC::UnaryF { op, .. }) => *op,
                        _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar UnaryF payload")) };
                    let mode = match op { UnaryOp::Leaf => ScalarMode::Leaf, UnaryOp::Not => ScalarMode::Not };
                    (mode, value, None, None)
                }
                SiteTag::RetIf => (ScalarMode::TestBool, value, None, None),
                SiteTag::RetCall => {
                    let return_type = match pre.top_frame() { Some(FrameRowC::ReturnF { return_type, .. }) => *return_type,
                        _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar ReturnF payload")) };
                    (ScalarMode::TestType, value, None, Some(return_type))
                }
                _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar selected site payload")),
            };
            writes.helper = Write::Set(Some(HelperControlC::Scalar {
                common: HelperCommon { site, origin_reference_cursor: pre.reference_cursor(), parent_resume: None },
                mode, operand_a, operand_b, expected, phase: ScalarPhase::First,
                reg_a: 0, staged: ScalarStaged::None,
            }));
        }
        // ROW b.helper.Scalar.tag
        RuleIdC::BHelperScalarTag => {
            let mut h = scalar_helper(pre)?;
            if let HelperControlC::Scalar { mode, operand_a, expected, reg_a, staged, phase, .. } = &mut h {
                match *mode {
                    ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt => {
                        if pre.kind_of(operand_a)? == LogicalKind::UInt {
                            *reg_a = pre.u128_of(operand_a)?;
                            *phase = ScalarPhase::Second;
                        } else {
                            *staged = ScalarStaged::PrimError { code: 11 };
                            *phase = ScalarPhase::Done;
                        }
                    }
                    ScalarMode::Cons => {
                        if pre.kind_of(operand_a)? == LogicalKind::UInt {
                            *phase = ScalarPhase::Second;
                        } else {
                            *staged = ScalarStaged::PrimError { code: 11 };
                            *phase = ScalarPhase::Done;
                        }
                    }
                    ScalarMode::Branch => {
                        if matches!(pre.kind_of(operand_a)?, LogicalKind::Leaf | LogicalKind::Branch) {
                            *phase = ScalarPhase::Second;
                        } else {
                            *staged = ScalarStaged::PrimError { code: 11 };
                            *phase = ScalarPhase::Done;
                        }
                    }
                    ScalarMode::TestBool => {
                        let sel = if pre.kind_of(operand_a)? == LogicalKind::Bool {
                            if pre.bool_of(operand_a)? { BoolSelector::True } else { BoolSelector::False }
                        } else { BoolSelector::NotBool };
                        *staged = ScalarStaged::Selector { sel };
                        *phase = ScalarPhase::Done;
                    }
                    ScalarMode::TestType => {
                        let expected = expected.as_ref().ok_or_else(|| scalar_payload_error(
                            HostViolationKind::WrongVariant, "Scalar expected sort payload"))?;
                        *staged = ScalarStaged::TypeTest { ok: pre.scalar_test_type_ok(operand_a, expected)? };
                        *phase = ScalarPhase::Done;
                    }
                    ScalarMode::Leaf | ScalarMode::Not => return Err(scalar_payload_error(
                        HostViolationKind::WrongVariant, "Scalar.tag mode payload")),
                }
            }
            writes.helper = Write::Set(Some(h));
        }
        // ROW b.helper.Scalar.arithmetic
        RuleIdC::BHelperScalarArithmetic => {
            let mut h = scalar_helper(pre)?;
            if let HelperControlC::Scalar { mode, operand_a, operand_b, reg_a, staged, phase, .. } = &mut h {
                *staged = match *mode {
                    ScalarMode::Not => {
                        if pre.kind_of(operand_a)? == LogicalKind::Bool {
                            ScalarStaged::BoolValue { b: !pre.bool_of(operand_a)? }
                        } else { ScalarStaged::PrimError { code: 11 } }
                    }
                    ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt => {
                        let b = scalar_operand_b(*operand_b)?;
                        if pre.kind_of(&b)? != LogicalKind::UInt {
                            ScalarStaged::PrimError { code: 11 }
                        } else {
                            let n = pre.u128_of(&b)?;
                            if *mode == ScalarMode::Lt {
                                ScalarStaged::BoolValue { b: *reg_a < n }
                            } else {
                                let result = match *mode {
                                    ScalarMode::Add => pure_leaves::checked_add(*reg_a, n),
                                    ScalarMode::Sub => pure_leaves::checked_sub(*reg_a, n),
                                    ScalarMode::Mul => pure_leaves::checked_mul(*reg_a, n),
                                    _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "checked arithmetic mode")),
                                };
                                match result {
                                    pure_leaves::CheckedArith::Value { n } => ScalarStaged::U128Value { n },
                                    pure_leaves::CheckedArith::Overflow => ScalarStaged::PrimError { code: 10 },
                                }
                            }
                        }
                    }
                    _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar.arithmetic mode")),
                };
                *phase = ScalarPhase::Done;
            }
            writes.helper = Write::Set(Some(h));
        }
        // ROW b.helper.Scalar.construct
        RuleIdC::BHelperScalarConstruct => {
            let mut h = scalar_helper(pre)?;
            if let HelperControlC::Scalar { mode, operand_a, operand_b, staged, phase, .. } = &mut h {
                *staged = match *mode {
                    ScalarMode::Leaf => {
                        if pre.kind_of(operand_a)? == LogicalKind::UInt {
                            ScalarStaged::PendingNode { tag: NodeKind::Leaf }
                        } else { ScalarStaged::PrimError { code: 11 } }
                    }
                    ScalarMode::Cons => {
                        let b = scalar_operand_b(*operand_b)?;
                        if matches!(pre.kind_of(&b)?, LogicalKind::Nil | LogicalKind::Cons) {
                            ScalarStaged::PendingNode { tag: NodeKind::Cons }
                        } else { ScalarStaged::PrimError { code: 11 } }
                    }
                    ScalarMode::Branch => {
                        let b = scalar_operand_b(*operand_b)?;
                        if matches!(pre.kind_of(&b)?, LogicalKind::Leaf | LogicalKind::Branch) {
                            ScalarStaged::PendingNode { tag: NodeKind::Branch }
                        } else { ScalarStaged::PrimError { code: 11 } }
                    }
                    _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar.construct mode")),
                };
                *phase = ScalarPhase::Done;
            }
            writes.helper = Write::Set(Some(h));
        }
        // ROW b.helper.Scalar.exit
        RuleIdC::BHelperScalarExit => {
            // Delta predicates are owner-owned, evaluated in the R4 when-list order.
            delta.alloc_data_nodes = if g.eval_guard(&GuardId::GScalarAllocNode, pre)? { 1 } else { 0 };
            let fields_one = if g.eval_guard(&GuardId::GScalarFields1, pre)? { 1 } else { 0 };
            let fields_two = if g.eval_guard(&GuardId::GScalarFields2, pre)? { 2 } else { 0 };
            delta.alloc_data_fields = fields_one + fields_two;
            let h = scalar_helper(pre)?;
            let (operand_a, operand_b, staged) = match h {
                HelperControlC::Scalar { operand_a, operand_b, staged, .. } => (operand_a, operand_b, staged),
                _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar helper payload")),
            };
            let result = match staged {
                ScalarStaged::U128Value { n } => {
                    writes.alloc_data = Some(AllocDataWrite::U128 { n });
                    HelperResultC::ScalarValue { value: RefC::Data(pre.active().data) }
                }
                ScalarStaged::BoolValue { b } => {
                    writes.alloc_data = Some(AllocDataWrite::Bool { b });
                    HelperResultC::ScalarValue { value: RefC::Data(pre.active().data) }
                }
                ScalarStaged::PendingNode { tag } => {
                    writes.alloc_data = Some(match tag {
                        NodeKind::Leaf => AllocDataWrite::Leaf { field: operand_a },
                        NodeKind::Cons => AllocDataWrite::Cons { fields: [operand_a, scalar_operand_b(operand_b)?] },
                        NodeKind::Branch => AllocDataWrite::Branch { fields: [operand_a, scalar_operand_b(operand_b)?] },
                    });
                    HelperResultC::ScalarValue { value: RefC::Data(pre.active().data) }
                }
                ScalarStaged::Selector { sel } => HelperResultC::ScalarSelector { sel },
                ScalarStaged::TypeTest { ok } => HelperResultC::ScalarTypeTest { ok },
                ScalarStaged::None | ScalarStaged::PrimError { .. } => return Err(scalar_payload_error(
                    HostViolationKind::WrongVariant, "Scalar exit staged payload")),
            };
            writes.result = Write::Set(Some(result));
            writes.helper = Write::Set(None);
        }
        // ROW b.helper.Scalar.failure
        RuleIdC::BHelperScalarFailure => {
            let h = scalar_helper(pre)?;
            let code = match h { HelperControlC::Scalar { staged: ScalarStaged::PrimError { code }, .. } => code,
                _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "Scalar failure payload")) };
            writes.result = Write::Set(Some(HelperResultC::ScalarError { code }));
            writes.helper = Write::Set(None);
        }
        _ => return Err(scalar_payload_error(HostViolationKind::WrongVariant, "foreign rule for family")),
    }
    Ok(EffectPlan { delta_assert: delta, writes })
}
