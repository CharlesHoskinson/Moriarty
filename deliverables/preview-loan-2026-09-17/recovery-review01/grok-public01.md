I'll start as an independent reviewer: load AGENTS.md, the develop skill, and CLI status, then inspect the frozen recovery candidate without touching source or live wallets.Loading the develop skill and CLI status next, then the frozen candidate snapshot.CLI reports pending Preview transactions; I'll fetch their public statuses, then freeze-read the candidate without touching wallets or submitting.Pending Preview txs are recorded; next I’ll hash-verify the frozen candidate against the snapshot and workspace, then inspect production recovery paths.Candidate lists 104 matching files; the snapshot has one extra WS test. I’ll compare that extra file and start reading the production recovery path.Hashes match for the 104 listed files. Next I’ll inspect recovery, transport, and completion production code in chunks.Preview recovery starts around line 275 in `recover-deployment.mjs`. I’ll read that path plus launch, bootstrap, transport, and completion.SDK `isReady` resolves on socket `connected`. Next I’ll pin constructor hashes, check loan amounts, and run focused offline tests.Midnight Preview transaction 00a91ec05fd1ae30da49f36dc05f8c19949858668b362e58ecee127649e154ee0c: confirmed.
Midnight Preview transaction 0074fcd5910e16809368733e60c97aa6e49724747c8231369bcd30f2d754487b69: unknown-finality.
Midnight Preview transaction 0081d838d9fb4c9924d99c01e2b32da6638e801ce44fc44ea7e00b989c0edbb6df: confirmed.
Midnight Preview transaction 0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a: confirmed.

Those are existing CLI-recorded Preview observations, not submissions from this review.

