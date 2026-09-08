# SP05.1 corrected utility review

**BLOCKED.** Candidate `c721b008c941dcc629b39ff3862841e0047641d1b601d8de8c6277e00b9d35dd` (197585 bytes). Scope is the fixed synthetic financial comparison utility only.

The 45 existing tests pass independently. The original numeric contradictions now reject, malformed nested JSON returns structured results, and exact new RED/GREEN captures verify. Two remaining collection-coverage defects allow equal invalid records to pass.

## C1: Additional loan dues and residual duties bypass fixed-case financial validation

requireDue and requireDuty check required IDs but never reject additional IDs. Loan accrue/settle do not require exactly the two admitted due records; residual duties lack an exact identity/cardinality check. Equality against an equally invalid expected record therefore launders extra obligations.

Locations: `experiments/moriarty-midnight-financial/src/differential.mjs:943`, `experiments/moriarty-midnight-financial/src/differential.mjs:956`, `experiments/moriarty-midnight-financial/src/differential.mjs:1081`, `experiments/moriarty-midnight-financial/src/differential.mjs:1197`.

A third principal due has outstanding999 after settlement while principal.due=0, interestDue=0, both admitted dues outstanding0 and nominalRemaining4500000000. Borrower/lender are valid existing role identities; no malformed or unbound-role prerequisite is needed.

```js
const x = structuredClone(loan); const s = x.stages.find(s => s.stage === 'settle'); s.liabilities.dues.push({ ...s.liabilities.dues[0], id: 'rogue-due', dueId: 'rogue-due', created: '0', settled: '0', outstanding: '999' }); compareFinancialEffects(x, structuredClone(x));
```

Actual: `{"ok":true,"errors":[],"networkAcceptance":false,"networkEvidence":"incompleteNetworkEvidence"}`.

The record claims a second open nominal-agreement-debt999 in addition to the fixed4500000000 remaining-notional duty, yet nominalRemaining stays4500000000.

```js
const x = structuredClone(loan); x.stages.find(s => s.stage === 'settle').residualDuties.push({ id: 'rogue-duty', kind: 'nominal-agreement-debt', amount: '999', unit: 'USD_micro', status: 'open' }); compareFinancialEffects(x, structuredClone(x));
```

Actual: `{"ok":true,"errors":[],"networkAcceptance":false,"networkEvidence":"incompleteNetworkEvidence"}`.

Required correction: Validate exact stage-specific due and residual-duty identity sets independently on both inputs. Loan setup dues must be empty; accrue/settle must contain exactly PR/IP with derived allocations; each stage has exactly its required remaining-notional duty. Reject extra positive or zero obligations and reconcile all admitted due totals. Add equal-input and expected-only/observed-only regression controls with the required due/residual error path.

## C2: Extra participant balances/effects and transfers escape the closed financial coverage checks

checkCoverage only enforces required IDs as a subset; it does not reject extra IDs or resolve every row actor/asset/unit against the closed binding map. requireTransfer finds known transfers without enforcing exact stage transfer coverage. Generic local conservation does not establish the fixed case supply or identity domain.

Locations: `experiments/moriarty-midnight-financial/src/differential.mjs:434`, `experiments/moriarty-midnight-financial/src/differential.mjs:822`, `experiments/moriarty-midnight-financial/src/differential.mjs:932`, `experiments/moriarty-midnight-financial/src/differential.mjs:986`, `experiments/moriarty-midnight-financial/src/differential.mjs:1269`.

The closed role map contains only trader/pool/provider/setup-mint, but all three stages can carry attacker ASSET_A balance999 and an attacker effect row. This also adds999 to the fixed1100000A supply outside any declared setup mint. Stage continuity and local transfer conservation both hold, so the missing fixed-domain checks matter.

