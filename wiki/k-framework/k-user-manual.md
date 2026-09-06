---
id: k.framework.user-manual
type: language
title: K user manual digest
status: active
updated_at: 2026-09-03T14:34:45Z
sources:
  - SRC-0036
---

# K User Manual Digest

This document provides an exhaustive, structured reference digest of the official K Framework User Manual. It is organized according to the user manual's own sections and serves as the normative reference for authors specifying formal programming language semantics, intermediate representations, and verification systems in K. It complements the introductory tutorial material in [k-tutorial-basic](k-tutorial-basic.md) and [k-tutorial-intermediate](k-tutorial-intermediate.md).

## Introduction to K and Specification Architecture

At the highest level, K specifications define a language or system through three distinct semantic components:
1. *System primitives*: the primitive datatypes, such as integers, booleans, lists, and maps, defined via `syntax` declarations.
2. *System state*: a hierarchical, labeled configuration record defined via `configuration` declarations.
3. *System behavior*: the state transitions and computational evaluations defined via `rule`, `context`, and `claim` sentences (CLM-0225; SRC-0036 docs/user_manual.md #introduction-to-k; source fact; not reproduced; high; S6).

A K specification is partitioned into files and modules:

```
   example.k file
  +=======================+
  | requires ".." --------|--> File_1
  | ...                   |
  | requires ".." --------|--> File_N
  |                       |
  |  +-----------------+  |
  |  | module ..       |  |
  |  |   imports .. ---|--|--> Module_1
  |  |   ...           |  |
  |  |   imports .. ---|--|--> Module_M
  |  |                 |  |
  |  |   sentences     |  |
  |  | endmodule       |  |
  |  +-----------------+  |
  +=======================+
```

Files require other files, and modules import other modules (CLM-0225; SRC-0036 docs/user_manual.md #introduction-to-k; source fact; not reproduced; high; S6). Both the file inclusion hierarchy and the module import hierarchy must form strict directed acyclic graphs (DAGs) without mutual or self-referential cycles (CLM-0225; SRC-0036 docs/user_manual.md #introduction-to-k; source fact; not reproduced; high; S6).

## Module System and Visibility

Modules are declared at top level using the `module` keyword, an identifier, optional attributes, a sequence of import statements, sentence declarations, and the concluding `endmodule` keyword (CLM-0226; SRC-0036 docs/user_manual.md #module-declaration; source fact; not reproduced; high; S6).

A module ID begins with an optional `#` followed by hyphen-separated components consisting of alphanumeric characters and underscores (CLM-0226; SRC-0036 docs/user_manual.md #module-declaration; source fact; not reproduced; high; S6).

### Import Visibility: Public and Private

Imports are declared with `imports [public|private] MODULE-NAME`:
- Public imports: By default, imports are public. All syntax and rules transitively exported by the imported module become available to any module that subsequently imports the current module (CLM-0226; SRC-0036 docs/user_manual.md #module-declaration; source fact; not reproduced; high; S6).
- Private imports: A private import incorporates sentences into the current module only. Downstream importing modules do not inherit the privately imported syntax or rules (CLM-0226; SRC-0036 docs/user_manual.md #module-declaration; source fact; not reproduced; high; S6).

### Module Visibility Attributes

Placing the `[private]` attribute directly on a module declaration flips the default visibility of all sentences and imports within that module to private (CLM-0226; SRC-0036 docs/user_manual.md #private-attribute; source fact; not reproduced; high; S6). Individual syntax productions or functions can be selectively exposed across module boundaries by tagging them with the `[public]` attribute (CLM-0226; SRC-0036 docs/user_manual.md #public-and-private-attribute; source fact; not reproduced; high; S6).

```k
module WIDGET-SYNTAX
  syntax Widget ::= foo()
  syntax WidgetHelper ::= bar() [private]
endmodule

module WIDGET [private]
  imports WIDGET-SYNTAX
  syntax Widget ::= fooImpl()
  syntax KItem ::= adjustWidget(Widget) [function, public]
endmodule
```

### Backend-Conditional Module Attributes

Modules may be tagged with `[concrete]` or `[symbolic]`:
- `[concrete]`: instructs the frontend to include this module only when compiling with the LLVM backend (CLM-0227; SRC-0036 docs/user_manual.md #symbolic-and-concrete-attribute; source fact; not reproduced; high; S6).
- `[symbolic]`: instructs the frontend to include this module only when compiling with the Haskell backend (CLM-0227; SRC-0036 docs/user_manual.md #symbolic-and-concrete-attribute; source fact; not reproduced; high; S6).

If a definition is compiled on the opposite backend, these modules are excised prior to parsing (CLM-0227; SRC-0036 docs/user_manual.md #symbolic-and-concrete-attribute; source fact; not reproduced; high; S6). The user manual notes a critical caveat: backend-conditional inclusion should be used sparingly because divergence between concrete execution and symbolic proofs undermines the trustworthiness of formal guarantees (CLM-0227; SRC-0036 docs/user_manual.md #symbolic-and-concrete-attribute; source fact; not reproduced; high; S6).

## Syntax and Lexical Declarations

Syntax declarations define sort categories, Backus-Naur Form (BNF) productions, tokens, operator precedence, and associativity.

### Sort Declarations, Subsorting, and Injections

A new sort is introduced by declaring `syntax SortName`. Subsort relationships are established by subsort BNF alternatives:

```k
syntax Exp ::= Val
syntax Val ::= Int
```

In this declaration, `Int` is a subsort of `Val`, and `Val` is a subsort of `Exp`. Every sort in K is automatically established as a subsort of `KItem`, and `KItem` is a subsort of `K` (CLM-0228; SRC-0036 include/kframework/builtin/kast.md #basic-k-sorts; source fact; not reproduced; high; S6). During compilation to KORE, subsorting is desugared into explicit unary injection symbols `inj{FromSort, ToSort}(Term)` (CLM-0228; SRC-0036 pyk/docs/pipeline.md #stage-4-kore-emission; source fact; not reproduced; high; S6).

### Named Non-Terminals

Nonterminals in BNF productions can be explicitly named using `name: Sort`:

```k
syntax Exp ::= lookup(table: Map, key: Id)
```

Named nonterminals document grammar slots unambiguously and trigger the automatic generation of projection functions (e.g. `table(lookup(M, K)) => M`) (CLM-0228; SRC-0036 docs/user_manual.md #named-non-terminals; source fact; not reproduced; high; S6).

### Production Naming and Symbol Mangling

Every BNF alternative corresponds to an underlying KLabel constructor. By default, K auto-mangles production signatures into long identifiers such as `Lblfoo'LParUndsCommUndsRParUnds'MODULE'Unds'Sort` (CLM-0229; SRC-0036 docs/user_manual.md #symbol_-attribute; source fact; not reproduced; high; S6). Attaching the `symbol(name)` attribute overrides this behavior, assigning an immutable, user-defined identifier:

```k
syntax Foo ::= foo(Int) [symbol(foo)]
```

Explicit symbols are required to be globally unique within the definition (CLM-0229; SRC-0036 docs/user_manual.md #symbol_-attribute; source fact; not reproduced; high; S6). For syntactic lists (`List{Sort, ","}`), the list terminator's label can be customized using `terminator-symbol(name)` (CLM-0229; SRC-0036 docs/user_manual.md #syntactic-lists; source fact; not reproduced; high; S6).

### Subsort Overloading

Subsort overloading allows a constructor to produce a more specific return sort when its arguments belong to more specific subsorts (CLM-0230; SRC-0036 docs/user_manual.md #overload-attribute; source fact; not reproduced; high; S6):

```k
syntax Exp  ::= LVal
              | Exp  "." Id [overload(_._)]
syntax LVal ::= LVal "." Id [overload(_._)]
```

Productions sharing an `overload(key)` attribute are organized into a partial order. Production $P$ is more specific than $Q$ if the argument and result sort tuple of $P$ is strictly smaller elementwise than that of $Q$ (CLM-0230; SRC-0036 docs/user_manual.md #overload-attribute; source fact; not reproduced; high; S6). During ambiguity resolution, the parser selects the most specific overloaded production (CLM-0230; SRC-0036 docs/user_manual.md #overload-attribute; source fact; not reproduced; high; S6).

### Parametric Productions and Polymorphism

Parametric productions accept a bracketed list of sort variables preceding the sort signature:

```k
syntax {Sort} Sort ::= "(" Sort ")" [bracket]
syntax {Sort} Sort ::= Sort "=>" Sort
syntax {Sort} Sort ::= "#if" Bool "#then" Sort "#else" Sort "#fi"
syntax {Sort1, Sort2} Sort1 ::= "#fun" "(" Sort2 "=>" Sort1 ")" "(" Sort2 ")"
```

Parametric sort variables ensure type safety across polymorphic constructs without collapsing intermediate sorts into `K` or `KBott` (CLM-0231; SRC-0036 docs/user_manual.md #parametric-productions-and-bracket-attributes; source fact; not reproduced; high; S6).

### Functions, Constructors, and Hooks

Productions represent one of three semantic categories:
1. *Constructors*: By default, syntax productions without attributes are constructors. They represent passive abstract syntax tree nodes and configuration structures that do not reduce unless matched by a rewrite rule (CLM-0232; SRC-0036 docs/user_manual.md #rule-declaration; source fact; not reproduced; high; S6).
2. *Functions*: Tagging a production with `[function]` instructs the rewriter to simplify the term greedily as soon as it appears anywhere in a configuration (CLM-0232; SRC-0036 docs/user_manual.md #function-and-total-attributes; source fact; not reproduced; high; S6). A function is partially defined; evaluating a function on unhandled arguments produces matching logic bottom (`#Bottom`), which causes LLVM backend interpreters to abort or crash (CLM-0232; SRC-0036 docs/user_manual.md #function-and-total-attributes; source fact; not reproduced; high; S6).
3. *Total functions*: Attaching `[total]` asserts that the function is defined over all inputs of its domain sorts, yielding exactly one result (CLM-0232; SRC-0036 docs/user_manual.md #function-and-total-attributes; source fact; not reproduced; high; S6).
4. *Hooked symbols*: The `[hook(HOOK.name)]` attribute binds a function or sort directly to optimized C++ native implementations in the LLVM backend and Haskell equivalents in the symbolic backend (CLM-0232; SRC-0036 docs/user_manual.md #pending-documentation; source fact; not reproduced; high; S6).

### Lexical Syntax, Tokens, and Precedence

Lexical tokens are defined using regular expression terminals with the `[token]` attribute (CLM-0233; SRC-0036 docs/user_manual.md #token-attribute; source fact; not reproduced; high; S6). When two token productions match prefixes of equal length, lexical ambiguity is resolved via the `prec(N)` attribute:

```k
syntax #LowerId ::= r"[a-z][a-zA-Z0-9]*" [prec(2), token]
syntax #UpperId ::= r"[A-Z][a-zA-Z0-9]*" [prec(2), token]
```

Token productions with higher numerical precedence are selected first (CLM-0233; SRC-0036 docs/user_manual.md #prec-attribute; source fact; not reproduced; high; S6). Whitespace and comment tokens are discarded from the scanner by declaring productions under sort `#Layout`:

```k
syntax #Layout ::= r"(/\\*([^\\*]|(\\*+([^\\*/])))*\\*+/)"
                 | r"(//[^\\n\\r]*)"
                 | r"([ \\n\\r\\t])"
```

If no `#Layout` productions are specified, `DEFAULT-LAYOUT` is imported automatically (CLM-0233; SRC-0036 include/kframework/builtin/kast.md #layout-information; source fact; not reproduced; high; S6).

### Operator Precedence, Associativity, and Priority Blocks

Grammar ambiguities in expression grammars are resolved using priority blocks and associativity declarations:

```k
syntax Exp ::= left:
               Exp "*" Exp [group(mult)]
             | Exp "/" Exp [group(div)]
             > left:
               Exp "+" Exp [group(add)]
             | Exp "-" Exp [group(sub)]
```

The `>` operator separates priority levels; operators preceding `>` bind tighter than operators succeeding it (CLM-0234; SRC-0036 docs/user_manual.md #symbol-priority-and-associativity; source fact; not reproduced; high; S6).

Associativity directives enforce structure across operators of identical priority:
- `left`: terms associate to the left; the operator cannot appear as the rightmost child of another operator in the block (CLM-0234; SRC-0036 docs/user_manual.md #symbol-priority-and-associativity; source fact; not reproduced; high; S6).
- `right`: terms associate to the right; the operator cannot appear as the leftmost child (CLM-0234; SRC-0036 docs/user_manual.md #symbol-priority-and-associativity; source fact; not reproduced; high; S6).
- `non-assoc`: nesting is forbidden in both leftmost and rightmost positions without explicit brackets (CLM-0234; SRC-0036 docs/user_manual.md #symbol-priority-and-associativity; source fact; not reproduced; high; S6).

When syntax declarations are distributed across multiple modules, priority and associativity can be declared out-of-line using named groups:

```k
syntax priority mult div > add sub
syntax left mult div
syntax right add sub
```

Authors must never combine different priority levels within a single `syntax left` or `syntax right` statement (CLM-0235; SRC-0036 docs/user_manual.md #symbol-priority-and-associativity; source fact; not reproduced; high; S6). Furthermore, the `applyPriority(N)` attribute overrides the default priority filtering on productions whose outer symbols are terminals, enforcing priority rejection at the $N$th nonterminal position (CLM-0234; SRC-0036 docs/user_manual.md #symbol-priority-and-associativity; source fact; not reproduced; high; S6).

### Exhaustive Table of Production Attributes

The following table catalogs every production attribute recognized by the K compiler along with its concise operational meaning (CLM-0236; SRC-0036 docs/user_manual.md #attribute-index; source fact; not reproduced; high; S6):

| Attribute | Category | Meaning |
| :--- | :--- | :--- |
| `alias` | prod | Macro rule reversed during unparsing to restore original surface syntax |
| `alias-rec` | prod | Recursive macro rule reversed during unparsing |
| `applyPriority(N)` | prod | Enforces priority filtering at the specified nonterminal position $N$ |
| `assoc` | prod | Marks binary operator as associative and flattens indentation during unparsing |
| `avoid` | prod | Demotes parse tree priority during ambiguity resolution |
| `binder` | prod | Designates production as variable-binding construct for capture-aware substitution |
| `bracket` | prod | Designates production as transparent bracket syntax inserted during unparsing |
| `bracketLabel` | prod | Internal attribute tracking bracket production label |
| `color(C)` | prod | Assigns terminal display color for terminal unparsing |
| `colors(Cs)` | prod | Assigns comma-separated list of colors for terminals and format escapes |
| `comm` | prod | Marks binary operator as commutative and enables algebraic sorting |
| `deprecated` | prod | Emits a compiler warning whenever the symbol is used in rules |
| `element(L)` | prod | Internal attribute recording element constructor for collection sorts |
| `format(F)` | prod | Controls unparsing indentation, newlines, and color positioning |
| `freshGenerator` | prod | Designates function for generating fresh values of a user-defined sort |
| `function` | prod | Marks symbol as a partial function evaluated eagerly in configurations |
| `group(G)` | prod | Assigns sentence to a user-defined group identifier for priority/associativity |
| `hook(H)` | prod | Binds symbol to backend native implementation hook |
| `hybrid` | prod | Treats strict production as KResult once its subterms are fully evaluated |
| `hybrid(Sorts)` | prod | Generates hybrid predicates for specified sorts |
| `idem` | prod | Marks operator as idempotent |
| `injective` | prod | Labels constructor as injective for pattern matching and unification |
| `internal` | prod | Marks production as reserved for compiler internal use |
| `klabel(L)` | prod | Legacy frontend label assignment (superseded by `symbol(_)`) |
| `left` | prod | Declares production as left-associative with respect to itself |
| `locations` | sort | Annotates parsed terms of this sort with file, line, and column metadata |
| `macro` | prod | Statically expands production before runtime execution; non-recursive |
| `macro-rec` | prod | Statically expands production recursively until reaching a fixpoint |
| `memo` | rule | Caches concrete function call results in the Haskell backend |
| `non-assoc` | prod | Declares production as non-associative with respect to itself |
| `not-lr1` | mod | Warns when generating LR(1) parser from a known non-LR(1) module |
| `overload(K)` | prod | Declares production as part of a subsort overloading family keyed by $K$ |
| `prec(N)` | token | Assigns numerical precedence for resolving lexical scanner ambiguities |
| `prefer` | prod | Promotes parse tree priority during ambiguity resolution |
| `private` | prod | Restricts production visibility exclusively to the declaring module |
| `public` | prod | Exports production across module import boundaries |
| `result(S)` | prod | Overrides target evaluation sort from `KResult` to specified sort $S$ |
| `right` | prod | Declares production as right-associative with respect to itself |
| `seqstrict` | prod | Evaluates arguments sequentially from left to right |
| `seqstrict(Is)` | prod | Evaluates arguments sequentially in the specified order of indices |
| `strict` | prod | Evaluates all arguments non-deterministically using heating/cooling |
| `strict(Is)` | prod | Evaluates only the specified argument positions |
| `symbol(S)` | prod | Sets unique, unmangled KORE symbol identifier for the production |
| `terminator-symbol(S)` | prod | Sets explicit symbol identifier for a syntactic list terminator |
| `token` | prod | Designates sort or production as primitive lexical token domain |
| `total` | prod | Asserts that a function has exactly one return value for all inputs |
| `unit(U)` | prod | Records unit element constructor for algebraic collections |
| `unused` | prod | Suppresses compiler warnings regarding unused syntax declarations |
| `userList(U)` | prod | Internal attribute marking desugared syntactic list productions |

## Configuration Declarations and Cell Attributes

The `configuration` declaration establishes the hierarchical unit of state (CLM-0237; SRC-0036 docs/user_manual.md #configuration-declaration; source fact; not reproduced; high; S6). Cells use an XML-like syntax enclosing sort-annotated terms or child cells:

```k
configuration <top>
                <k> $PGM:Pgm </k>
                <state> .Map </state>
                <threads>
                  <thread multiplicity="*" type="Set">
                    <id> 0:Int </id>
                    <k> .K </k>
                  </thread>
                </threads>
                <status exit=""> 0:Int </status>
              </top>
```

If a definition omits a configuration declaration, `DEFAULT-CONFIGURATION` is imported, providing `<k> $PGM:K </k>` (CLM-0237; SRC-0036 include/kframework/builtin/kast.md #default-configuration; source fact; not reproduced; high; S6).

### Cell Attributes Reference

Configuration cell start tags accept attributes specified in XML key-value syntax (CLM-0238; SRC-0036 docs/user_manual.md #configuration-declaration; source fact; not reproduced; high; S6):
- `exit=""`: Attached to exactly one cell of sort `Int`. Invocations of `krun` return this integer as the process exit code (CLM-0238; SRC-0036 docs/user_manual.md #exit-attribute; source fact; not reproduced; high; S6).
- `multiplicity`: Specifies cell cardinaity in the configuration:
  - `multiplicity="*"`: The cell can appear zero or more times, representing dynamic thread or resource pools (CLM-0238; SRC-0036 docs/user_manual.md #collection-cells-multiplicity-and-type-attributes; source fact; not reproduced; high; S6).
  - `multiplicity="?"`: The cell is optional, appearing zero or one times (CLM-0238; SRC-0036 docs/user_manual.md #collection-cells-multiplicity-and-type-attributes; source fact; not reproduced; high; S6).
- `type`: Governs the collection data structure managing multiplicity cells:
  - `type="Set"`: Cell occurrences are de-duplicated and matched non-deterministically (CLM-0238; SRC-0036 docs/user_manual.md #collection-cells-multiplicity-and-type-attributes; source fact; not reproduced; high; S6).
  - `type="List"`: Cells maintain insertion order, supporting addition or deletion at the front or back (CLM-0238; SRC-0036 docs/user_manual.md #collection-cells-multiplicity-and-type-attributes; source fact; not reproduced; high; S6).
  - `type="Map"`: The first child sub-cell is designated as the lookup key, enabling optimized index-based matching when the key is mentioned in rules (CLM-0238; SRC-0036 docs/user_manual.md #collection-cells-multiplicity-and-type-attributes; source fact; not reproduced; high; S6).
- `stream="name"`: Connects cell contents to standard system I/O streams (`stdin`, `stdout`, or `stderr`), triggering automatic stream buffering rules (CLM-0238; SRC-0036 docs/user_manual.md line 3100; source fact; not reproduced; high; S6).
- `format` and `color`: Controls formatting indentation and terminal coloration when pretty-printing configuration dumps (CLM-0238; SRC-0036 docs/user_manual.md #formatting; source fact; not reproduced; high; S6).

## The Semantics of Rewriting

Semantic transitions are specified via `rule` sentences:

```k
rule LHS => RHS requires REQ ensures ENS [ATTRS]
```

A rule defines a transition from pre-state `LHS #And REQ` to post-state `RHS #And ENS` (CLM-0239; SRC-0036 docs/user_manual.md #rule-structure; source fact; not reproduced; high; S6).

Execution proceeds in three discrete phases:
1. Matching / Unification: The rewriter checks whether pattern `LHS` matches the subject configuration, binding free variables to substitution $\alpha$ (CLM-0240; SRC-0036 docs/user_manual.md #rule-structure; source fact; not reproduced; high; S6).
2. Precondition Validation: The side condition $\alpha(REQ)$ is evaluated; if it fails to evaluate to `true` (or cannot be proven satisfiable by SMT), the rule does not apply (CLM-0240; SRC-0036 docs/user_manual.md #rule-structure; source fact; not reproduced; high; S6).
3. Post-State Instantiation: The successor term $\alpha(RHS)$ is constructed and constrained by $\alpha(ENS)$ (CLM-0240; SRC-0036 docs/user_manual.md #rule-structure; source fact; not reproduced; high; S6).

### Configuration Framing and Structural Completion

K features modular configuration framing. Rules only specify cells directly involved in the rewrite; omitted cells are preserved automatically by compiler completion passes (`concretizeCells` in `KoreBackend`) (CLM-0240; SRC-0036 pyk/docs/pipeline.md #stage-3-compilation-passes; source fact; not reproduced; high; S6). Inside a cell, the `...` ellipsis denotes an anonymous framing variable matching the unmentioned remainder of the collection or list (CLM-0240; SRC-0036 include/kframework/builtin/kast.md #syntax-of-cells; source fact; not reproduced; high; S6).

### Anonymous and Scoped Variables

Variables beginning with an underscore (such as `_` or `_Val`) represent anonymous values matching any term of the inferred sort. Each distinct occurrence of `_` creates a distinct, independent variable (CLM-0240; SRC-0036 docs/user_manual.md #variable-sort-inference; source fact; not reproduced; high; S6).

## Rule Attributes and Execution Modifiers

Rule behavior is configured by attaching attributes in square brackets:

### Priority and `owise`

Rules are selected according to numerical priority, attempting lower values first (CLM-0241; SRC-0036 docs/user_manual.md #owise-and-priority-attributes; source fact; not reproduced; high; S6):
- Default priority: Any rule without explicit priority attributes is assigned priority 50 (CLM-0241; SRC-0036 docs/user_manual.md #owise-and-priority-attributes; source fact; not reproduced; high; S6).
- `priority(N)`: Assigns explicit priority $N$. In the LLVM backend, priorities 50 to 150 inclusive are reserved for internal compiler ordering heuristics (CLM-0241; SRC-0036 docs/user_manual.md #owise-and-priority-attributes; source fact; not reproduced; high; S6).
- `owise`: Assigns priority 200, ensuring the rule applies only after all standard rules have failed to match (CLM-0241; SRC-0036 docs/user_manual.md #owise-and-priority-attributes; source fact; not reproduced; high; S6).

### Simplification Rules

The `[simplification]` attribute designates rules that simplify function expressions during symbolic execution:

```k
rule (X +Int Y) +Int Z => X +Int (Y +Int Z) [simplification]
```

Simplification rules match nested function arguments, which is forbidden in standard function rules (CLM-0242; SRC-0036 docs/user_manual.md #simplification-attribute; source fact; not reproduced; high; S6).

Author responsibility caveat: Neither the frontend nor the Haskell backend checks whether simplification rules are sound (CLM-0242; SRC-0036 docs/user_manual.md #simplification-attribute; source fact; not reproduced; high; S6). Simplification rules must preserve definedness: if the left-hand side is `#Bottom`, the right-hand side must evaluate to `#Bottom` or be guarded by a `requires` or `ensures` clause evaluating to `false` (CLM-0242; SRC-0036 docs/user_manual.md #simplification-attribute; source fact; not reproduced; high; S6).

### Pattern Constraints: `concrete` and `symbolic`

In the Haskell backend, simplification rules can be restricted to specific pattern categories:
- `[simplification, concrete]`: matches only when all arguments are concrete (ground terms without variables or unevaluated functions) (CLM-0243; SRC-0036 docs/user_manual.md #concrete-and-symbolic-attributes-haskell-backend; source fact; not reproduced; high; S6).
- `[simplification, symbolic]`: matches only when all arguments are symbolic (CLM-0243; SRC-0036 docs/user_manual.md #concrete-and-symbolic-attributes-haskell-backend; source fact; not reproduced; high; S6).
- `[simplification, concrete(X), symbolic(Y)]`: selectively enforces that variable $X$ matches concrete terms while $Y$ matches symbolic terms (CLM-0243; SRC-0036 docs/user_manual.md #concrete-and-symbolic-attributes-haskell-backend; source fact; not reproduced; high; S6).

### Unbound Variables

Normally, every variable on the right-hand side of a rule must appear on the left-hand side. The `[unboundVariables(Vars)]` attribute relaxes this check for non-deterministic or random constructs:

```k
rule randBounded(M, N) => I
  requires M <=Int I andBool I <=Int N
  [unboundVariables(I)]
```

In symbolic verification, unbound variables represent arbitrary values satisfying the precondition (CLM-0244; SRC-0036 docs/user_manual.md #the-unboundvariables-attribute; source fact; not reproduced; high; S6).

### Memoization Caveats

The `[memo]` attribute hints to the Haskell backend to cache function results:
- Memoization applies only when all function arguments are fully concrete (CLM-0245; SRC-0036 docs/user_manual.md #limitations-of-memoization-with-the-haskell-backend; source fact; not reproduced; high; S6).
- Recursive functions should not be memoized directly because intermediate calls fill the cache with single-use entries; a worker-wrapper pattern must be used instead (CLM-0245; SRC-0036 docs/user_manual.md #limitations-of-memoization-with-the-haskell-backend; source fact; not reproduced; high; S6).
- Functions using uninterpreted predicates in side conditions must never be memoized, as incomplete solver knowledge leads to unsound cache incoherence (CLM-0245; SRC-0036 docs/user_manual.md #limitations-of-memoization-with-the-haskell-backend; source fact; not reproduced; high; S6).

## Sort Inference and Casting

When variables in rules lack explicit sort annotations, the frontend infers sorts using an algebraic subtyping algorithm based on SimpleSub (CLM-0246; SRC-0036 docs/developers/sort_inference.md #design; source fact; not reproduced; high; S6). Only maximal sort solutions are retained (CLM-0246; SRC-0036 docs/user_manual.md #variable-sort-inference; source fact; not reproduced; high; S6).

To eliminate ambiguities or guide execution, three cast operators are available:
1. Semantic cast `Term:Sort`: asserts that `Term` is of sort `Sort` or a subsort thereof; generates a pattern matching an injection into `Sort` (CLM-0246; SRC-0036 docs/user_manual.md #semantic-casts; source fact; not reproduced; high; S6).
2. Strict cast `Term::Sort`: asserts that `Term` is strictly and exactly of sort `Sort`; rejects subsorts, resolving parser ambiguities (CLM-0246; SRC-0036 docs/user_manual.md #strict-casts; source fact; not reproduced; high; S6).
3. Projection cast `{Term}:>Sort`: strips subsort injections and projects `Term` down to `Sort`; execution gets stuck if `Term` is not of sort `Sort` (CLM-0246; SRC-0036 docs/user_manual.md #projection-casts; source fact; not reproduced; high; S6).

## Verification Claims Format

Formal properties are specified in specification modules using `claim` sentences:

```k
module VERIFY-SPEC
  claim <k> runProgram => .K </k>
        <state> S:Map => ?S':Map </state>
        ensures isValid(?S')
        [one-path]
endmodule
```

Claims distinguish between reachability modalities:
- `[one-path]`: asserts that there exists at least one execution path satisfying the claim (CLM-0247; SRC-0036 docs/user_manual.md #all-path-and-one-path-attributes-to-distinguish-reachability-claims; source fact; not reproduced; high; S6).
- `[all-path]`: asserts that all valid execution paths satisfy the claim (CLM-0247; SRC-0036 docs/user_manual.md #all-path-and-one-path-attributes-to-distinguish-reachability-claims; source fact; not reproduced; high; S6).
- `[trusted]`: instructs `kprove` to accept the claim without proof, adding it immediately to the coinductive circularity hypothesis set (CLM-0247; SRC-0036 docs/user_manual.md #trusted-claims; source fact; not reproduced; high; S6).

In claims, existential variables on the right-hand side are denoted by `?Var`, while `?_` indicates an anonymous value that changes without constraint (CLM-0247; SRC-0036 docs/user_manual.md #summary; source fact; not reproduced; high; S6).

## Collection Patterns and Cell Fragments

Collection matching decomposes complex collections on the left-hand side of rules:
- `List`: matched via `ListItem(E1) L:List ListItem(E2)` (CLM-0248; SRC-0036 docs/user_manual.md #collection-patterns; source fact; not reproduced; high; S6).
- `Set`: matched via `SetItem(K1) S:Set SetItem(K2)`; elements `K1` and `K2` are guaranteed distinct (CLM-0248; SRC-0036 docs/user_manual.md #collection-patterns; source fact; not reproduced; high; S6).
- `Map`: matched via `K1 |-> V1 K2 |-> V2 M:Map`; keys `K1` and `K2` are guaranteed distinct (CLM-0248; SRC-0036 docs/user_manual.md #collection-patterns; source fact; not reproduced; high; S6).

Unbound variables within collection keys are forbidden, ensuring that keys can be evaluated concretely before table lookup occurs (CLM-0248; SRC-0036 docs/user_manual.md #collection-patterns; source fact; not reproduced; high; S6).

Cell fragments allow matching sub-trees of configurations and passing them as arguments to helper functions:

```k
rule <k> #collectOdd => collectOdd(<fs> Fs </fs>) ... </k>
     <fs> Fs </fs>
```

The fragment must be wrapped in its parent cell constructor at the call site to maintain well-sortedness (CLM-0249; SRC-0036 docs/user_manual.md #matching-on-cell-fragments; source fact; not reproduced; high; S6).

## Formatting and Unparsing Control

Pretty-printing of terms is configured using the `format` and `color` attributes:

```k
syntax Exp ::= Exp "+" Exp [format(%1 %2 %3), color(blue)]
```

The `format` string interprets escape sequences:
- `%n`: inserts a newline followed by current indentation level (CLM-0250; SRC-0036 docs/user_manual.md #format-attribute; source fact; not reproduced; high; S6).
- `%i`: increases indentation level by 1 (CLM-0250; SRC-0036 docs/user_manual.md #format-attribute; source fact; not reproduced; high; S6).
- `%d`: decreases indentation level by 1 (CLM-0250; SRC-0036 docs/user_manual.md #format-attribute; source fact; not reproduced; high; S6).
- `%c`: advances to the next color in the color list (CLM-0250; SRC-0036 docs/user_manual.md #format-attribute; source fact; not reproduced; high; S6).
- `%r`: resets text color to default terminal foreground (CLM-0250; SRC-0036 docs/user_manual.md #format-attribute; source fact; not reproduced; high; S6).
- `%N` ($N \ge 1$): prints the $N$th terminal or nonterminal child (CLM-0250; SRC-0036 docs/user_manual.md #using-the-integer-escape-sequence; source fact; not reproduced; high; S6).

## SMT Solver Translation Attributes

During symbolic execution, K translates predicates into SMTLIB2 format for discharge by Z3 (CLM-0251; SRC-0036 docs/user_manual.md #smt-translation; source fact; not reproduced; high; S6):
- `smtlib(sym)`: maps the production to an uninterpreted SMT function symbol `sym` (CLM-0251; SRC-0036 docs/user_manual.md #smt-translation; source fact; not reproduced; high; S6).
- `smt-hook(term)`: encodes the production using an SMTLIB2 term template, where `#1`, `#2` represent argument positions (e.g. `(mod (^ #1 #2) #3)`) (CLM-0251; SRC-0036 docs/user_manual.md #smt-translation; source fact; not reproduced; high; S6).
- `smt-lemma`: encodes a rule `rule LHS => RHS requires REQ` as a conditional SMT equality `(=> REQ (= LHS RHS))` (CLM-0251; SRC-0036 docs/user_manual.md #smt-translation; source fact; not reproduced; high; S6).

## Tool Options Reference

### Kompile Command Line Options

The `kompile` compiler driver governs pipeline execution and backend generation (CLM-0252; SRC-0036 docs/user_manual.md #manual-objectives; source fact; not reproduced; high; S6):
- `--backend [llvm|haskell]`: selects the compilation target backend (CLM-0252; SRC-0036 docs/user_manual.md #symbolic-and-concrete-attribute; source fact; not reproduced; high; S6).
- `--main-module M`: specifies the top-level execution module (defaults to filename basename) (CLM-0252; SRC-0036 pyk/docs/pipeline.md; source fact; not reproduced; high; S6).
- `--syntax-module M`: specifies the top-level parsing module for user programs (CLM-0252; SRC-0036 pyk/docs/pipeline.md; source fact; not reproduced; high; S6).
- `-O1`, `-O2`, `-O3`: controls LLVM backend optimization levels during C++ compilation (CLM-0252; SRC-0036 docs/ktools.md line 280; source fact; not reproduced; high; S6).
- `--enable-llvm-debug`: generates debug symbols required for GDB stepping and breakpointing (CLM-0252; SRC-0036 docs/ktools.md line 93; source fact; not reproduced; high; S6).
- `--emit-json`: emits compiled KAST definitions in JSON format (CLM-0252; SRC-0036 pyk/docs/pipeline.md line 89; source fact; not reproduced; high; S6).
- `--gen-bison-parser`: generates fast ahead-of-time LR(1) C parsers (CLM-0252; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).
- `--gen-glr-bison-parser`: generates ahead-of-time GLR parsers for ambiguous grammars (CLM-0252; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).
- `--bison-lists`: transforms list grammars to bounded-stack left-associative rules, eliminating GLR conflicts (CLM-0252; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).

### Krun, Kast, and Kprove Options

- `krun --debugger`: launches GDB (or LLDB on macOS) attached to the LLVM compiled interpreter (CLM-0253; SRC-0036 docs/ktools.md #debugging; source fact; not reproduced; high; S6).
- `krun --depth N`: halts execution after exactly $N$ rewrite steps (CLM-0253; SRC-0036 include/kframework/builtin/substitution.md line 88; source fact; not reproduced; high; S6).
- `krun --search`: initiates non-deterministic state-space exploration (CLM-0253; SRC-0036 pyk/docs/regression-triage.md line 79; source fact; not reproduced; high; S6).
- `kast -s Sort -m Module`: parses a program under the specified sort and module (CLM-0253; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).
- `kprove --default-claim-type [one-path|all-path]`: establishes default reachability modality for verification (CLM-0253; SRC-0036 docs/user_manual.md #all-path-and-one-path-attributes-to-distinguish-reachability-claims; source fact; not reproduced; high; S6).

## Critical Pitfalls and Specification Caveats

The user manual highlights several critical failure modes:
1. *GLR Parser Memory Exhaustion*: Bison GLR parsers handle grammatical ambiguities by branching; grammars with high conflict counts consume exponential stack space and abort with "memory exhausted" (CLM-0254; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).
2. *Unsound Simplification Lemmas*: Haskell backend simplifications that do not preserve definedness cause the prover to prove false claims (CLM-0254; SRC-0036 docs/user_manual.md #simplification-attribute; source fact; not reproduced; high; S6).
3. *Memoization with Uninterpreted Predicates*: Caching function calls whose side conditions depend on uninterpreted predicates leads to unsound cache collisions (CLM-0254; SRC-0036 docs/user_manual.md #the-memo-attribute; source fact; not reproduced; high; S6).
4. *Bison Parser Limitations*: Ahead-of-time bison parsers ignore `prefer` and `avoid` attributes and do not expand macros; semantics authors must use anywhere rules when targeting bison parsing (CLM-0254; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).
5. *Reserved LLVM Priorities*: Assigning rule priorities between 50 and 150 disrupts internal LLVM pattern match ordering (CLM-0254; SRC-0036 docs/user_manual.md #owise-and-priority-attributes; source fact; not reproduced; high; S6).

For builtin standard library domains, see [k-builtins](k-builtins.md). For backend architectures and the compilation pipeline, see [k-backends-and-tools](k-backends-and-tools.md). For formal ZKIR circuit IR definitions, see [zkir-instruction-set](../zkir/zkir-instruction-set.md) and [zkir-vm-semantics](../zkir/zkir-vm-semantics.md).
