module c6PositiveDecoding {
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  // NORMAL CODE-IMAGE PRECONDITION on EVERY function below:
  // exact positive C6P1 widths/tags/zero-padding/presence flags/numeric kinds.
  // These functions are NOT validators. Values outside that declared image
  // have unspecified interpretation; no refusal/security policy is provided.
  // Raw discriminants are read from lanes, never from an original-variant hint.
  pure def c6DecodePositiveRef(rowArg:List[int], atArg:int):C6Ref =
    if(rowArg.nth(atArg) == 0) C6Const(rowArg.nth(atArg+1))
    else C6Data(rowArg.nth(atArg+1))
  pure def c6DecodePositiveEnv(rowArg:List[int], atArg:int):C6Env =
    if(rowArg.nth(atArg) == 0) C6EnvNone else C6EnvSome(rowArg.nth(atArg+1))
  pure def c6DecodePositiveFunction(rowArg:List[int], atArg:int):C6Function =
    if(rowArg.nth(atArg) == 0) C6FunctionNone else C6FunctionSome(rowArg.nth(atArg+1))
  pure def c6DecodePositiveSlice(rowArg:List[int], atArg:int):C6Slice =
    {start:rowArg.nth(atArg),len:rowArg.nth(atArg+1)}
  pure def c6DecodePositiveBinary(tagArg:int):C6Binary =
    if(tagArg == 0) C6Add else if(tagArg == 1) C6Sub
    else if(tagArg == 2) C6Mul else if(tagArg == 3) C6Eq
    else if(tagArg == 4) C6Lt else if(tagArg == 5) C6Cons else C6Branch
  pure def c6DecodePositiveUnary(tagArg:int):C6Unary =
    if(tagArg == 0) C6Leaf else C6Not
  pure def c6DecodePositiveFrame(rowArg:List[int]):C6Frame = {
    val tagArg = rowArg.nth(0)
    if(tagArg == 0) C6LetF({binder:rowArg.nth(1),body:rowArg.nth(2),
      saved_env:c6DecodePositiveEnv(rowArg,3),origin:rowArg.nth(5)})
    else if(tagArg == 1) C6LeftF({op:c6DecodePositiveBinary(rowArg.nth(1)),
      right:rowArg.nth(2),saved_env:c6DecodePositiveEnv(rowArg,3),origin:rowArg.nth(5)})
    else if(tagArg == 2) C6RightF({op:c6DecodePositiveBinary(rowArg.nth(1)),
      left:c6DecodePositiveRef(rowArg,2),origin:rowArg.nth(4)})
    else if(tagArg == 3) C6UnaryF({op:c6DecodePositiveUnary(rowArg.nth(1)),origin:rowArg.nth(2)})
    else if(tagArg == 4) C6IfF({then_expr:rowArg.nth(1),else_expr:rowArg.nth(2),
      saved_env:c6DecodePositiveEnv(rowArg,3),origin:rowArg.nth(5)})
    else if(tagArg == 5) C6MatchF({arms:c6DecodePositiveSlice(rowArg,1),
      saved_env:c6DecodePositiveEnv(rowArg,3),origin:rowArg.nth(5)})
    else if(tagArg == 6) C6ArgsF({function_name:rowArg.nth(1),
      completed:c6DecodePositiveSlice(rowArg,2),rest:c6DecodePositiveSlice(rowArg,4),
      saved_env:c6DecodePositiveEnv(rowArg,6),origin:rowArg.nth(8)})
    else if(tagArg == 7) C6ReturnF({return_type:rowArg.nth(1),
      saved_env:c6DecodePositiveEnv(rowArg,2),origin:rowArg.nth(4)})
    else if(tagArg == 8) C6ScopeF({saved_env:c6DecodePositiveEnv(rowArg,1),origin:rowArg.nth(3)})
    // Remaining VALID frame tag is9. No malformed-tag decision is claimed.
    else C6ConstructorFieldsF({type_id:rowArg.nth(1),ctor_id:rowArg.nth(2),
      index:rowArg.nth(3),field_type:rowArg.nth(4),checked:rowArg.nth(5) == 1,
      completed:c6DecodePositiveSlice(rowArg,6),rest:c6DecodePositiveSlice(rowArg,8),
      saved_env:c6DecodePositiveEnv(rowArg,10),origin:rowArg.nth(12)})
  }
  pure def c6DecodePositiveBinding(rowArg:List[int]):C6Binding =
    if(rowArg.nth(1) == 0) C6BindingBad else C6BindingOk(c6DecodePositiveEnv(rowArg,2))
  pure def c6DecodePositiveMatching(rowArg:List[int]):C6Matching =
    if(rowArg.nth(1) == 0) C6NoMatch else C6Matched(c6DecodePositiveEnv(rowArg,2))
  pure def c6DecodePositivePrimitive(rowArg:List[int]):C6Primitive =
    if(rowArg.nth(1) == 0) C6PrimitiveValue(c6DecodePositiveRef(rowArg,2))
    else C6PrimitiveError(rowArg.nth(2))
  pure def c6DecodePositiveControl(rowArg:List[int]):C6Control = {
    val tagArg = rowArg.nth(0)
    if(tagArg == 0) C6Eval({expr:rowArg.nth(1)})
    else if(tagArg == 1) C6Return({value:c6DecodePositiveRef(rowArg,1)})
    else if(tagArg == 2) C6ScheduleNext({frame:c6DecodePositiveFrame(rowArg.slice(1,14)),
      next:rowArg.nth(14),env:c6DecodePositiveEnv(rowArg,15)})
    else if(tagArg == 3) C6Invoke({function_name:rowArg.nth(1),args:c6DecodePositiveSlice(rowArg,2)})
    else if(tagArg == 4) C6Enter({function:c6DecodePositiveFunction(rowArg,1),args:c6DecodePositiveSlice(rowArg,3)})
    else if(tagArg == 5) C6Bound({binding:c6DecodePositiveBinding(rowArg),
      return_type:rowArg.nth(4),body:rowArg.nth(5)})
    else if(tagArg == 6) C6Cases({value:c6DecodePositiveRef(rowArg,1),
      arms:c6DecodePositiveSlice(rowArg,3),saved_env:c6DecodePositiveEnv(rowArg,5)})
    else if(tagArg == 7) C6CaseResult({matching:c6DecodePositiveMatching(rowArg),
      value:c6DecodePositiveRef(rowArg,4),body:rowArg.nth(6),
      rest:c6DecodePositiveSlice(rowArg,7),saved_env:c6DecodePositiveEnv(rowArg,9)})
    else if(tagArg == 8) C6PrimResult({result:c6DecodePositivePrimitive(rowArg)})
    else if(tagArg == 9) C6Error({code:rowArg.nth(1)})
    // Remaining VALID control tag is10. No malformed-tag decision is claimed.
    else C6Materialize({type_id:rowArg.nth(1),ctor_id:rowArg.nth(2),fields:c6DecodePositiveSlice(rowArg,3)})
  }
  pure def c6DecodePositiveExhaust(tagArg:int):C6Exhaust =
    if(tagArg == 0) C6Fuel else if(tagArg == 1) C6DataNodes
    else if(tagArg == 2) C6DataFields else if(tagArg == 3) C6EnvironmentCells
    else if(tagArg == 4) C6HelperTasks else if(tagArg == 5) C6HelperWork else C6SerializationBytes
  pure def c6DecodePositiveOutcome(rowArg:List[int]):C6Outcome =
    if(rowArg.nth(23) == 0) C6Running
    else if(rowArg.nth(23) == 1) C6Halted(c6DecodePositiveRef(rowArg,24))
    else if(rowArg.nth(23) == 2) C6Rejected(rowArg.nth(24))
    else C6Exhausted(c6DecodePositiveExhaust(rowArg.nth(24)))
  pure def c6DecodePositiveAlloc(rowArg:List[int], atArg:int):C6Alloc =
    {data_nodes:rowArg.nth(atArg),data_fields:rowArg.nth(atArg+1),environment_cells:rowArg.nth(atArg+2)}
  pure def c6DecodePositiveActive(rowArg:List[int], atArg:int):C6Active =
    {data:rowArg.nth(atArg),data_fields:rowArg.nth(atArg+1),
      env:rowArg.nth(atArg+2),value_slots:rowArg.nth(atArg+3)}
  pure def c6DecodePositiveSection(rowArg:List[int]):C6Section = {
    // Requires exactmagic/version and canonical count/length under positive domain.
    val framesArg = range(0,rowArg.nth(47)).foldl(List(),(framesSoFarArg,indexArg) =>
      framesSoFarArg.append(c6DecodePositiveFrame(rowArg.slice(48+13*indexArg,48+13*(indexArg+1)))))
    {control:c6DecodePositiveControl(rowArg.slice(2,19)),control_origin:rowArg.nth(19),
      env_root:c6DecodePositiveEnv(rowArg,20),stack_limit:rowArg.nth(22),
      outcome:c6DecodePositiveOutcome(rowArg),initial_fuel:rowArg.nth(26),
      remaining_work:rowArg.nth(27),consumed_work:rowArg.nth(28),helper_work:rowArg.nth(29),
      helper_visits:rowArg.nth(30),reference_cursor:rowArg.nth(31),
      alloc:c6DecodePositiveAlloc(rowArg,32),active:c6DecodePositiveActive(rowArg,35),
      seg_active:c6DecodePositiveActive(rowArg,39),seg_alloc:c6DecodePositiveAlloc(rowArg,43),
      seg_consumed_work:rowArg.nth(46),frames:framesArg}
  }
}
