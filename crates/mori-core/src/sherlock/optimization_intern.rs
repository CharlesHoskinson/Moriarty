//! UIREQ-024 candidate optimization: intern equal `Tree` nodes (hash-consing).
//!
//! CANDIDATE ONLY. Nothing in the reference machine calls this module. The
//! candidate replaces the physical representation of `sherlock-k-a/1` tree
//! values: structurally equal nodes share one allocation, both in program
//! literals (static) and in every boundary state the machine resumes from
//! (dynamic, via [`Machine::from_snapshot`]). [`check_preservation`] is the
//! executable preservation predicate over the declared observations: event
//! order, per-event charges and fuel/depth, complete boundary states, returned
//! values and rejection/exhaustion outcomes. The A profile carries no effect or
//! duty state, so effect/duty preservation is vacuous here and is reported as
//! such, never as evidence for an effectful profile.
//!
//! Physical identity is not source equality: snapshots compare structurally, so
//! a candidate that merges unequal nodes is caught as a divergence.
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use super::{Arm, Control, Environment, Expr, ExprRef, Frame, Machine, Outcome, Program};
use super::{Snapshot, StepRecord, Tree, Value};

pub const CANDIDATE_ID: &str = "sherlock-a-intern-equal-tree-nodes/1";
pub const REFERENCE_PROFILE: &str = super::PROFILE;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NodeKey {
    Leaf(u128),
    /// Children are already interned, so their addresses identify them.
    Branch(usize, usize),
}

/// Hash-consing table: one shared allocation per structurally distinct node.
#[derive(Default)]
pub struct TreeInterner {
    nodes: HashMap<NodeKey, Arc<Tree>>,
}

impl TreeInterner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of distinct nodes the table holds.
    pub fn distinct_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Intern `tree` bottom-up without host recursion.
    pub fn intern_tree(&mut self, tree: &Arc<Tree>) -> Arc<Tree> {
        // RED: no sharing yet; the table stays empty.
        let _ = (&mut self.nodes, NodeKey::Leaf(0), NodeKey::Branch(0, 0));
        tree.clone()
    }

    pub fn intern_value(&mut self, value: &Value) -> Value {
        match value {
            Value::Tree(tree) => {
                let root = self.intern_tree(&Arc::new(tree.clone()));
                Value::Tree((*root).clone())
            }
            other => other.clone(),
        }
    }

    fn intern_env(&mut self, env: &Environment) -> Environment {
        env.iter()
            .map(|(k, v)| (k.clone(), self.intern_value(v)))
            .collect()
    }

    fn intern_values(&mut self, values: &[Value]) -> Vec<Value> {
        values.iter().map(|v| self.intern_value(v)).collect()
    }

    /// Intern the literals of an expression (static code facts only).
    pub fn intern_expr(&mut self, expr: &ExprRef) -> ExprRef {
        let e = match &**expr {
            Expr::Literal(v) => Expr::Literal(self.intern_value(v)),
            Expr::Var(n) => Expr::Var(n.clone()),
            Expr::Let(n, a, b) => Expr::Let(n.clone(), self.intern_expr(a), self.intern_expr(b)),
            Expr::Binary(op, a, b) => Expr::Binary(*op, self.intern_expr(a), self.intern_expr(b)),
            Expr::Unary(op, a) => Expr::Unary(*op, self.intern_expr(a)),
            Expr::If(c, t, f) => Expr::If(
                self.intern_expr(c),
                self.intern_expr(t),
                self.intern_expr(f),
            ),
            Expr::Call(n, args) => Expr::Call(
                n.clone(),
                args.iter().map(|a| self.intern_expr(a)).collect(),
            ),
            Expr::Match(s, arms) => Expr::Match(self.intern_expr(s), self.intern_arms(arms)),
        };
        Arc::new(e)
    }

    fn intern_arms(&mut self, arms: &[Arm]) -> Vec<Arm> {
        arms.iter()
            .map(|a| Arm {
                pattern: a.pattern.clone(),
                body: self.intern_expr(&a.body),
            })
            .collect()
    }

    /// Intern every literal in function bodies and the entry, keeping function order.
    pub fn intern_program(&mut self, program: &Program) -> Program {
        let mut functions = program.functions.clone();
        for f in &mut functions {
            f.body = self.intern_expr(&f.body);
        }
        Program {
            functions,
            entry: self.intern_expr(&program.entry),
        }
    }

    fn intern_frame(&mut self, frame: &Frame) -> Frame {
        match frame {
            Frame::Let(n, e, env) => Frame::Let(n.clone(), e.clone(), self.intern_env(env)),
            Frame::Left(op, e, env) => Frame::Left(*op, e.clone(), self.intern_env(env)),
            Frame::Right(op, v) => Frame::Right(*op, self.intern_value(v)),
            Frame::Unary(op) => Frame::Unary(*op),
            Frame::If(t, f, env) => Frame::If(t.clone(), f.clone(), self.intern_env(env)),
            Frame::Match(arms, env) => Frame::Match(arms.clone(), self.intern_env(env)),
            Frame::Args(n, done, rest, env) => Frame::Args(
                n.clone(),
                self.intern_values(done),
                rest.clone(),
                self.intern_env(env),
            ),
            Frame::Return(ty, env) => Frame::Return(*ty, self.intern_env(env)),
            Frame::Scope(env) => Frame::Scope(self.intern_env(env)),
        }
    }

    fn intern_control(&mut self, control: &Control) -> Control {
        match control {
            Control::Eval(e) => Control::Eval(e.clone()),
            Control::Return(v) => Control::Return(self.intern_value(v)),
            Control::Push(f, c, env) => Control::Push(
                self.intern_frame(f),
                Box::new(self.intern_control(c)),
                self.intern_env(env),
            ),
            Control::Invoke(n, vs) => Control::Invoke(n.clone(), self.intern_values(vs)),
            Control::Enter(f, vs) => Control::Enter(f.clone(), self.intern_values(vs)),
            Control::Bound(env, ty, e) => {
                Control::Bound(env.as_ref().map(|x| self.intern_env(x)), *ty, e.clone())
            }
            Control::Cases(v, arms, env) => {
                Control::Cases(self.intern_value(v), arms.clone(), self.intern_env(env))
            }
            Control::CaseResult(env, v, e, arms, outer) => Control::CaseResult(
                env.as_ref().map(|x| self.intern_env(x)),
                self.intern_value(v),
                e.clone(),
                arms.clone(),
                self.intern_env(outer),
            ),
            Control::PrimitiveResult(r) => {
                Control::PrimitiveResult(r.as_ref().map(|v| self.intern_value(v)).map_err(|c| *c))
            }
            Control::Error(c) => Control::Error(*c),
        }
    }

    /// Intern every runtime value of a complete boundary state. Code, fuel,
    /// depth, stack limit and outcome class are carried unchanged.
    pub fn intern_snapshot(&mut self, s: &Snapshot) -> Snapshot {
        Snapshot {
            program: s.program.clone(),
            control: s.control.as_ref().map(|c| self.intern_control(c)),
            environment: self.intern_env(&s.environment),
            frames: s.frames.iter().map(|f| self.intern_frame(f)).collect(),
            stack_limit: s.stack_limit,
            remaining_fuel: s.remaining_fuel,
            initial_fuel: s.initial_fuel,
            outcome: match &s.outcome {
                Outcome::Done(v) => Outcome::Done(self.intern_value(v)),
                o => o.clone(),
            },
        }
    }
}

