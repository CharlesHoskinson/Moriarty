//! Source only UNCOMPILED_UNRUN. Real owner fixtures and public row expectations;
//! no SDK preprocessing/synthesis/proof was executed and no private oracle was read.
use super::*;
use super::super::c6_return_right_step::*;
use mori_core::sherlock::helper_machine::c6_phys::*;
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD,L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::complete_observation::{capture,record_step,ObservedStep,CompleteStepRecord,DataSnapshot};
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
#[derive(Clone,Copy)]
enum Kind { Ordinary,Overflow,SortMismatch }
#[derive(Clone,Copy)]
struct Case { op:BinaryOp,kind:Kind }
fn bounds()->BoundsC { BoundsC { code_rows:32,function_rows:4,code_child_slots:32,parameter_slots:8,pattern_rows:4,
    match_plan_rows:4,match_arm_rows:4,type_rows:4,constructor_rows:4,constructor_field_slots:8,max_arity:4,
    constant_nodes:16,constant_field_slots:16,data_nodes:4,data_field_slots:8,environment_cells:8,frames:8,
    helper_tasks:4,serialization_bytes:100000,max_reference_work:10000,max_helper_work:10000,step_cap:10000 } }
fn layout()->LayoutSchemaC { LayoutSchemaC { layout_version:("return-right".into(),"1".into()),field_id:None,widths:None,
    scalar_layout_id:None,bounds:bounds(),capacities:LayoutCapacitiesC { name_rows:16,name_bytes:256,occurrence_path_slots:64,value_slots:128 },
    ext_capacities:None,boundary_capacities_present:false } }
