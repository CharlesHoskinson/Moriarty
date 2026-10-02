# Grok publication audit — repaired R4/R5 candidate

**Verdict: APPROVE_SCOPED**

**Candidate digest:** `a775c1f5b20163ad65666fb2925bf02f154c4680215e3665acb52ea8e75ba4fe`

**Reviewer:** grok-4.7, requested effort xhigh, as pinned in `AUDIT-ROUTING-20261001.json`. This is one required publication audit. Missing providers do not approve. The author, root, is not an independent auditor. The earlier freeze `0c597a0a286271257a54ea1f047a0b9b82fd365fec6a8a9d4f2294fae209ec79` is not approval. This report is outside the candidate and does not require a review of itself.

No publication blockers remain in this digest.

## Scope

Worktree `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`, HEAD `1d0d598cd63af591be0b2a9f6a5cdeb6768b93bb`. Freeze file `PUBLICATION-R2-CHECKED-FREEZE.json` is 40360 bytes and hashes to the digest above. Its parent pointer is the earlier freeze. Scope text: exact repaired full publication candidate, not a new native result.

Loaded `moriarty-dev:develop` and the worktree `AGENTS.md`. The pinned route (gpt-6.1-sol high, claude-opus-5-5 high, grok-4.7 xhigh) overrides the September historical routes for this audit. No candidate file was edited. No native prover, verifier, build, or network command was run. Counterpart current R2 reports were not read. Host transcripts were not opened.

The guarded status command was not executed. `cmd_status` writes an `admin_intervals` row when the store exists (`plugins/moriarty-dev/scripts/moriarty_dev/cli.py` around the `record_admin_interval(..., "status")` call). A read-only immutable query of `/home/charl/Moriarty/.git/moriarty-dev/state.sqlite3` shows 32 delivered `confirmed` outbox rows and 0 undelivered. No pending Midnight notification line is outstanding. Those rows are program notification records. They are not a public transaction from this experiment.

## Freeze

Independent SHA-256 of all 197 listed paths: 197 match, 0 missing, 0 mismatch.

Against the parent freeze, 190 hashes are unchanged and 7 changed:

| Path | New SHA-256 |
|---|---|
| `evidence/source-inventory.csv` | `dd8f99df34153ce39183412400be01ba3a1aac8afef026db455a1296ccd5e0d7` |
| `wiki-llm/native-financial-kernel-2026-10-01/R4-R5-REVIEWED-SUCCESS.md` | `33849d39fc9210579ff6a6159137555c1311531c0d6b75a361c79d19b02495c7` |
| `wiki-llm/native-financial-kernel-2026-10-01/README.md` | `a9694ef1ef1f2993049e2dcb264de8da81c21e3182ee439ae062b99f866bf82a` |
| `wiki/hot.md` | `ec8a98f48a5483508841e84dc1fd3382608f95d3464d743e4233929cca59e570` |
| `wiki/index.md` | `7efd3a906589eb0e8e8d84fee196223bda9673d782c47d0f3920c2bd9e0d08a3` |
| `wiki/log.md` | `d22dab8420ef396cecc56a89ddb9566894ff457bfd1cec22674a9ac963abbd6a` |
| `wiki/meta/ledgers/source-ledger.json` | `c4d79efe35170247f0b6856d3dc7cf4ebaf548638cd0c7bd2bfdf81b47054654` |

`checked-repair-delta.json` records those same old and new hashes. `wiki/sessions/native-financial-kernel-2026-10-01.md` stays `225f06fcc3adba085312f3badaebaec63c2472d42c22ff3e68ec1ad8b6d819aa`.

The 186 `archive-manifest.json` paths are inside the unchanged set. The untracked publication trees (`raw/sources/native-financial-r4-r5-2026-10-02/`, `wiki-llm/native-financial-kernel-2026-10-01/evidence/native-financial-r4-r5/`, and `R4-R5-REVIEWED-SUCCESS.md`) contain 190 files, all listed in the freeze, with no extra file.

