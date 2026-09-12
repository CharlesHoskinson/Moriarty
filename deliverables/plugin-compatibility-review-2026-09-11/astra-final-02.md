# Independent final audit: candidate-02

Verdict: **APPROVE**

Reviewer: **gpt-6-astra**, fresh delegated independent reviewer (`/root/final_astra_review_02`). This is this reviewer's own source audit and test observation, not a reused approval or a claim of another provider's review.

Scope: the full frozen `plugins/moriarty-dev` package in `/home/charl/Moriarty-plugin-compatibility-20260911`, evaluated against its `COMPATIBILITY.md` contract. Read repository AGENTS.md and the development skill. Per explicit review scope, did not run production status, resume product queues, modify source/cache/production accounting, or perform installation/campaign work.

## Integrity and inspection

All 32 entries in `candidate-02.json` matched SHA-256 before testing and again after testing. Manifest SHA-256: `5c8163265ef8369a518c105652a73ee8504f95111718d760fdd98b5737dda275`.

Read all runtime modules in full: `__init__.py`, `cli.py`, `compatibility.py`, `hook.py`, `hook_protocol.py`, `notifications.py`, `policy.py`, `records.py`, `runner.py`, and `store.py`. Read the manifest and hook definitions, compatibility and README documentation, checked-in development guidance and execution-focus reference, host smoke procedure and fixture guidance. Inspected the complete relevant test modules, including source-admission, policy, runner, execution, historical campaigns, hooks, notification lifecycle, compatibility and connection cleanup coverage. This was a full-source review, not a diff-only review.

## Independently reproduced checks

Command from the candidate checkout:

```text
PYTHONDONTWRITEBYTECODE=1 python3 -B -m unittest discover -s plugins/moriarty-dev/tests -q
Ran 199 tests in 13.112s
OK
```

The suite exercises temporary repositories and retained read-only fixture sources. The runner tests use the existing local Foreman launcher with disposable checks; they do not dispatch production campaigns.

Additional independent temporary-fixture assertions passed:

- A transaction enqueued from a linked checkout remains readable from the main checkout after removal of the linked `.git` entry. Confirmation remains intact after duplicate submission, and low-level delivery empties the pending outbox. This delivery assertion used an explicit synthetic acknowledgement to exercise store behavior only; actual transcript observation is separately exercised by the notification suite.
- Explicit empty `resolvedFindings` takes precedence over legacy `resolved`; a malformed historical resolution string clears no blocker.
- A missing nonempty native `PLUGIN_ROOT` produces a visible process failure with empty stdout even when `CLAUDE_PLUGIN_ROOT` points to usable code.
- A missing `python3` produces a visible process failure with empty stdout, without executing a fallback package or emitting a permit response.

## Findings and disposition

No blocking correctness or regression finding in the reviewed compatibility contract.

The removed-worktree outbox concern is resolved: `store.py:111` validates database ownership before shared projections; `enqueue_tx` preserves an existing transaction without requiring its historical checkout to remain present; `get_undelivered_txs` reads pending rows from that verified database. The compatibility suite also rejects unrelated repository/database pairs before enqueue creates a database. Existing transaction status and delivery checks continue to apply.

Hook outputs preserve denials and bounded JSON, omit explicit allow, and keep Stop free of an event-specific envelope. Common `systemMessage` output remains valid. Dispatch recognition is intentionally limited to the documented simple forms and exact raw argv. Installation observations retain unknown activation/trust. Sprint completion uses dependencies rather than only local stage completion. Explicit stable finding resolutions preserve the intended historical behavior without row migration.

A minor documentation refinement may be made in a later candidate: README.md:85 describes the compatibility root variable without restating native-root precedence. COMPATIBILITY.md and the executable hook definitions already state and implement the correct precedence, so this is not a release blocker.

## Limits

Approval covers the exact source candidate and reproduced local tests. It does not establish fresh actual-host interception, uninterrupted installer upgrades, macOS/Windows support, another Python version, independent Grok approval, production admission, or financial/proof acceptance. Keep the documented host smoke and preserved-cache procedure for installation. Changed bytes require current review.

## Exact reviewed file hashes

