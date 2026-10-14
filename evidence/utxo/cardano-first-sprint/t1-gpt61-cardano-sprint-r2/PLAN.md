# First Cardano contract implementation plan — revision2

> **For agentic workers:** Execute inline with `superpowers:executing-plans` after the parent's independent expected-outcome review. GPT-6.1 is the sole source writer; no subagent implementation, commits or publication work is delegated by this plan.

**Goal:** Deliver the existing source/5 escrow through real UPLC, local Cardano settlement and Cardano Preview settlement, with complete state/effects and original-pair race evidence.

**Architecture:** Keep the proven custody lowerer and bootstrap recipe. Introduce one checked production context and thread it through the actual caller's existing encoders, output-fee helpers, body evaluator, witness and readback consumers. Keep UNIT adapters callable for controls; remove the production incomplete refusal only when that actual consumer has no UNIT inputs and all measurements qualify.

**Tech stack:** Node24 ESM/TypeScript profile/5 evaluator, Aiken1.1.21, pinned Cardano CLI11/node11 toolchain, existing CML/Lucid ABI, existing receipt helper.

## Global constraints

- SPEC.md CS01–CS11 applies to every task. No source change during this handoff.
- Original source SHA00c9893ee5d2cbf41adb619bbe2a4b37e4e3f4de64889aa4567bbfe0174ec957 remains immutable; substitute only approved time guards.
- Profile/5, contract/4, lifecycle/1, Head13/Lifecycle9, bound48/debit49, post975/49/16, cap128 and nonempty-obligation fence remain unchanged.
- No node/genesis/keygen/funding/mint/sign/submit/query/latch before two fresh full exact-current compiler+ledger source approvals; independent result review follows actual execution.
- No CML WASM/dependency edits, new scheduler, new language features, old wallet reuse, private key publication or historical output overwrite.
- Run controls in a new owned directory with short consistent HOME/TMPDIR; capture source/input/tool hashes before each command.
- Parent owns branchutxo, commits, GitHub publication, immutable archives and reviewer launch. No new approval requests for routine approved repairs.

Paths below are relative to `/home/charl/Moriarty/.worktrees/eutxo-candidate-219`.

## File and interface map

| File | Change and actual consumer |
| --- | --- |
| `experiments/moriarty-eutxo/src/native-production.mjs` (new) | Narrow context binder and final-body measurement receipt, invoked by native-caller; no process scheduler |
| `src/native-caller.mjs` in the same experiment | runNativeCaller, buildApprovedFunding, assembleConsumedFinancial, measuredCoin, convergeFee, plutusCostUnits, rebuildWithUnits, recountSignedWitnessFee and terminal readback use checked production bindings |
| `src/native-runtime.mjs` | queryProtocolParameters, acquireBodyEvaluation, compareBudget and loadAuthority consume complete measured artifacts and full current admissions |
| `src/apply-head.mjs` | buildApprovedCashPolicy/applyHeadToCashPolicy expose exact current source/script/seed/application identities; reject stale cached bytes |
| `src/admit-custody.mjs`, `src/compile-custody.mjs`, `src/custody-time.mjs`, `src/posix-parity-harness.mjs` | Preserve authentic in-process capture/admission and restrict runtime substitution to admitted bounds; bind current roles/genesis/seed and permitted network domains |
| `src/native-body.mjs`, `src/native-integrate.mjs`, `src/output-fee.mjs` | Encode actual policies/scripts; verify actual witness keys; keep body hash/protocol/output identity receipts through cost/fee changes |
| `src/native-bootstrap.mjs`, `src/native-producer.mjs` | Connect approved bootstrap and producer role frame to actual queried roles; preserve recipe155 and query readiness fences |
| `src/native-financial.mjs` | Full expected frame includes current policy-qualified payout role/amount/identity and explicit Core amount projection; economic packet is a semantic oracle, not a local-network override |
| `src/native-chain-observer.mjs` (new) | Checked actual block inclusion/ancestry/rollback adapter; unit feeds never issue chain confirmation |
| `src/cli.mjs` | Add a proposed `cardano-t1 --request <json> --out <owned-dir>` entry point; declarative request only, no caller functions/proof flags |
| `tests/native-production.test.mjs` (new) | Full production-context/body measurement binding controls using injected transport explicitly labeled UNIT |
| `tests/cardano-t1.test.mjs` (new) | Exported caller/CLI ordering, refusal, finality and environment selection controls |
| Existing caller/body/consumer/time/custody/auditrepair/quantity tests | Retain current meaning and cover production integration; do not change expected outcomes to implementation convenience |

