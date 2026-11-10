// -*- mode: Bluespec; -*-
// Sherlock 3.x C6 dispatch review: reads_result for Cases plan rules and full legacy universe.

module reads_result {

  // =========================================================================
  // 1. Full Universe of Sites (27 sites) and Legacy-Group Rules (46 rules)
  // =========================================================================

  type SiteTag =
    | SiteEvalLiteral
    | SiteEvalVar
    | SiteEvalLet
    | SiteEvalBinary
    | SiteEvalUnary
    | SiteEvalIf
    | SiteEvalMatch
    | SiteEvalCall
    | SiteEvalConstruct
    | SitePush
    | SiteRetLet
    | SiteRetLeft
    | SiteRetRight
    | SiteRetUnary
    | SiteRetIf
    | SiteRetMatch
    | SiteRetArgs
    | SiteRetCall
    | SiteRetScope
    | SiteRetCtorFields
    | SiteMaterialize
    | SiteInvoke
    | SiteEnter
    | SiteBound
    | SiteCases
    | SiteCaseResult
    | SitePrim

  type RuleId =
    // EvalLiteral (2)
    | KMachineEvalLiteralValid
    | KMachineEvalLiteralInvalid
    // EvalVar (2)
    | KMachineEvalVariablePresent
    | KMachineEvalVariableAbsent
    // EvalLet, EvalBinary, EvalUnary, EvalIf, EvalMatch (5)
    | KMachineEvalLet
    | KMachineEvalBinary
    | KMachineEvalUnary
    | KMachineEvalIf
    | KMachineEvalMatch
    // EvalCall (2)
    | KMachineEvalCallEmpty
    | KMachineEvalCallArguments
    // EvalConstruct (2)
    | XAdtEvalConstructNullary
    | XAdtEvalConstructFields
    // Push (2)
    | KMachinePushAccept
    | KMachinePushRefuse
    // RetLet, RetLeft, RetRight, RetUnary (4)
    | KMachineReturnLet
    | KMachineReturnLeft
    | KMachineReturnRight
    | KMachineReturnUnary
    // RetIf (3)
    | KMachineReturnIfTrue
    | KMachineReturnIfFalse
    | KMachineReturnIfInvalid
    // RetMatch (1)
    | KMachineReturnMatch
    // RetArgs (2)
    | KMachineReturnArgumentsNext
    | KMachineReturnArgumentsFinal
    // RetCall (2)
    | KMachineReturnCallValid
    | KMachineReturnCallInvalid
    // RetScope (1)
    | KMachineReturnScope
    // RetCtorFields (4)
    | XAdtReturnCtorFieldsTyped
    | XAdtReturnCtorFieldsMistyped
    | XAdtReturnCtorFieldsNext
    | XAdtReturnCtorFieldsFinal
    // Materialize (1)
    | XAdtMaterializeDone
    // Invoke (1)
    | KMachineInvokeFind
    // Enter (2)
    | KMachineEnterMissing
    | KMachineEnterBind
    // Bound (2)
    | KMachineBoundInvalid
    | KMachineBoundValid
    // Cases (4)
    | KMachineCasesEmpty
    | KMachineCasesTry
    | XAdtCasesPlanHit
    | XAdtCasesPlanMiss
    // CaseResult (2)
    | KMachineCasesMiss
    | KMachineCasesHit
    // Prim (2)
    | KMachinePrimitiveValue
    | KMachinePrimitiveError

  pure val allSites: Set[SiteTag] = Set(
    SiteEvalLiteral,
    SiteEvalVar,
    SiteEvalLet,
    SiteEvalBinary,
    SiteEvalUnary,
    SiteEvalIf,
    SiteEvalMatch,
    SiteEvalCall,
    SiteEvalConstruct,
    SitePush,
    SiteRetLet,
    SiteRetLeft,
    SiteRetRight,
    SiteRetUnary,
    SiteRetIf,
    SiteRetMatch,
    SiteRetArgs,
    SiteRetCall,
    SiteRetScope,
    SiteRetCtorFields,
    SiteMaterialize,
    SiteInvoke,
    SiteEnter,
    SiteBound,
    SiteCases,
    SiteCaseResult,
    SitePrim
  )

