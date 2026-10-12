# Native execution design

## Status and evidence

This package is a DRAFT. Every implementation task remains unchecked.
TASK means `/home/charl/moriarty-eutxo-validation-2026-10-12`.
WT means `/home/charl/Moriarty-wt-eutxo-validation-20261012`.
R means `TASK/native-validation-runs/<runId>`.
RunId is a fresh UTC timestamp plus 128 random bits, encoded as lowercase hexadecimal.

Stage00 proves `public-single-head-wallet-transfer/1` on an isolated Haskell node.
It includes two counter steps, a correctly signed phase-two rejection, and a sequential conflicting spend.
Its T3 stale signature proves only `InvalidWitnessesUTXOW`.
Its T2 Ogmios result proves evaluation rejection only.
Its isValid-true T3b rejection left head and collateral unchanged.
The former node is offline. Preserve its files without restart or CBOR replay.

Revision03 design candidate is `dcc62e21e9e351843bd04a3be537ebe260f8c1d0ec606cfcc5e15fd83ca7aff0`.
Both full design reviews approve scoped T1 design only.
Implementation18 candidate is `aed4090af8f1e94e3904140ac2b23be9aa79e85141806841f1066115e5386680`.
Its Aiken checks use synthetic transactions. `nativePins` and calibrated source are absent.
Required current implementation reviews remain separate from design votes.
Oracle19 packet is `9d71e736e3d3c159a83ff3bb21dbb3f5b2a9661dd90ed1e70414c2690b6552d6`.
Its source envelopes are independent expected outcomes, not native evidence.

Receipt helper SHA is `f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18`.
Its candidate SHA is `617e2e18ff03459a09419549d47bd4c43f65a310366c5109bd9bcad725fdeea6`.
Fresh Grok4.7 approved offline transport binding only.
Version1 remains rejected. Historical failures and dissent remain immutable.
The existing helper accepts roots only below `TASK/tooling/transaction-receipts` or `TASK/transaction-receipts-10`.
Use `TASK/tooling/transaction-receipts/native-validation/<runId>` for future receipts.
Do not broaden the approved helper namespace to match R.

## Proposed typed interfaces

All interfaces below are prospective. No claim states that these APIs exist.
Decimal integer strings use canonical nonnegative syntax before BigInt conversion.
Digests carry their algorithm and domain. Public evidence never contains private key bytes.

```typescript
type SHA256 = string // exactly 64 lowercase hex, never all zero
type ScriptHash = string // exactly 56 lowercase hex, never all zero
type UInt = string // /^(0|[1-9][0-9]*)$/ with declared bit bound
type OutRef = { txId: SHA256, index: UInt }
type Artifact = { path: string, sha256: SHA256, bytes: UInt }
type RuntimePins1 = {
  version: 'RuntimePins/1', runId: string,
  domainNetwork: string, domainDeployment: string,
  genesis: Artifact, protocolParameters: Artifact,
  cashPolicy: Artifact, threadPolicy: Artifact, spendingBlueprint: Artifact,
  cashPolicyId: ScriptHash, threadPolicyId: ScriptHash, spendingScriptHash: ScriptHash,
  cashAssetNameHex: '43617368', threadAssetNameHex: '48656164',
  lenderVkey: Artifact, borrowerVkey: Artifact,
  lenderKeyHash: ScriptHash, borrowerKeyHash: ScriptHash,
  mintSeed: OutRef, genesisHead: OutRef, instanceId: SHA256,
  pinDependencyOrder: string[], policyHashWitnessReview: Artifact | null,
  authenticity: { compiledPolicy: true, verificationKey: true, nativeGenesisObserved: true }
}
type NativeTimeWitness1 = {
  version: 'NativeTimeWitness/1', genesis: Artifact, tip: Artifact,
  sdkVersion: string, cliVersion: string, probeBlueprint: Artifact,
  probeBody: Artifact, probeContext: Artifact,
  zeroTimeMs: UInt, zeroSlot: UInt, tipSlot: UInt, slotLengthMs: '1000',
  cliPosix: UInt, sdkPosix: UInt, scriptContextLower: UInt,
  probeObservation: 'native-ledger-context' | 'native-node-evaluation-context',
  releaseStart: UInt, partition: UInt, refundExclusiveEnd: UInt,
  originalSource: Artifact, calibratedSource: Artifact, substitutionMap: Artifact,
  originalProgramHash: SHA256, calibratedProgramHash: SHA256, boundsHash: SHA256
}
type NativeCaseResult1 = {
  version: 'NativeCaseResult/1', familyId: string, caseId: string,
  originalSource: Artifact, executedSource: Artifact,
  expectedEnvelope: Artifact, loweringManifest: Artifact, runtimePins: Artifact,
  signedCbor: Artifact | null, cliTxId: SHA256 | null,
  body: Artifact | null, context: Artifact | null,
  transportStatus: string, ledgerStatus: 'unsubmitted' | 'unknown-finality' |
    'phase-one-rejected' | 'phase-two-rejected' | 'included-valid' | 'included-invalid',
  rawObservation: Artifact | null, block: Artifact | null,
  readbackBefore: Artifact, readbackAfter: Artifact,
  comparison: Artifact, protocolMeasurement: Artifact | null,
  nativeAcceptance: boolean, completeness: 'complete' | 'partial' | 'refused' | 'unverified'
}
type NativeRunManifest1 = {
  version: 'NativeRunManifest/1', runId: string, candidate: Artifact,
  inputArtifacts: Artifact[], toolVersions: Artifact, toolBinaries: Artifact[],
  designReviews: Artifact[], implementationReviews: Artifact[], helperReview: Artifact,
  resourcePlan: Artifact, compilerInterface: Artifact, ledgerInterface: Artifact,
  financeInterface: Artifact, conformanceInterface: Artifact, oraclePacket: Artifact,
  caseManifest: Artifact, runtimePins: Artifact | null, timeWitness: Artifact | null
}
```

