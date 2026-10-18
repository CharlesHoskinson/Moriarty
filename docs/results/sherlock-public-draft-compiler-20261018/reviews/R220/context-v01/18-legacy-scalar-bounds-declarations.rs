/// Caller-stated finite host extents for preprocessing. There is deliberately
/// no Default and no profile that supplies these numbers: the caller states
/// every extent, and a normal envelope is a caller-specific operational
/// choice, not a policy default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceHostBounds {
    pub max_source_actions: usize,
    pub max_source_expressions: usize,
    /// Statements summed over every action, because Core validation walks all.
    pub max_source_statements: usize,
    pub max_action_inputs: usize,
    pub max_supplied_inputs: usize,
    /// Unshared (per-edge) projection occurrences of the selected action,
    /// including synthetic scalar-lowering nodes, counted before
    /// prepare_scalar_action allocates them.
    pub max_projection_occurrences: usize,
    /// Generated A AST nodes (function body plus entry call and its literals),
    /// counted iteratively before source_input_bridge allocates the B tree.
    pub max_generated_ast_nodes: usize,
    /// Generated A AST nesting. The B CoreProgram drop is owner-iterative; the
    /// A Arc tree drop is not, so nesting is bounded explicitly. Native stack
    /// bytes are not measured or claimed by this count.
    pub max_ast_nesting: usize,
    pub max_input_name_bytes: usize,
    pub max_total_input_name_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostMeasure {
    SourceActions,
    SourceExpressions,
    SourceStatements,
    ActionInputs,
    SuppliedInputs,
    ProjectionOccurrences,
    GeneratedAstNodes,
    AstNesting,
    InputNameBytes,
    TotalInputNameBytes,
}

