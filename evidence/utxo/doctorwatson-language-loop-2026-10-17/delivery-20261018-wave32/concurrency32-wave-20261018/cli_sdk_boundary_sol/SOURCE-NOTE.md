Unapplied ordinary CLI contract regression, base 64c6dedff1d99b33c23832330d7911a5fad9ed59.

The callable predicate is checked_transfer_profiles_have_complete_cli_contracts in the existing crates/mori/tests/source6_expand.rs target. For Invoice/pay and MaxRebatePayout/pay_rebate, the real mori binary must return exact complete check --json stdout bytes, empty check stderr, and exit 0; normal explicit Source/6 expansion must return exact raw proposal bytes including final newline, the existing complete local-stipulation-only stderr line, and exit 0; quiet expansion must retain identical stdout, empty stderr, and exit 0. This is six public command invocations, using the existing temp-directory/child-process harness. No library/parser substitute is used in the assertions.

Change and reason

Existing checked_beta_reaches_real_expansion_library_and_raw_stdout checks only three JSON projections before complete invoice expansion. The added regression freezes the complete check report (all declarations, action, byte spans, source hash, diagnostics, field order/format/newline), adds the contrasting max/atoms/subtraction/multiplication input, and pairs each check with full normal/quiet emission. No existing test body or production body changes. Five new local fixture files and one existing integration-test append form the six-path patch. No new dependency, framework, manifest, lock change, external fixture path, #[path] mount or absolute source path is added.

Profile contract

Both checked inputs explicitly declare profile "moriarty-beta/1". The current Beta CheckReport has no profile or language field: it identifies file bytes, agreement, declarations and actions; it reports local_s0 support. Full expected check bytes and the explicit absent-language assertion retain that existing schema. The emitted text declares profile "moriarty-financial-agreement-source/6". Neither accepted Beta checking nor local transfer proposal presentation is a Core1/Core2/compiler/native/ledger result. No new profile field or semantic decision is proposed. Existing Core default expansion coverage remains unchanged.

Real consumer trace

check::run -> check_source::dispatch selects Beta when the first token is not language -> mori_parser::parse -> mori_checker::check -> beta_report -> CheckReport::new/fill -> core_io::print -> mori_core::json::export -> stdout. JSON mode suppresses check status/timing stderr. CheckReport fields serialize in struct order, and source spans include the declaration/action semicolon.

expand::run with --emit-source6-transfer -> feature-enabled run_source6 -> source6::run -> core_io::read_source -> mori_source6::select_transfer (real parser/checker) -> read_scenario -> SelectedTransfer::expand -> emit -> stdout write_all. The library includes the final newline; Shell::status produces the existing local-stipulation-only line unless quiet. This added test covers successful paths only. Existing refusals/hostile tests are preserved byte-for-byte and no new one is authored.

Expectation provenance and limits

independent-input/ preserves the original max-rebate freeze's input bytes, independent semantic Source/6 text, derivation, financial expectations, provenance, UNKNOWN, original manifest and freeze marker. Copied payload files were verified against the original manifest. Its manifest digest is bound in EXPECTATIONS-FROZEN-BEFORE-TEST.json. Original source input SHA256 f99b32789a8c6bd242e8a38cb7d675f99802823043460eb700a56ad79a1e4712 and scenario 3f4686133024bc48a8b9f5499a7811b007682f89e27b91dd620f5e6da11973d8 are unchanged.

Payout 800, fee 25 and gross 825 come from that existing independent freeze, not a fresh arithmetic claim by this author. The new max-rebate.source6 byte fixture removes indentation and the spaces around the validity range separator from the freeze's grammar-valid text, matching the inspected existing raw CLI presentation convention. Financial statement content/order and all values are retained. The freeze explicitly did not certify actual emitted byte formatting, check reports, exit/status or work. Therefore the new Source/6 byte mapping and both complete check-report fixtures are AUTHOR-DERIVED contract expectations, not an independently frozen new wire oracle. They were frozen before the test addition. AUTHOR-DERIVED-SPAN-EXPECTATIONS.json records each exact source substring and half-open byte interval; all current fixture source is ASCII. Reports were assembled from static field definitions and direct source positions; no product parser, checker, evaluator or emitter was run to generate an expected golden.

Previous source exposure: this author has read the CLI/core/checker/report/Source6 public consumers in earlier source tasks and again here. This lane supplies no fresh independent arithmetic or source audit. Existing invoice.source6 and source6.stderr.txt are inherited historical fixtures, not relabelled independent expectations. No observed max-rebate stdout, actual runtime output or provider stream was read for this task. Fresh nonauthor review of complete wire expectations remains necessary before acceptance.

Requirements and retained scope

The adopted ordinary Source/6 contract in docs/BETA-SOURCE6-EXPANSION.md specifies raw stdout with newline, quiet stderr behavior, profile separation, closed local transfer scenario and local-stipulation-only qualification. RUST_GUIDELINES.md permits this narrow real CLI output/exit/I/O regression; source/fixtures remain inside crates and the existing test harness. The consolidated EARS requirements and permissionless product contract remain in force. This finite child proposal closes no parent U/M/MC/SP/G, financial/composition/target, proof, security, installation or release requirement. It supplies no actual lowerer consumption, global state/work oracle or supported-general-language claim.

Root verification instruction (specified-only, not execution admission)

cargo test --frozen -j1 -p mori --no-default-features --features source6-expansion --test source6_expand checked_transfer_profiles_have_complete_cli_contracts -- --exact --test-threads=1

The existing target requires source6-expansion; default features include native, hence the explicit --no-default-features. The intended command invokes this one integration test and six ordinary binary children, reads/writes its existing isolated temporary fixtures, captures stdout/stderr and removes the owned temp directory on Drop. It does not request SDK/native features or any proof, wallet, network, model or ledger command. This is a source declaration trace, not a selected Cargo compilation-unit graph or resource/security approval. Root retains actual command inspection and execution ownership. No old SDK allocation is reused or reset.

Validation here is source-only: complete original files/blob/SHA pins in SOURCE-PINS.json, exact new source bodies under candidate/, unified candidate.patch, frozen expected fixtures, and in-memory patch application/source-byte checks in STATIC-VALIDATION.json. No Cargo, rustfmt, clippy, Rust/product/Node runtime, provider, network or native execution occurred. Runtime compile correctness and the exact expectations remain unobserved.
