module q42EnvGetTraversalInputs {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  // Fresh manual complete typed carriers, UNADMITTED; no old actual/expected data.
  pure val q42Bounds:C6MBounds = {code_rows:16,function_rows:4,code_child_slots:16,
    parameter_slots:8,pattern_rows:8,match_plan_rows:8,match_arm_rows:8,type_rows:4,
    constructor_rows:4,constructor_field_slots:8,max_arity:8,constant_nodes:8,
    constant_field_slots:8,data_nodes:8,data_field_slots:8,environment_cells:8,frames:8,
    helper_tasks:8,serialization_bytes:65536,max_reference_work:128,max_helper_work:256,step_cap:128}
  pure val q42Identity:C6MIdentity = {profile_id:0,grammar_domain:C6MDedicatedSorts,
    finance:C6MFinanceAbsent,layout_version:{first:List(113,52,50),second:List(49)},
    layout_digest:C6MUnselected,bounds_digest:C6MUnselected,field_id:C6MNoPair,
    program_membership_digest:C6MUnselected}
  pure val q42Layout:C6MLayout = {layout_version:q42Identity.layout_version,
    field_id:C6MNoPair,widths:C6MNoWidths,scalar_layout_id:C6MNoPair,bounds:q42Bounds,
    capacities:{name_rows:8,name_bytes:64,occurrence_path_slots:16,value_slots:8},
    boundary_capacities_present:true}
  pure val q42Program:C6PProgram = {entry:0,
    expressions:List(C6PELet({binder:0,value:1,body:2}),C6PELiteral({constant:0}),
      C6PELet({binder:1,value:3,body:4}),C6PEVar({binder:0}),C6PEVar({binder:0})),
    occurrences:List({owner_is_function:false,function:0,path:{start:0,len:0}},
      {owner_is_function:false,function:0,path:{start:0,len:1}},
      {owner_is_function:false,function:0,path:{start:1,len:1}},
      {owner_is_function:false,function:0,path:{start:2,len:2}},
      {owner_is_function:false,function:0,path:{start:4,len:2}}),
    occurrence_paths:List(0,1,1,0,1,1),children:List(),functions:List(),parameters:List(),
    arms:List(),patterns:List(),pattern_children:List(),types:List(),type_args:List(),
    constructors:List(),ctor_fields:List(),decl_param_counts:List(),decl_ctor_counts:List(),
    decl_templates:List(),plan_nodes:List(),plan_cases:List(),field_paths:List(),
    constants:List(C6PKUInt({marker_free:true,n:7}),C6PKUInt({marker_free:true,n:99})),
    constant_fields:List(),names:List(List(120),List(121),List(122))}
  pure val q42OneHopBefore:C6WholeHost = {
    admission:{identity:q42Identity,bounds:q42Bounds,view:q42Program,layout:q42Layout},
    image:{run_id:{identity:q42Identity,fuel_input:C6MNat(128),stack_limit_input:C6MNat(8)},
      control:C6Eval({expr:4}),control_origin:4,env_root:C6EnvSome(2),stack_limit:8,
      outcome:C6Running,initial_fuel:128,remaining_work:91,consumed_work:37,
      helper_work:11,helper_visits:9,reference_cursor:13,
      alloc:{data_nodes:2,data_fields:0,environment_cells:3},
      active:{data:2,data_fields:0,env:3,value_slots:1},
      helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,
        origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,
        cursor:C6HSomeIndex(2),found:C6HNoRef,status:C6HTriStatusPending})),result:C6HNoResult,
      seg_base:{active:{data:0,data_fields:0,env:0,value_slots:0},
        alloc:{data_nodes:0,data_fields:0,environment_cells:0},consumed_work:0},
      data:List(C6HU128({n:13}),C6HU128({n:21})),data_fields:List(),
      env:List({binder:0,value:C6Const(0),previous:C6EnvNone},
        {binder:0,value:C6Data(0),previous:C6EnvNone},
        {binder:1,value:C6Const(1),previous:C6EnvSome(0)}),
      frames:List(C6ScopeF({saved_env:C6EnvNone,origin:0}),
        C6ScopeF({saved_env:C6EnvSome(0),origin:2})),tasks:List(),value_slots:List(C6Data(1))}}
  pure val q42TwoHopBefore:C6WholeHost = {...q42OneHopBefore,image:{...q42OneHopBefore.image,
    env_root:C6EnvSome(3),remaining_work:87,consumed_work:41,helper_work:17,helper_visits:14,
    alloc:{data_nodes:2,data_fields:0,environment_cells:4},
    active:{data:2,data_fields:0,env:4,value_slots:1},
    helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,
      origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,
      cursor:C6HSomeIndex(3),found:C6HNoRef,status:C6HTriStatusPending})),
    env:List({binder:0,value:C6Const(0),previous:C6EnvNone},
      {binder:1,value:C6Const(1),previous:C6EnvSome(0)},
      {binder:0,value:C6Data(0),previous:C6EnvNone},
      {binder:2,value:C6Data(1),previous:C6EnvSome(1)})}}
}
