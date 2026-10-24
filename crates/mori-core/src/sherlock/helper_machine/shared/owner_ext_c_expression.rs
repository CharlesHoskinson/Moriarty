use super::super::raw::{
    ConstFieldPos as RawConstFieldPos, ConstFieldSlice as RawConstFieldSlice,
    ConstRow as RawConstRow, CtorId as RawCtorId, TypeId as RawTypeId,
};
use super::adt::{CtorIdC, TypeIdC};
use super::ext_program_c::{
    ExtCContinuation, ExtCProgramWriter, LiteralOccurrence, MatchOccurrence,
    NullaryConstructOccurrence, lower_ext_c1_to_c5,
};
use super::ext_program_c6::ExtC6ProgramWriter;
use super::ext_program_p::{
    ExtPProgramWriter, PatternBindingC, PlanArmSourceC, PlanVisitKindC, lower_ext_c1_to_p,
};
use super::owner_ext_admit::{
    ExtCapacitiesC, PreparationReportC, WorkMapEntryC, clone_type_decls_iterative,
};
use super::owner_ext_c_slots::{
    append_ids_bounded, fill_reserved_once, reserve_pair, reserve_slot,
};
use super::owner_ext_t3_uses::{BodyOwner, ExtExpr, ExtPattern, ExtValue, T3UseSite};
use super::owner_ext_type_builder::{ExtTypeBuildError, ExtTypeCapKind, ExtTypeRejectReason};
use super::{
    AdmissionReason, ArmRowC, BoundsC, CapacityKind, CorePattern, CoreValue, ExpressionRowC,
    FeatureKind, FunctionRowC, HostViolation, HostViolationKind, LayoutSchemaC, LocatorC,
    NeededNatC, OccurrenceRowC, ParameterRowC, PatternRowC, PlanNodeC, PreparedExtProgramSource,
    ProgramView, RuleIdC, SliceC, Tooling,
};

fn host_expr_err(kind: HostViolationKind, detail: &'static str) -> ExtTypeBuildError {
    ExtTypeBuildError::Host(HostViolation {
        kind,
        clause: None,
        rule: None,
        locator: Some(detail.to_string()),
    })
}

fn expected_child_count(expr: &ExtExpr) -> Result<usize, ExtTypeBuildError> {
    match expr {
        ExtExpr::Let { .. } | ExtExpr::Binary { .. } | ExtExpr::Await { .. } => Ok(2),
        ExtExpr::Unary { .. } | ExtExpr::Fin { .. } => Ok(1),
        ExtExpr::If { .. } => Ok(3),
        ExtExpr::Call { args, .. } => Ok(args.len()),
        ExtExpr::Construct { fields, .. } => Ok(fields.len()),
        ExtExpr::Match { arms, .. } => arms.len().checked_add(1).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "match child count overflow",
            )
        }),
        ExtExpr::Literal { .. }
        | ExtExpr::Var { .. }
        | ExtExpr::StateRead { .. }
        | ExtExpr::LiabilityPresent { .. }
        | ExtExpr::LiabilityAmount { .. } => Ok(0),
    }
}

fn canonical_child_expr<'s>(parent: &'s ExtExpr, child_position: usize) -> Option<&'s ExtExpr> {
    match parent {
        ExtExpr::Let { value, body, .. } => match child_position {
            0 => Some(value.as_ref()),
            1 => Some(body.as_ref()),
            _ => None,
        },
        ExtExpr::Binary { left, right, .. } => match child_position {
            0 => Some(left.as_ref()),
            1 => Some(right.as_ref()),
            _ => None,
        },
        ExtExpr::Unary { operand, .. } | ExtExpr::Fin { operand, .. } => match child_position {
            0 => Some(operand.as_ref()),
            _ => None,
        },
        ExtExpr::If {
            cond,
            then_expr,
            else_expr,
        } => match child_position {
            0 => Some(cond.as_ref()),
            1 => Some(then_expr.as_ref()),
            2 => Some(else_expr.as_ref()),
            _ => None,
        },
        ExtExpr::Call { args, .. } => args.get(child_position),
        ExtExpr::Construct { fields, .. } => fields.get(child_position),
        ExtExpr::Match { scrutinee, arms } => {
            if child_position == 0 {
                Some(scrutinee.as_ref())
            } else {
                arms.get(child_position - 1).map(|arm| &arm.body)
            }
        }
        ExtExpr::Await { guard, body, .. } => match child_position {
            0 => Some(guard.as_ref()),
            1 => Some(body.as_ref()),
            _ => None,
        },
        ExtExpr::Literal { .. }
        | ExtExpr::Var { .. }
        | ExtExpr::StateRead { .. }
        | ExtExpr::LiabilityPresent { .. }
        | ExtExpr::LiabilityAmount { .. } => None,
    }
}

fn core_stage_cap_err(kind: CapacityKind, current_len: usize, limit: usize) -> ExtTypeBuildError {
    let needed = match (current_len as u128).checked_add(1) {
        Some(n) => NeededNatC::Exact(n),
        None => NeededNatC::AboveU128,
    };
    ExtTypeBuildError::CoreTooling(Tooling::CapacityNotAdmitted {
        kind,
        needed,
        limit: limit as u128,
        locator: None,
    })
}

fn ext_stage_cap_err(kind: ExtTypeCapKind, current_len: usize, limit: usize) -> ExtTypeBuildError {
    let needed = match (current_len as u128).checked_add(1) {
        Some(n) => NeededNatC::Exact(n),
        None => NeededNatC::AboveU128,
    };
    ExtTypeBuildError::ExtCapacityNotAdmitted {
        kind,
        needed,
        limit: limit as u128,
        locator: None,
    }
}

fn pattern_node_at_preorder<'a>(
    root: &'a ExtPattern,
    target_ordinal: usize,
) -> Result<Option<&'a ExtPattern>, ExtTypeBuildError> {
    let mut stack: Vec<&'a ExtPattern> = vec![root];
    let mut current_ordinal: usize = 0;
    while let Some(node) = stack.pop() {
        if current_ordinal == target_ordinal {
            return Ok(Some(node));
        }
        current_ordinal = current_ordinal.checked_add(1).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "pattern preorder ordinal overflow",
            )
        })?;
        if let ExtPattern::Ctor { fields, .. } = node {
            for field in fields.iter().rev() {
                stack.push(field);
            }
        }
    }
    Ok(None)
}

enum ExpectedPatternStep<'s> {
    Enter {
        parent: Option<usize>,
        source: &'s ExtPattern,
    },
    Exit {
        ordinal: usize,
    },
}

struct EnteredPatternNode<'s> {
    source: &'s ExtPattern,
    parent: Option<usize>,
    expected_children: Vec<usize>,
    emitted_id: Option<usize>,
}

struct ActiveArmPatternState<'s> {
    arm_id: usize,
    match_expr_id: usize,
    owner: BodyOwner,
    local_expr: usize,
    arm_index: usize,
    arm_start_patterns_len: usize,
    visit_stack: Vec<ExpectedPatternStep<'s>>,
    entered_nodes: Vec<EnteredPatternNode<'s>>,
    root_pattern_id: Option<usize>,
}

struct ActiveFunctionState {
    function_id: usize,
    parameter_start: usize,
    expected_arity: usize,
    appended_params: usize,
}

enum ExpectedValueStep<'s> {
    Enter {
        parent: Option<usize>,
        source: &'s ExtValue,
    },
    Exit {
        ordinal: usize,
    },
}

struct EnteredValueNode<'s> {
    source: &'s ExtValue,
    parent: Option<usize>,
    expected_children: Vec<usize>,
    emitted_const_id: Option<usize>,
}

struct ActiveLiteralState<'s> {
    expr_id: usize,
    owner: BodyOwner,
    local_expr: usize,
    const_start_len: usize,
    visit_stack: Vec<ExpectedValueStep<'s>>,
    entered_nodes: Vec<EnteredValueNode<'s>>,
    root_const_id: Option<usize>,
}

struct ActiveNullaryConstructState {
    expr_id: usize,
    type_id: TypeIdC,
    ctor_id: CtorIdC,
    emitted_const_id: Option<usize>,
}

#[derive(Clone, Copy)]
enum PendingPlanNodeKind {
    Switch {
        path: SliceC,
        type_id: TypeIdC,
        ctor_start: CtorIdC,
        cases: SliceC,
        last_case_offset: Option<usize>,
        finished: bool,
    },
    Bind {
        path: SliceC,
        binder: usize,
        finished: bool,
    },
    Select {
        arm: usize,
    },
    NoMatch,
}

struct PendingPlanNodeState {
    source_match: usize,
    has_incoming_parent: bool,
    kind: PendingPlanNodeKind,
}

struct ActiveMatchPlanState<'s> {
    match_expr_id: usize,
    owner: BodyOwner,
    local_expr: usize,
    arm_rows: SliceC,
    source_arms: &'s [super::owner_ext_t3_uses::ExtArm],
    arms_checked: usize,
    plan_block_start: usize,
}

pub(super) struct BoundExprSource<'s> {
    pub(super) owner: BodyOwner,
    pub(super) local_expr: usize,
    pub(super) parent: Option<(usize, usize)>,
    pub(super) source: &'s ExtExpr,
    pub(super) expected_children: usize,
    pub(super) next_expected_child: usize,
    pub(super) code_children_slice: Option<SliceC>,
    pub(super) child_slice_linked: bool,
    pub(super) match_arms_slice: Option<SliceC>,
    pub(super) occurrence_path_slice: Option<SliceC>,
}

pub(super) struct PendingExtCExpressionStage<'s> {
    pub(super) prepared: &'s PreparedExtProgramSource,
    pub(super) bounds: BoundsC,
    pub(super) layout: &'s LayoutSchemaC,
    pub(super) ext_capacities: ExtCapacitiesC,
    pub(super) pattern_child_slots: usize,
    pub(super) expressions: Vec<Option<ExpressionRowC>>,
    pub(super) occurrences: Vec<Option<OccurrenceRowC>>,
    pub(super) expr_bindings: Vec<BoundExprSource<'s>>,
    pub(super) children: Vec<usize>,
    pub(super) occurrence_paths: Vec<usize>,
    pub(super) arms: Vec<Option<ArmRowC>>,
    pub(super) patterns: Vec<PatternRowC>,
    pub(super) pattern_children: Vec<usize>,
    pub(super) functions: Vec<Option<FunctionRowC>>,
    pub(super) parameters: Vec<ParameterRowC>,
    pub(super) constants: Vec<RawConstRow>,
    pub(super) constant_fields: Vec<usize>,
    pub(super) plan_nodes: Vec<Option<PlanNodeC>>,
    pub(super) plan_cases: Vec<Option<usize>>,
    pub(super) field_paths: Vec<usize>,
    pub(super) work_map: Vec<WorkMapEntryC>,
    pub(super) entry_root_id: Option<usize>,
    pub(super) function_root_ids: Vec<Option<usize>>,
    pub(super) entry_next_local: usize,
    pub(super) function_next_local: Vec<usize>,
    pub(super) next_occurrence_id: usize,
    active_arm: Option<ActiveArmPatternState<'s>>,
    current_match_expr_id: Option<usize>,
    current_match_arm_ids: Vec<usize>,
    active_function: Option<ActiveFunctionState>,
    active_literal: Option<ActiveLiteralState<'s>>,
    active_nullary_construct: Option<ActiveNullaryConstructState>,
    active_match_plan: Option<ActiveMatchPlanState<'s>>,
    pending_plan_nodes: Vec<PendingPlanNodeState>,
    p_visited_matches: Vec<usize>,
}

pub(super) struct PendingExtC<'s> {
    pub(super) stage: PendingExtCExpressionStage<'s>,
    pub(super) continuation: ExtCContinuation<'s>,
}

/// Literal-slot ownership for the stage checks. C1..C5 own unfilled Literal
/// slots; C6 binds them in ascending ExprId order; C6 completion and P read
/// only bound Literal rows. The C2..C5 structural constraints do not vary.
#[derive(Clone, Copy, PartialEq, Eq)]
enum LiteralSlotPhase {
    BeforeC6,
    DuringC6,
    AfterC6,
}

impl LiteralSlotPhase {
    fn wrong_literal_slot(self) -> &'static str {
        match self {
            LiteralSlotPhase::BeforeC6 => "literal slot must remain None prior to C6",
            LiteralSlotPhase::DuringC6 => {
                "literal slot is neither unfilled nor a C6 Literal row during C6"
            }
            LiteralSlotPhase::AfterC6 => "C6 literal expression row not filled",
        }
    }
}

impl<'s> PendingExtCExpressionStage<'s> {
    pub(super) fn new_checked(
        prepared: &'s PreparedExtProgramSource,
        layout: &'s LayoutSchemaC,
        pattern_child_slots: usize,
    ) -> Result<Self, ExtTypeBuildError> {
        let mut ext_capacities = layout
            .ext_capacities
            .unwrap_or_else(ExtCapacitiesC::all_zero);
        ext_capacities.pattern_child_slots = pattern_child_slots;
        Self::new_checked_with_capacities(prepared, &layout.bounds, layout, ext_capacities)
    }