fn config()->C6HostConfig { C6HostConfig::from_retained_layout(&layout(),C6CodecBudgets { type_expr_nodes:128,template_bytes:4096 }) }
fn limits()->C6HostLimits { C6HostLimits { max_total_lanes:1<<20,max_state_lanes:1<<20,max_metadata_bytes:8192,max_string_bytes:512,max_width_entries:64 } }
fn uint(n:u128)->CoreExpr { CoreExpr::Literal { value:CoreValue::UInt { n:i128_class::KIntC::InRange(n) } } }
fn tree(n:u128)->CoreExpr { CoreExpr::Literal { value:CoreValue::Tree { tree:CoreTree::Leaf { n:i128_class::KIntC::InRange(n) } } } }
fn program(c:Case)->CoreProgram {
    let (left,right)=match c.kind {
        Kind::Overflow=>(uint(u128::MAX),uint(1)),
        Kind::SortMismatch=>(CoreExpr::Literal { value:CoreValue::Bool { b:true } },uint(1)),
        Kind::Ordinary=>match c.op {
            BinaryOp::Branch=>(tree(7),tree(5)),
            BinaryOp::Cons=>(uint(7),CoreExpr::Literal { value:CoreValue::List { elements:vec![] } }),
            _=>(uint(7),uint(5)),
        },
    };
    CoreProgram { functions:vec![],entry:CoreExpr::Let { name:"caller".into(),value:Box::new(uint(99)),
        body:Box::new(CoreExpr::Binary { op:c.op,left:Box::new(left),right:Box::new(right) }) } }
}
fn selected(h:&HostSnapshot,c:Case)->bool {
    matches!(h.image.control,ControlC::Return { .. }) && matches!(h.image.frames.last(),Some(FrameRowC::RightF { op,.. }) if *op==c.op)
        && h.image.helper.is_none() && matches!(h.image.result,Some(HelperResultC::ScalarValue { .. }|HelperResultC::ScalarError { .. }|HelperResultC::EqualDone { .. }))
        && h.image.outcome==OutcomeC::Running && h.image.remaining_work>0
}
fn genuine(c:Case)->CompleteStepRecord {
    let e=host_owner_engine();let a=e.admit(&program(c),&bounds(),&layout()).unwrap_or_else(|_|panic!("actual public Core admission refused"));
    // R3/R4/R5 actual inputs:128<=10000,8<=8,128<=128. No forged genesis.
    let mut i=e.admit_run(&a,&RunIntC::Nat(128),&RunIntC::Nat(8),&FinanceInput::Absent).unwrap_or_else(|_|panic!("actual genesis refused"));
    for _ in 0..512 {
        let h=capture(&a,&i).unwrap_or_else(|_|panic!("actual capture refused"));
        if selected(&h,c) { return match record_step(&a,&mut i).unwrap_or_else(|_|panic!("actual ReturnRight step refused")) {
            ObservedStep::Event(r)=>r,_=>panic!("selected ReturnRight became terminal") }; }
        match record_step(&a,&mut i).unwrap_or_else(|_|panic!("actual prefix step refused")) {
            ObservedStep::Event(_)=>{},_=>panic!("terminal before selected ReturnRight") }
    }
    panic!("ReturnRight not reached within512 real steps")
}
fn base()->CompleteStepRecord { genuine(Case { op:BinaryOp::Add,kind:Kind::Ordinary }) }
fn expected(pre:&HostSnapshot)->HostSnapshot {
    let mut post=pre.clone();let top=post.image.frames.pop().unwrap();
    let FrameRowC::RightF { origin,.. }=top else { panic!("RightF expected"); };
    let result=match pre.image.result {
        Some(HelperResultC::ScalarValue { value }|HelperResultC::EqualDone { value })=>PrimOutcome::Value { value },
        Some(HelperResultC::ScalarError { code })=>PrimOutcome::Error { code },_=>panic!("selected result expected") };
    post.image.control=ControlC::PrimResult { result };post.image.control_origin=origin;post.image.result=None;
    post.image.remaining_work=pre.image.remaining_work.checked_sub(1).unwrap();
    post.image.consumed_work=pre.image.consumed_work.checked_add(1).unwrap();
    post.image.reference_cursor=pre.image.reference_cursor.checked_add(1).unwrap();post
}
fn modes()->[C6PushStatementBindingMode;2] { [C6PushStatementBindingMode::LocalPublic,C6PushStatementBindingMode::LedgerCall] }
fn pair(pre:&HostSnapshot,post:&HostSnapshot,en:bool)->C6ReturnRightStepWitness { witness_from_host_pair(en,pre,post,&config(),&limits()).unwrap() }
fn circuit(w:&C6ReturnRightStepWitness,m:C6PushStatementBindingMode)->C6ReturnRightStepCircuit {
    build_c6_return_right_step_circuit(w.source_params.clone(),w.host_prefix.meta.len(),w.host_prefix.prog.len(),w.host_prefix.meta_bounds_frames_offset,m,"return_right_test").unwrap()
}
fn reject(c:&C6ReturnRightStepCircuit,w:&C6ReturnRightStepWitness) { match check_c6_return_right_step_circuit(c,w) {
    Err(C6ReturnRightConsumerError::Backend(s))=>assert!(s.starts_with("SDK ProofPreimage::check failed:")),_=>panic!("expected SDK relation rejection, not producer refusal") } }
