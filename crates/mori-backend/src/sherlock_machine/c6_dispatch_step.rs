//! C6 dispatch owner: in-circuit rule selection over the padded
//! `sherlock-c6-phys/1` pre carriers.
//!
//! For one constant `RuleIdC`, the emitted block constrains (under
//! `enable == 1`) that Core `owner_step` selects exactly that rule on the
//! pre-state. The derivation mirrors, in order:
//!
//! * P0: `outcome == Running`;
//! * P1: publication with helper and result absent (`Error` control publishes
//!   `KAdminPublishError`, `Return` at depth 0 publishes `KAdminPublishValue`);
//! * P2: `remaining_work == 0` publishes `KAdminPublishExhausted`;
//! * `owner_select`: an active helper selects the first rule of
//!   `helper_group(kind)` whose `select_when` holds; a present result selects
//!   the first result-reading rule of `legacy_group(site)`; otherwise
//!   `owner_helper_plan(site)` selects that helper's Entry rule; otherwise the
//!   first non-result-reading `select_when` of `legacy_group(site)`.
//!
//! `site` is derived from the actual control tag, the actual Eval expression
//! row (full PROG parser walk) and the actual top frame (multiplexed by the
//! frame count). Data-dependent premises read the actual referenced rows
//! through `C6CheckedValueReadEmitter` (Validate Marker kind, Equal
//! `shallow_differ`). `FindRow`/`FindEmpty` use the PROG function count
//! captured by the same parser walk.
//!
//! Because each rule's path includes the negation of every earlier rule in
//! its group and the P0..P2/group precedence, the paths of distinct supported
//! rules are mutually exclusive on any pre-state: at most one constant-rule
//! block is satisfiable, and it is the rule Core selects.
//!
//! Supported rules: every enabled pure rule except the `XAdtHelperPlan*` and
//! `XAdtHelperBuild*` helper groups ([`dispatch_supported`]). Disabled,
//! extension and financial rules are refused.
//!
//! Retained premises (not discharged here): canonicality of the pre IMAGE and
//! HELPER sections (frame tags, padding, helper/task tags) is enforced by the
//! paired step fragment over the same lanes; PROG canonicality is enforced by
//! the parser walk below; DATA canonicality only where a value read is gated
//! on. Recursion/native composition with the step fragment is a caller
//! obligation.
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    FRAME_LANES, IMAGE_SECTION_HEADER_LANES, L_CONTROL, L_CONTROL_ORIGIN, L_FRAME_COUNT, L_OUTCOME,
    L_REMAINING_WORK, L_STACK_LIMIT,
};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, TASK_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextDispatchCaptureInputs,
};

const LANE_BITS: u32 = 128;

#[derive(Debug, Error)]
pub enum C6DispatchError {
    #[error("invalid scope identifier")]
    InvalidScope,
    #[error("rule {0:?} is not supported by the C6 dispatch owner")]
    Unsupported(RuleIdC),
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch {
        section: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("composed emitter error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6DispatchError {
    C6DispatchError::Backend(e.to_string())
}

#[derive(Clone, Debug)]
pub struct C6DispatchInputs {
    pub enable: Operand,
    pub caps: C6StateCaps,
    pub pre_prog: Vec<Operand>,
    pub pre_image: Vec<Operand>,
    pub pre_data: Vec<Operand>,
    pub pre_helper: Vec<Operand>,
}

#[derive(Clone, Debug)]
pub struct C6DispatchBlock {
    pub instructions: Vec<Instruction>,
}

/// Helper kinds in their C6 record tags.
const H_SCALAR: u128 = 1;
const H_ENVGET: u128 = 2;
const H_ENVPUT: u128 = 3;
const H_FIND: u128 = 4;
const H_BIND: u128 = 5;
const H_APPEND: u128 = 6;
const H_VALIDATE: u128 = 7;
const H_EQUAL: u128 = 8;
const H_MATCH: u128 = 9;

/// Site tags (C6 helper-record encoding, also used here as the site index).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Site {
    EvalLiteral = 0,
    EvalVar = 1,
    EvalLet = 2,
    EvalBinary = 3,
    EvalUnary = 4,
    EvalIf = 5,
    EvalMatch = 6,
    EvalCall = 7,
    Push = 8,
    RetLet = 9,
    RetLeft = 10,
    RetRight = 11,
    RetUnary = 12,
    RetIf = 13,
    RetMatch = 14,
    RetArgs = 15,
    RetCall = 16,
    RetScope = 17,
    Invoke = 18,
    Enter = 19,
    Bound = 20,
    Cases = 21,
    CaseResult = 22,
    Prim = 23,
    EvalConstruct = 24,
    RetCtorFields = 25,
    Materialize = 26,
}
const SITES: usize = 27;

/// The rules `owner_select` scans, in Core order (`helper_group`).
fn helper_group(kind: u128) -> &'static [RuleIdC] {
    use RuleIdC::*;
    match kind {
        H_SCALAR => &[
            BHelperScalarConstruct,
            BHelperScalarArithmetic,
            BHelperScalarTag,
            BHelperScalarFailure,
            BHelperScalarExit,
        ],
        H_ENVGET => &[
            BHelperEnvGetExit,
            BHelperEnvGetFailure,
            BHelperEnvGetCell,
            BHelperEnvGetEmpty,
        ],
        H_ENVPUT => &[BHelperEnvPutInsert, BHelperEnvPutExit],
        H_FIND => &[
            BHelperFindExit,
            BHelperFindFailure,
            BHelperFindRow,
            BHelperFindEmpty,
        ],
        H_BIND => &[
            BHelperBindExit,
            BHelperBindFailure,
            BHelperBindPair,
            BHelperBindEmpty,
            BHelperBindArity,
            BHelperBindType,
            BHelperBindDuplicate,
            BHelperBindInsert,
            BHelperBindPropagate,
        ],
        H_APPEND => &[
            BHelperAppendCopy,
            BHelperAppendCell,
            BHelperAppendEmpty,
            BHelperAppendTail,
            BHelperAppendExit,
        ],
        H_VALIDATE => &[
            BBookkeepingTaskRelease,
            BHelperValidateFailure,
            BHelperValidateExit,
            BHelperValidateEmpty,
            BHelperValidateInvalid,
            BHelperValidateOccurrence,
            BHelperValidateField,
        ],
        H_EQUAL => &[
            BBookkeepingTaskRelease,
            BHelperEqualExit,
            BHelperEqualEmpty,
            BHelperEqualDifferent,
            BHelperEqualPair,
            BHelperEqualField,
        ],
        H_MATCH => &[
            BHelperMatchExit,
            BHelperMatchFailure,
            BHelperMatchArm,
            BHelperMatchOccurrence,
            BHelperMatchDistinct,
            BHelperMatchField,
            BHelperMatchBinding,
            BHelperMatchSelected,
            BHelperMatchNext,
        ],
        _ => &[],
    }
}

