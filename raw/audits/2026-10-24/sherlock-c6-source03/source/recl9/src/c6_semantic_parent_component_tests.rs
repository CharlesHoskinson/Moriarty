//! Component-only testSOURCE. It is a proposal: uncompiled and UNRUN. There is
//! no genuine C6 child proof, VK or proof fixture here. Native cases send
//! same-shape witnesses through the ACTUAL component assertions via
//! MidnightCircuit/MockProver. C6_COMPONENT_MOCK_K is only the MidnightCircuit
//! `k` parameter: circuit params and ZkStdLib max_bit_len both receive k-1. It
//! is not the MockProver degree. In public midnight-proofs 0.8.2,
//! MockProver::run itself calls RowSizer::min_k, which is a dry-run synthesis,
//! and then synthesizes at the degree that call selected. Cargo patches
//! midnight-proofs to a private override. Its src/dev/mod.rs (MockProver,
//! RowSizer), src/dev/cost_model.rs, src/plonk/circuit.rs and src/lib.rs were
//! inspected and are whole-file byte-identical to public 0.8.2, so the public
//! run, selector-conversion and getter bodies are the patched bodies. The rest
//! of the override, the dependency graph, compilation and runtime/cryptographic
//! behaviour stay uninspected or unqualified. Each native case prints at most
//! one bounded scalar JSON record read from the SAME constructed MockProver
//! (see ProverStats). It is a measurement only after a separately authorized
//! run. This file supplies no C6_COMPONENT_MOCK_K value. It does not bound, reserve or authorize any native degree, memory
//! or other resource. The native cases stay ignored until there is a separate,
//! inspected decision on resources and execution for the whole path. All
//! values are arbitrary test data, not C6 roots or digests.
use super::*;
use super::C6CounterSense::{Decrement,Increment};
use midnight_circuits::instructions::{AssignmentInstructions,PublicInputInstructions};
use std::io;
use std::ops::Range;
use std::panic::{AssertUnwindSafe,catch_unwind};
use std::time::Instant;
use ff::Field;
use midnight_proofs::{circuit::Value,dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit,Relation,ZkStdLibArch};

#[derive(Clone,Copy,Debug)]
struct SpanV{start:F,end:F,delta:F}
#[derive(Clone,Debug)]
struct ChildV{program:Vec<F>,context:Vec<F>,pre:Vec<F>,post:Vec<F>,counters:C6Counters<SpanV>}
#[derive(Clone,Debug)]
struct PairV{left:ChildV,right:ChildV}
/// A test harness, not an accept-all relation. It assigns both children, calls
/// the component, and exposes the SAME returned parent cells.
#[derive(Clone,Debug)]
struct Harness{shape:C6BlockShape,senses:C6Counters<C6CounterSense>}

fn block(std:&ZkStdLib,l:&mut impl Layouter<F>,v:Value<Vec<F>>,n:usize)->Result<Vec<A>,Error>{std.assign_many(l,&v.transpose_vec(n))}
fn span(std:&ZkStdLib,l:&mut impl Layouter<F>,v:Value<SpanV>)->Result<C6U128Span,Error>{
 Ok(C6U128Span{start:std.assign(l,v.map(|s|s.start))?,end:std.assign(l,v.map(|s|s.end))?,delta:std.assign(l,v.map(|s|s.delta))?})
}
fn child(std:&ZkStdLib,l:&mut impl Layouter<F>,s:&C6BlockShape,w:Value<&ChildV>)->Result<C6RangeCells,Error>{
 let c=w.map(|c|c.counters);
 Ok(C6RangeCells{
  program:C6ProgramCells(block(std,l,w.map(|c|c.program.clone()),s.program_lanes)?),
  context:C6ContextCells(block(std,l,w.map(|c|c.context.clone()),s.context_lanes)?),
  pre:C6BoundaryCells(block(std,l,w.map(|c|c.pre.clone()),s.boundary_lanes)?),
  post:C6BoundaryCells(block(std,l,w.map(|c|c.post.clone()),s.boundary_lanes)?),
  counters:C6Counters{remaining_work:span(std,l,c.map(|c|c.remaining_work))?,consumed_work:span(std,l,c.map(|c|c.consumed_work))?,helper_work:span(std,l,c.map(|c|c.helper_work))?,helper_visits:span(std,l,c.map(|c|c.helper_visits))?,reference_cursor:span(std,l,c.map(|c|c.reference_cursor))?},
 })
}

