//! Canonical host-only L4 encoder. No evaluator, decoder call, SDK or binding.
use crate::sherlock::{Arm, BinaryOp, Control, Environment, Expr, ExprRef, Frame,
    Function, Outcome, Program, Snapshot, Tree, Type, UnaryOp, Value};
use super::rows::{ConstantNode, PreparedA, Slice, ValueRef};
use super::limits::Locator;
use super::state::*;

fn loc(field: &'static str,item: usize)->Locator {Locator{expression:None,function:None,field,item}}
fn host(at:Locator)->StateFailure {StateFailure::HostUnrepresentable{field:at.field,locator:at}}
fn push<T>(v:&mut Vec<T>,x:T,at:Locator)->StateResult<()> {v.try_reserve(1).map_err(|_|host(at))?;v.push(x);Ok(())}
fn outside(field:&'static str,at:Locator)->StateFailure {StateFailure::OutsideProgramClosure{field,locator:at}}
fn ty(t:Type)->u8 {match t {Type::UInt=>0,Type::Bool=>1,Type::List=>2,Type::Tree=>3}}
fn binary(t:BinaryOp)->u8 {match t {BinaryOp::Add=>0,BinaryOp::Sub=>1,BinaryOp::Mul=>2,BinaryOp::Eq=>3,BinaryOp::Lt=>4,BinaryOp::Cons=>5,BinaryOp::Branch=>6}}
fn unary(t:UnaryOp)->u8 {match t {UnaryOp::Leaf=>0,UnaryOp::Not=>1}}
fn data(index:usize)->ValueRef {ValueRef{space:true,index}}

// Comparisons always have a fully prepared source on the right. Its total
// unfolded occurrence count bounds any equal subtree. Exceeding that count
// returns unequal, not a new resource refusal. Pending pairs count immediately.
fn pair<'a,T>(work:&mut Vec<(&'a T,&'a T)>,a:&'a T,b:&'a T,
    scheduled:&mut usize,max:usize)->StateResult<bool> {
    if *scheduled>=max {return Ok(false);}
    *scheduled+=1;push(work,(a,b),loc("structural comparison",*scheduled))?;Ok(true)
}
fn values_equal(p:&PreparedA,a:&Value,b:&Value)->StateResult<bool> {
    Ok(match (a,b) {
        (Value::UInt(a),Value::UInt(b))=>a==b,(Value::Bool(a),Value::Bool(b))=>a==b,
        (Value::List(a),Value::List(b))=>a.len()==b.len() && a==b,
        (Value::Tree(a),Value::Tree(b))=>{
            let mut work=Vec::new();let mut n=0;let max=p.block.constants.active;
            if !pair(&mut work,a,b,&mut n,max)? {return Ok(false);}
            while let Some((a,b))=work.pop() {
                match (a,b) {
                    (Tree::Leaf(a),Tree::Leaf(b)) if a==b=>{},
                    (Tree::Branch(al,ar),Tree::Branch(bl,br))=>{
                        if !pair(&mut work,ar.as_ref(),br.as_ref(),&mut n,max)? ||
                            !pair(&mut work,al.as_ref(),bl.as_ref(),&mut n,max)? {return Ok(false);}
                    }
                    _=>return Ok(false),
                }
            }
            true
        }
        _=>false,
    })
}
fn expr_equal(p:&PreparedA,a:&ExprRef,b:&ExprRef)->StateResult<bool> {
    let mut work=Vec::new();let mut n=0;let max=p.block.expressions.active;
    if !pair(&mut work,a.as_ref(),b.as_ref(),&mut n,max)? {return Ok(false);}
    macro_rules! add {($a:expr,$b:expr)=>{if !pair(&mut work,($a).as_ref(),($b).as_ref(),&mut n,max)? {return Ok(false);}};}
    while let Some((a,b))=work.pop() {
        match (a,b) {
            (Expr::Literal(a),Expr::Literal(b))=>if !values_equal(p,a,b)? {return Ok(false);},
            (Expr::Var(a),Expr::Var(b)) if a==b=>{},
            (Expr::Let(an,av,ab),Expr::Let(bn,bv,bb)) if an==bn=>{add!(ab,bb);add!(av,bv);}
            (Expr::Binary(ao,al,ar),Expr::Binary(bo,bl,br)) if ao==bo=>{add!(ar,br);add!(al,bl);}
            (Expr::Unary(ao,a),Expr::Unary(bo,b)) if ao==bo=>add!(a,b),
            (Expr::If(ac,at,ae),Expr::If(bc,bt,be))=>{add!(ae,be);add!(at,bt);add!(ac,bc);}
            (Expr::Call(an,aa),Expr::Call(bn,ba)) if an==bn && aa.len()==ba.len()=>{
                for (a,b) in aa.iter().zip(ba).rev() {add!(a,b);}
            }
            (Expr::Match(av,aa),Expr::Match(bv,ba)) if aa.len()==ba.len()=>{
                for (a,b) in aa.iter().zip(ba).rev() {
                    if a.pattern!=b.pattern {return Ok(false);}add!(&a.body,&b.body);
                }
                add!(av,bv);
            }
            _=>return Ok(false),
        }
    }
    Ok(true)
}
fn functions_equal(p:&PreparedA,a:&Function,b:&Function)->StateResult<bool> {
    if a.name!=b.name || a.return_type!=b.return_type || a.parameters.len()!=b.parameters.len() || a.parameters!=b.parameters {return Ok(false);}
    expr_equal(p,&a.body,&b.body)
}
/// Shared with the real decoder. No recursive Program/Expr/Tree derived Eq.
pub(super) fn programs_equal(p:&PreparedA,other:&Program)->StateResult<bool> {
    let right=p.full_program.as_ref();
    if other.functions.len()!=right.functions.len() || !expr_equal(p,&other.entry,&right.entry)? {return Ok(false);}
    for (a,b) in other.functions.iter().zip(&right.functions) {
        if !functions_equal(p,a,b)? {return Ok(false);}
    }
    Ok(true)
}
fn name_id(p:&PreparedA,name:&str,at:Locator)->StateResult<usize> {
    p.names.binary_search_by(|s|s.as_bytes().cmp(name.as_bytes())).map_err(|_|outside("name",at))
}
fn expression_id(p:&PreparedA,expr:&ExprRef,at:Locator)->StateResult<usize> {
    for (id,meta) in p.occurrences.iter().enumerate() {if expr_equal(p,expr,&meta.source)? {return Ok(id);}}
    Err(outside("expression",at))
}
fn function_id(p:&PreparedA,function:&Function,at:Locator)->StateResult<usize> {
    for (id,f) in p.full_program.functions.iter().enumerate() {if functions_equal(p,function,f)? {return Ok(id);}}
    Err(outside("function",at))
}
fn expression_suffix(p:&PreparedA,held:&[ExprRef],at:Locator)->StateResult<Slice> {
    if held.is_empty() {return Ok(Slice::ZERO);}
    let mut best:Option<usize>=None;
    for (id,meta) in p.occurrences.iter().enumerate() {
        if let Expr::Call(_,args)=meta.source.as_ref() {
            if args.len()<held.len() {continue;}
            let offset=args.len()-held.len();let mut equal=true;
            for (a,b) in held.iter().zip(&args[offset..]) {if !expr_equal(p,a,b)? {equal=false;break;}}
            if equal {
                let start=p.block.expressions.rows[id].children.start.checked_add(offset).ok_or_else(||host(at))?;
                best=Some(best.map_or(start,|old|old.min(start)));
            }
        }
    }
    best.map(|start|Slice{start,len:held.len()}).ok_or_else(||outside("remainingExprSlice",at))
}
fn arm_suffix(p:&PreparedA,held:&[Arm],at:Locator)->StateResult<Slice> {
    if held.is_empty() {return Ok(Slice::ZERO);}
    let mut best:Option<usize>=None;
    for (id,meta) in p.occurrences.iter().enumerate() {
        if let Expr::Match(_,arms)=meta.source.as_ref() {
            if arms.len()<held.len() {continue;}
            let offset=arms.len()-held.len();let mut equal=true;
            for (a,b) in held.iter().zip(&arms[offset..]) {
                if a.pattern!=b.pattern || !expr_equal(p,&a.body,&b.body)? {equal=false;break;}
            }
            if equal {
                let start=p.block.expressions.rows[id].arms.start.checked_add(offset).ok_or_else(||host(at))?;
                best=Some(best.map_or(start,|old|old.min(start)));
            }
        }
    }
    best.map(|start|Slice{start,len:held.len()}).ok_or_else(||outside("armSlice",at))
}

const TABLES:[&str;6]=["Data","DataFields","EnvCells","ValueSlots","FrameSlots","ControlRows"];
struct Builder<'a> { prepared:&'a PreparedA,block:StateBlock,capacities:[u128;6],host_caps:[usize;6],reserved:[usize;6] }
#[derive(Clone,Copy)]
enum Input<'a>{UInt(u128),Bool(bool),List(&'a [u128]),Tree(&'a Tree)}
enum Work<'a>{Visit(Input<'a>),Finish{ty:u8,ctor:usize,fields:usize}}
impl<'a> Builder<'a> {
    fn reserve(&mut self,table:usize,n:usize,at:Locator)->StateResult<()> {
        let needed=(self.reserved[table] as u128).checked_add(n as u128).ok_or_else(||host(at))?;
        if needed>self.capacities[table] {return Err(StateFailure::Capacity{table:TABLES[table],needed,limit:self.capacities[table],locator:at});}
        self.reserved[table]=usize::try_from(needed).map_err(|_|host(at))?;Ok(())
    }
    fn schedule<'b>(&mut self,tasks:&mut Vec<Work<'b>>,input:Input<'b>,at:Locator)->StateResult<()> {
        self.reserve(0,1,at)?;push(tasks,Work::Visit(input),at)
    }
    fn value(&mut self,value:&Value,at:Locator)->StateResult<ValueRef> {
        let input=match value {Value::UInt(n)=>Input::UInt(*n),Value::Bool(b)=>Input::Bool(*b),Value::List(xs)=>Input::List(xs),Value::Tree(t)=>Input::Tree(t)};
        let mut tasks=Vec::new();let mut results=Vec::new();self.schedule(&mut tasks,input,at)?;
        while let Some(work)=tasks.pop() {
            let node=match work {
                Work::Visit(Input::UInt(n))=>Some(ConstantNode{uint_bytes:n.to_le_bytes(),..ConstantNode::default()}),
                Work::Visit(Input::Bool(b))=>Some(ConstantNode{kind:1,type_id:1,bool_value:b,..ConstantNode::default()}),
                Work::Visit(Input::List([]))=>Some(ConstantNode{kind:2,type_id:2,..ConstantNode::default()}),
                Work::Visit(Input::List(xs))=>{
                    self.reserve(1,2,at)?;push(&mut tasks,Work::Finish{ty:2,ctor:1,fields:2},at)?;
                    self.schedule(&mut tasks,Input::List(&xs[1..]),at)?;self.schedule(&mut tasks,Input::UInt(xs[0]),at)?;None
                }
                Work::Visit(Input::Tree(Tree::Leaf(n)))=>{
                    self.reserve(1,1,at)?;push(&mut tasks,Work::Finish{ty:3,ctor:2,fields:1},at)?;
                    self.schedule(&mut tasks,Input::UInt(*n),at)?;None
                }
                Work::Visit(Input::Tree(Tree::Branch(left,right)))=>{
                    self.reserve(1,2,at)?;push(&mut tasks,Work::Finish{ty:3,ctor:3,fields:2},at)?;
                    self.schedule(&mut tasks,Input::Tree(right),at)?;self.schedule(&mut tasks,Input::Tree(left),at)?;None
                }
                Work::Finish{ty,ctor,fields}=>{
                    let first=results.len().checked_sub(fields).ok_or_else(||host(at))?;let start=self.block.data_fields.len();
                    for &id in &results[first..] {push(&mut self.block.data_fields,data(id),at)?;}
                    results.truncate(first);
                    Some(ConstantNode{kind:2,type_id:ty,ctor_id:ctor,fields:Slice{start,len:fields},..ConstantNode::default()})
                }
            };
            if let Some(node)=node {let id=self.block.data.len();push(&mut self.block.data,node,at)?;push(&mut results,id,at)?;}
        }
        Ok(data(results.pop().ok_or_else(||host(at))?))
    }
    fn environment(&mut self,environment:&Environment,at:Locator)->StateResult<EnvRoot> {
        let mut parent=EnvRoot::default();
        // BTreeMap<String,...> iteration is ascending exact UTF-8 name order.
        for (name,value) in environment {
            self.reserve(2,1,at)?;let name_id=name_id(self.prepared,name,at)?;
            let value=self.value(value,at)?;let index=self.block.env_cells.len();
            push(&mut self.block.env_cells,EnvCell{name_id,value,parent},at)?;
            parent=EnvRoot{present:true,index};
        }
        Ok(parent)
    }
    fn values(&mut self,values:&[Value],at:Locator)->StateResult<Slice> {
        if values.is_empty() {return Ok(Slice::ZERO);}
        self.reserve(3,values.len(),at)?;let mut refs=Vec::new();
        for value in values {let value=self.value(value,at)?;push(&mut refs,value,at)?;}
        let start=self.block.value_slots.len();
        for value in refs {push(&mut self.block.value_slots,value,at)?;}
        Ok(Slice{start,len:values.len()})
    }
    fn frame(&mut self,frame:&Frame,at:Locator)->StateResult<FrameRow> {
        let mut r=FrameRow::default();
        match frame {
            Frame::Let(name,expr,env)=>{r.tag=0;r.name_id=name_id(self.prepared,name,at)?;r.expr0=expression_id(self.prepared,expr,at)?;r.saved_env=self.environment(env,at)?;}
            Frame::Left(op,expr,env)=>{r.tag=1;r.binary_op=binary(*op);r.expr0=expression_id(self.prepared,expr,at)?;r.saved_env=self.environment(env,at)?;}
            Frame::Right(op,value)=>{r.tag=2;r.binary_op=binary(*op);r.value=self.value(value,at)?;}
            Frame::Unary(op)=>{r.tag=3;r.unary_op=unary(*op);}
            Frame::If(a,b,env)=>{r.tag=4;r.expr0=expression_id(self.prepared,a,at)?;r.expr1=expression_id(self.prepared,b,at)?;r.saved_env=self.environment(env,at)?;}
            Frame::Match(arms,env)=>{r.tag=5;r.arms=arm_suffix(self.prepared,arms,at)?;r.saved_env=self.environment(env,at)?;}
            Frame::Args(name,done,rest,env)=>{r.tag=6;r.name_id=name_id(self.prepared,name,at)?;r.completed=self.values(done,at)?;r.remaining_exprs=expression_suffix(self.prepared,rest,at)?;r.saved_env=self.environment(env,at)?;}
            Frame::Return(t,env)=>{r.tag=7;r.type_id=ty(*t);r.saved_env=self.environment(env,at)?;}
            Frame::Scope(env)=>{r.tag=8;r.saved_env=self.environment(env,at)?;}
        }
        Ok(r)
    }
    fn control(&mut self,control:&Control)->StateResult<()> {
        let mut cursor=control;
        loop {
            let index=self.block.control_rows.len();let at=loc("ControlRows",index);self.reserve(5,1,at)?;
            let mut r=ControlRow::default();let mut next=None;
            match cursor {
                Control::Eval(expr)=>{r.tag=0;r.expr_id=expression_id(self.prepared,expr,at)?;}
                Control::Return(value)=>{r.tag=1;r.value=self.value(value,at)?;}
                Control::Push(frame,tail,environment)=>{
                    r.tag=2;r.pending_frame=self.frame(frame,loc("ControlRows.pendingFrame",index))?;
                    r.environment=self.environment(environment,loc("ControlRows.environment",index))?;next=Some(tail.as_ref());
                }
                Control::Invoke(name,values)=>{r.tag=3;r.name_id=name_id(self.prepared,name,at)?;r.values=self.values(values,at)?;}
                Control::Enter(function,values)=>{
                    r.tag=4;if let Some(f)=function {r.function_present=true;r.function_id=function_id(self.prepared,f,at)?;}
                    r.values=self.values(values,at)?;
                }
                Control::Bound(bindings,t,expr)=>{
                    r.tag=5;if let Some(env)=bindings {r.bindings_present=true;r.bindings=self.environment(env,at)?;}
                    r.type_id=ty(*t);r.expr_id=expression_id(self.prepared,expr,at)?;
                }
                Control::Cases(value,arms,env)=>{r.tag=6;r.value=self.value(value,at)?;r.arms=arm_suffix(self.prepared,arms,at)?;r.environment=self.environment(env,at)?;}
                Control::CaseResult(bindings,value,expr,arms,env)=>{
                    r.tag=7;if let Some(env)=bindings {r.bindings_present=true;r.bindings=self.environment(env,at)?;}
                    r.value=self.value(value,at)?;r.expr_id=expression_id(self.prepared,expr,at)?;
                    r.arms=arm_suffix(self.prepared,arms,at)?;r.environment=self.environment(env,at)?;
                }
                Control::PrimitiveResult(result)=>{r.tag=8;match result {Ok(value)=>{r.primitive_is_ok=true;r.value=self.value(value,at)?;},Err(code)=>r.error_code=*code}}
                Control::Error(code)=>{r.tag=9;r.error_code=*code;}
            }
            push(&mut self.block.control_rows,r,at)?;
            if let Some(tail)=next {cursor=tail;}else{break;}
        }
        Ok(())
    }
}

