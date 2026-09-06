# R2b Outcome Intents Implementation Plan

> Use superpowers:subagent-driven-development for isolated module tasks, parent integration and independent final review.

**Goal:** sign bounded outcomes before selecting a financial plan, check authority,
and simulate atomic single-use execution in the developer workspace.

**Architecture:** `outcome.ts` owns canonical intent/signature/independent trace
checks; `outcome-runtime.ts` owns trusted R2 agreement evaluation, shared balances
and atomic local consumption; `outcome-app.ts` owns the browser flow. Existing
R2 modules remain the finite financial evaluator and exact-plan comparison.

**Tech Stack:** existing TypeScript7, Node24, WebCrypto and Python Playwright.
No new runtime dependency or backend service.

## Global constraints

Use the [design](../specs/2026-09-06-r2b-outcome-intents-design.md) types and bounds
verbatim. No callbacks/Core extension, supplied-after-state trust, key self-trust,
netting away gross spend, eviction of consumed nonce, or real-proof success stub.
Preserve previous evidence and unrelated main work. No native proof run.

## Tasks and interfaces

- [x] 1. `src/language/outcome.ts`, `tests/outcome.test.mjs`: export all design
  types plus `readIntent(unknown):IntentIR` (throws on malformed),
  `hashIntent(unknown):Promise<string>`, `signIntent(IntentIR,CryptoKeyPair):Promise<SignedIntent>`,
  `verifyIntent(unknown,TrustContext,now:string):Promise<Checked|Rejected>`,
  `renderIntentSummary(unknown):string`, `assetKey(AssetId):string`,
  `checkOutcomeTrace(intent:unknown,before:unknown,effects:unknown,after:unknown):TraceChecked|Rejected`.
  `Checked={outcome:'checked'}`; `Rejected={outcome:'rejected',code,message}`;
  `TraceChecked={outcome:'checked',grossDebits:{asset:AssetId,amount:string}[],netCredits:{asset:AssetId,account:string,amount:string}[]}`.
  Tests independently construct balances/traces: 10 A debit cap across two
  recipients rejects 6+5 even with refund; min3 B rejects receipt3 minus fee1;
  valid positive, exact expiry boundary, changed required claims, wrong trust,
  malformed/oversized encoding and altered structured asset identity.
- [x] 2. Parent implements `src/language/outcome-runtime.ts`,
  `tests/outcome-runtime.test.mjs`. Export `createOutcomeDemo('swap'|'loan')`
  returning `Promise<{runtime:OutcomeRuntime,intent:IntentIR}>`;
  runtime methods `snapshot()`, `advanceTime(now:string)`,
  `propose(intentHash:string,route:number=0):Promise<PlanIR>`,
  `preview(signed:unknown,plan:unknown,trust:TrustContext)`,
  `simulate(signed:unknown,plan:unknown,trust:TrustContext)`,
  `verifyRealAcceptance(signed:unknown,plan:unknown,trust:TrustContext)`.
  `snapshot()` returns `{now:string,balances:Balance[],consumed:number,agreements:{id:string,programHash:string,state:CoreState}[]}`.
  Preview returns `Rejected` or `{outcome:'checked',receipt:LocalOutcomeReceipt}`;
  simulate returns `Rejected` or `{outcome:'simulated',receipt:LocalOutcomeReceipt}`;
  real acceptance returns `Rejected` or `{outcome:'unavailable',missing:string[]}`.
  Receipt fields: `kind:'LocalOutcomeReceipt',status:'eligible-local-simulation'|'simulated-complete',intentHash,planHash,beforeHash,afterHash,effects:AssetEffect[],dueUpdates:Effect[],grossDebits,netCredits,authorityStatus:'unconsumed'|'one-shot-consumed',proofStatus:'unavailable'`.
  Tests exercise actual compiled pool/loan, fresh program/state anchors, same
  signed outcome with two routes, two simultaneous simulate calls (one winner),
  replay with a different re-signed intent, stale shared balance and expiry.
- [x] 3. `intents.html`, `src/outcome-app.ts`, `tests/outcome_browser_check.py`,
  additive `styles.css` and `server.mjs` routes, small navigation links from
  original and `/language`. Consume task2 interface, task1 signing/summary and
  existing `generateLocalKey`. Keep all callbacks local; invalidate async and
  displayed evidence at edits. Expose editable intent/plan, route choices,
  local clock, signature, preview, simulation, missing claims and export.
  Browser controls demonstrate retained signature when switching plans,
  rejection of stricter limits, expired/replayed plan, and private-key-free export.
- [x] 4. Parent freezes held-out source cases, records design/evidence/wiki,
  integrates tests, requests one independent whole-change review and fixes
  material findings. Run fresh build + all Node tests + browser smoke; hash the
  tested files and preserve authentic failures. Commit and fast-forward local
  main preserving unrelated changes; checkpoint R3/R4/R5 remaining scope.

Each implementation task first records a meaningful failing control, then its
focused passing tests. Build before compiled-module tests. Final commands from
`experiments/moriarty-developer-mock`:

```sh
npm run build
npm test
MORIARTY_OUTCOME_URL=http://127.0.0.1:4175 python tests/outcome_browser_check.py
```

Use `/home/charl/Moriarty/.venv/bin/python` for available Playwright. The parent
reviews task interfaces and changing source; the final reviewer is independent
of implementation. User authorization already covers this sprint; routine module
choices do not require another approval cycle.

Implementation and review complete. Results and limits are recorded in the
[R2b evidence](../../../evidence/moriarty-r2b-outcomes-2026-09-06/README.md).
