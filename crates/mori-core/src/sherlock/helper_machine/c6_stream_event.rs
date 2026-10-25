//! Lossless bounded event lanes for the C6 range storage stream, not proof authority.
use super::shared::*;
use std::sync::Arc;
#[derive(Debug)]
pub enum EventCodecError {
    Bound,
    Invalid,
    Truncated,
    Trailing,
}
struct Enc {
    lanes: Vec<u128>,
    max: usize,
}
impl Enc {
    fn put(&mut self, n: u128) -> Result<(), EventCodecError> {
        if self.lanes.len() == self.max {
            return Err(EventCodecError::Bound);
        }
        self.lanes.push(n);
        Ok(())
    }
    fn opt<T>(
        &mut self,
        v: Option<T>,
        f: impl FnOnce(&mut Self, T) -> Result<(), EventCodecError>,
    ) -> Result<(), EventCodecError> {
        match v {
            None => self.put(0),
            Some(v) => {
                self.put(1)?;
                f(self, v)
            }
        }
    }
    fn bytes(&mut self, b: &[u8]) -> Result<(), EventCodecError> {
        self.put(b.len() as u128)?;
        for x in b {
            self.put((*x).into())?
        }
        Ok(())
    }
    fn signed(&mut self, n: i64) -> Result<(), EventCodecError> {
        self.put(u128::from(n < 0))?;
        self.put(n.unsigned_abs().into())
    }
}
struct Dec<'a> {
    lanes: &'a [u128],
    at: usize,
}
impl<'a> Dec<'a> {
    fn get(&mut self) -> Result<u128, EventCodecError> {
        let v = *self.lanes.get(self.at).ok_or(EventCodecError::Truncated)?;
        self.at += 1;
        Ok(v)
    }
    fn usize(&mut self) -> Result<usize, EventCodecError> {
        self.get()?.try_into().map_err(|_| EventCodecError::Invalid)
    }
    fn byte(&mut self) -> Result<u8, EventCodecError> {
        self.get()?.try_into().map_err(|_| EventCodecError::Invalid)
    }
    fn boolean(&mut self) -> Result<bool, EventCodecError> {
        match self.get()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(EventCodecError::Invalid),
        }
    }
    fn opt<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, EventCodecError>,
    ) -> Result<Option<T>, EventCodecError> {
        match self.get()? {
            0 => Ok(None),
            1 => Ok(Some(f(self)?)),
            _ => Err(EventCodecError::Invalid),
        }
    }
    fn string(&mut self) -> Result<String, EventCodecError> {
        let n = self.usize()?;
        if n > self.lanes.len() - self.at {
            return Err(EventCodecError::Truncated);
        }
        let mut b = Vec::with_capacity(n);
        for _ in 0..n {
            b.push(self.byte()?)
        }
        String::from_utf8(b).map_err(|_| EventCodecError::Invalid)
    }
    fn signed(&mut self) -> Result<i64, EventCodecError> {
        let neg = self.boolean()?;
        let n = self.get()?;
        if neg {
            if n == 0 || n > (1u128 << 63) {
                return Err(EventCodecError::Invalid);
            }
            Ok(-(n as i128) as i64)
        } else {
            n.try_into().map_err(|_| EventCodecError::Invalid)
        }
    }
}
// One explicit tag table expands to exhaustive encode and strict decode matches.
macro_rules! scalar_enum {
 ($enc:ident,$dec:ident,$ty:ident;$($variant:ident=$tag:literal),+)=>{
  #[allow(non_snake_case)] fn $enc(v:$ty)->u128{match v{$($ty::$variant=>$tag),+}}
  #[allow(non_snake_case)] fn $dec(n:u128)->Result<$ty,EventCodecError>{match n{$($tag=>Ok($ty::$variant),)+_=>Err(EventCodecError::Invalid)}}
 }
}
scalar_enum!(enc_RuleIdC,dec_RuleIdC,RuleIdC;KMachineEvalLiteralValid=0,KMachineEvalLiteralInvalid=1,KMachineEvalVariablePresent=2,KMachineEvalVariableAbsent=3,KMachineEvalLet=4,KMachineEvalBinary=5,KMachineEvalUnary=6,KMachineEvalIf=7,KMachineEvalMatch=8,KMachineEvalCallEmpty=9,KMachineEvalCallArguments=10,KMachinePushAccept=11,KMachinePushRefuse=12,KMachineReturnLet=13,KMachineReturnLeft=14,KMachineReturnRight=15,KMachineReturnUnary=16,KMachineReturnIfTrue=17,KMachineReturnIfFalse=18,KMachineReturnIfInvalid=19,KMachineReturnMatch=20,KMachineReturnArgumentsNext=21,KMachineReturnArgumentsFinal=22,KMachineReturnCallValid=23,KMachineReturnCallInvalid=24,KMachineReturnScope=25,KMachineInvokeFind=26,KMachineEnterMissing=27,KMachineEnterBind=28,KMachineBoundInvalid=29,KMachineBoundValid=30,KMachineCasesEmpty=31,KMachineCasesTry=32,KMachineCasesMiss=33,KMachineCasesHit=34,KMachinePrimitiveValue=35,KMachinePrimitiveError=36,BHelperScalarEntry=37,BHelperScalarTag=38,BHelperScalarArithmetic=39,BHelperScalarConstruct=40,BHelperScalarExit=41,BHelperScalarFailure=42,BHelperEnvGetEntry=43,BHelperEnvGetCell=44,BHelperEnvGetEmpty=45,BHelperEnvGetExit=46,BHelperEnvGetFailure=47,BHelperEnvPutEntry=48,BHelperEnvPutCell=49,BHelperEnvPutInsert=50,BHelperEnvPutEmpty=51,BHelperEnvPutExit=52,BHelperEnvPutFailure=53,BHelperFindEntry=54,BHelperFindRow=55,BHelperFindEmpty=56,BHelperFindExit=57,BHelperFindFailure=58,BHelperBindEntry=59,BHelperBindPair=60,BHelperBindEmpty=61,BHelperBindArity=62,BHelperBindType=63,BHelperBindDuplicate=64,BHelperBindInsert=65,BHelperBindPropagate=66,BHelperBindExit=67,BHelperBindFailure=68,BHelperAppendEntry=69,BHelperAppendCell=70,BHelperAppendEmpty=71,BHelperAppendTail=72,BHelperAppendCopy=73,BHelperAppendExit=74,BHelperAppendFailure=75,BHelperValidateEntry=76,BHelperValidateOccurrence=77,BHelperValidateField=78,BHelperValidateDeclaration=79,BHelperValidateEmpty=80,BHelperValidateInvalid=81,BHelperValidateExit=82,BHelperValidateFailure=83,BHelperEqualEntry=84,BHelperEqualPair=85,BHelperEqualField=86,BHelperEqualEmpty=87,BHelperEqualDifferent=88,BHelperEqualExit=89,BHelperEqualFailure=90,BHelperMatchEntry=91,BHelperMatchArm=92,BHelperMatchOccurrence=93,BHelperMatchField=94,BHelperMatchBinding=95,BHelperMatchDistinct=96,BHelperMatchDiscard=97,BHelperMatchNext=98,BHelperMatchSelected=99,BHelperMatchEmpty=100,BHelperMatchExit=101,BHelperMatchFailure=102,BBookkeepingTaskEnqueue=103,BBookkeepingTaskRelease=104,BBookkeepingResumeStore=105,BBookkeepingResumeRestore=106,KAdminStartupValid=107,KAdminStartupInvalid=108,KAdminPublishValue=109,KAdminPublishError=110,KAdminPublishExhausted=111,XAdtEvalConstructNullary=112,XAdtEvalConstructFields=113,XAdtReturnCtorFieldsTyped=114,XAdtReturnCtorFieldsMistyped=115,XAdtReturnCtorFieldsNext=116,XAdtReturnCtorFieldsFinal=117,XAdtMaterializeDone=118,XAdtCasesPlanHit=119,XAdtCasesPlanMiss=120,XAdtHelperBuildEntry=121,XAdtHelperBuildCell=122,XAdtHelperBuildField=123,XAdtHelperBuildExit=124,XAdtHelperPlanEntry=125,XAdtHelperPlanWalk=126,XAdtHelperPlanSwitch=127,XAdtHelperPlanBind=128,XAdtHelperPlanSelect=129,XAdtHelperPlanNoMatch=130,XAdtHelperPlanExit=131,XAdtHelperPlanFailure=132);
scalar_enum!(enc_EventKind,dec_EventKind,EventKind;LegacyComplete=0,HelperEnter=1,HelperVisit=2,HelperExit=3,HelperFail=4,Bookkeeping=5,Publish=6,Startup=7);
scalar_enum!(enc_SiteTag,dec_SiteTag,SiteTag;EvalLiteral=0,EvalVar=1,EvalLet=2,EvalBinary=3,EvalUnary=4,EvalIf=5,EvalMatch=6,EvalCall=7,Push=8,RetLet=9,RetLeft=10,RetRight=11,RetUnary=12,RetIf=13,RetMatch=14,RetArgs=15,RetCall=16,RetScope=17,Invoke=18,Enter=19,Bound=20,Cases=21,CaseResult=22,Prim=23,EvalConstruct=24,RetCtorFields=25,Materialize=26);
scalar_enum!(enc_HelperKind,dec_HelperKind,HelperKind;Scalar=0,EnvGet=1,EnvPut=2,Find=3,Bind=4,Append=5,Validate=6,Equal=7,Match=8,Plan=9,Build=10);
scalar_enum!(enc_StepDisposition,dec_StepDisposition,StepDisposition;Applied=0,Published=1,ExhaustedFuelUncharged=2,RefusedHelperWorkUncharged=3,RefusedCapCharged=4);
scalar_enum!(enc_ScalarPhase,dec_ScalarPhase,ScalarPhase;First=0,Second=1,Done=2);
scalar_enum!(enc_TriStatus,dec_TriStatus,TriStatus;Pending=0,Positive=1,Negative=2);
scalar_enum!(enc_BindPhase,dec_BindPhase,BindPhase;Pairing=0,UnwindType=1,UnwindDup=2,UnwindInsert=3,Propagate=4,Finished=5);
scalar_enum!(enc_AppendPhase,dec_AppendPhase,AppendPhase;Copy=0,Tail=1,Done=2);
scalar_enum!(enc_MatchPhase,dec_MatchPhase,MatchPhase;Arm=0,Tag=1,Distinct=2,Field=3,Bind=4,Ready=5,NoMatchReady=6,Done=7);
scalar_enum!(enc_BuildPhase,dec_BuildPhase,BuildPhase;Copy=0,Done=1);
scalar_enum!(enc_PlanPhase,dec_PlanPhase,PlanPhase;Walk=0,Test=1,Bind=2,Ready=3,NoMatchReady=4,Done=5);
scalar_enum!(enc_HelperResultTagC,dec_HelperResultTagC,HelperResultTagC;Validated=0,EnvFound=1,EnvMissing=2,EnvUpdated=3,FindFound=4,FindMissing=5,BindOk=6,BindBad=7,AppendDone=8,ScalarValue=9,ScalarError=10,ScalarSelector=11,ScalarTypeTest=12,EqualDone=13,MatchMatched=14,MatchNoMatch=15,PlanMatched=16,PlanNoMatch=17,BuildDone=18);
scalar_enum!(enc_ExhaustReason,dec_ExhaustReason,ExhaustReason;Fuel=0,DataNodes=1,DataFields=2,EnvironmentCells=3,HelperTasks=4,HelperWork=5,SerializationBytes=6,LogicalCap=7);
scalar_enum!(enc_OutcomeTagC,dec_OutcomeTagC,OutcomeTagC;Running=0,Halted=1,Rejected=2,Exhausted=3,Waiting=4);
scalar_enum!(enc_BoundaryKind,dec_BoundaryKind,BoundaryKind;None=0,Terminal=1,TechnicalCut=2,SourceSuspensionExtensionReserved=3);
fn phase(e: &mut Enc, p: HelperPhaseSummaryC) -> Result<(), EventCodecError> {
    match p {
        HelperPhaseSummaryC::Scalar(v) => {
            e.put(0)?;
            e.put(enc_ScalarPhase(v))
        }
        HelperPhaseSummaryC::Tri(v) => {
            e.put(1)?;
            e.put(enc_TriStatus(v))
        }
        HelperPhaseSummaryC::Bind(v) => {
            e.put(2)?;
            e.put(enc_BindPhase(v))
        }
        HelperPhaseSummaryC::Append(v) => {
            e.put(3)?;
            e.put(enc_AppendPhase(v))
        }
        HelperPhaseSummaryC::Match(v) => {
            e.put(4)?;
            e.put(enc_MatchPhase(v))
        }
        HelperPhaseSummaryC::Build(v) => {
            e.put(5)?;
            e.put(enc_BuildPhase(v))
        }
        HelperPhaseSummaryC::Plan(v) => {
            e.put(6)?;
            e.put(enc_PlanPhase(v))
        }
    }
}
fn read_phase(d: &mut Dec<'_>) -> Result<HelperPhaseSummaryC, EventCodecError> {
    Ok(match d.get()? {
        0 => HelperPhaseSummaryC::Scalar(dec_ScalarPhase(d.get()?)?),
        1 => HelperPhaseSummaryC::Tri(dec_TriStatus(d.get()?)?),
        2 => HelperPhaseSummaryC::Bind(dec_BindPhase(d.get()?)?),
        3 => HelperPhaseSummaryC::Append(dec_AppendPhase(d.get()?)?),
        4 => HelperPhaseSummaryC::Match(dec_MatchPhase(d.get()?)?),
        5 => HelperPhaseSummaryC::Build(dec_BuildPhase(d.get()?)?),
        6 => HelperPhaseSummaryC::Plan(dec_PlanPhase(d.get()?)?),
        _ => return Err(EventCodecError::Invalid),
    })
}
fn outcome(e: &mut Enc, v: OutcomeC) -> Result<(), EventCodecError> {
    match v {
        OutcomeC::Running => e.put(0),
        OutcomeC::Halted { value } => {
            e.put(1)?;
            match value {
                RefC::Const(n) => {
                    e.put(0)?;
                    e.put(n as u128)
                }
                RefC::Data(n) => {
                    e.put(1)?;
                    e.put(n as u128)
                }
            }
        }
        OutcomeC::Rejected { code } => {
            e.put(2)?;
            e.put(code.into())
        }
        OutcomeC::Exhausted { reason } => {
            e.put(3)?;
            e.put(enc_ExhaustReason(reason))
        }
        OutcomeC::Waiting { site } => {
            e.put(4)?;
            e.put(site.into())
        }
    }
}
fn read_outcome(d: &mut Dec<'_>) -> Result<OutcomeC, EventCodecError> {
    Ok(match d.get()? {
        0 => OutcomeC::Running,
        1 => {
            let tag = d.get()?;
            let n = d.usize()?;
            OutcomeC::Halted {
                value: match tag {
                    0 => RefC::Const(n),
                    1 => RefC::Data(n),
                    _ => return Err(EventCodecError::Invalid),
                },
            }
        }
        2 => OutcomeC::Rejected { code: d.byte()? },
        3 => OutcomeC::Exhausted {
            reason: dec_ExhaustReason(d.get()?)?,
        },
        4 => OutcomeC::Waiting {
            site: u16::try_from(d.get()?).map_err(|_| EventCodecError::Invalid)?,
        },
        _ => return Err(EventCodecError::Invalid),
    })
}
/// All28 fields are destructured exhaustively. run_id is exact shared before-state data.
pub fn encode_event(
    event: &StepEventC,
    run_id: &RunIdC,
    max_lanes: usize,
) -> Result<Vec<u128>, EventCodecError> {
    let StepEventC {
        step_id,
        rule_id,
        event_kind,
        site,
        helper_kind,
        disposition,
        work_delta,
        helper_work_delta,
        helper_visits_delta,
        reference_cursor_delta,
        alloc_deltas,
        value_slot_writes,
        logical_depth_before,
        logical_depth_after,
        task_cursor_before,
        task_cursor_after,
        helper_phase_before,
        helper_phase_after,
        helper_cursor_before,
        helper_cursor_after,
        result_tag,
        origin_reference_cursor,
        source_expr_id,
        source_occurrence,
        exhausted_kind,
        outcome_tag,
        outcome_payload,
        boundary,
    } = event;
    if step_id.run_id.as_ref() != run_id {
        return Err(EventCodecError::Invalid);
    }
    let mut e = Enc {
        lanes: Vec::new(),
        max: max_lanes,
    };
    e.put(step_id.ordinal)?;
    e.put(enc_RuleIdC(*rule_id))?;
    e.put(enc_EventKind(*event_kind))?;
    e.opt(*site, |e, v| e.put(enc_SiteTag(v)))?;
    e.opt(*helper_kind, |e, v| e.put(enc_HelperKind(v)))?;
    e.put(enc_StepDisposition(*disposition))?;
    for v in [
        *work_delta,
        *helper_work_delta,
        *helper_visits_delta,
        *reference_cursor_delta,
    ] {
        e.put(v.into())?
    }
    let AllocDeltasC {
        data_nodes,
        data_fields,
        environment_cells,
    } = alloc_deltas;
    for v in [*data_nodes, *data_fields, *environment_cells] {
        e.signed(v)?
    }
    e.put((*value_slot_writes).into())?;
    for v in [
        *logical_depth_before,
        *logical_depth_after,
        *task_cursor_before,
        *task_cursor_after,
    ] {
        e.put(v as u128)?
    }
    e.opt(*helper_phase_before, phase)?;
    e.opt(*helper_phase_after, phase)?;
    e.opt(*helper_cursor_before, |e, v| e.put(v as u128))?;
    e.opt(*helper_cursor_after, |e, v| e.put(v as u128))?;
    e.opt(*result_tag, |e, v| e.put(enc_HelperResultTagC(v)))?;
    e.opt(*origin_reference_cursor, |e, v| e.put(v))?;
    e.put(*source_expr_id as u128)?;
    match source_occurrence {
        LocatorC::Occurrence {
            owner_is_function,
            function,
            ast_path,
        } => {
            e.put(0)?;
            e.put(u128::from(*owner_is_function))?;
            e.put(*function as u128)?;
            e.put(ast_path.len() as u128)?;
            for n in ast_path {
                e.put(*n as u128)?
            }
        }
        LocatorC::Table { table, row } => {
            e.put(1)?;
            e.bytes(table.as_bytes())?;
            e.put(*row as u128)?
        }
        LocatorC::Field { name } => {
            e.put(2)?;
            e.bytes(name.as_bytes())?
        }
    }
    e.opt(*exhausted_kind, |e, v| e.put(enc_ExhaustReason(v)))?;
    e.put(enc_OutcomeTagC(*outcome_tag))?;
    outcome(&mut e, *outcome_payload)?;
    e.put(enc_BoundaryKind(*boundary))?;
    Ok(e.lanes)
}
pub fn decode_event(lanes: &[u128], run_id: &RunIdC) -> Result<StepEventC, EventCodecError> {
    let mut d = Dec { lanes, at: 0 };
    let step_id = ExpandedStepIdC {
        run_id: Arc::new(run_id.clone()),
        ordinal: d.get()?,
    };
    let rule_id = dec_RuleIdC(d.get()?)?;
    let event_kind = dec_EventKind(d.get()?)?;
    let site = d.opt(|d| dec_SiteTag(d.get()?))?;
    let helper_kind = d.opt(|d| dec_HelperKind(d.get()?))?;
    let disposition = dec_StepDisposition(d.get()?)?;
    let work_delta = d.byte()?;
    let helper_work_delta = d.byte()?;
    let helper_visits_delta = d.byte()?;
    let reference_cursor_delta = d.byte()?;
    let alloc_deltas = AllocDeltasC {
        data_nodes: d.signed()?,
        data_fields: d.signed()?,
        environment_cells: d.signed()?,
    };
    let value_slot_writes = d.byte()?;
    let logical_depth_before = d.usize()?;
    let logical_depth_after = d.usize()?;
    let task_cursor_before = d.usize()?;
    let task_cursor_after = d.usize()?;
    let helper_phase_before = d.opt(read_phase)?;
    let helper_phase_after = d.opt(read_phase)?;
    let helper_cursor_before = d.opt(Dec::usize)?;
    let helper_cursor_after = d.opt(Dec::usize)?;
    let result_tag = d.opt(|d| dec_HelperResultTagC(d.get()?))?;
    let origin_reference_cursor = d.opt(Dec::get)?;
    let source_expr_id = d.usize()?;
    let source_occurrence = match d.get()? {
        0 => {
            let owner_is_function = d.boolean()?;
            let function = d.usize()?;
            let n = d.usize()?;
            if n > d.lanes.len() - d.at {
                return Err(EventCodecError::Truncated);
            }
            let mut ast_path = Vec::with_capacity(n);
            for _ in 0..n {
                ast_path.push(d.usize()?)
            }
            LocatorC::Occurrence {
                owner_is_function,
                function,
                ast_path,
            }
        }
        1 => LocatorC::Table {
            table: d.string()?,
            row: d.usize()?,
        },
        2 => LocatorC::Field { name: d.string()? },
        _ => return Err(EventCodecError::Invalid),
    };
    let exhausted_kind = d.opt(|d| dec_ExhaustReason(d.get()?))?;
    let outcome_tag = dec_OutcomeTagC(d.get()?)?;
    let outcome_payload = read_outcome(&mut d)?;
    let boundary = dec_BoundaryKind(d.get()?)?;
    if d.at != lanes.len() {
        return Err(EventCodecError::Trailing);
    }
    Ok(StepEventC {
        step_id,
        rule_id,
        event_kind,
        site,
        helper_kind,
        disposition,
        work_delta,
        helper_work_delta,
        helper_visits_delta,
        reference_cursor_delta,
        alloc_deltas,
        value_slot_writes,
        logical_depth_before,
        logical_depth_after,
        task_cursor_before,
        task_cursor_after,
        helper_phase_before,
        helper_phase_after,
        helper_cursor_before,
        helper_cursor_after,
        result_tag,
        origin_reference_cursor,
        source_expr_id,
        source_occurrence,
        exhausted_kind,
        outcome_tag,
        outcome_payload,
        boundary,
    })
}
