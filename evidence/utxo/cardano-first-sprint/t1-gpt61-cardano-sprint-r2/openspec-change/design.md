# First Cardano contract design — revision2

Status: PROPOSED; independent expected-outcome re-reviews pending. No implementation/native/resource approval.

## Normative plan

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

## Complete concrete predicate contract

# Revision2 executable predicates and required controls

Status: proposed re-review contract. These predicates strengthen CS01–CS11 and the five PLAN tasks. They do not grant source/native approval or claim an existing Preview resource grant.

## Audit object and deterministic sibling (CS01/CS08; Tasks1/4/5)

Choose deterministic audited emitter derivation, not a post-bootstrap fresh-audit deadlock. Both fresh FULL compiler+ledger audits SHALL explicitly cover the exact frozen implementation/dependency/tool manifest, original source/seed/bounds/Cash/bootstrap inputs, parameter schemas, permitted network adapters, deterministic guard-substitution/elaboration/lowering/application functions and all native/negative drivers. Their public verdicts SHALL approve this dynamic-artifact rule substantively, not merely schema/hash validity.

`auditBinding` SHALL contain candidateManifestSha256, implementationSourceManifestSha256, dependencyManifestSha256 and exactly two audit records: canonical review record reference, provider terminal-result reference/hash, public-review hash, audited candidate hash, distinct actual session ID, requested muse-wonder-1.0, actual muse-wonder-1.0-contributor, FULL compiler AND ledger scope and explicit approval verdict. The actual admission consumer verifies these records against parent-verified provider outcomes in the canonical trusted review store and rehashes every candidate row/tool/input; caller-written JSON, self-review, two copies of one session, missing public approval, mismatched exclusions/scope/hash or a source delta fails before any native action. A hash-shaped or copied approval file does not establish a vote.

A runtime sibling is a DERIVED_AUDITED_EMITTER_ARTIFACT. Before funding, recompute the exact sibling bytes from the audited original source and admitted bounds using the audited function; recompute Core/program/schema/bounds/seed identities through the audited factory; compare the actual written source/lowered Aiken input byte-for-byte; retain the actual pinned compiler invocation/check/build output and blueprint binding; verify the definite parameter CBOR/order and resulting script identities with the existing Aiken/CML/CLI agreement. The original source and all non-time text remain unchanged. Body consumers use only those resulting scripts and current instance identities. An altered derived file, nonpermitted bound/network/parameter or handwritten sibling is refused, not automatically reclassified as an audited artifact. Any change to the frozen implementation/input schemas requires two fresh full audits; a runtime value generated by the explicitly approved parameterized emitter does not by itself change the implementation candidate.

## Scheduling amendment and freshness (CS01/CS02/CS04/CS11; Tasks1/2/5)

R1 historical formula is releaseSlot=captureTip+30, partition=releaseSlot+60, refundExclusive=partition+60. Actual417 compile tests took388854ms and290390ms, so a90slot release cutoff cannot safely contain the current compile/funding sequence. R2 proposes preparationSlots=900; releaseSlot=captureTip+900+30, partition=releaseSlot+60, refundExclusive=partition+60.1000ms clock,30slot minimum inclusion margin,60slot windows, forecast horizon1800s and unchanged guard-only substitution are retained. Refund end is1050slots after capture, within1800. Audits and outcome reviewers must explicitly approve the900slot scheduling input; it is not retrospectively called part of r1. If remaining time is insufficient after compilation/funding, refuse TIMING_BUDGET_EXHAUSTED and record failure; do not silently extend guards or sleep beyond forecast. At submission, current slot must be inside the body's half-open interval and before its end; at least30slots must remain in that interval when a new terminal body is first signed. A release/cancel cannot be signed in the refund partition; refund waits for its own interval. Confirmation may finish after expiry if inclusion occurred inside the valid interval.

