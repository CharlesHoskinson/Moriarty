# Publication audit — native financial R4/R5

**Verdict: REQUEST_CHANGES**

**Candidate digest:** `0c597a0a286271257a54ea1f047a0b9b82fd365fec6a8a9d4f2294fae209ec79`

The freeze file at `native-financial-r4-r5-publication/PUBLICATION-FREEZE.json` hashes to that digest. Every one of its 197 path entries matches the worktree byte for byte. This review does not approve that digest for merge until the navigation blockers below are corrected. A text fix changes those page hashes and requires a new freeze.

**Reviewer:** requested `grok-4.7` at xhigh, from `AUDIT-ROUTING-20261001.json` (`a415f7d6bdcf2381f2eb18253c2e96096b4abe99d4c104bb76db5e02fa5df9b4`). This session is Grok 4.7. No modelUsage object was exposed here, so this report states no build id.

**Base and tree:** `1d0d598cd63af591be0b2a9f6a5cdeb6768b93bb`. `HEAD` is that commit. The candidate is the uncommitted working tree named by the freeze: 7 modified tracked files and 190 untracked files. No other tracked or untracked path is in the worktree delta.

Loaded the installed `moriarty-dev:develop` skill at `/home/charl/plugins/moriarty-dev/skills/develop/SKILL.md` (SHA-256 `319ce19b11af19e3031ebc318e066b58c9af36b73cb39503ee86a6c4c79a92f1`, same as the Codex cache copy) and the worktree `AGENTS.md`. The checked-in skill hash differs; the installed skill was the one applied. Guarded `status` was not executed: `cmd_status` writes an admin interval when the store exists. A read-only query of `/home/charl/Moriarty/.git/moriarty-dev/state.sqlite3` shows 32 outbox rows, all `confirmed` and delivered. None are undelivered. No Midnight transaction line is pending for this review. No native prover, verifier, build, or network experiment was run.

## Reviewed scope

- All 197 freeze paths, not the diff alone.
- Live byte compare of all 186 `archive-manifest.json` entries to their `original_path` files, including the immutable research root.
- `check-archive.py` on the candidate.
- R4 proof receipt, partition effects, replay artifact, R4 partial-verify log, R5 history-control record, and the copied `accept` source.
- Three actual-result review bodies and their terminal receipts, plus root acceptance as a non-auditor record.
- Wiki transaction `wiki-bundle.json`, `wiki-inspect.json`, and `wiki-apply.json` against the five applied pages and against `HEAD`.
- `evidence/source-inventory.csv` row SRC-0119 against `wiki/meta/ledgers/source-ledger.json` and `raw/sources/native-financial-r4-r5-2026-10-02/collection.json`.
- External ELF pin, the 1130-input hash inventory, and a secret/thought/token/witness scan of the 197 files.

Root authored the acceptance record. That record is not an independent audit and is not counted here. Missing publication-audit providers are not counted as approval. This report does not ask for a review of itself. Later supplementary receipts are outside this freeze, as the freeze scope states.

## Evidence that matches the publication claim

`python3 wiki-llm/native-financial-kernel-2026-10-01/evidence/native-financial-r4-r5/check-archive.py` exited 0 and printed:

`files_checked` 186, `actual_frozen_files` 165, `proof_outputs` 22, `verification_outputs` 30, `three_actual_reviews` true, `recorded_result_invariants` pass, `native_execution` false.

The checker only reads bytes and recorded JSON. It does not spawn a prover, verifier, or network client. An independent compare, which the checker does not do, also matched:

- 165/165 original actual-result bodies exist at the recorded absolute paths, match the freeze hashes, and are byte-identical to the repository copies.
- 21 manifest files outside that 165, including the three actual-result reviews, three terminal receipts, the acceptance record, the 165 freeze itself, and the 13-file reward-funded Rust tree, are byte-identical to their originals.
- Proof directory file count is 22. R5 verification directory file count is 30. Both sets sit inside the 165.
- `NATIVE-FINANCIAL-V10-R5-ACTUAL-RESULT-FREEZE.json` hashes to `32305c38817649e1d6bca2d51a93360e8dc065584a5bf7c8fe8bf2c764fd3c26`.
- `NATIVE-FINANCIAL-V10-R5-ROOT-ACTUAL-ACCEPTANCE.json` hashes to `ad507ee85cd394ad2c5a9d567cae1905ff14a155b32b296d6161786ac0482be6`.

`receipt.json` (`3ffc8ee3105aa265087dcacb6f07e82eb120e07dae6077efd7858573a8569c65`) records `result` Success, `proof_count` 1, strictness `native default Real; proof-verifying enabled`, `runtime_ops` 293, `runtime_reads` 47, consumed DUST `1117490000000001`, allowance `100000000000000000000`, available `5000000000000000000000`, remainder `4999998882509999999999`. The relation `0 < fee <= allowance <= available` and `fee + remainder == available` holds on those integers. `proof.raw` is 6336 bytes. `replay-refusal.txt` is `Failure(ReplayProtectionViolation(IntentAlreadyExists))`. `partition-costs-effects.txt` shows unshielded A1 output total 1010, with claimed spends 1000 and 10. The same receipt bytes are the R5 `independent-ledger-receipt.json`.

