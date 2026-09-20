# Graph Report - aeon  (2026-09-19)

## Corpus Check
- Large corpus: 505 files · ~342,339 words. Semantic extraction will be expensive (many Claude tokens). Consider running on a subfolder.

## Summary
- 7521 nodes · 29825 edges · 250 communities (191 shown, 39 thin omitted)
- Extraction: 81% EXTRACTED · 19% INFERRED · 0% AMBIGUOUS · INFERRED: 5669 edges (avg confidence: 0.94)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Terms
- Decision Tree Test
- Liquid Typechecking Test
- Helpers
- Init
- Liquidjava Protocols Test
- Tactic Synth Test
- Smt
- Program
- Typeinfer
- Helpers
- Desugar
- Engine
- Decorators
- Pprint Helpers
- Parser
- Synthesis Ui
- Runner
- Compile
- Actions
- Linearity Test
- Program
- Completion
- Cuda Qtt Test
- Grammar Generation
- Llvm Ast
- Learning
- Lta Synth Test
- Constraints
- Horn
- Match
- Match Inductive Test
- Converter
- Generate Mbpp
- Lowerer
- Api
- Cata
- Tdsyn Direction Test
- Synthesizerfactory
- Linear Random Test
- Mutual Synthesis Test
- Lsp Infoview Test
- Generators
- Resource Meters
- Ortools Cpsat Test
- Parser
- Counterexample Test
- Linear Type Test
- Synthesizer
- Executor
- Automaton
- Llvm Ast
- Generator
- Adt
- Termination Test
- Gpu Module Test
- Converter
- Lsp Features Test
- Parser
- Learning
- Image
- Vcs
- Infer Test
- Aeon Adapter
- Smt Sets Test
- Lsp Test
- Ortools Cpsat
- End To End Test
- Readme
- Cuda
- Cuda
- Synthesizer
- Trust
- Knee Point Selection Test
- Synthesizer
- Machine Learning
- Cuda
- Forkjoin
- Synthesizer
- Optimization Test
- Method Call Test
- Parametric Refinements Test
- Dataframe
- Init
- Multiplicity Test
- Exit Codes Test
- Csg Metric
- Python Export Test
- Decidability
- Server
- Lsp Test
- Lta Synth Test
- Sygus Synthesis Test
- Cuda Binding Test
- Openai Client Test
- Linearity
- Mutual Recursion Test
- Polymorphic Uninterpreted Test
- Cuda
- Ollama Manager
- Claude
- Contracts
- Refined Subtyping Test
- Nn
- Navigation
- Typeindex
- Division By Zero Test
- Cuda
- Executor
- Synthesizer
- Synthesizers
- Sygus To Aeon
- Learning Test
- String Escapes Test
- Selfification Test
- Translate
- Ollama Manager Test
- Cuda
- Cuda
- Gpu
- Reflect Underscore Test
- Typeclass Test
- Wellformed Test
- Driver
- Entrypoint
- Refinements
- Cosynthesis
- Benchmark
- Index
- Readme
- Bench Llvm Quicksort
- Bench Horn
- Afta Pbe Test
- Api
- Run
- Ffi
- Recursion Soundness Test
- Cuda
- Stack Deque
- Evaluation Pool
- Forkjoin
- Csg Benchmark Test
- Learning
- Linear Distributions Test
- Pbe
- Synthesizer
- Exp2 Metric
- Grover Sim
- Collection Literal Test
- Llvm E2E Test
- Z3 Errors
- Translate
- Typestate Protocols
- Translate
- Translate
- Gen
- Build Stdlib Docs
- Lsp Test
- Ordered Comparison Test
- Grammar Generation
- Synthesizer
- Synthesizer
- Backend Test
- Trust Report Test
- Cuda
- Email
- Order
- Typevars Test
- Actions
- Recursive Refinement Induction
- Relax Flat Gen
- Linearity Test
- Table
- Vericoding Translator Test
- Relax Gen
- Mark Linear Params
- Dace Test
- Grammar Library Import Test
- Downloader
- Learning
- Init
- Translate
- Polytypes Test
- Tdsyn Direction Test
- Docs
- Iterator
- Existentials Replace Anf
- Readme
- Readme
- Reader
- Tensor
- Linearity
- Superscripts
- Models Integration
- Readme
- Lean Syntax Migrate
- Writer
- Llvm Ast
- Bottom Up
- Bottom Up
- Float Ng
- Guards
- Name
- Summarize
- Install
- Readme
- Run Examples
- Dependabot
- Python Publish
- Ruff
- Binding Utils
- Learning
- Learning
- Learning
- Init
- Init
- Run Sweep
- Language Components
- Liquefy Builtins Test
- Match Inductive Test
- Method Call Test
- Method Call Test
- Method Call Test
- Method Call Test
- Method Call Test
- Smt Sets Test
- Smt Sets Test
- Smt Sets Test
- Smt Sets Test
- Pyproject

## God Nodes (most connected - your core abstractions)
1. `Name` - 1261 edges
2. `Term` - 587 edges
3. `TypingContext` - 412 edges
4. `Var` - 288 edges
5. `AbstractionType` - 262 edges
6. `TypeConstructor` - 257 edges
7. `LiquidTerm` - 246 edges
8. `LiquidVar` - 244 edges
9. `LiquidApp` - 244 edges
10. `RefinedType` - 236 edges

## Surprising Connections (you probably didn't know these)
- `Modular verification conditions` --semantically_similar_to--> `SMT translation bottleneck`  [INFERRED] [semantically similar]
  docs/synthesizers.md → examples/nn/mnist/README.md
- `SMT-opaque typeclass methods` --semantically_similar_to--> `Python FFI trust boundary`  [INFERRED] [semantically similar]
  examples/nn/MODELS_INTEGRATION.md → docs/ffi.md
- `test_titanic_declared_shape_matches_fixture()` --indirect_call--> `csv_file()`  [INFERRED]
  tests/machine_learning_test.py → aeon/synthesis/decorators.py
- `test_polytype()` --uses--> `TypeConstructor`  [INFERRED]
  tests/polytypes_test.py → aeon/core/types.py
- `test_parse_by_block()` --uses--> `SBy`  [INFERRED]
  tests/tactic_synth_test.py → aeon/sugar/program.py

## Import Cycles
- 2-file cycle: `aeon/lsp/aeon_adapter.py -> aeon/lsp/server.py -> aeon/lsp/aeon_adapter.py`
- 2-file cycle: `aeon/synthesis/api.py -> aeon/synthesis/evaluation_pool.py -> aeon/synthesis/api.py`
- 2-file cycle: `aeon/llvm/core.py -> aeon/llvm/llvm_ast.py -> aeon/llvm/core.py`
- 2-file cycle: `aeon/sugar/program.py -> aeon/sugar/stypes.py -> aeon/sugar/program.py`
- 3-file cycle: `aeon/synthesis/api.py -> aeon/synthesis/evaluation_pool.py -> aeon/synthesis/fitness_eval.py -> aeon/synthesis/api.py`

## Hyperedges (group relationships)
- **Historically documented compiler pipeline (ANF stage is stale)** — claude_lark_surface_parser, claude_sugar_ast, claude_desugaring_and_elaboration, claude_core_ast, claude_administrative_normal_form, claude_liquid_typechecking, claude_z3_verification, claude_synthesis_backends, claude_runtime_evaluation [EXTRACTED 1.00]
- **Linear transaction protocol** — docs_database_database_typestate, docs_database_exactly_once_connection_lifecycle, docs_database_conn, docs_database_txn, docs_database_ffi_wrapper_trust_boundary [EXTRACTED 1.00]
- **Refinement-guided synthesis pipeline** — docs_index_typed_program_holes, docs_index_liquid_refinement_types, docs_index_decidable_refinement_fragment, docs_index_example_specifications [INFERRED 0.85]
- **Resource and state contract discipline** — docs_typestate_protocols_linear_refinement_typestate, docs_typestate_protocols_lock_protocol, docs_typestate_protocols_order_checkout_protocol, docs_typestate_protocols_iterator_probe_protocol [INFERRED 0.85]
- **Bounded Contata acceptance** — examples_synthesis_cata_readme_contata_symbolic_version_space, examples_synthesis_cata_contata_readme_bounded_recursive_synthesis_domain, examples_synthesis_cata_readme_observational_value_vector_merging [INFERRED 0.85]

## Communities (250 total, 39 thin omitted)

### Community 0 - "Terms"
Cohesion: 0.04
Nodes (169): _collect_refs(), export_function(), find_binding(), gen(), _gen_application(), _literal_to_python(), _native_import_module(), Export an Aeon function as a stand-alone, pure-Python definition. This backs… (+161 more)

### Community 1 - "Decision Tree Test"
Cohesion: 0.02
Nodes (166): Synthesizes code for multiple functions, one hole per function. When a function…, synthesize_holes(), GESynthesizer, GenomicNGSynthesizer, _knee_point(), _orient(), Any, Metadata (+158 more)

### Community 2 - "Liquid Typechecking Test"
Cohesion: 0.03
Nodes (170): canonicalize_type(), fresh(), core_liquid_equality(), core_type_equality(), Rebuilds a type into a deterministic alpha-equivalence normal form. Every…, Simultaneously rename every variable occurrence in a liquid term. Each…, Equality of liquid terms up to alpha renaming, Equality of types up to alpha renaming (+162 more)