A precompile pin request has no `genesisHead`. It is not RuntimePins/1.
Resolve policies and actor keys first. Compile the spending validator next.
Read genesisHead after mint and lock. Finalize RuntimePins/1 after authenticated ledger readback.
Compiler manifests bind source/Core/kernel/Aiken maps and exact evaluator domains.
Ledger interfaces bind authenticated credentials, full context, and injective output allocations.
Finance envelopes bind full ordinary/financial posts, ordered effects, authority, and fee policy.
`eutxo-conformance-and-private-feasibility` supplies family predicates and private refusal conditions.
Runtime pins cannot manufacture an unsupported compiler capability.

## Policy dependency amendment

Revision03 requires real policies before spending compilation.
A compile-policy cycle must produce `PIN_DEPENDENCY_CYCLE`.
The peers propose `PolicyHashWitness/1` and an own-ScriptContext credential to avoid this cycle.
Those amendments are unreviewed. They are not adopted by this draft.
Measure the exact dependency graph before any native compilation.
Bind a fresh reviewed amendment if the graph cannot be resolved within approved revision03.
Retain the approved 13-field ABI unless that review explicitly changes it.
Never compile label hashes, unit fixture `e5` bytes, or all-zero policy ids.

## Runtime pin generation

1. Reserve fresh task paths and ports with an exclusive file lock.
2. Verify actual node, CLI, Yaci, Aiken, Node, SDK, and stdlib versions.
3. Hash binaries, genesis files, lockfiles, and protocol parameters.
4. Generate task payment keys inside R/private with umask 077.
5. Derive payment key hashes through CLI and SDK independently.
6. Compile Cash and consumed-seed thread policies under the reviewed dependency plan.
7. Derive policy ids from compiled scripts through CLI and SDK independently.
8. Validate lowercase lengths, distinct policies, names, and nonzero identities.
9. Derive the instance preimage through the actual canonical codec.
10. Use `hashDomain("MORIARTY-INSTANCE-financial-agreement/5", canonical instanceRecord)`.
11. Compile the spending script with measured pins and actual source bindings.
12. Compare the blueprint credential with CLI and SDK script hashes.
13. Mint local assets from task free ADA only.
14. Lock source-derived heads with authenticated datum and future max-datum min-ADA.
15. Record actual mint seed, lock transaction, output index, value, address, and decoded datum.
16. Finalize the public RuntimePins/1 manifest after ledger observation.

InstanceRecord contains domainNetwork, domainDeployment, threadPolicyId, threadAssetNameHex, cashPolicyId, and cashAssetNameHex.
Escrow is script-owned. Its credential is not a verification-key hash.
T1 Cash and Head names are case-sensitive. ADA uses empty policy and asset name.
A consumed-seed policy alone does not prove authenticated general genesis.
S01 requires the ledger package’s program/address/datum/genesis constraints.

## Task-owned resource plan

