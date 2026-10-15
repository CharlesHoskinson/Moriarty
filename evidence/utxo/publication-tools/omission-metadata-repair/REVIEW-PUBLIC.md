# Nonauthor security review: unknown-source-digest omission metadata repair

**Verdict: `SCOPED_NO_DEFECT`.** This covers only the publication-helper source, the adoption tool and the targeted results below. It does not approve the corpus, the source language or anything native. It does not say the archive is ready: the actual adoption and the main `--verify-all` on the real generation have not been run.

- Requested reviewer: `claude-opus-5-5`, high effort, fresh nonauthor. Returned identity: Claude Opus 5.5 (`claude-opus-5-5`), self-reported. I cannot verify effort myself; confirm it from the provider result record.
- I did not author or edit the candidate. All my writes are in this directory.
- Inputs: all 50 files match `INPUT-PINS.json` (`99c54b46…f3b4`) at the start and at the end, with nothing extra or missing. All 49 scratch copies still match after every run.
- Prompt SHA-256: `4b378526…1f1c`.

## 1. Exact candidate

| File | Frozen SHA-256 | Candidate SHA-256 |
|---|---|---|
| `filter.py` | `b07b9cbed39e371e453061162fab4cd603f350a51b231f81a8dfacb371502c33` | `7d4c0c95d0c4cf9d8699483a1cff3044c9a28998b6aa98781478753284008b98` (+2 lines) |
| `reconstruct.py` | `2cebfd45fefc43aa7fa7d9cd0dd6ac6bce9579b2f40376679662f3ab6b71139a` | `168185e82e23d5737c6c7005529a2f8147b3b8db8a25b8299cfd4a5eca413b04` (+4 lines) |
| `adopt_omission_metadata.py` | new | `2090dde9853354082eb5fc890d6ca27ce2267dce81b5e338049c259050dd6c5f` |
| `test_omission_metadata.py` | new | `41ee8e866e231da229a4ed9128ea3bcec321ae46297feb425c39eb634a5d0a82` |

- I recomputed the diff myself. It is byte-equal to the author's `evidence/helpers.diff`: 6 added lines, 0 removed.
- These files are byte-identical to frozen:
  - `publication_policy.py` (`070aa6f8…baaf`)
  - `finalize.py`, `provenance_gate.py`, `build-publication.py` and `select_restoration.py`
  - all four existing tests
- All 25 `SHA256SUMS` entries match.

## 2. Source findings

**Runtime closure.** After finalization, `reconstruct.py` is the only reader of `public-log-extractions.jsonl` that I found:
- `finalize.py` reads it only to count rows and payloads for `SUMMARY.json`. It never rewrites it.
- `build-publication.py`, `select_restoration.py` and `provenance_gate.py` do not consume it.

**Verifier (`reconstruct.py:48-51`).** The new branch fires only when the row's `sha256` key is present and JSON `null`. It runs after the unchanged duplicate-key and path checks, and before the unchanged digest check. It accepts a row only if all of the following hold, and then `continue`s without adding the row to `extracts`:
- `status == "source-uninventoried"`
- a non-empty string `reason`
- `public_sha256` present and `null`
- no `public_bytes` key
- the key is not a published original path

Reachability comes only from `paths` and `extracts` (`verify_all`, line 89). So an unknown row can never reference an object, satisfy the orphan check, or be served by `--sha256` or `--root/--path`. My probes confirm this (section 3). Every other row still goes through the unchanged `invalid extraction source digest` check.

**Filter (`filter.py:39-40`).** An eligible exclusion row with no recorded digest (key absent or `null`) is emitted as an explicit unknown-digest omission. This happens before `is_file()` or `inspect()`, so the source is never opened. The two checks that run before it (`raw_seen` and `filename_reason`) work on the path string only.

In frozen code, a digestless row could only ever become `source-unavailable` or `source-changed-since-recorded-digest` with no payload. A computed digest never equals `None`. So the published content is unchanged: no payload is gained or lost, and only the status and the explicit `null` digest differ.

**Adoption tool.** Before it writes anything, it checks the following:
- the exact gate SHA;
- the attestation against the gate's input pin;
- that the gate's final manifest set is exactly four files;
- all four manifests against the gate's final pins;
- that no receipt exists and no symlink is anywhere on the paths;
- the reconstructor's bytes against its pin;
- that every digestless row is a payload-free legacy omission, newline-terminated, with exactly one exclusion origin that has no digest and the same reason;
- that the row count equals `--expect-rows`.

It installs exactly the bytes it hashed. The receipt keeps the old receipts and the new pins separate:
- It names the gate and attestation it chains from.
- It sets `original_attestations_retained_unchanged=true` and `original_attestations_cover_adopted_manifest=false`.
- It records the manifest SHA before and after, the line count and the exact before and after text of each changed line.
- It records the unchanged manifest pins and the reconstructor SHA before and after.
- It sets `publication_exception`, `source_approval` and `native_approval` to false.

It does not rewrite or relabel the attestation, the gate or `SUMMARY.json`. Frozen `finalize.py` then refuses the adopted generation (`stale filter manifest attestation`), which is the intended result.

