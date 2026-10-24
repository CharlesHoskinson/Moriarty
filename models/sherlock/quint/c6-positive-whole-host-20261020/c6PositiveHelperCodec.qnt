module c6PositiveHelperCodec {
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import c6PositiveEncoding.* from "./c6PositiveEncoding"
  import c6PositiveDecoding.* from "./c6PositiveDecoding"
  // Positive canonical code-image functions only, NOT validators.
  // All widths/tags/padding/kinds/caps/native fits are frozen external premises.
  // Off-image interpretation is unspecified; no refusal/security policy.
  pure def hEncodeSiteTag(x:C6HSiteTag):int = match x {
    | C6HSiteTagEvalLiteral => 0
    | C6HSiteTagEvalVar => 1
    | C6HSiteTagEvalLet => 2
    | C6HSiteTagEvalBinary => 3
    | C6HSiteTagEvalUnary => 4
    | C6HSiteTagEvalIf => 5
    | C6HSiteTagEvalMatch => 6
    | C6HSiteTagEvalCall => 7
    | C6HSiteTagPush => 8
    | C6HSiteTagRetLet => 9
    | C6HSiteTagRetLeft => 10
    | C6HSiteTagRetRight => 11
    | C6HSiteTagRetUnary => 12
    | C6HSiteTagRetIf => 13
    | C6HSiteTagRetMatch => 14
    | C6HSiteTagRetArgs => 15
    | C6HSiteTagRetCall => 16
    | C6HSiteTagRetScope => 17
    | C6HSiteTagInvoke => 18
    | C6HSiteTagEnter => 19
    | C6HSiteTagBound => 20
    | C6HSiteTagCases => 21
    | C6HSiteTagCaseResult => 22
    | C6HSiteTagPrim => 23
    | C6HSiteTagEvalConstruct => 24
    | C6HSiteTagRetCtorFields => 25
    | C6HSiteTagMaterialize => 26
  }
  pure def hDecodeSiteTag(t:int):C6HSiteTag =
    if(t == 0) C6HSiteTagEvalLiteral
    else if(t == 1) C6HSiteTagEvalVar
    else if(t == 2) C6HSiteTagEvalLet
    else if(t == 3) C6HSiteTagEvalBinary
    else if(t == 4) C6HSiteTagEvalUnary
    else if(t == 5) C6HSiteTagEvalIf
    else if(t == 6) C6HSiteTagEvalMatch
    else if(t == 7) C6HSiteTagEvalCall
    else if(t == 8) C6HSiteTagPush
    else if(t == 9) C6HSiteTagRetLet
    else if(t == 10) C6HSiteTagRetLeft
    else if(t == 11) C6HSiteTagRetRight
    else if(t == 12) C6HSiteTagRetUnary
    else if(t == 13) C6HSiteTagRetIf
    else if(t == 14) C6HSiteTagRetMatch
    else if(t == 15) C6HSiteTagRetArgs
    else if(t == 16) C6HSiteTagRetCall
    else if(t == 17) C6HSiteTagRetScope
    else if(t == 18) C6HSiteTagInvoke
    else if(t == 19) C6HSiteTagEnter
    else if(t == 20) C6HSiteTagBound
    else if(t == 21) C6HSiteTagCases
    else if(t == 22) C6HSiteTagCaseResult
    else if(t == 23) C6HSiteTagPrim
    else if(t == 24) C6HSiteTagEvalConstruct
    else if(t == 25) C6HSiteTagRetCtorFields
    else C6HSiteTagMaterialize
  pure def hEncodeScalarMode(x:C6HScalarMode):int = match x {
    | C6HScalarModeAdd => 0
    | C6HScalarModeSub => 1
    | C6HScalarModeMul => 2
    | C6HScalarModeLt => 3
    | C6HScalarModeCons => 4
    | C6HScalarModeBranch => 5
    | C6HScalarModeLeaf => 6
    | C6HScalarModeNot => 7
    | C6HScalarModeTestBool => 8
    | C6HScalarModeTestType => 9
  }
  pure def hDecodeScalarMode(t:int):C6HScalarMode =
    if(t == 0) C6HScalarModeAdd
    else if(t == 1) C6HScalarModeSub
    else if(t == 2) C6HScalarModeMul
    else if(t == 3) C6HScalarModeLt
    else if(t == 4) C6HScalarModeCons
    else if(t == 5) C6HScalarModeBranch
    else if(t == 6) C6HScalarModeLeaf
    else if(t == 7) C6HScalarModeNot
    else if(t == 8) C6HScalarModeTestBool
    else C6HScalarModeTestType
  pure def hEncodeScalarPhase(x:C6HScalarPhase):int = match x {
    | C6HScalarPhaseFirst => 0
    | C6HScalarPhaseSecond => 1
    | C6HScalarPhaseDone => 2
  }
  pure def hDecodeScalarPhase(t:int):C6HScalarPhase =
    if(t == 0) C6HScalarPhaseFirst
    else if(t == 1) C6HScalarPhaseSecond
    else C6HScalarPhaseDone
  pure def hEncodeNodeKind(x:C6HNodeKind):int = match x {
    | C6HNodeKindCons => 0
    | C6HNodeKindLeaf => 1
    | C6HNodeKindBranch => 2
  }
  pure def hDecodeNodeKind(t:int):C6HNodeKind =
    if(t == 0) C6HNodeKindCons
    else if(t == 1) C6HNodeKindLeaf
    else C6HNodeKindBranch
  pure def hEncodeBoolSelector(x:C6HBoolSelector):int = match x {
    | C6HBoolSelectorTrue => 0
    | C6HBoolSelectorFalse => 1
    | C6HBoolSelectorNotBool => 2
  }
  pure def hDecodeBoolSelector(t:int):C6HBoolSelector =
    if(t == 0) C6HBoolSelectorTrue
    else if(t == 1) C6HBoolSelectorFalse
    else C6HBoolSelectorNotBool
  pure def hEncodeTriStatus(x:C6HTriStatus):int = match x {
    | C6HTriStatusPending => 0
    | C6HTriStatusPositive => 1
    | C6HTriStatusNegative => 2
  }
  pure def hDecodeTriStatus(t:int):C6HTriStatus =
    if(t == 0) C6HTriStatusPending
    else if(t == 1) C6HTriStatusPositive
    else C6HTriStatusNegative
  pure def hEncodeBindPhase(x:C6HBindPhase):int = match x {
    | C6HBindPhasePairing => 0
    | C6HBindPhaseUnwindType => 1
    | C6HBindPhaseUnwindDup => 2
    | C6HBindPhaseUnwindInsert => 3
    | C6HBindPhasePropagate => 4
    | C6HBindPhaseFinished => 5
  }
  pure def hDecodeBindPhase(t:int):C6HBindPhase =
    if(t == 0) C6HBindPhasePairing
    else if(t == 1) C6HBindPhaseUnwindType
    else if(t == 2) C6HBindPhaseUnwindDup
    else if(t == 3) C6HBindPhaseUnwindInsert
    else if(t == 4) C6HBindPhasePropagate
    else C6HBindPhaseFinished
  pure def hEncodeBindStatus(x:C6HBindStatus):int = match x {
    | C6HBindStatusOk => 0
    | C6HBindStatusBad => 1
  }
  pure def hDecodeBindStatus(t:int):C6HBindStatus =
    if(t == 0) C6HBindStatusOk
    else C6HBindStatusBad
  pure def hEncodeAppendPhase(x:C6HAppendPhase):int = match x {
    | C6HAppendPhaseCopy => 0
    | C6HAppendPhaseTail => 1
    | C6HAppendPhaseDone => 2
  }
  pure def hDecodeAppendPhase(t:int):C6HAppendPhase =
    if(t == 0) C6HAppendPhaseCopy
    else if(t == 1) C6HAppendPhaseTail
    else C6HAppendPhaseDone
  pure def hEncodeMatchPhase(x:C6HMatchPhase):int = match x {
    | C6HMatchPhaseArm => 0
    | C6HMatchPhaseTag => 1
    | C6HMatchPhaseDistinct => 2
    | C6HMatchPhaseField => 3
    | C6HMatchPhaseBind => 4
    | C6HMatchPhaseReady => 5
    | C6HMatchPhaseNoMatchReady => 6
    | C6HMatchPhaseDone => 7
  }
  pure def hDecodeMatchPhase(t:int):C6HMatchPhase =
    if(t == 0) C6HMatchPhaseArm
    else if(t == 1) C6HMatchPhaseTag
    else if(t == 2) C6HMatchPhaseDistinct
    else if(t == 3) C6HMatchPhaseField
    else if(t == 4) C6HMatchPhaseBind
    else if(t == 5) C6HMatchPhaseReady
    else if(t == 6) C6HMatchPhaseNoMatchReady
    else C6HMatchPhaseDone
  pure def hEncodePlanPhase(x:C6HPlanPhase):int = match x {
    | C6HPlanPhaseWalk => 0
    | C6HPlanPhaseTest => 1
    | C6HPlanPhaseBind => 2
    | C6HPlanPhaseReady => 3
    | C6HPlanPhaseNoMatchReady => 4
    | C6HPlanPhaseDone => 5
  }
  pure def hDecodePlanPhase(t:int):C6HPlanPhase =
    if(t == 0) C6HPlanPhaseWalk
    else if(t == 1) C6HPlanPhaseTest
    else if(t == 2) C6HPlanPhaseBind
    else if(t == 3) C6HPlanPhaseReady
    else if(t == 4) C6HPlanPhaseNoMatchReady
    else C6HPlanPhaseDone
  pure def hEncodePlanAfterWalk(x:C6HPlanAfterWalk):int = match x {
    | C6HPlanAfterWalkTest => 0
    | C6HPlanAfterWalkBind => 1
  }
  pure def hDecodePlanAfterWalk(t:int):C6HPlanAfterWalk =
    if(t == 0) C6HPlanAfterWalkTest
    else C6HPlanAfterWalkBind
  pure def hEncodeBuildPhase(x:C6HBuildPhase):int = match x {
    | C6HBuildPhaseCopy => 0
    | C6HBuildPhaseDone => 1
  }
  pure def hDecodeBuildPhase(t:int):C6HBuildPhase =
    if(t == 0) C6HBuildPhaseCopy
    else C6HBuildPhaseDone
  pure def hEncodePatternTag(x:C6HPatternTag):int = match x {
    | C6HPatternTagPVar => 0
    | C6HPatternTagPNil => 1
    | C6HPatternTagPCons => 2
    | C6HPatternTagPLeaf => 3
    | C6HPatternTagPBranch => 4
    | C6HPatternTagWildcardExtensionReserved => 5
    | C6HPatternTagConstructorExtensionReserved => 6
  }
  pure def hDecodePatternTag(t:int):C6HPatternTag =
    if(t == 0) C6HPatternTagPVar
    else if(t == 1) C6HPatternTagPNil
    else if(t == 2) C6HPatternTagPCons
    else if(t == 3) C6HPatternTagPLeaf
    else if(t == 4) C6HPatternTagPBranch
    else if(t == 5) C6HPatternTagWildcardExtensionReserved
    else C6HPatternTagConstructorExtensionReserved
  pure def hEncodeTaskVisitC(x:C6HTaskVisitC):int = match x {
    | C6HTaskVisitCUnvisited => 0
    | C6HTaskVisitCFields => 1
  }
  pure def hDecodeTaskVisitC(t:int):C6HTaskVisitC =
    if(t == 0) C6HTaskVisitCUnvisited
    else C6HTaskVisitCFields
  pure def hBool(b:bool):int = if(b) 1 else 0
  pure def hEncodeIndex(x:C6HIndex):List[int] = match x {
    | C6HNoIndex => List(0,0) | C6HSomeIndex(i) => List(1,i)
  }
  pure def hDecodeIndex(l:List[int],a:int):C6HIndex =
    if(l.nth(a) == 0) C6HNoIndex else C6HSomeIndex(l.nth(a+1))
  pure def hEncodeType(x:C6HType):List[int] = match x {
    | C6HNoType => List(0,0) | C6HSomeType(i) => List(1,i)
  }
  pure def hDecodeType(l:List[int],a:int):C6HType =
    if(l.nth(a) == 0) C6HNoType else C6HSomeType(l.nth(a+1))
  pure def hEncodeOptRef(x:C6HOptRef):List[int] = match x {
    | C6HNoRef => List(0,0,0) | C6HSomeRef(r) => List(1).concat(c6EncodeRef(r))
  }
  pure def hDecodeOptRef(l:List[int],a:int):C6HOptRef =
    if(l.nth(a) == 0) C6HNoRef else C6HSomeRef(c6DecodePositiveRef(l,a+1))
  pure def hEncodeCommon(x:C6HCommon):List[int] =
    List(hEncodeSiteTag(x.site),x.origin_reference_cursor).concat(hEncodeIndex(x.parent_resume))
  pure def hDecodeCommon(l:List[int],a:int):C6HCommon =
    {site:hDecodeSiteTag(l.nth(a)),origin_reference_cursor:l.nth(a+1),parent_resume:hDecodeIndex(l,a+2)}
  pure def hEncodeStaged(x:C6HStaged):List[int] = match x {
    | C6HStagedNone => List(0,0)
    | C6HStagedU128Value(p) => List(1,p.n)
    | C6HStagedBoolValue(p) => List(2,hBool(p.b))
    | C6HStagedPendingNode(p) => List(3,hEncodeNodeKind(p.tag))
    | C6HStagedPrimError(p) => List(4,p.code)
    | C6HStagedSelector(p) => List(5,hEncodeBoolSelector(p.sel))
    | C6HStagedTypeTest(p) => List(6,hBool(p.ok))
  }
  pure def hDecodeStaged(l:List[int],a:int):C6HStaged =
    if(l.nth(a) == 0) C6HStagedNone
    else if(l.nth(a) == 1) C6HStagedU128Value({n:l.nth(a+1)})
    else if(l.nth(a) == 2) C6HStagedBoolValue({b:l.nth(a+1) == 1})
    else if(l.nth(a) == 3) C6HStagedPendingNode({tag:hDecodeNodeKind(l.nth(a+1))})
    else if(l.nth(a) == 4) C6HStagedPrimError({code:l.nth(a+1)})
    else if(l.nth(a) == 5) C6HStagedSelector({sel:hDecodeBoolSelector(l.nth(a+1))})
    else C6HStagedTypeTest({ok:l.nth(a+1) == 1})
  pure def hEncodeHelperValue(x:C6HHelper):List[int] = {
    val used = match x {
      | C6HScalar(p) => List(1).concat(hEncodeCommon(p.common)).concat(List(hEncodeScalarMode(p.mode))).concat(c6EncodeRef(p.operand_a)).concat(hEncodeOptRef(p.operand_b)).concat(hEncodeType(p.expected)).concat(List(hEncodeScalarPhase(p.phase))).concat(List(p.reg_a)).concat(hEncodeStaged(p.staged))
      | C6HEnvGet(p) => List(2).concat(hEncodeCommon(p.common)).concat(List(p.binder)).concat(hEncodeIndex(p.cursor)).concat(hEncodeOptRef(p.found)).concat(List(hEncodeTriStatus(p.status)))
      | C6HEnvPut(p) => List(3).concat(hEncodeCommon(p.common)).concat(List(p.binder)).concat(c6EncodeRef(p.value)).concat(c6EncodeEnv(p.base)).concat(hEncodeIndex(p.new_root)).concat(List(hEncodeTriStatus(p.status)))
      | C6HFind(p) => List(4).concat(hEncodeCommon(p.common)).concat(List(p.function_name)).concat(List(p.row_cursor)).concat(hEncodeIndex(p.found)).concat(List(hEncodeTriStatus(p.status)))
      | C6HBind(p) => List(5).concat(hEncodeCommon(p.common)).concat(List(p.function)).concat(c6EncodeSlice(p.params)).concat(c6EncodeSlice(p.args)).concat(List(p.p_cursor)).concat(List(p.a_cursor)).concat(c6EncodeEnv(p.partial)).concat(hEncodeIndex(p.dup_cursor)).concat(List(hEncodeBindPhase(p.phase))).concat(List(hEncodeBindStatus(p.status)))
      | C6HAppend(p) => List(6).concat(hEncodeCommon(p.common)).concat(c6EncodeSlice(p.src)).concat(c6EncodeRef(p.value)).concat(List(p.src_cursor)).concat(List(p.dest_start)).concat(hEncodeOptRef(p.staged)).concat(List(hEncodeAppendPhase(p.phase)))
      | C6HValidate(p) => List(7).concat(hEncodeCommon(p.common)).concat(c6EncodeRef(p.root)).concat(List(hEncodeTriStatus(p.status)))
      | C6HEqual(p) => List(8).concat(hEncodeCommon(p.common)).concat(c6EncodeRef(p.left)).concat(c6EncodeRef(p.right)).concat(List(hEncodeTriStatus(p.status)))
      | C6HMatch(p) => List(9).concat(hEncodeCommon(p.common)).concat(c6EncodeRef(p.scrutinee)).concat(List(p.arm)).concat(c6EncodeEnv(p.saved_env)).concat(c6EncodeEnv(p.partial)).concat(List(hEncodePatternTag(p.pat_tag))).concat(List(p.x)).concat(List(p.y)).concat(List(p.field_idx)).concat(hEncodeOptRef(p.staged)).concat(List(hEncodeMatchPhase(p.phase))).concat(List(hEncodeTriStatus(p.status)))
      | C6HPlan(p) => List(10).concat(hEncodeCommon(p.common)).concat(List(p.source_match)).concat(c6EncodeRef(p.scrutinee)).concat(c6EncodeEnv(p.saved_env)).concat(c6EncodeEnv(p.partial)).concat(List(p.node)).concat(List(hEncodePlanPhase(p.phase))).concat(List(hEncodePlanAfterWalk(p.after_walk))).concat(c6EncodeRef(p.cur)).concat(c6EncodeSlice(p.path)).concat(List(p.path_cursor)).concat(hEncodeIndex(p.arm)).concat(List(hEncodeTriStatus(p.status)))
      | C6HBuild(p) => List(11).concat(hEncodeCommon(p.common)).concat(List(p.type_id)).concat(List(p.ctor_id)).concat(c6EncodeSlice(p.src)).concat(List(p.src_cursor)).concat(List(p.field_start)).concat(hEncodeOptRef(p.staged)).concat(hEncodeOptRef(p.node)).concat(List(hEncodeBuildPhase(p.phase)))
    }
    c6Pad(used,23)
  }
  pure def hEncodeHelper(x:C6HOptHelper):List[int] = match x {
    | C6HNoHelper => c6Pad(List(),23)
    | C6HSomeHelper(v) => hEncodeHelperValue(v)
  }
  pure def hDecodeHelper(l:List[int]):C6HOptHelper =
    if(l.nth(0) == 0) C6HNoHelper
    else if(l.nth(0) == 1) C6HSomeHelper(C6HScalar({common:hDecodeCommon(l,1),mode:hDecodeScalarMode(l.nth(5)),operand_a:c6DecodePositiveRef(l,6),operand_b:hDecodeOptRef(l,8),expected:hDecodeType(l,11),phase:hDecodeScalarPhase(l.nth(13)),reg_a:l.nth(14),staged:hDecodeStaged(l,15)}))
    else if(l.nth(0) == 2) C6HSomeHelper(C6HEnvGet({common:hDecodeCommon(l,1),binder:l.nth(5),cursor:hDecodeIndex(l,6),found:hDecodeOptRef(l,8),status:hDecodeTriStatus(l.nth(11))}))
    else if(l.nth(0) == 3) C6HSomeHelper(C6HEnvPut({common:hDecodeCommon(l,1),binder:l.nth(5),value:c6DecodePositiveRef(l,6),base:c6DecodePositiveEnv(l,8),new_root:hDecodeIndex(l,10),status:hDecodeTriStatus(l.nth(12))}))
    else if(l.nth(0) == 4) C6HSomeHelper(C6HFind({common:hDecodeCommon(l,1),function_name:l.nth(5),row_cursor:l.nth(6),found:hDecodeIndex(l,7),status:hDecodeTriStatus(l.nth(9))}))
    else if(l.nth(0) == 5) C6HSomeHelper(C6HBind({common:hDecodeCommon(l,1),function:l.nth(5),params:c6DecodePositiveSlice(l,6),args:c6DecodePositiveSlice(l,8),p_cursor:l.nth(10),a_cursor:l.nth(11),partial:c6DecodePositiveEnv(l,12),dup_cursor:hDecodeIndex(l,14),phase:hDecodeBindPhase(l.nth(16)),status:hDecodeBindStatus(l.nth(17))}))
    else if(l.nth(0) == 6) C6HSomeHelper(C6HAppend({common:hDecodeCommon(l,1),src:c6DecodePositiveSlice(l,5),value:c6DecodePositiveRef(l,7),src_cursor:l.nth(9),dest_start:l.nth(10),staged:hDecodeOptRef(l,11),phase:hDecodeAppendPhase(l.nth(14))}))
    else if(l.nth(0) == 7) C6HSomeHelper(C6HValidate({common:hDecodeCommon(l,1),root:c6DecodePositiveRef(l,5),status:hDecodeTriStatus(l.nth(7))}))
    else if(l.nth(0) == 8) C6HSomeHelper(C6HEqual({common:hDecodeCommon(l,1),left:c6DecodePositiveRef(l,5),right:c6DecodePositiveRef(l,7),status:hDecodeTriStatus(l.nth(9))}))
    else if(l.nth(0) == 9) C6HSomeHelper(C6HMatch({common:hDecodeCommon(l,1),scrutinee:c6DecodePositiveRef(l,5),arm:l.nth(7),saved_env:c6DecodePositiveEnv(l,8),partial:c6DecodePositiveEnv(l,10),pat_tag:hDecodePatternTag(l.nth(12)),x:l.nth(13),y:l.nth(14),field_idx:l.nth(15),staged:hDecodeOptRef(l,16),phase:hDecodeMatchPhase(l.nth(19)),status:hDecodeTriStatus(l.nth(20))}))
    else if(l.nth(0) == 10) C6HSomeHelper(C6HPlan({common:hDecodeCommon(l,1),source_match:l.nth(5),scrutinee:c6DecodePositiveRef(l,6),saved_env:c6DecodePositiveEnv(l,8),partial:c6DecodePositiveEnv(l,10),node:l.nth(12),phase:hDecodePlanPhase(l.nth(13)),after_walk:hDecodePlanAfterWalk(l.nth(14)),cur:c6DecodePositiveRef(l,15),path:c6DecodePositiveSlice(l,17),path_cursor:l.nth(19),arm:hDecodeIndex(l,20),status:hDecodeTriStatus(l.nth(22))}))
    else C6HSomeHelper(C6HBuild({common:hDecodeCommon(l,1),type_id:l.nth(5),ctor_id:l.nth(6),src:c6DecodePositiveSlice(l,7),src_cursor:l.nth(9),field_start:l.nth(10),staged:hDecodeOptRef(l,11),node:hDecodeOptRef(l,14),phase:hDecodeBuildPhase(l.nth(17))}))
  pure def hEncodeResultValue(x:C6HResult):List[int] = {
    val used = match x {
      | C6HResultValidated(p) => List(1).concat(List(hBool(p.ok)))
      | C6HResultEnvFound(p) => List(2).concat(c6EncodeRef(p.value))
      | C6HResultEnvMissing => List(3)
      | C6HResultEnvUpdated(p) => List(4).concat(List(p.cell))
      | C6HResultFindFound(p) => List(5).concat(List(p.function))
      | C6HResultFindMissing => List(6)
      | C6HResultBindOk(p) => List(7).concat(c6EncodeEnv(p.env))
      | C6HResultBindBad => List(8)
      | C6HResultAppendDone(p) => List(9).concat(c6EncodeSlice(p.seq))
      | C6HResultScalarValue(p) => List(10).concat(c6EncodeRef(p.value))
      | C6HResultScalarError(p) => List(11).concat(List(p.code))
      | C6HResultScalarSelector(p) => List(12).concat(List(hEncodeBoolSelector(p.sel)))
      | C6HResultScalarTypeTest(p) => List(13).concat(List(hBool(p.ok)))
      | C6HResultEqualDone(p) => List(14).concat(c6EncodeRef(p.value))
      | C6HResultMatchMatched(p) => List(15).concat(c6EncodeEnv(p.env))
      | C6HResultMatchNoMatch => List(16)
      | C6HResultPlanMatched(p) => List(17).concat(List(p.arm)).concat(c6EncodeEnv(p.env))
      | C6HResultPlanNoMatch => List(18)
      | C6HResultBuildDone(p) => List(19).concat(c6EncodeRef(p.value))
    }
    c6Pad(used,4)
  }
  pure def hEncodeResult(x:C6HOptResult):List[int] = match x {
    | C6HNoResult => c6Pad(List(),4)
    | C6HSomeResult(v) => hEncodeResultValue(v)
  }
  pure def hDecodeResult(l:List[int]):C6HOptResult =
    if(l.nth(0) == 0) C6HNoResult
    else if(l.nth(0) == 1) C6HSomeResult(C6HResultValidated({ok:l.nth(1) == 1}))
    else if(l.nth(0) == 2) C6HSomeResult(C6HResultEnvFound({value:c6DecodePositiveRef(l,1)}))
    else if(l.nth(0) == 3) C6HSomeResult(C6HResultEnvMissing)
    else if(l.nth(0) == 4) C6HSomeResult(C6HResultEnvUpdated({cell:l.nth(1)}))
    else if(l.nth(0) == 5) C6HSomeResult(C6HResultFindFound({function:l.nth(1)}))
    else if(l.nth(0) == 6) C6HSomeResult(C6HResultFindMissing)
    else if(l.nth(0) == 7) C6HSomeResult(C6HResultBindOk({env:c6DecodePositiveEnv(l,1)}))
    else if(l.nth(0) == 8) C6HSomeResult(C6HResultBindBad)
    else if(l.nth(0) == 9) C6HSomeResult(C6HResultAppendDone({seq:c6DecodePositiveSlice(l,1)}))
    else if(l.nth(0) == 10) C6HSomeResult(C6HResultScalarValue({value:c6DecodePositiveRef(l,1)}))
    else if(l.nth(0) == 11) C6HSomeResult(C6HResultScalarError({code:l.nth(1)}))
    else if(l.nth(0) == 12) C6HSomeResult(C6HResultScalarSelector({sel:hDecodeBoolSelector(l.nth(1))}))
    else if(l.nth(0) == 13) C6HSomeResult(C6HResultScalarTypeTest({ok:l.nth(1) == 1}))
    else if(l.nth(0) == 14) C6HSomeResult(C6HResultEqualDone({value:c6DecodePositiveRef(l,1)}))
    else if(l.nth(0) == 15) C6HSomeResult(C6HResultMatchMatched({env:c6DecodePositiveEnv(l,1)}))
    else if(l.nth(0) == 16) C6HSomeResult(C6HResultMatchNoMatch)
    else if(l.nth(0) == 17) C6HSomeResult(C6HResultPlanMatched({arm:l.nth(1),env:c6DecodePositiveEnv(l,2)}))
    else if(l.nth(0) == 18) C6HSomeResult(C6HResultPlanNoMatch)
    else C6HSomeResult(C6HResultBuildDone({value:c6DecodePositiveRef(l,1)}))
  pure def hEncodeTaskValue(x:C6HTask):List[int] = {
    val used = match x {
      | C6HValidateTask(p) => List(0).concat(c6EncodeRef(p.r)).concat(List(hEncodeTaskVisitC(p.visit))).concat(List(p.cursor))
      | C6HEqualTask(p) => List(1).concat(c6EncodeRef(p.left)).concat(c6EncodeRef(p.right)).concat(List(hEncodeTaskVisitC(p.visit))).concat(List(p.cursor))
      | C6HPairTask(p) => List(2).concat(List(p.binder)).concat(List(p.sort)).concat(c6EncodeRef(p.arg))
    }
    c6Pad(used,7)
  }
  pure def hDecodeTask(l:List[int]):C6HTask =
    if(l.nth(0) == 0) C6HValidateTask({r:c6DecodePositiveRef(l,1),visit:hDecodeTaskVisitC(l.nth(3)),cursor:l.nth(4)})
    else if(l.nth(0) == 1) C6HEqualTask({left:c6DecodePositiveRef(l,1),right:c6DecodePositiveRef(l,3),visit:hDecodeTaskVisitC(l.nth(5)),cursor:l.nth(6)})
    else C6HPairTask({binder:l.nth(1),sort:l.nth(2),arg:c6DecodePositiveRef(l,3)})
  pure def hEncodeTables(x:C6HHelperTables):List[int] =
    x.tasks.foldl(List(C6_MAGIC,1,1212501072,x.tasks.length())
      .concat(hEncodeHelper(x.helper)).concat(hEncodeResult(x.result)),
      (acc,taskArg) => acc.concat(hEncodeTaskValue(taskArg)))
  pure def hDecodeTables(l:List[int]):C6HHelperTables = {
    val tasksArg:List[C6HTask] = range(0,l.nth(3)).foldl(List(),(acc,i) =>
      acc.append(hDecodeTask(l.slice(31+7*i,31+7*(i+1)))))
    {helper:hDecodeHelper(l.slice(4,27)),result:hDecodeResult(l.slice(27,31)),tasks:tasksArg}
  }
}
