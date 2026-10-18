# Required review scope and source custody

This packet is transport prepared by Sol. It supplies **no Sol security assessment, tests, patch, runtime grant, or product acceptance**. The transport checked source identity and copied bytes; it did not claim to semantically read the entire parser/checker/core closure. Reviewers must return their actual file/read scope, including any partial or unread files. A missing read cannot become an approval by inference.

The primary is below 110,000 bytes and each role's prompt is below 125,000 bytes. `MANIFEST.json` pins all frozen input files. `CURRENT-CANDIDATE.json` pins the eight changed files to the exact current checkout. `REPOSITORY-SOURCE-PINS.json` identifies 285 full tracked dependency/context source files copied from the same HEAD. All copies were made with read-only Git and file operations; no product module was imported or executed.

## Candidate and actual public path

Read all eight complete files in `candidate/`, including Cargo.lock. The primary embeds the complete three Rust files, both complete fixture JSONs and both Cargo manifests. Cargo.lock is physically complete and is not described as inline. Candidate HEAD is `adff55f33942310123dc32f272cd6291e5b5c4da`, tree `fe345ec28bd5444ea8c7204a4eb48d643962fe8e`, actual main base `e4d7a2b5709caa18a94f5256d03a0302364b8235`.

Inspect actual command dispatch through full `source/crates/mori/src/{main,cli}.rs` and `commands/mod.rs`; actual check routing is full `commands/check.rs` inline. Full `commands/core_io.rs`, error/display/shell/json sources are physical context for error reporting. Full source trees for mori-lexer, mori-parser, mori-checker, mori-ast, mori-span, mori-diagnostics and mori-core are present under `source/crates/`, including Core2 stage decode/admit/validate/profile and current public Error/re-export. The packet does not substitute a stale private-module Error spelling for the actual public re-export.

## Dependencies and observed graph

Read the actual `public-dependencies/lsp-server-0.10.0/src/msg.rs` Message implementation **in full**; it is inline. Full lsp-server, gen-lsp-types and selected crossbeam-channel 0.5.17 package sources are physically supplied with public archive provenance. `public-dependency-archives/` preserves all three public crate archives. `public-dependency-custody/` preserves full historical registry/archive/per-file provenance without updating historical unresolved labels. The successful current Cargo metadata observation, not those old labels, establishes selection of the three IDs for this candidate.

The full 37-row `api-compatibility/API-COMPATIBILITY.md` is inline; its 64 KB JSON and exact dependency definition excerpts, historical source checks and source-note qualifications remain physically complete. It is source inspection, not a compiler result. Read relevant generated definitions in gen-lsp-types as needed and state exact coverage.

`dependency-preparations/dependency-resolution-vendor-a2/STDOUT.json` is the complete observed Cargo metadata graph. RESULT records exit 0, 456 nodes, explicit 428-package local directory source, only three new package IDs and mori's two direct dependency additions. PACKAGE-SOURCE-PINS pins archive/checksum/source identities. Full START/CHILD/config/lock-before/after and results are supplied. Full original failed offline missing-index and first vendor zero-link preparations remain under their original separate names. No fresh resolution, build or protocol run was done by this task.

## Design, original whole variants, oracle and fixtures

Read full `design/DESIGN.md`, EARS-PROPOSAL, DESIGN-DECISION-CONTRACT, original actual-source facts and reviewed-subject pin. Those original documents retain their original proposed/historical scopes. `design-reviews/` contains the complete two original approved contexts' WHOLE-SCHEMA-VALID-CANDIDATES arrays, all whole variants, ambiguity records where present, process/public receipt/root disposition and schemas. Both full arrays and `design/DESIGN-ADOPTION.json` are inline. Their adoption was three ordinary design choices only, with no implementation/security/execution approval. `original-a1-design-preparations/` preserves the earlier source-fidelity failures and complete objects. No variant was spliced or silently selected.

Candidate `input-session.json` and `expected-session.json` are complete inline author-exposed proposed normal fixtures. They are unrun and are not an independent oracle. `independent-L12/` retains the separate full six source inputs, independent semantic expectations, normal event predicates, custody disclosure, parser/checker basis and normative protocol excerpts plus original retrieved official specification. L12 did not inspect this candidate/test/expected fixture, and its prior unrelated DevEx exposure is expressly preserved. Its semantic message and optional/help/source-field scope must be distinguished from the candidate's adapter expectations; do not rewrite either into agreement.

## Review roles and execution boundary

The ordinary context reviews complete candidate fit, actual API definitions/graph, command routing, normal diagnostic/readback correspondence and Rust guidelines. It must route security judgments to the designated fallback context.

The designated security fallback context reviews complete source, including actual protocol framing/allocation/document storage/error/lifecycle behavior, and may return exact source change requests or a qualified opinion. Sol supplies no judgment on those topics. The requested model is AGY `gemini-4-argon-high` at HIGH; actual canonical identity/effort remains unknown absent independent provider-envelope evidence. Two contexts are distinct tasks, not two votes inferred from whole variants.

Root reports Opus remains unavailable; `outage/OPUS-LAST-PUBLIC-PROCESS.json` preserves the newest public terminal failure. Preserve requested identity separately from absent actual assistant frames. The standing user outage fallback authorizes Gemini/Muse; this packet does not relabel it as Opus.

No exact frozen normal-protocol execution plan is supplied in this packet. Source opinions must not grant one or assert protocol safety. This does **not** create an extra execution approval gate: root retains the user's ordinary source/tooling authority to select an inspected bounded single normal native-OFF CLI protocol test. Security/hostile-test work remains owner-assigned; SDK/native/proof/ledger admissions remain distinct. A normal pass cannot become production security, broader protocol conformance, target/ledger qualification, or parent roadmap closure.

Do not execute tests/compiler/Cargo/server/model/SDK/native/proof/ledger work, modify files, use the network, read private captures, request user input or spawn agents. Public read-only file tools are available for this source review. Return the complete schema-valid result with honest exact source coverage and limitations.
