//! Ordinary borrowed name input over the existing ExtProgram source AST.
//! Keeps declaration/function/pattern traversal order and emits original borrows.
//! Checked insertion and Fin own-name projection are caller callbacks: this leaf
//! adds no source-shape check, interner, capacity, NameId or refusal policy.
//! The designated owner supplies projection/check placement and final binding.
//! R-only source preparation, uncompiled/unrun; no admission/runtime claim.

use super::owner_ext_source_program::ExtProgram;
use super::owner_ext_t3_uses::{ExtExpr, ExtFunction, ExtPattern};
use super::CorePattern;

enum NameAct<'s> {
    Function(&'s ExtFunction),
    Expr(&'s ExtExpr),
    Pattern(&'s ExtPattern),
}

/// Feed entry, then each function's name/parameters/body in table order.
/// Traversal uses the historical heap worklist. The callback owns name interning,
/// capacities and final binding; its error propagates at the first visited name.
/// The owner supplies its own-name projection at each node before its children.
pub fn for_each_ext_entry_and_function_name<'s, E>(
    functions: &'s [ExtFunction],
    entry: &'s ExtExpr,
    visit: &mut impl FnMut(&'s str) -> Result<(), E>,
    project_fin_name: &mut impl FnMut(&'s ExtExpr) -> Result<Option<&'s str>, E>,
) -> Result<(), E> {
    let mut stack = Vec::new();
    for function in functions.iter().rev() {
        stack.push(NameAct::Function(function));
    }
    stack.push(NameAct::Expr(entry));

    while let Some(act) = stack.pop() {
        match act {
            NameAct::Function(function) => {
                visit(function.name.as_str())?;
                for param in &function.parameters {
                    visit(param.name.as_str())?;
                }
                stack.push(NameAct::Expr(&function.body));
            }
            NameAct::Pattern(pattern) => match pattern {
                ExtPattern::Dedicated { pattern } => match pattern {
                    CorePattern::PVar { x } | CorePattern::PLeaf { x } => {
                        visit(x)?;
                    }
                    CorePattern::PCons { x, y } | CorePattern::PBranch { x, y } => {
                        visit(x)?;
                        visit(y)?;
                    }
                    CorePattern::PNil => {}
                },
                ExtPattern::Wildcard => {}
                ExtPattern::Ctor { fields, .. } => {
                    // Type and constructor texts are resolved by other passes.
                    // Binder names in child patterns retain pattern preorder.
                    for field in fields.iter().rev() {
                        stack.push(NameAct::Pattern(field));
                    }
                }
            },
            NameAct::Expr(expr) => {
                if let Some(name) = project_fin_name(expr)? {
                    visit(name)?;
                }
                match expr {
                // ExtValue and CoreValue have no Pass B names at any depth.
                // Their constructor/type texts are resolved, not interned.
                ExtExpr::Literal { .. } | ExtExpr::StateRead { .. } => {}
                ExtExpr::Var { name } => visit(name)?,
                ExtExpr::LiabilityPresent { name } | ExtExpr::LiabilityAmount { name } => {
                    visit(name)?;
                }
                ExtExpr::Let { name, value, body } => {
                    visit(name)?;
                    stack.push(NameAct::Expr(body));
                    stack.push(NameAct::Expr(value));
                }
                ExtExpr::Binary { left, right, .. } => {
                    stack.push(NameAct::Expr(right));
                    stack.push(NameAct::Expr(left));
                }
                ExtExpr::Unary { operand, .. } => stack.push(NameAct::Expr(operand)),
                ExtExpr::If { cond, then_expr, else_expr } => {
                    stack.push(NameAct::Expr(else_expr));
                    stack.push(NameAct::Expr(then_expr));
                    stack.push(NameAct::Expr(cond));
                }
                ExtExpr::Call { name, args } => {
                    visit(name)?;
                    for arg in args.iter().rev() {
                        stack.push(NameAct::Expr(arg));
                    }
                }
                ExtExpr::Match { scrutinee, arms } => {
                    // Literal R4 D-27 NameAct order: all arm patterns, then the
                    // scrutinee, then all arm bodies. This is a B-name walk;
                    // it does not select the pending T3 PD-1 interpretation.
                    for arm in arms.iter().rev() {
                        stack.push(NameAct::Expr(&arm.body));
                    }
                    stack.push(NameAct::Expr(scrutinee));
                    for arm in arms.iter().rev() {
                        stack.push(NameAct::Pattern(&arm.pattern));
                    }
                }
                ExtExpr::Construct { fields, .. } => {
                    for field in fields.iter().rev() {
                        stack.push(NameAct::Expr(field));
                    }
                }
                ExtExpr::Fin { operand, .. } => {
                    // Own names were supplied by the owner projection before this match.
                    stack.push(NameAct::Expr(operand));
                }
                ExtExpr::Await { label, binders, guard, body } => {
                    visit(label.as_str())?;
                    for binder in binders {
                        visit(binder.name.as_str())?;
                    }
                    stack.push(NameAct::Expr(body));
                    stack.push(NameAct::Expr(guard));
                }
                }
            },
        }
    }
    Ok(())
}

/// Emit declaration names, finance declaration names, then entry/functions in
/// the same pass order. Type parameters, TypeExprSrc names and constructor-use
/// texts are resolved elsewhere and are not additions to this global name pool.
///
/// The future owner preparation body supplies its existing checked insertion
/// callback, sorts/finalizes names after this walk, and binds declaration NameIds
/// against that final table. This helper does not create PassBTypeNameIds.
/// It performs no source-domain selection; the owner runs its checks first.
pub fn for_each_ext_program_name<'s, E>(
    program: &'s ExtProgram,
    visit: &mut impl FnMut(&'s str) -> Result<(), E>,
    project_fin_name: &mut impl FnMut(&'s ExtExpr) -> Result<Option<&'s str>, E>,
) -> Result<(), E> {
    for decl in &program.types {
        visit(decl.name.as_str())?;
        for ctor in &decl.ctors {
            visit(ctor.name.as_str())?;
            for field in &ctor.fields {
                visit(field.name.as_str())?;
            }
        }
    }
    for asset in &program.assets {
        visit(asset.name.as_str())?;
    }
    for state in &program.states {
        visit(state.name.as_str())?;
    }
    for input in &program.inputs {
        visit(input.name.as_str())?;
    }
    for_each_ext_entry_and_function_name(
        &program.functions, &program.entry, visit, project_fin_name,
    )
}
