---
id: k.framework.tutorial-basic
type: language
title: K tutorial section 1, basic concepts
status: active
updated_at: 2026-09-03T14:35:53Z
sources:
  - SRC-0036
created: 2026-09-03
updated: 2026-09-07
tags:
  - moriarty
  - research
---

# K tutorial section 1, basic concepts

## Overview of the basic tutorial series

Section 1 of the official K Framework tutorial introduces the core syntactic, operational, configuration, and verification concepts of the K language (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/README.md; source fact; not reproduced; high; S6).
The repository tutorial sets the pedagogical goal as follows:
> "By the end of this section, you'll be able to define a simple language in K and use its specifications to generate a fast interpreter for it. You'll also know how to write basic deductive program verification proofs over programs writen in your language." (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/README.md; source fact; not reproduced; high; S6).

The tutorial consists of twenty-two structured lessons progressing from local toolchain setup to reachability logic proofs (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/README.md; source fact; not reproduced; high; S6).
This document provides an exhaustive technical analysis of each lesson in sequence, detailing syntax declarations, rewriting mechanics, cell configurations, evaluation contexts, and proof techniques (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/README.md; source fact; not reproduced; high; S6).
For intermediate semantic constructs, refer to [Intermediate K Concepts](k-tutorial-intermediate.md).
For tool command references, see [K User Manual](k-user-manual.md) and [K Backends and Tooling](k-backends-and-tools.md).
For standard library domains, see [K Built-in Domains](k-builtins.md).
For ZKIR architecture mapping, consult [ZKIR Instruction Set Architecture](../zkir/zkir-instruction-set.md).

## Lesson 1.1: Setting up a K Environment

