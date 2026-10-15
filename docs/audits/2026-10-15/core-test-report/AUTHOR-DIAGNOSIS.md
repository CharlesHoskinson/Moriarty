# Aggregate Core test report diagnosis

Status: source-inspected diagnosis and proposed design only. Author: GPT-6.1 Sol, high effort. Checkout: `fix/core-test-report-20261015`; inspected HEAD: `9dffbb0a3f6996dbdae5030ed9bb0a5193d39d73`. No product command, Cargo build/test, native operation, source edit, staging or commit has been performed. The required plugin `status --json` was run; its historical SP01.6 blockers were preserved and do not select this task.

## Requested capability and falsifiable predicate

Repair the public Rust Core `mori test DIR` aggregate report under U7/M10/MC08. For an admitted manifest whose individual case reports fit the existing bounded exporter, an aggregate larger than 1 MiB must produce one complete JSON object with every full actual result in manifest order. Its `passed` flags, overall `status`, stderr counts and process exit must agree with the existing expected-poststate/effects/error-code comparisons. A deliberately wrong financial expectation must produce a complete failed report and exit 1. Shared reader/export limits, other command serialization and financial execution remain outside the source write set. This is a tooling child capability, not closure of U7/M10/MC08 or any other parent acceptance exit.

## Cause and complete caller path

Repository observation: `main.rs` parses `Cli`, initializes `Shell`, dispatches `commands::run`, and returns the subcommand exit code. An operation error is rendered on stderr and returns failure. `cli.rs` maps public `test DIR` to `Command::Test(test::Args)`. `commands/mod.rs` dispatches that variant directly to `test::run`.

`test.rs:38` reads `DIR/mori-tests.json` through `core_io::read_json`. `read_json` first uses `read_source` (read at most 1,048,577 bytes; reject more than 1,048,576), then the existing `mori_core::json::parse` and typed serde decoding. Manifest admission checks the exact `moriarty-tests/1` profile and 1..=256 cases. Each case must have a unique nonempty name. The source and scenario are independently read through those existing bounded functions.

`test.rs:58` calls `mori_core::parse` and `mori_core::simulate`. `mori-core/lib.rs` reexports `parser::parse` and `runtime::simulate`. The parser uses `mori_lexer`; `simulate` calls `simulate_with_meter(..., None)`, validates the program, constructs the scenario snapshot and interprets the finite statements on clones. It returns `Execution` containing semantics, action, pre, post, all steps and ordered effects, or a structured `Error`. The whole `Result` is retained as `actual` in each report. Expected accepted cases compare the full post and ordered effects; expected rejected cases compare the error code. Cases execute in manifest order.

After every case is evaluated and counted, `test.rs:72` prints the `Tested` stderr status. Lines 76–78 then call shared `core_io::print` on the **whole** `{status,cases}` value. `core_io::print` calls `mori_core::json::export`, then `println!`. `export` pretty-serializes the aggregate, calls bounded `parse`, performs its established ASCII escaping, reserves the final newline within its 1 MiB bound, and calls bounded `parse` again. The parser also retains its depth 32 / 65,536-value-node bounds. Thus independently successful cases can collectively fail during final report export, after stderr already says all passed. No stdout is printed on this serialization failure, and `main` returns 1 through its operation-error branch. The normal success/failure exit selection in `test.rs:79` is never reached.

The existing `examples/core/fee-pool/cases/COVERAGE.md` explicitly records the original 123-case successful evaluation followed by aggregate serialization failure. That is historical captured evidence, not a reproduction in this checkout. The current root manifest and one-case accepted manifests work around that recorded aggregate limitation. Complete `core_io.rs`, `test.rs`, CLI startup/dispatch and the FeePool integration test were read; no synthetic compiler path was substituted.

## Proposed narrow design, pending Opus constraints and root GO

Preferred approach: keep report composition private to `commands/test.rs`. Retain current evaluation/comparison logic and complete case values. Preflight every complete case object through unchanged `mori_core::json::export`; retain the resulting strings before emitting stdout. Compose the report object using trusted fixed JSON punctuation and the already-exported case objects in input order, with a trusted `passed` or `failed` status. Write through a locked stdout writer and propagate ordinary output errors through the existing operation-error path. Preserve one complete JSON document and final newline. The exact buffering, byte accounting and output formatting constraints are for Opus to prescribe; this document does not design a replacement escaping algorithm or make a security judgment.

Alternative 1: put a test-report-only composition helper in `core_io.rs`. It could be correct, but expands a shared helper file without a second consumer; the private `test.rs` helper has a smaller integration surface. Alternative 2: a custom general serializer or a change to shared `mori_core::json::export` would affect other commands and reader/artifact/signing boundaries and is outside the requested fix. Splitting manifests, dropping steps, truncating traces or constructing an application-specific compiler does not satisfy the aggregate predicate.

Proposed minimal source write set:

