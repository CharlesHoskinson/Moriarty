//! Explicit postorder stack. Reservations precede every recursive expansion.
use crate::sherlock::{Tree, Value};
use super::{rows::*, limits::{self, Limits, Locator, Result}};

#[derive(Clone, Copy)]
enum Input<'a> { UInt(u128), Bool(bool), List(&'a [u128]), Tree(&'a Tree) }
enum Task<'a> { Visit(Input<'a>), Finish { ty: u8, ctor: usize, fields: usize } }

fn schedule<'a>(input: Input<'a>, tasks: &mut Vec<Task<'a>>, limits: &mut Limits, loc: Locator) -> Result<()> {
    limits.reserve(TableId::Constants,1,loc)?;
    limits::push(tasks,Task::Visit(input),loc)
}

pub fn append(value: &Value, block: &mut ProgramBlock, limits: &mut Limits, loc: Locator) -> Result<ValueRef> {
    let input=match value { Value::UInt(n)=>Input::UInt(*n),Value::Bool(b)=>Input::Bool(*b),
        Value::List(xs)=>Input::List(xs),Value::Tree(t)=>Input::Tree(t) };
    let mut tasks=Vec::new(); let mut results=Vec::new();
    schedule(input,&mut tasks,limits,loc)?;
    while let Some(task)=tasks.pop() {
        let node=match task {
            Task::Visit(Input::UInt(n))=>Some(ConstantNode{uint_bytes:n.to_le_bytes(),..ConstantNode::default()}),
            Task::Visit(Input::Bool(b))=>Some(ConstantNode{kind:1,type_id:1,bool_value:b,..ConstantNode::default()}),
            Task::Visit(Input::List([]))=>Some(ConstantNode{kind:2,type_id:2,ctor_id:0,..ConstantNode::default()}),
            Task::Visit(Input::List(xs))=>{
                limits.reserve(TableId::ConstantFields,2,loc)?;
                limits::push(&mut tasks,Task::Finish{ty:2,ctor:1,fields:2},loc)?;
                schedule(Input::List(&xs[1..]),&mut tasks,limits,loc)?;
                schedule(Input::UInt(xs[0]),&mut tasks,limits,loc)?; None
            }
            Task::Visit(Input::Tree(Tree::Leaf(n)))=>{
                limits.reserve(TableId::ConstantFields,1,loc)?;
                limits::push(&mut tasks,Task::Finish{ty:3,ctor:2,fields:1},loc)?;
                schedule(Input::UInt(*n),&mut tasks,limits,loc)?; None
            }
            Task::Visit(Input::Tree(Tree::Branch(left,right)))=>{
                limits.reserve(TableId::ConstantFields,2,loc)?;
                limits::push(&mut tasks,Task::Finish{ty:3,ctor:3,fields:2},loc)?;
                schedule(Input::Tree(right),&mut tasks,limits,loc)?;
                schedule(Input::Tree(left),&mut tasks,limits,loc)?; None
            }
            Task::Finish{ty,ctor,fields}=>{
                let start=block.constant_fields.rows.len();
                let first=results.len().checked_sub(fields).ok_or_else(||limits::host(loc))?;
                // Child results are left-to-right; field slots are appended now,
                // with their parent, never reserved in the output before children.
                for &id in &results[first..] {
                    limits::push(&mut block.constant_fields.rows,ValueRef::constant(id),loc)?;
                }
                results.truncate(first);
                Some(ConstantNode{kind:2,type_id:ty,ctor_id:ctor,fields:Slice{start,len:fields},..ConstantNode::default()})
            }
        };
        if let Some(node)=node {
            let id=block.constants.rows.len();
            limits::push(&mut block.constants.rows,node,loc)?;
            limits::push(&mut results,id,loc)?;
        }
    }
    let root=results.pop().ok_or_else(||limits::host(loc))?;
    Ok(ValueRef::constant(root))
}
