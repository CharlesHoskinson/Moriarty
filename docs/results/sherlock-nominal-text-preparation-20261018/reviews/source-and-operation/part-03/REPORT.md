# Independent Nonauthor Source, Security, API, and Ordinary-Operation Review: Explicit Nominal Text Source (`moriarty-sherlock-nominal/1`)

- **Reviewer identity & context**: Fresh nonauthor context (Gemini 4 Argon High standing fallback during Opus outage).
- **Author & exception scope**: Candidate source (`nominal_source.rs`, `lib.rs` registration, `design.md`, `spec.md`, 3 `.mori` fixtures) and 8 finite parser/preparation tests are Sol-authored under the user's standing parser-depth exception. The source author is not an independent voter.
- **Payload coverage**: All 3 ordered parts read in full in a single fresh context (Part 1: offset `0`, `138,475` bytes; Part 2: offset `138,475`, `138,499` bytes; Part 3: offset `276,974`, `120,541` bytes; total `397,515` bytes across **BODY 01 – BODY 37**). No tools or commands were invoked.
- **Execution status**: Pre-execution source/security/operation audit only. All 10 proposed operations are currently **UNRUN**.

---

## 1. Overall Verdict & Mandatory Corrections

- **Candidate Verdict**: **`APPROVE`**
- **Mandatory Corrections / Required Hunks**: **None** (`0` hunks required). All candidate source files, module registrations, AST/provenance constructions, budget checks, test assertions, and operation definitions match the installed `mori_lexer` and `shared::owner_prepare` interfaces without defect.

---

## 2. Exact 10-Operation Eligibility

All ten proposed operations in `REVIEW-PROJECTION.json` (**BODY 36**, `sha256=10985ce2f7d8a33624bf59d295c233861bb7edd51998792aede022a42ebd09d6`) are **`ELIGIBLE`** under the ordinary SDK-free compiler/test execution class:

1. **`01-compileNormalCoreNominalTextRlib` — `ELIGIBLE`**
   - **Reason**: Invokes the pinned `1.99.0-x86_64-unknown-linux-gnu/bin/rustc` directly (no Cargo, no build scripts, no SDK/native libraries, `cfg(test)` false) on `snapshot/crates/mori-core/src/lib.rs` (`--crate-name mori_core --crate-type=rlib`), linking only the 6 pinned existing rlibs (`k256`, `mori_lexer`, `serde`, `serde_json`, `sha2`, `thiserror`), bounded by `timeout --signal=KILL 30s`, `prlimit --cpu=25:25 --as=2147483648:2147483648 --fsize=67108864:67108864 --core=0:0`, a 3-variable isolated environment (`PATH`, `LANG`, `TMPDIR`), `131,072`-byte per-stream capture caps, and `.d` dep-info closure verification (`build/core-rlib.d`) before downstream consumption.
2. **`02-compileNominalTextEightTests` — `ELIGIBLE`**
   - **Reason**: Invokes pinned `rustc --test` on `snapshot/crates/mori-core/tests/sherlock_nominal_source_positive.rs` against the freshly pinned `build/libmori_core.rlib` from Operation `01` under the same `30s` wall / `25s` CPU / `2 GiB` AS / `64 MiB` FSIZE / `0` core / `131,072`-byte stream caps and `.d` dep-info closure verification (`build/nominal-text-positive.d`).
3. **`03-positive-text_schedule_preserves_recursive_declarations_ordered_patterns_and_owner_types` — `ELIGIBLE`**
   - **Reason**: Runs `build/nominal-text-positive --exact positive::text_schedule_preserves_recursive_declarations_ordered_patterns_and_owner_types --test-threads=1 --nocapture` under `timeout --signal=KILL 10s` and `prlimit --cpu=8:8 --as=1073741824:1073741824 --fsize=16777216:16777216 --core=0:0`, requiring exact `1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out` (never zero-test `exit 0`).
4. **`04-positive-text_mutual_forward_references_and_generic_parameter_bindings_reach_owner` — `ELIGIBLE`**
   - **Reason**: Runs exact selector `positive::text_mutual_forward_references_and_generic_parameter_bindings_reach_owner` with `--test-threads=1 --nocapture` under the same `10s` wall / `8s` CPU / `1 GiB` AS / `16 MiB` FSIZE / `131,072`-byte stream caps and `1/0/0/0/7` test-membership check.
