# Luna Graphify fragment

This is a navigation-only additive fragment for the current Rust Groth16 proof and the later public verifier-record export. It extends the existing UTxO graph at SHA-256 `611852f80031229330446a0c4b36b68ecdea4b632c5e6580f9a43f669d7b08e1` without changing base node payloads, the ordered-link prefix, inherited metadata, or source-path bases.

Open `graphify-out/graph.html` for the interactive graph or inspect `graphify-out/graph.json` for the merged graph. `graphify-out/GRAPH_REPORT.md` records the narrow result and the still-open boundary.

Sources are the 12 public files listed in `SOURCE-MANIFEST.json`, read at published commit `8360447be4b6eeb737a670baf98a78114b7e63f2`. See `READ-COVERAGE.json` for the exact read set. Graphify 0.9.91 `to_json` serialized the additive graph entries and `to_html` generated the visualization; token accounting is unknown/unavailable. The HTML can load its visualization library from the documented CDN when opened.

## R2 citation repair

R2 preserves the R1 graph and all 16 added node payloads and 17 added links except one citation field. The exact edit is `luna_k7_capsule.source_location`: `#/publicationRelativePath` → `#/entrypoint`; its claim text and all other graph semantics remain unchanged. The corrected graph SHA-256 is `3c7d25e9a94dd3c6a010ea06d79e796cc77ca1e6311748e3d1d0890a57422f7a` (R1: `3353b93cc9b006452f0407f0de1681f2d63b4582e5b9693d27098db2f86fbc35`). Graphify 0.9.91 regenerated `graphify-out/graph.html`; its R2 SHA-256 is `1802b9d65cf2c3b7865008c617c2069688cfe9d9bbbd6c798788dbc4839d485c` (R1: `1802b9d65cf2c3b7865008c617c2069688cfe9d9bbbd6c798788dbc4839d485c`).

The root output path in `OWN-OUTPUT-ALLOWLIST.json` now names this R2 directory. `READ-COVERAGE.json` records these hashes and exact field changes.
