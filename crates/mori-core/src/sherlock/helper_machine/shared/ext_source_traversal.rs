//! Proposed read-only, per-carrier source traversal helpers.
//!
//! Each helper visits the supplied carrier in local preorder using a heap stack.
//! Callback errors pass through unchanged. These helpers make no feature/domain,
//! refusal, capacity, arithmetic, admission or source-ID decision. Allocation uses
//! ordinary Vec worklists as in the exact owner source.
//!
//! The expression helper visits expression children only. A Literal's value and a
//! Match's patterns remain visible in the complete borrowed expression callback;
//! the owner decides when to call the separate value/pattern/type helpers. There
//! is no global Pass A order or cross-pattern/scrutinee ordering in this module.

use super::owner_ext_t3_uses::{ExtExpr, ExtFunction, ExtPattern, ExtValue};
use super::owner_ext_type_builder::{TypeDeclSrc, TypeExprSrc};

pub fn walk_type_expr_preorder<'s, E, F>(root: &'s TypeExprSrc, mut visit: F) -> Result<(), E>
where
    F: FnMut(&'s TypeExprSrc) -> Result<(), E>,
{
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        visit(node)?;
        if let TypeExprSrc::Apply { args, .. } = node {
            for arg in args.iter().rev() {
                stack.push(arg);
            }
        }
    }
    Ok(())
}

pub fn walk_ext_value_preorder<'s, E, F>(root: &'s ExtValue, mut visit: F) -> Result<(), E>
where
    F: FnMut(&'s ExtValue) -> Result<(), E>,
{
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        visit(node)?;
        if let ExtValue::Constructor { fields, .. } = node {
            for field in fields.iter().rev() {
                stack.push(field);
            }
        }
    }
    Ok(())
}

pub fn walk_ext_pattern_preorder<'s, E, F>(root: &'s ExtPattern, mut visit: F) -> Result<(), E>
where
    F: FnMut(&'s ExtPattern) -> Result<(), E>,
{
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        visit(node)?;
        if let ExtPattern::Ctor { fields, .. } = node {
            for field in fields.iter().rev() {
                stack.push(field);
            }
        }
    }
    Ok(())
}

pub fn walk_ext_expr_preorder<'s, E, F>(root: &'s ExtExpr, mut visit: F) -> Result<(), E>
where
    F: FnMut(&'s ExtExpr) -> Result<(), E>,
{
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        // The full node reaches the callback before any child expression.
        // Fin optional fields, Await binder types and every other source field
        // remain available through the original immutable borrow.
        visit(node)?;
        match node {
            ExtExpr::Literal { .. }
            | ExtExpr::Var { .. }
            | ExtExpr::StateRead { .. }
            | ExtExpr::LiabilityPresent { .. }
            | ExtExpr::LiabilityAmount { .. } => {}
            ExtExpr::Let { value, body, .. } => {
                stack.push(body);
                stack.push(value);
            }
            ExtExpr::Binary { left, right, .. } => {
                stack.push(right);
                stack.push(left);
            }
            ExtExpr::Unary { operand, .. } | ExtExpr::Fin { operand, .. } => {
                stack.push(operand);
            }
            ExtExpr::If {
                cond,
                then_expr,
                else_expr,
            } => {
                stack.push(else_expr);
                stack.push(then_expr);
                stack.push(cond);
            }
            ExtExpr::Call { args, .. } => {
                for arg in args.iter().rev() {
                    stack.push(arg);
                }
            }
            ExtExpr::Match { scrutinee, arms } => {
                for arm in arms.iter().rev() {
                    stack.push(&arm.body);
                }
                stack.push(scrutinee);
            }
            ExtExpr::Construct { fields, .. } => {
                for field in fields.iter().rev() {
                    stack.push(field);
                }
            }
            ExtExpr::Await { guard, body, .. } => {
                stack.push(body);
                stack.push(guard);
            }
        }
    }
    Ok(())
}

/// Enumerate one declaration's field type roots, without traversing those types.
/// The caller retains the declaration/parameter context and chooses its classifier.
pub fn for_each_decl_field_type_root<'s, E, F>(decl: &'s TypeDeclSrc, mut visit: F) -> Result<(), E>
where
    F: FnMut(&'s TypeExprSrc) -> Result<(), E>,
{
    for ctor in &decl.ctors {
        for field in &ctor.fields {
            visit(&field.ty)?;
        }
    }
    Ok(())
}

/// Enumerate one function's parameter type roots, then its return type root.
/// This does not choose when that function occurs in any global feature pass.
pub fn for_each_function_signature_type_root<'s, E, F>(
    function: &'s ExtFunction,
    mut visit: F,
) -> Result<(), E>
where
    F: FnMut(&'s TypeExprSrc) -> Result<(), E>,
{
    for parameter in &function.parameters {
        visit(&parameter.ty)?;
    }
    visit(&function.return_type)?;
    Ok(())
}

/// Visit the real whole program's existing T3 source-use sequence. This ordinary
/// adapter owns no ID, type resolution, error mapping, domain or capacity policy.
/// It forwards the exact walker error or callback error at the first occurrence.
pub fn for_each_ext_program_type_use<'s>(
    program: &'s super::owner_ext_source_program::ExtProgram,
    mut visit: impl FnMut(
        super::owner_ext_t3_uses::T3Use<'s>,
    ) -> Result<(), super::owner_ext_type_builder::ExtTypeBuildError>,
) -> Result<(), super::owner_ext_type_builder::ExtTypeBuildError> {
    let mut walker = super::owner_ext_t3_uses::T3UseWalker::new(&program.functions, &program.entry);
    while let Some(source_use) = walker.next_use()? {
        visit(source_use)?;
    }
    Ok(())
}
