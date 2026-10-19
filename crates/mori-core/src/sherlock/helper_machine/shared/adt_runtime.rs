//! Checked readonly ADT owner interface and ordinary canonical payload loading.
//! Projection structs are ephemeral reads of existing machine rows, never a
//! second stored runtime, program, guard, or interpreter. No owner bodies here.
use super::*;
use super::adt::{CtorIdC, TypeIdC};

#[derive(Clone, Copy)]
pub struct ConstructorFieldsReadC {
    pub type_id: TypeIdC, pub ctor_id: CtorIdC, pub index: usize,
    pub field_type: TypeIdC, pub checked: bool, pub completed: ValSliceC,
    pub rest: SliceC, pub saved_env: EnvRootC, pub origin: usize,
}
impl ConstructorFieldsReadC {
    pub fn frame(self) -> FrameRowC {
        FrameRowC::ConstructorFieldsF { type_id: self.type_id, ctor_id: self.ctor_id,
            index: self.index, field_type: self.field_type, checked: self.checked,
            completed: self.completed, rest: self.rest, saved_env: self.saved_env, origin: self.origin }
    }
}
pub struct ConstructFieldsReadC {
    pub type_id: TypeIdC, pub ctor_id: CtorIdC, pub field_type: TypeIdC,
    pub first: usize, pub rest: SliceC, pub saved_env: EnvRootC, pub origin: usize,
}
pub struct ConstructorNextReadC {
    pub frame: ConstructorFieldsReadC, pub completed: ValSliceC,
    pub next_index: usize, pub field_type: TypeIdC, pub next: usize, pub rest: SliceC,
}
pub struct ConstructorFinalReadC {
    pub frame: ConstructorFieldsReadC, pub completed: ValSliceC,
}
pub struct ConstructorScalarReadC {
    pub common: HelperCommon, pub value: RefC, pub expected: TypeIdC,
}
pub struct ConstructorAppendReadC {
    pub common: HelperCommon, pub value: RefC, pub completed: ValSliceC, pub dest_start: usize,
}
pub struct PlanHitReadC {
    pub body: usize, pub saved_env: EnvRootC, pub arm_env: EnvRootC, pub origin: usize,
}
#[derive(Clone, Copy)]
pub struct BuildReadC {
    pub common: HelperCommon, pub type_id: TypeIdC, pub ctor_id: CtorIdC,
    pub src: ValSliceC, pub src_cursor: usize, pub field_start: usize,
    pub staged: Option<RefC>, pub node: Option<RefC>, pub phase: BuildPhase,
}
impl BuildReadC {
    pub fn helper(self) -> HelperControlC {
        HelperControlC::Build { common: self.common, type_id: self.type_id,
            ctor_id: self.ctor_id, src: self.src, src_cursor: self.src_cursor,
            field_start: self.field_start, staged: self.staged, node: self.node, phase: self.phase }
    }
}
pub struct BuildEntryReadC {
    pub common: HelperCommon, pub type_id: TypeIdC, pub ctor_id: CtorIdC,
    pub src: ValSliceC, pub field_start: usize,
}
pub struct BuildCellReadC { pub state: BuildReadC, pub value: RefC }
pub struct BuildFieldReadC {
    pub state: BuildReadC, pub staged: RefC, pub next_cursor: usize,
    pub fresh_node: RefC,
}
#[derive(Clone, Copy)]
pub struct PlanReadC {
    pub common: HelperCommon, pub source_match: usize, pub scrutinee: RefC,
    pub saved_env: EnvRootC, pub partial: EnvRootC, pub node: usize,
    pub phase: PlanPhase, pub after_walk: PlanAfterWalk, pub cur: RefC,
    pub path: SliceC, pub path_cursor: usize, pub arm: Option<usize>, pub status: TriStatus,
}
impl PlanReadC {
    pub fn helper(self) -> HelperControlC {
        HelperControlC::Plan { common: self.common, source_match: self.source_match,
            scrutinee: self.scrutinee, saved_env: self.saved_env, partial: self.partial,
            node: self.node, phase: self.phase, after_walk: self.after_walk,
            cur: self.cur, path: self.path, path_cursor: self.path_cursor,
            arm: self.arm, status: self.status }
    }
}
pub struct PlanEntryReadC {
    pub common: HelperCommon, pub source_match: usize, pub scrutinee: RefC,
    pub saved_env: EnvRootC, pub root: usize, pub row: PlanNodeC,
}
pub struct PlanWalkReadC {
    pub state: PlanReadC, pub value: RefC, pub next_cursor: usize, pub next_phase: PlanPhase,
}
pub struct PlanSwitchReadC {
    pub state: PlanReadC, pub case: Option<usize>, pub default: usize,
}
pub struct PlanBindReadC {
    pub state: PlanReadC, pub binder: usize, pub next: usize, pub fresh_env: usize,
}