The future run owner is the assigned native implementer, identified in R/resource-plan.json.
Exclusive root is R. Secret root is R/private, with mode 0700.
Network home is R/devnet. Cluster name is `native-<runId>`.
Candidate ports are admin 31010, node 33001, submit 38090, store 38080, Ogmios 31337.
Check every port before reservation. Occupied ports stop the run without killing their owner.
Persist PID, process-start timestamp, executable digest, socket path, and owner runId.
Stop only matching task-owned process groups after resource cleanup checks.
Do not use historical `npm start.mjs`, global PID files, resets, or old devnet homes.
Use loopback endpoints only. Reject public, testnet, and user-wallet configurations.

Proposed bounded resource envelope: one local cluster, four task roles, twenty families, and at most 160 submissions.
Free ADA ceiling is 5000 local ADA. Host ceiling is 60 minutes and 8 GiB peak memory.
Per-receipt confirmation budget is 90 seconds. No ceiling extends itself after failure.
These are proposed execution limits, not measured consumption or new execution authority.
The native implementation review binds concrete remaining authority before execution.
Unknown cost and provider usage remain unknown.

Key generation, bootstrap, and raw Yaci logs stay outside public evidence exports.
The log filter suppresses seed phrases, signing keys, private paths, and wallet-export arguments.
Public manifests contain verification keys, key hashes, script hashes, and transaction identities only.
Use separate fee and collateral inputs for A, B, C, and both R transactions.
Concurrent transactions share only head R. Their fee and collateral pools are pairwise disjoint.

## Time execution and logical equivalence

Read exact genesis systemStart, genesisZeroSlot, slotLength, and an advancing native tip.
Require slotLengthMs 1000. Use exact integer milliseconds before SDK safe-number conversion.
Reject negative POSIX, UInt128 overflow, and SDK-unrepresentable integers.
Probe V3 context for validity slots [s,s+1). Persist the actual node context and body.
Compute CLI POSIX as systemStartMs + (s-genesisZeroSlot)*1000.
Configure SDK slotToBeginUnixTime from those genesis values.
Require CLI POSIX = SDK POSIX = context lower POSIX.
A node evaluation context may establish parity but does not establish transaction inclusion.
Tag that distinction in NativeTimeWitness/1.
Require native-ledger-context evidence before reporting a ledger-executed probe.

Let t be the fresh tip slot. Let B(s) be the measured slot-begin function.
L=B(t+30), P=L+60000, E=P+60000.
Release/cancel body validity is [t+30,t+90). Args are lower L and upper P-1.
Refund body validity is [t+90,t+150). Args are lower P and upper E-1.
Both widths equal 60 slots. Lookahead is 150 seconds, below the 1800-second ceiling.
The source refund guard literal 1200 maps to E-1, preserving its inclusive comparison.
Original release/cancel guards 1000 and 1100 map to L and P.
Recompute tip and reject stale windows before signing. Do not sleep for the forecast ceiling.

Original E13 Args 1000/1099 cover 100 inclusive milliseconds.
Original E11 Args 1100/1100 cover one inclusive millisecond.
They remain source/unit controls, never live positives on 1000-millisecond slots.
Native sibling uses a monotone interval partition mapping of admissible original windows.
Compare all non-time envelope fields across that mapping.
Preserve before-boundary, exact-boundary, straddling, reversed, and last-millisecond failure classes.
Do not claim equality of wall-clock durations or synthetic clocks.
Write the substitution AST map and original/calibrated domain hashes.
Bounds hash stays unchanged only when its canonical payload bytes stay unchanged.
Reject runtime-guard substitution if source shape changes or current compiler refuses calibration.
Implementation18 currently refuses calibrated source and authentic native pins.
That refusal is a missing implementation task, never an instruction to bypass admission.

## Execution DAG

```text
exact-current-candidate + reviews + dependency-interface-check
  -> offline-regressions
  -> resource-reservation -> fresh-local-devnet -> tool/genesis/parameter measurements
  -> task-key/policy generation -> authentic precompile pins
  -> V3 parity -> guard-only sibling -> full evaluator correspondence
  -> actual compiler lowering -> artifact credential checks -> mint/lock/readback
  -> sequential positives + signed negatives + references + concurrent pair
  -> CLI-bound receipts -> body/context/readback comparison -> measurements
  -> all20 matrix + immutable reproduction bundle + task-owned cleanup
```

A refused stage leaves its dependent cases unsubmitted.
Independent source controls remain available without claiming native progress.
Do not replace a native result with an offline decision table.

## Native case manifest

