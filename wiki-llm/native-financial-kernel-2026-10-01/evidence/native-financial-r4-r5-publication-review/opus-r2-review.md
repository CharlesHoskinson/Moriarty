# Moriarty R4/R5 publication candidate: independent R2 audit (claude-opus-5-5, high)

Verdict: **APPROVE_SCOPED**

Candidate: `PUBLICATION-R2-CHECKED-FREEZE.json`, sha256 `a775c1f5b20163ad65666fb2925bf02f154c4680215e3665acb52ea8e75ba4fe`. Base `1d0d598cd63af591be0b2a9f6a5cdeb6768b93bb`. Worktree `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`, branch `evidence/native-financial-r4-r5-20261002`.

Requested reviewer: claude-opus-5-5, high effort, per `AUDIT-ROUTING-20261001.json` (sha256 `a415f7d6…`). I did not delegate, did not read the counterpart R2 reports, and did not count the earlier `0c597a0a…` freeze or any approval of it. I ran no native execution, build, proof or network experiment, and made no candidate edits. I did not inspect host conversation transcripts or credentials.

Approval scope: this is a publication audit of an already-accepted result. It approves faithful, byte-identical publication of the fixed trusted-public-fixture R4 finalized financial proof with default Real native Success, the complete R5 verify-only suite, the three actual-result reviews and receipts, and the root acceptance. It approves no new product, financial, PCD or Preview claim.

## Startup

- `moriarty-dev:develop` is not exposed by the host. I read and applied the checked-in `plugins/moriarty-dev/skills/develop/SKILL.md` and the current `AGENTS.md`.
- `cli.py --repo . status --json` (read-only) reports capability SP01.6 loan-swap-subset. The blocked action is unrelated to this publication. `pendingTransactions` is `[]`, so no Midnight notification is due.

## What I verified independently

1. **Freeze identity.**
   - The freeze file hashes to `a775c1f5…4fe`. It lists 197 paths.
   - All 197 worktree files rehash to their frozen values.
   - `git status --porcelain --untracked-files=all` returns exactly those 197 paths, with nothing extra and nothing missing.
   - The only changes are under `evidence/`, `raw/sources/`, `wiki/` and `wiki-llm/`. Nothing under `packages/` or other product paths changed, so the Rust/Python wrappers are provenance copies only.
   - `PUBLICATION-R2-FREEZE.json` differs from the checked freeze only in `wiki/index.md` and `wiki/log.md`, which is the R2b blank-line normalization.
   - Against the parent freeze `0c597a0a…`, exactly seven paths changed: the CSV, the result note, the README, hot, index, log and ledger. That matches `r2-repair-delta.json` plus R2b.
2. **Original-evidence identity.**
   - `archive-manifest.json` (sha256 `d074d0f8…`, equal to `collection.json` `archive_manifest_sha256`) maps 186 files.
   - I hashed every manifest entry against both its repository copy and its live `original_path`. All 186 match, none are absent, and no file in the evidence directory is unmanifested except the manifest itself and `check-archive.py`.
   - The 186 files are all 165 actual-result frozen bodies (which contain the R4 88-file and 67-file freezes), plus 21 extras. The extras are the 3 actual reviews, 3 actual receipts, the R5 actual freeze, the root acceptance, and the 13 R2 native-consumer source files.
   - The actual165 freeze is `32305c38…3fc26` and the root acceptance is `ad507ee8…2be6`, both equal to the stated values.
   - The external ELF `beta-native-ledger-consumer` rehashes to `ae7e8dab…18239`, matching the manifest and both run receipts.
3. **Archive checker.**
   - `check-archive.py` passed: 186 files, 165, 22, 30, three reviews, invariants pass, no native execution.
   - I read the checker in full. It only hashes and parses: it never imports or executes archived code, and it confines paths to the archive.
   - It checks the following, and I confirmed each by hand:
     - the 165, 88 and 67 maps;
     - the review and receipt hashes and all three `APPROVE_SCOPED` verdicts;
     - exit 0 with postconditions for prove/verify;
     - an exact output-file set of 22 and 30;
     - `0 < fee ≤ allowance ≤ available` and `fee + remainder = available`;
     - R4 `receipt.json` byte-equal to the R5 `independent-ledger-receipt.json`;
     - eight control log hashes with the expected exit codes;
     - 1035 refusal indices 0..1034, and the first good log byte-equal to the good-again log;
     - the R4 partial verify recorded as exit 1 with postconditions false;
     - the source hashes.
