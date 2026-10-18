# Public capabilities delivery checklist

Source base: `e4d7a2b5709caa18a94f5256d03a0302364b8235`. Candidate: the complete four-file v3 under `v3/after`. Root owns integration, applicable audits and actual execution. This lane inspected source and prepared a plan/correction; all product/build/test results below are UNRUN.

The Moriarty development skill and exact-base AGENTS were loaded. The explicitly requested read-only plugin status was refreshed from the active checkout. It reported the historical SP01 loan/swap blockers and no pending transactions. Those historical records do not select this task, authorize native execution or create a public developer prerequisite.

## 1. Freeze the candidate and resolve the source base

- [ ] Confirm the four reviewed paths and their exact hashes in `v3/membership.json`: CLI enum/import, command dispatch/module registration, capabilities leaf and capabilities integration target.
- [ ] Use `v3/candidate.patch` against the exact base; alternatively apply `v3/v1-to-v3.patch` to the already integrated frozen v1. Both full before/after files are supplied. New files are explicitly absent at base. Do not apply both patches.
- [ ] Resolve any main changes through exact source review. No stale local-main or unrelated worktree bytes were used for this packet.
- [ ] Review the metadata correction and the branch-specific inspection test repair described below. The original proposal and all its hashes remain unchanged. Changed v3 bytes require fresh exact-candidate audits; v1 review or future LSP review is not silently reused.
- [ ] If root formats or otherwise changes integrated bytes, capture and audit those actual resulting bytes. No formatting, compiler or test success is asserted here.

## 2. Resolve the ordinary build surface

Exact-source findings: `crates/mori/Cargo.toml` has default=[native]. Native enables optional backend, ledger and Midnight dependencies. Therefore plain Cargo defaults are not the source-only build. `--no-default-features -p mori` selects the ordinary CLI without that feature. The two optional ordinary feature flags are source6-expansion and core2-disjoint-draft.

The source6 flag enables the optional local mori-source6 crate and its transfer slice. Its manifest has no native dependency. The disjoint flag enables only mori-core/core2-disjoint-draft. Mori-core's optional Midnight compatibility dependencies are activated by a different, unselected midnight-compat-tests feature. Root workspace dependency declarations inject no additional features into those crates. Cargo resolver is3. The repository pins Rust1.99.0; if that toolchain or offline dependencies are absent, report a prerequisite failure rather than install or download within this plan.

- [ ] Use package scope `-p mori`, `--locked --offline`, and `--no-default-features` for the commands below. Do not add workspace/default/native/all-features/native-compatibility flags.
- [ ] Preserve the existing local execution/resource policy. This document adds no SDK/native/proof authority or execution framework.
- [ ] Tests build the actual CLI and use Cargo's CARGO_BIN_EXE_mori; no ambient PATH executable named mori is selected. The child PATH is empty. No external `which` or shell helper is needed.

## 3. Run the exact two normal consumer tests

First selector:

```sh
cargo test -p mori --locked --offline --no-default-features --test capabilities public_query_distinguishes_profiles_build_consumers_and_unchecked_targets -- --exact --nocapture
```

Expected observation: one selected test passes; product subprocesses exit0 with complete JSON on stdout and empty stderr. The test runs explicit `capabilities --json`, default `capabilities` and quiet spelling from an external directory containing a user file. It checks whole-document parsing, complete envelope sections, byte-identical deterministic output, actual build-feature values, unique identifiers, target-consumer references, profile/output/consumer distinctions, current editor stub rows and unchanged user-directory bytes.

Second selector:

```sh
cargo test -p mori --locked --offline --no-default-features --test capabilities reported_core_profiles_select_real_source_only_cli_consumers -- --exact --nocapture
```

Expected observation: one selected test passes. It consumes returned source headers to author two new bounded programs. Both actual check calls report accepted with the expected source language. Both actual source-only inspect calls exit successfully and return exact language/semantics with a present enforcement_report field equal to null. Core/1 has no status field, qualification source-only-unqualified and outer format moriarty-enforcement-report/1. Core/2 has status accepted, qualification local-reference-stage, authentication none, consumption untracked, no outer format and nested stage_plan.format moriarty-stage-plan/1. The complete source-directory bytes remain unchanged. These are positive actual consumer exercises, not source-marker or implementation-text comparisons.

