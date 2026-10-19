//! Read-only semantic reference projection of saved, actual B observations.
//!
//! No source/owner step is evaluated here. The caller retains the exact original
//! A syntax; all held expressions/functions/arms are associated with actual B
//! occurrence/table rows. This is trusted host diagnostics, not authentication,
//! canonical physical projection, state import, admission or ProjectRel/Link.
//! Raw B snapshots and helper intervals remain available separately. A logical
//! reference fuel input is explicit; B remaining/helper work is never renamed A
//! fuel. Unmapped/incomplete/resource-divergent observations return typed gaps.
//! Generic nominal constructs, planned matches and constructor state are outside
//! the dedicated `sherlock-k-a/1` grammar and return explicit UnsupportedGeneric.
use super::complete_observation::{AdmissionSnapshot, CompleteStepRecord, ConstantSnapshot, ExpressionSnapshot, HostSnapshot, ProgramSnapshot};
use super::shared::*;
use super::witness_plan::{self as wp, DecodedPayload, DecodedSnapshot, DecodedValue, InputValue, SlotValue, ValueNode};
use crate::sherlock as a;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionGap {
    Witness(wp::WitnessFailure),
    SourceAssociation { section: &'static str, row: usize },
    MissingField(&'static str),
    WrongPayload(&'static str),
    UnrepresentableValue,
    UnsupportedNominalType(usize),
    ReferenceFuel,
    AdmissionChanged,
    Discontinuity,
    UnmappedRule(RuleIdC),
    UnappliedSourceRule,
    IncompleteInterval,
    BResourceOutcome,
    /// A generic nominal form this finite dedicated A projection cannot represent.
    UnsupportedGeneric(GenericGap),
}
/// Generic forms outside the dedicated A value/expression/pattern/frame/control
/// grammar. Nominal identities are retained, never flattened into List/Tree; a
/// full generic reference projection remains a separate open obligation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenericGap {
    ConstructExpression { row: usize, type_id: TypeIdC, ctor_id: adt::CtorIdC },
    PlannedMatch { row: usize, plan: usize },
    WildcardPattern { row: usize },
    ConstructorPattern { row: usize, type_id: TypeIdC, ctor_id: adt::CtorIdC },
    ConstructorConstant { row: usize, type_id: TypeIdC, ctor_id: adt::CtorIdC },
    ConstructorValue { type_id: TypeIdC, ctor_id: adt::CtorIdC, arity: usize },
    ConstructorFieldsFrame { type_id: TypeIdC, ctor_id: adt::CtorIdC, index: usize },
    MaterializeControl { type_id: TypeIdC, ctor_id: adt::CtorIdC, arity: usize },
    /// Residual profile refusal: the actual admission identity is not
    /// DedicatedSorts or an ADT/declaration/plan table is nonempty. Every count
    /// is retained; declarations are never skipped as unused.
    GenericProfile { grammar_domain: GrammarDomain, types: usize, type_args: usize, constructors: usize,
        ctor_fields: usize, decl_param_counts: usize, decl_ctor_counts: usize, decl_templates: usize,
        plan_nodes: usize, plan_cases: usize, field_paths: usize, pattern_children: usize },
}
type Result<T> = std::result::Result<T, ProjectionGap>;
fn source_gap(section: &'static str, row: usize) -> ProjectionGap {
    ProjectionGap::SourceAssociation { section, row }
}
fn generic(gap: GenericGap) -> ProjectionGap { ProjectionGap::UnsupportedGeneric(gap) }
fn bin(op: BinaryOp) -> a::BinaryOp { match op {
    BinaryOp::Add => a::BinaryOp::Add, BinaryOp::Sub => a::BinaryOp::Sub,
    BinaryOp::Mul => a::BinaryOp::Mul, BinaryOp::Eq => a::BinaryOp::Eq,
    BinaryOp::Lt => a::BinaryOp::Lt, BinaryOp::Cons => a::BinaryOp::Cons,
    BinaryOp::Branch => a::BinaryOp::Branch,
} }
fn unary(op: UnaryOp) -> a::UnaryOp { match op {
    UnaryOp::Leaf => a::UnaryOp::Leaf, UnaryOp::Not => a::UnaryOp::Not,
} }
fn ty(id: TypeIdC) -> Result<a::Type> { Ok(match id.0 {
    0 => a::Type::UInt, 1 => a::Type::Bool, 2 => a::Type::List, 3 => a::Type::Tree,
    n => return Err(ProjectionGap::UnsupportedNominalType(n)),
}) }
/// Profile pre-scan over the full observed program: generic rows are refused
/// before any caller-supplied A source association, never as a source mismatch.
fn generic_rows(view: &ProgramSnapshot) -> Result<()> {
    for (row, expression) in view.expressions.iter().enumerate() { match *expression {
        ExpressionSnapshot::Construct { type_id, ctor_id, .. } => return Err(generic(GenericGap::ConstructExpression { row, type_id, ctor_id })),
        ExpressionSnapshot::Match { plan: Some(plan), .. } => return Err(generic(GenericGap::PlannedMatch { row, plan })),
        ExpressionSnapshot::Literal { .. } | ExpressionSnapshot::Var { .. } | ExpressionSnapshot::Let { .. } | ExpressionSnapshot::Binary { .. }
        | ExpressionSnapshot::Unary { .. } | ExpressionSnapshot::If { .. } | ExpressionSnapshot::Call { .. } | ExpressionSnapshot::Match { plan: None, .. } => {}
    } }
    for (row, pattern) in view.patterns.iter().enumerate() { match *pattern {
        PatternRowC::Wildcard => return Err(generic(GenericGap::WildcardPattern { row })),
        PatternRowC::Constructor { type_id, ctor_id, .. } => return Err(generic(GenericGap::ConstructorPattern { row, type_id, ctor_id })),
        PatternRowC::PVar { .. } | PatternRowC::PNil | PatternRowC::PCons { .. } | PatternRowC::PLeaf { .. } | PatternRowC::PBranch { .. } => {}
    } }
    for (row, constant) in view.constants.iter().enumerate() {
        if let ConstantSnapshot::Constructor { type_id, ctor_id, .. } = *constant { return Err(generic(GenericGap::ConstructorConstant { row, type_id, ctor_id })); }
    }
    for function in &view.functions { ty(function.return_type)?; }
    for parameter in &view.parameters { ty(parameter.sort)?; }
    Ok(())
}
/// Residual profile check, run after the specific row/signature scans: the
/// actual admission identity must be DedicatedSorts and every ADT, declaration
/// and plan table empty. Runs before any caller-supplied A source association.
fn generic_profile(admission: &AdmissionSnapshot) -> Result<()> {
    let (grammar_domain, view) = (admission.identity.grammar_domain, &admission.view);
    let counts = [view.types.len(), view.type_args.len(), view.constructors.len(), view.ctor_fields.len(),
        view.decl_param_counts.len(), view.decl_ctor_counts.len(), view.decl_templates.len(),
        view.plan_nodes.len(), view.plan_cases.len(), view.field_paths.len(), view.pattern_children.len()];
    if grammar_domain == GrammarDomain::DedicatedSorts && counts.iter().all(|&n| n == 0) { return Ok(()); }
    let [types, type_args, constructors, ctor_fields, decl_param_counts, decl_ctor_counts, decl_templates,
        plan_nodes, plan_cases, field_paths, pattern_children] = counts;
    Err(generic(GenericGap::GenericProfile { grammar_domain, types, type_args, constructors, ctor_fields,
        decl_param_counts, decl_ctor_counts, decl_templates, plan_nodes, plan_cases, field_paths, pattern_children }))
}
fn field<'a>(fields: &'a [wp::NamedValue], name: &'static str) -> Result<&'a SlotValue> {
    fields.iter().find(|f| f.field == name).map(|f| &f.value).ok_or(ProjectionGap::MissingField(name))
}
fn payload<'a>(value: &'a SlotValue, name: &'static str) -> Result<&'a DecodedPayload> {
    if let SlotValue::Payload(p) = value { Ok(p) } else { Err(ProjectionGap::WrongPayload(name)) }
}
fn sequence<'a>(value: &'a SlotValue, name: &'static str) -> Result<&'a [SlotValue]> {
    if let SlotValue::Sequence { items, .. } = value { Ok(items) } else { Err(ProjectionGap::WrongPayload(name)) }
}
/// Convert the existing flat, fully resolved witness value; no AST evaluation.
pub fn reference_value(value: &DecodedValue) -> Result<a::Value> {
    let mut nodes: Vec<a::Value> = Vec::with_capacity(value.nodes.len());
    for row in &value.nodes {
        let child = |i: usize| nodes.get(i).cloned().ok_or(ProjectionGap::UnrepresentableValue);
        let result = match *row {
            ValueNode::UInt(n) => a::Value::UInt(n), ValueNode::Bool(b) => a::Value::Bool(b),
            ValueNode::Nil => a::Value::List(Vec::new()),
            ValueNode::Cons { head, tail } => {
                let (a::Value::UInt(n), a::Value::List(tail)) = (child(head)?, child(tail)?) else { return Err(ProjectionGap::UnrepresentableValue); };
                let mut list = Vec::with_capacity(tail.len() + 1); list.push(n); list.extend(tail); a::Value::List(list)
            }
            ValueNode::Leaf { value } => {
                let a::Value::UInt(n) = child(value)? else { return Err(ProjectionGap::UnrepresentableValue); };
                a::Value::Tree(a::Tree::Leaf(n))
            }
            ValueNode::Branch { left, right } => {
                let (a::Value::Tree(l), a::Value::Tree(r)) = (child(left)?, child(right)?) else { return Err(ProjectionGap::UnrepresentableValue); };
                a::Value::Tree(a::Tree::Branch(Arc::new(l), Arc::new(r)))
            }
            ValueNode::U128OutOfRange => return Err(ProjectionGap::UnrepresentableValue),
            ValueNode::Constructor { type_id, ctor_id, ref fields } => return Err(generic(GenericGap::ConstructorValue { type_id, ctor_id, arity: fields.len() })),
        };
        nodes.push(result);
    }
    nodes.get(value.root).cloned().ok_or(ProjectionGap::UnrepresentableValue)
}
fn value(slot: &SlotValue) -> Result<a::Value> {
    if let SlotValue::Value(v) = slot { reference_value(v) } else { Err(ProjectionGap::WrongPayload("value")) }
}
fn values(slot: &SlotValue) -> Result<Vec<a::Value>> {
    sequence(slot, "values")?.iter().map(value).collect()
}
/// A's map meaning keeps the first visible name in the exact newest-first B
/// chain. Shadowed cells remain in the caller's complete raw/decoded snapshot.
fn environment(slot: &SlotValue) -> Result<a::Environment> {
    let SlotValue::Environment(entries) = slot else { return Err(ProjectionGap::WrongPayload("environment")); };
    let mut result = BTreeMap::new();
    for entry in entries {
        if !result.contains_key(&entry.name) { result.insert(entry.name.clone(), reference_value(&entry.value)?); }
    }
    Ok(result)
}

/// Retained exact source syntax associated with full observed program rows.
/// Source association is a tooling comparison, not a security/source theorem.
pub struct ReferenceSource {
    program: Arc<a::Program>, admission: AdmissionSnapshot,
    expressions: Vec<a::ExprRef>, arms: Vec<a::Arm>,
}
impl ReferenceSource {
    pub fn new(program: Arc<a::Program>, initial: &HostSnapshot) -> Result<Self> {
        let view = &initial.admission.view;
        generic_rows(view)?;
        generic_profile(&initial.admission)?;
        if view.expressions.len() != view.occurrences.len() || view.functions.len() != program.functions.len() {
            return Err(source_gap("program", 0));
        }
        let mut expressions = Vec::with_capacity(view.expressions.len());
        for (id, occurrence) in view.occurrences.iter().enumerate() {
            let mut expr = if occurrence.owner_is_function {
                program.functions.get(occurrence.function).ok_or(source_gap("function", id))?.body.clone()
            } else {
                if occurrence.function != 0 { return Err(source_gap("entry", id)); }
                program.entry.clone()
            };
            let end = occurrence.path.start.checked_add(occurrence.path.len).ok_or(source_gap("path", id))?;
            let path = view.occurrence_paths.get(occurrence.path.start..end).ok_or(source_gap("path", id))?;
            for &index in path {
                expr = match expr.as_ref() {
                    a::Expr::Let(_, x, y) | a::Expr::Binary(_, x, y) => match index { 0 => x.clone(), 1 => y.clone(), _ => return Err(source_gap("path", id)) },
                    a::Expr::Unary(_, x) if index == 0 => x.clone(),
                    a::Expr::If(x,y,z) => match index { 0=>x.clone(),1=>y.clone(),2=>z.clone(),_=>return Err(source_gap("path",id)) },
                    a::Expr::Call(_, args) => args.get(index).cloned().ok_or(source_gap("call path", id))?,
                    a::Expr::Match(x, arms) => if index == 0 { x.clone() } else { arms.get(index-1).ok_or(source_gap("arm path", id))?.body.clone() },
                    _ => return Err(source_gap("path", id)),
                };
            }
            expressions.push(expr);
        }
        let materialized = wp::materialize(initial).map_err(ProjectionGap::Witness)?;
        let constants = sequence(field(&materialized.evaluation.decoded.fields,"constants")?,"constants")?;
        let mut arms: Vec<Option<a::Arm>> = vec![None; view.arms.len()];
        let mut result = Self { program, admission: initial.admission.clone(), expressions, arms: Vec::new() };
        for (i, row) in view.expressions.iter().enumerate() {
            let source = result.expr(i)?;
            let equal = match (row,source.as_ref()) {
                (ExpressionSnapshot::Literal {constant}, a::Expr::Literal(v)) => value(constants.get(*constant).ok_or(source_gap("constant",i))?)? == *v,
                (ExpressionSnapshot::Var {binder}, a::Expr::Var(n)) => result.name(*binder)? == *n,
                (ExpressionSnapshot::Let {binder,value,body},a::Expr::Let(n,x,y)) => result.name(*binder)? == *n && result.expr(*value)? == *x && result.expr(*body)? == *y,
                (ExpressionSnapshot::Binary {op,left,right},a::Expr::Binary(o,x,y)) => bin(*op)==*o && result.expr(*left)?==*x && result.expr(*right)?==*y,
                (ExpressionSnapshot::Unary {op,operand},a::Expr::Unary(o,x)) => unary(*op)==*o && result.expr(*operand)?==*x,
                (ExpressionSnapshot::If {cond,then_expr,else_expr},a::Expr::If(x,y,z)) => result.expr(*cond)?==*x && result.expr(*then_expr)?==*y && result.expr(*else_expr)?==*z,
                (ExpressionSnapshot::Call {function_name,args},a::Expr::Call(n,xs)) => result.name(*function_name)?==*n && result.exprs(*args)?==*xs,
                (ExpressionSnapshot::Match {scrutinee,arms: slice,plan: None},a::Expr::Match(x,source_arms)) => {
                    if result.expr(*scrutinee)? != *x || slice.len != source_arms.len() { false } else {
                        for (offset, arm) in source_arms.iter().enumerate() {
                            let at = slice.start.checked_add(offset).ok_or(source_gap("arms",i))?;
                            let row = view.arms.get(at).ok_or(source_gap("arms",at))?;
                            if result.expr(row.body)? != arm.body || result.pattern(row.pattern)? != arm.pattern { return Err(source_gap("arms",at)); }
                            if let Some(previous) = &arms[at] { if previous != arm { return Err(source_gap("arm sharing",at)); } }
                            arms[at] = Some(arm.clone());
                        }
                        true
                    }
                }
                _ => false,
            };
            if !equal { return Err(source_gap("expression",i)); }
        }
        result.arms = arms.into_iter().enumerate().map(|(i,x)|x.ok_or(source_gap("unassociated arm",i))).collect::<Result<_>>()?;
        for (i,(row, function)) in view.functions.iter().zip(&result.program.functions).enumerate() {
            if result.name(row.function_name)?!=function.name || ty(row.return_type)?!=function.return_type || result.expr(row.body)?!=function.body || row.params.len!=function.parameters.len() {
                return Err(source_gap("function",i));
            }
            for (offset,parameter) in function.parameters.iter().enumerate() {
                let index=row.params.start.checked_add(offset).ok_or(source_gap("parameter",i))?;
                let p=view.parameters.get(index).ok_or(source_gap("parameter",index))?;
                if result.name(p.binder)?!=parameter.name || ty(p.sort)?!=parameter.ty { return Err(source_gap("parameter",index)); }
            }
        }
        if result.expr(view.entry)? != result.program.entry { return Err(source_gap("entry",view.entry)); }
        Ok(result)
    }
    fn name(&self, id: usize) -> Result<String> { self.admission.view.names.get(id).cloned().ok_or(source_gap("name",id)) }
    fn expr(&self, id: usize) -> Result<a::ExprRef> { self.expressions.get(id).cloned().ok_or(source_gap("expression",id)) }
    fn exprs(&self, slice: SliceC) -> Result<Vec<a::ExprRef>> {
        let end=slice.start.checked_add(slice.len).ok_or(source_gap("children",slice.start))?;
        self.admission.view.children.get(slice.start..end).ok_or(source_gap("children",slice.start))?.iter().map(|&i|self.expr(i)).collect()
    }
    fn arms(&self, slice: SliceC) -> Result<Vec<a::Arm>> {
        let end=slice.start.checked_add(slice.len).ok_or(source_gap("arms",slice.start))?;
        self.arms.get(slice.start..end).map(|x|x.to_vec()).ok_or(source_gap("arms",slice.start))
    }
    fn pattern(&self,id:usize)->Result<a::Pattern> { Ok(match *self.admission.view.patterns.get(id).ok_or(source_gap("pattern",id))? {
        PatternRowC::PVar{x}=>a::Pattern::Var(self.name(x)?), PatternRowC::PNil=>a::Pattern::Nil,
        PatternRowC::PCons{x,y}=>a::Pattern::Cons(self.name(x)?,self.name(y)?), PatternRowC::PLeaf{x}=>a::Pattern::Leaf(self.name(x)?),
        PatternRowC::PBranch{x,y}=>a::Pattern::Branch(self.name(x)?,self.name(y)?),
        PatternRowC::Wildcard=>return Err(generic(GenericGap::WildcardPattern{row:id})),
        PatternRowC::Constructor{type_id,ctor_id,..}=>return Err(generic(GenericGap::ConstructorPattern{row:id,type_id,ctor_id})),
    }) }
    fn frame(&self,p:&DecodedPayload)->Result<a::Frame> {
        let InputValue::Frame(raw)=&p.raw else { return Err(ProjectionGap::WrongPayload("frame")); };
        let get=|n|field(&p.fields,n);
        Ok(match *raw {
            FrameRowC::LetF{binder,body,..}=>a::Frame::Let(self.name(binder)?,self.expr(body)?,environment(get("saved_env")?)?),
            FrameRowC::LeftF{op,right,..}=>a::Frame::Left(bin(op),self.expr(right)?,environment(get("saved_env")?)?),
            FrameRowC::RightF{op,..}=>a::Frame::Right(bin(op),value(get("left")?)?),
            FrameRowC::UnaryF{op,..}=>a::Frame::Unary(unary(op)),
            FrameRowC::IfF{then_expr,else_expr,..}=>a::Frame::If(self.expr(then_expr)?,self.expr(else_expr)?,environment(get("saved_env")?)?),
            FrameRowC::MatchF{arms,..}=>a::Frame::Match(self.arms(arms)?,environment(get("saved_env")?)?),
            FrameRowC::ArgsF{function_name,rest,..}=>a::Frame::Args(self.name(function_name)?,values(get("completed")?)?,self.exprs(rest)?,environment(get("saved_env")?)?),
            FrameRowC::ReturnF{return_type,..}=>a::Frame::Return(ty(return_type)?,environment(get("saved_env")?)?),
            FrameRowC::ScopeF{..}=>a::Frame::Scope(environment(get("saved_env")?)?),
            FrameRowC::ConstructorFieldsF{type_id,ctor_id,index,..}=>return Err(generic(GenericGap::ConstructorFieldsFrame{type_id,ctor_id,index})),
        })
    }
    fn control(&self,p:&DecodedPayload)->Result<a::Control> {
        let InputValue::Control(raw)=&p.raw else { return Err(ProjectionGap::WrongPayload("control")); };
        let get=|n|field(&p.fields,n);
        Ok(match *raw {
            ControlC::Eval{expr}=>a::Control::Eval(self.expr(expr)?),
            ControlC::Return{..}=>a::Control::Return(value(get("value")?)?),
            ControlC::ScheduleNext{next,..}=>a::Control::Push(self.frame(payload(get("frame")?,"frame")?)?,Box::new(a::Control::Eval(self.expr(next)?)),environment(get("env")?)?),
            ControlC::Invoke{function_name,..}=>a::Control::Invoke(self.name(function_name)?,values(get("args")?)?),
            ControlC::Enter{function,..}=>a::Control::Enter(function.map(|i|self.program.functions.get(i).cloned().ok_or(source_gap("function",i))).transpose()?,values(get("args")?)?),
            ControlC::Bound{binding,return_type,body}=>a::Control::Bound(match binding {BoundBinding::Bad=>None,BoundBinding::Ok{..}=>Some(environment(get("binding")?)?)},ty(return_type)?,self.expr(body)?),
            ControlC::Cases{arms,..}=>a::Control::Cases(value(get("value")?)?,self.arms(arms)?,environment(get("saved_env")?)?),
            ControlC::CaseResult{matching,body,rest,..}=>a::Control::CaseResult(match matching {CaseMatching::NoMatch=>None,CaseMatching::Matched{..}=>Some(environment(get("matching")?)?)},value(get("value")?)?,self.expr(body)?,self.arms(rest)?,environment(get("saved_env")?)?),
            ControlC::PrimResult{result}=>a::Control::PrimitiveResult(match result {PrimOutcome::Value{..}=>Ok(value(get("value")?)?),PrimOutcome::Error{code}=>Err(code)}),
            ControlC::Error{code}=>a::Control::Error(code),
            ControlC::Materialize{type_id,ctor_id,fields}=>return Err(generic(GenericGap::MaterializeControl{type_id,ctor_id,arity:fields.len})),
        })
    }
    fn snapshot(&self,raw:&HostSnapshot,decoded:&DecodedSnapshot,initial:u128,remaining:u128)->Result<a::Snapshot> {
        if raw.admission!=self.admission || decoded.metadata.admission!=self.admission { return Err(ProjectionGap::AdmissionChanged); }
        if remaining>initial { return Err(ProjectionGap::ReferenceFuel); }
        let outcome=match raw.image.outcome {
            OutcomeC::Running=>a::Outcome::Pending,
            OutcomeC::Halted{..}=>a::Outcome::Done(value(field(&payload(field(&decoded.fields,"outcome")?,"outcome")?.fields,"value")?)?),
            OutcomeC::Rejected{code}=>a::Outcome::Rejected(code),
            OutcomeC::Exhausted{reason:ExhaustReason::Fuel} if remaining==0=>a::Outcome::Exhausted,
            OutcomeC::Exhausted{..}=>return Err(ProjectionGap::BResourceOutcome),
        };
        let control=if outcome==a::Outcome::Pending {Some(self.control(payload(field(&decoded.fields,"control")?,"control")?)?)} else {None};
        let frames=sequence(field(&decoded.fields,"frames")?,"frames")?.iter().map(|x|self.frame(payload(x,"frame")?)).collect::<Result<Vec<_>>>()?;
        Ok(a::Snapshot{program:self.program.clone(),control,environment:environment(field(&decoded.fields,"current_environment")?)?,frames,stack_limit:raw.image.stack_limit,remaining_fuel:remaining,initial_fuel:initial,outcome})
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReferenceBoundary {
    Complete { reference: a::Snapshot, raw: HostSnapshot, decoded: DecodedSnapshot },
    /// Complete phase/result/task payloads remain present, not coerced to A.
    PendingHelper { raw: HostSnapshot, decoded: DecodedSnapshot },
}
pub fn project_reference_boundary(source:&ReferenceSource,raw:&HostSnapshot,decoded:&DecodedSnapshot,
    reference_initial_fuel:u128,reference_remaining_fuel:u128)->Result<ReferenceBoundary> {
    if raw.image.helper.is_some() || raw.image.result.is_some() || !raw.image.tasks.is_empty() {
        return Ok(ReferenceBoundary::PendingHelper{raw:raw.clone(),decoded:decoded.clone()});
    }
    Ok(ReferenceBoundary::Complete{reference:source.snapshot(raw,decoded,reference_initial_fuel,reference_remaining_fuel)?,raw:raw.clone(),decoded:decoded.clone()})
}

/// Explicit A39 rule/charge table from the fixed public reference source. No
/// B helper/physical work counter is converted to A fuel by this table.
fn reference_rule(rule:RuleIdC)->Option<(usize,u8)> {
    use RuleIdC::*;
    Some(match rule {
        KMachineEvalLiteralValid=>(102,1),KMachineEvalVariablePresent=>(108,1),KMachineEvalVariableAbsent=>(111,1),
        KMachineEvalLet=>(114,1),KMachineEvalBinary=>(117,1),KMachineEvalUnary=>(120,1),KMachineEvalIf=>(123,1),KMachineEvalMatch=>(126,1),KMachineEvalCallEmpty=>(129,1),KMachineEvalCallArguments=>(131,1),
        KMachinePushAccept=>(135,1),KMachinePushRefuse=>(140,1),KMachineReturnLet=>(145,1),KMachineReturnLeft=>(149,1),KMachineReturnRight=>(153,1),KMachineReturnUnary=>(157,1),
        KMachineReturnIfTrue=>(161,1),KMachineReturnIfFalse=>(165,1),KMachineReturnIfInvalid=>(169,1),KMachineReturnMatch=>(173,1),KMachineReturnArgumentsNext=>(177,1),KMachineReturnArgumentsFinal=>(181,1),
        KMachineReturnCallValid=>(185,1),KMachineReturnCallInvalid=>(190,1),KMachineReturnScope=>(195,1),KMachineInvokeFind=>(200,1),KMachineEnterMissing=>(202,1),KMachineEnterBind=>(204,1),
        KMachineBoundInvalid=>(206,1),KMachineBoundValid=>(208,1),KMachineCasesEmpty=>(212,1),KMachineCasesTry=>(214,1),KMachineCasesMiss=>(217,1),KMachineCasesHit=>(220,1),KMachinePrimitiveValue=>(223,1),KMachinePrimitiveError=>(225,1),
        KAdminPublishValue=>(80,0),KAdminPublishError=>(82,0),KAdminPublishExhausted=>(84,0),_=>return None,
    })
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceSpan {
    pub b_start: usize, pub b_end_exclusive: usize, pub source_event: usize,
    /// All actual raw micro-records, including helper/bookkeeping intermediates.
    pub raw: Vec<CompleteStepRecord>, pub reference: a::StepRecord,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionProgress {
    Pending { raw: CompleteStepRecord },
    Complete(ReferenceSpan),
}
struct PendingCompletion { line:usize,charge:u8,source_event:usize }
/// Stream accumulator over SAVED owner records. It never steps either machine.
pub struct ReferencePrefixProjection {
    source: ReferenceSource, initial:a::Snapshot, returned:a::Snapshot,
    raw_returned:HostSnapshot, records:Vec<a::StepRecord>, next_b:usize,
    interval:Vec<CompleteStepRecord>, interval_start:usize,
    completion:Option<PendingCompletion>,
}
impl ReferencePrefixProjection {
    pub fn new(program:Arc<a::Program>,initial:&HostSnapshot,reference_fuel:u128)->Result<Self> {
        let source=ReferenceSource::new(program,initial)?;
        let materialized=wp::materialize(initial).map_err(ProjectionGap::Witness)?;
        let ReferenceBoundary::Complete{reference,..}=project_reference_boundary(&source,initial,&materialized.evaluation.decoded,reference_fuel,reference_fuel)? else {return Err(ProjectionGap::IncompleteInterval);};
        Ok(Self{source,initial:reference.clone(),returned:reference,raw_returned:initial.clone(),records:Vec::new(),next_b:0,interval:Vec::new(),interval_start:0,completion:None})
    }
    pub fn initial(&self)->&a::Snapshot{&self.initial}
    pub fn records(&self)->&[a::StepRecord]{&self.records}
    pub fn returned(&self)->&a::Snapshot{&self.returned}
    pub fn pending_records(&self)->&[CompleteStepRecord]{&self.interval}
    pub fn push(&mut self,record:&CompleteStepRecord)->Result<ProjectionProgress> {
        if record.before!=self.raw_returned {return Err(ProjectionGap::Discontinuity);}
        if record.before.admission!=self.source.admission || record.after.admission!=self.source.admission{return Err(ProjectionGap::AdmissionChanged);}
        let index=self.next_b;self.next_b=self.next_b.checked_add(1).ok_or(ProjectionGap::IncompleteInterval)?;
        self.raw_returned=record.after.clone();self.interval.push(record.clone());
        if matches!(record.event.event_kind,EventKind::LegacyComplete|EventKind::Publish) {
            let (line,charge)=reference_rule(record.event.rule_id).ok_or(ProjectionGap::UnmappedRule(record.event.rule_id))?;
            if self.completion.is_some(){return Err(ProjectionGap::IncompleteInterval);}
            if (charge==1 && record.event.disposition!=StepDisposition::Applied) || (charge==0 && !matches!(record.event.disposition,StepDisposition::Published|StepDisposition::ExhaustedFuelUncharged)) {return Err(ProjectionGap::UnappliedSourceRule);}
            self.completion=Some(PendingCompletion{line,charge,source_event:index});
        }
        let Some(completion)=&self.completion else {return Ok(ProjectionProgress::Pending{raw:record.clone()});};
        let remaining=self.returned.remaining_fuel.checked_sub(completion.charge as u128).ok_or(ProjectionGap::ReferenceFuel)?;
        let materialized=wp::materialize(&record.after).map_err(ProjectionGap::Witness)?;
        let ReferenceBoundary::Complete{reference,..}=project_reference_boundary(&self.source,&record.after,&materialized.evaluation.decoded,self.initial.initial_fuel,remaining)? else {return Ok(ProjectionProgress::Pending{raw:record.clone()});};
        let first=self.interval.first().ok_or(ProjectionGap::IncompleteInterval)?;
        let first_materialized=wp::materialize(&first.before).map_err(ProjectionGap::Witness)?;
        let ReferenceBoundary::Complete{reference:before,..}=project_reference_boundary(&self.source,&first.before,&first_materialized.evaluation.decoded,self.initial.initial_fuel,self.returned.remaining_fuel)? else {return Err(ProjectionGap::IncompleteInterval);};
        if before!=self.returned{return Err(ProjectionGap::Discontinuity);}
        let completed=a::StepRecord{rule_start_line:completion.line,charge:completion.charge,before,after:reference.clone()};
        let span=ReferenceSpan{b_start:self.interval_start,b_end_exclusive:self.next_b,source_event:completion.source_event,raw:std::mem::take(&mut self.interval),reference:completed.clone()};
        self.returned=reference;self.records.push(completed);self.completion=None;self.interval_start=self.next_b;
        Ok(ProjectionProgress::Complete(span))
    }
    /// A trailing helper interval is retained and cannot masquerade as a full
    /// reference prefix's returned boundary. Pending A calls at complete logical
    /// boundaries are allowed; terminality is a separate observed property.
    pub fn finish(&self)->Result<&a::Snapshot>{
        if !self.interval.is_empty() || self.completion.is_some(){Err(ProjectionGap::IncompleteInterval)}else{Ok(&self.returned)}
    }
}
