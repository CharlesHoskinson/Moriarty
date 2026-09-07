---
id: k.framework.tutorial-intermediate
type: language
title: K tutorial section 2, intermediate concepts
status: active
updated_at: 2026-09-03T14:34:08Z
sources:
  - SRC-0036
created: 2026-09-03
updated: 2026-09-07
tags:
  - moriarty
  - research
---

# K Tutorial Section 2, Intermediate Concepts

This document synthesizes Section 2 of the K Framework tutorial, covering intermediate language design, evaluation strategies, term representations, and verification mechanisms. It supplements the foundational concepts introduced in [k-tutorial-basic](k-tutorial-basic.md) and contextualizes the runtime architecture analyzed in [k-framework-overview](k-framework-overview.md).

## Section 2 Architecture and Repository Structure

Section 2 of the K tutorial comprises seventeen topical units designed to be studied independently or sequentially (CLM-0200; SRC-0036 k-distribution/k-tutorial/2_intermediate/README.md; source fact; not reproduced; high; S6). Inspection of the repository demonstrates that the directory `k-distribution/k-tutorial/2_intermediate/` contains directories `01_macros` through `17_debugging_proofs`, but lessons `02_fresh_constants` through `17_debugging_proofs` exist on disk as minimal stubs with navigation links pointing to the normative K user manual (CLM-0200; SRC-0036 k-distribution/k-tutorial/2_intermediate/README.md; repository observation; not reproduced; high; S6). In this synthesis, each intermediate lesson is reconstructed comprehensively from its controlling definitions in the user manual, builtin standard library modules, and tool documentation (CLM-0200; SRC-0036 docs/user_manual.md; source fact; not reproduced; high; S6).

## Lesson 2.1: Macros, Aliases, and Anywhere Rules

Lesson 2.1 introduces three specialized rule forms that alter the timing, reversibility, or syntactic placement of term rewriting (CLM-0201; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6).

### Macros and Syntactic Desugaring

A production tagged with the `macro` attribute designates its corresponding rules as static transformations:

> "A production is a macro if it has the `macro` attribute, and all rules whose top symbol on the left hand side is a macro are macro rules which define the behavior of the macro." (CLM-0201; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6).