    pub(super) fn new_checked_with_capacities(
        prepared: &'s PreparedExtProgramSource,
        bounds: &BoundsC,
        layout: &'s LayoutSchemaC,
        ext_capacities: ExtCapacitiesC,
    ) -> Result<Self, ExtTypeBuildError> {
        if layout.boundary_capacities_present {
            return Err(ExtTypeBuildError::CoreTooling(
                Tooling::UnsupportedFeature {
                    feature: FeatureKind::BoundarySection,
                    locator: None,
                },
            ));
        }
        if !prepared.domain.is_finance_absent() {
            return Err(ExtTypeBuildError::CoreTooling(
                Tooling::UnsupportedFeature {
                    feature: FeatureKind::FinancialInput,
                    locator: None,
                },
            ));
        }
        if !prepared.decl_name_ids.is_owner_verified() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "unverified Pass B declaration name table",
            ));
        }
        let name_count = prepared.interned_names.len();
        if prepared.decl_name_ids.name_count != name_count
            || prepared.tables.name_count != name_count
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "prepared name count mismatch",
            ));
        }
        let decl_count = prepared.program.types.len();
        if prepared.decl_name_ids.decls.len() != decl_count
            || prepared.tables.decl_param_counts.len() != decl_count
            || prepared.tables.decl_ctor_counts.len() != decl_count
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "prepared declaration table count mismatch",
            ));
        }
        for pair in prepared.interned_names.windows(2) {
            if pair[0].as_str() >= pair[1].as_str() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "interned names not strictly sorted",
                ));
            }
        }
        let fn_count = prepared.program.functions.len();
        Ok(Self {
            prepared,
            bounds: *bounds,
            layout,
            ext_capacities,
            pattern_child_slots: ext_capacities.pattern_child_slots,
            expressions: Vec::new(),
            occurrences: Vec::new(),
            expr_bindings: Vec::new(),
            children: Vec::new(),
            occurrence_paths: Vec::new(),
            arms: Vec::new(),
            patterns: Vec::new(),
            pattern_children: Vec::new(),
            functions: Vec::new(),
            parameters: Vec::new(),
            constants: Vec::new(),
            constant_fields: Vec::new(),
            plan_nodes: Vec::new(),
            plan_cases: Vec::new(),
            field_paths: Vec::new(),
            work_map: Vec::new(),
            entry_root_id: None,
            function_root_ids: vec![None; fn_count],
            entry_next_local: 0,
            function_next_local: vec![0; fn_count],
            next_occurrence_id: 0,
            active_arm: None,
            current_match_expr_id: None,
            current_match_arm_ids: Vec::new(),
            active_function: None,
            active_literal: None,
            active_nullary_construct: None,
            active_match_plan: None,
            pending_plan_nodes: Vec::new(),
            p_visited_matches: Vec::new(),
        })
    }

    fn check_stage_alignment(&self) -> Result<(), ExtTypeBuildError> {
        if self.expressions.len() != self.occurrences.len()
            || self.expressions.len() != self.expr_bindings.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "unaligned expression stage tables",
            ));
        }
        Ok(())
    }

    fn check_all_expressions_reserved(&self) -> Result<(), ExtTypeBuildError> {
        self.check_stage_alignment()?;
        if self.entry_root_id.is_none()
            || self.function_root_ids.len() != self.prepared.program.functions.len()
            || !self.function_root_ids.iter().all(Option::is_some)
            || !self
                .expr_bindings
                .iter()
                .all(|b| b.next_expected_child == b.expected_children)
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "C1 expression reservation incomplete",
            ));
        }
        Ok(())
    }

    fn check_path_matches_bound_ancestors(
        &self,
        id: usize,
        path: &[usize],
    ) -> Result<(), ExtTypeBuildError> {
        let mut cursor = id;
        for &step in path.iter().rev() {
            let node = self.expr_bindings.get(cursor).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "occurrence ancestor index out of bounds",
                )
            })?;
            match node.parent {
                Some((parent_id, child_pos)) if child_pos == step && parent_id < cursor => {
                    cursor = parent_id;
                }
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "occurrence path slice does not match bound parent chain",
                    ));
                }
            }
        }
        let root_node = self.expr_bindings.get(cursor).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "occurrence root index out of bounds",
            )
        })?;
        if root_node.parent.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "occurrence path does not terminate at owner root",
            ));
        }
        Ok(())
    }

    fn check_direct_child(
        &self,
        parent_id: usize,
        owner: BodyOwner,
        child_pos: usize,
        child_id: usize,
    ) -> Result<(), ExtTypeBuildError> {
        let child = self.expr_bindings.get(child_id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "child expression id out of bounds",
            )
        })?;
        if child_id <= parent_id
            || child.owner != owner
            || child.parent != Some((parent_id, child_pos))
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "child expression binding mismatch",
            ));
        }
        Ok(())
    }

    pub(super) fn reserve_expression(
        &mut self,
        source: &ExtExpr,
        owner: BodyOwner,
        parent: Option<(usize, usize)>,
    ) -> Result<(usize, usize), ExtTypeBuildError> {
        self.check_stage_alignment()?;
        let canonical_source: &'s ExtExpr = match parent {
            None => {
                if !self
                    .expr_bindings
                    .iter()
                    .all(|b| b.next_expected_child == b.expected_children)
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "prior expression subtree reservation incomplete before root reservation",
                    ));
                }
                match owner {
                    BodyOwner::Entry => {
                        if self.entry_root_id.is_some() || !self.expr_bindings.is_empty() {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "invalid or duplicate entry root reservation",
                            ));
                        }
                        &self.prepared.program.entry
                    }
                    BodyOwner::Function(function) => {
                        if self.entry_root_id.is_none() {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "function root reserved before entry root",
                            ));
                        }
                        let root_slot = self.function_root_ids.get(function).ok_or_else(|| {
                            host_expr_err(
                                HostViolationKind::ReadOutOfBounds,
                                "function owner index out of bounds",
                            )
                        })?;
                        if root_slot.is_some() {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "duplicate function root reservation",
                            ));
                        }
                        for prior in &self.function_root_ids[..function] {
                            if prior.is_none() {
                                return Err(host_expr_err(
                                    HostViolationKind::WrongVariant,
                                    "function root reserved out of function order",
                                ));
                            }
                        }
                        let func =
                            self.prepared
                                .program
                                .functions
                                .get(function)
                                .ok_or_else(|| {
                                    host_expr_err(
                                        HostViolationKind::ReadOutOfBounds,
                                        "function owner index out of bounds",
                                    )
                                })?;
                        &func.body
                    }
                }
            }
            Some((parent_id, child_position)) => {
                let par = self.expr_bindings.get(parent_id).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "parent expression id out of bounds",
                    )
                })?;
                if par.owner != owner {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "child expression owner mismatch with parent",
                    ));
                }
                let child = canonical_child_expr(par.source, child_position).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "child position out of bounds for parent expression",
                    )
                })?;
                if child_position != par.next_expected_child {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "child expression reserved out of preorder sequence",
                    ));
                }
                if !self.expr_bindings[parent_id + 1..]
                    .iter()
                    .all(|b| b.next_expected_child == b.expected_children)
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "sibling subtree reservation incomplete before child reservation",
                    ));
                }
                child
            }
        };
        if !std::ptr::eq(source, canonical_source) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "expression source reference does not match canonical prepared node",
            ));
        }
        let expected_children = expected_child_count(canonical_source)?;
        let (local_expr, next_local) = match owner {
            BodyOwner::Entry => {
                let local = self.entry_next_local;
                let next = local.checked_add(1).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ArithmeticUndefined,
                        "entry local expression ordinal overflow",
                    )
                })?;
                (local, next)
            }
            BodyOwner::Function(function) => {
                let local = *self.function_next_local.get(function).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "function owner index out of bounds",
                    )
                })?;
                let next = local.checked_add(1).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ArithmeticUndefined,
                        "function local expression ordinal overflow",
                    )
                })?;
                (local, next)
            }
        };
        if self.expressions.len() < self.layout.bounds.code_rows {
            if let ExtExpr::Call { args, .. } = canonical_source {
                if args.len() > self.layout.bounds.max_arity {
                    return Err(ExtTypeBuildError::CoreTooling(
                        Tooling::CapacityNotAdmitted {
                            kind: CapacityKind::MaxArity,
                            needed: NeededNatC::Exact(args.len() as u128),
                            limit: self.layout.bounds.max_arity as u128,
                            locator: None,
                        },
                    ));
                }
            }
        }
        let id = reserve_pair(
            &mut self.expressions,
            &mut self.occurrences,
            self.layout.bounds.code_rows,
            CapacityKind::CodeRows,
            None,
        )?;
        if let Some((parent_id, _)) = parent {
            if let Some(par) = self.expr_bindings.get_mut(parent_id) {
                par.next_expected_child =
                    par.next_expected_child.checked_add(1).ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ArithmeticUndefined,
                            "parent child cursor overflow",
                        )
                    })?;
            }
        } else {
            match owner {
                BodyOwner::Entry => self.entry_root_id = Some(id),
                BodyOwner::Function(function) => {
                    if let Some(slot) = self.function_root_ids.get_mut(function) {
                        *slot = Some(id);
                    }
                }
            }
        }
        match owner {
            BodyOwner::Entry => self.entry_next_local = next_local,
            BodyOwner::Function(function) => {
                if let Some(slot) = self.function_next_local.get_mut(function) {
                    *slot = next_local;
                }
            }
        }
        self.expr_bindings.push(BoundExprSource {
            owner,
            local_expr,
            parent,
            source: canonical_source,
            expected_children,
            next_expected_child: 0,
            code_children_slice: None,
            child_slice_linked: false,
            match_arms_slice: None,
            occurrence_path_slice: None,
        });
        Ok((id, local_expr))
    }

    pub(super) fn name_id(&self, name: &str) -> Result<usize, ExtTypeBuildError> {
        self.prepared
            .interned_names
            .binary_search_by(|entry| entry.as_str().cmp(name))
            .map_err(|_| {
                ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
                    reason: AdmissionReason::ProgramViewMalformed,
                    detail: "name not collected in pass B".to_string(),
                    locator: None,
                })
            })
    }

    pub(super) fn write_expression(
        &mut self,
        id: usize,
        row: ExpressionRowC,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_all_expressions_reserved()?;
        match self.expressions.get(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "reserved slot index out of bounds",
                ));
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserved slot already filled",
                ));
            }
            Some(None) => {}
        }
        let binding = self.expr_bindings.get(id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "reserved slot index out of bounds",
            )
        })?;
        if binding.next_expected_child != binding.expected_children {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "expression children not completely reserved before row write",
            ));
        }
        let owner = binding.owner;
        let local_expr = binding.local_expr;
        match (binding.source, &row) {
            (ExtExpr::Literal { .. }, _) | (_, ExpressionRowC::Literal { .. }) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "literal expression row remains None until C6",
                ));
            }
            (ExtExpr::Var { name }, ExpressionRowC::Var { binder }) => {
                if *binder != self.name_id(name)? {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "var binder name id mismatch",
                    ));
                }
            }
            (
                ExtExpr::Let { name, .. },
                ExpressionRowC::Let {
                    binder,
                    value,
                    body,
                },
            ) => {
                if *binder != self.name_id(name)? {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "let binder name id mismatch",
                    ));
                }
                self.check_direct_child(id, owner, 0, *value)?;
                self.check_direct_child(id, owner, 1, *body)?;
            }
            (ExtExpr::Binary { op: src_op, .. }, ExpressionRowC::Binary { op, left, right }) => {
                if *op != *src_op {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "binary operator mismatch",
                    ));
                }
                self.check_direct_child(id, owner, 0, *left)?;
                self.check_direct_child(id, owner, 1, *right)?;
            }
            (ExtExpr::Unary { op: src_op, .. }, ExpressionRowC::Unary { op, operand }) => {
                if *op != *src_op {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "unary operator mismatch",
                    ));
                }
                self.check_direct_child(id, owner, 0, *operand)?;
            }
            (
                ExtExpr::If { .. },
                ExpressionRowC::If {
                    cond,
                    then_expr,
                    else_expr,
                },
            ) => {
                self.check_direct_child(id, owner, 0, *cond)?;
                self.check_direct_child(id, owner, 1, *then_expr)?;
                self.check_direct_child(id, owner, 2, *else_expr)?;
            }
            (
                ExtExpr::Call { name, .. },
                ExpressionRowC::Call {
                    function_name,
                    args,
                },
            ) => {
                if *function_name != self.name_id(name)? || *args != SliceC::empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "call row initial state mismatch",
                    ));
                }
            }
            (
                ExtExpr::Match { .. },
                ExpressionRowC::Match {
                    scrutinee,
                    arms,
                    plan,
                },
            ) => {
                self.check_direct_child(id, owner, 0, *scrutinee)?;
                if *arms != SliceC::empty() || plan.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "match row initial state mismatch",
                    ));
                }
            }
            (
                ExtExpr::Construct {
                    ctor,
                    fields: src_fields,
                    ..
                },
                ExpressionRowC::Construct {
                    type_id,
                    ctor_id,
                    fields,
                    constant,
                },
            ) => {
                if *fields != SliceC::empty() || constant.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct row initial state mismatch",
                    ));
                }
                let type_row = self.prepared.tables.types.get(type_id.0).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct row type or constructor id out of table bounds",
                    )
                })?;
                let ctor_row = self
                    .prepared
                    .tables
                    .constructors
                    .get(ctor_id.0)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::WrongVariant,
                            "construct row type or constructor id out of table bounds",
                        )
                    })?;
                let ctor_end = type_row
                    .ctor_start
                    .0
                    .checked_add(type_row.ctor_len)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ArithmeticUndefined,
                            "type constructor range overflow",
                        )
                    })?;
                if type_row.decl.is_none()
                    || ctor_row.owning_type != *type_id
                    || ctor_id.0 < type_row.ctor_start.0
                    || ctor_id.0 >= ctor_end
                    || ctor_end > self.prepared.tables.constructors.len()
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct row constructor not in owning nominal type range",
                    ));
                }
                let expected_ctor_name_id = self.name_id(ctor)?;
                let field_end = ctor_row
                    .field_start
                    .checked_add(ctor_row.field_len)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ArithmeticUndefined,
                            "constructor field range overflow",
                        )
                    })?;
                if ctor_row.name_id != expected_ctor_name_id
                    || ctor_row.field_len != src_fields.len()
                    || field_end > self.prepared.tables.ctor_fields.len()
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct row name or arity mismatch with constructor table",
                    ));
                }
                let site = T3UseSite::Construct {
                    owner,
                    expr: local_expr,
                };
                let mut site_match: Option<(usize, Option<usize>)> = None;
                for rec in &self.prepared.t3_records {
                    if rec.site == site {
                        if site_match.is_some() {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "duplicate T3 construct record",
                            ));
                        }
                        site_match = Some((rec.type_id.0, rec.ctor_id.map(|c| c.0)));
                    }
                }
                if site_match != Some((type_id.0, Some(ctor_id.0))) {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct row does not match unique prepared T3 record",
                    ));
                }
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "expression row variant mismatch with bound source",
                ));
            }
        }
        fill_reserved_once(&mut self.expressions, id, row)
    }

    pub(super) fn append_code_children(
        &mut self,
        children: &[usize],
    ) -> Result<SliceC, ExtTypeBuildError> {
        self.check_all_expressions_reserved()?;
        for &child_id in children {
            if child_id >= self.expr_bindings.len() {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "code child expression id out of bounds",
                ));
            }
        }
        let parent_id = self
            .expr_bindings
            .iter()
            .position(|b| {
                matches!(b.source, ExtExpr::Call { .. } | ExtExpr::Construct { .. })
                    && b.code_children_slice.is_none()
            })
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "no pending Call or Construct expression for append_code_children",
                )
            })?;
        if self.expr_bindings[..parent_id].iter().any(|b| {
            if matches!(b.source, ExtExpr::Call { .. } | ExtExpr::Construct { .. }) {
                b.code_children_slice.is_none() || !b.child_slice_linked
            } else {
                b.code_children_slice.is_some() || b.child_slice_linked
            }
        }) || self.expr_bindings[parent_id + 1..]
            .iter()
            .any(|b| b.code_children_slice.is_some() || b.child_slice_linked)
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "code children allocated out of C2 parent order",
            ));
        }
        let par_owner = self.expr_bindings[parent_id].owner;
        match (
            self.expr_bindings[parent_id].source,
            self.expressions.get(parent_id),
        ) {
            (
                ExtExpr::Call { args, .. },
                Some(Some(ExpressionRowC::Call {
                    args: existing_args,
                    ..
                })),
            ) => {
                if args.len() != children.len() || *existing_args != SliceC::empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "call parent row state or arity mismatch in append_code_children",
                    ));
                }
            }
            (
                ExtExpr::Construct { fields, .. },
                Some(Some(ExpressionRowC::Construct {
                    fields: existing_fields,
                    constant: None,
                    ..
                })),
            ) => {
                if fields.len() != children.len() || *existing_fields != SliceC::empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct parent row state or arity mismatch in append_code_children",
                    ));
                }
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "parent Call or Construct row not in initial C1 written state",
                ));
            }
        }
        for (pos, &child_id) in children.iter().enumerate() {
            self.check_direct_child(parent_id, par_owner, pos, child_id)?;
        }
        let slice = append_ids_bounded(
            &mut self.children,
            children,
            self.layout.bounds.code_child_slots,
            CapacityKind::CodeChildSlots,
            None,
        )?;
        self.expr_bindings[parent_id].code_children_slice = Some(slice);
        Ok(slice)
    }

    pub(super) fn append_occurrence_path(
        &mut self,
        path: &[usize],
    ) -> Result<SliceC, ExtTypeBuildError> {
        self.check_all_expressions_reserved()?;
        let id = self.next_occurrence_id;
        if id >= self.expr_bindings.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "no pending expression for occurrence path append",
            ));
        }
        if self.occurrences[id].is_some()
            || self.expr_bindings[id].occurrence_path_slice.is_some()
            || (id > 0 && self.occurrences[id - 1].is_none())
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "occurrence path append out of C4 sequence",
            ));
        }
        self.check_path_matches_bound_ancestors(id, path)?;
        let slice = append_ids_bounded(
            &mut self.occurrence_paths,
            path,
            self.layout.capacities.occurrence_path_slots,
            CapacityKind::OccurrencePathSlots,
            None,
        )?;
        self.expr_bindings[id].occurrence_path_slice = Some(slice);
        Ok(slice)
    }

    pub(super) fn write_occurrence(
        &mut self,
        id: usize,
        row: OccurrenceRowC,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_all_expressions_reserved()?;
        match self.occurrences.get(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "reserved slot index out of bounds",
                ));
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserved slot already filled",
                ));
            }
            Some(None) => {}
        }
        if id != self.next_occurrence_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "occurrence written out of C4 ExprId order",
            ));
        }
        let binding = self.expr_bindings.get(id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "reserved slot index out of bounds",
            )
        })?;
        if binding.occurrence_path_slice != Some(row.path) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "occurrence path slice was not allocated for this expression",
            ));
        }
        match binding.owner {
            BodyOwner::Entry => {
                if row.owner_is_function || row.function != 0 {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "occurrence owner mismatch for entry expression",
                    ));
                }
            }
            BodyOwner::Function(expected_fn) => {
                if !row.owner_is_function || row.function != expected_fn {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "occurrence owner mismatch for function expression",
                    ));
                }
            }
        }
        if row.path.len == 0 && row.path.start != 0 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "unnormalized empty occurrence path slice",
            ));
        }
        let end = row.path.start.checked_add(row.path.len).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "occurrence path slice end overflow",
            )
        })?;
        if end > self.occurrence_paths.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "occurrence path slice out of bounds",
            ));
        }
        self.check_path_matches_bound_ancestors(id, &self.occurrence_paths[row.path.start..end])?;
        fill_reserved_once(&mut self.occurrences, id, row)?;
        self.next_occurrence_id = id.checked_add(1).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "next occurrence id overflow",
            )
        })?;
        Ok(())
    }

    fn check_c1_rows_written(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c1_rows_written_for(LiteralSlotPhase::BeforeC6)
    }

    fn check_c1_rows_written_for(&self, phase: LiteralSlotPhase) -> Result<(), ExtTypeBuildError> {
        self.check_all_expressions_reserved()?;
        for (id, binding) in self.expr_bindings.iter().enumerate() {
            match (binding.source, self.expressions.get(id)) {
                (ExtExpr::Literal { .. }, Some(None)) if phase != LiteralSlotPhase::AfterC6 => {}
                (ExtExpr::Literal { .. }, Some(Some(ExpressionRowC::Literal { .. })))
                    if phase != LiteralSlotPhase::BeforeC6 => {}
                (ExtExpr::Literal { .. }, _) => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        phase.wrong_literal_slot(),
                    ));
                }
                (_, Some(Some(_))) => {}
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "non-literal C1 expression row not yet written",
                    ));
                }
            }
        }
        Ok(())
    }

    fn check_c2_complete(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c1_rows_written()?;
        self.check_c2_code_children_linked()
    }

    fn check_c2_code_children_linked(&self) -> Result<(), ExtTypeBuildError> {
        for binding in &self.expr_bindings {
            if matches!(
                binding.source,
                ExtExpr::Call { .. } | ExtExpr::Construct { .. }
            ) {
                if binding.code_children_slice.is_none() || !binding.child_slice_linked {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "C2 Call or Construct child slice linking incomplete",
                    ));
                }
            } else if binding.code_children_slice.is_some() || binding.child_slice_linked {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "non-Call/Construct expression unexpectedly linked code children",
                ));
            }
        }
        Ok(())
    }

    fn check_c3_complete(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c2_complete()?;
        self.check_c3_match_arms_linked()
    }

    fn check_c3_match_arms_linked(&self) -> Result<(), ExtTypeBuildError> {
        if self.active_arm.is_some()
            || self.current_match_expr_id.is_some()
            || !self.current_match_arm_ids.is_empty()
            || !self.arms.iter().all(Option::is_some)
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "C3 match arm or pattern emission incomplete",
            ));
        }
        for binding in &self.expr_bindings {
            if matches!(binding.source, ExtExpr::Match { .. }) {
                if binding.match_arms_slice.is_none() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "C3 match arms slice not linked",
                    ));
                }
            } else if binding.match_arms_slice.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "non-Match expression unexpectedly linked match arms",
                ));
            }
        }
        Ok(())
    }

    fn check_c4_complete(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c3_complete()?;
        self.check_c4_occurrences_written()
    }

    fn check_c4_occurrences_written(&self) -> Result<(), ExtTypeBuildError> {
        if self.next_occurrence_id != self.expr_bindings.len()
            || !self.occurrences.iter().all(Option::is_some)
            || !self
                .expr_bindings
                .iter()
                .all(|b| b.occurrence_path_slice.is_some())
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "C4 occurrence emission incomplete",
            ));
        }
        Ok(())
    }

    fn find_bound_expr(
        &self,
        owner: BodyOwner,
        local_expr: usize,
    ) -> Result<(usize, &BoundExprSource<'s>), ExtTypeBuildError> {
        if let BodyOwner::Function(function) = owner {
            if function >= self.prepared.program.functions.len() {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "function owner index out of bounds",
                ));
            }
        }
        let mut matched: Option<(usize, &BoundExprSource<'s>)> = None;
        for (idx, binding) in self.expr_bindings.iter().enumerate() {
            if binding.owner == owner && binding.local_expr == local_expr {
                if matched.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "duplicate bound expression for owner and local ordinal",
                    ));
                }
                matched = Some((idx, binding));
            }
        }
        matched.ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "referenced expression ordinal not reserved",
            )
        })
    }

    pub(super) fn signature_type(&self, site: T3UseSite) -> Result<TypeIdC, ExtTypeBuildError> {
        self.check_stage_alignment()?;
        match site {
            T3UseSite::ParamType { function, param } => {
                let func = self
                    .prepared
                    .program
                    .functions
                    .get(function)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "signature parameter function index out of bounds",
                        )
                    })?;
                if param >= func.parameters.len() {
                    return Err(host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "signature parameter index out of bounds",
                    ));
                }
            }
            T3UseSite::ReturnType { function } => {
                if function >= self.prepared.program.functions.len() {
                    return Err(host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "signature return function index out of bounds",
                    ));
                }
            }
            T3UseSite::Construct { .. }
            | T3UseSite::LiteralCtor { .. }
            | T3UseSite::CtorPattern { .. } => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "non-signature T3 use site passed to signature_type",
                ));
            }
        }
        let mut matched_type: Option<TypeIdC> = None;
        for rec in &self.prepared.t3_records {
            if rec.site == site {
                if matched_type.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "duplicate T3 signature record",
                    ));
                }
                if rec.ctor_id.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "signature T3 record unexpectedly carries constructor id",
                    ));
                }
                if rec.type_id.0 >= self.prepared.tables.types.len() {
                    return Err(host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "signature TypeIdC out of type table bounds",
                    ));
                }
                matched_type = Some(rec.type_id);
            }
        }
        matched_type.ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "missing T3 signature record",
            )
        })
    }

    pub(super) fn constructor_use(
        &self,
        site: T3UseSite,
    ) -> Result<(TypeIdC, CtorIdC), ExtTypeBuildError> {
        self.check_stage_alignment()?;
        let (expected_ctor_name, expected_arity) = match site {
            T3UseSite::Construct { owner, expr } => {
                let (_, binding) = self.find_bound_expr(owner, expr)?;
                match binding.source {
                    ExtExpr::Construct { ctor, fields, .. } => (ctor.as_str(), fields.len()),
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "construct T3 use site bound to non-Construct expression",
                        ));
                    }
                }
            }
            T3UseSite::CtorPattern {
                owner,
                expr,
                arm,
                pattern,
            } => {
                let (_, binding) = self.find_bound_expr(owner, expr)?;
                let arms = match binding.source {
                    ExtExpr::Match { arms, .. } => arms,
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "pattern T3 use site bound to non-Match expression",
                        ));
                    }
                };
                let arm_src = arms.get(arm).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "pattern T3 use site arm index out of bounds",
                    )
                })?;
                let pat_node =
                    pattern_node_at_preorder(&arm_src.pattern, pattern)?.ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "pattern T3 use site preorder ordinal out of bounds",
                        )
                    })?;
                match pat_node {
                    ExtPattern::Ctor { ctor, fields, .. } => (ctor.as_str(), fields.len()),
                    ExtPattern::Dedicated { .. } | ExtPattern::Wildcard => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "pattern T3 use site is not a constructor pattern",
                        ));
                    }
                }
            }
            T3UseSite::ParamType { .. }
            | T3UseSite::ReturnType { .. }
            | T3UseSite::LiteralCtor { .. } => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "invalid T3 use site for constructor_use",
                ));
            }
        };
        let mut matched: Option<(TypeIdC, CtorIdC)> = None;
        for rec in &self.prepared.t3_records {
            if rec.site == site {
                if matched.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "duplicate T3 constructor record",
                    ));
                }
                let ctor_id = rec.ctor_id.ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::WrongVariant,
                        "constructor T3 record missing CtorIdC",
                    )
                })?;
                let type_row = self
                    .prepared
                    .tables
                    .types
                    .get(rec.type_id.0)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "constructor TypeIdC out of type table bounds",
                        )
                    })?;
                let ctor_row = self
                    .prepared
                    .tables
                    .constructors
                    .get(ctor_id.0)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "constructor CtorIdC out of constructor table bounds",
                        )
                    })?;
                let ctor_end = type_row
                    .ctor_start
                    .0
                    .checked_add(type_row.ctor_len)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ArithmeticUndefined,
                            "type constructor range overflow",
                        )
                    })?;
                if ctor_row.owning_type != rec.type_id
                    || ctor_id.0 < type_row.ctor_start.0
                    || ctor_id.0 >= ctor_end
                    || ctor_end > self.prepared.tables.constructors.len()
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "constructor T3 record does not belong to owning type",
                    ));
                }
                let field_end = ctor_row
                    .field_start
                    .checked_add(ctor_row.field_len)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ArithmeticUndefined,
                            "constructor field slice end overflow",
                        )
                    })?;
                if field_end > self.prepared.tables.ctor_fields.len()
                    || ctor_row.field_len != expected_arity
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "constructor T3 record field arity or slice mismatch",
                    ));
                }
                match (site, rec.type_id) {
                    (T3UseSite::CtorPattern { .. }, TypeIdC::List) => {
                        let valid_builtin_list = type_row.decl.is_none()
                            && type_row.ctor_start == CtorIdC(0)
                            && type_row.ctor_len == 2
                            && ctor_row.name_id == 0
                            && matches!(
                                (
                                    expected_ctor_name,
                                    ctor_id,
                                    expected_arity,
                                    ctor_row.field_start
                                ),
                                ("Nil", CtorIdC(0), 0, 0) | ("Cons", CtorIdC(1), 2, 0)
                            );
                        if !valid_builtin_list {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "invalid builtin List constructor pattern T3 record",
                            ));
                        }
                    }
                    (T3UseSite::CtorPattern { .. }, TypeIdC::Tree) => {
                        let valid_builtin_tree = type_row.decl.is_none()
                            && type_row.ctor_start == CtorIdC(2)
                            && type_row.ctor_len == 2
                            && ctor_row.name_id == 0
                            && matches!(
                                (
                                    expected_ctor_name,
                                    ctor_id,
                                    expected_arity,
                                    ctor_row.field_start
                                ),
                                ("Leaf", CtorIdC(2), 1, 2) | ("Branch", CtorIdC(3), 2, 3)
                            );
                        if !valid_builtin_tree {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "invalid builtin Tree constructor pattern T3 record",
                            ));
                        }
                    }
                    (T3UseSite::Construct { .. }, type_id)
                    | (T3UseSite::CtorPattern { .. }, type_id)
                        if type_id.0 >= 4 =>
                    {
                        let expected_ctor_name_id = self.name_id(expected_ctor_name)?;
                        if type_row.decl.is_none()
                            || ctor_id.0 < 4
                            || ctor_row.name_id != expected_ctor_name_id
                        {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "nominal constructor T3 record decl, id, or name mismatch",
                            ));
                        }
                    }
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "constructor T3 record type is not valid for use site",
                        ));
                    }
                }
                matched = Some((rec.type_id, ctor_id));
            }
        }
        matched.ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "missing T3 constructor record",
            )
        })
    }

    pub(super) fn set_call_args(
        &mut self,
        id: usize,
        args: SliceC,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c1_rows_written()?;
        let (owner, expected_fn_name, expected_len, allocated_slice) = {
            let binding = self.expr_bindings.get(id).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "call expression id out of bounds",
                )
            })?;
            if self.expr_bindings[..id].iter().any(|b| {
                if matches!(b.source, ExtExpr::Call { .. } | ExtExpr::Construct { .. }) {
                    b.code_children_slice.is_none() || !b.child_slice_linked
                } else {
                    b.code_children_slice.is_some() || b.child_slice_linked
                }
            }) || self.expr_bindings[id + 1..]
                .iter()
                .any(|b| b.code_children_slice.is_some() || b.child_slice_linked)
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_call_args called out of C2 parent progression order",
                ));
            }
            if binding.child_slice_linked {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "call args slice already linked",
                ));
            }
            let (src_name, src_args) = match binding.source {
                ExtExpr::Call { name, args } => (name, args),
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "set_call_args called on non-Call expression",
                    ));
                }
            };
            (
                binding.owner,
                self.name_id(src_name)?,
                src_args.len(),
                binding.code_children_slice,
            )
        };
        if allocated_slice != Some(args) || args.len != expected_len {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "call args slice does not match allocated C2 code_children_slice",
            ));
        }
        if args.len == 0 && args.start != 0 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "unnormalized empty call args slice",
            ));
        }
        let end = args.start.checked_add(args.len).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "call args slice end overflow",
            )
        })?;
        if end > self.children.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "call args slice out of children table bounds",
            ));
        }
        for (pos, &child_id) in self.children[args.start..end].iter().enumerate() {
            self.check_direct_child(id, owner, pos, child_id)?;
        }
        match self.expressions.get_mut(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "call expression slot index out of bounds",
                ));
            }
            Some(None) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "call expression row not written before set_call_args",
                ));
            }
            Some(Some(ExpressionRowC::Call {
                function_name,
                args: existing_args,
            })) => {
                if *function_name != expected_fn_name || *existing_args != SliceC::empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "call row state mismatch before set_call_args",
                    ));
                }
                *existing_args = args;
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_call_args target row is not ExpressionRowC::Call",
                ));
            }
        }
        if let Some(binding) = self.expr_bindings.get_mut(id) {
            binding.child_slice_linked = true;
        }
        Ok(())
    }

    pub(super) fn set_construct_fields(
        &mut self,
        id: usize,
        fields: SliceC,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c1_rows_written()?;
        let (owner, local_expr, expected_len, allocated_slice) = {
            let binding = self.expr_bindings.get(id).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "construct expression id out of bounds",
                )
            })?;
            if self.expr_bindings[..id].iter().any(|b| {
                if matches!(b.source, ExtExpr::Call { .. } | ExtExpr::Construct { .. }) {
                    b.code_children_slice.is_none() || !b.child_slice_linked
                } else {
                    b.code_children_slice.is_some() || b.child_slice_linked
                }
            }) || self.expr_bindings[id + 1..]
                .iter()
                .any(|b| b.code_children_slice.is_some() || b.child_slice_linked)
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_construct_fields called out of C2 parent progression order",
                ));
            }
            if binding.child_slice_linked {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "construct fields slice already linked",
                ));
            }
            let src_fields = match binding.source {
                ExtExpr::Construct { fields, .. } => fields,
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "set_construct_fields called on non-Construct expression",
                    ));
                }
            };
            (
                binding.owner,
                binding.local_expr,
                src_fields.len(),
                binding.code_children_slice,
            )
        };
        let (expected_type_id, expected_ctor_id) = self.constructor_use(T3UseSite::Construct {
            owner,
            expr: local_expr,
        })?;
        if allocated_slice != Some(fields) || fields.len != expected_len {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "construct fields slice does not match allocated C2 code_children_slice",
            ));
        }
        if fields.len == 0 && fields.start != 0 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "unnormalized empty construct fields slice",
            ));
        }
        let end = fields.start.checked_add(fields.len).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "construct fields slice end overflow",
            )
        })?;
        if end > self.children.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "construct fields slice out of children table bounds",
            ));
        }
        for (pos, &child_id) in self.children[fields.start..end].iter().enumerate() {
            self.check_direct_child(id, owner, pos, child_id)?;
        }
        match self.expressions.get_mut(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "construct expression slot index out of bounds",
                ));
            }
            Some(None) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "construct expression row not written before set_construct_fields",
                ));
            }
            Some(Some(ExpressionRowC::Construct {
                type_id,
                ctor_id,
                fields: existing_fields,
                constant,
            })) => {
                if *type_id != expected_type_id
                    || *ctor_id != expected_ctor_id
                    || *existing_fields != SliceC::empty()
                    || constant.is_some()
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "construct row state mismatch before set_construct_fields",
                    ));
                }
                *existing_fields = fields;
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_construct_fields target row is not ExpressionRowC::Construct",
                ));
            }
        }
        if let Some(binding) = self.expr_bindings.get_mut(id) {
            binding.child_slice_linked = true;
        }
        Ok(())
    }

    pub(super) fn write_finance_expression(
        &mut self,
        id: usize,
        source: &ExtExpr,
        children: &[usize],
    ) -> Result<(), ExtTypeBuildError> {
        self.check_all_expressions_reserved()?;
        match self.expressions.get(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "reserved slot index out of bounds",
                ));
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserved slot already filled",
                ));
            }
            Some(None) => {}
        }
        let binding = self.expr_bindings.get(id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "reserved slot index out of bounds",
            )
        })?;
        if !std::ptr::eq(source, binding.source) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finance expression source reference does not match canonical prepared node",
            ));
        }
        match binding.source {
            ExtExpr::StateRead { .. }
            | ExtExpr::LiabilityPresent { .. }
            | ExtExpr::LiabilityAmount { .. }
            | ExtExpr::Fin { .. }
            | ExtExpr::Await { .. } => {}
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "non-finance expression passed to write_finance_expression",
                ));
            }
        }
        if binding.next_expected_child != binding.expected_children
            || children.len() != binding.expected_children
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finance expression children length mismatch",
            ));
        }
        for (pos, &child_id) in children.iter().enumerate() {
            self.check_direct_child(id, binding.owner, pos, child_id)?;
        }
        if !self.prepared.domain.is_finance_absent() {
            return Err(ExtTypeBuildError::CoreTooling(
                Tooling::UnsupportedFeature {
                    feature: FeatureKind::FinancialInput,
                    locator: None,
                },
            ));
        }
        Err(ExtTypeBuildError::CoreTooling(
            Tooling::UnsupportedFeature {
                feature: FeatureKind::FinancialSyntax,
                locator: None,
            },
        ))
    }

    fn next_pending_match_expr_id(&self) -> Option<usize> {
        self.expr_bindings
            .iter()
            .position(|b| matches!(b.source, ExtExpr::Match { .. }) && b.match_arms_slice.is_none())
    }

    pub(super) fn reserve_arm(&mut self) -> Result<usize, ExtTypeBuildError> {
        self.check_c2_complete()?;
        if self.active_arm.is_some() || !self.arms.iter().all(Option::is_some) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "previous arm pattern or write_arm incomplete before reserve_arm",
            ));
        }
        let match_expr_id = match self.current_match_expr_id {
            Some(id) => id,
            None => {
                let id = self.next_pending_match_expr_id().ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::WrongVariant,
                        "no pending Match expression for reserve_arm",
                    )
                })?;
                if self.expr_bindings[id + 1..]
                    .iter()
                    .any(|b| b.match_arms_slice.is_some())
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "match arms reserved out of C3 Match ExprId order",
                    ));
                }
                id
            }
        };
        let binding = self.expr_bindings.get(match_expr_id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "match expression id out of bounds",
            )
        })?;
        let src_arms = match binding.source {
            ExtExpr::Match { arms, .. } => arms,
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "current match expression is not ExtExpr::Match",
                ));
            }
        };
        let arm_index = self.current_match_arm_ids.len();
        let arm_src = src_arms.get(arm_index).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "all arms of current Match expression already reserved",
            )
        })?;
        let owner = binding.owner;
        let local_expr = binding.local_expr;
        let arm_start_patterns_len = self.patterns.len();
        let arm_id = reserve_slot(
            &mut self.arms,
            self.layout.bounds.match_arm_rows,
            CapacityKind::MatchArmRows,
            None,
        )?;
        self.current_match_expr_id = Some(match_expr_id);
        self.active_arm = Some(ActiveArmPatternState {
            arm_id,
            match_expr_id,
            owner,
            local_expr,
            arm_index,
            arm_start_patterns_len,
            visit_stack: vec![ExpectedPatternStep::Enter {
                parent: None,
                source: &arm_src.pattern,
            }],
            entered_nodes: Vec::new(),
            root_pattern_id: None,
        });
        Ok(arm_id)
    }

    pub(super) fn visit_pattern_source(
        &mut self,
        owner: BodyOwner,
        expr: usize,
        arm: usize,
        ordinal: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c2_complete()?;
        let pattern_rows_limit = self.layout.bounds.pattern_rows;
        let active = self.active_arm.as_mut().ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "visit_pattern_source called without active reserved arm",
            )
        })?;
        if active.owner != owner
            || active.local_expr != expr
            || active.arm_index != arm
            || active.root_pattern_id.is_some()
            || ordinal != active.entered_nodes.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "visit_pattern_source site or preorder ordinal mismatch",
            ));
        }
        let (parent, source) = match active.visit_stack.last() {
            Some(ExpectedPatternStep::Enter { parent, source }) => (*parent, *source),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "visit_pattern_source called when pattern exit was expected",
                ));
            }
        };
        let planned_pattern_index = active
            .arm_start_patterns_len
            .checked_add(ordinal)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ArithmeticUndefined,
                    "planned pattern row count overflow",
                )
            })?;
        if planned_pattern_index >= pattern_rows_limit {
            return Err(core_stage_cap_err(
                CapacityKind::PatternRows,
                planned_pattern_index,
                pattern_rows_limit,
            ));
        }
        active.visit_stack.pop();
        active.entered_nodes.push(EnteredPatternNode {
            source,
            parent,
            expected_children: Vec::new(),
            emitted_id: None,
        });
        active
            .visit_stack
            .push(ExpectedPatternStep::Exit { ordinal });
        if let ExtPattern::Ctor { fields, .. } = source {
            for field in fields.iter().rev() {
                active.visit_stack.push(ExpectedPatternStep::Enter {
                    parent: Some(ordinal),
                    source: field,
                });
            }
        }
        Ok(())
    }

    pub(super) fn pattern_traversal_invariant(&self) -> ExtTypeBuildError {
        host_expr_err(
            HostViolationKind::WrongVariant,
            "pattern traversal terminated without emitting root pattern",
        )
    }

    pub(super) fn append_pattern(
        &mut self,
        row: PatternRowC,
        children: &[usize],
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c2_complete()?;
        let (owner, local_expr, arm_index, ordinal, node_source, parent_ordinal, expected_children) = {
            let active = self.active_arm.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_pattern called without active reserved arm",
                )
            })?;
            if active.root_pattern_id.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "arm root pattern already emitted",
                ));
            }
            let ordinal = match active.visit_stack.last() {
                Some(ExpectedPatternStep::Exit { ordinal }) => *ordinal,
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "append_pattern called out of postorder sequence",
                    ));
                }
            };
            let node = active.entered_nodes.get(ordinal).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pattern preorder ordinal out of bounds",
                )
            })?;
            if node.emitted_id.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pattern node already emitted",
                ));
            }
            (
                active.owner,
                active.local_expr,
                active.arm_index,
                ordinal,
                node.source,
                node.parent,
                node.expected_children.clone(),
            )
        };
        if children != expected_children.as_slice() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "append_pattern child row IDs do not match emitted child patterns",
            ));
        }
        for &child_pat_id in children {
            if child_pat_id >= self.patterns.len() {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pattern child row ID out of bounds",
                ));
            }
        }
        if self.patterns.len() >= self.layout.bounds.pattern_rows {
            return Err(core_stage_cap_err(
                CapacityKind::PatternRows,
                self.patterns.len(),
                self.layout.bounds.pattern_rows,
            ));
        }
        let final_row = match (node_source, row) {
            (
                ExtPattern::Dedicated {
                    pattern: CorePattern::PVar { x: src_x },
                },
                PatternRowC::PVar { x },
            ) => {
                if !children.is_empty() || x != self.name_id(src_x)? {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "PVar pattern row mismatch with source",
                    ));
                }
                PatternRowC::PVar { x }
            }
            (
                ExtPattern::Dedicated {
                    pattern: CorePattern::PNil,
                },
                PatternRowC::PNil,
            ) => {
                if !children.is_empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "PNil pattern cannot have children",
                    ));
                }
                PatternRowC::PNil
            }
            (
                ExtPattern::Dedicated {
                    pattern: CorePattern::PCons { x: src_x, y: src_y },
                },
                PatternRowC::PCons { x, y },
            ) => {
                if !children.is_empty() || x != self.name_id(src_x)? || y != self.name_id(src_y)? {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "PCons pattern row mismatch with source",
                    ));
                }
                PatternRowC::PCons { x, y }
            }
            (
                ExtPattern::Dedicated {
                    pattern: CorePattern::PLeaf { x: src_x },
                },
                PatternRowC::PLeaf { x },
            ) => {
                if !children.is_empty() || x != self.name_id(src_x)? {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "PLeaf pattern row mismatch with source",
                    ));
                }
                PatternRowC::PLeaf { x }
            }
            (
                ExtPattern::Dedicated {
                    pattern: CorePattern::PBranch { x: src_x, y: src_y },
                },
                PatternRowC::PBranch { x, y },
            ) => {
                if !children.is_empty() || x != self.name_id(src_x)? || y != self.name_id(src_y)? {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "PBranch pattern row mismatch with source",
                    ));
                }
                PatternRowC::PBranch { x, y }
            }
            (ExtPattern::Wildcard, PatternRowC::Wildcard) => {
                if !children.is_empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "Wildcard pattern cannot have children",
                    ));
                }
                PatternRowC::Wildcard
            }
            (
                ExtPattern::Ctor {
                    fields: src_fields, ..
                },
                PatternRowC::Constructor {
                    type_id,
                    ctor_id,
                    children: initial_slice,
                },
            ) => {
                if initial_slice != SliceC::empty() || children.len() != src_fields.len() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "Constructor pattern initial slice or child count mismatch",
                    ));
                }
                let (expected_type_id, expected_ctor_id) =
                    self.constructor_use(T3UseSite::CtorPattern {
                        owner,
                        expr: local_expr,
                        arm: arm_index,
                        pattern: ordinal,
                    })?;
                if type_id != expected_type_id || ctor_id != expected_ctor_id {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "Constructor pattern type or constructor id mismatch",
                    ));
                }
                let start = self.pattern_children.len();
                for &child_pat_id in children {
                    if self.pattern_children.len() >= self.pattern_child_slots {
                        return Err(ext_stage_cap_err(
                            ExtTypeCapKind::PatternChildSlots,
                            self.pattern_children.len(),
                            self.pattern_child_slots,
                        ));
                    }
                    self.pattern_children.push(child_pat_id);
                }
                PatternRowC::Constructor {
                    type_id,
                    ctor_id,
                    children: SliceC::normalized(start, children.len()),
                }
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pattern row variant mismatch with source pattern",
                ));
            }
        };
        let pattern_id = self.patterns.len();
        self.patterns.push(final_row);
        if let Some(active) = self.active_arm.as_mut() {
            active.visit_stack.pop();
            if let Some(node) = active.entered_nodes.get_mut(ordinal) {
                node.emitted_id = Some(pattern_id);
            }
            if let Some(par_ord) = parent_ordinal {
                let parent_node = active.entered_nodes.get_mut(par_ord).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "pattern parent ordinal out of bounds",
                    )
                })?;
                parent_node.expected_children.push(pattern_id);
            } else {
                if !active.visit_stack.is_empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "root pattern emitted while pattern visit stack non-empty",
                    ));
                }
                active.root_pattern_id = Some(pattern_id);
            }
        }
        Ok(pattern_id)
    }

    pub(super) fn write_arm(&mut self, id: usize, row: ArmRowC) -> Result<(), ExtTypeBuildError> {
        self.check_c2_complete()?;
        match self.arms.get(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "reserved slot index out of bounds",
                ));
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserved slot already filled",
                ));
            }
            Some(None) => {}
        }
        let (match_expr_id, owner, arm_index, expected_pattern) = {
            let active = self.active_arm.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "write_arm called without active reserved arm",
                )
            })?;
            if active.arm_id != id
                || !active.visit_stack.is_empty()
                || !active.entered_nodes.iter().all(|n| n.emitted_id.is_some())
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "write_arm called with mismatched arm id or incomplete pattern emission",
                ));
            }
            let root_pat = active.root_pattern_id.ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "write_arm called before root pattern emitted",
                )
            })?;
            (
                active.match_expr_id,
                active.owner,
                active.arm_index,
                root_pat,
            )
        };
        if row.pattern != expected_pattern || row.pattern >= self.patterns.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "arm pattern row id does not match emitted root pattern",
            ));
        }
        let child_pos = arm_index.checked_add(1).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "match arm child position overflow",
            )
        })?;
        self.check_direct_child(match_expr_id, owner, child_pos, row.body)?;
        fill_reserved_once(&mut self.arms, id, row)?;
        self.active_arm = None;
        self.current_match_arm_ids.push(id);
        Ok(())
    }

    pub(super) fn set_match_arms(
        &mut self,
        id: usize,
        arms: &[usize],
    ) -> Result<SliceC, ExtTypeBuildError> {
        self.check_c2_complete()?;
        if self.active_arm.is_some() || !self.arms.iter().all(Option::is_some) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_match_arms called while an arm is still open",
            ));
        }
        let expected_match_id = match self.current_match_expr_id {
            Some(cur_id) => cur_id,
            None => self.next_pending_match_expr_id().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "no pending Match expression for set_match_arms",
                )
            })?,
        };
        if id != expected_match_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_match_arms called out of C3 Match ExprId order",
            ));
        }
        if self.expr_bindings[id + 1..]
            .iter()
            .any(|b| b.match_arms_slice.is_some())
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_match_arms called out of C3 Match progression",
            ));
        }
        let binding = self.expr_bindings.get(id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "match expression id out of bounds",
            )
        })?;
        if binding.match_arms_slice.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "match arms slice already set",
            ));
        }
        let src_arms = match binding.source {
            ExtExpr::Match { arms: src_arms, .. } => src_arms,
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_match_arms called on non-Match expression",
                ));
            }
        };
        if arms.len() != src_arms.len() || arms != self.current_match_arm_ids.as_slice() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_match_arms arm IDs do not match reserved and written arms for this Match",
            ));
        }
        let start = arms.first().copied().unwrap_or(0);
        for (offset, &arm_id) in arms.iter().enumerate() {
            let expected_id = start.checked_add(offset).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ArithmeticUndefined,
                    "match arm slice index overflow",
                )
            })?;
            if arm_id != expected_id || arm_id >= self.arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "match arm IDs are not contiguous",
                ));
            }
        }
        let slice = SliceC::normalized(start, arms.len());
        match self.expressions.get_mut(id) {
            Some(Some(ExpressionRowC::Match {
                arms: existing_arms,
                plan: None,
                ..
            })) => {
                if *existing_arms != SliceC::empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "match row arms slice not empty before set_match_arms",
                    ));
                }
                *existing_arms = slice;
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "match expression row not in initial C1 state before set_match_arms",
                ));
            }
        }
        if let Some(binding) = self.expr_bindings.get_mut(id) {
            binding.match_arms_slice = Some(slice);
        }
        self.current_match_expr_id = None;
        self.current_match_arm_ids.clear();
        Ok(slice)
    }

    pub(super) fn begin_function(
        &mut self,
        function: usize,
        arity: usize,
    ) -> Result<(usize, usize), ExtTypeBuildError> {
        self.check_c4_complete()?;
        if self.active_function.is_some() || !self.functions.iter().all(Option::is_some) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "previous function incomplete before begin_function",
            ));
        }
        let src_fn = self
            .prepared
            .program
            .functions
            .get(function)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "function index out of bounds in begin_function",
                )
            })?;
        if function != self.functions.len() || arity != src_fn.parameters.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_function order or arity mismatch with source function",
            ));
        }
        if self.functions.len() < self.layout.bounds.function_rows
            && arity > self.layout.bounds.max_arity
        {
            return Err(ExtTypeBuildError::CoreTooling(
                Tooling::CapacityNotAdmitted {
                    kind: CapacityKind::MaxArity,
                    needed: NeededNatC::Exact(arity as u128),
                    limit: self.layout.bounds.max_arity as u128,
                    locator: None,
                },
            ));
        }
        let id = reserve_slot(
            &mut self.functions,
            self.layout.bounds.function_rows,
            CapacityKind::FunctionRows,
            None,
        )?;
        let parameter_start = self.parameters.len();
        self.active_function = Some(ActiveFunctionState {
            function_id: id,
            parameter_start,
            expected_arity: arity,
            appended_params: 0,
        });
        Ok((id, parameter_start))
    }

    pub(super) fn append_parameter(&mut self, row: ParameterRowC) -> Result<(), ExtTypeBuildError> {
        self.check_c4_complete()?;
        let (function_id, param_index) = {
            let active = self.active_function.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_parameter called without active function",
                )
            })?;
            if active.appended_params >= active.expected_arity {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "all parameters for active function already appended",
                ));
            }
            (active.function_id, active.appended_params)
        };
        let src_fn = self
            .prepared
            .program
            .functions
            .get(function_id)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "active function index out of bounds",
                )
            })?;
        let src_param = src_fn.parameters.get(param_index).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "active parameter index out of bounds",
            )
        })?;
        let expected_sort = self.signature_type(T3UseSite::ParamType {
            function: function_id,
            param: param_index,
        })?;
        let expected_binder = self.name_id(&src_param.name)?;
        if row.sort != expected_sort || row.binder != expected_binder {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "parameter row binder or sort mismatch with source signature",
            ));
        }
        if self.parameters.len() >= self.layout.bounds.parameter_slots {
            return Err(core_stage_cap_err(
                CapacityKind::ParameterSlots,
                self.parameters.len(),
                self.layout.bounds.parameter_slots,
            ));
        }
        self.parameters.push(row);
        if let Some(active) = self.active_function.as_mut() {
            active.appended_params = active.appended_params.checked_add(1).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ArithmeticUndefined,
                    "appended parameter count overflow",
                )
            })?;
        }
        Ok(())
    }

    pub(super) fn write_function(
        &mut self,
        id: usize,
        row: FunctionRowC,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c4_complete()?;
        match self.functions.get(id) {
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "reserved slot index out of bounds",
                ));
            }
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserved slot already filled",
                ));
            }
            Some(None) => {}
        }
        let (parameter_start, expected_arity) = {
            let active = self.active_function.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "write_function called without active function",
                )
            })?;
            if active.function_id != id || active.appended_params != active.expected_arity {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "write_function id mismatch or parameters incomplete",
                ));
            }
            (active.parameter_start, active.expected_arity)
        };
        let src_fn = self.prepared.program.functions.get(id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "function index out of bounds in write_function",
            )
        })?;
        let expected_name = self.name_id(&src_fn.name)?;
        let expected_return_type = self.signature_type(T3UseSite::ReturnType { function: id })?;
        let expected_params = SliceC::normalized(parameter_start, expected_arity);
        let expected_body = self
            .function_root_ids
            .get(id)
            .and_then(|root| *root)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "missing reserved C1 root ExprId for function body",
                )
            })?;
        if row.function_name != expected_name
            || row.params != expected_params
            || row.return_type != expected_return_type
            || row.body != expected_body
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "function row fields do not match prepared signature or body root",
            ));
        }
        fill_reserved_once(&mut self.functions, id, row)?;
        self.active_function = None;
        Ok(())
    }

    pub(super) fn check_c1_to_c5_completed(
        &self,
        continuation: &ExtCContinuation<'s>,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c4_complete()?;
        if self.active_function.is_some()
            || self.functions.len() != self.prepared.program.functions.len()
            || !self.functions.iter().all(Option::is_some)
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "C5 function emission incomplete",
            ));
        }
        let mut expected_literals = 0usize;
        let mut expected_nullary = 0usize;
        let mut expected_matches = 0usize;
        for (id, binding) in self.expr_bindings.iter().enumerate() {
            match binding.source {
                ExtExpr::Literal { value } => {
                    let lit = continuation
                        .literals
                        .get(expected_literals)
                        .ok_or_else(|| {
                            host_expr_err(
                                HostViolationKind::WrongVariant,
                                "missing literal occurrence in ExtCContinuation",
                            )
                        })?;
                    if lit.expr_id != id
                        || lit.owner != binding.owner
                        || lit.local_expr != binding.local_expr
                        || !std::ptr::eq(lit.value, value)
                    {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "literal continuation entry mismatch",
                        ));
                    }
                    expected_literals += 1;
                }
                ExtExpr::Construct { fields, .. } if fields.is_empty() => {
                    let nc = continuation
                        .nullary_constructs
                        .get(expected_nullary)
                        .ok_or_else(|| {
                            host_expr_err(
                                HostViolationKind::WrongVariant,
                                "missing nullary construct occurrence in ExtCContinuation",
                            )
                        })?;
                    match self.expressions.get(id) {
                        Some(Some(ExpressionRowC::Construct {
                            type_id,
                            ctor_id,
                            fields: row_fields,
                            constant: None,
                        })) if nc.expr_id == id
                            && nc.type_id == *type_id
                            && nc.ctor_id == *ctor_id
                            && *row_fields == SliceC::empty() => {}
                        _ => {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "nullary construct continuation entry mismatch",
                            ));
                        }
                    }
                    expected_nullary += 1;
                }
                ExtExpr::Match { arms, .. } => {
                    let m = continuation.matches.get(expected_matches).ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::WrongVariant,
                            "missing match occurrence in ExtCContinuation",
                        )
                    })?;
                    if m.expr_id != id
                        || m.owner != binding.owner
                        || m.local_expr != binding.local_expr
                        || binding.match_arms_slice != Some(m.arm_rows)
                        || m.arms.len() != arms.len()
                        || !std::ptr::eq(m.arms.as_ptr(), arms.as_ptr())
                    {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "match continuation entry mismatch",
                        ));
                    }
                    expected_matches += 1;
                }
                _ => {}
            }
        }
        if expected_literals != continuation.literals.len()
            || expected_nullary != continuation.nullary_constructs.len()
            || expected_matches != continuation.matches.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "ExtCContinuation length mismatch with bound expressions",
            ));
        }
        Ok(())
    }
}