  pure val allLegacyRules: Set[RuleId] = Set(
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineEvalLet,
    KMachineEvalBinary,
    KMachineEvalUnary,
    KMachineEvalIf,
    KMachineEvalMatch,
    KMachineEvalCallEmpty,
    KMachineEvalCallArguments,
    XAdtEvalConstructNullary,
    XAdtEvalConstructFields,
    KMachinePushAccept,
    KMachinePushRefuse,
    KMachineReturnLet,
    KMachineReturnLeft,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnMatch,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineReturnScope,
    XAdtReturnCtorFieldsTyped,
    XAdtReturnCtorFieldsMistyped,
    XAdtReturnCtorFieldsNext,
    XAdtReturnCtorFieldsFinal,
    XAdtMaterializeDone,
    KMachineInvokeFind,
    KMachineEnterMissing,
    KMachineEnterBind,
    KMachineBoundInvalid,
    KMachineBoundValid,
    KMachineCasesEmpty,
    KMachineCasesTry,
    XAdtCasesPlanHit,
    XAdtCasesPlanMiss,
    KMachineCasesMiss,
    KMachineCasesHit,
    KMachinePrimitiveValue,
    KMachinePrimitiveError
  )

  // Core `legacy_group(site)` in `owner_runtime.rs` L1674-1720
  pure def coreLegacyGroup(site: SiteTag): List[RuleId] = {
    match site {
      | SiteEvalLiteral => [KMachineEvalLiteralValid, KMachineEvalLiteralInvalid]
      | SiteEvalVar => [KMachineEvalVariablePresent, KMachineEvalVariableAbsent]
      | SiteEvalLet => [KMachineEvalLet]
      | SiteEvalBinary => [KMachineEvalBinary]
      | SiteEvalUnary => [KMachineEvalUnary]
      | SiteEvalIf => [KMachineEvalIf]
      | SiteEvalMatch => [KMachineEvalMatch]
      | SiteEvalCall => [KMachineEvalCallEmpty, KMachineEvalCallArguments]
      | SiteEvalConstruct => [XAdtEvalConstructNullary, XAdtEvalConstructFields]
      | SitePush => [KMachinePushAccept, KMachinePushRefuse]
      | SiteRetLet => [KMachineReturnLet]
      | SiteRetLeft => [KMachineReturnLeft]
      | SiteRetRight => [KMachineReturnRight]
      | SiteRetUnary => [KMachineReturnUnary]
      | SiteRetIf => [KMachineReturnIfTrue, KMachineReturnIfFalse, KMachineReturnIfInvalid]
      | SiteRetMatch => [KMachineReturnMatch]
      | SiteRetArgs => [KMachineReturnArgumentsNext, KMachineReturnArgumentsFinal]
      | SiteRetCall => [KMachineReturnCallValid, KMachineReturnCallInvalid]
      | SiteRetScope => [KMachineReturnScope]
      | SiteRetCtorFields => [
          XAdtReturnCtorFieldsTyped,
          XAdtReturnCtorFieldsMistyped,
          XAdtReturnCtorFieldsNext,
          XAdtReturnCtorFieldsFinal
        ]
      | SiteMaterialize => [XAdtMaterializeDone]
      | SiteInvoke => [KMachineInvokeFind]
      | SiteEnter => [KMachineEnterMissing, KMachineEnterBind]
      | SiteBound => [KMachineBoundInvalid, KMachineBoundValid]
      | SiteCases => [
          KMachineCasesEmpty,
          KMachineCasesTry,
          XAdtCasesPlanHit,
          XAdtCasesPlanMiss
        ]
      | SiteCaseResult => [KMachineCasesMiss, KMachineCasesHit]
      | SitePrim => [KMachinePrimitiveValue, KMachinePrimitiveError]
    }
  }