Unlike top-level transition rules, macro rules do not participate in configuration cell completion (CLM-0201; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6). Macro rules execute statically during compilation and before program execution on the initial configuration, eliminating runtime step overhead for syntactic desugaring (CLM-0201; SRC-0036 docs/user_manual.md #macros-and-aliases; source fact; not reproduced; high; S6).

```k
module LESSON-01
  imports BOOL

  syntax Stmt ::= "if" "(" Exp ")" Stmt             [macro]
                | "if" "(" Exp ")" Stmt "else" Stmt
                | "{" Stmts "}"
  syntax Stmts ::= List{Stmt,""}
  syntax Exp ::= Bool

  rule if ( E ) S => if ( E ) S else { .Stmts }
endmodule
```

By default, macros do not expand recursively within their own expansion to ensure termination during static compilation passes (CLM-0202; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6). When recursive expansion is required, the `macro-rec` attribute instructs the compiler to expand the macro until a fixpoint is reached:

```k
syntax Exp ::= "int" Exp ";" | "int" Exps ";" [macro-rec] | Exp Exp | Id
syntax Exps ::= List{Exp,","}

rule int X:Id, X':Id, Xs:Exps ; => int X ; int X', Xs ;
```

Under `macro-rec`, the input `int x, y, z;` expands completely to `int x; int y; int z;` (CLM-0202; SRC-0036 docs/user_manual.md #macros-and-aliases; source fact; not reproduced; high; S6).

### Reversible Aliases

An alias production, declared using `alias` or `alias-rec`, behaves identically to a macro during forward compilation, but is applied in reverse prior to unparsing:

> "An alias in K is very similar to a macro, with the exception that the rewrite rule will also be applied backwards during the pretty-printing process." (CLM-0203; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6).

When unparsing the final configuration, the printer treats the right-hand side of the alias rule as a match pattern and replaces it with the left-hand side (CLM-0203; SRC-0036 docs/user_manual.md #macros-and-aliases; source fact; not reproduced; high; S6). This preserves user-level syntax in tool output while allowing semantics to operate over normalized desugared AST representations.

### Anywhere Rules

An anywhere rule is declared by placing the `anywhere` attribute on a rule sentence:

```k
syntax Stmt ::= Stmt ";" Stmt

rule (S1 ; S2) ; S3 => S1 ; (S2 ; S3) [anywhere]
```

Anywhere rules apply at any position in the configuration matching the left-hand side, without cell completion, yet the matched top symbol remains a valid constructor that other rules can match against (CLM-0204; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6). The symbol behaves as a constructor modulo the equations defined by the anywhere rules (CLM-0204; SRC-0036 docs/user_manual.md #anywhere-rules; source fact; not reproduced; high; S6). Crucially, the `anywhere` attribute is supported only on the LLVM backend; compiling definitions containing anywhere rules on the Haskell backend triggers a compilation error (CLM-0204; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; source fact; not reproduced; high; S6).

## Lesson 2.2: Fresh Constants

K provides native support for generating distinct values during rewriting through the exclamation-point syntax `!VARNAME:VarSort` (CLM-0205; SRC-0036 docs/user_manual.md #freshgenerator-attribute; source fact; not reproduced; high; S6).

```k
rule <k> new var x ; => . ... </k>
     <env> ENV => ENV [ x <- !I:Int ] </env>
     <mem> MEM => MEM [ !I <- 0     ] </mem>
```

Fresh constants can only appear on the right-hand side of a rule (CLM-0205; SRC-0036 include/kframework/builtin/kast.md #variables-in-k; source fact; not reproduced; high; S6). Fresh integers (`freshInt`) and fresh identifiers (`freshId`) are built into the prelude, drawing unique values backed by an internal `<generatedCounter>` cell (CLM-0205; SRC-0036 include/kframework/builtin/kast.md #rule-cells; source fact; not reproduced; high; S6). For user-defined sorts, a custom generation function must be marked with the `freshGenerator` attribute:

```k
syntax Foo ::= "a" | "b" | "c" | d ( Int )
syntax Foo ::= freshFoo ( Int ) [freshGenerator, function, total]

rule freshFoo(0) => a
rule freshFoo(1) => b
rule freshFoo(2) => c
rule freshFoo(I) => d(I) [owise]
```

When `!F:Foo` is evaluated, the backend generates a fresh integer index and passes it to `freshFoo`, ensuring globally unique user-sort values (CLM-0205; SRC-0036 docs/user_manual.md #freshgenerator-attribute; source fact; not reproduced; high; S6).

## Lesson 2.3: KLabels and Abstract Syntax

In K, concrete syntax productions are mapped internally to KLabels, which serve as tree constructors in abstract syntax terms (CLM-0206; SRC-0036 docs/user_manual.md #symbol_-attribute; source fact; not reproduced; high; S6). By default, the compiler assigns an auto-mangled, unique string to each production (CLM-0206; SRC-0036 docs/user_manual.md #klabel_-and-symbol-attributes; source fact; not reproduced; high; S6).

To establish an explicit, readable external identifier, the `symbol(_)` attribute can be attached directly to a syntax production:

```k
module SYMBOLS
  syntax Foo ::= foo() [symbol(foo)]
               | bar()
endmodule
```

The compiler enforces strict uniqueness of all explicit symbol names across the definition (CLM-0206; SRC-0036 docs/user_manual.md #symbol_-attribute; source fact; not reproduced; high; S6). The legacy syntax `[klabel(name), symbol]` is supported for backwards compatibility, but modern specifications should use `[symbol(name)]` directly (CLM-0206; SRC-0036 docs/user_manual.md #klabel_-and-symbol-attributes; source fact; not reproduced; high; S6).

In abstract syntax representations, KLabel applications are constructed using the `#klabel` and `#KApply` forms defined in `kast.md`:

```k
syntax KBott ::= "#token" "(" KString "," KString ")"  [symbol(#KToken)]
               | "#klabel" "(" KLabel ")"              [symbol(#WrappedKLabel)]
               | KLabel "(" KList ")"                  [symbol(#KApply)]
```

Explicit symbols ensure stable JSON serialization when interfacing with external tools such as `pyk` (CLM-0206; SRC-0036 docs/user_manual.md #klabel_-and-symbol-attributes; source fact; not reproduced; high; S6).

## Lesson 2.4: Overloaded Symbols

K supports subsort overloading on symbols, permitting a constructor or operator to accept more specific sorts and produce a more specific return sort (CLM-0207; SRC-0036 docs/user_manual.md #overload-attribute; source fact; not reproduced; high; S6).

```k
syntax Exp  ::= LVal
              | Exp  "." Id [overload(_._)]
syntax LVal ::= LVal "." Id [overload(_._)]
```

When parsing ambiguous terms such as `L() . x`, the parser chooses the most specific overloaded production, giving the resulting term sort `LVal` rather than `Exp` (CLM-0207; SRC-0036 docs/user_manual.md #overload-attribute; source fact; not reproduced; high; S6).

Formally, production $P$ is a more specific overload of $Q$ if:
1. $P$ and $Q$ share the identical `overload(_)` attribute key.
2. The sort tuple $(S_P, S_{p1}, \dots, S_{pN})$ is elementwise strictly smaller than $(S_Q, S_{q1}, \dots, S_{qN})$ according to the subsort ordering of the definition (CLM-0207; SRC-0036 docs/user_manual.md #overload-attribute; source fact; not reproduced; high; S6).

Overloading resolves grammatical ambiguities without requiring manual syntactic injections or intermediate sorts.

## Lesson 2.5: Matching Logic Connectives and `#Or` Patterns

Matching logic serves as the logical substrate of the K Framework (CLM-0208; SRC-0036 docs/user_manual.md #introduction; source fact; not reproduced; high; S6). `kast.md` exposes concrete surface syntax for matching logic connectives within module `ML-SYNTAX`:

```k
syntax {Sort} Sort ::= "#Top" [symbol(#Top)]
                     | "#Bottom" [symbol(#Bottom)]
                     | "#Not" "(" Sort ")" [symbol(#Not)]
syntax {Sort1, Sort2} Sort2 ::= "#Ceil" "(" Sort1 ")" [symbol(#Ceil)]
                              | "#Floor" "(" Sort1 ")" [symbol(#Floor)]
                              | "{" Sort1 "#Equals" Sort1 "}" [symbol(#Equals)]
syntax {Sort} Sort ::= Sort "#And" Sort [symbol(#And)]
                     > Sort "#Or" Sort [symbol(#Or)]
                     > Sort "#Implies" Sort [symbol(#Implies)]
syntax {Sort1, Sort2} Sort2 ::= "#Exists" Sort1 "." Sort2 [symbol(#Exists)]
                              | "#Forall" Sort1 "." Sort2 [symbol(#Forall)]
```

These connectives map directly to KORE constructs: `#Top` to `\top`, `#Bottom` to `\bottom`, `#And` to `\and`, `#Or` to `\or`, and `#Equals` to `\equals` (CLM-0208; SRC-0036 include/kframework/builtin/kast.md #syntax-of-matching-logic; source fact; not reproduced; high; S6).

In semantic rules, the `#Or` connective can be used on the left-hand side to match alternative syntactic patterns simultaneously:

```k
rule foo #Or bar #Or baz => qux
```

The compiler desugars `#Or` patterns into multiple guarded rules (pass `guardOrs` in `KoreBackend`) (CLM-0209; SRC-0036 pyk/docs/pipeline.md #stage-3-compilation-passes; source fact; not reproduced; high; S6). An `#Or` pattern is well-formed only if every alternative binds the identical set of variables (CLM-0209; SRC-0036 docs/user_manual.md #or-patterns; source fact; not reproduced; high; S6).

## Lesson 2.6: Function Context

Functions in K are ordinarily pure term simplifications independent of configuration state. When a function must inspect configuration state, the function context syntax `[[ ... ]]` allows declarative matching against cells:

```k
syntax Int ::= foo(Int) [function]

rule [[ foo(0) => I ]]
     <bar> I </bar>

rule something => foo(0)
```

The K frontend desugars this notation entirely during compilation by transforming the function signature to accept the configuration as an auxiliary parameter (CLM-0210; SRC-0036 docs/user_manual.md #matching-global-context-in-function-rules; source fact; not reproduced; high; S6):

```k
syntax Int ::= foo(Int, GeneratedTopCell) [function]

rule foo(0, <generatedTop>
              <bar> I </bar>
              ...
            </generatedTop> #as Configuration) => I

rule <generatedTop>
       <k> something ... </k>
       ...
     </generatedTop> #as Configuration
  => <generatedTop>
       <k> foo(0, Configuration) ... </k>
       ...
     </generatedTop>
```

Rewrites inside function context cells are strictly forbidden because configuration updates cannot be propagated back to the global state from within a pure function (CLM-0210; SRC-0036 docs/user_manual.md #matching-global-context-in-function-rules; source fact; not reproduced; high; S6).

## Lesson 2.7: Record Productions and Named Nonterminals

K allows nonterminals in BNF productions to be assigned explicit names using the syntax `name: Sort` (CLM-0211; SRC-0036 docs/user_manual.md #named-non-terminals; source fact; not reproduced; high; S6).

When applied to prefix productions, this feature enables record-like matching:

```k
syntax Exp ::= foo(name: Id, age: Int, active: Bool)

rule foo(... age: A, name: N) => bar(N, A)
```

In record patterns, unmentioned fields are treated as anonymous don't-care variables (CLM-0211; SRC-0036 docs/user_manual.md #record-like-kapply-patterns; source fact; not reproduced; high; S6). The compiler automatically generates projection functions corresponding to each named nonterminal:

```k
syntax Id ::= name(Exp) [function]
syntax Int ::= age(Exp) [function]
syntax Bool ::= active(Exp) [function]

rule name(foo(N, _, _)) => N
rule age(foo(_, A, _)) => A
rule active(foo(_, _, B)) => B
```

Record productions decouple rules from positional argument offsets, protecting semantics from arity changes during refactoring (CLM-0211; SRC-0036 docs/user_manual.md #record-like-kapply-patterns; source fact; not reproduced; high; S6).

## Lesson 2.8: `#fun` and `#let`

To bind intermediate expressions or perform localized rewriting without declaring auxiliary top-level functions, K provides anonymous function expressions:

```k
syntax {Sort} Sort ::= "#fun" "(" Sort ")" "(" Sort ")" [symbol(#fun2), prefer]
syntax {Sort1, Sort2} Sort1 ::= "#fun" "(" Sort2 "=>" Sort1 ")" "(" Sort2 ")" [symbol(#fun3)]
syntax {Sort1, Sort2} Sort1 ::= "#let" Sort2 "=" Sort2 "#in" Sort1 [symbol(#let)]
```

The `#fun(Pattern => Body)(Arg)` form evaluates `Arg`, matches it against `Pattern`, and evaluates `Body` under the resulting substitution (CLM-0212; SRC-0036 docs/user_manual.md #anonymous-function-applications; source fact; not reproduced; high; S6). Local rewrites can also be embedded directly within structured patterns:

```k
rule foo(K, RECORD) =>
  #fun(record(... field: _ => K))(RECORD)
```

The `#let Var = Exp #in Body` construct provides a syntactic alias for `#fun(Var => Body)(Exp)`, establishing standard lexical scoping for intermediate values (CLM-0212; SRC-0036 include/kframework/builtin/kast.md lines 463-468; source fact; not reproduced; high; S6).

## Lesson 2.9: `#as` Patterns

The `#as` pattern matches a structured term against an inner pattern while simultaneously binding the entire composite term to a variable:

```k
Pattern #as V::Var
```

Matching succeeds only if `Pattern` matches the target term; if successful, both the internal variables of `Pattern` and the composite variable `V` are added to the rule substitution (CLM-0213; SRC-0036 docs/user_manual.md #as-patterns; source fact; not reproduced; high; S6). `#as` patterns are valid only on the left-hand side of a rewrite or within matching contexts; placing `#as` on the right-hand side is a static error (CLM-0213; SRC-0036 docs/user_manual.md #as-patterns; source fact; not reproduced; high; S6).

## Lesson 2.10: The Matching Operators `:=K` and `:/=K`

The `:=K` and `:/=K` operators evaluate pattern matching as a boolean predicate:

```k
syntax Bool ::= left:
                K ":=K" K   [function, total, symbol(_:=K_)]
              | K ":/=K" K  [function, total, symbol(_:/=K_)]
```

The expression `Pattern :=K Term` tests whether `Term` is an instance of `Pattern` (model membership), contrasting with `==K` which tests syntactic equality (CLM-0214; SRC-0036 docs/user_manual.md #pattern-matching-operator; source fact; not reproduced; high; S6). Variables in `Pattern` are not bound in the enclosing rule body; authors should use anonymous variables `_` on the left-hand side of `:=K` (CLM-0214; SRC-0036 docs/user_manual.md #pattern-matching-operator; source fact; not reproduced; high; S6). The frontend compiles `:=K` into an efficient fresh matching function (CLM-0214; SRC-0036 docs/user_manual.md #pattern-matching-operator; source fact; not reproduced; high; S6).

## Lesson 2.11: Uncommon Evaluation Order Concepts

K features a declarative evaluation system based on the heating and cooling of subterms within the `<k>` computation cell (CLM-0215; SRC-0036 docs/user_manual.md #strict-and-seqstrict-attributes; source fact; not reproduced; high; S6).

### Strictness and KResult

Marking a production `strict` instructs the compiler to generate heating and cooling rules for all its argument positions:

```k
syntax AExp ::= Int
              | AExp "+" AExp [strict, symbol(addExp)]
syntax KResult ::= Int
```

The compiler expands this into two heating rules and two cooling rules:

```k
rule [addExp1-heat]: <k> HOLE:AExp +  AE2:AExp => HOLE ~>  [] + AE2 ... </k> [heat]
rule [addExp2-heat]: <k>  AE1:AExp + HOLE:AExp => HOLE ~> AE1 +  [] ... </k> [heat]
rule [addExp1-cool]: <k> HOLE:AExp ~>  [] + AE2 => HOLE +  AE2 ... </k> [cool]
rule [addExp2-cool]: <k> HOLE:AExp ~> AE1 +  [] =>  AE1 + HOLE ... </k> [cool]
```

The `KResult` sort designates fully evaluated values (CLM-0215; SRC-0036 docs/user_manual.md #strict-and-seqstrict-attributes; source fact; not reproduced; high; S6). The compiler automatically attaches side conditions `notBool isKResult(HOLE)` to heating rules and `isKResult(HOLE)` to cooling rules, preventing infinite rewrite loops (CLM-0215; SRC-0036 docs/user_manual.md #strict-and-seqstrict-attributes; source fact; not reproduced; high; S6).

The `seqstrict` attribute enforces a sequential evaluation order across specified argument indices:

```k
syntax BExp ::= Bool
              | BExp "&&" BExp [seqstrict(1, 2)]
```

In `seqstrict(1, 2)`, argument 2 cannot heat until `isKResult` holds for argument 1 (CLM-0215; SRC-0036 docs/user_manual.md #strict-and-seqstrict-attributes; source fact; not reproduced; high; S6).

### Context Declarations and Context Rewrites

When evaluation depends on configuration state beyond the enclosing term, explicit `context` sentences define heating conditions:

```k
context <k> foo(HOLE:Bar) ... </k> <state> .Map </state> requires baz(HOLE)
```

Context declarations may include local rewrites on `HOLE`:

```k
context foo(HOLE:Bar => bar(HOLE))
```

This generates heating and cooling rules that wrap and unwrap the hole with `bar`:

```k
rule <k> foo(HOLE:Bar) => bar(HOLE) ~> foo([]) ... </k> [heat]
rule <k> bar(HOLE:Bar) ~> foo([]) => foo(HOLE) ... </k> [cool]
```

This pattern allows terms to evaluate under specialized auxiliary semantics (CLM-0216; SRC-0036 docs/user_manual.md #rewrites-in-context-declarations; source fact; not reproduced; high; S6).

### Context Aliases, Hybrid Productions, and Result Overrides

A context alias defines a reusable template for evaluation contexts:

```k
context alias [c]: <k> HERE:K ... </k> <evaluate> true </evaluate> [result(ExecResult)]

syntax Expr ::= Expr "=" Init [strict(c; 1)]
```

The production `strict(c; 1)` instantiates the template `c` for its first argument, evaluating the term to sort `ExecResult` rather than default `KResult` (CLM-0217; SRC-0036 docs/user_manual.md #context-aliases; source fact; not reproduced; high; S6).

The `hybrid` attribute specifies that a strict production can itself be considered a result once its arguments are evaluated:

```k
syntax BExp ::= BExp "||" BExp [strict(1), hybrid]
```

This generates `rule isKResult(BE1:BExp || BE2:BExp) => true requires isKResult(BE1)`, enabling lazy or partially evaluated structures to inhabit the result sort (CLM-0217; SRC-0036 docs/user_manual.md #hybrid-attribute; source fact; not reproduced; high; S6).

## Lesson 2.12: IEEE 754 Floating Point and Machine Integers (`MInt`)

K standardizes mathematical domains in `domains.md`.

### IEEE 754 Floating Point

The `FLOAT` module implements arbitrary-precision floating point numbers conforming to IEEE 754, backed by the GNU MPFR library on the LLVM backend (CLM-0218; SRC-0036 include/kframework/builtin/domains.md #ieee-754-floating-point-numbers; source fact; not reproduced; high; S6). Literals support bitwidth specification suffixes: `f` (binary32), `d` (binary64), or `pNxM` specifying $N$ precision bits and $M$ exponent bits (e.g. `p24x8` for single precision, `p53x11` for double precision) (CLM-0218; SRC-0036 include/kframework/builtin/domains.md #ieee-754-floating-point-numbers; source fact; not reproduced; high; S6). Floating comparisons satisfy IEEE 754 semantics: `NaN ==Float NaN` evaluates to `false`, whereas syntactic term equality `NaN ==K NaN` evaluates to `true` (CLM-0218; SRC-0036 include/kframework/builtin/domains.md #floating-point-comparisons; source fact; not reproduced; high; S6).

### Machine Integers (`MInt`)

The `MInt` module implements width-parametric, fixed-precision binary integers in two's complement representation (CLM-0219; SRC-0036 include/kframework/builtin/domains.md #machine-integers; source fact; not reproduced; high; S6):

```k
syntax {Width} MInt{Width} [hook(MINT.MInt)]
syntax {Width} MInt{Width} ::= r"[\\+\\-]?[0-9]+[pP][0-9]+" [token, hook(MINT.literal)]
```

The suffix `pN` fixes the bitwidth; for example, `0p8` has sort `MInt{8}`, and `100p256` has sort `MInt{256}` (CLM-0219; SRC-0036 include/kframework/builtin/domains.md #machine-integers; source fact; not reproduced; high; S6). `MInt` values do not possess intrinsic signs. Instead, operations explicitly interpret bits as signed or unsigned:
- Division: `/sMInt` (signed) versus `/uMInt` (unsigned)
- Modulo: `%sMInt` (signed) versus `%uMInt` (unsigned)
- Shift: `>>aMInt` (arithmetic) versus `>>lMInt` (logical)
- Comparison: `<sMInt`, `<=sMInt` versus `<uMInt`, `<=uMInt`
- Integer conversion: `MInt2Signed` versus `MInt2Unsigned` (CLM-0219; SRC-0036 include/kframework/builtin/domains.md #mint-arithmetic; source fact; not reproduced; high; S6).

## Lesson 2.13: Alpha-Renaming-Aware Substitution

K provides a capture-avoiding substitution engine in `substitution.md` for functional language semantics (CLM-0220; SRC-0036 include/kframework/builtin/substitution.md #capture-aware-substitution-in-k; source fact; not reproduced; high; S6).

The engine relies on two constructs: the `KVar` identifier sort and the `binder` production attribute:

```k
module KVAR-SYNTAX
  syntax KVar [token, hook(KVAR.KVar)]
endmodule

syntax Val ::= "lambda" KVar "." Exp [binder]
```

In any production marked `binder`, the first nonterminal must be of sort `KVar` (the bound variable), and the final nonterminal must be the body term in which the variable is bound (CLM-0220; SRC-0036 include/kframework/builtin/substitution.md #the-binder-attribute; source fact; not reproduced; high; S6). Capture-aware substitution is invoked via bracket operators:

```k
syntax {Sort} Sort ::= Sort "[" KItem "/" KItem "]" [function, hook(SUBSTITUTION.substOne), impure]
syntax {Sort} Sort ::= Sort "[" Map "]"             [function, hook(SUBSTITUTION.substMany), impure]
```

On the LLVM backend, substitution is implemented using de Bruijn indices for bound variables and unique numeric identifiers for free variables, preventing variable capture during beta-reduction (CLM-0220; SRC-0036 include/kframework/builtin/substitution.md #substitution; source fact; not reproduced; high; S6).

## Lesson 2.14: File I/O

The `K-IO` module provides POSIX-aligned file and stream input/output for concrete execution on the LLVM backend (CLM-0221; SRC-0036 include/kframework/builtin/domains.md #io-in-k; source fact; not reproduced; high; S6). Because I/O produces irreducible side effects, it is unsupported during symbolic execution on the Haskell backend (CLM-0221; SRC-0036 include/kframework/builtin/domains.md #io-in-k; source fact; not reproduced; high; S6).

Core operations return either result values or an `IOError` term reflecting system `errno` values:
- `#open(Path, Mode)`: opens a file and returns an integer file descriptor (`IOInt`)
- `#read(Fd, Length)`: reads up to `Length` characters (`IOString`)
- `#write(Fd, String)`: writes text to file descriptor, returning `.K`
- `#tell(Fd)` and `#seek(Fd, Offset)`: inspects and positions the file pointer
- `#close(Fd)`: releases the file descriptor (CLM-0221; SRC-0036 include/kframework/builtin/domains.md lines 2517-2574; source fact; not reproduced; high; S6).

High-level stream integration is achieved via configuration cell attributes `stream="stdin"` and `stream="stdout"`, which automatically instantiate the `STDIN-STREAM` and `STDOUT-STREAM` buffering rules (CLM-0221; SRC-0036 include/kframework/builtin/domains.md lines 2709-2867; source fact; not reproduced; high; S6).

## Lesson 2.15: String Buffers and Byte Sequences

### String Buffers

Repeated string concatenation via `+String` exhibits quadratic $O(N^2)$ time complexity. The `STRING-BUFFER` module introduces `StringBuffer`, backed by a mutable byte buffer on the LLVM backend that provides amortized $O(1)$ append operations:

```k
syntax StringBuffer ::= ".StringBuffer" [function, total, hook(BUFFER.empty)]
                      | StringBuffer "+String" String [function, total, hook(BUFFER.concat)]
syntax String ::= StringBuffer2String ( StringBuffer ) [function, total, hook(BUFFER.toString)]
```

`StringBuffer2String` copies the buffer contents into an immutable `String` upon completion (CLM-0222; SRC-0036 include/kframework/builtin/domains.md #string-buffers; source fact; not reproduced; high; S6).

### Byte Sequences (`Bytes`)

The `BYTES` module provides arbitrary-length byte arrays with syntax `b"..."` (CLM-0223; SRC-0036 include/kframework/builtin/domains.md #byte-arrays; source fact; not reproduced; high; S6). Conversions to and from integers require explicit endianness (`LE` or `BE`) and signedness (`Signed` or `Unsigned`):

```k
syntax Int ::= Bytes2Int(Bytes, Endianness, Signedness) [function, total, hook(BYTES.bytes2int)]
syntax Bytes ::= Int2Bytes(length: Int, Int, Endianness) [function, total, hook(BYTES.int2bytes)]
               | Int2Bytes(Int, Endianness, Signedness) [function, total, symbol(Int2BytesNoLen)]
```

The variant with an explicit length pads or truncates the integer magnitude, whereas the variant without length produces the minimal byte sequence capable of round-tripping through `Bytes2Int` (CLM-0223; SRC-0036 include/kframework/builtin/domains.md #integer-and-bytes-conversion; source fact; not reproduced; high; S6). Substring slicing (`substrBytes`), in-place overwriting (`replaceAtBytes`), and bulk initialization (`memsetBytes`) operate with linear complexity (CLM-0223; SRC-0036 include/kframework/builtin/domains.md lines 2122-2170; source fact; not reproduced; high; S6). Passing `--llvm-mutable-bytes` to `kompile` enables in-place mutation of byte arrays in the LLVM backend runtime (CLM-0223; SRC-0036 include/kframework/builtin/domains.md #byte-arrays; source fact; not reproduced; high; S6).

## Lesson 2.16: The Intermediate Language of K, KORE

KORE is the formal intermediate language of the K Framework (CLM-0224; SRC-0036 docs/user_manual.md #k-process-overview; source fact; not reproduced; high; S6). K definitions are compiled through 32 frontend passes into `definition.kore`, which contains:
1. Sort declarations and symbol declarations
2. Structural axioms (associativity, commutativity, idempotence, unit, injectivity)
3. Subsort injection symbols `inj{FromSort, ToSort}`
4. Rewrite rule axioms and function simplification equations (CLM-0224; SRC-0036 pyk/docs/pipeline.md #stage-4-kore-emission; source fact; not reproduced; high; S6).

Both the LLVM backend interpreter and the Haskell symbolic backend consume `definition.kore` directly (CLM-0224; SRC-0036 pyk/docs/pipeline.md #stage-5-backend-compilation; source fact; not reproduced; high; S6). Ahead-of-time bison parsers generated via `--gen-bison-parser` produce KORE AST terms directly, bypassing the JVM parser for maximal execution speed (CLM-0224; SRC-0036 docs/user_manual.md #parser-generation; source fact; not reproduced; high; S6).

## Lesson 2.17: Debugging Proofs Using the Haskell Backend REPL

When formal verification claims fail to discharge in `kprove`, the failure can be interactively debugged using `kore-repl` (CLM-0224; SRC-0036 docs/ktools.md #minimizing-output; source fact; not reproduced; high; S6).

Large proof configurations can be simplified using `pyk print` with filtering options:
- `--minimize`: strips uninteresting, unchanged configuration cells
- `--omit-cells`: removes specific noisy cells from view
- `--keep-cells`: restricts output exclusively to critical cells (CLM-0224; SRC-0036 docs/ktools.md #minimizing-output; source fact; not reproduced; high; S6).

The REPL permits step-by-step execution through the proof search graph, inspection of branch conditions, evaluation of SMT queries, and identification of missing domain lemmas or non-converging simplifications (CLM-0224; SRC-0036 docs/ktools.md lines 10-58; source fact; not reproduced; high; S6).

## Implications for ZKIR Circuit and VM Modelling (Inference)

The intermediate features detailed in this section provide the essential formal machinery for defining ZKIR semantics:
1. *Straight-line instruction sequences*: The `~>` sequence operator in `KSEQ` coupled with `KResult` predicates models the strictly ordered execution of straight-line ZKIR gates (inference based on CLM-0215, CLM-0224; SRC-0036 docs/user_manual.md).
2. *Field and wire representations*: Prime-field elements map directly to arbitrary-precision `Int` operations modulo $p$, while bit-constrained wires map to `MInt{N}` or `bitRangeInt` extractions (inference based on CLM-0219, CLM-0223; SRC-0036 include/kframework/builtin/domains.md).
3. *Circuit environment lookup*: The function context syntax `[[ eval(W) => V ]] <env> ... W |-> V ... </env>` provides clean, declarative wire lookup without polluting rule left-hand sides with full configuration paths (inference based on CLM-0210; SRC-0036 docs/user_manual.md).
4. *Macro desugaring*: High-level ZKIR pseudo-instructions (e.g. multi-wire equality checks or complex constraints) can be desugared statically into primitive gates via `macro` and `macro-rec`, avoiding runtime evaluation overhead (inference based on CLM-0201, CLM-0202; SRC-0036 k-tutorial/2_intermediate/01_macros/README.md).

For detailed reference specifications of production and cell attributes, see [k-user-manual](k-user-manual.md). For builtin domain signatures and hook implementations, see [k-builtins](k-builtins.md). For backend driver workflows and compilation pipelines, see [k-backends-and-tools](k-backends-and-tools.md). For ZKIR-specific instruction definitions and formal semantics, see [zkir-instruction-set](../zkir/zkir-instruction-set.md) and [zkir-vm-semantics](../zkir/zkir-vm-semantics.md).
