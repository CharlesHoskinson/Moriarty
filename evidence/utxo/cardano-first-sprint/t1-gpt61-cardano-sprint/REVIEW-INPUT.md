# Self-contained Muse/Wonder expected-outcome review input

Review only the first Cardano contract outcomes/design. Return APPROVE_EXPECTED_OUTCOMES or CHANGES_REQUIRED with specific requirement/test defects. This is not source/native admission. Requested tool/model: installed Meta Muse, muse-wonder-1.0; actual contributor identity must be recorded if returned. No source edits, shell, web or writes are needed. Two independent Meta Muse Code CLI sessions must reason independently; both request muse-wonder-1.0 and record returned muse-wonder-1.0-contributor identity. These are two sessions of one provider/model, not two vendors. GPT-6.1 authored this plan.

---

## Complete SPEC.md

# First Cardano contract sprint — OpenSpec/EARS requirements

Status: proposed expected-outcome review input; specified-only. Author: GPT-6.1. Two independent fresh Meta Muse Code CLI sessions, both requesting `muse-wonder-1.0` (returned identity `muse-wonder-1.0-contributor`), review expected outcomes; they are separate sessions of the same provider/model, not two vendors; this document is not a full source audit or permission to run a ledger command.

## Purpose and retained scope

Deliver one actual Moriarty source/Core → Aiken/UPLC → Cardano ledger escrow contract before expanding the wider 20-family suite. The first contract is the existing profile/5 public single-head custody escrow with release, refund and cancel. This milestone preserves all20 public pattern families, private/PCD, general obligations and the final reviewed matrix/graph as subsequent obligations. Cardano Preview is the public Cardano testnet exit target; it is distinct from Midnight Preview. This sprint does not claim Midnight acceptance.

Current immutable source417 is the 383-row manifest in `t1-gpt61-implementation-417/source-manifest.json`, published by the parent as commit `1db12527f77c991db52f1de6a5f0fdf256185a4f` on Moriarty `utxo`. No source417 change is authorized during this specification handoff. After independent outcome review, GPT-6.1 remains the sole source writer.

## Normative inherited design

Bootstrap155 is a recipe with two scoped approvals160, not ledger approval. Use era-based Conway create-testnet-data with one pool, one on-disk stake delegator, one UTxO funder and one genesis key. Replace only its Byron output using create-cardano with supply omitted; hash genesis files after replacement. Configuration paths are `genesis/<file>` relative to the node-config parent. Total supply45000000000000000 and delegated supply30000000000000000 lovelace; Shelley slot1 second, epoch3600, security18, fee coefficients1/1, KES3600/64; positive future start, fresh accepted magic and fresh owned run directory. Keep the pinned Alonzo/Conway specs, PlutusV3 cost model and approved ceiling values. An unknown linked ABI is recorded unknown, not infeasibility.

Preserve economic design29: original Layer S SHA256 `00c9893ee5d2cbf41adb619bbe2a4b37e4e3f4de64889aa4567bbfe0174ec957`; source profile/5, contract/4, lifecycle/1; Head13 and Lifecycle9; constructor bound48, debit49, open work1024/0/16, post975/49/16, collection cap128 and nonempty-obligation fence. Kernel Cash100, head thread quantity1, fees and lovelace minimums are distinct quantities.

Approved Cash54 source is SHA256 `3f897ac9ef56c01e11d191dba31736da508dc77982ca484f1684e66013b242c4`: parameter is the consumed Cash seed outref, token name43617368, mint100, burns fail, destination is the seed owner's enterprise payment key and that owner signs; staked/script seed fails. The closed fixture policy332f90... remains closed. Head application order remains head seed, Cash policy, lender, borrower, genesis. Design/hash agreement alone never establishes successful minting.

## ADDED Requirements

### Requirement CS01: Exact source-to-script chain
The system SHALL preserve the original source, elaborate it with the profile/5 factory and bind the generated Core, program, schema, bounds and seed identities to fresh compiled and parameter-applied UPLC. WHEN genuine queried-clock parity is admitted, the system SHALL substitute only the approved time guards and SHALL evaluate every terminal with those substituted arguments before body encoding.

#### Scenario: Calibrated sibling
- WHEN an authentic V3 capture supplies genesis zeroTime/zeroSlot and queried tip on a1000ms clock
- THEN release/cancel use [releaseLower, partition−1], refund uses [partition, refundInclusive], source/program identities change, bounds and economic semantics remain unchanged, and actual compileCustody emits the sibling blueprint used by the caller

#### Scenario: Forgery or stale source
- WHEN a transcript, callback, serialized/copied origin label, source, blueprint, application parameter or current head identity is substituted
- THEN the actual consumer refuses before adopting a production body

### Requirement CS02: Current runtime binding
WHEN the system selects production execution, it SHALL construct one checked context from authentic capture, the current queried seed/head/fee/collateral outrefs, freshly applied Cash/head scripts, current protocol query, current era history and genesis identity. Every downstream encoder/evaluator SHALL consume that context. IF any production field resolves to a UNIT policy, fixture script, fixed UTxO, old cache, offline protocol file or caller-supplied proof flag, THEN the system SHALL refuse and emit no production body.

#### Scenario: One identity everywhere
- WHEN Cash mint, head opening and terminal bodies are prepared
- THEN the Cash policy/name, head script/thread policy/name, program/source/schema/bounds/seed, genesis, instance, role addresses and consumed outrefs agree with the same checked context and actual body CBOR

### Requirement CS03: Production Cash and head opening
WHEN funding an escrow, the system SHALL use the approved Cash source applied to its actual available seed and SHALL observe Cash mint settlement before spending its output to open the head. The head seed SHALL be distinct from the Cash seed. The head opening SHALL mint exactly one Head and transfer the actual100 Cash into the bound script with the Head13 open datum. Private keys SHALL stay in the run-owned private paths; public artifacts SHALL contain only public hashes/addresses/outrefs and transaction results.

#### Scenario: Actual funding dependencies
- WHEN the Cash mint succeeds on the ledger
- THEN only its queried settled output can become the head-opening input; an unsigned-body txid is not an available output

#### Scenario: Forbidden seed/policy
- WHEN the seed is missing, staked, script-locked, already spent, wrong-network or bound to the closed fixture
- THEN funding is refused without reporting Cash mint acceptance

### Requirement CS04: Measured costs, minimum ADA and fee convergence
WHEN preparing each mint/open/release/refund/cancel body, the system SHALL evaluate the actual current scripts and body using the pinned supported calculate-plutus-script-cost command and current ledger context, match each returned script hash and redeemer purpose/index, rebuild with those units and reevaluate the resulting body. The system SHALL measure each output's minimum ADA with calculate-min-required-utxo against the same queried protocol and actual datum/assets/reference script. Every encoded output SHALL meet its measured minimum while preserving the approved sponsorship/payout semantics; a fixed permitted amount below the live minimum SHALL cause refusal, not a semantic flag override. The fee SHALL converge using the actual signing-key witness count, collateral, change and reference-script fee inputs. Receipt provenance SHALL bind body/protocol/script bytes; supplied units or a computed flag SHALL NOT qualify as measurement.

#### Scenario: Valid bounded body
- WHEN final evaluation and output minima match the final CBOR and current protocol, fee convergence succeeds and signed size/exunits fit current limits
- THEN submission eligibility is true for that exact body, with measured per-redeemer units, each output minimum, fee and byte size recorded

#### Scenario: Measurement refusal
- WHEN evaluator stdout is a trace-only failure, script hash/index is wrong, measured body differs, protocol is stale, budget exceeds caps, minimum ADA is missing, fee drifts or witness count changes
- THEN signing/submission is blocked and raw stdout/stderr/status are preserved; no fabricated1/1 or800000/400000 cost is adopted

### Requirement CS05: Terminal semantics and amount projection
WHEN the actual source/Core terminal is evaluated, the consumer SHALL explicitly project the evaluated fixed Cash effect amount into its consumer ABI; missing/null/noncanonical/negative/out-of-UInt128 redeemer amount SHALL be refused. Required obligation quantities SHALL not default to zero. Word64 lovelace and UInt128 kernel quantities SHALL remain distinct. Release SHALL pay100 Cash to Lender; refund/cancel SHALL pay100 Cash to Borrower; every post SHALL preserve Head13 identity, transport1/0, ordinary locked0/revision1, ordered IDs and work975/49/16 with no nonempty obligations admitted.

#### Scenario: Original windows
- WHEN original Layer S1000/1099,1100/1100 or1050/1051 is requested on the zero clock
- THEN FIXTURE_WINDOW_UNREPRESENTABLE is returned with original POSIX bounds and encodeSpendBodyCalled false; aligned40000/99999 is a UNIT40/100 probe only

### Requirement CS06: Signed exact-body settlement
WHEN a final body satisfies CS01–CS05, the system SHALL sign only that body, verify the complete witness-key set and required signers, submit the assembled transaction, retain the public transaction ID and original receipt, and query its actual outputs and input consumption. Final settlement SHALL require full independently expected Head13/Lifecycle9 and payout comparison plus named chain confirmation evidence. A mempool acknowledgement or query presence alone SHALL not establish finality.

#### Scenario: Complete effect
- WHEN release settles on the ledger
- THEN the consumed head outref is gone; the winner head/payout outrefs, policy-qualified quantities, role payout credential, all13 datum fields,9 lifecycle fields and remaining/spent/reserve/IDs match the source evaluator

#### Scenario: Wrong effects
- WHEN payout role, asset policy, amount, source/program/instance, work, transport, obligations or ordered IDs differ
- THEN the result is rejected even if total100 Cash or an outref is present

### Requirement CS07: Original-pair race
WHEN testing release/cancel competition, the system SHALL assemble two valid original bodies from one shared head with disjoint fees/collateral, submit both before any poll, and observe the winner without fixing submission order as the winner. It SHALL retain the original losing submit/outcome BadInputsUTxO receipt and complete queried winner post/payout/identity. It SHALL NOT make a third submission to obtain the losing error.

#### Scenario: Missing outcome
- WHEN both original submits are accepted into mempool and no original-pair loser failure can be observed
- THEN the race is incomplete; no third submit or synthetic negative receipt is allowed

### Requirement CS08: Independent admission and honest evidence
BEFORE any real node/genesis/keygen/funding/mint/sign/submit/query/latch action, two fresh independent Meta Muse Code CLI sessions using `muse-wonder-1.0` (record actual returned `muse-wonder-1.0-contributor` identity) SHALL each issue a FULL audit approving the exact-current compiler AND ledger source candidate. Each selected auditor covers both scopes; these are two total audits. Historical/scoped/compiler-only votes and self-review SHALL not count. AFTER actual execution, separate independent actual-result review SHALL assess source/Core/script correspondence, complete effects and failure receipts. UNIT controls SHALL remain unconfirmed with ledgerExecution false.

#### Scenario: Changed source or missing auditor
- WHEN candidate source changes or either full audit is unavailable/rejects
- THEN dependent native actions remain stopped; authorized source repair and read-only inspection may continue

### Requirement CS09: Local-devnet exit
WHEN the approved isolated Conway bootstrap runs after CS08 source admission, it SHALL demonstrate increasing real tip/block height, queried current protocol/era/genesis binding and authentic V3 parity, then execute approved Cash mint/head opening and source-derived release, refund and cancel on separately funded instances. It SHALL execute the CS07 race on another instance. Gross funding, fees, collateral, outputs and remaining balances SHALL reconcile; cleanup SHALL affect only the owned run. Local exit requires all positives, meaningful rejection controls and independent actual-result review; compilation alone does not close it.

### Requirement CS10: Cardano Preview exit
WHEN local CS09 passes and the run uses an authenticated current Cardano Preview context, the same reviewed compiler/caller SHALL consume that network's genesis identity, network magic/address domain, actual funded role outrefs and queried protocol/era parameters without substituting local genesis or UNIT pins. Execution SHALL use fresh testnet-only identities and documented authorized faucet/test-ADA resources, preserve old wallets and public money, and record the existing resource authority and fee ceiling before spending. A missing resource or endpoint binding is an explicit execution dependency, not a new design-vote loop.

