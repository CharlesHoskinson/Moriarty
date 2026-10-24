//! Test-first normal specimens. ALL UNRUN. No Machine/evaluator or SDK calls.
use super::state::*;
use super::state_encode::encode_observed_a;
use super::state_decode::decode_observed_a;
use super::prepare::prepare_a;
use super::rows::{Capacities, PreparedA, Slice, ValueRef};
use crate::sherlock::*;
use std::sync::Arc;

fn e(x: Expr)->ExprRef { Arc::new(x) }
fn env(n:u128)->Environment { [("x".into(),Value::UInt(n))].into_iter().collect() }
fn fixture()->(PreparedA,ExprRef,ExprRef,Vec<Arm>) {
    let a=e(Expr::Literal(Value::UInt(7)));let b=e(Expr::Literal(Value::UInt(9)));
    let arms=vec![Arm{pattern:Pattern::Var("x".into()),body:a.clone()},Arm{pattern:Pattern::Nil,body:b.clone()}];
    let program=Program{entry:e(Expr::Call("f".into(),vec![a.clone(),b.clone()])),functions:vec![Function{
        name:"f".into(),parameters:vec![Parameter{name:"x".into(),ty:Type::UInt},Parameter{name:"y".into(),ty:Type::UInt}],
        return_type:Type::UInt,body:e(Expr::Match(e(Expr::Var("x".into())),arms.clone())),
    }]};
    // Local normal-example dimensions, never capacity defaults or admission.
    (prepare_a(Arc::new(program),Capacities{tables:[64;15],data:64}).unwrap(),a,b,arms)
}
fn caps()->StateCapacities {StateCapacities{data:64,data_fields:64,env_cells:32,value_slots:32,frame_slots:16,control_rows:16,stack_limit:32}}
fn observed(p:&PreparedA,control:Control)->Snapshot {Snapshot{program:p.full_program.clone(),control:Some(control),environment:Environment::new(),frames:vec![],stack_limit:16,remaining_fuel:7,initial_fuel:12,outcome:Outcome::Pending}}
fn roundtrip(p:&PreparedA,s:&Snapshot)->StateBlock {
    let c=caps();let block=encode_observed_a(p,&c,s).unwrap();
    let image=AImage{profile:PROFILE,full_program:&s.program,program:&p.block,state:&block};
    let decoded=decode_observed_a(p,&c,&image,DecodeMode::Canonical).unwrap();
    assert_eq!(decoded,*s);block
}

#[test]
fn multi_push_visits_pending_payload_then_its_environment_before_next_row() {
    let (p,_,_,_)=fixture();
    let control=Control::Push(Frame::Right(BinaryOp::Add,Value::UInt(7)),Box::new(
        Control::Push(Frame::Right(BinaryOp::Mul,Value::UInt(9)),Box::new(Control::Return(Value::UInt(12))),env(10))),env(8));
    let mut s=observed(&p,control);s.environment=env(13);s.frames=vec![Frame::Right(BinaryOp::Sub,Value::UInt(14)),Frame::Scope(env(15))];
    let b=roundtrip(&p,&s);
    assert_eq!(b.data[..8].iter().map(|r|u128::from_le_bytes(r.uint_bytes)).collect::<Vec<_>>(),vec![7,8,9,10,12,13,14,15]);
    assert_eq!(b.control_rows[..3].iter().map(|r|r.tag).collect::<Vec<_>>(),vec![2,2,1]);
    assert_eq!(b.env_cells[..4].iter().map(|r|r.value.index).collect::<Vec<_>>(),vec![1,3,5,7]);
    assert!(b.env_cells[..4].iter().all(|r|!r.parent.present));
    assert_eq!(b.root.environment,EnvRoot{present:true,index:2});
    assert_eq!(b.frame_slots[1].saved_env,EnvRoot{present:true,index:3});
    assert_eq!(b.root.depth,2);assert_eq!(b.root.control_length,3);
}

#[test]
fn every_frame_form_and_least_equal_source_reference_is_retained() {
    let (p,a,b,arms)=fixture();let mut s=observed(&p,Control::Return(Value::UInt(12)));
    s.frames=vec![Frame::Let("x".into(),a.clone(),env(1)),Frame::Left(BinaryOp::Add,b.clone(),env(2)),
        Frame::Right(BinaryOp::Mul,Value::UInt(3)),Frame::Unary(UnaryOp::Not),Frame::If(a.clone(),b.clone(),env(4)),
        Frame::Match(arms,env(5)),Frame::Args("f".into(),vec![Value::UInt(7)],vec![b],env(6)),
        Frame::Return(Type::UInt,env(7)),Frame::Scope(env(8))];
    let out=roundtrip(&p,&s);
    assert_eq!(out.frame_slots[..9].iter().map(|r|r.tag).collect::<Vec<_>>(),vec![0,1,2,3,4,5,6,7,8]);
    assert_eq!(out.frame_slots[0].expr0,1);assert_eq!(out.frame_slots[1].expr0,2);
    assert_eq!(out.frame_slots[6].remaining_exprs,Slice{start:1,len:1});
    assert_eq!(out.frame_slots[2].saved_env,EnvRoot::default());
    assert_eq!(out.root.depth,9);
}