`PUBLICATION-R2-FREEZE.json` hashes to `ac8695935531d1da750a5546587c86c9eb515ff3f3465b49062cbd0ce00062a5`. It is an intermediate map. It differs from this candidate only in `wiki/index.md` and `wiki/log.md`, at the pre-normalization hashes below. It is not the audited digest.

## Repair

SRC-0119 confidence is now the same string in `evidence/source-inventory.csv` and in ledger legacy record `src-bc08b8c8d52f26ff1ec1`:

`high for recorded identities and fixed-fixture Success/verification; retained historical failures preserved; broader product gates open`

Every other SRC-0119 field matches between the CSV row and that ledger record. Parsed ledger comparison against the initial applied body (`/tmp/moriarty-r4-r5-wiki-drafts/wiki/meta/ledgers/source-ledger.json`, hash `78e0bcd3a73586418fc3439c6998ba1259a157d4973052101b4f636a578e462d`) shows one record change, and that change is only this confidence field. Schema, `generated_at` (`2026-10-01T15:13:53Z`), source count (6227), and legacy id map are unchanged. SRC-0118 still carries `high for retained identities/Failure; broader product gates open`.

The initial ledger contained one `\\u2013` escape. The current file contains one UTF-8 en-dash (`e2 80 93`) and no `\\u` escape. The character is in SRC-0105 notes: “Four original papers remain SRC-0100–0103.” Parsed text is unchanged by that spelling.

`R4-R5-REVIEWED-SUCCESS.md` now says the acceptance path enforced escrow 8990, A1 outputs 1000 and 10, storage agreement, and conservation of the 1e12 NIGHT fixture before the receipt was emitted. That matches `accept` in the copied `main.rs`: the 8990 storage check, sorted A1 outputs `(recipient, 1000)` and `(fee_recipient, 10)`, and the same-owner NIGHT output of `fixture.night_value` run before the receipt JSON is returned. `receipt.json` still has no literal 8990 field. Its recorded values are Success, `proof_count` 1, strictness `native default Real; proof-verifying enabled`, fee `1117490000000001`, allowance `100000000000000000000`, available `5000000000000000000000`, remainder `4999998882509999999999`, 293 operations, and 47 reads. Replay text is `Failure(ReplayProtectionViolation(IntentAlreadyExists))` with the accepted state preserved. Application fee 10 and protocol DUST stay separate quantities in that note.

The same note says the canonical alternate fixture also changes operation registration and is not an isolated timestamp-only control. `history-control-results.json` records `alternative_history_qualification`: “canonical valid local keyless history also differs in operation registration; not isolated timestamp-only test.”

Current navigation:

- `wiki/hot.md` updated date `2026-10-02`. The R3 section is headed historical. Lines 29–31 are the current fixed-fixture R4 Success and complete R5 verification, with generic correspondence, authenticated custody/history/PCD, and Preview still open.
- `wiki/index.md` lines 33–35 state SRC-0119, the fixed public fixture, three actual-result approvals, historical SRC-0118, and the same open product gates. Line 341 is a historical 2026-10-01 preparation snapshot.
- `wiki/log.md` opens with the 2026-10-02 SRC-0119 entry and keeps the dated SRC-0118 Failure entry beneath it. The new entry says the local proof, application, and verification gaps in that R3 entry are superseded, and that there is no public transaction and no canonical financial claim.
- `wiki-llm/native-financial-kernel-2026-10-01/README.md` leads with the reviewed R4/R5 result. The evidence table is headed historical through R3. The acceptance row is “Open at R3” and points at R4/R5 for the local fixture gaps. V8 and R3 sections are headed historical. Authenticated custody and Preview remain open there.
- The session page already ends with the 2026-10-02 R4/R5 section (lines 59–63). Its hash did not change in this repair. Its opening sections remain dated snapshots, and line 16 says current execution evidence is recorded below.

## Evidence identity