#### Scenario: Preview demonstration
- WHEN one fresh approved escrow contract is compiled/applied for the actual Preview instance
- THEN real Cash mint, head opening and release settle and pass full CS06 comparison with public txids and chain confirmation evidence; a separate instance demonstrates refund, and another demonstrates cancel, with all source-defined action coverage retained

#### Scenario: Preview race and result
- WHEN two competing valid bodies use a fresh Preview head
- THEN the original-pair race satisfies CS07 or is honestly reported incomplete; the full sprint exit stays open until it satisfies CS07 and independent result review passes

### Requirement CS11: Bounded execution and retained failures
The system SHALL use existing approved resource limits and the bootstrap/time recipe (1000ms, margin30 slots, windows60 slots, horizon1800s). Each attempt SHALL have a new owned directory, short socket path, consistent child HOME/TMPDIR, source/tool/input pins captured before execution, bounded command/poll/fee-iteration limits and retained stdout/stderr/exit. Repeating a failed defect class twice SHALL trigger a concrete reproducer and changed repair strategy. This sprint SHALL not add a scheduler, audit framework, dependency reinstall, CML WASM modification or new language feature.

## Exit matrix

| Stage | Required result | Current417 status |
| --- | --- | --- |
| Expected outcomes | Independent Meta Muse Code/Wonder session review of this spec/plan | Pending; not source admission |
| Offline implementation | Actual consumer positives and independent negatives; full compile/build/apply | Partial, UNIT only |
| Full source audits | Two fresh exact-current full compiler+ledger approvals | Missing |
| Local ledger | CS01–CS09 with actual runtime receipts and result review | Unexecuted |
| Cardano Preview | CS01–CS08 andCS10 with all action/race results and result review | Specified-only |
| Wider program | All20/privatePCD/general obligations/final matrix/graph | Retained open |

---

## Complete PLAN.md

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

---

## Complete DIAGNOSTIC.md

# Current417 production diagnosis and review boundary

Repository observations,2026-10-14. No source changes or native commands executed in this planning turn. The Moriarty development skill and writing-plans skill were loaded; guarded status was refreshed through the main checkout CLI because the linked candidate uses the preserved experiment tree. It reports stale SP01.6 bindings/accounting and no pending financial transactions. That older guarded campaign is not current Cardano source acceptance and is not bypassed by this plan.

The next demonstrable capability is one profile/5 escrow source/Core compiled to UPLC, parameter-applied to actual current Cardano instance identities, funded/opened/spent on the real isolated ledger, then Cardano Preview. Wider20 coverage follows; it is not a prerequisite substitute for the first real contract.

## Concrete missing production chain

| ID | Observed mechanism | Required implementation outcome |
| --- | --- | --- |
| D1 | runNativeCaller2920 explicitly refuses PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE | Remove only after the actual context/measurement consumer meets CS01–CS04; do not flip the flag |
| D2 | assembleConsumedFinancial2408+ uses UNIT_CASH_POLICY/UNIT_HEAD_POLICY, fixture spend script and OFFLINE_PROTOCOL_PARAMS | Every mint/head/spend uses current approved Cash/head artifact/policy, queried role outrefs and matching protocol |
| D3 | buildApprovedFunding1763+ uses supplied800000/400000 units and offline protocol and derives holding from unsigned mint txid | Real settle/query dependency before head input, measured exact-body cost/fee/minADA and current signatures |
| D4 | measuredCoin2112, convergeFee2132, recountSignedWitnessFee2160+ and plutusCostUnits2217 use offline protocol constant; witness seed assumption exists before recount | One production protocol receipt, all-output measured minima and converged actual witness/reference-script fee inputs |
| D5 | acquireBodyEvaluation982+ can return BODY_EVALUATED but still nativeBudgetMeasured:false and submitBlocked:true | Actual successful command result becomes a byte-bound measured receipt; downstream admission additionally requires all output minima |
| D6 | compareBudget1216+ leaves protocolMinAda unmeasured and requires minAdaSource/minAdaMeasured inputs not carried by caller3030 | Checked actual minima/protocol/body receipt with caps, no caller flags or whole-tx minADA formula |
| D7 | current source admission/economic comparison is tied to local domain yaci-isolated-t1 and t1-custody-revision03 | Cardano Preview needs explicit checked network/genesis/magic/domain-instance binding; no local config reuse |
| D8 | readback has full semantic comparator but current assetQty selects name; payout credential and policy-qualified asset equality must be bound to context | Full Head13/Lifecycle9 plus actual role address, current Head/Cash policies, payout and input consumption |
| D9 | race original2 submits/full-post checks exist, but runOwnedRace always returns ok:false/ledger flags:false; presence polling alone is not finality | Native success derives original pair receipts plus complete expected state/effects and named confirmation; UNIT remains false |
| D10 | original clock/compile/strict amount defects were repaired locally, but genuine runtime forward path has not executed | Preserve regression controls; prove actual calibrated source/artifact/current identities under both full source audits and later result review |

The approved Cash source is already available and parameter-applied by buildApprovedCashPolicy, separate from closed332f90 fixture. Bootstrap155 has two scoped design approvals160. These facts authorize routine wiring repairs; neither is source or ledger approval. The historical linked-node ABI gaps stay unknown and must be reported beside actual tool/runtime observations; they are not a reason to call Cardano infeasible.

## Retained failures and scoped evidence

Both prior full audits391/392 reject the older full candidate. Their essential findings are production clock/sibling reachability, five stale LayerS success expectations, original loser resubmission/full-post gap, missing/null amount default and incomplete genuine body-budget-minADA chain. They do not approve417. Scoped compiler400/407 observations are not full approvals.

417 attempt003 reproduced forged fully pinned transcript admission; attempt005 reproduced GUARD_FAILED when actual terminal consumption used original arguments on substituted source. Attempt007 reproduced dangling Aiken build/packages dependency, with a disclosed historical t230 output-root mistake. Attempt012 reproduced missing production compiler-callback refusal. Subsequent controls repair those measured cases. Current417 results:36/36 custody/compiler tests including actual94 Aiken checks/build/application and CLI compile,16/16 consumer controls,24/24 parity/time/race controls,68/68 quantity controls. They establish their named UNIT/offline predicates only. No current full source admissions and no actual node/genesis/keygen/fund/mint/sign/submit/query/latch exist.

## Outcome-review questions

1. Do CS01–CS04 force actual source/Core→UPLC and current Cash/head/protocol/body provenance, including the circular-looking bootstrap/parity/calibrated-output sequence without permitting pre-audit ledger commands?
2. Are complete state/policy/role/payout/work/IDs/transport and finality/original-pair receipt criteria sufficient and independently testable, with UNIT controls unable to grant production authority?
3. Does the local→Cardano Preview exit retain existing economic/bootstrap/resource authority and all three source actions, without confusing Cardano and Midnight Preview or granting new public-money authority?
4. Are the proposed context/measurement APIs and actual consumer file changes a minimal coherent repair, and are any additional required positives/negatives missing?

The outcome reviewers receive self-contained excerpts and retained evidence in REVIEW-INPUT.md. This review is expected-outcome/design scope, not full383-source/native admission, not execution authorization and not actual-result approval. Parent freezes/publishes the spec and obtains reviews before source changes.

---

## Focused current417 code excerpts

These are actual source excerpts with line numbers and whole-file SHA256, selected for outcome review. They are not whole-source coverage. All production commands remain unexecuted.

### src/native-caller.mjs — SHA256 3c621003f9d4c5e00e9b5efafd21ee058aebb2d8886569b199670561b1c9f469

```js
225: export function compileGuardSibling({ compilerBin, sourcePath, previousProgramHash, evidenceRoot = null, production = null } = {}) {
226:   if (!compilerBin || !existsSync(compilerBin) || !sourcePath || !existsSync(sourcePath)) {
227:     return closed('SIBLING_NOT_COMPILED_IN_THIS_RUN', { siblingEmitted: false, spawned: false });
228:   }
229:   if (!realAiken(compilerBin)) {
230:     const result = spawnSync(compilerBin, ['check', sourcePath], { encoding: 'utf8' });
231:     const hashLine = /programHash=([0-9a-f]{56,64})/.test(String(result.stdout || ''));
232:     return closed('HASH_LINE_NOT_A_COMPILE', {
233:       siblingEmitted: false,
234:       spawned: true,
235:       unitOnly: true,
236:       programHashIgnored: hashLine,
237:       compilerIdentity: 'path-is-not-aiken',
238:       cliStatus: result.status,
239:     });
240:   }
241:   const pin = stdlibTreeSha256(STDLIB_TREE);
242:   if (!pin.ok || pin.sha256 !== STDLIB_TREE_SHA256 || pin.files !== 65) {
243:     return closed('STDLIB_PIN_MISMATCH', { siblingEmitted: false, spawned: false, pin });
244:   }
245:   const sourceText = readFileSync(sourcePath, 'utf8');
246:   const sourceHash = createHash('sha256').update(sourceText).digest('hex');
247:   const manifest = JSON.parse(readFileSync(MANIFEST_PATH, 'utf8'));
248:   if (production) {
249:     const bounds = production.measuredBounds;
250:     const seed = production.queriedSeed;
251:     const authentic = isAuthenticProductionAdmission(production) && production.code === 'PARITY_ADMITTED'
252:       && production.nativeProbe === true
253:       && production.captureOrigin === 'executeParityCapture'
254:       && bounds?.releaseLower && bounds?.partition && bounds?.refundInclusive
255:       && /^[0-9a-f]{64}$/.test(String(production.genesisSha256 || ''))
256:       && seed && !isFixedUtxoProbe(seed);
257:     if (!authentic) {
258:       return closed('CALIBRATED_SOURCE_NOT_THIS_PHASE', {
259:         siblingEmitted: false,
260:         productionAdmitted: false,
261:         calibrated: false,
262:         nativeProbe: false,
263:       });
264:     }
265:     const outDir = join(evidenceRoot || join(tmpdir(), 'moriarty-sibling'), 'production-compile');
266:     const compiled = compileCustody({
267:       sourcePath,
268:       seedPath: join(DESIGNER, 'financial-seed-custody-v5.json'),
269:       casesPath: join(DESIGNER, 'cases.json'),
270:       expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
271:       expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
272:       residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
273:       manifest,
274:       outDir,
275:       syntheticTime: {
276:         evidenceClass: 'node-genesis-v3-probe',
277:         productionArtifact: true,
278:         unitFixture: false,
279:         probeExecuted: true,
280:         genesisSha256: production.genesisSha256,
281:         queriedSeed: seed,
282:         bounds,
283:         productionAdmission: production,
284:       },
285:     });
286:     const programHash = compiled.hashes?.programHash || null;
287:     if (!compiled.ok || !programHash || programHash === previousProgramHash || compiled.hashes?.sourceHash === sourceHash) {
288:       return closed(compiled.code || 'SIBLING_COMPILE_REJECTED', {
289:         siblingEmitted: false,
290:         productionAdmitted: false,
291:         calibrated: false,
292:         nativeProbe: false,
293:         detail: compiled.detail || null,
294:       });
295:     }
296:     return {
297:       ok: true,
298:       code: 'SIBLING_COMPILED',
299:       siblingEmitted: true,
300:       calibrated: true,
301:       productionAdmitted: true,
302:       productionArtifact: true,
303:       nativeProbe: true,
304:       spawned: true,
305:       compilerInvoked: true,
306:       cacheHit: false,
307:       programHash,
308:       boundsHash: compiled.hashes.boundsHash,
309:       sourceHash: compiled.hashes.sourceHash,
310:       schemaHash: compiled.hashes.schemaHash,
311:       seedHash: compiled.hashes.seedHash,
312:       blueprintSha256: compiled.blueprintPath ? fileHash(compiled.blueprintPath) : null,
313:       blueprintPath: compiled.blueprintPath,
314:       sourcePath: join(outDir, 'sibling.mori'),
315:       applied: compiled.applied,
316:       productionAdmission: production,
317:       zeroTime: production.zeroTime,
318:       zeroSlot: production.zeroSlot,
319:       measuredBounds: bounds,
320:       genesisSha256: production.genesisSha256,
321:       queriedSeed: seed,
322:       siblingBudgetBound: false,
323:       minAdaMeasured: false,
324:       protocolMinAda: 'unmeasured',
325:       stdlibSha256: STDLIB_TREE_SHA256,
326:       stdlibFiles: 65,
327:       previousProgramHash,
328:       unitOnly: false,
329:       compilerIdentity: 'aiken-realpath',
330:       ledgerExecution: false,
331:       nativePins: false,
332:     };
333:   }
334:   const cachePath = sourceCachePath(sourceHash);
```

