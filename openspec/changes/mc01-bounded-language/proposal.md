# Change: Bounded language and semantic contract

## Why

Define and implement the Moriarty authoring language over a finite typed Core.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze the language profile.
- Implement the frontend.
- Implement semantic elaboration.
- Implement the initial Compact mapping.

## Impact

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
- `experiments/moriarty-language/examples/loan.moriarty`
- `experiments/moriarty-language/examples/swap.moriarty`
- `experiments/moriarty-language/tests/frontend.test.mjs`
- `experiments/moriarty-language/tests/semantics.test.mjs`
- `experiments/moriarty-language/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC01/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Opus and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