## Proposed concrete interfaces

These are specified interfaces, not implemented in417. Implement them in the narrow new module and actual consumers together.

```js
// Bound context is an in-process object issued only after actual capture/query/
// script application. Serialized records retain hashes for evidence, not authority.
bindProductionContext({ admission, auditBinding, source, cash, head, roles, protocol, era, genesis })
// -> { ok:true, context } or { ok:false, code, bodyBuilt:false }
// source: { path, hashes:{programHash,boundsHash,sourceHash,schemaHash,seedHash},
//           blueprintPath, blueprintSha256 }
// cash/head: { sourceHash, seed:{txId,index}, scriptPath, scriptSha256,
//              policyId, applicationSha256 }; head also { instanceId }
// roles: { funder,lender,borrower,issuer,head,cashSeed,mintFee,mintCollateral,
//          headFee,headCollateral,leftFee,leftCollateral,rightFee,rightCollateral,
//          cancelFee,cancelCollateral }; each input has txId,index,value,address,
//          queriedAtTip and queryArtifactSha256; each role has paymentVkh,address
// protocol: { path, sha256, genesisSha256, networkMagic, queriedAtTip,
//             maxTxExecutionUnits, maxTxSize, queryReceipt }
// era/genesis: { path, sha256, networkMagic, zeroTime, zeroSlot, slotLengthMs }

measureProductionBody({ context, kind, encodeAtFee, witnessKeys, outputSpecs, workDir })
// -> { ok:true, body, receipt } or { ok:false, code, submitBlocked:true }
// kind = cash-mint|head-open|release|refund|cancel
// body: { bodyFile,cborHex,cborSha256,txid,bytes,redeemers,outputs }
// receipt: { bodySha256,protocolSha256,scriptSha256s,networkMagic,genesisSha256,
//            queriedAtTip,costs:[{scriptHash,purpose,index,memory,steps}],
//            outputMinima:[{index,outputSha256,lovelace}],feeLovelace,
//            witnessKeyHashes,referenceScriptSize,commandReceipts,finalEvaluation }

assertProductionBody({ context, body, receipt, freshObservation, gate, signedEnvelope })
// -> { ok:true, submitEligible:true } or { ok:false,code,submitBlocked:true }
// gate=pre-sign|pre-submit. Nonserialized issuer metadata, canonical candidate/two-audit binding,
// freshObservation (≤5slot drift+protocol/era/input/ancestry equality), exact final body
// and at pre-submit signedEnvelope identity/complete witnesses are mandatory.
// Byte-perfect UNIT/serialized receipts always return submitEligible:false, ledgerExecution:false.
```

The CLI JSON names network target, source/cases/seed, owned output path, selected approved authority/resource files, current runtime adapter and role-key paths. It never contains private key bytes, a compile callback, `productionAdmitted`, `representableSpendBounds` or fabricated cost/proof objects. `local-devnet` uses the approved isolated bootstrap; `cardano-preview` consumes that network's checked node configuration and queried context. Record unresolved endpoint/resource dependencies instead of creating defaults.

### Task1: Bind the real source/scripts/network context (CS01–CS03)

**Files:** new native-production.mjs; native-caller.mjs; apply-head.mjs; custody-time.mjs; compile-custody.mjs; admit-custody.mjs; tests/native-production.test.mjs.