### Community 3 - "Helpers"
Cohesion: 0.04
Nodes (160): bind_lq(), base(), Returns the base type of a Refined Type, Yield failing Horn-style fragments (meaningful when ``entails()`` is false)., constraint_to_parts(), Prepares a constraint into a list of sub-problems for error messages. Yields…, conclusion_variables(), conjunctive_normal_form() (+152 more)

### Community 4 - "Init"
Cohesion: 0.04
Nodes (148): merge_elab_contexts(), format_type(), Format a type for display. Like ``str(stype)`` but strips the leading ``'``…, Peel arrows / refinements / quantifiers off ``stype`` and return the head type-…, return_type_head(), ElaborationTypingContext, Creates a new context with an extra in-scope given constraint., Creates a new context, with an extra variable. (+140 more)

### Community 5 - "Liquidjava Protocols Test"
Cohesion: 0.03
Nodes (145): AeonConfig, AeonDriver, Whether the program declares any ``@property`` functions or ``@example``…, setup_logger(), SilentSynthesisUI, _aeon_sum_source(), main(), _prepare() (+137 more)

### Community 6 - "Tactic Synth Test"
Cohesion: 0.05
Nodes (110): Annotation, Hole, Position → type / scope index for the LSP. The Aeon type checker has no typed…, ABC, Declare the per-candidate computations this backend wants the shared evaluation…, Synthesizer, _mutual_group_ids(), Map each top-level function name to its ``mutual`` group id (``None`` if not… (+102 more)

### Community 7 - "Smt"
Cohesion: 0.03
Nodes (128): t[alpha |-> beta], instantiation., type_variable_instantiation(), rec(), tv(), instantiate_refinement_with_horn_in_liquid(), rec(), Replace LiquidApp(ρ, args) with LiquidHornApplication(horn_name, ...) for…, substitutes name in the term t with the new replacement term rep. (+120 more)

### Community 8 - "Program"
Cohesion: 0.07
Nodes (118): _as_binary_op(), _flatten_app(), format_term(), Flatten left-leaning ``SApplication`` chain into ``(head, [args left-to-…, If ``term`` is ``((op lhs) rhs)`` with ``op`` a known infix operator, return it., Render an STerm for docs using surface spellings and infix operators.…, elaborate_foralls(), go() (+110 more)

### Community 9 - "Typeinfer"
Cohesion: 0.04
Nodes (132): t[alpha := beta], standard substition., type_substitution(), rec(), instantiate_refinement_in_type(), instantiate_refinement_with_horn_in_type(), rec(), predicate_domains(), Inlines the refinement predicate in the type. Implements s[ρ := φ] from… (+124 more)

### Community 10 - "Helpers"
Cohesion: 0.03
Nodes (112): ensure_liqterm(), is_safe_for_application(), liquid_free_vars(), LiquidHole, LiquidLiteralBool, LiquidLiteralFloat, LiquidTerm, mk_liquid_and() (+104 more)

### Community 11 - "Desugar"
Cohesion: 0.03
Nodes (99): clear_import_cache(), get_package_libraries_dir(), Path, Resolve ``import`` paths to ``.ae`` source files and parse them., Clear the import parse cache. Useful for tests and LSP reloads., Return the path to the libraries directory shipped inside the aeon package., Split ``AEONPATH`` using the platform path separator (``;`` on Windows)., Parse a module referenced by ``imp``, using the standard search path. (+91 more)

### Community 12 - "Engine"
Cohesion: 0.04
Nodes (97): Literal, get_core(), A constant value of ``ty``'s base carrier, used to stand in for a not-yet-…, Wrap a concrete scalar value back into a ``Literal`` term (for building a call…, _trivial_stub(), _value_literal(), Translate a synthesized core term to a Z3 expression., _term_to_z3() (+89 more)

### Community 13 - "Decorators"
Cohesion: 0.04
Nodes (106): metadata_update(), metadata_update_by_name(), Any, Metadata, Like ``metadata_update`` but keyed by function name (no ``Definition`` object)., apply_decorators(), collect_core_decorator_queue(), _is_decorator_name_prefix() (+98 more)

### Community 14 - "Pprint Helpers"
Cohesion: 0.05
Nodes (62): concat(), mk_parser(), align_refined_binder(), Rename a top-level refinement binder to ``param`` (and in its predicate). Used…, add_parens_if_needed(), Associativity, format_infix_application(), get_infix_op() (+54 more)

### Community 15 - "Parser"
Cohesion: 0.04
Nodes (6): ensure_list(), Program, Each entry in ``fn_args`` is one of: * a ``(name, type)`` 2-tuple from the…, _split_arg_multiplicities(), TreeToSugar, v_args

### Community 16 - "Synthesis Ui"
Cohesion: 0.04
Nodes (72): EvaluationContext, export_log(), levels_filter(), Disable ANSI coloring on Windows where TERM/VT handling is unreliable., _register_levels(), _stderr_colorize(), LSPProgressUI, _pretty() (+64 more)

### Community 17 - "Runner"
Cohesion: 0.03
Nodes (97): eval(), make_closure(), v(), v(), eval_with_trace(), is_native_import(), is_native_var(), Any (+89 more)

### Community 18 - "Compile"
Cohesion: 0.05
Nodes (85): HoleEvaluationError, Exception, Evaluation reached a synthesis hole in a context that rejects open holes., clear_unit_cache(), _collect_trusted_names(), compile_and_link(), compile_and_link_program(), compile_file() (+77 more)

### Community 19 - "Actions"
Cohesion: 0.04
Nodes (84): refined_to_unrefined_type(), Harmless placeholder body for the hole while building a probe environment. Must…, _synthesis_dummy_literal(), _comparison_operand(), _numeric_literal(), Build a numeric literal with the correct base type (Int or Float)., Build the left-hand side of a tree split comparison., ``<=`` comparison preserving sklearn's floating-point split threshold. (+76 more)

### Community 20 - "Linearity Test"
Cohesion: 0.04
Nodes (82): ErasedUsedAtRuntimeError, _format_location(), LinearBranchMismatchError, LinearUsedTooManyTimesError, A ``1``-bound name was used more than once in its scope, either because it…, A ``0``-bound name was referenced from a runtime position. ``0`` is intended…, The two branches of an ``if`` use a ``1``-bound name a different number of…, Render a ``Location`` as a short, user-readable ``file:line:col`` pair when we… (+74 more)

### Community 21 - "Program"
Cohesion: 0.04
Nodes (54): Multiplicity, Enum, Multiplicities (Quantitative Type Theory). Each binder in Aeon may carry a…, The QTT multiplicity semiring ``{0, 1, ω}`` plus the polymorphism marker ``MN``., Kind, Enum, _get_cached_parser(), MethodDotPostLex (+46 more)

### Community 22 - "Completion"
Cohesion: 0.05
Nodes (80): ExistentialType, Form B: a type wrapped with a list of existential binders. Each binder ``(name,…, has_nontrivial_refinement(), Whether ``t`` carries a refinement that is not trivially ``true`` — i.e. the…, base_type_name(), Completion, compute_completions(), _dedup_latest() (+72 more)

### Community 23 - "Cuda Qtt Test"
Cohesion: 0.05
Nodes (79): LinearityError, LinearUnusedError, Marker base for usage-discipline violations under QTT., A ``1``-bound name was never referenced in its scope., _errors(), _float_array(), _int_array(), _linearity_errors() (+71 more)

### Community 24 - "Grammar Generation"
Cohesion: 0.05
Nodes (74): collect_all_abstractions(), _collect_type_arg_types(), create_abstraction_node(), get_core(), create_abstraction_nodes(), create_application_node(), get_core(), create_application_nodes() (+66 more)

### Community 25 - "Llvm Ast"
Cohesion: 0.05
Nodes (19): LLVMVisitor, Any, LLVMADTConstruct, LLVMADTEliminate, LLVMAlloc, LLVMFoldN, LLVMGetElementPtr, LLVMLoad (+11 more)

### Community 26 - "Learning"
Cohesion: 0.08
Nodes (73): curried(), _fit(), Learning_adaboost_classifier(), Learning_adaboost_regressor(), Learning_ard_regression(), Learning_bagging_classifier(), Learning_bagging_regressor(), Learning_bayesian_ridge() (+65 more)

### Community 27 - "Lta Synth Test"
Cohesion: 0.08
Nodes (67): _operator_names(), Map each DSL operator string to a concrete in-scope **head term**. The…, denot_state(), language(), LTA, nonempty_finals(), The denotation [[q]] = union of denotations of transitions targeting q. Safe on…, [[A]] = union over final states q of [[q]]. (+59 more)

### Community 28 - "Constraints"
Cohesion: 0.06
Nodes (53): A symbol from the LTA alphabet F. The `kind` field discriminates between…, Symbol, AEntail, AEq, AFalse, Atom, atoms_in(), go() (+45 more)

### Community 29 - "Horn"
Cohesion: 0.08
Nodes (61): LiquidHornApplication, entailment(), liquid_admissible(), Project a variable's type onto the base sort it may contribute as a qualifier…, Closed-world validity: same answer as ``check_type(context, term, expected)``…, Solve the lifted VC with an explicit **Q** (and optional typing context for…, extract_qualifier_atoms(), Finite set Q of boolean liquid atoms from refinements in scope. Sources:… (+53 more)

### Community 30 - "Match"
Cohesion: 0.06
Nodes (57): collect_top_level_fn_types(), graft_spine(), Replace the innermost body of ``outer``'s Rec chain with ``inner``., Term, apply_handler(), _constructor_key(), handler_arity(), _known_constructor_key() (+49 more)

### Community 31 - "Match Inductive Test"
Cohesion: 0.08
Nodes (63): bind_ids(), elaborate(), read_file(), bind(), bind_program(), Program, desugar(), DesugaredProgram (+55 more)

### Community 32 - "Converter"
Cohesion: 0.07
Nodes (31): CPULLVMIRGenerator, as_i64(), int_binop(), emit_case(), body(), Any, Function, Value (+23 more)

### Community 33 - "Generate Mbpp"
Cohesion: 0.06
Nodes (61): Attribute, Call, _aeon_type_for(), aggregate_return_kinds(), analyze_strategies(), classify_literal(), CodeGenError, _collect_literals() (+53 more)

### Community 34 - "Lowerer"
Cohesion: 0.09
Nodes (19): LLVMLowerer, ValidationStep, CPULLVMLowerer, low_term(), CUDALLVMLowerer, LLVMAddressSpace, LLVMCast, LLVMPointerType (+11 more)

### Community 35 - "Api"
Cohesion: 0.06
Nodes (28): AeonError, ContractViolationError, CoreTypingRelation, InstanceResolutionError, MethodResolutionError, ModuleNotFoundAeonError, NonOrderableComparisonError, Exception (+20 more)

### Community 36 - "Cata"
Cohesion: 0.07
Nodes (58): _atoms(), _build_ufs(), _concrete_bool(), _concrete_int(), _concrete_list(), _contains_member_app(), ContataResult, _denote() (+50 more)

### Community 37 - "Tdsyn Direction Test"
Cohesion: 0.08
Nodes (54): _goal_for_hole(), The goal type (and typing context) of the hole named ``hole_name``, recovered…, SynthesisNotSuccessful, backward_close_candidates(), Backward close tactic: close the goal with an in-scope variable of the goal's…, Demonstrative single-step expansion using one type-directed action. Applies its…, TDSynOneStepSynthesizer, fresh_hole() (+46 more)

### Community 38 - "Synthesizerfactory"
Cohesion: 0.05
Nodes (49): InfoViewData, A JSON-serialisable payload for the ``aeon/infoView`` response., AeonLanguageServer, code_action(), _emit_synthesis_pending(), Push an immediate ``aeon/synthesisProgress`` update so the info view can show…, Exception, Raised when ``-s``/``--synthesizer`` names a backend that does not exist. (+41 more)

### Community 39 - "Linear Random Test"
Cohesion: 0.05
Nodes (48): A refinement predicate uses syntax outside the decidable fragment Liquid Types…, UndecidableRefinementError, Render the program's trusted computing base — the axioms and refined ``native``…, Check every ``@property`` function and ``@example`` assertion, print a pytest-…, A human-readable rendering of a :class:`TrustReport`., render_report(), Property-based testing for Aeon (issue #37). Random inputs for ``@property``…, ExampleResult (+40 more)

### Community 40 - "Mutual Synthesis Test"
Cohesion: 0.05
Nodes (49): _partition_targets(), Split synthesis targets into independent ones and mutual groups (2+ members…, _contata_version_space_group(), _cosynthesize_group(), _joint_accepts(), Metadata, Contata Algorithm 2, lines 11-13: does the *joint* candidate assignment satisfy…, Try the ``contata`` version space over the whole mutual group, returning a… (+41 more)

### Community 41 - "Lsp Infoview Test"
Cohesion: 0.07
Nodes (57): compute_info_view(), _dedup_innermost(), _entries_to_vars(), _hole_at(), InfoEntry, _is_hidden(), One context line: ``name : type`` with an optional refinement predicate…, The turnstile target: a hole's goal type, or the type of the expression under… (+49 more)

### Community 42 - "Generators"
Cohesion: 0.06
Nodes (42): merge_typing_contexts(), create_grammar(), _determinize_grammar(), propagate_constants(), Metadata, Sort a grammar's productions and their alternatives by class name so the seed…, remove_uninterpreted_functions(), remove_uninterpreted_functions_wrap() (+34 more)

### Community 43 - "Resource Meters"
Cohesion: 0.06
Nodes (45): InvalidIndividualException, Raised by an evaluator when a candidate has no well-defined fitness (e.g. its…, Goal, NamedTuple, _cluster_function(), make_evaluators(), _make_fitness(), fitness() (+37 more)

### Community 44 - "Ortools Cpsat Test"
Cohesion: 0.08
Nodes (50): ProgramSynthesizer, A synthesizer that fills *all* of a program's holes jointly. The per-hole…, SynthesisError, FloatHoleNGSynthesizer, decode(), evaluate(), fill(), _is_array_float() (+42 more)

### Community 45 - "Parser"
Cohesion: 0.05
Nodes (24): from_token(), Parse a surface-syntax multiplicity token (``"0"``, ``"1"``, ``"omega"`` /…, Returns a type variable that does not exist in context., Returns the type of the variable name. Looks up regular bindings and…, Resolve a method call ``receiver.method`` (issue #27) to the bound ``Name`` of…, Concrete term variables in scope, honoring lexical shadowing. ``with_var``…, Name, is_synthesized_name() (+16 more)

### Community 46 - "Counterexample Test"
Cohesion: 0.07
Nodes (46): LiquidTypeCheckingFailedRelation, A concrete assignment that falsifies this verification condition, rendered as…, model_for_invalid(), When ``constraint`` is *invalid*, return a witnessing counterexample as a list…, Render the counterexample for an invalid ``constraint`` as a compact ``name =…, render_counterexample(), _errors(), _liquid_failure() (+38 more)

### Community 47 - "Linear Type Test"
Cohesion: 0.07
Nodes (47): LinearTypeNotBoundLinearlyError, A binder whose type is declared ``linear type`` was not bound at multiplicity…, _linearity_errors(), _parse(), Linearity-discipline tests for the QTT Array library. ``Array`` is a ``linear…, ``copy`` consumes the single linear reference once and hands back an…, ``copy`` is refinement-parametric: the element predicate rides on ``a<p>``…, Each ``let 1`` handle is consumed once by the next op. (+39 more)

### Community 48 - "Synthesizer"
Cohesion: 0.08
Nodes (29): Metric program synthesis backend (SyMetric), registered as ``symetric``., base_key(), Component, _decompose(), Random, A library function/constructor usable to build candidate terms. ``type_args``…, Instantiate polymorphic constructors against the demanded ground parametric…, The head term of a component: its ``Var`` wrapped in the leading… (+21 more)

### Community 49 - "Executor"
Cohesion: 0.08
Nodes (14): LLVMExecutionEngine, LLVMIRGenerator, LLVMOptimizer, LLVMPipeline, ABC, CUDAExecutionEngine, CUDAExecutionError, Any (+6 more)

### Community 50 - "Automaton"
Cohesion: 0.06
Nodes (27): _build_term_from_trans(), go(), _denot_state(), go(), Core LTA data structures (Definition 2 of the paper). A Liquid Tree Automaton…, A constrained tree-automaton transition f(q_1,...,q_n) -ψ→ q., All transitions whose target is q., Maximum nesting depth of the automaton, defined as the longest chain of… (+19 more)

### Community 51 - "Llvm Ast"
Cohesion: 0.14
Nodes (26): LLVMIRGenerationError, LLVMCall, LLVMCharType, LLVMFunction, LLVMFunctionType, LLVMIf, LLVMLet, LLVMLiteral (+18 more)

### Community 52 - "Generator"
Cohesion: 0.08
Nodes (40): ConstructorDoc, DocComment, extract_comments(), extract_documentation(), find_header_comment(), find_preceding_comment(), format_decorator(), format_name() (+32 more)

### Community 53 - "Adt"
Cohesion: 0.11
Nodes (30): ADTConstructorInfo, constructor_order(), core_type_to_llvm(), finalize_constructor(), is_registered_inductive(), lookup_constructor(), lookup_recursor(), Tagged-heap layouts for immutable inductive types in the CPU LLVM backend.… (+22 more)

### Community 54 - "Termination Test"
Cohesion: 0.13
Nodes (41): _inner_ctx_and_expect_ty(), _liquefy_metric_at(), peel_abstractions(), peel_application_chain(), peel_type_formal_names(), r"""Lexicographic ``m(call) <* m(entry)`` (Liquid Haskell ``/ [m0, m1, …]``).…, Left-associate application: ((f x) y) -> f, [x, y]., substitute_formals() (+33 more)

### Community 55 - "Gpu Module Test"
Cohesion: 0.13
Nodes (36): build_typing_context(), ElabTypeDecl, ElabTypeVarBinder, ElabTypingContextEntry, ElabUninterpretedBinder, ElabVariableBinder, ABC, Creates a new context, with an extra type variable (+28 more)

### Community 56 - "Converter"
Cohesion: 0.12
Nodes (14): Backend, LLVMBackendError, Exception, CUDALLVMIRGenerator, Function, Value, LLVMVectorReduce, MultiBackendPipeline (+6 more)

### Community 57 - "Lsp Features Test"
Cohesion: 0.09
Nodes (30): build_type_index(), Build a :class:`TypeIndex` for ``core`` checked under ``typing_ctx``. Wraps…, analyse(), by_label(), col_after(), _half_for_receiver(), labels(), _MockDocument (+22 more)

### Community 59 - "Learning"
Cohesion: 0.07
Nodes (37): _as_xy(), _counts(), _impute(), Learning_class_counts(), Learning_copy(), Learning_create_dataset(), Learning_fill_constant(), Learning_fill_mean() (+29 more)

### Community 60 - "Image"
Cohesion: 0.07
Nodes (35): Image_blur(), Image_copy(), Image_crop(), Image_diff(), Image_diff_mse(), Image_draw_rectangle(), Image_fit_inside(), Image_fitness_bw_mse() (+27 more)

### Community 61 - "Vcs"
Cohesion: 0.11
Nodes (29): eval_liquid(), eval_liquid_bool(), Any, Exception, Runtime evaluation of liquid refinement predicates (issue #443). Deliberately…, Predicate fragment outside the runtime interpreter., Predicate mentions an uninterpreted symbol — no runtime meaning., UnevaluableLiquid (+21 more)

### Community 62 - "Infer Test"
Cohesion: 0.11
Nodes (34): test_a_is_bool(), test_a_is_not_bool(), test_abs(), test_abs_f(), test_abs_if(), test_abs_is_int(), test_and(), test_capture_avoiding_subs() (+26 more)

### Community 63 - "Aeon Adapter"
Cohesion: 0.10
Nodes (33): cache_driver_analysis(), clear_cache(), get_core(), get_errors(), get_metadata(), get_type_index(), get_typing_ctx(), _open() (+25 more)

### Community 64 - "Smt Sets Test"
Cohesion: 0.11
Nodes (31): Verifies if a constraint is true using Z3., smt_valid(), implication_constraint(), _imp(), Tests for SMT Set support in refinement types., Set_sub(a, Set_empty) should NOT be valid for all a., Set_sub(Set_empty, s) is always true., Set_mem(x, Set_sng(x)) is always true. (+23 more)

### Community 65 - "Lsp Test"
Cohesion: 0.12
Nodes (29): find_holes_in_source(), HolePosition, ParseResult, Find all ?identifier holes in source text and return their LSP ranges…, CodeAction, _build_code_actions(), make_range(), Range (+21 more)

### Community 66 - "Ortools Cpsat"
Cohesion: 0.15
Nodes (19): _array_elem(), _base(), CPSatHoleSynthesizer, _is_float(), _is_int(), _Num, Any, Exception (+11 more)

### Community 67 - "End To End Test"
Cohesion: 0.09
Nodes (30): check_compile_core(), check_compile_expr(), Any, When the Horn solver fails, the error location should point to the AST element…, Calling a function that expects negative with a positive argument fails; error…, `-->` is registered in the prelude, so refinements may use logical implication…, The implication is genuinely enforced by the verifier: an implementation that…, `!a || b` parses as `(!a) || b`, not `!(a || b)` — the `!` operator is no… (+22 more)

### Community 68 - "Readme"
Cohesion: 0.09
Nodes (24): Example runner, Precommit and mypy checks, Pytest suite, Python version matrix CI, Formatting lint and metadata gates, Dafny to Aeon subset translation, Historical undefined arithmetic gap, Integer boolean single-return subset (+16 more)

### Community 69 - "Cuda"
Cohesion: 0.11
Nodes (26): Declared CSV metadata, Linear ML resource ownership, Logical split provenance, Native boundary assumptions, Training and testing typestate, Array size measure, Element predicate parameter, Linear Array (+18 more)

### Community 70 - "Cuda"
Cohesion: 0.09
Nodes (21): as_read_only(), Buffer, buffer_access(), buffer_bytes(), buffer_device(), buffer_elem_size(), buffer_extent_x(), buffer_extent_y() (+13 more)

### Community 71 - "Synthesizer"
Cohesion: 0.12
Nodes (19): make_skip_fn(), skip(), Metadata, Create a skip function for use in actions., Check if a variable should be skipped during synthesis., should_skip(), _get_elapsed_time(), _is_better() (+11 more)

### Community 72 - "Trust"
Cohesion: 0.12
Nodes (25): _application_head(), _clean(), compute_trust_report(), _Def, _display_name(), _format_loc(), _is_native_import(), _native_code() (+17 more)

### Community 73 - "Knee Point Selection Test"
Cohesion: 0.16
Nodes (25): _knee_point_individual(), Pick a single individual from a Pareto front by knee-point (compromise)…, Fitness, Individual, _ind(), _invalid(), _make_problem(), Any (+17 more)

### Community 74 - "Synthesizer"
Cohesion: 0.10
Nodes (23): _close(), _example_literal(), _example_rows(), consider(), add_bank(), insert_goal(), matches_examples(), obs_default() (+15 more)

### Community 75 - "Machine Learning"
Cohesion: 0.10
Nodes (24): Open ``path``, run a read-only ``sql`` query, and return all rows. The…, read_all(), accuracy(), consume_split(), decision_tree_classifier(), _preprocessor(), Any, DataFrame (+16 more)

### Community 76 - "Cuda"
Cohesion: 0.17
Nodes (22): _coerce_values(), Device, device_id(), Launch2D, launch2d_device(), launch2d_height(), launch2d_threads_x(), launch2d_threads_y() (+14 more)

### Community 77 - "Forkjoin"
Cohesion: 0.13
Nodes (26): fork(), fork2(), halves(), invoke(), join(), join2(), join_timeout(), par_map() (+18 more)

### Community 78 - "Synthesizer"
Cohesion: 0.10
Nodes (19): has_io_examples(), AFTASynthesizer, consider(), active_atoms(), add_bank(), cegar_pass(), obs_value(), signature() (+11 more)

### Community 79 - "Optimization Test"
Cohesion: 0.13
Nodes (26): clear_constructor_registry(), _constructor_registry(), eq(), fixture, test_opt_and(), test_opt_app(), test_opt_destructor_let_known_value(), test_opt_destructor_let_variable_scrutinee() (+18 more)

### Community 80 - "Method Call Test"
Cohesion: 0.09
Nodes (23): check_compile(), test_liquid_types_syntax(), End-to-end: destructuring a single-constructor value binds its fields., The destructuring let supports constructors with more than two fields., test_let_pattern_evaluates_single_constructor(), test_let_pattern_three_arguments(), ``x.method`` for a local variable ``x`` (lexed as a single QUALIFIED_ID) is…, test_chained_method_call_on_variable_receiver() (+15 more)

### Community 81 - "Parametric Refinements Test"
Cohesion: 0.07
Nodes (26): An argument that is not known to satisfy an uninterpreted-predicate…, When the argument is known to satisfy the uninterpreted predicate (its type…, A refinement mentioning a regular defined boolean function is a checked…, Regression for issue #209: explicit `forall <p>` in signature with no matching…, Chained id cannot justify a strictly stronger predicate., forall <p> on Int only; no type polymorphism; Λ < p => in term., test_const_propagates_first_argument_predicate(), test_const_rejects_wrong_predicate_on_result() (+18 more)

### Community 82 - "Dataframe"
Cohesion: 0.15
Nodes (25): assign_const(), col_as_array(), columns(), copy_df(), drop(), dropna(), fillna(), filter_rows() (+17 more)

### Community 83 - "Init"
Cohesion: 0.12
Nodes (20): default_openai_model(), generate(), llm_provider(), Unified LLM text generation for synthesis (Ollama or OpenAI-compatible)., Return ``ollama`` or ``openai`` for the active LLM backend., get_elapsed_time(), is_better(), llm_synthesizer_label() (+12 more)

### Community 84 - "Multiplicity Test"
Cohesion: 0.12
Nodes (20): add(), mul(), Semiring multiplication — combine an outer multiplicity with an inner one (e.g.…, Semiring addition — combine usage tallies in a single scope. ``0`` is the…, _find_rec(), Phase 1 — Multiplicity (QTT) infrastructure. These tests verify: 1. The…, A `(1 x: T)` parameter survives desugaring + lowering and ends up on the…, Existing-style programs (no multiplicity) lower to ``MOmega`` — no behavioural… (+12 more)

### Community 85 - "Exit Codes Test"
Cohesion: 0.16
Nodes (23): ExitCode, IntEnum, True when ``-s``/``--synthesizer`` or ``--budget`` was passed on the CLI., synthesis_requested(), CompletedProcess, _make(), parametrize, Path (+15 more)

### Community 86 - "Csg Metric"
Cohesion: 0.14
Nodes (22): bitmap(), contains(), jaccard_distance(), Native (PIL) helpers for the Aeon inverse-CSG benchmark. Implements the…, Pixel-difference (Hamming) distance between e's bitmap and the target. This is…, The paper's distance metric (Sec. 4.2): Jaccard distance between bitmaps.…, Is scaled-space pixel (x, y) inside CSG shape e? (Fig. 4 of the paper.), Rasterise e to a list of rows (y flipped to match the tool's display). (+14 more)

### Community 87 - "Python Export Test"
Cohesion: 0.17
Nodes (22): Exception, PythonExportError, Raised when a term cannot be translated to pure Python., _driver(), _eval_with_python(), _export(), parametrize, Export ``fun_name`` to Python, exec it, and apply it (curried) to ``args``. (+14 more)

### Community 88 - "Decidability"
Cohesion: 0.13
Nodes (16): LiquidLiteralUnit, The single inhabitant of the Unit type. Distinct from LiquidLiteralBool(True):…, FileLocation, _classify(), collect_decidability_warnings(), DecidabilityWarning, _location_in_file(), _mentions_variable() (+8 more)

### Community 89 - "Server"
Cohesion: 0.09
Nodes (16): definition(), document_symbol(), hover(), info_view(), inlay_hint(), lsp_completion(), _get_word_at_position(), Diagnostic (+8 more)

### Community 90 - "Lsp Test"
Cohesion: 0.16
Nodes (21): execute_synthesize(), Blocking synthesis function, meant to run in a thread executor., _run_synthesis(), ShowMessageParams, _FakeOllamaResponse, ls(), make_driver(), MockLS (+13 more)

### Community 91 - "Lta Synth Test"
Cohesion: 0.11
Nodes (18): Liquid Tree Automata (LTA) for refinement-typed Component-Based Synthesis.…, LTASynthesizer, Component-based synthesis via Liquid Tree Automata. The algorithm iteratively…, The simplest end-to-end use: with a variable in context that already has the…, With a polymorphic identity function in Γ, synthesis still succeeds — the…, test_factory_returns_lta(), test_lta_synthesizer_picks_existing_var(), evaluate() (+10 more)

### Community 92 - "Sygus Synthesis Test"
Cohesion: 0.13
Nodes (21): SyGuS-based synthesis backend (issue #49). Synthesises code for a *valid…, Synthesize base-type holes (Int/Bool/Float) by reduction to SyGuS, solved by…, SygusSynthesizer, eval_int_program(), Tests for the SyGuS synthesis backend (issue #49). Covers the three phases:…, A plain ``Int`` hole should synthesize some well-typed integer., Substitute ``term`` for the hole and evaluate the defined value., ``{x:Int | x == 35}`` should synthesize the constant 35. (+13 more)

### Community 93 - "Cuda Binding Test"
Cohesion: 0.13
Nodes (14): _cuda_device_or_skip(), cuda_module(), fixture, test_as_read_only_transfers_ownership(), test_device_close_cleans_live_buffers_and_pending_results(), test_device_reports_max_allocation(), test_device_reports_warp_and_shared_limits(), test_download_failure_releases_buffer() (+6 more)

### Community 94 - "Openai Client Test"
Cohesion: 0.15
Nodes (15): generate(), openai_api_key(), openai_base_url(), OpenAI-compatible ``/v1/chat/completions`` client for LLM synthesis., Generate text from an OpenAI-compatible chat/completions endpoint., _timeout_seconds(), _FakeHTTPResponse, Tests for the OpenAI-compatible LLM client. (+7 more)

### Community 95 - "Linearity"
Cohesion: 0.16
Nodes (19): _add_usage(), _alias_project(), _branch_merge(), _check_binder(), _counts_add(), _counts_scale(), _drop(), _Mismatch (+11 more)

### Community 96 - "Mutual Recursion Test"
Cohesion: 0.14
Nodes (21): _driver(), End-to-end tests for Lean-style ``mutual ... end`` recursion. A ``mutual``…, A member's refinement may apply a sibling to its result binder (``{r | g r =…, The reflected sibling is precise, not vacuous: a relational spec that does not…, A non-terminating group must not be able to inhabit ``{b:Bool | false}``: the…, A ``mutual`` group whose members range over *distinct* ADT sorts reflects into…, Compile + evaluate. Returns (errors, result) where result is None on error., A group of three functions cycling through one another. (+13 more)

### Community 97 - "Polymorphic Uninterpreted Test"
Cohesion: 0.09
Nodes (21): Regression tests for polymorphic uninterpreted functions reaching the SMT…, ``Pair_mk_fst`` (auto-generated from ``inductive Pair a b | mk``) can be used…, Different parametric ``Pair`` instantiations get distinct sorts., Two variables of ``Pair Dataset Dataset`` share a Z3 sort, so a premise ``q =…, A polymorphic uninterpreted function specialises to ``Int`` correctly., ``Pair_mk_fst p = Pair_mk_fst q`` is *not* assumed for two independent ``Pair…, A multi-argument *monomorphic* uninterpreted function reaches the SMT layer and…, Without a premise linking the arguments, the solver must *not* assume ``add2 a… (+13 more)

### Community 98 - "Cuda"
Cohesion: 0.11
Nodes (18): check_ok(), CUDAError, CUDAStateError, discard_status(), Like :func:`synchronize`, but returns a :class:`StatusBuffer` instead of…, Base exception for CUDA binding failures., Raised when a consumed CUDA handle is used again., Linear-style completion token that must be checked or discarded. (+10 more)

### Community 99 - "Ollama Manager"
Cohesion: 0.16
Nodes (20): _estimate_vram_bytes(), free_memory_for(), _installed_model_names(), is_model_installed(), _memory_budget_bytes(), _ollama_base_url(), _ollama_model_tag(), prepare_ollama_model() (+12 more)

### Community 100 - "Claude"
Cohesion: 0.17
Nodes (18): Administrative normal form (historical; stale documentation), AeonConfig, AeonDriver, Compiler pipeline, Core AST, Desugaring and elaboration, Import search order, Language server protocol (+10 more)

### Community 101 - "Contracts"
Cohesion: 0.22
Nodes (15): build_runtime_liquid_env(), _check_liquid(), check_param_refinement(), check_return_refinement(), ContractClosure, ContractState, Any, Runtime refinement contracts with Findler–Felleisen blame (issue #443). (+7 more)

### Community 102 - "Refined Subtyping Test"
Cohesion: 0.19
Nodes (19): atom(), build_grammar_and_lits(), conj(), literal_for(), Tests for refined-type subtyping in the synthesis grammar (issue #312). Stage 1…, Two intervals denoting the same set collapse to one class, no crash. Over the…, A 3-element chain together with an incomparable refinement must build without…, Without a context, no chain is built: refined classes stay attached to the base… (+11 more)

### Community 103 - "Nn"
Cohesion: 0.15
Nodes (18): _activation_deriv(), _backprop(), _forward_cache(), NN_backward(), NN_backward_ce(), NN_sgd_step(), NN_softmax_cross_entropy(), Any (+10 more)

### Community 104 - "Navigation"
Cohesion: 0.20
Nodes (17): build_def_index(), DefIndex, document_symbols(), inlay_hints(), InlayInfo, iter_terms(), _name_range_after(), Structural LSP features over the core AST: document symbols, go-to-definition… (+9 more)

### Community 105 - "Typeindex"
Cohesion: 0.14
Nodes (12): recording_synth(), _contains(), One node's synthesized type and the context it was synthesized in., A comparable size of the range: (line span, column span). Smaller is tighter,…, Whether the (1-indexed) position ``(line1, col1)`` lies within ``obs``'s half-…, Queryable index of per-node types and scopes for a single document., The smallest observation whose range contains the 0-indexed cursor. Ties…, The inferred type (and its source range) of the tightest expression containing… (+4 more)

### Community 106 - "Division By Zero Test"
Cohesion: 0.19
Nodes (18): Division and modulo by a statically-zero divisor must be rejected. The divisor…, _rejected(), _run(), test_both_operands_literal_division_rejected(), test_both_operands_literal_modulo_rejected(), test_float_division_by_literal_ok(), test_float_division_guarded_ok(), test_float_division_refinement_bounded_ok() (+10 more)

### Community 107 - "Cuda"
Cohesion: 0.18
Nodes (9): _CudaDriver, CUDAUnavailableError, _driver(), num_devices(), Any, Raised when the CUDA driver or a CUDA device is unavailable., Typed, lazy facade over ``libcuda``., Return the number of visible CUDA devices without creating a context. (+1 more)

### Community 108 - "Executor"
Cohesion: 0.27
Nodes (6): CPULLVMExecutionEngine, vector_get(), vector_set(), LLVMExecutionError, Any, Best-effort size for pointer results: last Int arg, else len of first list.

### Community 109 - "Synthesizer"
Cohesion: 0.16
Nodes (13): validate(), create_problem(), fitness_fun(), single_fitness_fun(), Metadata, _goals_for(), Metadata, Read the minimize/maximize goals for this hole, robust to Name identity. (+5 more)

### Community 110 - "Synthesizers"
Cohesion: 0.14
Nodes (16): Genetic programming, Hill climbing, Interactive backward steps, Interactive forward steps, Liquid Tree Automata, Local LLM candidate generation, Modular verification conditions, One-plus-one evolution (+8 more)

### Community 111 - "Sygus To Aeon"
Cohesion: 0.21
Nodes (16): Benchmark, _collect_symbols(), convert_file(), extract(), is_call_to(), main(), parse(), parse_all() (+8 more)

### Community 112 - "Learning Test"
Cohesion: 0.21
Nodes (17): Type-level pipeline-defect detection in the ``Learning`` library. These tests…, test_clean_accuracy_discharged_by_balanced_witness(), test_clean_csv_path_with_require_enough_data(), test_clean_imbalanced_with_f1(), test_clean_regression_per_split_imputation(), test_clean_static_pipeline(), test_clean_timeseries_unshuffled_with_f1(), test_defect_balance_sensitive_metric_on_imbalanced() (+9 more)

### Community 113 - "String Escapes Test"
Cohesion: 0.20
Nodes (17): _core(), String-literal escape handling in both parsers (sugar and core). These tests…, _sugar(), test_core_backslash_escape(), test_core_newline_escape(), test_core_plain(), test_core_quote_escape(), test_core_tab_escape() (+9 more)

### Community 114 - "Selfification Test"
Cohesion: 0.18
Nodes (16): lower_to_core(), Converts Surface terms into Core terms., Selfification of application results and let-fact hypotheses in termination…, A refined native's contract flows through a let binder — no axiom needed., An axiom instantiated in one branch must not justify the other branch's…, Full front-to-typecheck pipeline; True iff no type errors (mirrors…, ``fdiv n 10`` as an argument satisfies ``{v | v >= 0}`` via the axiom., The let-bound result stays connected to the call through selfification. (+8 more)

### Community 115 - "Translate"
Cohesion: 0.21
Nodes (16): _first_violated(), The index of the coarsest pool predicate not yet active that ``vec`` violates…, _aeon_args(), emit_aeon(), _extract_clauses(), _needed_builtins(), _parse_args(), parse_dafny() (+8 more)

### Community 116 - "Ollama Manager Test"
Cohesion: 0.17
Nodes (15): _auto_pull_enabled(), pull_model(), Download ``model`` if it is not already present locally., Tests for automated Ollama model pull and memory release., ollama-python list entries expose ``model`` but not ``name``., _running(), test_free_memory_unloads_other_running_models(), test_free_memory_unloads_target_when_over_budget() (+7 more)

### Community 117 - "Cuda"
Cohesion: 0.14
Nodes (13): Launch1D, launch_1d(), launch_1d_warped(), launch_device(), launch_grid_size(), launch_items(), launch_threads(), Validated one-dimensional launch geometry. (+5 more)

### Community 118 - "Cuda"
Cohesion: 0.13
Nodes (7): close(), create_stream(), default_stream(), A CUDA stream owned by one device context., Stream, stream_device(), stream_id()

### Community 119 - "Gpu"
Cohesion: 0.15
Nodes (10): gpu_dot(), gpu_filter(), gpu_imap(), gpu_map(), run(), gpu_reduce(), with_initial(), gpu_run() (+2 more)

### Community 120 - "Reflect Underscore Test"
Cohesion: 0.23
Nodes (15): Expand the ``_`` reflection marker in each definition's return-type refinement.…, reflect_underscore_in_definitions(), test_explicit_recurrence_with_metric_verifies(), test_if_body_reflects_and_verifies(), test_if_body_wrong_claim_fails(), test_marker_composes_with_manual_refinement(), test_native_body_rejected_with_clear_error(), test_no_marker_is_left_unchanged() (+7 more)

### Community 121 - "Typeclass Test"
Cohesion: 0.23
Nodes (15): _errors(), _run(), test_constrained_instance_dispatches_inner_eq(), test_constrained_instance_resolves(), test_default_method_false_branch(), test_default_method_overridden(), test_default_method_used(), test_instance_dict_typechecks_standalone() (+7 more)

### Community 122 - "Wellformed Test"
Cohesion: 0.30
Nodes (14): apply_subs_name(), bind_ctx(), bind_type(), check_name(), get_last_id(), RenamingSubstitions, built_std_context(), test_dependent() (+6 more)

### Community 123 - "Driver"
Cohesion: 0.15
Nodes (5): Any, Return a stand-alone, pure-Python definition of ``fun_name``. Synthesis runs…, Synthesize the open holes, substitute the results into ``self.core`` in place,…, measure(), RecordTime

### Community 124 - "Entrypoint"
Cohesion: 0.24
Nodes (15): ErrorInSynthesis, TimeoutInEvaluationException, make_evaluator(), evaluate(), evaluate_individual(), _pool_backed(), evaluate(), output_value() (+7 more)

### Community 125 - "Refinements"
Cohesion: 0.29
Nodes (14): conditional_to_interval(), flatten_conditions(), Any, sympy_exp_to_bounded_interval(), contains_tuples(), get_metahandler_union(), interval_to_metahandler(), intervals_to_metahandlers() (+6 more)

### Community 126 - "Cosynthesis"
Cohesion: 0.18
Nodes (13): Any, Encode a concrete scalar/opaque value as a z3 term. Ints/bools/reals map to…, Contata's joint validity check + unsat-core refinement (Algorithm 2, lines…, _smt_unsat_core_obligations(), app(), func_of(), ir_to_z3(), _z3_sort() (+5 more)

### Community 127 - "Benchmark"
Cohesion: 0.24
Nodes (14): _cell(), _extract_quality(), _extract_solution(), main(), parse_args(), _print_table(), Path, Run synthesis for *example* with *synthesizer* and return a Result. (+6 more)

### Community 128 - "Index"
Cohesion: 0.19
Nodes (13): Aeon, Compiler exit categories, Example specifications, Inductive measures, Liquid refinement types, Multiobjective synthesis, Property-based synthesis corpus, Refinement polymorphism (+5 more)

### Community 129 - "Readme"
Cohesion: 0.20
Nodes (12): Bounded recursive synthesis domain, Checked reference solutions, Contata thirty-benchmark artifact, Non-path-sensitive match, CATA refinement enumeration, Contata symbolic version space, Observational value-vector merging, Relational counterexample guidance (+4 more)

### Community 130 - "Bench Llvm Quicksort"
Cohesion: 0.24
Nodes (6): RuntimeError, CPU, CUDA, main(), optimize(), Benchmark emitted quicksort IR; requires only llvmlite, plus CUDA for --gpu.…

### Community 131 - "Bench Horn"
Cohesion: 0.16
Nodes (7): Any, Tracks best valid fitness and valid/total counts across a run., RecordingUI, main(), Compare Horn optimizations against a Git revision, using isolated processes.…, worker(), valid()

### Community 132 - "Afta Pbe Test"
Cohesion: 0.19
Nodes (14): Tests for the AFTA backend's example-driven (PBE) mode. These exercise the…, A single Fig.17 operator: flip a matrix left-right., First three characters of a phone number -- the canonical SyGuS string…, When every output equals its input, the smallest consistent program is the…, Integer PBE over polymorphic arithmetic: double an input., A single-component transformation: upper-case the input., The paper's running matrix example: reshape a length-6 vector row-wise into a…, _run_pbe() (+6 more)

### Community 133 - "Api"
Cohesion: 0.29
Nodes (8): check(), parse(), Program, Any, Path, Small public Python API for Aeon. Implementation modules remain private…, Parse, elaborate, and type-check source text or an ``.ae`` file., synthesize()

### Community 134 - "Run"
Cohesion: 0.25
Nodes (13): fetch_dafny(), load_task_ids(), main(), Path, Harness: translate Vericoding Dafny tasks to Aeon and run synthesis. Fetches…, Return the Dafny source for `task_id`, fetching and caching if needed., Fetch + translate `task_id`, writing the .ae file to the cache. Returns the…, Invoke aeon synthesis. Returns (success, solution-or-error, elapsed). (+5 more)

### Community 135 - "Ffi"
Cohesion: 0.22
Nodes (12): Curried Python helpers, native, Native axiom no-ops, native_import, Opaque types, Python FFI trust boundary, Runtime refinement witnesses, Uninterpreted measures (+4 more)

### Community 136 - "Recursion Soundness Test"
Cohesion: 0.25
Nodes (13): Soundness tests for well-founded recursion. A recursive definition is typed…, Run the full front-to-typecheck pipeline; True iff no type errors. Mirrors the…, test_absurd_refinement_on_nonterminating_fn_is_rejected(), test_explicit_recurrence_refinement_is_accepted(), test_factorial_nat_is_accepted(), test_inferred_metric_guarded_nat_is_accepted(), test_multiarg_no_metric_absurd_refinement_is_rejected(), test_multiarg_recurrence_with_explicit_metric_is_accepted() (+5 more)

### Community 137 - "Cuda"
Cohesion: 0.19
Nodes (7): discard(), Pending, pending_device(), pending_size(), pending_stream(), The not-yet-synchronized output of an asynchronous kernel launch., synchronize()

### Community 139 - "Evaluation Pool"
Cohesion: 0.24
Nodes (6): EvaluationPool, Any, Run every computation on each term, returning one ``{name: (status, value)}``…, Fills the hole in the whole program with a given term. Exposed so a backend can…, _Worker, _worker_main()

### Community 140 - "Forkjoin"
Cohesion: 0.19
Nodes (10): ForkJoin, Linear Future and Pool, Nonempty exhaustive halves, Size-preserving parallel map, Exit-code refinement, Linear Process lifecycle, OS refinement bindings, Subprocess argv-only binding (+2 more)

### Community 141 - "Csg Benchmark Test"
Cohesion: 0.17
Nodes (10): parametrize, Validation for the inverse-CSG benchmark suite (examples/synthesis/csg/).…, The original program renders to exactly the committed target bitmap. Both the…, The generated .ae benchmark parses and type-checks (no synthesis)., The Jaccard metric is in (0, 1] for a shape that is not the target., Run one benchmark through Aeon synthesis to exercise the FFI metric., test_benchmark_parses_and_typechecks(), test_jaccard_discriminates() (+2 more)

### Community 142 - "Learning"
Cohesion: 0.17
Nodes (12): Learning_accuracy(), Learning_classification_report(), Learning_confusion_matrix(), Learning_explained_variance(), Learning_f1(), Learning_mae(), Learning_mse(), Learning_precision() (+4 more)

### Community 143 - "Linear Distributions Test"
Cohesion: 0.23
Nodes (10): poisson_batch(), Any, Batch sampling helpers for ``libraries/Distributions.ae`` (issue #441). All…, Draw *n* Poisson(*lam*) samples using Knuth's algorithm on *g*., Linear batch samplers in ``libraries/Distributions.ae`` (issue #441 follow-up)., test_distribution_chain_typechecks(), test_extracted_successor_must_be_used(), test_poisson_batch_helper() (+2 more)

### Community 144 - "Pbe"
Cohesion: 0.26
Nodes (11): _length(), Any, Bottom-up PBE synthesis with predicate-abstraction CEGAR over the example…, synthesize_pbe(), abstractly_accepting(), add_bank(), cegar_pass(), len_pred() (+3 more)

### Community 145 - "Synthesizer"
Cohesion: 0.23
Nodes (10): register(), add_bank(), consider(), step(), wrap(), _peel_abstractions(), Metadata, Index in-scope bindings as builders (the ranked alphabet, including recursive… (+2 more)

### Community 146 - "Exp2 Metric"
Cohesion: 0.26
Nodes (11): _dense_grid(), max_rel_error(), mre_dense(), pade_mre(), pade_mre_dense(), Native helpers for the Aeon AutoNumerics-Zero exp2 example. Implements the…, Dense-grid error of the Pade skeleton, for reporting., Maximum relative error of candidate ``func`` against 2**x on (0, 1]. ``func``… (+3 more)

### Community 147 - "Grover Sim"
Cohesion: 0.23
Nodes (11): _apply_single(), fitness(), _gate_list(), p_target(), Native helpers for the Aeon HUMIES-2026 Grover-circuit benchmark. Noiseless…, Exact probability of measuring basis state `target` (0..7)., The paper's minimised scalar fitness (0.12 is optimal for target 0)., Apply a 2x2 unitary m to qubit q of the 8-amplitude state, in place. (+3 more)

### Community 148 - "Collection Literal Test"
Cohesion: 0.30
Nodes (11): Lean-style collection literals: ``[1, 2, 3]`` (List) and ``#[1, 2, 3]``…, _run(), test_array_literal_length(), test_empty_array_literal(), test_empty_list_literal_with_annotation(), test_list_literal_as_spaced_application_argument(), test_list_literal_elements_evaluate(), test_list_literal_length() (+3 more)

### Community 149 - "Llvm E2E Test"
Cohesion: 0.30
Nodes (11): compile_and_run(), skip, test_e2e_llvm_array_count(), test_e2e_llvm_array_filter(), test_e2e_llvm_array_map(), test_e2e_llvm_array_sum(), test_e2e_llvm_array_zip_with(), test_e2e_llvm_fibonacci() (+3 more)

### Community 150 - "Z3 Errors"
Cohesion: 0.29
Nodes (10): Helpers for surfacing Z3 failures in LSP diagnostics and messages., Return a readable message from a :class:`Z3Exception`. Z3's C++ layer sometimes…, z3_diagnostic_message(), z3_exception_message(), z3_synthesis_message(), test_parse_z3_exception_keeps_cached_holes(), boom(), parse_then_z3() (+2 more)

### Community 151 - "Translate"
Cohesion: 0.20
Nodes (11): _expand_iff(), _expand_implication(), Split `s` on top-level (depth-0) occurrences of `sep`., Apply `fn` to each top-level parenthesized subexpression in `s`., Rewrite Dafny `==>` into aeon's `-->`, at every depth. Implication is right-…, Rewrite `<==>` (left-assoc) as `==`, at every depth., _recurse_into_parens(), _split_balanced() (+3 more)

### Community 152 - "Typestate Protocols"
Cohesion: 0.36
Nodes (10): Deque size ghost, Downloader progress protocol, Email builder protocol, Iterator probe protocol, Linear refinement typestate, Lock protocol, Order checkout protocol, Reader protocol (+2 more)

### Community 153 - "Translate"
Cohesion: 0.24
Nodes (10): _expand_chained_comparison(), _expand_chained_comparison_pure(), _find_top_ops(), _paren_unless_atomic(), Return positions of top-level (depth-0) occurrences of any op in `ops`.…, Rewrite `a <= b <= c` into `(a <= b) && (b <= c)`. Caller must guarantee `s`…, Recursively expand chained comparisons at every nesting depth. Strategy: first…, Wrap `s` in parens unless it's already a parenthesized expression or a single… (+2 more)

### Community 154 - "Translate"
Cohesion: 0.20
Nodes (10): _expand_var_bindings(), Rewrite a leading sequence of `var X := EXPR;` (Dafny `function`/`predicate`…, Translate a single Dafny expression (Boolean or arithmetic) to Aeon. Order…, _strip(), translate_expression(), _wrap_negative_literals(), test_chained_comparison_does_not_split_implication(), test_negative_identifier_after_implication() (+2 more)

### Community 155 - "Gen"
Cohesion: 0.31
Nodes (9): affine(), fnum(), main(), pool(), ndarray, Generate an Aeon local-robustness query for a trained ReLU digit classifier.…, Average-pool a stack of 8x8 images by k x k -> (8//k)^2 features., Aeon-safe float literal: wrap negatives as ``(0.0 - m)``. (+1 more)

### Community 156 - "Build Stdlib Docs"
Cohesion: 0.36
Nodes (9): build_index(), build_module_pages(), _display_path(), main(), Path, Generate AeonDoc HTML for every stdlib library and an index page. Used by the…, Like ``Path.relative_to(REPO_ROOT)`` but falls back to the absolute path when…, Generate one HTML page per stdlib module. Returns sorted module names. (+1 more)

### Community 157 - "Lsp Test"
Cohesion: 0.20
Nodes (4): MockDocument, MockProtocol, MockWorkspace, Stand-in for pygls' LanguageServerProtocol: custom notifications are sent via…

### Community 158 - "Ordered Comparison Test"
Cohesion: 0.36
Nodes (9): _errors(), _run(), test_bool_comparison_rejected(), test_float_comparison_ok(), test_gt_bool_comparison_rejected(), test_int_comparison_ok(), test_polymorphic_helper_at_int_ok(), test_set_comparison_rejected() (+1 more)

### Community 159 - "Grammar Generation"
Cohesion: 0.28
Nodes (9): _atom_to_interval(), _intersect_high(), _intersect_low(), _literal_value(), Parse a single comparison atom over ``var`` against a numeric literal into an…, Best-effort parse of a refinement into a single interval over the bound…, refinement_to_interval(), go() (+1 more)

### Community 160 - "Synthesizer"
Cohesion: 0.22
Nodes (5): Finite Tree Automata (FTA) program-synthesis backend, registered as ``fta``., FTASynthesizer, build(), Any, Component-based synthesis by building a finite tree automaton bottom-up and…

### Community 161 - "Synthesizer"
Cohesion: 0.33
Nodes (7): _ground_parametric(), _has_typevar(), Peel surrounding refinements, exposing the bare base type., Collect every ground (type-variable-free) *parametric* constructor — one with…, Index the in-scope bindings as constructors (functions) and atoms. Monomorphic…, Peel ``forall`` type binders, returning their names (in declaration order) and…, _strip()

### Community 162 - "Backend Test"
Cohesion: 0.42
Nodes (8): test_application(), test_if(), test_if_str(), test_let(), test_literal(), test_rec(), test_type_abs_app(), weval()

### Community 163 - "Trust Report Test"
Cohesion: 0.39
Nodes (8): The ``--trust-report`` tool (issue #442). A program's verified guarantees rest…, _report(), test_axiom_is_reported_and_classified(), test_module_prefix_rendered_with_dot(), test_native_refined_binding_is_trusted(), test_trust_for_narrows_to_dependencies(), test_unrefined_native_is_not_trusted(), test_verified_definition_is_not_trusted()

### Community 164 - "Cuda"
Cohesion: 0.25
Nodes (4): _compile_vector_add_ptx(), Generate the two fixed vector-add kernels and compile them to PTX., Synchronize, release children and destroy this context. Cleanup is best-effort…, c_void_p

### Community 167 - "Typevars Test"
Cohesion: 0.50
Nodes (7): get_type_vars(), help_type_vars(), test_get_abstraction(), test_get_poly(), test_get_simple_refined(), test_get_simple_var(), test_get_type_vars()

### Community 168 - "Actions"
Cohesion: 0.25
Nodes (8): forward_let_abs_candidates(), forward_let_if_candidates(), forward_let_tabs_candidates(), _let_candidate(), Wrap a value in ``let v : value_type := value in ?goal``, reopening the goal…, Forward let-if tactic: bind an if-then-else over the goal type. Produces ``let…, Forward let-abstraction tactic: bind a function producing the goal type. For…, Forward let-type-abstraction tactic: bind a type-polymorphic value. Produces…

### Community 169 - "Recursive Refinement Induction"
Cohesion: 0.39
Nodes (7): Explicit induction tactic, Ground refinement reflection, Lemmas as terminating functions, Proof by Logical Evaluation, Quantifier-free recursion scope, Termination-gated refined recursion, Decidable refinement fragment

### Community 170 - "Relax Flat Gen"
Cohesion: 0.39
Nodes (7): affine_terms(), fnum(), main(), pool(), ndarray, Emit a *flattened* linear-relaxation robustness query that actually scales.…, Left-nested ``base + sum_i coef_i * name_i`` (skips zero coefficients).

### Community 171 - "Linearity Test"
Cohesion: 0.25
Nodes (8): _evaluate(), Lower, type-check, and run a program. Returns ``main``'s result., A ``0``-bound let's value is skipped at runtime — even a value that would crash…, Same erasure for ``Rec``: an erased recursive definition's body is never…, The companion check: an unrestricted (default) let's value *is* evaluated, so a…, test_m0_let_val_is_not_evaluated_at_runtime(), test_m0_rec_val_is_not_evaluated_at_runtime(), test_omega_let_val_is_evaluated()

### Community 172 - "Table"
Cohesion: 0.29
Nodes (6): group_by(), melt(), pivot(), Pivots the table to reorganize data, similar to a spreadsheet pivot.…, Unpivots the table from wide to long format. Parameters: table (list of dict):…, Groups rows in the table by the value in the specified column. Parameters:…

### Community 173 - "Vericoding Translator Test"
Cohesion: 0.29
Nodes (7): Exception, Raised when a Dafny task uses features outside the v1 subset., TranslationError, `forall j: int :: ...` is a quantifier even though it has a typed binder — the…, Dafny `month in {1, 3, 5}` uses a set literal — outside the int/bool subset.…, test_rejects_set_literal(), test_rejects_typed_quantifier()

### Community 174 - "Relax Gen"
Cohesion: 0.52
Nodes (6): affine(), fnum(), main(), pool(), ndarray, Emit a *linear-relaxation* (DeepPoly/CROWN-style) robustness query. Same setup…

### Community 175 - "Mark Linear Params"
Cohesion: 0.43
Nodes (6): _is_linear_type(), main(), _matching_paren(), Insert the ``1`` multiplicity on parameters whose type is a linear type.…, Index of the ``)`` closing the ``(`` at ``open_idx``, or None., transform()

### Community 176 - "Dace Test"
Cohesion: 0.52
Nodes (6): parametrize, Tests for the DACE data-completion examples (FTA paper, OOPSLA'17). Covers (a)…, _run(), test_fta_completes_cell(), test_fta_pbe_completion(), test_table_pipeline_runs()

### Community 177 - "Grammar Library Import Test"
Cohesion: 0.48
Nodes (6): _grammar_for_hole(), Regression test: building the synthesis grammar for a hole whose context…, test_higher_order_library_use_does_not_crash(), test_inline_dependent_refinement_on_sibling_binder_does_not_crash(), test_inline_nonzero_refinement_does_not_crash(), test_open_math_library_does_not_crash()

### Community 179 - "Learning"
Cohesion: 0.33
Nodes (6): Learning_load(), Learning_target(), Learning_target_at(), Designate ``column`` as the target; return the ``(X, y)`` Dataset. ``temporal``…, Designate the column at ``index`` (0-based) as the target., Convenience: read a CSV and use the *last* column as the target.

### Community 180 - "Init"
Cohesion: 0.33
Nodes (5): get_elapsed_time(), _get_function_args(), Metadata, The elapsed time since the start in seconds., Extract the function's argument names and types from the context. Arguments are…

### Community 181 - "Translate"
Cohesion: 0.40
Nodes (6): _paren_if_needed(), Rewrite Dafny call syntax `f(a, b, c)` into aeon curried form `(f a b c)`.…, Wrap argument in parens if it contains top-level operators or whitespace., _rewrite_calls(), test_calls_become_curried(), test_keyword_in_is_not_a_call()

### Community 182 - "Polytypes Test"
Cohesion: 0.33
Nodes (5): test_polytype(), test_polytypes_e2e(), test_polytypes_link1(), test_polytypes_link2(), test_polytypes_missing_decl()

### Community 183 - "Tdsyn Direction Test"
Cohesion: 0.33
Nodes (5): fake(), test_backward_combined_only_abstracts_when_abs_applies(), make_fake(), make_fake(), fake()

### Community 184 - "Docs"
Cohesion: 0.60
Nodes (3): AeonDoc standard-library HTML, Jekyll Pages build, Master-only GitHub Pages deployment

### Community 186 - "Existentials Replace Anf"
Cohesion: 0.70
Nodes (4): ANF elimination, ExistentialType, Latte alias tracking, Skolemized existential binders

### Community 187 - "Readme"
Cohesion: 0.60
Nodes (4): Grammar hiding by shadowing, AFTA predicate-abstraction CEGAR, Grammar-scoped synthesis components, SyGuS PBE Strings

### Community 188 - "Readme"
Cohesion: 0.70
Nodes (4): DeepPoly refinement relaxation, Exact SMT robustness verification, SMT translation bottleneck, Stable-neuron folding

### Community 190 - "Reader"
Cohesion: 0.50
Nodes (3): Helpers for ``libraries/Reader.ae``., Read one byte; return ``(code, reader)`` with ``code == -1`` at EOF., read_step()

### Community 191 - "Tensor"
Cohesion: 0.50
Nodes (3): Python bindings for the Aeon ``Tensor`` library. Only the operations that need…, Convert an Aeon ``List Float`` into a 1-D numpy float array., Tensor_from_list()

### Community 194 - "Models Integration"
Cohesion: 0.67
Nodes (3): Binder-aware name resolution, Classifier and Regressor typeclasses, SMT-opaque typeclass methods

### Community 195 - "Readme"
Cohesion: 0.83
Nodes (3): Benchmark fidelity tiers, N-ary abstract refinement gap, Synquid sixty-four benchmark ports

### Community 196 - "Lean Syntax Migrate"
Cohesion: 0.67
Nodes (3): main(), migrate(), Migrate Aeon surface syntax to the Lean-style convention. Outside of string…

### Community 199 - "Bottom Up"
Cohesion: 1.00
Nodes (3): freeze(), Any, A hashable, order-stable view of a candidate's output, so that equal outputs…

### Community 201 - "Float Ng"
Cohesion: 1.00
Nodes (3): walk(), A liquid application of an array length measure (e.g. ``Array_size``)., _size_app()

### Community 202 - "Guards"
Cohesion: 1.00
Nodes (3): Operators of every relational atom appearing *anywhere* in ``atoms`` --…, _relational_ops_in(), walk()

## Ambiguous Edges - Review These
- `CLAUDE.md` → `Administrative normal form (historical; stale documentation)`  [AMBIGUOUS]
  CLAUDE.md · relation: references
- `Compiler pipeline` → `Administrative normal form (historical; stale documentation)`  [AMBIGUOUS]
  CLAUDE.md · relation: references
- `Core AST` → `Administrative normal form (historical; stale documentation)`  [AMBIGUOUS]
  CLAUDE.md · relation: conceptually_related_to
- `Administrative normal form (historical; stale documentation)` → `Liquid typechecking`  [AMBIGUOUS]
  CLAUDE.md · relation: conceptually_related_to
- `docs/http.md` → `Positive timeout contract`  [AMBIGUOUS]
  docs/http.md · relation: conceptually_related_to
- `docs/index.md` → `Classifier and Regressor typeclasses`  [AMBIGUOUS]
  docs/index.md · relation: conceptually_related_to

## Knowledge Gaps
- **10 isolated node(s):** `install.sh script`, `PATH`, `OperationInfo`, `run_sweep.sh script`, `AeonLang` (+5 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2304 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **39 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `CLAUDE.md` and `Administrative normal form (historical; stale documentation)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Compiler pipeline` and `Administrative normal form (historical; stale documentation)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Core AST` and `Administrative normal form (historical; stale documentation)`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Administrative normal form (historical; stale documentation)` and `Liquid typechecking`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `docs/http.md` and `Positive timeout contract`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `docs/index.md` and `Classifier and Regressor typeclasses`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **Why does `AeonDriver` connect `Liquidjava Protocols Test` to `Api`, `Contracts`, `Tdsyn Direction Test`, `Linear Random Test`, `Program`, `Desugar`, `Parser`, `Synthesis Ui`, `Compile`, `Converter`, `Driver`, `Match`, `Match Inductive Test`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
## Coverage and accounting qualifications

Directed structural Python graph plus semantic extraction of 40 documents. Excludes 49 image assets. Graphify does not parse .ae and .lark files; selected examples were inspected separately in the study. Agent token usage is unavailable from the host; numeric zero fields are placeholders, not zero-cost claims. Graph edges are navigation evidence, not proof of runtime behavior.
