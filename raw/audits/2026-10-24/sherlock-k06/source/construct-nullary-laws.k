requires "construct-nullary.k"

// Literal expected states from the PRE-PRODUCER manual oracle freeze.
// No expected post below calls cnStep, cnNullary, cnCharge or any producer.
module SHERLOCK-CONSTRUCT-NULLARY-LAWS
  imports SHERLOCK-CONSTRUCT-NULLARY
  syntax CNView ::= cnManualView() [function]
  rule cnManualView() => cnView(cnRow(cnLet(2,1,2),cnRow(cnConstruct(4,4,c6Slice(0,0),c6Some(0)),cnRow(cnVar(2),cnNoRows()))),cnRow(cnType(0,0),cnRow(cnType(0,0),cnRow(cnType(0,2),cnRow(cnType(2,2),cnRow(cnType(4,1),cnNoRows()))))),cnRow(cnCtor(2,0,0),cnRow(cnCtor(2,0,2),cnRow(cnCtor(3,2,1),cnRow(cnCtor(3,3,2),cnRow(cnCtor(4,5,0),cnNoRows()))))),cnRow(cnConstant(true,4,4,c6Slice(0,0)),cnNoRows()),5)
  syntax C6Raw ::= cnManualProgram() [function] | cnResidual7() [function] | cnResidual3() [function]
  rule cnManualProgram() => c6Raw("{\"bounds\":{\"code_child_slots\":8,\"code_rows\":8,\"constant_field_slots\":2,\"constant_nodes\":2,\"constructor_field_slots\":5,\"constructor_rows\":5,\"data_field_slots\":2,\"data_nodes\":2,\"environment_cells\":2,\"frames\":2,\"function_rows\":0,\"helper_tasks\":2,\"match_arm_rows\":0,\"match_plan_rows\":0,\"max_arity\":2,\"max_helper_work\":16,\"max_reference_work\":16,\"parameter_slots\":0,\"pattern_rows\":0,\"serialization_bytes\":4096,\"step_cap\":16,\"type_rows\":5},\"identity\":{\"bounds_digest\":\"Unselected\",\"field_id\":null,\"finance\":\"Absent\",\"grammar_domain\":\"GenericNominalExtensionReserved\",\"layout_digest\":\"Unselected\",\"layout_version\":[\"lv\",\"1\"],\"profile_id\":2,\"program_membership_digest\":\"Unselected\"},\"layout\":{\"boundary_capacities_present\":false,\"bounds\":{\"code_child_slots\":8,\"code_rows\":8,\"constant_field_slots\":2,\"constant_nodes\":2,\"constructor_field_slots\":5,\"constructor_rows\":5,\"data_field_slots\":2,\"data_nodes\":2,\"environment_cells\":2,\"frames\":2,\"function_rows\":0,\"helper_tasks\":2,\"match_arm_rows\":0,\"match_plan_rows\":0,\"max_arity\":2,\"max_helper_work\":16,\"max_reference_work\":16,\"parameter_slots\":0,\"pattern_rows\":0,\"serialization_bytes\":4096,\"step_cap\":16,\"type_rows\":5},\"capacities\":{\"name_bytes\":9,\"name_rows\":3,\"occurrence_path_slots\":2,\"value_slots\":8},\"field_id\":null,\"layout_version\":[\"lv\",\"1\"],\"scalar_layout_id\":null,\"widths\":null},\"view\":{\"arms\":[],\"children\":[],\"constant_fields\":[],\"constants\":[{\"Constructor\":{\"ctor_id\":4,\"fields\":{\"len\":0,\"start\":0},\"marker_free\":true,\"type_id\":4}}],\"constructors\":[{\"field_len\":0,\"field_start\":0,\"name_id\":0,\"owning_type\":2,\"source_order\":0},{\"field_len\":2,\"field_start\":0,\"name_id\":0,\"owning_type\":2,\"source_order\":1},{\"field_len\":1,\"field_start\":2,\"name_id\":0,\"owning_type\":3,\"source_order\":0},{\"field_len\":2,\"field_start\":3,\"name_id\":0,\"owning_type\":3,\"source_order\":1},{\"field_len\":0,\"field_start\":5,\"name_id\":0,\"owning_type\":4,\"source_order\":0}],\"ctor_fields\":[{\"name_id\":0,\"type_id\":0},{\"name_id\":0,\"type_id\":2},{\"name_id\":0,\"type_id\":0},{\"name_id\":0,\"type_id\":3},{\"name_id\":0,\"type_id\":3}],\"decl_ctor_counts\":[1],\"decl_param_counts\":[0],\"decl_templates\":[{\"ctors\":[{\"fields\":[],\"name\":\"Done\"}],\"name\":\"Unit\",\"parameters\":[]}],\"entry\":0,\"expressions\":[{\"Let\":{\"binder\":2,\"body\":2,\"value\":1}},{\"Construct\":{\"constant\":0,\"ctor_id\":4,\"fields\":{\"len\":0,\"start\":0},\"type_id\":4}},{\"Var\":{\"binder\":2}}],\"field_paths\":[],\"functions\":[],\"names\":[\"Done\",\"Unit\",\"x\"],\"occurrence_paths\":[0,1],\"occurrences\":[{\"function\":0,\"owner_is_function\":false,\"path\":{\"len\":0,\"start\":0}},{\"function\":0,\"owner_is_function\":false,\"path\":{\"len\":1,\"start\":0}},{\"function\":0,\"owner_is_function\":false,\"path\":{\"len\":1,\"start\":1}}],\"parameters\":[],\"pattern_children\":[],\"patterns\":[],\"plan_cases\":[],\"plan_nodes\":[],\"type_args\":[],\"types\":[{\"args\":{\"len\":0,\"start\":0},\"ctor_len\":0,\"ctor_start\":0,\"decl\":null,\"name_id\":0},{\"args\":{\"len\":0,\"start\":0},\"ctor_len\":0,\"ctor_start\":0,\"decl\":null,\"name_id\":0},{\"args\":{\"len\":0,\"start\":0},\"ctor_len\":2,\"ctor_start\":0,\"decl\":null,\"name_id\":0},{\"args\":{\"len\":0,\"start\":0},\"ctor_len\":2,\"ctor_start\":2,\"decl\":null,\"name_id\":0},{\"args\":{\"len\":0,\"start\":0},\"ctor_len\":1,\"ctor_start\":4,\"decl\":0,\"name_id\":1}]}}")
  rule cnResidual7() => c6Raw("{\"active\":{\"data\":0,\"data_fields\":0,\"env\":0,\"value_slots\":0},\"alloc\":{\"data_fields\":0,\"data_nodes\":0,\"environment_cells\":0},\"data\":[],\"data_fields\":[],\"env\":[],\"run_id\":{\"fuel_input\":{\"Nat\":7},\"identity\":{\"bounds_digest\":\"Unselected\",\"field_id\":null,\"finance\":\"Absent\",\"grammar_domain\":\"GenericNominalExtensionReserved\",\"layout_digest\":\"Unselected\",\"layout_version\":[\"lv\",\"1\"],\"profile_id\":2,\"program_membership_digest\":\"Unselected\"},\"stack_limit_input\":{\"Nat\":2}},\"seg_base\":{\"active\":{\"data\":0,\"data_fields\":0,\"env\":0,\"value_slots\":0},\"alloc\":{\"data_fields\":0,\"data_nodes\":0,\"environment_cells\":0},\"consumed_work\":0,\"meaning\":\"genesis_uncut public accessor values; not construction of sealed runtime object\"},\"tasks\":[],\"value_slots\":[]}")
  rule cnResidual3() => c6Raw("{\"active\":{\"data\":0,\"data_fields\":0,\"env\":0,\"value_slots\":0},\"alloc\":{\"data_fields\":0,\"data_nodes\":0,\"environment_cells\":0},\"data\":[],\"data_fields\":[],\"env\":[],\"run_id\":{\"fuel_input\":{\"Nat\":3},\"identity\":{\"bounds_digest\":\"Unselected\",\"field_id\":null,\"finance\":\"Absent\",\"grammar_domain\":\"GenericNominalExtensionReserved\",\"layout_digest\":\"Unselected\",\"layout_version\":[\"lv\",\"1\"],\"profile_id\":2,\"program_membership_digest\":\"Unselected\"},\"stack_limit_input\":{\"Nat\":2}},\"seg_base\":{\"active\":{\"data\":0,\"data_fields\":0,\"env\":0,\"value_slots\":0},\"alloc\":{\"data_fields\":0,\"data_nodes\":0,\"environment_cells\":0},\"consumed_work\":0,\"meaning\":\"genesis_uncut public accessor values; not construction of sealed runtime object\"},\"tasks\":[],\"value_slots\":[]}")
  syntax C6Frames ::= cnManualFrames() [function]
  rule cnManualFrames() => c6Frame(c6LetF(2,2,c6None(),0),c6NoFrames())
  syntax CNState ::= cnManual(C6Control,Int,C6Frames,CNClock,CNPad,C6Raw) [function]
  rule cnManual(C,O,Fs,K,Pad,R) => cnState(cnProfile(2,"GenericNominalExtensionReserved","Absent"),
    cnManualView(),c6Boundary(cnManualProgram(),C,O,c6None(),Fs,c6NoResult(),R),
    cnRuntime(K,2,2,false,0,true,Pad))
  syntax CNState ::= cnGenesis() [function] | cnAfterLet() [function]
                   | cnPre() [function] | cnPost() [function]
                   | cnLastPre() [function] | cnLastPost() [function]
  rule cnGenesis() => cnManual(c6Eval(0),0,c6NoFrames(),cnClock(7,7,0,0,0,0),cnZeroFrame(cnZeroFrame(cnNoPad())),cnResidual7())
  rule cnAfterLet() => cnManual(c6ScheduleNext(c6LetF(2,2,c6None(),0),1,c6None()),0,c6NoFrames(),cnClock(7,6,1,0,0,1),cnZeroFrame(cnZeroFrame(cnNoPad())),cnResidual7())
  rule cnPre() => cnManual(c6Eval(1),1,cnManualFrames(),cnClock(7,5,2,0,0,2),cnZeroFrame(cnNoPad()),cnResidual7())
  rule cnPost() => cnManual(c6Return(c6Const(0)),1,cnManualFrames(),cnClock(7,4,3,0,0,3),cnZeroFrame(cnNoPad()),cnResidual7())
  rule cnLastPre() => cnManual(c6Eval(1),1,cnManualFrames(),cnClock(3,1,2,0,0,2),cnZeroFrame(cnNoPad()),cnResidual3())
  rule cnLastPost() => cnManual(c6Return(c6Const(0)),1,cnManualFrames(),cnClock(3,0,3,0,0,3),cnZeroFrame(cnNoPad()),cnResidual3())

  // Source-data mutation constructors only; they do not call the transition.
  syntax CNRows ::= cnReplace(CNRows,Int,CNRow) [function]
  rule cnReplace(cnRow(_,Rs),0,R) => cnRow(R,Rs)
  rule cnReplace(cnRow(H,Rs),I,R) => cnRow(H,cnReplace(Rs,I -Int 1,R)) requires I >Int 0
  rule cnReplace(cnNoRows(),_,_) => cnNoRows()
  rule cnReplace(Rs,I,_) => Rs requires I <Int 0
  syntax CNView ::= cnExprRow(CNView,Int,CNRow) [function] | cnTypeRow(CNView,Int,CNRow) [function]
                  | cnCtorRow(CNView,Int,CNRow) [function] | cnConstantRow(CNView,Int,CNRow) [function]
  rule cnExprRow(cnView(E,T,C,K,N),I,R) => cnView(cnReplace(E,I,R),T,C,K,N)
  rule cnTypeRow(cnView(E,T,C,K,N),I,R) => cnView(E,cnReplace(T,I,R),C,K,N)
  rule cnCtorRow(cnView(E,T,C,K,N),I,R) => cnView(E,T,cnReplace(C,I,R),K,N)
  rule cnConstantRow(cnView(E,T,C,K,N),I,R) => cnView(E,T,C,cnReplace(K,I,R),N)
  syntax CNState ::= cnViewIn(CNState,CNView) [function] | cnClockIn(CNState,CNClock) [function]
                   | cnFramesIn(CNState,C6Frames) [function] | cnPadIn(CNState,CNPad) [function]
                   | cnControlIn(CNState,C6Control,Int) [function] | cnRuntimeIn(CNState,CNRuntime) [function]
                   | cnResidualIn(CNState,C6Raw) [function] | cnProfileIn(CNState,CNProfile) [function]
                   | cnResultIn(CNState,C6Result) [function]
  rule cnViewIn(cnState(P,_,B,R),V) => cnState(P,V,B,R)
  rule cnClockIn(cnState(P,V,B,cnRuntime(_,L,N,H,T,R,D)),K) => cnState(P,V,B,cnRuntime(K,L,N,H,T,R,D))
  rule cnFramesIn(cnState(P,V,c6Boundary(R,C,O,E,_,H,X),K),Fs) => cnState(P,V,c6Boundary(R,C,O,E,Fs,H,X),K)
  rule cnPadIn(cnState(P,V,B,cnRuntime(K,L,N,H,T,R,_)),D) => cnState(P,V,B,cnRuntime(K,L,N,H,T,R,D))
  rule cnControlIn(cnState(P,V,c6Boundary(R,_,_,E,Fs,H,X),K),C,O) => cnState(P,V,c6Boundary(R,C,O,E,Fs,H,X),K)
  rule cnRuntimeIn(cnState(P,V,B,_),R) => cnState(P,V,B,R)
  rule cnResidualIn(cnState(P,V,c6Boundary(R,C,O,E,Fs,H,_),K),X) => cnState(P,V,c6Boundary(R,C,O,E,Fs,H,X),K)
  rule cnProfileIn(cnState(_,V,B,R),P) => cnState(P,V,B,R)
  rule cnResultIn(cnState(P,V,c6Boundary(R,C,O,E,Fs,_,X),K),H) => cnState(P,V,c6Boundary(R,C,O,E,Fs,H,X),K)

  // ordinary-let-full; manual ordinaryClockDerivation row1.
  syntax Bool ::= cnLaw01() [function]
  rule cnLaw01() => cnMatches(cnStep(cnGenesis()),"KMachineEvalLet",cnAfterLet())

  // ordinary-push-full; manual ordinaryClockDerivation row2.
  syntax Bool ::= cnLaw02() [function]
  rule cnLaw02() => cnMatches(cnStep(cnAfterLet()),"KMachinePushAccept",cnPre())

  // nullary-full; manual complete pre/post.
  syntax Bool ::= cnLaw03() [function]
  rule cnLaw03() => cnRelation(1,cnPre(),cnPost())

  // nullary-step-full; manual ordinaryClockDerivation row3.
  syntax Bool ::= cnLaw04() [function]
  rule cnLaw04() => cnMatches(cnStep(cnPre()),"XAdtEvalConstructNullary",cnPost())

  // last-fuel-running; lastFuelPositive.
  syntax Bool ::= cnLaw05() [function]
  rule cnLaw05() => cnRelation(1,cnLastPre(),cnLastPost())

  // disabled-identity; disabledPositive.
  syntax Bool ::= cnLaw06() [function]
  rule cnLaw06() => cnRelation(0,cnPre(),cnPre())

  // disabled-advanced; disabled-advanced-post.
  syntax Bool ::= cnLaw07() [function]
  rule cnLaw07() => notBool cnRelation(0,cnPre(),cnPost())

  // bad-enable; nonboolean-enable.
  syntax Bool ::= cnLaw08() [function]
  rule cnLaw08() => notBool cnRelation(2,cnPre(),cnPost())

  // selected-let; selected discriminator / occurrence profile.
  syntax Bool ::= cnLaw09() [function]
  rule cnLaw09() => cnIsOutside(cnNullary(cnControlIn(cnPre(),c6Eval(0),0)))

  // selected-var; selected discriminator / occurrence profile.
  syntax Bool ::= cnLaw10() [function]
  rule cnLaw10() => cnIsOutside(cnNullary(cnControlIn(cnPre(),c6Eval(2),2)))

  // selected-missing; selected discriminator / occurrence profile.
  syntax Bool ::= cnLaw11() [function]
  rule cnLaw11() => cnIsOutside(cnNullary(cnControlIn(cnPre(),c6Eval(3),3)))

  // origin-mismatch; selected discriminator / occurrence profile.
  syntax Bool ::= cnLaw12() [function]
  rule cnLaw12() => cnIsOutside(cnNullary(cnControlIn(cnPre(),c6Eval(1),0)))

  // builtin-type; named semantic matrix.
  syntax Bool ::= cnLaw13() [function]
  rule cnLaw13() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(2,4,c6Slice(0,0),c6Some(0))))))

  // missing-type; named semantic matrix.
  syntax Bool ::= cnLaw14() [function]
  rule cnLaw14() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(5,4,c6Slice(0,0),c6Some(0))))))

  // wrong-ctor; named semantic matrix.
  syntax Bool ::= cnLaw15() [function]
  rule cnLaw15() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(4,0,c6Slice(0,0),c6Some(0))))))

  // missing-ctor; named semantic matrix.
  syntax Bool ::= cnLaw16() [function]
  rule cnLaw16() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(4,5,c6Slice(0,0),c6Some(0))))))

  // nonzero-fields; named semantic matrix.
  syntax Bool ::= cnLaw17() [function]
  rule cnLaw17() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(4,4,c6Slice(0,1),c6Some(0))))))

  // no-constant; named semantic matrix.
  syntax Bool ::= cnLaw18() [function]
  rule cnLaw18() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(4,4,c6Slice(0,0),c6None())))))

  // missing-constant; named semantic matrix.
  syntax Bool ::= cnLaw19() [function]
  rule cnLaw19() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnExprRow(cnManualView(),1,cnConstruct(4,4,c6Slice(0,0),c6Some(1))))))

  // owner-mismatch; checked ctor arity actual source.
  syntax Bool ::= cnLaw20() [function]
  rule cnLaw20() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnCtorRow(cnManualView(),4,cnCtor(3,5,0)))))

  // arity-one; checked ctor arity actual source.
  syntax Bool ::= cnLaw21() [function]
  rule cnLaw21() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnCtorRow(cnManualView(),4,cnCtor(4,4,1)))))

  // field-start-oob; checked ctor arity actual source.
  syntax Bool ::= cnLaw22() [function]
  rule cnLaw22() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnCtorRow(cnManualView(),4,cnCtor(4,6,0)))))

  // outside-interval; named semantic matrix.
  syntax Bool ::= cnLaw23() [function]
  rule cnLaw23() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnTypeRow(cnManualView(),4,cnType(0,1)))))

  // empty-interval; named semantic matrix.
  syntax Bool ::= cnLaw24() [function]
  rule cnLaw24() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnTypeRow(cnManualView(),4,cnType(4,0)))))

  // marker-false; named semantic matrix.
  syntax Bool ::= cnLaw25() [function]
  rule cnLaw25() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnConstantRow(cnManualView(),0,cnConstant(false,4,4,c6Slice(0,0))))))

  // constant-type; named semantic matrix.
  syntax Bool ::= cnLaw26() [function]
  rule cnLaw26() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnConstantRow(cnManualView(),0,cnConstant(true,3,4,c6Slice(0,0))))))

  // constant-ctor; named semantic matrix.
  syntax Bool ::= cnLaw27() [function]
  rule cnLaw27() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnConstantRow(cnManualView(),0,cnConstant(true,4,3,c6Slice(0,0))))))

  // constant-fields; named semantic matrix.
  syntax Bool ::= cnLaw28() [function]
  rule cnLaw28() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnConstantRow(cnManualView(),0,cnConstant(true,4,4,c6Slice(0,1))))))

  // constant-kind; named semantic matrix.
  syntax Bool ::= cnLaw29() [function]
  rule cnLaw29() => cnIsOutside(cnNullary(cnViewIn(cnPre(),cnConstantRow(cnManualView(),0,cnOther(c6Raw("scalar"))))))

  // stale-body; active frame/profile; dropped frame also mismatches capacity.
  syntax Bool ::= cnLaw30() [function]
  rule cnLaw30() => cnIsOutside(cnNullary(cnFramesIn(cnPre(),c6Frame(c6LetF(2,1,c6None(),0),c6NoFrames()))))

  // stale-origin; active frame/profile; dropped frame also mismatches capacity.
  syntax Bool ::= cnLaw31() [function]
  rule cnLaw31() => cnIsOutside(cnNullary(cnFramesIn(cnPre(),c6Frame(c6LetF(2,2,c6None(),2),c6NoFrames()))))

  // foreign-frame; active frame/profile; dropped frame also mismatches capacity.
  syntax Bool ::= cnLaw32() [function]
  rule cnLaw32() => cnIsOutside(cnNullary(cnFramesIn(cnPre(),c6Frame(c6ScopeF(c6None(),0),c6NoFrames()))))

  // dropped-frame; multiple defects: missing waiting frame and pad/capacity mismatch.
  // Exact outside state is preserved; this is not a general Core rejection.
  syntax Bool ::= cnLaw33() [function]
  rule cnLaw33() => cnOutsideMatches(cnNullary(cnFramesIn(cnPre(),c6NoFrames())),cnFramesIn(cnPre(),c6NoFrames()))

  // bad-inactive-pad; stale inactive padding.
  syntax Bool ::= cnLaw34() [function]
  rule cnLaw34() => cnIsOutside(cnNullary(cnPadIn(cnPre(),cnBadPad(c6Raw("nonzero"),cnNoPad()))))

  // pending-helper; incoming current profile.
  syntax Bool ::= cnLaw35() [function]
  rule cnLaw35() => cnIsOutside(cnNullary(cnRuntimeIn(cnPre(),cnRuntime(cnClock(7,5,2,0,0,2),2,2,true,0,true,cnZeroFrame(cnNoPad())))))

  // live-task; incoming current profile.
  syntax Bool ::= cnLaw36() [function]
  rule cnLaw36() => cnIsOutside(cnNullary(cnRuntimeIn(cnPre(),cnRuntime(cnClock(7,5,2,0,0,2),2,2,false,1,true,cnZeroFrame(cnNoPad())))))

  // not-running; incoming current profile.
  syntax Bool ::= cnLaw37() [function]
  rule cnLaw37() => cnIsOutside(cnNullary(cnRuntimeIn(cnPre(),cnRuntime(cnClock(7,5,2,0,0,2),2,2,false,0,false,cnZeroFrame(cnNoPad())))))

  // stack-overflow; incoming current profile.
  syntax Bool ::= cnLaw38() [function]
  rule cnLaw38() => cnIsOutside(cnNullary(cnRuntimeIn(cnPre(),cnRuntime(cnClock(7,5,2,0,0,2),0,2,false,0,true,cnZeroFrame(cnNoPad())))))

  // pending-result; pending-result.
  syntax Bool ::= cnLaw39() [function]
  rule cnLaw39() => cnIsOutside(cnNullary(cnResultIn(cnPre(),c6ScalarTypeTest(true))))

  // zero-fuel; clock profile and UInt128 range.
  syntax Bool ::= cnLaw40() [function]
  rule cnLaw40() => cnIsOutside(cnNullary(cnClockIn(cnPre(),cnClock(2,0,2,0,0,2))))

  // reference-overflow; clock profile and UInt128 range.
  syntax Bool ::= cnLaw41() [function]
  rule cnLaw41() => cnIsOutside(cnNullary(cnClockIn(cnPre(),cnClock(7,5,2,0,0,340282366920938463463374607431768211455))))

  // non-u128; clock profile and UInt128 range.
  syntax Bool ::= cnLaw42() [function]
  rule cnLaw42() => cnIsOutside(cnNullary(cnClockIn(cnPre(),cnClock(7,5,2,0,0,340282366920938463463374607431768211456))))

  // negative-clock; clock profile and UInt128 range.
  syntax Bool ::= cnLaw43() [function]
  rule cnLaw43() => cnIsOutside(cnNullary(cnClockIn(cnPre(),cnClock(7,5,2,0,0,-1))))

  // inconsistent-clock; clock profile and UInt128 range.
  syntax Bool ::= cnLaw44() [function]
  rule cnLaw44() => cnIsOutside(cnNullary(cnClockIn(cnPre(),cnClock(7,4,2,0,0,2))))

  // post-ref-not-charged; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw45() [function]
  rule cnLaw45() => notBool cnRelation(1,cnPre(),cnClockIn(cnPost(),cnClock(7,4,3,0,0,2)))

  // post-work-not-charged; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw46() [function]
  rule cnLaw46() => notBool cnRelation(1,cnPre(),cnClockIn(cnPost(),cnClock(7,5,2,0,0,3)))

  // post-wrong-ref-kind; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw47() [function]
  rule cnLaw47() => notBool cnRelation(1,cnPre(),cnControlIn(cnPost(),c6Return(c6Data(0)),1))

  // post-frame-dropped; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw48() [function]
  rule cnLaw48() => notBool cnRelation(1,cnPre(),cnFramesIn(cnPost(),c6NoFrames()))

  // post-frame-mutated; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw49() [function]
  rule cnLaw49() => notBool cnRelation(1,cnPre(),cnFramesIn(cnPost(),c6Frame(c6LetF(2,1,c6None(),0),c6NoFrames())))

  // post-padding-mutated; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw50() [function]
  rule cnLaw50() => notBool cnRelation(1,cnPre(),cnPadIn(cnPost(),cnBadPad(c6Raw("nonzero"),cnNoPad())))

  // post-residual-mutated; full-post framing; selected subset of matrix.
  syntax Bool ::= cnLaw51() [function]
  rule cnLaw51() => notBool cnRelation(1,cnPre(),cnResidualIn(cnPost(),c6Raw("changed full residual")))

  // wrong-profile; declared generic profile.
  syntax Bool ::= cnLaw52() [function]
  rule cnLaw52() => cnIsOutside(cnNullary(cnProfileIn(cnPre(),cnProfile(1,"Financial","Present"))))

  // nonzero-empty-start-not-invented-refusal; selected-reader guard probe only.
  // Frozen raw children=[] cannot hold start1 even at len0 under Core SI24.
  // No zero-start refusal is invented; supplied-table bounds are separate Laws63/64.
  syntax Bool ::= cnLaw53() [function]
  rule cnLaw53() => cnRelation(1,cnViewIn(cnPre(),cnConstantRow(cnExprRow(cnManualView(),1,cnConstruct(4,4,c6Slice(1,0),c6Some(0))),0,cnConstant(true,4,4,c6Slice(1,0)))),cnViewIn(cnPost(),cnConstantRow(cnExprRow(cnManualView(),1,cnConstruct(4,4,c6Slice(1,0),c6Some(0))),0,cnConstant(true,4,4,c6Slice(1,0)))))

  // helper-counters-preserved; explicit ordinary owner charge; supplied-window, not this genesis prefix.
  syntax Bool ::= cnLaw54() [function]
  rule cnLaw54() => cnRelation(1,cnClockIn(cnPre(),cnClock(7,5,2,9,11,2)),cnClockIn(cnPost(),cnClock(7,4,3,9,11,3)))

  // three-row-prefix; three adjacent independent expected windows with rule identity.
  // This conjunction is not an observed composed execution or a full program run.
  syntax Bool ::= cnLaw55() [function]
  rule cnLaw55() => cnMatches(cnStep(cnGenesis()),"KMachineEvalLet",cnAfterLet()) andBool cnMatches(cnStep(cnAfterLet()),"KMachinePushAccept",cnPre()) andBool cnMatches(cnStep(cnPre()),"XAdtEvalConstructNullary",cnPost())

  // Frozen independent Source06 expectations; all top-level expected Booltrue; UNRUN.