```js
const x = structuredClone(swap); for (const s of x.stages) { for (const side of ['pre', 'post']) s.balances[side].push({ id: 'attacker|ASSET_A|AssetA_quantum', actor: 'attacker', asset: 'ASSET_A', unit: 'AssetA_quantum', amount: '999' }); s.actorEffects.push({ id: 'attacker|ASSET_A|AssetA_quantum', actor: 'attacker', asset: 'ASSET_A', unit: 'AssetA_quantum', grossDebit: '0', refund: '0', netCredit: '0', fee: '0' }); } compareFinancialEffects(x, structuredClone(x));
```

Actual: `{"ok":true,"errors":[],"networkAcceptance":false,"networkEvidence":"incompleteNetworkEvidence"}`.

Close accepts a third transfer ordinal2 from trader to provider amount0, despite the source-defined close returning exactly two pool-to-provider reserve transfers. This is a secondary closed-effect-coverage witness; no claim of monetary theft from the zero transfer.

```js
const x = structuredClone(swap); x.stages.find(s => s.stage === 'close').transfers.push({ id: 'zero-extra', ordinal: '2', from: 'trader', to: 'provider', color: 'ASSET_A', unit: 'AssetA_quantum', amount: '0' }); compareFinancialEffects(x, structuredClone(x));
```

Actual: `{"ok":true,"errors":[],"networkAcceptance":false,"networkEvidence":"incompleteNetworkEvidence"}`.

Required correction: Require exact admitted participant/asset balance and actor-effect identity sets for every stage; validate every financial actor/asset/unit reference against declared bindings. Keep setup-mint as the sole explicit funding exception. Require the exact finite transfer specification for setup/settle/swap/close and no transfers for accrue; reject extra records even when amount0 or both inputs agree. Preserve legitimate order-insensitive comparison only where the record schema explicitly treats a collection as indexed.

## Original findings

- **R1:** PARTIALLY_RESOLVED. Exact closed role-map fields and source-pin values now validate/compare. Extra financial rows can still reference roles outside that map (C2).
- **R2:** RESOLVED for exercised ordinary JSON cases: structured failure, deterministic errors and no input mutation under original controls plus2210 container probes; no universal claim for arbitrary JavaScript proxies/getters or unbounded resources.
- **R3:** PARTIALLY_RESOLVED. Original contradictory scalar/erased-duty/fee/work examples reject;313 numeric scalar increments all reject. Additional loan obligations still pass equal-input validation (C1).
- **R4:** PARTIALLY_RESOLVED. Missing required balances/effects, duplicate ordinals/role names and zero quantum reject. Exact collection coverage and all role references remain incomplete (C1,C2).
- **R5:** RESOLVED. New exactUTF8stdout/stderr digests and byte counts match; historical mismatched record and strings are preserved exactly with explicit unverified interpretation. Independent45pass rerun confirmsGREEN current behavior.

## Independent evidence

- 45 tests pass; no skips or cancellation.
- 2,523 adversarial numeric/container calls: all expected rejections, no exception, input mutation, network claim or nondeterministic result.
- Four extra-record probes and a fifth valid-role extra-due probe reproduce the false acceptances above.
- All six fixture stages reconcile by independent integer arithmetic and transfer replay. Loan retains4500000000 notional; swap fee 30 remains in the pool and closure pays exact reserves.
- New RED stdout 25692 bytes/hash `88d72f59836773cbefa50197158ecc174db88760790e7c2c88861fd2fa6127d7`; GREEN stdout 6893 bytes/hash `7ecadc5351e5137c803cebc80420a1a5095b4bb9047249fe9cf73e738a848881`. Both stderr captures are empty and hash correctly. Historical mismatched evidence is preserved exactly and labeled unverified.
- All nine frozen owned files and six pinned inputs match before/after.

One inline reviewer Python attempt had a syntax error; its corrected rerun passed. All other attempted commands exited 0. No source/tests were modified, and no build, wallet, network, proof, git change or subagent was used. Full command inventory and checks are in `gpt6-review2.json`.

Symbolic role/asset IDs and absence of raw receipt decoding remain stated scope limits. This report accepts no ledger settlement, finality, Preview, proof, language or whole SP05 gate.
