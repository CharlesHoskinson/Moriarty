//! Proposed Source46 binary verification kernel, source-only/uncompiled/unrun.
//! Fixed keys/shapes/modes are supplied by a NEW authoritative relation. This
//! helper neither authenticates source nor constrains semantic range adjacency.
//! Its result carries deferred pairing obligations; mandatory D remains.
use midnight_circuits::{instructions::*,types::AssignedNative,verifier::{Accumulator,AssignedAccumulator}};
use midnight_proofs::{circuit::{Layouter,Value},plonk::Error};
use midnight_zk_stdlib::{MidnightVK,ZkStdLib};
use crate::{inner::{acc_pi_len,prepare_offcircuit,verify_child_incircuit,vk_name,S},sherlock_child::{MAX_CHILD_INSTANCE,MAX_CHILD_PROOF_BYTES},util::{vk_bytes,F}};

#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize)]
pub enum ChildKind { Leaf,Recursive }
/// All public nonaccumulator fields, including every binding/semantic role.
/// No numeric C6 statement layout is chosen by this interface.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize)]
pub struct ChildShape { pub statement_lanes:usize,pub kind:ChildKind }
impl ChildShape {
 pub fn carries(&self)->bool{self.kind==ChildKind::Recursive}
 pub fn instance_lanes(&self)->Result<usize,String>{
  if self.statement_lanes==0{return Err("empty semantic statement".into())}
  let n=self.statement_lanes.checked_add(if self.carries(){acc_pi_len()}else{0}).ok_or("child instance length overflow")?;
  if !(2..=MAX_CHILD_INSTANCE).contains(&n){return Err("child instance exceeds existing loader bound".into())}Ok(n)
 }
 pub fn check(&self,n:usize)->Result<(),String>{if n!=self.instance_lanes()?{return Err("child instance differs from fixed complete shape".into())}Ok(())}
}
/// Not a key issuer: caller must reconstruct/pin these VKs independently and
/// serialize them with the new semantic relation as fixed circuit constants.
pub struct FixedChild<'a>{vk:&'a MidnightVK,shape:ChildShape}
impl<'a> FixedChild<'a>{
 pub fn new(vk:&'a MidnightVK,shape:ChildShape)->Result<Self,String>{shape.instance_lanes()?;Ok(Self{vk,shape})}
 fn name(&self)->String{vk_name("c6_binary_child",self.vk)}
}
fn check_pair(children:&[FixedChild<'_>;2])->Result<(),String>{
 for child in children{child.shape.instance_lanes()?;}
 if children[0].name()==children[1].name()&&vk_bytes(children[0].vk)!=vk_bytes(children[1].vk){return Err("fixed VK namespace collision".into())}Ok(())
}
fn proof_size(n:usize)->Result<(),String>{if n==0||n>MAX_CHILD_PROOF_BYTES{return Err("empty or oversized child proof".into())}Ok(())}

/// Off-circuit mirror only: no final pairing or semantic-validity acceptance.
pub fn prepare_binary_deferred(children:&[FixedChild<'_>;2],instances:[&[F];2],proofs:[&[u8];2])->Result<Accumulator<S>,String>{
 check_pair(children)?;for i in 0..2{children[i].shape.check(instances[i].len())?;proof_size(proofs[i].len())?;}
 let left=prepare_offcircuit(children[0].vk,&children[0].name(),instances[0],proofs[0],children[0].shape.carries())?;
 let right=prepare_offcircuit(children[1].vk,&children[1].name(),instances[1],proofs[1],children[1].shape.carries())?;
 let mut combined=Accumulator::accumulate(&[left,right]);combined.collapse();Ok(combined)
}
/// Same left/right, own/carried, resolve/collapse order as the host mirror.
/// Caller constrains complete semantic roles over THESE SAME instance cells
/// and exposes this resulting accumulator as the new parent's public tail.
pub fn verify_binary_incircuit(std:&ZkStdLib,l:&mut impl Layouter<F>,children:&[FixedChild<'_>;2],instances:[&[AssignedNative<F>];2],proofs:[Value<Vec<u8>>;2])->Result<AssignedAccumulator<S>,Error>{
 let syn=Error::Synthesis;check_pair(children).map_err(syn)?;
 for i in 0..2{
  children[i].shape.check(instances[i].len()).map_err(syn)?;
  proofs[i].as_ref().map(|p|p.len()).error_if_known_and(|n|*n==0||*n>MAX_CHILD_PROOF_BYTES)?;
 }
 let left=verify_child_incircuit(std,l,children[0].vk,&children[0].name(),instances[0],proofs[0].clone(),children[0].shape.carries())?;
 let right=verify_child_incircuit(std,l,children[1].vk,&children[1].name(),instances[1],proofs[1].clone(),children[1].shape.carries())?;
 let mut combined=std.verifier().accumulate(l,&[left,right])?;
 combined.collapse(l,std.bls12_381(),std.bls12_381().scalar_field_chip())?;Ok(combined)
}

#[cfg(test)]
mod tests{
 use super::*;
 // Shape-only source tests. No key/proof/SDK operation is executed in this packet.
 #[test]fn leaf_shape_never_implicitly_carries(){let s=ChildShape{statement_lanes:2,kind:ChildKind::Leaf};assert!(!s.carries());assert_eq!(s.instance_lanes().unwrap(),2);assert!(s.check(3).is_err());}
 #[test]fn recursive_shape_requires_the_entire_carry_tail(){let s=ChildShape{statement_lanes:2,kind:ChildKind::Recursive};assert!(s.carries());assert_eq!(s.instance_lanes().unwrap(),2+acc_pi_len());assert!(s.check(2).is_err());}
 #[test]fn shapes_refuse_empty_overflow_and_loader_bound(){for s in [ChildShape{statement_lanes:0,kind:ChildKind::Leaf},ChildShape{statement_lanes:usize::MAX,kind:ChildKind::Recursive},ChildShape{statement_lanes:MAX_CHILD_INSTANCE,kind:ChildKind::Recursive}]{assert!(s.instance_lanes().is_err());}}
}

// Nonsecurity data plumbing only. No parsing/admission/constraint is added.
#[derive(serde::Serialize)]
struct ConstantParts<'a>{shape:&'a ChildShape,vk_bytes:&'a [u8]}
fn write_constant_parts<W:std::io::Write>(writer:&mut W,parts:&[ConstantParts<'_>;2])->serde_json::Result<()>{serde_json::to_writer(writer,parts)}
/// Ordered, complete public relation-parameter view for a future Opus-authored
/// relation writer. This is not an authoritative relation/proof format and does
/// not replace its separately reviewed versioned serialization or key binding.
pub fn write_fixed_children_json<W:std::io::Write>(writer:&mut W,children:&[FixedChild<'_>;2])->serde_json::Result<()>{
 let left=vk_bytes(children[0].vk);let right=vk_bytes(children[1].vk);
 write_constant_parts(writer,&[ConstantParts{shape:&children[0].shape,vk_bytes:&left},ConstantParts{shape:&children[1].shape,vk_bytes:&right}])
}
#[cfg(test)]
mod serialization_tests{
 use super::*;
 #[test]fn ordered_constant_data_preserves_every_field_and_byte(){
  let left=ChildShape{statement_lanes:2,kind:ChildKind::Leaf};let right=ChildShape{statement_lanes:3,kind:ChildKind::Recursive};let mut bytes=Vec::new();
  write_constant_parts(&mut bytes,&[ConstantParts{shape:&left,vk_bytes:&[0,255]},ConstantParts{shape:&right,vk_bytes:&[1,1,2]}]).unwrap();
  assert_eq!(String::from_utf8(bytes).unwrap(),r#"[{"shape":{"statement_lanes":2,"kind":"Leaf"},"vk_bytes":[0,255]},{"shape":{"statement_lanes":3,"kind":"Recursive"},"vk_bytes":[1,1,2]}]"#);
 }
 struct Refused;
 impl std::io::Write for Refused{
  fn write(&mut self,_:&[u8])->std::io::Result<usize>{Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied,"constant-writer injected"))}
  fn flush(&mut self)->std::io::Result<()>{Ok(())}
 }
 #[test]fn ordered_constant_data_propagates_owned_writer_io_refusal(){
  let shape=ChildShape{statement_lanes:2,kind:ChildKind::Leaf};let error=write_constant_parts(&mut Refused,&[ConstantParts{shape:&shape,vk_bytes:&[]},ConstantParts{shape:&shape,vk_bytes:&[1]}]).unwrap_err();assert!(error.is_io());assert!(error.to_string().contains("constant-writer injected"));
 }
}