**Consumes:** authentic production admission from productionParityGate; compileGuardSibling output; buildApprovedCashPolicy and applyHeadToCashPolicy results; actual producer role queries.

**Produces:** bindProductionContext and current identity-carrying context used by Tasks2–5.

- [ ] Write caller-level negative controls for every substituted source/blueprint/seed/role/genesis/protocol/script and each UNIT policy; require no encode/sign/submit call. Add a positive structural emitter control with UNIT data and recorded transport, without issuing genuine production admission or submitting. Keep ledgerExecution false; production provenance admission itself remains negative-only offline and its genuine positive is Task5.
- [ ] Run the new controls red. Preserve all failures together with pre-command source/tool/input pins.
- [ ] Implement context equality and provenance checks, including actual file rehashes and policy/script CBOR hash agreement. Validate domain selection from the selected network's genesis/config; retain local `yaci-isolated-t1` historical semantics and give Preview a distinct bound instance/network domain. Do not use arbitrary caller domain strings.
- [ ] Thread the emitted sibling path/blueprint/hashes/time arguments and current applied head/Cash artifacts into the selected production encoder. Cash cached results require current script-byte pins, seed and tool identity; an old cached path is not an artifact measurement.
- [ ] Run the actual exported caller control: production ingredients must reach encodeUnsignedBody/encodeSpendBody, never UNIT_HEAD_POLICY/UNIT_CASH_POLICY or fixture spend script. Original source remains byte-identical.

Concrete rejection assertion (test helper `productionAttempt` is local to the new test file and invokes runNativeCaller with recorded UNIT transport; it is not exported production authority):

```js
const result = await productionAttempt({ substitution: 'cash-script-bytes' });
assert.equal(result.report.stages.body.ok, false);
assert.equal(result.calls.encodeSpend, 0);
assert.equal(result.calls.sign, 0);
assert.equal(result.calls.submit, 0);
assert.equal(result.report.ledgerExecution, false);
```

### Task2: Close body/protocol/cost/minimum-ADA/fee measurement (CS04)

**Files:** native-production.mjs; native-runtime.mjs; native-caller.mjs; output-fee.mjs; native-body.mjs; native-integrate.mjs; tests/native-production.test.mjs.

**Consumes:** Task1 context including frozen candidate/two canonical full audit records, current queried protocol/era/UTxO files and actual output specifications.

**Produces:** measureProductionBody, assertProductionBody and exact-body submission eligibility.

- [ ] Write red controls for swapped body after evaluation, mismatched cost script hash/purpose/index, missing output minimum, stale protocol, missing collateral witness, altered reference-script size, excessive signed size/exunits and fee nonconvergence.
- [ ] Make measuredCoin/convergeFee/recountSignedWitnessFee/plutusCostUnits receive the context's protocol path/provenance instead of OFFLINE_PROTOCOL_PARAMS. Measure all outputs with exact actual datum/assets; never multiply whole transaction bytes by a protocol rate to invent minimum ADA.
- [ ] Use the supported calculate-plutus-script-cost online command through plutusCostArgv, retain stdout/stderr/status and consumed cost rows. Match all required scripts/redeemers against decoded current CBOR, rebuild execution units, recalculate script-data hash, then reevaluate final bytes. A trace-only failure or1/1 supplied shortcut remains a refusal.
- [ ] Use existing bounded four-pass fee convergence, actual signing-key witness count and current reference-script fee input; reject drift. Changing fee, units, datum, assets, outputs or witness count invalidates prior body/cost/minimum-ADA receipt and requires a new checked final measurement.
- [ ] Repair acquireBodyEvaluation/compareBudget so successful measurement reports complete receipts and actual minima instead of permanent nativeBudgetMeasured false or unmeasured placeholders. Enforce current protocol caps on final signed bytes, not the unsigned estimate alone.
- [ ] Verify positives and all negative controls through runNativeCaller. Only then replace PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE with the authentic production path; offline structural positives always remain submitEligible:false and cannot demonstrate genuine submission eligibility; preserve refusal for every incomplete binding.