No compiler/target/proof/SDK command is invoked by either test. No hostile or security test is proposed. The second test exercises source-only check/inspect; it does not simulate an action or measure logical work. The first command's metadata rows describing compilers are observations of the report, not executions of those compilers.

An optional ordinary-feature observation uses the first selector only, because that selector checks dynamic build availability:

```sh
cargo test -p mori --locked --offline --no-default-features --features source6-expansion,core2-disjoint-draft --test capabilities public_query_distinguishes_profiles_build_consumers_and_unchecked_targets -- --exact --nocapture
```

Expected: one selected test passes; source6-expansion and core2-disjoint-draft are true, native remains false. Only the transfer-proposal and disjoint-draft consumer availability changes relative to the no-default report. Base Core/2 remains present and compiler rows remain unavailable. This query does not execute transfer expansion or a disjoint test project.

Do not broaden or rerun passed checks solely to refresh a dashboard. A test/build failure needs its own exact diagnosis. The initial test-before-dispatch RED would reject capabilities as an unknown subcommand at the unmodified base, but that result is specified and was not observed in this lane.

## 4. Consume the full expected public JSON contract

`specified-expected/no-default-features.json` and `specified-expected/ordinary-optins.json` contain the complete proposed parsed envelopes, including every exact field and descriptive string. They are independently authored expected contract documents, not captured command output, not generated from a running binary and not imported by the integration tests. Map key order in these examples is not a byte-order requirement; object shape and values are. The real query uses the existing canonical bounded exporter; the first test checks deterministic real bytes across invocation spellings.

Top-level fields are exactly format, producer, catalogue, profiles, consumers, targets, otherExecutables and limitations. The format is moriarty-cli-capabilities/1. No successful runtime status or target acceptance field is added.

Producer has implementation=rust-workspace-cli; packageVersion=0.1.0 at this source; binarySourceRevision=null; responseKind=command-self-report; and buildFeatures with exactly native, source6-expansion and core2-disjoint-draft Bool values. These flags report cfg values of the selected build. A catalogue source revision is not the binary revision.

Catalogue has sourceRevision=e4d7a2b5709caa18a94f5256d03a0302364b8235, coverage=selected-public-profile-consumers, basis=source-inspection, runtimeEvidence=not-included and a note distinguishing curated notes from linked constants/build flags. Routing/editor changes require explicit catalogue maintenance.

Each of the three profile objects has id, sourceProfile, sourceHeader, semanticsProfile, parser, implementation, grammar, qualification and limitations. Core/1 maps language moriarty/1 to semantics moriarty-core/1 via mori_core::parse. Core/2 maps moriarty/2 to moriarty-core/2 via mori_core::stage::load. Beta maps profile moriarty-beta/1 to parser/checker with semanticsProfile null. Core types are UInt/Bool. Grammar has coverage, types, declarations, statements, expressions, unsupported, bounds and notes. Other grammar lists and bounds are deliberately unenumerated, not claims of absent or admitted syntax. Qualification is source-description-only.

Every consumer has id, entrypoint, inputProfile, semanticsProfile, outputProfile, requiresBuildFeatures, implementation, availableInBuild, qualification, limitations and sourceRefs. The complete fourteen-row table is frozen in the specified examples. Important cross-field expectations are:

| Consumer | No-default availability | Ordinary opt-ins availability | Result/profile distinction |
|---|---|---|---|
| core1-check / core2-check / beta-check | true | true | Three distinct frontend/profile routes |
| core1-reference | true | true | Core/1 local reference consumer; not a trace/native result |
| core2-reference | true | true | Unversioned outcome envelope; only Waiting has nested continuation/1 |
| core1-inspect | true | true | Whole enforcement-report/1 with no target supplied |
| core2-inspect | true | true | Unversioned envelope with nested stage-plan/1 |
| core2-target-inspect | false | false | Unsupported profile even when its separate native requirement is enabled |
| core2-disjoint-draft | false | true | Optional draft test-project consumer, independent of base Core/2 |
| beta-to-source6-transfer | false | true | Beta input, Source/6 proposal output; no Source/6 input parser/Core5 preparation |
| core1-compile / core2-compile | false | false | Distinct artifact/3 versus staged artifact/4; native not enabled |
| rust-lsp / rust-mcp | false | false | Both implementation=stub at this exact source |

