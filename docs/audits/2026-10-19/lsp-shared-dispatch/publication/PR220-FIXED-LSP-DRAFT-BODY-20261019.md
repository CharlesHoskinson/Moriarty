This implements `mori lsp` diagnostics for the existing Beta, Core/1 and Core/2 source profiles, including initialization, full document updates, diagnostic clearing and normal shutdown.

The repair routes LSP checks through the same source-profile dispatcher as `mori check`. An unsupported language header now produces the same diagnostic through both commands, rather than drifting into a different parser path. UTF-16 diagnostic positions and the existing profiles remain covered by the selected protocol cases.

Validation on `ffbf3456888ba91940c590a6b7987aedee6db051`: the checker-parity regression failed before the frozen one-file repair; both selected Rust LSP tests passed afterward. The checks compare complete semantic protocol messages and four public-check diagnostic projections. These are ordinary, source-exposed expectations. No tests were rerun for this publication.

Draft: fresh full source/result and security reviews remain missing; earlier attempts produced no eligible opinion. Workspace-wide checks, clippy, native proof and ledger qualification remain unrun for this candidate. Current main merges without file conflicts; this is a source integration check, not execution of the merged tree.
