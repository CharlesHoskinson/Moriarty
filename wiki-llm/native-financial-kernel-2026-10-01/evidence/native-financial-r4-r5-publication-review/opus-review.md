# R4/R5 publication candidate: independent Opus audit

## Verdict

**REQUEST_CHANGES.** There are two narrow metadata/navigation blockers. The evidence archive itself, its byte-identity, hash bindings, scope qualifications, failure/dissent preservation and privacy all pass. No evidence body needs to change. After correcting the two items below, I expect APPROVE_SCOPED, subject to a rehash of the changed files.

- Reviewer: claude-opus-5-5, requested effort high, per candidate `AUDIT-ROUTING-20261001.json` (sha256 `a415f7d6bdcf2381f2eb18253c2e96096b4abe99d4c104bb76db5e02fa5df9b4`). This is a fresh, non-delegated session. I am not the author, and I did not read the counterpart publication reports.
- Candidate: `PUBLICATION-FREEZE.json` sha256 **`0c597a0a286271257a54ea1f047a0b9b82fd365fec6a8a9d4f2294fae209ec79`** (40199 bytes, verified). Base `1d0d598cd63af591be0b2a9f6a5cdeb6768b93bb`, which matches worktree HEAD.
- Startup: I loaded the checked-in `moriarty-dev:develop` skill and AGENTS.md. Guarded `status --json` was run read-only and reported no pending transactions. I made no candidate edits, ran no native execution/build/proof and made no network requests.

## Blockers

**B1. SRC-0119 confidence is mislabelled "Failure" in both canonical source records.**
- `evidence/source-inventory.csv:117` and `wiki/meta/ledgers/source-ledger.json:127414` (legacy record for SRC-0119) both read: `"confidence": "high for retained identities/Failure; broader product gates open"`.
- This text is copied verbatim from SRC-0118 (`source-ledger.json:127374`, CSV line 116), which recorded an actual native application Failure. SRC-0119 records R4 financial **Success** and complete R5 verification; its own scope column says so.
- The two files are in sync, but both are wrong. The canonical source record for this intake therefore misstates the classification of the result it inventories.
- Fix: correct the wording in both files together, through the inspected transaction for the ledger. For example: "high for recorded identities and fixed-fixture Success/verification; retained historical failures preserved; broader product gates open". `content_sha256` and `legacy_hash_matches_observed` are unaffected, because they hash `collection.json`, not the ledger text.

**B2. The current-direction hot cache still states the superseded status as current.**
- `wiki/hot.md:27` (unchanged) still says: "Retained-genesis native diagnosis and supply-consistent funding are next; independent verification, full financial Success, authenticated custody/history and Preview acceptance remain open."
- `wiki/hot.md:29` (new) then reports reviewed R4 financial Success and complete R5 independent verification. The update does not mark line 27 as historical or superseded.
- `hot.md` is the agent recovery/"current direction" page, and as written it contradicts itself on exactly the two predicates this publication closes. The dossier README was correctly relabelled ("Historical R3 application refusal"); the hot cache needs the same treatment.
- Fix: mark line 27 as the historical 2026-10-01 R3 state, or rewrite it so that only the still-open gates are listed as open. Then rehash `hot.md` in the wiki transaction.

## Verification performed

**Freeze and candidate coverage**
- All 197 freeze entries exist and hash-match.
- `git status --porcelain -uall` lists exactly those 197 paths: no unfrozen changes, and no frozen entry is unchanged.

**Archive byte-identity**
- All 186 `archive-manifest.json` entries match their recorded size and sha256.
- I compared each repository copy byte-for-byte with its `original_path` under the research root and the pinned cargo checkout: 186/186 identical.
- The directory holds 188 files: the 186 manifest entries plus `archive-manifest.json` and `check-archive.py`.

**Frozen sets**
- The R5 actual freeze has 165 entries. The R4 actual-result freeze (88) and R4 proof-result freeze (67) are subsets of it. All 165 are present in the manifest, and all 165 originals still match on disk.
- The 21 manifest entries outside the 165 are the R5 actual-result freeze, the root acceptance, three actual reviews, three terminal receipts, and the 13-file native consumer source/patch set. That matches the "final reports outside original165 explicitly added" requirement.