- `crates/mori/src/commands/test.rs`: report-specific composition and its actual call site.
- `crates/mori/tests/core_test_report.rs`: focused public executable integration regression.
- No `core_io.rs` change is currently needed. No Cargo manifests, `mori-core/src/json.rs`, signing, native pipeline or other command serializer changes are proposed.

The failure mode of a **single** case that exceeds the existing exporter remains a question for Opus/root; this proposal does not silently increase that bound or promise support beyond individually exportable cases.

## Functional regression design and independent expectations

All proposed tests are nonsecurity functional tests. Opus owns security tests, escaping/bounds requirements and security judgments. The root-supplied independent oracle must be frozen before implementing/executing the tests.

1. Invoke the actual `env!("CARGO_BIN_EXE_mori")` executable with `--color never test TEMP_PROJECT`. Use a compact admitted aggregate manifest with 87 accepted cases from the existing independently frozen FeePool financial fixture, preserving names/order and pointing source/scenario paths to their existing real files. This is one aggregate product invocation. Compare every returned case name, flag, actual post and ordered effects to the independent expectations, inspect the presence of full pre/post/steps/effects, and assert output exceeds 1 MiB and ends with a newline. The frozen fixture supplies independent final money; merely comparing output to another invocation would supply internal agreement only.
2. Freeze a small ordinary financial source/scenario and literal full `Execution` expectation independently, including every step's statement, value, snapshot and effects, plus preserved balances and a residual duty. Repeat it with unique names to exercise exactly 256 ordered cases while keeping the manifest itself below the existing input and node bounds. The selected trace length must make the aggregate exceed 1 MiB. Compare full actual objects to that independent literal oracle; do not import `simulate` to produce expected values. The parent oracle will determine this fixture before source authoring.
3. Change one expected poststate amount (or ordered effect) in that lawful aggregate while leaving execution inputs unchanged. Assert exit 1, overall `failed`, exactly one `passed:false`, unchanged complete actual results, all cases still ordered, and accurate stderr counts. This is an intentionally incorrect test expectation, not a hostile input case.
4. Use the small single-case fixture to retain ordinary small-report shape, content, count and exit behavior. Keep whitespace assertions limited to valid complete JSON and final newline unless compatibility requirements explicitly require old pretty bytes.

Ordinary Python inspection of the existing source fixture found `cases/expected.json` is 1,069,885 bytes as checked in, so copying its pretty bytes into the public manifest would fail the unchanged input cap before evaluation. Its compact 162-case representation is 474,817 bytes and 31,155 value nodes; the accepted-only 87-case representation is 460,517 bytes and 30,555 value nodes before temporary path rewriting. The test must measure its final generated manifest and retain unchanged reader admission. No product evaluator was run to obtain these figures. Existing per-case accepted manifests are useful historical/local baseline consumers, not a substitute for the aggregate regression.

## Build graph, startup and candidate ordinary verification commands

`mori` is the actual workspace package and binary. It depends directly on `mori-core`, `mori-backend` and `mori-ledger`. Backend depends on the pinned Midnight ZKIR and crypto crates; ledger depends on the pinned Midnight ledger/runtime/VM and proof crates. `mori` also declares Midnight dev dependencies. A focused CLI integration target therefore still builds/links that normal dependency graph; it must not be replaced by a split manifest or synthetic binary. No repository `build.rs` was found by tracked-source listing. Third-party build scripts were not inspected or executed. Product startup is ordinary clap parsing plus `Shell` construction; `Command::Test` reaches only the inspected parser/reference evaluator path. These are source observations, not execution-boundary approval.

Proposals only, subject to Opus command-boundary advice and root GO:

```bash
rustfmt +1.99.0 --edition 2024 --check crates/mori/src/commands/test.rs crates/mori/tests/core_test_report.rs
cargo +1.99.0 --offline --locked test -p mori --test core_test_report -j2 -- --test-threads=1
cargo +1.99.0 --offline --locked test -p mori --test init -j2 both_starters_check_simulate_and_test_with_complete_literal_expectations -- --exact --test-threads=1
git diff --check
```

The new regression should first be observed failing against unchanged source, then passing with the minimal change, within approved commands. No broad `cargo test`, native test target, SDK relation/check/degree/model operation, VM, key/proof/deploy/ledger invocation or private material/cache access is proposed. Existing full `fee_pool_component` includes static compile and negative/security-oriented cases; it is not proposed as an unfiltered author-run check.

## Questions routed to Opus/root

- Confirm the permitted report-specific composition/buffering and output-error contract, including preflight-before-stdout behavior and whether byte-identical old pretty formatting is required.
- Specify the retained per-case export boundary and any report-specific aggregate accounting requirements; the author will not redesign bounds.
- Supply escaping and security tests/requirements, and judge the composed ASCII output using the existing exporter.
- Confirm the exact permitted Cargo/rustfmt commands and actual executable path after considering the normal dependency graph; native authority remains zero.
- Supply/approve the independent lawful 256-case financial oracle and functional test scope before author implementation.

Implementation, runtime evidence and independent audits are pending. The author awaits root GO.