  // Circuit `legacy_group(site)` in `c6_dispatch_step.rs` L215-261
  pure def circuitLegacyGroup(site: SiteTag): List[RuleId] = {
    match site {
      | SiteEvalLiteral => [KMachineEvalLiteralValid, KMachineEvalLiteralInvalid]
      | SiteEvalVar => [KMachineEvalVariablePresent, KMachineEvalVariableAbsent]
      | SiteEvalLet => [KMachineEvalLet]
      | SiteEvalBinary => [KMachineEvalBinary]
      | SiteEvalUnary => [KMachineEvalUnary]
      | SiteEvalIf => [KMachineEvalIf]
      | SiteEvalMatch => [KMachineEvalMatch]
      | SiteEvalCall => [KMachineEvalCallEmpty, KMachineEvalCallArguments]
      | SiteEvalConstruct => [XAdtEvalConstructNullary, XAdtEvalConstructFields]
      | SitePush => [KMachinePushAccept, KMachinePushRefuse]
      | SiteRetLet => [KMachineReturnLet]
      | SiteRetLeft => [KMachineReturnLeft]
      | SiteRetRight => [KMachineReturnRight]
      | SiteRetUnary => [KMachineReturnUnary]
      | SiteRetIf => [KMachineReturnIfTrue, KMachineReturnIfFalse, KMachineReturnIfInvalid]
      | SiteRetMatch => [KMachineReturnMatch]
      | SiteRetArgs => [KMachineReturnArgumentsNext, KMachineReturnArgumentsFinal]
      | SiteRetCall => [KMachineReturnCallValid, KMachineReturnCallInvalid]
      | SiteRetScope => [KMachineReturnScope]
      | SiteRetCtorFields => [
          XAdtReturnCtorFieldsTyped,
          XAdtReturnCtorFieldsMistyped,
          XAdtReturnCtorFieldsNext,
          XAdtReturnCtorFieldsFinal
        ]
      | SiteMaterialize => [XAdtMaterializeDone]
      | SiteInvoke => [KMachineInvokeFind]
      | SiteEnter => [KMachineEnterMissing, KMachineEnterBind]
      | SiteBound => [KMachineBoundInvalid, KMachineBoundValid]
      | SiteCases => [
          KMachineCasesEmpty,
          KMachineCasesTry,
          XAdtCasesPlanHit,
          XAdtCasesPlanMiss
        ]
      | SiteCaseResult => [KMachineCasesMiss, KMachineCasesHit]
      | SitePrim => [KMachinePrimitiveValue, KMachinePrimitiveError]
    }
  }

  // Site of each legacy rule (`legacy_site_of` in `c6_dispatch_step.rs` L348-353)
  pure def siteOfRule(r: RuleId): SiteTag = {
    match r {
      | KMachineEvalLiteralValid => SiteEvalLiteral
      | KMachineEvalLiteralInvalid => SiteEvalLiteral
      | KMachineEvalVariablePresent => SiteEvalVar
      | KMachineEvalVariableAbsent => SiteEvalVar
      | KMachineEvalLet => SiteEvalLet
      | KMachineEvalBinary => SiteEvalBinary
      | KMachineEvalUnary => SiteEvalUnary
      | KMachineEvalIf => SiteEvalIf
      | KMachineEvalMatch => SiteEvalMatch
      | KMachineEvalCallEmpty => SiteEvalCall
      | KMachineEvalCallArguments => SiteEvalCall
      | XAdtEvalConstructNullary => SiteEvalConstruct
      | XAdtEvalConstructFields => SiteEvalConstruct
      | KMachinePushAccept => SitePush
      | KMachinePushRefuse => SitePush
      | KMachineReturnLet => SiteRetLet
      | KMachineReturnLeft => SiteRetLeft
      | KMachineReturnRight => SiteRetRight
      | KMachineReturnUnary => SiteRetUnary
      | KMachineReturnIfTrue => SiteRetIf
      | KMachineReturnIfFalse => SiteRetIf
      | KMachineReturnIfInvalid => SiteRetIf
      | KMachineReturnMatch => SiteRetMatch
      | KMachineReturnArgumentsNext => SiteRetArgs
      | KMachineReturnArgumentsFinal => SiteRetArgs
      | KMachineReturnCallValid => SiteRetCall
      | KMachineReturnCallInvalid => SiteRetCall
      | KMachineReturnScope => SiteRetScope
      | XAdtReturnCtorFieldsTyped => SiteRetCtorFields
      | XAdtReturnCtorFieldsMistyped => SiteRetCtorFields
      | XAdtReturnCtorFieldsNext => SiteRetCtorFields
      | XAdtReturnCtorFieldsFinal => SiteRetCtorFields
      | XAdtMaterializeDone => SiteMaterialize
      | KMachineInvokeFind => SiteInvoke
      | KMachineEnterMissing => SiteEnter
      | KMachineEnterBind => SiteEnter
      | KMachineBoundInvalid => SiteBound
      | KMachineBoundValid => SiteBound
      | KMachineCasesEmpty => SiteCases
      | KMachineCasesTry => SiteCases
      | XAdtCasesPlanHit => SiteCases
      | XAdtCasesPlanMiss => SiteCases
      | KMachineCasesMiss => SiteCaseResult
      | KMachineCasesHit => SiteCaseResult
      | KMachinePrimitiveValue => SitePrim
      | KMachinePrimitiveError => SitePrim
    }
  }

  // =========================================================================
  // 2. Reader Sets: Derived vs Tables
  // =========================================================================

