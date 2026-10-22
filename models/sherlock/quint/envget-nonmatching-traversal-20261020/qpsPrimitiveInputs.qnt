module qpsPrimitiveInputs {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import qpsPrimitiveTypes.* from "./qpsPrimitiveTypes"
  import q42EnvGetTraversalInputs.* from "./q42EnvGetTraversalInputs"
  // Typed synthetic inputs: no claim globalSI/admission/genuine owner prefix.
  pure def qpsInput(controlArg:C6Control,fuelArg:int,emptyStack:bool):C6WholeHost =
    {...q42OneHopBefore,image:{...q42OneHopBefore.image,control:controlArg,
      control_origin:4,helper:C6HNoHelper,result:C6HNoResult,tasks:List(),
      remaining_work:fuelArg,consumed_work:128-fuelArg,
      frames:(if(emptyStack) List() else q42OneHopBefore.image.frames),
      data_fields:List(C6Data(0),C6Const(1)),
      active:{...q42OneHopBefore.image.active,data_fields:2},
      alloc:{...q42OneHopBefore.image.alloc,data_fields:2},
      seg_base:{active:{data:1,data_fields:1,env:1,value_slots:1},
        alloc:{data_nodes:1,data_fields:1,environment_cells:1},consumed_work:11}}}
  pure val qpsRefs:List[C6Ref]=List(C6Const(0),C6Data(0),C6Const(1),C6Data(1))
  pure val qpsChargedCases:List[QPSCase]=List(1,3).foldl(List(),(prior,fuelArg)=>
    prior.concat(qpsRefs.foldl(List(),(rows,r)=>rows.append({expected:QPSValue,
      before:qpsInput(C6PrimResult({result:C6PrimitiveValue(r)}),fuelArg,false)})))
      .concat(List(10,11).foldl(List(),(rows,codeArg)=>rows.append({expected:QPSError,
      before:qpsInput(C6PrimResult({result:C6PrimitiveError(codeArg)}),fuelArg,true)}))))
  // Codes0/255 are full-u8 local publication diagnostics, not valid SI16 owners.
  pure val qpsPublishCases:List[QPSCase]=List(0,1,3).foldl(List(),(prior,fuelArg)=>
    prior.concat(List(0,10,11,255).foldl(List(),(rows,codeArg)=>rows.append({expected:QPSPublishError,
      before:qpsInput(C6Error({code:codeArg}),fuelArg,false)}))))
  pure val qpsFuelCases:List[QPSCase]=List(
    {expected:QPSFuel,before:qpsInput(C6PrimResult({result:C6PrimitiveValue(C6Data(0))}),0,false)},
    {expected:QPSFuel,before:qpsInput(C6PrimResult({result:C6PrimitiveError(10)}),0,false)},
    {expected:QPSFuel,before:qpsInput(C6Return({value:C6Const(0)}),0,false)},
    {expected:QPSFuel,before:qpsInput(C6Eval({expr:4}),0,false)})
  pure val qpsCases=qpsChargedCases.concat(qpsPublishCases).concat(qpsFuelCases)
  pure val qpsMaxClock=(2^128)-1
  pure val qpsFreeMaxBefore:C6WholeHost={...qpsInput(C6Error({code:11}),0,false),
    image:{...qpsInput(C6Error({code:11}),0,false).image,consumed_work:qpsMaxClock,
      reference_cursor:qpsMaxClock,helper_work:qpsMaxClock,helper_visits:qpsMaxClock}}
}
