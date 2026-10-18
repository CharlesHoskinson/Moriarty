# Public language capability inspection proposal

Status: advisory ordinary source proposal, unapplied and unbuilt. Source baseline is exact commit `74a909d69639df8823a527a4d608a7516ccd69a2`, inspected through read-only `git show`, not local main. No subject/runtime, build, SDK, provider or ledger operation was performed. This L23 output owns only this new directory.

The proposed command is `mori capabilities --json` in the Rust workspace CLI. It emits a bounded catalogue of this executable's source profiles, grammar summary, consumers and compiled Cargo feature flags. It accepts no source, target, wallet or configuration file and performs no evaluation, compilation or execution. Editors can choose the correct parser/check command and suppress unavailable actions without treating a language name, command spelling or enabled build feature as proof of runtime or target support.

The command does not exist at the inspected commit. The complete examples are specified responses, not observed command output. The four-file patch is provided for root integration only.

## Concrete source findings

| Surface | Exact source finding at the pinned commit | Public response |
| --- | --- | --- |
| Rust Core/1 | `mori_core::parse` consumes `language "moriarty/1";` and constructs `moriarty-core/1`; finite UInt/Bool statements and expressions use the Core parser. | Independent source and semantics profile IDs; curated grammar and local-reference consumers. |
| Rust Core/2 | `stage::load` consumes `language "moriarty/2";`, validates/plans `moriarty-core/2`; `simulate` routes start/resume and `test` routes `moriarty-tests/2`. | Stage source support is present independently of `core2-disjoint-draft`; retain local-reference-stage, authentication:none and consumption:untracked. |
| Core/2 inspection | Source-only `inspect` returns the stage plan; `inspect --target` explicitly refuses Core/2. | Command-specific restriction, not a blanket statement about every compiler consumer. |
| Core/2 compilation | `commands/compile.rs` explicitly calls `mori_backend::stage::compile_staged` for `/2`; backend declares distinct artifact/4, ABI/4 and relation/4, with unsigned target restriction. | Compiler source is present behind `native`. Public source says legacy /3 proof/signed/native consumers cannot accept the /4 type. No observed compiler or target success is inferred. |
| Optional disjoint composition | `core2-disjoint-draft` enables the separate disjoint module and `moriarty-tests/disjoint-draft/1` test route. | Separate draft consumer and feature flag; do not disable base Core/2 when the draft feature is absent. |
| Rust beta frontend | `mori_parser::parse` accepts only `moriarty-beta/1`; `check` uses it when the first token is not `language`, then invokes `mori_checker::check`. | Separate frontend record; syntax acceptance and named-action support differ. No invented Core semantics ID. |
| Optional Source/6 transfer output | `source6-expansion` enables Rust `expand --emit-source6-transfer`. `select_transfer` parses/checks **beta input**, requires `LocalS0` plus operation `transfer`, and emits raw Source/6 proposal text. | Input `moriarty-beta/1`; output `Source/6`; no Source/6 input parser or Core/5 preparation claim. Preserve local-stipulation-only. |
| Source/6 local scenario | CLI requires `moriarty-local-scenario/1` with `kind:local-stipulation`, exactly three balance cells, and excludes obligation/candidate-effect overrides. | Scope notes under that consumer, not a claim of general Source/6 or beta-family support. |
| Rust editor servers | `commands/lsp.rs` and `mcp.rs` contain `todo!`. | `implementation:stub`, `availableInBuild:false`, even though clap registers the commands. |
| Node `mori` | `@moriarty-lang/beta` supplies another executable named `mori` with its own frontend and Source/6/Core/5 path. | Separate executable identity; installation/availability unknown. No Rust or editor-server capability inheritance. |

The Core inequality spelling in the inspected parser is `not =`, not `!=`. Core numeric values use checked u128 and canonical decimal JSON strings; beta syntax separately permits asset quantities and record/list construction. These distinctions are useful for editor hints. The grammar summaries deliberately remain curated: `check` is authoritative for a particular document, and an omitted entry is unknown rather than supported or necessarily forbidden.

## Response contract

`capabilities.schema.json` defines proposed `moriarty-cli-capabilities/1`. Two complete examples accompany it:

- `RESPONSE.default-build.example.json`: Cargo defaults, `native=true`, Source/6 and disjoint draft features false. This is a model of the manifest defaults, not a query of a running binary.
- `RESPONSE.opt-in-core.example.json`: native false, both ordinary opt-in features true. This shows that Source/6 proposal output and disjoint draft availability do not imply Midnight support.

The response has four essential distinctions. `profiles` describes source header, parser and semantics separately. `consumers` says which command uses each profile, its output profile and required build features. `producer.buildFeatures` describes this executable's compiled feature flags. `targets` records that no target was configured or qualified by this query.

`implementation` values are `source-present`, `stub`, `unsupported` and `unknown`. `availableInBuild` is true only for a source-present consumer whose listed build features are all enabled. This means the code path is compiled in; it does not assert that a request will pass admission or that the path was executed. An existing Core/2 compiler cannot be marked unsupported merely because another Core/1 consumer rejects its profile.