  // Semantic derivation from `owner_select_when` arms in
  // `evidence/ref_owner_runtime_select_when_L499-1203.rs` (L541-735):
  // True iff the rule's `owner_select_when` arm calls `res(pre)`.
  pure def selectWhenInspectsRes(r: RuleId): bool = {
    match r {
      | KMachineEvalLiteralValid => true      // L549: matches!(res(pre), Some(Validated { ok: true }))
      | KMachineEvalLiteralInvalid => true    // L550-552: matches!(res(pre), Some(Validated { ok: false }))
      | KMachineEvalVariablePresent => true   // L553: matches!(res(pre), Some(EnvFound { .. }))
      | KMachineEvalVariableAbsent => true    // L554: matches!(res(pre), Some(EnvMissing))
      | KMachineEvalLet => false              // L555-558: true
      | KMachineEvalBinary => false           // L555-558: true
      | KMachineEvalUnary => false            // L555-558: true
      | KMachineEvalIf => false               // L555-558: true
      | KMachineEvalMatch => false            // L555-558: true
      | KMachineReturnLeft => false           // L555-558: true
      | KMachineReturnMatch => false          // L555-558: true
      | KMachineReturnScope => false          // L555-558: true
      | KMachineEvalCallEmpty => false        // L559-564: inspects pre.control() / pre.expr()
      | KMachineEvalCallArguments => false    // L565-570: inspects pre.control() / pre.expr()
      | KMachinePushAccept => false           // L571: pre.depth() < pre.stack_limit()
      | KMachinePushRefuse => false           // L572: pre.depth() >= pre.stack_limit()
      | KMachineReturnLet => true             // L573: matches!(res(pre), Some(EnvUpdated { .. }))
      | KMachineReturnRight => true           // L574-581: matches!(res(pre), Some(ScalarValue | ScalarError | EqualDone))
      | KMachineReturnUnary => true           // L582-585: matches!(res(pre), Some(ScalarValue | ScalarError))
      | KMachineReturnIfTrue => true          // L586-591: matches!(res(pre), Some(ScalarSelector { True }))
      | KMachineReturnIfFalse => true         // L592-597: matches!(res(pre), Some(ScalarSelector { False }))
      | KMachineReturnIfInvalid => true       // L598-603: matches!(res(pre), Some(ScalarSelector { NotBool }))
      | KMachineReturnArgumentsNext => true   // L604-607: matches!(res(pre), Some(AppendDone)) && top_frame
      | KMachineReturnArgumentsFinal => true  // L608-611: matches!(res(pre), Some(AppendDone)) && top_frame
      | KMachineReturnCallValid => true       // L612-614: matches!(res(pre), Some(ScalarTypeTest { ok: true }))
      | KMachineReturnCallInvalid => true     // L615-617: matches!(res(pre), Some(ScalarTypeTest { ok: false }))
      | KMachineInvokeFind => true            // L618-621: matches!(res(pre), Some(FindFound | FindMissing))
      | KMachineEnterMissing => false         // L622: inspects pre.control()
      | KMachineEnterBind => true             // L623-626: matches!(res(pre), Some(BindOk | BindBad))
      | KMachineBoundInvalid => false         // L627-633: inspects pre.control()
      | KMachineBoundValid => false           // L634-640: inspects pre.control()
      | KMachineCasesEmpty => false           // L641-647: inspects pre.control() and control_origin()
      | KMachineCasesTry => true              // L648-656: control_origin() && matches!(res(pre), Some(MatchMatched | MatchNoMatch))
      | KMachineCasesMiss => false            // L657-663: inspects pre.control()
      | KMachineCasesHit => false             // L664-670: inspects pre.control()
      | KMachinePrimitiveValue => false       // L671-676: inspects pre.control()
      | KMachinePrimitiveError => false       // L677-682: inspects pre.control()
      | XAdtEvalConstructNullary => false     // L683-688: inspects pre.control() / pre.expr()
      | XAdtEvalConstructFields => false      // L689-694: inspects pre.control() / pre.expr()
      | XAdtReturnCtorFieldsTyped => true     // L695-700: top_frame && matches!(res(pre), Some(ScalarTypeTest { ok: true }))
      | XAdtReturnCtorFieldsMistyped => true  // L701-706: top_frame && matches!(res(pre), Some(ScalarTypeTest { ok: false }))
      | XAdtReturnCtorFieldsNext => true      // L707-710: top_frame && matches!(res(pre), Some(AppendDone))
      | XAdtReturnCtorFieldsFinal => true     // L711-714: top_frame && matches!(res(pre), Some(AppendDone))
      | XAdtMaterializeDone => true           // L715-718: pre.control() && matches!(res(pre), Some(BuildDone))
      | XAdtCasesPlanHit => true              // L719-726: pre.control() && control_origin() && matches!(res(pre), Some(PlanMatched))
      | XAdtCasesPlanMiss => true             // L727-734: pre.control() && control_origin() && matches!(res(pre), Some(PlanNoMatch))
    }
  }