`python3 wiki-llm/native-financial-kernel-2026-10-01/evidence/native-financial-r4-r5/check-archive.py` exited 0:

`{"files_checked": 186, "actual_frozen_files": 165, "proof_outputs": 22, "verification_outputs": 30, "three_actual_reviews": true, "recorded_result_invariants": "pass", "native_execution": false}`

The checker compares repository copies with the manifest and the embedded freezes. It does not execute native code and does not read the live research root. This audit did not re-run `/tmp/moriarty-r5-inspect-actual.py`.

Manifest and acceptance agree:

- actual165 `32305c38817649e1d6bca2d51a93360e8dc065584a5bf7c8fe8bf2c764fd3c26`
- root acceptance `ad507ee85cd394ad2c5a9d567cae1905ff14a155b32b296d6161786ac0482be6`
- three reviews, each `APPROVE_SCOPED`: gpt-6.1-sol, claude-opus-5-5, grok-4.7

Prove receipt: exit 0, `native_postconditions_passed` true, elapsed `287.61424126900965` seconds. Verify receipt: exit 0, same postcondition flag, elapsed `67.03620270098327` seconds, sampled CPU `58.18` seconds, sampled RSS `716906496` bytes. The R4 partial verify receipt remains exit 1 and `native_postconditions_passed` false. Eight history controls: `all_six_refusals` true, `pristine_good_again` true. The public development key remains `SigningKey::from_bytes(&[0x42; 32])` with the comment that it is not a wallet import.

SRC-0119 collection scope is the fixed public fixture only. Its limitations text matches the CSV notes: 165 frozen bodies plus final acceptance and reviews; nested 1130 closures are hash inventories; ELF, PK, SRS, Cargo/runtime caches, and unrelated transcripts stay outside Git; no self-contained native rebuild; no user wallet secret, public transaction, or native allocation. Status in the inventory row is S3. Source-owner/native-payer, authenticated funding/deployment, general compiler/history/PCD, and Preview financial settlement remain open.

Recorded utility logs were read and not re-executed. `beta-tests-configured.txt` reports tests 193, pass 193, fail 0. `r2-lint-comparison.json` reports candidate exit 1, 20 preexisting issues, and `no_new_findings` true. The two stale index entries are preexisting missing U0 targets at `wiki/index.md` lines 55 and 67.

## Wiki transactions

Canonical bundle hash is `json.dumps(..., sort_keys=True, separators=(",", ":"), ensure_ascii=False)`.

| Transaction | Operation | Canonical bundle | Approval | Result |
|---|---|---|---|---|
| Initial | `ingest-native-financial-r4-r5-20261002` | `3a6569b9341cf6a1275c752ef02255275c9cfb9d52497cb822a92298af1f0bf6` | `b7360ec04e94af93b523f34e2d3c7e85b070ef876d1d87f63a0e02fcde921206` | inspect `valid` true; apply `complete`; inspect and apply hashes equal |
| R2 | `ingest-native-financial-r4-r5-20261002-r2` | `fed442318a2c4c2fab3f136310eef788406c4e72551b960c5909a6d71f5ea6bf` | `818d283fd652dac1a33f7ea516e72d2773b5b30dab2f651d4f1c519b0f8d7b5e` | same; expected hashes equal the initial apply hashes |
| R2b | `ingest-native-financial-r4-r5-20261002-r2b` | `d87229f02bcf6e2a30d7ecc4a854f47c4284e72d6dac010733686d3aae3e917e` | `802ac903f9d0601d3c27cac682b262116cbc8e1c2b90a167d1ebf05e3ebc97bc` | same; expected hashes equal the R2 apply hashes |

R2 apply hashes: session unchanged `225f06fc…`; ledger `c4d79efe…`; hot `ec8a98f4…`; index `01140efb6f72960091ec43824ff4d2b9863da2cb1b3b61917909ad34cf33b54f`; log `6a429a65c9305739397aa3c9b17fea17bada653aaaf2cbd6ac1783d3d112320d`.

