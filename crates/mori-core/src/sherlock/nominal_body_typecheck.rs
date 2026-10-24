//! Additive ordinary body-typing proposal over the actual owner-prepared AST.
//! No evaluator, admission change, budget policy or native/H1 claim.
pub use crate::sherlock::helper_machine::shared::adt::TypeIdC;
use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtExpr, ExtPattern, ExtValue, T3Record, T3UseKind, T3UseWalker,
};
use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
    ExtTypeBuildError, TypeExprSrc,
};
use crate::sherlock::helper_machine::shared::{
    BinaryOp, CorePattern, CoreValue, PreparedExtProgramSource, UnaryOp,
};
pub use crate::sherlock_draft::SourceLocation;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NominalBodyTypeSummary {
    /// Canonical IDs in the borrowed preparation's unchanged type table.
    pub function_returns: Vec<TypeIdC>,
    pub entry_type: TypeIdC,
}

pub struct NominalBodyTypeError {
    /// Expression child indices: Match scrutinee=0, arm bodies=1..; other
    /// children retain source order. After a Literal expression, any remaining
    /// indices identify ExtValue constructor fields, not Expr children or T3
    /// LiteralCtor value ordinals. Pattern errors name the owning arm body.
    pub source: SourceLocation,
    pub kind: NominalBodyTypeErrorKind,
}

pub enum NominalBodyTypeErrorKind {
    UnknownVariable(String),
    UnknownFunction(String),
    Arity {
        name: String,
        expected: usize,
        actual: usize,
    },
    Expected {
        expected: TypeIdC,
        actual: TypeIdC,
    },
    BranchTypes {
        yes: TypeIdC,
        no: TypeIdC,
    },
    ArmTypes {
        expected: TypeIdC,
        actual: TypeIdC,
    },
    /// Proposed static rejection, pending nominal spec-owner adoption. The
    /// existing runtime Bind rejects duplicate names when a call is bound;
    /// rejecting even an uncalled declaration is a separate static judgment.
    DuplicateParameter {
        name: String,
        index: usize,
    },
    /// Agrees with the owner G4 implementation on planned Matches. Applying
    /// this to dedicated-only unplanned Matches is a proposed static rule,
    /// pending adoption; the legacy runtime treats a repeated binder as a miss.
    DuplicatePatternBinder(String),
    EmptyMatch,
    UnsupportedDomain,
    UnsupportedExpression(&'static str),
    /// The borrow is not a complete unchanged owner-prepared source product.
    IncompletePreparation(&'static str),
    Preparation(ExtTypeBuildError),
}

// The owner error has no Debug implementation. Preserve it intact while making
// the additive result usable with normal Rust Result diagnostics and tests.
impl std::fmt::Debug for NominalBodyTypeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownVariable(n) => f.debug_tuple("UnknownVariable").field(n).finish(),
            Self::UnknownFunction(n) => f.debug_tuple("UnknownFunction").field(n).finish(),
            Self::Arity {
                name,
                expected,
                actual,
            } => f
                .debug_struct("Arity")
                .field("name", name)
                .field("expected", expected)
                .field("actual", actual)
                .finish(),
            Self::Expected { expected, actual } => f
                .debug_struct("Expected")
                .field("expected", expected)
                .field("actual", actual)
                .finish(),
            Self::BranchTypes { yes, no } => f
                .debug_struct("BranchTypes")
                .field("yes", yes)
                .field("no", no)
                .finish(),
            Self::ArmTypes { expected, actual } => f
                .debug_struct("ArmTypes")
                .field("expected", expected)
                .field("actual", actual)
                .finish(),
            Self::DuplicateParameter { name, index } => f
                .debug_struct("DuplicateParameter")
                .field("name", name)
                .field("index", index)
                .finish(),
            Self::DuplicatePatternBinder(n) => {
                f.debug_tuple("DuplicatePatternBinder").field(n).finish()
            }
            Self::EmptyMatch => f.write_str("EmptyMatch"),
            Self::UnsupportedDomain => f.write_str("UnsupportedDomain"),
            Self::UnsupportedExpression(n) => {
                f.debug_tuple("UnsupportedExpression").field(n).finish()
            }
            Self::IncompletePreparation(n) => {
                f.debug_tuple("IncompletePreparation").field(n).finish()
            }
            Self::Preparation(_) => f.write_str("Preparation(owner ExtTypeBuildError)"),
        }
    }
}
impl std::fmt::Debug for NominalBodyTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NominalBodyTypeError")
            .field("source", &self.source)
            .field("kind", &self.kind)
            .finish()
    }
}