5. **`05-positive-text_nested_applications_patterns_and_complete_occurrence_paths` — `ELIGIBLE`**
   - **Reason**: Runs exact selector `positive::text_nested_applications_patterns_and_complete_occurrence_paths` with `--test-threads=1 --nocapture` under the same finite resource bounds and `1/0/0/0/7` test-membership check.
6. **`06-budgets-source_bytes_and_eof_inclusive_tokens_keep_exact_existing_resources` — `ELIGIBLE`**
   - **Reason**: Runs exact selector `budgets::source_bytes_and_eof_inclusive_tokens_keep_exact_existing_resources` with `--test-threads=1 --nocapture` under the same finite resource bounds and `1/0/0/0/7` test-membership check.
7. **`07-budgets-expression_occurrences_and_structural_depth_have_inclusive_boundaries` — `ELIGIBLE`**
   - **Reason**: Runs exact selector `budgets::expression_occurrences_and_structural_depth_have_inclusive_boundaries` with `--test-threads=1 --nocapture` under the same finite resource bounds and `1/0/0/0/7` test-membership check.
8. **`08-budgets-parser_descent_includes_parentheses_and_nested_type_applications` — `ELIGIBLE`**
   - **Reason**: Runs exact selector `budgets::parser_descent_includes_parentheses_and_nested_type_applications` with `--test-threads=1 --nocapture` under the same finite resource bounds and `1/0/0/0/7` test-membership check.
9. **`09-budgets-function_and_signature_parameter_extents_keep_existing_meanings` — `ELIGIBLE`**
   - **Reason**: Runs exact selector `budgets::function_and_signature_parameter_extents_keep_existing_meanings` with `--test-threads=1 --nocapture` under the same finite resource bounds and `1/0/0/0/7` test-membership check.
10. **`10-budgets-nested_nominal_patterns_count_each_occurrence_with_an_explicit_budget` — `ELIGIBLE`**
    - **Reason**: Runs exact selector `budgets::nested_nominal_patterns_count_each_occurrence_with_an_explicit_budget` with `--test-threads=1 --nocapture` under the same finite resource bounds and `1/0/0/0/7` test-membership check.

---

## 3. Source, Security, and API Audit (`nominal_source.rs` & `lib.rs`)

### 3.1 Module Registration (`lib.rs`, BODY 05)
- `lib.rs` (2,015 bytes, `sha256=94d4acfb6b34dee481d199222c5ec2f30cae30116fdaa61c4ce8c23061ec9d80`) adds only:
  ```rust
  // Explicit nominal text source -> existing owner type preparation, no evaluation.
  #[path = "sherlock/nominal_source.rs"]
  pub mod sherlock_nominal_source;
  ```
- Existing Core parser (`mod parser`), dedicated draft modules (`sherlock_draft`, `sherlock_draft_types`), and `sherlock` submodules remain untouched.

### 3.2 Header, Lexer Spans, and Reserved Words (`nominal_source.rs`, BODY 06)
- **Header check (`L196–L202`)**: Requires `language` followed by a `TokenKind::String` whose `mori_lexer::decode_string(self.text())` equals `NOMINAL_LANGUAGE` (`"moriarty-sherlock-nominal/1"`), followed by `; program <Ident> {`. Core headers and `"moriarty-sherlock-draft/1"` are rejected on this path and retain their own parsers.
- **Real lexer spans (`L156–L169`, `L171–L174`)**: `Parser::index` clamps lookahead to `self.tokens.len() - 1` (safe because `mori_lexer` always appends `TokenKind::Eof`). `Parser::span` delegates to `self.tokens.span(self.index(0), self.source)`. `Parser::name` requires `self.kind(0) == TokenKind::Ident && !reserved(self.text())`, where `reserved` (`L354`) covers `"language" | "program" | "type" | "construct" | "wildcard" | "fn" | "entry" | "let" | "in" | "if" | "else" | "match" | "UInt" | "Bool" | "List" | "Tree" | "nil" | "cons" | "leaf" | "branch" | "not"`.
- **Wildcard spelling (`L335`)**: Because `mori_lexer` rejects leading `_` in identifiers (`diagnostics::leading_underscore`, **BODY 16** `L268`), `wildcard` is reserved and parsed to `ExtPattern::Wildcard` without modifying the lexer.