```js
const result = await productionAttempt({ substitution: 'body-after-cost' });
assert.equal(result.report.stages.budget.submitBlocked, true);
assert.equal(result.calls.sign, 0);
assert.equal(result.calls.submit, 0);
```

### Task3: Settle dependency stages and compare complete effects (CS03, CS05–CS07)

**Files:** native-caller.mjs; native-financial.mjs; native-runtime.mjs; native-producer.mjs; native-integrate.mjs; tests/cardano-t1.test.mjs; existing financial/race tests.

**Consumes:** checked context and exact measured bodies from Tasks1–2.

**Produces:** settled Cash input → settled head input → complete terminal result, original pair race receipt and public evidence records.

- [ ] Write red ordered-transport tests proving head opening cannot consume a predicted unsubmitted mint outref; a terminal cannot consume an unconfirmed head; each fee/collateral belongs to its actual selected role. Retain original transaction IDs and outcome receipts.
- [ ] Wire actual mint submission/readback, then head submission/readback, then terminal bodies. Source amount100 is projected from evaluated effect; no economic-oracle fallback supplies a missing redeemer amount.
- [ ] Bind Head13 and payout outputs to policy-qualified Head/Cash quantities and the expected recipient address/payment credential. Compare every datum/lifecycle field, work975/49/16, ordered used IDs, transport1/0, locked0/revision1 and input consumption. Record named inclusion/confirmation evidence; query presence is not finality. Repair runOwnedRace native completion aggregation only from the original-pair receipts and full readback: it currently returns ok:false and ledgerExecution:false even for its RACE_OBSERVED code. Preserve UNIT false results.
- [ ] Use separate fresh escrow instances for release/refund/cancel and another shared head for the race. Submit two original race bodies before polling, preserve disjoint fee/collateral and original loser failure; no post-poll third submit. Missing original loser receipt or ambiguous winner remains incomplete.
- [ ] Run wrong-role, foreign-policy, amount-null, stale-source/instance, wrong-work/IDs, absent-payout and changed-body witness controls through actual exported consumers. UNIT transport remains unconfirmed and never grants a real receipt.

```js
const result = await productionAttempt({ substitution: 'payout-credential' });
assert.equal(result.report.stages.readback.financialMatch, false);
assert.equal(result.report.confirmed, false);
```

### Task4: Expose and audit the first contract command (CS08, CS11)

**Files:** cli.mjs; native-caller.mjs; native-runtime.mjs; native-bootstrap.mjs; tests/cardano-t1.test.mjs; public request schema/documentation alongside the experiment.

**Consumes:** Tasks1–3 checked APIs and existing loadAuthority/resource controls.

**Produces:** proposed cardano-t1 command, declarative request validation, complete current source candidate for both independent audits.

- [ ] Add cardano-t1 CLI parsing that calls runNativeCaller and writes only public result artifacts to a validated owned output path. Reject callbacks, supplied proof/measurement flags, unsafe destinations and unknown network selectors before any child process.
- [ ] Preserve bootstrap155 sequence/config path joins, source pins, supplied/delegated conservation, actual role-key discovery and readiness before queries. Keep body/quantity negatives and94 custody checks.
- [ ] Run the focused offline suite with a fresh owned short HOME/TMPDIR and source hashes before every attempt:

```bash
node --test tests/native-production.test.mjs tests/cardano-t1.test.mjs \
  tests/native-caller.test.mjs tests/native-financial-body.test.mjs \
  tests/native-financial-caller-188.test.mjs tests/native-financial-consumer-129.test.mjs \
  tests/time-capability-228.test.mjs tests/native-bootstrap-closure.test.mjs \
  tests/native-producer-repair.test.mjs tests/audit-repair-immediate.test.mjs \
  tests/audit-repair-remaining.test.mjs tests/production-origin-417.test.mjs \
  tests/custody.test.mjs
```