  // `refReads`: derived from `owner_select_when` arms inspecting `res(pre)`
  pure val refReads: Set[RuleId] =
    allLegacyRules.filter(r => selectWhenInspectsRes(r))

  // `coreTable`: Core `reads_result_root` in `owner_runtime.rs` L507-536
  pure val coreTable: Set[RuleId] = Set(
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineReturnLet,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineInvokeFind,
    KMachineEnterBind,
    KMachineCasesTry,
    XAdtReturnCtorFieldsTyped,
    XAdtReturnCtorFieldsMistyped,
    XAdtReturnCtorFieldsNext,
    XAdtReturnCtorFieldsFinal,
    XAdtMaterializeDone,
    XAdtCasesPlanHit,
    XAdtCasesPlanMiss
  )

  // `validateMirror`: `validate_plan` `reads_r` list in `owner_runtime.rs` L3095-3121
  pure val validateMirror: Set[RuleId] = Set(
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineReturnLet,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineInvokeFind,
    KMachineEnterBind,
    KMachineCasesTry,
    XAdtReturnCtorFieldsTyped,
    XAdtReturnCtorFieldsMistyped,
    XAdtReturnCtorFieldsNext,
    XAdtReturnCtorFieldsFinal,
    XAdtMaterializeDone,
    XAdtCasesPlanHit,
    XAdtCasesPlanMiss
  )

  // `circuitReadsOld`: circuit `reads_result` before the change (`change.diff` L5-12)
  pure val circuitReadsOld: Set[RuleId] = Set(
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineReturnLet,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineInvokeFind,
    KMachineEnterBind,
    KMachineCasesTry,
    XAdtReturnCtorFieldsTyped,
    XAdtReturnCtorFieldsMistyped,
    XAdtReturnCtorFieldsNext,
    XAdtReturnCtorFieldsFinal,
    XAdtMaterializeDone
  )

  // `circuitReads`: circuit `reads_result` after the change (`c6_dispatch_step.rs` L297-326)
  pure val circuitReads: Set[RuleId] = Set(
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineReturnLet,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineInvokeFind,
    KMachineEnterBind,
    KMachineCasesTry,
    XAdtReturnCtorFieldsTyped,
    XAdtReturnCtorFieldsMistyped,
    XAdtReturnCtorFieldsNext,
    XAdtReturnCtorFieldsFinal,
    XAdtMaterializeDone,
    XAdtCasesPlanHit,
    XAdtCasesPlanMiss
  )