```json
{
  "schema": "moriarty.independent-source-review/1",
  "reviewer": {
    "requested": "Grok 4.6 high",
    "returnedIdentity": "grok-4.6",
    "effort": "high",
    "role": "fresh independent high reviewer"
  },
  "verdict": "CHANGES_REQUIRED",
  "candidate": {
    "path": "deliverables/preview-loan-2026-09-17/recovery-source02/candidate.json",
    "sha256": "dc10a23829c448f5d112846bab74f060af21debf6def04b7e58b97e087fc6936",
    "bytes": 21835,
    "listedFiles": 104,
    "verifiedMatchingWorkspaceAndSnapshot": 104,
    "snapshotExtraFilesNotInCandidate": 1,
    "snapshotExtra": {
      "path": "experiments/moriarty-midnight-network/hello-world/src/preview-ws-submission.test.ts",
      "bytes": 3965,
      "sha256": "858d638730ce0347cb393e260db25eb131c6bba01f66353adc54bfc9c455f99e",
      "alsoIn": "recovery-source02/candidate-final.json"
    },
    "candidateFinalSha256": "7d007fb98030a70afe5c8c24db099798ed61879ed1d5eb5a601d49cd09edb5bd",
    "candidateFinalFileCount": 105
  },
  "scope": "Offline full-source review of frozen Preview recovery candidate and snapshot. No source edits, wallet/seed reads, network submissions, or live wallet. I2 uncertified fixed-loan observation only; not full new lifecycle or PCD/MC05 acceptance. Does not admit live run.",
  "cliStatus": {
    "oldSp01Campaign": "blocked; stale bindings/accounting; no registered current Preview financial action",
    "doNotReviveOrBypass": true
  },
  "closedBindingsObserved": {
    "deployment": {
      "nativeHash": "ecd8468a01cf85896b2bc53f08c6621029659d31f127b6dc50c7f9eb71b9d400",
      "contract": "8034dffa6ce124cf34135799969d34831da5ce44fd287a3bead2fa3ce293a341",
      "block": 908628,
      "blockHash": "e3f51b9a3d1518980506e9cb5efbbb8487c8acc9989590628cfb5b7330a4f8a4",
      "stateSha256": "a7c4b42737a5d6090e5767f5e59e06912c70f75b5f99536903310bf2ce0a1189",
      "identifiers": [
        "0001ec58451698a6d3816ef48038be1c5aae635f8e148eb2eed75a2e2f632f1f4c",
        "0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a"
      ],
      "historicalDustFeeOnce": "300000000000001",
      "conservativeReservedDustFee": "600000000000002",
      "remainingDustFee": "1399999999999998",
      "originalReservationStopped": true,
      "originalAllocationIdPreserved": "sp05-preview-loan-20260917-01",
      "buildReceiptSha256": "de8bb6bc1adfa6da144e02af4905c96db8e3c65593ab0fbf882365af0a0626a5",
      "noRecompile": true
    },
    "networkGenesis": "3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796",
    "protocolVersion": 1000000,
    "rolesFirstAddress": "e0ec036a0ac15298e1acc6d56c1066910078f3432f41a6276bc7bac2ceccdad5",
    "walletPublicSha256": "24f8d97827b789481309e6f2f345f5a291a384106062431ebe8afe3ae4a42e53"
  },
  "productionPathAssessment": {
    "consumersInspected": [
      "preview-bootstrap.mjs",
      "integrate-preview.mjs",
      "run-local.mjs",
      "recover-store.mjs",
      "recover-deployment.mjs",
      "local-tip.mjs",
      "preview-launch.mjs",
      "providers.mjs",
      "receipt.mjs",
      "financial-comparison.mjs",
      "preview-http-submission.ts",
      "wallet.ts"
    ],
    "publicPreflightBeforeSecretsWalletStore": true,
    "originalStoreOpenedNever_copyThenInspect": true,
    "constructorFullStateThenPrivateReadbackBeforeSingleUseSkipDeploy": true,
    "immediatelyRepeatedCurrentStateOnSkipDeployObserve": true,
    "freshDustNoPendingRequired": true,
    "noRedeployPrepareOrDeployContractOnRecovery": true,
    "stopOnAmbiguityNoAutoResubmit": true,
    "completionRequiresExactDeployPlus3CurrentReservationsPlus2HistoricalIdsPlus4StageReceipts": true,
    "deployFeeCountedOnceInComparator": true,
    "installedSdkIsReady": "WsProvider.connect() returns after socket construct; isReady resolves on onopen/connected; autoConnectMs false disables reconnect; ApiPromise uses existing isConnected and does not reconnect",
    "onePersistentWsPerTxWhenSelected": true,
    "privateMaterialNotProjected": true
  },
  "findings": [
    {
      "id": "F1",
      "severity": "blocking-freeze",
      "finding": "candidate.json is not a complete freeze of the source snapshot. All 104 listed files match workspace and snapshot, but snapshot and candidate-final.json include preview-ws-submission.test.ts (delayed open and late open after abort). Those tests are the claimed source02 WS readiness coverage and are required for the stated transport 19."
    },
    {
      "id": "F2",
      "severity": "blocking-live-run-source",
      "finding": "Recovery plan validation does not require submissionTransport preview-ws. Original HTTP submission failed 403. A structurally valid recovery plan may omit transport or select preview-http and use the SDK default. Continuation template sets preview-ws but is outside the freeze and has deadlineMs 0. Require preview-ws whenever deploymentRecovery is present."
    },
    {
      "id": "N1",
      "severity": "non-blocking",
      "finding": "PINNED_NM default worktree path is not the launch engine; unset is fail-closed. Launch must bind MORIARTY_MIDNIGHT_NODE_MODULES."
    },
    {
      "id": "N2",
      "severity": "non-blocking",
      "finding": "Per-submit HTTP/WS timeout remains 180s and does not inherit the launch deadline. Ambiguous timeout stops without replay, which matches the no-resubmit rule."
    },
    {
      "id": "N3",
      "severity": "non-blocking-test-env",
      "finding": "recover-store empty-destination 0o755 case is umask-sensitive. This review shell umask 0077 made mkdir 0o755 become 0o700; 20/21 store tests passed. Production launch sets umask 0o077 and requires 0o700 destinations."
    }
  ],
  "testsAndObservations": {
    "bundledPreviewTap": "recovery-source02/preview-tests.tap 125/125 pass",
    "thisReviewRan": {
      "preview-recovery.test.mjs": "28/28 pass",
      "preview-launch.test.mjs": "14/14 pass",
      "recover-store.test.mjs": "20/21; umask-sensitive destination-mode case",
      "preview-http-submission.test.ts": "15/15 pass via tsx",
      "preview-ws-submission.test.ts": "4/4 pass via node --experimental-test-module-mocks --experimental-strip-types"
    },
    "constructorSdkPinsMatchInstalled": {
      "compact-js.mjs": "7c34e5ac44b1406080f1cbf9f514c1d887b7bc947327a3f83031d9794caa85b7",
      "midnight-js-types/dist/index.mjs": "465600b3e07a1779ec1f418c550ecf7535d5ad32afe4e145452e3fc2b08ca94d"
    },
    "financialExpectationsMatchStatedTrace": {
      "initializeMintUsdTestAsset": "20000000000",
      "accruePrincipal": "500000000",
      "accrueInterest": "33972602",
      "settleLender": "533972602",
      "borrowerChange": "19466027398",
      "remainingNotional": "4500000000"
    },
    "noLiveWalletOrSubmissionThisReview": true
  },
  "resourceVote": {
    "verdict": "APPROVED",
    "liveAdmission": false,
    "prerequisite": "Source F1/F2 closed on a complete freeze before any dependent live run.",
    "allocationId": "sp05-preview-loan-recovery-20260917-01",
    "separateFromOriginalAllocation": true,
    "originalAllocationDeadlinePreservedUnmutated": true,
    "limits": {
      "systemd": "fresh 30-minute Type=exec unit",
      "MemoryMax": "8GiB",
      "MemorySwapMax": 0,
      "TasksMax": 256,
      "submissions": 3,
      "automaticRetries": 0,
      "dustFeeRemaining": "1399999999999998",
      "historicalReservationsRetained": "600000000000002",
      "overallDustAuthorizationUnchanged": "2000000000000000",
      "actualNativeDeployFeeInComparatorOnce": "300000000000001",
      "grossByLogicalAsset": {"USD_TEST_ASSET": "20000000000"}
    },
    "rationale": [
      "Three new initialize/accrue/settle transactions only; immutable original stopped reservation is not reused or redeployed.",
      "Remaining 1.399999999999998e15 SPECK is 2e15 minus two retained 300000000000001 reservations; comparator charges the actual deploy vFee once.",
      "20bn gross covers consuming the initialize mint; net 533972602 and 19466027398 change are not a reduced gross cap.",
      "Same 30-minute/8GiB/0 swap/256-task envelope as the original four-stage Preview unit, with proving on the existing local proof server outside the unit cgroup.",
      "Stop after ambiguity; no auto resubmit."
    ]
  },
  "networkAcceptance": false,
  "financialAcceptance": false,
  "proofAcceptance": false,
  "liveAdmission": false
}
```
