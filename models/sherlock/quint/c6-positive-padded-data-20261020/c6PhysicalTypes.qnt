module c6PhysicalTypes {
  // Positive owned-record vocabulary only. No admission or malformed-lane policy.
  type C6Ref = C6Const(int) | C6Data(int)
  type C6Env = C6EnvNone | C6EnvSome(int)
  type C6Function = C6FunctionNone | C6FunctionSome(int)
  type C6Slice = {start:int,len:int}
  type C6Binary = C6Add | C6Sub | C6Mul | C6Eq | C6Lt | C6Cons | C6Branch
  type C6Unary = C6Leaf | C6Not
  type C6Frame =
    | C6LetF({binder:int,body:int,saved_env:C6Env,origin:int})
    | C6LeftF({op:C6Binary,right:int,saved_env:C6Env,origin:int})
    | C6RightF({op:C6Binary,left:C6Ref,origin:int})
    | C6UnaryF({op:C6Unary,origin:int})
    | C6IfF({then_expr:int,else_expr:int,saved_env:C6Env,origin:int})
    | C6MatchF({arms:C6Slice,saved_env:C6Env,origin:int})
    | C6ArgsF({function_name:int,completed:C6Slice,rest:C6Slice,saved_env:C6Env,origin:int})
    | C6ReturnF({return_type:int,saved_env:C6Env,origin:int})
    | C6ScopeF({saved_env:C6Env,origin:int})
    | C6ConstructorFieldsF({type_id:int,ctor_id:int,index:int,field_type:int,
        checked:bool,completed:C6Slice,rest:C6Slice,saved_env:C6Env,origin:int})
  type C6Binding = C6BindingBad | C6BindingOk(C6Env)
  type C6Matching = C6NoMatch | C6Matched(C6Env)
  type C6Primitive = C6PrimitiveValue(C6Ref) | C6PrimitiveError(int)
  type C6Control =
    | C6Eval({expr:int})
    | C6Return({value:C6Ref})
    | C6ScheduleNext({frame:C6Frame,next:int,env:C6Env})
    | C6Invoke({function_name:int,args:C6Slice})
    | C6Enter({function:C6Function,args:C6Slice})
    | C6Bound({binding:C6Binding,return_type:int,body:int})
    | C6Cases({value:C6Ref,arms:C6Slice,saved_env:C6Env})
    | C6CaseResult({matching:C6Matching,value:C6Ref,body:int,rest:C6Slice,saved_env:C6Env})
    | C6PrimResult({result:C6Primitive})
    | C6Error({code:int})
    | C6Materialize({type_id:int,ctor_id:int,fields:C6Slice})
  type C6Exhaust = C6Fuel | C6DataNodes | C6DataFields | C6EnvironmentCells
    | C6HelperTasks | C6HelperWork | C6SerializationBytes
  type C6Outcome = C6Running | C6Halted(C6Ref) | C6Rejected(int) | C6Exhausted(C6Exhaust)
  type C6Alloc = {data_nodes:int,data_fields:int,environment_cells:int}
  type C6Active = {data:int,data_fields:int,env:int,value_slots:int}
  type C6Section = {control:C6Control,control_origin:int,env_root:C6Env,stack_limit:int,
    outcome:C6Outcome,initial_fuel:int,remaining_work:int,consumed_work:int,
    helper_work:int,helper_visits:int,reference_cursor:int,alloc:C6Alloc,active:C6Active,
    seg_active:C6Active,seg_alloc:C6Alloc,seg_consumed_work:int,frames:List[C6Frame]}
}
