# L01 ordinary dependency API comparison

The source-only predicate is: every newly used public lsp-server/gen-lsp-types API in the seven-path LSP candidate has an exact export, signature, field, enum variant or required trait/conversion in adopted lsp-server 0.10.0 and gen-lsp-types 0.11.0. The frozen custody packet supplied the package sources; no new web retrieval was needed.

Result: **no API-definition or signature mismatch found** across 37 rows covering all 24 imported dependency symbols and the implicitly inferred conversion types. No source patch is justified by this inspection. This does not claim compiler success, protocol conformance or product execution.

Source identity: I HEAD was the root-provided current main commit 74a909d69639df8823a527a4d608a7516ccd69a2. The seven files were uncommitted candidate bytes, copied from I and rechecked byte-for-byte after copying. `INPUT-SOURCE-PINS.json` records complete original paths, copied paths, sizes and hashes. The candidate snapshot is not assigned the base commit's tree identity. In particular, it includes the current manifests and formatted Rust candidate, not a stale local-main branch or earlier source-proposal formatting.

The actual public consumer is `commands/lsp.rs`: root-exported lsp_server Message read/write, Request/Notification field dispatch, Response construction, typed generated initialize/document parameters, initialize capabilities and publish-diagnostics serialization. `lsp/diagnostics.rs` consumes the generated diagnostic, range, position, code/message and related-information constructors. The public binary test manually frames JSON and does not call either dependency directly; its full normal input/expected fixtures are retained with the candidate. Those author-derived expectations were not executed or independently rederived in this lane.

Important exact definitions checked:

- PublishDiagnosticsClientCapabilities has `version_support` and nested `diagnostics_capabilities.related_information`. Serde flatten does not create an additional direct Rust field.
- VersionedTextDocumentIdentifier exposes `version` and nested `text_document_identifier.uri`.
- PublishDiagnosticsParams::new takes `(uri, version, diagnostics)`.
- Diagnostic::new takes nine parameters, including a generated diagnostic Message enum. The candidate's String conversions select its String variant. This is distinct from lsp_server::Message.
- Position derives Copy, so `Range::new(at, at)` is supported.
- TextDocumentSyncOptions has the explicit From conversion used for the capability union; Full serializes through its declared conversion as 1 and UTF16 as "utf-16".
- The no-URI-feature Uri declaration derives Clone/Eq/Hash, matching the candidate HashMap key use. This source condition is recorded rather than a resolved-Cargo claim.

`API-COMPATIBILITY.md` is the concise table. `API-COMPATIBILITY.json` preserves exact consumer anchors, line numbers, complete definition excerpts, conclusions and limits. Full relevant sources are copied separately: all 30 text files from both adopted dependency versions and the seven candidate files. The current Mori core Error definition/re-export files are also pinned because the candidate converts its actual String code/message fields into generated diagnostic fields. Original custody manifest, freeze, per-file archive provenance and retrieval/package records are preserved intact under `custody-provenance/`.

No dependency version was selected, Cargo/resolver/parser/compiler/server/package source was run, lock/cache/worktree/Git state was changed, or provider was dispatched. Startup used the develop skill, I AGENTS/product contract and read-only development status; historical SP01 accounting/binding blockers were retained and did not select new work. Root retains integration. Actual resolved dependency/feature/toolchain checks remain separate. Input/security/resource judgments are outside this lane; no such judgment, execution admission or roadmap closure is supplied.
