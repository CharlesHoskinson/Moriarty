You are the user-selected Grok4.6 high implementation worker. Add the concrete regression tests now with Edit/Write. This is Moriarty. Parent completed source review, actual reproduction, isolated worktree, resource binding. No new planning or permission cycle. Latest user routing is Grok implements, GPT6 reviews.

OBJECTIVE: Tests-only phase for two confirmed frontend diagnostics defects. Own only /home/charl/Moriarty/.worktrees/sp01-frontend-grok/experiments/moriarty-language/tests/frontend.test.mjs and /home/charl/Moriarty/.worktrees/sp01-frontend-grok/FOREMAN_REPORT.md/.json. Do not edit any implementation. No shell, tests, Git writes, agents/providers, installs, web, wallet/native work.

CURRENT TEST FILE OPENING:
```js
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { parseSource, compile, canonicalEncode, canonicalDecode } from '../src/frontend.ts';
import {checkAndLower} from '../src/checker.ts';
import {validateSource} from '../src/validate.ts';
import {normalizeError,throwDiagnostic} from '../src/diagnostics.ts';
// INTERNAL NONADMITTED configurations isolate limit checks. No public API accepts these.
function lowerNonAdmittedTestConfiguration(input,boundsInput,validateManifest=true){
  try{const source=parseSource(input);const configured=JSON.parse(boundsInput);const error=validateSource(source,configured);if(error)throwDiagnostic(error);return {source,...checkAndLower(source,new TextEncoder().encode(boundsInput),validateManifest)};}catch(e){throwDiagnostic(normalizeError(e));}
}
const bounds = readFileSync(new URL('../spec/bounds.json', import.meta.url));
const prefix = 'agreement Generic profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000;';
const minimal = body => `${prefix} state closed: UInt128 = uint(0); observation now: UInt128; status episode closed_when closed == uint(1); status agreement no_remaining_notional; action run(actor: Text) { ${body} } }`;

test('complete parser rejects comments, plain division, chained comparison, trailing input and unknown syntax', () => {
  for (const body of ['//comment\nguard true, "x";', 'let x = uint(8) / uint(2);', 'guard uint(1) < uint(2) < uint(3), "x";', 'while true {}', 'let constructor = uint(1);', 'let x = uint(01);', 'let x = uint(-1);', 'let x = uint(1e3);']) {
    assert.throws(() => parseSource(minimal(body)), undefined, body);
```
EXACT REPRODUCTION INPUTS AND EXISTING API (parent ran successfully):
```js
import {readFileSync} from 'node:fs';
import {compile} from '/home/charl/Moriarty/.worktrees/sp01-frontend-grok/experiments/moriarty-language/src/frontend.ts';
const bounds=readFileSync('/home/charl/Moriarty/.worktrees/sp01-frontend-grok/experiments/moriarty-language/spec/bounds.json');
const status=`agreement Generic profile "moriarty-bounded-atomic/1" {
 lifetime 2; horizon 100;
 status episode closed_when closed == uint(1);
 state closed: UInt128 = uint(0);
 observation now: UInt128;
 status agreement no_remaining_notional;
 action run(actor: Text) { guard uint(1), "bad"; }
}`;
const ordinal=n=>`agreement Generic profile "moriarty-bounded-atomic/1" {
 lifetime 2; horizon 100; unit U;
 state closed: UInt128 = uint(0); observation now: UInt128;
 status episode closed_when closed == uint(1); status agreement no_remaining_notional;
 effect Transfer { asset: Text; from: Text; to: Text; amount: Amount; }
 policy p targets effect(run,${n},amount) { unit U; derivation ""; rounding none; remainder ""; comparison ""; proof "p"; }
 action run(actor: Text) { emit Transfer { asset: text("U"), from: arg.actor, to: text("recipient"), amount: amount(1,U) }; }
}`;
const cases={prematureStatusAndBadGuard:status,prematureStatusOnly:status.replace('guard uint(1)','guard true'),declaredStatusBadGuard:status.replace('status episode closed_when closed == uint(1);\n state closed: UInt128 = uint(0);','state closed: UInt128 = uint(0);\n status episode closed_when closed == uint(1);'),ordinal0:ordinal('0'),ordinal1:ordinal('1'),ordinalMaximum:ordinal('340282366920938463463374607431768211455'),ordinalOverflow:ordinal('340282366920938463463374607431768211456')};
for(const [name,source] of Object.entries(cases)){try{compile(source,bounds);console.log(JSON.stringify({name,result:'accepted'}));}catch(e){console.log(JSON.stringify({name,result:'rejected',diagnostic:e.diagnostic??{code:e.code,message:e.message}}));}}

```
ACTUAL BASELINE: prematureStatusAndBadGuard returns TYPE_MISMATCH stage6 incorrectly. prematureStatusOnly returns NAME_RESOLUTION stage5. declaredStatusBadGuard returns TYPE_MISMATCH stage6. ordinal0 compiles. ordinal1 and UInt128 maximum return POLICY_TARGET stage6. ordinalOverflow currently returns POLICY_TARGET stage6 incorrectly.

REQUIRED TESTS: Append tests with names starting `SP01 frontend`. Use compile and its error.diagnostic to assert code, stage, and exact primarySpan where required. Derive expected spans independently from ASCII fixture substring offsets, not production parser/diagnostic helpers. Preserve all existing tests. Use distinct helper names. Include:
1. Premature episode status plus later invalid guard must return NAME_RESOLUTION stage '5' at the full status declaration. Valid guard also returns NAME_RESOLUTION. Moving state before status changes bad-guard outcome to TYPE_MISMATCH stage '6'. Globally missing field with valid guard returns STATUS_RULE stage '6'.
2. Same controls for notional status. Use `status agreement remaining_notional principal;` and a later `state principal: Amount<U> = amount(1,U);` with unit U declared first. Ensure syntax matches current grammar if this spelling is wrong, by reading only that narrow production. Required expected diagnostics are unchanged.
3. Policy before and after its action: ordinal0 compiles; ordinal1 and 2^128-1 return POLICY_TARGET stage6; ordinal2^128 returns UINT_RANGE stage6 at exact `effect(run,ORDINAL,amount)` target span. Move entire policy declaration after action without changing meaning.
4. Policy after action with earlier independent guard uint(1) still returns earlier TYPE_MISMATCH stage6, even with invalid ordinal.
5. Policy after action with earlier uncovered Amount write still returns earlier POLICY_TARGET. Include state principal:Amount<U>, set principal=amount(1,U), and the overflowing emit policy.
6. An uncovered emit in another action or another nominal unit still produces its earlier POLICY_TARGET with an invalid ordinal in run. Do not accidentally create a stage5 reference or missing observation error.
7. A policy with an independently invalid target earlier than an overflowing target reports earlier POLICY_TARGET. Put `write(run,missing)` before overflowing effect target in same policy.

INTERFACES/CONSTRAINTS: Use registered bounds unchanged. Error field is e.diagnostic.code/stage/primarySpan. `compile` returning record is acceptance for frontend only. These fixtures do not prove evaluator settlement or Compact support. No blanket assert.throws without checking diagnostic for the distinguishing cases. Supply exact ASCII source builders. Preserve tests that show policy metadata may precede/follow actions.

VERIFICATION: Parent runs focused RED against unchanged source and freezes test digest. Parent then dispatches a separate source-only phase. Write concise FOREMAN reports explaining tests, no tests run by you, and any fixture adjustments. Finish within590 seconds.