/// Distinct physical tree allocations reachable from `value` (by address).
pub fn physical_tree_nodes(value: &Value) -> usize {
    let Value::Tree(tree) = value else { return 0 };
    let mut seen: BTreeSet<usize> = BTreeSet::new();
    let mut work: Vec<&Tree> = vec![tree];
    let mut count = 1;
    while let Some(t) = work.pop() {
        if let Tree::Branch(l, r) = t {
            for c in [l, r] {
                if seen.insert(Arc::as_ptr(c) as usize) {
                    count += 1;
                    work.push(c);
                }
            }
        }
    }
    count
}

/// Structural tree node occurrences of `value` (sharing unfolded).
pub fn structural_tree_nodes(value: &Value) -> usize {
    let Value::Tree(tree) = value else { return 0 };
    let mut work: Vec<&Tree> = vec![tree];
    let mut count = 0;
    while let Some(t) = work.pop() {
        count += 1;
        if let Tree::Branch(l, r) = t {
            work.push(l);
            work.push(r);
        }
    }
    count
}

/// A complete actual run: initial state, every record, final state.
#[derive(Clone, Debug)]
pub struct Observation {
    pub initial: Snapshot,
    pub records: Vec<StepRecord>,
    pub final_state: Snapshot,
}

/// Reference realization: the unchanged machine on the unchanged program.
pub fn observe_reference(
    program: Arc<Program>,
    fuel: u128,
    stack: usize,
    max_events: usize,
) -> Observation {
    let mut m = Machine::new(program, fuel, stack);
    let initial = m.snapshot();
    let records = m.run_steps(max_events);
    Observation {
        initial,
        records,
        final_state: m.snapshot(),
    }
}

/// Candidate realization: interned program literals, and every boundary state
/// re-interned before the machine resumes from it.
pub fn observe_candidate(
    program: &Program,
    fuel: u128,
    stack: usize,
    max_events: usize,
    interner: &mut TreeInterner,
) -> Observation {
    let program = Arc::new(interner.intern_program(program));
    let mut m = Machine::new(program, fuel, stack);
    let initial = m.snapshot();
    let mut records = Vec::new();
    while records.len() < max_events {
        let Some(r) = m.step() else { break };
        let terminal = r.after.outcome != Outcome::Pending;
        records.push(r);
        if terminal {
            break;
        }
        m = Machine::from_snapshot(interner.intern_snapshot(&m.snapshot()));
    }
    Observation {
        initial,
        records,
        final_state: m.snapshot(),
    }
}

