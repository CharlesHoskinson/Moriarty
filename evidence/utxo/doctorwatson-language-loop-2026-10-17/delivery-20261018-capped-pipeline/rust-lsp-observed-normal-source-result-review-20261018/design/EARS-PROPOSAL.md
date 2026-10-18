# Proposed normal Rust LSP child clauses — unadopted

Existing UNI-018/024, SP02/SP12 and all U/M/MC/SP/G exits stay authoritative. These proposed child clauses do not modify the original clauses or close them.

LSP-N01 — WHEN a public Rust mori lsp client initializes a stdio session, THEN the server SHALL return the literal supported Full-sync UTF16 diagnostic capabilities and local-check-only source-profile metadata; it SHALL advertise no unimplemented editor feature.

LSP-N02 — WHEN the client opens or supplies a newer full version of a Moriarty document, THEN the server SHALL analyze the supplied text through the same profile-selected public parse/check/load APIs as mori check and publish complete diagnostics with the matching URI and version. Profile and Rust-consumer qualification SHALL remain explicit; no compilation/target/evaluation result is inferred.

LSP-N03 — WHEN a normal source error has a primary/fallback source label, THEN the published diagnostic SHALL retain its code/message/help, severity and UTF16 range; other located labels SHALL remain related information. For existing Core error values without full labels it SHALL preserve the existing optional offset and use the proposed explicit zero-width fallback rather than inventing spans.

LSP-N04 — WHEN a normal full edit removes the source mistake, THEN the matching newer version SHALL publish diagnostics []; WHEN the document closes, THEN its retained text is dropped and diagnostics [] is published without a version.

LSP-N05 — WHEN the normal proposed initialization/edit/close/shutdown/exit transcript completes, THEN the actual Rust CLI SHALL return exit0, write exactly the9 specified ordered framed semantic JSON objects on stdout, and leave stderr empty. This finite positive protocol predicate establishes no hostile-input, resource, editor-platform, compiler/SDK/proof or ledger qualification.

Owner pending: input/resource/hostile/security clauses and tests must be separately authored/reviewed by the designated owner. This ordinary proposal does not assert them.
