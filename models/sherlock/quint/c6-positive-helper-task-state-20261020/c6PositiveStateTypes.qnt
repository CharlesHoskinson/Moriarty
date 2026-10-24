module c6PositiveStateTypes {

  // Positive owned-record vocabulary only. No admission or malformed-lane policy.
  type C6Ref = C6Const(int) | C6Data(int)
  type C6Env = C6EnvNone | C6EnvSome(int)
  type C6Function = C6FunctionNone | C6FunctionSome(int)
  type C6Slice = {start:int,len:int}
  type C6Binary = C6Add | C6Sub | C6Mul | C6Eq | C6Lt | C6Cons | C6Branch
  type C6Unary = C6Leaf | C6Not
  type C6Frame =
    | C6LetF({binder:int,body:int,saved_env:C6Env,origin:int})
    | C6LeftF({op:C6Binary,right:int,saved_env:C6Env,origin:int})
    | C6RightF({op:C6Binary,left:C6Ref,origin:int})
    | C6UnaryF({op:C6Unary,origin:int})
    | C6IfF({then_expr:int,else_expr:int,saved_env:C6Env,origin:int})
    | C6MatchF({arms:C6Slice,saved_env:C6Env,origin:int})
    | C6ArgsF({function_name:int,completed:C6Slice,rest:C6Slice,saved_env:C6Env,origin:int})
    | C6ReturnF({return_type:int,saved_env:C6Env,origin:int})
    | C6ScopeF({saved_env:C6Env,origin:int})
    | C6ConstructorFieldsF({type_id:int,ctor_id:int,index:int,field_type:int,
        checked:bool,completed:C6Slice,rest:C6Slice,saved_env:C6Env,origin:int})
  type C6Binding = C6BindingBad | C6BindingOk(C6Env)
  type C6Matching = C6NoMatch | C6Matched(C6Env)
  type C6Primitive = C6PrimitiveValue(C6Ref) | C6PrimitiveError(int)
  type C6Control =
    | C6Eval({expr:int})
    | C6Return({value:C6Ref})
    | C6ScheduleNext({frame:C6Frame,next:int,env:C6Env})
    | C6Invoke({function_name:int,args:C6Slice})
    | C6Enter({function:C6Function,args:C6Slice})
    | C6Bound({binding:C6Binding,return_type:int,body:int})
    | C6Cases({value:C6Ref,arms:C6Slice,saved_env:C6Env})
    | C6CaseResult({matching:C6Matching,value:C6Ref,body:int,rest:C6Slice,saved_env:C6Env})
    | C6PrimResult({result:C6Primitive})
    | C6Error({code:int})
    | C6Materialize({type_id:int,ctor_id:int,fields:C6Slice})
  type C6Exhaust = C6Fuel | C6DataNodes | C6DataFields | C6EnvironmentCells
    | C6HelperTasks | C6HelperWork | C6SerializationBytes
  type C6Outcome = C6Running | C6Halted(C6Ref) | C6Rejected(int) | C6Exhausted(C6Exhaust)
  type C6Alloc = {data_nodes:int,data_fields:int,environment_cells:int}
  type C6Active = {data:int,data_fields:int,env:int,value_slots:int}
  type C6Section = {control:C6Control,control_origin:int,env_root:C6Env,stack_limit:int,
    outcome:C6Outcome,initial_fuel:int,remaining_work:int,consumed_work:int,
    helper_work:int,helper_visits:int,reference_cursor:int,alloc:C6Alloc,active:C6Active,
    seg_active:C6Active,seg_alloc:C6Alloc,seg_consumed_work:int,frames:List[C6Frame]}
  // Complete positive helper/result/task/state carriers; declarations only.
  type C6HIndex = C6HNoIndex | C6HSomeIndex(int)
  type C6HType = C6HNoType | C6HSomeType(int)
  type C6HOptRef = C6HNoRef | C6HSomeRef(C6Ref)
  type C6HSiteTag = C6HSiteTagEvalLiteral | C6HSiteTagEvalVar | C6HSiteTagEvalLet | C6HSiteTagEvalBinary | C6HSiteTagEvalUnary | C6HSiteTagEvalIf | C6HSiteTagEvalMatch | C6HSiteTagEvalCall | C6HSiteTagPush | C6HSiteTagRetLet | C6HSiteTagRetLeft | C6HSiteTagRetRight | C6HSiteTagRetUnary | C6HSiteTagRetIf | C6HSiteTagRetMatch | C6HSiteTagRetArgs | C6HSiteTagRetCall | C6HSiteTagRetScope | C6HSiteTagInvoke | C6HSiteTagEnter | C6HSiteTagBound | C6HSiteTagCases | C6HSiteTagCaseResult | C6HSiteTagPrim | C6HSiteTagEvalConstruct | C6HSiteTagRetCtorFields | C6HSiteTagMaterialize
  type C6HScalarMode = C6HScalarModeAdd | C6HScalarModeSub | C6HScalarModeMul | C6HScalarModeLt | C6HScalarModeCons | C6HScalarModeBranch | C6HScalarModeLeaf | C6HScalarModeNot | C6HScalarModeTestBool | C6HScalarModeTestType
  type C6HScalarPhase = C6HScalarPhaseFirst | C6HScalarPhaseSecond | C6HScalarPhaseDone
  type C6HNodeKind = C6HNodeKindCons | C6HNodeKindLeaf | C6HNodeKindBranch
  type C6HBoolSelector = C6HBoolSelectorTrue | C6HBoolSelectorFalse | C6HBoolSelectorNotBool
  type C6HTriStatus = C6HTriStatusPending | C6HTriStatusPositive | C6HTriStatusNegative
  type C6HBindPhase = C6HBindPhasePairing | C6HBindPhaseUnwindType | C6HBindPhaseUnwindDup | C6HBindPhaseUnwindInsert | C6HBindPhasePropagate | C6HBindPhaseFinished
  type C6HBindStatus = C6HBindStatusOk | C6HBindStatusBad
  type C6HAppendPhase = C6HAppendPhaseCopy | C6HAppendPhaseTail | C6HAppendPhaseDone
  type C6HMatchPhase = C6HMatchPhaseArm | C6HMatchPhaseTag | C6HMatchPhaseDistinct | C6HMatchPhaseField | C6HMatchPhaseBind | C6HMatchPhaseReady | C6HMatchPhaseNoMatchReady | C6HMatchPhaseDone
  type C6HPlanPhase = C6HPlanPhaseWalk | C6HPlanPhaseTest | C6HPlanPhaseBind | C6HPlanPhaseReady | C6HPlanPhaseNoMatchReady | C6HPlanPhaseDone
  type C6HPlanAfterWalk = C6HPlanAfterWalkTest | C6HPlanAfterWalkBind
  type C6HBuildPhase = C6HBuildPhaseCopy | C6HBuildPhaseDone
  type C6HPatternTag = C6HPatternTagPVar | C6HPatternTagPNil | C6HPatternTagPCons | C6HPatternTagPLeaf | C6HPatternTagPBranch | C6HPatternTagWildcardExtensionReserved | C6HPatternTagConstructorExtensionReserved
  type C6HTaskVisitC = C6HTaskVisitCUnvisited | C6HTaskVisitCFields
  type C6HCommon = {site:C6HSiteTag,origin_reference_cursor:int,parent_resume:C6HIndex}
  type C6HStaged = C6HStagedNone | C6HStagedU128Value({n:int})
    | C6HStagedBoolValue({b:bool}) | C6HStagedPendingNode({tag:C6HNodeKind})
    | C6HStagedPrimError({code:int}) | C6HStagedSelector({sel:C6HBoolSelector})
    | C6HStagedTypeTest({ok:bool})
  type C6HHelper =
    | C6HScalar({common:C6HCommon,mode:C6HScalarMode,operand_a:C6Ref,operand_b:C6HOptRef,expected:C6HType,phase:C6HScalarPhase,reg_a:int,staged:C6HStaged})
    | C6HEnvGet({common:C6HCommon,binder:int,cursor:C6HIndex,found:C6HOptRef,status:C6HTriStatus})
    | C6HEnvPut({common:C6HCommon,binder:int,value:C6Ref,base:C6Env,new_root:C6HIndex,status:C6HTriStatus})
    | C6HFind({common:C6HCommon,function_name:int,row_cursor:int,found:C6HIndex,status:C6HTriStatus})
    | C6HBind({common:C6HCommon,function:int,params:C6Slice,args:C6Slice,p_cursor:int,a_cursor:int,partial:C6Env,dup_cursor:C6HIndex,phase:C6HBindPhase,status:C6HBindStatus})
    | C6HAppend({common:C6HCommon,src:C6Slice,value:C6Ref,src_cursor:int,dest_start:int,staged:C6HOptRef,phase:C6HAppendPhase})
    | C6HValidate({common:C6HCommon,root:C6Ref,status:C6HTriStatus})
    | C6HEqual({common:C6HCommon,left:C6Ref,right:C6Ref,status:C6HTriStatus})
    | C6HMatch({common:C6HCommon,scrutinee:C6Ref,arm:int,saved_env:C6Env,partial:C6Env,pat_tag:C6HPatternTag,x:int,y:int,field_idx:int,staged:C6HOptRef,phase:C6HMatchPhase,status:C6HTriStatus})
    | C6HPlan({common:C6HCommon,source_match:int,scrutinee:C6Ref,saved_env:C6Env,partial:C6Env,node:int,phase:C6HPlanPhase,after_walk:C6HPlanAfterWalk,cur:C6Ref,path:C6Slice,path_cursor:int,arm:C6HIndex,status:C6HTriStatus})
    | C6HBuild({common:C6HCommon,type_id:int,ctor_id:int,src:C6Slice,src_cursor:int,field_start:int,staged:C6HOptRef,node:C6HOptRef,phase:C6HBuildPhase})
  type C6HResult =
    | C6HResultValidated({ok:bool})
    | C6HResultEnvFound({value:C6Ref})
    | C6HResultEnvMissing
    | C6HResultEnvUpdated({cell:int})
    | C6HResultFindFound({function:int})
    | C6HResultFindMissing
    | C6HResultBindOk({env:C6Env})
    | C6HResultBindBad
    | C6HResultAppendDone({seq:C6Slice})
    | C6HResultScalarValue({value:C6Ref})
    | C6HResultScalarError({code:int})
    | C6HResultScalarSelector({sel:C6HBoolSelector})
    | C6HResultScalarTypeTest({ok:bool})
    | C6HResultEqualDone({value:C6Ref})
    | C6HResultMatchMatched({env:C6Env})
    | C6HResultMatchNoMatch
    | C6HResultPlanMatched({arm:int,env:C6Env})
    | C6HResultPlanNoMatch
    | C6HResultBuildDone({value:C6Ref})
  type C6HTask =
    | C6HValidateTask({r:C6Ref,visit:C6HTaskVisitC,cursor:int})
    | C6HEqualTask({left:C6Ref,right:C6Ref,visit:C6HTaskVisitC,cursor:int})
    | C6HPairTask({binder:int,sort:int,arg:C6Ref})
  type C6HOptHelper = C6HNoHelper | C6HSomeHelper(C6HHelper)
  type C6HOptResult = C6HNoResult | C6HSomeResult(C6HResult)
  type C6HDataRow = C6HU128({n:int}) | C6HBool({b:bool}) | C6HNil
    | C6HCons({fields:C6Slice}) | C6HLeaf({fields:C6Slice}) | C6HBranch({fields:C6Slice})
    | C6HConstructor({type_id:int,ctor_id:int,fields:C6Slice})
  type C6HEnvCell = {binder:int,value:C6Ref,previous:C6Env}
  type C6HDataTables = {data:List[C6HDataRow],data_fields:List[C6Ref],env:List[C6HEnvCell],value_slots:List[C6Ref]}
  type C6HHelperTables = {helper:C6HOptHelper,result:C6HOptResult,tasks:List[C6HTask]}
  type C6HRoot = {control:C6Control,control_origin:int,env_root:C6Env,stack_limit:int,
    outcome:C6Outcome,initial_fuel:int,remaining_work:int,consumed_work:int,
    helper_work:int,helper_visits:int,reference_cursor:int,alloc:C6Alloc,active:C6Active,
    seg_active:C6Active,seg_alloc:C6Alloc,seg_consumed_work:int}
  type C6HState = {root:C6HRoot,frames:List[C6Frame],data:C6HDataTables,helper:C6HHelperTables}
  type C6HCaps = {max_frames:int,max_expressions:int,max_names:int,max_arms:int,max_children:int,max_value_slots:int,max_constants:int,max_data_nodes:int,max_env_cells:int,max_functions:int,max_arity:int,max_types:int,max_occurrences:int,max_occurrence_paths:int,max_parameters:int,max_patterns:int,max_pattern_children:int,max_constructors:int,max_ctor_fields:int,max_decl_templates:int,max_plan_nodes:int,max_plan_cases:int,max_field_paths:int,max_constant_fields:int,max_name_bytes:int,max_type_expr_nodes:int,max_template_bytes:int,max_data_field_slots:int,max_helper_tasks:int}
  type C6HLimits = {max_total_lanes:int}
  type C6HContext = {caps:C6HCaps,ext_present:bool,limits:C6HLimits}
  type C6HHelperRecordCase = {case_id:str,helper:C6HOptHelper}
  type C6HResultRecordCase = {case_id:str,result:C6HOptResult}
  type C6HTaskRecordCase = {case_id:str,task:C6HTask}
  type C6HHelperSectionCase = {case_id:str,tables:C6HHelperTables}
  type C6HStateCase = {case_id:str,context:C6HContext,state:C6HState}
  type C6HInputs = {schema:str,profile:str,caps:C6HCaps,helper_record_cases:List[C6HHelperRecordCase],result_record_cases:List[C6HResultRecordCase],task_record_cases:List[C6HTaskRecordCase],helper_section_cases:List[C6HHelperSectionCase],state_cases:List[C6HStateCase]}
  type C6HHelperRecordObservation = {encoded_lanes:List[int],decoded_helper:C6HOptHelper}
  type C6HResultRecordObservation = {encoded_lanes:List[int],decoded_result:C6HOptResult}
  type C6HTaskRecordObservation = {encoded_lanes:List[int],decoded_task:C6HTask}
  type C6HTaskSpan = {index:int,start:int,length:int,end:int,lanes:List[int]}
  type C6HHelperSectionObservation = {encoded_lanes:List[int],decoded_tables:C6HHelperTables,task_spans:List[C6HTaskSpan],final_cursor:int}
  type C6HChildSection = {name:str,tag:int,start:int,length:int,end:int,lanes:List[int]}
  type C6HStateObservation = {encoded_lanes:List[int],decoded_state:C6HState,retained_context:C6HContext,capacity_descriptor:List[int],child_sections:List[C6HChildSection],final_cursor:int}
  type C6HCaseIds = {helper_record_cases:List[str],result_record_cases:List[str],task_record_cases:List[str],helper_section_cases:List[str],state_cases:List[str]}
  type C6HExpectedRoot = {schema:str,profile:str,case_ids:C6HCaseIds,helper_record_observations:List[C6HHelperRecordObservation],result_record_observations:List[C6HResultRecordObservation],task_record_observations:List[C6HTaskRecordObservation],helper_section_observations:List[C6HHelperSectionObservation],state_observations:List[C6HStateObservation]}
}
