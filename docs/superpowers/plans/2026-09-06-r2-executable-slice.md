# R2 executable agreement slice implementation plan

> Use superpowers:subagent-driven-development for bounded independent work and review.

**Goal:** execute one loan episode and AMM epoch through the same finite Core,
check signed intent/effects independently and expose it in the developer UI.

**Architecture:** TypeScript modules under the existing local experiment; typed
JSON agreement elaboration, generic evaluator, separate policy and claim modules.
No runtime dependencies. Node24 and browser WebCrypto provide local signatures.

**Design:** [R2 decisions](../specs/2026-09-06-r2-executable-slice-design.md).
User “begin” authorizes this sprint. Preserve unrelated A4/A5 and session changes.

## Global constraints

- Finite checked UInt128 intermediates, source/AST/collection/step limits.
- No financial package tag in evaluator dispatch; no eval or host callbacks.
- No shared evaluator call inside the independent effect checker.
- Exact first-period quantization and reference discrepancy stay visible.
- Local signature validity, effect checks and PCD acceptance are separate results.
- Required missing proofs/certificates return unavailable, never verified.
- Bind program, domain, state, action, effects, policy and mandatory claim manifest.
- No native proof campaign; R3 is specified-only in this sprint.

## Tasks and interfaces

- [x] 1. Generic Core + package elaborator. Own `src/language/core.ts`,
  `packages.ts`, `tests/language.test.mjs`. RED cases: default swap19743,
  min19744 rejection; PR/IP accrues33972602 microUSD on old principal;
  settlement consumes dues and transfers533972602; wrong event/asset/actor,
  insufficient balance, zero division, intermediate overflow, unknown syntax,
  exhausted budget and rollback reject. Include another generic program to
  demonstrate the evaluator is not a product-specific dispatcher.
- [x] 2. Independent policy + canonical claims. Own `src/language/policy.ts`,
  `claims.ts`, `tests/policy.test.mjs`, `tests/claims.test.mjs`.
  RED cases: correctly executed wrong-recipient transaction rejected by policy;
  added fee/approval/hidden write/incomplete accounting; canonical encoding and
  hash vectors; signature/public-key substitution; altered state/domain/program/
  policy/claim mode; missing mandatory claim or unknown verifier rejects;
  valid signed local plan with missing PCD remains unavailable.
- [x] 3. Browser integration. Own `language.html`, `src/language-app.ts`,
  `server.mjs` route additions, minimal shared styles/link, browser smoke.
  Show editable JSON -> elaborated Core -> evaluated state/effects -> independent
  intent checks -> local signing -> required proofs unavailable. Advance only
  local simulation state, preserve fail-closed real acceptance.
- [x] 4. Inspect and specify native R3 experiment, then independent review,
  integration tests, docs/evidence/checkpoint and local main integration.

The module boundary is:

```ts
type Values = Record<string,string>;
type CoreState = {instance:string; revision:string; remaining:string; values:Values};
type Action = {name:string; args:Values};
type Effect = {kind:'Transfer'|'Fee'|'DueCreated'|'DueSettled'; fields:Values};
type Evaluation = {outcome:'evaluated'; before:CoreState; after:CoreState;
  effects:Effect[]; writes:string[]; steps:string} |
  {outcome:'rejected'; code:string; message:string};
// Core exports Program, these types and evaluate(program,state,action).
// Packages exports exampleSource('loan'|'swap'), elaborate(unknown),
// and defaultAction(bundle,state); elaboration yields
// {outcome:'elaborated',program,initialState,source,description} or rejection.
```

Each implementer writes meaningful failing tests before behavior, records the
failure, implements and runs that task's tests. Integration uses `npm test`,
`npm run build` and Python Playwright with the existing environment. Review
source/binding assumptions before treating a passing test as acceptance evidence.

## Intents amendment and completion scope

The [intents report amendment](../../research/2026-09-06-intents-report-integration.md)
adds R2b. This sprint implements the explicit exact-plan profile; general
outcome IntentIR, expiry/nonces and residual authority remain next work.
Gross caps count refunds as separate movements; net minimum credits include
fees. Required proof stubs remain unavailable. Independent review found and
corrected shared schema state, misleading quantization and stale UI panels.
The [R3 specification](../../../experiments/moriarty-native-ivc-r3/README.md)
contains the proposed financial transition, native interface and bounded run
contract. It was not executed. See the
[scoped evidence](../../../evidence/moriarty-r2-language-2026-09-06/README.md)
for actual checks and remaining obligations.