/// `legacy_group(site)` in Core order.
fn legacy_group(site: Site) -> &'static [RuleIdC] {
    use RuleIdC::*;
    use Site::*;
    match site {
        EvalLiteral => &[KMachineEvalLiteralValid, KMachineEvalLiteralInvalid],
        EvalVar => &[KMachineEvalVariablePresent, KMachineEvalVariableAbsent],
        EvalLet => &[KMachineEvalLet],
        EvalBinary => &[KMachineEvalBinary],
        EvalUnary => &[KMachineEvalUnary],
        EvalIf => &[KMachineEvalIf],
        EvalMatch => &[KMachineEvalMatch],
        EvalCall => &[KMachineEvalCallEmpty, KMachineEvalCallArguments],
        EvalConstruct => &[XAdtEvalConstructNullary, XAdtEvalConstructFields],
        Push => &[KMachinePushAccept, KMachinePushRefuse],
        RetLet => &[KMachineReturnLet],
        RetLeft => &[KMachineReturnLeft],
        RetRight => &[KMachineReturnRight],
        RetUnary => &[KMachineReturnUnary],
        RetIf => &[
            KMachineReturnIfTrue,
            KMachineReturnIfFalse,
            KMachineReturnIfInvalid,
        ],
        RetMatch => &[KMachineReturnMatch],
        RetArgs => &[KMachineReturnArgumentsNext, KMachineReturnArgumentsFinal],
        RetCall => &[KMachineReturnCallValid, KMachineReturnCallInvalid],
        RetScope => &[KMachineReturnScope],
        RetCtorFields => &[
            XAdtReturnCtorFieldsTyped,
            XAdtReturnCtorFieldsMistyped,
            XAdtReturnCtorFieldsNext,
            XAdtReturnCtorFieldsFinal,
        ],
        Materialize => &[XAdtMaterializeDone],
        Invoke => &[KMachineInvokeFind],
        Enter => &[KMachineEnterMissing, KMachineEnterBind],
        Bound => &[KMachineBoundInvalid, KMachineBoundValid],
        Cases => &[
            KMachineCasesEmpty,
            KMachineCasesTry,
            XAdtCasesPlanHit,
            XAdtCasesPlanMiss,
        ],
        CaseResult => &[KMachineCasesMiss, KMachineCasesHit],
        Prim => &[KMachinePrimitiveValue, KMachinePrimitiveError],
    }
}

const ALL_SITES: [Site; SITES] = {
    use Site::*;
    [
        EvalLiteral,
        EvalVar,
        EvalLet,
        EvalBinary,
        EvalUnary,
        EvalIf,
        EvalMatch,
        EvalCall,
        Push,
        RetLet,
        RetLeft,
        RetRight,
        RetUnary,
        RetIf,
        RetMatch,
        RetArgs,
        RetCall,
        RetScope,
        Invoke,
        Enter,
        Bound,
        Cases,
        CaseResult,
        Prim,
        EvalConstruct,
        RetCtorFields,
        Materialize,
    ]
};

/// Core `reads_result_root` (AST Root{r} membership, fixed per row).
fn reads_result(rule: RuleIdC) -> bool {
    use RuleIdC::*;
    matches!(
        rule,
        KMachineEvalLiteralValid
            | KMachineEvalLiteralInvalid
            | KMachineEvalVariablePresent
            | KMachineEvalVariableAbsent
            | KMachineReturnLet
            | KMachineReturnRight
            | KMachineReturnUnary
            | KMachineReturnIfTrue
            | KMachineReturnIfFalse
            | KMachineReturnIfInvalid
            | KMachineReturnArgumentsNext
            | KMachineReturnArgumentsFinal
            | KMachineReturnCallValid
            | KMachineReturnCallInvalid
            | KMachineInvokeFind
            | KMachineEnterBind
            | KMachineCasesTry
            | XAdtReturnCtorFieldsTyped
            | XAdtReturnCtorFieldsMistyped
            | XAdtReturnCtorFieldsNext
            | XAdtReturnCtorFieldsFinal
            | XAdtMaterializeDone
    )
}

fn entry_kind(rule: RuleIdC) -> Option<u128> {
    use RuleIdC::*;
    Some(match rule {
        BHelperScalarEntry => H_SCALAR,
        BHelperEnvGetEntry => H_ENVGET,
        BHelperEnvPutEntry => H_ENVPUT,
        BHelperFindEntry => H_FIND,
        BHelperBindEntry => H_BIND,
        BHelperAppendEntry => H_APPEND,
        BHelperValidateEntry => H_VALIDATE,
        BHelperEqualEntry => H_EQUAL,
        BHelperMatchEntry => H_MATCH,
        _ => return None,
    })
}

fn helper_kind_of(rule: RuleIdC) -> Option<u128> {
    (1..=9).find(|k| helper_group(*k).contains(&rule))
}

fn legacy_site_of(rule: RuleIdC) -> Option<Site> {
    ALL_SITES
        .iter()
        .copied()
        .find(|s| legacy_group(*s).contains(&rule))
}