#[test]
fn every_control_form_roundtrips_with_options_and_source_slices() {
    let (p,a,_,arms)=fixture();let function=p.full_program.functions[0].clone();
    let controls=vec![Control::Eval(a.clone()),Control::Return(Value::Bool(false)),
        Control::Push(Frame::Scope(env(2)),Box::new(Control::Error(255)),env(1)),
        Control::Invoke("f".into(),vec![Value::UInt(7),Value::UInt(9)]),
        Control::Enter(Some(function),vec![Value::UInt(7),Value::UInt(9)]),
        Control::Bound(Some(Environment::new()),Type::UInt,a.clone()),
        Control::Cases(Value::List(vec![]),arms.clone(),env(3)),
        Control::CaseResult(Some(Environment::new()),Value::UInt(7),a,arms[1..].to_vec(),env(4)),
        Control::PrimitiveResult(Ok(Value::Tree(Tree::Leaf(7)))),Control::Error(254)];
    for (tag,control) in controls.into_iter().enumerate() {
        let s=observed(&p,control);let b=roundtrip(&p,&s);assert_eq!(b.control_rows[0].tag,tag as u8);
        if tag==4 {assert!(b.control_rows[0].function_present);assert_eq!(b.control_rows[0].function_id,0);}
        if tag==5 || tag==7 {assert!(b.control_rows[0].bindings_present);assert_eq!(b.control_rows[0].bindings,EnvRoot::default());}
    }
}

#[test]
fn distinct_arguments_and_independent_roots_keep_their_full_values() {
    let (p,_,_,_)=fixture();let mut s=observed(&p,Control::Invoke("f".into(),vec![Value::UInt(7),Value::UInt(9)]));
    s.environment=env(9);s.frames=vec![Frame::Scope(env(7))];let b=roundtrip(&p,&s);
    assert_eq!(b.control_rows[0].values,Slice{start:0,len:2});
    assert_eq!(b.value_slots[..2],[ValueRef{space:true,index:0},ValueRef{space:true,index:1}]);
    assert_eq!(b.data[..4].iter().map(|r|u128::from_le_bytes(r.uint_bytes)).collect::<Vec<_>>(),vec![7,9,9,7]);
    assert_eq!(b.root.environment,EnvRoot{present:true,index:0});
    assert_eq!(b.frame_slots[0].saved_env,EnvRoot{present:true,index:1});
}

#[test]
fn shared_tree_unfolds_with_postorder_field_blocks_and_all_data_space() {
    let (p,_,_,_)=fixture();let leaf=Arc::new(Tree::Leaf(7));
    let s=observed(&p,Control::Return(Value::Tree(Tree::Branch(leaf.clone(),leaf))));let b=roundtrip(&p,&s);
    assert_eq!(b.root.active_data,5);assert_eq!(b.root.active_data_fields,4);
    assert_eq!(b.data_fields[..4].iter().map(|r|(r.space,r.index)).collect::<Vec<_>>(),vec![(true,0),(true,2),(true,1),(true,3)]);
    assert_eq!(b.data[..5].iter().map(|r|r.fields).collect::<Vec<_>>(),vec![Slice::ZERO,Slice{start:0,len:1},Slice::ZERO,Slice{start:1,len:1},Slice{start:2,len:2}]);
}

#[test]
fn outcomes_and_all_u8_error_payloads_are_positive_typed_data() {
    let (p,_,_,_)=fixture();
    for code in 0..=u8::MAX {
        let a=roundtrip(&p,&observed(&p,Control::Error(code)));assert_eq!(a.control_rows[0].error_code,code);
        let b=roundtrip(&p,&observed(&p,Control::PrimitiveResult(Err(code))));assert!(!b.control_rows[0].primitive_is_ok);assert_eq!(b.control_rows[0].error_code,code);
        let mut s=observed(&p,Control::Error(code));s.control=None;s.outcome=Outcome::Rejected(code);
        let c=roundtrip(&p,&s);assert_eq!(c.root.outcome_tag,2);assert_eq!(c.root.outcome_error,code);
    }
    for (tag,outcome) in [(1,Outcome::Done(Value::List(vec![7,9]))),(3,Outcome::Exhausted)] {
        let mut s=observed(&p,Control::Return(Value::UInt(0)));s.control=None;s.outcome=outcome;
        let b=roundtrip(&p,&s);assert_eq!(b.root.outcome_tag,tag);assert_eq!(b.root.control_length,0);
        assert_eq!(b.root.remaining_fuel,7u128.to_le_bytes());assert_eq!(b.root.initial_fuel,12u128.to_le_bytes());assert_eq!(b.root.stack_limit,16);
    }
}

#[test]
fn none_options_and_empty_sequences_remain_distinct_from_successful_empty_bindings() {
    let (p,a,_,_)=fixture();
    for control in [Control::Enter(None,vec![]),Control::Bound(None,Type::UInt,a.clone()),
        Control::Cases(Value::UInt(0),vec![],Environment::new()),
        Control::CaseResult(None,Value::UInt(0),a,vec![],Environment::new())] {
        let b=roundtrip(&p,&observed(&p,control));let row=&b.control_rows[0];
        assert!(!row.function_present);assert!(!row.bindings_present);
        assert_eq!(row.values,Slice::ZERO);assert_eq!(row.arms,Slice::ZERO);
    }
}

// Added during ordinary source review; the preceding seven tests were authored first.
#[test]
fn separately_allocated_equal_program_and_held_syntax_use_structural_lookup() {
    let (prepared,_,_,_)=fixture();let (other,_,_,_)=fixture();
    assert!(!Arc::ptr_eq(&prepared.full_program,&other.full_program));
    assert!(!Arc::ptr_eq(&prepared.full_program.entry,&other.full_program.entry));
    let s=observed(&other,Control::Enter(Some(other.full_program.functions[0].clone()),vec![]));
    let block=roundtrip(&prepared,&s);
    assert!(block.control_rows[0].function_present);assert_eq!(block.control_rows[0].function_id,0);
    let s=observed(&other,Control::Eval(other.full_program.entry.clone()));
    assert_eq!(roundtrip(&prepared,&s).control_rows[0].expr_id,0);
}
