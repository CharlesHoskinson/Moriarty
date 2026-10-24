module qpsPrimitiveTest {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import qpsPrimitiveTypes.* from "./qpsPrimitiveTypes"
  import qpsPrimitiveInputs.* from "./qpsPrimitiveInputs"
  import qpsPrimitivePredicates.* from "./qpsPrimitivePredicates"
  import qpsPrimitiveRelation.* from "./qpsPrimitiveRelation"
  run qpsChargedPrimitivePayloadsTest = and {
    qpsChargedCases.length()==12,
    qpsChargedCases.foldl(true,(prior,c)=>{
      val computed=qpsApply(c.before)
      prior and qpsCanStep(c.before) and computed.kind==c.expected
        and qpsAfterPredicate(c.before,computed.after,c.expected)
        and computed.event==qpsExpectedEvent(c.before,c.expected)
    }),
    qpsApply(qpsInput(C6PrimResult({result:C6PrimitiveValue(C6Const(0))}),1,false)).after.image.control
      !=qpsApply(qpsInput(C6PrimResult({result:C6PrimitiveValue(C6Data(0))}),1,false)).after.image.control,
  }
  run qpsFreePublishPriorityAndMaxClockTest = {
    val maximal=qpsApply(qpsFreeMaxBefore)
    and {qpsPublishCases.length()==12,
      qpsPublishCases.foldl(true,(prior,c)=>{
        val computed=qpsApply(c.before)
        prior and computed.kind==QPSPublishError and qpsAfterPredicate(c.before,computed.after,c.expected)
          and computed.event==qpsExpectedEvent(c.before,c.expected)
      }),
      maximal.kind==QPSPublishError,qpsAfterPredicate(qpsFreeMaxBefore,maximal.after,QPSPublishError),
      maximal.event==qpsExpectedEvent(qpsFreeMaxBefore,QPSPublishError),
      maximal.after.image.consumed_work==qpsMaxClock,maximal.after.image.reference_cursor==qpsMaxClock,
      maximal.event.work_delta==0,maximal.event.reference_cursor_delta==0,
    }
  }
  run qpsFuelZeroSeparateFromAppliedTest = {
    val priority=qpsInput(C6Error({code:10}),0,false)
    val outside=qpsInput(C6Return({value:C6Data(0)}),0,true)
    and {qpsFuelCases.length()==4,
      qpsFuelCases.foldl(true,(prior,c)=>{
        val computed=qpsApply(c.before)
        prior and computed.kind==QPSFuel and qpsAfterPredicate(c.before,computed.after,QPSFuel)
          and computed.event==qpsExpectedEvent(c.before,QPSFuel)
          and computed.after.image.control==c.before.image.control
      }),
      qpsClassify(priority)==QPSPublishError,qpsApply(priority).after.image.outcome==C6Rejected(10),
      // Outside denotes unmodeled P1PublishValue, NEVER a Core refusal/error.
      qpsClassify(outside)==QPSOutside,
    }
  }
  run qpsTwoRowErrorPublicationHistoryTest = {
    val initial=qpsInput(C6PrimResult({result:C6PrimitiveError(10)}),1,true)
    val first=qpsApply(initial)
    val second=qpsApply(first.after)
    and {first.kind==QPSError,second.kind==QPSPublishError,
      qpsAfterPredicate(initial,first.after,QPSError),qpsAfterPredicate(first.after,second.after,QPSPublishError),
      first.event==qpsExpectedEvent(initial,QPSError),second.event==qpsExpectedEvent(first.after,QPSPublishError),
      first.after.image.outcome==C6Running,first.after.image.remaining_work==0,
      second.after.image.outcome==C6Rejected(10),second.after.image.control==C6Error({code:10}),
      second.after.image.consumed_work==initial.image.consumed_work+1,
      second.after.image.reference_cursor==initial.image.reference_cursor+1,
      second.event.step_id.ordinal==first.event.step_id.ordinal+1,
      second.event.work_delta==0,second.event.reference_cursor_delta==0,
      first.after.admission==initial.admission,second.after.admission==initial.admission,
    }
  }
  run qpsFullStateAndEventMutationSensitivityTest = {
    val initial=qpsInput(C6PrimResult({result:C6PrimitiveValue(C6Data(0))}),1,false)
    val good=qpsApply(initial)
    val free=qpsInput(C6Error({code:11}),0,false)
    val published=qpsApply(free)
    val zero=qpsFuelCases.nth(0).before
    val exhausted=qpsApply(zero)
    and {qpsAfterPredicate(initial,good.after,QPSValue),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,
        control:C6Return({value:C6Const(0)})}},QPSValue)),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,outcome:C6Rejected(10)}},QPSValue)),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,frames:List()}},QPSValue)),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,control_origin:0}},QPSValue)),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,env_root:C6EnvNone}},QPSValue)),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,data_fields:List()}},QPSValue)),
      not(qpsAfterPredicate(initial,{...good.after,image:{...good.after.image,helper_work:initial.image.helper_work+1}},QPSValue)),
      not(qpsAfterPredicate(free,{...published.after,image:{...published.after.image,
        consumed_work:published.after.image.consumed_work+1}},QPSPublishError)),
      {...good.event,site:Q41NoText}!=qpsExpectedEvent(initial,QPSValue),
      {...good.event,result_tag:Q41SomeText("ScalarValue")}!=qpsExpectedEvent(initial,QPSValue),
      {...published.event,site:Q41SomeText("Prim")}!=qpsExpectedEvent(free,QPSPublishError),
      {...published.event,reference_cursor_delta:1}!=qpsExpectedEvent(free,QPSPublishError),
      {...exhausted.event,disposition:"Applied"}!=qpsExpectedEvent(zero,QPSFuel),
      {...exhausted.event,work_delta:1}!=qpsExpectedEvent(zero,QPSFuel),
    }
  }
}
