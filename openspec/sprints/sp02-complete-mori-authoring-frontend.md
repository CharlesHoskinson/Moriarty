# SP02: Complete .mori authoring frontend implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Let a developer write, check and format the successor language with stable diagnostics.

**Architecture:** This sprint contributes to MC01, MC08; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js 24 (existing native TypeScript source execution), `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP01. Stage scope: successor-frontend. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-language/spec/successor/lexical.md` | Tokenization, encoding and source locations |
| create | `experiments/moriarty-language/spec/successor/grammar.ebnf` | Complete ISO/IEC 14977 source grammar |
| create | `experiments/moriarty-language/spec/successor/static-semantics.md` | Typing, scoping, effects and resource judgments |
| create | `experiments/moriarty-language/spec/successor/bounds.json` | Newly registered immutable successor bounds |
| create | `experiments/moriarty-language/spec/successor/examples/loan.mori` | Executable successor authoring fixture |
| create | `experiments/moriarty-language/src/successor/frontend.ts` | Separate profile frontend |
| create | `experiments/moriarty-language/src/successor/format.ts` | Canonical source formatting |
| create | `experiments/moriarty-language/src/cli.ts` | Profile-explicit developer CLI |
| create | `experiments/moriarty-language/tests/successor-frontend.test.mjs` | Lexer, parser, types, diagnostics and format controls |
| modify | `experiments/moriarty-language/package.json` | Checking/formatting CLI and verification entry points |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP02.1: Finish the source specification

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP02.md`.
- [ ] Document BNF production rules, ISO/IEC 14977 EBNF and RFC 5234 ABNF in the specification introduction. Select EBNF for the source grammar. Specify identifiers, literals, Unicode/UTF-8 policy, whitespace, comments, escapes and byte limits separately using regular expressions or a small lexical grammar. Resolve // and nonnested block comments, Boolean precedence and source spans. Publish every production and reserved word. Compare the three matched syntax specimens in the research dossier on authoring and debugging tasks.
- [ ] Verify: Every source construct has a defined production and lexical tokens. Check exact ISO/IEC 14977 notation, parser/grammar agreement and valid/invalid examples. No undefined production or ambiguous update interpretation remains. Record a formative developer evaluation; model opinions alone are not empirical usability evidence.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP02` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP02.2: Implement versioned parsing and checking

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP02.md`.
- [ ] Keep existing frontend.ts signatures and registered atomic bytes stable. Add a successor API selected by explicit profile hash. Implement typed assets, nominal amounts, shares, rates, times, bounded collections and obligation/request identities from SP01. Reject source recursion, unbounded loops and unsupported operations.
- [ ] Verify: Positive loan, swap, partial payment and request programs check. Mixed asset arithmetic, duplicate next writes, next reads, invalid post placement and excessive depth reject with stable spans.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP02` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP02.3: Deliver formatter and checking CLI

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP02.md`.
- [ ] Implement check and format commands using the same frontend. Specify stdout JSON, stderr diagnostics and exit codes before implementation. Add explicit source locations and profile hashes. Specify the simulation input/output contract for SP03, without claiming an evaluator exists.
- [ ] Verify: Formatting preserves parsed meaning and is idempotent. CLI/API results agree. Comments cannot alter signed Core meaning; unknown profiles reject. Simulation acceptance belongs to SP03.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP02` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-language run build
npm --prefix experiments/moriarty-language test
node experiments/moriarty-language/src/cli.ts check --profile experiments/moriarty-language/spec/successor/bounds.json experiments/moriarty-language/spec/successor/examples/loan.mori
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

Complete lexical/EBNF/static specification and working successor frontend/formatter/CLI, with current audits. Source syntax alone does not close MC01-EXTENSIONS.