impl Relation for Harness{
 type Instance=Vec<F>;
 type Witness=PairV;
 type Error=Error;
 fn format_instance(i:&Vec<F>)->Result<Vec<F>,Error>{Ok(i.clone())}
 fn circuit(&self,std:&ZkStdLib,l:&mut impl Layouter<F>,_instance:Value<Vec<F>>,w:Value<PairV>)->Result<(),Error>{
  let left=child(std,l,&self.shape,w.as_ref().map(|p|&p.left))?;
  let right=child(std,l,&self.shape,w.as_ref().map(|p|&p.right))?;
  let parent=constrain_c6_adjacent_semantic_pair(std,l,&self.shape,&self.senses,&left,&right)?;
  // Expose the SAME returned cells; there is no host semantic prefilter.
  for x in parent.program.0.iter().chain(&parent.context.0).chain(&parent.pre.0).chain(&parent.post.0){std.constrain_as_public_input(l,x)?;}
  for s in parent.counters.in_order(){for x in [&s.start,&s.end,&s.delta]{std.constrain_as_public_input(l,x)?;}}
  Ok(())
 }
 fn used_chips(&self)->ZkStdLibArch{ZkStdLibArch{nr_pow2range_cols:1,..ZkStdLibArch::default()}}
 fn write_relation<W:io::Write>(&self,w:&mut W)->io::Result<()>{
  for n in [self.shape.program_lanes,self.shape.context_lanes,self.shape.boundary_lanes]{w.write_all(&(n as u32).to_le_bytes())?;}
  for s in self.senses.in_order(){w.write_all(&[match s{Increment=>0u8,Decrement=>1u8}])?;}Ok(())
 }
 fn read_relation<R:io::Read>(_:&mut R)->io::Result<Self>{Err(io::Error::other("component test harness is not a deserializable relation"))}
}

fn f128(x:u128)->F{let t=F::from(1u64<<32);F::from((x>>64) as u64)*t*t+F::from(x as u64)}
fn lanes(tag:u64,n:usize)->Vec<F>{(0..n as u64).map(|i|F::from(tag*100+i)).collect()}
// These senses are local to the test. They are not an authoritative C6 transition law.
fn harness()->Harness{Harness{shape:C6BlockShape{program_lanes:3,context_lanes:2,boundary_lanes:4},senses:C6Counters::from_order([Decrement,Increment,Increment,Increment,Increment])}}
fn spans(h:&Harness,starts:[u128;5],deltas:[u128;5])->(C6Counters<SpanV>,[u128;5]){
 let senses=h.senses.in_order();let mut ends=[0u128;5];
 let v=std::array::from_fn(|i|{ends[i]=match *senses[i]{Increment=>starts[i]+deltas[i],Decrement=>starts[i]-deltas[i]};SpanV{start:f128(starts[i]),end:f128(ends[i]),delta:f128(deltas[i])}});
 (C6Counters::from_order(v),ends)
}
fn valid(h:&Harness)->PairV{
 let s=h.shape;
 let (lc,mid)=spans(h,[1000,0,7,3,u128::MAX-20],[10,10,4,2,5]);
 let (rc,_)=spans(h,mid,[5,6,1,1,15]);
 let left=ChildV{program:lanes(1,s.program_lanes),context:lanes(2,s.context_lanes),pre:lanes(3,s.boundary_lanes),post:lanes(4,s.boundary_lanes),counters:lc};
 let right=ChildV{program:left.program.clone(),context:left.context.clone(),pre:left.post.clone(),post:lanes(5,s.boundary_lanes),counters:rc};
 PairV{left,right}
}
fn parent_pis(p:&PairV)->Vec<F>{
 let mut v=[&p.left.program[..],&p.left.context[..],&p.left.pre[..],&p.right.post[..]].concat();
 for (a,b) in p.left.counters.in_order().into_iter().zip(p.right.counters.in_order()){v.extend([a.start,b.end,a.delta+b.delta]);}
 v
}