// exact-rule-and-state; A matching literal expected rule and complete state are sufficient for the answer matcher.
  syntax Bool ::= cnLaw56() [function]
  rule cnLaw56() => cnMatches(cnApplied("XAdtEvalConstructNullary",cnPost()),"XAdtEvalConstructNullary",cnPost())

// wrong-rule-same-post; Same complete state with a different applied rule must fail; Source05 erased this distinction.
  syntax Bool ::= cnLaw57() [function]
  rule cnLaw57() => notBool cnMatches(cnApplied("KMachineEvalLet",cnPost()),"XAdtEvalConstructNullary",cnPost())

// right-rule-wrong-post; Correct applied rule never excuses a different complete state.
  syntax Bool ::= cnLaw58() [function]
  rule cnLaw58() => notBool cnMatches(cnApplied("XAdtEvalConstructNullary",cnPre()),"XAdtEvalConstructNullary",cnPost())

// outside-never-applied; Outside never satisfies an applied-answer match even when state equals post.
  syntax Bool ::= cnLaw59() [function]
  rule cnLaw59() => notBool cnMatches(cnOutside("supplied outside",cnPost()),"XAdtEvalConstructNullary",cnPost())

// empty-frames-repaired-padding-profile-only; Depth0+pad2 fits capacity2 and readiness holds; current-let profile still has no active awaiting LetF. This is not Core or raw-circuit rejection.
  syntax Bool ::= cnLaw60() [function]
  rule cnLaw60() => cnReady(cnPadIn(cnFramesIn(cnPre(),c6NoFrames()),cnZeroFrame(cnZeroFrame(cnNoPad())))) andBool cnOutsideMatches(cnNullary(cnPadIn(cnFramesIn(cnPre(),c6NoFrames()),cnZeroFrame(cnZeroFrame(cnNoPad())))),cnPadIn(cnFramesIn(cnPre(),c6NoFrames()),cnZeroFrame(cnZeroFrame(cnNoPad()))))

