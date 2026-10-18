# Caller-bounded nominal text source

New public module remains `mori_core::sherlock_nominal_source`.
`parse_nominal_source_bounded(&str, DraftParseBudget)` returns the actual owned
`ParsedNominal { name, program: ExtProgram, provenance }`.
`prepare_nominal_source_types_bounded(&str, DraftParseBudget, &LayoutSchemaC,
ExtDomain, Option<LocatorC>)` first calls that bounded parser, then invokes the
actual public `shared::prepare_ext_program_types`, retaining the actual owner
result in `PreparedNominalText`. It never falls back to the unbounded entry.
The two previous unbounded APIs remain explicit H1 APIs; their grammar is unchanged.

The same eight existing caller fields/resources are used, with no defaults:
source bytes before lexing; EOF-inclusive token limit in the real lexer; function
count; per-function and total signature parameters before growth; each syntactic
Expr and Pattern occurrence before construction; active parser descent before
entry (now including nominal type applications/nested patterns); and Expr
root-to-leaf structural depth before parent construction. Parentheses add descent,
not an Expr. Provenance growth rechecks existing occurrence/depth extents.
Type parameters do not become function parameters; declarations/types/constructor
fields do not become Expr nodes. Their syntax consumes bounded tokens, and type
nesting consumes the same caller parser descent. No separate nominal declaration,
constructor, field or type-occurrence policy/default exists in the adopted contract;
any additional release cap is an explicit owner decision, not fixture policy.

Existing lexer hard limits remain 65,536 source bytes and 8,192 tokens; a larger
caller value does not bypass them. These limits are existing source facts, not
new chosen release defaults. The new parser preserves all prior nominal AST,
identity/name spellings, declaration/field/arm order, qualification and source
provenance. Existing Core/draft parser, owner, EF/runtime and witness bytes are
unchanged; the existing-source integration is only the proposed public module
registration in Core lib.rs.

The three normal text fixtures/tests now consume the bounded parser and bounded
owner-preparation wrapper with explicit test-local extents. A separate five-test
module checks exact finite source/token/AST/structural/descent/signature boundaries
and nested nominal pattern occurrence counting. Under the user's standing parser
exception these are Sol-authored parser tests, pending fresh Opus/Gemini review.
They are not owner-admission or malformed-Witness guard tests. All eight tests,
source compilation and runtime are UNRUN.

## Actual public compile boundary

Current `compile_artifact::compile_draft_host` calls
`compile_artifact/recursive_source::prepare_checked_draft_bounded`, which parses
four-sort `Arc<Program>`, invokes `check_program_types_bounded`, then bounded-copies
that AST through `source_input_bridge` and fixed B admission. Its extents come
from actual `draft_contract::DraftPreprocessingExtents::parser_budget` with no
chosen values. Current `ProgramView` still has twelve fields and no nominal
constructor/type/match-plan arrays. The installed ExtProgram preparation returns
separate type/T3 tables and does not produce that admitted view. This batch supplies
real bounded public text-to-owner preparation, not generic public CLI compile,
B admission/evaluation, MatchPlan execution, witness or constraint export.
Connecting those remaining carriers/continuations is genuine subsequent production
work; it is not a reason to relabel preparation as successful compilation.

Owner Pass-B/T memory/cleanup/refusal ordering, whole-process H1, hostile generic
admission and full original034/refinements remain separate. No provider, compiler,
test, source import/evaluator, model, SDK/native, W or Git action was performed.