#[derive(Debug,PartialEq,Eq)]
enum Verdict{Accept,ConstraintReject,SynthesisReject,Panic}
/// The MidnightCircuit `k` parameter only (params and max_bit_len get k-1).
/// MockProver::run still selects its own degree, and this value bounds nothing.
/// A value of 0 would underflow k-1, and values above 256 would truncate in `as u8`.
fn caller_k()->u32{
 let k:u32=std::env::var("C6_COMPONENT_MOCK_K").expect("C6_COMPONENT_MOCK_K must supply the MidnightCircuit k parameter; it is not a MockProver degree or resource bound").parse().expect("C6_COMPONENT_MOCK_K must be a u32");
 assert!((1..=256).contains(&k),"C6_COMPONENT_MOCK_K must be in 1..=256 so k-1 neither underflows nor truncates as u8");k
}
/// Bounded scalars read from the SAME constructed MockProver after run(), through
/// shown public getters only. run() sets usable_rows=0..n-(blinding_factors()+1),
/// so n=usable_end+blinding_factors+1 and row_k=log2(n) are the rows run()
/// allocated. They are neither C6_COMPONENT_MOCK_K nor cs_degree (the constraint
/// polynomial degree). cs() is the POST-conversion system: num_selectors() is 0
/// there, its fixed columns include one per converted selector, and its gates and
/// lookups carry the replaced selector expressions. selector_assignment_buffers is
/// the length of the selector assignment buffer, which the conversion asserted
/// equal to the pre-conversion count. Lookups are arguments, not table rows.
/// Buffer getters are read for dimensions only. No cell, witness or instance
/// value is read.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct ProverStats{n:usize,row_k:u32,usable_end:usize,blinding_factors:usize,cs_degree:usize,gates:usize,gate_polynomials:usize,advice_columns:usize,fixed_columns_post_conversion:usize,instance_columns:usize,selector_assignment_buffers:usize,selectors_post_conversion:usize,challenges:usize,lookup_arguments:usize,trash_arguments:usize}
/// One native outcome. stats is None exactly when run() built no prover
/// (SynthesisReject, or Panic inside run()), and no n or k is invented then. A
/// Panic inside verify() keeps the stats of its constructed prover. run_us spans
/// run(): configure, the RowSizer::min_k dry-run synthesis, the full synthesis and
/// the selector conversion. verify_us spans verify() only.
struct Observation{verdict:Verdict,stats:Option<ProverStats>,run_us:u128,verify_us:Option<u128>}

