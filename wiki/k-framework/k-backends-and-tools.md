---
id: k.framework.backends-tools
type: runtime
title: K backends, tools, and pyk
status: active
updated_at: 2026-09-03T14:35:47Z
sources:
  - SRC-0036
---

# K Backends, Tools, and pyk

This document provides a comprehensive technical analysis of the execution backends, developer tools, compiler pipelines, type inference systems, and the `pyk` Python SDK within the K Framework repository at commit `4a46d1231473b599c699160132fd6e76a5c46406` (version `v7.1.337`). It synthesizes the runtime and tooling architecture supporting language semantics defined in [k-tutorial-intermediate](k-tutorial-intermediate.md), [k-user-manual](k-user-manual.md), and [k-builtins](k-builtins.md).

## Repository Architecture and Developer Workflow

The K Framework repository is organized as a monorepo maintaining two technology stacks: a Java/Scala toolchain for frontend parsing and compiler drivers, and a Python SDK (`pyk/`) for programmatic orchestration and proof construction (CLM-0280; SRC-0036 CLAUDE.md #project-overview; repository observation; not reproduced; high; S6).

```
k-frontend/                  K language parser and frontend (Java/Scala)
k-distribution/              Main distribution bundle (bin/, lib/, ktest.mak)
haskell-backend/             Haskell-based symbolic execution backend
llvm-backend/                LLVM-based concrete execution backend
pyk/                         Python SDK sub-project (self-contained, uv package manager)
k-tutorial/                  Tutorial K definitions
```

The Java/Scala components are compiled using Maven (`mvn package -DskipTests`) requiring Java 17 and Scala 2.13, formatted with Spotless (`mvn spotless:apply`), and tested via JUnit (`mvn verify`) (CLM-0280; SRC-0036 CLAUDE.md lines 36-60; repository observation; not reproduced; high; S6). The Python `pyk/` sub-project uses `hatchling` and `uv`, enforcing strict linting via Black (line length 120), isort, flake8, mypy (`disallow_untyped_defs = true`), and pydocstyle (CLM-0280; SRC-0036 CLAUDE.md lines 46-80; repository observation; not reproduced; high; S6). Regression test suites comprise 229 test directories under `k-distribution/tests/regression-new` and 213 test directories under `pyk/regression-new` (CLM-0280; SRC-0036 CLAUDE.md lines 107-125; repository observation; not reproduced; high; S6).

## The LLVM Backend: High-Speed Concrete Execution

The LLVM backend compiles K definitions into standalone native C++ binaries, prioritizing concrete execution speed and memory efficiency (CLM-0281; SRC-0036 docs/ktools.md #debugging; source fact; not reproduced; high; S6). During compilation with `kompile --backend llvm`, the frontend emits `definition.kore`, which the backend lowers to optimized decision trees and LLVM bitcode, finally linking against GMP, MPFR, and native runtime libraries (CLM-0281; SRC-0036 docs/ktools.md lines 285-296; source fact; not reproduced; high; S6).

### Interactive Debugging with GDB and LLDB

The LLVM backend integrates directly with standard debuggers when definitions are compiled with `--enable-llvm-debug` and executed via `krun --debugger` (CLM-0282; SRC-0036 docs/ktools.md #stepping; source fact; not reproduced; high; S6).

Developers can set breakpoints on specific execution events:
- Stepping through global rewrite steps by placing a breakpoint on `definition.kore:k_step` (CLM-0282; SRC-0036 docs/ktools.md lines 114-131; source fact; not reproduced; high; S6).
- Intercepting specific rule applications by breaking on `<ModuleName>.<ruleLabel>.rhs` (CLM-0282; SRC-0036 docs/ktools.md lines 135-150; source fact; not reproduced; high; S6).
- Intercepting rule side conditions by breaking on `<ModuleName>.<ruleLabel>.sc` (CLM-0282; SRC-0036 docs/ktools.md lines 151-165; source fact; not reproduced; high; S6).
- Breaking on individual function evaluations by breaking on function symbols prefixed with `Lbl` (CLM-0282; SRC-0036 docs/ktools.md lines 215-250; source fact; not reproduced; high; S6).
- Breaking across rule sets using regular expressions: `rbreak Lbl` breaks on all non-hooked functions, while `rbreak hook_INT` breaks on integer operations (CLM-0282; SRC-0036 docs/ktools.md lines 268-278; source fact; not reproduced; high; S6).

On macOS, `krun --debugger` launches LLDB, though GDB pretty-printing scripts are not yet fully ported to LLDB (CLM-0282; SRC-0036 docs/ktools.md lines 67-74; source fact; not reproduced; high; S6).

### Semantics Profiling

Performance profiling is conducted using the Linux `perf` tool (CLM-0283; SRC-0036 docs/ktools.md #profiling-your-k-semantics; source fact; not reproduced; high; S6). To preserve call frames, runtime dependencies must be compiled with frame pointers enabled (`-fno-omit-frame-pointer -DNDEBUG -O2`) (CLM-0283; SRC-0036 docs/ktools.md lines 285-297; source fact; not reproduced; high; S6). Profiling traces captured with `perf record -g` identify hot functions:
- `step` and `step_<ordinal>` indicate the computational cost of term matching (CLM-0283; SRC-0036 docs/ktools.md lines 347-349; source fact; not reproduced; high; S6).
- `side_condition_<ordinal>` reflects side condition evaluation (CLM-0283; SRC-0036 docs/ktools.md line 349; source fact; not reproduced; high; S6).
- `apply_rule_<ordinal>` measures right-hand side term construction (CLM-0283; SRC-0036 docs/ktools.md lines 349-350; source fact; not reproduced; high; S6).

The helper script `llvm-kompile-compute-loc <ordinal> <definition-dir>` maps rule ordinals back to source lines in the original `.k` specification (CLM-0283; SRC-0036 docs/ktools.md lines 350-365; source fact; not reproduced; high; S6).

## The Haskell Backend: Deductive Verification and Symbolic Execution

The Haskell backend (`kore-exec`) implements symbolic execution, reachability logic theorem proving (`kprove`), and matching logic deduction, interfacing directly with the Z3 SMT solver (CLM-0284; SRC-0036 docs/user_manual.md #manual-objectives; source fact; not reproduced; high; S6).

### Execution Model and SMT Integration

The Haskell backend operates over terms containing symbolic variables and matching logic constraints (CLM-0284; SRC-0036 include/kframework/builtin/kast.md #syntax-of-matching-logic; source fact; not reproduced; high; S6). During rewriting, path conditions are accumulated as matching logic formulas:
$$\phi \land \psi$$
At branching points, satisfiability queries are dispatched to Z3 to prune infeasible paths (CLM-0284; SRC-0036 docs/user_manual.md #smt-translation; source fact; not reproduced; high; S6).

### Interactive Proof Debugging in the Haskell REPL

When `kprove` fails to discharge a reachability claim, developers debug the proof interactively using `kore-repl` (CLM-0285; SRC-0036 docs/ktools.md #minimizing-output; source fact; not reproduced; high; S6). The REPL allows users to inspect the symbolic execution tree, display current node configurations, step along execution paths, apply specific rewrite axioms manually, and inspect SMT formulas (CLM-0285; SRC-0036 docs/ktools.md lines 10-58; source fact; not reproduced; high; S6).

### Output Minimization and Formatting

Symbolic configurations in proofs can become massive. The `pyk print` CLI utility compacts configuration output by filtering uninteresting cells:

```sh
kprove --output json --definition DEFN ... \
    | jq .term \
    | pyk print DEFN /dev/stdin --omit-cells ... --keep-cells ...
```

Supported flags comprise:
- `--minimize`: automatically strips unchanged or uninformative configuration cells (CLM-0286; SRC-0036 docs/ktools.md lines 22-24; source fact; not reproduced; high; S6).
- `--no-minimize`: retains the complete configuration structure (CLM-0286; SRC-0036 docs/ktools.md line 51; source fact; not reproduced; high; S6).
- `--omit-cells cell1,cell2`: suppresses specific comma-separated cells (whitespace around commas is strictly forbidden) (CLM-0286; SRC-0036 docs/ktools.md lines 52-57; source fact; not reproduced; high; S6).
- `--keep-cells cell1,cell2`: restricts output strictly to the named cells (CLM-0286; SRC-0036 docs/ktools.md line 53; source fact; not reproduced; high; S6).

### JVM Performance Optimization via `kserver`

To eliminate JVM startup overhead during repetitive test suites, K provides `kserver`, based on Nailgun (CLM-0287; SRC-0036 docs/ktools.md #running-tests---kserver; source fact; not reproduced; high; S6). `kserver` maintains a persistent in-memory process that shares thread pools and caches across consecutive `kompile`, `krun`, and `kprove` invocations, reducing integration test runtime from 8 minutes to 2 minutes on 32-core systems (CLM-0287; SRC-0036 docs/ktools.md lines 373-380; source fact; not reproduced; high; S6). However, the Nailgun implementation is incompatible with Java 18 and newer releases, and servers must be restarted between runs to prevent cache corruption (CLM-0287; SRC-0036 docs/ktools.md lines 384-386; source fact; not reproduced; high; S6).

## The KORE Intermediate Representation and KAST Formats

KORE is the formal textual intermediate language consumed by all K execution backends (CLM-0288; SRC-0036 pyk/docs/pipeline.md #stage-4-kore-emission; source fact; not reproduced; high; S6).

A KORE specification file (`definition.kore`) consists of:
1. Sort declarations: `sort S{} []`
2. Symbol declarations: `symbol Lblfoo{}(Sort1{}, Sort2{}) : ReturnSort{} []`
3. Axioms: structural equations, algebraic properties (associativity, commutativity, unit, idempotence), and semantic transitions `axiom{} \rewrites{...} []` (CLM-0288; SRC-0036 pyk/docs/pipeline.md lines 99-106; source fact; not reproduced; high; S6).

KAST terms are exchanged between tools using JSON serialization format (CLM-0289; SRC-0036 pyk/docs/pipeline.md lines 24-26; source fact; not reproduced; high; S6). The format specifies:
```json
{
  "format": "KAST",
  "version": 3,
  "term": { ... }
}
```
Intermediate compilation products are preserved as `compiled.json` in the compiled definition directory, establishing a clean serialization boundary between frontend parsing passes and backend emission (CLM-0289; SRC-0036 pyk/docs/pipeline.md lines 89-91; source fact; not reproduced; high; S6).

## The 5-Stage Compilation Pipeline

The K compiler transforms `.k` and `.md` source files into executable backend artifacts across five logical stages (CLM-0290; SRC-0036 pyk/docs/pipeline.md #overview; source fact; not reproduced; high; S6):

```
Outer parsing → Inner parsing → 32 compilation passes → Kore emission → Backend compilation
```

### Stage 1: Outer Parsing

The outer parser resolves file `require` statements and module import edges, producing a `KDefinition` of `KFlatModule`s where rule bodies remain as unparsed string tokens ("bubbles") (CLM-0290; SRC-0036 pyk/docs/pipeline.md #stage-1-outer-parsing; source fact; not reproduced; high; S6).
- Java implementation: JavaCC grammar at `k-frontend/src/main/javacc/Outer.jj`.
- Python implementation: `pyk/src/pyk/kast/outer_parser.py` via entry point `pyk.kast.utils.parse_outer()`.
- Pipeline seam: The compiler accepts `--outer-parsed-json <file>`, allowing Python `kompilex` to perform outer parsing and bypass JavaCC entirely (CLM-0292; SRC-0036 pyk/docs/pipeline.md lines 24-32; source fact; not reproduced; high; S6).

### Stage 2: Inner Parsing

The inner parser resolves the unparsed rule bubbles using the concrete context-free grammar generated from user syntax declarations (CLM-0290; SRC-0036 pyk/docs/pipeline.md #stage-2-inner-parsing-bubble-resolution; source fact; not reproduced; high; S6). This stage converts concrete expressions into fully resolved KAST terms.

### Stage 3: 32 Compilation Passes

The compiler executes thirty-two ordered, pure AST transformations in `KoreBackend.java` (CLM-0291; SRC-0036 pyk/docs/pipeline.md #stage-3-compilation-passes; source fact; not reproduced; high; S6):
1. `resolveComm`: resolves commutative simplification rules.
2. `resolveIO`: resolves I/O stream configuration cells.
3. `resolveFun`: resolves `#fun` anonymous function applications.
4. `resolveFunctionWithConfig`: desugars function context `[[ ... ]]` into auxiliary arguments.
5. `resolveStrict`: expands `strict` and `seqstrict` into heating and cooling rules.
6. `resolveAnonVars`: replaces anonymous `_` variables with fresh names.
7. `resolveContexts`: resolves context holes and context rewrites.
8. `numberSentences1`: assigns unique integer identifiers to sentences.
9. `resolveHeatCoolAttribute`: expands `heat` and `cool` attributes.
10. `resolveSemanticCasts`: resolves `#` cast operators.
11. `subsortKItem1`: adds injections from all sorts to `KItem`.
12. `constantFolding`: folds constant expressions via native reflection hooks.
13. `propagateMacroToRules`: propagates `macro` labels from productions to rules.
14. `guardOrs`: transforms `#Or` patterns into guarded alternative rules.
15. `resolveFreshConfigConstants`: resolves `!Var` constants in configurations.
16. `generateSortPredicateSyntax1`: generates `isSort(...)` predicate signatures.
17. `generateSortProjections1`: generates sort projection functions.
18. `expandMacros`: statically expands macro rules.
19. `addImplicitComputationCell`: inserts `<k>` cell into rules lacking explicit cells.
20. `resolveFreshConstants`: resolves `!Var` constants in rules via `<generatedCounter>`.
21. `generateSortPredicateSyntax2`: second sort predicate syntax pass.
22. `generateSortProjections2`: second sort projection pass.
23. `checkSimplificationRules`: validates that simplification left-hand sides contain function symbols.
24. `subsortKItem2`: second subsorting pass to `KItem`.
25. `concretizeCells`: closes and sorts cell structures (`AddTopCellToRules` $\to$ `AddParentCells` $\to$ `CloseCells` $\to$ `SortCells`).
26. `genCoverage`: generates coverage instrumentation.
27. `addSemanticsModule`: adds synthetic `LANGUAGE-PARSING` module.
28. `resolveConfigVar`: injects configuration variables into rule left-hand sides.
29. `addCoolLikeAtt`: tags rules with `cool-like` attribute.
30. `removeAnywhereRules`: filters out anywhere rules when compiling for the Haskell backend.
31. `generateSortPredicateRules`: generates rewrite rules implementing sort predicates.
32. `numberSentences2`: final sentence renumbering (CLM-0291; SRC-0036 pyk/docs/pipeline.md lines 49-85; source fact; not reproduced; high; S6).

Twenty-six of these thirty-two passes are completely pure AST transformations without external I/O (CLM-0291; SRC-0036 pyk/docs/pipeline.md lines 86-87; source fact; not reproduced; high; S6).

### Stage 4: KORE Emission

The post-pipeline KAST is converted into KORE syntax, generating `definition.kore` and `syntaxDefinition.kore` via `ModuleToKORE.java` or pyk's `_module_to_kore.py` (CLM-0288; SRC-0036 pyk/docs/pipeline.md #stage-4-kore-emission; source fact; not reproduced; high; S6).

### Stage 5: Backend Compilation

The target backend compiles `definition.kore` into its final runtime artifact: a native C++ executable in the LLVM backend or an optimized mathematical definition in the Haskell backend (CLM-0290; SRC-0036 pyk/docs/pipeline.md #stage-5-backend-compilation; source fact; not reproduced; high; S6).

## The `pyk` Python SDK and Verification Architecture

`pyk` is the official Python SDK for programmatically inspecting, transforming, executing, and proving K definitions (CLM-0293; SRC-0036 pyk/README.md; source fact; not reproduced; high; S6).

### SDK Architecture and Capabilities

`pyk` provides Python bindings located in `pyk/src/pyk/`:
- `pyk.kast`: classes representing KAST terms, outer syntax (`KDefinition`, `KFlatModule`), inner syntax, and parsers (CLM-0293; SRC-0036 CLAUDE.md line 26; repository observation; not reproduced; high; S6).
- `pyk.kore`: classes representing the KORE intermediate language.
- `pyk.ktool`: Python wrappers driving `kompile`, `krun`, and `kprove` as subprocesses or RPC clients.
- `pyk.kcfg`: K Control Flow Graphs, representing the symbolic state space as directed graphs where nodes are symbolic configurations and edges are rewrite transitions (CLM-0293; SRC-0036 CLAUDE.md line 26; repository observation; not reproduced; high; S6).
- `pyk.proof`: theorem proving infrastructure automating inductive proofs and circularity coinduction across KCFG graphs (CLM-0293; SRC-0036 CLAUDE.md line 26; repository observation; not reproduced; high; S6).

### Regression Suite Triage and Semantic Discrepancies

Triage documentation in `pyk/docs/regression-triage.md` reveals that 105 tests are skipped in `pyk/regression-new` due to backend and tool dependencies:
- Category A (39 tests): depend on the Haskell symbolic backend (CLM-0294; SRC-0036 pyk/docs/regression-triage.md lines 7-26; source fact; not reproduced; high; S6).
- Category B (16 tests): require ahead-of-time GLR/Bison C compilation (CLM-0294; SRC-0036 pyk/docs/regression-triage.md lines 27-39; source fact; not reproduced; high; S6).
- Category C (4 tests): require the legacy Kore bytecode interpreter (CLM-0294; SRC-0036 pyk/docs/regression-triage.md lines 40-49; source fact; not reproduced; high; S6).
- Category D (10 tests): missing `pyk parse` subcommand (CLM-0294; SRC-0036 pyk/docs/regression-triage.md lines 50-69; source fact; not reproduced; high; S6).
- Category E (8 tests): missing passthrough CLI flags in `pyk run` (e.g. `-cVAR=VAL`, `--search`) (CLM-0294; SRC-0036 pyk/docs/regression-triage.md lines 70-86; source fact; not reproduced; high; S6).

Category G resolved an intentional architectural divergence: Java `krun` strips synthetic `<generatedTop>` and `<generatedCounter>` cells from output, whereas `pyk run` intentionally retains them to preserve full structural integrity (CLM-0295; SRC-0036 pyk/docs/regression-triage.md lines 98-109; source fact; not reproduced; high; S6).

## Sort Inference Engine: The SimpleSub Algorithm

Sort inference in K (`SortInferencer.java`) implements Lionel Parreaux's SimpleSub algorithm for algebraic subtyping (CLM-0296; SRC-0036 docs/developers/sort_inference.md #design; source fact; not reproduced; high; S6).

### Polarities and Set-Theoretic Types

The type syntax admits primitives, function arrows, type variables $\alpha$, top $\top$, bottom $\bot$, type joins $\tau \sqcup \tau$, and type meets $\tau \sqcap \tau$ subject to a strict polarity discipline (CLM-0296; SRC-0036 docs/developers/sort_inference.md lines 13-24; source fact; not reproduced; high; S6):
- Negative polarity: describes values provided as inputs to a term (upper bounds). Type meets $\tau \sqcap \tau$ may occur only in negative positions (CLM-0297; SRC-0036 docs/developers/sort_inference.md lines 25-44; source fact; not reproduced; high; S6).
- Positive polarity: describes values produced as outputs from a term (lower bounds). Type joins $\tau \sqcup \tau$ may occur only in positive positions (CLM-0297; SRC-0036 docs/developers/sort_inference.md lines 25-44; source fact; not reproduced; high; S6).

Bounds $L <: \alpha <: U$ on type variables are represented in compact set-theoretic form as `CompactType(vars, prims, fun)` (CLM-0297; SRC-0036 docs/developers/sort_inference.md lines 53-60, 139-145; source fact; not reproduced; high; S6). Unnecessary parametricity is eliminated using co-occurrence simplifications:
1. If type variable $\alpha$ always co-occurs positively with $\beta$ and vice-versa, $\alpha$ and $\beta$ are unified (CLM-0297; SRC-0036 docs/developers/sort_inference.md lines 148-151; source fact; not reproduced; high; S6).
2. If $\alpha$ always co-occurs both positively and negatively with concrete type $T$, $\alpha$ is eliminated (e.g. $\alpha \sqcap \text{Int} \to \alpha \sqcup \text{Int}$ simplifies to $\text{Int} \to \text{Int}$) (CLM-0297; SRC-0036 docs/developers/sort_inference.md lines 152-155; source fact; not reproduced; high; S6).

### Ambiguity Slices

To resolve parse ambiguities, parse trees are partitioned into unambiguous sub-trees called *slices*, cut at each ambiguity node (CLM-0298; SRC-0036 docs/developers/sort_inference.md #ambiguities; source fact; not reproduced; high; S6). Types are inferred for each slice parametric over child ambiguity variables:
$$\lambda amb_1 \dots \lambda amb_K . \lambda x_1 \dots \lambda x_N . \text{translateBody}(t)$$
The inference engine folds children upward from the leaves; paths encountering subtyping errors during function application are pruned (CLM-0298; SRC-0036 docs/developers/sort_inference.md lines 208-217; source fact; not reproduced; high; S6). While worst-case complexity is $O(2^N)$, typical factor sizes remain small (CLM-0298; SRC-0036 docs/developers/sort_inference.md line 216; source fact; not reproduced; high; S6).

## Backend Option Interactions and Compatibility

Compilation and execution options interact strictly across backends:
- Module exclusion: Modules marked `[concrete]` compile exclusively under `--backend llvm`; modules marked `[symbolic]` compile exclusively under `--backend haskell` (CLM-0299; SRC-0036 docs/user_manual.md #symbolic-and-concrete-attribute; source fact; not reproduced; high; S6).
- Rule attribute conflicts: The `anywhere` attribute is supported exclusively on the LLVM backend; compiling `anywhere` rules under the Haskell backend raises a compiler error (CLM-0299; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md lines 144-148; source fact; not reproduced; high; S6).
- Simplification lemmas: The `simplification` attribute with `concrete(...)` or `symbolic(...)` filters is interpreted exclusively by the Haskell backend; the LLVM backend ignores these directives (CLM-0299; SRC-0036 docs/user_manual.md #concrete-and-symbolic-attributes-haskell-backend; source fact; not reproduced; high; S6).
- Proof claims: Claims declared via `claim` can only be verified using `kprove` with `--backend haskell`; the LLVM backend cannot verify symbolic claims (CLM-0299; SRC-0036 docs/user_manual.md #all-path-and-one-path-attributes-to-distinguish-reachability-claims; source fact; not reproduced; high; S6).

For tutorial fundamentals, see [k-tutorial-basic](k-tutorial-basic.md) and [k-tutorial-intermediate](k-tutorial-intermediate.md). For language attributes, see [k-user-manual](k-user-manual.md). For standard library signatures, see [k-builtins](k-builtins.md). For ZKIR VM specifications and formal execution models, see [zkir-instruction-set](../zkir/zkir-instruction-set.md) and [zkir-vm-semantics](../zkir/zkir-vm-semantics.md).
