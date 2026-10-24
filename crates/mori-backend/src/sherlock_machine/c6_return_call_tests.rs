//! Source-only tests. UNCOMPILED_UNRUN. SDK .check below is preprocessing,
//! not synthesis/proving. Typed invalid-result and boundary carriers are explicitly
//! unadmitted diagnostic pairs, not claimed reachable owner states or SI proofs.
use super::*;
use super::super::c6_return_call_step::*;
use mori_core::sherlock::helper_machine::c6_phys::*;
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD,L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::complete_observation::{capture,ObservedStep,CompleteStepRecord};
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;

fn bounds() -> BoundsC {
    BoundsC { code_rows:32,function_rows:4,code_child_slots:32,parameter_slots:8,
        pattern_rows:4,match_plan_rows:4,match_arm_rows:4,type_rows:4,constructor_rows:4,
        constructor_field_slots:8,max_arity:4,constant_nodes:16,constant_field_slots:16,
        data_nodes:4,data_field_slots:8,environment_cells:8,frames:8,helper_tasks:4,
        serialization_bytes:100000,max_reference_work:10000,max_helper_work:10000,step_cap:10000 }
}
fn layout() -> LayoutSchemaC {
    LayoutSchemaC { layout_version:("return-call".into(),"1".into()),field_id:None,widths:None,
        scalar_layout_id:None,bounds:bounds(),capacities:LayoutCapacitiesC {
            name_rows:16,name_bytes:256,occurrence_path_slots:64,value_slots:64 },
        ext_capacities:None,boundary_capacities_present:false }
}
fn config() -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(),C6CodecBudgets { type_expr_nodes:128,template_bytes:4096 })
}
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes:1<<20,max_state_lanes:1<<20,max_metadata_bytes:8192,
        max_string_bytes:512,max_width_entries:64 }
}
fn uint(n:u128) -> CoreExpr {
    CoreExpr::Literal { value:CoreValue::UInt { n:i128_class::KIntC::InRange(n) } }
}
fn program(data:bool) -> CoreProgram {
    let body=if data { CoreExpr::Binary { op:BinaryOp::Add,left:Box::new(uint(3)),right:Box::new(uint(4)) } }
        else { uint(7) };
    CoreProgram { functions:vec![CoreFunction { name:"f".into(),parameters:vec![],return_type:SortTag::UInt,body }],
        entry:CoreExpr::Let { name:"caller".into(),value:Box::new(uint(99)),
            body:Box::new(CoreExpr::Call { name:"f".into(),args:vec![] }) } }
}
fn selected(h:&HostSnapshot) -> bool {
    matches!(h.image.control,ControlC::Return { .. })
        && matches!(h.image.frames.last(),Some(FrameRowC::ReturnF { .. }))
        && h.image.helper.is_none()
        && h.image.result==Some(HelperResultC::ScalarTypeTest { ok:true })
        && h.image.outcome==OutcomeC::Running && h.image.remaining_work>0
}
fn genuine(data:bool) -> CompleteStepRecord {
    let e=host_owner_engine();
    // Real actual errors deliberately not formatted with Debug/expect.
    let a=e.admit(&program(data),&bounds(),&layout()).unwrap_or_else(|_|panic!("actual Core admission refused"));
    let mut i=e.admit_run(&a,&RunIntC::Nat(64),&RunIntC::Nat(8),&FinanceInput::Absent)
        .unwrap_or_else(|_|panic!("actual genesis refused"));
    for _ in 0..512 {
        let h=capture(&a,&i).unwrap_or_else(|_|panic!("actual capture refused"));
        if selected(&h) {
            return match mori_core::sherlock::helper_machine::complete_observation::record_step(&a,&mut i)
                .unwrap_or_else(|_|panic!("actual selected recorder refused")) {
                    ObservedStep::Event(r)=>r,_=>panic!("selected return unexpectedly terminal") };
        }
        match mori_core::sherlock::helper_machine::complete_observation::record_step(&a,&mut i)
            .unwrap_or_else(|_|panic!("actual prefix step refused")) {
                ObservedStep::Event(_)=>{},_=>panic!("terminated before ReturnCallValid") }
    }
    panic!("selected return not reached within 512 real steps")
}
fn expected(pre:&HostSnapshot,valid:bool) -> HostSnapshot {
    // Independent source-denotation oracle: never imports emitter receipts.
    let mut post=pre.clone();
    post.image.remaining_work=pre.image.remaining_work.checked_sub(1).unwrap();
    post.image.consumed_work=pre.image.consumed_work.checked_add(1).unwrap();
    post.image.reference_cursor=pre.image.reference_cursor.checked_add(1).unwrap();
    post.image.result=None;
    if valid {
        let f=post.image.frames.pop().unwrap();
        match f { FrameRowC::ReturnF { saved_env,origin,.. }=>{
            post.image.env_root=saved_env;post.image.control_origin=origin; },_=>panic!("ReturnF expected") }
    } else { post.image.control=ControlC::Error { code:6 }; }
    post
}
fn modes()->[C6PushStatementBindingMode;2] {
    [C6PushStatementBindingMode::LocalPublic,C6PushStatementBindingMode::LedgerCall]
}
fn pair(pre:&HostSnapshot,post:&HostSnapshot,valid:bool,en:bool)->C6ReturnCallStepWitness {
    witness_from_host_pair(en,valid,pre,post,&config(),&limits()).unwrap()
}
fn circuit(w:&C6ReturnCallStepWitness,m:C6PushStatementBindingMode)->C6ReturnCallStepCircuit {
    build_c6_return_call_step_circuit(w.source_params.clone(),w.host_prefix.meta.len(),
        w.host_prefix.prog.len(),w.host_prefix.meta_bounds_frames_offset,m,"return_call_test").unwrap()
}
fn reject(c:&C6ReturnCallStepCircuit,w:&C6ReturnCallStepWitness) {
    match check_c6_return_call_step_circuit(c,w) {
        Err(C6ReturnCallConsumerError::Backend(s))=>assert!(s.starts_with("SDK ProofPreimage::check failed:")),
        _=>panic!("expected actual SDK relation rejection, not producer refusal") }
}
fn raw(c:&C6ReturnCallStepCircuit,s:Vec<Fr>)->Result<Vec<Option<usize>>,String> {
    // CURRENT statement AND CURRENT post-state commitment, not a stale-CC test.
    let prefix=c.meta_len+c.prog_len+CTX_LANES;
    let post_start=4+prefix+c.geometry.state_total_lanes+prefix;
    let post=&s[post_start..];
    let p=match c.mode {
        C6PushStatementBindingMode::LocalPublic=>ProofPreimage {
            inputs:vec![],private_transcript:vec![],public_transcript_inputs:s.clone(),
            public_transcript_outputs:s,binding_input:Fr::from(0u64),communications_commitment:None,
            key_location:KeyLocation("c6_return_call_raw_test".into()) },
        C6PushStatementBindingMode::LedgerCall=>{
            let mut comm=s.clone();comm.extend_from_slice(post);let z=Fr::from(0u64);
            ProofPreimage { inputs:s.clone(),private_transcript:vec![],public_transcript_inputs:s,
                public_transcript_outputs:vec![],binding_input:z,
                communications_commitment:Some((transient_commit(&comm,z),z)),
                key_location:KeyLocation("c6_return_call_raw_test".into()) }
        }
    };
    p.check(&c.ir).map_err(|e|e.to_string())
}