impl<'s> ExtCProgramWriter for PendingExtCExpressionStage<'s> {
    fn reserve_expression(
        &mut self,
        source: &ExtExpr,
        owner: BodyOwner,
        parent: Option<(usize, usize)>,
    ) -> Result<(usize, usize), ExtTypeBuildError> {
        Self::reserve_expression(self, source, owner, parent)
    }

    fn name_id(&self, name: &str) -> Result<usize, ExtTypeBuildError> {
        Self::name_id(self, name)
    }

    fn signature_type(&self, site: T3UseSite) -> Result<TypeIdC, ExtTypeBuildError> {
        Self::signature_type(self, site)
    }

    fn constructor_use(&self, site: T3UseSite) -> Result<(TypeIdC, CtorIdC), ExtTypeBuildError> {
        Self::constructor_use(self, site)
    }

    fn write_expression(
        &mut self,
        id: usize,
        row: ExpressionRowC,
    ) -> Result<(), ExtTypeBuildError> {
        Self::write_expression(self, id, row)
    }

    fn append_code_children(&mut self, children: &[usize]) -> Result<SliceC, ExtTypeBuildError> {
        Self::append_code_children(self, children)
    }

    fn set_call_args(&mut self, id: usize, args: SliceC) -> Result<(), ExtTypeBuildError> {
        Self::set_call_args(self, id, args)
    }