/// Ordinary table-payload loading, after the owner's checked row read. Every
/// node starts its own root-first walk from the ORIGINAL scrutinee; no cache or
/// shared test suppresses a visit. This contains no value/type/constructor test.
pub fn load_plan_node(mut state: PlanReadC, id: usize, row: PlanNodeC) -> PlanReadC {
    state.node = id;
    state.cur = state.scrutinee;
    state.path_cursor = 0;
    match row {
        PlanNodeC::Switch { path, .. } => {
            state.path = path; state.after_walk = PlanAfterWalk::Test; state.arm = None;
            state.phase = if path.len == 0 { PlanPhase::Test } else { PlanPhase::Walk };
        }
        PlanNodeC::Bind { path, .. } => {
            state.path = path; state.after_walk = PlanAfterWalk::Bind; state.arm = None;
            state.phase = if path.len == 0 { PlanPhase::Bind } else { PlanPhase::Walk };
        }
        PlanNodeC::Select { arm, .. } => {
            state.path = SliceC::empty(); state.after_walk = PlanAfterWalk::Test;
            state.arm = Some(arm); state.phase = PlanPhase::Ready;
        }
        PlanNodeC::NoMatch { .. } => {
            state.path = SliceC::empty(); state.after_walk = PlanAfterWalk::Test;
            state.arm = None; state.phase = PlanPhase::NoMatchReady;
        }
    }
    state
}

/// Implement only on the concrete owner PreImage and its recording read-ledger
/// forwarding, bound to the exact Selected/pre-image. These declarations own no
/// reads/guard policy themselves. No whole ProgramView or mutable arena escapes.
pub trait AdtRuntimeReads {
    fn foreign_rule(&self, selected: &Selected) -> HostViolation;
    fn nullary_construct(&self, selected: &Selected) -> Result<RefC, HostViolation>;
    fn construct_fields(&self, selected: &Selected) -> Result<ConstructFieldsReadC, HostViolation>;
    fn constructor_frame(&self, selected: &Selected) -> Result<ConstructorFieldsReadC, HostViolation>;
    fn constructor_next(&self, selected: &Selected) -> Result<ConstructorNextReadC, HostViolation>;
    fn constructor_final(&self, selected: &Selected) -> Result<ConstructorFinalReadC, HostViolation>;
    fn constructor_scalar(&self, selected: &Selected) -> Result<ConstructorScalarReadC, HostViolation>;
    fn constructor_append(&self, selected: &Selected) -> Result<ConstructorAppendReadC, HostViolation>;
    fn application_error(&self, selected: &Selected) -> Result<u8, HostViolation>;
    fn materialized_value(&self, selected: &Selected) -> Result<RefC, HostViolation>;
    fn plan_hit(&self, selected: &Selected) -> Result<PlanHitReadC, HostViolation>;
    fn build_entry(&self, selected: &Selected) -> Result<BuildEntryReadC, HostViolation>;
    fn build_cell(&self, selected: &Selected) -> Result<BuildCellReadC, HostViolation>;
    fn build_field(&self, selected: &Selected) -> Result<BuildFieldReadC, HostViolation>;
    fn build_done(&self, selected: &Selected) -> Result<RefC, HostViolation>;
    fn plan_entry(&self, selected: &Selected) -> Result<PlanEntryReadC, HostViolation>;
    fn plan_walk(&self, selected: &Selected) -> Result<PlanWalkReadC, HostViolation>;
    /// Owner performs kind/type/ctor-class/caseSlot judgment and exact checked
    /// reads. case is a real present planCases target or None for default.
    fn plan_switch(&self, selected: &Selected) -> Result<PlanSwitchReadC, HostViolation>;
    fn plan_bind(&self, selected: &Selected) -> Result<PlanBindReadC, HostViolation>;
    fn plan_state(&self, selected: &Selected) -> Result<PlanReadC, HostViolation>;
    fn plan_node(&self, selected: &Selected, id: &usize) -> Result<PlanNodeC, HostViolation>;
    fn plan_matched(&self, selected: &Selected) -> Result<(usize, EnvRootC), HostViolation>;
}