## 3. Commands and actual results

Setup for every run:
- Python 3.14.4 with `-B` and no bytecode.
- `HOME` and `TMPDIR` in scratch, `GIT_CONFIG_GLOBAL=/dev/null` and `GIT_CONFIG_NOSYSTEM=1`.
- Every `UTXO_PUBLICATION_*` default pointed at nonexistent scratch paths.
- A logging git guard first on `PATH`. It allows only calls with `-C` inside my scratch directory and refuses network subcommands.

The runner is `scratch/tools/run_logged.py`. Logs are in `scratch/logs/`.

| Log | Command (cwd) | Exit | Result |
|---|---|---|---|
| R01 | `python3 -B test_omission_metadata.py` (scratch `cand`) | 0 | 11/11 pass |
| R02 | same with `OMISSION_HELPERS=scratch/frozen` | 1 | Expected red: 4 failures and 2 errors. The chain fails with the actual `ValueError: invalid extraction source digest`, and the audit hook catches the frozen filter opening the source. The author recorded 5 failures, because their adoption tool was not yet present. In my run it is present, so `test_adoption_refusals_leave_generation_unchanged` passes. |
| R03 ×8 | `python3 -B test_{publication,review_repairs,restoration,extraction_repairs}.py`, candidate and frozen | 0 on all 8 | 5+3+3+1 = 12/12 on each side. This includes the four git-fixture tests the author skipped. `test_restoration.test_provenance_bound_actual_chain_and_poison` runs the changed `filter.py` and `reconstruct.py --verify-all`. |
| R04 | `python3 -B -I tools/probe_review.py` | 0 | My independent probes, below |
| R05 | `python3 -B -I tools/classify.py` | 0 | Classification under the current policy (section 5) |
| R06 | `python3 -B -I tools/final_integrity.py` | 0 | Pins, copies, git and leftovers (below) |

**First R03 attempt (preserved in `logs/attempt1-guard-root-too-narrow/`).** My guard first allowed only `scratch/tmp`. Four tests put their git fixtures under the test directory, so `git init` was refused (exit 97) and those four errored. I widened the guard root to my scratch directory and reran. This was a harness problem, not a candidate result.

**Git (R06).** 102 calls, all to scratch fixtures:
- subcommands: `init`, `remote`, `add`, `commit`, `rev-parse`, `config`, `cat-file`;
- remotes: `example.invalid` only;
- no network subcommands;
- no `.git`, `__pycache__` or temp leftovers.

**Verifier probes (R04).**
- **Orphan object**, where the only "reference" is an unknown row: `orphan object`. This holds when the row names the digest in extra fields, has a duplicate JSON key with `null` last, or carries `status_before_metadata_adoption`. `--sha256` hash mode exits 1 with `orphan object` and writes no output.
- **Path mode** on the uninventoried path: `original path absent or excluded`, no output. The published-path control succeeds.
- **`public_sha256` set to `false`, `""` or `0`; `public_bytes: null`; wrong-case status; list reason**: all give `invalid unknown-digest extraction omission`.
- **Actual row shape (no key, legacy status)**: still `invalid extraction source digest`. With an explicit `null` and a legacy status: `invalid unknown-digest extraction omission`.

**Filter differential (R04).** Frozen and candidate `filter.py` plus `finalize.py` ran on one synthetic corpus with ten exclusions, covering:
- absent and `null` digests;
- `.jsonl`, provider-reason and reasoning-reason rows;
- a missing source and a filename-class source;
- matching, changed and missing hashed rows.

Results:
- `paths`, `exclusions`, `objects` and `SUMMARY.json` are byte-identical between the two.
- Every row that carries a digest is identical.
- The five digestless rows differ only in `status` and the `sha256` key.
- The candidate ran under an audit hook guarding all six digestless sources and passed.
- The frozen filter under the same hook fails with `PermissionError: guarded uninventoried source opened`.
- The candidate generation verifies.
- No synthetic marker appears in either archive.

**Adoption probes (R04).** Every negative exits 1 and leaves the generation snapshot unchanged:
- `--expect-rows` set to n−1 or n+1;
- an extra key in the gate's manifest set;
- a duplicate or missing exclusion origin;
- a legacy row without a trailing newline;
- a blank manifest line;
- a dangling receipt symlink;
- a symlinked archive `reconstruct.py`;
- an archive reached through a symlink;
- a reconstructor SHA mismatch;
- a generation produced by the candidate filter, whose rows are already explicit;
- a second adoption.

The positive case (5 legacy rows, run under an audit hook guarding both roots) succeeded:
- Exactly three files change.
- The line count is unchanged, and only the receipt's listed lines differ.
- The receipt alone restores the gate-pinned manifest bytes.
- The receipt's unchanged-manifest pins equal the gate.
- The attestation, the gate and `SUMMARY.json` are unchanged.
- The installed reconstructor verifies.
- No marker appears in the receipt.

**Interrupted adoption (R04, failure injected into `replace`).**

