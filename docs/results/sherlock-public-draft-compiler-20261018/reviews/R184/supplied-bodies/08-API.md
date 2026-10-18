R181 source-only parser candidate, authored by Sol under the latest scoped user
authorization. Fresh designated Opus/Gemini audit is required before execution.

Entry: `parse_draft_source_bounded(source: &str, budget: DraftParseBudget)
-> Result<ParsedDraft, DraftError>` in `crates/mori-core/src/sherlock/draft_source.rs`.

`DraftParseBudget` has eight explicit inclusive `usize` fields:

| Field | Meaning |
| --- | --- |
| `max_source_bytes` | UTF-8 source bytes, checked before the lexer |
| `max_tokens` | Real lexer tokens including EOF, excluding comments |
| `max_functions` | Function declarations in original order |
| `max_parameters_per_function` | Parameters in each function |
| `max_total_parameters` | Parameters across all functions |
| `max_ast_occurrences` | Each source Expr and Pattern occurrence, without pointer deduplication |
| `max_parser_recursion_depth` | Simultaneously active block/expression/atom/arguments/match/pattern descent bodies |
| `max_structural_ast_depth` | Longest Expr root-to-leaf path; leaf depth is one |

No constructor supplies defaults. Zero allows zero of a resource. Every draft
program needs at least one Expr, one EOF token and the necessary parser descent
depth. Parentheses increase parser depth but add no AST node. Left-associated
Pratt chains increase structural AST depth even at shallow parser depth.

Errors: original `DraftError::Lex(String)`, `Syntax { span, expected, found }`
and `UIntLiteral { span, text }` payloads are retained. Added variant:
`DraftError::Budget(DraftBudgetExceeded { resource, limit, observed })`.
`resource` is a `DraftBudgetResource` variant: `SourceBytes`, `Tokens`,
`Functions`, `ParametersPerFunction`, `TotalParameters`, `AstOccurrences`,
`ParserRecursionDepth`, `StructuralAstDepth`. `observed` is source size or the
first attempted count/depth above the limit. No machine outcome code is reused.

The original `parse_draft_source` uses no caller budget and preserves its API and
H1 warning. It still uses `mori_lexer::lex` with the original hard limits. The
new lexer entry `lex_with_token_limit` returns `TokenLimitedLexError::Lex` or
`TokenLimit { limit, observed }`; it shares the existing scanning/emission path,
retains the hard limits, reserves EOF before each real token, and initializes
bounded-path token capacity to `min(caller_limit, MAX_TOKENS)`. It does not scan
and allocate the whole token stream before checking the caller token count.

Occurrence checks precede Expr/Pattern construction and their owned data.
Function and parameter checks precede declaration/binder allocation. Structural
depth is checked before parent Expr/Node construction, using child depths.
Descents check the caller recursion extent before entering each descent body;
fixed wrapper/guard frames are implementation overhead, not independent caller
depth units. Provenance collection is iterative: it checks root/path depth,
scheduled expression occurrences and cumulative output occurrences before
path/binder clones or site growth. Existing entry-preorder then function-table
order is retained. Its path storage has occurrence-times-structural-depth cost;
this interface is not a total byte-allocation budget or wall-time guarantee.

Root integration: R120 `max_ast_nodes` needs the explicit occurrence meaning
above, `max_parse_depth` maps to parser recursion, and config needs a distinct
structural AST depth extent. `max_type_depth` and `max_copy_depth` are later
phase limits; neither is silently reused as this parser's structural extent.
Root must extend R120 `draft_diagnostics.rs`'s exhaustive `DraftError` match for
`Budget`. This candidate does not change config/CLI or select numerical caps.

The test files are source proposals intended as child modules of the parser and
lexer respectively; they are not wired or executed here. Their numerical values
are fixture inputs, not public defaults. Root chooses the exact reviewed checks
and freezes independent expectations before any execution.

Remaining boundaries: checker/type recursion, general AST copying/reporting,
partial/returned AST recursive destruction and cleanup, evaluator/native H1
admission, public compile wiring, config policy and downstream acceptance remain
separate obligations. Caller-chosen finite depth does not establish native-stack
safety for arbitrary caller limits. This candidate supplies no admission claim.
