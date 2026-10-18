# Static custody check of changed-method full review input

The 109881-byte frozen prompt matches its pins. All 72 member files are unchanged. All 54 fenced blocks have explicit byte offsets and hashes. The four complete Rust sources match both the physical packet copies and exact Git objects at ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75 (tree 3b5d75a115712b155b68398d83ce18cef78a3d87). This is a custody/fact check by the author/transport agent, not independent source approval, result approval or acceptance.

REVIEW-INPUT.md SHA256: 965b6af2239b1714fd4192aa95e2626f7dfda1fc9fb2db28a96178226e03ff63.
MEMBERSHIP.json SHA256: 92c7d32f3f5582b39c885d87748639104578a49aada13119b92fbd5dae7d53e6.

The verifier compared every full changed-source byte and every embedded E4D excerpt byte. All 28 preserved physical context files match both E4D and the final source revision. It parsed and compared all 11 delivered complete JSON objects with their preserved input files. Both complete actual CLI output objects equal their separately preserved expected metadata contracts, including canonical JSON encoding equality. Original physical whitespace hashes remain distinct; compact inline encoding is not claimed to reproduce original stdout whitespace. Captured check-plan, result, cleanup, per-mode test and query receipts also match root's original execution evidence. No command or product test was rerun here.

The exact source test functions are:

- public_query_distinguishes_profiles_build_consumers_and_unchecked_targets
- reported_core_profiles_select_real_source_only_cli_consumers

The no-default-features command selects the complete capabilities test target: two passed, zero filtered. The ordinary-optins command enables source6-expansion,core2-disjoint-draft and selects only the first function using --exact: one passed, one filtered. Both commands use --frozen -j1 -p mori --no-default-features and --test-threads=1; complete argv arrays appear in TESTS-AND-RESULTS.json. Each build/test wrapper has 90 seconds plus 5 seconds cleanup grace. Each direct CLI query has 10 seconds plus 5 seconds kill grace. Captured 53.7933/5.5957-second wrapper durations include compilation; they are not query-latency measurements. Only direct-query stderr is empty. Root's captured source pin/current-tree/cleanup claims remain captured evidence rather than new observations by this custody task.

The instructions require substantive reasoning about all four complete files and both actual modes. Each finding must quote an existing excerpt/value, cite its exact line or JSON pointer, distinguish fact from inference, and give a user-visible consequence and minimal correction. The schema requires citation/reasoning fields, separate source/result dispositions, and at least six concrete checks. It cannot establish that those six cover six distinct scopes or that reasoning is truthful; root must inspect the response itself. Offset anchors for these instructions appear in OFFSETS-AND-HASHES.json.

The existing schema requires reviewToken but leaves it as an unconstrained string. Before a dispatch, root must create a separate derived schema adding /properties/reviewToken/const with the actual unique per-job token, use that same token in the runner task and explicit dispatch instructions, then pin the derived artifacts. EXTERNAL-SCHEMA-TOKEN-INSERTION.json contains an unapplied template. Its placeholder is not a real token. No original schema or prompt was changed. No external review handle, provider call, model identity or approval is invented here.

Current E4D/final Rust LSP and MCP are stubs. Future PR220/adff55 integration requires explicit capability metadata/profile/output/limitation, inspection-basis and test updates with fresh evidence. catalogue.sourceRevision remains curated E4D inspection provenance; binarySourceRevision is null. This check does not make source-currentness automatic and does not alter U/M/MC/SP/G exits.

The complete prompt was decoded and all fenced blocks compared. Omitted context remains omitted from provider-delivered text, even though full physical context copies were hash-compared here. Frozen preflight provenance and unembedded logs were opaque, hash-only membership checks; their contents were not interpreted. No private/provider log was consulted. No build, SDK, native, proof, wallet, runtime or provider command ran; only ordinary file reads/hashes, JSON parsing and read-only exact Git object reads were used.

Artifacts:

- CUSTODY-REPORT.json: machine-readable static conclusions and limits.
- OFFSETS-AND-HASHES.json: all 54 blocks and instruction anchors, zero-based half-open UTF-8 byte offsets.
- FROZEN-MEMBERSHIP-RECHECK.json: unchanged 72-file hashes before/after, plus frozen membership hash.
- SOURCE-CONTEXT-RECHECK.json: complete physical E4D context hashes and final-head equality.
- TESTS-AND-RESULTS.json: exact original commands, limits, selectors and captured results.
- EXTERNAL-SCHEMA-TOKEN-INSERTION.json: unapplied derived-schema token template.
- verify_custody.py: reproducible static verifier; writes only this sibling directory.
