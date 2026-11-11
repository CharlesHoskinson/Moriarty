//! Per-rule memory maxima and the one-arena non-interleaving check
//! (research/sherlock-final-design-20261111/EXPERIMENTS-MAXIMA.md).
//! Test-only. Nothing here is used by the owner, a family or a circuit.
//!
//! Two layers per rule:
//! * static: a rule-sliced text scan of the family, fragment, ADT-runtime
//!   and guard code (`scan`). It gives which `PlannedWrites` fields a rule
//!   can set (upper bound of appends and stack operations) and read call
//!   sites (upper bound of reads when the sliced code has no loop).
//! * dynamic: real owner steps over a corpus. Before each step the same
//!   selection and family plan run over a logging `PreImageView`; every
//!   read is logged as a record; appends follow the plan in the order of
//!   `owner_apply_validated`. A one-arena address map (DATA, FIELDS, ENV and
//!   SLOTS in one append chain; frames and tasks outside) and a four-arena
//!   map give segments, flushed (old) segments and read ages.
//!
//! The test asserts that every dynamic value is within the static bound,
//! the step-local and slice-contiguity properties on every corpus step, and
//! the over-all-rules premises of the non-interleaving argument. With
//! `MAXIMA_OUT=<dir>` it writes `rule_maxima.csv`, `rule_maxima.md` and
//! `noninterleave.json` there.
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use super::*;
use crate::sherlock::helper_machine::families::HostFamilyPlans;
use crate::sherlock::helper_machine::shared::owner_ext_source_program::ExtDomain;
use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;

mod scan;

const SRC_SHARED: &str = include_str!("../../../shared.rs");
const SRC_OWNER: &str = include_str!("../../owner_runtime.rs");
const SRC_ADT: &str = include_str!("../owner_adt_runtime.rs");
const SRC_F_LEGACY: &str = include_str!("../../../families/legacy.rs");
const SRC_FR_LEGACY: &str = include_str!("../../../fragments/legacy.proposed.rs");
const SRC_F_SCALAR: &str = include_str!("../../../families/scalar.rs");
const SRC_FR_SCALAR: &str = include_str!("../../../fragments/scalar.proposed.rs");
const SRC_F_ENV: &str = include_str!("../../../families/environment.rs");
const SRC_F_STRUCT: &str = include_str!("../../../families/structural.rs");
const SRC_F_BUILD: &str = include_str!("../../../families/adt_build.rs");
const SRC_F_PLAN: &str = include_str!("../../../families/adt_plan.rs");
const SRC_F_CTOR: &str = include_str!("../../../families/adt_constructor.rs");

// ---------------------------------------------------------------- kinds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Mem {
    Data = 0,
    Field = 1,
    Env = 2,
    Slot = 3,
}
const MEMS: [Mem; 4] = [Mem::Data, Mem::Field, Mem::Env, Mem::Slot];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Rom {
    Expr,
    Child,
    Function,
    FnCount,
    Parameter,
    Arm,
    Pattern,
    Occurrence,
    OccPath,
    Const,
    ConstField,
    CtorType,
    ProgView,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Rd {
    M(Mem, usize),
    R(Rom, usize),
}

fn rule_name(r: &RuleIdC) -> String {
    format!("{r:?}")
}

// ---------------------------------------------------------------- logging view
struct LogView<'a> {
    inner: &'a PreImage<'a>,
    log: RefCell<Vec<Rd>>,
}

impl<'a> LogView<'a> {
    fn new(inner: &'a PreImage<'a>) -> Self {
        Self {
            inner,
            log: RefCell::new(Vec::new()),
        }
    }
    fn put(&self, r: Rd) {
        self.log.borrow_mut().push(r);
    }
    fn val(&self, r: &RefC) {
        match r {
            RefC::Data(i) => self.put(Rd::M(Mem::Data, *i)),
            RefC::Const(i) => self.put(Rd::R(Rom::Const, *i)),
        }
    }
    fn ctor(&self, k: &LogicalKind) {
        if let LogicalKind::Constructor { type_id, ctor_id } = k {
            self.put(Rd::R(Rom::CtorType, type_id.0 * 4096 + ctor_id.0));
        }
    }
    fn take(&self) -> Vec<Rd> {
        std::mem::take(&mut *self.log.borrow_mut())
    }
}