Clock/parity capture establishes the admitted conversion against the same genesis/domain/era, not a five-slot lifetime for a several-minute compile. After compilation and each intermediate settlement, refresh protocol/era/tip/all selected UTxOs, retaining the admitted clock, audited emitter/artifacts and scheduled window. A measurement receipt records the tip at its final successful body evaluation. Immediately before signing and again before submission, query a stable tip and current protocol/era/required-input state. A stable query bundle has initial/final tip on the same observed ancestry and ≤5slots drift; at most four query-bundle attempts are allowed. Pre-sign/pre-submit tip must descend from the final measurement tip, have nonnegative drift ≤5slots and satisfy the actual validity-window condition. Protocol bytes/cost model, era bytes, genesis/config/magic/domain, all selected input outrefs+address+value+datum, scripts and instance must equal the bound context. Any missing/spent input or role/network/artifact mismatch refuses. Tip drift>5, protocol/era change or rollback invalidates receipt/signature eligibility and requires refreshed context plus reevaluation/minima/fee checks before signing; an era/clock mapping change requires fresh genuine parity and regenerated sibling under the same audited emitter. No stale receipt remains eligible by copying hashes. An already signed body is not submitted after any gate changes. Any source-code change returns to full source audits.

## Authentic issuing and final-body receipt (CS02/CS04; Tasks1/2)

Production context and measurement provenance are nonserialized in-process identities issued only by the audited actual transport/compiler/query/evaluator consumers after canonical audit admission. Keep issuer metadata in private WeakMaps or equivalent noncaller-settable state; do not export a production issuer or accept caller callbacks as real transport. Metadata records `evidenceClass=PRODUCTION_AUTHENTIC_TRANSPORT`, auditBinding and actual child/connection result references. A JSON/hash copy, constructed transcript, custom compile/spawn callback, fixture CLI, UNIT context, offline cost/minima arithmetic or Boolean marker cannot carry issuer identity. Offline structurally perfect arithmetic/emitter positives always return submitEligible:false and ledgerExecution:false. Successful production context/measurement alone also has ledgerExecution:false; only actual settlement evidence can establish ledger effects.

The final receipt binds candidate and two audit identities, genesis/config/domain/network, clock bounds/capture, exact body CBOR SHA+txid+bytes, scripts/application/source/program/bounds/schema/seed/instance, queried inputs, validity start/end slots and corresponding redeemer POSIX lower/upper, scriptDataHash recomputed from exact purposes/redeemers/datums/language views, fee, ordinary input order, collateral outrefs/values/key hashes, totalCollateral, collateralReturn output/address/assets/datum, every ordinary output/change outref/index/address/policy-qualified asset bundle/datum/reference-script hash, per-purpose/index/hash units, each output's measured minimum, reference-script fee size and complete signing-key witness hash set. Verify final signed bytes/witness coverage/size without changing the body. Receipt equality is necessary but insufficient: assertProductionBody requires authentic issuer metadata, canonical auditBinding and fresh pre-sign/pre-submit state. Interval-only, script-data-hash-only, collateral-only or change-only mutation invalidates eligibility before signing/submission.

Exactly four fee/rebuild/evaluate passes are allowed per body attempt; any fee/unit/body change requires full new final-body reevaluation and output-minimum checks within that bound. Nonconvergence is FEE_DRIFT/MEASUREMENT_NONCONVERGENT, no fifth pass. Units/minima/gross/input/change/fees remain exact quantities; no supplied unit shortcut or transaction-byte minADA formula.

## Cash mint and distinct dependency inputs (CS03; Tasks1/3)

A valid Cash mint consumes the actual approved Cash seed and requires the seed owner's enterprise payment credential in the witness/required-signer set. Its selected holding output pays that enterprise owner, has exactly100 of this instance's applied Cash policy/name43617368, no Head or unapproved foreign assets, no stake/script payment credential, and lovelace≥its measured minimum. Preserve the policy's approved allowance for foreign mint on a separate reviewed purpose; it cannot add unapproved assets to this holding output or confuse this Cash identity. Separate change goes to the actual funding owner's address with an exactly reconciled policy-qualified bundle; bind collateral return separately. Required owner/signing paths are actual current identities, not encoding fixtures.

