{
  "identity_and_role": {
    "reviewer_model": "Gemini 4 Argon (High) (standing route gemini-4-argon-high HIGH under Root-supplied primary Opus HIGH quota outage)",
    "role": "Fresh independent non-author designated-security reviewer for R181",
    "candidate_author": "Sol (R181 bounded draft parser/lexer source candidate)",
    "author_independence": "Reviewer did not author R181; R179 candidate, peer reviews, and desired verdicts were not supplied.",
    "execution_and_tool_discipline": "Source-only audit before any candidate execution. Read all 42 required pages across BODY-01..BODY-31 exactly once each via view_file with AbsolutePath only and default full-file parameters. Zero compiler, Cargo, test, lexer, parser, evaluator, shell, write/edit, Git, network, or native actions."
  },
  "read_coverage_and_limits": {
    "bodies_read_complete": "BODY-01 (3 pages, 25464B, sha256=caa5bbad2cedf9856caa2896908971d1c8920f94e985cc7543dda855c812e9ef) through BODY-31 (1 page, 8813B, sha256=e0018814b139bbde17b3a375d7b05b43d9a893e9af3eea14c99dcb67461cfec2); all 42 pages read in full without clipping or missing pages.",
    "abstentions": "None on supplied bodies.",
    "evidence_limits": [
      "Static source inspection only; zero rustc typecheck/borrowcheck or test execution performed.",
      "Unprovided third-party implementation bodies (std, std::sync::Arc, soa_rs::Soa/Soars, miette, serde/serde_json) are external dependencies/limits, not audited.",
      "Does not promise absence of all defects."
    ]
  },
  "decision": {
    "verdict": "APPROVE_WITH_LIMITS",
    "root_readiness": "Ready for Root consideration of selected ordinary checks after wiring/adapting downstream R120 exhaustive DraftError match and strengthening proposed test assertions. This is NOT a generic H1, evaluator, public compiler, or native authority grant.",
    "preserved_invariants": ["Native0", "C4OPEN", "R2UNSAT", "60460HOLD", "stopped Amendment10"]
  },
  "findings_by_question": {
    "Q1_positive_capability_grammar_and_interface": {
      "verdict": "PASS_WITH_DOWNSTREAM_INTERFACE_OBLIGATION",
      "verified_predicates": [
        "parse_draft_source_bounded (BODY-01 L132-145) is a real parser entry, not a refusal stub: it checks SourceBytes, lexes via mori_lexer::lex_with_token_limit(source, budget.max_tokens), and runs Parser::new(source, tokens, Some(budget)).program().",
        "Unbounded parse_draft_source (BODY-01 L123-126) retains its signature, H1 warning (BODY-01 L120-122), mori_lexer::lex call, and Parser::new(source, tokens, None).program() path where all budget checks are no-ops.",
        "Grammar, binary precedence/left-associativity (==, < at 1; +, - at 2; * at 3 via self.expression(precedence + 1) at BODY-01 L339, L348-357), eager argument order (BODY-01 L438-449), reserved words (BODY-01 L515-518), function table order (BODY-01 L268-307), and SourceProvenance order (entry preorder then bodies in declaration order, BODY-01 L316-319) match preimage BODY-03.",
        "In atom_inner (BODY-01 L364-370), moving `if self.kind(0) == TokenKind::LParen` above `self.reserve_occurrence()?; self.structural_depth(0)?;` and `self.word(\"let\")` preserves grammar because LParen is punctuation (`is_word()` is false, BODY-11 L132-134) while ensuring parentheses add no AST occurrence or structural depth and still update `node.span = SourceSpan { start, end: self.end }`.",
        "Against R115 independent fixtures ChainTotal (BODY-27) and BranchTotal (BODY-28), Parser::program + collect_sites matches R115 `preparation.provenance` spans, kinds, child_paths, and pattern binder sites (BODY-26, BODY-29). Downstream R115 `preparation.types`, `owner_mapping`, and 12-table `program_view` belong to typechecking and A-to-B owner lowering, not parser output.",
        "All 4 lexer hard limits (MAX_SOURCE_BYTES=65536, MAX_TOKENS=8192, MAX_IDENT_BYTES=64, MAX_STRING_BYTES=1024 in BODY-02 L29-35) remain enforced on both `lex` and `lex_with_token_limit`."
      ],
      "findings": [
        {
          "id": "F-Q1-1",
          "severity": "MEDIUM (Downstream Compile Hazard)",
          "location": "BODY-01 L71-76 vs BODY-23 L8-18",
          "snippet": "pub enum DraftError { Lex(String), Syntax { .. }, UIntLiteral { .. }, Budget(DraftBudgetExceeded) }",
          "analysis": "Adding `DraftError::Budget` causes a non-exhaustive match compile error in R120 `draft_diagnostics.rs` `pub(super) fn source(error: DraftError) -> Value` (BODY-23 L8-18), which currently only matches `Lex`, `Syntax`, and `UIntLiteral`. Disclosed in BODY-08 L56-57 and BODY-09 L20."
        },
        {
          "id": "F-Q1-2",
          "severity": "LOW (Test Wiring Context)",
          "location": "BODY-06 L1-2; BODY-07 L1-2",
          "snippet": "use super::*;",
          "analysis": "Proposed test files use `use super::*;` as child modules of `draft_source.rs` and `mori-lexer/src/lib.rs` respectively and are not yet declared via `#[cfg(test)] mod` in BODY-01/BODY-02."
        }
      ]
    },
    "Q2_eight_prework_budgets_and_stack_model": {
      "verdict": "PASS_WITH_IMPLEMENTATION_NOTES",
      "trace_of_8_limits": {
        "1_max_source_bytes": "BODY-01 L135: `within(DraftBudgetResource::SourceBytes, budget.max_source_bytes, source.len())?;` checked before `lex_with_token_limit`. Exact `source.len() == max_source_bytes` passes; `max_source_bytes + 1` fails with `observed: source.len()` before lexing.",
        "2_max_tokens": "BODY-02 L76-80, L134-142, L159-165: `max_tokens == 0` fails with `TokenLimit { limit: 0, observed: 1 }` before `Lexer::with_token_capacity(source, max_tokens.min(MAX_TOKENS))`. In `run_limited`, before scanning any non-whitespace/non-comment token (`!skips_token`), checks `self.tokens.len() >= limit.saturating_sub(1)` (reserving 1 slot for `TokenKind::Eof`) and returns `TokenLimit { limit, observed: self.tokens.len().saturating_add(2) }`. Comments (`//`, `/* */`) skip token counting and are bounded by source byte limits.",
        "3_max_functions": "BODY-01 L269-272: inside `while self.word(\"fn\")`, checks `within(DraftBudgetResource::Functions, budget.max_functions, functions.len().saturating_add(1))?;` before bumping `fn`, parsing signature/body, or pushing to `functions`/`sites`/`bodies`.",
        "4_max_parameters_per_function": "BODY-01 L282-284: inside parameter loop, checks `within(DraftBudgetResource::ParametersPerFunction, budget.max_parameters_per_function, parameters.len().saturating_add(1))?;` before `self.name()`, `self.ty()`, or pushing `Parameter`/`ParameterSite`.",
        "5_max_total_parameters": "BODY-01 L285-288: checked immediately after `ParametersPerFunction` via `within(DraftBudgetResource::TotalParameters, budget.max_total_parameters, self.total_parameters.saturating_add(1))?;` before incrementing `self.total_parameters` and parsing the parameter.",
        "6_max_ast_occurrences": "BODY-01 L187-194: `reserve_occurrence` checks `self.ast_occurrences.saturating_add(1)` against `budget.max_ast_occurrences` before incrementing. Called in `atom_inner` (L371, after `LParen` early branch) before `let`/`if`/`match`/literal/constructor/call/var parsing, in `expression_inner` (L341) before `Node::new(Expr::Binary(..))`, and in `pattern_inner` (L483) before pattern parsing/allocation.",
        "7_max_parser_recursion_depth": "BODY-01 L175-186: `descend` checks `self.parser_depth.saturating_add(1)` against `budget.max_parser_recursion_depth` before incrementing `self.parser_depth` and entering `block_inner` (L322), `expression_inner` (L331), `atom_inner` (L359), `arguments_inner` (L435), `match_expression_inner` (L451), and `pattern_inner` (L480). All recursive parser methods go through `descend`.",
        "8_max_structural_ast_depth": "BODY-01 L156-159, L195-201: `Node::new` computes `depth = children.iter().map(|child| child.depth).max().unwrap_or(0) + 1`. `self.structural_depth(0)` is checked at `atom_inner` (L372) for leaves/atoms, and `self.structural_depth(max_child_depth)` is checked before `Node::new` in `expression_inner` (L342, Pratt binary chains), `let` (L380), `if` (L390), constructors (L416), `Call` (L429), and `match_expression_inner` (L473). Parentheses (L364-370) add 0 to `Node.depth` while adding 2 (`expression` + `atom`) to `parser_depth`."
      },
      "findings": [
        {
          "id": "F-Q2-1",
          "severity": "LOW (Behavioral Ordering Note)",
          "location": "BODY-01 L334-346, L371-433",
          "snippet": "self.structural_depth(children.iter().map(|child| child.depth).max().unwrap_or(0))?;",
          "analysis": "Because composite `StructuralAstDepth` is checked bottom-up from completed child `Node.depth` values before parent `Node::new` (as documented in BODY-08 L43), top-down nesting (e.g. `leaf(leaf(leaf(1)))`) is gated during descent by `max_parser_recursion_depth` and `max_ast_occurrences`, and triggers `StructuralAstDepth` when unwinding to construct the first parent `Node` whose depth would exceed `max_structural_ast_depth`. Consequently, no `Node` or `Expr` with `depth > max_structural_ast_depth` is ever constructed on the heap."
        },
        {
          "id": "F-Q2-2",
          "severity": "LOW (Preallocation Sizing Note)",
          "location": "BODY-02 L79",
          "snippet": "Lexer::with_token_capacity(source, max_tokens.min(MAX_TOKENS))",
          "analysis": "When `max_tokens` is large (e.g. `MAX_TOKENS = 8192`) and `source` is tiny, `with_token_capacity` preallocates `min(max_tokens, 8192)` token slots instead of `(source.len() / 8 + 1).min(max_tokens).min(MAX_TOKENS)`. Still strictly bounded by `min(max_tokens, 8192)`."
        },
        {
          "id": "F-Q2-3",
          "severity": "INFO (Fixed Stack Overhead & Unmetered Cleanup)",
          "location": "BODY-01 L147-160, L175-186, L320",
          "snippet": "struct Node { expression: ExprRef, span: SourceSpan, children: Vec<Node>, .. }",
          "analysis": "Each logical `parser_depth` unit spans multiple native Rust call frames (wrapper + `descend` + closure + `*_inner`), so arbitrary caller-supplied `max_parser_recursion_depth` is NOT guaranteed native-stack safe. Dropping `Node` trees (at end of `Parser::program` or on error unwinding) and dropping `Arc<Program>`/`ExprRef` uses unmetered recursive Rust `Drop` bounded by `max_structural_ast_depth`."
        }
      ]
    },
    "Q3_provenance_collector_and_r120_integration": {
      "verdict": "PASS_WITH_INTEGRATION_PREDICATES",
      "verified_predicates": [
        "collect_sites (BODY-01 L520-565) is iterative using `todo: Vec<(&Node, Vec<usize>)>` and `.rev()` child push, preserving entry-first preorder then function-body preorder and arm order.",
        "Before cloning `path`/`source`/`binders` or growing `provenance.expressions`, `provenance.patterns`, or `todo`, `collect_sites` checks `StructuralAstDepth` (`root.depth`, `path.len() + 1`, `path.len() + 2`) and `AstOccurrences` (`provenance.expressions.len() + provenance.patterns.len() + 1` and `scheduled + 1`).",
        "Path cloning cost `O(max_ast_occurrences * max_structural_ast_depth)` and scope boundaries are candidly disclosed in BODY-08 L49-50, L64-68; no overclaim is made."
      ],
      "findings": [
        {
          "id": "F-Q3-1",
          "severity": "LOW (Collector Lookahead Scope)",
          "location": "BODY-01 L527, L555-558",
          "snippet": "let mut scheduled = 1usize;",
          "analysis": "`scheduled` resets to `1usize` on each `collect_sites` invocation (entry and each function body) and counts only expressions in that subtree, whereas `provenance.expressions.len() + provenance.patterns.len() + 1` (L535-537, L544-546) is cumulative across the whole program. Since `Parser::program` already enforces `self.ast_occurrences <= budget.max_ast_occurrences` across the whole program during parsing, this cannot overflow budget in `parse_draft_source_bounded`, though initializing `scheduled` from cumulative `provenance.expressions.len() + provenance.patterns.len() + 1` (and incrementing on pattern emission) would make lookahead scheduling globally cumulative per-call as well."
        }
      ]
    },
    "Q4_test_source_coverage_and_gaps": {
      "verdict": "GAPS_IDENTIFIED_IN_UNRUN_TEST_SOURCE",
      "coverage_summary": "BODY-06 (`tests/bounded_parser.rs`) and BODY-07 (`tests/lexer_token_limit.rs`) are unexecuted source proposals. BODY-06 exercises positive parity with `parse_draft_source`, inclusive exact and `limit - 1` errors for all 8 `DraftBudgetResource` variants, zero limits for functions/parameters/occurrences/structural depth, and legacy lex/syntax/UIntLiteral error preservation. BODY-07 exercises inclusive token limits with comments, zero token limit, and all 4 lexer hard limits (`MAX_SOURCE_BYTES`, `MAX_IDENT_BYTES`, `MAX_STRING_BYTES`, `MAX_TOKENS`). Existing lexer tests in BODY-16 and BODY-17 remain compatible with unchanged `mori_lexer::lex`.",
      "findings": [
        {
          "id": "F-Q4-1",
          "severity": "MEDIUM (Missing Independent Positive/Provenance Expectations)",
          "location": "BODY-06 L29-40",
          "snippet": "assert_eq!(parse_draft_source_bounded(&text, limits()), parse_draft_source(&text));",
          "analysis": "Positive test relies solely on equality with legacy `parse_draft_source(&text)`. It does not assert independent AST structure (e.g. `1 + 2 * 3` parsed as `Add(1, Mul(2, 3))`) or independent `SourceProvenance` spans/paths/kinds (such as R115's independently derived `ChainTotal` BODY-27 / `BranchTotal` BODY-28 provenance in BODY-26)."
        },
        {
          "id": "F-Q4-2",
          "severity": "LOW (Missing Exact Recursion Boundary 5 on `((1))` and Non-Binary Structural Depth)",
          "location": "BODY-06 L94-108, L117-125",
          "snippet": "exceeds(&source(\"((1))\"), budget, DraftBudgetResource::ParserRecursionDepth, 4, 5);\nbudget.max_parser_recursion_depth = 6;",
          "analysis": "For `((1))` (which requires parser recursion depth 6), BODY-06 tests limit 4 (`observed: 5`) and jumps to 6 (ok), omitting the exact `limit - 1` boundary `max_parser_recursion_depth = 5` (`observed: 6`). Also omits `max_parser_recursion_depth = 1` (and `0`) on `source(\"1\")` (where exact depth is 2), and only tests `StructuralAstDepth` on binary `+` and literal `1` rather than `let`/`if`/`match`/`call`/constructor or function body expressions."
        },
        {
          "id": "F-Q4-3",
          "severity": "LOW (Missing Parser-Level Lexer Hard-Limit Precedence Check & Pre-Scan Token Limit Test)",
          "location": "BODY-06 L128-133; BODY-07 L5-28",
          "snippet": "fn legacy_diagnostics_remain_the_original_payloads()",
          "analysis": "BODY-06 does not test `parse_draft_source_bounded` when `budget.max_source_bytes > MAX_SOURCE_BYTES` or `budget.max_tokens > MAX_TOKENS` to verify lexer hard limits still yield `DraftError::Lex` through the bounded parser entry. BODY-07 does not test that `lex_with_token_limit(\"a @\", 2)` returns `TokenLimit { limit: 2, observed: 3 }` before scanning the invalid `@` token."
        }
      ]
    }
  },
  "mandatory_minimal_corrections": [
    {
      "target": "BODY-23 (crates/mori-core/src/sherlock/compile_artifact/draft_diagnostics.rs L8-18) & BODY-22 (draft_contract.rs L53-63)",
      "correction": "Before compiling R120 with R181, add `DraftError::Budget(exceeded)` arm to `draft_diagnostics::source`, document `max_ast_nodes` as syntactic `Expr + Pattern` occurrences (`max_ast_occurrences`), map `max_parse_depth` to `max_parser_recursion_depth`, and add a distinct `max_structural_ast_depth: UsizeDecimal` field to `DraftPreprocessingExtents` without reusing `max_type_depth` or `max_copy_depth`."
    },
    {
      "target": "BODY-06 (tests/bounded_parser.rs L29-40, L94-125)",
      "correction": "Add independent AST and `SourceProvenance` assertions (including R115 `ChainTotal` / `BranchTotal` provenance spans, child_paths, expression kinds, and pattern binders, plus `1 + 2 * 3` precedence structure) rather than only `assert_eq!(..., parse_draft_source(&text))`. Add exact `limit - 1` recursion boundary `max_parser_recursion_depth = 5` (`observed: 6`) for `((1))`, `max_parser_recursion_depth = 1` (`observed: 2`) for `source(\"1\")`, and a non-binary `StructuralAstDepth` case (e.g. `leaf(1)` or `if`/`match`/function body)."
    },
    {
      "target": "BODY-07 (tests/lexer_token_limit.rs L5-14)",
      "correction": "Add assertion `assert!(matches!(lex_with_token_limit(\"a @\", 2), Err(TokenLimitedLexError::TokenLimit { limit: 2, observed: 3 })))` to verify pre-scan EOF slot reservation before lexical error on the excess token."
    },
    {
      "target": "BODY-01 (collect_sites L527) & BODY-02 (lex_with_token_limit L79) [Optional Hardening]",
      "correction": "In `lex_with_token_limit` (BODY-02 L79), cap initial capacity at `(source.len() / 8 + 1).min(max_tokens).min(MAX_TOKENS)` to avoid over-reserving 8192 token slots on tiny inputs when `max_tokens` is large. In `collect_sites` (BODY-01 L527), initialize `scheduled` from `provenance.expressions.len().saturating_add(provenance.patterns.len()).saturating_add(1)` (and increment on pattern checks) so lookahead scheduling is globally cumulative across entry and function bodies."
    }
  ],
  "remaining_integration_and_security_limitations": {
    "checker_and_type_depth": "Unadmitted in R181; `sherlock_draft_types` typechecking recursion/work and `max_type_depth` enforcement remain separate.",
    "copy_and_report_bounds": "Unadmitted in R181; general AST cloning, `max_copy_depth`, A-to-B owner lowering (`prepare_core` / `ProgramView`), and report serialization remain separate.",
    "recursive_drop_and_cleanup": "Unmetered; dropping `Node` (`Vec<Node>`) and `Program` (`Arc<Expr>`) recurses on the host stack up to `max_structural_ast_depth`.",
    "public_compile_consumer_and_config": "Unwired; R120 `compile_draft_host`, config schema extension for `max_structural_ast_depth`, and numerical cap policy selection remain Root-owned.",
    "H1_and_native_authority": "H1 (unmetered host evaluator) remains open; native authority is 0; arbitrary caller limits do not guarantee native stack safety."
  }
}