Run from `experiments/moriarty-eutxo`. Expected: all selected assertions pass with UNIT/native classification intact. Re-run the existing68 quantity control against current files in a new owned attempt, preserving UInt128/Word64 distinctions.

- [ ] Parent freezes the complete source union, imported dependencies, tool/config/evidence pins and actual outputs. Two fresh independent Meta Muse Code CLI sessions, both requesting `muse-wonder-1.0` and recording the returned contributor identity, each review FULL compiler AND ledger scope against exactly that candidate and CS01–CS11. A missing/rejecting audit blocks Task5; repairs remain authorized. Outcome review of this plan is not one of those source votes.

### Task5: Demonstrate actual local ledger then Cardano Preview (CS06–CS11)

**Files:** owned request/run/result artifacts only, plus measured source corrections if an actual failure identifies one. Implementation source corrections require a fresh exact candidate and fresh full audits before renewed native action; deterministic runtime sibling generation follows the explicit audited-emitter rule below.

**Consumes:** both full current source approvals, existing run/resource authority, task-specific private role keys and network configuration after gates.

**Produces:** complete local and Preview transaction evidence plus separate independent actual-result review.

- [ ] After both full approvals, execute the approved fresh isolated bootstrap in an owned short path with consistent child HOME/TMPDIR. Retain command status, actual tip progression, socket/node identity, post-replacement genesis hashes and current protocol/era queries.
- [ ] Capture genuine V3 clock parity; compile the measured sibling and retain source/Core/UPLC/application identity chain. Dynamic outputs instantiate the audited emitter; source edits never inherit its audits automatically.
- [ ] Run cash mint → confirmed readback → head open → confirmed readback → terminal settlement for release, refund and cancel on separate instances, plus the original-pair race on another head. Compare complete expected results and balances/fees/collateral, preserve all receipts and run-owned cleanup evidence.
- [ ] Obtain separate independent actual-result review of the local execution, including known-negative controls and original loser receipt. If any required outcome is absent, retain local exit open and repair the measured consumer without retrying broad administration.
- [ ] Select checked Cardano Preview config/endpoint and fresh testnet identities/resources only after the prospective Preview authority/config/faucet/ceiling/funding contract is actually bound and supported; no existing executable Preview authority is asserted. Record actual bounded fee/poll policy. Rebind network genesis/magic/domain/roles/scripts/protocol and rerun source-derived instances and race. No local genesis creation or old wallet reuse on Preview.
- [ ] Retain public txids and named chain confirmation evidence, source/UPLC/readback hashes and complete semantic/economic comparison; obtain independent actual-result review. Close this first-contract sprint only if SPEC.md exit rows pass. Wider20/privatePCD/general-obligation/final-matrix work remains open.

Proposed commands, to be implemented and inspected before use; none are executed by this plan:

```bash
node src/cli.mjs cardano-t1 --request local-request.json --out owned-local-results
node src/cli.mjs cardano-t1 --request preview-request.json --out owned-preview-results
```

## Spec-to-plan coverage

CS01/CS02 → Task1; CS03 → Tasks1/3; CS04 → Task2; CS05/CS06/CS07 → Task3; CS08 → Tasks4/5; CS09/CS10 → Task5; CS11 → every task. Baseline417 repaired failures remain regression requirements, not approvals. No source changes or ledger commands occur during this planning handoff.

## Revision2 mandatory implementation contract

Read CONCRETE-PREDICATES.md in full (also reproduced in SPEC.md). Its audit, provenance, freshness, interval/receipt, mint, I1/F18 rollback, race-cause, Preview resource and O01–O50/L01–L03/P01–P02 contracts are mandatory extensions of every applicable task. They are part of the proposed change, not optional later receipts.

### Task1 additional steps — CS01/CS02/CS03/CS08

