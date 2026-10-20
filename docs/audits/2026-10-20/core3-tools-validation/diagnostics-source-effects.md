# Diagnostics library source/effects inspection

This is source-only preparation for compiling and separately running the diagnostics library's 18 tests on J. The complete local definitions, helpers and nine existing snapshot bodies are retained. No Cargo, compiler, test, provider, SDK, CLI, VM, proof, wallet or ledger operation was invoked by this preparer. No source was changed. Proposed commands remain unselected data; root owns any next finite allocation. Previous failed/max-one checks remain consumed.

## Subject and observed evidence

J is `/home/charl/Moriarty/.worktrees/doctorwatson-utxo-core3-tools-integration-20261020`, baseline evidence HEAD `a9588a67eb3a415c9af003f4006946f751e02b0a` with the uncommitted exact 52-body integration overlay. Applicable develop/current AGENTS/product/status were read at startup. J local relevant bodies are pinned in INPUT-INVENTORY.json; transported bodies were compared byte-exact to frozen `core3-mcp-lsp-diagnostic-full-integration-source-preparation-20261020/after`. This is a source identity claim, not source approval or inherited runtime evidence.

The earlier Core-library attempt failed offline resolution before any compiler artifact because lsp-server 0.10.0 was absent. Root separately repaired only the vendor view with the exact three frozen LSP package versions. Repaired Core `--lib` typechecking passed in 14.871535565005615s (68 compiler-artifact events); the two Core lexical harnesses compiled in 33.04590623700642s (102 events); root separately observed all six lexical tests pass. Those observations do not observe diagnostics' 18 unit functions or this renderer's ICU compilation. Earlier I/N test results do not transfer to changed J source.

Root's complete combined Core+diagnostics Cargo tree actual exited zero, stderr empty, no Midnight package text. Its 119 distinct package/version identities cover `-e normal,build,dev` for both roots, not 119 diagnostics-only packages and not a Cargo unit/feature/build execution graph. Prior successful artifact streams contain 59 Core-library and 86 Core-lexical distinct package/version identities. Counts deduplicate local path packages by real package name. Event counts and package counts differ.

## Required dependency and feature delta

The diagnostic manifest's dev-dependency explicitly selects `miette/fancy-no-backtrace`. `src/lib.rs:11-12` makes `render` available under `cfg(test)`, so `-p mori-diagnostics --lib` needs no added `--features fancy`. It compiles the real graphical renderer, not a weakened renderer/no-syscall feature. The normal dependency `mori-span` reaches only its real span definitions and miette. This root does not select Core, lexer, backend, CLI, LSP, MCP, SDK or conformance-crate targets.

The full manifest chain is `miette 7.6.0 fancy-no-backtrace -> fancy-base(owo-colors,textwrap) + terminal_size/supports-hyperlinks/supports-color/supports-unicode`; textwrap 0.16.4 enables `unicode-linebreak,unicode-width`, and unicode-linebreak enables `icu_segmenter 2.3.0 compiled_data`. Segmenter reaches ICU collections/provider/locale fallback and baked locale/segmentation data plus UTF/zero-copy/derive support. miette's separate backtrace, syntect and derive defaults are not selected by these manifests/tree; textwrap's complex segmentation/auto/LSTM path is not selected. This is existing frozen dependency behavior, not an introduced feature or a data-generation job.

There are **33 package/version additions** versus the observed Core lexical compilation artifact identities:

displaydoc 0.2.7, icu_collections 2.3.0, icu_locale_core 2.3.0, icu_locale_fallback 2.3.0, icu_locale_fallback_data 2.3.0, icu_provider 2.3.1, icu_segmenter 2.3.0, icu_segmenter_data 2.3.0, indoc 2.0.7, is_ci 1.2.0, litemap 0.8.3, owo-colors 4.4.0, potential_utf 0.1.6, rustversion 1.0.23, smallvec 1.16.2, stable_deref_trait 1.2.1, supports-color 3.0.2, supports-hyperlinks 3.2.0, supports-unicode 3.0.0, synstructure 0.14.0, terminal_size 0.4.4, textwrap 0.16.4, tinystr 0.8.4, unicode-width 0.2.2, utf8_iter 1.0.4, writeable 0.6.4, yoke 0.8.3, yoke-derive 0.8.4, zerofrom 0.1.8, zerofrom-derive 0.1.8, zerotrie 0.2.5, zerovec 0.11.8, zerovec-derive 0.11.6.