type Environment = BTreeMap<String, TypeIdC>;
type Result<T> = std::result::Result<T, NominalBodyTypeError>;
struct Checker<'a> {
    prepared: &'a PreparedExtProgramSource,
    // Borrow-only identity, never serialized or dereferenced as a raw pointer.
    uses: BTreeMap<*const TypeExprSrc, T3Record>,
}
fn error(source: &SourceLocation, kind: NominalBodyTypeErrorKind) -> NominalBodyTypeError {
    NominalBodyTypeError {
        source: source.clone(),
        kind,
    }
}
fn incomplete(source: &SourceLocation, detail: &'static str) -> NominalBodyTypeError {
    error(
        source,
        NominalBodyTypeErrorKind::IncompletePreparation(detail),
    )
}
fn child(source: &SourceLocation, index: usize) -> SourceLocation {
    let mut source = source.clone();
    source.child_path.push(index);
    source
}
fn require(actual: TypeIdC, expected: TypeIdC, source: &SourceLocation) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(error(
            source,
            NominalBodyTypeErrorKind::Expected { expected, actual },
        ))
    }
}

/// Check every body and closed entry under the caller's existing explicit
/// owner-prepared pure domain. Named calls consult signatures without unfolding.
/// Type IDs and constructor field substitutions come from Pass T, not a second
/// resolver. The caller must retain the unchanged preparation for interpreting
/// returned IDs. This is a proposed static predicate, not owner admission or a
/// typechecked public profile. Structural Eq permits any two welltyped operands;
/// its static extension follows universal A/B value-sum equality, pending spec
/// adoption. Duplicate-parameter rejection and duplicate-binder rejection on
/// dedicated-only unplanned Matches also remain proposed static judgments.
///
/// Uses heap worklists rather than native recursive descent. No new depth/work/
/// heap/time limits or whole-process untrusted-input guarantee are introduced.
pub fn check_nominal_program_bodies(
    prepared: &PreparedExtProgramSource,
) -> Result<NominalBodyTypeSummary> {
    let root = SourceLocation {
        function_index: None,
        child_path: vec![],
    };
    if !prepared.domain.is_finance_absent() || !prepared.domain.is_generic_nominal() {
        return Err(error(&root, NominalBodyTypeErrorKind::UnsupportedDomain));
    }
    let mut checker = Checker {
        prepared,
        uses: BTreeMap::new(),
    };
    let program = &prepared.program;
    let mut walker = T3UseWalker::new(&program.functions, &program.entry);
    for record in &prepared.t3_records {
        let use_site = walker
            .next_use()
            .map_err(|e| error(&root, NominalBodyTypeErrorKind::Preparation(e)))?
            .ok_or_else(|| incomplete(&root, "extra T3 record"))?;
        if use_site.site != record.site || record.type_id.0 >= prepared.tables.types.len() {
            return Err(incomplete(&root, "T3 record site or type"));
        }
        let ty = match use_site.kind {
            T3UseKind::Signature { ty } => {
                if record.ctor_id.is_some() {
                    return Err(incomplete(&root, "signature constructor ID"));
                }
                ty
            }
            T3UseKind::Constructor { ty, .. } | T3UseKind::PatternCtor { ty, .. } => ty,
        };
        checker.uses.insert(ty as *const _, *record);
    }
    if walker
        .next_use()
        .map_err(|e| error(&root, NominalBodyTypeErrorKind::Preparation(e)))?
        .is_some()
    {
        return Err(incomplete(&root, "missing T3 record"));
    }
    let mut function_returns = Vec::new();
    for (index, function) in program.functions.iter().enumerate() {
        let source = SourceLocation {
            function_index: Some(index),
            child_path: vec![],
        };
        let mut environment = Environment::new();
        for (index, parameter) in function.parameters.iter().enumerate() {
            let ty = checker.resolved(&parameter.ty, &source)?.type_id;
            if environment.insert(parameter.name.clone(), ty).is_some() {
                return Err(error(
                    &source,
                    NominalBodyTypeErrorKind::DuplicateParameter {
                        name: parameter.name.clone(),
                        index,
                    },
                ));
            }
        }
        let actual = checker.infer(&function.body, environment, source.clone())?;
        require(
            actual,
            checker.resolved(&function.return_type, &source)?.type_id,
            &source,
        )?;
        function_returns.push(actual);
    }
    let entry_type = checker.infer(&program.entry, Environment::new(), root)?;
    Ok(NominalBodyTypeSummary {
        function_returns,
        entry_type,
    })
}