```json
{
  "plugins/moriarty-dev/.codex-plugin/plugin.json": "b7429246050235577957f83880a159ffb3fa1c3a7ef79d3109e8e75ade48ae31",
  "plugins/moriarty-dev/COMPATIBILITY.md": "0d1700726352f941e76e8cee1c2a999aba2155f6061c49765e5c669d52f36e08",
  "plugins/moriarty-dev/README.md": "51e842e6bde1ab3aab836cc598b297aebb5b69e289629e3322de21c252a8fcb9",
  "plugins/moriarty-dev/hooks/hooks.json": "e379a1b6ef37b8be3b0a372dcbb14ff6b6ad749ee2e13260d0fbe582310c6b8a",
  "plugins/moriarty-dev/scripts/moriarty_dev/__init__.py": "f39f3bfc8060c098f28f8595d295ee282396a8a3f1c233e2295108e2fc128c06",
  "plugins/moriarty-dev/scripts/moriarty_dev/cli.py": "01305ac6242d4df1bf8e98891c94df4c5d212f4f09afcbe833e419e58c664c88",
  "plugins/moriarty-dev/scripts/moriarty_dev/compatibility.py": "f5532aab2ca44d90458f68fe613a54e4e23b2ed733b64d55be70a639e2c79e91",
  "plugins/moriarty-dev/scripts/moriarty_dev/hook.py": "c185d2f0344599784c28aa52864971e9c336ca905bafeb09ba2a2d2db4e38084",
  "plugins/moriarty-dev/scripts/moriarty_dev/hook_protocol.py": "b20e35f769827144a71cdba2cae927aa6fabe2074b69e4df12074994025848f8",
  "plugins/moriarty-dev/scripts/moriarty_dev/notifications.py": "ec96e339cc5a29e0b611be8b6c2404c09910fa0712d319980dfc9b55ed608e6c",
  "plugins/moriarty-dev/scripts/moriarty_dev/policy.py": "fc374c7bebdb9b1476d8758fc2eded93f459071da472aac4dde8ac527a83fc02",
  "plugins/moriarty-dev/scripts/moriarty_dev/records.py": "fc060de774f74e3209939fb12a3ab1093793946463d80b0cafcdcb93dd6ff093",
  "plugins/moriarty-dev/scripts/moriarty_dev/runner.py": "5f90aeba35dd5f0ee1b0083efe0e7f753191e54f430d08d7f7d51e0b9dab045a",
  "plugins/moriarty-dev/scripts/moriarty_dev/store.py": "66417c0a1b73fd0c1ad9ee627c74430fdb1edfc8afba2aec155479dba994dfa5",
  "plugins/moriarty-dev/skills/develop/SKILL.md": "319ce19b11af19e3031ebc318e066b58c9af36b73cb39503ee86a6c4c79a92f1",
  "plugins/moriarty-dev/skills/develop/references/execution-focus.md": "53a6eb5e6cbf570942955402c4872e0456b281ebb1e53e4e998de1b6be4c3780",
  "plugins/moriarty-dev/tests/fixtures/README.md": "bf5578ce76a9cb2f4afe94b6a89e3f0270a61ab3c5d83b84fdb4f08d928a9070",
  "plugins/moriarty-dev/tests/fixtures/campaign-admission.json": "f4cd155c4e719cedde7df0795c7cde79f6e639c95f6f943000addb9896b107e3",
  "plugins/moriarty-dev/tests/fixtures/sp01-financial-contract-and-execution-admission.md": "fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172",
  "plugins/moriarty-dev/tests/host-smoke.md": "9c193dc29275fb5b36230dd878c04b968601424d6820aac1c564466dab8ab2cf",
  "plugins/moriarty-dev/tests/reproduce_driver.py": "b4da8422ffbc5c7a7f661d369125114a90ca1d26564f36e00316c27d72a7767a",
  "plugins/moriarty-dev/tests/test_campaign_history.py": "1d85385b36466734904db31b86eaea8ca7e12f1006995f3a84dcc0b0f0d74c12",
  "plugins/moriarty-dev/tests/test_compatibility.py": "ebec6b386c6528df3b5df308be97d7db2fee7b2ef2bd720d27d77ad455cc1847",
  "plugins/moriarty-dev/tests/test_execution.py": "812d49931df4f94773f47866bc71bfc01a8ed782dee115ffef04a29a8dca393e",
  "plugins/moriarty-dev/tests/test_hooks.py": "93c5402a29e1d195645722eb144cf14cc272fe4a270ef6982157279e9e1b5398",
  "plugins/moriarty-dev/tests/test_host_adapter.py": "736b261c2d4042dbc12ad4e9bc42a7bf2a758000c3ce0445ca999af2b6f33b6b",
  "plugins/moriarty-dev/tests/test_notifications.py": "f571603950fe7712d895bb4d257683d6b5112c93312b628086a58888d0ca968c",
  "plugins/moriarty-dev/tests/test_policy.py": "a9b8f1d15502b9c26dc57b2d5b9895637bf0216571408a32b3181be3ed57a912",
  "plugins/moriarty-dev/tests/test_records.py": "cbb5840c876e9fa56a7531242a80bfc0390a123a9fd0159681bcc467b2f0ab08",
  "plugins/moriarty-dev/tests/test_regressions.py": "e2ef91eab1a1ae8a7adbd5f3aa39be859661ee879a0fe253b54cf66c388b74cb",
  "plugins/moriarty-dev/tests/test_runner.py": "21702e33e7a681a9042966f10015ab39b6c797016290d7383cf64b248af4bac4",
  "plugins/moriarty-dev/tests/test_store_connections.py": "f8ddd4cbfe798a2688545dcbb95e0c1fd11f5135b95db3e1018d542d155f5396"
}
```