impl<'a> PreImageView for LogView<'a> {
    fn adt_reads(&self) -> &dyn adt_runtime::AdtRuntimeReads {
        self
    }
    fn control(&self) -> &ControlC {
        self.inner.control()
    }
    fn control_origin(&self) -> usize {
        self.inner.control_origin()
    }
    fn env_root(&self) -> EnvRootC {
        self.inner.env_root()
    }
    fn depth(&self) -> usize {
        self.inner.depth()
    }
    fn task_live(&self) -> usize {
        self.inner.task_live()
    }
    fn stack_limit(&self) -> usize {
        self.inner.stack_limit()
    }
    fn outcome(&self) -> &OutcomeC {
        self.inner.outcome()
    }
    fn initial_fuel(&self) -> u128 {
        self.inner.initial_fuel()
    }
    fn remaining_work(&self) -> u128 {
        self.inner.remaining_work()
    }
    fn consumed_work(&self) -> u128 {
        self.inner.consumed_work()
    }
    fn helper_work(&self) -> u128 {
        self.inner.helper_work()
    }
    fn helper_visits(&self) -> u128 {
        self.inner.helper_visits()
    }
    fn reference_cursor(&self) -> u128 {
        self.inner.reference_cursor()
    }
    fn helper(&self) -> Option<&HelperControlC> {
        self.inner.helper()
    }
    fn result(&self) -> Option<&HelperResultC> {
        self.inner.result()
    }
    fn top_frame(&self) -> Option<&FrameRowC> {
        self.inner.top_frame()
    }
    fn top_task(&self) -> Option<&HelperTaskC> {
        self.inner.top_task()
    }
    fn active(&self) -> ActiveLengths {
        self.inner.active()
    }
    fn alloc(&self) -> AllocCounters {
        self.inner.alloc()
    }
    fn expr(&self, id: &usize) -> Result<&ExpressionRowC, HostViolation> {
        self.put(Rd::R(Rom::Expr, *id));
        self.inner.expr(id)
    }
    fn child(&self, pos: &usize) -> Result<usize, HostViolation> {
        self.put(Rd::R(Rom::Child, *pos));
        self.inner.child(pos)
    }
    fn function_row(&self, id: &usize) -> Result<&FunctionRowC, HostViolation> {
        self.put(Rd::R(Rom::Function, *id));
        self.inner.function_row(id)
    }
    fn admitted_function_count(&self) -> Result<usize, HostViolation> {
        self.put(Rd::R(Rom::FnCount, 0));
        self.inner.admitted_function_count()
    }
    fn parameter(&self, pos: &usize) -> Result<&ParameterRowC, HostViolation> {
        self.put(Rd::R(Rom::Parameter, *pos));
        self.inner.parameter(pos)
    }
    fn arm(&self, id: &usize) -> Result<&ArmRowC, HostViolation> {
        self.put(Rd::R(Rom::Arm, *id));
        self.inner.arm(id)
    }
    fn pattern(&self, id: &usize) -> Result<&PatternRowC, HostViolation> {
        self.put(Rd::R(Rom::Pattern, *id));
        self.inner.pattern(id)
    }
    fn occurrence(&self, id: &usize) -> Result<&OccurrenceRowC, HostViolation> {
        self.put(Rd::R(Rom::Occurrence, *id));
        self.inner.occurrence(id)
    }
    fn occurrence_path(&self, s: &SliceC) -> Result<&[usize], HostViolation> {
        for k in 0..s.len {
            self.put(Rd::R(Rom::OccPath, s.start + k));
        }
        self.inner.occurrence_path(s)
    }
    fn constant_borrowed(
        &self,
        id: &usize,
    ) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation> {
        self.put(Rd::R(Rom::Const, *id));
        self.inner.constant_borrowed(id)
    }
    fn constant_field(&self, id: &usize) -> Result<usize, HostViolation> {
        self.put(Rd::R(Rom::ConstField, *id));
        self.inner.constant_field(id)
    }
    fn marker_free(&self, id: &usize) -> Result<bool, HostViolation> {
        self.put(Rd::R(Rom::Const, *id));
        self.inner.marker_free(id)
    }
    fn deref_borrowed(&self, r: &RefC) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation> {
        self.val(r);
        self.inner.deref_borrowed(r)
    }
    fn field_at(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation> {
        self.val(r);
        match r {
            RefC::Data(j) => {
                if let Some(row) = self.inner.img.data.get(*j) {
                    let (start, _) = data_fields(row);
                    self.put(Rd::M(Mem::Field, start + *i));
                }
            }
            RefC::Const(j) => {
                if let Ok(v) = pure_view(self.inner)
                    && let Some(row) = v.constants.get(*j)
                {
                    let (start, _) = const_fields(row);
                    self.put(Rd::R(Rom::ConstField, start + *i));
                }
            }
        }
        self.inner.field_at(r, i)
    }
    fn env_cell(&self, id: &usize) -> Result<&EnvCellC, HostViolation> {
        self.put(Rd::M(Mem::Env, *id));
        self.inner.env_cell(id)
    }
    fn value_slot(&self, pos: &usize) -> Result<RefC, HostViolation> {
        self.put(Rd::M(Mem::Slot, *pos));
        self.inner.value_slot(pos)
    }
    fn kind_of(&self, r: &RefC) -> Result<LogicalKind, HostViolation> {
        self.val(r);
        self.inner.kind_of(r)
    }
    fn u128_of(&self, r: &RefC) -> Result<u128, HostViolation> {
        self.val(r);
        self.inner.u128_of(r)
    }
    fn bool_of(&self, r: &RefC) -> Result<bool, HostViolation> {
        self.val(r);
        self.inner.bool_of(r)
    }
    fn checked_arity(&self, k: &LogicalKind) -> Result<usize, HostViolation> {
        self.ctor(k);
        self.inner.checked_arity(k)
    }
    fn checked_sort(&self, k: &LogicalKind) -> Result<TypeIdC, HostViolation> {
        self.ctor(k);
        self.inner.checked_sort(k)
    }
    fn scalar_test_type_ok(&self, r: &RefC, expected: &TypeIdC) -> Result<bool, HostViolation> {
        self.val(r);
        if let Ok(k) = self.inner.kind_of(r) {
            self.ctor(&k);
        }
        self.inner.scalar_test_type_ok(r, expected)
    }
}

impl<'a> AdtPreImageAccess for LogView<'a> {
    fn program_view(&self) -> Result<&ProgramView, HostViolation> {
        self.put(Rd::R(Rom::ProgView, 0));
        pure_view(self.inner)
    }
}

impl_adt_runtime_reads!(LogView<'a>);

// ---------------------------------------------------------------- rule universe
const KINDS: [HelperKind; 11] = [
    HelperKind::Scalar,
    HelperKind::EnvGet,
    HelperKind::EnvPut,
    HelperKind::Find,
    HelperKind::Bind,
    HelperKind::Append,
    HelperKind::Validate,
    HelperKind::Equal,
    HelperKind::Match,
    HelperKind::Plan,
    HelperKind::Build,
];
const SITES: [SiteTag; 27] = [
    SiteTag::EvalLiteral,
    SiteTag::EvalVar,
    SiteTag::EvalLet,
    SiteTag::EvalBinary,
    SiteTag::EvalUnary,
    SiteTag::EvalIf,
    SiteTag::EvalMatch,
    SiteTag::EvalCall,
    SiteTag::Push,
    SiteTag::RetLet,
    SiteTag::RetLeft,
    SiteTag::RetRight,
    SiteTag::RetUnary,
    SiteTag::RetIf,
    SiteTag::RetMatch,
    SiteTag::RetArgs,
    SiteTag::RetCall,
    SiteTag::RetScope,
    SiteTag::Invoke,
    SiteTag::Enter,
    SiteTag::Bound,
    SiteTag::Cases,
    SiteTag::CaseResult,
    SiteTag::Prim,
    SiteTag::EvalConstruct,
    SiteTag::RetCtorFields,
    SiteTag::Materialize,
];
const PUBLISH: [RuleIdC; 3] = [
    RuleIdC::KAdminPublishValue,
    RuleIdC::KAdminPublishError,
    RuleIdC::KAdminPublishExhausted,
];

/// Variant names of `pub enum <name>` in shared.rs, in source order.
fn enum_names(name: &str) -> Vec<String> {
    let src = scan::strip(SRC_SHARED);
    let head = format!("pub enum {name} {{");
    let p = src.find(&head).unwrap_or_else(|| panic!("enum {name}"));
    let body = &src[p + head.len()..];
    let e = body.find('}').unwrap_or(body.len());
    body[..e]
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

struct Universe {
    /// Every enabled step rule, in group order, with "in the 108" flag.
    rules: Vec<(RuleIdC, bool)>,
    unreached: Vec<String>,
}

fn universe() -> Universe {
    // The 108 of mori-backend `supported_rules()`: publications, helper
    // groups of kinds 1..=9 (Scalar..Match), legacy groups, nine entries.
    let mut v108: Vec<RuleIdC> = PUBLISH.to_vec();
    let nine = &KINDS[..9];
    for k in nine {
        for r in helper_group(*k) {
            if !v108.contains(r) {
                v108.push(*r);
            }
        }
    }
    for s in SITES {
        for r in legacy_group(s) {
            if !v108.contains(r) {
                v108.push(*r);
            }
        }
    }
    for k in nine {
        v108.push(entry_rule(*k));
    }
    let mut all: Vec<(RuleIdC, bool)> = v108.iter().map(|r| (*r, true)).collect();
    for k in [HelperKind::Plan, HelperKind::Build] {
        let mut extra = vec![entry_rule(k)];
        extra.extend_from_slice(helper_group(k));
        for r in extra {
            if !all.iter().any(|(x, _)| *x == r) {
                all.push((r, false));
            }
        }
    }
    let names: BTreeSet<String> = all.iter().map(|(r, _)| rule_name(r)).collect();
    let unreached = enum_names("RuleIdC")
        .into_iter()
        .filter(|n| !names.contains(n))
        .collect();
    Universe {
        rules: all,
        unreached,
    }
}

fn group_of(r: RuleIdC) -> String {
    for k in KINDS {
        if entry_rule(k) == r {
            return format!("entry:{k:?}");
        }
    }
    for k in KINDS {
        if helper_group(k).contains(&r) {
            return format!("helper:{k:?}");
        }
    }
    for s in SITES {
        if legacy_group(s).contains(&r) {
            return format!("legacy:{s:?}");
        }
    }
    if PUBLISH.contains(&r) {
        return "publish".into();
    }
    "other".into()
}

// ---------------------------------------------------------------- static scan
#[derive(Default, Clone, Debug)]
struct Static {
    family: String,
    reached: Vec<String>,
    alloc_data: bool,
    data_variants: BTreeSet<String>,
    alloc_data_field: bool,
    alloc_constructor: bool,
    alloc_env: bool,
    alloc_value_slot: bool,
    frame_push: bool,
    frame_pop: bool,
    frame_rewrite: bool,
    task_push: bool,
    task_pop: bool,
    task_rewrite: bool,
    /// read call sites: value (DATA or CONST row), field_at, env, slot, ROM
    s_value: usize,
    s_field_at: usize,
    s_env: usize,
    s_slot: usize,
    s_rom: usize,
    s_sel_value: usize,
    s_sel_field_at: usize,
    s_sel_env: usize,
    s_sel_slot: usize,
    loops: Vec<String>,
    mentions_active_slots: bool,
    mentions_active_fields: bool,
}

impl Static {
    fn app(&self) -> [usize; 4] {
        let fl = self
            .data_variants
            .iter()
            .map(|v| match v.as_str() {
                "Cons" | "Branch" => 2,
                "Leaf" => 1,
                _ => 0,
            })
            .max()
            .unwrap_or(if self.alloc_data { 2 } else { 0 });
        let a_data = if self.alloc_data { 1 } else { 0 };
        let a_fields = if self.alloc_data { fl } else { 0 };
        let b_data = usize::from(self.alloc_constructor);
        let b_fields = usize::from(self.alloc_data_field);
        // owner_validate_plan refuses alloc_data together with a Build field
        // or constructor, so the two DATA+FIELDS shapes are alternatives.
        let (d, f) = if a_data + a_fields >= b_data + b_fields {
            (a_data, a_fields)
        } else {
            (b_data, b_fields)
        };
        [
            d,
            f,
            usize::from(self.alloc_env),
            usize::from(self.alloc_value_slot),
        ]
    }
    fn heap_read_bound(&self) -> usize {
        self.s_value
            + 2 * self.s_field_at
            + self.s_env
            + self.s_slot
            + self.s_sel_value
            + 2 * self.s_sel_field_at
            + self.s_sel_env
            + self.s_sel_slot
    }
}

const VALUE_READS: [&str; 5] = [
    "deref_borrowed",
    "kind_of",
    "u128_of",
    "bool_of",
    "scalar_test_type_ok",
];
const ROM_READS: [&str; 15] = [
    "expr",
    "child",
    "function_row",
    "admitted_function_count",
    "parameter",
    "arm",
    "pattern",
    "occurrence",
    "occurrence_path",
    "constant_borrowed",
    "constant_field",
    "marker_free",
    "checked_arity",
    "checked_sort",
    "program_view",
];

fn modules() -> Vec<scan::Module> {
    vec![
        scan::Module {
            name: "legacy",
            texts: vec![SRC_F_LEGACY, SRC_FR_LEGACY],
        },
        scan::Module {
            name: "scalar",
            texts: vec![SRC_F_SCALAR, SRC_FR_SCALAR],
        },
        scan::Module {
            name: "environment",
            texts: vec![SRC_F_ENV],
        },
        scan::Module {
            name: "structural",
            texts: vec![SRC_F_STRUCT],
        },
        scan::Module {
            name: "adt_build",
            texts: vec![SRC_F_BUILD],
        },
        scan::Module {
            name: "adt_plan",
            texts: vec![SRC_F_PLAN],
        },
        scan::Module {
            name: "adt_constructor",
            texts: vec![SRC_F_CTOR],
        },
        scan::Module {
            name: "owner_adt_runtime",
            texts: vec![SRC_ADT],
        },
        scan::Module {
            name: "owner_runtime",
            texts: vec![SRC_OWNER],
        },
    ]
}

fn family_entry(f: FamilyId) -> (&'static str, &'static str) {
    match f {
        FamilyId::Legacy => ("legacy", "dispatch"),
        FamilyId::Scalar => ("scalar", "dispatch"),
        FamilyId::EnvGet => ("environment", "plan_env_get"),
        FamilyId::EnvPut => ("environment", "plan_env_put"),
        FamilyId::Find => ("environment", "plan_find"),
        FamilyId::Bind => ("structural", "plan_bind"),
        FamilyId::Append => ("environment", "plan_append"),
        FamilyId::Validate => ("structural", "plan_validate"),
        FamilyId::Equal => ("structural", "plan_equal"),
        FamilyId::Match => ("structural", "plan_match"),
        FamilyId::BookkeepingRelease => ("environment", "plan_bookkeeping_release"),
        FamilyId::Build => ("adt_build", "dispatch"),
        FamilyId::Plan => ("adt_plan", "dispatch"),
    }
}

fn fn_body(idx: &scan::Index, module: &str, name: &str) -> String {
    idx.fns
        .iter()
        .find(|f| f.module == module && f.name == name)
        .map(|f| f.body.clone())
        .unwrap_or_else(|| panic!("fn {module}::{name}"))
}

fn analyse_static(idx: &scan::Index, r: RuleIdC) -> Static {
    let name = rule_name(&r);
    let mut s = Static::default();
    if PUBLISH.contains(&r) {
        // P1/P2 of owner_step: outcome write only, no family, no select_when.
        s.family = "owner_step publish (P1/P2)".into();
        return s;
    }
    let fam = family_of_rule(&r);
    s.family = format!("{fam:?}");
    let (text, reached) = idx.region(&name, family_entry(fam));
    s.reached = reached;
    s.alloc_data = scan::sets_field(&text, "alloc_data");
    for v in ["U128", "Bool", "Nil", "Cons", "Leaf", "Branch"] {
        if text.contains(&format!("AllocDataWrite::{v}")) {
            s.data_variants.insert(v.into());
        }
    }
    s.alloc_data_field = scan::sets_field(&text, "alloc_data_field");
    s.alloc_constructor = scan::sets_field(&text, "alloc_constructor");
    s.alloc_env = scan::sets_field(&text, "alloc_env");
    s.alloc_value_slot = scan::sets_field(&text, "alloc_value_slot");
    s.task_rewrite = scan::sets_field(&text, "task_rewrite_top");
    s.frame_push = text.contains("FrameWrite::Push");
    s.frame_pop = text.contains("FrameWrite::Pop");
    s.frame_rewrite = text.contains("FrameWrite::RewriteTop");
    s.task_push = text.contains("TaskOp::Push");
    s.task_pop = text.contains("TaskOp::Pop");
    s.s_value = VALUE_READS
        .iter()
        .map(|m| scan::count_method(&text, m))
        .sum();
    s.s_field_at = scan::count_method(&text, "field_at");
    s.s_env = scan::count_method(&text, "env_cell");
    s.s_slot = scan::count_method(&text, "value_slot");
    s.s_rom = ROM_READS.iter().map(|m| scan::count_method(&text, m)).sum();
    for w in ["for", "while", "loop"] {
        if scan::count_word(&text, w) > 0 {
            s.loops.push(w.into());
        }
    }
    for m in ["iter", "into_iter", "iter_mut", "windows", "chunks"] {
        if scan::count_method(&text, m) > 0 {
            s.loops.push(format!(".{m}()"));
        }
    }
    s.mentions_active_slots = text.contains("active().value_slots");
    s.mentions_active_fields = text.contains("active().data_fields");
    // Selection: select_when arms of this rule and of every rule before it
    // in its group (owner_select evaluates them in order); entries are
    // selected by owner_helper_plan (no family read, ROM and frame only).
    let sel_body = scan::strip(&fn_body(idx, "owner_runtime", "owner_select_when"));
    let mut group: Vec<RuleIdC> = Vec::new();
    for k in KINDS {
        if helper_group(k).contains(&r) {
            group = helper_group(k).to_vec();
        }
    }
    for st in SITES {
        if legacy_group(st).contains(&r) {
            group = legacy_group(st).to_vec();
        }
    }
    let mut sel_text = String::new();
    for g in group {
        let pruned = scan::prune(&sel_body, &rule_name(&g), &idx.rules);
        sel_text.push_str(&pruned);
        for (gname, garm) in &idx.guard_arms {
            if pruned.contains(&format!("GuardId::{gname}")) {
                sel_text.push_str(garm);
            }
        }
        if g == r {
            break;
        }
    }
    s.s_sel_value = VALUE_READS
        .iter()
        .map(|m| scan::count_method(&sel_text, m))
        .sum();
    s.s_sel_field_at = scan::count_method(&sel_text, "field_at");
    s.s_sel_env = scan::count_method(&sel_text, "env_cell");
    s.s_sel_slot = scan::count_method(&sel_text, "value_slot");
    s
}

// ---------------------------------------------------------------- dynamic
/// One-arena and four-arena address maps.
struct Arena {
    len: usize,
    addr: [Vec<usize>; 4],
    born: [Vec<i64>; 4],
}

impl Arena {
    fn new(img: &MachineImage) -> Self {
        let mut a = Arena {
            len: 0,
            addr: Default::default(),
            born: Default::default(),
        };
        let lens = [
            img.data.len(),
            img.data_fields.len(),
            img.env.len(),
            img.value_slots.len(),
        ];
        for (k, n) in MEMS.iter().zip(lens) {
            for _ in 0..n {
                a.push(*k, -1);
            }
        }
        a
    }
    fn push(&mut self, k: Mem, step: i64) {
        self.addr[k as usize].push(self.len);
        self.born[k as usize].push(step);
        self.len += 1;
    }
    fn klen(&self, k: Mem) -> usize {
        self.addr[k as usize].len()
    }
}

#[derive(Default, Clone, Debug)]
struct Obs {
    fired: usize,
    applied: usize,
    app: [usize; 4],
    app_total: usize,
    frame_push: usize,
    frame_pop: usize,
    frame_rewrite: usize,
    task_push: usize,
    task_pop: usize,
    task_rewrite: usize,
    rd: [usize; 4],
    rd_heap: usize,
    rd_sel_heap: usize,
    rd_rom: usize,
    rom_tables: usize,
    rom_blocks: usize,
    seg_read_1: usize,
    seg_old_1: usize,
    seg_app_1: usize,
    seg_all_1: usize,
    seg_read_4: usize,
    seg_old_4: usize,
    seg_app_4: usize,
    seg_all_4: usize,
    max_age: i64,
    old_seen: bool,
}

impl Obs {
    fn merge(&mut self, o: &Obs) {
        self.fired += o.fired;
        self.applied += o.applied;
        for k in 0..4 {
            self.app[k] = self.app[k].max(o.app[k]);
            self.rd[k] = self.rd[k].max(o.rd[k]);
        }
        macro_rules! mx {
            ($($f:ident),*) => { $( self.$f = self.$f.max(o.$f); )* };
        }
        mx!(
            app_total,
            frame_push,
            frame_pop,
            frame_rewrite,
            task_push,
            task_pop,
            task_rewrite,
            rd_heap,
            rd_sel_heap,
            rd_rom,
            rom_tables,
            rom_blocks,
            seg_read_1,
            seg_old_1,
            seg_app_1,
            seg_all_1,
            seg_read_4,
            seg_old_4,
            seg_app_4,
            seg_all_4,
            max_age
        );
        self.old_seen |= o.old_seen;
    }
}

struct Planned {
    rule: RuleIdC,
    appends: Vec<Mem>,
    frame: (usize, usize, usize),
    task: (usize, usize, usize),
    reads: Vec<Rd>,
    sel_reads: Vec<Rd>,
    pre_helper: Option<HelperKind>,
}

fn call_family_logged(
    fam: &HostFamilyPlans,
    sel: &Selected,
    lv: &LogView<'_>,
    g: &OwnerGuards,
) -> Result<EffectPlan, HostViolation> {
    match family_of_rule(&sel.rule) {
        FamilyId::Legacy => fam.plan_legacy(sel, lv, g),
        FamilyId::Scalar => fam.plan_scalar(sel, lv, g),
        FamilyId::EnvGet => fam.plan_env_get(sel, lv, g),
        FamilyId::EnvPut => fam.plan_env_put(sel, lv, g),
        FamilyId::Find => fam.plan_find(sel, lv, g),
        FamilyId::Bind => fam.plan_bind(sel, lv, g),
        FamilyId::Append => fam.plan_append(sel, lv, g),
        FamilyId::Validate => fam.plan_validate(sel, lv, g),
        FamilyId::Equal => fam.plan_equal(sel, lv, g),
        FamilyId::Match => fam.plan_match(sel, lv, g),
        FamilyId::Build => fam.plan_build(sel, lv, g),
        FamilyId::Plan => fam.plan_plan(sel, lv, g),
        FamilyId::BookkeepingRelease => fam.plan_bookkeeping_release(sel, lv, g),
    }
}

/// Selection (select_when reads, logged) and the family plan for a
/// pre-image that owner_step will select on (no publish, fuel left).
fn plan_logged(
    pre: &PreImage<'_>,
    fam: &HostFamilyPlans,
    g: &OwnerGuards,
) -> Result<Planned, HostViolation> {
    let sel = owner_select(pre, g)?;
    let lv = LogView::new(pre);
    if let Some(h) = pre.helper() {
        for r in helper_group(helper_kind_of(h)) {
            if g.eval_select_when(r, &lv)? {
                break;
            }
        }
    } else {
        let site = owner_site(pre)?;
        if pre.result().is_some() {
            for r in legacy_group(site) {
                if g.reads_result_root(r) && g.eval_select_when(r, &lv)? {
                    break;
                }
            }
        } else if owner_helper_plan(site, pre).is_none() {
            for r in legacy_group(site) {
                if !g.reads_result_root(r) && g.eval_select_when(r, &lv)? {
                    break;
                }
            }
        }
    }
    let sel_reads = lv.take();
    let plan = call_family_logged(fam, &sel, &lv, g)?;
    let reads = lv.take();
    let w = &plan.writes;
    let mut appends = Vec::new();
    if let Some(a) = &w.alloc_data {
        for _ in 0..a.field_len() {
            appends.push(Mem::Field);
        }
        appends.push(Mem::Data);
    }
    if w.alloc_data_field.is_some() {
        appends.push(Mem::Field);
    }
    if w.alloc_constructor.is_some() {
        appends.push(Mem::Data);
    }
    if w.alloc_env.is_some() {
        appends.push(Mem::Env);
    }
    if w.alloc_value_slot.is_some() {
        appends.push(Mem::Slot);
    }
    let frame = match w.frame {
        Some(FrameWrite::Push(_)) => (1, 0, 0),
        Some(FrameWrite::Pop) => (0, 1, 0),
        Some(FrameWrite::RewriteTop(_)) => (0, 0, 1),
        None => (0, 0, 0),
    };
    let task = (
        usize::from(matches!(w.task_op, TaskOp::Push(_))),
        usize::from(matches!(w.task_op, TaskOp::Pop)),
        usize::from(w.task_rewrite_top.is_some()),
    );
    Ok(Planned {
        rule: sel.rule,
        appends,
        frame,
        task,
        reads,
        sel_reads,
        pre_helper: pre.helper().map(helper_kind_of),
    })
}

fn slot_slices(img: &MachineImage) -> Vec<(&'static str, SliceC)> {
    let mut v = Vec::new();
    match &img.control {
        ControlC::Invoke { args, .. } | ControlC::Enter { args, .. } => {
            v.push(("control.args", *args))
        }
        ControlC::Materialize { fields, .. } => v.push(("control.materialize", *fields)),
        ControlC::ScheduleNext {
            frame:
                FrameRowC::ArgsF { completed, .. } | FrameRowC::ConstructorFieldsF { completed, .. },
            ..
        } => v.push(("control.schedule.completed", *completed)),
        _ => {}
    }
    for f in &img.frames {
        if let FrameRowC::ArgsF { completed, .. }
        | FrameRowC::ConstructorFieldsF { completed, .. } = f
        {
            v.push(("frame.completed", *completed));
        }
    }
    match &img.helper {
        Some(HelperControlC::Bind { args, .. }) => v.push(("helper.bind.args", *args)),
        Some(HelperControlC::Append {
            src, dest_start, ..
        }) => {
            v.push(("helper.append.src", *src));
            let done = img.value_slots.len().saturating_sub(*dest_start);
            v.push((
                "helper.append.dest_partial",
                SliceC::normalized(*dest_start, done),
            ));
        }
        Some(HelperControlC::Build { src, .. }) => v.push(("helper.build.src", *src)),
        _ => {}
    }
    if let Some(HelperResultC::AppendDone { seq }) = &img.result {
        v.push(("result.append_done", *seq));
    }
    v
}

fn field_slices(img: &MachineImage) -> Vec<(&'static str, SliceC)> {
    let mut v = Vec::new();
    for row in &img.data {
        let (s, l) = data_fields(row);
        if l > 0 {
            v.push(("data.fields", SliceC::normalized(s, l)));
        }
    }
    if let Some(HelperControlC::Build {
        field_start,
        src_cursor,
        node: None,
        ..
    }) = &img.helper
    {
        v.push((
            "helper.build.fields_partial",
            SliceC::normalized(*field_start, *src_cursor),
        ));
    }
    v
}

#[derive(Default)]
struct Checks {
    steps: usize,
    applied: usize,
    slices_checked: usize,
    a_violations: Vec<String>,
    b_violations: Vec<String>,
    exclusivity_violations: Vec<String>,
    entry_violations: Vec<String>,
    static_violations: Vec<String>,
    max_cells_per_step: usize,
}

fn check_contiguous(a: &Arena, k: Mem, what: &str, s: SliceC, ctx: &str, out: &mut Checks) {
    if s.len < 2 {
        return;
    }
    out.slices_checked += 1;
    let addr = &a.addr[k as usize];
    let Some(base) = addr.get(s.start) else {
        out.b_violations
            .push(format!("{ctx}: {what} start {} beyond {k:?}", s.start));
        return;
    };
    for t in 1..s.len {
        match addr.get(s.start + t) {
            Some(x) if *x == base + t => {}
            other => {
                out.b_violations.push(format!(
                    "{ctx}: {what} {k:?}[{}..+{}] element {t} at {other:?}, expected {}",
                    s.start,
                    s.len,
                    base + t
                ));
                return;
            }
        }
    }
}

fn kind_runs_ok(order: &[Mem]) -> bool {
    let mut seen = Vec::new();
    for (i, k) in order.iter().enumerate() {
        if i > 0 && order[i - 1] == *k {
            continue;
        }
        if seen.contains(k) {
            return false;
        }
        seen.push(*k);
    }
    true
}

fn rom_table(r: Rom) -> usize {
    r as usize
}

struct Program {
    name: &'static str,
    source: String,
    fuel: u128,
    stack: u128,
    data_nodes: usize,
}

fn bounds_for(p: &Program) -> (BoundsC, LayoutSchemaC) {
    let bounds = BoundsC {
        code_rows: 512,
        function_rows: 16,
        code_child_slots: 512,
        parameter_slots: 32,
        pattern_rows: 256,
        match_plan_rows: 128,
        match_arm_rows: 64,
        type_rows: 64,
        constructor_rows: 64,
        constructor_field_slots: 256,
        max_arity: 12,
        constant_nodes: 256,
        constant_field_slots: 256,
        data_nodes: p.data_nodes,
        data_field_slots: 1024,
        environment_cells: 1024,
        frames: 128,
        helper_tasks: 128,
        serialization_bytes: 1_048_576,
        max_reference_work: 100_000,
        max_helper_work: 100_000,
        step_cap: 1,
    };
    let layout = LayoutSchemaC {
        layout_version: ("rule-maxima".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds,
        capacities: LayoutCapacitiesC {
            name_rows: 128,
            name_bytes: 4096,
            occurrence_path_slots: 4096,
            value_slots: 8192,
        },
        ext_capacities: Some(ExtCapacitiesC {
            plan_case_slots: 256,
            field_path_slots: 512,
            pattern_child_slots: 256,
            ..ExtCapacitiesC::all_zero()
        }),
        boundary_capacities_present: false,
    };
    (bounds, layout)
}

fn corpus() -> Vec<Program> {
    let p = |name, body: &str, fuel, stack, data_nodes| Program {
        name,
        source: format!("language \"moriarty-sherlock-nominal/1\"; program P {{ {body} }}"),
        fuel,
        stack,
        data_nodes,
    };
    let wide_t = "type Wide { W(a: UInt, b: UInt, c: UInt, d: UInt, e: UInt, f: UInt, g: UInt, h: UInt, i: UInt, j: UInt); }";
    let choice_t = "type Choice { A(x: UInt); B(y: UInt, z: UInt); }";
    let box_t = "type Box<T> { Wrap(v: T); }";
    vec![
        p(
            "fact",
            "fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(4);",
            4096,
            64,
            512,
        ),
        p(
            "treefold",
            "fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } } entry fold(branch(branch(leaf(1), leaf(2)), leaf(3)));",
            4096,
            64,
            512,
        ),
        p(
            "listsum",
            "fn sum(xs: List): UInt { match xs { nil => 0, cons(h, t) => h + sum(t) } } entry sum(cons(1, cons(2, cons(3, nil))));",
            4096,
            64,
            512,
        ),
        p(
            "listbuild",
            "fn mk(n: UInt): List { if n == 0 { nil } else { cons(n, mk(n - 1)) } } fn len(xs: List): UInt { match xs { nil => 0, cons(h, t) => 1 + len(t) } } entry len(mk(6));",
            8192,
            64,
            512,
        ),
        p(
            "treebuild",
            "fn tb(n: UInt): Tree { if n == 0 { leaf(0) } else { branch(tb(n - 1), leaf(n)) } } fn sz(t: Tree): UInt { match t { leaf(x) => 1, branch(l, r) => sz(l) + sz(r) } } entry sz(tb(4));",
            8192,
            64,
            512,
        ),
        p(
            "args3",
            "fn f(a: UInt, b: UInt, c: UInt): UInt { a + b * c } entry f(1, 2, 3) + f(4, 5, 6);",
            4096,
            64,
            512,
        ),
        p(
            "letif",
            "entry let x = 3 in let y = x + 1 in if not(x < y) { 0 } else { x * y };",
            4096,
            64,
            512,
        ),
        p("literal", "entry 7;", 4096, 64, 512),
        p("constlist", "entry cons(1, cons(2, nil));", 4096, 64, 512),
        p(
            "eqlist",
            "entry cons(1, cons(2, nil)) == cons(1, cons(3, nil));",
            4096,
            64,
            512,
        ),
        p(
            "eqtree",
            "fn t(n: UInt): Tree { branch(leaf(n), leaf(2)) } entry t(1) == t(1);",
            4096,
            64,
            512,
        ),
        p("underflow", "entry 1 - 2;", 4096, 64, 512),
        p(
            "overflow",
            "entry 340282366920938463463374607431768211455 + 1;",
            4096,
            64,
            512,
        ),
        p(
            "stack",
            "fn down(n: UInt): UInt { if n == 0 { 0 } else { 1 + down(n - 1) } } entry down(50);",
            8000,
            8,
            512,
        ),
        p(
            "fuel",
            "fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(6);",
            60,
            64,
            512,
        ),
        p(
            "cap",
            "fn mk(n: UInt): List { if n == 0 { nil } else { cons(n, mk(n - 1)) } } entry mk(40);",
            8000,
            128,
            24,
        ),
        p(
            "wide10",
            &format!(
                "{wide_t} fn s(w: Wide): UInt {{ match w {{ Wide::W(a, b, c, d, e, f, g, h, i, j) => a + b + c + d + e + f + g + h + i + j }} }} fn mk(n: UInt): Wide {{ construct Wide::W(n, n + 1, n + 2, n + 3, n + 4, n + 5, n + 6, n + 7, n + 8, n + 9) }} entry s(mk(1));"
            ),
            8192,
            64,
            512,
        ),
        p(
            "wideconst",
            &format!("{wide_t} entry construct Wide::W(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);"),
            8192,
            64,
            512,
        ),
        p(
            "choice",
            &format!(
                "{choice_t} fn pick(c: Choice): UInt {{ match c {{ Choice::A(x) => x, Choice::B(y, z) => y + z }} }} fn mkb(n: UInt): Choice {{ construct Choice::B(n, n) }} entry pick(mkb(2)) + pick(construct Choice::A(5));"
            ),
            8192,
            64,
            512,
        ),
        p(
            "choicemiss",
            &format!(
                "{choice_t} fn mkb(n: UInt): Choice {{ construct Choice::B(n, n) }} entry match mkb(1) {{ Choice::A(x) => x }};"
            ),
            8192,
            64,
            512,
        ),
        p(
            "nested",
            &format!(
                "{box_t} type Pair {{ P(l: Box<UInt>, r: UInt); }} fn mk(n: UInt): Pair {{ construct Pair::P(construct Box<UInt>::Wrap(n), n + 1) }} entry match mk(4) {{ Pair::P(Box<UInt>::Wrap(x), y) => x + y }};"
            ),
            8192,
            64,
            512,
        ),
        p(
            "wildcard",
            &format!(
                "{choice_t} fn mkb(n: UInt): Choice {{ construct Choice::B(n, n) }} entry match mkb(3) {{ Choice::A(x) => x, wildcard => 9 }};"
            ),
            8192,
            64,
            512,
        ),
        p(
            "call0",
            "fn z(): UInt { 5 } entry z() + z();",
            4096,
            64,
            512,
        ),
        p("boolconst", "entry true;", 4096, 64, 512),
        p(
            "treeconst",
            "entry branch(leaf(1), branch(leaf(2), leaf(3)));",
            4096,
            64,
            512,
        ),
        p(
            "nullary",
            "type Opt { Nothing(); Some(v: UInt); } fn get(o: Opt): UInt { match o { Opt::Nothing() => 0, Opt::Some(v) => v } } fn mk(n: UInt): Opt { if n == 0 { construct Opt::Nothing() } else { construct Opt::Some(n) } } entry get(mk(0)) + get(mk(2));",
            8000,
            64,
            512,
        ),
        p(
            "boxconst",
            &format!("{box_t} entry construct Box<UInt>::Wrap(3);"),
            4096,
            64,
            512,
        ),
        p(
            "adtlist",
            &format!(
                "{box_t} fn sum(xs: List): UInt {{ match xs {{ nil => 0, cons(h, t) => h + sum(t) }} }} fn unwrap(b: Box<UInt>): UInt {{ match b {{ Box<UInt>::Wrap(v) => v }} }} entry unwrap(construct Box<UInt>::Wrap(sum(cons(1, cons(2, nil)))));"
            ),
            8192,
            64,
            512,
        ),
    ]
}

#[derive(Default)]
struct Run {
    name: String,
    status: String,
    steps: usize,
    outcome: String,
}

fn run_program(
    p: &Program,
    obs: &mut BTreeMap<String, Obs>,
    stat: &BTreeMap<String, Static>,
    checks: &mut Checks,
) -> Run {
    let mut run = Run {
        name: p.name.into(),
        ..Run::default()
    };
    let parsed = match crate::sherlock_nominal_source::parse_nominal_source(&p.source) {
        Ok(x) => x,
        Err(e) => {
            run.status = format!("parse refused: {e:?}");
            return run;
        }
    };
    let (bounds, layout) = bounds_for(p);
    let admitted = match admit_ext_program(
        parsed.program,
        ExtDomain::GENERIC_ABSENT,
        &bounds,
        &layout,
        None,
    ) {
        Ok((a, _)) => a,
        Err(_) => {
            run.status = "admission refused".into();
            return run;
        }
    };
    let mut img = match host_owner_engine().admit_run(
        &admitted,
        &RunIntC::Nat(p.fuel),
        &RunIntC::Nat(p.stack),
        &FinanceInput::Absent,
    ) {
        Ok(i) => i,
        Err(_) => {
            run.status = "admit_run refused".into();
            return run;
        }
    };
    let fam = HostFamilyPlans;
    let g = OwnerGuards::new_owner();
    let mut arena = Arena::new(&img);
    let mut step: i64 = 0;
    loop {
        if img.outcome != OutcomeC::Running || step > 200_000 {
            break;
        }
        let publishes = img.helper.is_none()
            && img.result.is_none()
            && (matches!(img.control, ControlC::Error { .. })
                || (matches!(img.control, ControlC::Return { .. }) && img.frames.is_empty()));
        let planned = if publishes || img.remaining_work == 0 {
            None
        } else {
            let pre = match PreImage::bind(&admitted, &img) {
                Ok(x) => x,
                Err(_) => {
                    run.status = "bind failed".into();
                    return run;
                }
            };
            match plan_logged(&pre, &fam, &g) {
                Ok(x) => Some(x),
                Err(e) => {
                    run.status = format!("plan failed at step {step}: {:?}", e.kind);
                    return run;
                }
            }
        };
        let pre_lens = [
            img.data.len(),
            img.data_fields.len(),
            img.env.len(),
            img.value_slots.len(),
        ];
        let pre_frames = img.frames.len();
        let pre_tasks = img.tasks.len();
        let ctx = format!("{} step {step}", p.name);
        let ev = match owner_step(&admitted, &mut img, &fam, &g) {
            Ok(StepResult::Terminal) => break,
            Ok(StepResult::Event { event }) => event,
            Err(e) => {
                run.status = format!("owner_step failed at step {step}: {:?}", e.kind);
                return run;
            }
        };
        checks.steps += 1;
        let rname = rule_name(&ev.rule_id);
        let o = obs.entry(rname.clone()).or_default();
        let mut cur = Obs {
            fired: 1,
            ..Obs::default()
        };
        if let Some(pl) = &planned {
            assert_eq!(pl.rule, ev.rule_id, "{ctx}: logged selection differs");
            if let Some(k) = pl.pre_helper
                && !helper_group(k).contains(&ev.rule_id)
            {
                checks
                    .exclusivity_violations
                    .push(format!("{ctx}: helper {k:?} active, fired {rname}"));
            }
        }
        if ev.disposition == StepDisposition::Applied {
            let pl = planned
                .as_ref()
                .unwrap_or_else(|| panic!("{ctx}: applied without plan"));
            checks.applied += 1;
            cur.applied = 1;
            // appends in owner_apply_validated order
            let len_pre = arena.len;
            let klen_pre: Vec<usize> = MEMS.iter().map(|k| arena.klen(*k)).collect();
            let mut app_addr = Vec::new();
            let mut app_seg4 = BTreeSet::new();
            for k in &pl.appends {
                app_seg4.insert((*k as usize, arena.klen(*k) / 8));
                app_addr.push(arena.len);
                arena.push(*k, step);
            }
            if !kind_runs_ok(&pl.appends) {
                checks
                    .a_violations
                    .push(format!("{ctx}: {rname} append kinds {:?}", pl.appends));
            }
            checks.max_cells_per_step = checks.max_cells_per_step.max(pl.appends.len());
            let post_lens = [
                img.data.len(),
                img.data_fields.len(),
                img.env.len(),
                img.value_slots.len(),
            ];
            for k in MEMS {
                let n = pl.appends.iter().filter(|x| **x == k).count();
                cur.app[k as usize] = n;
                assert_eq!(
                    post_lens[k as usize],
                    pre_lens[k as usize] + n,
                    "{ctx}: {rname} {k:?} length vs plan"
                );
                assert_eq!(arena.klen(k), post_lens[k as usize], "{ctx}: arena {k:?}");
            }
            cur.app_total = pl.appends.len();
            (cur.frame_push, cur.frame_pop, cur.frame_rewrite) = pl.frame;
            (cur.task_push, cur.task_pop, cur.task_rewrite) = pl.task;
            assert_eq!(
                img.frames.len() as i64 - pre_frames as i64,
                pl.frame.0 as i64 - pl.frame.1 as i64,
                "{ctx}: frames"
            );
            assert_eq!(
                img.tasks.len() as i64 - pre_tasks as i64,
                pl.task.0 as i64 - pl.task.1 as i64,
                "{ctx}: tasks"
            );
            // reads
            let all: BTreeSet<Rd> = pl
                .reads
                .iter()
                .chain(pl.sel_reads.iter())
                .copied()
                .collect();
            let sel_only: BTreeSet<Rd> = pl.sel_reads.iter().copied().collect();
            let mut seg1 = BTreeSet::new();
            let mut old1 = BTreeSet::new();
            let mut seg4 = BTreeSet::new();
            let mut old4 = BTreeSet::new();
            let mut rom_tables = BTreeSet::new();
            let mut rom_blocks = BTreeSet::new();
            for r in &all {
                match r {
                    Rd::M(k, i) => {
                        cur.rd[*k as usize] += 1;
                        cur.rd_heap += 1;
                        let Some(a) = arena.addr[*k as usize].get(*i) else {
                            panic!("{ctx}: read {k:?}[{i}] beyond arena");
                        };
                        seg1.insert(a / 8);
                        if a / 8 < len_pre / 8 {
                            old1.insert(a / 8);
                            cur.old_seen = true;
                        }
                        seg4.insert((*k as usize, i / 8));
                        if i / 8 < klen_pre[*k as usize] / 8 {
                            old4.insert((*k as usize, i / 8));
                        }
                        let born = arena.born[*k as usize][*i];
                        cur.max_age = cur.max_age.max(step - born);
                    }
                    Rd::R(t, i) => {
                        cur.rd_rom += 1;
                        rom_tables.insert(rom_table(*t));
                        rom_blocks.insert((rom_table(*t), i / 8));
                    }
                }
            }
            cur.rd_sel_heap = sel_only.iter().filter(|r| matches!(r, Rd::M(..))).count();
            cur.rom_tables = rom_tables.len();
            cur.rom_blocks = rom_blocks.len();
            let app_seg1: BTreeSet<usize> = app_addr.iter().map(|a| a / 8).collect();
            cur.seg_read_1 = seg1.len();
            cur.seg_old_1 = old1.len();
            cur.seg_app_1 = app_seg1.len();
            cur.seg_all_1 = seg1.union(&app_seg1).count();
            cur.seg_read_4 = seg4.len();
            cur.seg_old_4 = old4.len();
            cur.seg_app_4 = app_seg4.len();
            cur.seg_all_4 = seg4.union(&app_seg4).count();
            // dynamic within static
            if let Some(s) = stat.get(&rname) {
                let sa = s.app();
                for k in MEMS {
                    if cur.app[k as usize] > sa[k as usize] {
                        checks.static_violations.push(format!(
                            "{ctx}: {rname} {k:?} appends {} > static {}",
                            cur.app[k as usize], sa[k as usize]
                        ));
                    }
                }
                let flags = [
                    (cur.frame_push, s.frame_push, "frame push"),
                    (cur.frame_pop, s.frame_pop, "frame pop"),
                    (cur.frame_rewrite, s.frame_rewrite, "frame rewrite"),
                    (cur.task_push, s.task_push, "task push"),
                    (cur.task_pop, s.task_pop, "task pop"),
                    (cur.task_rewrite, s.task_rewrite, "task rewrite"),
                ];
                for (d, st, what) in flags {
                    if d > 0 && !st {
                        checks
                            .static_violations
                            .push(format!("{ctx}: {rname} {what} not in static scan"));
                    }
                }
                if s.loops.is_empty() && cur.rd_heap > s.heap_read_bound() {
                    checks.static_violations.push(format!(
                        "{ctx}: {rname} heap reads {} > static bound {}",
                        cur.rd_heap,
                        s.heap_read_bound()
                    ));
                }
            }
            // entry captures
            match (&ev.rule_id, &img.helper) {
                (RuleIdC::BHelperAppendEntry, Some(HelperControlC::Append { dest_start, .. }))
                    if *dest_start != img.value_slots.len() =>
                {
                    checks.entry_violations.push(format!(
                        "{ctx}: AppendEntry dest_start {dest_start} != slots {}",
                        img.value_slots.len()
                    ));
                }
                (
                    RuleIdC::XAdtHelperBuildEntry,
                    Some(HelperControlC::Build { field_start, .. }),
                ) if *field_start != img.data_fields.len() => {
                    checks.entry_violations.push(format!(
                        "{ctx}: BuildEntry field_start {field_start} != fields {}",
                        img.data_fields.len()
                    ));
                }
                _ => {}
            }
            // slice contiguity on the post image
            for (what, s) in field_slices(&img) {
                check_contiguous(&arena, Mem::Field, what, s, &ctx, checks);
            }
            for (what, s) in slot_slices(&img) {
                check_contiguous(&arena, Mem::Slot, what, s, &ctx, checks);
            }
        }
        o.merge(&cur);
        step += 1;
    }
    run.steps = step as usize;
    run.outcome = format!("{:?}", img.outcome);
    run.status = "ok".into();
    run
}

// ---------------------------------------------------------------- output
fn apply_order() -> Vec<String> {
    let body = scan::strip(&fn_body_raw("owner_apply_validated"));
    let mut v: Vec<(usize, String)> = [
        "img.data_fields.push",
        "img.data.push",
        "img.env.push",
        "img.value_slots.push",
    ]
    .iter()
    .flat_map(|pat| {
        body.match_indices(pat)
            .map(|(i, _)| (i, (*pat).to_string()))
    })
    .collect();
    v.sort();
    v.into_iter().map(|(_, s)| s).collect()
}

fn fn_body_raw(name: &str) -> String {
    scan::functions("owner_runtime", &scan::strip(SRC_OWNER))
        .into_iter()
        .find(|f| f.name == name)
        .map(|f| f.body)
        .unwrap_or_else(|| panic!("fn {name}"))
}

fn b(x: bool) -> &'static str {
    if x { "1" } else { "0" }
}

#[test]
fn rule_maxima_table_and_one_arena_noninterleaving() {
    let uni = universe();
    let n108 = uni.rules.iter().filter(|(_, x)| *x).count();
    assert_eq!(n108, 108, "the 108 supported rules");
    // Every RuleIdC variant outside the step-rule universe is one of the
    // rules is_disabled / is_extension refuse, or a startup rule (no step).
    let expected_unreached: BTreeSet<String> = [
        "BHelperEnvPutCell",
        "BHelperEnvPutEmpty",
        "BHelperEnvPutFailure",
        "BHelperAppendFailure",
        "BHelperEqualFailure",
        "BHelperMatchEmpty",
        "BBookkeepingTaskEnqueue",
        "BHelperValidateDeclaration",
        "BHelperMatchDiscard",
        "BBookkeepingResumeStore",
        "BBookkeepingResumeRestore",
        "KAdminStartupValid",
        "KAdminStartupInvalid",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect();
    let got_unreached: BTreeSet<String> = uni.unreached.iter().cloned().collect();
    assert_eq!(
        got_unreached, expected_unreached,
        "rules outside the universe"
    );
    for (r, _) in &uni.rules {
        assert!(!is_disabled(r) && !is_extension(r), "{r:?} is not enabled");
    }
    let all_names: BTreeSet<String> = enum_names("RuleIdC").into_iter().collect();
    let idx = scan::Index::new(&modules(), all_names, &fn_body_raw("eval_guard"));
    let mut stat = BTreeMap::new();
    for (r, _) in &uni.rules {
        stat.insert(rule_name(r), analyse_static(&idx, *r));
    }

    // dynamic corpus
    let mut obs: BTreeMap<String, Obs> = BTreeMap::new();
    let mut checks = Checks::default();
    let mut runs = Vec::new();
    for p in corpus() {
        runs.push(run_program(&p, &mut obs, &stat, &mut checks));
    }

    // over-all-rules premises of the non-interleaving argument (static)
    let mut premises: Vec<(String, bool, String)> = Vec::new();
    let in_group = |k: HelperKind, r: &RuleIdC| helper_group(k).contains(r);
    let mut cex = Vec::new();
    for (r, _) in &uni.rules {
        let s = &stat[&rule_name(r)];
        let a = s.app();
        let kinds: Vec<&str> = MEMS
            .iter()
            .filter(|k| a[**k as usize] > 0)
            .map(|k| match k {
                Mem::Data => "DATA",
                Mem::Field => "FIELDS",
                Mem::Env => "ENV",
                Mem::Slot => "SLOTS",
            })
            .collect();
        let append_entry_or_group =
            in_group(HelperKind::Append, r) || *r == RuleIdC::BHelperAppendEntry;
        let build_entry_or_group =
            in_group(HelperKind::Build, r) || *r == RuleIdC::XAdtHelperBuildEntry;
        if append_entry_or_group && kinds.iter().any(|k| *k != "SLOTS") {
            cex.push(format!("{r:?}: Append-helper rule appends {kinds:?}"));
        }
        if build_entry_or_group
            && (a[Mem::Env as usize] > 0 || a[Mem::Slot as usize] > 0 || s.alloc_data)
        {
            cex.push(format!("{r:?}: Build-helper rule appends {kinds:?}"));
        }
        if s.alloc_value_slot && !in_group(HelperKind::Append, r) {
            cex.push(format!(
                "{r:?}: appends a value slot outside the Append helper"
            ));
        }
        if (s.alloc_data_field || s.alloc_constructor) && *r != RuleIdC::XAdtHelperBuildField {
            cex.push(format!(
                "{r:?}: Build field/constructor write outside XAdtHelperBuildField"
            ));
        }
        if s.alloc_data && (s.alloc_data_field || s.alloc_constructor) {
            cex.push(format!(
                "{r:?}: alloc_data together with a Build write (validator refuses)"
            ));
        }
        if (*r == RuleIdC::BHelperAppendEntry || *r == RuleIdC::XAdtHelperBuildEntry)
            && a.iter().sum::<usize>() > 0
        {
            cex.push(format!("{r:?}: entry rule appends"));
        }
    }
    let order = apply_order();
    let order_ok = order
        == vec![
            "img.data_fields.push",
            "img.data.push",
            "img.data_fields.push",
            "img.data.push",
            "img.env.push",
            "img.value_slots.push",
        ];
    premises.push((
        "S1 apply order (owner_apply_validated text): alloc_data fields, alloc_data node, alloc_data_field, alloc_constructor, env, slot".into(),
        order_ok,
        format!("{order:?}"),
    ));
    premises.push((
        "S2 every Append-helper rule (entry and group) appends SLOTS only; every Build-helper rule appends FIELDS and the closing constructor only; SLOTS only in the Append group; Build writes only in XAdtHelperBuildField; no rule mixes alloc_data with a Build write; entries append nothing".into(),
        cex.is_empty(),
        format!("{cex:?}"),
    ));
    let ae = &stat["BHelperAppendEntry"];
    let be = &stat["XAdtHelperBuildEntry"];
    premises.push((
        "S3 entries capture the active length: BHelperAppendEntry reads active().value_slots, XAdtHelperBuildEntry reads active().data_fields (text)".into(),
        ae.mentions_active_slots && be.mentions_active_fields,
        format!(
            "append_entry={} build_entry={}",
            ae.mentions_active_slots, be.mentions_active_fields
        ),
    ));
    let sel_src = scan::strip(&fn_body_raw("owner_select"));
    let excl = sel_src.contains("if let Some(h) = pre.helper()")
        && sel_src.contains("helper_group(helper_kind_of(h))");
    premises.push((
        "S4 owner_select: an active helper selects only from helper_group(kind) (text), and dynamically on every corpus step".into(),
        excl && checks.exclusivity_violations.is_empty(),
        format!("{:?}", checks.exclusivity_violations),
    ));

    // tables
    let mut csv = String::new();
    let cols = [
        "rule",
        "in_108",
        "group",
        "family",
        "fired_steps",
        "applied_steps",
        "st_app_data",
        "st_app_fields",
        "st_app_env",
        "st_app_slots",
        "st_app_total",
        "st_frame_push",
        "st_frame_pop",
        "st_frame_rewrite",
        "st_task_push",
        "st_task_pop",
        "st_task_rewrite",
        "st_sites_value",
        "st_sites_field_at",
        "st_sites_env",
        "st_sites_slot",
        "st_sites_rom",
        "st_sel_sites_heap",
        "st_heap_read_bound",
        "st_loops",
        "dy_app_data",
        "dy_app_fields",
        "dy_app_env",
        "dy_app_slots",
        "dy_app_total",
        "dy_frame_push",
        "dy_frame_pop",
        "dy_frame_rewrite",
        "dy_task_push",
        "dy_task_pop",
        "dy_task_rewrite",
        "dy_rd_data",
        "dy_rd_fields",
        "dy_rd_env",
        "dy_rd_slots",
        "dy_rd_heap",
        "dy_rd_sel_heap",
        "dy_rd_rom",
        "dy_rom_tables",
        "dy_rom_blocks8",
        "dy_seg_read_1a",
        "dy_seg_old_1a",
        "dy_seg_app_1a",
        "dy_seg_all_1a",
        "dy_seg_read_4a",
        "dy_seg_old_4a",
        "dy_seg_app_4a",
        "dy_seg_all_4a",
        "dy_max_read_age_steps",
        "dy_old_read_seen",
        "hash_calls_no_cold",
        "hash_calls_per_cold_read",
    ];
    csv.push_str(&cols.join(","));
    csv.push('\n');
    let mut md = String::new();
    md.push_str("| rule | 108 | fired | app D/F/E/S (st) | app tot st / dy | frame st | task st | heap read bound st | heap reads dy | old segs 1a dy | segs all 1a dy | ROM reads dy | hash calls st |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut max_st = [0usize; 8];
    let mut max_dy = Obs::default();
    let mut max_st_108 = [0usize; 8];
    for (r, in108) in &uni.rules {
        let n = rule_name(r);
        let s = &stat[&n];
        let o = obs.get(&n).cloned().unwrap_or_default();
        let a = s.app();
        let tot: usize = a.iter().sum();
        let fr = [s.frame_push, s.frame_pop, s.frame_rewrite];
        let tk = [s.task_push, s.task_pop, s.task_rewrite];
        let stack_hash =
            usize::from(s.frame_push || s.frame_pop) + usize::from(s.task_push || s.task_pop);
        let hash = tot + stack_hash;
        let row_st = [
            a[0],
            a[1],
            a[2],
            a[3],
            tot,
            usize::from(fr.iter().any(|x| *x)),
            usize::from(tk.iter().any(|x| *x)),
            s.heap_read_bound(),
        ];
        for k in 0..8 {
            max_st[k] = max_st[k].max(row_st[k]);
            if *in108 {
                max_st_108[k] = max_st_108[k].max(row_st[k]);
            }
        }
        max_dy.merge(&o);
        let mut line: Vec<String> = vec![
            n.clone(),
            b(*in108).into(),
            group_of(*r),
            s.family.clone(),
            o.fired.to_string(),
            o.applied.to_string(),
        ];
        line.extend(a.iter().map(|x| x.to_string()));
        line.push(tot.to_string());
        line.extend(fr.iter().chain(tk.iter()).map(|x| b(*x).to_string()));
        line.extend(
            [s.s_value, s.s_field_at, s.s_env, s.s_slot, s.s_rom]
                .iter()
                .map(|x| x.to_string()),
        );
        line.push((s.s_sel_value + 2 * s.s_sel_field_at + s.s_sel_env + s.s_sel_slot).to_string());
        line.push(s.heap_read_bound().to_string());
        line.push(s.loops.join(" "));
        line.extend(o.app.iter().map(|x| x.to_string()));
        line.push(o.app_total.to_string());
        line.extend(
            [
                o.frame_push,
                o.frame_pop,
                o.frame_rewrite,
                o.task_push,
                o.task_pop,
                o.task_rewrite,
            ]
            .iter()
            .map(|x| x.to_string()),
        );
        line.extend(o.rd.iter().map(|x| x.to_string()));
        line.extend(
            [
                o.rd_heap,
                o.rd_sel_heap,
                o.rd_rom,
                o.rom_tables,
                o.rom_blocks,
                o.seg_read_1,
                o.seg_old_1,
                o.seg_app_1,
                o.seg_all_1,
                o.seg_read_4,
                o.seg_old_4,
                o.seg_app_4,
                o.seg_all_4,
            ]
            .iter()
            .map(|x| x.to_string()),
        );
        line.push(o.max_age.to_string());
        line.push(b(o.old_seen).into());
        line.push(hash.to_string());
        line.push("1+TOP+3".into());
        csv.push_str(&line.join(","));
        csv.push('\n');
        let dash = |x: usize| {
            if o.applied == 0 {
                "-".to_string()
            } else {
                x.to_string()
            }
        };
        let _ = writeln!(
            md,
            "| {n} | {} | {} | {}/{}/{}/{} | {tot} / {} | {}{}{} | {}{}{} | {}{} | {} | {} | {} | {} | {hash} |",
            if *in108 { "y" } else { "n" },
            o.fired,
            a[0],
            a[1],
            a[2],
            a[3],
            dash(o.app_total),
            if s.frame_push { "P" } else { "" },
            if s.frame_pop { "p" } else { "" },
            if s.frame_rewrite { "r" } else { "" },
            if s.task_push { "P" } else { "" },
            if s.task_pop { "p" } else { "" },
            if s.task_rewrite { "r" } else { "" },
            s.heap_read_bound(),
            if s.loops.is_empty() {
                String::new()
            } else {
                format!(" (loop: {})", s.loops.join(" "))
            },
            dash(o.rd_heap),
            dash(o.seg_old_1),
            dash(o.seg_all_1),
            dash(o.rd_rom),
        );
    }
    let fired_108 = uni
        .rules
        .iter()
        .filter(|(r, x)| *x && obs.get(&rule_name(r)).is_some_and(|o| o.fired > 0))
        .count();
    let fired_all = uni
        .rules
        .iter()
        .filter(|(r, _)| obs.get(&rule_name(r)).is_some_and(|o| o.fired > 0))
        .count();
    let summary = serde_json::json!({
        "format": "moriarty-sherlock-rule-maxima/1",
        "rules_total": uni.rules.len(),
        "rules_108": n108,
        "rules_unreached_enum_variants": uni.unreached,
        "fired_rules_108": fired_108,
        "fired_rules_all": fired_all,
        "steps": checks.steps,
        "applied_steps": checks.applied,
        "slices_checked": checks.slices_checked,
        "max_cells_per_step_dynamic": checks.max_cells_per_step,
        "static_max_all": {
            "app_data": max_st[0], "app_fields": max_st[1], "app_env": max_st[2], "app_slots": max_st[3],
            "app_total": max_st[4], "frame_op": max_st[5], "task_op": max_st[6], "heap_read_bound": max_st[7],
        },
        "static_max_108": {
            "app_data": max_st_108[0], "app_fields": max_st_108[1], "app_env": max_st_108[2], "app_slots": max_st_108[3],
            "app_total": max_st_108[4], "frame_op": max_st_108[5], "task_op": max_st_108[6], "heap_read_bound": max_st_108[7],
        },
        "dynamic_max_all": {
            "app": max_dy.app, "app_total": max_dy.app_total,
            "frame_push": max_dy.frame_push, "frame_pop": max_dy.frame_pop, "frame_rewrite": max_dy.frame_rewrite,
            "task_push": max_dy.task_push, "task_pop": max_dy.task_pop, "task_rewrite": max_dy.task_rewrite,
            "rd": max_dy.rd, "rd_heap": max_dy.rd_heap, "rd_sel_heap": max_dy.rd_sel_heap, "rd_rom": max_dy.rd_rom,
            "rom_tables": max_dy.rom_tables, "rom_blocks8": max_dy.rom_blocks,
            "seg_read_1a": max_dy.seg_read_1, "seg_old_1a": max_dy.seg_old_1, "seg_app_1a": max_dy.seg_app_1, "seg_all_1a": max_dy.seg_all_1,
            "seg_read_4a": max_dy.seg_read_4, "seg_old_4a": max_dy.seg_old_4, "seg_app_4a": max_dy.seg_app_4, "seg_all_4a": max_dy.seg_all_4,
            "max_read_age_steps": max_dy.max_age, "old_read_seen": max_dy.old_seen,
        },
        "premises": premises.iter().map(|(k, ok, d)| serde_json::json!({"premise": k, "holds": ok, "detail": d})).collect::<Vec<_>>(),
        "dynamic_checks": {
            "A_step_local_violations": checks.a_violations,
            "B_slice_contiguity_violations": checks.b_violations,
            "entry_capture_violations": checks.entry_violations,
            "helper_exclusivity_violations": checks.exclusivity_violations,
            "dynamic_above_static": checks.static_violations,
        },
        "runs": runs.iter().map(|r| serde_json::json!({"program": r.name, "status": r.status, "steps": r.steps, "outcome": r.outcome})).collect::<Vec<_>>(),
        "reached": stat.iter().map(|(k, s)| (k.clone(), serde_json::json!(s.reached))).collect::<BTreeMap<_, _>>(),
    });
    if let Ok(dir) = std::env::var("MAXIMA_OUT") {
        let d = std::path::Path::new(&dir);
        std::fs::create_dir_all(d).unwrap_or_else(|e| panic!("{e}"));
        std::fs::write(d.join("rule_maxima.csv"), &csv).unwrap_or_else(|e| panic!("{e}"));
        std::fs::write(d.join("rule_maxima.md"), &md).unwrap_or_else(|e| panic!("{e}"));
        std::fs::write(
            d.join("noninterleave.json"),
            serde_json::to_string_pretty(&summary).unwrap_or_default(),
        )
        .unwrap_or_else(|e| panic!("{e}"));
    }
    println!(
        "RULE-MAXIMA {}",
        serde_json::to_string(&summary["static_max_all"]).unwrap_or_default()
    );
    println!(
        "RULE-MAXIMA-DYN {}",
        serde_json::to_string(&summary["dynamic_max_all"]).unwrap_or_default()
    );
    for r in &runs {
        println!(
            "RULE-MAXIMA-RUN {} {} steps={} {}",
            r.name, r.status, r.steps, r.outcome
        );
    }
    for (k, ok, d) in &premises {
        println!("RULE-MAXIMA-PREMISE {ok} {k} {d}");
    }
    assert!(checks.a_violations.is_empty(), "{:?}", checks.a_violations);
    assert!(checks.b_violations.is_empty(), "{:?}", checks.b_violations);
    assert!(
        checks.entry_violations.is_empty(),
        "{:?}",
        checks.entry_violations
    );
    assert!(
        checks.static_violations.is_empty(),
        "{:?}",
        checks.static_violations
    );
    assert!(premises.iter().all(|(_, ok, _)| *ok), "{premises:?}");
    assert!(checks.applied > 0);
}