    fn set_construct_fields(&mut self, id: usize, fields: SliceC) -> Result<(), ExtTypeBuildError> {
        Self::set_construct_fields(self, id, fields)
    }

    fn write_finance_expression(
        &mut self,
        id: usize,
        source: &ExtExpr,
        children: &[usize],
    ) -> Result<(), ExtTypeBuildError> {
        Self::write_finance_expression(self, id, source, children)
    }

    fn reserve_arm(&mut self) -> Result<usize, ExtTypeBuildError> {
        Self::reserve_arm(self)
    }

    fn visit_pattern_source(
        &mut self,
        owner: BodyOwner,
        expr: usize,
        arm: usize,
        ordinal: usize,
    ) -> Result<(), ExtTypeBuildError> {
        Self::visit_pattern_source(self, owner, expr, arm, ordinal)
    }

    fn pattern_traversal_invariant(&self) -> ExtTypeBuildError {
        Self::pattern_traversal_invariant(self)
    }

    fn append_pattern(
        &mut self,
        row: PatternRowC,
        children: &[usize],
    ) -> Result<usize, ExtTypeBuildError> {
        Self::append_pattern(self, row, children)
    }

    fn write_arm(&mut self, id: usize, row: ArmRowC) -> Result<(), ExtTypeBuildError> {
        Self::write_arm(self, id, row)
    }

    fn set_match_arms(&mut self, id: usize, arms: &[usize]) -> Result<SliceC, ExtTypeBuildError> {
        Self::set_match_arms(self, id, arms)
    }

    fn append_occurrence_path(&mut self, path: &[usize]) -> Result<SliceC, ExtTypeBuildError> {
        Self::append_occurrence_path(self, path)
    }

    fn write_occurrence(
        &mut self,
        id: usize,
        row: OccurrenceRowC,
    ) -> Result<(), ExtTypeBuildError> {
        Self::write_occurrence(self, id, row)
    }

    fn begin_function(
        &mut self,
        function: usize,
        arity: usize,
    ) -> Result<(usize, usize), ExtTypeBuildError> {
        Self::begin_function(self, function, arity)
    }

    fn append_parameter(&mut self, row: ParameterRowC) -> Result<(), ExtTypeBuildError> {
        Self::append_parameter(self, row)
    }

    fn write_function(&mut self, id: usize, row: FunctionRowC) -> Result<(), ExtTypeBuildError> {
        Self::write_function(self, id, row)
    }
}

fn ext_value_node_at_preorder<'a>(
    root: &'a ExtValue,
    target_ordinal: usize,
) -> Result<Option<&'a ExtValue>, ExtTypeBuildError> {
    let mut stack: Vec<&'a ExtValue> = vec![root];
    let mut current_ordinal: usize = 0;
    while let Some(node) = stack.pop() {
        if current_ordinal == target_ordinal {
            return Ok(Some(node));
        }
        current_ordinal = current_ordinal.checked_add(1).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "literal value preorder ordinal overflow",
            )
        })?;
        if let ExtValue::Constructor { fields, .. } = node {
            for field in fields.iter().rev() {
                stack.push(field);
            }
        }
    }
    Ok(None)
}

fn const_row_marker_free(row: &RawConstRow) -> bool {
    match row {
        RawConstRow::U128 { marker_free, .. }
        | RawConstRow::Bool { marker_free, .. }
        | RawConstRow::Nil { marker_free }
        | RawConstRow::Cons { marker_free, .. }
        | RawConstRow::Leaf { marker_free, .. }
        | RawConstRow::Branch { marker_free, .. }
        | RawConstRow::Constructor { marker_free, .. }
        | RawConstRow::U128OutOfRange { marker_free } => *marker_free,
    }
}

fn const_row_g5_type(row: &RawConstRow) -> TypeIdC {
    match row {
        RawConstRow::U128 { .. } | RawConstRow::U128OutOfRange { .. } => TypeIdC::UInt,
        RawConstRow::Bool { .. } => TypeIdC::Bool,
        RawConstRow::Nil { .. } | RawConstRow::Cons { .. } => TypeIdC::List,
        RawConstRow::Leaf { .. } | RawConstRow::Branch { .. } => TypeIdC::Tree,
        RawConstRow::Constructor { type_id, .. } => TypeIdC(type_id.0),
    }
}