// multiple-defects-rejection-preserves-complete-input; Zero fuel and bad inactive pad independently violate readiness; whichever guard establishes outside, the complete supplied state including raw bytes must be unchanged.
  syntax Bool ::= cnLaw61() [function]
  rule cnLaw61() => cnOutsideMatches(cnNullary(cnClockIn(cnPadIn(cnPre(),cnBadPad(c6Raw("nonzero"),cnNoPad())),cnClock(2,0,2,0,0,2))),cnClockIn(cnPadIn(cnPre(),cnBadPad(c6Raw("nonzero"),cnNoPad())),cnClock(2,0,2,0,0,2)))

// outside-mutated-state-rejected; Rejection classification alone must not hide a changed complete state.
  syntax Bool ::= cnLaw62() [function]
  rule cnLaw62() => notBool cnOutsideMatches(cnOutside("supplied outside",cnPost()),cnPre())

// empty-start-outside-supplied-table; The selected read does not force start0, but a complete source slice still must fit its supplied table.1+0 exceeds length0.
  syntax Bool ::= cnLaw63() [function]
  rule cnLaw63() => notBool cnSliceFits(c6Slice(1,0),0)

// nonzero-empty-start-in-supplied-table; A nonzero empty start1 fits table length1. Rejecting all nonzero empty starts would invent an invalid restriction.
  syntax Bool ::= cnLaw64() [function]
  rule cnLaw64() => cnSliceFits(c6Slice(1,0),1)

// slice-end-outside-maximum-usize-table; All fields individually fit usize yet len1 cannot fit Total-start0; no wrapped addition may manufacture acceptance.
  syntax Bool ::= cnLaw65() [function]
  rule cnLaw65() => notBool cnSliceFits(c6Slice(18446744073709551615,1),18446744073709551615)

// negative-slice-start; Negative starts do not belong to the usize source carrier.
  syntax Bool ::= cnLaw66() [function]
  rule cnLaw66() => notBool cnSliceFits(c6Slice(-1,0),0)

endmodule
