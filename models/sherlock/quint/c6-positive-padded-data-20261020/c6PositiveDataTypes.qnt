module c6PositiveDataTypes {
  import c6PhysicalTypes.* from "./c6PhysicalTypes"
  // Positive DATA vocabulary only; no validation/refusal/admission policy.
  type C6DataNode =
    | C6DataU128({n:int})
    | C6DataBool({b:bool})
    | C6DataNil
    | C6DataCons({fields:C6Slice})
    | C6DataLeaf({fields:C6Slice})
    | C6DataBranch({fields:C6Slice})
    | C6DataConstructor({type_id:int,ctor_id:int,fields:C6Slice})
  type C6DataEnvCell = {binder:int,value:C6Ref,previous:C6Env}
  type C6DataTables = {data:List[C6DataNode],data_fields:List[C6Ref],
    env:List[C6DataEnvCell],value_slots:List[C6Ref]}
  // Only the existing DATA consumer's nine relevant fields, not full Bounds.
  type C6DataRelevantCaps = {max_data_nodes:int,max_data_field_slots:int,
    max_env_cells:int,max_value_slots:int,max_constants:int,max_types:int,
    max_constructors:int,max_names:int,max_arity:int}
  type C6DataGeometry = {data_base:int,fields_base:int,env_base:int,
    values_base:int,total:int}
  type C6DataPositiveObservation = {geometry:C6DataGeometry,counts:List[int],
    padded_lanes:List[int],decoded_tables:C6DataTables}
}