### 3.3 Source Order, Parameter vs. Named/Apply Binding, and Qualification
- **Declaration, constructor, and field order (`L203–L204`, `L236–L263`)**:
  - `program()` parses `while self.word("type")` in source order into `types: Vec<TypeDeclSrc>` and `declarations: Vec<DeclarationSite>`.
  - `declaration()` parses optional `<T1, T2, ...>` into `parameters` (`Vec<NamedSite>`) and `parameter_names` (`Vec<String>`), then loops `while self.kind(0) != TokenKind::RBrace` to parse each constructor `name` and optional `(field: Type, ...)` terminated by `;`. Nullary constructor declarations may omit `()` (e.g., `Done;`) or include `()`.
- **`TypeExprSrc` parameter vs. `Named` vs. `Apply` (`L175–L195`)**:
  - `type_expr_inner(parameters: &[String])` accepts non-reserved `TokenKind::Ident` or `"UInt" | "Bool" | "List" | "Tree"`.
  - If followed by `TokenKind::Lt` (`<`), it parses comma-separated argument types via recursive `self.type_expr(parameters)` calls and returns `TypeExprSrc::Apply { name, args }` preserving argument order.
  - Otherwise, it matches `"UInt" => TypeExprSrc::UInt`, `"Bool" => TypeExprSrc::Bool`, `"List" => TypeExprSrc::List`, `"Tree" => TypeExprSrc::Tree`, `_ if parameters.iter().any(|p| p == &name) => TypeExprSrc::Param { name }`, and `_ => TypeExprSrc::Named { name }`.
  - Only `declaration()` passes `&parameter_names` (`L251`); function parameter types (`L217`), function return types (`L223`), `construct` type qualifications (`L299`), and constructor pattern type qualifications (`L343`) pass `&[]`. Thus `TypeExprSrc::Param` is produced only within its declaring `type` binder scope, and forward/mutual nominal type names remain `TypeExprSrc::Named` without premature lookup or reordering.
- **Constructor vs. Call & Pattern Qualification (`L298–L301`, `L317–L319`, `L342–L351`)**:
  - Expressions use explicit `construct <Type>::<Ctor>(<args>)` (`ExtExpr::Construct { ty, ctor, fields }`), whereas `<name>(<args>)` is `ExtExpr::Call { name, args }` and bare `<name>` is `ExtExpr::Var { name }`.
  - Dedicated syntax (`nil`, `cons`, `leaf`, `branch`, `not` in expressions at `L309–L316`; `nil`, `cons`, `leaf`, `branch` in patterns at `L336–L341`) is preserved with exact arity checks (`1` for `leaf`/`not`, `2` for `cons`/`branch`).
  - In `pattern_inner` (`L342`), `self.kind(0).is_word() && (self.kind(1) == TokenKind::Lt || self.kind(1) == TokenKind::Colon)` unambiguously selects a type-qualified constructor pattern `<Type>::<Ctor>(<subpatterns>)` (`ExtPattern::Ctor { ty, ctor, fields }`) versus a bare binder `self.name()?` (`ExtPattern::Dedicated { pattern: CorePattern::PVar { x } }`), with no casing heuristic or symbol-table dependency.
- **`u128` Literal Diagnostic Behavior (`L302–L305`)**:
  - `TokenKind::Number` strips `_` and parses via `.parse::<u128>()`, mapping overflow or fractional numbers (which `mori_lexer` tokenizes as `TokenKind::Number`) to `DraftError::UIntLiteral { span, text }` and valid integers to `ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::UInt { n: KIntC::InRange(n) } } }`.

### 3.4 Recursive Descent, AST Occurrences, Structural Depth, Provenance, and Drop
- **Active parser descent (`Parser::descend`, `L135–L140`)**:
  - Checks `within(DraftBudgetResource::ParserRecursionDepth, b.max_parser_recursion_depth, self.parser_depth.saturating_add(1))?` before incrementing `self.parser_depth`.
  - Wraps `type_expr` (`L175`), `block` (`L265`), `expression` (`L276`), `atom` (`L286`), `arguments` (`L321`), `match_expression` (`L329`), and `pattern` (`L351`).
  - Parentheses (`L290`) recurse `atom -> expression -> atom`, consuming descent without calling `reserve_occurrence()` or adding a `Site` depth level.
  - Nested type applications (`Box<Box<UInt>>`) and nested constructor patterns (`Schedule::Both(Schedule::Pay(...), ...)`) recurse through `type_expr` and `pattern`, bounding call-stack depth by `max_parser_recursion_depth`.
- **Syntactic `Expr` + `Pattern` occurrences (`Parser::reserve_occurrence`, `L141–L145`)**:
  - Checked before constructing each non-parenthesized atom (`L291`), each binary operator node (`L282`), and each pattern node (`L334`, including every nested pattern inside `ExtPattern::Ctor`).
