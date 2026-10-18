# Catalog metadata proposal design and plan

The assigned predicate is a developer-facing typed schema, an unapplied JSON/TypeScript proposal, and a concrete display example using two existing source-declared predicates. This scoped assignment authorizes preparation in this directory. It authorizes no shared source integration, Git mutation or runtime execution.

The selected approach is a lossless display projection around the retained source rows. An alternative current-head-only index would hide historical versions; a new runtime proof registry would introduce acceptance and execution responsibilities outside the assignment. The selected approach preserves143 rows while separately recording89 heads computed from explicit supersession links.

The proposal separates ordinary metadata shape from designated-owner security semantics. Source fields, binding declarations, descriptions, statuses and assumptions are copied rather than interpreted. Missing typed input/output fields, setup records, selected profile and target are explicit nulls. The existing catalogue SemanticEntry remains unchanged. Source SHA-256 is a file-provenance field, separate from predicate identity and cryptographic digest rules.

Files have these responsibilities:

- `sources/`: byte-identical public inputs from exact main74a909.
- `SOURCE-MANIFEST.json`: source locations, SHA-256, byte counts and read limits.
- `catalogue.ts`: typed source/display contracts and pure loader/projection/selection/display functions.
- `catalogue.schema.proposal.json`: proposed ordinary JSON metadata shape.
- `catalogue.view.proposal.json`: all143 source rows projected with explicit owner dependencies.
- `example.ts`: concrete two-entry selection and WebDiagram prop mapping.
- `two-predicates.expected-display.json`: source-derived expected fields and display labels, not runtime output.
- `README.md`: use, provenance, integration boundaries and verification limitations.

Preparation steps:

- [x] Read exact source object74a909; distinguish it from the older working checkout and the non-Git evidence workspace.
- [x] Inspect retained catalogue shape, four typed rows and the selected P-L0/P-M0@2 declarations; inspect actual WebDiagram props and related Webs evidence exposition.
- [x] Pin/copy only public source inputs in this owned directory.
- [x] Prepare all-row JSON projection and proposed JSON/TS types without changing original predicate or acceptance semantics.
- [x] Prepare a concrete loader/display example and expected source-derived output for8/12 fields.
- [ ] Typecheck/run the TypeScript and compare example output: unrun because this assignment prohibits runtime execution.
- [ ] Obtain applicable nonauthor ordinary review and owner review of any future security-dependent use: not supplied by this proposal.

Self-review checks cover source/version membership, ordered statement fields, exact original rows, absence of invented selected target/setup/predicate digest, explicit source-status display and the limits of the WebDiagram adapter. No implementation merge, broad test suite, runtime test, publication or ledger acceptance follows from a source proposal.