fn raw(c:&C6ReturnRightStepCircuit,s:Vec<Fr>)->Result<Vec<Option<usize>>,String> {
    let prefix=c.meta_len+c.prog_len+CTX_LANES;let post_start=3+2*prefix+c.geometry.state_total_lanes;let post=&s[post_start..];
    let p=match c.mode {
        C6PushStatementBindingMode::LocalPublic=>ProofPreimage { inputs:vec![],private_transcript:vec![],public_transcript_inputs:s.clone(),public_transcript_outputs:s,
            binding_input:Fr::from(0u64),communications_commitment:None,key_location:KeyLocation("c6_right_raw_test".into()) },
        C6PushStatementBindingMode::LedgerCall=>{let mut comm=s.clone();comm.extend_from_slice(post);let z=Fr::from(0u64);
            ProofPreimage { inputs:s.clone(),private_transcript:vec![],public_transcript_inputs:s,public_transcript_outputs:vec![],binding_input:z,
                communications_commitment:Some((transient_commit(&comm,z),z)),key_location:KeyLocation("c6_right_raw_test".into()) }}
    };p.check(&c.ir).map_err(|e|e.to_string())
}
#[test]
fn all_seven_ops_genuine_scalar_and_equal_results_preserve_full_owner_pairs() {
    for op in [BinaryOp::Add,BinaryOp::Sub,BinaryOp::Mul,BinaryOp::Eq,BinaryOp::Lt,BinaryOp::Cons,BinaryOp::Branch] {
        let r=genuine(Case { op,kind:Kind::Ordinary });assert_eq!(r.event.rule_id,RuleIdC::KMachineReturnRight);
        assert_eq!(r.event.event_kind,EventKind::LegacyComplete);assert_eq!(r.event.site,Some(SiteTag::RetRight));assert_eq!(r.event.disposition,StepDisposition::Applied);
        assert_eq!((r.event.work_delta,r.event.helper_work_delta,r.event.helper_visits_delta,r.event.reference_cursor_delta),(1,0,0,1));
        assert_eq!(r.before.image.frames.len(),r.after.image.frames.len()+1);assert!(r.after.image.frames.len()>0);
        assert_eq!(r.after.image.env_root,r.before.image.env_root);assert_eq!(r.after.image.outcome,OutcomeC::Running);
        let value=match (op,r.before.image.result) {
            (BinaryOp::Eq,Some(HelperResultC::EqualDone { value }))=>value,
            (_,Some(HelperResultC::ScalarValue { value }))=>value,_=>panic!("actual expected Scalar/Equal result missing") };
        let RefC::Data(id)=value else { panic!("real helper should allocate Data result"); };
        match (op,&r.before.image.data[id]) {
            (BinaryOp::Add,DataSnapshot::U128 { n })=>assert_eq!(*n,12),
            (BinaryOp::Sub,DataSnapshot::U128 { n })=>assert_eq!(*n,2),
            (BinaryOp::Mul,DataSnapshot::U128 { n })=>assert_eq!(*n,35),
            (BinaryOp::Eq|BinaryOp::Lt,DataSnapshot::Bool { b })=>assert!(!*b),
            (BinaryOp::Cons,DataSnapshot::Cons { .. })| (BinaryOp::Branch,DataSnapshot::Branch { .. })=>{},_=>panic!("independent result kind/value mismatch") }
        assert_eq!(expected(&r.before),r.after);
        for m in modes() { let w=pair(&r.before,&r.after,true);assert!(check_c6_return_right_step_circuit(&circuit(&w,m),&w).is_ok()); }
    }
}
#[test]
fn genuine_overflow10_and_sort11_stage_exact_error_without_early_publication() {
    for (kind,code) in [(Kind::Overflow,10u8),(Kind::SortMismatch,11u8)] {
        let r=genuine(Case { op:BinaryOp::Add,kind });assert_eq!(r.before.image.result,Some(HelperResultC::ScalarError { code }));
        assert_eq!(r.after.image.control,ControlC::PrimResult { result:PrimOutcome::Error { code } });
        assert_eq!(r.after.image.outcome,OutcomeC::Running);assert_eq!(expected(&r.before),r.after);
        for m in modes() { let w=pair(&r.before,&r.after,true);assert!(check_c6_return_right_step_circuit(&circuit(&w,m),&w).is_ok()); }
    }
}
#[test]
fn typed_diagnostic_const_data_zero_refs_and_u8_error_boundaries_are_not_conflated() {
    let r=base();assert!(!r.before.image.data.is_empty());
    // These deliberately test bare codec/row composition, not admitted SI21/22 truth.
    for result in [HelperResultC::ScalarValue { value:RefC::Const(0) },HelperResultC::ScalarValue { value:RefC::Data(0) },
        HelperResultC::EqualDone { value:RefC::Const(0) },HelperResultC::EqualDone { value:RefC::Data(0) },
        HelperResultC::ScalarError { code:0 },HelperResultC::ScalarError { code:255 }] {
        let mut pre=r.before.clone();pre.image.result=Some(result);let post=expected(&pre);
        for m in modes() { let w=pair(&pre,&post,true);assert!(check_c6_return_right_step_circuit(&circuit(&w,m),&w).is_ok()); }
    }
}
#[test]
fn wrong_primitive_ref_mode_code_and_all_error_padding_lanes_reject() {
    let r=base();
    for m in modes() {
        let w=pair(&r.before,&r.after,true);let c=circuit(&w,m);assert!(check_c6_return_right_step_circuit(&c,&w).is_ok());
        let mut q=r.after.clone();q.image.control=ControlC::PrimResult { result:PrimOutcome::Value { value:RefC::Const(0) } };reject(&c,&pair(&r.before,&q,true));
        let mut q=r.after.clone();q.image.control=ControlC::PrimResult { result:PrimOutcome::Error { code:10 } };reject(&c,&pair(&r.before,&q,true));
    }
    let r=genuine(Case { op:BinaryOp::Add,kind:Kind::Overflow });
    for m in modes() {
        let w=pair(&r.before,&r.after,true);let c=circuit(&w,m);assert!(check_c6_return_right_step_circuit(&c,&w).is_ok());
        let mut q=r.after.clone();q.image.control=ControlC::PrimResult { result:PrimOutcome::Error { code:11 } };reject(&c,&pair(&r.before,&q,true));
        let s=w.to_statement_frs();let image=3+c.meta_len+c.prog_len+CTX_LANES;let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
        for lane in 3..CONTROL_LANES { let mut z=s.clone();z[post_image+L_CONTROL+lane]=Fr::from(1u64);assert!(raw(&c,z).is_err(),"error padding lane{lane}"); }
    }
}
#[test]
fn wrong_pop_origin_env_lowerframe_and_uncleared_result_reach_sdk() {
    let r=base();
    for m in modes() {
        let w=pair(&r.before,&r.after,true);let c=circuit(&w,m);assert!(check_c6_return_right_step_circuit(&c,&w).is_ok());
        let mut q=r.after.clone();q.image.frames=r.before.image.frames.clone();reject(&c,&pair(&r.before,&q,true));
        let mut q=r.after.clone();q.image.control_origin=if q.image.control_origin==0{1}else{0};reject(&c,&pair(&r.before,&q,true));
        let mut q=r.after.clone();q.image.env_root=None;assert_ne!(q.image.env_root,r.after.image.env_root);reject(&c,&pair(&r.before,&q,true));
        let mut q=r.after.clone();q.image.result=r.before.image.result;reject(&c,&pair(&r.before,&q,true));
        let mut q=r.after.clone();match &mut q.image.frames[0] { FrameRowC::ScopeF { origin,.. }=>*origin=if *origin==0{1}else{0},_=>panic!("lower ScopeF expected") };reject(&c,&pair(&r.before,&q,true));
        let mut z=w.to_statement_frs();let image=3+c.meta_len+c.prog_len+CTX_LANES;let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
        z[post_image+IMAGE_SECTION_HEADER_LANES+(r.before.image.frames.len()-1)*FRAME_LANES]=Fr::from(2u64);assert!(raw(&c,z).is_err());
    }
}
#[test]
fn raw_unsupported_result_tag_ref_boolean_code_width_and_selected_shape_reject() {
    let r=base();let w=pair(&r.before,&r.after,true);
    for m in modes() {
        let c=circuit(&w,m);let s=w.to_statement_frs();assert!(raw(&c,s.clone()).is_ok());let image=3+c.meta_len+c.prog_len+CTX_LANES;
        let helper=image+c.geometry.image_total_lanes+c.geometry.data_total_lanes;let top=image+IMAGE_SECTION_HEADER_LANES+(r.before.image.frames.len()-1)*FRAME_LANES;
        for (at,v) in [(helper+L_RESULT_RECORD,0u128),(helper+L_RESULT_RECORD,13),(helper+L_RESULT_RECORD+1,2),
            (helper+L_HELPER_RECORD,1),(image+L_CONTROL,0),(image+L_CONTROL+3,1),(image+L_FRAME_COUNT,0),(top,1),(top+1,7)] {
            let mut z=s.clone();z[at]=Fr::from(v);assert!(raw(&c,z).is_err());
        }
    }
    let r=genuine(Case { op:BinaryOp::Add,kind:Kind::Overflow });let w=pair(&r.before,&r.after,true);
    for m in modes() {
        let c=circuit(&w,m);let mut z=w.to_statement_frs();let image=3+c.meta_len+c.prog_len+CTX_LANES;
        let helper=image+c.geometry.image_total_lanes+c.geometry.data_total_lanes;let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
        z[helper+L_RESULT_RECORD+1]=Fr::from(256u64);z[post_image+L_CONTROL+2]=Fr::from(256u64);
        assert!(raw(&c,z).is_err(),"paired error-code256 must fail u8 domain, not copying alone");
    }
}
#[test]
fn clocks_no_helper_charge_fuel_zero_and_checked_u128_wrap() {
    let r=base();let mut pre=r.before.clone();pre.image.helper_work=bounds().max_helper_work;
    // Bare typed boundary, not an admitted SI03 positive specimen.
    let post=expected(&pre);
    for m in modes() {
        let w=pair(&pre,&post,true);let c=circuit(&w,m);assert!(check_c6_return_right_step_circuit(&c,&w).is_ok());
        for lane in [L_CONSUMED_WORK,L_REFERENCE_CURSOR] { let mut z=w.to_statement_frs();let image=3+c.meta_len+c.prog_len+CTX_LANES;
            let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
            z[image+lane]=Fr::from(u128::MAX);z[post_image+lane]=Fr::from(0u64);assert!(raw(&c,z).is_err()); }
        let mut q=post.clone();q.image.helper_work+=1;reject(&c,&pair(&pre,&q,true));let mut q=post.clone();q.image.helper_visits+=1;reject(&c,&pair(&pre,&q,true));
        let mut z=w.to_statement_frs();let image=3+c.meta_len+c.prog_len+CTX_LANES;z[image+L_REMAINING_WORK]=Fr::from(0u64);assert!(raw(&c,z).is_err());
    }
}
#[test]
fn disabled_full_identity_raw128_gate2_and_derived129_domain() {
    let r=base();let w=pair(&r.before,&r.before,false);
    for m in modes() {
        let c=circuit(&w,m);let s=w.to_statement_frs();assert!(raw(&c,s.clone()).is_ok());let image=3+c.meta_len+c.prog_len+CTX_LANES;
        let post_image=image+c.geometry.state_total_lanes+c.meta_len+c.prog_len+CTX_LANES;
        let mut z=s.clone();z[0]=Fr::from(2u64);assert!(raw(&c,z).is_err());let mut z=s.clone();z[post_image+L_CONSUMED_WORK] = z[post_image+L_CONSUMED_WORK] + Fr::from(1u64);assert!(raw(&c,z).is_err());
        let high=Fr::from(u128::MAX)+Fr::from(1u64);let mut z=s.clone();z[image+L_CONSUMED_WORK]=high;z[post_image+L_CONSUMED_WORK]=high;assert!(raw(&c,z).is_err());
        let mut z=s.clone();for at in [IMAGE_SECTION_HEADER_LANES+1,IMAGE_SECTION_HEADER_LANES+2] { z[image+at]=Fr::from(u128::MAX);z[post_image+at]=Fr::from(u128::MAX); }
        assert!(raw(&c,z).is_ok(),"gate0 nonmatching sliceend2^129-2 fits comparator129");
    }
}
#[test]
fn old_data_program_context_helper_metadata_preserve_with_current_commitment() {
    let r=base();let w=pair(&r.before,&r.after,true);
    for m in modes() {
        let c=circuit(&w,m);let s=w.to_statement_frs();assert!(raw(&c,s.clone()).is_ok());let image=3+c.meta_len+c.prog_len+CTX_LANES;let post_base=image+c.geometry.state_total_lanes;
        for at in [1,2,3+c.meta_len,3+c.meta_len+c.prog_len,post_base+c.meta_len,post_base+c.meta_len+c.prog_len,
            post_base+c.meta_len+c.prog_len+CTX_LANES+c.geometry.image_total_lanes+7,
            post_base+c.meta_len+c.prog_len+CTX_LANES+c.geometry.image_total_lanes+c.geometry.data_total_lanes+L_HELPER_RECORD] {
            let mut z=s.clone();z[at] = z[at] + Fr::from(1u64);assert!(raw(&c,z).is_err()); }
    }
}
fn inputs(w:&C6ReturnRightStepWitness)->C6ReturnRightStepInputs {
    let cv=|xs:&[u128]|xs.iter().map(|&v|Operand::Immediate(Fr::from(v))).collect::<Vec<_>>();
    C6ReturnRightStepInputs { enable:Operand::Immediate(Fr::from(w.enable)),host_schema:Operand::Immediate(Fr::from(C6_HOST_SCHEMA)),carrier_version:Operand::Immediate(Fr::from(C6_PADDED_CARRIER_VERSION)),
        geometry:w.source_params.geometry(),source_params:w.source_params.clone(),meta_bounds_frames_offset:w.host_prefix.meta_bounds_frames_offset,
        pre_meta:cv(&w.host_prefix.meta),post_meta:cv(&w.host_prefix.meta),pre_prog:cv(&w.host_prefix.prog),post_prog:cv(&w.host_prefix.prog),
        pre_ctx:std::array::from_fn(|i|Operand::Immediate(Fr::from(w.host_prefix.ctx[i]))),post_ctx:std::array::from_fn(|i|Operand::Immediate(Fr::from(w.host_prefix.ctx[i]))),
        pre_image:cv(&w.pre_image),post_image:cv(&w.post_image),pre_data:cv(&w.pre_data),post_data:cv(&w.post_data),pre_helper:cv(&w.pre_helper),post_helper:cv(&w.post_helper) }
}
#[test]
fn all_fifteen_input_groups_five_prefixes_and_serial_reuse_contract() {
    let r=base();let w=pair(&r.before,&r.after,true);
    for seg in ["local","c6data","c6helper","c6meta","c6progctx"] { for group in 0..15 {
        let mut i=inputs(&w);let op=Operand::Variable(Identifier(format!("%reserved.{seg}.temp.0")));
        match group {0=>i.enable=op,1=>i.host_schema=op,2=>i.carrier_version=op,3=>i.pre_meta[0]=op,4=>i.post_meta[0]=op,5=>i.pre_prog[0]=op,6=>i.post_prog[0]=op,
            7=>i.pre_ctx[0]=op,8=>i.post_ctx[0]=op,9=>i.pre_image[0]=op,10=>i.post_image[0]=op,11=>i.pre_data[0]=op,12=>i.post_data[0]=op,13=>i.pre_helper[0]=op,_=>i.post_helper[0]=op }
        assert!(matches!(C6ReturnRightStepEmitter::new("reserved").unwrap().emit(&i),Err(C6ReturnRightStepError::ReservedOperand { .. })));
    }}
    let i=inputs(&w);let mut em=C6ReturnRightStepEmitter::new("serial").unwrap();let a=em.emit(&i).unwrap();let b=em.emit(&i).unwrap();
    let ids=|v:Vec<Instruction>|v.into_iter().filter_map(|ins|serde_json::to_value(ins).unwrap().get("output").and_then(|x|x.as_str()).map(str::to_owned)).collect::<std::collections::BTreeSet<_>>();
    let aa=ids(a.instructions);let bb=ids(b.instructions);assert!(!aa.is_empty()&&!bb.is_empty());assert!(aa.is_disjoint(&bb));
}
