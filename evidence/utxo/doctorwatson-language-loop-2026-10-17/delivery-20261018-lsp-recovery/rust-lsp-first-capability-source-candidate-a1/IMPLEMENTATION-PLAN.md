# Rust full-sync diagnostics source candidate implementation plan

Goal: replace the public Rust mori lsp stub with the adopted local-check-only Full-sync diagnostics slice, preserving the frozen11-to9-message normal oracle.

Architecture: lsp.rs owns synchronous typed protocol state and inherited stdio through lsp_server::Message. lsp/diagnostics.rs projects the existing public parser/checker/Core errors into typed UTF16 diagnostics. One real-binary integration test consumes the frozen normal transcript, not a mock/helper server.

Constraints: root-adopted D1/D2/D3 only; confirmed remote-main full7396 source basis; no repository/Git writes, package execution, Cargo/lock generation, SDK/proof/runtime/model or network acquisition. Root's explicit source-only instruction overrides execution steps: write test first, but RED/GREEN remain NOT RUN and no TDD pass is claimed. Security/hostile/resource judgments/tests remain designated-owner work before actual protocol execution. Original all U/M/MC/SP/G obligations and caps/failures stay unchanged.

1. Copy the pre-implementation normal input/expected byte files into integration fixtures, then write crates/mori/tests/lsp.rs to spawn the actual Rust binary, independently frame/read stdio, and assert nine full ordered semantic JSON values, exit0 and empty stderr. No producer-generated expected values.
2. Add exact workspace dependency proposals and workspace references in the mori manifest without editing Cargo.lock. Preserve native/default/source6-expansion features and all existing dependencies/tests.
3. Replace commands/lsp.rs todo with typed initialize/Full sync/close/shutdown/exit state, public Message read/write, explicit local-check-only/profile capabilities, supplied text held by opaque URI and ordinary parser/checker diagnostics.
4. Add the direct diagnostic adapter and UTF8-byte to UTF16 line/character conversion, using public mori_core::Error. Preserve existing code/message/help and available primary/related labels, and avoid fabricated Core end spans.
5. Compare before/after bytes statically, generate the complete unapplied patch and pin every source/fixture. Verify original frozen oracle byte identity and current Cargo.lock identity. No compilation/formatter/parser/test execution.
6. Handoff complete source, selected call graph/exposure and exact unrun test selector. Actual new dependency resolution/source review, full candidate ordinary/security owner reviews, launch-time effects/resource inspection and any actual test remain separate root steps. No language/editor-release acceptance inferred.