fn pad<T:Default+Clone>(rows:&mut Vec<T>,capacity:usize,at:Locator)->StateResult<()> {
    let extra=capacity.checked_sub(rows.len()).ok_or_else(||host(at))?;
    rows.try_reserve_exact(extra).map_err(|_|host(at))?;rows.resize(capacity,T::default());Ok(())
}

pub(super) fn encode_observed_a(prepared:&PreparedA,caps:&StateCapacities,snapshot:&Snapshot)->StateResult<StateBlock> {
    if caps.data!=prepared.capacities.data {
        return Err(StateFailure::WrongRepresentation{block:"StateBlock",table:"Root",row:0,column:"D",definition:"StateCapacities.data must equal PreparedA.capacities.data"});
    }
    if !programs_equal(prepared,&snapshot.program)? {return Err(StateFailure::ProgramMismatch{locator:loc("Snapshot.program",0)});}
    if snapshot.remaining_fuel>snapshot.initial_fuel {return Err(StateFailure::OutsideObservationDomain{condition:"O1",locator:loc("Snapshot.remaining_fuel",0)});}
    if snapshot.frames.len()>snapshot.stack_limit {return Err(StateFailure::OutsideObservationDomain{condition:"O2",locator:loc("Snapshot.frames",0)});}
    if matches!(&snapshot.outcome,Outcome::Pending)!=snapshot.control.is_some() {return Err(StateFailure::OutsideObservationDomain{condition:"O3",locator:loc("Root.controlLength",0)});}
    if snapshot.stack_limit as u128>caps.stack_limit {
        return Err(StateFailure::Capacity{table:"Root.stackLimit",needed:snapshot.stack_limit as u128,limit:caps.stack_limit,locator:loc("Snapshot.stack_limit",0)});
    }
    let capacities=[caps.data,caps.data_fields,caps.env_cells,caps.value_slots,caps.frame_slots,caps.control_rows];let mut host_caps=[0;6];
    for (i,n) in capacities.iter().enumerate() {host_caps[i]=usize::try_from(*n).map_err(|_|host(loc(TABLES[i],0)))?;}
    let mut b=Builder{prepared,block:StateBlock::default(),capacities,host_caps,reserved:[0;6]};
    if let Some(control)=&snapshot.control {b.control(control)?;}
    b.block.root.environment=b.environment(&snapshot.environment,loc("Root.environment",0))?;
    for (i,frame) in snapshot.frames.iter().enumerate() {
        let at=loc("FrameSlots",i);b.reserve(4,1,at)?;let row=b.frame(frame,at)?;push(&mut b.block.frame_slots,row,at)?;
    }
    match &snapshot.outcome {
        Outcome::Pending=>b.block.root.outcome_tag=0,
        Outcome::Done(value)=>{b.block.root.outcome_tag=1;b.block.root.outcome_value=b.value(value,loc("Root.outcomeValue",0))?;}
        Outcome::Rejected(code)=>{b.block.root.outcome_tag=2;b.block.root.outcome_error=*code;}
        Outcome::Exhausted=>b.block.root.outcome_tag=3,
    }
    b.block.root.active_data=b.block.data.len();b.block.root.active_data_fields=b.block.data_fields.len();
    b.block.root.active_env_cells=b.block.env_cells.len();b.block.root.active_value_slots=b.block.value_slots.len();
    b.block.root.depth=b.block.frame_slots.len();b.block.root.control_length=b.block.control_rows.len();
    b.block.root.stack_limit=snapshot.stack_limit as u128;b.block.root.remaining_fuel=snapshot.remaining_fuel.to_le_bytes();b.block.root.initial_fuel=snapshot.initial_fuel.to_le_bytes();
    pad(&mut b.block.data,b.host_caps[0],loc("Data padding",0))?;pad(&mut b.block.data_fields,b.host_caps[1],loc("DataFields padding",0))?;
    pad(&mut b.block.env_cells,b.host_caps[2],loc("EnvCells padding",0))?;pad(&mut b.block.value_slots,b.host_caps[3],loc("ValueSlots padding",0))?;
    pad(&mut b.block.frame_slots,b.host_caps[4],loc("FrameSlots padding",0))?;pad(&mut b.block.control_rows,b.host_caps[5],loc("ControlRows padding",0))?;
    Ok(b.block)
}
