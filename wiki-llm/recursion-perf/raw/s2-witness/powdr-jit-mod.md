# powdr JIT modules
> Source: https://raw.githubusercontent.com/powdr-labs/powdr-legacy/main/executor/src/witgen/jit/mod.rs
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Initially fetched with Scrapling; raw HTTP text retained below because HTML extraction drops source-code angle brackets. SHA-256 `5eb02618f6a0598ba6d8ff8170faf2e20a887910fedd1e0cb5eca4258c58b6ec`. Mutable branch snapshot, not a pinned upstream release.

```text
mod block_machine_processor;
pub(crate) mod code_cleaner;
mod compiler;
mod debug_formatter;
mod effect;
pub(crate) mod function_cache;
mod identity_queue;
mod interpreter;
mod processor;
mod prover_function_heuristics;
mod single_step_processor;
mod variable;
pub(crate) mod witgen_inference;

type QuadraticSymbolicExpression<T, V> =
    powdr_constraint_solver::grouped_expression::GroupedExpression<
        powdr_constraint_solver::symbolic_expression::SymbolicExpression<T, V>,
        V,
    >;

#[cfg(test)]
pub(crate) mod test_util;

```
