//! UIREQ-024 frame/sharing optimization: the intern-equal-tree-nodes candidate
//! against the unchanged `sherlock-k-a/1` reference machine.
//!
//! Positive: under normal, fuel-exhaustion and frame-cap rejection budgets the
//! candidate run (interned literals, every boundary state re-interned before
//! resume) preserves event order, per-event charge/fuel/depth, every complete
//! boundary state, the returned value and the outcome class, while physically
//! sharing equal nodes. Rejection: an unsound candidate that merges unequal
//! nodes is caught by the preservation predicate.
//! Expected literals (fold sums, node counts, outcomes) are derived by hand from
//! the source program, not from either run.

use mori_core::sherlock::optimization_intern::{
    Clause, TreeInterner, check_preservation, observe_candidate, observe_reference,
    physical_tree_nodes, structural_tree_nodes,
};
use mori_core::sherlock::{
    Arm, BinaryOp, Expr, ExprRef, Function, Outcome, Parameter, Pattern, Program, Tree, Type, Value,
};
use std::sync::Arc;

fn var(name: &str) -> ExprRef {
    Arc::new(Expr::Var(name.into()))
}
fn call(arg: ExprRef) -> ExprRef {
    Arc::new(Expr::Call("10".into(), vec![arg]))
}
fn leaf(n: u128) -> Arc<Tree> {
    Arc::new(Tree::Leaf(n))
}
fn branch(l: Arc<Tree>, r: Arc<Tree>) -> Arc<Tree> {
    Arc::new(Tree::Branch(l, r))
}

/// fold(t) = match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) }
fn fold_program(root: &Arc<Tree>) -> Program {
    let arms = vec![
        Arm {
            pattern: Pattern::Leaf("12".into()),
            body: var("12"),
        },
        Arm {
            pattern: Pattern::Branch("13".into(), "14".into()),
            body: Arc::new(Expr::Binary(
                BinaryOp::Add,
                call(var("13")),
                call(var("14")),
            )),
        },
    ];
    Program {
        functions: vec![Function {
            name: "10".into(),
            parameters: vec![Parameter {
                name: "11".into(),
                ty: Type::Tree,
            }],
            return_type: Type::UInt,
            body: Arc::new(Expr::Match(var("11"), arms)),
        }],
        entry: call(Arc::new(Expr::Literal(Value::Tree((**root).clone())))),
    }
}

/// branch(branch(leaf 4, leaf 5), branch(leaf 4, leaf 5)), every node separate.
fn separate_tree() -> Arc<Tree> {
    branch(branch(leaf(4), leaf(5)), branch(leaf(4), leaf(5)))
}

fn compare(fuel: u128, stack: usize) -> (Outcome, usize) {
    let program = fold_program(&separate_tree());
    let reference = observe_reference(Arc::new(program.clone()), fuel, stack, 4096);
    let mut interner = TreeInterner::new();
    let candidate = observe_candidate(&program, fuel, stack, 4096, &mut interner);
    let kept = check_preservation(&reference, &candidate)
        .unwrap_or_else(|d| panic!("candidate diverged: {d:?}"));
    assert!(kept.duties_vacuous, "A profile has no effect/duty state");
    assert_eq!(kept.events, reference.records.len());
    // the candidate really shares: 4 leaves + 2 inner + root structural, 4 physical
    let Some(Expr::Call(_, args)) = Some(&*candidate.initial.program.entry) else {
        unreachable!()
    };
    let Expr::Literal(lit) = &*args[0] else {
        panic!("entry literal")
    };
    assert_eq!(structural_tree_nodes(lit), 7);
    assert_eq!(physical_tree_nodes(lit), 4);
    (kept.outcome, kept.events)
}

#[test]
fn interner_shares_equal_nodes_and_keeps_structure() {
    let separate = Value::Tree((*separate_tree()).clone());
    assert_eq!(structural_tree_nodes(&separate), 7);
    assert_eq!(physical_tree_nodes(&separate), 7);
    let mut interner = TreeInterner::new();
    let interned = interner.intern_value(&separate);
    assert_eq!(
        interned, separate,
        "interning preserves structural equality"
    );
    assert_eq!(structural_tree_nodes(&interned), 7);
    // root + branch(4,5) + leaf 4 + leaf 5
    assert_eq!(physical_tree_nodes(&interned), 4);
    assert_eq!(interner.distinct_nodes(), 4);
    let Value::Tree(Tree::Branch(l, r)) = &interned else {
        panic!("branch")
    };
    assert!(Arc::ptr_eq(l, r), "equal subtrees share one allocation");
}

#[test]
fn intern_preserves_normal_run() {
    let (outcome, events) = compare(512, 32);
    assert_eq!(outcome, Outcome::Done(Value::UInt(18)), "4+5+4+5");
    assert!(events > 0);
}

#[test]
fn intern_preserves_fuel_exhaustion() {
    let (outcome, _) = compare(20, 32);
    assert_eq!(outcome, Outcome::Exhausted);
}

#[test]
fn intern_preserves_frame_cap_rejection() {
    let (outcome, _) = compare(512, 2);
    assert_eq!(outcome, Outcome::Rejected(4), "K-A frame-cap error4");
}

#[test]
fn preservation_rejects_unsound_interning() {
    // An unsound candidate keyed only by shape merges leaf 5 into leaf 4.
    let program = fold_program(&separate_tree());
    let reference = observe_reference(Arc::new(program.clone()), 512, 32, 4096);
    let four = leaf(4);
    let merged = branch(four.clone(), four);
    let mutated = fold_program(&branch(merged.clone(), merged));
    let mut interner = TreeInterner::new();
    let candidate = observe_candidate(&mutated, 512, 32, 4096, &mut interner);
    let d = check_preservation(&reference, &candidate)
        .expect_err("merging unequal nodes must be rejected");
    assert_eq!(
        d.event, None,
        "first difference is the initial code literal"
    );
    assert_eq!(d.clause, Clause::Boundaries);
}

#[test]
fn preservation_rejects_changed_budget() {
    // A candidate that silently changes the resource schedule is rejected.
    let program = fold_program(&separate_tree());
    let reference = observe_reference(Arc::new(program.clone()), 512, 32, 4096);
    let mut interner = TreeInterner::new();
    let candidate = observe_candidate(&program, 511, 32, 4096, &mut interner);
    let d = check_preservation(&reference, &candidate).expect_err("fuel change must be rejected");
    assert_eq!(d.clause, Clause::ResourceSchedule);
}
