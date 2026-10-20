module c6PositiveProgramTypes {
  // Positive structural carriers; byte/preorder projection is external.
  type C6POption = C6PNone | C6PSome(int)
  type C6PSlice = {start:int,len:int}
  type C6PBinary = C6PAdd | C6PSub | C6PMul | C6PEq | C6PLt | C6PCons | C6PBranch
  type C6PUnary = C6PLeaf | C6PNot
  type C6PExpr =
    | C6PELiteral({constant:int}) | C6PEVar({binder:int})
    | C6PELet({binder:int,value:int,body:int})
    | C6PEBinary({op:C6PBinary,left:int,right:int})
    | C6PEUnary({op:C6PUnary,operand:int})
    | C6PEIf({cond:int,then_expr:int,else_expr:int})
    | C6PECall({function_name:int,args:C6PSlice})
    | C6PEMatch({scrutinee:int,arms:C6PSlice,plan:C6POption})
    | C6PEConstruct({type_id:int,ctor_id:int,fields:C6PSlice,constant:C6POption})
  type C6POccurrence = {owner_is_function:bool,function:int,path:C6PSlice}
  type C6PFunction = {function_name:int,params:C6PSlice,return_type:int,body:int}
  type C6PParameter = {binder:int,sort:int}
  type C6PArm = {pattern:int,body:int}
  type C6PPattern =
    | C6PPVar({x:int}) | C6PPNil | C6PPCons({x:int,y:int})
    | C6PPLeaf({x:int}) | C6PPBranch({x:int,y:int}) | C6PWildcard
    | C6PPConstructor({type_id:int,ctor_id:int,children:C6PSlice})
  type C6PTypeRow = {name_id:int,decl:C6POption,args:C6PSlice,ctor_start:int,ctor_len:int}
  type C6PCtorRow = {owning_type:int,name_id:int,source_order:int,field_start:int,field_len:int}
  type C6PCtorField = {name_id:int,type_id:int}
  type C6PTypeToken = C6PTUInt | C6PTBool | C6PTList | C6PTTree
    | C6PTNamed({name:List[int]}) | C6PTParam({name:List[int]})
    | C6PTApply({name:List[int],arity:int})
  type C6PDeclField = {name:List[int],ty:List[C6PTypeToken]}
  type C6PCtorDecl = {name:List[int],fields:List[C6PDeclField]}
  type C6PDecl = {name:List[int],parameters:List[List[int]],ctors:List[C6PCtorDecl]}
  type C6PPlan =
    | C6PSwitch({source_match:int,path:C6PSlice,type_id:int,cases:C6PSlice,default:int})
    | C6PBind({source_match:int,path:C6PSlice,binder:int,next:int})
    | C6PSelect({source_match:int,arm:int}) | C6PNoMatch({source_match:int})
  type C6PConstant =
    | C6PKUInt({marker_free:bool,n:int}) | C6PKBool({marker_free:bool,b:bool})
    | C6PKNil({marker_free:bool})
    | C6PKCons({marker_free:bool,fields:C6PSlice})
    | C6PKLeaf({marker_free:bool,fields:C6PSlice})
    | C6PKBranch({marker_free:bool,fields:C6PSlice})
    | C6PKConstructor({marker_free:bool,type_id:int,ctor_id:int,fields:C6PSlice})
    | C6PKOutOfRange({marker_free:bool})
  type C6PProgram = {
    entry:int,expressions:List[C6PExpr],occurrences:List[C6POccurrence],
    occurrence_paths:List[int],children:List[int],functions:List[C6PFunction],
    parameters:List[C6PParameter],arms:List[C6PArm],patterns:List[C6PPattern],
    pattern_children:List[int],types:List[C6PTypeRow],type_args:List[int],
    constructors:List[C6PCtorRow],ctor_fields:List[C6PCtorField],
    decl_param_counts:List[int],decl_ctor_counts:List[int],decl_templates:List[C6PDecl],
    plan_nodes:List[C6PPlan],plan_cases:List[C6POption],field_paths:List[int],
    constants:List[C6PConstant],constant_fields:List[int],names:List[List[int]],
  }
  type C6PCodecCaps = {
    max_frames:int,max_expressions:int,max_names:int,max_arms:int,max_children:int,
    max_value_slots:int,max_constants:int,max_data_nodes:int,max_env_cells:int,
    max_functions:int,max_arity:int,max_types:int,max_occurrences:int,
    max_occurrence_paths:int,max_parameters:int,max_patterns:int,
    max_pattern_children:int,max_constructors:int,max_ctor_fields:int,
    max_decl_templates:int,max_plan_nodes:int,max_plan_cases:int,max_field_paths:int,
    max_constant_fields:int,max_name_bytes:int,max_type_expr_nodes:int,max_template_bytes:int,
  }
  type C6PInputCase = {case_id:str,caps:C6PCodecCaps,program:C6PProgram}
  type C6PPathStep = C6PFieldStep(str) | C6PIndexStep(int)
  type C6PRowSpan = {row_index:int,start:int,end:int}
  type C6PTableSpan = {
    table:str,count_lane:int,row_count:int,rows_start:int,rows_end:int,row_spans:List[C6PRowSpan],
  }
  type C6PTextSpan = {
    table:str,row_index:int,path:List[C6PPathStep],budget:str,length_lane:int,
    bytes_start:int,bytes_end:int,utf8_bytes:List[int],
  }
  type C6PObservation = {
    encoded_lanes:List[int],decoded_program:C6PProgram,table_counts:List[int],
    table_spans:List[C6PTableSpan],text_spans:List[C6PTextSpan],final_cursor:int,
  }
}
