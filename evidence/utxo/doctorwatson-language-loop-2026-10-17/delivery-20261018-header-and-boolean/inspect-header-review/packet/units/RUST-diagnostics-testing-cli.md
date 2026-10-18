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
- Use `thiserror` for errors that never point at source, such as I/O. No
  `anyhow` in library crates.

## Testing

- Snapshot tests use [`insta`](https://insta.rs) and `cargo-insta`.
- The finite backend `financial_observation_relation` target uses frozen
  complete Program/Execution literals and positive SDK relation checks instead
  of insta snapshots. The exact sources and independent expectations remain in
  `crates/fixtures/financial-report-contract/`; expected values must not come
  from the evaluator, compiler output or generated baselines. This exception
  adds no authority to execute SDK checks or claim native/proof acceptance.
- The finite Core `financial_program_wire_contract` target uses independently
  frozen complete financial Program JSON and the public Program/v1 decoder
  instead of generated snapshots. It selects no Midnight compatibility feature
  and makes no compiler-Artifact, SDK, native-proof or ledger acceptance claim.
- The `guarded_quantity_fee_positive`, `program_plan_roundtrip`, and
  `core_reserve_spend_refund` regressions compare frozen financial or continuation
  values directly instead of generating insta snapshots. Their sources and
  expected values stay in `crates/fixtures/`; they use the ordinary Core library
  and, for the reserve case, its public CLI. Source exposure and the exact
  assertion boundaries are recorded with the test evidence. These tests do not
  establish SDK, native-proof, compiler-Artifact or ledger correspondence.
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
- **Test semantics in the library crates; keep the CLI a thin layer over them.**
  The CLI feature-boundary work has a narrow exception for real binary tests
  of public dispatch, feature availability, frozen complete output, process
  exits and I/O. This exception does not make a shared evaluator an independent
  oracle. Never snapshot `--help`; use clap schema assertions for that contract.
- Existing external fixture paths and test-support mounts are retained during
  the feature-boundary cut; they are not precedents for new paths or mounts.
  `core_continuations` includes the three `examples/core/continuations/*.mori`
  sources and `normal-oracle.json`; `stage_test_public` reads
  `models/core-stage-tests`; `fee_pool_component` and one `core_test_report`
  case read `examples/core/fee-pool`. Native targets also retain their existing
  example/deliverable inputs. Their provenance and the rule against fixtures
  outside `crates/` need separate reconciliation before claiming full guideline
  conformance. Preserve originals and evidence identities.
- Existing `#[path = "../../mori-ledger/tests/support/mod.rs"]` mounts in
  `prepare_call`, `installed_call` and `signed_installed_call` remain unchanged
  in the native-only targets. They are existing deviations from the no-remount
  rule above; this cut adds none and does not claim to resolve them.

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

