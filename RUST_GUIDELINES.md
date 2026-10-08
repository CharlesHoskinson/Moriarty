# Rust toolchain guidelines

Rules for the Rust implementation of the Moriarty toolchain in `crates/`.

## Target language

Port `moriarty-beta/1`, the `.mori` language. The reference implementation is
[`packages/moriarty-beta/src/frontend.ts`](packages/moriarty-beta/src/frontend.ts);
the [reference page](https://charleshoskinson.github.io/Moriarty/reference.html#language)
documents it. Beta lowers to Source/6 and Core/5
([contract](experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md)).

The surface syntax is not frozen. Keep grammar-specific code separate from the
later phases so it can change. The Rust toolchain accepts and rejects the same
programs as the TypeScript frontend, except for the intentional differences
below. Error codes, messages and spans are our own.

### Intentional differences

The TypeScript frontend is the reference for what the language accepts and
what values mean, not for incidental implementation choices. Every deliberate
difference is listed here and agreed before it is made.

- **Comments do not count toward the 8,192-token bound.** The bound limits
  analysis, and comments are not analyzed. The TypeScript lexer counts them, so
  a file whose comments push it past the bound is rejected there and accepted
  here. The 65,536-byte source bound still caps comments.
- **`mori format` formats any file that parses,** including files with name or
  type errors, as rustfmt and prettier do. The TypeScript formatter refuses any
  file with an error. Its layout also differs: ours fits lines to 80 columns.
  And `mori format` (alias `fmt`) writes in place by default, with `--check` instead of the
  TypeScript CLI's `--write`.
- **Syntax errors are reported before name and type errors.** The TypeScript
  frontend resolves names while parsing, so in a file with several errors it
  may report a different first one. Files with a single error behave the same.

## Workspace

- Every crate lives in `crates/*`. The `mori` crate is the CLI entry point.
- The toolchain is pinned in `rust-toolchain.toml` (Rust 1.99.0, edition 2024).
- Declare every dependency version once in the root `[workspace.dependencies]`.
  Crates reference it with `dep = { workspace = true }` or `dep.workspace = true`.
- The pre-existing Rust crates under `experiments/`, `raw/` and `wiki-llm/` are
  not part of this toolchain. They are excluded from the workspace so they still
  build standalone; the beta CLI tells users to build `experiments/midnight-crypto`
  with `--manifest-path`, which fails if the workspace claims it.

## Front end

- **Lexer first, then parser.** A hand-written lexer tokenizes the whole source
  in one pass, then a hand-written recursive-descent parser walks the token
  array, as Zig does.
- **Data-oriented layout.** Tokens, AST nodes and comments are struct-of-arrays
  containers built with [`soa-rs`](https://crates.io/crates/soa-rs)
  (`#[derive(Soars)]`). Nodes refer to tokens and other nodes by index handles,
  never by pointers or boxes. Variable-length node data goes in an `extra`
  `Vec`. Tag enums are `#[repr(u8)]`. Node payloads stay plain `lhs`/`rhs`
  fields interpreted by tag, not Rust enums.
- **Small handles.** Beta caps source at 65,536 bytes and tokens and nodes at
  8,192 each, so token and node handles are `u16` newtypes and byte offsets are
  `u32`. `soa-rs` indexes by `usize`; convert at the handle boundary.
- **Preallocate.** Size the token container with `source.len() / 8`, as Zig does.
- **Tokens store only `tag` and `start`.** The end is recovered by re-lexing.
- **Comments are kept for the formatter** in their own `start`/`end` table, not
  in the token stream. Only tokens, including the end-of-file token, count toward
  the 8,192-token bound; whitespace and comments do not.
- **Crates.** `mori-span` holds spans, `mori-lexer` the tokens and lexer,
  `mori-ast` the struct-of-arrays AST and its tree printer, `mori-parser` the
  parser that produces it, and `mori-checker` the checker that consumes it.
- **Separate phases.** Lex → parse → resolve and check → evaluate constants.
  Do not copy the TypeScript parser, which evaluates while parsing.
- **Numbers.** Values fit in `u128`; no bigint library. Decimal quantities are
  scaled by their asset during checking.
- **Node spans are computed, not stored.** Like Zig's `firstToken` and
  `lastToken`, `Ast` derives a node's first and last token from its tag and
  children when a diagnostic needs it.

## Checker

- **One `mori-checker` crate,** with modules for names, values, declaration
  rules, calls and intents. Checking is one ordered walk: each declaration's
  names are resolved, its value computed and its rules checked before the next,
  because later declarations depend on earlier values. Its codes are
  `mori::check::<kind>`.
- **Output is side tables and a value store,** in the same style as the AST: a
  `Soa` of declarations (kind, name token, value handle) and a `Soa` of values
  with `u16` handles, an `extra` array for records and lists, and amounts in a
  separate `u128` array. No boxed value trees.
- **`check` returns what was checked together with its diagnostics,** so valid
  declarations stay usable when others fail, as a language server needs. A
  program is accepted only when there are no diagnostics.
- **It reports every independent error.** A declaration that fails is marked
  failed, and anything depending on it is skipped without further errors, so
  one mistake never cascades. Which programs are accepted does not change.
- **The built-in call schemas are a static Rust table:** each call's name and
  its required and optional arguments with their expected types.

## Formatter

- **`mori-fmt` is a Wadler-style pretty printer.** It builds a document of
  text, line breaks, groups and indentation from the AST, then fits each group
  on one line within 80 columns or breaks it one item per line, with trailing
  commas when broken.
- **No `Format` trait.** The document is built by a `match` over `NodeTag`,
  like the other phases, into an arena of rows.
- **Output depends only on the program and its comments.** The profile line and
  every agreement item are followed by exactly one blank line, and comments
  above an item stay attached to it. Numbers and strings keep their spelling.
- **Comments come from the comment table** and stay next to the token they
  were written beside: on the same line, or on their own line above.
- **Correctness is proven by tests, not checked at run time.** Snapshot tests
  show each layout case with its `.mori` source, and every one also asserts
  that formatting the output again changes nothing. A property test checks
  that output reparses to the same tokens and comments.
- **`mori format [PATHS]...` (alias `fmt`) rewrites files in place.** A file formats that
  file, a directory every `.mori` file under it, and no arguments the current
  directory. Directories are walked with `ignore` (respecting `.gitignore`)
  and files are formatted in parallel with `rayon`; output is printed in
  sorted path order. A file that does not parse is reported and skipped.
- **`mori format --check [PATHS]...`** writes nothing and exits 1 when a file is
  not formatted, for CI. Each such file gets a `mori::fmt::not_formatted`
  diagnostic header (no snippet, no help), then the diff exactly as
  `cargo fmt --check` prints it (one `Diff in FILE:LINE:` per region, three
  context lines, red `-` and green `+`, via `similar`), indented to line up
  with the message. One summary line at the end says how to fix it.

## Diagnostics

- Follow [oxc](https://github.com/oxc-project/oxc): one open `MoriDiagnostic`
  struct (message, labels, help, severity, code) in a `mori-diagnostics` crate,
  implementing miette's `Diagnostic` trait. Do not create a struct or enum
  variant per error.
- Each compiler crate has a `diagnostics.rs` of builder functions:

  ```rust
  pub fn quantity_separator(span: Span, source: &str) -> MoriDiagnostic {
      MoriDiagnostic::error("A quantity needs a space between the number and its asset.")
          .with_code("mori::syntax::quantity_without_space")
          .with_label(span.label("no space here"))
          .with_help_code("Put a space between them:", corrected_line(source, span, ...))
  }
  ```

- Every phase builds a `MoriDiagnostic` where the error is found, through a
  builder in its `diagnostics.rs`. Errors are rare and never on a hot path.
- Messages must be Elm-quality: say what was being parsed, what was expected,
  show a correct example and give a specific hint. Errors should teach the
  language.
- To support that, the parser keeps a context stack (agreement body,
  declaration, record fields, call arguments, type annotation) and records the
  set of expected token tags at each failure.
- Common mistakes get their own codes and teaching messages, for example:
  `10USD` (missing space before the asset), a decimal without an asset, a
  forward reference, an unknown call (suggest the closest of the known calls),
  a reserved word as a name, `/` (no division), a dotted name that is not a
  call, and adding quantities of different assets.
- Codes follow miette's style, one per kind of error:
  `mori::<phase>::<kind>`, such as `mori::lex::leading_zero` or
  `mori::syntax::missing_comma`.
- No title banners. A help that suggests a fix shows the corrected code on its
  own indented lines, not inside a sentence:

  ```
    help: Fields in a record are separated by commas:

              { domain: Preview, id: "A" }
  ```
- Spans are UTF-8 byte ranges, matching miette's `SourceSpan` and beta. Editors
  and a future language server need UTF-16 line/character positions, converted
  from the same compact records.
- The lexer and parser stop at the first error, like the TypeScript frontend.
  Recovery at declaration boundaries may come later.
- `mori-span` converts `Span` into miette's span types, as `oxc_span` does.
- The `fancy` feature of `mori-diagnostics` enables miette's
  `fancy-no-backtrace`. The `mori` CLI turns it on; library crates turn it on
  only in `[dev-dependencies]`.
- Use `thiserror` for errors that never point at source, such as I/O.

## Testing

- Snapshot tests use [`insta`](https://insta.rs) and `cargo-insta`.
- Test programs shared by several crates live in `crates/fixtures/`, which is
  excluded from the workspace. Nothing under `crates/` reads files outside it;
  copy anything needed in.
- **Any test that consumes `.mori` source must put the source, not the Rust
  expression, in the snapshot.** Use a macro that takes an `indoc!` literal and
  sets `description` and `omit_expression`:

  ```rust
  insta::with_settings!({
      description => format!("Code:\n\n{}", input),
      omit_expression => true,
  }, {
      insta::assert_snapshot!(render(&output));
  });
  ```

- Use separate macros for success and error cases. Error snapshots render the
  diagnostic through miette's `GraphicalReportHandler` with
  `GraphicalTheme::unicode_nocolor()` and a fixed width, so the snapshot is
  exactly what a user sees.
- Snapshot the AST through a tree printer, not `Debug` of the struct-of-arrays.
- Every phase that consumes source has a `proptest` property that it never
  panics, fed random strings, sequences of tricky fragments and, from the
  parser on, real programs with random damage.
- Formatter tests also assert that formatting the output again changes nothing.
- **Never write tests that invoke the CLI binary**, and never snapshot `--help`.
  Test the library crates; keep the CLI a thin layer over them.

## CLI

- [`clap`](https://crates.io/crates/clap) with derive, styled with cargo's colors
  through `clap::builder::styling::Styles`.
- Cargo-style status lines on stderr: a right-aligned, bold, colored verb, then
  details (`    Checking invoice.mori`, `    Finished check in 3.2ms`). Use
  `anstream` and `anstyle`.
- Results go to stdout. `--json` turns status lines off. JSON reports are our
  own format, not the TypeScript beta's; spans are UTF-8 byte ranges and every
  diagnostic has exactly one primary label.
- Global `-q`/`--quiet`, `-v`/`--verbose` and `--color` (`auto` respects
  `NO_COLOR` and non-terminal output). Verbose adds stage timings and
  token/node counts.
- Final verbs say exactly what happened, for example `Prepared (unqualified)`.
  Never imply proof, signature authority or ledger acceptance that did not occur.

## Later

- Port signature verification from `experiments/midnight-crypto`
  (`src/intent.rs`, `src/codec.rs`) so `verify-intent` runs natively. Its
  Midnight dependencies are Git-pinned; keep them behind a feature.
  `fixtures/intent-vectors.json` can serve as test vectors. A valid signature
  does not prove the signer controls the account; keep reporting possession as
  unchecked.
- The product target is ZKIRv3 on Midnight.
