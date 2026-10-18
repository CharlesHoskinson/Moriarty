# Declaration-only decoded observation field contract

This describes names/order and representations, not algorithms, observations or expected results. Raw HostSnapshot and ObservedStep remain complete and unchanged. No A/B clock conversion is implied.

DecodedSnapshot.fields order:
`constants, constant_fields, data, data_fields, environment_cells, current_environment, frames, control, helper, tasks, result, value_slots, outcome, names, expressions, parameters, arms, patterns, occurrences, occurrence_paths, children`.

`metadata` retains complete AdmissionSnapshot; RunIdC; control_origin; env_root; stack_limit; initial_fuel; remaining_work; consumed_work; helper_work; helper_visits; reference_cursor; AllocCounters; ActiveLengths; SegBase; and six actual image table lengths in data/data_fields/env/frames/tasks/value_slots order. Original function rows and complete program/layout/identity fields remain in metadata.admission.

constants/data/value_slots: Sequence(Values), in original table/slot order. Every value is a fully unfolded flat occurrence tree with postorder children preceding their parents; root is the final node. Node payloads: UInt(u128), Bool(bool), Nil, U128OutOfRange, Cons(head,tail), Leaf(value), Branch(left,right). Child indices are local decoded node indices. Const/Data arena indices and marker_free remain in raw snapshots. Repeated occurrences remain repeated.

constant_fields/data_fields: Sequence(References), retaining raw ordered RefC. environment_cells: Sequence(Environments), one decoded chain per original env cell row. current_environment: Environment. Each environment is ordered newest-to-oldest and retains shadowed entries; each entry has binder index, literal name, decoded value. None root is an empty chain; raw tags distinguish failed binding from successful empty binding.

frames: Sequence(Frames), TOP-FIRST (reverse of raw B storage order). Each element is Payload(Frame). Raw frame tags/scalars/origins retained. Ordered resolved fields:
- LetF: binder, body, saved_env, origin.
- LeftF: right, saved_env, origin.
- RightF: left, origin.
- UnaryF: origin.
- IfF: then_expr, else_expr, saved_env, origin.
- MatchF: arms, saved_env, origin.
- ArgsF: function_name, completed, rest, saved_env, origin.
- ReturnF, ScopeF: saved_env, origin.

control: Payload(Control). Ordered resolved fields:
- Eval: expr. Return: value.
- ScheduleNext: frame, next, env. frame is Payload(Frame) with the contract above.
- Invoke: function_name, args.
- Enter: function when Some, then args.
- Bound: binding when Ok, then body. Successful empty binding has an empty Environment field; Bad has no binding field.
- Cases: value, arms, saved_env.
- CaseResult: matching when Matched, then value, body, rest, saved_env.
- PrimResult Value: value; PrimResult Error and Error: no resolved fields.

helper: Payload(Helper), raw Option preserved; None has no fields. Ordered resolved fields:
- Scalar: operand_a, then operand_b when Some.
- EnvGet: binder, cursor, then found when Some.
- EnvPut: binder, value, base, new_root.
- Find: function_name, then found when Some.
- Bind: function, params, args, partial, dup_cursor.
- Append: src, value, then staged when Some.
- Validate: root. Equal: left, right.
- Match: scrutinee, arm, saved_env, partial; only after phase Arm, x for PVar/PLeaf and x then y for PCons/PBranch; no x/y resolved fields during Arm or for PNil/reserved tags; then staged when Some. The entry PVar/x=0 placeholders remain raw until the actual Arm read.

All helper common fields, phases, staged immediate scalar values, statuses, cursors, optional tags, partial state and parent_resume remain exact in raw InputValue::Helper; no erased or invented payload.

tasks: Sequence(Tasks), original task storage order. Payload(Task) fields: ValidateTask r; EqualTask left,right; PairTask binder,arg. Raw task visit/cursor/sort preserved.

result: Payload(Result), raw Option preserved. Ordered fields: EnvFound/ScalarValue/EqualDone value; EnvUpdated cell; FindFound function; BindOk/MatchMatched env; AppendDone seq. Other variants and None have no resolved fields and keep exact raw tags/scalars.

outcome: Payload(Outcome). Halted has value; Running/Rejected/Exhausted have no resolved fields and retain exact raw reason/code/tag.

Resolved name fields are Source(Name), expression fields Source(Expression), origin fields Source(Occurrence), function fields Source(Function), arm fields Source(Arm). Saved/current env/cursor/new_root fields are Environment; value/ref fields are Value. params is Sequence(Parameters); args/completed/src/seq are Sequence(Values); rest in ArgsF is Sequence(Expressions); arms/rest in match controls are Sequence(Arms). Static names/expressions/parameters/arms/patterns/occurrences/paths/children outputs retain original table order and exact Source payloads. No source identity or provenance is inferred from display text.

WitnessEvaluation contains decoded and diagnostic counters. MaterializedSnapshot contains raw, plan and evaluation. PlannedObservedStep contains observed plus before and after MaterializedSnapshot. Any materialization error follows the already-observed fixed step and retains the complete raw ObservedStep; it is not rollback or an owner semantic result.

Public module path after Root registration: `mori_core::sherlock::helper_machine::witness_plan`. The producer is `mori_core::sherlock::helper_machine::complete_observation::record_step_with_plan`. IDs and stable_topological_order positions are zero-based. Definition.last_use_order is the last consuming order POSITION, or the producer position when unused. A live-out has last_use_order equal to stable_topological_order.len(), denoting boundary publication after all nodes, rather than a node index. Definitions retain duplicate consumer edges in stable evaluation order then dependency order. No last-use reclamation or cache is performed.
