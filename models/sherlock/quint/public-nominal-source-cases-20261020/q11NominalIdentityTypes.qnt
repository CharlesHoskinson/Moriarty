module q11NominalIdentityTypes {
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import nominalPublicSimulation.* from "./nominalPublicSimulation"

  // Names are externally supplied exact UTF-8 bytes; IDs are Core usize indices.
  // No frontend digest, source parser, admission or malformed-input policy.
  type Q11FieldBinding = {row_id:int,ordinal:int,raw:C6PCtorField,name:List[int]}
  type Q11CtorBinding = {ctor_id:int,raw:C6PCtorRow,name:List[int],fields:List[Q11FieldBinding]}
  type Q11TypeBinding = {type_id:int,raw:C6PTypeRow,name:List[int],args:List[int],constructors:List[int]}
  type Q11Bindings = {retained_view:C6PProgram,types:List[Q11TypeBinding],constructors:List[Q11CtorBinding]}
  type Q11ReadNode = {token:Token,children:List[C6Ref]}
  type Q11ReadState = {pending:List[C6Ref],tokens:Value}
  type Q11ReadTables = {data:List[C6HDataRow],data_fields:List[C6Ref]}
  type Q11ReadEvidence = {retained_view:C6PProgram,retained_image:C6HostImage,
    bindings:Q11Bindings,root:C6Ref,structural_read:Q11ReadState}
  type Q11TypeMatch = {pending:List[int],holds:bool}
}