/// The six declared observation planes of the UIREQ-024 record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clause {
    Values,
    RejectionExhaustion,
    EvaluationEffectOrder,
    Boundaries,
    Duties,
    ResourceSchedule,
}

impl Clause {
    pub const ALL: [Clause; 6] = [
        Clause::Values,
        Clause::RejectionExhaustion,
        Clause::EvaluationEffectOrder,
        Clause::Boundaries,
        Clause::Duties,
        Clause::ResourceSchedule,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Clause::Values => "values",
            Clause::RejectionExhaustion => "rejection/exhaustion",
            Clause::EvaluationEffectOrder => "evaluation/effect order",
            Clause::Boundaries => "boundaries",
            Clause::Duties => "duties",
            Clause::ResourceSchedule => "resource schedule",
        }
    }
}

/// First place the candidate's observation differs from the reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Divergence {
    pub clause: Clause,
    /// Event index, or `None` for the initial/final state.
    pub event: Option<usize>,
    pub detail: String,
}

/// What a passing comparison covered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preserved {
    pub events: usize,
    pub charged: u128,
    pub outcome: Outcome,
    /// Always true for `sherlock-k-a/1`: no effect/duty state exists to compare.
    pub duties_vacuous: bool,
}

fn outcome_class(o: &Outcome) -> String {
    match o {
        Outcome::Pending => "Pending".into(),
        Outcome::Done(_) => "Done".into(),
        Outcome::Rejected(c) => format!("Rejected({c})"),
        Outcome::Exhausted => "Exhausted".into(),
    }
}

fn state_clause(a: &Snapshot, b: &Snapshot) -> Option<(Clause, String)> {
    if outcome_class(&a.outcome) != outcome_class(&b.outcome) {
        return Some((
            Clause::RejectionExhaustion,
            format!(
                "outcome {} != {}",
                outcome_class(&a.outcome),
                outcome_class(&b.outcome)
            ),
        ));
    }
    if a.outcome != b.outcome {
        return Some((Clause::Values, "returned value differs".into()));
    }
    if a.remaining_fuel != b.remaining_fuel
        || a.initial_fuel != b.initial_fuel
        || a.depth() != b.depth()
        || a.stack_limit != b.stack_limit
    {
        return Some((
            Clause::ResourceSchedule,
            format!(
                "fuel/depth/limit {}/{}/{} != {}/{}/{}",
                a.remaining_fuel,
                a.depth(),
                a.stack_limit,
                b.remaining_fuel,
                b.depth(),
                b.stack_limit
            ),
        ));
    }
    if a != b {
        return Some((
            Clause::Boundaries,
            "complete boundary state (control/environment/frames/code) differs".into(),
        ));
    }
    None
}

/// The executable preservation predicate: every declared observation equal.
pub fn check_preservation(
    reference: &Observation,
    candidate: &Observation,
) -> Result<Preserved, Divergence> {
    if let Some((clause, detail)) = state_clause(&reference.initial, &candidate.initial) {
        return Err(Divergence {
            clause,
            event: None,
            detail: format!("initial: {detail}"),
        });
    }
    let n = reference.records.len().max(candidate.records.len());
    for i in 0..n {
        let (Some(a), Some(b)) = (reference.records.get(i), candidate.records.get(i)) else {
            return Err(Divergence {
                clause: Clause::EvaluationEffectOrder,
                event: Some(i),
                detail: format!(
                    "event count {} != {}",
                    reference.records.len(),
                    candidate.records.len()
                ),
            });
        };
        if a.rule_start_line != b.rule_start_line {
            return Err(Divergence {
                clause: Clause::EvaluationEffectOrder,
                event: Some(i),
                detail: format!("rule {} != {}", a.rule_start_line, b.rule_start_line),
            });
        }
        if a.charge != b.charge {
            return Err(Divergence {
                clause: Clause::ResourceSchedule,
                event: Some(i),
                detail: format!("charge {} != {}", a.charge, b.charge),
            });
        }
        for (side, x, y) in [
            ("before", &a.before, &b.before),
            ("after", &a.after, &b.after),
        ] {
            if let Some((clause, detail)) = state_clause(x, y) {
                return Err(Divergence {
                    clause,
                    event: Some(i),
                    detail: format!("{side}: {detail}"),
                });
            }
        }
    }
    if let Some((clause, detail)) = state_clause(&reference.final_state, &candidate.final_state) {
        return Err(Divergence {
            clause,
            event: None,
            detail: format!("final: {detail}"),
        });
    }
    Ok(Preserved {
        events: reference.records.len(),
        charged: reference.final_state.charged_steps(),
        outcome: reference.final_state.outcome.clone(),
        duties_vacuous: true,
    })
}