fn row_dims(usable:&Range<usize>,blinding:usize)->Result<(usize,u32),&'static str>{
 if usable.start!=0{return Err("usable_rows_start_not_zero")}
 let n=usable.end.checked_add(blinding).and_then(|n|n.checked_add(1)).ok_or("usable_end_plus_blinding_overflows")?;
 if !n.is_power_of_two(){return Err("derived_n_not_power_of_two")}
 Ok((n,n.trailing_zeros()))
}
fn prover_stats(p:&MockProver<F>,instance_columns:usize)->Result<ProverStats,&'static str>{
 let cs=p.cs();let blinding_factors=cs.blinding_factors();
 let (n,row_k)=row_dims(p.usable_rows(),blinding_factors)?;
 if n<cs.minimum_rows(){return Err("derived_n_below_minimum_rows")}
 let (adv,fix,ins,sel)=(p.advice(),p.fixed(),p.instance(),p.selectors());
 if adv.len()!=cs.num_advice_columns()||fix.len()!=cs.num_fixed_columns()||ins.len()!=cs.num_instance_columns()||ins.len()!=instance_columns{return Err("buffer_column_count_mismatch")}
 if cs.num_selectors()!=0||sel.len()>fix.len(){return Err("selector_conversion_state_mismatch")}
 if !(adv.iter().all(|c|c.len()==n)&&fix.iter().all(|c|c.len()==n)&&ins.iter().all(|c|c.len()==n)&&sel.iter().all(|c|c.len()==n)){return Err("buffer_row_length_mismatch")}
 Ok(ProverStats{n,row_k,usable_end:p.usable_rows().end,blinding_factors,cs_degree:cs.degree(),gates:cs.gates().len(),gate_polynomials:cs.gates().iter().map(|g|g.polynomials().len()).sum(),advice_columns:adv.len(),fixed_columns_post_conversion:fix.len(),instance_columns:ins.len(),selector_assignment_buffers:sel.len(),selectors_post_conversion:cs.num_selectors(),challenges:cs.num_challenges(),lookup_arguments:cs.lookups().len(),trash_arguments:cs.trashcans().len()})
}
fn json_or_null(x:Option<String>)->String{x.unwrap_or_else(||"null".into())}
/// Prints the single bounded scalar record of one native case.
fn emit(case:&str,k_param:u32,verdict:Option<&Verdict>,stats:Option<&ProverStats>,run_us:u128,verify_us:Option<u128>,fault:Option<&str>){
 let prover=json_or_null(stats.map(|s|format!(r#"{{"n":{},"row_k":{},"usable_end":{},"blinding_factors":{},"cs_degree":{},"gates":{},"gate_polynomials":{},"advice_columns":{},"fixed_columns_post_conversion":{},"instance_columns":{},"selector_assignment_buffers":{},"selectors_post_conversion":{},"challenges":{},"lookup_arguments":{},"trash_arguments":{}}}"#,s.n,s.row_k,s.usable_end,s.blinding_factors,s.cs_degree,s.gates,s.gate_polynomials,s.advice_columns,s.fixed_columns_post_conversion,s.instance_columns,s.selector_assignment_buffers,s.selectors_post_conversion,s.challenges,s.lookup_arguments,s.trash_arguments)));
 let (verdict,fault,verify_us)=(json_or_null(verdict.map(|v|format!(r#""{v:?}""#))),json_or_null(fault.map(|f|format!(r#""{f}""#))),json_or_null(verify_us.map(|t|t.to_string())));
 println!(r#"{{"schema":"c6-component-mock-stats/1","case":"{case}","midnight_circuit_k_param":{k_param},"verdict":{verdict},"harness_fault":{fault},"prover":{prover},"run_us":{run_us},"verify_us":{verify_us}}}"#);
}
fn observe(case:&str,h:&Harness,w:PairV,pis:Vec<F>)->Observation{
 let k_param=caller_k();
 let circuit=MidnightCircuit::new(h,Value::known(pis.clone()),Value::known(w),Some(k_param));
 let instance=vec![vec![],pis];let instance_columns=instance.len();
 let t=Instant::now();
 let run=catch_unwind(AssertUnwindSafe(||MockProver::run(&circuit,instance)));
 let run_us=t.elapsed().as_micros();
 let o=match run{
  Err(_)=>Observation{verdict:Verdict::Panic,stats:None,run_us,verify_us:None},
  Ok(Err(_))=>Observation{verdict:Verdict::SynthesisReject,stats:None,run_us,verify_us:None},
  Ok(Ok(p))=>{
   // A guard failure is a harness fault. It panics OUTSIDE catch_unwind, so it is
   // never read as Accept, ConstraintReject, SynthesisReject or Panic.
   let stats=prover_stats(&p,instance_columns).unwrap_or_else(|e|{emit(case,k_param,None,None,run_us,None,Some(e));panic!("{case}: stats guard {e} failed after MockProver::run; harness fault, not a verdict")});
   let t=Instant::now();
   let verdict=match catch_unwind(AssertUnwindSafe(||p.verify())){Ok(Ok(()))=>Verdict::Accept,Ok(Err(_))=>Verdict::ConstraintReject,Err(_)=>Verdict::Panic};
   Observation{verdict,stats:Some(stats),run_us,verify_us:Some(t.elapsed().as_micros())}
  }
 };
 emit(case,k_param,Some(&o.verdict),o.stats.as_ref(),o.run_us,o.verify_us,None);o
}
/// Accept and ConstraintReject must be verify() verdicts on a constructed prover
/// whose dimensions passed the guards. These are author-written consistency
/// guards, not an independent oracle.
#[track_caller]
fn constructed(o:&Observation)->&ProverStats{
 let s=o.stats.as_ref().expect("Accept/ConstraintReject must come from a constructed MockProver");
 assert!(o.verify_us.is_some(),"verify phase of the constructed prover was not timed");
 assert_eq!((s.selectors_post_conversion,s.instance_columns),(0,2),"post-conversion selector count and the two submitted instance columns");
 s
}
/// Negative cases: the rejection must come from verify() on a constructed,
/// guard-passing prover, not from synthesis, a panic or the reporting path.
#[track_caller]
fn rejected(o:Observation){assert_eq!(o.verdict,Verdict::ConstraintReject);constructed(&o);}
fn mutated(case:&str,f:impl FnOnce(&mut PairV))->Observation{let h=harness();let mut w=valid(&h);f(&mut w);let pis=parent_pis(&w);observe(case,&h,w,pis)}

#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_same_shape_valid_pair_accepts(){
 let h=harness();let w=valid(&h);let pis=parent_pis(&w);let exposed=pis.len();
 let o=observe("native_same_shape_valid_pair_accepts",&h,w,pis);assert_eq!(o.verdict,Verdict::Accept);
 // Positive control: acceptance must come from a prover that actually carries gate
 // constraints and holds every exposed parent cell in its usable rows.
 let s=constructed(&o);
 assert!(s.gates>0&&s.gate_polynomials>0,"accepting prover carries no gate constraint");
 assert!(exposed<=s.usable_end,"exposed parent cells exceed usable rows");
}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_wrong_program_lane_rejected(){rejected(mutated("native_wrong_program_lane_rejected",|w|w.right.program[1]+=F::ONE));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_wrong_context_lane_rejected(){rejected(mutated("native_wrong_context_lane_rejected",|w|w.right.context[0]+=F::ONE));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_wrong_boundary_lane_rejected(){rejected(mutated("native_wrong_boundary_lane_rejected",|w|w.right.pre[3]+=F::ONE));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_wrong_work_delta_rejected(){rejected(mutated("native_wrong_work_delta_rejected",|w|w.left.counters.consumed_work.delta+=F::ONE));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_counter_adjacency_gap_rejected(){rejected(mutated("native_counter_adjacency_gap_rejected",|w|{let s=&mut w.right.counters.helper_work;s.start+=F::ONE;s.end+=F::ONE;}));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_u128_increment_overflow_rejected(){rejected(mutated("native_u128_increment_overflow_rejected",|w|w.right.counters.reference_cursor=SpanV{start:f128(u128::MAX-15),end:f128(u128::MAX)+F::ONE,delta:f128(16)}));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_u128_decrement_wrap_rejected(){rejected(mutated("native_u128_decrement_wrap_rejected",|w|w.right.counters.remaining_work=SpanV{start:f128(990),end:f128(990)-f128(991),delta:f128(991)}));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_cumulative_endpoint_sum_is_not_parent_delta(){
 let h=harness();let w=valid(&h);let mut pis=parent_pis(&w);let n=pis.len();
 pis[n-1]=w.left.counters.reference_cursor.end+w.right.counters.reference_cursor.end;
 rejected(observe("native_cumulative_endpoint_sum_is_not_parent_delta",&h,w,pis));
}

// Range mutants. Each one puts exactly one cell outside [0,2^128). Every child
// equation, adjacency, the composed equation and every other bound still hold;
// range_mutants_keep_every_field_equation checks the equations on the host.
// The right start is never a target, because it equals the left end cell and
// that cell is already checked as an end. A Decrement start cannot be
// field-negative when its end and delta are in range (end+delta<2^129<q), so
// the closest Decrement start case is a start just above u128::MAX. The older
// adjacency and work-delta cases also break the composed equation, so they do
// not isolate adjacency or the composed check, which are redundant by
// construction.
fn inc_right_delta_negative(w:&mut PairV){w.right.counters.consumed_work=SpanV{start:f128(10),end:f128(9),delta:-F::ONE};}
fn inc_left_start_negative(w:&mut PairV){w.left.counters.consumed_work=SpanV{start:-f128(5),end:f128(10),delta:f128(15)};}
fn dec_right_delta_negative(w:&mut PairV){w.right.counters.remaining_work=SpanV{start:f128(990),end:f128(991),delta:-F::ONE};}
fn dec_left_start_above_u128(w:&mut PairV){
 w.left.counters.remaining_work=SpanV{start:f128(u128::MAX)+f128(10),end:f128(u128::MAX),delta:f128(10)};
 w.right.counters.remaining_work=SpanV{start:f128(u128::MAX),end:f128(u128::MAX-5),delta:f128(5)};
}
const RANGE_MUTANTS:[fn(&mut PairV);4]=[inc_right_delta_negative,inc_left_start_negative,dec_right_delta_negative,dec_left_start_above_u128];
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_increment_field_negative_delta_rejected(){rejected(mutated("native_increment_field_negative_delta_rejected",inc_right_delta_negative));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_increment_field_negative_start_rejected(){rejected(mutated("native_increment_field_negative_start_rejected",inc_left_start_negative));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_decrement_field_negative_delta_rejected(){rejected(mutated("native_decrement_field_negative_delta_rejected",dec_right_delta_negative));}
#[test]#[ignore="native MockProver UNRUN: MockProver::run selects its own degree; no resource bound or execution decision here"]
fn native_decrement_start_above_u128_rejected(){rejected(mutated("native_decrement_start_above_u128_rejected",dec_left_start_above_u128));}

// Pure host cases (shape/refusal and the range-mutant equation oracle). They are
// written here but remain UNRUN.
fn span_law(sense:C6CounterSense,s:&SpanV)->bool{match sense{Increment=>s.start+s.delta==s.end,Decrement=>s.end+s.delta==s.start}}
fn field_equations_hold(h:&Harness,w:&PairV)->bool{
 let (s,a,b)=(h.senses.in_order(),w.left.counters.in_order(),w.right.counters.in_order());
 w.left.program==w.right.program&&w.left.context==w.right.context&&w.left.post==w.right.pre&&(0..5).all(|i|{
  let out=SpanV{start:a[i].start,end:b[i].end,delta:a[i].delta+b[i].delta};
  span_law(*s[i],a[i])&&span_law(*s[i],b[i])&&a[i].end==b[i].start&&span_law(*s[i],&out)
 })
}
#[test]fn range_mutants_keep_every_field_equation(){
 let h=harness();assert!(field_equations_hold(&h,&valid(&h)));
 let mut broken=valid(&h);broken.left.counters.consumed_work.delta+=F::ONE;assert!(!field_equations_hold(&h,&broken));
 for m in RANGE_MUTANTS{let mut w=valid(&h);m(&mut w);assert!(field_equations_hold(&h,&w));}
}
#[test]fn shape_refuses_empty_blocks_overflow_and_loader_bound(){
 let ok=harness().shape;assert_eq!(ok.total_lanes().unwrap(),3+2+8+15);
 for s in [C6BlockShape{program_lanes:0,..ok},C6BlockShape{context_lanes:0,..ok},C6BlockShape{boundary_lanes:0,..ok},C6BlockShape{boundary_lanes:usize::MAX,..ok},C6BlockShape{program_lanes:MAX_CHILD_INSTANCE,..ok}]{assert!(s.total_lanes().is_err());}
}
#[test]fn carry_split_keeps_the_entire_tail(){
 let n=acc_pi_len();let v:Vec<u32>=(0..(n as u32+2)).collect();
 let (stmt,tail)=split_carry_tail(&v,true).unwrap();assert_eq!(stmt,&v[..2]);assert_eq!(tail,&v[2..]);
 let (stmt,tail)=split_carry_tail(&v,false).unwrap();assert_eq!(stmt.len(),v.len());assert!(tail.is_empty());
 assert!(split_carry_tail(&v[..n],true).is_err());assert!(split_carry_tail::<u32>(&[],false).is_err());
}
#[test]fn leaf_role_adapter_is_explicitly_unsupported(){
 match c6_range_cells_from_verified_statement(&[],&harness().shape){Err(e)=>assert_eq!(e.prerequisite,C6_LEAF_ROLE_ADAPTER_PREREQUISITE),Ok(_)=>panic!("adapter must remain Unsupported")}
}
#[test]fn counter_order_is_named_not_positional_leaf_roles(){
 let c=C6Counters::from_order(C6_COUNTER_NAMES);assert_eq!(c.remaining_work,"remaining_work");assert_eq!(c.reference_cursor,"reference_cursor");assert_eq!(c.in_order().map(|s|*s),C6_COUNTER_NAMES);
}