```js
1135:   else if (winner && !observed) code = 'RACE_LOSER_RECEIPT_ABSENT';
1136:   else if (winner && observed && post.postCode === 'RACE_POST_MISMATCH') code = 'RACE_POST_MISMATCH';
1137:   else if (unitOnly) code = 'RACE_UNIT_OBSERVATION';
1138:   else if (winner && observed && post.postCode === 'ECONOMIC_ACTION_MATCHED') code = 'RACE_OBSERVED';
1139:   else if (winner && observed) code = 'RACE_INCOMPLETE';
1140:   const replayExecuted = unitOnly !== true && code === 'RACE_OBSERVED';
1141:   const replay = {
1142:     separate: true,
1143:     executed: replayExecuted,
1144:     observed,
1145:     expectedClass: 'BadInputsUTxO',
1146:     ledgerSubmitted: false,
1147:     assumedSecondIsLoser: false,
1148:     loserTxFile: loser?.txFile || null,
1149:     winnerTxid: winner?.txid || null,
1150:     resubmitted: false,
1151:   };
1152:   return {
1153:     ok: false,
1154:     code,
1155:     submitCount: submits.length,
1156:     submitsBeforeConfirms: submits.length === 2,
1157:     events,
1158:     resubmitted: false,
1159:     winnerOutRef,
1160:     postCode: post.postCode,
1161:     postJudge: post.judged?.code || null,
1162:     winnerReadback: post.readback || null,
1163:     loserCode: winner && !observed ? 'RACE_LOSER_RECEIPT_ABSENT' : null,
1164:     submissions: sides.map((side) => ({
1165:       actor: side.actor,
1166:       resource: side.resource,
1167:       submitText: side.submitText,
1168:       submitStatus: side.submitStatus,
1169:       submittedAt: side.submittedAt,
1170:       txid: side.txid || null,
1171:       outRef: side.txid ? `${side.txid}#0` : null,
1172:       bodyFile: side.bodyFile,
1173:     })),
1174:     actors: [left.actor, right.actor],
1175:     resources: [left.resource, right.resource],
1176:     sharedHead: pair.members[0].head,
1177:     overlap: overlap || null,
1178:     cashQuantity,
1179:     refundRaced: overlap === 'release-refund',
1180:     confirmed: false,
1181:     failureClass: observed ? 'BadInputsUTxO' : null,
1182:     ledgerExecution: false,
1183:     ledgerSubmitted: false,
1184:     submitted: unitOnly ? false : submits.length === 2,
1185:     assembled: true,
1186:     genesisMagicBound: true,
1187:     networkMagic: Number(boundMagic.text),
1188:     nativePins: false,
1189:     unitOnly,
1190:     assumedSecondIsLoser: false,
1191:     poll: 'query-utxo-tx-in',
1192:     winnerTxid: winner?.txid || null,
1193:     winnerActor: winner?.actor || null,
1194:     replay,
1195:   };
```

```js
1763: export function buildApprovedFunding(stages, request, spec) {
1764:   const manifest = JSON.parse(readFileSync(MANIFEST_PATH, 'utf8'));
1765:   const hashes = stages.source?.hashes;
1766:   if (!stages.inputs?.cashSeed || !stages.inputs.mintFee || !stages.inputs.mintCollateral) {
1767:     return {
1768:       cashCompile: null,
1769:       headApply: null,
1770:       mint: closed('CASH_SEED_ABSENT', { cashCanMint: false, ledgerFunding: false }),
1771:       body: closed('HEAD_OPEN_BLOCKED', { cashCanMint: false, ledgerFunding: false }),
1772:     };
1773:   }
1774:   if (!hashes?.programHash || !stages.source?.blueprintPath) {
1775:     return {
1776:       cashCompile: null,
1777:       headApply: null,
1778:       mint: closed('SOURCE_COMPILE_FAILED', { compiled: false, cashCanMint: false }),
1779:       body: closed('HEAD_OPEN_BLOCKED', { cashCanMint: false }),
1780:     };
1781:   }
1782:   const seed = stages.inputs.cashSeed;
1783:   const cashRoot = request.evidenceRoot ? join(request.evidenceRoot, 'checks', 'cash-policy') : CASH_WORK;
1784:   const workDir = join(cashRoot, `${seed.txId}-${seed.index}`);
1785:   const frozenCash = join(FROZEN_CASH_WORK, `${seed.txId}-${seed.index}`, 'cash-policy.json');
1786:   if (!existsSync(join(workDir, 'cash-policy.json')) && existsSync(frozenCash)) {
1787:     mkdirSync(workDir, { recursive: true });
1788:     writeFileSync(join(workDir, 'cash-policy.json'), readFileSync(frozenCash));
1789:   }
1790:   const cashCompile = buildApprovedCashPolicy({
1791:     sourcePath: request.cashDesign.sourcePath,
1792:     seedTxId: seed.txId,
1793:     seedIndex: seed.index,
1794:     workDir,
1795:   });
1796:   if (!cashCompile.ok) {
1797:     return { cashCompile, headApply: null, mint: { ...cashCompile, cashCanMint: false }, body: closed('HEAD_OPEN_BLOCKED', { cashCanMint: false }) };
1798:   }
1799:   const head = stages.inputs.head;
1800:   const headApply = applyHeadToCashPolicy({
1801:     blueprintPath: stages.source.blueprintPath,
1802:     cashPolicyId: cashCompile.policyId,
1803:     hashes,
1804:     manifest,
1805:     seedTxId: head.txId,
1806:     seedIndex: head.index,
1807:     workDir: join(workDir, 'head'),
1808:   });
1809:   if (!headApply.ok) {
1810:     return { cashCompile, headApply, mint: closed('HEAD_APPLY_FAILED', { cashCanMint: false }), body: headApply };
1811:   }
1812:   const cashRef = inputRef(seed);
1813:   const headRef = inputRef(head);
1814:   const common = {
1815:     outputLovelace: spec.outputLovelace,
1816:     feeLovelace: spec.feeLovelace,
1817:     invalidBefore: spec.invalidBefore,
1818:     invalidHereafter: spec.invalidHereafter,
1819:     requiredSignerHash: spec.requiredSignerHash,
1820:     paymentVkeyCbor: spec.paymentVkeyCbor,
1821:     networkMagic: spec.networkMagic ?? request.networkMagic ?? request.magic ?? null,
1822:     offlineExecutionUnits: { memory: 800000, steps: 400000 },
1823:     workRoot: workDir,
1824:   };
1825:   const offlineBody = {
1826:     protocolParamsPath: OFFLINE_PROTOCOL_PARAMS,
1827:   };
1828:   const payoutVkeyHex = request.payoutVkeyHex || request.payoutVkeyCbor || `5820${'34'.repeat(32)}`;
1829:   const mint = encodeUnsignedBody({
1830:     ...common,
1831:     ...offlineBody,
1832:     head: seed,
```

```js
2112: async function measuredCoin(request, workDir, address, datum, assets, referenceScriptPath) {
2113:   const protocolPath = OFFLINE_PROTOCOL_PARAMS;
2114:   const protocolSha = createHash('sha256').update(readFileSync(protocolPath)).digest('hex');
2115:   const calculated = await calculateOutputMinimum({
2116:     cliPath: request.cliPath || CARDANO_CLI,
2117:     protocolPath,
2118:     protocolProvenance: protocolSha,
2119:     cliIdentity: unitCliIdentity(request),
2120:     workDir,
2121:     output: {
2122:       address,
2123:       lovelace: '0',
2124:       assets,
2125:       datumData: datum || undefined,
2126:       referenceScriptPath: referenceScriptPath || undefined,
2127:     },
2128:   });
2129:   return { ...calculated, protocolSha };
2130: }
2131: 
2132: async function convergeFee(encodeAtFee, request, workDir, referenceScriptSize, coin) {
2133:   const protocolPath = OFFLINE_PROTOCOL_PARAMS;
2134:   const parts = protocolFeeParts(protocolPath);
2135:   if (!parts) return { ok: false, code: 'FEE_UNMEASURED', body: null, fee: null };
2136:   let fee = feeSeed(parts, coin);
2137:   if (!fee) return { ok: false, code: 'FEE_UNMEASURED', body: null, fee: null };
2138:   let body = null;
2139:   let feeResult = null;
2140:   for (let pass = 0; pass < 4; pass += 1) {
2141:     body = encodeAtFee(fee);
2142:     if (!body.ok) return { ok: false, code: body.code, body, fee, feeResult };
2143:     feeResult = await calculateBodyMinimumFee({
2144:       cliPath: request.cliPath || CARDANO_CLI,
2145:       protocolPath,
2146:       protocolProvenance: createHash('sha256').update(readFileSync(protocolPath)).digest('hex'),
2147:       cliIdentity: unitCliIdentity(request),
2148:       bodyPath: body.bodyFile,
2149:       workDir: join(workDir, `fee-${pass}`),
2150:       witnessCount: 1,
2151:       witnessCountProvenance: 'ASSUMED-unsigned-body-no-key-analysis',
2152:       referenceScriptSize,
2153:       requiresReferenceScriptFee: referenceScriptSize != null,
2154:     });
2155:     const next = String(feeResult.parsed?.fee || '');
2156:     if (!/^[1-9][0-9]*$/.test(next) || next === '100' || next === body.quantities.outputLovelace) {
2157:       return { ok: false, code: 'FEE_UNMEASURED', body, fee: next, feeResult };
2158:     }
2159:     if (next === fee) return { ok: true, code: 'FEE_CONVERGED', body, fee, feeResult, passes: pass + 1 };
2160:     fee = next;
2161:   }
2162:   return { ok: false, code: 'FEE_DRIFT', body, fee, feeResult };
```

```js
2217: function plutusCostUnits(cliPath, bodyFile, context = {}) {
2218:   const before = createHash('sha256').update(readFileSync(cliPath)).digest('hex');
2219:   const mode = context.mode === 'online' ? 'online' : 'offline';
2220:   const built = plutusCostArgv({
2221:     cli: cliPath,
2222:     txFile: bodyFile,
2223:     mode,
2224:     startTime: context.startTimeUtc,
2225:     startTimePosix: context.startTimePosix,
2226:     byronGenesis: context.byronGenesis,
2227:     eraHistoryFile: context.eraHistoryFile,
2228:     utxoFile: context.utxoFile,
2229:     protocolParamsFile: OFFLINE_PROTOCOL_PARAMS,
2230:     socket: context.socket,
2231:     magic: context.magic,
2232:     magicSource: context.magicSource,
2233:   });
2234:   if (!built.ok) {
2235:     return {
2236:       cost: { ok: false, code: built.code || 'EVALUATION_CONTEXT_ABSENT' },
2237:       cliSha256: before,
2238:       cliSha256After: before,
2239:       cliDigestStable: true,
2240:       status: null,
2241:       spawned: false,
2242:     };
2243:   }
2244:   if (mode === 'online' && realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
2245:     return {
2246:       cost: { ok: false, code: 'REAL_NODE_NOT_ARMED', command: built.command },
2247:       cliSha256: before,
2248:       cliSha256After: before,
2249:       cliDigestStable: true,
2250:       status: null,
2251:       spawned: false,
2252:     };
2253:   }
2254:   const args = built.argv.slice(1);
2255:   const run = spawnSync(cliPath, args, { encoding: 'utf8' });
2256:   const after = createHash('sha256').update(readFileSync(cliPath)).digest('hex');
2257:   let parsed = run.stdout;
2258:   try { parsed = JSON.parse(run.stdout); } catch { /* consumePlutusCost rejects text that is not a cost list */ }
2259:   let cost = run.status === 0 ? consumePlutusCost(parsed, { unitOnly: !realCli(cliPath) }) : { ok: false, code: 'EVALUATION_UNAVAILABLE' };
2260:   if (cost.ok === true && cost.cost?.memory === 1 && cost.cost?.steps === 1) {
2261:     cost = { ...cost, ok: false, code: 'SUPPLIED_UNITS_NOT_A_MEASUREMENT' };
2262:   }
2263:   if (cost.ok === true && Array.isArray(cost.items) && cost.items.some((item) => !/^[0-9a-f]{56}$/i.test(String(item.scriptHash || '')))) {
2264:     cost = { ...cost, ok: false, code: 'SCRIPT_HASH_REQUIRED' };
2265:   }
2266:   return { cost, cliSha256: before, cliSha256After: after, cliDigestStable: before === after, status: run.status };
2267: }
2268: 
```

```js
2408: export async function assembleConsumedFinancial(stages, request) {
2409:   const inputs = stages.inputs;
2410:   const workDir = join(request.evidenceRoot || join(tmpdir(), 'moriarty-financial'), 'assemble');
2411:   mkdirSync(workDir, { recursive: true });
2412:   const datum = request.detailedData;
2413:   if (!datum || datum.constructor !== 0 || !Array.isArray(datum.fields) || datum.fields.length !== 13) {
2414:     const absent = closed('DETAILED_DATUM_ABSENT', { fieldCount: Array.isArray(datum?.fields) ? datum.fields.length : 0 });
2415:     return { outputFee: absent, mint: absent, body: absent, rebuild: absent, spend: absent };
2416:   }
2417:   const magic = stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null;
2418:   const magicText = genesisMagicText(magic);
2419:   if (!magicText.ok) {
2420:     const refused = closed(magicText.code, { spawned: false });
2421:     return { outputFee: refused, mint: refused, body: refused, rebuild: refused, spend: refused };
2422:   }
2423:   const headInterval = consumedHeadInterval(stages.time?.windows);
2424:   if (!headInterval.ok) {
2425:     const window = closed(headInterval.code || 'TIME_WINDOW_ABSENT', { spawned: false, cliIntervalEmitted: false });
2426:     return { outputFee: window, mint: window, body: window, rebuild: window, spend: window };
2427:   }
2428:   const vkey = String(request.paymentVkeyCbor || '');
2429:   const address = enterpriseAddress(request.cliPath || CARDANO_CLI, vkey, magicText.text, workDir);
2430:   if (!address.ok) {
2431:     const failed = closed(address.code, { detail: address.detail, spawned: false });
2432:     return { outputFee: failed, mint: failed, body: failed, rebuild: failed, spend: failed };
2433:   }
2434:   const refStat = request.referenceScriptPath ? statSync(request.referenceScriptPath) : null;
2435:   if (request.referenceScriptPath && (!refStat?.isFile())) {
2436:     const missing = closed('REFERENCE_SCRIPT_MISSING', { spawned: false });
2437:     return { outputFee: missing, mint: missing, body: missing, rebuild: missing, spend: missing };
2438:   }
2439:   const referenceScriptSize = refStat ? refStat.size : null;
2440:   const cashAssets = [{ policyId: UNIT_CASH_POLICY, assetNameHex: '43617368', quantity: 100 }];
2441:   let coin;
2442:   try {
2443:     coin = await measuredCoin(request, join(workDir, 'coin'), address.address, datum, cashAssets, request.referenceScriptPath || null);
2444:   } catch (error) {
2445:     const failed = closed(error?.code || 'HELPER77_FAILED', { detail: error?.message || 'coin-threw', spawned: false });
2446:     return { outputFee: failed, mint: failed, body: failed, rebuild: failed, spend: failed };
2447:   }
2448:   const measuredText = firstCoinText([
2449:     coin.minimumLovelace,
2450:     coin.lovelaceText,
```

```js
2480:     eraHistoryFile,
2481:     utxoFile,
2482:     socket: request.costSocket || request.readback?.socket || request.time?.socket || '',
2483:     magic: Number(magicText.text),
2484:     magicSource: request.costMagicSource || stages.ownedProducer?.schema71?.magicSource || null,
2485:   };
2486:   const base = {
2487:     outputLovelace,
2488:     invalidBefore: headInterval.invalidBefore,
2489:     invalidHereafter: headInterval.invalidHereafter,
2490:     paymentVkeyCbor: vkey,
2491:     networkMagic: Number(magicText.text),
2492:     protocolParamsPath: OFFLINE_PROTOCOL_PARAMS,
2493:     requireChange: true,
2494:   };
2495:   const mintFile = join(workDir, 'cash-mint.body.json');
2496:   const encodeMint = (fee, units = null) => encodeUnsignedBody({
2497:     ...base,
2498:     feeLovelace: fee,
2499:     head: inputs.cashSeed,
2500:     fee: inputs.mintFee,
2501:     collateral: inputs.mintCollateral,
2502:     inputValues: valuesFor(inputs, ['cashSeed', 'mintFee', 'mintCollateral']),
2503:     outFile: mintFile,
2504:     offlineExecutionUnits: units,
2505:     mintAssets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
2506:     assets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
2507:   }, request.cliPath);
2508:   const minted = await convergeFee(encodeMint, request, join(workDir, 'mint-fee'), referenceScriptSize, outputLovelace);
2509:   if (!minted.ok) {
2510:     const mint = {
2511:       ...(minted.body || closed(minted.code)),
2512:       cashPolicyId: UNIT_CASH_POLICY,
2513:       compiledScript: false,
2514:       markerNotNativeFunding: true,
2515:       cashCanMint: false,
```

```js
2548:     fee: inputs.headFee,
2549:     collateral: inputs.headCollateral,
2550:     extraTxIns: [holding],
2551:     inputValues: headInputs,
2552:     outFile: headFile,
2553:     offlineExecutionUnits: units,
2554:     inlineDatumJson: JSON.stringify(datum),
2555:     mintAssets: [{ quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' }],
2556:     assets: [
2557:       { quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' },
2558:       { quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' },
2559:     ],
2560:   }, request.cliPath);
2561:   const openedFee = await convergeFee(encodeHead, request, join(workDir, 'head-fee'), referenceScriptSize, outputLovelace);
2562:   let body = openedFee.body ? {
2563:     ...openedFee.body,
2564:     holding,
2565:     mintTxid: mint.txid,
2566:     mintSeparate: mint.ok === true,
2567:     headMintsCash: false,
2568:     kernelCash: '100',
```

```js
2900:       const releaseUpper = (partition - 1n).toString(10);
2901:       request.productionSiblingAdmitted = true;
2902:       request.calibratedSiblingTerminals = {
2903:         release: { lower: bounds.releaseLower, upper: releaseUpper },
2904:         cancel: { lower: bounds.releaseLower, upper: releaseUpper },
2905:         refund: { lower: bounds.partition, upper: bounds.refundInclusive },
2906:       };
2907:     }
2908:     const cliIsReal = realCli(request.cliPath || '');
2909:     const timeOpen = request.time != null || request.parityCapture != null;
2910:     const nativeBodyBlocked = request.financialBody === true && cliIsReal && (
2911:       timeOpen ? stages.productionTime?.code !== 'PARITY_ADMITTED' : !request.cashDesign
2912:     );
2913:     // The connected encoder still uses UNIT policy/script/budget inputs. A genuine
2914:     // parity receipt cannot promote those placeholders into a production body.
2915:     const productionConsumerIncomplete = request.financialBody === true && cliIsReal
2916:       && stages.sibling?.productionAdmitted === true;
2917:     if (productionConsumerIncomplete) {
2918:       stages.body = closed('PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE', {
2919:         submitBlocked: true, bodyBuilt: false, cliIntervalEmitted: false,
2920:         reason: 'current-policy-script-budget-minAda-binding-required',
2921:       });
2922:       stages.spend = closed('PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE', { bodyBuilt: false, encodeSpendBodyCalled: false });
2923:     }
2924:     if (nativeBodyBlocked) {
2925:       stages.body = closed('PARITY_NOT_ADMITTED', { ledgerFunding: false, submitBlocked: true, bodyBuilt: false, cliIntervalEmitted: false });
2926:       stages.spend = closed('PARITY_NOT_ADMITTED', { bodyBuilt: false, cliIntervalEmitted: false });
2927:     }
2928:     const constraintBlocked = stages.budget?.code === 'BUDGET_EXCEEDED';
2929:     if (!nativeBodyBlocked && !productionConsumerIncomplete && request.buildBody && stages.inputs?.ok && !constraintBlocked) {
2930:       const spec = bodySpec(stages, request);
2931:       if (stages.time?.code === 'SLOT_OBSERVED' && spec.timeSeparation?.ok !== true) {
2932:         const refused = spec.timeSeparation?.code || 'TIME_WINDOW_ABSENT';
2933:         stages.body = closed(refused, { ledgerFunding: false, submitBlocked: true, bodyBuilt: false, cliIntervalEmitted: false });
2934:         stages.spend = closed(refused, { bodyBuilt: false, cliIntervalEmitted: false, encodeSpendBodyCalled: false });
2935:       } else if (request.financialBody && stages.inputs?.twelveInputFrameSatisfied === true) {
2936:         const consumed = await assembleConsumedFinancial(stages, request);
2937:         stages.outputFee = consumed.outputFee;
2938:         stages.mint = consumed.mint;
2939:         stages.body = consumed.body;
2940:         stages.rebuild = consumed.rebuild;
2941:         stages.spend = consumed.spend;
2942:         stages.posts = consumed.posts;
2943:       } else if (request.financialBody && stages.cashDesign?.designAdopted === true) {
2944:         const funded = await buildApprovedFunding(stages, request, spec);
2945:         stages.cashCompile = funded.cashCompile;
2946:         stages.headApply = funded.headApply;
2947:         stages.mint = funded.mint;
2948:         stages.body = funded.body;
2949:         stages.posts = funded.posts;
2950:         if (funded.spend) {
2951:           const failed = ['release', 'refund', 'cancel'].map((kind) => funded.spend[kind]).find((item) => item?.ok === false);
2952:           stages.spend = failed
2953:             ? { ...funded.spend, ok: false, code: failed.code }
2954:             : { ...funded.spend, ok: true, code: 'SPEND_BODIES_ENCODED' };
2955:         }
2956:       } else if (request.financialBody && stages.cashDesign && stages.cashDesign.ok === false) {
2957:         stages.mint = closed(stages.cashDesign.code, { cashCanMint: false, compiledScript: false, ledgerFunding: false });
```

```js
2990:       } else {
2991:         stages.body = encodeUnsignedBody(spec, request.cliPath);
2992:       }
2993:       if (stages.body?.ok && stages.sibling?.programHash) {
2994:         stages.body = {
2995:           ...stages.body,
2996:           guardProgramHash: stages.sibling.programHash,
2997:           guardSourceHash: stages.sibling.sourceHash,
2998:           guardUsed: stages.sibling.programHash !== stages.sibling.previousProgramHash,
2999:         };
3000:       }
3001:     } else if (request.buildBody && constraintBlocked) {
3002:       stages.body = closed('BUDGET_EXCEEDED', { submitBlocked: true });
3003:     }
3004:     if (stages.body?.ok && stages.budget?.code === 'EVALUATION_UNAVAILABLE') {
3005:       const bodyPath = stages.spend?.release?.bodyFile || stages.body.bodyFile || '';
3006:       const measured = acquireBodyEvaluation({
3007:         cliPath: request.cliPath || CARDANO_CLI,
3008:         bodyPath,
3009:         socket: request.protocol?.socket || request.utxoQuery?.socket || request.readback?.socket || '',
3010:         execute: true,
3011:       });
3012:       stages.evaluation = measured;
3013:       stages.budget = {
3014:         ...measured,
3015:         ok: measured.nativeBudgetMeasured === true,
3016:         submitBlocked: measured.nativeBudgetMeasured !== true,
3017:         suppliedCostIgnored: measured.nativeBudgetMeasured !== true,
3018:         nativeBudgetMeasured: measured.nativeBudgetMeasured === true,
3019:         minAdaMeasured: false,
3020:         protocolMinAda: 'unmeasured',
3021:         code: measured.nativeBudgetMeasured === true ? 'BODY_EVALUATED' : (measured.code || 'EVALUATION_UNAVAILABLE'),
3022:       };
3023:       if (measured.nativeBudgetMeasured === true && stages.protocol) {
3024:         const compared = compareBudget({
3025:           queried: true,
3026:           source: 'cardano-cli-query',
3027:           unitOnly: false,
3028:           queryProvenance: measured.queryProvenance,
3029:           measuredByEvaluator: true,
3030:           executionSource: 'body-evaluator',
3031:           maxTxExecutionUnits: stages.protocol.maxTxExecutionUnits,
3032:           maxTxSize: stages.protocol.maxTxSize,
3033:           utxoCostPerByte: stages.protocol.utxoCostPerByte,
3034:           cost: measured.cost,
3035:           txBytes: measured.txBytes,
3036:           offlineUnits: false,
3037:         });
3038:         stages.budget = { ...compared, evaluation: measured };
3039:       }
3040:     }
```

### src/native-runtime.mjs — SHA256 4b42f8452dc4500674bef41bbba2874bfdba64a9ad80ba5b2df9702dfafcaf76

```js
982: export function acquireBodyEvaluation({ cliPath = CARDANO_CLI, bodyPath = '', socket = '', execute = false, ...rest } = {}) {
983:   const held = {
984:     nativeBudgetMeasured: false,
985:     minAdaMeasured: false,
986:     submitBlocked: true,
987:     offlineUnitsAreNotMeasurement: true,
988:     protocolMinAda: 'unmeasured',
989:   };
990:   const command = 'conway transaction calculate-plutus-script-cost';
991:   if (rest.command === 'evaluate' || rest.evaluate === true) {
992:     return closed('NO_CLI_EVALUATE_COMMAND', { ...held, spawned: false, command });
993:   }
994:   if (!bodyPath || !existsSync(bodyPath)) return closed('EVALUATION_UNAVAILABLE', { ...held, reason: 'body-absent', spawned: false, command });
995:   if (execute !== true) return closed('EVALUATION_UNAVAILABLE', { ...held, reason: 'not-run', spawned: false, command });
996:   const plan = plutusCostArgv({ ...rest, cli: cliPath, cliPath, bodyPath, socket });
997:   if (realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
998:     return closed('REAL_QUERY_NOT_ARMED', { ...held, spawned: false, command, argv: plan.argv || null });
999:   }
1000:   if (!plan.ok) return closed(plan.code, { ...held, spawned: false, command: plan.command || command, argv: plan.argv || null });
1001:   if (plan.mode !== 'offline' && (!socket || !existsSync(socket))) {
1002:     return closed('EVALUATION_UNAVAILABLE', { ...held, reason: 'socket-absent', spawned: false, command, argv: plan.argv });
1003:   }
1004:   const result = spawnSync(cliPath, plan.argv.slice(1), { encoding: 'utf8' });
1005:   let parsed = result.stdout || '';
1006:   try { parsed = JSON.parse(parsed); } catch { /* a non-JSON cost body fails closed in the parser */ }
1007:   const consumed = consumePlutusCost(parsed, { unitOnly: !realCli(cliPath) });
1008:   if (!consumed.ok || result.status !== 0) {
1009:     return closed(consumed.code || 'EVALUATION_UNAVAILABLE', {
1010:       ...held,
1011:       spawned: true,
1012:       unitOnly: !realCli(cliPath),
1013:       command,
1014:       cliStatus: result.status,
1015:     });
1016:   }
1017:   const measured = realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE === '1';
1018:   return {
1019:     ...consumed,
1020:     ok: measured,
1021:     code: measured ? 'BODY_EVALUATED' : 'PLUTUS_COST_PARSED',
1022:     spawned: true,
1023:     nativeBudgetMeasured: false,
1024:     submitBlocked: true,
1025:     executionSource: measured ? 'body-evaluator' : null,
1026:     measuredByEvaluator: measured,
1027:     queryProvenance: measured ? 'spawned-query' : 'unit-cli-spawn',
1028:     unitOnly: !measured,
1029:     command,
1030:     ledgerExecution: false,
1031:     nativePins: false,
1032:   };
1033: }
```

```js
1216: export function compareBudget(input = {}) {
1217:   const params = input.protocolParams || input;
1218:   if (params.queried !== true) {
1219:     return closed('PROTOCOL_PARAMS_ABSENT', { submitBlocked: true, nativeBudgetMeasured: false, minAdaMeasured: false });
1220:   }
1221:   const units = params.maxTxExecutionUnits;
1222:   const cost = input.cost || params.cost;
1223:   if (!units || cost == null || units.memory == null || units.steps == null || cost.memory == null || cost.steps == null) {
1224:     return closed('PROTOCOL_PARAMS_ABSENT', { submitBlocked: true, nativeBudgetMeasured: false, minAdaMeasured: false });
1225:   }
1226:   let over = false;
1227:   try {
1228:     over = BigInt(cost.memory) > BigInt(units.memory) || BigInt(cost.steps) > BigInt(units.steps);
1229:   } catch {
1230:     return closed('MALFORMED_DATA', { submitBlocked: true });
1231:   }
1232:   const sizeOver = params.maxTxSize != null && input.txBytes != null && Number(input.txBytes) > Number(params.maxTxSize);
1233:   const perByte = params.utxoCostPerByte ?? input.utxoCostPerByte ?? null;
1234:   const observedAlias = perByte == null ? (params.coinsPerUTxOByte ?? input.coinsPerUTxOByte ?? null) : null;
1235:   const rate = perByte ?? observedAlias;
1236:   const txBytes = input.txBytes;
1237:   const protocolMinAda = 'unmeasured';
1238:   const minAdaMeasured = input.minAdaSource === 'calculate-min-required-utxo' && input.minAdaMeasured === true;
1239:   const formulaRefusal = rate != null && txBytes != null && Number.isFinite(Number(txBytes))
1240:     ? refuseCallerMinAdaFormula(rate, txBytes)
1241:     : null;
1242:   const evaluatorMeasured = input.measuredByEvaluator === true
1243:     && input.executionSource === 'body-evaluator'
1244:     && params.source === 'cardano-cli-query'
1245:     && params.unitOnly !== true
1246:     && params.queryProvenance === 'spawned-query';
1247:   const nativeBudgetMeasured = evaluatorMeasured && minAdaMeasured && formulaRefusal == null;
1248:   const quantities = {
1249:     kernelCash: '100',
1250:     unitPayout: '2000000',
1251:     protocolMinAda,
1252:   };
1253:   if (over || sizeOver) {
1254:     return closed('BUDGET_EXCEEDED', {
1255:       submitBlocked: true,
1256:       nativeBudgetMeasured: false,
1257:       unitLogIsNotBudget: true,
1258:       minAdaMeasured,
1259:       quantities,
1260:     });
1261:   }
1262:   return {
1263:     ok: true,
1264:     code: 'BUDGET_WITHIN_SUPPLIED_MAX',
1265:     submitBlocked: evaluatorMeasured && minAdaMeasured !== true,
1266:     nativeBudgetMeasured,
1267:     unitOnly: params.unitOnly === true,
1268:     minAdaMeasured,
1269:     formulaRefusal,
1270:     minAdaCommand: 'conway transaction calculate-min-required-utxo',
1271:     quantities,
1272:     ledgerExecution: false,
1273:     nativePins: false,
1274:   };
1275: }
```

### src/native-financial.mjs — SHA256 e535dc4b9e5f8a3082305ca74de485d44ef49494e4daa52d008730d4924e9b12

```js
331: 
332: function terminalFromEval(kind, evaluated, args, preObligations) {
333:   if (!evaluated || evaluated.status !== 'FundedExpressionPrepared') {
334:     return closed(evaluated?.code || 'SOURCE_POST_REJECTED', { kind, status: evaluated?.status || null });
335:   }
336:   const post = evaluated.financialPost;
337:   const kept = assertObligationsPreserved(preObligations, post?.obligations);
338:   if (!kept.ok) return { ...kept, kind };
339:   const fence = fenceLifecycle(post);
340:   if (!fence.ok) return { ...fence, kind, bodyBuilt: false, fenceBypass: false };
341:   if (asInt(post?.work?.spent) !== CONSTRUCTOR_WORK) return closed('CONSTRUCTOR_WORK_MISMATCH', { kind, spent: post?.work?.spent });
342:   const effect = evaluated.effects?.[0];
343:   const effectAmount = effect ? asInt(effect.amount) : null;
344:   if (!effect || effect.asset !== 'Cash' || effectAmount !== 100) return closed('CASH_NOT_CONSERVED', { kind });
345:   const redeemer = redeemerFromArgs(kind, args);
346:   if (!redeemer.ok) return redeemer;
347:   // Custody core fixes the Cash effect at 100. The Plutus redeemer fields are
348:   // actor, recipient, transferId, lower, upper. The consumer projection copies
349:   // that evaluated effect amount and does not substitute the economic spec.
350:   return {
351:     ok: true,
352:     kind,
353:     financialPost: post,
354:     post: evaluated.post,
355:     effects: evaluated.effects,
356:     transportRevision: '1',
357:     transportRemaining: '0',
358:     args,
359:     redeemer: { ...redeemer, amount: effectAmount },
360:     constructorWorkCount: CONSTRUCTOR_WORK,
361:     collectionCapacity: COLLECTION_CAPACITY,
362:     continuingCash: 0,
363:     payoutCash: 100,
364:     headQuantity: 1,
365:   };
366: }
367: 
368: export function loadSourceTerminals(sourceText, timeBinding = null) {
369:   const inputFor = (kind, text) => {
370:     if (!timeBinding) return text;
371:     const ordinary = JSON.parse(text);
372:     const bounds = timeBinding.bounds;
373:     ordinary.Args.lower = kind === 'refund' ? bounds.partition : bounds.releaseLower;
374:     ordinary.Args.upper = kind === 'refund' ? bounds.refundInclusive : (BigInt(bounds.partition) - 1n).toString();
375:     return canonicalEncode(ordinary);
376:   };
377:   if (typeof sourceText !== 'string' || sourceText.length < 1000) return closed('SOURCE_POST_ABSENT');
378:   const cases = JSON.parse(readFileSync(join(DESIGNER, 'cases.json'), 'utf8'));
379:   const seedText = readFileSync(join(DESIGNER, 'financial-seed-custody-v5.json'), 'utf8');
380:   const residualText = readFileSync(join(DESIGNER, 'financial-residual-custody-v5.json'), 'utf8');
381:   const evaluator = createFinancialAgreementSourceV5();
382:   const preObligations = [];
383:   const terminals = {};
384:   for (const kind of ['release', 'refund']) {
385:     const spec = TERMINALS[kind];
386:     const row = pickCase(cases, spec.caseId);
387:     if (!row) return closed('SOURCE_POST_ABSENT', { kind });
388:     const input = inputFor(kind, row.ordinaryInputCanonicalJSON);
389:     const ordinary = JSON.parse(input);
390:     const evaluated = evaluator.evaluate(sourceText, spec.action, input, seedText);
391:     const built = terminalFromEval(kind, evaluated, ordinary.Args, preObligations);
392:     if (!built.ok) return built;
393:     terminals[kind] = built;
394:   }
395:   const cancelInput = inputFor('cancel', TERMINALS.cancel.input);
396:   const cancelOrdinary = JSON.parse(cancelInput);
397:   const cancelEval = evaluator.evaluate(sourceText, 'cancel', cancelInput, seedText);
398:   const cancel = terminalFromEval('cancel', cancelEval, cancelOrdinary.Args, preObligations);
399:   if (!cancel.ok) return cancel;
400:   terminals.cancel = cancel;
401:   const residualCase = pickCase(cases, 'E12-cancel-preserves-duty');
402:   const residualEval = residualCase
403:     ? evaluator.evaluate(sourceText, 'cancel', inputFor('cancel', residualCase.ordinaryInputCanonicalJSON), residualText)
404:     : null;
405:   const residualObligations = residualEval?.financialPost?.obligations || [];
406:   const residualFence = fenceLifecycle(residualEval?.financialPost || {});
407:   return {
408:     ok: true,
409:     code: 'SOURCE_POSTS_EVALUATED',
410:     terminals,
411:     residual: {
412:       bodyBuilt: false,
```

```js
434: export function expectedFrame(terminal, identity) {
435:   const post = terminal.financialPost;
436:   return {
437:     program_hash: identity.programHash || null,
438:     bounds_hash: identity.boundsHash || null,
439:     source_hash: identity.sourceHash,
440:     schema_hash: identity.schemaHash || null,
441:     seed_hash: identity.seedHash || null,
442:     domain_network: 'yaci-isolated-t1',
443:     domain_deployment: 't1-custody-revision03',
444:     balances: {
445:       Escrow: partyAmount(post.balances, 'Escrow'),
446:       Borrower: partyAmount(post.balances, 'Borrower'),
447:       Lender: partyAmount(post.balances, 'Lender'),
448:     },
449:     allowances: {
450:       Escrow: allowanceOf(post.allowances, 'Escrow'),
451:       Borrower: allowanceOf(post.allowances, 'Borrower'),
452:       Lender: allowanceOf(post.allowances, 'Lender'),
453:     },
454:     obligations: post.obligations,
455:     work: {
456:       remaining: asInt(post.work.remaining),
457:       spent: asInt(post.work.spent),
458:       closureReserve: asInt(post.work.closureReserve),
459:     },
460:     usedTransferIds: post.usedTransferIds,
461:     usedAllocationIds: post.usedAllocationIds || [],
462:     usedOriginationIds: post.usedOriginationIds || [],
463:     usedAccrualIds: post.usedAccrualIds || [],
464:     transport_remaining: asInt(terminal.transportRemaining),
465:     transport_revision: asInt(terminal.transportRevision),
466:     ordinary_locked: asInt(terminal.post.locked),
467:     ordinary_revision: asInt(terminal.post.revision),
468:     sourceHash: identity.sourceHash,
469:     instanceId: identity.instanceId,
470:     headQuantity: 1,
471:     continuingCash: 0,
472:     payoutCash: 100,
473:     constructorWorkCount: CONSTRUCTOR_WORK,
474:     collectionCapacity: COLLECTION_CAPACITY,
475:   };
476: }
477: 
478: function sameJson(left, right) {
479:   return JSON.stringify(left) === JSON.stringify(right);
480: }
481: 
```

```js
603:   reject('allowances', ['Escrow', 'Borrower', 'Lender'].every((party) => (
604:     sameQty(allowances[party]?.remaining, spec.allowances[party]?.remaining)
605:     && sameQty(allowances[party]?.spent, spec.allowances[party]?.spent)
606:   )));
607:   reject('obligations', canonicalSame(decoded.obligations || [], spec.obligations));
608:   reject('work', sameQty(decoded.work?.remaining, spec.work.remaining)
609:     && sameQty(decoded.work?.spent, spec.work.spent)
610:     && sameQty(decoded.work?.closureReserve ?? decoded.work?.closure_reserve, spec.work.closureReserve));
611:   reject('transfer', Array.isArray(decoded.usedTransferIds)
612:     && decoded.usedTransferIds.length === 1
613:     && decoded.usedTransferIds[0] === legalTransferId);
614:   reject('usedAllocationIds', canonicalSame(decoded.usedAllocationIds || [], spec.usedAllocationIds));
615:   reject('usedOriginationIds', canonicalSame(decoded.usedOriginationIds || [], spec.usedOriginationIds));
616:   reject('usedAccrualIds', canonicalSame(decoded.usedAccrualIds || [], spec.usedAccrualIds));
617:   reject('transport', sameQty(decoded.transport_revision, spec.counters.transport_revision)
618:     && sameQty(decoded.transport_remaining, spec.counters.transport_remaining));
619:   reject('ordinary', sameQty(decoded.ordinary_locked, spec.counters.ordinary_locked)
620:     && sameQty(decoded.ordinary_revision, spec.counters.ordinary_revision));
621:   reject('domain', decoded.domain_network === spec.counters.domain_network
622:     && decoded.domain_deployment === spec.counters.domain_deployment);
623:   const redeemerRecipient = redeemer.recipient || redeemer.payoutParty || null;
624:   if (redeemerRecipient) reject('recipient', redeemerRecipient === spec.recipient);
625:   const amountRead = readKernelQuantity(redeemer.amount);
626:   reject('amount', amountRead.ok === true && sameQty(amountRead.digits, spec.amount));
627:   reject('headQuantity', sameQty(decoded.headQuantity, 1));
628:   reject('continuingCash', sameQty(decoded.continuingCash, spec.balances.Escrow));
629:   reject('payoutCash', sameQty(decoded.payoutCash, spec.amount));
630:   reject('phase', decoded.phase == null && decoded.nonce == null);
631:   reject('fakeConfirmed', decoded.confirmed !== true);
632:   reject('identity', decoded.program_hash !== '00'.repeat(32)
633:     && decoded.instanceId !== '00'.repeat(32)
634:     && !packet.rejectedInstanceIds.has(decoded.instanceId));
635:   if (context.pre && typeof context.pre === 'object') {
636:     const pairs = [
637:       ['program_hash', 'program_hash'],
638:       ['bounds_hash', 'bounds_hash'],
639:       ['source_hash', 'source_hash'],
640:       ['schema_hash', 'schema_hash'],
641:       ['seed_hash', 'seed_hash'],
642:       ['instanceId', 'instance_id'],
643:     ];
644:     for (const [decodedName, preName] of pairs) {
645:       const prior = context.pre[preName] ?? context.pre[decodedName];
646:       if (prior != null && decoded[decodedName] != null) reject(decodedName, decoded[decodedName] === prior);
647:     }
648:   }
649:   if (mismatches.length) return refused(mismatches);
```

### src/custody-time.mjs — SHA256 950517d1d5472d1bbb92226abe59528b79ec446be0da32e068ed41bd08ec3f03

```js
420:   }
421:   if (!transcript.bodyPath || !existsSync(transcript.bodyPath) || fileSha(transcript.bodyPath) !== bindings.body.sha256) {
422:     return refuse('BODY_BINDING_MISMATCH');
423:   }
424:   if (!transcript.readbackPath || !existsSync(transcript.readbackPath) || fileSha(transcript.readbackPath) !== bindings.readback.sha256) {
425:     return refuse('READBACK_BINDING_MISMATCH');
426:   }
427:   if (String(transcript.cliPosix) !== evidence.cliPosix || String(transcript.lucidPosix) !== evidence.lucidPosix || String(transcript.scriptContextLower) !== evidence.scriptContextLower) {
428:     return refuse('IMPOSSIBLE_BOUNDARY');
429:   }
430:   if (bindings.artifact.sha256 === bindings.genesis.sha256 || bindings.artifact.sha256 === bindings.body.sha256) {
431:     return refuse('TAUTOLOGICAL_BINDING');
432:   }
433:   if (hex64(bindings.source?.sha256) && bindings.artifact.sha256 === bindings.source.sha256) {
434:     return refuse('TAUTOLOGICAL_BINDING');
435:   }
436:   const queriedSeed = evidence.queriedSeed || transcript.queriedSeed || null;
437:   if (queriedSeed && isFixedUtxoProbe(queriedSeed)) return refuse('FIXED_UTXO_PROBE');
438:   if (transcript.originTool !== 'executeParityCapture' || evidence.origin !== 'executeParityCapture') {
439:     return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
440:   }
441:   if (typeof transcript.commandLogSha256 !== 'string' || !hex64(transcript.commandLogSha256)) {
442:     return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
443:   }
444:   if (!transcript.commandLogPath || !existsSync(transcript.commandLogPath) || fileSha(transcript.commandLogPath) !== transcript.commandLogSha256) {
445:     return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
446:   }
447:   const commandLog = readFileSync(transcript.commandLogPath, 'utf8');
448:   if (!commandLog.includes('query tip') || !commandLog.includes('query era-history') || !commandLog.includes(String(transcript.genesisPath))) {
449:     return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
450:   }
451:   const capture = evidence.capture;
452:   if (!isAuthenticParityCapture(capture) || capture.transcriptPath !== evidence.transcriptPath
453:       || capture.cliPosix !== evidence.cliPosix || capture.lucidPosix !== evidence.lucidPosix
454:       || capture.scriptContextLower !== evidence.scriptContextLower
455:       || JSON.stringify(capture.bindings) !== JSON.stringify(bindings)) {
456:     return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
457:   }
458:   const windows = planWindows({ zeroTime: capture.zeroTime, zeroSlot: capture.zeroSlot,
459:     slotLengthMs: capture.slotLengthMs, tipSlot: capture.tipSlot });
460:   if (!windows.ok) return windows;
461:   const result = {
462:     ok: true,
463:     code: 'PARITY_ADMITTED',
464:     sourceEmitted: false,
465:     productionArtifact: false,
466:     ledgerExecution: false,
467:     siblingEmitted: false,
468:     nativeProbe: true,
469:     captureOrigin: 'executeParityCapture',
470:     measuredBounds: { releaseLower: windows.releaseStart, partition: windows.partition, refundInclusive: windows.refundArgs.upper },
471:     zeroTime: capture.zeroTime, zeroSlot: capture.zeroSlot, tipSlot: capture.tipSlot,
472:     genesisSha256: bindings.genesis.sha256,
473:     queriedSeed,
474:   };
475:   authenticAdmissions.set(result, JSON.stringify(result));
476:   return result;
477: }
478: 
479: export function productionParityGate(time) {
480:   if (!time || time.unitOnly === true) {
481:     return {
482:       ok: false,
483:       code: 'PARITY_NOT_ADMITTED',
484:       siblingEmitted: false,
485:       productionArtifact: false,
486:       sourceEmitted: false,
487:       ledgerExecution: false,
488:     };
489:   }
490:   return calibrateGuards({
491:     probeExecuted: time.probeExecuted === true,
492:     evidenceClass: time.evidenceClass,
493:     slotLengthMs: time.slotLengthMs || time.observed,
494:     lucidPosix: time.lucidPosix,
495:     cliPosix: time.cliPosix,
496:     scriptContextLower: time.scriptContextLower,
497:     unitOnly: time.unitOnly === true,
498:     bindings: time.bindings,
499:     transcriptPath: time.transcriptPath,
500:     measuredBounds: time.measuredBounds || null,
501:     queriedSeed: time.queriedSeed || null,
502:     origin: time.origin || null,
503:     capture: time.capture || null,
504:   });
505: }
```

### src/posix-parity-harness.mjs — SHA256 305e415524eb5f15940e666045874b648af480260b37deef7b4c731513424a54

```js
732: 
733: export function executeParityCapture(input = {}) {
734:   if (input == null || typeof input !== 'object') return closed('MALFORMED_DATA');
735:   if (input.armed !== true) {
736:     const prepared = prepareParityCommands({
737:       cliPath: input.cliPath || 'cardano-cli',
738:       socketPath: input.socketPath || '/tmp/m228/node.sock',
739:       magic: input.magic ?? input.networkMagic ?? 764,
740:       tipSlot: input.tipSlot,
741:       systemStart: input.systemStart,
742:     });
743:     return closed('PARITY_NOT_EXECUTED', {
744:       armed: false,
745:       evidenceClass: 'harness-refused-not-a-probe',
746:       preparedCommands: prepared.ok ? prepared.commands : null,
747:       prepareCode: prepared.ok ? prepared.code : prepared.code,
748:       reason: 'Unarmed capture does not query a node and does not read a ScriptContext.',
749:     });
750:   }
751:   if (input.forged === true || input.productionAdmitted === true) return closed('FORGED_PARITY', { armed: true });
752:   if (input.slotLengthMs != null && String(input.slotLengthMs) !== '1000') {
753:     return closed('SLOT_LENGTH_REFUSED', { armed: true, observed: String(input.slotLengthMs) });
754:   }
755:   if (input.offlineContext === true) {
756:     if (input.unitOnly === true || input.evidenceClass === 'UNIT' || input.evidenceClass === 'UNIT/SYNTHETIC') {
757:       return closed('UNIT_PARITY_REFUSED', { armed: true, probeExecuted: false });
758:     }
759:     return executeOfflineProbe(input);
760:   }
761:   const unit = input.unitOnly === true || input.evidenceClass === 'UNIT' || input.evidenceClass === 'UNIT/SYNTHETIC';
762:   if (!unit) {
763:     if (!input.reviewA || !input.reviewB || !existsSync(input.reviewA) || !existsSync(input.reviewB)) {
764:       return closed('REVIEW_ABSENT', { armed: true });
765:     }
766:   }
767:   if (!input.cliPath || !existsSync(input.cliPath) || !input.socketPath || !existsSync(input.socketPath)) {
768:     return closed('TIME_QUERY_ABSENT', { armed: true });
769:   }
770:   if (unit) return runUnitSequence(input);
771:   if (typeof input.compile === 'function') {
772:     return closed('COMPILE_CALLBACK_NOT_PRODUCTION', { armed: true, reason: 'real capture requires the pinned written-artifact compiler' });
773:   }
774:   if (process.env.MORIARTY_ALLOW_REAL_NODE !== '1') return closed('REAL_QUERY_NOT_ARMED', { armed: true });
775:   if (!input.auditA || !input.auditB || !existsSync(input.auditA) || !existsSync(input.auditB)) {
776:     return closed('REAL_QUERY_NOT_ARMED', { armed: true, reason: 'two FULL audit files are not attached' });
777:   }
778:   const injected = typeof input.spawn === 'function';
779:   const spawnImpl = injected ? input.spawn : nodeSpawn;
780:   const result = runNodeObservation(input, spawnImpl, injected);
781:   if (!injected && spawnImpl === nodeSpawn && realCli(input.cliPath)
782:       && result.productionAdmitted === true && result.probeExecuted === true) {
783:     authenticCaptures.set(result, JSON.stringify(result));
784:   }
785:   return result;
786: }
787: 
788: function runCli(argv) {
789:   if (!argv.includes('--armed')) {
790:     const prepared = prepareParityCommands({
791:       cliPath: 'cardano-cli',
792:       socketPath: '/tmp/m228/node.sock',
793:       magic: 764,
794:     });
795:     return {
```

### src/apply-head.mjs — SHA256 663bfd1e887c48e6363f8080727d277b7d8db4b7d9caaa77a516fc52e44f3202

```js
256: export function buildApprovedCashPolicy({ sourcePath, seedTxId, seedIndex, workDir, aiken = AIKEN } = {}) {
257:   if (!sourcePath || !existsSync(sourcePath) || !workDir) return refuse('CASH_SOURCE_ABSENT', 'cash fixture path');
258:   const sourceBytes = readFileSync(sourcePath);
259:   const sourceHash = createHash('sha256').update(sourceBytes).digest('hex');
260:   if (sourceHash !== CASH_SOURCE_SHA256) return refuse('CASH_DESIGN_SOURCE_MISMATCH', sourceHash);
261:   if (!/^[0-9a-f]{64}$/.test(String(seedTxId || '')) || !/^\d+$/.test(String(seedIndex))) {
262:     return refuse('CASH_SEED_ABSENT', 'seed outref');
263:   }
264:   const pin = stdlibTreeSha256(STDLIB_TREE);
265:   if (!pin.ok || pin.sha256 !== STDLIB_TREE_SHA256 || pin.files !== 65) {
266:     return refuse('STDLIB_PIN_MISMATCH', JSON.stringify(pin));
267:   }
268:   const identity = run(aiken, ['--version']);
269:   let realAiken = aiken;
270:   try { realAiken = realpathSync(aiken); } catch { return refuse('AIKEN_IDENTITY', 'realpath'); }
271:   if (identity.status !== 0 || !String(identity.stdout || '').includes('aiken v1.1.21')) {
272:     return refuse('AIKEN_IDENTITY', identity.stderr || identity.stdout);
273:   }
274:   mkdirSync(workDir, { recursive: true });
275:   const cachePath = join(workDir, 'cash-policy.json');
276:   if (existsSync(cachePath)) {
277:     const cached = JSON.parse(readFileSync(cachePath, 'utf8'));
278:     if (cached.sourceHash === sourceHash && cached.seedTxId === seedTxId && String(cached.seedIndex) === String(seedIndex)
279:       && cached.aikenRealpath === realAiken && cached.stdlibSha256 === STDLIB_TREE_SHA256
280:       && cached.policyId && existsSync(cached.scriptPath)) {
281:       return { ...cached, ok: true, cacheHit: true, compilerInvoked: false, ledgerExecution: false, nativePins: false, cashCanMint: false };
282:     }
283:   }
284:   const project = join(workDir, 'project');
285:   rmSync(project, { recursive: true, force: true });
```

```js
297:   const blueprint = JSON.parse(readFileSync(blueprintPath, 'utf8'));
298:   const cash = findValidator(blueprint, 'mint', 'cash_fixture') || findValidator(blueprint, 'mint', 'cash');
299:   if (!cash?.compiledCode || !cash.hash) return refuse('CASH_COMPILE_FAILED', 'cash_fixture mint missing');
300:   const unapplied = agree(cash.compiledCode, cash.hash, 'cash-fixture-unapplied');
301:   if (!unapplied.ok) return refuse('SCRIPT_HASH_DIVERGENCE', 'unapplied cash fixture');
302:   const seedLocal = definiteDataHex('output-reference', { transactionId: seedTxId, index: Number(seedIndex) });
303:   if (!seedLocal.ok) return seedLocal;
304:   const appliedPath = join(workDir, 'cash.applied.json');
305:   const appliedRun = spawnSync(aiken, ['blueprint', 'apply', '-i', blueprintPath, '-o', appliedPath, '-m', 'cash_fixture', '-v', 'cash_fixture', seedLocal.hex], {
306:     cwd: project, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024,
307:   });
308:   if (appliedRun.status !== 0 || !existsSync(appliedPath)) return refuse('APPLY_FAILED', appliedRun.stderr || appliedRun.stdout);
309:   const appliedBp = JSON.parse(readFileSync(appliedPath, 'utf8'));
310:   const applied = findValidator(appliedBp, 'mint', 'cash_fixture') || findValidator(appliedBp, 'mint', 'cash');
311:   if (!applied?.compiledCode) return refuse('APPLY_FAILED', 'applied cash mint missing');
312:   const agreement = agree(applied.compiledCode, applied.hash, 'cash-fixture-applied');
313:   if (!agreement.ok) return refuse('SCRIPT_HASH_DIVERGENCE', 'applied cash fixture');
314:   if (applied.hash === FAIL_CLOSED_CASH || applied.hash === 'aa'.repeat(28)) {
315:     return refuse('SYNTHETIC_POLICY_REJECTED', applied.hash);
316:   }
317:   const scriptPath = join(workDir, 'cash.applied.script.json');
318:   writeFileSync(scriptPath, JSON.stringify({ type: 'PlutusScriptV3', description: 'approved cash fixture applied to the cash seed', cborHex: applied.compiledCode }));
319:   const record = {
320:     ok: true,
321:     code: 'CASH_POLICY_COMPILED',
322:     cacheHit: false,
323:     compilerInvoked: true,
324:     sourceHash,
325:     seedTxId,
326:     seedIndex: Number(seedIndex),
327:     aikenRealpath: realAiken,
328:     aikenVersion: String(identity.stdout || '').trim(),
329:     stdlibSha256: STDLIB_TREE_SHA256,
330:     stdlibFiles: 65,
331:     checkStatus: check.status,
332:     buildStatus: build.status,
333:     unappliedHash: cash.hash,
334:     policyId: applied.hash,
335:     scriptPath,
336:     blueprintSha256: createHash('sha256').update(readFileSync(blueprintPath)).digest('hex'),
337:     ledgerExecution: false,
338:     nativePins: false,
339:     cashCanMint: false,
340:     ledgerFunding: false,
341:   };
342:   writeFileSync(cachePath, `${JSON.stringify(record, null, 2)}\n`);
343:   return record;
344: }
345: 
```

### src/native-bootstrap.mjs — SHA256 d0901f1549660d3108c84906379d4871de58ab599eb9008604264f839b98f254

```js
1316: export function planApprovedBootstrap(input = {}) {
1317:   const flags = {
1318:     spawned: false,
1319:     executed: false,
1320:     executeNow: false,
1321:     nodeStarted: false,
1322:     byronReplaced: false,
1323:     hashesBound: false,
1324:     ledgerExecution: false,
1325:     nativeBudgetMeasured: false,
1326:     confirmed: false,
1327:   };
1328:   if (input.signingKey || input.cborHex || input.budget || input.oldGenesis || input.bundledKeys) {
1329:     return closed('BUNDLED_INPUT_REFUSED', flags);
1330:   }
1331:   if (!Number.isInteger(input.magic) || input.magic <= 0 || input.magic === 42) {
1332:     return closed('MAGIC_SOURCE_REFUSED', flags);
1333:   }
1334:   const start = approvedStart(input.nowMs, input.leadSeconds);
1335:   if (!start) return closed('START_UNBOUND', flags);
1336:   if (typeof input.runDir !== 'string' || !isAbsolute(input.runDir)) return closed('RUN_DIR_ABSENT', flags);
1337:   const alonzo = pinFile('alonzo');
1338:   if (!alonzo.ok) return { ...alonzo, ...flags };
1339:   const conway = pinFile('conway');
1340:   if (!conway.ok) return { ...conway, ...flags };
1341:   const shelley = shelleySpec(start);
1342:   if (!shelley) return closed('LOSSY_RECIPE_INTEGER', flags);
1343:   const run = input.runDir;
1344:   const magic = String(input.magic);
1345:   const cli = 'cardano-cli';
1346:   const node = 'cardano-node';
1347:   const steps = [
1348:     {
1349:       id: 'S0-spec-files',
1350:       executed: false,
1351:       writesGenesis: false,
1352:       outputs: ['spec/shelley.spec.json', 'spec/alonzo.spec.json', 'spec/conway.spec.json', 'spec/byron-protocol-params.json'],
1353:     },
1354:     {
1355:       id: 'S1-create-testnet-data',
1356:       executed: false,
1357:       argv: [
1358:         cli, 'conway', 'genesis', 'create-testnet-data',
1359:         '--spec-shelley', join(run, 'spec/shelley.spec.json'),
1360:         '--spec-alonzo', join(run, 'spec/alonzo.spec.json'),
1361:         '--spec-conway', join(run, 'spec/conway.spec.json'),
1362:         '--testnet-magic', magic,
1363:         '--start-time', start,
1364:         '--genesis-keys', '1',
1365:         '--pools', '1',
1366:         '--stake-delegators', '1',
1367:         '--utxo-keys', '1',
1368:         '--total-supply', '45000000000000000',
1369:         '--delegated-supply', '30000000000000000',
1370:         '--out-dir', join(run, 'genesis'),
1371:       ],
1372:     },
1373:     {
1374:       id: 'S2-create-cardano-byron-zero',
1375:       executed: false,
1376:       supplyOmitted: true,
1377:       argv: [
1378:         cli, 'conway', 'genesis', 'create-cardano',
1379:         '--genesis-dir', join(run, 'byron-only'),
1380:         '--gen-genesis-keys', '1',
1381:         '--gen-utxo-keys', '0',
1382:         '--start-time', start,
1383:         '--security-param', '18',
1384:         '--slot-length', '1000',
1385:         '--slot-coefficient', '0.05',
1386:         '--testnet-magic', magic,
1387:         '--byron-template', join(run, 'spec/byron-protocol-params.json'),
1388:         '--shelley-template', join(run, 'spec/shelley.spec.json'),
1389:         '--alonzo-template', join(run, 'genesis/alonzo-genesis.json'),
1390:         '--conway-template', join(run, 'genesis/conway-genesis.json'),
```

### src/output-fee.mjs — SHA256 c204c1cb544a575a0f6ab6d5d6033f8e20063d64fedb90f4ed9362637e6f14b8

```js
502: export async function calculateOutputMinimum({
503:   cliPath,
504:   protocolPath,
505:   output,
506:   workDir,
507:   protocolProvenance,
508:   cliIdentity = null,
509: }) {
510:   if (!output || typeof output !== "object" || Array.isArray(output)) {
511:     throw new OutputFeeError("UNSUPPORTED_OUTPUT", "output must be an object", { spawned: false });
512:   }
513:   assertPlainPath(workDir, "workDir");
514:   rejectWitnessScriptKeys(output);
515:   const lovelace = canonicalInteger(output.lovelace, {
516:     name: "lovelace",
517:     min: 0n,
518:     max: MAX_LOVELACE,
519:     rangeCode: "UNSUPPORTED_OUTPUT",
520:   });
521:   const assets = normalizeAssets(output.assets);
522:   let datumPath = null;
523:   let datumSha = null;
524:   let fieldCount = null;
525:   if (output.datumData != null) {
526:     if (typeof output.datumData === "number") {
527:       throw new OutputFeeError("MALFORMED_DATUM", "pass the full detailed schema object, not a field count", {
528:         spawned: false,
529:       });
530:     }
531:     validateDetailed(output.datumData, 0);
532:     fieldCount = Array.isArray(output.datumData.fields) ? output.datumData.fields.length : null;
533:     await mkdir(workDir, { recursive: true });
534:     datumPath = path.join(workDir, `inline-datum-${randomBytes(8).toString("hex")}.json`);
535:     const text = `${JSON.stringify(output.datumData)}\n`;
536:     await writeFile(datumPath, text, { mode: 0o644 });
537:     datumSha = sha256Text(text);
538:   }
539:   let referenceScriptSha = null;
540:   const referenceScriptPath = output.referenceScriptPath;
```

```js
711: export async function calculateBodyMinimumFee({
712:   cliPath,
713:   protocolPath,
714:   bodyPath,
715:   witnessCount,
716:   witnessCountProvenance,
717:   workDir,
718:   protocolProvenance,
719:   referenceScriptSize = null,
720:   requiresReferenceScriptFee = false,
721:   executionUnitsProvenance = null,
722:   signingKeyPaths = null,
723:   cliIdentity = null,
724: }) {
725:   assertPlainPath(workDir, "workDir");
726:   assertPlainPath(bodyPath, "bodyPath");
727:   const witnesses = canonicalInteger(witnessCount, {
728:     name: "witnessCount",
729:     min: 0n,
730:     max: MAX_WITNESS_COUNT,
731:     rangeCode: "UNSUPPORTED_WITNESS_COUNT",
732:   });
733:   const provenance = classifyWitnessCountProvenance(witnessCountProvenance);
734:   if (Array.isArray(signingKeyPaths) && BigInt(signingKeyPaths.length) !== witnesses) {
735:     throw new OutputFeeError("WITNESS_COUNT_KEY_MISMATCH", "witness count is not the signing-key count", {
736:       spawned: false,
737:       signingKeyCount: signingKeyPaths.length,
738:     });
739:   }
740:   if (requiresReferenceScriptFee === true && referenceScriptSize == null) {
741:     throw new OutputFeeError("REFERENCE_SCRIPT_SIZE_REQUIRED", "a reference-script fee needs the script size in bytes", {
742:       spawned: false,
743:     });
744:   }
745:   let referenceBytes = null;
746:   if (referenceScriptSize != null) {
747:     referenceBytes = canonicalInteger(referenceScriptSize, {
748:       name: "referenceScriptSize",
749:       min: 0n,
750:       max: 1000000000n,
```

---

## Actual retained consumer failures and final scoped results

### t1-gpt61-implementation-417/attempt003/stdout

```text
✖ constructed fully pinned transcript and copied origin names cannot admit production (2.888575ms)
ℹ tests 1
ℹ suites 0
ℹ pass 0
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 570.850756

✖ failing tests:

test at experiments/moriarty-eutxo/tests/production-origin-417.test.mjs:9:1
✖ constructed fully pinned transcript and copied origin names cannot admit production (2.888575ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
  
  true !== false
  
      at TestContext.<anonymous> (file:///home/charl/Moriarty/.worktrees/eutxo-candidate-219/experiments/moriarty-eutxo/tests/production-origin-417.test.mjs:35:10)
      at Test.runInAsyncScope (node:async_hooks:227:14)
      at Test.run (node:internal/test_runner/test:1402:25)
      at Test.start (node:internal/test_runner/test:1262:17)
      at startSubtestAfterBootstrap (node:internal/test_runner/harness:387:17) {
    generatedMessage: true,
    code: 'ERR_ASSERTION',
    actual: true,
    expected: false,
    operator: 'strictEqual',
    diff: 'simple'
  }

```

### t1-gpt61-implementation-417/attempt005/stdout

```text
✔ constructed fully pinned transcript and copied origin names cannot admit production (15.697221ms)
✖ source terminals evaluate substituted time guards and project the fixed Core amount (315.203103ms)
ℹ tests 2
ℹ suites 0
ℹ pass 1
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 713.44565

✖ failing tests:

test at experiments/moriarty-eutxo/tests/production-origin-417.test.mjs:39:1
✖ source terminals evaluate substituted time guards and project the fixed Core amount (315.203103ms)
  AssertionError [ERR_ASSERTION]: GUARD_FAILED
  
  false !== true
  
      at TestContext.<anonymous> (file:///home/charl/Moriarty/.worktrees/eutxo-candidate-219/experiments/moriarty-eutxo/tests/production-origin-417.test.mjs:47:10)
      at async Test.run (node:internal/test_runner/test:1409:7)
      at async Test.processPendingSubtests (node:internal/test_runner/test:974:7) {
    generatedMessage: false,
    code: 'ERR_ASSERTION',
    actual: false,
    expected: true,
    operator: 'strictEqual',
    diff: 'simple'
  }

```

### t1-gpt61-implementation-417/attempt012/stdout

```text
✖ production capture refuses a caller-supplied compiler before any real command (3.231824ms)
ℹ tests 1
ℹ suites 0
ℹ pass 0
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 620.703502

✖ failing tests:

test at experiments/moriarty-eutxo/tests/production-origin-417.test.mjs:130:1
✖ production capture refuses a caller-supplied compiler before any real command (3.231824ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
  + actual - expected
  
  + 'REAL_QUERY_NOT_ARMED'
  - 'COMPILE_CALLBACK_NOT_PRODUCTION'
  
      at TestContext.<anonymous> (file:///home/charl/Moriarty/.worktrees/eutxo-candidate-219/experiments/moriarty-eutxo/tests/production-origin-417.test.mjs:138:10)
      at async Test.run (node:internal/test_runner/test:1409:7)
      at async startSubtestAfterBootstrap (node:internal/test_runner/harness:387:3) {
    generatedMessage: true,
    code: 'ERR_ASSERTION',
    actual: 'REAL_QUERY_NOT_ARMED',
    expected: 'COMPILE_CALLBACK_NOT_PRODUCTION',
    operator: 'strictEqual',
    diff: 'simple'
  }

```

### t1-gpt61-implementation-417/attempt013/stdout

```text
✔ F6 digit supply is preserved and an unsafe number is refused (55.271519ms)
✔ B4 unit race does not treat query tip or submission order as the winner (157.07717ms)
✔ B4 the queried output names the winner and the other body is the loser (224.141327ms)
✔ F1 an incomplete queried utxo is refused without the offline fixture ids (10.59555ms)
✔ F1 a unit command sequence parses the hashed artifact and does not admit production (10.906489ms)
✔ constructed fully pinned transcript and copied origin names cannot admit production (13.636669ms)
✔ source terminals evaluate substituted time guards and project the fixed Core amount (747.204288ms)
✔ economic consumer refuses absent or invalid amounts and obligation encoder refuses incomplete quantities (4457.893424ms)
✔ original pair race refuses missing loser receipt, ambiguous winner and wrong complete post (8388.290813ms)
✔ production capture refuses a caller-supplied compiler before any real command (1.462356ms)
✔ design window keeps POSIX redeemer bounds and half-open slot bounds apart (15.066618ms)
✔ a 1000 ms slot converts and 999 or 1001 do not (0.6774ms)
✔ overflow, negative, rounding, empty, and refund upper-exclusive intervals fail (0.405643ms)
✔ measured fixture windows stay unrepresentable on the approved clock (0.50955ms)
✔ redeemer fields stay POSIX while a supplied clock reports slot bounds (0.87199ms)
✔ guard substitution touches only the time predicates (21.53487ms)
✔ synthetic sibling changes source and program hashes and keeps bounds (365.097241ms)
✔ missing, mismatched, forged, and unit-only parity refuse production (0.442645ms)
✔ unarmed parity harness does not invent a V3 lower bound (944.818002ms)
✔ capacity oracle derives the release window from the selected action (621.523399ms)
✔ parity capture stays fail-closed and a unit stub cannot admit production (7.902054ms)
✔ synthetic oracle cases derive the action window and keep economic failures (15.996666ms)
✔ real cli financial body refuses before encode without admitted parity (26.131346ms)
✔ armed observation binds queried tip and readback files and does not admit production (8.295238ms)
ℹ tests 24
ℹ suites 0
ℹ pass 24
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 14288.845782

```

### t1-gpt61-implementation-417/RESULT.json

```text
{
  "schema": "moriarty-gpt61-implementation-417/result/1",
  "status": "WORK_IN_PROGRESS",
  "sourceStable": true,
  "sourceRows": 383,
  "dependenciesRehashed": 9606,
  "dependenciesChanged": 0,
  "sourceApproval": false,
  "nativeApproval": false,
  "ledgerExecution": false,
  "goalComplete": false,
  "productionGap": "current-policy-script-budget-minAda-binding-required",
  "attempt008": {
    "exit": 0,
    "tests": 36,
    "pass": 36,
    "fail": 0,
    "runtimeMs": 686698.656673
  },
  "quantityChecks": 68,
  "quantityFailures": 0,
  "latestConsumerTests": 16,
  "latestConsumerFailures": 0,
  "realAikenChecks": 94,
  "realAikenFailures": 0,
  "realUplcBuilt": true,
  "realParametersApplied": true,
  "scope": "UNIT custody export, not production/native",
  "latestHookTimeRaceTests": 24,
  "latestHookTimeRaceFailures": 0,
  "productionSourceComplete": false
}

```

## Prior FULL audit rejection summaries (historical, not current approvals)

audit386a-verified-final-packet-freeze391
```json
{
  "decision": "CHANGES_REQUIRED",
  "sourceApproval": false,
  "nativeApproval": false,
  "repairAcceptance": [
    "Pass measured releaseLower, partition, and refundInclusive from an admitted parity transcript into substituteTimeGuards and compileCustody. Do not mark the no-syntheticTime compile as a calibrated sibling.",
    "Change the five stale spend expectations to FIXTURE_WINDOW_UNREPRESENTABLE with encodeSpendBodyCalled false. Do not encode 1000/1099, 1100/1100, or 1050/1051 as CLI slots.",
    "Observe BadInputsUTxO on the original losing submit. Bind the winner to the outref and the full Head 13 post. Do not resubmit the loser to produce the error."
  ]
}
```

audit386b-verified-final-packet-freeze392
```json
{
  "decision": "CHANGES_REQUIRED",
  "sourceApproval": false,
  "nativeApproval": false,
  "repairAcceptance": [
    "measured-clock sibling through compileCustody into body budget and min-ADA",
    "remove post-poll loser resubmit",
    "refuse null redeemer amount",
    "replace five SPEND_BODIES_ENCODED Layer S expectations"
  ]
}
```

## Review response contract

Report verdict; substantive reasoning; requirement IDs and exact expected outcome corrections; sufficient positive/negative tests; local/Preview exit gaps; any unclear authority/resource dependency. Do not approve missing behavior from test counts, inventories or historical votes. Preserve dissent and unexecuted classifications. Do not assume a constructed genuine receipt is a valid offline positive.