Cash seed≠head seed. Before intended spends, all selected Cash/head/mintFee/headFee/leftFee/rightFee/cancelFee and collateral inputs are distinct; the only allowed repeats are the settled Cash holding carried into head opening, the settled head carried into a terminal, and the one settled head shared between the two original race members. Each race member has disjoint fee/collateral; ordinary inputs and collateral do not intersect. A funding fanout transaction may create these distinct outputs, but cannot manufacture reused or unobserved inputs. After intermediate confirmation, requery the exact original mint holding outref and owner/value; only that exact confirmed output is consumed in the head body. Different same-value outref, wrong recipient, missing owner witness, same seed, staked/script/wrong-network seed or closed policy refuses before normal encoding/sign/submission.

## Operational confirmation and rollback (CS03/CS06/CS09/CS10; Tasks3/5)

Define I1 intermediate confirmation: actual containing block hash,height,slot and raw block/transaction inclusion proof from an authenticated checked chain observation; the exact submitted txid/body must be in that block, and the current observed canonical chain must contain that block with at least1subsequent block. Full output/input comparison and requery still must pass. Both mint→head and head→terminal use I1. Define F18 terminal confirmation: the same inclusion/ancestry/full effect conditions with at least18subsequent blocks (`tipHeight−inclusionHeight≥18`). Apply F18 to standalone terminal results, race winner and local/Preview final exit. These are operational testing rules, not mathematically irreversible finality, not inferred from securityParam18 and not proof of Cardano's generic security assumptions.

The checked observer records exact inclusion block CBOR/header hash,height,slot, txid/body membership, chain intersection/ancestor path and current tip hash/height/slot. It must use an inspected pinned node-to-client chain observation or checked Preview provider connection; no invented cardano-cli query command, caller-supplied block metadata or unit fixture counts as observation. Implement the actual adapter and audit it; if the toolchain cannot supply the required block/ancestry predicate, native readiness/exit is BLOCKED_BY_CHAIN_OBSERVER rather than presumed finality.

Every observed rollback affecting the inclusion/measurement anchor invalidates confirmations and downstream eligibility; reset confirmation tracking from new actual canonical inclusion, requeries and fresh measurements. Lower height, missing ancestor, disappeared output or conflicting confirmed identity triggers reset/refusal. After an intermediate rollback, do not continue spending its predicted output; report dependent transactions' actual states and preserve any already-sent txids. A visible mempool transaction or output at depth0 is not I1; depth1 permits the dependency only after full comparison; depth17 is not F18; depth18 permits the scoped operational confirmed result. Chain-sync depth errors or unavailable ancestry leave results unconfirmed.

## Original shared-head loser proof (CS07; Tasks3/5)

Both original race bodies must be measured/fresh and valid against the same checked candidate/genesis/instance/shared-head state with overlapping half-open validity intervals before either submission. Fees/collateral are distinct and available at that gate. Exactly two original submissions precede every poll. Preserve each body's exact CBOR/txid/digest, timestamp, required inputs, original command/result and any original-pair outcome record.

A valid loser record is (a) a typed BadInputsUTxO result at that loser's original submit, or (b) an authenticated original-pair outcome explicitly tied to that original txid/body, without resubmitting. Its parsed missing-input set must identify the exact shared head consumed by the confirmed winner, with no unavailable loser fee/collateral/other input as an alternate cause. Bare error-name text, missing cause details, different-input failure or unrelated txid does not qualify. Race complete requires exactly one F18-confirmed full expected winner consuming the same shared head plus that original loser record. Submit-time loser evidence may be recorded before winner confirmation but completes only after F18 winner/source/effect comparison.

Exhaustive classification: one valid confirmed winner+shared-head-specific original loser = COMPLETE_ORIGINAL_PAIR; exactly one winner but both submits accepted and loser silent/dropped/no receipt = INCOMPLETE_LOSER_RECEIPT; both pending/no winner = INCOMPLETE_WINNER; multiple winners/conflicting chain evidence = INCOMPLETE_AMBIGUOUS; wrong state/payout/source or fee/collateral/nonshared-head failure = INCOMPLETE_CAUSE_OR_POST. UNIT remains unconfirmed/ledgerExecution:false regardless of code. No outcome permits a third submission to manufacture a receipt.

## Preview dependency/resource contract (CS10; Task5)