`producer.binarySourceRevision` stays null until a real build identity mechanism supplies it; no new mechanism is proposed. `catalogue.sourceRevision` identifies the source snapshot used to write the capability catalogue. It is not a binary, ABI, key or deployment identity. The patch uses actual `cfg!` flags and the package version at compile time, and does not stamp the catalogue commit as the binary's actual commit. Source-reference paths resolve at the catalogue commit. Future changes to public routing, feature flags, grammar, semantics or output formats require a catalogue update in the same product change.

The proposed query has no `allowed`, maintainer approval, reviewer, campaign, provider, security-owner or execution-authority fields. Public consumers are described by objective profile/build/input requirements. No review membership or project workflow becomes a developer prerequisite. No runtime checks are performed merely to fill capability metadata.

## Editor behavior and compatibility

An editor records the selected executable's identity and capabilities, not just the name `mori`. It selects Core/1 or Core/2 from the explicit `language` header and beta from the explicit `profile` header. This selection is a hint until the appropriate check succeeds. The Source/6 transfer operation is offered only for the beta input profile with its feature enabled; the actual checker must still establish the action subset. It must not offer a Source/6 input parser on that basis.

An editor can use the actual `check --json` command for document diagnostics. It should not launch Rust LSP/MCP on this catalogue's stub rows. UTF-8 diagnostic spans still need conversion into the editor's requested position encoding; the capability query adds no language server or span converter.

The current patch always prints JSON; `--json` is an optional explicit spelling for clients. Successful serialization exits 0 and means the report was produced. It does not mean all listed consumers are available. Unknown command/flags retain clap errors; serialization/export failure retains existing CLI errors. No source parsing or target I/O is needed. Output ordering is stable; no timestamps, random IDs, paths to private files or environment dumps are included.

Schema version changes are explicit. Consumers should treat unknown major versions as unsupported and unknown feature IDs as unknown; within this version, duplicate profile/consumer IDs and references to absent consumer IDs are invalid. The JSON Schema validates shape; cross-field consistency rules below are additionally required. Empty `types` for the curated beta summary means that its type catalogue was not enumerated here, not that beta has no nominal types.

## Unapplied source proposal

`UNAPPLIED-cli-capabilities.patch` changes only two dispatcher files and adds one leaf module plus its embedded catalogue:

1. `crates/mori/src/cli.rs`: add `Capabilities` with its argument type.
2. `crates/mori/src/commands/mod.rs`: register and dispatch the module.
3. `crates/mori/src/commands/capabilities.rs`: assemble metadata, calculate availability from actual feature flags, and use existing bounded JSON export.
4. `crates/mori/src/commands/capabilities.catalogue.json`: the bounded source catalogue.

`proposal/` holds the full resulting files for exact-candidate review. Shared dispatch edits remain root-owned and unapplied. No new dependency, feature, Cargo mutation, build script, source hash identity, provider integration or scheduler is introduced. The existing `core_io::print` calls Core JSON export and stdout; the new route does not call the compiler, evaluator, SDK or native APIs. The surrounding crate can still include SDK dependencies in a native build; this source-only proposal does not claim an SDK-free native build.

A static catalogue is intentionally narrower than runtime probing and safer for an editor startup query. Its tradeoff is maintenance: feature strings and grammar summaries can drift. Deriving all grammar and admissions automatically would require a larger cross-crate interface. Defer that work until a concrete consumer exposes catalogue drift; keep this first command limited.

## Proposed ordinary verification, not executed here

After integration is authorized and exact commands are inspected, use a no-default-feature CLI build for the ordinary path. Check the complete JSON output against independently frozen expected data, including package version and actual feature combination, rather than updating expectations from output. The useful matrix covers no-default features, Source/6-only, disjoint-only and both opt-ins; the native combination needs its existing independent build/execution scope and is not necessary to observe the ordinary command.

Check invariants: unique IDs; all build-feature dependencies declared; availability equals source-presence plus compiled features; stub consumers false; target consumer references resolve; no configured target appears; no profile inheritance from Source/6 output or Node executable; base Core/2 stays present without disjoint draft; Core/2 staged compile and target-inspection refusal remain distinct. A successful no-file query establishes only this reporting consumer. No security-control or native/proof tests are specified by this proposal.

No Rust formatting, clippy, Cargo build/test, actual CLI invocation or changed-source result review was run. Static JSON/schema and patch-content checks are recorded separately in `STATIC-CHECKS.json`. Passing those checks does not compile the patch or establish product behavior.

## Inspection limits and handoff

`SOURCE-INVENTORY.json` records the exact public blobs and hashes. Full blobs are captured for reproducibility, but inspection was limited to relevant public parser/profile/dispatch/format sections and documented qualifications. The staged compiler was read only for its public routing and source-declared artifact boundary; no cryptographic correctness or security judgment is made. The Node CLI was inspected as a separate executable; its complete bridge/lowerer and installed availability were not audited. No web or private configuration was read.

The installed development guidance was loaded, but its status/runtime command was not run: this assignment expressly permits source-only inspection at a commit without a checked-out current tree and prohibits runtime execution. Historical status would not select this task. The newer installed beta guidance does not widen this L23 assignment.

Root must adopt or revise the public contract, integrate the exact dispatch patch, refresh the catalogue if its candidate differs, obtain the applicable ordinary nonauthor review, and run allowed ordinary checks before claiming this command exists or works. Broader language, compiler, security and ledger acceptance remain separate.
