# Independent final audit of candidate-01

Verdict: **CHANGES_REQUIRED**

Reviewer: fresh delegated GPT-6 Astra (`gpt-6-astra`), separate from the author. Identity is the assigned runtime identity; no separate external provider-returned model field is available in this tool context. Scope: full plugin quality and compatibility, without campaign work. All runtime modules and changed tests/docs were read in full, not diff-only.

Candidate checkout: `/home/charl/Moriarty-plugin-compatibility-20260911`. Frozen manifest SHA-256: `c8e4be3e2deefb49ab90cc87b71f619117717f71d2faf9d17afcbb2f8e0ba6cf`. All 32 candidate file hashes matched during the completed audit and test verification. Before artifact creation, the parent began the next candidate: `tests/test_compatibility.py` changed to `ebec6b386c6528df3b5df308be97d7db2fee7b2ef2bd720d27d77ad455cc1847`. This review and its test results apply to the original candidate-01 hashes below, not those subsequent repairs.

## ASTRA-COMPAT-01 — P2: removed worktrees hide pending notifications

Repository observation: `plugins/moriarty-dev/scripts/moriarty_dev/store.py:846` still resolves each outbox row's original checkout to establish ownership. Removing that linked checkout makes `get_db_path(owner)` fall back to the vanished checkout's `.moriarty-dev/state.sqlite3`. That differs from the surviving common-directory DB, so the reader silently drops the pending row. The duplicate-submission check at `store.py:748` has the same defect.

Experiment observation using temporary fixtures only:

1. Create main Git metadata and a linked checkout with a `commondir` pointer.
2. Enqueue a synthetic submitted notification through the linked checkout into the shared DB.
3. Before removal, `get_undelivered_txs(db, main)` returns that row.
4. Remove the linked checkout. The same call returns `[]`, while `get_undelivered_txs(db)` still returns the pending row.
5. Re-enqueue the same ID from main: `StoreError: Transaction belongs to another repository`.

Impact: status, report, session/post-tool reminders and CLI delivery all select notifications through this reader. A pending observation loses normal visibility and delivery eligibility after routine worktree retirement. The outbox code predates this patch, but is inconsistent with the candidate's new common-directory ownership fix and documented removed-worktree history contract. This matters to the requested full compatibility review.

Recommendation: use validated database ownership consistently for outbox reads and idempotent submission. Preserve the historical rows; retain unrelated-repository isolation. Add removed-worktree outbox and duplicate-submission regressions. No migration or historical owner rewriting is needed.

No other blocking findings were identified in this review.

## Tests and dispositions

- Full suite: `python3 -m unittest discover -s plugins/moriarty-dev/tests -q` — **197 passed**, 12.483 seconds, Python 3.14.4 on Linux. Original baseline count 176 was supplied; baseline tree was not rerun here.
- Copied exact package to a temporary root and ran self-contained `test_compatibility.py` — **21 passed**, 1.073 seconds. No repository fixtures copied into installed caches.
- Registered PreToolUse shell command with missing native `PLUGIN_ROOT` and valid copied `CLAUDE_PLUGIN_ROOT` — exit **2**, empty stdout, native missing path visible in stderr. No silent cross-version fallback.
- Protocol tests cover malformed/non-object/invalid-UTF-8/oversized payload diagnostics before repository handling, bounded JSON, common-output Stop, camel/snake aliases, permitted calls without explicit allow, preserved denials, exact argv dispatch and harmless action mentions.
- Doctor/report share one observation helper. Package presence remains distinct from activation/trust. Python 3.11 minimum, POSIX shell, deployment-specific Linux runner and untested other hosts are disclosed. This audit does not establish actual host interception or installer continuity.
- Sprint completion uses a least fixed point. Missing requirements, cycles and empty stages stay open; transitive completed dependencies propagate independent of ordering.
- Failure history now accepts common-directory identity and removed-worktree legacy rows while rejecting unrelated DB requests. Explicit stable finding IDs resolve across candidates. Absent/empty resolution lists clear nothing; malformed new lists reject before writes and historical malformed lists clear nothing. Outbox ownership remains the exception above.

Startup: read `AGENTS.md` and the development skill, then ran the required `status --json`. It showed stale source/admission inputs and unavailable accounting; pending transactions were empty. No campaign dispatch followed. Existing status code may append routine administration accounting in the shared DB; that required-command side effect was disclosed to the parent. No further production status calls, source/cache/trust edits, wallet changes or notification mutations were performed. All mutating experiments used temporary fixtures. This file is the only intentional persistent review output.

## Full-file coverage and hashes

Full read means the entire file was inspected, including unchanged runtime modules. Other frozen tests/fixtures were hash-verified and exercised by the suite, without a claim of full manual source reading. Repository AGENTS.md was also read outside this manifest.

