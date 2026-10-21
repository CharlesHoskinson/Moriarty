module q11ActualMapQueueReturned {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"

  pure val mapActualReturned:C6WholeHost = {admission:{identity:{profile_id:2,grammar_domain:C6MGenericNominalExtensionReserved,finance:C6MFinanceAbsent,
layout_version:{first:List(110,111,109,105,110,97,108,45,112,117,98,108,105,99,45,99,111,109,109,97,110,100),second:List(49)},
layout_digest:C6MUnselected,bounds_digest:C6MUnselected,field_id:C6MNoPair,program_membership_digest:C6MUnselected},bounds:{code_child_slots:256,code_rows:256,constant_field_slots:128,constant_nodes:128,
constructor_field_slots:128,constructor_rows:64,data_field_slots:128,data_nodes:64,
environment_cells:64,frames:64,function_rows:8,helper_tasks:64,match_arm_rows:32,
match_plan_rows:64,max_arity:4,max_helper_work:100000,max_reference_work:100000,
parameter_slots:16,pattern_rows:128,serialization_bytes:1048576,step_cap:1,type_rows:64},layout:{boundary_capacities_present:false,bounds:{code_child_slots:256,code_rows:256,constant_field_slots:128,constant_nodes:128,
constructor_field_slots:128,constructor_rows:64,data_field_slots:128,data_nodes:64,
environment_cells:64,frames:64,function_rows:8,helper_tasks:64,match_arm_rows:32,
match_plan_rows:64,max_arity:4,max_helper_work:100000,max_reference_work:100000,
parameter_slots:16,pattern_rows:128,serialization_bytes:1048576,step_cap:1,type_rows:64},
capacities:{name_bytes:1024,name_rows:64,occurrence_path_slots:1024,value_slots:4096},
field_id:C6MNoPair,layout_version:{first:List(110,111,109,105,110,97,108,45,112,117,98,108,105,99,45,99,111,109,109,97,110,100),second:List(49)},
scalar_layout_id:C6MNoPair,widths:C6MNoWidths},view:{
    arms:List({pattern:0, body:8}, {pattern:3, body:9}),
    children:List(1, 2, 3, 4, 5, 10, 13, 14, 16, 18, 17),
    constant_fields:List(),
    constants:List(C6PKUInt({marker_free:true, n:3}), C6PKUInt({marker_free:true, n:8}), C6PKUInt({marker_free:true, n:2}), C6PKBool({b:true, marker_free:true}), C6PKConstructor({ctor_id:4, fields:{start:0, len:0}, marker_free:true, type_id:4}), C6PKConstructor({ctor_id:4, fields:{start:0, len:0}, marker_free:true, type_id:4})),
    constructors:List({field_len:0, field_start:0, name_id:0, owning_type:2, source_order:0}, {field_len:2, field_start:0, name_id:0, owning_type:2, source_order:1}, {field_len:1, field_start:2, name_id:0, owning_type:3, source_order:0}, {field_len:2, field_start:3, name_id:0, owning_type:3, source_order:1}, {field_len:0, field_start:5, name_id:0, owning_type:4, source_order:0}, {field_len:2, field_start:5, name_id:2, owning_type:4, source_order:1}, {field_len:2, field_start:7, name_id:4, owning_type:5, source_order:0}),
    ctor_fields:List({name_id:0, type_id:0}, {name_id:0, type_id:2}, {name_id:0, type_id:0}, {name_id:0, type_id:3}, {name_id:0, type_id:3}, {name_id:12, type_id:0}, {name_id:6, type_id:4}, {name_id:13, type_id:4}, {name_id:5, type_id:1}),
    decl_ctor_counts:List(2, 1),
    decl_param_counts:List(1, 2),
    decl_templates:List({name:List(81, 117, 101, 117, 101), parameters:List(List(84)), ctors:List({name:List(69, 109, 112, 116, 121), fields:List()}, {name:List(80, 117, 115, 104), fields:List({name:List(122, 95, 105, 116, 101, 109), ty:List(C6PTParam({name:List(84)}))}, {name:List(97, 95, 116, 97, 105, 108), ty:List(C6PTApply({name:List(81, 117, 101, 117, 101), arity:1}), C6PTParam({name:List(84)}))})})}, {name:List(69, 110, 118, 101, 108, 111, 112, 101), parameters:List(List(65), List(66)), ctors:List({name:List(87, 114, 97, 112), fields:List({name:List(122, 95, 105, 116, 101, 109, 115), ty:List(C6PTParam({name:List(65)}))}, {name:List(97, 95, 109, 97, 114, 107, 101, 114), ty:List(C6PTParam({name:List(66)}))})})}),
    entry:0,
    expressions:List(C6PECall({args:{start:0, len:1}, function_name:9}), C6PEConstruct({constant:C6PNone, ctor_id:5, fields:{start:1, len:2}, type_id:4}), C6PELiteral({constant:0}), C6PEConstruct({constant:C6PNone, ctor_id:5, fields:{start:3, len:2}, type_id:4}), C6PELiteral({constant:1}), C6PEConstruct({constant:C6PSome(4), ctor_id:4, fields:{start:0, len:0}, type_id:4}), C6PEMatch({arms:{start:0, len:2}, plan:C6PSome(0), scrutinee:7}), C6PEVar({binder:11}), C6PEConstruct({constant:C6PSome(5), ctor_id:4, fields:{start:0, len:0}, type_id:4}), C6PEConstruct({constant:C6PNone, ctor_id:5, fields:{start:5, len:2}, type_id:4}), C6PEBinary({left:11, op:C6PAdd, right:12}), C6PEVar({binder:8}), C6PELiteral({constant:2}), C6PECall({args:{start:7, len:1}, function_name:7}), C6PEVar({binder:10}), C6PEConstruct({constant:C6PNone, ctor_id:6, fields:{start:8, len:2}, type_id:5}), C6PECall({args:{start:10, len:1}, function_name:7}), C6PEVar({binder:11}), C6PELiteral({constant:3})),
    field_paths:List(0, 1),
    functions:List({function_name:7, params:{start:0, len:1}, return_type:4, body:6}, {function_name:9, params:{start:1, len:1}, return_type:5, body:15}),
    names:List(List(69, 109, 112, 116, 121), List(69, 110, 118, 101, 108, 111, 112, 101), List(80, 117, 115, 104), List(81, 117, 101, 117, 101), List(87, 114, 97, 112), List(97, 95, 109, 97, 114, 107, 101, 114), List(97, 95, 116, 97, 105, 108), List(98, 117, 109, 112), List(105, 116, 101, 109), List(112, 97, 99, 107), List(116, 97, 105, 108), List(120, 115), List(122, 95, 105, 116, 101, 109), List(122, 95, 105, 116, 101, 109, 115)),
    occurrence_paths:List(0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 2, 2, 0, 2, 0, 0, 2, 0, 1, 2, 1, 2, 1, 0, 0, 0, 0, 1),
    occurrences:List({owner_is_function:false, function:0, path:{start:0, len:0}}, {owner_is_function:false, function:0, path:{start:0, len:1}}, {owner_is_function:false, function:0, path:{start:1, len:2}}, {owner_is_function:false, function:0, path:{start:3, len:2}}, {owner_is_function:false, function:0, path:{start:5, len:3}}, {owner_is_function:false, function:0, path:{start:8, len:3}}, {owner_is_function:true, function:0, path:{start:0, len:0}}, {owner_is_function:true, function:0, path:{start:11, len:1}}, {owner_is_function:true, function:0, path:{start:12, len:1}}, {owner_is_function:true, function:0, path:{start:13, len:1}}, {owner_is_function:true, function:0, path:{start:14, len:2}}, {owner_is_function:true, function:0, path:{start:16, len:3}}, {owner_is_function:true, function:0, path:{start:19, len:3}}, {owner_is_function:true, function:0, path:{start:22, len:2}}, {owner_is_function:true, function:0, path:{start:24, len:3}}, {owner_is_function:true, function:1, path:{start:0, len:0}}, {owner_is_function:true, function:1, path:{start:27, len:1}}, {owner_is_function:true, function:1, path:{start:28, len:2}}, {owner_is_function:true, function:1, path:{start:30, len:1}}),
    parameters:List({binder:11, sort:4}, {binder:11, sort:4}),
    pattern_children:List(1, 2),
    patterns:List(C6PPConstructor({children:{start:0, len:0}, ctor_id:4, type_id:4}), C6PPVar({x:8}), C6PPVar({x:10}), C6PPConstructor({children:{start:0, len:2}, ctor_id:5, type_id:4})),
    plan_cases:List(C6PSome(1), C6PSome(2)),
    plan_nodes:List(C6PSwitch({cases:{start:0, len:2}, default:5, path:{start:0, len:0}, source_match:6, type_id:4}), C6PSelect({arm:0, source_match:6}), C6PBind({binder:8, next:3, path:{start:0, len:1}, source_match:6}), C6PBind({binder:10, next:4, path:{start:1, len:1}, source_match:6}), C6PSelect({arm:1, source_match:6}), C6PNoMatch({source_match:6})),
    type_args:List(0, 4, 1),
    types:List({name_id:0, decl:C6PNone, args:{start:0, len:0}, ctor_start:0, ctor_len:0}, {name_id:0, decl:C6PNone, args:{start:0, len:0}, ctor_start:0, ctor_len:0}, {name_id:0, decl:C6PNone, args:{start:0, len:0}, ctor_start:0, ctor_len:2}, {name_id:0, decl:C6PNone, args:{start:0, len:0}, ctor_start:2, ctor_len:2}, {name_id:3, decl:C6PSome(0), args:{start:0, len:1}, ctor_start:4, ctor_len:2}, {name_id:1, decl:C6PSome(1), args:{start:1, len:2}, ctor_start:6, ctor_len:1})
  }},image:{
active:{data:7,data_fields:10,env:8,value_slots:19},alloc:{data_fields:10,data_nodes:7,environment_cells:8},
consumed_work:359,control:C6Return({value:C6Data(6)}),control_origin:0,
data:List(
C6HConstructor({type_id:4,ctor_id:5,fields:{start:0,len:2}}),
C6HConstructor({type_id:4,ctor_id:5,fields:{start:2,len:2}}),
C6HU128({n:5}),C6HU128({n:10}),
C6HConstructor({type_id:4,ctor_id:5,fields:{start:4,len:2}}),
C6HConstructor({type_id:4,ctor_id:5,fields:{start:6,len:2}}),
C6HConstructor({type_id:5,ctor_id:6,fields:{start:8,len:2}})),
data_fields:List(C6Const(1),C6Const(4),C6Const(0),C6Data(0),C6Data(3),C6Const(5),C6Data(2),C6Data(4),C6Data(5),C6Const(3)),
env:List(
{binder:11,value:C6Data(1),previous:C6EnvNone},
{binder:11,value:C6Data(1),previous:C6EnvNone},
{binder:8,value:C6Const(0),previous:C6EnvSome(1)},
{binder:10,value:C6Data(0),previous:C6EnvSome(2)},
{binder:11,value:C6Data(0),previous:C6EnvNone},
{binder:8,value:C6Const(1),previous:C6EnvSome(4)},
{binder:10,value:C6Const(4),previous:C6EnvSome(5)},
{binder:11,value:C6Const(4),previous:C6EnvNone}),
env_root:C6EnvNone,frames:List(),helper:C6HNoHelper,helper_visits:108,helper_work:249,
initial_fuel:4096,outcome:C6Halted(C6Data(6)),reference_cursor:110,remaining_work:3737,result:C6HNoResult,
run_id:{identity:{profile_id:2,grammar_domain:C6MGenericNominalExtensionReserved,finance:C6MFinanceAbsent,
layout_version:{first:List(110,111,109,105,110,97,108,45,112,117,98,108,105,99,45,99,111,109,109,97,110,100),second:List(49)},
layout_digest:C6MUnselected,bounds_digest:C6MUnselected,field_id:C6MNoPair,program_membership_digest:C6MUnselected},fuel_input:C6MNat(4096),stack_limit_input:C6MNat(64)},
seg_base:{active:{data:0,data_fields:0,env:0,value_slots:0},alloc:{data_fields:0,data_nodes:0,environment_cells:0},consumed_work:0},
stack_limit:64,tasks:List(),
value_slots:List(C6Const(0),C6Const(1),C6Const(1),C6Const(4),C6Const(0),C6Data(0),C6Data(1),C6Data(1),C6Data(2),C6Data(0),C6Data(3),C6Const(4),C6Data(3),C6Const(5),C6Data(2),C6Data(4),C6Data(5),C6Data(5),C6Const(3))
}}
  pure val mapActualRoot:C6Ref = C6Data(6)
  pure val mapAdmissionStatus:str = "admitted"
  pure val mapAdmissionRepresentation:str = "HostOnly"
}
