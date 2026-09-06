---
id: k-framework.best-practices
type: reference
title: K best practices for the ZKIR definition
status: active
updated_at: 2026-09-05T20:15:19Z
sources:
  - SRC-0039
  - SRC-0036
---

# K Best Practices for the ZKIR Definition

This document establishes the normative engineering standards and semantic modeling best practices for developing, executing, and formally verifying Zero-Knowledge Intermediate Representation (ZKIR v3) semantics within the K Framework (v7.1.337). Formal semantics developed for zero-knowledge virtual machines must balance two conflicting engineering objectives: high-performance concrete execution via the LLVM backend for simulation and differential fuzzing against native Rust implementations, and clean algebraic properties suitable for symbolic execution, reachability logic theorem proving (`kprove`), and interactive proof manipulation through the `kore-rpc` interface and `pyk`. Adhering to the guidelines documented here ensures that semantic definitions remain mathematically sound, executionally performant, and fully portable across both the concrete LLVM and symbolic Haskell backends.

## 1. Functional Programming Discipline

Functions in K provide deterministic, instantaneous term rewriting without creating execution steps or branching the transition graph. Adhering to strict functional discipline prevents runtime interpreter crashes and proof divergences.

### Total and Partial Functions

In the K Framework, a syntax production annotated with the `[function]` attribute defines a functional symbol rather than a state constructor (CLM-0800; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Functional rules evaluate immediately at the site of term creation and are treated as equations rather than operational transitions (CLM-0801; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). 

A function should be explicitly annotated with the `[total]` attribute whenever it is mathematically defined over all possible terms inhabiting its argument sorts (CLM-0802; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). The `[total]` attribute signals to the compiler frontend and backends that pattern matching over the domain is exhaustive and that evaluation cannot fail (CLM-0803; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).

When a function is partial—meaning it lacks defining rules for certain input patterns—invoking that function on an unhandled input term reduces to matching logic bottom (`#Bottom`), representing logical undefinedness (CLM-0804; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). In the LLVM backend, encountering `#Bottom` during concrete execution causes the generated native interpreter to crash or terminate with an unhandled exception (CLM-0805; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). In the Haskell backend, reachability claims whose right-hand side reduces to `#Bottom` cannot be verified because checking satisfiability of `#Bottom` is generally undecidable, causing the prover to emit warnings or diverge in recursive loops (CLM-0806; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). The official K documentation explicitly mandates writing total functions as the most preferred design option to prevent definedness hazards across all backends (CLM-0807; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Rule Priority and Fallback Rules with `owise`

To achieve totality without duplicating complex pattern guards, K provides the `[owise]` rule attribute (CLM-0808; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). A rule annotated with `[owise]` acts as a fallback case that matches only after all higher-priority rules have been attempted and failed to match (CLM-0809; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6).

The `[owise]` attribute operates by assigning the rule an implicit numerical priority of 200 (CLM-0810; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). In contrast, standard function rules without explicit priority annotations are automatically assigned an implicit default priority of 50 (CLM-0811; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). The K execution engine evaluates matching rules in strictly ascending numerical order, trying priority 0 first, then priority 50, and finally priority 200 (CLM-0812; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). 

Rules can also be assigned explicit numerical priorities using `[priority(N)]`, where smaller integers take precedence over larger integers (CLM-0813; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). If multiple rules for the same function have identical priorities and overlapping patterns that evaluate to differing right-hand sides, the function is ill-defined and execution becomes non-deterministic, which violates the algebraic requirement of functional confluence (CLM-0814; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). Developers must ensure that all overlapping rules either carry distinct priorities or evaluate to identical normal forms.

### Simplification Rules and Definedness

Simplification rules, designated by the `[simplification]` attribute, define rewrite equalities outside the primary operational semantics to simplify functional terms during deduction and symbolic evaluation (CLM-0815; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Unlike standard function definition rules that apply via unification, simplification rules apply strictly by pattern matching on the function's arguments (CLM-0816; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). This fundamental distinction allows function symbols to appear nested within arguments on the left-hand side of a simplification rule, a construct strictly rejected in regular function definitions (CLM-0817; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

Simplification rules accept an optional priority integer that defaults to 50 when omitted (CLM-0818; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). However, compiler backends are free to apply simplification rules at any time during proof search, meaning developers must ensure that every simplification lemma is logically sound regardless of the application order (CLM-0819; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Furthermore, simplification rules must strictly preserve definedness: if the left-hand side contains a partial function that evaluates to `#Bottom`, the right-hand side must evaluate to `#Bottom`, or the rule must supply an `ensures false` or `requires false` clause to avoid introducing unsound proofs (CLM-0820; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Controlling Simplifications: `concrete` and `symbolic`

In symbolic verification with the Haskell backend, unconstrained simplification rules can easily lead to non-terminating rewrite cycles. To prevent cyclic re-association while still permitting evaluation, rules can be restricted using the `[concrete]` and `[symbolic]` attributes (CLM-0821; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

When a simplification rule is annotated with `[concrete]`, it matches only if all of its argument terms are completely concrete, containing no logical variables or unevaluated functions (CLM-0822; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Conversely, the `[symbolic]` attribute requires all arguments to contain symbolic variables or unevaluated terms (CLM-0823; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Finer granularity is achieved by passing variable subsets, such as `[concrete(X, Y), symbolic(A)]`, which allows developers to direct algebraic commutativity and associativity so that concrete constants collect together and simplify without triggering infinite symbolic loops (CLM-0814, CLM-0815; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### SMT Integration: `smtlib`, `smt-hook`, and `smt-lemma`

The Haskell backend delegates arithmetic and logical satisfiability checks to the Z3 SMT solver during symbolic execution. Three attributes govern the translation of K symbols into SMT-LIB2 logic:
1. `smtlib(symbol)`: declares a new SMT function symbol in Z3 with uninterpreted function semantics, allowing the solver to reason about equality over foreign constructors (CLM-0824; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).
2. `smt-hook(term)`: maps a K production directly to an expression in SMT-LIB2 syntax using pre-declared solver primitives, referencing production arguments via `#1`, `#2`, and positional tokens (CLM-0825; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).
3. `[smt-lemma]`: annotates a conditional rewrite rule `rule LHS => RHS requires REQ` so that K translates it into a quantified conditional equality `(=> REQ (= LHS RHS))` sent directly to Z3's assertion stack (CLM-0826; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Every symbol in an `smt-lemma` must carry an `smtlib` or `smt-hook` declaration.

---

## 2. Rule Construction and Execution Discipline

Operational transition rules define how a program state evolves over time. Clean rule construction ensures readability, high performance, and rapid automated verification.

### Preconditions and Postconditions

A rule's applicability is constrained using the `requires` keyword, which introduces a boolean side condition that must evaluate to `true` under the matched substitution before the rule can fire (CLM-0827; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). 

When a rule introduces fresh existential variables on its right-hand side using the `?X` syntax, the rule can specify logical constraints on those variables using the `ensures` keyword (CLM-0828; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/21_symbolic_execution/; source fact; reproduced; high; S6). While `requires` constrains the existing state prior to the step, `ensures` constrains the newly created state after the step, providing the logical foundation for symbolic branching and reachability claims (CLM-0829; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/21_symbolic_execution/; source fact; reproduced; high; S6).

### Configuration Framing and Local Rewrites

K configurations organize machine state into hierarchical, labelled cells. Rules should specify only the minimal set of cells necessary to execute the transition, relying on K's configuration concision to automatically infer the surrounding cell hierarchy (CLM-0830; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). 

Within individual cells containing lists, sets, maps, or computations, the ellipsis notation `...` provides structural framing (CLM-0831; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). For example, `<k> PGM ... </k>` matches `PGM` at the head of the computation while leaving the remainder of the continuation intact (CLM-0832; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). 

Furthermore, developers should prefer local rewrite syntax (`=>`) inside cells rather than rewriting whole cell contents (CLM-0833; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Writing `<mem> M => M[X <- V] </mem>` expresses the precise local mutation and avoids duplicating unmodified environment terms across the left and right sides of the rule (CLM-0834; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Rule Priorities and Anywhere Rules

Just as with functions, transition rules can carry explicit priority annotations `[priority(N)]` to resolve rule contention and enforce deterministic execution order (CLM-0835; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/07_side_conditions/; source fact; reproduced; high; S6). In virtual machine semantics, high-priority rules (e.g., priority 20–30) are commonly assigned to trap error states, panic halts, and gas exhaustion before normal instruction dispatch at default priority 50 takes place.

Rules annotated with the `[anywhere]` attribute apply anywhere within the configuration hierarchy where their left-hand side matches, operating independently of the `<k>` cell context (CLM-0836; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). `anywhere` rules are typically used to maintain global algebraic normal forms, such as re-associating expression trees or simplifying independent state subterms after each execution step (CLM-0837; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Compile-Time Metaprogramming: Macros and Aliases

A production tagged with `[macro]` or `[alias]` defines a syntactic substitution applied statically during compilation and before program execution (CLM-0838; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Macro rules cannot have side conditions, and to prevent infinite loops, they do not expand recursively within their own expansion unless explicitly declared with `[macro-rec]` (CLM-0839; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). When a macro intentionally contains variables on its right-hand side that are unbound on the left, the compiler requires the `[unboundVariables(...)]` attribute to pass well-formedness checks (CLM-0840; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Continuation Discipline and Strictness

The primary computation cell `<k>` organizes execution as a sequential task pipeline using the associative continuation operator `~>`, which terminates in `.K` or `.` (CLM-0841; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/14_evaluation_order/; source fact; reproduced; high; S6). 

In traditional programming language semantics, operators use `[strict]` or `[seqstrict]` attributes to evaluate nested sub-expressions (CLM-0842; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). The K compiler desugars `[strict]` into heating rules that freeze the outer context and place the sub-expression on top of the `<k>` cell, and cooling rules that plug the evaluated result back into the context hole (CLM-0843; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). The `KResult` sort and `isKResult` predicate break potential infinite heating/cooling loops by guaranteeing that fully evaluated terms cannot be heated again (CLM-0844; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

However, an abstract syntax definition whose programs are parsed and generated externally—such as a ZKIR definition driven by `pyk` or compiled from JSON—has no need for heating, cooling, or syntactic strictness (CLM-0845; SRC-0039 https://kframework.org/docs/user_manual/; inference; reproduced; high; S6). In such systems, instructions are already flattened into linear instruction sequences and all sub-terms are explicit operands. Eliminating `strict` annotations completely bypasses heating/cooling cycles, drastically reducing the generated state space and maximizing execution speed (CLM-0846; SRC-0039 https://kframework.org/docs/user_manual/; inference; reproduced; high; S6).

---

## 3. Collection Domains: Map, List, and Set

K provides built-in collection sorts in `domains.md` that model machine state. Proper structural access patterns are essential for both execution performance and symbolic satisfiability.

### Map Operations and Indexing

The `Map` sort represents finite associative mappings from `KItem` to `KItem`, constructed using `Key |-> Value` and the empty map `.Map` (CLM-0847; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). 

In the LLVM backend, map matching executes in $O(1)$ constant time if all keys mentioned in the left-hand side pattern are deterministically known or bound (CLM-0848; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). However, if map keys contain unbound variables that must be discovered by non-deterministic search, matching incurs a severe polynomial performance penalty: linear time for one unbound key, quadratic for two, and cubic for three (CLM-0849; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6).

Direct lookup using the map indexing operator `M[Key]` is a partial function. Attempting to look up a non-existent key fails with `#Bottom`, which causes the LLVM runtime to abort (CLM-0850; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). To prevent lookup crashes, rules must test key existence using the boolean predicate `Key in_keys(M)` in a `requires` side condition before performing the lookup projection (CLM-0851; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6).

### List Operations: Cons versus Append

The `List` sort represents associative sequences composed of `ListItem(Item)` and the empty list `.List` (CLM-0852; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). List patterns supporting prefix and suffix decomposition—such as `ListItem(Head) Rest:List` or `Prefix:List ListItem(Tail)`—match in $O(\log N)$ time (CLM-0853; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). 

Modern K implements the `ARRAY` module directly over the `List` sort, providing $O(\log N)$ indexed lookup `Arr[I]` and functional update `Arr[I <- V]` (CLM-0854; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).

For state accumulation (such as collecting execution traces, outputs, or emitted constraints), prepending elements (`ListItem(X) L`, cons) is an $O(1)$ constant-time operation (CLM-0855; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). In contrast, appending to the end of a list (`L ListItem(X)`) or concatenating two lists requires rebalancing the underlying associative binary tree (CLM-0856; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). When collecting thousands of instructions or constraints, developers should accumulate in reverse order using cons and reverse the collection upon completion, or isolate appends to bounded vectors.

### Set Operations and Set Variables

The `Set` sort represents unordered collections of deduplicated elements (`SetItem(Item)` and `.Set`) (CLM-0857; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6). When matching against a set pattern containing multiple items, K guarantees that all matched items represent distinct elements; pattern matching fails if distinct witnesses cannot be identified (CLM-0858; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/16_collections/; source fact; reproduced; high; S6).

Set variables, denoted by an `@` prefix (e.g., `@S:Set`), represent sets of states within Matching Mu Logic (CLM-0859; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). They are used to write simplification rules that handle undefinedness or non-deterministic disjunctions (CLM-0860; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). Set variables are supported exclusively by the Haskell backend; the LLVM backend cannot compile rules containing set variables (CLM-0861; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

---

## 4. Builtin Domains and Hooks

The K standard library provides native primitive types implemented through C++ hooks in LLVM and Haskell libraries in the symbolic prover.

### Integer Arithmetic (`INT`)

Integers in K have arbitrary precision and are backed directly by the GNU Multiple Precision Arithmetic Library (GMP) in the LLVM backend, guaranteeing immunity from overflow and underflow (CLM-0862; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).

For cryptographic and circuit semantics, division and modulo conventions require careful discrimination:
1. *Euclidean modulo (`modInt`)*: satisfies the Euclidean division theorem, guaranteeing that the remainder is strictly non-negative ($0 \le r < |d|$) regardless of operand signs (CLM-0863; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). This is the correct operator for prime field arithmetic.
2. *Truncated division and remainder (`/Int`, `%Int`)*: truncates quotients toward zero ($t$-division), which can yield negative remainders when operands are negative (CLM-0864; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).
3. *Modular exponentiation (`^%Int`)*: hooks directly into GMP's `mpz_powm` via the `INT.powmod` hook, computing $(base^{exp}) \pmod{mod}$ efficiently without materializing astronomical intermediate powers (CLM-0865; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).
4. *Bitwise operations*: `<<Int` and `>>Int` provide arbitrary-width logical bit shifts, while `&Int`, `|Int`, and `xorInt` provide bitwise boolean operations over two's-complement representations (CLM-0866; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).

### Byte Arrays (`BYTES`)

Byte sequences are managed through the `BYTES` module, which represents immutable byte buffers (CLM-0867; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). Key primitive hooks include:
- `Int2Bytes(length, value, endianness)`: converts an arbitrary integer into a fixed-width byte array using big-endian or little-endian byte ordering (CLM-0868; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).
- `substrBytes(bytes, start, end)`: extracts sub-slices in $O(1)$ or linear time (CLM-0869; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).
- `replaceAtBytes(dest, offset, src)`: updates byte buffers at specified byte offsets (CLM-0870; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).
- `padRightBytes(bytes, length, fillByte)`: pads a byte buffer to a specified width (CLM-0871; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6).

### Strings (`STRING`)

The `STRING` domain provides string manipulation primitives including string concatenation `+String`, string length `lengthString`, substring extraction `substrString`, and string-integer conversions `String2Int` and `Int2String` (CLM-0872; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). In virtual machine semantics, `String` is commonly used for symbolic register identifiers, variable names, and error payloads.

### Universal Structural Equality (`K-EQUAL`)

The `K-EQUAL` module provides polymorphic structural equality across any K sort using the `==K` and `=/=K` operators, which evaluate to terms of sort `Bool` (CLM-0873; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). In contrast, the pattern-matching operators `:=K` and `:/=K` express model membership, evaluating whether a term matches a specified pattern structure (CLM-0874; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Machine Integers (`MINT`)

The `MINT` module provides fixed-width bitvector arithmetic (`MInt{W}`) instantiated using the constructor `mi(Width, Value)` (CLM-0875; SRC-0039 https://kframework.org/k-distribution/include/kframework/builtin/domains/; source fact; reproduced; high; S6). While useful for modeling hardware architectures with strict 32-bit or 64-bit truncation, cryptographic circuit virtual machines like ZKIR typically operate over large prime order scalar fields (e.g., BLS12-381 $r$) and 256-bit foreign curves. For such domains, GMP-backed arbitrary-precision `Int` with explicit `modInt` reductions provides far greater flexibility and superior SMT translation properties than fixed-width `MInt`.

---

## 5. Backend Interoperability and Pyk Integration

Developing a cross-backend K definition requires structuring semantics so that both the concrete LLVM backend and the symbolic Haskell backend interpret the identical Kore specification without divergence.

### The LLVM Backend

The LLVM backend is K's high-performance concrete execution engine (CLM-0876; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/20_backends/; source fact; reproduced; high; S6). It compiles K rules into LLVM intermediate representation and native machine code, optimizing execution via LLVM compiler passes (`-O1`, `-O2`, `-O3`) (CLM-0850, CLM-0851; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/20_backends/; source fact; reproduced; high; S6). It supports `--emit-json` to export compiled definitions and configurations as JSON structures, and `--enable-llvm-debug` to generate native debugging symbols (CLM-0877; SRC-0039 https://kframework.org/docs/cheat_sheet/; source fact; reproduced; high; S6).

### The Haskell Backend and Theorem Prover

The Haskell backend is an interpretive symbolic execution engine and deductive theorem prover (CLM-0878; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/20_backends/; source fact; reproduced; high; S6). It evaluates formal reachability claims declared with the `claim` keyword using `kprove`, proving whether all execution paths (`all-path`) or at least one execution path (`one-path`) reaching a specified postcondition satisfy invariant constraints (CLM-0879; SRC-0039 https://kframework.org/k-distribution/k-tutorial/1_basic/20_backends/; source fact; reproduced; high; S6). The Haskell backend is also deployed as a persistent JSON-RPC server (`kore-rpc`) that allows external client tools to perform step-wise symbolic execution, state-merging, and proof-graph traversal (CLM-0880; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).

### Cross-Backend Compatibility Rules

To ensure a K definition compiles and executes identically across both backends, developers must observe several compatibility rules:
1. Eliminate partial functions: ensure all functions provide exhaustive pattern coverage or explicit `[owise]` fallbacks to prevent LLVM `#Bottom` crashes (CLM-0803, CLM-0854; SRC-0039 https://kframework.org/docs/user_manual/; inference; reproduced; high; S6).
2. Avoid backend-specific attributes: do not use set variables (`@`), uninterpreted functions lacking operational rewrite rules, or existential RHS variables without concrete generator rules unless guarded by `[concrete]` or `[symbolic]` module separation (CLM-0841, CLM-0854; SRC-0039 https://kframework.org/docs/user_manual/; inference; reproduced; high; S6).
3. Provide concrete operational semantics for all symbols: symbols declared with `smtlib` or `smt-hook` for Z3 reasoning must still carry operational rewrite rules if they are to be evaluated in the LLVM backend (CLM-0881; SRC-0039 https://kframework.org/docs/user_manual/; inference; reproduced; high; S6).

### Pyk Tooling Integration

`pyk` is the official Python SDK for interacting programmatically with the K Framework. It enables building end-to-end compiler pipelines and proof orchestrators:
- AST Serialization: `pyk` converts between Python-native syntax trees and Kore via `kast_to_kore` and `kore_to_kast` (CLM-0882; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).
- Execution Drivers: `pyk` wraps the `krun` command via `KRun` for concrete test execution, and wraps `kprove` via `KProve` for batch verification (CLM-0883; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).
- RPC Proof Architecture: in modern verification workflows, `pyk` communicates directly with `kore-rpc` using the `KoreClient` interface, constructing Proof Trees (KCFGs) and interactively guiding branch exploration without spawning sub-processes (CLM-0853, CLM-0856; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).

---

## 6. Symbol Naming, Syntax Declarations, and Grammar Design

The interface between K and external tools depends on predictable symbol names and clean grammar declarations.

### Explicit Symbol Naming with `symbol(_)`

By default, the K compiler automatically generates internal identifiers (mangled KLabels) for each syntactic production, encoding module names, argument sorts, and parameter positions (e.g., `'Lblfoo'LParUndsRParUnds'MYMODULE'Unds'FooBarBaz`) (CLM-0884; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). These auto-generated strings are fragile and break whenever sort signatures or module hierarchies change (CLM-0885; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

Applying the `[symbol(name)]` attribute to a production explicitly overrides this behavior, assigning an unmangled identifier (e.g., `Lblname`) in the compiled Kore definition (CLM-0886; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). The compiler strictly enforces global uniqueness across all explicitly declared symbols (CLM-0887; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Syntactic List Terminators with `terminator-symbol(_)`

When a grammar declares a syntactic list production using `List{Sort, Delimiter}`, K desugars the syntax into a cons production and an empty list terminator production (CLM-0888; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). By default, the terminator label is mangled. Developers can assign an explicit, clean symbol name to the empty list using the `[terminator-symbol(name)]` attribute (CLM-0889; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Overloading and Subsort Disambiguation

K supports subsort overloading via the `[overload(operator)]` attribute (CLM-0890; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). When two productions share identical syntax but operate over subsorted domains (for example, expressions versus left-values), marking both productions with the same `overload(...)` key instructs the parser to select the most specific production in the subsort partial order, resolving ambiguity without requiring manual AST casts (CLM-0891; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6). The legacy approach of pairing `klabel(_)` with `symbol` is deprecated and superseded by `symbol(_)` and `overload(_)` (CLM-0892; SRC-0039 https://kframework.org/docs/user_manual/; source fact; reproduced; high; S6).

### Referencing Symbols in Pyk

External programmatic drivers written in `pyk` construct abstract syntax trees directly in KORE JSON format. Using explicit `[symbol(...)]` attributes on every constructor and operator provides a stable, human-readable API contract that protects `pyk` scripts against internal compiler mangling updates (CLM-0893; SRC-0039 https://kframework.org/docs/user_manual/; inference; reproduced; high; S6).

---

## 7. Testing and Debugging Workflows

Debugging complex semantics requires utilizing K's multi-tiered debugging infrastructure.

### Execution Bounding and State Exploration

When debugging infinite loops or divergence during execution, the `--depth N` flag to `krun` limits rewriting to exactly $N$ steps, printing the intermediate configuration state at step $N$ (CLM-0894; SRC-0039 https://kframework.org/docs/cheat_sheet/; source fact; reproduced; high; S6).

To analyze non-deterministic behavior or explore all branching outcomes, `krun --search` or `--search-all` commands the execution engine to explore the full reachable transition graph, collecting all final or intermediate states (CLM-0895; SRC-0039 https://kframework.org/docs/cheat_sheet/; source fact; reproduced; high; S6).

### Compiler Debugging and AST Inspection

Passing `--enable-llvm-debug` to `kompile` builds the LLVM backend binary with native debugging symbols and disable optimizations that obscure execution traces (CLM-0896; SRC-0039 https://kframework.org/docs/cheat_sheet/; source fact; reproduced; high; S6).

The `kast` CLI tool parses source programs or expressions and unparses them into structured KAST or KORE formats (CLM-0897; SRC-0039 https://kframework.org/docs/cheat_sheet/; source fact; reproduced; high; S6). Running `kast --output json input.file` verifies that concrete syntax parses into the intended AST before debugging operational rules (CLM-0898; SRC-0039 https://kframework.org/docs/cheat_sheet/; source fact; reproduced; high; S6).

### Interactive Debugging with GDB

The LLVM backend integrates natively with GDB when executed with `krun --debugger` or when invoking the compiled interpreter binary directly (CLM-0899; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6):
1. Step Breakpoint: setting a breakpoint on `definition.kore:k_step` breaks the debugger before every execution step, displaying the full configuration term in the frame argument (CLM-0900; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).
2. Rule Breakpoints: rules given an explicit label `rule [label]: ...` generate distinct C++ functions in the compiled interpreter. Setting a breakpoint on `MODULE.label.rhs` breaks when the rule applies, exposing the variable substitution in the stack frame (CLM-0901; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6). Setting a breakpoint on `MODULE.label.sc` breaks when the rule's side condition is evaluated, allowing inspection of the candidate variables (CLM-0902; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).
3. Function Breakpoints: setting a breakpoint on a function's mangled symbol `Lblfoo...` breaks upon function entry, exposing function arguments as numbered parameters `_1`, `_2`, etc. (CLM-0903; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).
4. Auto-load Safe Path: to enable Python pretty-printing of K terms within GDB, users must configure `set auto-load safe-path <path>` in `~/.gdbinit`, granting GDB permission to load the interpreter's bundled runtime scripts (CLM-0904; SRC-0039 https://kframework.org/docs/ktools/; source fact; reproduced; high; S6).

---

## 8. Applied to the ZKIR Definition

This section audits the ZKIR v3 semantics implementation located in `experiments/zkir-k/semantics/` (`zkir-vm.k`, `zkir-ops.k`, `zkir-constraints.k`, and `zkir-values.k`), evaluating its adherence to each established best practice.

1. *Totality of Preimage Projections*: In `zkir-vm.k`, all preimage field extraction functions (`#preInputs`, `#preBinding`, `#preComm`, `#prePrivate`, `#prePubIn`, `#prePubOut`) and decoding helpers are explicitly annotated with `[function, total]`, ensuring that configuration initialization never produces `#Bottom` (CLM-0905; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
2. *Totality and Fallback Handling in Value Operations*: In `zkir-ops.k`, arithmetic and curve operations (`addV`, `mulV`, `negV`, `invV`, `ecMulV`, `fromCoordinatesV`) are declared `[function, total]` and implement exhaustive `[owise]` fallbacks returning typed error payloads (`vErr`, `pErr`, `cErr`), completely eliminating LLVM `#Bottom` crashes on malformed circuit inputs (CLM-0906; experiments/zkir-k/semantics/zkir-ops.k; repository observation; not reproduced; medium; S3).
3. *Rule Priority for Witness Error Handling*: In `zkir-vm.k`, the rule that handles `#exec(impact(_, Xs))` once the witness has failed carries `[priority(30)]` so that it wins over the generic rule that discards any `#exec(_)` in the error state; without the priority both rules would match and the public-input index bookkeeping (`<deadPis>`) would depend on the backend's rule order (CLM-0907; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
4. *Configuration Framing and Local Rewriting*: In `zkir-vm.k`, transition rules strictly utilize `...` cell framing and local rewrite syntax `<mem> M => M[X <- V] </mem>`, preserving clean structural modularity (CLM-0908; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
5. *Continuation Discipline*: In `zkir-vm.k`, instruction execution is sequenced linearly on the `<k>` cell using the continuation operator `I ~> Is`, terminating at `#finish` upon exhaustion of the instruction stream (CLM-0909; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
6. *Omission of Heating and Cooling*: Because ZKIR programs enter as abstract terms built by `tools/zkir_kast.py` and every instruction operand is already a value or a register name, `zkir-vm.k` needs no `strict` annotations, heating/cooling rules or `isKResult` predicates; each instruction is one rewrite (CLM-0910; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
7. *Guarded Map Access*: In `zkir-vm.k` and `zkir-constraints.k`, operand resolution (`resolve`, `rd`) tests variable existence in register memory using `requires X in_keys(M)` before projecting `{M[X]}:>Value`, returning an explicit error string if the register is unbound (CLM-0911; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
8. *Collection Performance on Trace Accumulation*: In `zkir-vm.k`, public inputs `<pi>` and circuit constraints `<constraints>` are accumulated by appending at the end (`Cs => Cs ListItem(...)`), which keeps them in program order; the LLVM backend's `List` is a persistent sequence, so append is not a bottleneck at the corpus sizes seen (at most a few hundred instructions), but this has not been measured on larger circuits (CLM-0912; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
9. *Integer and Bitwise Builtins*: In `zkir-values.k`, prime field reduction and foreign-field limb encoding (`encForeign`, `decForeign`) rely on GMP-backed `Int` with `modInt` and bit shifts (`<<Int`, `>>Int`), ensuring exact cryptographic arithmetic without bit-truncation errors (CLM-0913; experiments/zkir-k/semantics/zkir-values.k; repository observation; not reproduced; medium; S3).
10. *Byte Array Management*: In `zkir-values.k`, 32-byte hash inputs and outputs are modeled using the `BYTES` module, utilizing `padRightBytes` to initialize zeroed byte buffers and `substrBytes` for byte extraction (CLM-0914; experiments/zkir-k/semantics/zkir-values.k; repository observation; not reproduced; medium; S3).
11. *Universal Structural Equality*: In `zkir-ops.k`, value comparisons for `test_eq` and `constrain_eq` utilize `==K` and `=/=K` from `K-EQUAL` across polymorphic `Value` terms, preventing sort-mismatch ambiguities (CLM-0915; experiments/zkir-k/semantics/zkir-ops.k; repository observation; not reproduced; medium; S3).
12. *Explicit Symbol Naming*: In `zkir-values.k`, `zkir-vm.k`, and `zkir-constraints.k`, all primary syntax constructors (`nativeV`, `bytes32V`, `jubjubPointV`, `gate`, `verdict`, `job`, `preimage`) are explicitly annotated with `[symbol(...)]`, presenting clean, stable AST interfaces to external `pyk` testing harnesses (CLM-0916; experiments/zkir-k/semantics/zkir-values.k; repository observation; not reproduced; medium; S3).
13. *Dual-Backend Compatibility*: The definition uses total functions with `owise` fallbacks, no set variables and no uninterpreted symbols, which are the preconditions for running under the Haskell backend as well; whether it kompiles and executes under the Haskell backend has not been verified at the time of writing and is one of the questions of the 2026-09-05 review (CLM-0917; experiments/zkir-k/semantics/zkir-vm.k; repository observation; not reproduced; medium; S3).