4. **Recorded results against the data and source.**
   - The arithmetic holds: 5e21 − 4999998882509999999999 = 1117490000000001 (> 0 and ≤ 1e20 allowance). Fixture config: `night_value` 1e12, allowance 1e20, trust `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`.
   - The published `src/main.rs` accept path enforces the claims in the note before it emits the receipt:
     - default `well_formed`, `TransactionResult::Success`, escrow 8990 and storage equality;
     - funding input consumed;
     - A1 outputs exactly 1000 and 10, and a single same-owner NIGHT output of `night_value`;
     - one `DustInitialUtxo`, with `0 < consumed ≤ allowance` and non-negative balancing;
     - replay `Failure` with an unchanged state hash.
   - 293 operations and 47 reads are enforced at construction (`main.rs:168`) and appear as literals in the receipt. The note's qualification that R5 did not recount them is accurate.
   - `application-result.txt` records Success. `replay-refusal.txt` records `ReplayProtectionViolation(IntentAlreadyExists)`.
   - The R4 proof outputs, the R5 copies and the `good-again` outputs are byte-identical.
   - `proof.raw` is 6,336 bytes.
   - Recorded resources match the note: R4 prove 287.614 s; R5 verify 67.036 s, 58.18 s sampled CPU, 716906496 B RSS; cleanup delta 14917431296 B.
   - The alternate-history caveat added in R2 is supported by the Opus F4 and Sol actual reports and by the controls file.
5. **Failures, dissent and resource qualifications are preserved.**
   - The R4 partial verify receipt is retained as exit 1.
   - The R3 interruption, disk-stop and cleanup freezes are retained.
   - The Opus R4 `REFUSE_CURRENT_LIVE_ADMISSION` CPU clarification and the root 2–1 decision text ("Opus CPU live-admission dissent is preserved, not claimed satisfied") are present unchanged.
   - The note and session text present all of these as failures or dissent, not success.
6. **Inventory and ledger.**
   - The CSV row SRC-0119 and the ledger `legacy_records[0]` agree field-for-field across all 15 columns, including the repaired confidence.
   - Both `sha256` values equal the collection hash `96349659…3fc5`.
   - The ledger diff is append-only: one source entry plus a `legacy_source_ids` mapping. The non-ASCII byte count is unchanged from HEAD (3 → 3), which is consistent with the restored Unicode spelling.
   - `git diff --check` is clean. The new files have no trailing whitespace and end in a single newline.
7. **Canonical wiki transactions.**
   - The initial bundle's `expected_hashes` equal the HEAD blobs for all five paths.
   - The initial apply outputs equal the R2 `expected_hashes`, the R2 apply outputs equal the R2b `expected_hashes`, and the R2b apply output hashes equal the current worktree and freeze.
   - All three inspect records are `valid: true` with matching approval and bundle digests, and all three applies are `complete`.
   - The R2 and R2b drafts are preserved in the publication directory. The initial drafts still exist in `/tmp` with matching hashes (see Q4).
8. **Navigation and current-versus-historical status.**
   - Index and hot now lead with the R4/R5 result and keep R3 under explicit historical headings and dated wording.
   - Log adds a dated 2026-10-02 entry above the retained R3 entry.
   - The README retitles its R3, V8 and pre-V7 sections as historical and points to the reviewed result.
   - The session page adds a dated R4/R5 section.
   - All relative links in the five authored pages resolve.
   - I re-ran `claude-obsidian lint --strict --as-of 2026-10-02` on the final tree, writing to a scratch file I then deleted, and confirmed the tree was unmodified. It found 20 findings: 11 dead links, 7 duplicate basenames and 2 stale index entries. All 20 targets are identical to the baseline. The only differences are line shifts (54→55, 66→67) caused by the added `SRC-0119` frontmatter line, so there are no new findings. The archived `candidate-lint.json` predates R2; my re-run covers the final bytes.
9. **Open gates and claim scope.**
   - Generic compiler correspondence, source-owner/native-payer, authenticated funding/deployment, general contract/intent/transition/history/PCD and Preview financial settlement are stated as open in the note, session, README, index, log, hot, CSV, ledger and collection.
   - Every location says no public transaction was submitted and no native allocation was made.
   - The source is S3 experiment data, and no canonical financial claim is promoted.
   - Non-self-contained replay is stated: the nested 1130-input closure, ELF, PK, SRS and caches are hash-bound and external.
   - The strict checker is described as an integrity utility only.