- **Structural `Expr` depth (`Parser::structural_depth` / `structural_sites`, `L146–L153`)**:
  - Leaf atoms check `structural_depth(0)` (depth `1`); composite expressions (`Binary` `L282`, `Let` `L294`, `If` `L297`, `Construct` `L299`, dedicated constructors `L311`, `Call` `L318`, `Match` `L330`) check `max_child_depth + 1` against `max_structural_ast_depth` before constructing `Site::new` (which computes `depth = max_child_depth.saturating_add(1)`).
- **Iterative provenance collection (`collect`, `L355–L374`)**:
  - Uses explicit heap stacks (`todo` for expressions, `pending` for nested arm patterns) with `.enumerate().rev()` child pushes to emit exact preorder `ExpressionOccurrence` (entry first, then function bodies in table order) and `PatternOccurrence` (`match_path`, `arm`, `field_path`), rechecking `StructuralAstDepth` and `AstOccurrences` before each push.
- **Iterative `Drop` and bounded preparation flow (`L76–L104`)**:
  - `ExtProgram` carriers (`TypeExprSrc` in **BODY 24** `L27–L38`; `ExtExpr`, `ExtValue`, `ExtPattern` in **BODY 26** `L74–L119`) implement iterative heap-worklist `Drop`. Parse-local `Site` depth is bounded by `max_parser_recursion_depth`.
  - `prepare_nominal_source_types_bounded` (`L95–L104`) runs `parse_nominal_source_bounded(source, budget)` first and only on `Ok(parsed)` invokes `shared::prepare_ext_program_types(parsed.program, layout, domain, locator)`. It never falls back to `parse_nominal_source`.

---

## 4. Assessment of `DraftParseBudget` Sufficiency and Policy

- **Existing 8-field contract**: `DraftParseBudget` (**BODY 15** `L85–L95`) has no default values and comprises `max_source_bytes`, `max_tokens`, `max_functions`, `max_parameters_per_function`, `max_total_parameters`, `max_ast_occurrences`, `max_parser_recursion_depth`, and `max_structural_ast_depth`.
- **Sufficiency for parser resource bounding**:
  - Every nominal `type` declaration, type parameter, constructor, field, and `TypeExprSrc` node consumes at least one non-EOF lexical token and is therefore strictly bounded in count by `min(budget.max_tokens, mori_lexer::MAX_TOKENS)` (`8,192`), while `source.len()` is bounded by `min(budget.max_source_bytes, mori_lexer::MAX_SOURCE_BYTES)` (`65,536`).
  - Every nested `TypeExprSrc::Apply` argument and nested `ExtPattern::Ctor` child pattern enters `Parser::descend`, bounding native stack frames by `budget.max_parser_recursion_depth`.
  - Not charging `type` declarations/fields against `max_functions`, `max_parameters_per_function`, `max_total_parameters`, or `max_ast_occurrences` preserves the exact meanings of those existing fields.
- **Concrete remaining policy requirements (for future public compiler/admission integration, not missing from this leaf)**:
  1. If a future public nominal host-compile configuration schema (analogous to `DraftPreprocessingExtents` in `draft_contract.rs`, **BODY 33**) requires separate explicit source-level caps on nominal declaration count, type parameters per declaration, constructors per declaration, fields per constructor, `TypeExprSrc` node count, or nested pattern depth (independent of `max_tokens`, `max_parser_recursion_depth`, and owner `BoundsC`), those fields must be explicitly added to that future nominal preprocessing config contract rather than silently aliased onto `DraftParseBudget`.
  2. Whole-process H1 admission (bounding owner Pass-B/Pass-T heap, recursive drop of parse-local `Site` if `max_parser_recursion_depth` is set dangerously high by a caller, and wall/CPU time) remains a caller/controller responsibility, not a property of `DraftParseBudget` alone.

---

## 5. Verification of All 8 Author-Owned Tests and 3 `.mori` Fixtures

