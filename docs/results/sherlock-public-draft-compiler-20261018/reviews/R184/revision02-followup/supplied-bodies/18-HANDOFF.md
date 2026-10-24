R181 revision02 contains implemented test additions and a separate R120 mapping
patch. Neither R181 production afterimage changed. No defect requiring a parser
or lexer source edit was identified during this scoped feedback implementation.

Review findings were checked against the actual R181 source and test text and
R120 consumers before edits. R120's old match omits Budget; the old positive
assertions use parser parity; the exact recursion, nonbinary depth and excess
invalid token cases were absent. R184's optional capacity/collector suggestions
remain unapplied. This source inspection does not replace its security opinion.

`tests/bounded_parser.rs` retains the original tests and adds complete literal
ChainTotal/BranchTotal Program ASTs and SourceProvenance comparisons, independent
precedence structure, exact parser depth 5-to-6 on `((1))`, 1-to-2 on literal
`1` (also 0-to-1), and structural depth 1-to-2 on `leaf(1)` with positive depth2.
`tests/lexer_token_limit.rs` adds the `a @`, limit2, observed3 pre-scan assertion.
The exact LF-terminated R115 fixtures live under `tests/inputs/`.

Independent expectations: R115 EXPECTED.json / DERIVATION.md and exact fixtures
are retained unchanged and pinned as inputs in SOURCE-FREEZE.json. Expected ASTs
are manually transcribed from the independent row/child derivation; complete
provenance literals are transcribed from its expected objects. No parser output,
evaluated result or new subject execution supplies an expectation. Source-text
checks compare every expression-site literal, declaration/parameter span,
pattern span/binder and fixture byte to that frozen independent data. This is
transcription evidence, not a Rust test result or oracle independence audit.

`r120-mapping/candidate.patch` changes only draft_contract.rs and
draft_diagnostics.rs against exact R120 candidate preimages. The contract adds
required `max_structural_ast_depth: UsizeDecimal`, documents Expr+Pattern
occurrences and parser depth, and implements `parser_budget()` as an exact eight
field projection. `max_type_depth` and `max_copy_depth` remain separate and are
not used by this projection. Diagnostics add the exhaustive typed Budget arm;
resource names match DraftBudgetResource variants and limit/observed are decimal
strings, consistent with R120's existing usize diagnostic representation.

Root should copy the revised tests with `tests/inputs` preserved, then select
module wiring after fresh designated review. These files still use `super::*`
as child modules. No production module declarations were added. The mapping
patch is against unintegrated R120, not W; Root must compare exact preimages
before adoption and call `preprocessing.parser_budget()` in its real consumer.

No compiler, formatter, Cargo, Rust test, parser, checker, evaluator, bridge,
provider, model, adapter, SDK or native action ran. Only source reads, copies,
edits, hashes and stdlib literal/JSON comparisons were performed. No W/Git writes.
Fresh narrow Opus/Gemini review is required before execution. Public consumer
wiring, numerical policy, checker/type recursion, general copying/reporting,
recursive destruction/cleanup, H1 admission and downstream acceptance remain
separate. Native0/C4/R2/60460/stopped Amendment10 are unchanged.