impl<'s> PendingExtCExpressionStage<'s> {
    fn check_c5_functions_written(&self) -> Result<(), ExtTypeBuildError> {
        if self.active_function.is_some()
            || self.functions.len() != self.prepared.program.functions.len()
            || !self.functions.iter().all(Option::is_some)
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "C5 function emission incomplete",
            ));
        }
        Ok(())
    }

    fn check_c5_structure_for(&self, phase: LiteralSlotPhase) -> Result<(), ExtTypeBuildError> {
        self.check_c1_rows_written_for(phase)?;
        self.check_c2_code_children_linked()?;
        self.check_c3_match_arms_linked()?;
        self.check_c4_occurrences_written()?;
        self.check_c5_functions_written()
    }

    fn check_c6_in_progress(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c5_structure_for(LiteralSlotPhase::DuringC6)?;
        if self.active_literal.is_some() && self.active_nullary_construct.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "both C6 Literal and nullary Construct states active",
            ));
        }
        let mut first_unbound_literal: Option<usize> = None;
        let mut last_literal_constant: Option<usize> = None;
        let mut first_unfilled_nullary: Option<usize> = None;
        let mut first_nullary_constant: Option<usize> = None;
        let mut last_nullary_constant: Option<usize> = None;
        for (id, binding) in self.expr_bindings.iter().enumerate() {
            match (binding.source, self.expressions.get(id)) {
                (ExtExpr::Literal { .. }, Some(None)) => {
                    if first_unbound_literal.is_none() {
                        first_unbound_literal = Some(id);
                    }
                }
                (ExtExpr::Literal { .. }, Some(Some(ExpressionRowC::Literal { constant }))) => {
                    if first_unbound_literal.is_some() {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "C6 Literal slots not bound as an ascending ExprId prefix",
                        ));
                    }
                    if *constant >= self.constants.len()
                        || matches!(last_literal_constant, Some(prev) if *constant <= prev)
                    {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "C6 Literal constant out of bounds or out of canonical order",
                        ));
                    }
                    last_literal_constant = Some(*constant);
                }
                (ExtExpr::Literal { .. }, _) => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        LiteralSlotPhase::DuringC6.wrong_literal_slot(),
                    ));
                }
                (
                    ExtExpr::Construct { fields, .. },
                    Some(Some(ExpressionRowC::Construct { constant, .. })),
                ) => {
                    if !fields.is_empty() {
                        if constant.is_some() {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "non-nullary Construct unexpectedly carries constant",
                            ));
                        }
                    } else if let Some(cid) = constant {
                        if first_unfilled_nullary.is_some() {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "C6 nullary Construct constants not filled as an ascending ExprId prefix",
                            ));
                        }
                        if *cid >= self.constants.len()
                            || matches!(last_nullary_constant, Some(prev) if *cid <= prev)
                        {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "C6 nullary Construct constant out of bounds or out of canonical order",
                            ));
                        }
                        if first_nullary_constant.is_none() {
                            first_nullary_constant = Some(*cid);
                        }
                        last_nullary_constant = Some(*cid);
                    } else if first_unfilled_nullary.is_none() {
                        first_unfilled_nullary = Some(id);
                    }
                }
                (ExtExpr::Construct { .. }, _) => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "Construct expression slot is not a Construct row during C6",
                    ));
                }
                _ => {}
            }
        }
        if let Some(first_nullary) = first_nullary_constant {
            if first_unbound_literal.is_some()
                || matches!(last_literal_constant, Some(last) if first_nullary <= last)
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "nullary Construct constant filled before C6 literals completed",
                ));
            }
        }
        if let Some(active) = self.active_literal.as_ref() {
            if Some(active.expr_id) != first_unbound_literal {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "active C6 Literal is not the first unbound Literal slot",
                ));
            }
        }
        if let Some(active_nc) = self.active_nullary_construct.as_ref() {
            if first_unbound_literal.is_some() || Some(active_nc.expr_id) != first_unfilled_nullary
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "active C6 nullary Construct is not the first unfilled nullary Construct",
                ));
            }
        }
        Ok(())
    }

    fn check_c6_complete(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c5_structure_for(LiteralSlotPhase::AfterC6)?;
        if self.active_literal.is_some() || self.active_nullary_construct.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "C6 literal or nullary construct emission still active",
            ));
        }
        let mut last_literal_constant: Option<usize> = None;
        let mut first_nullary_constant: Option<usize> = None;
        let mut last_nullary_constant: Option<usize> = None;
        let mut nullary_count = 0usize;
        for (id, binding) in self.expr_bindings.iter().enumerate() {
            match (binding.source, self.expressions.get(id)) {
                (ExtExpr::Literal { value }, Some(Some(ExpressionRowC::Literal { constant }))) => {
                    let row = self.constants.get(*constant).ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "C6 literal constant id out of bounds",
                        )
                    })?;
                    if matches!(last_literal_constant, Some(prev) if *constant <= prev) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "C6 Literal root constants not in ascending Literal ExprId order",
                        ));
                    }
                    if matches!(value, ExtValue::Constructor { .. })
                        && !matches!(row, RawConstRow::Constructor { .. })
                    {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "C6 constructor Literal root is not a Constructor constant row",
                        ));
                    }
                    last_literal_constant = Some(*constant);
                }
                (ExtExpr::Literal { .. }, _) => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "C6 literal expression row not filled",
                    ));
                }
                (
                    ExtExpr::Construct { fields, .. },
                    Some(Some(ExpressionRowC::Construct {
                        type_id,
                        ctor_id,
                        constant,
                        ..
                    })),
                ) => {
                    if fields.is_empty() {
                        let cid = match constant {
                            Some(cid) if *cid < self.constants.len() => *cid,
                            _ => {
                                return Err(host_expr_err(
                                    HostViolationKind::WrongVariant,
                                    "C6 nullary Construct constant not filled",
                                ));
                            }
                        };
                        match self.constants.get(cid) {
                            Some(RawConstRow::Constructor {
                                marker_free: true,
                                type_id: row_type_id,
                                ctor_id: row_ctor_id,
                                fields: row_fields,
                            }) if row_type_id.0 == type_id.0
                                && row_ctor_id.0 == ctor_id.0
                                && row_fields.start.0 == 0
                                && row_fields.len == 0 => {}
                            _ => {
                                return Err(host_expr_err(
                                    HostViolationKind::WrongVariant,
                                    "C6 nullary Construct constant is not its nullary Constructor row",
                                ));
                            }
                        }
                        if matches!(last_nullary_constant, Some(prev) if cid <= prev) {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "C6 nullary Construct constants not in ascending Construct ExprId order",
                            ));
                        }
                        if first_nullary_constant.is_none() {
                            first_nullary_constant = Some(cid);
                        }
                        last_nullary_constant = Some(cid);
                        nullary_count += 1;
                    } else if constant.is_some() {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "non-nullary Construct unexpectedly carries constant",
                        ));
                    }
                }
                (_, Some(Some(_))) => {}
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "expression slot unfilled after C6",
                    ));
                }
            }
        }
        if let Some(first_nullary) = first_nullary_constant {
            if matches!(last_literal_constant, Some(last) if first_nullary <= last)
                || self.constants.len().checked_sub(nullary_count) != Some(first_nullary)
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "C6 nullary Construct constants are not the trailing block after all Literal constants",
                ));
            }
        }
        Ok(())
    }

    pub(super) fn begin_literal(
        &mut self,
        literal: &LiteralOccurrence<'_>,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        if self.active_literal.is_some() || self.active_nullary_construct.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "previous C6 literal or nullary construct still active",
            ));
        }
        let expr_id = literal.expr_id;
        let binding = self.expr_bindings.get(expr_id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "literal expression id out of bounds",
            )
        })?;
        if binding.owner != literal.owner || binding.local_expr != literal.local_expr {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "literal occurrence owner or local ordinal mismatch",
            ));
        }
        match binding.source {
            ExtExpr::Literal { value } if std::ptr::eq(value, literal.value) => {}
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "literal occurrence source pointer mismatch",
                ));
            }
        }
        if !matches!(self.expressions.get(expr_id), Some(None)) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "literal slot not in unfilled state before begin_literal",
            ));
        }
        for (idx, b) in self.expr_bindings.iter().enumerate() {
            if matches!(b.source, ExtExpr::Literal { .. }) {
                if idx < expr_id {
                    if !matches!(
                        self.expressions.get(idx),
                        Some(Some(ExpressionRowC::Literal { .. }))
                    ) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "literal begun out of ascending C6 ExprId order",
                        ));
                    }
                } else if idx > expr_id && !matches!(self.expressions.get(idx), Some(None)) {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "later literal slot already filled before earlier literal",
                    ));
                }
            } else if let Some(Some(ExpressionRowC::Construct {
                constant: Some(_), ..
            })) = self.expressions.get(idx)
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "nullary construct constant filled before C6 literals completed",
                ));
            }
        }
        let canonical_value = match binding.source {
            ExtExpr::Literal { value } => value,
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "bound literal expression is not ExtExpr::Literal",
                ));
            }
        };
        self.active_literal = Some(ActiveLiteralState {
            expr_id,
            owner: binding.owner,
            local_expr: binding.local_expr,
            const_start_len: self.constants.len(),
            visit_stack: vec![ExpectedValueStep::Enter {
                parent: None,
                source: canonical_value,
            }],
            entered_nodes: Vec::new(),
            root_const_id: None,
        });
        Ok(())
    }

    pub(super) fn visit_literal_value(
        &mut self,
        owner: BodyOwner,
        expr: usize,
        ordinal: usize,
        source: &ExtValue,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        let const_limit = self.bounds.constant_nodes;
        let active = self.active_literal.as_mut().ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "visit_literal_value called without active literal",
            )
        })?;
        if active.owner != owner
            || active.local_expr != expr
            || active.root_const_id.is_some()
            || ordinal != active.entered_nodes.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "visit_literal_value site or preorder ordinal mismatch",
            ));
        }
        let (parent, expected_source) = match active.visit_stack.last() {
            Some(ExpectedValueStep::Enter { parent, source }) => (*parent, *source),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "visit_literal_value called when value exit was expected",
                ));
            }
        };
        if !std::ptr::eq(source, expected_source) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "visit_literal_value source reference mismatch",
            ));
        }
        let planned_const_index = active.const_start_len.checked_add(ordinal).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ArithmeticUndefined,
                "planned constant row count overflow",
            )
        })?;
        if planned_const_index >= const_limit {
            return Err(core_stage_cap_err(
                CapacityKind::ConstantNodes,
                planned_const_index,
                const_limit,
            ));
        }
        active.visit_stack.pop();
        active.entered_nodes.push(EnteredValueNode {
            source: expected_source,
            parent,
            expected_children: Vec::new(),
            emitted_const_id: None,
        });
        active.visit_stack.push(ExpectedValueStep::Exit { ordinal });
        if let ExtValue::Constructor { fields, .. } = expected_source {
            for field in fields.iter().rev() {
                active.visit_stack.push(ExpectedValueStep::Enter {
                    parent: Some(ordinal),
                    source: field,
                });
            }
        }
        Ok(())
    }

    pub(super) fn literal_constructor_use(
        &self,
        site: T3UseSite,
    ) -> Result<(TypeIdC, CtorIdC), ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        let (owner, expr, value) = match site {
            T3UseSite::LiteralCtor { owner, expr, value } => (owner, expr, value),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "invalid T3 use site for literal_constructor_use",
                ));
            }
        };
        let (_, binding) = self.find_bound_expr(owner, expr)?;
        let root_val = match binding.source {
            ExtExpr::Literal { value } => value,
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "literal_constructor_use site does not address a Literal expression",
                ));
            }
        };
        let val_node = ext_value_node_at_preorder(root_val, value)?.ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "literal T3 use site value preorder ordinal out of bounds",
            )
        })?;
        let (expected_ctor_name, expected_fields) = match val_node {
            ExtValue::Constructor { ctor, fields, .. } => (ctor.as_str(), fields.as_slice()),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "literal_constructor_use value node is not ExtValue::Constructor",
                ));
            }
        };
        if let Some(active) = self.active_literal.as_ref() {
            if active.owner == owner && active.local_expr == expr {
                if let Some(entered) = active.entered_nodes.get(value) {
                    if !std::ptr::eq(entered.source, val_node) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "literal_constructor_use value node does not match active literal state",
                        ));
                    }
                }
            }
        }
        let expected_ctor_name_id = self.name_id(expected_ctor_name)?;
        let mut matched: Option<(TypeIdC, CtorIdC)> = None;
        for rec in &self.prepared.t3_records {
            if rec.site == site {
                if matched.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "duplicate T3 record for literal constructor use site",
                    ));
                }
                let ctor_id = rec.ctor_id.ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::WrongVariant,
                        "T3 literal constructor record missing resolved ctor_id",
                    )
                })?;
                if rec.type_id.0 < 4 || ctor_id.0 < 4 {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "T3 literal constructor record uses primitive or invalid TypeIdC/CtorIdC",
                    ));
                }
                let type_row = self
                    .prepared
                    .tables
                    .types
                    .get(rec.type_id.0)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "T3 literal constructor type_id out of bounds",
                        )
                    })?;
                if type_row.decl.is_none() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "T3 literal constructor type_id does not name a declared ADT",
                    ));
                }
                let ctor_row = self
                    .prepared
                    .tables
                    .constructors
                    .get(ctor_id.0)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "T3 literal constructor ctor_id out of bounds",
                        )
                    })?;
                let ctor_end = type_row
                    .ctor_start
                    .0
                    .checked_add(type_row.ctor_len)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "ADT constructor range overflow in literal_constructor_use",
                        )
                    })?;
                if ctor_row.owning_type != rec.type_id
                    || ctor_id.0 < type_row.ctor_start.0
                    || ctor_id.0 >= ctor_end
                    || ctor_end > self.prepared.tables.constructors.len()
                {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "T3 literal constructor ctor_id does not belong to owning type_id range",
                    ));
                }
                if ctor_row.name_id != expected_ctor_name_id {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "T3 literal constructor name_id mismatch with source literal node",
                    ));
                }
                if ctor_row.field_len != expected_fields.len() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "T3 literal constructor field count mismatch with source literal node",
                    ));
                }
                let field_end = ctor_row
                    .field_start
                    .checked_add(ctor_row.field_len)
                    .ok_or_else(|| {
                        host_expr_err(
                            HostViolationKind::ReadOutOfBounds,
                            "ADT constructor field range overflow in literal_constructor_use",
                        )
                    })?;
                if field_end > self.prepared.tables.ctor_fields.len() {
                    return Err(host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "ADT constructor field slice out of bounds in literal_constructor_use",
                    ));
                }
                matched = Some((rec.type_id, ctor_id));
            }
        }
        matched.ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "no T3 record found for literal constructor use site",
            )
        })
    }

    pub(super) fn append_core_literal(
        &mut self,
        source: &CoreValue,
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        if self.active_nullary_construct.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "append_core_literal called while a nullary Construct is active",
            ));
        }
        let (ordinal, parent) = {
            let active = self.active_literal.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_core_literal called without an active Literal expression",
                )
            })?;
            if active.root_const_id.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "active Literal already emitted its root constant",
                ));
            }
            let ordinal = match active.visit_stack.last() {
                Some(&ExpectedValueStep::Exit { ordinal }) => ordinal,
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "append_core_literal does not match current postorder exit step",
                    ));
                }
            };
            let node = active.entered_nodes.get(ordinal).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "active Literal exit node ordinal out of bounds",
                )
            })?;
            if node.emitted_const_id.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "active Literal node already emitted a constant row",
                ));
            }
            if !node.expected_children.is_empty() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "Core literal node cannot have child constants",
                ));
            }
            match node.source {
                ExtValue::Core { value } if std::ptr::eq(value, source) => {}
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "append_core_literal source reference does not match active ExtValue::Core",
                    ));
                }
            }
            (ordinal, node.parent)
        };
        let const_id = super::owner_prepare::emit_literal(
            source,
            &mut self.constants,
            &mut self.constant_fields,
            &self.bounds,
        )
        .map_err(ExtTypeBuildError::CoreTooling)?;
        let active = self.active_literal.as_mut().ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "active Literal missing after emit_literal",
            )
        })?;
        active.visit_stack.pop();
        active.entered_nodes[ordinal].emitted_const_id = Some(const_id);
        if let Some(par_ord) = parent {
            active.entered_nodes[par_ord]
                .expected_children
                .push(const_id);
        } else {
            if !active.visit_stack.is_empty() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "root Literal constant emitted before traversal stack emptied",
                ));
            }
            active.root_const_id = Some(const_id);
        }
        Ok(const_id)
    }

    pub(super) fn append_constructor_constant(
        &mut self,
        type_id: TypeIdC,
        ctor_id: CtorIdC,
        fields: &[usize],
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        enum Mode {
            Literal {
                ordinal: usize,
                parent: Option<usize>,
            },
            NullaryConstruct,
        }
        let mode = if self.active_literal.is_some() {
            if self.active_nullary_construct.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "both Literal and nullary Construct states active simultaneously",
                ));
            }
            let (owner, local_expr, ordinal, parent) = {
                let active = self.active_literal.as_ref().unwrap();
                if active.root_const_id.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "active Literal already emitted its root constant",
                    ));
                }
                let ordinal = match active.visit_stack.last() {
                    Some(&ExpectedValueStep::Exit { ordinal }) => ordinal,
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "append_constructor_constant does not match current postorder exit step",
                        ));
                    }
                };
                let node = active.entered_nodes.get(ordinal).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "active Literal exit node ordinal out of bounds",
                    )
                })?;
                if node.emitted_const_id.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "active Literal constructor node already emitted a constant row",
                    ));
                }
                let src_fields = match node.source {
                    ExtValue::Constructor { fields: sf, .. } => sf.as_slice(),
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "active Literal node is not ExtValue::Constructor",
                        ));
                    }
                };
                if node.expected_children.as_slice() != fields || fields.len() != src_fields.len() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "append_constructor_constant fields slice does not match emitted child constants",
                    ));
                }
                (active.owner, active.local_expr, ordinal, node.parent)
            };
            let (expected_ty, expected_ctor) =
                self.literal_constructor_use(T3UseSite::LiteralCtor {
                    owner,
                    expr: local_expr,
                    value: ordinal,
                })?;
            if expected_ty != type_id || expected_ctor != ctor_id {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_constructor_constant (type_id, ctor_id) mismatch with T3 literal record",
                ));
            }
            Mode::Literal { ordinal, parent }
        } else if let Some(active_nc) = self.active_nullary_construct.as_ref() {
            if active_nc.emitted_const_id.is_some() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "active nullary Construct already emitted its constant row",
                ));
            }
            if active_nc.type_id != type_id || active_nc.ctor_id != ctor_id {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_constructor_constant (type_id, ctor_id) mismatch with active nullary Construct",
                ));
            }
            if !fields.is_empty() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "nullary Construct constant must have zero fields",
                ));
            }
            Mode::NullaryConstruct
        } else {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "append_constructor_constant called without an active Literal or nullary Construct",
            ));
        };

        if type_id.0 < 4 || ctor_id.0 < 4 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor constant requires declared ADT TypeIdC and CtorIdC (>= 4)",
            ));
        }
        let type_row = self.prepared.tables.types.get(type_id.0).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "constructor constant type_id out of bounds",
            )
        })?;
        if type_row.decl.is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor constant type_id does not name a declared ADT",
            ));
        }
        let ctor_row = self
            .prepared
            .tables
            .constructors
            .get(ctor_id.0)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor constant ctor_id out of bounds",
                )
            })?;
        let ctor_end = type_row
            .ctor_start
            .0
            .checked_add(type_row.ctor_len)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "ADT constructor range overflow in append_constructor_constant",
                )
            })?;
        if ctor_row.owning_type != type_id
            || ctor_id.0 < type_row.ctor_start.0
            || ctor_id.0 >= ctor_end
            || ctor_end > self.prepared.tables.constructors.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor constant ctor_id does not belong to owning type_id",
            ));
        }
        if ctor_row.field_len != fields.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor constant field count does not match declared constructor arity",
            ));
        }
        let field_end = ctor_row
            .field_start
            .checked_add(ctor_row.field_len)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor constant field range overflow",
                )
            })?;
        if field_end > self.prepared.tables.ctor_fields.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "constructor constant field slice out of bounds",
            ));
        }

        let mut marker_free = true;
        for (j, &field_const_id) in fields.iter().enumerate() {
            let const_row = self.constants.get(field_const_id).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor constant child ConstantId out of bounds",
                )
            })?;
            let actual_ty = const_row_g5_type(const_row);
            let expected_ty = self.prepared.tables.ctor_fields[ctor_row.field_start + j].type_id;
            if actual_ty != expected_ty {
                return Err(ExtTypeBuildError::ExtAdmissionRejected {
                    reason: ExtTypeRejectReason::ConstantFieldType,
                    detail: format!(
                        "constructor constant field {} has type {:?}, expected {:?} (G-5)",
                        j, actual_ty, expected_ty
                    ),
                    locator: None,
                });
            }
            if !const_row_marker_free(const_row) {
                marker_free = false;
            }
        }

        if self.constants.len() >= self.bounds.constant_nodes {
            return Err(core_stage_cap_err(
                CapacityKind::ConstantNodes,
                self.constants.len(),
                self.bounds.constant_nodes,
            ));
        }
        let field_slice = if fields.is_empty() {
            RawConstFieldSlice {
                start: RawConstFieldPos(0),
                len: 0,
            }
        } else {
            let start = self.constant_fields.len();
            for &f in fields {
                if self.constant_fields.len() >= self.bounds.constant_field_slots {
                    return Err(core_stage_cap_err(
                        CapacityKind::ConstantFieldSlots,
                        self.constant_fields.len(),
                        self.bounds.constant_field_slots,
                    ));
                }
                self.constant_fields.push(f);
            }
            RawConstFieldSlice {
                start: RawConstFieldPos(start),
                len: fields.len(),
            }
        };
        let const_id = self.constants.len();
        self.constants.push(RawConstRow::Constructor {
            marker_free,
            type_id: RawTypeId(type_id.0),
            ctor_id: RawCtorId(ctor_id.0),
            fields: field_slice,
        });

        match mode {
            Mode::Literal { ordinal, parent } => {
                let active = self.active_literal.as_mut().unwrap();
                active.visit_stack.pop();
                active.entered_nodes[ordinal].emitted_const_id = Some(const_id);
                if let Some(par_ord) = parent {
                    active.entered_nodes[par_ord]
                        .expected_children
                        .push(const_id);
                } else {
                    if !active.visit_stack.is_empty() {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "root Literal constructor constant emitted before stack emptied",
                        ));
                    }
                    active.root_const_id = Some(const_id);
                }
            }
            Mode::NullaryConstruct => {
                let active_nc = self.active_nullary_construct.as_mut().unwrap();
                active_nc.emitted_const_id = Some(const_id);
            }
        }
        Ok(const_id)
    }

    pub(super) fn finish_literal(
        &mut self,
        expr_id: usize,
        constant: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        if self.active_nullary_construct.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_literal called while a nullary Construct is active",
            ));
        }
        let active = self.active_literal.as_ref().ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_literal called without an active Literal expression",
            )
        })?;
        if active.expr_id != expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_literal expr_id does not match active Literal expression",
            ));
        }
        if !active.visit_stack.is_empty()
            || !active
                .entered_nodes
                .iter()
                .all(|n| n.emitted_const_id.is_some())
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_literal called before all Literal value nodes emitted constants",
            ));
        }
        if active.root_const_id != Some(constant) || constant >= self.constants.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_literal constant does not match active Literal root constant",
            ));
        }
        fill_reserved_once(
            &mut self.expressions,
            expr_id,
            ExpressionRowC::Literal { constant },
        )?;
        self.active_literal = None;
        Ok(())
    }

    pub(super) fn begin_nullary_construct(
        &mut self,
        source: &NullaryConstructOccurrence,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        if self.active_literal.is_some() || self.active_nullary_construct.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_nullary_construct called while another C6 literal or nullary construct is active",
            ));
        }
        for (idx, b) in self.expr_bindings.iter().enumerate() {
            if matches!(b.source, ExtExpr::Literal { .. }) {
                if !matches!(
                    self.expressions.get(idx),
                    Some(Some(ExpressionRowC::Literal { .. }))
                ) {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "begin_nullary_construct called before all Literal expressions were finished",
                    ));
                }
            }
        }
        let binding = self.expr_bindings.get(source.expr_id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "begin_nullary_construct expr_id out of bounds",
            )
        })?;
        match binding.source {
            ExtExpr::Construct { fields, .. } if fields.is_empty() => {}
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "begin_nullary_construct expr_id does not address a nullary ExtExpr::Construct",
                ));
            }
        }
        let (expected_ty, expected_ctor) = self.constructor_use(T3UseSite::Construct {
            owner: binding.owner,
            expr: binding.local_expr,
        })?;
        if expected_ty != source.type_id || expected_ctor != source.ctor_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_nullary_construct (type_id, ctor_id) does not match resolved Construct site",
            ));
        }
        match self.expressions.get(source.expr_id) {
            Some(Some(ExpressionRowC::Construct {
                type_id,
                ctor_id,
                fields,
                constant: None,
            })) if *type_id == source.type_id
                && *ctor_id == source.ctor_id
                && *fields == SliceC::empty() => {}
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "begin_nullary_construct target expression is not an unfilled nullary Construct row",
                ));
            }
        }
        for (idx, b) in self.expr_bindings.iter().enumerate() {
            if matches!(b.source, ExtExpr::Construct { fields, .. } if fields.is_empty()) {
                if idx < source.expr_id {
                    if !matches!(
                        self.expressions.get(idx),
                        Some(Some(ExpressionRowC::Construct {
                            constant: Some(_),
                            ..
                        }))
                    ) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "earlier nullary Construct must be finished before begin_nullary_construct",
                        ));
                    }
                } else if idx > source.expr_id {
                    if !matches!(
                        self.expressions.get(idx),
                        Some(Some(ExpressionRowC::Construct { constant: None, .. }))
                    ) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "later nullary Construct already populated before begin_nullary_construct",
                        ));
                    }
                }
            }
        }
        self.active_nullary_construct = Some(ActiveNullaryConstructState {
            expr_id: source.expr_id,
            type_id: source.type_id,
            ctor_id: source.ctor_id,
            emitted_const_id: None,
        });
        Ok(())
    }

    pub(super) fn finish_nullary_construct(
        &mut self,
        expr_id: usize,
        constant: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_in_progress()?;
        if self.active_literal.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_nullary_construct called while a Literal is active",
            ));
        }
        let active_nc = self.active_nullary_construct.as_ref().ok_or_else(|| {
            host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_nullary_construct called without an active nullary Construct",
            )
        })?;
        if active_nc.expr_id != expr_id || active_nc.emitted_const_id != Some(constant) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_nullary_construct arguments do not match active nullary Construct state",
            ));
        }
        match self.constants.get(constant) {
            Some(RawConstRow::Constructor {
                marker_free: true,
                type_id,
                ctor_id,
                fields,
            }) if type_id.0 == active_nc.type_id.0
                && ctor_id.0 == active_nc.ctor_id.0
                && fields.start.0 == 0
                && fields.len == 0 => {}
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_nullary_construct constant is not a valid nullary constructor constant row",
                ));
            }
        }
        match self.expressions.get_mut(expr_id) {
            Some(Some(ExpressionRowC::Construct {
                type_id,
                ctor_id,
                fields,
                constant: slot,
            })) if *type_id == active_nc.type_id
                && *ctor_id == active_nc.ctor_id
                && *fields == SliceC::empty()
                && slot.is_none() =>
            {
                *slot = Some(constant);
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_nullary_construct expression slot is not an unfilled nullary Construct row",
                ));
            }
        }
        self.active_nullary_construct = None;
        Ok(())
    }

    pub(super) fn literal_traversal_invariant(&self) -> ExtTypeBuildError {
        host_expr_err(
            HostViolationKind::WrongVariant,
            "literal traversal terminated without emitting root constant",
        )
    }
}