### Positive Parser + Owner Type-Preparation Tests (`nominal_text_source_positive.rs`, BODY 08)
1. **`positive::text_schedule_preserves_recursive_declarations_ordered_patterns_and_owner_types`** (`schedule.mori`, **BODY 10**, 587 bytes):
   - Verifies `p.name == "OrderedSchedule"`, 1 declaration `Schedule` with 0 type parameters, constructors `["Done", "Pay", "Both"]`, field lists `[]`, `[("amount", "UInt"), ("tail", "named:Schedule")]`, `[("left", "named:Schedule"), ("right", "named:Schedule")]`, function `choose(s: Schedule): UInt`, and 4 ordered match arms (`Schedule::Both(Schedule::Pay(x, wildcard), wildcard)`, `Schedule::Both(wildcard, r)`, `Schedule::Pay(x, t)`, `Schedule::Done()`).
   - Calls `prepare_nominal_source_types_bounded(..., ExtDomain::GENERIC_ABSENT, None)` and verifies exact owner table counts:
     - `tables.types.len() == 5` (4 builtins `0..=3` + `Schedule` at `TypeIdC(4)`).
     - `tables.constructors.len() == 7` (4 builtin list/tree ctors + 3 `Schedule` ctors).
     - `tables.ctor_fields.len() == 9` (5 builtin list/tree fields + `0 + 2 + 2` `Schedule` fields).
     - `decl_param_counts == [0]`, `decl_ctor_counts == [3]`.
     - `t3_records` contains `ParamType { function: 0, param: 0 }` with `type_id == TypeIdC(4)` and `CtorPattern { arm: 0, pattern: 1, .. }` (`Schedule::Pay` inside `Schedule::Both`) with `type_id == TypeIdC(4)` and `ctor_id.is_some()`.
2. **`positive::text_mutual_forward_references_and_generic_parameter_bindings_reach_owner`** (`mutual-generic.mori`, **BODY 11**, 437 bytes):
   - Verifies declarations `["Even", "Odd", "Box"]`, forward reference `named:Odd` in `Even::SuccOdd`, mutual reference `named:Even` in `Odd::SuccEven`, scoped `param:T` in `Box<T>::Box`, and `apply:Box<named:Even>` in `wrap`/`unwrap` signatures.
   - Verifies owner preparation produces `tables.types.len() == 7` (4 builtins + `Even` `TypeIdC(4)` + `Odd` `TypeIdC(5)` + instantiated `Box<Even>` `TypeIdC(6)`), `decl_param_counts == [0, 0, 1]`, `decl_ctor_counts == [2, 1, 1]`, and `T3UseSite` records binding `ReturnType { function: 0 }` and `ParamType { function: 1, param: 0 }` to `TypeIdC(6)`.
3. **`positive::text_nested_applications_patterns_and_complete_occurrence_paths`** (`nested-application.mori`, **BODY 12**, 319 bytes):
   - Verifies `first(b: Box<Box<UInt>>)` parameter type `"apply:Box<apply:Box<UInt>>"`, nested constructor pattern `"apply:Box<apply:Box<UInt>>::Box(apply:Box<UInt>::Box(bind:value))"`, `Function(0)` pattern provenance `[(0, [], "constructor-pattern"), (0, [0], "constructor-pattern"), (0, [0, 0], "binder"), (1, [], "wildcard")]`, `Entry` expression provenance `[([], "call"), ([0], "construct"), ([0, 0], "construct"), ([0, 0, 0], "uint")]`, and owner table counts `tables.types.len() == 6` (4 builtins + `Box<UInt>` `TypeIdC(4)` + `Box<Box<UInt>>` `TypeIdC(5)`), `decl_param_counts == [1]`, `tables.type_args.len() == 2`.

### Finite Parser-Budget Boundary Tests (`nominal_text_source_budgets.rs`, BODY 09)
4. **`budgets::source_bytes_and_eof_inclusive_tokens_keep_exact_existing_resources`**:
   - On `LITERAL` (`67` bytes, `10` real tokens + `1` `Eof` = `11` tokens): exact pass at `max_source_bytes = 67, max_tokens = 11`; exact refusal `(SourceBytes, 66, 67)` at `66`; exact refusal `(Tokens, 10, 11)` at `10` (lexer reserves 1 slot for `Eof` at token index 9); exact refusal `(Tokens, 0, 1)` at `0`.
5. **`budgets::expression_occurrences_and_structural_depth_have_inclusive_boundaries`**:
   - On `LITERAL`: pass at `(max_ast_occurrences = 1, max_structural_ast_depth = 1)`, refusal `(AstOccurrences, 0, 1)` and `(StructuralAstDepth, 0, 1)` at `0`.
   - On `1 + 2 * 3` (3 leaves + 2 binary nodes = `5` occurrences; root-to-leaf path `+ -> * -> 2/3` = depth `3`): pass at `(5, 3)`, refusal `(AstOccurrences, 4, 5)` at `4` and `(StructuralAstDepth, 2, 3)` at `2`.
