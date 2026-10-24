module c6PositiveDataInputs {
  import c6PhysicalTypes.* from "./c6PhysicalTypes"
  import c6PositiveDataTypes.* from "./c6PositiveDataTypes"
  // Static transcription of the frozen public INPUT ONLY. No output lanes.
  pure val inputProfile:str = "sherlock-c6-data-positive-padded/1"
  pure val inputCaseId:str = "seven-variants-partial-unequal-capacities-01"
  pure val inputGate:int = 1
  pure val inputCaps:C6DataRelevantCaps = {max_data_nodes:8,max_data_field_slots:5,
    max_env_cells:4,max_value_slots:3,max_constants:6,max_types:10,
    max_constructors:11,max_names:9,max_arity:3}
  pure val inputTables:C6DataTables = {
    data:List(C6DataU128({n:340282366920938463463374607431768211455}),
      C6DataBool({b:true}),C6DataNil,
      C6DataCons({fields:{start:0,len:2}}),
      C6DataLeaf({fields:{start:2,len:1}}),
      C6DataBranch({fields:{start:4,len:0}}),
      C6DataConstructor({type_id:9,ctor_id:10,fields:{start:1,len:2}})),
    data_fields:List(C6Const(5),C6Data(7),C6Const(0),C6Data(1)),
    env:List({binder:8,value:C6Data(6),previous:C6EnvNone},
      {binder:0,value:C6Const(5),previous:C6EnvSome(0)},
      {binder:3,value:C6Data(7),previous:C6EnvSome(3)}),
    value_slots:List(C6Data(7),C6Const(5))}
}
