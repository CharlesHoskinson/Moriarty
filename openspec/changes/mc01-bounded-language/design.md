# Design: Bounded language and semantic contract

## Inputs and dependencies

Dependencies: Existing approved target-first design and retained evidence.

- `docs/superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md`
- `docs/research/2026-09-06-intents-report-integration.md`
- `experiments/moriarty-developer-mock/src/language/core.ts`
- `experiments/moriarty-developer-mock/src/language/outcome.ts`

## Interfaces

parse(source) -> Result<AgreementAST, Diagnostic[]>; check(ast, profile) -> Result<TypedAgreement, Diagnostic[]>; elaborate(typed) -> CoreProgram; evaluate(program, state, action, authority, observations) -> Rejected | Complete | Pending. All wire encodings carry schema and semantic versions.

## Outputs and ownership

- `experiments/moriarty-language/spec/grammar.ebnf`
- `experiments/moriarty-language/spec/numeric-profile.json`
- `experiments/moriarty-language/spec/semantics.md`
- `experiments/moriarty-language/spec/bounds.json`
- `experiments/moriarty-language/src/ast.ts`
- `experiments/moriarty-language/src/parser.ts`
- `experiments/moriarty-language/src/typecheck.ts`
- `experiments/moriarty-language/src/elaborate.ts`
- `experiments/moriarty-language/src/evaluate.ts`
- `experiments/moriarty-language/src/codec.ts`
- `experiments/moriarty-language/src/lower-compact.ts`
- `experiments/moriarty-language/src/errors.ts`
- `experiments/moriarty-language/examples/loan.mori`
- `experiments/moriarty-language/examples/swap.mori`
- `experiments/moriarty-language/tests/frontend.test.mjs`
- `experiments/moriarty-language/tests/semantics.test.mjs`
- `experiments/moriarty-language/package.json`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable 5.1 at medium effort and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