- [ ] Implement canonical auditBinding in context: candidate/source/dependency manifests and2 distinct provider/session/full compiler+ledger approval record references+hashes; rehash current files/tools/input schemas and validate actual substantive public verdicts against parent-verified canonical provider results. Run O01–O08 before any positive native context can be issued.
- [ ] Implement the explicitly proposed900slot preparation lead and deterministic derived-artifact verification described in SPEC: exact source rewrite, recomputed Core/program hashes, exact lowered input, pinned actual compiler/build and canonical application binding. Preserve historical tip+30/90 schedule as r1 evidence; refuse exhausted schedule rather than extending it implicitly.
- [ ] Encode the context's evidence class/audit identity only as metadata backed by a private issuer map. Do not accept JSON/hash-copied UNIT context or a publicly callable production issuer. Internal audited encodeAtFee closures are permitted; caller-supplied compile/spawn/encoder hooks are not production authority.
- [ ] Run O19–O27 seed-owner recipient/signer/bundle/same-seed/staked/script/network/input-collision controls through actual context/funding consumer before encoding; settled holding outref equality is required, not amount equality alone.

### Task2 additional steps — CS02/CS04/CS11

- [ ] Extend final receipt with auditBinding+issuer class, validity slots/POSIX, scriptDataHash, input/collateral/totalCollateral/return and every output/change identity, as well as source/program/policy/instance, units/minima/fee/witness/reference-script bindings. Run interval-only, script-data-only and collateral/change-only mutation controls.
- [ ] At final evaluation, pre-sign and pre-submit gates, obtain actual fresh query bundles using at most4 stable-bundle attempts. Enforce ancestry,0..5slot drift, protocol/era/genesis/domain equality and exact selected-input state. Requery after long compilation without expiring the admitted clock automatically; receipt freshness is measured from final evaluation. Protocol/UTxO/era drift and rollback follow the SPEC invalidation rules; an old signature is not submitted.
- [ ] Enforce exactly4 combined fee/unit/rebuild/final-evaluation passes. Reject nonconvergence on pass4 without a fifth pass. Run O09–O18 and record actual command counts, signed-size failures and eligibility:false for byte-perfect UNIT evidence.

### Task3 additional steps — CS03/CS05/CS06/CS07

- [ ] Implement native-chain-observer.mjs `observeChainInclusion({ txid, bodySha256, networkContext, connection, workDir })` returning authentic containing-block/header/body-membership/height/slot/ancestor-path/tip/rollback observation, and `judgeOperationalConfirmation({ issuedObservation, expectedEffects, stage })` where stage=intermediate-I1|terminal-F18. Real issuance requires an inspected pinned node-to-client or checked Preview provider interface; an unsupported interface blocks readiness. Do not invent a CLI block-query command. Unit observations cannot issue I1/F18 confirmation.
- [ ] Test actual consumer I1 depth0/refusal, depth1/complete dependency; F18 depth17/refusal, depth18/complete terminal; missing ancestry and rollback reset. A missing block observer leaves outcomes unconfirmed. Implement output/inputs/policies/role/effects comparison before either status.
- [ ] Implement all O28–O42 quantity/obligation/Word64/seed/window controls explicitly. Partition-boundary positives use complete representable slot windows; point values that violate the rounding rule are negatives.
- [ ] Implement O43–O50 original loser attribution: parse typed original-body error/outcome with exact missing shared head, no fee/collateral alternate cause and F18-confirmed full winner. Submit-time receipt is valid evidence but does not alone finish a race. Accepted/silent loser, unrelated missing input, ambiguous winner and different measurement contexts stay incomplete.

### Task4 additional steps — CS08/CS11

- [ ] Freeze and audit negative driver alongside all production paths and chain observer. It may submit only named intentional-invalid L02/L03/P02 cases after both full audits, with declared expected predicate and resource accounting; it never grants normal production eligibility or disguises a rejection as success.
- [ ] Run named offline O01–O50 through actual exported consumers and retain the68 boundary controls and94 custody compile checks. Full source audits explicitly approve the900slot schedule, deterministic runtime artifact rule, fresh measurement/confirmation adapters and exactly2 original race submits.

### Task5 additional steps — CS09/CS10/CS11

