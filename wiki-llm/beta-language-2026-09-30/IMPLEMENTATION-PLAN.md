# Moriarty beta implementation plan

> For agentic workers: use focused subagent development and independent review.
> The user's AFK instruction selects execution now, without another question.

Goal: ship a standalone authoring beta and truthful local S0 preparation.
Architecture: one strict text frontend feeds existing Source/6 and Core/5; CLI,
LSP and MCP consume the same library. Horizon examples preserve the full language.
Stack: Node >=24, TypeScript 7.0.2, esbuild 0.28.2, @types/node 26.6.3.

## Global constraints

Apply BETA-DESIGN plus CONVERGENCE v2. No signature, proof, ledger or provider
qualification. No changes to old source profiles/Core judgments. Source/scenario
text only at public interfaces. All finite budgets and financial nominal rules
in CONVERGENCE apply. Other-family operations are recognized authoring only.
Each worker owns named files; independent components may run in parallel after
their interface is frozen. Root integrates, reviews and runs actual artifacts.

## Interface contract

`packages/moriarty-beta/src/frontend.ts` exports:

```ts
type Span = { start: number; end: number }; // UTF8 bytes
type Diagnostic = { code: string; message: string; span: Span };
type Value = { span: Span } & (
 | { tag: 'string'; value: string } | { tag: 'bool'; value: boolean }
 | { tag: 'scalar'; value: string } | { tag: 'qty'; asset: string; atoms: string }
 | { tag: 'tag'; name: 'None' | 'SuccessOnly' }
 | { tag: 'array'; items: Value[] } | { tag: 'record'; fields: Record<string,Value> }
 | { tag: 'call'; name: string; args: Record<string,Value> }
 | { tag: 'entity'; kind: string; name: string; fields: Record<string,Value> });
type Declaration = { kind: string; name: string; span: Span; nameSpan: Span; value: Value };
type Action = { name: string; intent: string; span: Span; nameSpan: Span;
 support: 'LocalS0' | 'SpecifiedOnly' };
type Analysis = { status: 'AuthoringChecked' | 'AuthoringRejected'; sourceHash: string;
 diagnostics: Diagnostic[]; agreement: string | null; declarations: Declaration[];
 actions: Action[] };
function analyze(source: string): Analysis;
function check(source: string): object; // JSON-safe summary, no expanded values
function format(source: string): {text: string|null; diagnostics: Diagnostic[]};
function byteToPosition(source: string, byte: number): {line:number;character:number};
```

Internal values are immutable declaration DAG references, strings for atom/scalar
integers. Entities use declaration names for reference, economic IDs from fields.
No public caller-built Analysis reaches lowering: bridge always re-analyzes text.

`bridge.ts` exports `expand(source:string, action:string, scenario:string)` and
`simulate(source:string, action:string, scenario:string)` JSON-safe objects with
named status/diagnostics, source/scenario digests, no partial artifact on failure.
Expand success includes `source6`, `fieldMap`, `origins`; simulate adds unchanged
Source/6 wrapper result. `index.ts` re-exports public check/format/inspect/expand/
simulate. `inspect` includes identity claims/operative bounds/support/open gates.
`servers.ts` exports `runLsp()` and `runMcp()`; CLI routes stdio commands only.

## Task 1: strict frontend and formatter

Own frontend.ts and tests/frontend.test.mjs. First create failing tests for exact
decimal Qty, nominal mismatch, forward/duplicate refs/keys, economic ID conflicts,
schema closure, recognized horizon registry, resource bounds, UTF16 conversion,
comment-preserving idempotent format and strict incomplete edits. Run with
`node --test packages/moriarty-beta/tests/frontend.test.mjs` and record failure.
Implement bounded lexer/parser/evaluator, one DAG value per declaration, sealed
S0 intent/operation validation and field-specific ranges. Name all diagnostics.
Registry schemas come from CONVERGENCE with fixed named args; publish coverage.
Run tests and root strict tsc. Hand off exact exports above and test evidence.

## Task 2: scenario closure and Source/6 bridge (root)

Own json.ts, bridge.ts, index.ts and tests/bridge.test.mjs. First tests use
independently authored beta+Source6 transfer/repay fixtures and explicit expected
effects/post/work/replay. Include raw duplicate escaped keys, unknown/missing
scenario inputs, overpay (31 vs debt30), expired+overpay, receiver overflow,
stale/replayed head, fee/cap changes, explicit missing fee candidate, zero fee.
Run failing tests, implement closed bounded duplicate-safe JSON and exact emitter.
Proposal builder is total; no Core acceptance clone. Mark overpay placeholder.
Every emitted line has source/scenario/derived origin and input digests. Preserve
Core first judgment/code and all four premises/unverified bindings. Run bridge
tests and existing S0 tests after integration.

## Task 3: stdio language and AI services

Own servers.ts, protocol.ts, tests/protocol.test.mjs, editor VSCode/Nvim assets,
AI adapter packs. Test framing/oversize/split chunks, real subprocess initialize/
open/update/close/format/definition, Unicode positions and stale versions; MCP
client initialize/tools/list/check/inspect/expand/preview and invalid args.
Write failing tests before handlers. Shared check/format/inspect/expand/simulate
only; never caller AST/shell/path/network/signing. Fixed frame/doc/output bounds.
Provide truthful capabilities, TextMate grammar/snippets and VSCode stdio client.
Package lexical extension and test actual Neovim LSP. Provider instructions inert,
project-scoped; actual provider/VSCode activation unperformed unless observed.

## Task 4: CLI and distributable project (root)

Own cli.ts, starter.ts, package.json, build.mjs, tsconfig.json, README.md and
tests/cli.test.mjs. First subprocess tests check JSON/nonzero invalid flags,
fmt stdout/write, init no overwrite, finite case runner, expand/simulate. Init
writes .mori source, closed JSON scenario and case manifest. `test DIR` reads
only finite contained files/closed cases, runs same public text APIs, asserts
status/code and requested complete expected results. No fake success fallback.
Bundle library and CLI (S0 modules included); imports have no CLI side effects.
Pin dev tools, build, npm pack, clean install outside checkout, run CLI/API without
global TS/repo metadata. Print required unqualified premises in success results.

## Task 5: complete mockup, evidence and independent review

Own horizon docs/research synthesis separately from code workers. Complete all
audit04 findings with typed proposed syntax explicitly unsupported in beta. Freeze
candidate implementation hash and obtain fresh whole-code independent reviews,
repair all high/medium defects and re-review relevant exact bytes. Run package
typecheck/tests, full existing language regressions, clean install, stdio clients,
real Neovim and VSIX metadata/package checks. Record actual counts/commands/
digests/limitations. Publish reviewed branch per existing user authorization;
preserve external acceptance gates and original audits.

## Progress

- [x] Nine domain research threads and five PL design audits.
- [x] Convergence amendments selected; final concurrence recorded in audits.
- [x] Frontend, exact nominal arithmetic and bounded S0 authoring checks.
- [x] Source/6 bridge and strict local scenarios.
- [x] CLI/distribution and stdio/editor/AI tooling; actual VS Code/provider activation remains unperformed.
- [ ] Complete typed horizon and requirements matrix.
- [ ] Independent code audits, repair, fresh verification and publication.
