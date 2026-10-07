# Rust toolchain guidelines

Rules for the Rust implementation of the Moriarty toolchain in `crates/`.

## Target language

Port `moriarty-beta/1`, the `.mori` language. The reference implementation is
[`packages/moriarty-beta/src/frontend.ts`](packages/moriarty-beta/src/frontend.ts);
the [reference page](https://charleshoskinson.github.io/Moriarty/reference.html#language)
documents it. Beta lowers to Source/6 and Core/5
([contract](experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md)).

The surface syntax is not frozen. Keep grammar-specific code separate from the
later phases so it can change. Parity with the TypeScript frontend is measured
on accept/reject and `BETA_*` diagnostic codes, not message text or spans,
except for the intentional differences below.

### Intentional differences

The TypeScript frontend is the reference for what the language accepts and
what values mean, not for incidental implementation choices. Every deliberate
difference is listed here and agreed before it is made.

- **Comments do not count toward the 8,192-token bound.** The bound limits
  analysis, and comments are not analyzed. The TypeScript lexer counts them, so
  a file whose comments push it past the bound is rejected there and accepted
  here. The 65,536-byte source bound still caps comments.
- **Syntax errors are reported before name and type errors.** The TypeScript
  frontend resolves names while parsing, so in a file with several errors it
  may report a different first one. Files with a single error behave the same,
  and parity checks use such files.

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
  the 8,192-token bound (`BETA_TOKEN_BOUND`); whitespace and comments do not.
- **Crates.** `mori-span` holds spans, `mori-lexer` the tokens and lexer,
  `mori-ast` the struct-of-arrays AST and its tree printer, and `mori-parser`
  the parser that produces it.
- **Separate phases.** Lex → parse → resolve and check → evaluate constants.
  Do not copy the TypeScript parser, which evaluates while parsing.
- **Numbers.** Values fit in `u128`; no bigint library. Decimal quantities are
  scaled by their asset during checking.

## Diagnostics

- Follow [oxc](https://github.com/oxc-project/oxc): one open `MoriDiagnostic`
  struct (message, labels, help, severity, code) in a `mori-diagnostics` crate,
  implementing miette's `Diagnostic` trait. Do not create a struct or enum
  variant per error.
- Each compiler crate has a `diagnostics.rs` of builder functions:

  ```rust
  pub fn quantity_separator(span: Span) -> MoriDiagnostic {
      MoriDiagnostic::error("A quantity needs a space before its asset")
          .with_label(span)
          .with_help("Write it like `10 USD`")
  }
  ```

- Hot paths record compact data (code, token, context, expected tokens). Build
  `MoriDiagnostic`s only when reporting.
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
- Diagnostic codes keep the existing `BETA_*` strings.
- Spans are UTF-8 byte ranges, matching miette's `SourceSpan` and beta. Editors
  and a future language server need UTF-16 line/character positions, converted
  from the same compact records.
- The lexer and parser stop at the first error, like the TypeScript frontend.
  Recovery at declaration boundaries may come later; if it does, parity compares
  only the first error.
- `mori-span` converts `Span` into miette's span types, as `oxc_span` does.
- The `fancy` feature of `mori-diagnostics` enables miette's
  `fancy-no-backtrace`. The `mori` CLI turns it on; library crates turn it on
  only in `[dev-dependencies]`.
- Use `thiserror` for errors that never point at source, such as I/O.

## Testing

- Snapshot tests use [`insta`](https://insta.rs) and `cargo-insta`.
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
- Formatter tests also assert that formatting the output again changes nothing.
- **Never write tests that invoke the CLI binary**, and never snapshot `--help`.
  Test the library crates; keep the CLI a thin layer over them.

## CLI

- [`clap`](https://crates.io/crates/clap) with derive, styled with cargo's colors
  through `clap::builder::styling::Styles`.
- Cargo-style status lines on stderr: a right-aligned, bold, colored verb, then
  details (`    Checking invoice.mori`, `    Finished check in 3.2ms`). Use
  `anstream` and `anstyle`.
- Results go to stdout. `--json` turns status lines off.
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