#[test]
fn genuine_const_and_data_call_return_matches_whole_owner_pair_both_modes() {
    for data in [false,true] {
        let r=genuine(data);
        assert_eq!(r.event.rule_id,RuleIdC::KMachineReturnCallValid);
        assert_eq!(r.event.event_kind,EventKind::LegacyComplete);
        assert_eq!(r.event.disposition,StepDisposition::Applied);
        assert_eq!((r.event.work_delta,r.event.helper_work_delta,r.event.helper_visits_delta,r.event.reference_cursor_delta),(1,0,0,1));
        assert_eq!(r.before.image.frames.len(),r.after.image.frames.len()+1);
        assert!(r.before.image.frames.len()>1); // lower caller ScopeF is preserved
        assert_eq!(expected(&r.before,true),r.after);
        match r.before.image.control { ControlC::Return { value }=>
            assert_eq!(matches!(value,RefC::Data(_)),data),_=>panic!("Return expected") }
        for m in modes() { let w=pair(&r.before,&r.after,true,true);assert!(check_c6_return_call_step_circuit(&circuit(&w,m),&w).is_ok()); }
    }
}
#[test]
fn invalid_staged_result_carrier_preserves_frame_env_origin_and_sets_error6() {
    let mut pre=genuine(false).before;pre.image.result=Some(HelperResultC::ScalarTypeTest { ok:false });
    let post=expected(&pre,false);
    assert_eq!(post.image.frames,pre.image.frames);assert_eq!(post.image.env_root,pre.image.env_root);
    assert_eq!(post.image.control_origin,pre.image.control_origin);assert_eq!(post.image.outcome,OutcomeC::Running);
    for m in modes() { let w=pair(&pre,&post,false,true);assert!(check_c6_return_call_step_circuit(&circuit(&w,m),&w).is_ok()); }
}
#[test]
fn exact_saved_environment_none_and_some_zero_are_distinct() {
    let base=genuine(false).before;assert!(!base.image.env.is_empty());
    for env in [None,Some(0)] {
        let mut pre=base.clone();match pre.image.frames.last_mut().unwrap() {
            FrameRowC::ReturnF { saved_env,.. }=>*saved_env=env,_=>panic!("ReturnF expected") }
        let post=expected(&pre,true);
        for m in modes() {
            let w=pair(&pre,&post,true,true);let c=circuit(&w,m);assert!(check_c6_return_call_step_circuit(&c,&w).is_ok());
            let mut wrong=post.clone();wrong.image.env_root=if env.is_some(){None}else{Some(0)};
            reject(&c,&pair(&pre,&wrong,true,true));
        }
    }
}
#[test]
fn wrong_pop_origin_ref_and_lower_frame_mutations_reach_sdk_rejection() {
    let r=genuine(false);let w=pair(&r.before,&r.after,true,true);
    for m in modes() {
        let c=circuit(&w,m);assert!(check_c6_return_call_step_circuit(&c,&w).is_ok());
        let mut wrong=r.after.clone();wrong.image.frames=r.before.image.frames.clone();reject(&c,&pair(&r.before,&wrong,true,true));
        let mut wrong=r.after.clone();wrong.image.control_origin=if wrong.image.control_origin==0{1}else{0};reject(&c,&pair(&r.before,&wrong,true,true));
        let mut wrong=r.after.clone();wrong.image.control=ControlC::Return { value:RefC::Const(0) };
        if wrong.image.control==r.after.image.control { wrong.image.control=ControlC::Return { value:RefC::Const(1) }; }
        reject(&c,&pair(&r.before,&wrong,true,true));
        let mut wrong=r.after.clone();match &mut wrong.image.frames[0] {
            FrameRowC::ScopeF { origin,.. }=>*origin=if *origin==0{1}else{0},_=>panic!("genuine lower ScopeF required") }
        reject(&c,&pair(&r.before,&wrong,true,true));
    }
}
#[test]
fn invalid_must_not_pop_or_restore_and_must_clear_result() {
    let mut pre=genuine(false).before;pre.image.result=Some(HelperResultC::ScalarTypeTest { ok:false });
    let post=expected(&pre,false);
    for m in modes() {
        let w=pair(&pre,&post,false,true);let c=circuit(&w,m);assert!(check_c6_return_call_step_circuit(&c,&w).is_ok());
        let mut q=post.clone();q.image.frames.pop();reject(&c,&pair(&pre,&q,false,true));
        let mut q=post.clone();q.image.result=pre.image.result;reject(&c,&pair(&pre,&q,false,true));
        let mut q=post.clone();q.image.control=ControlC::Error { code:11 };reject(&c,&pair(&pre,&q,false,true));
        let mut q=post.clone();q.image.env_root=if pre.image.env_root.is_some(){None}else{Some(0)};reject(&c,&pair(&pre,&q,false,true));
    }
}
#[test]
fn raw_selected_tags_padding_selector_and_staged_boolean_are_constrained() {
    let r=genuine(false);let w=pair(&r.before,&r.after,true,true);
    for m in modes() {
        let c=circuit(&w,m);let s=w.to_statement_frs();assert!(raw(&c,s.clone()).is_ok());
        let image=4+c.meta_len+c.prog_len+CTX_LANES;
        let helper=image+c.geometry.image_total_lanes+c.geometry.data_total_lanes;
        let top=image+IMAGE_SECTION_HEADER_LANES+(r.before.image.frames.len()-1)*FRAME_LANES;
        for (at,val) in [(1,0),(helper+L_RESULT_RECORD,1),(helper+L_RESULT_RECORD+1,2),
            (helper+L_RESULT_RECORD+2,1),(helper+L_HELPER_RECORD,1),(image+L_CONTROL,0),
            (image+L_CONTROL+3,1),(image+L_FRAME_COUNT,0),(top,8),(top+5,1)] {
            let mut z=s.clone();z[at]=Fr::from(val as u128);assert!(raw(&c,z).is_err(),"raw selected mutation at {at}");
        }
        let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
        let mut z=s.clone();z[post_image+IMAGE_SECTION_HEADER_LANES+(r.before.image.frames.len()-1)*FRAME_LANES]=Fr::from(7u64);
        assert!(raw(&c,z).is_err(),"popped padding must be zero");
    }
}
#[test]
fn clocks_do_not_charge_helper_at_max_and_do_not_wrap_u128() {
    let mut pre=genuine(false).before;pre.image.helper_work=bounds().max_helper_work;
    let post=expected(&pre,true);
    for m in modes() {
        let w=pair(&pre,&post,true,true);let c=circuit(&w,m);assert!(check_c6_return_call_step_circuit(&c,&w).is_ok());
        let mut q=post.clone();q.image.helper_work+=1;reject(&c,&pair(&pre,&q,true,true));
        for lane in [L_CONSUMED_WORK,L_REFERENCE_CURSOR] {
            let mut s=w.to_statement_frs();let image=4+c.meta_len+c.prog_len+CTX_LANES;
            let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
            s[image+lane]=Fr::from(u128::MAX);s[post_image+lane]=Fr::from(0u64);
            assert!(raw(&c,s).is_err(),"checked counter max must reject, not wrap");
        }
        let mut s=w.to_statement_frs();let image=4+c.meta_len+c.prog_len+CTX_LANES;
        s[image+L_REMAINING_WORK]=Fr::from(0u64);assert!(raw(&c,s).is_err());
    }
}
#[test]
fn raw_range_gate_and_disabled_equality_hold_with_current_commitments() {
    let r=genuine(false);let w=pair(&r.before,&r.before,true,false);
    for m in modes() {
        let c=circuit(&w,m);let s=w.to_statement_frs();assert!(raw(&c,s.clone()).is_ok());
        let image=4+c.meta_len+c.prog_len+CTX_LANES;
        let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
        let mut z=s.clone();z[0]=Fr::from(2u64);assert!(raw(&c,z).is_err());
        let mut z=s.clone();z[post_image+L_HELPER_VISITS] = z[post_image+L_HELPER_VISITS] + Fr::from(1u64);assert!(raw(&c,z).is_err());
        let high=Fr::from(u128::MAX)+Fr::from(1u64);
        let mut z=s.clone();z[image+L_CONSUMED_WORK]=high;z[post_image+L_CONSUMED_WORK]=high;
        assert!(raw(&c,z).is_err(),"disabled does not waive raw unsigned128 domain");
        let mut z=s.clone();z[1]=Fr::from(2u64);assert!(raw(&c,z).is_err());
    }
}
#[test]
fn data_program_context_and_schema_mutations_cannot_hide_behind_current_cc() {
    let r=genuine(true);let w=pair(&r.before,&r.after,true,true);
    for m in modes() {
        let c=circuit(&w,m);let s=w.to_statement_frs();assert!(raw(&c,s.clone()).is_ok());
        let image=4+c.meta_len+c.prog_len+CTX_LANES;
        let post_base=image+c.geometry.state_total_lanes;
        for at in [2,3,4+c.meta_len,4+c.meta_len+c.prog_len,
            post_base+c.meta_len,post_base+c.meta_len+c.prog_len,
            post_base+c.meta_len+c.prog_len+CTX_LANES+c.geometry.image_total_lanes+7] {
            let mut z=s.clone();z[at] = z[at] + Fr::from(1u64);assert!(raw(&c,z).is_err());
        }
    }
}
fn inputs(w:&C6ReturnCallStepWitness)->C6ReturnCallStepInputs {
    let cv=|xs:&[u128]| xs.iter().map(|&v|Operand::Immediate(Fr::from(v))).collect::<Vec<_>>();
    C6ReturnCallStepInputs { enable:Operand::Immediate(Fr::from(w.enable)),selector_is_valid:Operand::Immediate(Fr::from(w.selector_is_valid)),
        host_schema:Operand::Immediate(Fr::from(C6_HOST_SCHEMA)),carrier_version:Operand::Immediate(Fr::from(C6_PADDED_CARRIER_VERSION)),
        geometry:w.source_params.geometry(),source_params:w.source_params.clone(),meta_bounds_frames_offset:w.host_prefix.meta_bounds_frames_offset,
        pre_meta:cv(&w.host_prefix.meta),post_meta:cv(&w.host_prefix.meta),pre_prog:cv(&w.host_prefix.prog),post_prog:cv(&w.host_prefix.prog),
        pre_ctx:std::array::from_fn(|i|Operand::Immediate(Fr::from(w.host_prefix.ctx[i]))),post_ctx:std::array::from_fn(|i|Operand::Immediate(Fr::from(w.host_prefix.ctx[i]))),
        pre_image:cv(&w.pre_image),post_image:cv(&w.post_image),pre_data:cv(&w.pre_data),post_data:cv(&w.post_data),pre_helper:cv(&w.pre_helper),post_helper:cv(&w.post_helper) }
}
#[test]
fn all_sixteen_operand_groups_refuse_each_composed_reserved_namespace() {
    let r=genuine(false);let w=pair(&r.before,&r.after,true,true);
    for seg in ["local","c6data","c6helper","c6meta","c6progctx"] {
        for group in 0..16 {
            let mut i=inputs(&w);let op=Operand::Variable(Identifier(format!("%reserved.{seg}.temp.0")));
            match group {0=>i.enable=op,1=>i.selector_is_valid=op,2=>i.host_schema=op,3=>i.carrier_version=op,
                4=>i.pre_meta[0]=op,5=>i.post_meta[0]=op,6=>i.pre_prog[0]=op,7=>i.post_prog[0]=op,
                8=>i.pre_ctx[0]=op,9=>i.post_ctx[0]=op,10=>i.pre_image[0]=op,11=>i.post_image[0]=op,
                12=>i.pre_data[0]=op,13=>i.post_data[0]=op,14=>i.pre_helper[0]=op,_=>i.post_helper[0]=op }
            assert!(matches!(C6ReturnCallStepEmitter::new("reserved").unwrap().emit(&i),Err(C6ReturnCallStepError::ReservedOperand { .. })));
        }
    }
}
#[test]
fn retained_serial_reuses_no_output_identifier_across_two_full_blocks() {
    let r=genuine(false);let w=pair(&r.before,&r.after,true,true);let i=inputs(&w);
    let mut em=C6ReturnCallStepEmitter::new("serial").unwrap();
    let a=em.emit(&i).unwrap();let b=em.emit(&i).unwrap();
    let ids=|v:Vec<Instruction>| { v.into_iter().filter_map(|ins|{
        serde_json::to_value(ins).unwrap().get("output").and_then(|v|v.as_str()).map(str::to_owned)
    }).collect::<std::collections::BTreeSet<_>>() };
    let aa=ids(a.instructions);let bb=ids(b.instructions);
    assert!(!aa.is_empty());assert!(!bb.is_empty());assert!(aa.is_disjoint(&bb));
}