Each case specifies originalSource, executedSource, profile, action, input state, Args, observations, expected envelope, and admission predicate.
Each case also specifies head, actor keys, fee input, collateral input, validity slots, mutation, and observation class.
Generate exact cases from designer source verifiers and expected files.
Compare independent packet19 envelopes before construction. Do not copy expected files into observations.
Include lender initial balance 7, borrower initial balance 3, Witness balance 5, prior used id, and alternate transfer id.
Include allowance 11/2, ordinary revision 4, additive work, exact work exhaustion, and closureReserve 1.
These distinguish real lowering from hardcoded E13/E11 outputs.

For T1, use fresh heads A=release, B=refund, C=cancel, and R=release-versus-cancel.
Empty-seed positive post uses work 975/49/16, ordinary locked 0/revision 1, and transport 1/0.
Require full allowance consumption 100/0 to 0/100 and correct usedTransferId.
E12 retains residual duty outstanding 40 at source, but T1 rejects `NONEMPTY_OBLIGATIONS` before spending.
Profile /6 interval-boundary remains `PROFILE_MISMATCH` until its proper compiler exists.
Source code `GUARD_FAILED` is distinct from node `PlutusFailure`.
Source `WORK_EXHAUSTED` is distinct from kernel `INSUFFICIENT_WORK`.
Record actual codes by layer. Do not relabel these codes to match fixture names.

The full family expansion is in CASE-MATRIX.json.
Each future supported row requires its own admitted source and native positive plus signed negative.
Unsupported rows require the exact refusal and absence of submission.
S03/S04 may need source-language capabilities before native construction.
Authenticated observation controls require actual feed/publisher credentials and full-window freshness.
A ledger package that currently lacks observation authentication must refuse those rows.

## Receipts and observation classes

Import only `TASK/tooling/transaction-receipts/revision02/submit-with-receipt.mjs` after its exact digest check.
Use its existing submitWithReceipt API through a native driver adapter.
Persist CLI text-envelope decode, TxId, signed-CBOR SHA256, and body SHA256 before submit.
TxId hashes the body. Signed-CBOR SHA256 binds witnesses and is a distinct authenticator.
Changing only witnesses can preserve TxId while changing the signed-CBOR digest.
Bind both digests to actor key identities and the full evidence tuple.
Persist callback events before polling and require returned TxId equality.
Notify each actual public transaction id/status in the host conversation when execution occurs.

Provider eval-only is `evaluation-rejected` or `evaluation-succeeded`, with nativeAcceptance false.
A submit callback is transport evidence only.
Node rejection requires exact public node error evidence.
Inclusion requires block identity or actual UTXO readback bound to that TxId.
An absent response is unknown-finality, not invented rejection or inclusion.
A late or hanging poll leaves immutable confirmation-timeout with nativeAcceptance false.
Later ledger evidence is a separate append-only observation, never a rewritten original receipt.

Construct phase-two negatives from fresh valid bodies.
Decode with CLI11. Mutate one semantic field. Remove stale witnesses. Recompute integrity data when required.
Sign the final body with task keys. Verify signatures before submission.
Retain the original body and mutation map without submitting the original preimage.
Require isValid-true rejection to preserve normal inputs and collateral.
Run a separate reviewed isValid-false control to measure included collateral consumption and return.
Never infer collateral consumption from phase-two rejection alone.

## Full readback and measurements

Persist before/after native queries for all affected inputs and outputs for every control.
Decode the entire HeadDatum and financial lifecycle through the admitted schema.
Preserve canonical array order and all used-id lists.
Compare allowances, liabilities, outstanding duties, work, ordinary state, transport fields, and authority nonce.
Compare complete ordered effects with injective output allocations.
Separate Cash lifecycle balances from ADA fee, collateral, min-ADA sponsorship, and wallet change.
For partial repayment, require preserved residual liability and later valid final settlement.
Admission or partial submission cannot establish economic settlement.

Read actual coinsPerUTxOByte, maxTxSize, protocol version, prices, and maximum transaction execution units.
Retain actual CBOR size, CPU, memory, fee, collateral percent, total collateral, and minimum output ADA.
Measure maximum pre/post datum sizes before lock to preserve continuation sponsorship.
Capture provider evaluation estimates separately from node-observed execution evidence.
Record measured margins and exact parameter digests. Stop on a limit without silently widening it.

## Failure handling and review boundary