R2b apply hashes equal the worktree and this freeze for all five paths: hot `ec8a98f48a5483508841e84dc1fd3382608f95d3464d743e4233929cca59e570`, index `7efd3a906589eb0e8e8d84fee196223bda9673d782c47d0f3920c2bd9e0d08a3`, log `d22dab8420ef396cecc56a89ddb9566894ff457bfd1cec22674a9ac963abbd6a`, ledger `c4d79efe35170247f0b6856d3dc7cf4ebaf548638cd0c7bd2bfdf81b47054654`, session `225f06fcc3adba085312f3badaebaec63c2472d42c22ff3e68ec1ad8b6d819aa`.

R2 and R2b drafts match their declared write hashes. Session, ledger, and hot drafts are byte-identical between R2 and R2b. Index (29771 vs 29770 bytes) and log (90687 vs 90686 bytes) differ by one trailing newline only. Both correction bundles have empty `address_requests` and empty `source_manifest_updates`. Modes are 0644 for hot, index, log, and the ledger, and 0600 for the session page, matching the transactions and the worktree. Initial draft files under `/tmp/moriarty-r4-r5-wiki-drafts/` still match the initial write hashes.

## Privacy

The seven changed files contain no PEM private key, bearer token, API token, provider thought, or environment secret. The fixed `[0x42; 32]` development key is public fixture material in unchanged source. Historical absolute paths and original configs remain evidence. They are not portable execution commands.

## Nonblocking qualifications

- Escrow 8990 is an acceptance-path check in `main.rs`, then the receipt is written. It is not a field of `receipt.json`.
- Verify CPU 58.18 seconds and RSS 716906496 bytes are sampled. The result note says sampling can miss peaks and is not a hard cgroup bound. Prove sampled CPU 321.87 seconds and RSS 2905444352 bytes are separate figures.
- The R4 partial verification remains a failure. Earlier disk stops, the signal interruption, unsorted offers, and the NightBalance refusal remain failures. Historical Opus R4 CPU dissent and the recorded 2–1 decision stay visible. Root acceptance is not an independent audit.
- Nested 1130 dependency, runtime, ELF, PK, and SRS closures stay external and hash-bound. A fresh clone can check copied bytes. It cannot rebuild and replay the native experiment from this packet alone.
- Generic compiler correspondence, source-owner/native-payer correspondence, authenticated funding and deployment, general history/PCD, and Preview financial settlement remain open. This is an S3 fixed-fixture experiment. It does not promote a canonical financial acceptance claim. No public transaction and no native allocation are claimed.
- `wiki/index.md` and `wiki/log.md` have `updated: 2026-10-02` and also `updated_at: 2026-09-30T21:34:04Z`. The visible current ledes use 2026-10-02. The lint file’s two stale index entries are older missing U0 links.
- Inside the historical R3 heading, `wiki/hot.md` line 27 still uses the link label “Current native proof work.” The next heading is the current R4/R5 result.
- In the README section headed historical through R3, the cryptography row still says the independent verifier and full Success remain open. The following row and the current lede place R4/R5 over those local fixture gaps.
- The dated SRC-0118 log sentence still says independent verification and the full financial, authentication, and Preview gates stay open. The newer SRC-0119 head supersedes the local proof, application, and verification gaps and leaves the product gates open.
- Recorded Beta and lint results above were inspected as files. They were not run again in this audit.
- Supplemental review and check receipts are expected to be appended later with their own hashes.

## Approval boundary

This approves publication of the already reviewed fixed-public-fixture R4 native Success and the complete R5 verify-only suite, together with the three actual-result reviews, receipts, root acceptance, preserved failures, and the repaired navigation and ledger metadata in digest `a775c1f5b20163ad65666fb2925bf02f154c4680215e3665acb52ea8e75ba4fe`.

Existing Rust and source wrappers in the packet are provenance. They are not a product-behavior change. Merge of this reviewed publication is not product acceptance of the open gates above.