  // Circuit `guard` arm analysis in `c6_dispatch_step.rs` L902-1045:
  // True iff the rule's circuit `guard` arm calls `rt_is` or `r1_is`.
  pure def circuitGuardInspectsRtOrR1(r: RuleId): bool = {
    match r {
      | KMachineEvalLiteralValid => true      // L908-912: rt_is(&[1]), r1_is(1)
      | KMachineEvalLiteralInvalid => true    // L913-917: rt_is(&[1]), r1_is(0)
      | KMachineEvalVariablePresent => true   // L918: rt_is(&[2])
      | KMachineEvalVariableAbsent => true    // L919: rt_is(&[3])
      | KMachineEvalLet => false              // L920-922: Em::imm(1)
      | KMachineEvalBinary => false           // L920-922: Em::imm(1)
      | KMachineEvalUnary => false            // L920-922: Em::imm(1)
      | KMachineEvalIf => false               // L920-922: Em::imm(1)
      | KMachineEvalMatch => false            // L920-922: Em::imm(1)
      | KMachineReturnLeft => false           // L920-922: Em::imm(1)
      | KMachineReturnMatch => false          // L920-922: Em::imm(1)
      | KMachineReturnScope => false          // L920-922: Em::imm(1)
      | KMachineEvalCallEmpty => false        // L923-927: eval_row
      | KMachineEvalCallArguments => false    // L928-932: eval_row
      | KMachinePushAccept => false           // L933: depth < stack_limit
      | KMachinePushRefuse => false           // L934-937: !(depth < stack_limit)
      | KMachineReturnLet => true             // L938: rt_is(&[4])
      | KMachineReturnRight => true           // L939: rt_is(&[10, 11, 14])
      | KMachineReturnUnary => true           // L940: rt_is(&[10, 11])
      | KMachineReturnIfTrue => true          // L941-950: rt_is(&[12]), r1_is(0)
      | KMachineReturnIfFalse => true         // L941-950: rt_is(&[12]), r1_is(1)
      | KMachineReturnIfInvalid => true       // L941-950: rt_is(&[12]), r1_is(2)
      | KMachineReturnArgumentsNext => true   // L951-961: rt_is(&[9])
      | KMachineReturnArgumentsFinal => true  // L951-961: rt_is(&[9])
      | KMachineReturnCallValid => true       // L962-966: rt_is(&[13]), r1_is(1)
      | KMachineReturnCallInvalid => true     // L962-966: rt_is(&[13]), r1_is(0)
      | KMachineInvokeFind => true            // L967: rt_is(&[5, 6])
      | KMachineEnterMissing => false         // L968-972: ct, k[1]
      | KMachineEnterBind => true             // L973: rt_is(&[7, 8])
      | KMachineBoundInvalid => false         // L974-978: ct, k[1]
      | KMachineBoundValid => false           // L974-978: ct, k[1]
      | KMachineCasesEmpty => false           // L979-985: ct, k[4], origin_row
      | KMachineCasesTry => true              // L986-991: origin_row, rt_is(&[15, 16])
      | KMachineCasesMiss => false            // L992-996: ct, k[1]
      | KMachineCasesHit => false             // L992-996: ct, k[1]
      | KMachinePrimitiveValue => false       // L997-1001: ct, k[1]
      | KMachinePrimitiveError => false       // L997-1001: ct, k[1]
      | XAdtEvalConstructNullary => false     // L1002-1007: eval_row
      | XAdtEvalConstructFields => false      // L1008-1013: eval_row
      | XAdtReturnCtorFieldsTyped => true     // L1014-1020: top, rt_is(&[13]), r1_is(1)
      | XAdtReturnCtorFieldsMistyped => true  // L1014-1020: top, rt_is(&[13]), r1_is(0)
      | XAdtReturnCtorFieldsNext => true      // L1021-1032: top, rt_is(&[9])
      | XAdtReturnCtorFieldsFinal => true     // L1021-1032: top, rt_is(&[9])
      | XAdtMaterializeDone => true           // L1033-1037: ct, rt_is(&[19])
      | XAdtCasesPlanHit => true              // L1038-1044: ct, origin_row, rt_is(&[17])
      | XAdtCasesPlanMiss => true             // L1038-1044: ct, origin_row, rt_is(&[18])
    }
  }

  // `circuitGuardReads`: rules whose circuit `guard` arm uses `rt_is` or `r1_is`
  pure val circuitGuardReads: Set[RuleId] =
    allLegacyRules.filter(r => circuitGuardInspectsRtOrR1(r))

  // Parameterized check that a candidate reader set `candReads` agrees with `refReads`
  // and all other reference sets across the full 46-rule legacy universe.
  pure def readSetsAgreeFor(candReads: Set[RuleId]): bool = {
    allLegacyRules.forall(r =>
      (candReads.contains(r) == refReads.contains(r))
        and (candReads.contains(r) == coreTable.contains(r))
        and (candReads.contains(r) == validateMirror.contains(r))
        and (candReads.contains(r) == circuitGuardReads.contains(r))
    )
  }

  // Check that Core and Circuit `legacy_group` definitions agree on every site
  // and partition the 46 legacy rules.
  pure val legacyGroupsAgree: bool = {
    allSites.forall(s => coreLegacyGroup(s) == circuitLegacyGroup(s))
      and allLegacyRules.forall(r =>
        coreLegacyGroup(siteOfRule(r)).foldl(false, (acc, x) => acc or (x == r))
      )
  }

  // Invariant `readSetsAgree`: for every rule in the universe, `circuitReads`
  // membership equals `refReads` membership (and equals the other three sets).
  pure val readSetsAgree: bool =
    legacyGroupsAgree and readSetsAgreeFor(circuitReads)

  // =========================================================================
  // 3. Abstract Dispatch Model for `Cases` (and `EvalLet`)
  // =========================================================================

  type ResultTag =
    | ResNone
    | ResMatchMatched
    | ResMatchNoMatch
    | ResPlanMatched
    | ResPlanNoMatch
    | ResOther

  pure val allResultTags: Set[ResultTag] = Set(
    ResNone,
    ResMatchMatched,
    ResMatchNoMatch,
    ResPlanMatched,
    ResPlanNoMatch,
    ResOther
  )

  pure val dispatchSites: Set[SiteTag] = Set(SiteCases, SiteEvalLet)

  type DispatchState = {
    site: SiteTag,
    resultTag: ResultTag,
    hasPlan: bool,
    armsEmpty: bool
  }

  pure val allBools: Set[bool] = Set(false, true)