fn is_publication(rule: RuleIdC) -> bool {
    matches!(
        rule,
        RuleIdC::KAdminPublishValue | RuleIdC::KAdminPublishError | RuleIdC::KAdminPublishExhausted
    )
}

/// Every rule [`dispatch_supported`] accepts, in group order.
pub fn supported_rules() -> Vec<RuleIdC> {
    let mut out = vec![
        RuleIdC::KAdminPublishValue,
        RuleIdC::KAdminPublishError,
        RuleIdC::KAdminPublishExhausted,
    ];
    for kind in 1..=9 {
        for r in helper_group(kind) {
            if !out.contains(r) {
                out.push(*r);
            }
        }
    }
    for site in ALL_SITES {
        for r in legacy_group(site) {
            if !out.contains(r) {
                out.push(*r);
            }
        }
    }
    use RuleIdC::*;
    for r in [
        BHelperScalarEntry,
        BHelperEnvGetEntry,
        BHelperEnvPutEntry,
        BHelperFindEntry,
        BHelperBindEntry,
        BHelperAppendEntry,
        BHelperValidateEntry,
        BHelperEqualEntry,
        BHelperMatchEntry,
    ] {
        out.push(r);
    }
    out
}

/// Whether [`C6DispatchEmitter::emit`] accepts this rule.
pub fn dispatch_supported(rule: RuleIdC) -> bool {
    if matches!(rule, RuleIdC::BBookkeepingTaskRelease) {
        return true;
    }
    is_publication(rule)
        || entry_kind(rule).is_some()
        || helper_kind_of(rule).is_some()
        || legacy_site_of(rule).is_some()
}

struct Em {
    scope: String,
    next: usize,
    ins: Vec<Instruction>,
}

impl Em {
    fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
    }
    fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.c6dispatch.temp.{}", self.scope, self.next));
        self.next += 1;
        id
    }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::TestEq {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn eqc(&mut self, a: &Operand, c: u128) -> Operand {
        self.teq(a.clone(), Self::imm(c))
    }
    fn lt(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::LessThan {
            a,
            b,
            bits: LANE_BITS,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Mul {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Not {
            a,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn and(&mut self, a: Operand, b: Operand) -> Operand {
        self.mul(a, b)
    }
    fn all(&mut self, xs: &[Operand]) -> Operand {
        let mut acc = Self::imm(1);
        for x in xs {
            acc = self.and(acc, x.clone());
        }
        acc
    }
    /// Boolean OR of mutually unconstrained booleans (via De Morgan).
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a);
        let nb = self.not(b);
        let both = self.mul(na, nb);
        self.not(both)
    }
    fn any(&mut self, xs: &[Operand]) -> Operand {
        let mut acc = Self::imm(0);
        for x in xs {
            acc = self.or(acc, x.clone());
        }
        acc
    }
    fn in_set(&mut self, a: &Operand, cs: &[u128]) -> Operand {
        let bits: Vec<Operand> = cs.iter().map(|c| self.eqc(a, *c)).collect();
        self.any(&bits)
    }
    fn nonzero(&mut self, a: &Operand) -> Operand {
        let z = self.eqc(a, 0);
        self.not(z)
    }
    fn cbool(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainToBoolean { val: v });
    }
    fn cbits(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainBits {
            val: v,
            bits: LANE_BITS,
        });
    }
    /// Under `gate`, `c` must be 1.
    fn gassert(&mut self, gate: Operand, c: Operand) {
        let e = self.sel(gate, c, Self::imm(1));
        self.ins.push(Instruction::Assert { cond: e });
    }
    /// Multiplex `rows[i][lane]` by `index == i + 1` (zero when no row selected).
    fn mux_top(&mut self, count: &Operand, rows: &[&[Operand]], width: usize) -> Vec<Operand> {
        let mut out = vec![Self::imm(0); width];
        for (i, row) in rows.iter().enumerate() {
            let hit = self.eqc(count, (i as u128) + 1);
            for (lane, value) in out.iter_mut().enumerate() {
                *value = self.sel(hit.clone(), row[lane].clone(), value.clone());
            }
        }
        out
    }
}

/// The pre-state facts every rule path reads.
struct Facts {
    running: Operand,
    no_helper: Operand,
    no_result: Operand,
    ct: Operand,
    k: Vec<Operand>,
    depth: Operand,
    depth_zero: Operand,
    stack_limit: Operand,
    fuel_zero: Operand,
    top: Vec<Operand>,
    eval_row: [Operand; 7],
    origin_row: [Operand; 7],
    functions_count: Operand,
    site: Vec<Operand>,
    site_defined: Operand,
    htag: Operand,
    h: Vec<Operand>,
    rt: Operand,
    r1: Operand,
    live_pos: Operand,
    task: Vec<Operand>,
    validate_marker: Operand,
    equal_differ: Operand,
}

pub struct C6DispatchEmitter {
    scope: String,
    next: usize,
    prog: C6ProgramContextCanonicalEmitter,
    read_a: C6CheckedValueReadEmitter,
    read_b: C6CheckedValueReadEmitter,
    read_c: C6CheckedValueReadEmitter,
}

