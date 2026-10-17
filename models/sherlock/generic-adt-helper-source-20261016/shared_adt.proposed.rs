//! helper_machine::shared::adt -- PROPOSED, uncompiled, not installed, not adopted.
//! GenericNominal carriers for exactly the 12 x.adt.helper.{Build,Plan}.* rows.
//! Source-only host carriers: no bounded width, field, limb, encoding, profile or ABI.
//! Under (DedicatedSorts, Absent) no value of any type in this module may exist in an
//! image, view or plan (SI-X01); the owner never invokes an ADT family there.

use super::{ActiveLengths, ControlC, EnvCellC, EnvRootC, EventKind, FieldSliceC,
    HostViolation, PlanDelta, RefC, SliceC, TriStatus, ValSliceC, Write};

// ---------- closed index carriers (host usize; widths stay symbolic) ----------
// TypeId: id(typeRows); 0..3 = UInt, Bool, List, Tree; >= 4 user type or instance.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TypeIdC(pub usize);
// CtorId: id(constructorRows); 0..3 = Nil, Cons, Leaf, Branch.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CtorIdC(pub usize);
// PlanNodeId: id(matchPlanRows).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PlanNodeIdC(pub usize);
// FieldPathSlice: {start: FieldPathPos, len}; (0, 0) is the root occurrence.
pub type FieldPathSliceC = SliceC;

// ---------- extension enums (schema new_enums / enum_appends) ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanNodeTag { Switch, Bind, Select, NoMatch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanPhase { Walk, Test, Bind, Ready, NoMatchReady, Done }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanAfterWalk { Test, Bind }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildPhase { Copy, Done }
// HelperKind appended ordinals: Plan 9, Build 10.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtHelperKind { Plan, Build }
// SiteTag: Cases = core ordinal 21; Materialize = appended ordinal 26.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtHelperSite { Cases, Materialize }
// Selected-leaf identifier. Extended RuleId ordinals 121..132 in this order
// (112 core IDs, then the 59 x.* IDs in extension-rows.json file order).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtHelperRuleC { BuildEntry, BuildCell, BuildField, BuildExit,
    PlanEntry, PlanWalk, PlanSwitch, PlanBind, PlanSelect, PlanNoMatch,
    PlanExit, PlanFailure }
// extension_guards: G-BUILD-LAST is the only guard any of the 12 rows names.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtGuardId { BuildLast }
// kind(ref) under GenericNominal; schema LogicalKind ordinals 0..7 (Constructor 7).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogicalKindExt { UInt, Bool, Nil, Cons, Leaf, Branch, Marker,
    Constructor { type_id: TypeIdC, ctor_id: CtorIdC } }

// ---------- read-only program rows (code fetch) ----------
// prog.types row (adt.program_tables.prog.types).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypeRowC { pub name_id: usize, pub decl: Option<usize>, pub args: SliceC,
    pub ctor_start: CtorIdC, pub ctor_len: usize }
// prog.planNodes row by tag; columns zero for other tags are not carried.
// match_expr = column `match`; default_node = column `default`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanNodeRowC {
    Switch { match_expr: usize, path: FieldPathSliceC, type_id: TypeIdC,
        case_start: usize, case_len: usize, default_node: PlanNodeIdC },
    Bind { match_expr: usize, path: FieldPathSliceC, binder: usize, next: PlanNodeIdC },
    Select { match_expr: usize, arm: usize },
    NoMatch { match_expr: usize },
}

// ---------- control register (core control or the Materialize extension) ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlExtC { Core(ControlC),
    Materialize { type_id: TypeIdC, ctor_id: CtorIdC, fields: ValSliceC } }

// ---------- helper register payloads (records.HelperControl.Plan / .Build) ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AdtHelperCommon { pub site: AdtHelperSite,
    pub origin_reference_cursor: u128,
    pub parent_resume: Option<usize> } // always None while X-05 is dormant (SI-08)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlanHelperC { pub common: AdtHelperCommon, pub match_expr: usize,
    pub scrutinee: RefC, pub saved_env: EnvRootC, pub partial: EnvRootC,
    pub node: PlanNodeIdC, pub phase: PlanPhase, pub after_walk: PlanAfterWalk,
    pub cur: RefC, pub path: FieldPathSliceC, pub path_cursor: usize,
    pub arm: Option<usize>, pub status: TriStatus }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BuildHelperC { pub common: AdtHelperCommon, pub type_id: TypeIdC,
    pub ctor_id: CtorIdC, pub src: ValSliceC, pub src_cursor: usize,
    pub field_start: usize, pub staged: Option<RefC>, pub node: Option<RefC>,
    pub phase: BuildPhase }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtHelperC { Plan(PlanHelperC), Build(BuildHelperC) }