The complete list, combined tree feature strings and exact installed manifest/checksum/build-source pins are in DEPENDENCY-DELTA.json and INPUT-INVENTORY.json. Core cryptographic/parser/proptest dependencies appearing elsewhere in the combined tree are not silently assigned to this diagnostics-only target. A standalone diagnostics Cargo unit graph was not executed by this preparer; exact new unit count/cache hits/timing remain unobserved. Existing package identity does not prove feature-unit reuse. In particular rustix 1.1.5 already existed for insta/tempfile `fs,std`, but terminal_size adds `termios`.

## Compiler host effects

| Source-selected unit | Existing concrete effects | Difference from prior Core lexical compilation |
| --- | --- | --- |
| icu_segmenter_data 2.3.0 and icu_locale_fallback_data 2.3.0 `build.rs:1-11` / `src/lib.rs` | Read `ICU4X_DATA_DIR`; emit cfg/rerun/check-cfg lines. Each lib includes custom `/mod.rs` only if that variable is present, otherwise the shipped `../data/mod.rs`. | Two new build scripts. The inherited closed environment omits ICU4X_DATA_DIR, hence bundled data. Generated static data is compiler input/TCB; this report is not a semantic audit of every table. No download/generator is called by these scripts. |
| rustversion 1.0.23 `build/build.rs:18-119`, `build/rustc.rs` | Runs the RUSTC command with `--version` (RUSTVERSION=1), captures/parses version output, writes OUT_DIR/version.expr; optional wrapper recognition can repeat the version query for recognized wrapper output. HOST controls a Windows cfg. | New script and procedural macro, from indoc. Closed env pins RUSTC and omits RUSTC_WRAPPER. This is a prospective compiler subprocess/probe, not a probe performed here. |
| rustix 1.1.5 `build.rs:8-195,228-281` | Reads target/cfg/feature env and architecture source existence; runs rustc metadata-only feature probes with source on stdin and output in OUT_DIR/rustix_test_can_compile, waits and removes that output. No generated probe is executed. | Existing script with termios feature-unit change; do not assume cache reuse. Current std x86_64 Linux selects the ordinary path; total actual probe/cache work must be observed, not inferred from prior artifacts. |
| displaydoc 0.2.7, indoc 2.0.7, rustversion 1.0.23, yoke-derive 0.8.4, zerofrom-derive 0.1.8, zerovec-derive 0.11.6 | Host procedural macros execute within rustc; all have real pinned source/manifests. Proc-macro2/quote/syn/synstructure are their support dependencies. | Six new proc-macro packages versus Core lexical artifact identities. Their entire semantic/security adequacy is not claimed from manifest inspection. |
| insta 1.49.0, console 0.16.6, similar 2.7.0 | All declare build=false. Existing insta defaults select colors/console; console's selected features are alloc/std, similar's default/inline/text. | Already in Core lexical compilation. No new insta/console/similar build script, but newly compiling another harness still has ordinary compiler/linker/output effects. |

INPUT-INVENTORY also records all 17 build-script identities occurring in the combined tree. Many belong to the Core root, not diagnostics. It is not a claim those 17 scripts all execute on the next command. Existing shared compiler/proc-macro/linker/toolchain effects and inherited tool pins remain part of the ordinary compiler TCB, not newly cleared SDK/runtime effects. The three LSP packages in the owned vendor union are needed for frozen offline workspace resolution; they are not a selected diagnostics compilation route.

Compilation writes only the selected existing owned target/tmp/tooling locations under the inherited closed environment and may read the pinned Cargo configuration, lock, manifests and vendor sources. No source/lock update, dependency upgrade, build graph substitution, network fallback or SDK linking is proposed. Exact Cargo argvs and environment-reference are in PROSPECTIVE-COMMANDS.json. Root may select a fresh 90TERM+5/max-one ordinary compilation action; this report does not reset or reserve one.

## Runtime effects and test closure

The complete repository-local Rust closure is the five diagnostics modules (`lib`, `diagnostic`, `source`, `render`, `suggest`) and `mori-span/src/lib.rs`; nine full external snapshot files are existing expectations. The 18 functions inspect diagnostic labels, byte/scalar/CRLF/astral positions, named-source delegation/errors, colored/plain report formatting and edit-distance suggestions. They do not execute language programs or financial transitions. Source names are listed below and in TEST-SCOPE.json; actual harness enumeration/result/hash remains future evidence.