**Anchors**
- Actual-result freeze `32305c38…3fc26` and root acceptance `ad507ee8…be6` match the manifest, `collection.json`, the acceptance file and the brief.
- `collection.json` pins manifest sha `d074d0f8…1930`, which matches.

**Checker**
- `python3 wiki-llm/native-financial-kernel-2026-10-01/evidence/native-financial-r4-r5/check-archive.py` exited 0 and printed `files_checked 186, actual 165, proof 22, verification 30, three reviews, invariants pass, native_execution false`.
- I read its source. It is hash/JSON/log parsing only: no subprocess, no archived-code execution.
- Its invariants match the real data:
  - prove and verify receipts: exit 0, no stop or error, postconditions passed, exact output sets of 22 and 30;
  - receipt: Success, proof_count 1, default Real;
  - `0 < fee ≤ allowance ≤ available` and `fee + remainder = available`;
  - R4 receipt byte-equals the R5 independent receipt;
  - eight control records with logs hash-checked, six exit 1 and two exit 0;
  - first and final pristine logs are byte-equal, with 1035 ordered PI refusals;
  - the partial R4 verify stays exit 1 with postconditions false;
  - source hashes match.

**Claimed effects, positive fee and replay, checked against the native source**
- `src/main.rs:272-301`, inside `accept()`, writes `application-result`/`poststate` and then refuses unless all of the following hold:
  - native `Success`;
  - escrow `== 8990` with exact expected storage (`:285`);
  - funding input consumed;
  - exact A1 outputs {1000, 10} and the same-owner NIGHT `night_value` (1e12, from the config fixture) (`:288-290`);
  - a single DUST event with `0 < consumed ≤ allowance`;
  - balance non-negative;
  - replay returns `Failure` with an unchanged `state_hash`.
- `receipt.json` is written only after `accept` returns.
- I checked the actual values directly. `1117490000000001 + 4999998882509999999999 = 5e21`. The allowance is 1e20. The replay file is `ReplayProtectionViolation(IntentAlreadyExists)`. The effects transcript shows claimed spends of 1000/10 and unshielded outputs of 1010. `proof.raw` is 6336 bytes.
- The 293/47 figure is enforced at construction (`main.rs:168`) and repeated as receipt literals. The page and acceptance disclose that R5 did not recount it.

**R5 controls**
- Each control config differs from the good config in exactly one field (`funding_history` or `funding_claim`).
- Log tails:
  - suffix cases: "Not all bytes read";
  - truncation cases: `UnexpectedEof`;
  - canonical alternates: "supervisor-pinned funding predecessor history differs from independent native reconstruction".
- No `*-output` directories exist in the original tree.
- The good-again `poststate`, `application-result`, `replay-refusal` and `receipt` match the R5 primary outputs, which match the R4 proof outputs byte-for-byte.

**Resource figures**
- 287.614 s, 67.036 s, CPU 58.18 s and RSS 716906496 match the receipts. The sampling qualification is present.
- The cleanup increase of 14917431296 matches the acceptance. The cleanup record hash `5a324f21…` matches.

**Reviews and identities**
- The three actual reviews each state APPROVE_SCOPED, and their report and receipt hashes match the acceptance.
- Returned identities:
  - Sol: turn_context gpt-6.1-sol/high;
  - Opus: canonicalModel claude-opus-5-5, firstParty, end_turn;
  - Grok: modelUsage grok-4.7-build, end_turn.

**Dissent and failures preserved**
- The Opus R4 CPU `REFUSE_CURRENT_LIVE_ADMISSION` clarification (`c387e212…`) is retained.
- The R4 root resource decision records the Sol+Grok majority with the Opus dissent "preserved, not claimed satisfied".
- The disk-stop, R3-interrupted and R4 partial-verify receipts are retained as failures.
- I found nothing that converts a historical failure into a success.

**Source binding**
- The 13 native consumer source/patch files hash-match `SOURCE-HASHES.json` and appear in both the R4 and R5 input freezes.
- R5 input freeze: 1130 entries. The ELF `ae7e8dab…` (698019744 bytes) is external and hash-bound, consistent with "not a self-contained native replay".
- No repository product code (crates/compiler) is in the candidate. The wrappers and source are provenance copies under `wiki-llm/…/evidence`.