10. **Privacy.**
    - A key, token and PEM pattern scan of all 197 files found one false positive: `disk-stop0d15…` in the R4 resource handoff.
    - No thought, reasoning or encrypted-content fields are present. Provider stdout files carry `provider_thought_omitted`, and the Opus stdout carries only a `thinkingTokens` count.
    - No environment dumps or credential assignments appear.
    - Signing keys in the source are fixed public development constants, such as `[0x43;32]`.
    - Sol receipts contain host-transcript path strings, which are metadata only. I did not open them.

## Blockers

None for this publication scope.

## Qualifications (not blocking)

- **Q1: reviewer-session continuity behind the word "fresh".** Two of the three actual-result reviews ran in sessions that were already in use. The candidate does not disclose this:
  - **Grok:** the actual-result receipt `sessionId 01a0f98f-899c-7472-9416-a43ef2607997` is the same session used for the R4 proof review (`native-financial-v10-r4-grok-stdout.json`) and the R5 verify source/resource review. The unpublished root `native-financial-v10-r5-actual-grok-stderr.log` reads "Session … found locally", so the actual-result review resumed that session.
  - **Sol:** the actual receipt names agent `/root/native_v10_r4_sol_audit` and shows seven turn contexts, starting at 2026-10-01T22:21:47Z. That is the same thread as the R4 Sol review, its CPU clarification and the R5 verify review.
  - **Opus:** session `44c332e3…` is distinct from all earlier sessions.

  All three reviewers are separate from the source author (Grok author session `01a0f981…`) and from root. The reviews are new, full actual-result reviews of the exact freeze, which satisfies the develop skill's "fresh audit, not reused source approval" wording. However, "fresh" here means a new review invocation, not new context for Sol and Grok. Two places use the word without that distinction:
  - `R4-R5-REVIEWED-SUCCESS.md:9` attributes it to the root acceptance.
  - `wiki/sessions/native-financial-kernel-2026-10-01.md:61` states "Three fresh actual-result audits" in wiki voice.

  The immutable root acceptance (`accepted_after_all_three_fresh_actual_result_reviews`) is the accepted authority. The publication repeats it rather than upgrading it, and the receipts that reveal the continuity are published byte-identically. I therefore do not block. I recommend that the supplementary receipt, or the next wiki touch, record this distinction. Whether new-context reruns are required is a root/user governance question outside this publication audit.
- **Q2: actual-stage terminal outputs are not copied.** The R5 actual-stage prompts and the Grok/Opus stdout/stderr exist in the research root but are not copied. The receipts bind the stdout hashes: Grok `f7919685…`, which I confirmed, and Opus `d99fc2a4…`, which I confirmed. The required scope (reviews, receipts and acceptance) is met, but earlier stages do publish their prompts and stdout.
- **Q3: stale `updated_at` in index and log.** `wiki/index.md` and `wiki/log.md` keep `updated_at: 2026-09-30T21:34:04Z` while `updated:` is now 2026-10-02. This pattern predates the candidate.
- **Q4: initial drafts live only in `/tmp`.** The initial transaction's `content_file`s are in `/tmp/moriarty-r4-r5-wiki-drafts/`, which is not durable. Their hashes are bound by the inspect and apply records and by the R2 expected hashes.
- **Q5: unmapped SRC-0117.** The portable ledger `legacy_source_ids` has no SRC-0117 mapping. This predates the candidate and is outside its scope.
- **Q6: checker robustness.** The checker uses `assert`, so it is a no-op under `python3 -O`. Its manifest and acceptance hash anchors are self-contained within the candidate. I anchored them externally to the stated `ad507ee8…` and `32305c38…` values and to the live originals.
- **Q7: sampled resource figures.** Resource figures are sampled and are not cgroup hard bounds. R4 prove peaked at 321.87 s sampled CPU and 2.9 GB RSS, within its 1200 s and 4 GiB limits. The note correctly reports only R4 wall time and the R5 samples.
- **Q8: reviewer model identity.** The Grok actual report says the session showed no `modelUsage`, while the root receipt records `grok-4.7-build`. Sol's identity rests on the host `turn_context` attestation. Both points are as recorded by root.

Report author: claude-opus-5-5 (high). This report is review output for the user; it is not an authored commit.