  // Full finite universe of abstract pre-states (2 sites * 6 result tags * 2 * 2 = 48 states)
  pure val allDispatchStates: Set[DispatchState] =
    dispatchSites.fold(Set(), (accS, s) =>
      accS.union(
        allResultTags.fold(Set(), (accR, rt) =>
          accR.union(
            allBools.fold(Set(), (accP, hp) =>
              accP.union(
                allBools.map(ae => {
                  site: s,
                  resultTag: rt,
                  hasPlan: hp,
                  armsEmpty: ae
                })
              )
            )
          )
        )
      )
    )

  // Subset of abstract states at the `Cases` site (24 states)
  pure val allCasesStates: Set[DispatchState] =
    allDispatchStates.filter(st => st.site == SiteCases)

  // Rules in the abstract dispatch sites (`Cases` + `EvalLet`)
  pure val dispatchSiteRules: Set[RuleId] = Set(
    KMachineCasesEmpty,
    KMachineCasesTry,
    XAdtCasesPlanHit,
    XAdtCasesPlanMiss,
    KMachineEvalLet
  )

  // Helper plan options at the abstract sites (`owner_helper_plan` in `owner_runtime.rs` L1268-1312)
  type HelperPlanOpt =
    | PlanNone
    | PlanMatchHelper
    | PlanPlanHelper

  pure def ownerHelperPlan(st: DispatchState): HelperPlanOpt = {
    match st.site {
      | SiteCases =>
        if (st.hasPlan) {
          // L1300-1302: Match { plan: Some(_) } => Some(HelperKind::Plan)
          PlanPlanHelper
        } else if (not(st.armsEmpty)) {
          // L1303-1307: Match { plan: None } if arms.len > 0 => Some(HelperKind::Match)
          PlanMatchHelper
        } else {
          PlanNone
        }
      | _ => PlanNone
    }
  }

  // Core `owner_select_when(rule, pre)` on the abstract state (`owner_runtime.rs` L541-735)
  pure def refSelectWhen(r: RuleId, st: DispatchState): bool = {
    match r {
      | KMachineCasesEmpty =>
        // L641-647: Cases { arms } if arms.len == 0 && Match { plan: None }
        (st.site == SiteCases) and st.armsEmpty and not(st.hasPlan)
      | KMachineCasesTry =>
        // L648-656: Match { plan: None } && res in { MatchMatched, MatchNoMatch }
        not(st.hasPlan)
          and ((st.resultTag == ResMatchMatched) or (st.resultTag == ResMatchNoMatch))
      | XAdtCasesPlanHit =>
        // L719-726: Cases && Match { plan: Some(_) } && res == PlanMatched
        (st.site == SiteCases) and st.hasPlan and (st.resultTag == ResPlanMatched)
      | XAdtCasesPlanMiss =>
        // L727-734: Cases && Match { plan: Some(_) } && res == PlanNoMatch
        (st.site == SiteCases) and st.hasPlan and (st.resultTag == ResPlanNoMatch)
      | KMachineEvalLet =>
        // L555-558: true
        true
      | _ => false
    }
  }

  // Outcome of Core `owner_select` (`owner_runtime.rs` L1722-1775).
  // States where Core selects a helper Entry rule (e.g., `BHelperMatchEntry` or
  // unsupported `XAdtHelperPlanEntry`) return `SelHelperEntry`, which is distinct
  // from `SelLegacyRule(r)` for every legacy rule `r`.
  type RefSelection =
    | SelLegacyRule(RuleId)
    | SelHelperEntry(HelperPlanOpt)
    | SelNone

  pure def scanLegacyGroupCore(grp: List[RuleId], wantReader: bool, st: DispatchState): RefSelection = {
    grp.foldl(
      SelNone,
      (acc, r) => {
        if (acc != SelNone) {
          acc
        } else if ((coreTable.contains(r) == wantReader) and refSelectWhen(r, st)) {
          SelLegacyRule(r)
        } else {
          SelNone
        }
      }
    )
  }

  // `refSelect(state)` following Core `owner_select` (`owner_runtime.rs` L1747-1774)
  pure def refSelect(st: DispatchState): RefSelection = {
    val grp = coreLegacyGroup(st.site)
    if (st.resultTag != ResNone) {
      // L1748-1760: result is present -> first rule in legacy_group(site) with reads_result_root && select_when
      scanLegacyGroupCore(grp, true, st)
    } else {
      // L1761-1763: else if owner_helper_plan(site) is Some(k) -> entry_rule(k)
      val hp = ownerHelperPlan(st)
      if (hp != PlanNone) {
        SelHelperEntry(hp)
      } else {
        // L1764-1768: else first rule in legacy_group(site) with !reads_result_root && select_when
        scanLegacyGroupCore(grp, false, st)
      }
    }
  }