**Wiki transaction**
- The bundle's `expected_hashes` equal the HEAD blobs of all five paths. The write hashes equal the current files and the `/tmp` drafts.
- I recomputed the canonical bundle hash read-only with the pinned claude-obsidian `bundle_sha256`: `3a6569b9…f0bf6`, which equals inspect `input_bundle_sha256` and apply `bundle_sha256`. Apply status is `complete`.
- Read-only `claude-obsidian lint --as-of 2026-10-02` left git status unchanged. It reported 0 provenance, orphan and frontmatter errors. Its two `wiki/index.md` dead links, at lines 54 and 66, predate the candidate.

**Privacy**
- No credential patterns: the hits were false positives such as "task-".
- No `thinking`, `reasoning` or `encrypted_content` fields. The model stdout files contain only final `result`/`text`, and `provider_thought_omitted: true` is set where present.
- No environment dumps.
- The signing key is the hard-coded public development constant `[0x42;32]` (`main.rs:55`), with a wrong-key control `[0x43;32]`. The fixture trust is `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY` on network `undeployed`.
- No public transaction or native allocation is claimed. The acceptance has `new_native_allocation_authorized: false`.

**S3 and claims**
- The ledger entry is `review_status: unreviewed`, with no claim-ledger changes. The session page states that no canonical financial claim is promoted.

## Nonblocking qualifications

1. `R4-R5-REVIEWED-SUCCESS.md:5`: "The native receipt records escrow 8990, recipient A1 outputs…". `receipt.json` itself records Success, fee/DUST values and state hashes. Escrow, outputs and conservation are enforced postconditions that gate the receipt's emission (`main.rs:285-290`). Suggested wording: "R4's native acceptance path enforced … before emitting the receipt."
2. `README.md:64`: the relabelled historical R3 section still says "The immediate predicate is native diagnosis…". It is acceptable under the Historical heading, but past tense would be clearer.
3. The page does not repeat the `history-control-results.json` qualification that the canonical alternate history "also differs in operation registration; not isolated timestamp-only test". Consider adding it to line 7.
4. `source-ledger.json:4329`: the transaction re-serialised an unrelated SRC-0100–0103 note, turning a literal en dash into `–`. It is semantically identical but an incidental byte change outside SRC-0119.
5. Published provenance includes host-identifying absolute paths: `/home/charl/...`, Trash paths in the cleanup record, and a Codex transcript *path* in the Sol receipt. These are not secrets, and no transcript content is included. They are acceptable as historical identity.
6. Grok's "xhigh" is requested effort only, with no returned-effort attestation. Sol's effort is host-attested. The page's "Grok 4.7 xhigh" reflects the routing request.
7. "Current native Rust source" (`R4-R5-REVIEWED-SUCCESS.md:19`) means the native consumer used for R4/R5, not Moriarty product source. That is clear from context, but could be named explicitly.
8. AGENTS.md still names the September Grok 4.6/Astra routing. The candidate's pinned routing record governs this audit. Reconciling AGENTS.md is outside this candidate's scope.
9. `check-archive.py` does not self-verify `archive-manifest.json`. That binding comes from `collection.json` and the publication freeze, both verified above.
10. The filesystem mode of the session page is 0600. This is not tracked by git and has no effect.

## Scope reviewed

- All 197 candidate paths: full reads of the new page, the README diff, the session, hot, index and log diffs, the CSV row, the ledger diff and `collection.json`.
- In the evidence archive: the archive manifest and checker; the root acceptance, inspection and resource decision; the prove and verify receipts (R4 prove, R4 partial verify, R5 verify); the native `receipt`/`application`/`replay`/effects files; all R5 control configs and logs; the full native consumer `main.rs`/`verify.rs` and the relevant `funding.rs` lines; the routing record; the review receipts and verdict sections; the stdout/stderr structure; and the cleanup record.
- Byte comparison of all 186 archived files against their originals.

Not performed: native execution, rebuild, proof or verification; network requests; reading host conversation transcripts or credentials; recursive review of this report.
