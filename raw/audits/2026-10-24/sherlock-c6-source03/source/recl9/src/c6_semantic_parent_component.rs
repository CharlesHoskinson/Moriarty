//! Proposed Source46 C6 semantic parent COMPONENT, source-only/uncompiled/unrun.
//! It works only on assigned cells that the caller supplies. It constrains three
//! things: the COMPLETE program and context blocks of two adjacent children are
//! equal, the left post boundary equals the right pre boundary, and the C6
//! work/helper/reference counters compose as exact bounded/nonwrapping UInt128
//! spans. It is NOT an admitted C6 parent. The adapter from a verified child to
//! typed roles is Unsupported. This file adds no effect/fee/asset/authority/
//! liability, dispatch/StepInv/Init, parent public statement, carry-tail
//! exposure or terminal D obligation.
use midnight_circuits::{instructions::{ArithInstructions,AssertionInstructions,RangeCheckInstructions},types::AssignedNative};
use midnight_proofs::{circuit::Layouter,plonk::Error};
use midnight_zk_stdlib::ZkStdLib;
use num_bigint::BigUint;
use crate::{inner::acc_pi_len,sherlock_child::MAX_CHILD_INSTANCE,util::F};

type A=AssignedNative<F>;

/// Width of the C6 `u128` ImageSnapshot counters composed here.
pub const C6_U128_BITS:usize=128;
/// Declared order of the named counter fields. This is an ordering, never lane
/// indices. `initial_fuel` is left out because no authoritative span or
/// constant law for it is shown.
pub const C6_COUNTER_NAMES:[&str;5]=["remaining_work","consumed_work","helper_work","helper_visits","reference_cursor"];
/// Start, end and delta cells of one counter span.
pub const C6_SPAN_CELLS:usize=3;
fn u128_bound()->BigUint{BigUint::from(1u8)<<C6_U128_BITS}

/// Fixed complete block lengths. A future authoritative C6 ABI must supply
/// them; this component does not choose any C6 lane layout.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct C6BlockShape{pub program_lanes:usize,pub context_lanes:usize,pub boundary_lanes:usize}
impl C6BlockShape{
 pub fn total_lanes(&self)->Result<usize,String>{
  if self.program_lanes==0||self.context_lanes==0||self.boundary_lanes==0{return Err("empty semantic block".into())}
  let counters=C6_COUNTER_NAMES.len()*C6_SPAN_CELLS;
  let n=self.boundary_lanes.checked_mul(2).and_then(|n|n.checked_add(self.program_lanes)).and_then(|n|n.checked_add(self.context_lanes)).and_then(|n|n.checked_add(counters)).ok_or("semantic block lane overflow")?;
  if n>MAX_CHILD_INSTANCE{return Err("semantic blocks exceed existing loader bound".into())}Ok(n)
 }
}

/// The five named C6 UInt128 counters of one range.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct C6Counters<T>{pub remaining_work:T,pub consumed_work:T,pub helper_work:T,pub helper_visits:T,pub reference_cursor:T}
impl<T> C6Counters<T>{
 pub fn from_order([remaining_work,consumed_work,helper_work,helper_visits,reference_cursor]:[T;5])->Self{Self{remaining_work,consumed_work,helper_work,helper_visits,reference_cursor}}
 pub fn in_order(&self)->[&T;5]{[&self.remaining_work,&self.consumed_work,&self.helper_work,&self.helper_visits,&self.reference_cursor]}
}
/// Direction of each counter, supplied by the caller. There is no default, so
/// this component never infers the missing C6 machine transition law.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum C6CounterSense{Increment,Decrement}
/// Supplied start/end/delta cells of one counter over one contiguous range.
#[derive(Clone,Debug)]
pub struct C6U128Span{pub start:A,pub end:A,pub delta:A}

#[derive(Clone,Debug)]pub struct C6ProgramCells(pub Vec<A>);
#[derive(Clone,Debug)]pub struct C6ContextCells(pub Vec<A>);
#[derive(Clone,Debug)]pub struct C6BoundaryCells(pub Vec<A>);
/// Typed, named cell blocks of one contiguous C6 range (a child or a composition).
#[derive(Clone,Debug)]
pub struct C6RangeCells{pub program:C6ProgramCells,pub context:C6ContextCells,pub pre:C6BoundaryCells,pub post:C6BoundaryCells,pub counters:C6Counters<C6U128Span>}
impl C6RangeCells{
 /// Shape refusal only. It gives no semantic proof credit.
 pub fn check_shape(&self,shape:&C6BlockShape)->Result<(),String>{
  shape.total_lanes()?;
  for (name,got,want) in [("program",self.program.0.len(),shape.program_lanes),("context",self.context.0.len(),shape.context_lanes),("pre boundary",self.pre.0.len(),shape.boundary_lanes),("post boundary",self.post.0.len(),shape.boundary_lanes)]{
   if got!=want{return Err(format!("{name} block differs from fixed complete shape"))}
  }Ok(())
 }
}

fn assert_complete_equal(std:&ZkStdLib,l:&mut impl Layouter<F>,x:&[A],y:&[A])->Result<(),Error>{
 if x.len()!=y.len(){return Err(Error::Synthesis("complete blocks differ in length".into()))}
 for (a,b) in x.iter().zip(y){std.assert_equal(l,a,b)?;}Ok(())
}