| Interrupted | State left | `--verify-all` | Rerun |
|---|---|---|---|
| Before the manifest replace | Only the receipt exists | Fails closed with the legacy message | Refuses (`already adopted`), state unchanged |
| Between the manifest and reconstructor replaces | Receipt and adopted manifest; old reconstructor | Fails closed | Refuses (`manifest differs from finalization gate`), state unchanged |

No temp files were left in either case.

**Actual prediction (R04, from the supplied row metadata only).**
- I re-serialized the supplied row exactly as `filter.py` writes it. The before line hashes to `5b9e9a2a…1bb6`.
- I placed it at line 6387 of a synthetic 6391-line manifest with a matching gate, plus an exclusion origin taken from the row's own fields.
- The frozen reconstructor fails with the actual message. Adoption changes exactly line 6387.
- The after line hashes to `f705baaa…4377`, +75 bytes. Both equal the author's prediction.
- The installed candidate reconstructor verifies.
- The retained traceback's line texts (48, 114 and 129) match frozen `reconstruct.py` (`2cebfd45…139a`). This is consistent with the receipt's expected "before" reconstructor.

## 4. Observations (non-blocking)

- **O1.** The verifier does not require an unknown row to have an exclusion origin. It also compares against published paths without normalizing them, so `./public.txt` or an empty path is accepted. This is the same path handling as the existing hashed rows. It affects labels only, never payload or reachability. The adoption tool enforces a unique origin for the actual row.
- **O2.** Adoption is atomic per file, not one transaction across three files. Interruptions fail closed, as shown above. Recovery is manual, using the receipt's before lines. The parent directory is not fsynced. Read the author's "atomically replaces" as per file.
- **O3.** The tool records the archive's prior `reconstruct.py` SHA but does not pin it. Operator step 1 checks it, and the traceback match in section 3 supports `2cebfd45…`.
- **O4.** Rows with a non-null but malformed digest are left alone by adoption and still refused by `--verify-all`, so they fail closed.
- **O5 (historical, repository observation).** The status `source-changed-since-recorded-digest` is reachable only after frozen `filter.py:42` calls `inspect()`. So the actual C3 run opened and hashed the private source in-process. Under the frozen code, the result was used only for the comparison and was not persisted. The manifest row holds metadata only. The receipt's `source_bodies_read=false` describes the adoption run only. The candidate filter prevents this from recurring; the audit-hook control above demonstrates it.
- **O6.** The adopted row's `bytes`, `path` and `reason` come from the existing C3 exclusion row. The repair derives nothing new.
- **O7.** Downstream pins must be reissued for the adopted `public-log-extractions.jsonl`, the new `reconstruct.py` and the receipt. Old pins describe the pre-adoption generation.

## 5. Publication classification (exact SHAs in `RESULT.json`; no exception granted)

The current policy (`070aa6f8…`) `inspect` returns `reason=None, provider_capture=False` for every file below:

| File group | Classification |
|---|---|
| Candidate `filter.py`, `reconstruct.py`, `adopt_omission_metadata.py`, `SHA256SUMS`, `evidence/helpers.diff`, `AUTHOR-REPORT.md` | Direct public source or text, metadata only |
| Author `evidence/*.py` and logs `01`–`06` | Safe to publish directly: synthetic tracebacks, test names, refusal messages and metadata. No synthetic marker or envelope text appears in any log. |
| `test_omission_metadata.py` | Safe to publish directly, with no exception. The synthetic private marker and the thinking-envelope dict exist only as Python literals. They are written to temporary fixtures at runtime and never persisted; no leftovers remain. If a later policy withholds this file, accept the withholding. Do not create an exception for it. |
| My `REVIEW-PUBLIC.md`, `RESULT.json`, `scratch/tools/*` and `scratch/logs/*.log` | Direct public: synthetic or metadata only, `reason=None` and `provider_capture=False`. Each was classified (see `RESULT.json`). |
| My `scratch/logs/git-invocations.jsonl` and `scratch/logs/attempt1-guard-root-too-narrow/git-invocations.jsonl` | Withhold the bodies, under the existing policy. They hold only git argv metadata, but the policy marks them `provider_capture=True`. Keep their identity and SHA. No exception is granted. The git summary in R06 is the public record. |
| Unchanged `test_publication.py` (`e66dea85…`) | Withhold, under its existing classification. The current policy marks it `provider_capture=True` because of its pre-existing synthetic provider poison. It is not changed here, and no exception is granted. |
| `process.stderr.private`, `provider-result.private.json` | Not public |

No private provider bytes were read, hashed or copied by me, by the author's runs, by the adoption tool or by this review. Every body used is synthetic.

## 6. Limits

- I ran no corpus, real roots, actual generation, archive or prototype worktree. I did no replay and no network, dependency, SDK, native, key, proof, sign, query, fund, submit, auth or agent work.
- The actual adoption and the main `--verify-all` are unobserved.
- The single-bad-row claim comes from the supplied metadata. At runtime, `--expect-rows 1` enforces it.
- Python 3.14.4 only.
- The original C3 receipts, the failure record and the attestation and gate are preserved untouched.