  // Circuit `any_plan` (`c6_dispatch_step.rs` L888-899):
  // Includes `plan_kind(1..=9)` (at Cases: `no_plan && arms > 0`) plus the
  // explicit `Cases && is_match && has_plan` term (L894-897).
  pure def circuitAnyPlan(st: DispatchState): bool = {
    match st.site {
      | SiteCases =>
        val matchPlan = not(st.hasPlan) and not(st.armsEmpty)
        val adtPlan = st.hasPlan
        matchPlan or adtPlan
      | _ => false
    }
  }

  // Circuit `guard(rule)` over the C6 lanes (`c6_dispatch_step.rs` L902-1045)
  pure def circuitGuard(r: RuleId, st: DispatchState): bool = {
    match r {
      | KMachineCasesEmpty =>
        // L979-985: all(&[cases, empty, is_match, no_plan])
        (st.site == SiteCases) and st.armsEmpty and not(st.hasPlan)
      | KMachineCasesTry =>
        // L986-991: all(&[is_match, no_plan, rt_is(&[15, 16])])
        not(st.hasPlan)
          and ((st.resultTag == ResMatchMatched) or (st.resultTag == ResMatchNoMatch))
      | XAdtCasesPlanHit =>
        // L1038-1044: all(&[cases, is_match, plan, rt_is(&[17])])
        (st.site == SiteCases) and st.hasPlan and (st.resultTag == ResPlanMatched)
      | XAdtCasesPlanMiss =>
        // L1038-1044: all(&[cases, is_match, plan, rt_is(&[18])])
        (st.site == SiteCases) and st.hasPlan and (st.resultTag == ResPlanNoMatch)
      | KMachineEvalLet =>
        // L920-922: Em::imm(1)
        true
      | _ => false
    }
  }

  // Circuit `path(rule)` for a legacy rule, parameterized by reader set `reads`
  // (`c6_dispatch_step.rs` L1291-1315).
  pure def circuitSelectableWith(reads: Set[RuleId], rule: RuleId, st: DispatchState): bool = {
    val ruleSite = siteOfRule(rule)
    val atSite = (st.site == ruleSite)
    val resultReader = reads.contains(rule)
    val branchOk =
      if (resultReader) {
        // L1297-1299: terms.push(em.not(f.no_result))
        st.resultTag != ResNone
      } else {
        // L1300-1303: terms.push(f.no_result); terms.push(em.not(any_plan))
        (st.resultTag == ResNone) and not(circuitAnyPlan(st))
      }
    val grp = circuitLegacyGroup(ruleSite)
    val scan = grp.foldl(
      { seenRule: false, guardsOk: true },
      (acc, r) => {
        if (acc.seenRule) {
          acc
        } else if (r == rule) {
          { seenRule: true, guardsOk: acc.guardsOk and circuitGuard(r, st) }
        } else if (reads.contains(r) == resultReader) {
          { seenRule: false, guardsOk: acc.guardsOk and not(circuitGuard(r, st)) }
        } else {
          acc
        }
      }
    )
    atSite and branchOk and scan.seenRule and scan.guardsOk
  }

  // Default `circuitSelectable(rule, state)` using `circuitReads` (after the change)
  pure def circuitSelectable(rule: RuleId, st: DispatchState): bool =
    circuitSelectableWith(circuitReads, rule, st)

  // Agreement check on a single state for a given reader set `reads`
  pure def dispatchAgreesOnState(reads: Set[RuleId], st: DispatchState): bool = {
    val refSel = refSelect(st)
    dispatchSiteRules.forall(r =>
      circuitSelectableWith(reads, r, st) == (refSel == SelLegacyRule(r))
    )
  }

  // Agreement check across all abstract states for a given reader set `reads`
  pure def dispatchAgreesFor(reads: Set[RuleId]): bool =
    allDispatchStates.forall(st => dispatchAgreesOnState(reads, st))

  // =========================================================================
  // 4. State Machine for `quint run --invariant=...`
  // =========================================================================

  var curState: DispatchState

  action init = {
    nondet s = oneOf(allDispatchStates)
    curState' = s
  }

  action step = {
    nondet s = oneOf(allDispatchStates)
    curState' = s
  }

  // State-level invariant `dispatchAgrees`: holds on `curState` and across the full state space
  val dispatchAgrees: bool =
    dispatchAgreesOnState(circuitReads, curState) and dispatchAgreesFor(circuitReads)

  // Combined invariant for `quint run --invariant=allInvariants`
  val allInvariants: bool =
    readSetsAgree and dispatchAgrees

}
