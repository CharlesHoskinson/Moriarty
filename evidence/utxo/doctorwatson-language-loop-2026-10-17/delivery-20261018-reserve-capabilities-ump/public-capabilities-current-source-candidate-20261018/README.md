# Public capabilities: current-source implementation proposal

Base: `e4d7a2b5709caa18a94f5256d03a0302364b8235`. This packet is a source implementation proposal, unapplied, unbuilt and untested. It does not adopt capability claims as release/target acceptance. No shared checkout, Git state, editor module or Cargo manifest was changed.

`candidate.patch` adds the actual `mori capabilities --json` route and leaf implementation plus two normal public integration tests. `before/` captures all inspected baseline blobs, `after/` contains the four complete resulting candidate files, and both membership JSON files record their exact hashes. The two new files are explicitly absent in the baseline. The earlier L23 schema was an advisory input; the implementation was re-authored against this actual main revision. No prior catalogue or source implementation was copied as its authority.

## Result and scope

The command emits one bounded JSON object and a final newline, with no status line. `--json` is an optional explicit spelling; plain `capabilities` and quiet mode emit the same JSON. It accepts no file, target, network, wallet, configuration or project metadata. Its only production call outside metadata construction is existing `core_io::print`, which uses bounded Core JSON export and stdout. It does not run the parser/checker/evaluator/compiler, probe an installation, read the filesystem or network, or invoke SDK/proof operations to fill the report. The native feature can still link optional SDK dependencies in that build; that is separate from what the new query executes.

The three feature booleans come directly from `cfg!`: native, source6-expansion and core2-disjoint-draft. Source/semantics IDs use exported constants from linked Core/Core stage/beta parser crates. Consumer notes and output artifact IDs are deliberately curated source observations; availableInBuild means only source-present plus its compiled feature requirement. It does not say that any input is admitted or a compiler, proof, target or network was qualified.

The response keeps the proposed `moriarty-cli-capabilities/1` envelope and its source/consumer/target/executable distinctions. It is intentionally smaller than L23's catalogue: grammar lists are explicitly not enumerated, apart from the Core UInt/Bool type summary. It is not a grammar-completion API. An omitted capability remains unknown. No unsupported guessed grammar, complete backend inventory or application admission registry is added.

The meaningful distinctions are:

- Core/1 and Core/2 have separate actual source/semantics/parser records. Base Core/2 remains present without the optional disjoint draft feature.
- Beta checking has its own source profile and no invented Core semantics ID. The optional transfer expansion consumes beta and emits raw Source/6 proposal text; it supplies neither a Source/6 input parser nor Core/5 preparation.
- The actual public compile command routes Core/1 to artifact/3 and Core/2 to the distinct staged artifact/4, behind native. Core/2 source-only inspect is present while its target-inspection consumer refuses. These are separate rows rather than a blanket Core/2 support claim.
- The target row requires an independently supplied descriptor, has configuredTarget null and qualification not-checked. Native build availability cannot turn it into a configured/observed target.
- The separate Node package also installs `mori`; its installed availability and capabilities are not inherited by this Rust executable.
- E4D's LSP and MCP bodies remain stubs. Neither is advertised as available. No unmerged LSP220/adff55 behavior is advertised.

The report has no approval, reviewer, campaign, provider, security-owner or maintainer-permission input/field. Public program use remains governed by the objective supported profile, owner authority and actual proof/ledger requirements.

## Currentness and the unmerged LSP change

`catalogue.sourceRevision` is the exact inspected snapshot of the curated consumer notes, not the binary commit, ABI, deployment or key identity. `producer.binarySourceRevision` is null because this repository supplies no build identity mechanism for it. The report explicitly warns that routing/editor changes require a catalogue update. It does not claim to discover source changes automatically.

When PR220/adff55 or another editor implementation is merged, root must inspect the exact merged implementation and update the rust-lsp consumer's implementation, availability status, admitted source profiles/output description and limitations to that actual scope. Update the catalogue inspection revision/notes and the integration contract's expected editor row in the same candidate. MCP remains a stub until its own source changes. A normal diagnostic LSP does not imply formatting, completion, MCP, target inspection or native support. Changed full candidate bytes require their applicable fresh audits.

This is an explicit integration obligation. There is no include_str/source-marker mirror test, source classification/reflection, editor-module mutation or speculative normal-diagnostics flag in this lane. Root may later choose a module-owned declarative status interface, but none is introduced here.

## Proposed normal verification

The tests were authored before the new leaf implementation but cannot be run in this lane. Their red/green results are UNRUN. They use the real CARGO_BIN_EXE_mori with empty PATH and closed stdin from an isolated directory. They create their own ordinary fixtures, not project metadata or test-only production hooks.

The first test consumes the entire JSON response, checks deterministic whole stdout across explicit/default/quiet spellings, verifies all top-level sections, uniqueness and target-consumer references, and compares the actual build feature bits. It checks parser/source/semantics and transfer-output distinctions, optional draft independence, separate artifact versions, target-inspection refusal, source-current editor stub rows and unconfigured targets. It retains a user file and checks that the directory bytes remain unchanged.

The second test acts like an editor client: selects the two Core profile records, uses their returned source headers to author a new finite program (Core/2 includes await), then invokes real source-only check and inspect. It compares accepted language/semantics against the independently expected profile, retains the full input directory bytes and requires source-only enforcement_report null. It invokes no compiler, target, proof or SDK path. The tests do not assert source markers or parse production source text.

Integrator commands, proposed only:

```sh
cargo test -p mori --locked --offline --no-default-features --test capabilities
cargo test -p mori --locked --offline --no-default-features --features source6-expansion,core2-disjoint-draft --test capabilities
```

Expected: two selected integration tests pass in each build. The first build reports all feature bits false; the second reports only the two ordinary opt-ins true. In both, base Core/2 remains present, compiler rows remain unavailable because native is false, Core/2 target-inspection remains unsupported, targets remain not-checked and editor rows remain stubs at this source snapshot. A no-default source-only query is enough to observe the new reporting consumer; no default/native build or broad test suite is requested by this packet. Formatting/clippy and any wider feature matrix remain integrator checks under their existing scope.

A baseline run of the tests before the source/dispatch patch would fail because capabilities is not a subcommand; that result was not executed here. Offline dependency failure would be a build prerequisite failure, not a test observation. Source/static packet inspection does not establish compilation, successful CLI execution or product closure.

## Evidence status

Source facts: exact pinned current main public routing, profile constants, feature declarations, source-only readers/export and compiler output boundaries were inspected. Ordinary implementation: proposed full source and integration tests are supplied. Actual execution: none. No Cargo/rustfmt/clippy, product/runtime/model/compiler/SDK/native/provider execution, hosted service, install or network operation was performed. No security or hostile tests/judgments are authored. Root retains review, integration and actual-check ownership.
