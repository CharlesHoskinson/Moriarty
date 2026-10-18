# Nominal text source

## Requirement: Explicit nominal text preparation
WHEN a caller selects the nominal text header, the parser SHALL produce the actual
ExtProgram source carriers and preserve type, parameter, constructor, field,
function, expression occurrence, nested pattern and match-arm source order.

### Scenario: Recursive ordered Schedule
- GIVEN Schedule with Done, Pay and Both and nested constructor patterns.
- WHEN text is parsed and passed to the existing owner type preparation API.
- THEN nominal uses retain their explicit TypeExprSrc/constructor spelling and
  source order, and the existing owner produces the resolved type-use tables.

### Scenario: Forward mutual references and generic application
- GIVEN mutually referring declarations and Box<T> with Box<UInt> signatures and uses.
- WHEN nominal text is parsed.
- THEN parameter references are Param only within their declaring binder scope,
  applications preserve all arguments, and no forward reference is substituted,
  reordered or evaluated.

### Scenario: Existing source versions
- GIVEN an ordinary Core or dedicated draft/1 program.
- WHEN its existing parser is used.
- THEN its existing grammar, budgets, errors and profile selection remain unchanged.

Preparation is not generic runtime admission. Existing owner policies, H1 limits,
security review and broader original034/refinement obligations remain separate.

## Requirement: Explicit existing parser extents
WHEN a caller supplies DraftParseBudget to nominal text preparation, the parser
SHALL enforce its existing source/token/function/signature/Expr+Pattern/descent/
Expr-structural meanings before the associated work, including descent through
nominal type applications and nested constructor patterns, without unbounded fallback.

### Scenario: Exact caller extent and next attempted unit
- GIVEN a normal text input and explicit inclusive parser extents.
- WHEN one extent equals the needed count or is one unit smaller.
- THEN the exact boundary parses or returns the existing typed Budget resource,
  caller limit and first attempted count; no parser refusal becomes an owner outcome.

No default release extent or generic compilation/admission capability is implied.