| File | Candidate-01 SHA-256 | Coverage |
| --- | --- | --- |
| `plugins/moriarty-dev/.codex-plugin/plugin.json` | `b7429246050235577957f83880a159ffb3fa1c3a7ef79d3109e8e75ade48ae31` | Full read |
| `plugins/moriarty-dev/COMPATIBILITY.md` | `fc675e1fd85772be593cd7b635dcaea1ce95ccb060e6b480f7d82a6dfddf5ea0` | Full read |
| `plugins/moriarty-dev/README.md` | `51e842e6bde1ab3aab836cc598b297aebb5b69e289629e3322de21c252a8fcb9` | Full read |
| `plugins/moriarty-dev/hooks/hooks.json` | `e379a1b6ef37b8be3b0a372dcbb14ff6b6ad749ee2e13260d0fbe582310c6b8a` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/__init__.py` | `f39f3bfc8060c098f28f8595d295ee282396a8a3f1c233e2295108e2fc128c06` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/cli.py` | `01305ac6242d4df1bf8e98891c94df4c5d212f4f09afcbe833e419e58c664c88` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/compatibility.py` | `f5532aab2ca44d90458f68fe613a54e4e23b2ed733b64d55be70a639e2c79e91` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/hook.py` | `c185d2f0344599784c28aa52864971e9c336ca905bafeb09ba2a2d2db4e38084` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/hook_protocol.py` | `b20e35f769827144a71cdba2cae927aa6fabe2074b69e4df12074994025848f8` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/notifications.py` | `ec96e339cc5a29e0b611be8b6c2404c09910fa0712d319980dfc9b55ed608e6c` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/policy.py` | `fc374c7bebdb9b1476d8758fc2eded93f459071da472aac4dde8ac527a83fc02` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/records.py` | `fc060de774f74e3209939fb12a3ab1093793946463d80b0cafcdcb93dd6ff093` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/runner.py` | `5f90aeba35dd5f0ee1b0083efe0e7f753191e54f430d08d7f7d51e0b9dab045a` | Full read |
| `plugins/moriarty-dev/scripts/moriarty_dev/store.py` | `0a7b210fe73e93a8e5d85d81971f49575300f4e96f877bb875d982e49c0bebf2` | Full read |
| `plugins/moriarty-dev/skills/develop/SKILL.md` | `319ce19b11af19e3031ebc318e066b58c9af36b73cb39503ee86a6c4c79a92f1` | Full read |
| `plugins/moriarty-dev/skills/develop/references/execution-focus.md` | `53a6eb5e6cbf570942955402c4872e0456b281ebb1e53e4e998de1b6be4c3780` | Full read |
| `plugins/moriarty-dev/tests/fixtures/README.md` | `bf5578ce76a9cb2f4afe94b6a89e3f0270a61ab3c5d83b84fdb4f08d928a9070` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/fixtures/campaign-admission.json` | `f4cd155c4e719cedde7df0795c7cde79f6e639c95f6f943000addb9896b107e3` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/fixtures/sp01-financial-contract-and-execution-admission.md` | `fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/host-smoke.md` | `9c193dc29275fb5b36230dd878c04b968601424d6820aac1c564466dab8ab2cf` | Full read |
| `plugins/moriarty-dev/tests/reproduce_driver.py` | `b4da8422ffbc5c7a7f661d369125114a90ca1d26564f36e00316c27d72a7767a` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_campaign_history.py` | `1d85385b36466734904db31b86eaea8ca7e12f1006995f3a84dcc0b0f0d74c12` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_compatibility.py` | `4c768670779b3813026a66ca44263245a87db64c167acf1f11bfa4558e009f3c` | Full read |
| `plugins/moriarty-dev/tests/test_execution.py` | `812d49931df4f94773f47866bc71bfc01a8ed782dee115ffef04a29a8dca393e` | Full read |
| `plugins/moriarty-dev/tests/test_hooks.py` | `93c5402a29e1d195645722eb144cf14cc272fe4a270ef6982157279e9e1b5398` | Full read |
| `plugins/moriarty-dev/tests/test_host_adapter.py` | `736b261c2d4042dbc12ad4e9bc42a7bf2a758000c3ce0445ca999af2b6f33b6b` | Full read |
| `plugins/moriarty-dev/tests/test_notifications.py` | `f571603950fe7712d895bb4d257683d6b5112c93312b628086a58888d0ca968c` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_policy.py` | `a9b8f1d15502b9c26dc57b2d5b9895637bf0216571408a32b3181be3ed57a912` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_records.py` | `cbb5840c876e9fa56a7531242a80bfc0390a123a9fd0159681bcc467b2f0ab08` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_regressions.py` | `e2ef91eab1a1ae8a7adbd5f3aa39be859661ee879a0fe253b54cf66c388b74cb` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_runner.py` | `21702e33e7a681a9042966f10015ab39b6c797016290d7383cf64b248af4bac4` | Hash verified; suite evidence |
| `plugins/moriarty-dev/tests/test_store_connections.py` | `f8ddd4cbfe798a2688545dcbb95e0c1fd11f5135b95db3e1018d542d155f5396` | Hash verified; suite evidence |