// ---------- result register payloads (records.HelperResult_appended) ----------
// HelperResultTag appended ordinals: PlanMatched 16, PlanNoMatch 17, BuildDone 18.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtHelperResultC { PlanMatched { arm: usize, env: EnvRootC }, PlanNoMatch,
    BuildDone { value: RefC } }

// ---------- authoritative selection fact (owner-produced; families never recompute) ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SelectedAdt { pub rule: AdtHelperRuleC, pub event_kind: EventKind,
    pub site: AdtHelperSite, pub helper_kind: AdtHelperKind,
    pub helper_eligible: bool, pub helper_visits: u8 }

// ---------- planned effects (family proposal; owner validates) ----------
// DataRow.Constructor node; fields = slice(h.fieldStart, h.src.len) (CPA-04 block).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ConstructorNodeWrite { pub type_id: TypeIdC, pub ctor_id: CtorIdC,
    pub fields: FieldSliceC }
// helper: Set(Some(h')) = install or full post-helper; Set(None) = clearHelper.
// result: Some(r) = set reg.result := r; None = unchanged (no ADT helper row clears it).
// alloc_field: allocField at pre.active.dataFields, applied before alloc_constructor.
// alloc_constructor: allocData at pre.active.data. alloc_env: allocEnv at pre.active.env.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AdtPlannedWrites {
    pub helper: Write<Option<AdtHelperC>>,
    pub result: Option<AdtHelperResultC>,
    pub alloc_field: Option<RefC>,
    pub alloc_constructor: Option<ConstructorNodeWrite>,
    pub alloc_env: Option<EnvCellC>,
}
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct AdtEffectPlan { pub delta_assert: PlanDelta, pub writes: AdtPlannedWrites }