The target array contains only midnight-ledger9-zkirv3. Its fields are id, consumerIds=[core1-compile,core2-compile], descriptorRequired=true, configuredTarget=null, qualification=not-checked and notes. It does not configure a network/anchor or inspect parameters, keys, proofs, installation, deployment or finality.

OtherExecutables contains node-beta-cli with package @moriarty-lang/beta, binaryName mori, a separate-executable relationship and availability not-inspected. The limitations array explicitly restricts build availability, catalogue completeness and runtime/target inference. No project admission, campaign/reviewer/provider credential or maintainer approval field exists.

For compiler consumer rows, outputProfile describes the produced artifact file, not a claim that compile emits that object on stdout. For Source/6 it describes the raw proposal text profile. For the corrected Core/2 rows there is no single named whole-result format, so outputProfile is null and the nested DTO paths are described explicitly. The semicolon in core2-reference.entrypoint separates the start and resume spellings; it is descriptive metadata, not one runnable command.

## 5. Apply the source-field precision correction and normal-test repair

The exact Core/2 source in stage/report.rs constructs status waiting/complete/rejected envelopes with qualifications, but no format field. Only Waiting contains continuation. Source-only inspect likewise builds an envelope without format and embeds a stage_plan. Therefore v1's outputProfile=continuation/1 or stage-plan/1 could mislead a client choosing a whole-response decoder.

The preserved v2 sets those two outputProfile values to null and states their exact nested locations/conditions in limitations. The existing source-only inspect integration test now checks actual outer versus nested formats; the first test also checks that the Core/2 reference row has no single output profile. Root subsequently found a separate v1/v2 test error: Core/1 inspect has no status field, so a shared status=accepted assertion was incorrect. Frozen v3 changes that test to the exact branch-specific fields listed above, including actual Core/2 stage labels and a present null enforcement_report. The capabilities leaf is byte-identical to v2; the two dispatcher files are byte-identical to v1. Test names and count remain unchanged. V1 and v2 are preserved. No editor module, source semantics, feature or target behavior changes.

This is a source/DTO precision repair, not a claimed runtime failure or security judgment. All remaining asserted check/inspect fields and the capability shape were rechecked against the exact source; no other concrete inaccuracy was found in that inspected scope. This is source inspection, not an observed test pass. All curated facts still require fresh runtime observation and applicable nonauthor review before delivery claims.

## 6. Handle LSP220/currentness before final integration

At exact E4D both lsp.rs and mcp.rs are todo! stubs; their command registration does not make them implemented. The frozen report correctly advertises implementation stub and availableInBuild false. It makes no claim that PR220/adff55 is merged or qualified.

- [ ] If the integrated source still has the E4D stubs, retain those rows and their expected stub values.
- [ ] If PR220 or a later LSP implementation is integrated, root must inspect its actual merged scope and change the LSP implementation, availability, admitted input/output descriptions and limitations to that source. Update the catalogue inspection basis/notes and applicable output-contract expectation. Normal diagnostics alone do not imply completion, formatting, MCP or target support.
- [ ] Keep MCP a stub until its own implementation changes. Never inherit LSP claims into MCP.
- [ ] Obtain fresh exact-candidate review after any changed report/test bytes. There is no source reflection, include_str marker test, automatic classification or invented binary SHA to perform this update.

## 7. Report evidence and retained acceptance separately

- [ ] Implementation evidence: identify integrated exact source/tree, build feature combination and applicable full-candidate audits.
- [ ] Ordinary result evidence: retain actual commands, subprocess/test exits, selected test counts, stdout/stderr and their bindings. A supplied expected JSON file is not observed output.
- [ ] Acceptance: a passing result establishes the selected read-only reporting and source-only client paths for that exact build. It establishes no compiler execution, native proof, authenticated continuation, private composition, deployment or financial ledger settlement.
- [ ] Preserve every U0–U7, M1–M17 and MC/SP/G exit; no exit is closed by this plan, source patch, metadata or a passing local query. Preview/Fenrir and other target observations stay separate.

No shared checkout or Git state was changed. Apart from the explicitly requested startup status read, no product/runtime/build/test/SDK/native/proof/wallet/provider or network execution was performed. All expected commands and results remain proposed for root.