impl Checker<'_> {
    fn resolved(&self, ty: &TypeExprSrc, source: &SourceLocation) -> Result<T3Record> {
        self.uses
            .get(&(ty as *const _))
            .copied()
            .ok_or_else(|| incomplete(source, "unrecorded source type use"))
    }
    fn fields(
        &self,
        ty: &TypeExprSrc,
        ctor: &str,
        arity: usize,
        source: &SourceLocation,
    ) -> Result<(TypeIdC, Vec<TypeIdC>)> {
        let record = self.resolved(ty, source)?;
        let id = record
            .ctor_id
            .ok_or_else(|| incomplete(source, "missing constructor ID"))?;
        let row = self
            .prepared
            .tables
            .constructors
            .get(id.0)
            .ok_or_else(|| incomplete(source, "constructor ID"))?;
        // Builtin constructor texts are NOT Pass-B interned: their name_id is
        // a sentinel zero. The owner resolves qualified builtin patterns by
        // these fixed source ordinals (List: Nil/Cons, Tree: Leaf/Branch).
        let name_matches = match (record.type_id, row.source_order) {
            (TypeIdC::List, 0) => ctor == "Nil",
            (TypeIdC::List, 1) => ctor == "Cons",
            (TypeIdC::Tree, 0) => ctor == "Leaf",
            (TypeIdC::Tree, 1) => ctor == "Branch",
            (ty, _) if ty.0 >= 4 => {
                self.prepared
                    .interned_names
                    .get(row.name_id)
                    .map(String::as_str)
                    == Some(ctor)
            }
            _ => false,
        };
        if row.owning_type != record.type_id || row.field_len != arity || !name_matches {
            return Err(incomplete(
                source,
                "constructor record does not match source",
            ));
        }
        let end = row
            .field_start
            .checked_add(row.field_len)
            .ok_or_else(|| incomplete(source, "constructor field slice"))?;
        let fields = self
            .prepared
            .tables
            .ctor_fields
            .get(row.field_start..end)
            .ok_or_else(|| incomplete(source, "constructor field slice"))?;
        if fields
            .iter()
            .any(|f| f.type_id.0 >= self.prepared.tables.types.len())
        {
            return Err(incomplete(source, "constructor field type"));
        }
        Ok((record.type_id, fields.iter().map(|f| f.type_id).collect()))
    }

    fn literal(&self, root: &ExtValue, source: &SourceLocation) -> Result<TypeIdC> {
        enum Task<'a> {
            Visit(&'a ExtValue, SourceLocation),
            Finish(TypeIdC, Vec<TypeIdC>, SourceLocation),
        }
        let mut work = vec![Task::Visit(root, source.clone())];
        let mut results = Vec::new();
        while let Some(task) = work.pop() {
            match task {
                Task::Visit(value, source) => match value {
                    ExtValue::Core { value } => results.push(match value {
                        CoreValue::UInt { .. } => TypeIdC::UInt,
                        CoreValue::Bool { .. } => TypeIdC::Bool,
                        CoreValue::List { .. } => TypeIdC::List,
                        CoreValue::Tree { .. } => TypeIdC::Tree,
                    }),
                    ExtValue::Constructor { ty, ctor, fields } => {
                        let (ty, expected) = self.fields(ty, ctor, fields.len(), &source)?;
                        work.push(Task::Finish(ty, expected, source.clone()));
                        for (i, field) in fields.iter().enumerate().rev() {
                            work.push(Task::Visit(field, child(&source, i)));
                        }
                    }
                },
                Task::Finish(ty, expected, source) => {
                    let actual = take_results(&mut results, expected.len(), &source)?;
                    for (i, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
                        require(actual, expected, &child(&source, i))?;
                    }
                    results.push(ty);
                }
            }
        }
        finish_result(results, source)
    }

    fn pattern(
        &self,
        root: &ExtPattern,
        ty: TypeIdC,
        mut env: Environment,
        source: &SourceLocation,
    ) -> Result<Environment> {
        let mut work = vec![(root, ty)];
        let mut seen = BTreeSet::new();
        while let Some((pattern, ty)) = work.pop() {
            let mut bind = |name: &String, ty| -> Result<()> {
                if !seen.insert(name.clone()) {
                    return Err(error(
                        source,
                        NominalBodyTypeErrorKind::DuplicatePatternBinder(name.clone()),
                    ));
                }
                env.insert(name.clone(), ty);
                Ok(())
            };
            match pattern {
                ExtPattern::Wildcard => {}
                ExtPattern::Dedicated { pattern } => match pattern {
                    CorePattern::PVar { x } => bind(x, ty)?,
                    CorePattern::PNil => require(ty, TypeIdC::List, source)?,
                    CorePattern::PCons { x, y } => {
                        require(ty, TypeIdC::List, source)?;
                        bind(x, TypeIdC::UInt)?;
                        bind(y, TypeIdC::List)?;
                    }
                    CorePattern::PLeaf { x } => {
                        require(ty, TypeIdC::Tree, source)?;
                        bind(x, TypeIdC::UInt)?;
                    }
                    CorePattern::PBranch { x, y } => {
                        require(ty, TypeIdC::Tree, source)?;
                        bind(x, TypeIdC::Tree)?;
                        bind(y, TypeIdC::Tree)?;
                    }
                },
                ExtPattern::Ctor {
                    ty: source_ty,
                    ctor,
                    fields,
                } => {
                    let (expected, field_types) =
                        self.fields(source_ty, ctor, fields.len(), source)?;
                    require(ty, expected, source)?;
                    work.extend(fields.iter().zip(field_types).rev());
                }
            }
        }
        Ok(env)
    }

    fn infer(&self, root: &ExtExpr, env: Environment, source: SourceLocation) -> Result<TypeIdC> {
        enum Task<'a> {
            Visit(&'a ExtExpr, Environment, SourceLocation),
            Let(&'a str, &'a ExtExpr, Environment, SourceLocation),
            Binary(BinaryOp, SourceLocation),
            Unary(UnaryOp, SourceLocation),
            If(SourceLocation),
            Call(Vec<TypeIdC>, TypeIdC, SourceLocation),
            Match(
                &'a [crate::sherlock::helper_machine::shared::owner_ext_t3_uses::ExtArm],
                Environment,
                SourceLocation,
            ),
            Arms(usize, SourceLocation),
        }
        let mut work = vec![Task::Visit(root, env, source.clone())];
        let mut results = Vec::new();
        while let Some(task) = work.pop() {
            match task {
                Task::Visit(expr, env, source) => match expr {
                    ExtExpr::Literal { value } => results.push(self.literal(value, &source)?),
                    ExtExpr::Var { name } => results.push(*env.get(name).ok_or_else(|| {
                        error(
                            &source,
                            NominalBodyTypeErrorKind::UnknownVariable(name.clone()),
                        )
                    })?),
                    ExtExpr::Let { name, value, body } => {
                        work.push(Task::Let(name, body, env.clone(), source.clone()));
                        work.push(Task::Visit(value, env, child(&source, 0)));
                    }
                    ExtExpr::Binary { op, left, right } => {
                        work.push(Task::Binary(*op, source.clone()));
                        work.push(Task::Visit(right, env.clone(), child(&source, 1)));
                        work.push(Task::Visit(left, env, child(&source, 0)));
                    }
                    ExtExpr::Unary { op, operand } => {
                        work.push(Task::Unary(*op, source.clone()));
                        work.push(Task::Visit(operand, env, child(&source, 0)));
                    }
                    ExtExpr::If {
                        cond,
                        then_expr,
                        else_expr,
                    } => {
                        work.push(Task::If(source.clone()));
                        work.push(Task::Visit(else_expr, env.clone(), child(&source, 2)));
                        work.push(Task::Visit(then_expr, env.clone(), child(&source, 1)));
                        work.push(Task::Visit(cond, env, child(&source, 0)));
                    }
                    ExtExpr::Call { name, args } => {
                        let function = self
                            .prepared
                            .program
                            .functions
                            .iter()
                            .find(|f| &f.name == name)
                            .ok_or_else(|| {
                                error(
                                    &source,
                                    NominalBodyTypeErrorKind::UnknownFunction(name.clone()),
                                )
                            })?;
                        if function.parameters.len() != args.len() {
                            return Err(error(
                                &source,
                                NominalBodyTypeErrorKind::Arity {
                                    name: name.clone(),
                                    expected: function.parameters.len(),
                                    actual: args.len(),
                                },
                            ));
                        }
                        let expected = function
                            .parameters
                            .iter()
                            .map(|p| self.resolved(&p.ty, &source).map(|r| r.type_id))
                            .collect::<Result<Vec<_>>>()?;
                        let ty = self.resolved(&function.return_type, &source)?.type_id;
                        work.push(Task::Call(expected, ty, source.clone()));
                        for (i, arg) in args.iter().enumerate().rev() {
                            work.push(Task::Visit(arg, env.clone(), child(&source, i)));
                        }
                    }
                    ExtExpr::Construct { ty, ctor, fields } => {
                        let (ty, expected) = self.fields(ty, ctor, fields.len(), &source)?;
                        work.push(Task::Call(expected, ty, source.clone()));
                        for (i, field) in fields.iter().enumerate().rev() {
                            work.push(Task::Visit(field, env.clone(), child(&source, i)));
                        }
                    }
                    ExtExpr::Match { scrutinee, arms } => {
                        work.push(Task::Match(arms, env.clone(), source.clone()));
                        work.push(Task::Visit(scrutinee, env, child(&source, 0)));
                    }
                    ExtExpr::StateRead { .. } => {
                        return Err(error(
                            &source,
                            NominalBodyTypeErrorKind::UnsupportedExpression("StateRead"),
                        ));
                    }
                    ExtExpr::LiabilityPresent { .. } => {
                        return Err(error(
                            &source,
                            NominalBodyTypeErrorKind::UnsupportedExpression("LiabilityPresent"),
                        ));
                    }
                    ExtExpr::LiabilityAmount { .. } => {
                        return Err(error(
                            &source,
                            NominalBodyTypeErrorKind::UnsupportedExpression("LiabilityAmount"),
                        ));
                    }
                    ExtExpr::Fin { .. } => {
                        return Err(error(
                            &source,
                            NominalBodyTypeErrorKind::UnsupportedExpression("Fin"),
                        ));
                    }
                    ExtExpr::Await { .. } => {
                        return Err(error(
                            &source,
                            NominalBodyTypeErrorKind::UnsupportedExpression("Await"),
                        ));
                    }
                },
                Task::Let(name, body, mut env, source) => {
                    env.insert(name.into(), pop(&mut results, &source)?);
                    work.push(Task::Visit(body, env, child(&source, 1)));
                }
                Task::Binary(op, source) => {
                    let right = pop(&mut results, &source)?;
                    let left = pop(&mut results, &source)?;
                    let (l, r, ty) = match op {
                        BinaryOp::Eq => {
                            results.push(TypeIdC::Bool);
                            continue;
                        }
                        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                            (TypeIdC::UInt, TypeIdC::UInt, TypeIdC::UInt)
                        }
                        BinaryOp::Lt => (TypeIdC::UInt, TypeIdC::UInt, TypeIdC::Bool),
                        BinaryOp::Cons => (TypeIdC::UInt, TypeIdC::List, TypeIdC::List),
                        BinaryOp::Branch => (TypeIdC::Tree, TypeIdC::Tree, TypeIdC::Tree),
                    };
                    require(left, l, &child(&source, 0))?;
                    require(right, r, &child(&source, 1))?;
                    results.push(ty);
                }
                Task::Unary(op, source) => {
                    let actual = pop(&mut results, &source)?;
                    let (expected, ty) = match op {
                        UnaryOp::Leaf => (TypeIdC::UInt, TypeIdC::Tree),
                        UnaryOp::Not => (TypeIdC::Bool, TypeIdC::Bool),
                    };
                    require(actual, expected, &child(&source, 0))?;
                    results.push(ty);
                }
                Task::If(source) => {
                    let no = pop(&mut results, &source)?;
                    let yes = pop(&mut results, &source)?;
                    require(
                        pop(&mut results, &source)?,
                        TypeIdC::Bool,
                        &child(&source, 0),
                    )?;
                    if yes != no {
                        return Err(error(
                            &source,
                            NominalBodyTypeErrorKind::BranchTypes { yes, no },
                        ));
                    }
                    results.push(yes);
                }
                Task::Call(expected, ty, source) => {
                    let actual = take_results(&mut results, expected.len(), &source)?;
                    for (i, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
                        require(actual, expected, &child(&source, i))?;
                    }
                    results.push(ty);
                }
                Task::Match(arms, env, source) => {
                    let ty = pop(&mut results, &source)?;
                    if arms.is_empty() {
                        return Err(error(&source, NominalBodyTypeErrorKind::EmptyMatch));
                    }
                    // Bind patterns in source order before scheduling bodies.
                    let mut visits = Vec::new();
                    for (i, arm) in arms.iter().enumerate() {
                        let source = child(&source, i + 1);
                        let inner = self.pattern(&arm.pattern, ty, env.clone(), &source)?;
                        visits.push(Task::Visit(&arm.body, inner, source));
                    }
                    work.push(Task::Arms(arms.len(), source));
                    work.extend(visits.into_iter().rev());
                }
                Task::Arms(count, source) => {
                    let actual = take_results(&mut results, count, &source)?;
                    let expected = actual[0]; // EmptyMatch was checked before scheduling.
                    for (i, actual) in actual.into_iter().enumerate().skip(1) {
                        if actual != expected {
                            return Err(error(
                                &child(&source, i + 1),
                                NominalBodyTypeErrorKind::ArmTypes { expected, actual },
                            ));
                        }
                    }
                    results.push(expected);
                }
            }
        }
        finish_result(results, &source)
    }
}

fn pop(results: &mut Vec<TypeIdC>, source: &SourceLocation) -> Result<TypeIdC> {
    results
        .pop()
        .ok_or_else(|| incomplete(source, "internal type result stack"))
}
fn take_results(
    results: &mut Vec<TypeIdC>,
    count: usize,
    source: &SourceLocation,
) -> Result<Vec<TypeIdC>> {
    let start = results
        .len()
        .checked_sub(count)
        .ok_or_else(|| incomplete(source, "internal type result arity"))?;
    Ok(results.split_off(start))
}
fn finish_result(mut results: Vec<TypeIdC>, source: &SourceLocation) -> Result<TypeIdC> {
    if results.len() != 1 {
        return Err(incomplete(source, "internal type result cardinality"));
    }
    pop(&mut results, source)
}

#[cfg(test)]
#[path = "nominal_body_typecheck_tests.rs"]
mod tests;