Preview currently has no executable binding/resource grant evidenced in this packet. Status is BLOCKED_BY_PREVIEW_DEPENDENCIES. Before any Preview spend, the parent must bind a prospective `preview-execution-dependency.json` to the actual user-authority reference and a fresh checked endpoint/config, faucet/test-ADA source and runtime identities. Creating a filled record is not itself user authorization and does not invent an existing grant. No repeated design vote is required for routine repair; absent authority/binding is a concrete execution dependency.

Required record: authorityRef+hash+scope; endpoint/config paths+hashes+actual Cardano Preview genesis/networkMagic/domain; faucet/test-ADA source reference and funding receipts; Word64 canonical decimal values for perTxFeeCeilingLovelace, aggregateFeeCeilingLovelace, aggregateGrossDebitCeilingLovelace and aggregateCollateralLiabilityCeilingLovelace; per-instance allocations for release/refund/cancel/race with separate Cash seed/head seed, mint/head/terminal fee and collateral outrefs, ADA output sponsorship/minimum/change reserves, and the race's two separate fee/collateral allocations. Each of4instances has100 kernel Cash under its actual instance's seed-applied policy; total intended kernel allocation400Cash, not400lovelace. The record lists individual exact test-ADA allocations and checked totals, planned maximum transaction counts (including negative tests), per-command/poll timeout limits and four-pass measurement limit. The requested numerical envelope below is prospective and unapproved; the executable dependency record keeps authority/bindings/actual allocations null until supported by actual authority and funding observations. Any missing executable field blocks execution.

Prospective numerical resource request for review (NOT an existing grant): per-transaction fee ceiling5000000lovelace; aggregate gross fee ceiling100000000; aggregate gross debit ceiling2000000000; aggregate worst-case collateral liability ceiling200000000; requested testnet-only faucet liquidity450000000lovelace. Proposed per-standalone-instance ADA allocation90000000 = Cash seed5000000 + head seed5000000 + mint fee10000000/collateral10000000 + head fee10000000/collateral10000000 + terminal fee10000000/collateral10000000 + output-minimum reserve10000000 + separate change reserve10000000. Release/refund/cancel each use that allocation. Race allocation110000000 adds another fee10000000 and collateral10000000 for its second original member. Thus4instance allocations total380000000. Preview phase1 negative allocates fee5000000 and collateral10000000 (15000000), and funding transaction fee/change reserve10000000; planned allocations405000000, remaining liquidity reserve45000000. Planned collateral outpoints total140000000 (four instances120000000, race extra10000000, negative10000000), within the prospective200000000 collateral-liability ceiling. Reserve is not authorization or a refund of gross fees. Planned positive submissions13, negative1, funding fanout maximum2, total maximum16;100000000 aggregate fee ceiling accommodates16×5000000 with20000000 remaining ceiling margin. Before execution, actual measured allocations/minima/collateral percentages and every funding/debit/reserve total must be checked against this request and the supported authority; otherwise remain BLOCKED_BY_PREVIEW_DEPENDENCIES. A reviewer approval of this proposed envelope is not user spending authority. No faucet receipt, endpoint or authority is fabricated.

Check before every sign/submit: measured fee≤perTx ceiling, cumulative gross fees including failed/negative transactions plus reserved outstanding liabilities≤aggregate fee ceiling; gross debits and worst-case collateral liabilities≤their aggregate ceilings. Refunds/change/returned collateral never erase gross accounting. Actual available input balances must cover the complete4instance plan and minima/fees/collateral reserves before beginning; partial funding does not close Preview exit. Fail closed when fees increase beyond reserves or a ceiling is missing/exceeded.

Preview sequence independently queries its real genesis/config/protocol/era/tip/roles, executes a genuine V3 parity probe against the actual Preview context, applies the approved900slot preparation schedule, generates/revalidates/recompiles its sibling through the audited emitter and applies fresh Preview seeds/roles. Never reuse local genesis, capture, seed/outref, scripts/instance or authority record. Refresh measurements as specified above. One deterministic sibling blueprint for one schedule may be applied to4fresh instance seeds/roles under the audited schema; it does not permit sharing issued instance/body receipts. Prepare/fund these instances within that same schedule and reject an exhausted window. Then run4instances and require I1/F18/full race/effect/budget reconciliation and separate independent actual-result review.

## Required test IDs and placement (CS05/CS09/CS11; Tasks1–5)