The copied provenance source writes that receipt only after the accept gates. `main.rs` lines 285–301 require contract balance 8990, expected storage, A1 outputs `(recipient, 1000)` and `(fee recipient, 10)`, one same-owner NIGHT output of `fixture.night_value`, one dust event, positive consumed fee within allowance, and replay `Failure` with unchanged state hash. `fixture.night_value` in the retained prove config is `1000000000000`. The prove receipt records exit 0, `native_postconditions_passed` true, and elapsed `287.61424126900965` seconds. The R5 verify receipt records exit 0, `native_postconditions_passed` true, elapsed `67.03620270098327` seconds, sampled group CPU `58.18` seconds, and sampled RSS `716906496` bytes.

R5 `history-control-results.json` has eight controls, `all_six_refusals` true, and `pristine_good_again` true. The six refusal exits are 1. The alternate producer and pristine repeat exit 0. `history-suffix.log` contains the typed decoder text `Not all bytes read deserializing 'midnight:vec(ledger-state[v18]):'; 1 bytes remaining`. `history-truncated.log` contains `UnexpectedEof`. R5 `native-financial-v10-r5-verify.log` contains `NATIVE_PUBLIC_HISTORY_CONTROL_SUITE_GOOD_AGAIN_OK`.

The partial R4 verify stays a failure. `native-financial-v10-r4-verify-receipt.json` has exit code 1, stop reason null, supervisor error null, and `native_postconditions_passed` false. Its log ends in `AssertionError` at the copied helper's `expected_error` check after the history-suffix `Not all bytes read` line. That log does not contain `NATIVE_PUBLIC_HISTORY_CONTROL_SUITE_GOOD_AGAIN_OK`. The success note's statement that R4 stopped on the first history-suffix helper assertion, and that the partial suite remains a failure, matches this log.

The three actual-result files state `APPROVE_SCOPED` in their own text, for the fixed public fixture only:

- Sol, `NATIVE-FINANCIAL-V10-R5-ACTUAL-SOL-REVIEW.md` line 3, requested gpt-6.1-sol / high.
- Opus, `NATIVE-FINANCIAL-V10-R5-ACTUAL-OPUS-REVIEW.md` line 5, requested claude-opus-5-5 / high.
- Grok, `NATIVE-FINANCIAL-V10-R5-ACTUAL-GROK-REVIEW.md` lines 13 and 163, requested grok-4.7 / xhigh.

Their report and receipt hashes match the acceptance record. Historical Opus R4 CPU refusal and the 2–1 admission are still described as historical in the success note (line 9), the acceptance `historical_failure_and_dissent_preservation` field, and the three reviews. Disk stop, R3 signal interruption, unsorted-offer refusal, and NightBalance failure are not rewritten as success. Acceptance `open_gates` still lists generic compiler correspondence, source-owner/native-payer and authenticated funding/deployment, generic history/PCD, and Preview settlement. `new_native_allocation_authorized` is false.

`NATIVE-FINANCIAL-V10-R5-VERIFY-INPUT-FREEZE.json` holds 1130 hashes, not 1130 bodies. 111 of those paths are also copied evidence files. The other 1019, including Cargo checkout inputs, stay external. The manifest `external_binary` and that input freeze name ELF `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer`. A read-only hash of that file is `ae7e8dab21c4cf09143fa7d58741c0e18ec5f525018de1b8b4099f6fe5a18239` at 698019744 bytes, matching the pin. The evidence directory's largest file is 214260 bytes. PK, SRS, and the ELF body are not in the git candidate. Prove config keeps the original absolute IR, contract, runtime, preimage, PK, VK, and SRS paths, with `trust` `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`. The manifest says those configs are maps to repository copies, not relocated run commands.

The Rust tree under `native-ledger-reward-funded-v10-r2-candidate/` is a byte copy of the research-root candidate, including `SOURCE-HASHES.json`. It is not wired into a product crate. The worktree delta contains no compiler, runtime, or package change. `fixture_key` at `main.rs` lines 53–55 is the constant `[0x42; 32]`, commented as a public deterministic development key and not a wallet import.

SRC-0119 is status S3 in both `evidence/source-inventory.csv` and the new ledger object `src-bc08b8c8d52f26ff1ec1`. Title, scope, local path, content hash `9634965937a5cad7ae9300157c9daf0025e86692efe4efa1f1fe992b69cc3fd5`, reproduction text, confidence text, and notes match across the CSV row and the ledger legacy record. Collection `collection.json` carries the same id, hash, S3-oriented scope, and the external-body limitations. Parsed ledger comparison against `HEAD` adds only that source object and the `SRC-0119` legacy id. No other source object changed. Session `wiki/sessions/native-financial-kernel-2026-10-01.md` lines 59–63 record the result as an experiment observation, keep the R3 refusal in the previous section, and say no canonical financial claim is promoted. Ledger `review_status` for the new dataset is `unreviewed`, the same marker used across existing ledger entries, not an accepted-claim promotion.