/// The contract for one span: start, end and delta are each in [0,2^128).
/// For Increment, start+delta==end. For Decrement, end+delta==start. Both
/// addends are below 2^128, so the field sum is exact integer arithmetic, and
/// the range check on the result rules out u128 wrapping. The start and delta
/// checks are needed as well: a field-negative delta would reverse the sense,
/// and the composed span reuses the left start cell as its own start.
pub fn constrain_u128_span(std:&ZkStdLib,l:&mut impl Layouter<F>,sense:C6CounterSense,span:&C6U128Span)->Result<(),Error>{
 let bound=u128_bound();
 for x in [&span.start,&span.end,&span.delta]{std.assert_lower_than_fixed(l,x,&bound)?;}
 let (from,to)=match sense{C6CounterSense::Increment=>(&span.start,&span.end),C6CounterSense::Decrement=>(&span.end,&span.start)};
 let sum=std.add(l,from,&span.delta)?;
 std.assert_equal(l,&sum,to)
}

/// Composes two adjacent spans: each one is valid, left.end==right.start, and
/// the composed span is (left.start, right.end, left.delta+right.delta), which
/// is checked again under the same contract. Deltas are added. Cumulative
/// endpoints are never added. Both extra checks are sound redundancy. Given
/// valid child spans, adjacency implies the composed equation and the composed
/// delta bound, and the composed equation implies adjacency. Deleting either
/// one alone admits no new witness, so no test can isolate adjacency, the
/// composed check or a composed-delta overflow.
pub fn compose_u128_spans(std:&ZkStdLib,l:&mut impl Layouter<F>,sense:C6CounterSense,left:&C6U128Span,right:&C6U128Span)->Result<C6U128Span,Error>{
 constrain_u128_span(std,l,sense,left)?;
 constrain_u128_span(std,l,sense,right)?;
 std.assert_equal(l,&left.end,&right.start)?;
 let delta=std.add(l,&left.delta,&right.delta)?;
 let out=C6U128Span{start:left.start.clone(),end:right.end.clone(),delta};
 constrain_u128_span(std,l,sense,&out)?;Ok(out)
}

/// The implemented component predicate. Callers must pass the SAME cells that
/// were verified as child instance lanes. The returned cells alias the inputs
/// (left program/context/pre, right post, outer counter endpoints). The only
/// new cells are the composed delta sums, and an eventual verifier must see
/// exactly these cells.
pub fn constrain_c6_adjacent_semantic_pair(std:&ZkStdLib,l:&mut impl Layouter<F>,shape:&C6BlockShape,senses:&C6Counters<C6CounterSense>,left:&C6RangeCells,right:&C6RangeCells)->Result<C6RangeCells,Error>{
 let syn=Error::Synthesis;
 left.check_shape(shape).map_err(syn)?;
 right.check_shape(shape).map_err(syn)?;
 assert_complete_equal(std,l,&left.program.0,&right.program.0)?;
 assert_complete_equal(std,l,&left.context.0,&right.context.0)?;
 assert_complete_equal(std,l,&left.post.0,&right.pre.0)?;
 let (s,a,b)=(senses,&left.counters,&right.counters);
 let counters=C6Counters{
  remaining_work:compose_u128_spans(std,l,s.remaining_work,&a.remaining_work,&b.remaining_work)?,
  consumed_work:compose_u128_spans(std,l,s.consumed_work,&a.consumed_work,&b.consumed_work)?,
  helper_work:compose_u128_spans(std,l,s.helper_work,&a.helper_work,&b.helper_work)?,
  helper_visits:compose_u128_spans(std,l,s.helper_visits,&a.helper_visits,&b.helper_visits)?,
  reference_cursor:compose_u128_spans(std,l,s.reference_cursor,&a.reference_cursor,&b.reference_cursor)?,
 };
 Ok(C6RangeCells{program:left.program.clone(),context:left.context.clone(),pre:left.pre.clone(),post:right.post.clone(),counters})
}

/// Splits a complete child instance into its semantic statement prefix and its
/// carry tail of acc_pi_len() fields. The tail is the same set of cells that
/// verify_child_incircuit decodes with reconstruct_acc and asserts equal to the
/// collapsed accumulator encoding. Here it is only split, never interpreted.
pub fn split_carry_tail<T>(instance:&[T],carried:bool)->Result<(&[T],&[T]),String>{
 if instance.len()>MAX_CHILD_INSTANCE{return Err("child instance exceeds existing loader bound".into())}
 let tail=if carried{acc_pi_len()}else{0};
 if instance.len()<=tail{return Err("semantic statement empty or carry tail incomplete".into())}
 Ok(instance.split_at(instance.len()-tail))
}

/// The smallest missing prerequisite for mapping a verified child to typed roles.
pub const C6_LEAF_ROLE_ADAPTER_PREREQUISITE:&str="authoritative versioned C6 child statement ABI from the C6 leaf/range issuer: for each pinned child VK, a lane map from the split_carry_tail statement prefix to the complete program, context, pre-boundary and post-boundary blocks and to the start/end/delta cells of remaining_work, consumed_work, helper_work, helper_visits and reference_cursor with each counter sense, source-bound into that child's VK and relation serialization";
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct C6Unsupported{pub prerequisite:&'static str}
/// Explicitly Unsupported. Native Segment16, StateVec12, link16+carry12 and the
/// one-family C6 carrier builder are different ABIs, and none of them is
/// relabelled as this statement.
pub fn c6_range_cells_from_verified_statement(statement:&[A],shape:&C6BlockShape)->Result<C6RangeCells,C6Unsupported>{
 let _=(statement,shape);
 Err(C6Unsupported{prerequisite:C6_LEAF_ROLE_ADAPTER_PREREQUISITE})
}

#[cfg(test)]
#[path="c6_semantic_parent_component_tests.rs"]
mod tests;