impl<'s> ExtC6ProgramWriter for PendingExtCExpressionStage<'s> {
    fn begin_literal(&mut self, literal: &LiteralOccurrence<'_>) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::begin_literal(self, literal)
    }

    fn visit_literal_value(
        &mut self,
        owner: BodyOwner,
        expr: usize,
        ordinal: usize,
        source: &ExtValue,
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::visit_literal_value(self, owner, expr, ordinal, source)
    }

    fn literal_constructor_use(
        &self,
        site: T3UseSite,
    ) -> Result<(TypeIdC, CtorIdC), ExtTypeBuildError> {
        PendingExtCExpressionStage::literal_constructor_use(self, site)
    }

    fn append_core_literal(&mut self, source: &CoreValue) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::append_core_literal(self, source)
    }

    fn append_constructor_constant(
        &mut self,
        type_id: TypeIdC,
        ctor_id: CtorIdC,
        fields: &[usize],
    ) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::append_constructor_constant(self, type_id, ctor_id, fields)
    }

    fn finish_literal(&mut self, expr_id: usize, constant: usize) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::finish_literal(self, expr_id, constant)
    }

    fn begin_nullary_construct(
        &mut self,
        source: &NullaryConstructOccurrence,
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::begin_nullary_construct(self, source)
    }

    fn finish_nullary_construct(
        &mut self,
        expr_id: usize,
        constant: usize,
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::finish_nullary_construct(self, expr_id, constant)
    }

    fn literal_traversal_invariant(&self) -> ExtTypeBuildError {
        PendingExtCExpressionStage::literal_traversal_invariant(self)
    }
}

fn p_ext_cap_err(
    kind: ExtTypeCapKind,
    current_len: usize,
    limit: usize,
    locator: Option<LocatorC>,
) -> ExtTypeBuildError {
    let needed = match (current_len as u128).checked_add(1) {
        Some(n) => NeededNatC::Exact(n),
        None => NeededNatC::AboveU128,
    };
    ExtTypeBuildError::ExtCapacityNotAdmitted {
        kind,
        needed,
        limit: limit as u128,
        locator,
    }
}

fn ext_pattern_is_planned(pat: &ExtPattern) -> bool {
    let mut stack = vec![pat];
    while let Some(node) = stack.pop() {
        match node {
            ExtPattern::Dedicated { .. } => {}
            ExtPattern::Wildcard | ExtPattern::Ctor { .. } => return true,
        }
    }
    false
}