- [ ] Local run: reserve distinct negative-test fee/collateral, perform4instance positives with I1/F18, and mandatory L01 shared-head race loss, L02 missing signer phase1 and L03 wrong payout phase2 ledger rejection. If evaluator catches L03 before submission, record that layer and execute the audited negative driver for actual ledger evidence or keep L03 open. Include actual collateral disposition in gross reconciliation.
- [ ] Do not begin Preview spends until the prospective preview-execution-dependency.json below is complete and actually authorized.4instances require100Cash each, actual separate ADA minima/fee/change/collateral allocations and both race partners, plus P02 phase1 negative and declared funding transaction counts. No existing Preview grant is manufactured.
- [ ] Preview independently captures genuine V3 parity from checked Preview genesis/era/tip, generates900lead schedule, recompiles/verifies the derived sibling and reapplies fresh seeds/roles/scripts. Fresh pre-sign/pre-submit observations and I1/F18 apply; local proof/config/artifacts do not substitute.
- [ ] Require Preview positives/race/P01/P02, budget/resource reconciliation and independent actual-result review; collateral-consuming L03 is mandatory on local devnet, not duplicated on Preview. This is explicit classified negative coverage, not a weakening of baseline semantic obligations.

## Proposed blocked Preview dependency record (fully defined schema)

The initial record has status BLOCKED_BY_PREVIEW_DEPENDENCIES, null authority/config/faucet/ceilings/allocations and no executable grant. These nulls are an explicit blocker, not deferred acceptance. Parent must supply and validate actual prospective bounded values before execution. Required keys/types:

```js
{
  status: 'BLOCKED_BY_PREVIEW_DEPENDENCIES',
  authority: { ref:null, sha256:null, scope:null },
  endpoint: { ref:null, configPath:null, configSha256:null, genesisSha256:null, networkMagic:null, domain:null },
  faucet: { sourceRef:null, fundingReceiptRefs:[] },
  ceilings: { perTxFeeCeilingLovelace:null, aggregateFeeCeilingLovelace:null,
    aggregateGrossDebitCeilingLovelace:null, aggregateCollateralLiabilityCeilingLovelace:null },
  instances: ['release','refund','cancel','race'].map(action => ({ action, kernelCash:'100',
    cashSeed:null,headSeed:null,mintFee:null,mintCollateral:null,headFee:null,headCollateral:null,
    terminalFee:null,terminalCollateral:null,raceOtherFee:null,raceOtherCollateral:null,
    adaAllocationLovelace:null,outputMinimaReserveLovelace:null,changeReserveLovelace:null })),
  negativeAllocations: { phase1Fee:null, phase1Collateral:null },
  fundingTransactionCount:null, positiveTransactionCount:'13', negativeTransactionCount:'1',
  commandTimeoutMs:null,pollTimeoutMs:null,measurementPassLimit:'4',queryBundleAttemptLimit:'4'
}
```

Positive13 =3mint+3open+3terminal standalone, plus1mint+1open+2original race submissions. Funding transactions are counted separately and bounded explicitly. CONCRETE-PREDICATES.md specifies the proposed numerical request (fee5000000/tx,100000000 aggregate fees,2000000000 gross debit,200000000 collateral liability,450000000 test-ADA liquidity). The executable schema still has null bindings/values until actual authority and funding are checked; this is not an existing grant. All filled ceiling/allocation/count/timeout values are canonical nonnegative Word64 decimal strings (positive where required); identity references/hashes must match actual artifacts and authority. Totals and worst-case collateral must fit the actual available test-ADA before starting; change/refunds do not subtract gross limits. No public-money/old-key authority is inferred.

Prospective Preview public funding limits: standalone instances90000000 each, race110000000, phase1-negative15000000, funding reserve10000000, total planned405000000 plus45000000 liquidity reserve. Max positive13+negative1+funding2=16 submissions. Actual measured output/collateral/fee requirements must fit this bounded request and actual authority, or execution remains blocked.