All offline controls call actual exported consumers; injected outputs remain UNIT and cannot grant eligibility. Mandatory offline IDs:

- O01 wrong candidate, O02 missing/mismatched audit hash, O03 duplicate audit session, O04 public verdict lacks full compiler+ledger scope, O05 byte-perfect serialized UNIT context/measurement, O06 production compile/spawn callback, O07 altered sibling source/program/blueprint/application, O08 context reused across instance/network.
- O09 tip drift6slots (5allowed), O10 protocol/cost-model change, O11 era change, O12 seed/head/fee/collateral spent between measurement and signing, O13 rollback or missing measurement ancestor, O14 interval-only mutation, O15 script-data hash mutation, O16 collateral/return/change-only mutation, O17 witness-key or final signed-size mutation, O18 missing output-minimum or four-pass nonconvergence. Each refuses eligibility and normal sign/submit; invalid source/seed/amount inputs also prevent normal encoding.
- O19 wrong mint recipient, O20 missing seed-owner witness, O21 same Cash/head seed, O22 staked seed, O23 script-locked seed, O24 wrong-network/closed seed, O25 changed same-value mint holding outref, O26 shared fee/collateral or ordinary/collateral collision, O27 wrong holding/change asset bundle.
- O28 amount missing, O29 null, O30 leading-zero/fractional/noncanonical, O31 negative, O32 UInt128 overflow, O33 required obligation quantity absent (not zero), O34 invalid obligation quantity, O35 Word64 overflow coin, O36 valid UInt128 value wrongly used as Word64 coin, O37 valid Word64/kernel boundaries preserving exact decimal bytes; retain68 quantity controls.
- O38 calibrated release/cancel upper=partition−1 accepts exact full release window; upper=partition refuses; O39 release lower=partition refuses, refund lower=partition accepts exact refund window; O40 refund upper=refundInclusive accepts, upper=refundInclusive+1 refuses; O41 cancel partition−1 accepts exact window, partition/refund window refuses; O42 expired schedule/insufficient30remaining slots refuses. Single-point POSIX pairs may be unrepresentable: valid bounds assertions use complete slot-aligned windows and do not weaken rounding fences.
- O43 mint presence/depth0 blocks head; I1depth1+ancestry+full mint comparison permits dependency, O44 final depth17 remains unconfirmed/depth18 permits F18 only with full effect, O45 rollback resets both stages, O46 original submit-time typed shared-head loser+F18 full winner completes, O47 accepted then silent loser stays incomplete, O48 missing-input set names fee/collateral/different head stays incomplete, O49 ambiguous/wrongpost winner, O50 both body measurement contexts/interval overlap mismatch. Post-submission O46–O49 controls assert submitCount2 and no resubmit; O50 preflight mismatch asserts0submits and no resubmit.

Mandatory real local-devnet positives: genuine clock/emitter/measurement and4instances Cash mint/head opening/release/refund/cancel/race, I1 dependencies and F18 terminal winners. Mandatory local on-ledger negatives: L01 replay/shared-head consumption via the original losing race submit; L02 missing role witness/required signer phase1 rejection; L03 wrong payout recipient with an otherwise valid body, source script evaluation/ledger phase2 refusal (record actual collateral disposition). L02/L03 use a separately audited negative-test driver with declared expected invalid predicate and bounded resource accounting; it does not issue normal submit eligibility, count rejected intent as settlement or bypass gates for positive bodies. If pre-submit evaluator catches the wrong payout, retain that as offline/evaluation evidence; obtain actual ledger rejection with the separately audited negative driver to close L03, or leave that mandatory negative open. No fake rejection logs.

Mandatory Cardano Preview: genuine Preview parity/recompile/context;4instances full positives and original-pair shared-head rejection P01; actual missing-role witness P02 phase1 rejection using the audited negative driver. Local L03 is mandatory phase2 rejection evidence; duplicate phase2 rejection on Preview is not required and does not silently stand in for other outcomes. All remaining hostile quantity/forgery/window mutations are mandatory offline, not public collateral-burning tests. Record which named predicate actually rejects each case, real command receipts and collateral effects. O01–O50, L01–L03 and P01–P02 remain mandatory classified controls; failures/execution unavailability stay open, never converted into semantic success.
