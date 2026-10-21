module nominalPublicSourceTest {
  import nominalPublicSimulation.* from "./nominalPublicSimulation"
  import nominalPublicSourceInputs.* from "./nominalPublicSourceInputs"
  import nominalPublicSourceExpected.* from "./nominalPublicSourceExpected"
  // Finite driver ONLY calls the existing model's general advance; no alternate evaluator.
  pure def q10Trace(p:Program):List[Snapshot] =
    range(0,256).foldl(List(initial(p,256)),(traceArg,ignored) => {
      val lastArg = traceArg.nth(traceArg.length()-1)
      if(lastArg.outcome == Pending) traceArg.append(advance(lastArg)) else traceArg
    })
  pure def q10Final(traceArg:List[Snapshot]):Snapshot = traceArg.nth(traceArg.length()-1)
  pure def q10PrimitiveEvents(traceArg:List[Snapshot]):int =
    traceArg.foldl(0,(n,s) => n+(if(s.event == PrimitiveReturn) 1 else 0))
  pure def q10CompleteAccounting(traceArg:List[Snapshot]):bool =
    traceArg.foldl(true,(okArg,s) => okArg and and {
      s.remaining >= 0,s.consumed+s.remaining == 256,s.helperWork == 0,
      s.constructorNodes == s.builds.length(),
      s.constructorFieldSlots == s.builds.foldl(0,(n,b) => n+b.fields.length()),
      s.frames.length() <= FRAME_LIMIT,s.maxDepth <= FRAME_LIMIT,
      s.environment.keys().forall(n => validValue(s.program,s.environment.get(n))),
      s.builds.foldl(true,(good,b) => good and validValue(s.program,b.value)),
    })
  run q10MapQueueTypedReturnAndOrderTest = {
    val traceArg = q10Trace(q10MapProgram)
    val finalArg = q10Final(traceArg)
    and {finalArg.outcome == Done(q10MapExpected),q10CompleteAccounting(traceArg),
      finalArg.selectedArms == q10MapArms,finalArg.enteredArguments.length() == 4,
      finalArg.constructorNodes == 7,finalArg.constructorFieldSlots == 10,
      q10PrimitiveEvents(traceArg) == 2,
      q10TypeBindings.nth(1).args == List("Queue<UInt>","Bool"),
      q10FieldNames.get((10,1)) == List("z_item","a_tail"),
      finalArg.builds.nth(4).fields.head() == uint(10),
      finalArg.builds.nth(5).fields.head() == uint(5),
      finalArg.builds.nth(6).fields == List(fieldsOf(q10MapExpected).head(),List(BoolToken(true)))}
  }
  run q10MirrorArborTypedShapeAndOrderTest = {
    val traceArg = q10Trace(q10MirrorProgram)
    val finalArg = q10Final(traceArg)
    and {finalArg.outcome == Done(q10MirrorExpected),q10CompleteAccounting(traceArg),
      finalArg.selectedArms == q10MirrorArms,finalArg.enteredArguments.length() == 6,
      finalArg.constructorNodes == 11,finalArg.constructorFieldSlots == 16,
      q10PrimitiveEvents(traceArg) == 3,
      q10TypeBindings.nth(3).args == List("Arbor<Bool>","UInt"),
      q10FieldNames.get((30,1)) == List("z_left","a_right"),
      finalArg.builds.nth(5).fields == List(List(BoolToken(false))),
      finalArg.builds.nth(6).fields == List(List(BoolToken(true))),
      finalArg.builds.nth(8).fields == List(List(BoolToken(false))),
      finalArg.builds.nth(10).fields.nth(1) == uint(41)}
  }
  run q10SameTypeFieldSwapIsDistinguishedTest = {
    val finalArg = q10Final(q10Trace(q10MirrorProgram))
    match finalArg.outcome {
      | Done(v) => {
        val treeArg = fieldsOf(v).head()
        val childrenArg = fieldsOf(treeArg)
        val swappedArg = List(ConstructorToken({typeId:30,ctorId:1,arity:2}))
          .concat(childrenArg.nth(1)).concat(childrenArg.nth(0))
        and {v == q10MirrorExpected,
          childrenArg.nth(0).head() == ConstructorToken({typeId:30,ctorId:1,arity:2}),
          childrenArg.nth(1).head() == ConstructorToken({typeId:30,ctorId:0,arity:1}),
          validValue(q10MirrorProgram,swappedArg),swappedArg != treeArg}
      }
      | _ => false }
  }
  run q10ExistingCallBuildPathAndPreservationTest = {
    val left = q10Trace(q10MapProgram)
    val right = q10Trace(q10MirrorProgram)
    and {left.foldl(true,(okArg,s) => okArg and s.program == q10MapProgram),
      right.foldl(true,(okArg,s) => okArg and s.program == q10MirrorProgram),
      q10Final(left).environment == Map(),q10Final(left).frames == List(),
      q10Final(right).environment == Map(),q10Final(right).frames == List(),
      q10Final(left).event == TerminalPublication,q10Final(right).event == TerminalPublication,
      right.foldl(0,(n,s) => n+(if(s.event == UnaryEvent) 1 else 0)) == 3,
      left.foldl(0,(n,s) => n+(if(s.event == CallEnter) 1 else 0)) == 4,
      right.foldl(0,(n,s) => n+(if(s.event == CallEnter) 1 else 0)) == 6}
  }
}