miette graphical rendering uses explicit Unicode themes, fixed width and links disabled (`render.rs:12-49`). Report creation may initialize its handler and query environment/terminal state. Public supports-* libraries read color/link/locale/CI variables and IsTerminal; terminal_size's Linux route checks standard file descriptors with isatty/tcgetwinsize. Its `stty` subprocess appears only in its own `#[test]` function, which is not selected by `-p mori-diagnostics --lib`. Vendor tests/doctests are not proposed. These are ordinary terminal/render effects, not SDK calls.

insta's `src/env.rs:452-513` is a concrete subprocess boundary: absent INSTA_WORKSPACE_ROOT it invokes `cargo metadata --format-version=1 --no-deps`. The existing root lexical run already used runtime INSTA_WORKSPACE_ROOT=J, INSTA_UPDATE=no, INSTA_FORCE_PASS=0, INSTA_REQUIRE_FULL_MATCH=1 and CI=true. Reusing those source-supported controls bypasses that metadata subprocess and prevents snapshot acceptance/forced-pass. No generic `cargo-insta`/nextest probe is reached: its fallback code is behind the unselected `_cargo_insta_internal` feature.

Insta still reads snapshot bodies and tries `.config/insta.yaml`, `insta.yaml`, `.insta.yaml` beneath J; all three are absent at inspection and explicitly recorded. Optional reference/warning files and deprecated force-update env must not be inherited. INSTA_UPDATE=no disables writing new expected/pending snapshots (`env.rs:420-438`, `runtime.rs:565-641`), **but matching assertions still attempt cleanup** of `.snap.new` and `.snap.new.*` (`runtime.rs:502-560,879`). Therefore “no snapshot updates” alone is not “no filesystem cleanup.” Source-supported INSTA_PENDING_DIR can relocate those cleanup attempts into a fresh root-owned path while expected snapshots continue to be read from their original paths (`env.rs:637-675`). This is optional concrete effect containment, not altered expectations or a fabricated pass. No such directory/action was created or run here.

The next meaningful actual predicate is compile exactly the diagnostics library test harness, bind its emitted path/hash and source/expectation/owned-output preimages, then separately run the direct ELF single-threaded under closed snapshot controls and observe exactly 18 passed, zero failed/ignored/measured/filtered, with EOF/reaping/owned-group/source/expectation checks. Fresh binary identity cannot be invented before compilation. No CLI execution, financial evaluator, source compiler, SDK/VM/native proof, crypto/security adequacy or whole 355-test acceptance follows. Prior six lexical PASS remains a different actual scope. All language/financial/composition/Cardano/Midnight goals and owner-blocked B1 obligations remain open.

## Exact selected names

- `diagnostic::first_label_offset_tests::no_label_is_distinct_from_a_zero_length_label_at_zero`
- `diagnostic::first_label_offset_tests::accessor_preserves_insertion_order_and_does_not_consume_labels`
- `render::tests::error_with_code_label_and_help`
- `render::tests::warning_without_labels`
- `render::scalar_column_tests::ascii_header_preserves_the_byte_position`
- `render::scalar_column_tests::multibyte_prefix_header_counts_scalars`
- `render::scalar_column_tests::multibyte_eof_header_counts_scalars`
- `render::scalar_column_tests::crlf_context_preserves_lines_and_byte_positions`
- `render::scalar_column_tests::astral_prefix_header_is_not_utf16_or_display_width`
- `render::scalar_column_tests::unspanned_diagnostic_does_not_invent_a_location`
- `source::tests::normal_points_preserve_bytes_and_count_scalars`
- `source::tests::crlf_halves_keep_the_delegated_line_in_every_context`
- `source::tests::surrounding_context_uses_the_returned_span_start`
- `source::tests::non_boundary_returned_start_keeps_delegated_column`
- `source::tests::delegated_out_of_bounds_errors_are_unchanged`
- `source::tests::generic_named_source_and_report_api_remain_usable`
- `source::tests::colored_and_plain_attachments_share_scalar_headers`
- `suggest::tests::suggests_within_two_edits`

## Limits and custody

This report separates root-provided actual evidence from our read-only source/TOML/JSON/hash inspection and proposed future commands. No actual diagnostics compile/run is claimed. Public vendor source manifests and selected effect definitions are pinned; full transitive vendor implementation reading/security review is not claimed. No private captures/auth/operator HOME were read. Only this new external report directory was written; J/I/W and prior freezes are unchanged. This is ordinary source/effect diagnosis, not a security judgment or runtime admission.
