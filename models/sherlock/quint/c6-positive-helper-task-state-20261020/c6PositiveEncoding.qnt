module c6PositiveEncoding {
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  // Source counterpart of proposed/unadopted sherlock-c6-phys/1 Chunk1 ONLY.
  // Inputs are positive typed owned records; int-to-hostwidth, admission,
  // capacity/refusal policy and full HostSnapshot/native binding are external.
  pure val C6_PROFILE: str = "sherlock-c6-phys/1"
  pure val C6_MAGIC: int = 18918541878711089
  pure def c6Pad(rowArg:List[int], widthArg:int):List[int] =
    range(rowArg.length(),widthArg).foldl(rowArg,(cellsArg,indexArg) => cellsArg.append(0))
  pure def c6EncodeRef(refArg:C6Ref):List[int] = match refArg {
    | C6Const(indexArg) => List(0,indexArg)
    | C6Data(indexArg) => List(1,indexArg)
  }
  pure def c6EncodeEnv(envArg:C6Env):List[int] = match envArg {
    | C6EnvNone => List(0,0)
    | C6EnvSome(indexArg) => List(1,indexArg)
  }
  pure def c6EncodeFunction(functionArg:C6Function):List[int] = match functionArg {
    | C6FunctionNone => List(0,0)
    | C6FunctionSome(indexArg) => List(1,indexArg)
  }
  pure def c6EncodeSlice(sliceArg:C6Slice):List[int] = List(sliceArg.start,sliceArg.len)
  pure def c6BinaryTag(opArg:C6Binary):int = match opArg {
    | C6Add => 0 | C6Sub => 1 | C6Mul => 2 | C6Eq => 3
    | C6Lt => 4 | C6Cons => 5 | C6Branch => 6
  }
  pure def c6UnaryTag(opArg:C6Unary):int = match opArg { | C6Leaf => 0 | C6Not => 1 }
  pure def c6EncodeFrame(frameArg:C6Frame):List[int] = {
    val usedLanes = match frameArg {
      | C6LetF(payloadArg) => List(0,payloadArg.binder,payloadArg.body)
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6LeftF(payloadArg) => List(1,c6BinaryTag(payloadArg.op),payloadArg.right)
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6RightF(payloadArg) => List(2,c6BinaryTag(payloadArg.op))
          .concat(c6EncodeRef(payloadArg.left)).append(payloadArg.origin)
      | C6UnaryF(payloadArg) => List(3,c6UnaryTag(payloadArg.op),payloadArg.origin)
      | C6IfF(payloadArg) => List(4,payloadArg.then_expr,payloadArg.else_expr)
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6MatchF(payloadArg) => List(5).concat(c6EncodeSlice(payloadArg.arms))
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6ArgsF(payloadArg) => List(6,payloadArg.function_name)
          .concat(c6EncodeSlice(payloadArg.completed)).concat(c6EncodeSlice(payloadArg.rest))
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6ReturnF(payloadArg) => List(7,payloadArg.return_type)
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6ScopeF(payloadArg) => List(8).concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
      | C6ConstructorFieldsF(payloadArg) => List(9,payloadArg.type_id,payloadArg.ctor_id,
          payloadArg.index,payloadArg.field_type,if(payloadArg.checked) 1 else 0)
          .concat(c6EncodeSlice(payloadArg.completed)).concat(c6EncodeSlice(payloadArg.rest))
          .concat(c6EncodeEnv(payloadArg.saved_env)).append(payloadArg.origin)
    }
    c6Pad(usedLanes,13)
  }
  pure def c6EncodeBinding(bindingArg:C6Binding):List[int] = match bindingArg {
    | C6BindingBad => List(0,0,0)
    | C6BindingOk(envArg) => List(1).concat(c6EncodeEnv(envArg))
  }
  pure def c6EncodeMatching(matchingArg:C6Matching):List[int] = match matchingArg {
    | C6NoMatch => List(0,0,0)
    | C6Matched(envArg) => List(1).concat(c6EncodeEnv(envArg))
  }
  pure def c6EncodePrimitive(primitiveArg:C6Primitive):List[int] = match primitiveArg {
    | C6PrimitiveValue(refArg) => List(0).concat(c6EncodeRef(refArg))
    | C6PrimitiveError(codeArg) => List(1,codeArg,0)
  }
  pure def c6EncodeControl(controlArg:C6Control):List[int] = {
    val usedLanes = match controlArg {
      | C6Eval(payloadArg) => List(0,payloadArg.expr)
      | C6Return(payloadArg) => List(1).concat(c6EncodeRef(payloadArg.value))
      | C6ScheduleNext(payloadArg) => List(2).concat(c6EncodeFrame(payloadArg.frame))
          .append(payloadArg.next).concat(c6EncodeEnv(payloadArg.env))
      | C6Invoke(payloadArg) => List(3,payloadArg.function_name).concat(c6EncodeSlice(payloadArg.args))
      | C6Enter(payloadArg) => List(4).concat(c6EncodeFunction(payloadArg.function))
          .concat(c6EncodeSlice(payloadArg.args))
      | C6Bound(payloadArg) => List(5).concat(c6EncodeBinding(payloadArg.binding))
          .append(payloadArg.return_type).append(payloadArg.body)
      | C6Cases(payloadArg) => List(6).concat(c6EncodeRef(payloadArg.value))
          .concat(c6EncodeSlice(payloadArg.arms)).concat(c6EncodeEnv(payloadArg.saved_env))
      | C6CaseResult(payloadArg) => List(7).concat(c6EncodeMatching(payloadArg.matching))
          .concat(c6EncodeRef(payloadArg.value)).append(payloadArg.body)
          .concat(c6EncodeSlice(payloadArg.rest)).concat(c6EncodeEnv(payloadArg.saved_env))
      | C6PrimResult(payloadArg) => List(8).concat(c6EncodePrimitive(payloadArg.result))
      | C6Error(payloadArg) => List(9,payloadArg.code)
      | C6Materialize(payloadArg) => List(10,payloadArg.type_id,payloadArg.ctor_id)
          .concat(c6EncodeSlice(payloadArg.fields))
    }
    c6Pad(usedLanes,17)
  }
  pure def c6ExhaustTag(reasonArg:C6Exhaust):int = match reasonArg {
    | C6Fuel => 0 | C6DataNodes => 1 | C6DataFields => 2 | C6EnvironmentCells => 3
    | C6HelperTasks => 4 | C6HelperWork => 5 | C6SerializationBytes => 6
  }
  pure def c6EncodeOutcome(outcomeArg:C6Outcome):List[int] = match outcomeArg {
    | C6Running => List(0,0,0)
    | C6Halted(refArg) => List(1).concat(c6EncodeRef(refArg))
    | C6Rejected(codeArg) => List(2,codeArg,0)
    | C6Exhausted(reasonArg) => List(3,c6ExhaustTag(reasonArg),0)
  }
  pure def c6EncodeAlloc(allocArg:C6Alloc):List[int] =
    List(allocArg.data_nodes,allocArg.data_fields,allocArg.environment_cells)
  pure def c6EncodeActive(activeArg:C6Active):List[int] =
    List(activeArg.data,activeArg.data_fields,activeArg.env,activeArg.value_slots)
  pure def c6EncodeSection(sectionArg:C6Section):List[int] = {
    val header = List(C6_MAGIC,1).concat(c6EncodeControl(sectionArg.control))
      .append(sectionArg.control_origin).concat(c6EncodeEnv(sectionArg.env_root))
      .append(sectionArg.stack_limit).concat(c6EncodeOutcome(sectionArg.outcome))
      .concat(List(sectionArg.initial_fuel,sectionArg.remaining_work,sectionArg.consumed_work,
        sectionArg.helper_work,sectionArg.helper_visits,sectionArg.reference_cursor))
      .concat(c6EncodeAlloc(sectionArg.alloc)).concat(c6EncodeActive(sectionArg.active))
      .concat(c6EncodeActive(sectionArg.seg_active)).concat(c6EncodeAlloc(sectionArg.seg_alloc))
      .append(sectionArg.seg_consumed_work).append(sectionArg.frames.length())
    // Raw Core stack order is preserved; top=last, no reverse/canonicalization.
    sectionArg.frames.foldl(header,(lanesArg,frameArg) => lanesArg.concat(c6EncodeFrame(frameArg)))
  }
}
