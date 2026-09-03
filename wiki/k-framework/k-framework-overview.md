---
id: k.framework.overview
type: overview
title: K Framework overview
status: active
updated_at: 2026-09-03T14:34:30Z
sources:
  - SRC-0023
  - SRC-0024
---

# K Framework overview

## Introduction and core theoretical foundations

The K Framework is a semantic engineering platform designed for the formal specification, execution, and verification of programming languages, virtual machines, and hardware architectures (CLM-0123; SRC-0023 README.md; source fact; not reproduced; high; S6).
The repository README defines the system with the following normative statement:
> "The K Framework is a tool for designing and modeling programming languages and software/hardware systems. At the core of the K Framework is a programming, modeling, and specification language called K. The K Framework includes tools for compiling K specifications to build interpreters, model checkers, verifiers, associated documentation, and more." (CLM-0123; SRC-0023 README.md; source fact; not reproduced; high; S6).

At the foundational level, K is grounded in term rewriting logic and matching logic (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
In rewriting logic, program states are formalized as algebraic terms, and program computations are formalized as transitions governed by rewrite rules (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Matching logic unifies operational semantics, type systems, and axiomatic verification within a single formal framework (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Unlike classical logics that evaluate formulas strictly to truth values, matching logic evaluates formulas, called patterns, to sets of matching configurations (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
A pattern simultaneously specifies the structural shape of a state and the logical constraints that must hold over that state (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Matching logic introduces first-class connectives that operate directly over patterns, including `#Top` representing the entire universe of configurations, `#Bottom` representing the empty set of configurations, `#And` representing pattern conjunction or intersection, `#Or` representing pattern disjunction or union, `#Not` representing pattern complementation, and `#Equals` asserting term equality within a pattern (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).

All language specifications written in K are compiled to an intermediate representation known as KORE, which stands for K Outer Representation (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
KORE serves as the universal mathematical exchange format between the language frontend and all execution backends (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
A KORE specification represents modules, sorts, symbols, axioms, and rewrite rules in a stripped, mathematically uniform syntax (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Every high-level K definition is compiled by the frontend compiler into a canonical KORE file named `definition.kore` (CLM-0124; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
When terms are converted into KORE ASTs, subsorting relationships are formalized explicitly using the runtime injection symbol `inj{SortFrom{}, SortTo{}}` (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
This two-tiered architecture ensures that front-end syntactic conveniences, such as literate programming in Markdown, custom precedence declarations, and configuration abstraction, remain decoupled from the verification and execution backends (CLM-0124; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

The grammar of K is strictly bifurcated into outer syntax and inner syntax (CLM-0125; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
The repository tutorial defines this division as follows:
> "K's grammar is divided into two components: the outer syntax of K and the inner syntax of K. Outer syntax refers to the parsing of requires, modules, imports, and sentences in a K definition. Inner syntax refers to the parsing of rules and programs. Unlike the outer syntax of K, which is predetermined, much of the inner syntax of K is defined by you, the developer." (CLM-0125; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
The outer syntax defines the top-level structural scaffolding of a definition, including file requirements, module boundaries, module imports, and sentence categories such as syntax declarations, context declarations, configurations, and rules (CLM-0125; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
The inner syntax defines the user-specified grammar of the target language being modeled, including expressions, statements, types, and rewriting rules (CLM-0125; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Rules are parsed within the syntactic context of the specific module in which they are declared (CLM-0125; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Target programs executed by the generated interpreter are parsed within the syntactic context of the designated main syntax module (CLM-0125; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).

Further foundational details on syntax, parsing, and rule mechanics appear in [Section 1 Basic Concepts](k-tutorial-basic.md) and [Intermediate K Concepts](k-tutorial-intermediate.md).
Built-in mathematical domains and data structures are documented in [K Built-in Domains](k-builtins.md).

## Monorepo layout and architectural organization

The canonical K Framework implementation is maintained as a multi-language monorepo incorporating Java, Scala, Haskell, C++, Python, and shell scripts (CLM-0126; SRC-0023 README.md; repository observation; not reproduced; high; S6).
The public crawl at `kframework.org` establishes that `runtimeverification/k` is the primary and authoritative repository for the toolchain (CLM-0126; SRC-0024; source fact; not reproduced; high; S6).
The root directory coordinates sub-projects, native Git submodules, Maven POM hierarchies, Nix build expressions, and packaging definitions (CLM-0126; SRC-0023 pom.xml; repository observation; not reproduced; high; S6).

The `k-frontend` directory contains the compiler frontend implemented in Java and Scala (CLM-0127; SRC-0023 pom.xml; repository observation; not reproduced; high; S6).
The frontend coordinates all initial processing of K source files (CLM-0127; SRC-0023 k-frontend/src/main/java/org/kframework/kompile; repository observation; not reproduced; high; S6).
It parses outer syntax using JavaCC grammar definitions (CLM-0127; SRC-0023 README.md; repository observation; not reproduced; high; S6).
It parses literate Markdown documents by extracting code blocks that match specified selectors (CLM-0127; SRC-0023 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
It recursively traverses `requires` directives across file boundaries and resolves module `imports` hierarchies (CLM-0127; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
It performs macro expansions, sort inference, and type disambiguation across rewrite rules (CLM-0127; SRC-0023 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).
The frontend computes configuration abstraction, automatically expanding sparse cell references into the full configuration tree (CLM-0127; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
It synthesizes heating and cooling rules from production strictness annotations (`[strict]`, `[seqstrict]`) (CLM-0127; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Finally, it lowers the high-level K definition into the normalized KORE intermediate representation (CLM-0127; SRC-0023 k-frontend/src/main/java/org/kframework/kompile; repository observation; not reproduced; high; S6).

The `k-distribution` directory defines the assembly and packaging configuration for the complete distribution (CLM-0128; SRC-0023 README.md; repository observation; not reproduced; high; S6).
It constructs the complete Java classpath required to execute the compiler and distribution tools (CLM-0128; SRC-0023 README.md; repository observation; not reproduced; high; S6).
The directory hosts the standard library definitions under `k-distribution/include/kframework/builtin/`, including `domains.md` and `kast.md` (CLM-0128; SRC-0023 k-distribution/k-tutorial/1_basic/06_ints_and_bools/README.md; repository observation; not reproduced; high; S6).
The standard tutorial series, regression test harnesses, and command-line wrapper scripts reside within this subsystem (CLM-0128; SRC-0023 k-distribution/k-tutorial/README.md; repository observation; not reproduced; high; S6).
Binary scripts exposed to end users, such as `kompile`, `krun`, `kast`, `kprove`, and `kserver`, are staged from `k-distribution/src/main/scripts/bin/` (CLM-0128; SRC-0023 k-distribution/src/main/scripts/bin; repository observation; not reproduced; high; S6).

The `llvm-backend` component is maintained as a native Git submodule at `llvm-backend/src/main/native/llvm-backend`, tracking `https://github.com/runtimeverification/llvm-backend` (CLM-0129; SRC-0023 .gitmodules; repository observation; not reproduced; high; S6).
Implemented in C++ and LLVM, this backend compiles KORE definitions into native LLVM bitcode (CLM-0129; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
It compiles pattern-matching rules into deterministic decision trees to maximize concrete execution throughput (CLM-0129; SRC-0023 llvm-backend/src/main/native/llvm-backend; repository observation; not reproduced; high; S6).
The backend links generated code against high-performance runtime libraries, including GMP for arbitrary-precision integers, MPFR for floating-point arithmetic, and Jemalloc for memory management (CLM-0129; SRC-0023 README.md; source fact; not reproduced; high; S6).
It provides GNU Bison integration to emit ahead-of-time LR(1) and GLR binary parsers for target programming languages (CLM-0129; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
It also injects debug metadata into generated binaries to enable source-level debugging in GDB and LLDB (CLM-0129; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).

The `haskell-backend` component is maintained as a native Git submodule at `haskell-backend/src/main/native/haskell-backend`, tracking `https://github.com/runtimeverification/haskell-backend` (CLM-0130; SRC-0023 .gitmodules; repository observation; not reproduced; high; S6).
Implemented in Haskell, this subsystem functions as an interpreter and symbolic execution engine for KORE definitions (CLM-0130; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
It evaluates KORE rewrite rules and claims using term unification rather than one-way pattern matching (CLM-0130; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
It drives reachability logic proofs in `kprove`, verifying that language specifications satisfy stated temporal and functional assertions (CLM-0130; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
The Haskell backend communicates directly with the Z3 SMT solver via its C API, checking the satisfiability of path conditions and pruning unreachable execution branches (CLM-0130; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
It also powers the interactive proof debugger `kore-repl` (CLM-0130; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).

The `pyk` directory hosts the Python SDK and orchestration library for K (CLM-0131; SRC-0023 README.md; repository observation; not reproduced; high; S6).
The repository notes that primary Python support for K is hosted under `runtimeverification/pyk` (CLM-0131; SRC-0023 README.md; source fact; not reproduced; high; S6).
The library provides object models for KORE data structures, AST parsing and unparsing helpers, and serialization pipelines (CLM-0131; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
`pyk` implements client bindings for the Kore RPC server and booster daemon, enabling external programs to drive execution, submit simplification queries, and guide reachability proofs programmatically (CLM-0131; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
It also includes output minimization utilities (`pyk print --minimize`) to filter uninteresting cells from large symbolic configurations during verification (CLM-0131; SRC-0023 docs/ktools.md; source fact; not reproduced; high; S6).

For in-depth analysis of backend architecture and tool components, see [K Backends and Tooling](k-backends-and-tools.md).

## Supported install paths and system requirements

The K Framework provides multiple distribution channels tailored to end users, continuous integration infrastructure, and core compiler developers (CLM-0132; SRC-0023 README.md; source fact; not reproduced; high; S6).

The preferred installation mechanism for non-developer users across all systems supporting Nix is the `kup` package manager (CLM-0132; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
The tool abstracts Nix internals and automates toolchain downloads (CLM-0132; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
The streamlined setup executes via a two-step shell sequence:
```shell
bash <(curl https://kframework.org/install)
kup install k
```
Users inspect available versions using `kup list k` and install specific releases using `kup install k --version <version>` (CLM-0132; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Releases marked with a checkmark in `kup` are cached in Runtime Verification's Nix binary cache, enabling near-instantaneous installation of pre-compiled binaries (CLM-0132; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).

The repository provides first-class support for reproducible builds using Nix flakes (CLM-0133; SRC-0023 README.md; source fact; not reproduced; high; S6).
Using Nix version 2.4 or higher with `nix-command` and `flakes` enabled in `nix.conf`, developers build the entire framework from source using:
```shell
nix build .
```
The resulting binaries are symlinked into the local `result/bin` directory (CLM-0133; SRC-0023 README.md; source fact; not reproduced; high; S6).
A developer can add K binaries, including `kompile` and `kast`, to their transient shell session via `nix shell .` (CLM-0133; SRC-0023 README.md; source fact; not reproduced; high; S6).
Integration test suites are executed via `nix build .#test` (CLM-0133; SRC-0023 README.md; source fact; not reproduced; high; S6).
Whenever dependencies in `pom.xml` change, developers update the lockfile via `nix run .#update-maven` (CLM-0133; SRC-0023 README.md; source fact; not reproduced; high; S6).
To prevent lengthy compilation from source, Runtime Verification maintains a public binary cache at `https://app.cachix.org/cache/k-framework` (CLM-0133; SRC-0023 README.md; source fact; not reproduced; high; S6).

For Debian-based distributions, specifically Ubuntu Jammy Jellyfish (22.04), pre-built binary packages are distributed as `.deb` archives via GitHub Releases (CLM-0134; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Installation is performed using the standard package manager:
```shell
sudo apt install ./kframework_amd64_ubuntu_jammy.deb
```
On Linux, K installs its binaries and assets under the `/usr` prefix (CLM-0134; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Installing the full package requires approximately 1.4 GB of platform and library dependencies (CLM-0134; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).

For macOS, official binary bottles are distributed via Homebrew (CLM-0135; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Users install K by tapping the official package repository:
```shell
brew tap runtimeverification/k
brew install kframework
```
On macOS, K installs under `/usr/local` on x86-64 hardware or the Homebrew prefix `/opt/homebrew` on Apple Silicon (CLM-0135; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Apple Silicon machines (M1/M2/M3) are fully tested and supported, but require explicit environment configuration when building from source due to upstream issues in the Haskell Stack and Homebrew LLVM toolchains (CLM-0135; SRC-0023 README.md; source fact; not reproduced; high; S6).
Specifically, Homebrew LLVM is keg-only, requiring users to place Homebrew's `llvm-config`, `flex`, and `bison` binaries ahead of system tools in the `PATH` variable using `direnv` and `macos-envrc` (CLM-0135; SRC-0023 README.md; source fact; not reproduced; high; S6).

Container images with pre-installed K toolchains are published to Docker Hub under `runtimeverificationinc/kframework-k` (CLM-0136; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Every continuous integration release produces a tag of the form `ubuntu-jammy-<COMMIT_ID>` (CLM-0136; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).
Developers run interactive verification environments directly:
```shell
docker run -it runtimeverificationinc/kframework-k:ubuntu-jammy-<COMMIT_ID>
```
Dockerfiles can base multi-stage builds directly on these release tags:
```dockerfile
FROM runtimeverificationinc/kframework-k:ubuntu-jammy-<COMMIT_ID>
```

Building the K Framework directly from source requires Maven, JDK 17 or greater, and Haskell Stack (CLM-0137; SRC-0023 README.md; source fact; not reproduced; high; S6).
Platform dependencies include Bison, Boost, CMake, Flex, Fmt, GCC, GMP, Libjemalloc, Libsecp256k1, LibYAML, LLVM/Clang 15 or greater, LLD, GNU Make, MPFR, Pkg-config, Python 3, Stack, XXD, Zlib, and Z3 (CLM-0137; SRC-0023 README.md; source fact; not reproduced; high; S6).
A contradiction exists in documentation regarding the exact required version of Z3: `README.md` explicitly mandates Z3 version 4.12.1, noting that other versions exhibit bugs and performance regressions in test suites, whereas `INSTALL.md` references version 4.8.15 (CLM-0137; SRC-0023 README.md, INSTALL.md; contradiction; not reproduced; high; S6).
Building the release distribution is initiated with `mvn package` from the repository root (CLM-0137; SRC-0023 README.md; source fact; not reproduced; high; S6).
Setting `MAVEN_OPTS="-XX:+TieredCompilation"` is strongly recommended to accelerate incremental builds (CLM-0137; SRC-0023 README.md; source fact; not reproduced; high; S6).

System architecture boundaries are strictly enforced (CLM-0138; SRC-0023 README.md; source fact; not reproduced; high; S6).
K can only be built and executed on 64-bit Linux-like operating systems (CLM-0138; SRC-0023 README.md; source fact; not reproduced; high; S6).
All 32-bit platforms are completely unsupported (CLM-0138; SRC-0023 README.md; source fact; not reproduced; high; S6).
Native Windows execution is not supported; Windows 10 and 11 users must install K inside the Windows Subsystem for Linux or within a 64-bit Linux virtual machine (CLM-0138; SRC-0023 k-distribution/INSTALL.md; source fact; not reproduced; high; S6).

For Windows Subsystem for Linux, WSL version 2 is mandatory (CLM-0139; SRC-0023 README.md; source fact; not reproduced; high; S6).
WSL version 1 fails when building or running the Haskell backend due to a known file-locking defect in GHC under the WSL1 compatibility layer (CLM-0139; SRC-0023 README.md; source fact; not reproduced; high; S6).
The failure produces the following diagnostic:
```text
ghc-pkg: Couldn't open database $HOME/.stack/programs/x86_64-linux/.../package.cache.lock:
{handle: ...}: hLock: invalid argument (Invalid argument)
```
Users on WSL1 must upgrade to WSL2, switch to a native Linux installation, or skip compiling the Haskell backend by passing `-Dhaskell.backend.skip` to Maven (CLM-0139; SRC-0023 README.md; source fact; not reproduced; high; S6).

For operational command syntax and system configuration guidelines, consult [K User Manual](k-user-manual.md).

## Toolchain commands and operational workflow

The K Framework provides a comprehensive command-line toolchain that supports every phase of language design, syntactic analysis, native execution, reachability verification, and debugging (CLM-0140; SRC-0023 docs/cheat_sheet.md; source fact; not reproduced; high; S6).

### `kompile`

The `kompile` executable is the central compiler of the K Framework (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
It ingests a K definition, parses its outer and inner syntax, verifies sort signatures, elaborates configuration hierarchies, synthesizes evaluation contexts, and produces a compiled directory `<name>-kompiled/` containing the execution engine (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Essential command-line flags include:
- `--backend [llvm|haskell]`: selects the compilation target backend, defaulting to the concrete `llvm` backend (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
- `--main-module <NAME>`: sets the primary semantics module, defaulting to the uppercase filename base (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
- `--syntax-module <NAME>`: sets the program parsing module, defaulting to `<MAIN-MODULE>-SYNTAX` (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
- `--gen-bison-parser`: generates an ahead-of-time LR(1) parser using GNU Bison for fast program parsing (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
- `--gen-glr-bison-parser`: generates an ahead-of-time GLR parser using GNU Bison, preserving ambiguous parse forests (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
- `--enable-llvm-debug`: compiles the LLVM interpreter with debugging symbols and preserves rule labels for GDB and LLDB (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
- `--enable-search`: prepares LLVM backend definitions for state-space exploration and non-deterministic search (CLM-0140; SRC-0023 docs/cheat_sheet.md; source fact; not reproduced; high; S6).
- `--md-selector "<expr>"`: evaluates boolean expressions over Markdown code block tags in literate definitions (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; source fact; not reproduced; high; S6).
- `-I <dir>`: prepends directory paths for resolving relative `requires` statements (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
- `--no-haskell-binary`: disables binary serialization on Apple Silicon to prevent crashes in the Haskell backend (CLM-0140; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

### `krun`

The `krun` tool executes programs using the compiled language definition (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
It parses the program input, initializes configuration cells, applies top-level rewrite rules until a terminal configuration is reached, and pretty-prints the output configuration (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/09_unparsing/README.md; source fact; not reproduced; high; S6).
Operational options include:
- `-c<VAR>=<value>`: passes explicit configuration variables on the command line, such as `-cPGM='...'` or custom flags (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
- `--definition <dir>`: specifies the directory containing the compiled language definition (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
- `--search`: explores all non-deterministic execution paths, returning all reachable final configurations (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
- `--search-all`: explores all non-deterministic paths, returning both final and intermediate states (CLM-0141; SRC-0023 docs/cheat_sheet.md; source fact; not reproduced; high; S6).
- `--depth <N>`: bounds program execution to a maximum of `N` rewrite steps (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
- `--debugger`: launches the program inside a native debugger with K inspection scripts loaded (CLM-0141; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).

### `kast`

The `kast` utility is K's just-in-time parser and AST transformer (CLM-0142; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
It generates an in-memory Generalized Left-to-right (GLL) parser on the fly, allowing it to parse arbitrary context-free grammars and identify parse ambiguities (CLM-0142; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Key options include:
- `--output [kore|kast|json]`: controls the format of the emitted AST, emitting raw KORE, surface K terms, or JSON (CLM-0142; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
- `-e` or `--expression "<string>"`: parses inline text passed as a command argument rather than reading a file (CLM-0142; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).

### `kprove`

The `kprove` command is the automated program verifier of K, implementing reachability logic proof procedures (CLM-0143; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
It ingests a verification specification file containing `claim` declarations and attempts to prove that the claims are inductive consequences of the underlying operational semantics (CLM-0143; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
`kprove` relies on the Haskell backend and queries the Z3 SMT solver to discharge path constraints (CLM-0143; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
When a proof succeeds, `kprove` outputs `#Top` (CLM-0143; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
When a proof fails or gets stuck, it outputs the unproven symbolic configuration along with positive and negative path constraints (CLM-0143; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).

### `ksearch`

The `ksearch` tool is a specialized CLI executable for compiling and querying search patterns across state spaces (CLM-0144; SRC-0023 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
It allows users to search for configurations satisfying specific structural properties or patterns across non-deterministic transition systems (CLM-0144; SRC-0023 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
In modern development workflows, search functionality is typically accessed via `krun --search` or orchestrated programmatically via `pyk` (CLM-0144; SRC-0023 docs/cheat_sheet.md; repository observation; not reproduced; high; S6).

### `kore-repl`

The `kore-repl` executable provides an interactive symbolic execution shell and proof debugger (CLM-0145; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
Included with the Haskell backend, `kore-repl` allows engineers to step through reachability proof trees interactively (CLM-0145; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
Users can inspect proof branching points, apply individual rewrite rules, invoke domain-specific simplifications, examine SMT solver formulas, and diagnose why inductive hypotheses failed to apply (CLM-0145; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).

### Auxiliary utilities: `kserver`, `kparse`, and `kore-print`

The toolchain provides specialized utilities to enhance performance and usability (CLM-0146; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
`kserver` is a daemon based on Nailgun that maintains a resident JVM process, reducing JVM startup latency and caching rule parsing tables across repeated compiler invocations (CLM-0146; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
The repository notes that Nailgun has not been updated in recent years and is not compatible with Java 18 onwards (CLM-0146; SRC-0023 docs/ktools.md; source fact; not reproduced; high; S6).
`kparse` directly executes ahead-of-time Bison binary parsers, bypassing the just-in-time parser for maximum parsing speed (CLM-0146; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
`kore-print` pretty-prints KORE AST terms back into user-defined concrete syntax using the unparsing rules defined in the compiled semantics (CLM-0146; SRC-0023 docs/cheat_sheet.md; source fact; not reproduced; high; S6).

## Organization of a language definition

A complete language specification in K is structured around four primary concepts: modules, syntax definitions, configuration declarations, and rewrite rules (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).

### Modules and import scoping

Modules are the fundamental organizational units of a K definition (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
A module begins with the keyword `module`, followed by a module name, and concludes with `endmodule` (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Module names consist of alphanumeric identifiers and hyphens, conventionally styled in uppercase (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
A module incorporates sentences from other modules using `imports <MODULE>` declarations (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
To combine definitions across multiple disk files, developers place `requires "<file>"` directives at the top of files before module declarations (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
The compiler recursively parses all required files and collects all declared modules into a global namespace (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Relative paths in `requires` statements are resolved against the current working directory, include directories passed via `-I`, and the built-in library directory `include/kframework/builtin/` (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).
Standard convention separates syntax from semantics by defining a `<NAME>-SYNTAX` module containing grammar productions and a `<NAME>` module containing rewrite rules and operational logic (CLM-0147; SRC-0023 k-distribution/k-tutorial/1_basic/05_modules/README.md; source fact; not reproduced; high; S6).

### Syntax productions and grammar disambiguation

Syntax declarations establish the grammar of both target programs and internal semantic operators (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Productions are declared using the `syntax` keyword, specifying a sort name, the `::=` operator, and a sequence of terminals in double quotes and non-terminal sort names (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
Sort names must begin with an uppercase letter (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
K distinguishes between constructor productions that build immutable AST nodes and function productions marked with `[function]` that evaluate eagerly (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
Subsorting is declared via productions of the form `syntax S ::= S2`, injecting sort `S2` into `S` (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/11_casts/README.md; source fact; not reproduced; high; S6).

Grammar disambiguation is controlled through several attributes and constructs:
- Priority blocks: productions separated by `>` define relative operator precedence, where tighter binding operators cannot appear as direct boundary children of looser binding operators (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
- Associativity: priority groups can be prefixed with `left:`, `right:`, or `non-assoc:` to resolve repeated applications of operators with equal precedence (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
- Explicit priority declarations: `syntax priority <groups>` and `syntax left/right/non-assoc <groups>` declare relations across disparate modules using `group(...)` attributes (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).
- Brackets: productions carrying `[bracket]` must contain exactly one non-terminal of the same sort; they guide parsing and are discarded from AST representations (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
- Tokens: productions carrying `[token]` define lexical tokens via regular expressions `r"..."` using Flex regex syntax, avoiding AST clutter (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).
- Syntactic lists: `List{Sort, "separator"}` and `NeList{Sort, "separator"}` declare arbitrary-length sequences and non-empty sequences (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/12_syntactic_lists/README.md; source fact; not reproduced; high; S6).
- Preferences: `[prefer]` and `[avoid]` attributes resolve remaining context-free ambiguities, such as the dangling-else problem, without altering unambiguous parses (CLM-0148; SRC-0023 k-distribution/k-tutorial/1_basic/04_disambiguation/README.md; source fact; not reproduced; high; S6).

### Configuration declarations and cell structure

The configuration declaration establishes the abstract machine state (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Declared via the `configuration` keyword, the configuration specifies a hierarchical, nested collection of cells using XML-like syntax (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
If no configuration is specified, K automatically imports the default configuration:
```text
configuration <k> $PGM:K </k>
```
Configuration variables, designated with a `$` prefix like `$PGM`, act as dynamic inputs supplied via `krun` command-line flags or input files (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
The sort attached to `$PGM` acts as the parser start symbol for program inputs (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Cells can be statically initialized with constant values, such as `<sum> 0 </sum>` or `<state> .Map </state>` (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).

Configuration abstraction allows rules to mention only the cells necessary for a given transition (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
The compiler completes the missing ancestor and sibling cell structures automatically based on the configuration declaration (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; source fact; not reproduced; high; S6).
Cells can be typed as collections using the `multiplicity` attribute: `multiplicity="?"` designates optional cells that can be added or removed using `.Bag`, while `multiplicity="*"` designates repeating cell collections typed as sets or maps (CLM-0149; SRC-0023 k-distribution/k-tutorial/1_basic/17_cell_multiplicity/README.md; source fact; not reproduced; high; S6).

### Rewrite rules and reduction semantics

Rewrite rules specify operational state transitions over configurations and terms (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
A rule is declared with `rule LHS => RHS`, where `LHS` matches the current state and `RHS` constructs the successor state (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Variables are uppercase identifiers that bind matching subterms (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/02_basics/README.md; source fact; not reproduced; high; S6).
An underscore `_` denotes an anonymous placeholder variable that discards its bound value (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
Local rewrites can be placed inside nested subterms or cells, such as `rule <k> (E1 + E2 => E1 +Int E2) ...</k>`, leaving the surrounding context unchanged (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Cell ellipses `...` represent anonymous frame variables matching uninspected prefixes or suffixes of sequences, sets, or maps (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).

Rules support rich control mechanisms:
- Side conditions: `requires <Bool-expression>` specifies preconditions that must evaluate to true for the rule to apply (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
- Postconditions: `ensures <Bool-expression>` specifies constraints over fresh variables introduced on the right-hand side (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
- Rule priority: rules are tried in increasing numerical order of priority; default rules have priority 50, rules with `[owise]` have priority 200, and explicit priorities are set via `[priority(N)]` (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/07_side_conditions/README.md; source fact; not reproduced; high; S6).
- Rule labels: rules can be named using `rule [label-name]: ...` to enable targeted breakpoints in debuggers and simplification rules (CLM-0150; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).

### Evaluation strategies: strictness, contexts, and heating/cooling

The `<k>` cell stores a computation sequence composed of terms of sort `K` separated by the associative sequencing operator `~>` (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Every sort is an implicit subsort of `KItem`, allowing arbitrary language terms to be injected into a K sequence (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/13_rewrite_rules/README.md; source fact; not reproduced; high; S6).
Evaluation of complex sub-expressions within the `<k>` cell is structured through heating and cooling (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Heating suspends an unevaluated expression, moves an unevaluated argument to the front of the K sequence, and places a freezer item on the sequence to preserve context (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Cooling restores the evaluated argument from the front of the K sequence back into the suspended freezer once it satisfies the `isKResult` predicate (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
Instead of requiring manual authoring of freezers and heating/cooling rules, K provides high-level annotations:
- `context <k> HOLE + E ...</k>` explicitly declares an evaluation context with a `HOLE` variable (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
- `[seqstrict]` automatically generates sequential left-to-right heating and cooling rules for specified non-terminal positions (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
- `[strict]` generates non-deterministic heating and cooling rules, allowing arguments to evaluate in arbitrary order (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).
- `context alias [name]: <k> HERE ...</k>` creates reusable context templates for custom evaluation strategies (CLM-0151; SRC-0023 k-distribution/k-tutorial/1_basic/14_evaluation_order/README.md; source fact; not reproduced; high; S6).

## Execution backends and selection criteria

The K Framework architecture separates the compilation frontend from backend execution engines, supporting two active backends and one deprecated legacy backend (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

```text
+-------------------+--------------------+------------------------+
| Backend           | Implementation     | Primary Use Case       |
+-------------------+--------------------+------------------------+
| LLVM Backend      | C++ / LLVM         | Concrete execution,    |
| (default)         | Native code gen    | conformance tests,     |
|                   | Bison parsers      | GDB/LLDB debugging     |
+-------------------+--------------------+------------------------+
| Haskell Backend   | Haskell            | Reachability logic,    |
|                   | Term interpreter   | symbolic execution,    |
|                   | Z3 integration     | deductive proofs       |
+-------------------+--------------------+------------------------+
| Java Backend      | Java (legacy)      | Historical precursor,  |
|                   |                    | completely deprecated  |
+-------------------+--------------------+------------------------+
```

### The LLVM backend

The LLVM Backend is the default backend of K (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
It is optimized for high-performance concrete execution and exhaustive state-space search (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The backend translates KORE AST definitions into LLVM IR, which is compiled and linked into an optimized native machine code binary interpreter named `interpreter` inside `<name>-kompiled/` (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
Concrete execution enforces that terms in configurations must not contain logical variables or uninterpreted functions (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
The LLVM backend incorporates ahead-of-time parser generation using GNU Bison, producing native C parsers (`parser_PGM`) that eliminate parsing bottlenecks during large-scale program execution (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; source fact; not reproduced; high; S6).

The LLVM backend provides first-class support for source-level debugging via GDB on Linux and LLDB on macOS (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Compiling with `--enable-llvm-debug` and running with `krun --debugger` launches the interpreter within the debugger with Python inspection scripts loaded (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
Developers can set breakpoints on rule labels (`break <MODULE>.<label>.rhs`), break on rule side conditions (`break <MODULE>.<label>.sc`), break on function productions (`break Lbl<function>`), step through individual rewrite transitions (`k step`), and diagnose pattern-matching failures using `k match <MODULE>.<label> subject` (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; source fact; not reproduced; high; S6).
The LLVM backend is the recommended engine for running large test suites, conformance validation suites, and high-speed interpreter applications (CLM-0152; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

### The Haskell backend

The Haskell Backend is designed for formal deductive verification and symbolic reasoning (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
Unlike the LLVM backend, it does not compile definitions to native binaries; instead, it is an interpreter implemented in Haskell that executes KORE ASTs directly (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The Haskell backend supports symbolic configurations containing logical variables and uninterpreted functions (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Rewriting operates via term unification, computing the most general unifier (MGU) between symbolic states and rule patterns (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
Fresh existential variables can be introduced on the right-hand side of rules using the `?X` syntax, constrained by `ensures` clauses (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).

Path condition feasibility is checked by delegating constraint formulas to the Z3 SMT solver, automatically pruning mathematically unsatisfiable symbolic branches (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; source fact; not reproduced; high; S6).
The Haskell backend is the execution engine behind `kprove`, proving temporal and functional claims expressed in reachability logic (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; source fact; not reproduced; high; S6).
It also powers the interactive proof shell `kore-repl` (CLM-0153; SRC-0023 docs/ktools.md; repository observation; not reproduced; high; S6).
While the Haskell backend can execute concrete programs, its performance is orders of magnitude slower than the LLVM backend (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
On Apple Silicon ARM64 machines, a known issue with the `Compact` library requires passing `--no-haskell-binary` to both `kompile` and `krun` when using the Haskell backend (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

### The legacy Java backend

The Java Backend was the original execution and verification engine of earlier K releases (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
It has been superseded by the Haskell backend for formal verification and by the LLVM backend for concrete execution (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).
The tutorial explicitly notes that the Java Backend is deprecated legacy software and should not be used for new projects (CLM-0153; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; source fact; not reproduced; high; S6).

For in-depth backend selection guidelines and performance trade-offs, consult [K Backends and Tooling](k-backends-and-tools.md).

## The rolling-release development model

The K Framework is maintained, tested, and distributed under a continuous rolling-release engineering model (CLM-0154; SRC-0023 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
The repository tutorial documents this process explicitly:
> "K is developed as a rolling release, with each change to K that passes our CI infrastructure being deployed on GitHub for download. The latest release of K can be downloaded here. This page also contains information on how to install K. It is recommended that you fully uninstall the old version of K prior to installing the new one, as K does not maintain entries in package manager databases, with the exception of Homebrew on MacOS." (CLM-0154; SRC-0023 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).

Every commit merged into the `master` branch that passes the continuous integration pipeline automatically deploys a new release archive and container image (CLM-0154; SRC-0023 README.md, INSTALL.md; source fact; not reproduced; high; S6).
Releases carry incremental patch tags such as `v7.1.337` (CLM-0154; SRC-0023 README.md; repository observation; not reproduced; high; S6).
Because Linux package managers do not maintain internal database entries for rolling `.deb` builds, users upgrading binary packages must purge previous packages before installing newer versions (CLM-0154; SRC-0023 k-distribution/k-tutorial/1_basic/01_installing/README.md; source fact; not reproduced; high; S6).
In Nix-based workflows using `kup` or Nix flakes, deterministic reproducibility is achieved by locking the precise Git commit hash of the `runtimeverification/k` repository in `flake.lock` (CLM-0154; SRC-0023 README.md; repository observation; not reproduced; high; S6).

## What this means for a ZKIR-in-K project

The architectural properties and toolchain capabilities of the K Framework have direct structural consequences for designing and implementing an executable formal semantics of ZKIR (Midnight zero-knowledge intermediate representation) (CLM-0155; SRC-0023 README.md; inference; not reproduced; high; S2).
The operational requirements of ZKIR align with specific features of K's logical foundation and execution engines (CLM-0155; SRC-0023 README.md; inference; not reproduced; high; S2).

First, ZKIR is an intermediate representation characterized by static single assignment (SSA) registers, field arithmetic operations over elliptic curve scalar fields (such as BN254 or BLS12-381), cryptographic commitments, Merkle tree membership checks, and arithmetic constraint gates (CLM-0155; SRC-0023 README.md; inference; not reproduced; high; S2).
A formal semantics in K requires a structured configuration that cleanly separates instructions under execution from register states, memory tables, public ledger contexts, and synthesized arithmetic constraint bags (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; inference; not reproduced; high; S2).
K's cell hierarchy allows the project to declare a top cell `<zkir>` encapsulating an instruction stream `<k>`, an SSA register file `<registers> .Map </registers>`, a witness environment `<witness> .Map </witness>`, and a constraint accumulation cell `<constraints> .Set </constraints>` (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/16_collections/README.md; inference; not reproduced; high; S2).
Configuration abstraction ensures that simple register-to-register arithmetic rules, such as field addition or boolean tests, need only mention the `<k>` cell and `<registers>` cell, leaving cryptographic or constraint cells unaffected (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/15_configurations/README.md; inference; not reproduced; high; S2).

Second, the dual-backend design of K provides a direct path for differential testing and compiler validation (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/20_backends/README.md; inference; not reproduced; high; S2).
By compiling the ZKIR semantics with `--backend llvm` and `--gen-bison-parser`, the Moriarty project obtains a high-performance native interpreter capable of executing millions of concrete ZKIR instruction steps against Midnight conformance test suites (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/03_parsing/README.md; inference; not reproduced; high; S2).
When an execution anomaly or divergence arises during testing, developers can invoke `krun --debugger` to step through native instructions in GDB/LLDB, setting breakpoints on specific instruction rule labels and inspecting intermediate register substitutions (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/19_debugging/README.md; inference; not reproduced; high; S2).

Third, the Haskell backend enables formal symbolic verification of ZKIR program equivalence and translation correctness (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; inference; not reproduced; high; S2).
Compiler transformations from higher-level Compact contract representations into ZKIR can be verified by proving reachability logic claims with `kprove` (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/22_proofs/README.md; inference; not reproduced; high; S2).
Symbolic variables can represent unconstrained private witness inputs, while `ensures` clauses state that generated R1CS or PLONK constraints are satisfiable if and only if high-level contract invariants hold (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/21_symbolic_execution/README.md; inference; not reproduced; high; S2).
Whenever a proof step fails to converge, proof engineers can launch `kore-repl` to interactively inspect unproven path constraints, identify missing field arithmetic simplifications, and refine lemma libraries (CLM-0155; SRC-0023 docs/ktools.md; inference; not reproduced; high; S2).

Fourth, the literate programming support in K allows the ZKIR semantics to be authored directly in CommonMark Markdown files (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; inference; not reproduced; high; S2).
The mathematical specification, normative prose, opcode definitions, typing rules, and executable K code blocks can reside in identical documents (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; inference; not reproduced; high; S2).
Using `--md-selector`, the build pipeline can extract production semantics while ignoring exploratory sketches or testing fixtures (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/08_literate_programming/README.md; inference; not reproduced; high; S2).

Finally, because K follows a continuous rolling-release model, engineering a formal semantics for ZKIR requires strict dependency pinning (CLM-0155; SRC-0023 k-distribution/k-tutorial/1_basic/01_installing/README.md; inference; not reproduced; high; S2).
Formal proofs are sensitive to minor changes in Z3 versions and KORE simplification rules (CLM-0155; SRC-0023 README.md; inference; not reproduced; high; S2).
The project repository must lock the exact Git commit of `runtimeverification/k`, pin Z3 to version 4.12.1, and automate builds through Nix flakes or pinned Docker containers (CLM-0155; SRC-0023 README.md; inference; not reproduced; high; S2).

For details on the target intermediate representation, consult the [ZKIR Instruction Set Architecture](../zkir/zkir-instruction-set.md).