The wiki transaction `ingest-native-financial-r4-r5-20261002` is `valid: true` on inspect and `status: complete` on apply. Canonical bundle SHA-256 is `3a6569b9341cf6a1275c752ef02255275c9cfb9d52497cb822a92298af1f0bf6`, matching both records. Approval SHA-256 `b7360ec04e94af93b523f34e2d3c7e85b070ef876d1d87f63a0e02fcde921206` matches. The five `expected_hashes` match `HEAD`. The five write hashes match the worktree, the freeze, and the draft content files. Unix modes match the apply record: 0644 for four pages and 0600 for the session page.

## Blockers

Current-status navigation still states the pre-R4 refusal as the open result. The session section and `R4-R5-REVIEWED-SUCCESS.md` state the fixture Success. These surfaces contradict that result.

1. `wiki/hot.md` line 27, under the page title "Moriarty current direction" and the heading "Native financial iteration — 2026-10-01", still says the sorted R3 attempt returned excess-NIGHT Failure and that independent verification and full financial Success remain open. Line 29 then says reviewed R4 financial Success and complete R5 verification occurred. The paragraph is not marked as a retained 2026-10-01 snapshot, and the bullet does not say it supersedes that paragraph.

2. `wiki/index.md` line 34, the lede of "Native financial kernel — 2026-10-01", still says the latest sorted attempt refused excess NIGHT and that independent verification and financial Success remain open. The 2026-10-02 success bullet is at line 342, after unrelated research sections. Line 340, immediately above it, still says proof and application gates remain open.

3. `wiki/log.md` line 28 is the newest head entry. It files SRC-0118 and still says retained-genesis diagnosis and supply-consistent funding remain subsequent predicates, with independent verification and full financial gates open. The 2026-10-02 SRC-0119 note is a bullet at line 1271, after the 2026-09-28 and lower-file 2026-10-01 SRC-0115 entries, not a head entry beside line 28. Frontmatter `updated` remains 2026-09-30.

4. `wiki-llm/native-financial-kernel-2026-10-01/README.md` lines 5–7 correctly name R4/R5 as the latest reviewed experiment and keep the open generic gates. The same page then contradicts that status: line 21, in the predicate table, says actual ledger financial acceptance is Open and that the latest sorted proof returned NightBalance Failure, with complete effects, consumed fees, and independent verification still required; line 58 is still headed "Latest actual native refusal"; line 64, under the retitled historical R3 section, still says the immediate predicate is genesis diagnosis and a future complete attempt. The table's section heading says the rows are historical, but the cells use "Latest" and present-tense "remain required" with no as-of date.

These four locations are the publication blockers. The artifact archive, the three actual-result approvals, the S3 ledger row, and the open-gate qualifications do not need to be rewritten to fix them. The fix is to make the current lede, log head, and README "latest" lines agree with the session section, while leaving the R3, disk-stop, and partial-R4 texts as historical failures.

## Nonblocking qualifications

- `receipt.json` does not contain the numeral 8990. Escrow 8990, the 1000/10 split, storage equality, and NIGHT conservation are accept predicates in the copied source, attested because Success is written only after they pass. The printed partition shows the 1010 / 1000 / 10 A1 effects. The success page's phrase "the native receipt records escrow 8990" means that attestation, not a literal receipt field.
- SRC-0119 confidence in both the CSV and the ledger is `high for retained identities/Failure`, copied from the SRC-0118 failure row. Title, scope, and status S3 say Success and do not promote a canonical claim. The two records match each other. The confidence wording is misleading and should be corrected in both places together.
- The ledger rewrite encodes one historical en dash in the SRC-0105 notes as `\u2013` instead of UTF-8 `e2 80 93` (first raw difference at byte 187943). Parsed source objects other than the SRC-0119 addition are unchanged.
- Resource figures stay sampled. Acceptance says sampled group accounting can miss peaks and is not a hard cgroup bound, and that 293 operations / 47 reads were enforced at R4 construction rather than recounted by R5. Cleanup free-space increase `14917431296` bytes stays qualified by concurrency and hardlinks.
- Six candidate files cite local Codex session paths (`NATIVE-FINANCIAL-V10-R5-ACTUAL-SOL-REVIEW.md`, `NATIVE-FINANCIAL-V10-R5-VERIFY-SOL-REVIEW.md`, and four Sol receipt JSON files). The transcript bodies are not in the candidate. Those reviews are byte-identical historical copies; this audit does not ask to redact them and did not open the transcripts. A scan of all 197 files found no PEM private key, bearer token, API-key assignment, provider thinking dump, or env token assignment. The single `sk-` pattern hit is the word `disk-stop` joined to a hash in `NATIVE-FINANCIAL-V10-R4-PROOF-RESOURCE-HANDOFF.md`.
- `hot.md` and `index.md` frontmatter `updated` dates stay 2026-09-30.
- This publication audit is one required reviewer. It is not the Sol or Opus publication audit. Their absence is not approval and is not a defect in this freeze.