The first step in using the K Framework is configuring a working environment and installing the toolchain executables (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
Developers select between installing pre-compiled binary packages or building the complete framework from source (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
K is developed and released on a continuous rolling-release model, where every commit that passes continuous integration on GitHub is published as a deployable release (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
Because K binary packages outside macOS Homebrew do not maintain state inside operating system package manager databases, users must completely uninstall previous versions prior to installing a new build (CLM-0156; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
Building from source requires cloning the repository with the `--recursive` flag to ensure all native submodule dependencies for the LLVM and Haskell backends are downloaded (CLM-0157; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
Editor integration modes for text editors such as Vim and Emacs are hosted in the auxiliary repository `https://github.com/kframework/k-editor-support` (CLM-0157; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
Because K allows developers to define arbitrary custom context-free grammars for their modeled languages, static editor highlighters can only approximate syntax highlighting (CLM-0157; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
Bug reporting on GitHub organizes tool issues around the primary toolchain commands: `kompile`, `kast`, `krun`, `kprove`, and `ksearch` (CLM-0157; SRC-0036 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).

## Lesson 1.2: Basics of Functional K

Functional K formalizes computation as mathematical function evaluation over algebraic data types without mutable side effects (CLM-0158; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
A basic K definition is stored in a file with the `.k` extension and is structured into one or more modules declared with `module <NAME>` and `endmodule` (CLM-0158; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Module names must be written in capital letters and conventionally match the filename (CLM-0158; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Data sorts are introduced using the `syntax` keyword and must always begin with an uppercase letter, resembling data types in functional languages (CLM-0158; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Sort productions can define constructors or functions, where function productions carry the explicit `[function]` attribute or the `[total]` attribute for total mathematical functions (CLM-0158; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
The compiler command `kompile lesson-02-a.k` processes the definition and creates a compiled execution directory named `lesson-02-a-kompiled/` (CLM-0158; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Function behaviors are defined via rewrite rules beginning with the `rule` keyword, connecting a left-hand pattern to a right-hand pattern with the rewrite arrow `=>` (CLM-0159; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Variables in rules are written as uppercase identifiers that match any term of the appropriate sort (CLM-0159; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Functions in K can be partial; invoking a function on a constructor term that lacks a matching rule halts concrete execution with an uninterpreted term error (CLM-0159; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Programs are executed using the `krun` tool, either by passing a file containing the program term or by passing an inline expression via `krun -cPGM='<term>'` (CLM-0159; SRC-0036 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-02-A
  syntax Color ::= Yellow() | Blue()
  syntax Fruit ::= Banana() | Blueberry()
  syntax Color ::= colorOf(Fruit) [function]

  rule colorOf(Banana()) => Yellow()
  rule colorOf(Blueberry()) => Blue()
endmodule
```

## Lesson 1.3: BNF Syntax and Parser Generation

K allows developers to specify custom concrete syntax for both rules and target programs using Backus-Naur Form (BNF) productions (CLM-0160; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
The parser distinguishes between terminals, which are fixed character sequences enclosed in double quotation marks, and non-terminals, which reference declared sort names (CLM-0160; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
K automatically generates lexical scanners for language definitions using Flex, applying longest-match rules to tokenize source inputs (CLM-0160; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
The default just-in-time parser `kast` implements Generalized Left-to-right (GLL) parsing, which is capable of parsing arbitrary context-free grammars and reporting parse ambiguities (CLM-0160; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
The `--output` flag of `kast` controls whether parsed abstract syntax trees are emitted as KORE intermediate representation, surface KAST, or structured JSON (CLM-0160; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Grammar ambiguities occur when an input string can be parsed into multiple distinct valid syntax trees (CLM-0161; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Bracket productions carrying the `[bracket]` attribute must contain exactly one non-terminal of the same sort as the production and are erased from the parsed AST (CLM-0161; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Token productions carrying the `[token]` attribute match regular expressions preceded by the `r` prefix, converting matching sequences directly into typed token AST nodes (CLM-0161; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Explicit regular expressions can be defined modularly using `syntax lexical <Name> = r"..."` and referenced inside token definitions using curly braces (CLM-0161; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
To accelerate parsing of large programs, `kompile --gen-bison-parser` generates an ahead-of-time LR(1) parser executable named `parser_PGM`, while `--gen-glr-bison-parser` produces a GLR parser that records ambiguities as `Lblamb` nodes (CLM-0161; SRC-0036 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-03-D
  syntax Boolean ::= "true" | "false"
                   | "(" Boolean ")" [bracket]
                   | "!" Boolean [function]
                   | Boolean "&&" Boolean [function]
                   | Boolean "^" Boolean [function]
                   | Boolean "||" Boolean [function]
endmodule
```

## Lesson 1.4: Disambiguating Parses

Disambiguation techniques transform an ambiguous context-free grammar into an efficient, deterministic language specification (CLM-0162; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
Parsing unambiguous grammars is asymptotically faster than parsing ambiguous grammars, making grammar disambiguation essential for performance (CLM-0162; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
Operator precedence is defined using priority blocks separated by the `>` operator, establishing a transitive partial order where tighter-binding operators take precedence over looser-binding operators (CLM-0162; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
A symbol with lesser priority cannot appear as an immediate boundary child of a symbol with greater priority (CLM-0162; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
This priority restriction applies conditionally only when the child non-terminal is the first or last production item of the parent production (CLM-0162; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
Associativity rules resolve ambiguities between operators of equal priority by prefixing priority groups with `left:`, `right:`, or `non-assoc:` (CLM-0162; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
Under left-associativity, an operator cannot appear as the direct rightmost child of an operator with equal priority; under right-associativity, it cannot appear as the direct leftmost child (CLM-0163; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
Precedence and associativity can be declared across disparate modules using explicit `syntax priority`, `syntax left`, `syntax right`, and `syntax non-assoc` sentences over named groups tagged with `group(...)` (CLM-0163; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
Ambiguities that cannot be resolved by boundary priority rules, such as the dangling-else problem, are resolved using the `[prefer]` and `[avoid]` attributes (CLM-0163; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
When an ambiguity occurs between candidate parse trees, the parser discards any branch whose root contains the `[avoid]` attribute, or discards all branches that lack the `[prefer]` attribute, while leaving unambiguous parses unaffected (CLM-0163; SRC-0036 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-04-B
  syntax Boolean ::= "true" | "false"
                   | "(" Boolean ")" [bracket]
                   > "!" Boolean [function]
                   > left: Boolean "&&" Boolean [function]
                   > left: Boolean "^" Boolean [function]
                   > left: Boolean "||" Boolean [function]
endmodule
```

## Lesson 1.5: Modules, Imports, and Requires

Language specifications in K scale across complex architectures through modular decomposition and file inclusion mechanisms (CLM-0164; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
The basic unit of sentence grouping is the module, which consists of a module name, optional attributes, import statements, and language sentences (CLM-0164; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Modules incorporate other modules using the `imports <MODULE>` declaration, bringing all sentences from the imported module into the importing module's scope (CLM-0164; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
When parsing rewrite rules in a module, K uses the specific syntax of that module enriched with built-in K syntax, preventing unimported symbols from being used in rule bodies (CLM-0164; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
A K definition designates two distinct entry point modules: the main semantics module and the main syntax module (CLM-0164; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
The main semantics module and its transitive imports define the operational rewrite rules, while the main syntax module defines the grammar used to parse input programs executed by `krun` (CLM-0164; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Multiple physical files are combined into a single definition via `requires "<path>"` statements placed at the very top of a file before module declarations (CLM-0165; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
The compiler recursively processes all required files and collects all declared modules into the available import namespace (CLM-0165; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Relative paths in `requires` statements are resolved against include directories passed with `-I <dir>`, the current working directory, and the installation library directory `include/kframework/builtin/` (CLM-0165; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Separating program syntax from internal semantic functions into distinct modules prevents internal rewrite operators from leaking into the target language grammar (CLM-0165; SRC-0036 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).

```k
requires "fruits.k"
requires "colors.k"

module COLOROF-SYNTAX
  imports COLORS
  imports FRUITS
  syntax Color ::= colorOf(Fruit) [function]
endmodule

module COLOROF
  imports COLOROF-SYNTAX
  rule colorOf(Banana()) => Yellow()
  rule colorOf(Blueberry()) => Blue()
endmodule
```

## Lesson 1.6: Integers and Booleans

The K standard library defines fundamental built-in mathematical sorts in `include/kframework/builtin/domains.md` (CLM-0166; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
The `Bool` sort represents truth values `true` and `false`, provided by importing the `BOOL` module (CLM-0166; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
Functions returning a `Bool` are called predicates (CLM-0166; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
Standard boolean operations in K include `andBool`, `orBool`, `xorBool`, `impliesBool`, and `notBool` (CLM-0166; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
Built-in functions carry sort-name suffixes to prevent syntactic collision with target programming languages being modeled (CLM-0166; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
The standard library provides `BOOL-SYNTAX` containing only literal values `true` and `false` without operators, allowing developers to import boolean literals into target language syntax without operator pollution (CLM-0166; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
The `Int` sort in K represents mathematical, arbitrary-precision integers backed by GMP rather than fixed-width machine words (CLM-0167; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
The `INT` module provides arithmetic operations including `+Int`, `-Int`, `*Int`, `/Int`, `divInt`, and `%Int`, alongside relational comparisons `<Int`, `<=Int`, `>Int`, `>=Int`, `==Int`, and `=/=Int` (CLM-0167; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
K provides three modules for integers: `INT` with full arithmetic syntax, `INT-SYNTAX` with signed integer literals, and `UNSIGNED-INT-SYNTAX` with non-negative decimal literals (CLM-0167; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
Importing `UNSIGNED-INT-SYNTAX` prevents lexical ambiguities in target languages where unary minus is an operator rather than part of an integer literal token (CLM-0167; SRC-0036 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; source fact; not reproduced; high; S6).
Low-level operations map directly to native backend hooks via the `[hook]` attribute in the standard library (CLM-0167; SRC-0036 docs/ktools.md; repository observation; not reproduced; high; S6).

```k
module LESSON-06-C-SYNTAX
  imports BOOL-SYNTAX
  syntax Bool ::= "(" Bool ")" [bracket]
                > "!" Bool [function]
                > left:
                  Bool "&&" Bool [function]
                | Bool "^" Bool [function]
                | Bool "||" Bool [function]
endmodule

module LESSON-06-C
  imports LESSON-06-C-SYNTAX
  imports BOOL
  rule ! B => notBool B
  rule A && B => A andBool B
  rule A ^ B => A xorBool B
  rule A || B => A orBool B
endmodule
```

## Lesson 1.7: Side Conditions and Rule Priority

Conditional rewriting in K extends standard rules with Boolean guards and execution order controls (CLM-0168; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
A conditional rule appends a side condition introduced by the `requires` keyword following the rule body (CLM-0168; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
The side condition is a Boolean expression that must evaluate to `true` for the rewrite step to apply (CLM-0168; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Variables matched on the left-hand side of the rule are bound and substituted into the side condition prior to its evaluation (CLM-0168; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Side conditions are only evaluated if the left-hand pattern matches the candidate term successfully (CLM-0168; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Rules that act as fallbacks when no other rules match are marked with the `[owise]` attribute, meaning "otherwise" (CLM-0168; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
The `[owise]` attribute is a special case of K's numerical rule priority mechanism, where rules are attempted in increasing order of priority value (CLM-0169; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Standard rules carry a default priority of 50, whereas `[owise]` rules carry an implicit priority of 200 (CLM-0169; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Explicit priorities can be assigned to rules using `[priority(N)]`, where `N` is an arbitrary non-negative integer (CLM-0169; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Rules can incorporate anonymous wildcard variables using an underscore `_`, matching any subterm without binding a variable name on the right-hand side or in side conditions (CLM-0169; SRC-0036 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Variables that are bound but not referenced can be marked with the `[unused]` attribute to suppress compiler warnings (CLM-0169; SRC-0036 docs/cheat_sheet.md; repository observation; not reproduced; high; S6).

```k
module LESSON-07-D
  imports BOOL
  imports INT

  syntax Grade ::= "letter-A" | "letter-B" | "letter-C" | "letter-D" | "letter-F"
                 | gradeFromPercentile(Int) [function]

  rule gradeFromPercentile(I) => letter-A requires I >=Int 90 [priority(50)]
  rule gradeFromPercentile(I) => letter-B requires I >=Int 80 [priority(51)]
  rule gradeFromPercentile(I) => letter-C requires I >=Int 70 [priority(52)]
  rule gradeFromPercentile(I) => letter-D requires I >=Int 60 [priority(53)]
  rule gradeFromPercentile(_) => letter-F                     [priority(54)]
endmodule
```

## Lesson 1.8: Literate Programming with Markdown

K natively supports the literate programming paradigm, allowing formal language definitions and human-readable documentation to reside in identical CommonMark Markdown files (CLM-0170; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
When a file with the `.md` extension is passed to `kompile`, the compiler scans the document for fenced code blocks, extracts their contents, concatenates them, and compiles them as a K specification (CLM-0170; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
Standard K code blocks open with triple backticks followed by the `k` tag: ` ```k ` (CLM-0170; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
All prose outside of fenced K code blocks is treated as non-executable commentary (CLM-0170; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
Multiple K code blocks distributed across a Markdown document are assembled sequentially into the enclosing module (CLM-0170; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
Code blocks can carry custom tags or multiple selectors enclosed in curly braces, such as ` ```{.k .selector} ` (CLM-0171; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
The `--md-selector` compiler option controls which code blocks are selected during compilation by evaluating a Boolean expression over tags (CLM-0171; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
Selector expressions support conjunction `&`, disjunction `|`, negation `!`, and grouping parentheses `()` (CLM-0171; SRC-0036 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
By passing `--md-selector "k & (! exclude)"`, developers can maintain alternative language dialects, testing fixtures, or excluded sketches within a single document (CLM-0171; SRC-0036 k-distribution/k-tutorial/1_basic/commands.sh; repository observation; not reproduced; high; S6).
Syntactic macro rules carrying the `[macro]` attribute perform early AST desugaring during the frontend compilation pass (CLM-0171; SRC-0036 k-distribution/k-tutorial/2_intermediate/01_macros/README.md; repository observation; not reproduced; high; S6).

```k
module LESSON-08
  imports INT

  syntax Int ::= Int "+" Int [function]
  rule I1 + I2 => I1 +Int I2
endmodule
```

## Lesson 1.9: Unparsing and the Format and Color Attributes

Program execution in `krun` proceeds through three distinct phases: parsing the program text into an AST, executing rewrite steps on configurations, and unparsing the final AST back into human-readable text (CLM-0172; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Constructing an exact, sound reversibility algorithm between arbitrary grammars and ASTs is NP-hard, leading K to employ a linear-time approximation algorithm that prioritizes readable pretty-printing (CLM-0172; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
The unparser automatically reconstructs whitespace and injects parentheses where necessary to preserve precedence without cluttering terms whose associativity is unambiguous (CLM-0172; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Developers customize the unparsing layout of specific productions using the `format` attribute (CLM-0172; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
The `format` attribute accepts format codes prefixed with `%`, where non-code characters are printed verbatim (CLM-0172; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Format code `%n` inserts a newline followed by the current indentation level (CLM-0173; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Format code `%i` increases the current indentation level by one, while `%d` decreases indentation by one (CLM-0173; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
A 1-based integer index, such as `%1` or `%2`, references the corresponding terminal or non-terminal item in the production (CLM-0173; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Terminal text coloring is controlled via the `color` and `colors` attributes, combined with format codes `%c` to advance to the next color and `%r` to reset to the default terminal foreground (CLM-0173; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Unparsed configurations can be previewed without program execution using the pipeline `kparse <file> | kore-print -` (CLM-0173; SRC-0036 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-09-C
  imports BOOL

  syntax Stmt ::= "{" Stmt "}" [format(%1%i%n%2%d%n%3)] | "{" "}" [format(%1%2)]
                > right:
                  Stmt Stmt [format(%1%n%2)]
                | "if" "(" Bool ")" Stmt [format(%1 %2%3%4 %5)]
                | "if" "(" Bool ")" Stmt "else" Stmt [avoid, format(%1 %2%3%4 %5 %6 %7)]
endmodule
```

## Lesson 1.10: Strings

The `String` sort represents sequences of text characters in K (CLM-0174; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
Literal string syntax is provided by the `STRING-SYNTAX` module, while built-in string manipulation functions are imported via `STRING` from `domains.md` (CLM-0174; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
String literals are enclosed in double quotes and support standard escape sequences, including `\"`, `\\`, `\n`, `\r`, `\t`, and `\f` (CLM-0174; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
Hexadecimal and unicode code points are encoded using `\x00` for 8-bit bytes, `\u0000` for 16-bit code points, and `\U00000000` for 32-bit code points up to `0x10FFFF` (CLM-0174; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
The tutorial notes that unicode support is incomplete and code points above `0xFF` may trigger errors in certain backend operations (CLM-0174; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
String concatenation is performed with `+String`, an operation with $O(N)$ asymptotic time complexity (CLM-0175; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
String length is computed in code points using `lengthString` (CLM-0175; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
Substrings are extracted using `substrString(S, start, end)`, which is defined over the half-open index range $[start..end)$ with bounds checks $0 \le start \le end \le \text{length}$ (CLM-0175; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; source fact; not reproduced; high; S6).
Low-level byte sequences and binary buffers are modeled using the companion `Bytes` domain in `domains.md` (CLM-0175; SRC-0036 k-distribution/k-tutorial/1_basic/10_strings/README.md; repository observation; not reproduced; high; S6).

```k
module LESSON-10
  imports STRING

  syntax String ::= msg(String) [function]
  rule msg(S) => "The string you provided: " +String S +String "\nHave a nice day!"
endmodule
```

## Lesson 1.11: Casting Terms

Casting expressions in K resolve grammatical ambiguities and enforce static and runtime sort constraints (CLM-0176; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
K provides three distinct cast operators: semantic casts, strict casts, and projection casts (CLM-0176; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
A semantic cast is written `Term:Sort` and provides a compile-time annotation that the enclosed term must belong to the specified sort or one of its subsorts (CLM-0176; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
Semantic casts are completely erased at runtime and do not generate additional symbols in the abstract syntax tree (CLM-0176; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
Casting a variable in a rule, such as `X:Exp`, restricts pattern matching so that the variable matches only terms of sort `Exp` (CLM-0176; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
A strict cast is written `Term::Sort` or `{Term}::Sort` and specifies that the term must be exactly of sort `Sort`, explicitly rejecting subsorts (CLM-0177; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
Strict casts prevent ambiguities when multiple overloaded productions exist across parent and subsort hierarchies (CLM-0177; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
A projection cast is written `{Term}:>Sort` and performs a dynamic runtime cast that attempts to extract an injected subsort term from a parent sort container (CLM-0177; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
If the runtime term inside the parent container does not match the target subsort, the projection cast fails to evaluate, halting execution and returning the stuck projection symbol (CLM-0177; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
Subsort injections are tracked internally by the runtime symbol `inj{FromSort, ToSort}` (CLM-0177; SRC-0036 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-11-E
  imports INT
  imports BOOL

  syntax Exp ::= Int | Bool | Exp "+" Exp | Exp "&&" Exp
  syntax Exp ::= eval(Exp) [function]

  rule eval(I:Int) => I
  rule eval(B:Bool) => B
  rule eval(E1 + E2) => {eval(E1)}:>Int +Int {eval(E2)}:>Int
  rule eval(E1 && E2) => {eval(E1)}:>Bool andBool {eval(E2)}:>Bool
endmodule
```

## Lesson 1.12: Syntactic Lists

Syntactic repetition in K grammars is expressed cleanly using built-in syntactic list constructs (CLM-0178; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
The production `List{ElementSort, "separator"}` defines a sequence of zero or more elements separated by the specified terminal string (CLM-0178; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
Lists separated solely by whitespace are defined using the empty string separator `""` (CLM-0178; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
Syntactic lists are internally represented as cons-lists, where the empty list is written as a dot followed by the sort name, such as `.Ints` (CLM-0178; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
When writing rewrite rules over syntactic lists, K automatically inserts implicit list terminators whenever a bare element appears where a list is expected (CLM-0178; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
When parsing input programs with `krun`, K elaborates `List{}` into helper non-terminals `#NonEmpty<Sort>` and `#<Sort>Terminator` to parse comma-separated items without requiring trailing terminators (CLM-0179; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
The `NeList{ElementSort, "separator"}` construct enforces that the list must contain at least one element (CLM-0179; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
While `NeList{}` behaves identically to `List{}` when parsing rules, it rejects empty inputs when parsing user programs (CLM-0179; SRC-0036 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
The default parser entry sort `K` excludes syntactic lists from being direct start symbols to avoid grammar ambiguity errors on single-element inputs (CLM-0179; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-12-C
  imports LESSON-12-A
  imports INT

  syntax Int ::= sum(Ints) [function]
  rule sum(I:Int) => I
  rule sum(I1:Int, I2:Int, Is:Ints) => sum(I1 +Int I2, Is)
endmodule
```

## Lesson 1.13: Basics of K Rewriting

Top-level rewrite rules model operational state transitions over program configurations (CLM-0180; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
A top-level rewrite rule mentions one or more cells on its left-hand side, distinguishing it from an eager function rule (CLM-0180; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
The `<k>` cell is the primary computation cell, holding a sequence of tasks of sort `K` linked by the associative sequencing operator `~>` (CLM-0180; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Every sort declared in a definition is an implicit subsort of `KItem`, allowing arbitrary language terms to be injected into a K sequence (CLM-0180; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
The empty computation sequence is represented as `.` or `.K` (CLM-0180; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Execution proceeds by matching the configuration against all top-level rewrite rules, applying the substitution to the right-hand side to construct the successor state, and repeating until a final state is reached (CLM-0180; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
When multiple top-level rules match concurrently, concrete execution picks one non-deterministically; passing `--search` to `krun` explores all branches (CLM-0181; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
In concrete execution on the LLVM backend, rules cannot match on uninterpreted function symbols on the left-hand side (CLM-0181; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Local rewrites can be nested inside terms, such as `rule <k> (I1 + I2 => I1 +Int I2) ...</k>`, leaving the enclosing context untouched (CLM-0181; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Cell ellipses `...` indicate that unmentioned elements of a cell sequence or collection are preserved unchanged (CLM-0181; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Execution step limits can be enforced during testing by passing `--depth <N>` to `krun` (CLM-0181; SRC-0036 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-13-C
  imports LESSON-13-C-SYNTAX
  imports INT
  imports BOOL

  rule <k> I1:Int + I2:Int => I1 +Int I2 ...</k>
  rule <k> B1:Bool && B2:Bool => B1 andBool B2 ...</k>

  syntax KItem ::= freezer1(Val) | freezer2(Exp)
  rule <k> E1:Val + E2:Exp => E2 ~> freezer1(E1) ...</k> [priority(51)]
  rule <k> E1:Exp + E2:Exp => E1 ~> freezer2(E2) ...</k> [priority(52)]
  rule <k> E2:Val ~> freezer1(E1) => E1 + E2 ...</k>
  rule <k> E1:Val ~> freezer2(E2) => E1 + E2 ...</k>
endmodule
```

## Lesson 1.14: Defining Evaluation Order

Evaluation orders of nested expressions in K are formalized using heating and cooling rules or declarative strictness attributes (CLM-0182; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Heating suspends an unevaluated expression, moves a subterm to the front of the `<k>` cell, and places a freezer item on the computation sequence to store the evaluation context (CLM-0182; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Cooling restores the evaluated result from the front of the `<k>` cell back into the suspended freezer once the subterm evaluates to a value (CLM-0182; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
The predicate `isKResult(K)` identifies whether a term is fully evaluated, returning `true` for values and `false` for terms requiring further reduction (CLM-0182; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
The rule attributes `[heat]` and `[cool]` mark heating and cooling rules, causing the compiler to insert implicit `isKResult` side conditions over the special variable `HOLE` (CLM-0182; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Context sentences declared with `context <k> E + HOLE ...</k> requires isKResult(E)` replace manual heating, cooling, and freezer declarations with a single specification (CLM-0182; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Declarative strictness attributes eliminate context sentences entirely: `[seqstrict]` generates sequential left-to-right evaluation contexts for specified non-terminal positions (CLM-0183; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
The `[strict]` attribute generates non-deterministic heating rules, allowing sub-expressions to evaluate in arbitrary order as in C arithmetic (CLM-0183; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
The `[hybrid]` attribute marks constructors that act as values when their arguments are values but heat when their arguments are reducible (CLM-0183; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; repository observation; not reproduced; high; S6).
Named context aliases declared with `context alias [name]: <k> HERE ...</k>` allow strictness attributes to reference specialized evaluation contexts (CLM-0183; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Omitting argument indices in `[seqstrict]` or `[strict]` applies evaluation ordering to every non-terminal in the production (CLM-0183; SRC-0036 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-14-D-SYNTAX
  imports UNSIGNED-INT-SYNTAX
  imports BOOL-SYNTAX

  syntax Exp ::= Int | Bool
               > left: Exp "+" Exp [seqstrict]
               > left: Exp "&&" Exp [seqstrict]
endmodule

module LESSON-14-D
  imports LESSON-14-D-SYNTAX
  imports INT
  imports BOOL

  rule <k> I1:Int + I2:Int => I1 +Int I2 ...</k>
  rule <k> B1:Bool && B2:Bool => B1 andBool B2 ...</k>

  syntax Bool ::= isKResult(K) [function, symbol]
  rule isKResult(_:Int) => true
  rule isKResult(_:Bool) => true
  rule isKResult(_) => false [owise]
endmodule
```

## Lesson 1.15: Configuration Declarations and Cell Nesting

The abstract machine state of a language in K is formalized through configuration declarations (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
A configuration is declared using the `configuration` keyword, specifying a tree of nested cells using XML-style tags (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
If no configuration is explicitly provided, K imports `configuration <k> $PGM:K </k>` from `DEFAULT-CONFIGURATION` in `kast.md` (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Configuration variables begin with a `$` prefix, such as `$PGM`, and act as dynamic inputs supplied via `krun` (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
The sort ascribed to `$PGM` establishes the parser start symbol for input programs (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Cells can be statically initialized with constant values, such as `<sum> 0 </sum>` or `<state> .Map </state>`, without command-line variables (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Cells can be nested to create hierarchical configurations, such as placing `<first>` and `<second>` cells inside a `<state>` container enclosed within `<T>` (CLM-0184; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Configuration abstraction automatically projects rules mentioning a subset of cells into the full configuration tree, eliminating boilerplate (CLM-0185; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Unlike top-level rules, cells inside function rules complete only up to the topmost cell mentioned in the rule body (CLM-0185; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Each cell declaration generates an implicit cell sort named by capitalizing the cell tag and appending `Cell`, such as `StateCell` (CLM-0185; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Cell ellipses `...` on the left-hand side ignore unmentioned sibling cells, while ellipses on the right-hand side reset unmentioned cells to their default configuration values (CLM-0185; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
The phrase `.Bag` represents the empty set of cells (CLM-0185; SRC-0036 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-15-C
  imports LESSON-15-C-SYNTAX
  imports INT
  imports BOOL

  configuration <T>
                  <k> . </k>
                  <state>
                    <first> $FIRST:Int </first>
                    <second> $SECOND:Int </second>
                  </state>
                </T>

  rule <k> . => FIRST >Int SECOND </k>
       <first> FIRST </first>
       <second> SECOND </second>
endmodule
```

## Lesson 1.16: Maps, Semantic Lists, and Sets

K provides built-in associative collection sorts in `domains.md`: `Map`, `List`, and `Set` (CLM-0186; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
The `Map` sort, defined in the `MAP` module, maps terms of sort `KItem` to terms of sort `KItem` (CLM-0186; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Individual key-value bindings are constructed using `Key |-> Value`, and the empty map is represented by `.Map` (CLM-0186; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Deterministic map lookups match in $O(1)$ time, while non-deterministic map matching introduces polynomial complexity per unconstrained key (CLM-0186; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Left-hand-side map patterns may contain at most one variable of sort `Map` to capture remaining bindings (CLM-0186; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Map updates are performed using the symbol `Map [ Key <- Value ]` (CLM-0186; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
The `List` sort, defined in the `LIST` module, represents associative sequences of `KItem` elements wrapped in `ListItem(Item)` (CLM-0187; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Empty lists are written `.List`, and list patterns support $O(\log N)$ prefix and suffix matching with at most one `List` variable on the left-hand side (CLM-0187; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Cell ellipses in list cells allow `<cell> ListItem(X) ...</cell>` for head matching and `<cell>... ListItem(X) </cell>` for tail matching (CLM-0187; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
The `Set` sort, defined in the `SET` module, represents deduplicated collections of `KItem` elements wrapped in `SetItem(Item)`, with `.Set` denoting the empty set (CLM-0187; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Set membership is tested using `Item in Set` or `notBool (Item in Set)` (CLM-0187; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).
Program variable identifiers are modeled using the built-in `Id` sort from `ID-SYNTAX`, with concrete tokens declared via `syntax Id ::= "name" [token]` (CLM-0187; SRC-0036 k-distribution/k-tutorial/1_basic/16_collections/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-16-C
  imports LESSON-16-C-SYNTAX
  imports BOOL
  imports SET

  configuration <T>
                  <k> $PGM:Pgm </k>
                  <state> .Map </state>
                  <declared> .Set </declared>
                </T>

  rule <k> int X:Id = I:Int ; => . ...</k>
       <state> STATE => STATE [ X <- I ] </state>
       <declared> D => D SetItem(X) </declared>
    requires notBool X in D

  rule <k> X:Id => I ...</k>
       <state>... X |-> I ...</state>
       <declared>... SetItem(X) ...</declared>
endmodule
```

## Lesson 1.17: Cell Multiplicity and Cell Collections

Cell multiplicity attributes allow configuration cells to be optional or to repeat dynamically (CLM-0188; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
Cell attributes in configuration declarations are specified using XML-like syntax, such as `<optional multiplicity="?"> 0 </optional>` (CLM-0188; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
A multiplicity of `?` indicates an optional cell that appears zero or one times in the configuration (CLM-0188; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
Optional cells are absent by default in the initial state unless populated by a configuration variable or given the attribute `initial=""` (CLM-0188; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
Rules create optional cells by rewriting `.Bag => <cell> ... </cell>` and destroy them by rewriting `<cell> ... </cell> => .Bag` (CLM-0188; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
A multiplicity of `*` defines a cell collection that repeats zero or more times, representing dynamic structures such as concurrent threads or object instances (CLM-0188; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
A repeating cell must be the sole child of its parent cell, following the convention where the inner cell is singular and the outer cell is plural (CLM-0189; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
Cell collections require a `type` attribute set to either `Set` or `Map` (CLM-0189; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
When typed as `Map`, the first subcell of the repeating cell acts as the map key, and the remaining subcells act as the value (CLM-0189; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
The computation cell `<k>` can reside inside a repeating `<thread>` cell, enabling concurrent thread semantics with fork/join operations (CLM-0189; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).
Rules that omit thread identifiers are completed automatically through configuration abstraction to match any thread in the collection (CLM-0189; SRC-0036 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).

```k
configuration <threads>
                <thread multiplicity="*" type="Map">
                  <id> 0 </id>
                  <k> $PGM:K </k>
                </thread>
              </threads>
              <state> .Map </state>
              <next-id> 1 </next-id>

rule <thread>...
       <k> spawn { Ss } => NEXTID ...</k>
     ...</thread>
     <next-id> NEXTID => NEXTID +Int 1 </next-id>
     (.Bag =>
     <thread>
       <id> NEXTID </id>
       <k> Ss </k>
     </thread>)
```

## Lesson 1.18: Term Equality and the Ternary Operator

Term equality in K can be evaluated through pattern variable unification or through explicit equality operators (CLM-0190; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
Syntactic equality occurs when two terms match the same variable name on the left-hand side of a rule (CLM-0190; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
Semantic equality across arbitrary terms of sort `K` is provided by the `K-EQUAL` module in `domains.md` via the `==K` operator (CLM-0190; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
The `==K` operator returns `true` if two terms are equal, correctly evaluating non-structural equality over built-in collections such as `Map` and `Set` (CLM-0190; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
The corresponding semantic inequality operator is `=/=K` (CLM-0190; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
Conditional branching on the right-hand side of rules can be written using the polymorphic ternary operator `#if B #then E1 #else E2 #fi` from `K-EQUAL` (CLM-0190; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
The condition `B` must evaluate to sort `Bool`, while `E1` and `E2` may be of any sort provided both share the same sort (CLM-0191; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
While the concrete execution backend evaluates only the chosen branch of an `#if` expression, symbolic execution in verification may explore both branches (CLM-0191; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
The tutorial warns against using `#if` expressions where an unselected branch could be mathematically undefined, advising rule side conditions instead for short-circuiting behavior (CLM-0191; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).
K supports polymorphic built-in operators such as `#if`, but user-defined polymorphic productions are not currently permitted (CLM-0191; SRC-0036 k-distribution/k-tutorial/1_basic/18_equality_and_conditionals/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-18
  imports INT
  imports BOOL
  imports K-EQUAL

  syntax Exp ::= Int | Bool | "if" "(" Exp ")" Exp "else" Exp [strict(1)]
  syntax Bool ::= isKResult(K) [function, symbol]
  rule isKResult(_:Int) => true
  rule isKResult(_:Bool) => true

  rule if (B:Bool) E1:Exp else E2:Exp => #if B #then E1 #else E2 #fi
endmodule
```

## Lesson 1.19: Debugging with GDB or LLDB

K provides native debugger integration for compiled LLVM interpreters on Linux via GDB and macOS via LLDB (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Enabling debug support requires compiling the semantics with `kompile --enable-llvm-debug` and running the program with `krun --debugger` (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
On Linux, users must add `add-auto-load-safe-path` to `~/.gdbinit` to trust the compiled interpreter directory and enable Python AST pretty-printers (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
On macOS, LLDB loads debug scripts from the `.dSYM` directory using `settings set target.load-script-from-symbol-file true` (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
The command `k start` initializes the configuration and breaks immediately before the first rewrite step (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
The command `k step` advances execution by one rewrite step, while `k step <N>` advances by `N` steps (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Breakpoints can be placed on source lines using `break <file>:<line>` in GDB or `breakpoint set --file <file> --line <line>` in LLDB (CLM-0192; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Breakpoints on rule applications use rule labels with the syntax `<MODULE>.<label>.rhs`, exposing matching variable substitutions (CLM-0193; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Breakpoints on rule side conditions use the syntax `<MODULE>.<label>.sc`, allowing inspection of guard parameters before evaluation (CLM-0193; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Function calls can be intercepted by breaking on the mangled symbol name `Lbl<function>` (CLM-0193; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Regular expression breakpoints can be set across groups of functions using `rbreak <regex>` (CLM-0193; SRC-0036 docs/ktools.md; source fact; not reproduced; high; S6).
The command `k match <MODULE>.<label> subject` logs the exact subterm and pattern mismatch explaining why a candidate rule failed to match (CLM-0193; SRC-0036 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-19-C
  imports INT
  imports BOOL

  syntax Bool ::= isEven(Int) [function]
  rule [isEven]: isEven(I) => true requires I %Int 2 ==Int 0
  rule [isOdd]: isEven(I) => false requires I %Int 2 =/=Int 0
endmodule
```

## Lesson 1.20: K Backends and the Haskell Backend

The K compiler separates frontend processing from backend execution engines (CLM-0194; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The default backend is the LLVM Backend, which compiles K definitions to LLVM bitcode and native machine binaries optimized for concrete execution and state-space search (CLM-0194; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The Haskell Backend is an interpreter written in Haskell optimized for symbolic execution and formal verification (CLM-0194; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
Target backends are selected via `kompile --backend <backend>`, choosing between `llvm` and `haskell` (CLM-0194; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
Unlike the LLVM backend which generates specialized C++ and machine code, the Haskell backend interprets the KORE intermediate representation directly (CLM-0194; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
While the Haskell backend can execute concrete programs, its performance is significantly slower than the LLVM backend (CLM-0195; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
On Apple Silicon ARM64 macOS machines, a known issue with the `Compact` library requires passing `--no-haskell-binary` to `kompile` and `krun` to prevent crashes (CLM-0195; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The legacy Java Backend was the historical precursor to the Haskell backend and is completely deprecated (CLM-0195; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The LLVM backend is recommended for running large test suites, while the Haskell backend is mandatory for deductive proofs (CLM-0195; SRC-0036 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-20
  imports INT

  rule I => I +Int 1
    requires I <Int 100
endmodule
```

## Lesson 1.21: Unification and Symbolic Execution

Symbolic execution in K generalizes concrete execution by evaluating configurations containing logical variables and constraints (CLM-0196; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Concrete execution on the LLVM backend requires terms without free logical variables, whereas the Haskell backend executes over symbolic terms (CLM-0196; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Symbolic execution replaces one-way pattern matching with two-way unification, computing the most general unifier (MGU) between symbolic configurations and rule patterns (CLM-0196; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Path condition satisfiability is checked by querying the Z3 SMT solver, which prunes mathematically infeasible execution branches (CLM-0196; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Symbolic variables can be introduced on the right-hand side of rules by prefixing uppercase variable names with `?`, such as `?X:Int` (CLM-0196; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Rules introducing fresh existential variables can specify constraints on those variables using the `ensures` clause (CLM-0197; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Symbolic configurations are output as disjunctions of conjunctions using matching logic connectives `#And`, `#Or`, and `#Equals` (CLM-0197; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Each conjunction represents a reachable configuration paired with the mathematical path condition derived during execution (CLM-0197; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Constraints on fresh variables in `ensures` clauses prevent infinite symbolic loops by constraining repeated application of generative rules (CLM-0197; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Symbolic execution forms the underlying foundation of deductive program verification in K (CLM-0197; SRC-0036 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).

```k
module LESSON-21
  imports INT

  rule <k> 0 => ?X:Int ... </k> ensures ?X =/=Int 0
  rule <k> X:Int => 5  ... </k> requires X >=Int 10
endmodule
```

## Lesson 1.22: Basics of Deductive Program Verification using K

Deductive program verification in K proves that programs satisfy formal specifications using reachability logic (CLM-0198; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
The verification target is formalized as a set of specifications containing `claim` declarations in a separate specification file (CLM-0198; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
The verifier is invoked using `kprove <specification-file>.k` against a semantics compiled with `--backend haskell` (CLM-0198; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
When all claims are proven sound inductive consequences of the semantics, `kprove` returns the pattern `#Top` (CLM-0198; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
Claims reason about concrete program execution, branching conditional paths, and loops (CLM-0198; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
When a proof fails or gets stuck, `kprove` outputs diagnostic information consisting of unproven reachability implications, negative matching logic constraints, the stuck configuration, and positive path conditions (CLM-0198; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
Unproven constraints are discharged by introducing domain-specific simplification rules marked with the `[simplification]` attribute into a verification module (CLM-0199; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
Simplification rules apply eagerly across symbolic terms and constraints without completing to the top of the configuration (CLM-0199; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
Loop verification requires formulating claims that act as inductive loop invariants over symbolic variables (CLM-0199; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
Recursive functions and function calls are verified by stacking continuation frames and unrolling calls against function store maps (CLM-0199; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
Unique symbol declarations carry `[symbol]` or `[klabel(...)]` to prevent label collision across verification modules (CLM-0199; SRC-0036 k-distribution/k-tutorial/1_basic/22_proofs/README.md; repository observation; not reproduced; high; S6).

```k
requires "lesson-22.k"
requires "domains.md"

module VERIFICATION
  imports K-EQUAL
  imports LESSON-22
  imports MAP-SYMBOLIC

  rule { M:Map [ K <- V ] #Equals M [ K <- V' ] } => { V #Equals V' } [simplification]
endmodule

module LESSON-22-SPEC
  imports VERIFICATION

  claim <k> while ( 0 < $n ) {
              $s = $s + $n;
              $n = $n - 1;
            } => . ... </k>
        <store>
          $s |-> (S:Int => S +Int ((N +Int 1) *Int N /Int 2))
          $n |-> (N:Int => 0)
        </store>
    requires N >=Int 0
endmodule
```
