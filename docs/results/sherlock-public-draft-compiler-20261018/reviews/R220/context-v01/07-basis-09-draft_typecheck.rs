//! SOURCE_ONLY / UNRUN: ordinary monomorphic typing proposal for the actual AST.
//! No execution, source profile adoption, H1 admission or host resource guarantee.
use crate::sherlock::{BinaryOp, Expr, Function, Name, Pattern, Program, Type, UnaryOp, Value};
use crate::sherlock_draft::SourceLocation;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftTypeSummary {
    pub function_returns: Vec<Type>,
    pub entry_type: Type,
}

/// Local preparation diagnostics, not runtime Outcomes or existing error codes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftTypeError {
    pub source: SourceLocation,
    pub kind: DraftTypeErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DraftTypeErrorKind {
    UnknownVariable(Name),
    UnknownFunction(Name),
    Arity { name: Name, expected: usize, actual: usize },
    Expected { expected: Type, actual: Type },
    BranchTypes { yes: Type, no: Type },
    ArmTypes { expected: Type, actual: Type },
    DuplicateParameter { name: Name, index: usize },
    DuplicatePatternBinder(Name),
    EmptyMatch,
    Budget(DraftTypeBudgetExceeded),
}

/// Distinct preparation resource, never an evaluator Outcome/error code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftTypeBudgetResource { TypeDepth }

/// `source` on the containing DraftTypeError names the admitted parent.
/// `child_index` names its refused child, or None for a refused body/entry root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftTypeBudgetExceeded {
    pub resource: DraftTypeBudgetResource,
    pub limit: usize,
    /// First attempted depth; None means checked depth arithmetic overflowed.
    pub observed: Option<usize>,
    pub child_index: Option<usize>,
}

#[derive(Clone, Copy)]
struct DepthBudget { limit: Option<usize> }

impl DepthBudget {
    fn next_depth(self, parent_depth: usize, source: &SourceLocation,
                  child_index: Option<usize>) -> Result<usize, DraftTypeError> {
        let Some(limit) = self.limit else {
            // Legacy traversal has no hidden cap and needs no depth arithmetic.
            return Ok(0);
        };
        let observed = parent_depth.checked_add(1);
        if let Some(next) = observed {
            if next <= limit { return Ok(next); }
        }
        // Copy only the already-admitted parent path, never the refused child.
        Err(error(source, DraftTypeErrorKind::Budget(DraftTypeBudgetExceeded {
            resource: DraftTypeBudgetResource::TypeDepth, limit, observed, child_index,
        })))
    }
}

type Types = BTreeMap<Name, Type>;

/// Inspect every body under fresh parameter types, then entry under an empty
/// environment. Named calls consult declarations, never recurse into callees.
/// Thus mutual recursion requires no source unrolling or inferred recursion cap.
/// This checker is a review-pending ordinary typed subset, not admission policy.
///
/// # Security Warning (H1 — Unmetered Host Evaluator)
/// This machine is a host diagnostic reference with zero native authority and no host time,
/// heap, or native stack admission bounds. Do not expose it to untrusted inputs.
pub fn check_program_types(program: &Program) -> Result<DraftTypeSummary, DraftTypeError> {
    check_program_types_inner(program, DepthBudget { limit: None })
}

/// Bound active structural Expr inference depth with an explicit inclusive limit.
/// Roots have depth1; each Expr child adds1; every body and entry resets the root.
/// Calls inspect signatures/arguments without descending into named callees.
/// Checks precede child-path copies, descent and Let/Match environment copies.
/// A budget error records the admitted parent source plus attempted child index;
/// refused roots record their root source with no child index. This permits a
/// precise location without first constructing the over-budget child path.
/// Zero refuses the first root before parameter environment construction.
/// No default, arbitrary cap, global work/heap or native-stack guarantee is given;
/// H1 and parser/copy/report/Drop/public-consumer obligations remain separate.
pub fn check_program_types_bounded(program: &Program, max_type_depth: usize)
    -> Result<DraftTypeSummary, DraftTypeError> {
    check_program_types_inner(program, DepthBudget { limit: Some(max_type_depth) })
}