impl C6DispatchEmitter {
    pub fn new(scope: &str) -> Result<Self, C6DispatchError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6dispatch.") {
            return Err(C6DispatchError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next: 0,
            prog: C6ProgramContextCanonicalEmitter::new(&format!("{scope}.dispatchprog"))
                .map_err(be)?,
            read_a: C6CheckedValueReadEmitter::new(&format!("{scope}.dispatchreada"))
                .map_err(be)?,
            read_b: C6CheckedValueReadEmitter::new(&format!("{scope}.dispatchreadb"))
                .map_err(be)?,
            read_c: C6CheckedValueReadEmitter::new(&format!("{scope}.dispatchreadc"))
                .map_err(be)?,
        })
    }

    fn check_lengths(inputs: &C6DispatchInputs) -> Result<(), C6DispatchError> {
        let caps = &inputs.caps;
        let image = IMAGE_SECTION_HEADER_LANES + caps.phys.max_frames * FRAME_LANES;
        if inputs.pre_image.len() != image {
            return Err(C6DispatchError::LengthMismatch {
                section: "pre_image",
                expected: image,
                actual: inputs.pre_image.len(),
            });
        }
        let helper = L_TASKS + caps.max_helper_tasks * TASK_LANES;
        if inputs.pre_helper.len() != helper {
            return Err(C6DispatchError::LengthMismatch {
                section: "pre_helper",
                expected: helper,
                actual: inputs.pre_helper.len(),
            });
        }
        Ok(())
    }

    fn facts(&mut self, em: &mut Em, inputs: &C6DispatchInputs) -> Result<Facts, C6DispatchError> {
        let caps = inputs.caps;
        let img = &inputs.pre_image;
        let hl = &inputs.pre_helper;
        for lane in img.iter().chain(hl.iter()) {
            em.cbits(lane.clone());
        }
        let running = em.eqc(&img[L_OUTCOME], 0);
        let k: Vec<Operand> = img[L_CONTROL..L_CONTROL + 17].to_vec();
        let ct = k[0].clone();
        let depth = img[L_FRAME_COUNT].clone();
        let depth_zero = em.eqc(&depth, 0);
        let stack_limit = img[L_STACK_LIMIT].clone();
        let fuel_zero = em.eqc(&img[L_REMAINING_WORK], 0);
        let frames: Vec<&[Operand]> = (0..caps.phys.max_frames)
            .map(|i| {
                let at = IMAGE_SECTION_HEADER_LANES + i * FRAME_LANES;
                &img[at..at + FRAME_LANES]
            })
            .collect();
        let top = em.mux_top(&depth, &frames, FRAME_LANES);

        // One PROG walk: Eval expression, control-origin expression, function count.
        let is_eval = em.eqc(&ct, 0);
        let cap = self
            .prog
            .emit_prog_dispatch_capture(&C6ProgramContextDispatchCaptureInputs {
                gate: inputs.enable.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps,
                first_index: k[1].clone(),
                second_index: img[L_CONTROL_ORIGIN].clone(),
                child_index: Em::imm(0),
            })
            .map_err(be)?;
        em.ins.extend(cap.instructions);
        let eval_gate = em.and(inputs.enable.clone(), is_eval.clone());
        em.gassert(eval_gate, cap.first_found.clone());
        let eval_row = cap.first_row;
        let origin_row = cap.second_row;

        // Site one-hot.
        let mut site = vec![Em::imm(0); SITES];
        let e = eval_row[0].clone();
        for (tag, s) in [
            (0, Site::EvalLiteral),
            (1, Site::EvalVar),
            (2, Site::EvalLet),
            (3, Site::EvalBinary),
            (4, Site::EvalUnary),
            (5, Site::EvalIf),
            (6, Site::EvalCall),
            (7, Site::EvalMatch),
            (8, Site::EvalConstruct),
        ] {
            let is_tag = em.eqc(&e, tag);
            site[s as usize] = em.and(is_eval.clone(), is_tag);
        }
        let is_return = em.eqc(&ct, 1);
        let depth_pos = em.not(depth_zero.clone());
        let ret = em.and(is_return, depth_pos);
        for (tag, s) in [
            (0, Site::RetLet),
            (1, Site::RetLeft),
            (2, Site::RetRight),
            (3, Site::RetUnary),
            (4, Site::RetIf),
            (5, Site::RetMatch),
            (6, Site::RetArgs),
            (7, Site::RetCall),
            (8, Site::RetScope),
            (9, Site::RetCtorFields),
        ] {
            let is_tag = em.eqc(&top[0], tag);
            site[s as usize] = em.and(ret.clone(), is_tag);
        }
        for (tag, s) in [
            (2, Site::Push),
            (3, Site::Invoke),
            (4, Site::Enter),
            (5, Site::Bound),
            (6, Site::Cases),
            (7, Site::CaseResult),
            (8, Site::Prim),
            (10, Site::Materialize),
        ] {
            site[s as usize] = em.eqc(&ct, tag);
        }
        let site_defined = em.any(&site.clone());
        // A Cases site reads its control-origin expression; Core errors when absent.
        let cases_gate = em.and(inputs.enable.clone(), site[Site::Cases as usize].clone());
        em.gassert(cases_gate, cap.second_found.clone());

        let htag = hl[L_HELPER_RECORD].clone();
        let no_helper = em.eqc(&htag, 0);
        let h: Vec<Operand> = hl[L_HELPER_RECORD..L_HELPER_RECORD + 23].to_vec();
        let rt = hl[L_RESULT_RECORD].clone();
        let r1 = hl[L_RESULT_RECORD + 1].clone();
        let no_result = em.eqc(&rt, 0);
        let live = hl[3].clone();
        let live_zero = em.eqc(&live, 0);
        let live_pos = em.not(live_zero);
        let tasks: Vec<&[Operand]> = (0..caps.max_helper_tasks)
            .map(|i| {
                let at = L_TASKS + i * TASK_LANES;
                &hl[at..at + TASK_LANES]
            })
            .collect();
        let task = em.mux_top(&live, &tasks, TASK_LANES);

        // Data reads, gated exactly when the guard consults them.
        let pending = em.eqc(&h[7], 0);
        let validate = em.eqc(&htag, H_VALIDATE);
        let validate_task = em.eqc(&task[0], 0);
        let validate_unvisited = em.eqc(&task[3], 0);
        let vgate = em.all(&[
            inputs.enable.clone(),
            validate,
            pending,
            live_pos.clone(),
            validate_task,
            validate_unvisited,
        ]);
        let va = self
            .read_a
            .emit(&C6CheckedValueReadInputs {
                gate: vgate.clone(),
                ref_kind_is_data: task[1].clone(),
                ref_index: task[2].clone(),
                pre_prog: inputs.pre_prog.clone(),
                pre_data: inputs.pre_data.clone(),
                caps,
            })
            .map_err(be)?;
        em.ins.extend(va.instructions.clone());
        em.gassert(vgate, va.selected_found.clone());
        let validate_marker = va.is_marker.clone();

        let equal = em.eqc(&htag, H_EQUAL);
        let epending = em.eqc(&h[9], 0);
        let equal_task = em.eqc(&task[0], 1);
        let equal_unvisited = em.eqc(&task[5], 0);
        let egate = em.all(&[
            inputs.enable.clone(),
            equal,
            epending,
            live_pos.clone(),
            equal_task,
            equal_unvisited,
        ]);
        let left = self
            .read_b
            .emit(&C6CheckedValueReadInputs {
                gate: egate.clone(),
                ref_kind_is_data: task[1].clone(),
                ref_index: task[2].clone(),
                pre_prog: inputs.pre_prog.clone(),
                pre_data: inputs.pre_data.clone(),
                caps,
            })
            .map_err(be)?;
        em.ins.extend(left.instructions.clone());
        let right = self
            .read_c
            .emit(&C6CheckedValueReadInputs {
                gate: egate.clone(),
                ref_kind_is_data: task[3].clone(),
                ref_index: task[4].clone(),
                pre_prog: inputs.pre_prog.clone(),
                pre_data: inputs.pre_data.clone(),
                caps,
            })
            .map_err(be)?;
        em.ins.extend(right.instructions.clone());
        em.gassert(egate.clone(), left.selected_found.clone());
        em.gassert(egate, right.selected_found.clone());
        // shallow_differ: kind, uint and bool projections (absent projections are 0).
        let same_kind = em.teq(left.kind_code.clone(), right.kind_code.clone());
        let same_uint = em.teq(left.uint_value.clone(), right.uint_value.clone());
        let same_uint_p = em.teq(left.uint_present.clone(), right.uint_present.clone());
        let same_bool = em.teq(left.bool_value.clone(), right.bool_value.clone());
        let same_bool_p = em.teq(left.bool_present.clone(), right.bool_present.clone());
        let same_type = em.teq(left.type_id.clone(), right.type_id.clone());
        let same_ctor = em.teq(left.ctor_id.clone(), right.ctor_id.clone());
        let same = em.all(&[
            same_kind,
            same_uint,
            same_uint_p,
            same_bool,
            same_bool_p,
            same_type,
            same_ctor,
        ]);
        let equal_differ = em.not(same);

        Ok(Facts {
            running,
            no_helper,
            no_result,
            ct,
            k,
            depth,
            depth_zero,
            stack_limit,
            fuel_zero,
            top,
            eval_row,
            origin_row,
            functions_count: cap.functions_count,
            site,
            site_defined,
            htag,
            h,
            rt,
            r1,
            live_pos,
            task,
            validate_marker,
            equal_differ,
        })
    }

    /// Core `owner_helper_plan(site)` as a per-kind boolean.
    fn plan_kind(em: &mut Em, f: &Facts, kind: u128) -> Operand {
        let s = |x: Site| f.site[x as usize].clone();
        match kind {
            H_VALIDATE => s(Site::EvalLiteral),
            H_ENVGET => s(Site::EvalVar),
            H_ENVPUT => s(Site::RetLet),
            H_EQUAL => {
                let eq = em.eqc(&f.top[1], 3);
                em.and(s(Site::RetRight), eq)
            }
            H_SCALAR => {
                let eq = em.eqc(&f.top[1], 3);
                let neq = em.not(eq);
                let right = em.and(s(Site::RetRight), neq);
                let unchecked = em.eqc(&f.top[5], 0);
                let ctor = em.and(s(Site::RetCtorFields), unchecked);
                em.any(&[
                    right,
                    s(Site::RetUnary),
                    s(Site::RetIf),
                    s(Site::RetCall),
                    ctor,
                ])
            }
            H_APPEND => {
                let checked = em.eqc(&f.top[5], 1);
                let ctor = em.and(s(Site::RetCtorFields), checked);
                em.or(s(Site::RetArgs), ctor)
            }
            H_FIND => s(Site::Invoke),
            H_BIND => {
                let present = em.eqc(&f.k[1], 1);
                em.and(s(Site::Enter), present)
            }
            H_MATCH => {
                let is_match = em.eqc(&f.origin_row[0], 7);
                let no_plan = em.eqc(&f.origin_row[4], 0);
                let arms = em.nonzero(&f.k[4]);
                em.all(&[s(Site::Cases), is_match, no_plan, arms])
            }
            _ => Em::imm(0),
        }
    }

    /// Plan/Build helper plans exist in Core; they select no supported rule but
    /// still preempt the legacy scan, so they are included in `any_plan`.
    fn any_plan(em: &mut Em, f: &Facts) -> Operand {
        let mut bits = Vec::new();
        for kind in 1..=9 {
            bits.push(Self::plan_kind(em, f, kind));
        }
        let is_match = em.eqc(&f.origin_row[0], 7);
        let has_plan = em.eqc(&f.origin_row[4], 1);
        let plan = em.all(&[f.site[Site::Cases as usize].clone(), is_match, has_plan]);
        bits.push(plan);
        bits.push(f.site[Site::Materialize as usize].clone());
        em.any(&bits)
    }

    /// Core `select_when` for one rule, over the C6 lanes.
    fn guard(em: &mut Em, f: &Facts, rule: RuleIdC) -> Result<Operand, C6DispatchError> {
        use RuleIdC::*;
        let h = |j: usize| f.h[j].clone();
        let rt_is = |em: &mut Em, cs: &[u128]| em.in_set(&f.rt, cs);
        let r1_is = |em: &mut Em, c: u128| em.eqc(&f.r1, c);
        Ok(match rule {
            KMachineEvalLiteralValid => {
                let a = rt_is(em, &[1]);
                let b = r1_is(em, 1);
                em.and(a, b)
            }
            KMachineEvalLiteralInvalid => {
                let a = rt_is(em, &[1]);
                let b = r1_is(em, 0);
                em.and(a, b)
            }
            KMachineEvalVariablePresent => rt_is(em, &[2]),
            KMachineEvalVariableAbsent => rt_is(em, &[3]),
            KMachineEvalLet | KMachineEvalBinary | KMachineEvalUnary | KMachineEvalIf
            | KMachineEvalMatch | KMachineReturnLeft | KMachineReturnMatch
            | KMachineReturnScope => Em::imm(1),
            KMachineEvalCallEmpty => {
                let call = em.eqc(&f.eval_row[0], 6);
                let empty = em.eqc(&f.eval_row[3], 0);
                em.and(call, empty)
            }
            KMachineEvalCallArguments => {
                let call = em.eqc(&f.eval_row[0], 6);
                let args = em.nonzero(&f.eval_row[3]);
                em.and(call, args)
            }
            KMachinePushAccept => em.lt(f.depth.clone(), f.stack_limit.clone()),
            KMachinePushRefuse => {
                let lt = em.lt(f.depth.clone(), f.stack_limit.clone());
                em.not(lt)
            }
            KMachineReturnLet => rt_is(em, &[4]),
            KMachineReturnRight => rt_is(em, &[10, 11, 14]),
            KMachineReturnUnary => rt_is(em, &[10, 11]),
            KMachineReturnIfTrue | KMachineReturnIfFalse | KMachineReturnIfInvalid => {
                let sel = match rule {
                    KMachineReturnIfTrue => 0,
                    KMachineReturnIfFalse => 1,
                    _ => 2,
                };
                let a = rt_is(em, &[12]);
                let b = r1_is(em, sel);
                em.and(a, b)
            }
            KMachineReturnArgumentsNext | KMachineReturnArgumentsFinal => {
                let a = rt_is(em, &[9]);
                let args = em.eqc(&f.top[0], 6);
                let rest_empty = em.eqc(&f.top[5], 0);
                let rest = if rule == KMachineReturnArgumentsNext {
                    em.not(rest_empty)
                } else {
                    rest_empty
                };
                em.all(&[a, args, rest])
            }
            KMachineReturnCallValid | KMachineReturnCallInvalid => {
                let a = rt_is(em, &[13]);
                let b = r1_is(em, u128::from(rule == KMachineReturnCallValid));
                em.and(a, b)
            }
            KMachineInvokeFind => rt_is(em, &[5, 6]),
            KMachineEnterMissing => {
                let enter = em.eqc(&f.ct, 4);
                let missing = em.eqc(&f.k[1], 0);
                em.and(enter, missing)
            }
            KMachineEnterBind => rt_is(em, &[7, 8]),
            KMachineBoundInvalid | KMachineBoundValid => {
                let bound = em.eqc(&f.ct, 5);
                let ok = em.eqc(&f.k[1], u128::from(rule == KMachineBoundValid));
                em.and(bound, ok)
            }
            KMachineCasesEmpty => {
                let cases = em.eqc(&f.ct, 6);
                let empty = em.eqc(&f.k[4], 0);
                let is_match = em.eqc(&f.origin_row[0], 7);
                let no_plan = em.eqc(&f.origin_row[4], 0);
                em.all(&[cases, empty, is_match, no_plan])
            }
            KMachineCasesTry => {
                let is_match = em.eqc(&f.origin_row[0], 7);
                let no_plan = em.eqc(&f.origin_row[4], 0);
                let r = rt_is(em, &[15, 16]);
                em.all(&[is_match, no_plan, r])
            }
            KMachineCasesMiss | KMachineCasesHit => {
                let case = em.eqc(&f.ct, 7);
                let hit = em.eqc(&f.k[1], u128::from(rule == KMachineCasesHit));
                em.and(case, hit)
            }
            KMachinePrimitiveValue | KMachinePrimitiveError => {
                let prim = em.eqc(&f.ct, 8);
                let err = em.eqc(&f.k[1], u128::from(rule == KMachinePrimitiveError));
                em.and(prim, err)
            }
            XAdtEvalConstructNullary => {
                let construct = em.eqc(&f.eval_row[0], 8);
                let no_fields = em.eqc(&f.eval_row[4], 0);
                let constant = em.eqc(&f.eval_row[5], 1);
                em.all(&[construct, no_fields, constant])
            }
            XAdtEvalConstructFields => {
                let construct = em.eqc(&f.eval_row[0], 8);
                let fields = em.nonzero(&f.eval_row[4]);
                let no_constant = em.eqc(&f.eval_row[5], 0);
                em.all(&[construct, fields, no_constant])
            }
            XAdtReturnCtorFieldsTyped | XAdtReturnCtorFieldsMistyped => {
                let ctor = em.eqc(&f.top[0], 9);
                let unchecked = em.eqc(&f.top[5], 0);
                let r = rt_is(em, &[13]);
                let ok = r1_is(em, u128::from(rule == XAdtReturnCtorFieldsTyped));
                em.all(&[ctor, unchecked, r, ok])
            }
            XAdtReturnCtorFieldsNext | XAdtReturnCtorFieldsFinal => {
                let ctor = em.eqc(&f.top[0], 9);
                let checked = em.eqc(&f.top[5], 1);
                let rest_empty = em.eqc(&f.top[9], 0);
                let rest = if rule == XAdtReturnCtorFieldsNext {
                    em.not(rest_empty)
                } else {
                    rest_empty
                };
                let r = rt_is(em, &[9]);
                em.all(&[ctor, checked, rest, r])
            }
            XAdtMaterializeDone => {
                let mat = em.eqc(&f.ct, 10);
                let r = rt_is(em, &[19]);
                em.and(mat, r)
            }
            XAdtCasesPlanHit | XAdtCasesPlanMiss => {
                let cases = em.eqc(&f.ct, 6);
                let is_match = em.eqc(&f.origin_row[0], 7);
                let plan = em.eqc(&f.origin_row[4], 1);
                let r = rt_is(em, &[if rule == XAdtCasesPlanHit { 17 } else { 18 }]);
                em.all(&[cases, is_match, plan, r])
            }
            // Scalar: 5 mode, 13 phase, 15 staged tag.
            BHelperScalarConstruct => {
                let first = em.eqc(&h(13), 0);
                let leaf = em.eqc(&h(5), 6);
                let a = em.and(first, leaf);
                let second = em.eqc(&h(13), 1);
                let node = em.in_set(&h(5), &[4, 5]);
                let b = em.and(second, node);
                em.or(a, b)
            }
            BHelperScalarArithmetic => {
                let first = em.eqc(&h(13), 0);
                let not = em.eqc(&h(5), 7);
                let a = em.and(first, not);
                let second = em.eqc(&h(13), 1);
                let arith = em.in_set(&h(5), &[0, 1, 2, 3]);
                let b = em.and(second, arith);
                em.or(a, b)
            }
            BHelperScalarTag => {
                let first = em.eqc(&h(13), 0);
                let excluded = em.in_set(&h(5), &[6, 7]);
                let included = em.not(excluded);
                em.and(first, included)
            }
            BHelperScalarFailure | BHelperScalarExit => {
                let done = em.eqc(&h(13), 2);
                let error = em.eqc(&h(15), 4);
                let e = if rule == BHelperScalarFailure {
                    error
                } else {
                    em.not(error)
                };
                em.and(done, e)
            }
            // EnvGet: 6 cursor present, 11 status.
            BHelperEnvGetExit => em.eqc(&h(11), 1),
            BHelperEnvGetFailure => em.eqc(&h(11), 2),
            BHelperEnvGetCell | BHelperEnvGetEmpty => {
                let pending = em.eqc(&h(11), 0);
                let present = em.eqc(&h(6), u128::from(rule == BHelperEnvGetCell));
                em.and(pending, present)
            }
            // EnvPut: 12 status.
            BHelperEnvPutInsert => em.eqc(&h(12), 0),
            BHelperEnvPutExit => em.eqc(&h(12), 1),
            // Find: 6 row cursor, 9 status.
            BHelperFindExit => em.eqc(&h(9), 1),
            BHelperFindFailure => em.eqc(&h(9), 2),
            BHelperFindRow => {
                let pending = em.eqc(&h(9), 0);
                let lt = em.lt(h(6), f.functions_count.clone());
                em.and(pending, lt)
            }
            BHelperFindEmpty => {
                let pending = em.eqc(&h(9), 0);
                let eq = em.teq(h(6), f.functions_count.clone());
                em.and(pending, eq)
            }
            // Bind: 7 params.len, 9 args.len, 10 p_cursor, 11 a_cursor, 16 phase, 17 status.
            BHelperBindExit | BHelperBindFailure => {
                let finished = em.eqc(&h(16), 5);
                let st = em.eqc(&h(17), u128::from(rule == BHelperBindFailure));
                em.and(finished, st)
            }
            BHelperBindPair => {
                let pairing = em.eqc(&h(16), 0);
                let p = em.lt(h(10), h(7));
                let a = em.lt(h(11), h(9));
                em.all(&[pairing, p, a])
            }
            BHelperBindEmpty => {
                let pairing = em.eqc(&h(16), 0);
                let p = em.teq(h(10), h(7));
                let a = em.teq(h(11), h(9));
                em.all(&[pairing, p, a])
            }
            BHelperBindArity => {
                let pairing = em.eqc(&h(16), 0);
                let p = em.teq(h(10), h(7));
                let a = em.teq(h(11), h(9));
                let same = em.teq(p, a);
                let differ = em.not(same);
                em.and(pairing, differ)
            }
            BHelperBindType => em.eqc(&h(16), 1),
            BHelperBindDuplicate => em.eqc(&h(16), 2),
            BHelperBindInsert => em.eqc(&h(16), 3),
            BHelperBindPropagate => em.eqc(&h(16), 4),
            // Append: 6 src.len, 9 src_cursor, 11 staged present, 14 phase.
            BHelperAppendCopy => {
                let copy = em.eqc(&h(14), 0);
                let staged = em.eqc(&h(11), 1);
                em.and(copy, staged)
            }
            BHelperAppendCell => {
                let copy = em.eqc(&h(14), 0);
                let unstaged = em.eqc(&h(11), 0);
                let lt = em.lt(h(9), h(6));
                em.all(&[copy, unstaged, lt])
            }
            BHelperAppendEmpty => {
                let copy = em.eqc(&h(14), 0);
                let unstaged = em.eqc(&h(11), 0);
                let eq = em.teq(h(9), h(6));
                em.all(&[copy, unstaged, eq])
            }
            BHelperAppendTail => em.eqc(&h(14), 1),
            BHelperAppendExit => em.eqc(&h(14), 2),
            // Validate: 7 status; tasks: 0 tag, 3 visit.
            BBookkeepingTaskRelease => {
                let validate = em.eqc(&f.htag, H_VALIDATE);
                let vneg = em.eqc(&h(7), 2);
                let v = em.and(validate, vneg);
                let equal = em.eqc(&f.htag, H_EQUAL);
                let eneg = em.eqc(&h(9), 2);
                let e = em.and(equal, eneg);
                let either = em.or(v, e);
                em.and(either, f.live_pos.clone())
            }
            BHelperValidateFailure => {
                let neg = em.eqc(&h(7), 2);
                let zero = em.not(f.live_pos.clone());
                em.and(neg, zero)
            }
            BHelperValidateExit => em.eqc(&h(7), 1),
            BHelperValidateEmpty => {
                let pending = em.eqc(&h(7), 0);
                let zero = em.not(f.live_pos.clone());
                em.and(pending, zero)
            }
            BHelperValidateInvalid | BHelperValidateOccurrence => {
                let pending = em.eqc(&h(7), 0);
                let task = em.eqc(&f.task[0], 0);
                let unvisited = em.eqc(&f.task[3], 0);
                let marker = if rule == BHelperValidateInvalid {
                    f.validate_marker.clone()
                } else {
                    em.not(f.validate_marker.clone())
                };
                em.all(&[pending, f.live_pos.clone(), task, unvisited, marker])
            }
            BHelperValidateField => {
                let pending = em.eqc(&h(7), 0);
                let task = em.eqc(&f.task[0], 0);
                let fields = em.eqc(&f.task[3], 1);
                em.all(&[pending, f.live_pos.clone(), task, fields])
            }
            // Equal: 9 status; tasks: 0 tag, 5 visit.
            BHelperEqualExit => {
                let pos = em.eqc(&h(9), 1);
                let neg = em.eqc(&h(9), 2);
                let zero = em.not(f.live_pos.clone());
                let neg_done = em.and(neg, zero);
                em.or(pos, neg_done)
            }
            BHelperEqualEmpty => {
                let pending = em.eqc(&h(9), 0);
                let zero = em.not(f.live_pos.clone());
                em.and(pending, zero)
            }
            BHelperEqualDifferent | BHelperEqualPair => {
                let pending = em.eqc(&h(9), 0);
                let task = em.eqc(&f.task[0], 1);
                let unvisited = em.eqc(&f.task[5], 0);
                let differ = if rule == BHelperEqualDifferent {
                    f.equal_differ.clone()
                } else {
                    em.not(f.equal_differ.clone())
                };
                em.all(&[pending, f.live_pos.clone(), task, unvisited, differ])
            }
            BHelperEqualField => {
                let pending = em.eqc(&h(9), 0);
                let task = em.eqc(&f.task[0], 1);
                let fields = em.eqc(&f.task[5], 1);
                em.all(&[pending, f.live_pos.clone(), task, fields])
            }
            // Match: 19 phase, 20 status.
            BHelperMatchExit | BHelperMatchFailure => {
                let done = em.eqc(&h(19), 7);
                let st = em.eqc(&h(20), if rule == BHelperMatchExit { 1 } else { 2 });
                em.and(done, st)
            }
            BHelperMatchArm => em.eqc(&h(19), 0),
            BHelperMatchOccurrence => em.eqc(&h(19), 1),
            BHelperMatchDistinct => em.eqc(&h(19), 2),
            BHelperMatchField => em.eqc(&h(19), 3),
            BHelperMatchBinding => em.eqc(&h(19), 4),
            BHelperMatchSelected => em.eqc(&h(19), 5),
            BHelperMatchNext => em.eqc(&h(19), 6),
            other => return Err(C6DispatchError::Unsupported(other)),
        })
    }

    fn path(em: &mut Em, f: &Facts, rule: RuleIdC) -> Result<Operand, C6DispatchError> {
        let quiet = em.and(f.no_helper.clone(), f.no_result.clone());
        let is_error = em.eqc(&f.ct, 9);
        let p1_error = em.and(quiet.clone(), is_error);
        let is_return = em.eqc(&f.ct, 1);
        let ret0 = em.and(is_return, f.depth_zero.clone());
        let p1_value = em.and(quiet.clone(), ret0);
        match rule {
            RuleIdC::KAdminPublishError => return Ok(em.and(f.running.clone(), p1_error)),
            RuleIdC::KAdminPublishValue => return Ok(em.and(f.running.clone(), p1_value)),
            _ => {}
        }
        let p1 = em.or(p1_error, p1_value);
        let not_p1 = em.not(p1);
        if rule == RuleIdC::KAdminPublishExhausted {
            return Ok(em.all(&[f.running.clone(), not_p1, f.fuel_zero.clone()]));
        }
        let fueled = em.not(f.fuel_zero.clone());
        let base = em.all(&[f.running.clone(), not_p1, fueled, f.site_defined.clone()]);
        if rule == RuleIdC::BBookkeepingTaskRelease {
            // First rule of both the Validate and the Equal groups; its guard
            // names both helper kinds, so no earlier rule is negated.
            let g = Self::guard(em, f, rule)?;
            return Ok(em.and(base, g));
        }
        if let Some(kind) = helper_kind_of(rule) {
            let group = helper_group(kind);
            let is_kind = em.eqc(&f.htag, kind);
            let mut terms = vec![base, is_kind];
            for r in group {
                let g = Self::guard(em, f, *r)?;
                if *r == rule {
                    terms.push(g);
                    break;
                }
                terms.push(em.not(g));
            }
            return Ok(em.all(&terms));
        }
        if let Some(kind) = entry_kind(rule) {
            let plan = Self::plan_kind(em, f, kind);
            return Ok(em.all(&[base, quiet, plan]));
        }
        let Some(site) = legacy_site_of(rule) else {
            return Err(C6DispatchError::Unsupported(rule));
        };
        let at_site = f.site[site as usize].clone();
        let mut terms = vec![base, f.no_helper.clone(), at_site];
        let result_reader = reads_result(rule);
        if result_reader {
            terms.push(em.not(f.no_result.clone()));
        } else {
            terms.push(f.no_result.clone());
            let plan = Self::any_plan(em, f);
            terms.push(em.not(plan));
        }
        for r in legacy_group(site) {
            if reads_result(*r) != result_reader {
                continue;
            }
            let g = Self::guard(em, f, *r)?;
            if *r == rule {
                terms.push(g);
                break;
            }
            terms.push(em.not(g));
        }
        Ok(em.all(&terms))
    }

    /// Constrain that Core selects `rule` on the pre carriers when `enable == 1`.
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6DispatchInputs,
    ) -> Result<C6DispatchBlock, C6DispatchError> {
        if !dispatch_supported(rule) {
            return Err(C6DispatchError::Unsupported(rule));
        }
        Self::check_lengths(inputs)?;
        let mut em = Em {
            scope: self.scope.clone(),
            next: self.next,
            ins: Vec::new(),
        };
        em.cbool(inputs.enable.clone());
        let facts = self.facts(&mut em, inputs)?;
        let selected = Self::path(&mut em, &facts, rule)?;
        em.gassert(inputs.enable.clone(), selected);
        self.next = em.next;
        Ok(C6DispatchBlock {
            instructions: em.ins,
        })
    }
}

pub fn emit_c6_dispatch_block(
    rule: RuleIdC,
    inputs: &C6DispatchInputs,
    scope: &str,
) -> Result<C6DispatchBlock, C6DispatchError> {
    C6DispatchEmitter::new(scope)?.emit(rule, inputs)
}
