# Bounded type-checker implementation plan

Status: SOURCE_ONLY / UNRUN. Existing approved narrow scope; no new design or permission loop. The old R192 request returned zero source and supplies no accepted implementation/API.

Goal: guard caller-selected active structural type inference depth before descent, child-path and Let/Match environment copies while preserving legacy semantics through one shared implementation.

- [x] Refresh W AGENTS and read-only status; inspect full original checker/tests, actual AST, SourceLocation, R192 source/context and R120/R181 phase contracts.
- [x] Confirm unused R200; freeze complete preimages and the API/error/location/depth convention.
- [x] Write independent AST normal/boundary/error-precedence test SOURCE first; retain original tests byte-for-byte as a prefix.
- [x] Add the Budget diagnostic and bounded entry; share original traversal with legacy via optional internal extent. Guard roots before parameter environment work, and every child before path/environment copies.
- [x] Inspect the full exact diff/recursion/copy sites and freeze final source/API hashes. These are source observations, never a security approval or execution result.
- [x] Supply Root a separate diagnostic/public-call recipe and remaining premises. Fresh designated Opus/Gemini review must precede all candidate execution.

Chosen mechanism: a private optional-depth policy returns an admitted child depth before constructing its path. Alternative duplicated bounded checker was rejected because it would drift; a preflight walk was rejected because it would reorder normal errors and duplicate traversal. Legacy uses None without any synthetic numeric cap or depth-counter arithmetic. Each bounded root independently starts at1.

Files: afterimages/crates/mori-core/src/sherlock/draft_typecheck.rs and draft_typecheck_tests.rs. Only runtime R200 artifacts are written by this author. Root owns W/Git/diagnostics/integration/checks.

Falsifiable predicate: exact/first-over/root reset and every real Expr branch follow API.md, while sufficient-budget results/errors/paths/order match independently specified original normal cases. No compiled or executed check is claimed; required source-first tests remain UNRUN until designated review and Root selection.