impl<'s> PendingExtCExpressionStage<'s> {
    pub(super) fn begin_match(
        &mut self,
        source: &MatchOccurrence<'_>,
    ) -> Result<Option<Vec<PlanArmSourceC>>, ExtTypeBuildError> {
        self.check_c6_complete()?;
        if self.active_match_plan.is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_match called while a previous planned Match is still active",
            ));
        }
        if self.plan_nodes.len() != self.pending_plan_nodes.len()
            || self.plan_nodes.iter().any(|n| n.is_none())
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_match called with unfinished plan nodes in arena",
            ));
        }
        let binding = self.expr_bindings.get(source.expr_id).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "begin_match expr_id out of bounds",
            )
        })?;
        if binding.owner != source.owner || binding.local_expr != source.local_expr {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_match owner or local_expr mismatch with bound expression",
            ));
        }
        let expected_arms = match binding.source {
            ExtExpr::Match { arms, .. } => arms.as_slice(),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "begin_match expr_id does not address an ExtExpr::Match",
                ));
            }
        };
        if expected_arms.len() != source.arms.len()
            || !std::ptr::eq(expected_arms.as_ptr(), source.arms.as_ptr())
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_match source arms slice pointer mismatch",
            ));
        }
        if binding.match_arms_slice != Some(source.arm_rows) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_match arm_rows slice does not match C3 bound match_arms_slice",
            ));
        }
        match self.expressions.get(source.expr_id) {
            Some(Some(ExpressionRowC::Match {
                arms, plan: None, ..
            })) if *arms == source.arm_rows => {}
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "begin_match target Match row missing, mismatched, or already has a plan",
                ));
            }
        }
        if let Some(&last_id) = self.p_visited_matches.last() {
            if source.expr_id <= last_id {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "begin_match called out of ascending Match ExprId order",
                ));
            }
        }
        let mut prior_matches = 0usize;
        for (idx, b) in self.expr_bindings.iter().enumerate() {
            if let ExtExpr::Match { arms: m_arms, .. } = b.source {
                if idx < source.expr_id {
                    if self.p_visited_matches.get(prior_matches) != Some(&idx) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "earlier Match expression skipped before begin_match",
                        ));
                    }
                    prior_matches += 1;
                    let earlier_planned = m_arms.iter().any(|a| ext_pattern_is_planned(&a.pattern));
                    let has_plan = matches!(
                        self.expressions.get(idx),
                        Some(Some(ExpressionRowC::Match { plan: Some(_), .. }))
                    );
                    if earlier_planned != has_plan {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "earlier Match plan state inconsistent with planned-arm predicate",
                        ));
                    }
                } else if idx > source.expr_id {
                    if !matches!(
                        self.expressions.get(idx),
                        Some(Some(ExpressionRowC::Match { plan: None, .. }))
                    ) {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "later Match expression already populated before begin_match",
                        ));
                    }
                }
            }
        }
        if prior_matches != self.p_visited_matches.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "visited Match history length mismatch in begin_match",
            ));
        }
        if source.arm_rows.len != source.arms.len()
            || source.arm_rows != SliceC::normalized(source.arm_rows.start, source.arm_rows.len)
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "begin_match arm_rows slice length or normalization mismatch",
            ));
        }
        let arm_end = source
            .arm_rows
            .start
            .checked_add(source.arm_rows.len)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "begin_match arm_rows range overflow",
                )
            })?;
        if arm_end > self.arms.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "begin_match arm_rows slice out of bounds",
            ));
        }
        let mut plan_arms = Vec::with_capacity(source.arm_rows.len);
        let mut is_planned = false;
        for (i, src_arm) in source.arms.iter().enumerate() {
            let arm_id = source.arm_rows.start + i;
            let arm_row = self
                .arms
                .get(arm_id)
                .and_then(|r| r.as_ref())
                .ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::WrongVariant,
                        "begin_match referenced C3 arm slot is unfilled",
                    )
                })?;
            let pat_row = self.patterns.get(arm_row.pattern).ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "begin_match arm pattern row id out of bounds",
                )
            })?;
            let src_planned = ext_pattern_is_planned(&src_arm.pattern);
            let row_planned = matches!(
                pat_row,
                PatternRowC::Wildcard | PatternRowC::Constructor { .. }
            );
            if src_planned != row_planned {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "arm root PatternRowC disagrees with source ExtPattern planned status",
                ));
            }
            if row_planned {
                is_planned = true;
            }
            plan_arms.push(PlanArmSourceC {
                arm: arm_id,
                pattern: arm_row.pattern,
            });
        }
        self.p_visited_matches.push(source.expr_id);
        if !is_planned {
            return Ok(None);
        }
        let active_source_arms = match binding.source {
            ExtExpr::Match { arms, .. } => arms.as_slice(),
            _ => unreachable!(),
        };
        self.active_match_plan = Some(ActiveMatchPlanState {
            match_expr_id: source.expr_id,
            owner: source.owner,
            local_expr: source.local_expr,
            arm_rows: source.arm_rows,
            source_arms: active_source_arms,
            arms_checked: 0,
            plan_block_start: self.plan_nodes.len(),
        });
        Ok(Some(plan_arms))
    }

    pub(super) fn pattern(&self, id: usize) -> Result<PatternRowC, ExtTypeBuildError> {
        self.check_c6_complete()?;
        if self.active_match_plan.is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "pattern read called without an active planned Match",
            ));
        }
        self.patterns.get(id).copied().ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "pattern row id out of bounds in Pass P",
            )
        })
    }

    pub(super) fn pattern_children(&self, slice: SliceC) -> Result<Vec<usize>, ExtTypeBuildError> {
        self.check_c6_complete()?;
        if self.active_match_plan.is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "pattern_children read called without an active planned Match",
            ));
        }
        if slice.len == 0 && slice.start != 0 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "unnormalized empty pattern_children slice",
            ));
        }
        let end = slice.start.checked_add(slice.len).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "pattern_children slice range overflow",
            )
        })?;
        if end > self.pattern_children.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "pattern_children slice out of bounds",
            ));
        }
        let children = &self.pattern_children[slice.start..end];
        for &child_id in children {
            if child_id >= self.patterns.len() {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pattern_children element references out-of-bounds pattern row",
                ));
            }
        }
        Ok(children.to_vec())
    }

    pub(super) fn check_arm_binders(
        &mut self,
        source: &MatchOccurrence<'_>,
        arm: usize,
        binders: &[PatternBindingC],
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;
        let src_pat = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "check_arm_binders called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source.expr_id
                || active.owner != source.owner
                || active.local_expr != source.local_expr
                || active.arm_rows != source.arm_rows
                || active.source_arms.len() != source.arms.len()
                || !std::ptr::eq(active.source_arms.as_ptr(), source.arms.as_ptr())
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "check_arm_binders source MatchOccurrence does not match active planned Match",
                ));
            }
            if self.plan_nodes.len() != active.plan_block_start
                || self.pending_plan_nodes.len() != active.plan_block_start
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "check_arm_binders called after plan node emission began",
                ));
            }
            if arm != active.arms_checked || arm >= active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "check_arm_binders called out of ascending source arm order",
                ));
            }
            &active.source_arms[arm].pattern
        };

        let mut expected_binders: Vec<(usize, Vec<usize>)> = Vec::new();
        let mut stack: Vec<(&ExtPattern, Vec<usize>)> = vec![(src_pat, Vec::new())];
        while let Some((pat, path)) = stack.pop() {
            match pat {
                ExtPattern::Wildcard => {}
                ExtPattern::Dedicated { pattern } => match pattern {
                    CorePattern::PVar { x } => {
                        expected_binders.push((self.name_id(x)?, path));
                    }
                    CorePattern::PNil => {}
                    CorePattern::PCons { x, y } => {
                        let mut p0 = path.clone();
                        p0.push(0);
                        let mut p1 = path;
                        p1.push(1);
                        expected_binders.push((self.name_id(x)?, p0));
                        expected_binders.push((self.name_id(y)?, p1));
                    }
                    CorePattern::PLeaf { x } => {
                        let mut p0 = path;
                        p0.push(0);
                        expected_binders.push((self.name_id(x)?, p0));
                    }
                    CorePattern::PBranch { x, y } => {
                        let mut p0 = path.clone();
                        p0.push(0);
                        let mut p1 = path;
                        p1.push(1);
                        expected_binders.push((self.name_id(x)?, p0));
                        expected_binders.push((self.name_id(y)?, p1));
                    }
                },
                ExtPattern::Ctor { fields, .. } => {
                    for (pos, field) in fields.iter().enumerate().rev() {
                        let mut child_path = path.clone();
                        child_path.push(pos);
                        stack.push((field, child_path));
                    }
                }
            }
        }
        if expected_binders.len() != binders.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "check_arm_binders binder count does not match source arm pattern",
            ));
        }
        for ((exp_binder, exp_path), actual) in expected_binders.iter().zip(binders.iter()) {
            if *exp_binder != actual.binder || exp_path.as_slice() != actual.path.as_slice() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "check_arm_binders binder or static path mismatch with source arm pattern",
                ));
            }
            for &step in &actual.path {
                if step >= self.bounds.max_arity {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "pattern binder path step exceeds max_arity",
                    ));
                }
            }
        }

        let mut seen: Vec<usize> = Vec::with_capacity(binders.len());
        for b in binders {
            if b.binder >= self.prepared.interned_names.len() {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pattern binder NameId out of bounds",
                ));
            }
            if seen.contains(&b.binder) {
                return Err(ExtTypeBuildError::ExtAdmissionRejected {
                    reason: ExtTypeRejectReason::DuplicatePatternBinder,
                    detail: format!(
                        "duplicate pattern binder '{}' in planned match arm {} (G-4)",
                        self.prepared.interned_names[b.binder], arm
                    ),
                    locator: None,
                });
            }
            seen.push(b.binder);
        }

        self.active_match_plan.as_mut().unwrap().arms_checked += 1;
        Ok(())
    }

    pub(super) fn constructor_arity(&self, ctor: CtorIdC) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_complete()?;
        if self.active_match_plan.is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor_arity called without an active planned Match",
            ));
        }
        let ctor_row = self
            .prepared
            .tables
            .constructors
            .get(ctor.0)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor_arity CtorIdC out of bounds",
                )
            })?;
        let type_row = self
            .prepared
            .tables
            .types
            .get(ctor_row.owning_type.0)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor_arity owning TypeIdC out of bounds",
                )
            })?;
        let ctor_end = type_row
            .ctor_start
            .0
            .checked_add(type_row.ctor_len)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor_arity owning type constructor slice overflow",
                )
            })?;
        if ctor.0 < type_row.ctor_start.0
            || ctor.0 >= ctor_end
            || ctor_end > self.prepared.tables.constructors.len()
            || ctor_row.source_order != ctor.0 - type_row.ctor_start.0
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor_arity row does not match owning type constructor slice",
            ));
        }
        if ctor.0 < 4 {
            let valid_builtin = match ctor.0 {
                0 => ctor_row.owning_type == TypeIdC::List && ctor_row.field_len == 0,
                1 => ctor_row.owning_type == TypeIdC::List && ctor_row.field_len == 2,
                2 => ctor_row.owning_type == TypeIdC::Tree && ctor_row.field_len == 1,
                3 => ctor_row.owning_type == TypeIdC::Tree && ctor_row.field_len == 2,
                _ => false,
            };
            if !valid_builtin {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "builtin constructor row corrupted in constructor_arity",
                ));
            }
        } else if ctor_row.owning_type.0 < 4 || type_row.decl.is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "user constructor row owned by non-declared type in constructor_arity",
            ));
        }
        let field_end = ctor_row
            .field_start
            .checked_add(ctor_row.field_len)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "constructor_arity field slice overflow",
                )
            })?;
        if ctor_row.field_len > self.bounds.max_arity
            || field_end > self.prepared.tables.ctor_fields.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "constructor_arity field_len exceeds max_arity or ctor_fields bounds",
            ));
        }
        Ok(ctor_row.field_len)
    }

    pub(super) fn reserve_switch(
        &mut self,
        source_match: usize,
        path: &[usize],
        type_id: TypeIdC,
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_complete()?;
        let plan_block_start = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_switch called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source_match {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_switch source_match does not match active planned Match",
                ));
            }
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_switch called before all match arms completed G-4 binder checks",
                ));
            }
            active.plan_block_start
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in reserve_switch",
            ));
        }
        if self.pending_plan_nodes.len() > plan_block_start + 1 {
            if !self.pending_plan_nodes.last().unwrap().has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "previous non-root plan node was not linked to a parent before reserve_switch",
                ));
            }
        }
        let type_row = self.prepared.tables.types.get(type_id.0).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "reserve_switch type_id out of bounds",
            )
        })?;
        if type_row.ctor_len == 0 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "reserve_switch type_id has zero constructors",
            ));
        }
        let ctor_end = type_row
            .ctor_start
            .0
            .checked_add(type_row.ctor_len)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "reserve_switch type constructor slice overflow",
                )
            })?;
        if ctor_end > self.prepared.tables.constructors.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "reserve_switch type constructor slice out of bounds",
            ));
        }
        for &step in path {
            if step >= self.bounds.max_arity {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_switch path step exceeds max_arity",
                ));
            }
        }

        // 1. Reserve plan node FIRST against bounds.match_plan_rows
        let node_id = self.plan_nodes.len();
        if node_id >= self.bounds.match_plan_rows {
            return Err(p_ext_cap_err(
                ExtTypeCapKind::MatchPlanRows,
                node_id,
                self.bounds.match_plan_rows,
                None,
            ));
        }
        self.plan_nodes.push(None);

        // 2. Reserve complete absent case block for type_row.ctor_len against ext_capacities.plan_case_slots
        let case_start = self.plan_cases.len();
        let case_len = type_row.ctor_len;
        for _ in 0..case_len {
            if self.plan_cases.len() >= self.ext_capacities.plan_case_slots {
                return Err(p_ext_cap_err(
                    ExtTypeCapKind::PlanCaseSlots,
                    self.plan_cases.len(),
                    self.ext_capacities.plan_case_slots,
                    None,
                ));
            }
            self.plan_cases.push(None);
        }
        let cases_slice = SliceC::normalized(case_start, case_len);

        // 3. Append private unshared path cells against ext_capacities.field_path_slots
        let path_slice = if path.is_empty() {
            SliceC::empty()
        } else {
            let path_start = self.field_paths.len();
            for &step in path {
                if self.field_paths.len() >= self.ext_capacities.field_path_slots {
                    return Err(p_ext_cap_err(
                        ExtTypeCapKind::FieldPathSlots,
                        self.field_paths.len(),
                        self.ext_capacities.field_path_slots,
                        None,
                    ));
                }
                self.field_paths.push(step);
            }
            SliceC::normalized(path_start, path.len())
        };

        self.pending_plan_nodes.push(PendingPlanNodeState {
            source_match,
            has_incoming_parent: false,
            kind: PendingPlanNodeKind::Switch {
                path: path_slice,
                type_id,
                ctor_start: type_row.ctor_start,
                cases: cases_slice,
                last_case_offset: None,
                finished: false,
            },
        });
        Ok(node_id)
    }

    pub(super) fn set_switch_case(
        &mut self,
        node: usize,
        ctor: CtorIdC,
        target: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;
        let (match_expr_id, plan_block_start) = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_switch_case called without an active planned Match",
                )
            })?;
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_switch_case called before G-4 arm binder checks completed",
                ));
            }
            (active.match_expr_id, active.plan_block_start)
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in set_switch_case",
            ));
        }
        if node < plan_block_start || node >= self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "set_switch_case switch node out of current match plan block bounds",
            ));
        }
        if target <= node || target >= self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case target must be a forward node in current match plan block",
            ));
        }
        if target != self.pending_plan_nodes.len() - 1 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case target must be the freshly reserved/appended preorder root",
            ));
        }
        for idx in (node + 1)..target {
            if self.plan_nodes[idx].is_none() || !self.pending_plan_nodes[idx].has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "earlier case subtree node unfinished or unlinked before set_switch_case",
                ));
            }
        }
        let target_state = &self.pending_plan_nodes[target];
        if target_state.source_match != match_expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case target belongs to a different Match expression",
            ));
        }
        if target_state.has_incoming_parent {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case target already has an incoming parent link",
            ));
        }

        let node_state = &self.pending_plan_nodes[node];
        if node_state.source_match != match_expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case switch node belongs to a different Match expression",
            ));
        }
        if node > plan_block_start && !node_state.has_incoming_parent {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case switch node was not linked to its parent",
            ));
        }
        if self.plan_nodes[node].is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case called on an already finished Switch node",
            ));
        }
        let (type_id, ctor_start, cases, last_case_offset) = match node_state.kind {
            PendingPlanNodeKind::Switch {
                type_id,
                ctor_start,
                cases,
                last_case_offset,
                finished: false,
                ..
            } => (type_id, ctor_start, cases, last_case_offset),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_switch_case node is not an unfinished Switch",
                ));
            }
        };
        if ctor.0 < ctor_start.0 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case ctor precedes Switch type ctor_start",
            ));
        }
        let offset = ctor.0 - ctor_start.0;
        if offset >= cases.len {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case ctor offset outside Switch type constructor slice",
            ));
        }
        let ctor_row = self
            .prepared
            .tables
            .constructors
            .get(ctor.0)
            .ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "set_switch_case ctor out of bounds",
                )
            })?;
        if ctor_row.owning_type != type_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "set_switch_case ctor does not belong to Switch type_id",
            ));
        }
        if let Some(prev_offset) = last_case_offset {
            if offset <= prev_offset {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_switch_case called out of strictly ascending CtorIdC order",
                ));
            }
        }
        let slot_index = cases.start.checked_add(offset).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "set_switch_case slot_index overflow",
            )
        })?;
        match self.plan_cases.get(slot_index) {
            Some(None) => {}
            Some(Some(_)) => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "set_switch_case case slot already filled",
                ));
            }
            None => {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "set_switch_case case slot index out of bounds",
                ));
            }
        }
        for i in 0..cases.len {
            if let Some(Some(prev_target)) = self.plan_cases.get(cases.start + i) {
                if i >= offset || *prev_target >= target {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "set_switch_case case target order violation",
                    ));
                }
            }
        }

        self.plan_cases[slot_index] = Some(target);
        if let PendingPlanNodeKind::Switch {
            last_case_offset: ref mut last_off,
            ..
        } = self.pending_plan_nodes[node].kind
        {
            *last_off = Some(offset);
        }
        self.pending_plan_nodes[target].has_incoming_parent = true;
        Ok(())
    }

    pub(super) fn finish_switch(
        &mut self,
        node: usize,
        default: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;
        let (match_expr_id, plan_block_start) = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_switch called without an active planned Match",
                )
            })?;
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_switch called before G-4 arm binder checks completed",
                ));
            }
            (active.match_expr_id, active.plan_block_start)
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in finish_switch",
            ));
        }
        if node < plan_block_start || node >= self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "finish_switch node out of current match plan block bounds",
            ));
        }
        if default <= node || default >= self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch default must be a forward node in current match plan block",
            ));
        }
        if default != self.pending_plan_nodes.len() - 1 {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch default must be the freshly reserved/appended preorder root",
            ));
        }
        for idx in (node + 1)..default {
            if self.plan_nodes[idx].is_none() || !self.pending_plan_nodes[idx].has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "case subtree node unfinished or unlinked before finish_switch",
                ));
            }
        }
        let default_state = &self.pending_plan_nodes[default];
        if default_state.source_match != match_expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch default belongs to a different Match expression",
            ));
        }
        if default_state.has_incoming_parent {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch default node already has an incoming parent link",
            ));
        }

        let node_state = &self.pending_plan_nodes[node];
        if node_state.source_match != match_expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch switch node belongs to a different Match expression",
            ));
        }
        if node > plan_block_start && !node_state.has_incoming_parent {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch switch node was not linked to its parent",
            ));
        }
        if self.plan_nodes[node].is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch called on an already finished Switch node",
            ));
        }
        let (path, type_id, ctor_start, cases, last_case_offset) = match node_state.kind {
            PendingPlanNodeKind::Switch {
                path,
                type_id,
                ctor_start,
                cases,
                last_case_offset,
                finished: false,
            } => (path, type_id, ctor_start, cases, last_case_offset),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_switch node is not an unfinished Switch",
                ));
            }
        };
        if last_case_offset.is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch called on Switch with no present constructor cases",
            ));
        }
        let cases_end = cases.start.checked_add(cases.len).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "finish_switch cases slice overflow",
            )
        })?;
        if cases_end > self.plan_cases.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "finish_switch cases slice out of bounds",
            ));
        }
        let mut any_case_present = false;
        for slot in &self.plan_cases[cases.start..cases_end] {
            if let Some(case_target) = *slot {
                any_case_present = true;
                if case_target <= node || case_target >= default {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "finish_switch case target outside (node, default) preorder range",
                    ));
                }
            }
        }
        if !any_case_present {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_switch Switch has empty case block",
            ));
        }

        fill_reserved_once(
            &mut self.plan_nodes,
            node,
            PlanNodeC::Switch {
                source_match: match_expr_id,
                path,
                type_id,
                cases,
                default,
            },
        )?;
        self.pending_plan_nodes[node].kind = PendingPlanNodeKind::Switch {
            path,
            type_id,
            ctor_start,
            cases,
            last_case_offset,
            finished: true,
        };
        self.pending_plan_nodes[default].has_incoming_parent = true;
        Ok(())
    }

    pub(super) fn reserve_bind(
        &mut self,
        source_match: usize,
        path: &[usize],
        binder: usize,
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_complete()?;
        let plan_block_start = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_bind called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source_match {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_bind source_match does not match active planned Match",
                ));
            }
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_bind called before G-4 arm binder checks completed",
                ));
            }
            active.plan_block_start
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in reserve_bind",
            ));
        }
        if self.pending_plan_nodes.len() > plan_block_start + 1 {
            if !self.pending_plan_nodes.last().unwrap().has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "previous non-root plan node was not linked before reserve_bind",
                ));
            }
        }
        if binder >= self.prepared.interned_names.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "reserve_bind binder NameId out of bounds",
            ));
        }
        for &step in path {
            if step >= self.bounds.max_arity {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "reserve_bind path step exceeds max_arity",
                ));
            }
        }
        let node_id = self.plan_nodes.len();
        if node_id >= self.bounds.match_plan_rows {
            return Err(p_ext_cap_err(
                ExtTypeCapKind::MatchPlanRows,
                node_id,
                self.bounds.match_plan_rows,
                None,
            ));
        }
        self.plan_nodes.push(None);
        let path_slice = if path.is_empty() {
            SliceC::empty()
        } else {
            let path_start = self.field_paths.len();
            for &step in path {
                if self.field_paths.len() >= self.ext_capacities.field_path_slots {
                    return Err(p_ext_cap_err(
                        ExtTypeCapKind::FieldPathSlots,
                        self.field_paths.len(),
                        self.ext_capacities.field_path_slots,
                        None,
                    ));
                }
                self.field_paths.push(step);
            }
            SliceC::normalized(path_start, path.len())
        };
        self.pending_plan_nodes.push(PendingPlanNodeState {
            source_match,
            has_incoming_parent: false,
            kind: PendingPlanNodeKind::Bind {
                path: path_slice,
                binder,
                finished: false,
            },
        });
        Ok(node_id)
    }

    pub(super) fn finish_bind(
        &mut self,
        node: usize,
        next: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;
        let (match_expr_id, plan_block_start) = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_bind called without an active planned Match",
                )
            })?;
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_bind called before G-4 arm binder checks completed",
                ));
            }
            (active.match_expr_id, active.plan_block_start)
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in finish_bind",
            ));
        }
        if node < plan_block_start || node >= self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "finish_bind node out of current match plan block bounds",
            ));
        }
        if next <= node
            || next >= self.pending_plan_nodes.len()
            || next != node + 1
            || next != self.pending_plan_nodes.len() - 1
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_bind next must be the immediately following freshly created node",
            ));
        }
        let next_state = &self.pending_plan_nodes[next];
        if next_state.source_match != match_expr_id || next_state.has_incoming_parent {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_bind next node match mismatch or already linked",
            ));
        }
        if !matches!(
            next_state.kind,
            PendingPlanNodeKind::Bind {
                finished: false,
                ..
            } | PendingPlanNodeKind::Select { .. }
        ) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_bind next node must be an unfinished Bind or Select node",
            ));
        }
        let node_state = &self.pending_plan_nodes[node];
        if node_state.source_match != match_expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_bind node belongs to a different Match expression",
            ));
        }
        if node > plan_block_start && !node_state.has_incoming_parent {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_bind node was not linked to its parent",
            ));
        }
        if self.plan_nodes[node].is_some() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_bind called on an already finished Bind node",
            ));
        }
        let (path, binder) = match node_state.kind {
            PendingPlanNodeKind::Bind {
                path,
                binder,
                finished: false,
            } => (path, binder),
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_bind node is not an unfinished Bind",
                ));
            }
        };
        fill_reserved_once(
            &mut self.plan_nodes,
            node,
            PlanNodeC::Bind {
                source_match: match_expr_id,
                path,
                binder,
                next,
            },
        )?;
        self.pending_plan_nodes[node].kind = PendingPlanNodeKind::Bind {
            path,
            binder,
            finished: true,
        };
        self.pending_plan_nodes[next].has_incoming_parent = true;
        Ok(())
    }

    pub(super) fn append_select(
        &mut self,
        source_match: usize,
        arm: usize,
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_complete()?;
        let (plan_block_start, arm_rows) = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_select called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source_match {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_select source_match does not match active planned Match",
                ));
            }
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_select called before G-4 arm binder checks completed",
                ));
            }
            (active.plan_block_start, active.arm_rows)
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in append_select",
            ));
        }
        if self.pending_plan_nodes.len() > plan_block_start + 1 {
            if !self.pending_plan_nodes.last().unwrap().has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "previous non-root plan node was not linked before append_select",
                ));
            }
        }
        let arm_end = arm_rows.start.checked_add(arm_rows.len).ok_or_else(|| {
            host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "append_select match arm_rows overflow",
            )
        })?;
        if arm < arm_rows.start || arm >= arm_end || arm >= self.arms.len() {
            return Err(host_expr_err(
                HostViolationKind::ReadOutOfBounds,
                "append_select arm id out of current match arm_rows bounds",
            ));
        }
        if self.arms[arm].is_none() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "append_select referenced C3 arm slot is unfilled",
            ));
        }
        let node_id = self.plan_nodes.len();
        if node_id >= self.bounds.match_plan_rows {
            return Err(p_ext_cap_err(
                ExtTypeCapKind::MatchPlanRows,
                node_id,
                self.bounds.match_plan_rows,
                None,
            ));
        }
        self.plan_nodes
            .push(Some(PlanNodeC::Select { source_match, arm }));
        self.pending_plan_nodes.push(PendingPlanNodeState {
            source_match,
            has_incoming_parent: false,
            kind: PendingPlanNodeKind::Select { arm },
        });
        Ok(node_id)
    }

    pub(super) fn append_no_match(
        &mut self,
        source_match: usize,
    ) -> Result<usize, ExtTypeBuildError> {
        self.check_c6_complete()?;
        let plan_block_start = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_no_match called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source_match {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_no_match source_match does not match active planned Match",
                ));
            }
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "append_no_match called before G-4 arm binder checks completed",
                ));
            }
            active.plan_block_start
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes and pending_plan_nodes length mismatch in append_no_match",
            ));
        }
        if self.pending_plan_nodes.len() > plan_block_start + 1 {
            if !self.pending_plan_nodes.last().unwrap().has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "previous non-root plan node was not linked before append_no_match",
                ));
            }
        }
        let node_id = self.plan_nodes.len();
        if node_id >= self.bounds.match_plan_rows {
            return Err(p_ext_cap_err(
                ExtTypeCapKind::MatchPlanRows,
                node_id,
                self.bounds.match_plan_rows,
                None,
            ));
        }
        self.plan_nodes
            .push(Some(PlanNodeC::NoMatch { source_match }));
        self.pending_plan_nodes.push(PendingPlanNodeState {
            source_match,
            has_incoming_parent: false,
            kind: PendingPlanNodeKind::NoMatch,
        });
        Ok(node_id)
    }

    pub(super) fn record_plan_work(
        &mut self,
        node: usize,
        source_match: usize,
        candidate_arms: &[usize],
        winner: Option<usize>,
        path: &[usize],
        kind: PlanVisitKindC,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;
        let (match_expr_id, plan_block_start, arm_rows) = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "record_plan_work called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source_match {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "record_plan_work source_match does not match active planned Match",
                ));
            }
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "record_plan_work called before G-4 arm binder checks completed",
                ));
            }
            (
                active.match_expr_id,
                active.plan_block_start,
                active.arm_rows,
            )
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len()
            || node < plan_block_start
            || node != self.work_map.len()
            || node + 1 != self.pending_plan_nodes.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "record_plan_work node index does not match newly created plan node",
            ));
        }
        if !matches!(self.occurrences.get(match_expr_id), Some(Some(_))) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "record_plan_work source_match has no C4 occurrence row",
            ));
        }
        for &ord in candidate_arms {
            if ord >= arm_rows.len {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "record_plan_work candidate_arms ordinal out of match arm bounds",
                ));
            }
        }
        for pair in candidate_arms.windows(2) {
            if pair[0] >= pair[1] {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "record_plan_work candidate_arms not in strictly ascending source order",
                ));
            }
        }
        for &step in path {
            if step >= self.bounds.max_arity {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "record_plan_work path step exceeds max_arity",
                ));
            }
        }
        let pstate = &self.pending_plan_nodes[node];
        if pstate.source_match != match_expr_id {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "record_plan_work pending node source_match mismatch",
            ));
        }
        let (live_arms, final_event) = match (kind, pstate.kind) {
            (
                PlanVisitKindC::Switch,
                PendingPlanNodeKind::Switch {
                    path: node_path,
                    finished: false,
                    ..
                },
            ) => {
                if winner.is_some() || candidate_arms.is_empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "record_plan_work Switch requires winner == None and non-empty candidate_arms",
                    ));
                }
                let end = node_path.start.checked_add(node_path.len).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "record_plan_work Switch path slice overflow",
                    )
                })?;
                if end > self.field_paths.len() || &self.field_paths[node_path.start..end] != path {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "record_plan_work Switch path does not match owned field_paths slice",
                    ));
                }
                let arms = candidate_arms
                    .iter()
                    .map(|&ord| arm_rows.start + ord)
                    .collect::<Vec<_>>();
                (arms, RuleIdC::XAdtHelperPlanSwitch)
            }
            (
                PlanVisitKindC::Bind,
                PendingPlanNodeKind::Bind {
                    path: node_path,
                    finished: false,
                    ..
                },
            ) => {
                let w = match (winner, candidate_arms.first()) {
                    (Some(w), Some(&first)) if w == first => w,
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "record_plan_work Bind requires winner == Some(candidate_arms[0])",
                        ));
                    }
                };
                let end = node_path.start.checked_add(node_path.len).ok_or_else(|| {
                    host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "record_plan_work Bind path slice overflow",
                    )
                })?;
                if end > self.field_paths.len() || &self.field_paths[node_path.start..end] != path {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "record_plan_work Bind path does not match owned field_paths slice",
                    ));
                }
                let arms = candidate_arms
                    .iter()
                    .map(|&ord| arm_rows.start + ord)
                    .collect::<Vec<_>>();
                let _ = w;
                (arms, RuleIdC::XAdtHelperPlanBind)
            }
            (PlanVisitKindC::Select, PendingPlanNodeKind::Select { arm }) => {
                let w = match (winner, candidate_arms.first()) {
                    (Some(w), Some(&first)) if w == first => w,
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "record_plan_work Select requires winner == Some(candidate_arms[0])",
                        ));
                    }
                };
                if arm != arm_rows.start + w || !path.is_empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "record_plan_work Select arm or non-empty path mismatch",
                    ));
                }
                let arms = candidate_arms
                    .iter()
                    .map(|&ord| arm_rows.start + ord)
                    .collect::<Vec<_>>();
                (arms, RuleIdC::XAdtHelperPlanSelect)
            }
            (PlanVisitKindC::NoMatch, PendingPlanNodeKind::NoMatch) => {
                if winner.is_some() || !candidate_arms.is_empty() || !path.is_empty() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "record_plan_work NoMatch requires empty candidate_arms, winner == None, and empty path",
                    ));
                }
                (Vec::new(), RuleIdC::XAdtHelperPlanNoMatch)
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "record_plan_work kind does not match pending plan node variant",
                ));
            }
        };
        let mut events_per_visit = Vec::with_capacity(path.len() + 1);
        for _ in 0..path.len() {
            events_per_visit.push(RuleIdC::XAdtHelperPlanWalk);
        }
        events_per_visit.push(final_event);
        self.work_map.push(WorkMapEntryC {
            source_match: match_expr_id,
            live_arms,
            field_path: path.to_vec(),
            events_per_visit,
        });
        Ok(())
    }

    pub(super) fn finish_match(
        &mut self,
        source: &MatchOccurrence<'_>,
        root: usize,
    ) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;
        let (plan_block_start, arm_rows) = {
            let active = self.active_match_plan.as_ref().ok_or_else(|| {
                host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match called without an active planned Match",
                )
            })?;
            if active.match_expr_id != source.expr_id
                || active.owner != source.owner
                || active.local_expr != source.local_expr
                || active.arm_rows != source.arm_rows
                || active.source_arms.len() != source.arms.len()
                || !std::ptr::eq(active.source_arms.as_ptr(), source.arms.as_ptr())
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match source MatchOccurrence does not match active planned Match",
                ));
            }
            if active.arms_checked != active.source_arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match called before G-4 arm binder checks completed",
                ));
            }
            (active.plan_block_start, active.arm_rows)
        };
        if self.plan_nodes.len() != self.pending_plan_nodes.len()
            || self.work_map.len() != self.plan_nodes.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "plan_nodes, pending_plan_nodes, or work_map length mismatch in finish_match",
            ));
        }
        if root != plan_block_start || root >= self.plan_nodes.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "finish_match root does not equal first node of current match plan block",
            ));
        }
        for idx in plan_block_start..self.plan_nodes.len() {
            if self.plan_nodes[idx].is_none() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match called with unfilled reserved plan node in match block",
                ));
            }
            let pstate = &self.pending_plan_nodes[idx];
            if pstate.source_match != source.expr_id {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match plan block node belongs to a different Match expression",
                ));
            }
            if idx == root {
                if pstate.has_incoming_parent {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "finish_match root plan node unexpectedly has an incoming parent",
                    ));
                }
            } else if !pstate.has_incoming_parent {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match non-root plan node is unlinked from parent",
                ));
            }
            match pstate.kind {
                PendingPlanNodeKind::Switch { finished: true, .. }
                | PendingPlanNodeKind::Bind { finished: true, .. }
                | PendingPlanNodeKind::Select { .. }
                | PendingPlanNodeKind::NoMatch => {}
                _ => {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "finish_match plan node state still marked unfinished",
                    ));
                }
            }
        }
        match self.expressions.get_mut(source.expr_id) {
            Some(Some(ExpressionRowC::Match {
                arms, plan: slot, ..
            })) if *arms == arm_rows && slot.is_none() => {
                *slot = Some(root);
            }
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "finish_match target Match row missing, mismatched, or already has a plan",
                ));
            }
        }
        self.active_match_plan = None;
        Ok(())
    }

    pub(super) fn plan_traversal_invariant(&self) -> ExtTypeBuildError {
        host_expr_err(
            HostViolationKind::WrongVariant,
            "match plan compilation terminated without producing a root plan node",
        )
    }
}