After two repeated defect cycles, preserve one minimal reproducer and change the hypothesis before retry.
Retain forecast, hashes, failed bodies, exact statuses, and full readbacks.
Freeze the complete native implementation with tool pins before required independent implementation review.
Changed implementation bytes invalidate its implementation approval.
Native result review consumes the complete run bundle, including failed and refused cases.
All20 native claims require all20 source-specific predicates and full result evidence.
This operational experiment does not certify private/PCD, frozen Complete, or Midnight release.

## Prospective artifact paths

| Stage | Script below TASK/tooling/native-validation | Public output below R |
| --- | --- | --- |
| Candidate check | candidate.mjs | candidate-check.json, run-manifest.json, CANDIDATE-SHA256.json |
| Reservation/bootstrap/cleanup | resources.mjs | resource-plan.json, public-processes.json, genesis.json, protocol-parameters.json |
| Policy/key provenance | pins.mjs | runtime-pin-request.json, runtime-pins.json, pins-dependency-graph.json |
| Parity and sibling | time.mjs | time/parity.json, time/probe-context.json, time/substitution-map.json, sources/calibrated.mori |
| Independent expected state | oracles.mjs | oracles/full-envelopes.json, oracles/logical-correspondence.json |
| Decode/context check | body.mjs | cases/<caseId>/body.json, cases/<caseId>/context.json |
| Receipt adapter | receipts.mjs | cases/<caseId>/receipt-reference.json, host-notifications.jsonl |
| Mutation controls | controls.mjs | cases/<caseId>/mutation.json, cases/<caseId>/witness-check.json |
| Full comparison | readback.mjs | cases/<caseId>/before.json, cases/<caseId>/after.json, cases/<caseId>/comparison.json |
| Twenty-family construction | families.mjs | case-manifest.json, CASE-MATRIX.json |
| Conflict/reference checks | race.mjs | races/concurrent.json, races/sequential.json, references/readback.json |
| Protocol limits | budgets.mjs | cases/<caseId>/protocol-measurement.json, limits-summary.json |
| Frozen reproduction | repro.mjs | REPRODUCTION.json, INPUT-SHA256.json, OUTPUT-SHA256.json |
| Final matrix | matrix.mjs | RESULT-MATRIX.json, RESULT.md |

The driver is `TASK/tooling/native-validation/run.mjs`.
The checker is `TASK/tooling/native-validation/check.mjs`.
The T1 compiler adapter stays under `TASK/tooling/t1-custody-runtime/`.
The existing compile-public path must not overwrite the public atomic validator.

## Regression assertions for prospective tests

The following assertions specify observable behavior. Fixture factories and adapters must be implemented first.

```javascript
// Offline regression specification. No provider calls occur.
assert.equal(malformedCliHash.submitCalls, 0)
assert.equal(staleCandidate.startedProcesses, 0)
assert.equal(shapePins.nativeCompileCalls, 0)
assert.equal(sharedCollateral.submitCalls, 0)
assert.equal(wrongSource.submitted, false)
assert.equal(skewedParity.calibratedSourceWritten, false)
assert.equal(lateConfirmation.nativeAcceptance, false)
assert.equal(lateConfirmation.receipt.status, 'confirmation-timeout')
assert.equal(lateConfirmation.initialReceiptSha, lateConfirmation.afterLateResponseSha)
assert.equal(hangingPoll.receipt.status, 'confirmation-timeout')
assert.equal(returnedIdMismatch.pollCalls, 0)
assert.equal(signatureOnlyReject.semanticNegativePassed, false)
assert.equal(evalOnly.nativeAcceptance, false)
assert.equal(unknownLoser.concurrentPass, false)
assert.equal(sequentialPair.concurrentPass, false)
assert.equal(partialPayout.settlementAccepted, false)
assert.equal(omittedAllowance.comparisonPassed, false)
assert.equal(residualDutyErased.comparisonPassed, false)
assert.equal(unsupportedPrivate.submitCalls, 0)
// Native-only assertions consume independent ledger observations.
assert.deepEqual(validRelease.fullLifecycle, independentReleaseEnvelope.financialPost)
assert.equal(isValidTrueReject.collateralConsumed, false)
assert.equal(includedInvalid.collateralConsumed, true)
assert.equal(concurrentPair.includedBranches, 1)
assert.equal(concurrentPair.sharedAuxiliaryInputs.length, 0)
```

Serialize every native assertion input as a public artifact before comparison.
Derive failure outcomes from node and ledger evidence, never from mutation names.
Use a separate valid baseline for each negative after a prior head has been consumed.
