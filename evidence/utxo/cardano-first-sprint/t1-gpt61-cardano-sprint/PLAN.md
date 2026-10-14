# First Cardano contract implementation plan

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
| `src/cli.mjs` | Add a proposed `cardano-t1 --request <json> --out <owned-dir>` entry point; declarative request only, no caller functions/proof flags |
| `tests/native-production.test.mjs` (new) | Full production-context/body measurement binding controls using injected transport explicitly labeled UNIT |
| `tests/cardano-t1.test.mjs` (new) | Exported caller/CLI ordering, refusal, finality and environment selection controls |
| Existing caller/body/consumer/time/custody/auditrepair/quantity tests | Retain current meaning and cover production integration; do not change expected outcomes to implementation convenience |

## Proposed concrete interfaces

These are specified interfaces, not implemented in417. Implement them in the narrow new module and actual consumers together.

```js
// Bound context is an in-process object issued only after actual capture/query/
// script application. Serialized records retain hashes for evidence, not authority.
bindProductionContext({ admission, source, cash, head, roles, protocol, era, genesis })
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

assertProductionBody({ context, body, receipt })
// -> { ok:true, submitEligible:true } or { ok:false,code,submitBlocked:true }
// Success requires issued provenance and byte/identity equality, not Boolean fields.
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

**Consumes:** Task1 context, current queried protocol/era/UTxO files and actual output specifications.

**Produces:** measureProductionBody, assertProductionBody and exact-body submission eligibility.

- [ ] Write red controls for swapped body after evaluation, mismatched cost script hash/purpose/index, missing output minimum, stale protocol, missing collateral witness, altered reference-script size, excessive signed size/exunits and fee nonconvergence.
- [ ] Make measuredCoin/convergeFee/recountSignedWitnessFee/plutusCostUnits receive the context's protocol path/provenance instead of OFFLINE_PROTOCOL_PARAMS. Measure all outputs with exact actual datum/assets; never multiply whole transaction bytes by a protocol rate to invent minimum ADA.
- [ ] Use the supported calculate-plutus-script-cost online command through plutusCostArgv, retain stdout/stderr/status and consumed cost rows. Match all required scripts/redeemers against decoded current CBOR, rebuild execution units, recalculate script-data hash, then reevaluate final bytes. A trace-only failure or1/1 supplied shortcut remains a refusal.
- [ ] Use existing bounded four-pass fee convergence, actual signing-key witness count and current reference-script fee input; reject drift. Changing fee, units, datum, assets, outputs or witness count invalidates prior body/cost/minimum-ADA receipt and requires a new checked final measurement.
- [ ] Repair acquireBodyEvaluation/compareBudget so successful measurement reports complete receipts and actual minima instead of permanent nativeBudgetMeasured false or unmeasured placeholders. Enforce current protocol caps on final signed bytes, not the unsigned estimate alone.
- [ ] Verify positives and all negative controls through runNativeCaller. Only then replace PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE with checked submission eligibility; preserve refusal for every incomplete binding.

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

**Files:** owned request/run/result artifacts only, plus measured source corrections if an actual failure identifies one. Source corrections require a fresh exact candidate and fresh full audits before renewed native action.

**Consumes:** both full current source approvals, existing run/resource authority, task-specific private role keys and network configuration after gates.

**Produces:** complete local and Preview transaction evidence plus separate independent actual-result review.

- [ ] After both full approvals, execute the approved fresh isolated bootstrap in an owned short path with consistent child HOME/TMPDIR. Retain command status, actual tip progression, socket/node identity, post-replacement genesis hashes and current protocol/era queries.
- [ ] Capture genuine V3 clock parity; compile the measured sibling and retain source/Core/UPLC/application identity chain. Dynamic outputs instantiate the audited emitter; source edits never inherit its audits automatically.
- [ ] Run cash mint → confirmed readback → head open → confirmed readback → terminal settlement for release, refund and cancel on separate instances, plus the original-pair race on another head. Compare complete expected results and balances/fees/collateral, preserve all receipts and run-owned cleanup evidence.
- [ ] Obtain separate independent actual-result review of the local execution, including known-negative controls and original loser receipt. If any required outcome is absent, retain local exit open and repair the measured consumer without retrying broad administration.
- [ ] Select checked Cardano Preview config/endpoint and fresh testnet identities/resources under existing user authority; record actual allowed faucet/test-ADA resource and bounded fee/poll policy. Rebind network genesis/magic/domain/roles/scripts/protocol and rerun source-derived instances and race. No local genesis creation or old wallet reuse on Preview.
- [ ] Retain public txids and named chain confirmation evidence, source/UPLC/readback hashes and complete semantic/economic comparison; obtain independent actual-result review. Close this first-contract sprint only if SPEC.md exit rows pass. Wider20/privatePCD/general-obligation/final-matrix work remains open.

Proposed commands, to be implemented and inspected before use; none are executed by this plan:

```bash
node src/cli.mjs cardano-t1 --request local-request.json --out owned-local-results
node src/cli.mjs cardano-t1 --request preview-request.json --out owned-preview-results
```

## Spec-to-plan coverage

CS01/CS02 → Task1; CS03 → Tasks1/3; CS04 → Task2; CS05/CS06/CS07 → Task3; CS08 → Tasks4/5; CS09/CS10 → Task5; CS11 → every task. Baseline417 repaired failures remain regression requirements, not approvals. No source changes or ledger commands occur during this planning handoff.
