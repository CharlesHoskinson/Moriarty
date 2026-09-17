Continuing the same audit on the corrected source03 freeze: I’ll verify the 105-file manifest, inspect the WS-requirement change and its tests, and confirm everything else is unchanged.Hash-verifying the 105-file freeze against snapshot, workspace, and the prior audited set, then reading the WS-requirement change.```json
{
  "schema": "moriarty.independent-source-review/1",
  "reviewer": {
    "requested": "Grok 4.6 high",
    "returnedIdentity": "grok-4.6",
    "effort": "high",
    "role": "same independent high reviewer, continuation on source03"
  },
  "verdict": "APPROVED",
  "candidate": {
    "path": "deliverables/preview-loan-2026-09-17/recovery-source03/candidate.json",
    "sha256": "f880341f129bf6a1f37d209b7c79b624ddd531aa403cf10eb319ad56c13edc16",
    "bytes": 22136,
    "listedFiles": 105,
    "verifiedMatchingWorkspaceAndSnapshot": 105,
    "snapshotExtraFiles": 0,
    "supersedes": "recovery-source02/candidate-final.json",
    "unchangedFromPriorAudited105": 103,
    "changedFiles": [
      {
        "path": "experiments/moriarty-midnight-financial/ledger/recover-deployment.mjs",
        "sha256": "21dab7504edb5d3669fd1d605d40a24745c8a95ce890e39343a1fc4a99200f00",
        "delta": "single production line: validatePreviewRecoveryPlan requires submissionTransport==='preview-ws'"
      },
      {
        "path": "experiments/moriarty-midnight-financial/ledger/preview-recovery.test.mjs",
        "sha256": "659f3bf72d44e38ed379482f9c0981b4e14eedb41339449bba81a576e3769943",
        "delta": "three negatives: omitted, default, preview-http rejected as PREVIEW_RECOVERY_TRANSPORT at launch plan, recovery plan, and integratePreviewFinancialCase"
      }
    ]
  },
  "priorFindings": {
    "F1": {
      "status": "RESOLVED",
      "resolution": "One complete 105-file authoritative manifest; includes preview-ws-submission.test.ts 858d638730ce0347cb393e260db25eb131c6bba01f66353adc54bfc9c455f99e. Snapshot has no extra files."
    },
    "F2": {
      "status": "RESOLVED",
      "resolution": "validatePreviewRecoveryPlan now checks p.submissionTransport==='preview-ws'. preview-launch and integrate-preview already call that validator, so omitted/default/HTTP recovery cannot reach bootstrap factory selection or driver calls. Independent 31/31 recovery tests include the three new negatives."
    }
  },
  "remainingFindings": [
    {
      "id": "N1",
      "severity": "non-blocking",
      "finding": "PINNED_NM default worktree path is not the launch engine; unset is fail-closed. Launch must bind MORIARTY_MIDNIGHT_NODE_MODULES."
    },
    {
      "id": "N2",
      "severity": "non-blocking",
      "finding": "Per-submit WS timeout remains 180s and does not inherit the launch deadline. Ambiguous timeout stops without replay."
    }
  ],
  "testsObserved": {
    "bundledPreviewTap": "recovery-source03/preview-tests.tap 128/128 pass",
    "thisContinuationRan": {
      "preview-recovery.test.mjs": "31/31 pass, including omitted/default/preview-http transport rejects"
    },
    "priorAuditStillApplies": "source02 full production path, installed WsProvider isReady semantics, financial expectations, constructor pins, and remaining 103 files unchanged"
  },
  "resourceVote": {
    "verdict": "APPROVED",
    "unchangedFromPriorVote": true,
    "liveAdmission": false,
    "allocationId": "sp05-preview-loan-recovery-20260917-01",
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
    }
  },
  "scope": "Offline source approval of the complete current Preview recovery freeze on uncommitted feat/preview-loan-settlement-20260917. Closed deployment/build/roles/current-state/fee/constructor/native-byte/WS-readiness semantics unchanged. Uncertified I2 observation only; not full lifecycle or PCD. This review does not dispatch.",
  "networkAcceptance": false,
  "financialAcceptance": false,
  "proofAcceptance": false,
  "liveAdmission": false
}
```