impl<'s> PendingExtCExpressionStage<'s> {
    pub(super) fn check_p_complete(&self) -> Result<(), ExtTypeBuildError> {
        self.check_c6_complete()?;

        if let Some(active) = &self.active_match_plan {
            let _ = active.match_expr_id;
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "pass P finished with an active match plan still open",
            ));
        }

        if self.plan_nodes.len() != self.pending_plan_nodes.len()
            || self.work_map.len() != self.plan_nodes.len()
        {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "pass P finished with inconsistent plan_nodes, pending_plan_nodes, or work_map length",
            ));
        }

        for (node_idx, (slot, pending)) in self
            .plan_nodes
            .iter()
            .zip(self.pending_plan_nodes.iter())
            .enumerate()
        {
            let Some(plan_row) = slot else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P left an unpopulated plan_nodes slot",
                ));
            };
            match &pending.kind {
                PendingPlanNodeKind::Switch {
                    path,
                    type_id,
                    cases: expected_cases,
                    finished,
                    ..
                } => {
                    if !*finished {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "pass P left an unfinished Switch plan node",
                        ));
                    }
                    match plan_row {
                        PlanNodeC::Switch {
                            source_match,
                            path: row_path,
                            type_id: row_type_id,
                            cases,
                            default,
                        } => {
                            if *source_match != pending.source_match
                                || *row_path != *path
                                || *row_type_id != *type_id
                                || *cases != *expected_cases
                                || *default >= self.plan_nodes.len()
                                || *default <= node_idx
                            {
                                return Err(host_expr_err(
                                    HostViolationKind::WrongVariant,
                                    "pass P Switch plan node row does not match pending state or forward DAG ordering",
                                ));
                            }
                        }
                        _ => {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "pass P Switch pending plan node does not match PlanNodeC::Switch",
                            ));
                        }
                    }
                }
                PendingPlanNodeKind::Bind {
                    path,
                    binder,
                    finished,
                } => {
                    if !*finished {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "pass P left an unfinished Bind plan node",
                        ));
                    }
                    match plan_row {
                        PlanNodeC::Bind {
                            source_match,
                            path: row_path,
                            binder: row_binder,
                            next,
                        } => {
                            if *source_match != pending.source_match
                                || *row_path != *path
                                || *row_binder != *binder
                                || *next >= self.plan_nodes.len()
                                || *next <= node_idx
                            {
                                return Err(host_expr_err(
                                    HostViolationKind::WrongVariant,
                                    "pass P Bind plan node row does not match pending state or forward DAG ordering",
                                ));
                            }
                        }
                        _ => {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "pass P Bind pending plan node does not match PlanNodeC::Bind",
                            ));
                        }
                    }
                }
                PendingPlanNodeKind::Select { arm } => match plan_row {
                    PlanNodeC::Select {
                        source_match,
                        arm: row_arm,
                    } => {
                        if *source_match != pending.source_match || *row_arm != *arm {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "pass P Select plan node row does not match pending state",
                            ));
                        }
                    }
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "pass P Select pending plan node does not match PlanNodeC::Select",
                        ));
                    }
                },
                PendingPlanNodeKind::NoMatch => match plan_row {
                    PlanNodeC::NoMatch { source_match } => {
                        if *source_match != pending.source_match {
                            return Err(host_expr_err(
                                HostViolationKind::WrongVariant,
                                "pass P NoMatch plan node row does not match pending state",
                            ));
                        }
                    }
                    _ => {
                        return Err(host_expr_err(
                            HostViolationKind::WrongVariant,
                            "pass P NoMatch pending plan node does not match PlanNodeC::NoMatch",
                        ));
                    }
                },
            }

            let Some(work_entry) = self.work_map.get(node_idx) else {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pass P missing work_map entry for plan node",
                ));
            };
            if work_entry.source_match != pending.source_match
                || work_entry.events_per_visit.is_empty()
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P work_map entry does not match plan node source_match or events_per_visit",
                ));
            }
        }

        let mut expected_match_visit_idx = 0usize;
        for (expr_id, binding) in self.expr_bindings.iter().enumerate() {
            if !matches!(binding.source, ExtExpr::Match { .. }) {
                continue;
            }
            if self
                .p_visited_matches
                .get(expected_match_visit_idx)
                .copied()
                != Some(expr_id)
            {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P did not visit Match expression in canonical ascending ExprId order",
                ));
            }
            expected_match_visit_idx += 1;

            let Some(Some(ExpressionRowC::Match { arms, plan, .. })) =
                self.expressions.get(expr_id)
            else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P missing finalized ExpressionRowC::Match slot",
                ));
            };

            let Some(arms_end) = arms.start.checked_add(arms.len) else {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pass P Match arms slice overflow",
                ));
            };
            if arms_end > self.arms.len() {
                return Err(host_expr_err(
                    HostViolationKind::ReadOutOfBounds,
                    "pass P Match arms slice out of bounds",
                ));
            }

            let mut is_planned = false;
            for arm_idx in arms.start..arms_end {
                let Some(Some(arm_row)) = self.arms.get(arm_idx) else {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "pass P Match references unpopulated ArmRowC slot",
                    ));
                };
                let Some(pat_row) = self.patterns.get(arm_row.pattern) else {
                    return Err(host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "pass P ArmRowC pattern out of bounds",
                    ));
                };
                if matches!(
                    pat_row,
                    PatternRowC::Wildcard | PatternRowC::Constructor { .. }
                ) {
                    is_planned = true;
                }
            }

            if !is_planned {
                if plan.is_some() {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "pass P attached decision-DAG plan root to dedicated-only Match",
                    ));
                }
            } else {
                let Some(root) = *plan else {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "pass P left planned Match without decision-DAG plan root",
                    ));
                };
                let Some(root_pending) = self.pending_plan_nodes.get(root) else {
                    return Err(host_expr_err(
                        HostViolationKind::ReadOutOfBounds,
                        "pass P Match plan root out of plan_nodes range",
                    ));
                };
                if root_pending.source_match != expr_id || root_pending.has_incoming_parent {
                    return Err(host_expr_err(
                        HostViolationKind::WrongVariant,
                        "pass P Match plan root does not belong to source_match or has incoming parent",
                    ));
                }
            }
        }

        if expected_match_visit_idx != self.p_visited_matches.len() {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "pass P recorded visited Match count mismatching expression table Match count",
            ));
        }

        if !self.expressions.is_empty() && self.entry_root_id != Some(0) {
            return Err(host_expr_err(
                HostViolationKind::WrongVariant,
                "pass P completed without canonical entry_root_id == Some(0)",
            ));
        }

        for (fn_idx, fn_root_opt) in self.function_root_ids.iter().enumerate() {
            let Some(fn_root) = fn_root_opt else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P completed with unrecorded function_root_id",
                ));
            };
            let Some(Some(fn_row)) = self.functions.get(fn_idx) else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P completed with unpopulated function row",
                ));
            };
            if fn_row.body != *fn_root || *fn_root >= self.expressions.len() {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "pass P completed with function_root_id mismatching FunctionRowC.body",
                ));
            }
        }

        Ok(())
    }

    pub(super) fn into_program_view_and_report(
        self,
    ) -> Result<(ProgramView, PreparationReportC), ExtTypeBuildError> {
        self.check_p_complete()?;

        let entry = match self.entry_root_id {
            Some(root_id) if root_id < self.expressions.len() => root_id,
            _ => {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "cannot materialize ProgramView without valid entry_root_id",
                ));
            }
        };
        let prepared = self.prepared;

        let mut expressions = Vec::with_capacity(self.expressions.len());
        for slot in self.expressions {
            let Some(row) = slot else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "cannot materialize ProgramView with unpopulated expression row",
                ));
            };
            expressions.push(row);
        }

        let mut occurrences = Vec::with_capacity(self.occurrences.len());
        for slot in self.occurrences {
            let Some(row) = slot else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "cannot materialize ProgramView with unpopulated occurrence row",
                ));
            };
            occurrences.push(row);
        }

        let mut functions = Vec::with_capacity(self.functions.len());
        for slot in self.functions {
            let Some(row) = slot else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "cannot materialize ProgramView with unpopulated function row",
                ));
            };
            functions.push(row);
        }

        let mut arms = Vec::with_capacity(self.arms.len());
        for slot in self.arms {
            let Some(row) = slot else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "cannot materialize ProgramView with unpopulated arm row",
                ));
            };
            arms.push(row);
        }

        let mut plan_nodes = Vec::with_capacity(self.plan_nodes.len());
        for slot in self.plan_nodes {
            let Some(row) = slot else {
                return Err(host_expr_err(
                    HostViolationKind::WrongVariant,
                    "cannot materialize ProgramView with unpopulated plan_nodes row",
                ));
            };
            plan_nodes.push(row);
        }

        let view = ProgramView {
            entry,
            expressions,
            occurrences,
            occurrence_paths: self.occurrence_paths,
            children: self.children,
            functions,
            parameters: self.parameters,
            arms,
            patterns: self.patterns,
            pattern_children: self.pattern_children,
            types: prepared.tables.types.clone(),
            type_args: prepared.tables.type_args.clone(),
            constructors: prepared.tables.constructors.clone(),
            ctor_fields: prepared.tables.ctor_fields.clone(),
            decl_param_counts: prepared.tables.decl_param_counts.clone(),
            decl_ctor_counts: prepared.tables.decl_ctor_counts.clone(),
            decl_templates: clone_type_decls_iterative(&prepared.program.types),
            plan_nodes,
            plan_cases: self.plan_cases,
            field_paths: self.field_paths,
            constants: self.constants,
            constant_fields: self.constant_fields,
            names: prepared.interned_names.clone(),
        };
        let report = PreparationReportC {
            work_map: self.work_map,
        };
        Ok((view, report))
    }
}

impl<'s> ExtPProgramWriter for PendingExtCExpressionStage<'s> {
    fn begin_match(
        &mut self,
        source: &MatchOccurrence<'_>,
    ) -> Result<Option<Vec<PlanArmSourceC>>, ExtTypeBuildError> {
        PendingExtCExpressionStage::begin_match(self, source)
    }

    fn pattern(&self, id: usize) -> Result<PatternRowC, ExtTypeBuildError> {
        PendingExtCExpressionStage::pattern(self, id)
    }

    fn pattern_children(&self, slice: SliceC) -> Result<Vec<usize>, ExtTypeBuildError> {
        PendingExtCExpressionStage::pattern_children(self, slice)
    }

    fn check_arm_binders(
        &mut self,
        source: &MatchOccurrence<'_>,
        arm: usize,
        binders: &[PatternBindingC],
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::check_arm_binders(self, source, arm, binders)
    }

    fn constructor_arity(&self, ctor: CtorIdC) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::constructor_arity(self, ctor)
    }

    fn reserve_switch(
        &mut self,
        source_match: usize,
        path: &[usize],
        type_id: TypeIdC,
    ) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::reserve_switch(self, source_match, path, type_id)
    }

    fn set_switch_case(
        &mut self,
        node: usize,
        ctor: CtorIdC,
        target: usize,
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::set_switch_case(self, node, ctor, target)
    }

    fn finish_switch(&mut self, node: usize, default: usize) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::finish_switch(self, node, default)
    }

    fn reserve_bind(
        &mut self,
        source_match: usize,
        path: &[usize],
        binder: usize,
    ) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::reserve_bind(self, source_match, path, binder)
    }

    fn finish_bind(&mut self, node: usize, next: usize) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::finish_bind(self, node, next)
    }

    fn append_select(
        &mut self,
        source_match: usize,
        arm: usize,
    ) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::append_select(self, source_match, arm)
    }

    fn append_no_match(&mut self, source_match: usize) -> Result<usize, ExtTypeBuildError> {
        PendingExtCExpressionStage::append_no_match(self, source_match)
    }

    fn record_plan_work(
        &mut self,
        node: usize,
        source_match: usize,
        candidate_arms: &[usize],
        winner: Option<usize>,
        path: &[usize],
        kind: PlanVisitKindC,
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::record_plan_work(
            self,
            node,
            source_match,
            candidate_arms,
            winner,
            path,
            kind,
        )
    }

    fn finish_match(
        &mut self,
        source: &MatchOccurrence<'_>,
        root: usize,
    ) -> Result<(), ExtTypeBuildError> {
        PendingExtCExpressionStage::finish_match(self, source, root)
    }

    fn plan_traversal_invariant(&self) -> ExtTypeBuildError {
        PendingExtCExpressionStage::plan_traversal_invariant(self)
    }
}

pub(super) fn prepare_ext_c1_to_c5<'s>(
    prepared: &'s PreparedExtProgramSource,
    layout: &'s LayoutSchemaC,
    pattern_child_slots: usize,
) -> Result<PendingExtC<'s>, ExtTypeBuildError> {
    let mut stage = PendingExtCExpressionStage::new_checked(prepared, layout, pattern_child_slots)?;
    let continuation = lower_ext_c1_to_c5(prepared, &mut stage)?;
    stage.check_c1_to_c5_completed(&continuation)?;
    Ok(PendingExtC {
        stage,
        continuation,
    })
}

pub(super) fn prepare_ext_c1_to_p<'s>(
    prepared: &'s PreparedExtProgramSource,
    bounds: &BoundsC,
    layout: &'s LayoutSchemaC,
    ext_capacities: ExtCapacitiesC,
) -> Result<(ProgramView, PreparationReportC), ExtTypeBuildError> {
    let mut stage = PendingExtCExpressionStage::new_checked_with_capacities(
        prepared,
        bounds,
        layout,
        ext_capacities,
    )?;
    lower_ext_c1_to_p(prepared, &mut stage)?;
    stage.into_program_view_and_report()
}