// ---------- family-side faults (owner maps each to HostViolation; never an Outcome) ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdtRowFaultKind { UndefinedRoot, InactiveVariant, UnwrapAbsent,
    UndefinedFunction, ArithmeticBelowZero, HostIndexOverflow }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AdtRowFault { pub kind: AdtRowFaultKind, pub at: &'static str }
pub enum AdtFamilyError { Host(HostViolation), Fault(AdtRowFault) }
impl From<HostViolation> for AdtFamilyError {
    fn from(v: HostViolation) -> Self { AdtFamilyError::Host(v) } }
impl From<AdtRowFault> for AdtFamilyError {
    fn from(f: AdtRowFault) -> Self { AdtFamilyError::Fault(f) } }

// ---------- owner-implemented checked pre-image view (abstract dependency) ----------
// Every method reads the PRE-image only, never mutates, and returns Err (never panics)
// on any bound, variant or domain violation. Register methods are never counted reads.
pub trait AdtPreImageView {
    fn control_ext(&self) -> &ControlExtC;          // c
    fn control_origin(&self) -> usize;              // pre.controlOrigin
    fn reference_cursor(&self) -> u128;             // pre.referenceCursor
    fn active(&self) -> ActiveLengths;              // pre.active.*
    fn adt_helper(&self) -> Option<&AdtHelperC>;    // h (Plan or Build only)
    // prog.expressions[e].plan; Err if e >= active(prog.expressions) or expr(e) not Match.
    fn match_plan_root(&self, e: &usize) -> Result<Option<PlanNodeIdC>, HostViolation>;
    // planNode(n); bound active(prog.planNodes).
    fn plan_node(&self, n: &PlanNodeIdC) -> Result<&PlanNodeRowC, HostViolation>;
    // prog.planCases[pos]; bound active(prog.planCases).
    fn plan_case(&self, pos: &usize) -> Result<Option<PlanNodeIdC>, HostViolation>;
    // prog.fieldPaths[pos]; bound active(prog.fieldPaths).
    fn field_path_cell(&self, pos: &usize) -> Result<usize, HostViolation>;
    // prog.types[t]; bound active(prog.types).
    fn type_row(&self, t: &TypeIdC) -> Result<&TypeRowC, HostViolation>;
    // vslot(s, i): requires i < s.len and s.start + s.len <= pre.active.valueSlots.
    fn vslot(&self, s: &ValSliceC, i: &usize) -> Result<RefC, HostViolation>;
    // kind(ref) over Const and Data spaces including Constructor rows.
    fn kind_ext(&self, r: &RefC) -> Result<LogicalKindExt, HostViolation>;
    // field(ref, i) on the PRE-image (x.adt.helper.Plan.walk reads[1], bound 'its space'):
    // header Data(d): d < pre.active.data; Const(c): c < active(prog.constants); then
    // i < arity(kind(ref)) (Constructor{t, c}: prog.constructors[c].fieldLen; Marker: Err) and
    // i < row.fields.len; slot row.fields.start + i < pre.active.dataFields (Data) or
    // active(prog.constantFields) (Const). Header + this one slot = one record, one ledger entry.
    fn field_ext(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation>;
}

// ---------- owner-implemented guard evaluator (abstract dependency) ----------
pub trait AdtGuardEvaluator {
    fn eval_adt_guard(&self, g: &AdtGuardId, pre: &dyn AdtPreImageView)
        -> Result<bool, HostViolation>;
}

// ---------- Sol family contract (pure row effects only) ----------
// A method presented a rule of the other family returns exactly the core literal:
// HostViolation { kind: WrongVariant, clause: None, rule: None,
//   locator: Some("foreign rule for family".to_string()) } wrapped in AdtFamilyError::Host.
pub trait AdtHelperFamilyPlans {
    fn plan_adt_build(&self, sel: &SelectedAdt, pre: &dyn AdtPreImageView,
        g: &dyn AdtGuardEvaluator) -> Result<AdtEffectPlan, AdtFamilyError>;
    fn plan_adt_plan(&self, sel: &SelectedAdt, pre: &dyn AdtPreImageView)
        -> Result<AdtEffectPlan, AdtFamilyError>;
}

// ---------- pure leaves (row_language / extension_functions definitions) ----------
pub mod adt_pure {
    use super::{AdtRowFault, AdtRowFaultKind, CtorIdC, FieldPathSliceC, LogicalKindExt,
        PlanAfterWalk, PlanNodeRowC, PlanNodeTag, PlanPhase, SliceC, TypeIdC, TypeRowC};
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct CtorOfKind { pub type_id: TypeIdC, pub ctor_id: CtorIdC }
    // notation.added_functions typeOfKind(k); None = undefined (Marker).
    pub fn type_of_kind(k: &LogicalKindExt) -> Option<TypeIdC> {
        match k {
            LogicalKindExt::UInt => Some(TypeIdC(0)),
            LogicalKindExt::Bool => Some(TypeIdC(1)),
            LogicalKindExt::Nil | LogicalKindExt::Cons => Some(TypeIdC(2)),
            LogicalKindExt::Leaf | LogicalKindExt::Branch => Some(TypeIdC(3)),
            LogicalKindExt::Constructor { type_id, .. } => Some(*type_id),
            LogicalKindExt::Marker => None,
        }
    }
    // notation.added_functions ctorOfKind(k); None = undefined.
    pub fn ctor_of_kind(k: &LogicalKindExt) -> Option<CtorOfKind> {
        match k {
            LogicalKindExt::Nil => Some(CtorOfKind { type_id: TypeIdC(2), ctor_id: CtorIdC(0) }),
            LogicalKindExt::Cons => Some(CtorOfKind { type_id: TypeIdC(2), ctor_id: CtorIdC(1) }),
            LogicalKindExt::Leaf => Some(CtorOfKind { type_id: TypeIdC(3), ctor_id: CtorIdC(2) }),
            LogicalKindExt::Branch => Some(CtorOfKind { type_id: TypeIdC(3), ctor_id: CtorIdC(3) }),
            LogicalKindExt::Constructor { type_id, ctor_id } =>
                Some(CtorOfKind { type_id: *type_id, ctor_id: *ctor_id }),
            LogicalKindExt::UInt | LogicalKindExt::Bool | LogicalKindExt::Marker => None,
        }
    }
    // extension_functions ctorClass(k).
    pub fn ctor_class(k: &LogicalKindExt) -> bool {
        matches!(k, LogicalKindExt::Nil | LogicalKindExt::Cons | LogicalKindExt::Leaf
            | LogicalKindExt::Branch | LogicalKindExt::Constructor { .. })
    }
    pub fn plan_node_tag(nd: &PlanNodeRowC) -> PlanNodeTag {
        match nd {
            PlanNodeRowC::Switch { .. } => PlanNodeTag::Switch,
            PlanNodeRowC::Bind { .. } => PlanNodeTag::Bind,
            PlanNodeRowC::Select { .. } => PlanNodeTag::Select,
            PlanNodeRowC::NoMatch { .. } => PlanNodeTag::NoMatch,
        }
    }
    // extension_functions loadPhase(n), applied to nd = planNode(n).
    pub fn load_phase(nd: &PlanNodeRowC) -> PlanPhase {
        match nd {
            PlanNodeRowC::Switch { path, .. } =>
                if path.len > 0 { PlanPhase::Walk } else { PlanPhase::Test },
            PlanNodeRowC::Bind { path, .. } =>
                if path.len > 0 { PlanPhase::Walk } else { PlanPhase::Bind },
            PlanNodeRowC::Select { .. } => PlanPhase::Ready,
            PlanNodeRowC::NoMatch { .. } => PlanPhase::NoMatchReady,
        }
    }
    // extension_functions loadAfter(n).
    pub fn load_after(nd: &PlanNodeRowC) -> PlanAfterWalk {
        match nd {
            PlanNodeRowC::Bind { .. } => PlanAfterWalk::Bind,
            PlanNodeRowC::Switch { .. } | PlanNodeRowC::Select { .. }
            | PlanNodeRowC::NoMatch { .. } => PlanAfterWalk::Test,
        }
    }
    // extension_functions loadPath(n).
    pub fn load_path(nd: &PlanNodeRowC) -> FieldPathSliceC {
        match nd {
            PlanNodeRowC::Switch { path, .. } | PlanNodeRowC::Bind { path, .. } => *path,
            PlanNodeRowC::Select { .. } | PlanNodeRowC::NoMatch { .. } => SliceC::empty(),
        }
    }
    // extension_functions loadArm(n).
    pub fn load_arm(nd: &PlanNodeRowC) -> Option<usize> {
        match nd {
            PlanNodeRowC::Select { arm, .. } => Some(*arm),
            PlanNodeRowC::Switch { .. } | PlanNodeRowC::Bind { .. }
            | PlanNodeRowC::NoMatch { .. } => None,
        }
    }
    // extension_record_aliases PlanAfterWalk_to_PlanPhase (by name).
    pub fn after_walk_phase(a: PlanAfterWalk) -> PlanPhase {
        match a { PlanAfterWalk::Test => PlanPhase::Test, PlanAfterWalk::Bind => PlanPhase::Bind }
    }
    // extension_functions caseSlot(nd, k). Caller passes ty = prog.types[nd.typeId]
    // obtained through AdtPreImageView::type_row; evaluated only when ctorClass(k) and
    // typeOfKind(k) = nd.typeId (short-circuit, CPA-11).
    pub fn case_slot(nd: &PlanNodeRowC, k: &LogicalKindExt, ty: &TypeRowC)
        -> Result<usize, AdtRowFault> {
        let case_start = match nd {
            PlanNodeRowC::Switch { case_start, .. } => *case_start,
            _ => return Err(AdtRowFault { kind: AdtRowFaultKind::InactiveVariant, at: "nd.caseStart" }),
        };
        let ctor = match ctor_of_kind(k) {
            Some(c) => c.ctor_id.0,
            None => return Err(AdtRowFault { kind: AdtRowFaultKind::UndefinedFunction, at: "ctorOfKind(k)" }),
        };
        let sum = match case_start.checked_add(ctor) {
            Some(s) => s,
            None => return Err(AdtRowFault { kind: AdtRowFaultKind::HostIndexOverflow,
                at: "nd.caseStart + ctorOfKind(k).ctor" }),
        };
        match sum.checked_sub(ty.ctor_start.0) {
            Some(slot) => Ok(slot),
            None => Err(AdtRowFault { kind: AdtRowFaultKind::ArithmeticBelowZero, at: "caseSlot(nd, k)" }),
        }
    }
}