6. **`budgets::parser_descent_includes_parentheses_and_nested_type_applications`**:
   - On `LITERAL`: `expression -> atom` = depth `2` (pass at `2`, refusal `(ParserRecursionDepth, 1, 2)` at `1`).
   - On `Nested` (`b: Box<Box<UInt>>` and body `{ 1 }`): `type_expr(Box<Box<UInt>>) -> type_expr(Box<UInt>) -> type_expr(UInt)` = depth `3`, and `block -> expression -> atom` = depth `3` (pass at `3`, refusal `(ParserRecursionDepth, 2, 3)` at `2`).
   - On `Parens` (`entry (7);`): `expression -> atom('(') -> expression -> atom(7)` = depth `4` (refusal `(ParserRecursionDepth, 3, 4)` at `3`).
7. **`budgets::function_and_signature_parameter_extents_keep_existing_meanings`**:
   - On `fn f(x:UInt):UInt { x }`: pass at `(max_functions = 1, max_parameters_per_function = 1, max_total_parameters = 1)`; exact refusals `(Functions, 0, 1)`, `(ParametersPerFunction, 0, 1)`, and `(TotalParameters, 0, 1)` at `0`.
8. **`budgets::nested_nominal_patterns_count_each_occurrence_with_an_explicit_budget`**:
   - On `match 7 { Box<UInt>::Box(Box<UInt>::Box(x)) => 0, wildcard => 1 }`: counts `match` (`1`) + scrutinee `7` (`1`) + arm 0 nested patterns (`3`: outer `Ctor`, inner `Ctor`, binder `x`) + arm 0 body `0` (`1`) + arm 1 pattern `wildcard` (`1`) + arm 1 body `1` (`1`) = `8` occurrences (`type Box<T>` and `Box<UInt>` type annotations do not consume `AstOccurrences`). Exact pass at `8`, refusal `(AstOccurrences, 7, 8)` at `7`.

---

## 6. Coverage and Explicit Limitations

- **Reviewed in full in this audit**:
  - All 37 bodies (`397,515` payload bytes), including `nominal_source.rs`, `lib.rs`, `design.md`, `spec.md`, the 3 `.mori` fixtures, both test modules (8 tests), `draft_source.rs`, `mori-lexer` (`lib.rs`, `token.rs`, `number.rs`, `scan.rs`, `string.rs`, `diagnostics.rs`), `shared.rs`, `owner_prepare.rs`, `owner_ext_type_builder.rs`, `owner_ext_source_program.rs`, `owner_ext_t3_uses.rs`, `owner_ext_prepare_types.rs`, `ext_pass_b_walk.rs`, `ext_source_traversal.rs`, `nominal_declaration_order.rs`, `adt.rs`, `adt_storage.rs`, `draft_contract.rs`, `recursive_source.rs`, `compile_draft_host`, `REVIEW-PROJECTION.json`, and the 6 selected capture/verification functions in `run_selected_13.py`.
- **Explicit limitations (not claimed or discharged by this review)**:
  1. **No execution performed yet**: All 2 compiles and 8 test runs in `REVIEW-PROJECTION.json` are unrun proposals awaiting Root dispatch and post-run artifact/dep-info/output verification.
  2. **Not a public generic compiler or runtime admission**: `prepare_nominal_source_types_bounded` prepares nominal text into `ExtProgram` and runs owner Pass-B name collection, declaration validation, T1/T2 type/constructor/field table construction, and Pass-T (`T3UseWalker`) resolution (`PreparedExtProgramSource`). It does **not** connect to `compile_draft_host` (which remains wired to dedicated 4-sort `Arc<Program>` / `CoreProgram` and the 12-field `ProgramView`), nor does it implement generic Pass-C/Pass-P `MatchPlan` compilation, generic B-machine admission/evaluation, witness generation, or native export.
  3. **Author-owned pre-execution tests**: The 8 tests are Sol-authored parser and owner-type-preparation checks under the standing parser-depth exception, not independent runtime semantic goldens or hostile owner-guard / malformed-witness security suites.
  4. **Retained toolchain & broader crate pins**: The 148 unchanged `mori-core` Rust files (`4afb` baseline) and 321 retained ordinary toolchain/library inputs in `FLAT-INPUT-PINS.json` are byte-pinned rather than freshly semantically re-audited in full.

