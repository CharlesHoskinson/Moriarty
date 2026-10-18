# Prepare nominal text without evaluation

Use the explicit header `language "moriarty-sherlock-nominal/1";`. Core and
`moriarty-sherlock-draft/1` retain their existing parsers and meaning.
The [Schedule fixture](../../../crates/mori-core/tests/fixtures/nominal-text-source/schedule.mori)
shows recursive declarations, qualified construction and ordered nested patterns.

Import `mori_core::sherlock_nominal_source` and supply every `DraftParseBudget`
field explicitly. Call `parse_nominal_source_bounded(source, budget)` to obtain
the owned `ExtProgram` and source provenance. To resolve declaration and source
use types, call `prepare_nominal_source_types_bounded(source, budget, layout,
domain, locator)` with your explicit `LayoutSchemaC` and `ExtDomain`. It parses
with that budget before invoking the existing owner preparation API; there is
no unbounded fallback. Preparation failures retain the owner error value.

Budget fields cover source bytes, EOF-inclusive tokens, functions, signature
parameters, Expr/Pattern occurrences, parser descent and expression structural
depth. Type application and pattern nesting consume parser descent; nominal
lists consume bounded lexical tokens. No release defaults are supplied. Parser
extents do not bound all owner allocations or establish process-wide H1 limits.

The new text path prepares types and source uses. It does not execute constructor
fields or matches, admit a generic B image, extend the public compile command,
or produce a witness, proof or native artifact. The existing dedicated compile
path remains separate.

The [actual evidence](../../results/sherlock-nominal-text-preparation-20261018/README.md)
records two successful compiles and eight selected tests. Three tests consume
real nominal text through owner preparation; five check finite parser-budget
boundaries. Their assertions are author-owned, not independent semantic oracles.
