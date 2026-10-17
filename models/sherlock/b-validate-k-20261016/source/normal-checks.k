requires "validate.k"

module SHERLOCK-B-VALIDATE-NORMAL-CHECKS
  imports SHERLOCK-B-VALIDATE-EVENTS
  // Source-only relational checks; no frozen oracle/QA or host output used.
  // Clients can apply bvNext once and inspect exact projected pre/post images.
  syntax Bool ::= bvClockLaw(BVClocks, BVClocks, String) [function]
  rule bvClockLaw(bvClocks(F, C, H, V, R), bvClocks(F2, C2, H2, V2, R2), "Helper")
    => F2 ==Int F -Int 1 andBool C2 ==Int C +Int 1
       andBool H2 ==Int H +Int 1 andBool R2 ==Int R
       andBool (V2 ==Int V orBool V2 ==Int V +Int 1)
  rule bvClockLaw(Clock, Clock, "Publish") => true
  rule bvClockLaw(bvClocks(F, C, H, V, R), bvClocks(F2, C2, H, V, R2), "Legacy")
    => F2 ==Int F -Int 1 andBool C2 ==Int C +Int 1 andBool R2 ==Int R +Int 1
  syntax Bool ::= bvProjectedPreservation(BVImage, BVTransition) [function]
  rule bvProjectedPreservation(
      bvImage(Fix, Ctrl, PreHelper, PreResult, PreTasks, PreClocks, bvRunning()),
      bvTransition(_, Kind,
        bvImage(Fix, Ctrl, PostHelper, PostResult, PostTasks, PostClocks, bvRunning())))
    => bvClockLaw(PreClocks, PostClocks, "Helper")
    requires Kind ==String "HelperEnter" orBool Kind ==String "HelperVisit"
       orBool Kind ==String "HelperExit"
  rule bvProjectedPreservation(
      bvImage(Fix, bvEval(0), bvNoHelper(), bvValidated(true), bvTasksNil(), Pre, bvRunning()),
      bvTransition("k.machine.eval.literal.valid", "LegacyComplete",
        bvImage(Fix, bvReturn(_), bvNoHelper(), bvNoResult(), bvTasksNil(), Post, bvRunning())))
    => bvClockLaw(Pre, Post, "Legacy")
  rule bvProjectedPreservation(
      bvImage(Fix, bvReturn(Ref), bvNoHelper(), bvNoResult(), bvTasksNil(), Clock, bvRunning()),
      bvTransition("k.admin.publish.value", "Publish",
        bvImage(Fix, bvReturn(Ref), bvNoHelper(), bvNoResult(), bvTasksNil(), Clock, bvHalted(Ref))))
    => bvClockLaw(Clock, Clock, "Publish")
  rule bvProjectedPreservation(_, _) => false [owise]
  syntax Bool ::= bvCheckNext(BVImage) [function]
  rule bvCheckNext(Image) => bvProjectedPreservation(Image, bvNext(Image))
  // Scalar, exact remaining zero at publication; unary/two-field and repeated
  // child-reference traversal inputs cover all positive traversal branches.
  syntax BVRequest ::= bvScalarInput() [function] | bvSharedTreeInput() [function]
                     | bvListInput() [function] | bvBoolInput() [function]
  rule bvScalarInput()
    => bvRequest(bvRowsCons(bvUInt(7), bvRowsNil()), bvConst(0), 5, 0, 4, 1)
  rule bvSharedTreeInput()
    => bvRequest(bvRowsCons(bvUInt(7), bvRowsCons(bvLeaf(bvConst(0)),
         bvRowsCons(bvBranch(bvConst(1), bvConst(1)), bvRowsNil()))),
         bvConst(2), 20, 0, 20, 3)
  rule bvListInput()
    => bvRequest(bvRowsCons(bvUInt(3), bvRowsCons(bvNil(),
         bvRowsCons(bvCons(bvConst(0), bvConst(1)), bvRowsNil()))),
         bvConst(2), 12, 0, 12, 3)
  rule bvBoolInput()
    => bvRequest(bvRowsCons(bvBool(true), bvRowsNil()), bvConst(0), 5, 0, 4, 1)
  syntax Bool ::= bvNormalInputs() [function]
  rule bvNormalInputs() => bvNormal(bvScalarInput()) andBool bvNormal(bvSharedTreeInput())
      andBool bvNormal(bvListInput()) andBool bvNormal(bvBoolInput())
endmodule
