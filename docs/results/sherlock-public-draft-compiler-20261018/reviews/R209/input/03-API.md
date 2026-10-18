# R200 bounded type-checker source contract

Status: SOURCE_ONLY / UNRUN. Sol authoring is explicitly authorized by the October18UTC narrow type-checker exception in current W AGENTS. Fresh designated Opus/Gemini review is required before ANY candidate execution. R192 returned no source or prior API; its historical request is source context only.

New entry:
`pub fn check_program_types_bounded(program: &Program, max_type_depth: usize) -> Result<DraftTypeSummary, DraftTypeError>`.
Legacy `check_program_types` retains its exact signature and has no implicit limit. Both call one implementation; all original error kinds and normal payloads are retained. New public declarations (derive Clone, Debug, PartialEq, Eq; resource also Copy):

```rust
pub enum DraftTypeBudgetResource { TypeDepth }
pub struct DraftTypeBudgetExceeded {
    pub resource: DraftTypeBudgetResource,
    pub limit: usize,
    pub observed: Option<usize>,
    pub child_index: Option<usize>,
}
// One added variant in the existing DraftTypeErrorKind:
// Budget(DraftTypeBudgetExceeded)
```

Depth is simultaneously active structural Expr inference depth: each body root and entry root is1; each structural Expr child adds1. Pattern checking is part of its admitted arm-body depth and adds no extra Expr. Named calls infer arguments and use the first matching declaration's signature; no callee unrolling. Each body and entry resets root depth independently. Body traversal remains original table order, including shadowed/unreachable bodies, before entry.

The caller supplies an inclusive maximum with no Default, constructor-selected value or arbitrary cap. Zero rejects the first root BEFORE parameter-name/environment work; exact maximum is accepted. A checked-add computes the next depth before descent, child-path cloning/creation, or Let/Match environment copying. Exhaustion has a distinct Budget kind, resource TypeDepth and caller limit. `observed=Some(next_depth)` is the first attempted over-limit depth, never a completed visit count; `None` means the attempted next depth overflowed usize. Overflow returns the same Budget variant, never wraps, saturates or substitutes a type/runtime error.

Budget-error `DraftTypeError.source` is the precise deepest ADMITTED parent SourceLocation; `child_index=Some(i)` identifies the attempted child without constructing/copying its over-budget path. For a refused root, source is its function/entry root and child_index=None. The error copies only the admitted parent path (at most limit-1 child indices), not the rejected child path. Root0 refusal has an empty path. This diagnostic allocation is not a blanket heap guarantee.

Normal inference/type-error ordering and locations are preserved when the budget permits traversal. Otherwise depth refusal precedes errors inside the refused Expr or its pending Let/Match environment/pattern processing; ordinary errors already encountered retain precedence. Unknown-function/arity checks precede argument admission; If condition inference and Bool check precede branch admission; Binary/Eq both operand inferences precede sort requirements. Mixed-sort Eq remains accepted after inference. Root rejection precedes duplicate parameters because it precedes environment construction.

No parser, copy bridge, owner preparation, reporting, Drop, config or whole public compiler consumer is changed. Root must adapt the exhaustive ordinary diagnostic mapping for the new Budget variant and later consume max_type_depth at the public compiler boundary. H1 and remaining parser/copy/report/Drop/general resource premises remain open. Depth bounds do not limit wide inputs, total nodes/work/bytes, parameter/name/environment sizes, allocation failure or native stack safety for arbitrary caller extents.
