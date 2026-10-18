# Author report: explicit unknown-digest omission metadata (C3 reconstruction repair)

**Status: source-author result only, awaiting a fresh nonauthor Opus review.** This does not approve the corpus, the source language or anything native. I did not touch the actual corpus. The adoption commands in section 6 are for the operator to run after review.

- Author: Claude Opus 5.5 (`claude-opus-5-5`), security source author, fresh context.
- Frozen inputs: `/home/charl/utxo-publication-build/omission-metadata-frozen-inputs`. All 22 files match `INPUT-PINS.json` (`79c726b6…a344849`) after all runs, with nothing extra or missing (`evidence/06-integrity.log`).
- Writes: only this directory.
- Not done: no network, dependencies, git, agents, SDK, native, key, proof, sign, query, fund or submit work.
- Corpus and source access: I made no corpus or archive access and replayed nothing. I never read, hashed or copied the private source `validation/publication-archive-review-426/review-private-events.jsonl`. Every fixture is synthetic.

## 1. Actual failure (preserved, not re-run)

`MAIN-C3-RECONSTRUCTION.json` and `verify-all-C3.log` record that `reconstruct.py --verify-all` exited 1 at `load()`, line 48, with `ValueError: invalid extraction source digest`.

`ACTUAL-C3-INVALID-EXTRACTION-DIGEST.json` gives the only bad row. It is line 6387 of `public-log-extractions.jsonl` (`5255527b…2f8d`):

- It has **no `sha256` key**.
- `status` is `source-changed-since-recorded-digest`.
- `public_sha256` is `null`.
- `reason` is "new path after capture inventory; publication deferred to explicit frozen public supplement".

## 2. Root cause (repository observation)

Two helpers disagree about one kind of exclusion: a row that was never inventoried and has no recorded digest.

1. **`filter.py:35-42` (frozen).** A `.jsonl` exclusion row without a digest still goes through `inspect(source)`. That call opens and hashes the live private source. The digest is then compared with `r.get('sha256')`, which is `None`, so the comparison can never match. The row is therefore labelled `source-changed-since-recorded-digest`, even though no digest was ever recorded. It is emitted without a `sha256` key. If the source were missing, the same row would be emitted as `source-unavailable`, also without a digest.
2. **`reconstruct.py:48` (frozen).** Every extraction row must carry a 64-hex source digest. There is no representation for "source digest unknown, nothing published".

So the status in the actual row is semantically false, the unknown digest is only implicit (an absent key), and the verifier has no strict rule for such a row.

Observation: in the actual C3 run the frozen filter opened and hashed the private source locally. The result was used only for that comparison and was not persisted, and nothing from the body was published. The row holds metadata only.

## 3. Change (smallest found)

`evidence/helpers.diff` is the complete diff. Only two helpers change. `publication_policy.py` (`070aa6f8…baaf`) and every other helper and existing test are byte-identical.

| File | Frozen SHA-256 | Candidate SHA-256 | Change |
|---|---|---|---|
| `reconstruct.py` | `2cebfd45fefc43aa7fa7d9cd0dd6ac6bce9579b2f40376679662f3ab6b71139a` | `168185e82e23d5737c6c7005529a2f8147b3b8db8a25b8299cfd4a5eca413b04` | +4 lines |
| `filter.py` | `b07b9cbed39e371e453061162fab4cd603f350a51b231f81a8dfacb371502c33` | `7d4c0c95d0c4cf9d8699483a1cff3044c9a28998b6aa98781478753284008b98` | +2 lines |
| `adopt_omission_metadata.py` | none | `2090dde9853354082eb5fc890d6ca27ce2267dce81b5e338049c259050dd6c5f` | new; metadata-only adoption of the completed corpus |
| `test_omission_metadata.py` | none | `41ee8e866e231da229a4ed9128ea3bcec321ae46297feb425c39eb634a5d0a82` | new focused tests |

**Verify invariant (`reconstruct.py`).** An extraction row may carry an unknown digest only if every one of these holds:

- it has an explicit `"sha256": null` (an absent key is still refused);
- its `status` is exactly `source-uninventoried`;
- it has a nonempty string `reason`, which is the omission origin;
- `public_sha256` is present and `null`;
- there is no `public_bytes` key;
- its `(root, path)` is not also a published original path.

A row like that is never added to `extracts`. It therefore cannot reference an object, cannot satisfy reachability, and cannot hide an orphan.

The following checks run before or after the new branch, unchanged:

- path, traversal and absolute-path checks;
- duplicate and conflicting-path checks;
- the `invalid extraction source digest` check for every other row;
- object, range, blob inventory, orphan and byte-count checks.

**Generator semantics (`filter.py`).** An eligible exclusion row with no recorded digest is now emitted as `{**row, "sha256": null, "status": "source-uninventoried", "public_sha256": null}`, and its source is **never opened**. Rows that have a digest follow exactly the frozen path. This makes "uninventoried" a distinct status from "source changed since its recorded digest", which needs a digest to have been recorded. The source is not classified, and no publication exception is created.

## 4. Commands and results

Every run used Python 3.14.4 with `-B`/`PYTHONDONTWRITEBYTECODE=1`. Logs record the command, the combined stdout and stderr, and the exit code. They were written by `evidence/run_logged.py`.

| Log | Command | Exit | Result |
|---|---|---|---|
| `01-red-frozen-helpers.log` | `OMISSION_HELPERS=<frozen> python3 -B test_omission_metadata.py` | 1 | **RED, as expected.** 11 tests: 5 failures, 2 errors. The late and unavailable uninventoried analogs both fail with the actual `ValueError: invalid extraction source digest` at frozen `reconstruct.py` line 48. The audit hook catches the frozen filter opening the uninventoried source. The explicit-unknown acceptance fails, and the adoption tool does not yet exist. The 4 negative-invariant tests pass on frozen, as they should. |
| `02-green-candidate-helpers.log` | `python3 -B test_omission_metadata.py` | 0 | 11/11 pass |
| `03-regression-candidate.log` | `python3 -B -m unittest -v` with 8 existing tests (listed in the log) | 0 | 8/8 pass, including `test_actual_filter_finalizer_chain_426`, `test_manifest_and_atomic_output_rejections` and `test_actual_alias_skill_gate_tamper_and_orphan` |
| `04-refusal-reasons.log` | `python3 -B evidence/probe_refusals.py` | 0 | Every negative is refused with its intended message (table below) |
| `05-predicted-actual-adoption.log` | `python3 -B -I evidence/predict_actual_adoption.py <frozen>/ACTUAL-C3-INVALID-EXTRACTION-DIGEST.json` | 0 | Exact predicted line-6387 transformation (section 6) |
| `06-integrity.log` | `python3 -B -I evidence/integrity.py <frozen> <candidate> evidence/helpers.diff` | 0 | Frozen pins all match. 2 helpers changed. No `__pycache__` or temp leftovers. |

**What the focused tests cover** (`test_omission_metadata.py`):

- **Actual chain** (`UninventoriedChain`). The real `filter.py` → `finalize.py` → copied `reconstruct.py --verify-all` runs on synthetic roots for both the late analog and the source-unavailable analog. The result is `verified 2 unique objects and 1 original paths`. The omission row equals the expected explicit-unknown shape, and the original exclusion row is unchanged.
  - The hashed extraction (`capture.jsonl` → `PUBLIC EXTRACT` object) and the hashed path object both still verify.
  - The synthetic private marker does not appear in any decoded object or archive file.
- **Never opened.** The filter runs under an audit hook that raises on any `open()` of the uninventoried source.
- **Unrelated rows are byte-identical.** On the same corpus, the frozen and candidate filters plus finalize produce identical `paths`, `exclusions`, `objects`, `SUMMARY.json`, blob bytes and every extraction row other than the omission.
- **Verify invariants** (`VerifyInvariants`; refusal messages from log 04):

| Case | Result |
|---|---|
| Explicit unknown digest with a hashed extraction or path present | accepted |
| Unknown digest that also has `public_sha256` (an existing object, or `0`×64), `public_bytes` set, or the `public_sha256` key missing | `invalid unknown-digest extraction omission` |
| The actual row shape: no key, status `source-changed` | `invalid extraction source digest` (the actual failure is preserved) |
| Implicit (absent) digest | `invalid extraction source digest` |
| Null digest with status `source-unavailable`, status `source-changed` or no status; empty, absent or non-string reason; path also published | `invalid unknown-digest extraction omission` |
| Invalid digests: `''`, 63 or 65 hex, uppercase, non-hex, `123`, `False`, list | `invalid extraction source digest` |
| Duplicate unknown rows, or unknown plus hashed on the same path | `duplicate public extraction source` |
| Traversal (`../`, `a/../../`), absolute path, unknown root | `invalid extraction source path` |
| Hashed extraction with a missing object; byte-count mismatch | unchanged refusals |
| Orphan object, trailing blob bytes, range beyond blob, orphan blob file, object unreachable when only an unknown row remains | unchanged refusals |

- **Adoption** (`Adoption`, run against frozen-generated corpora, so the input is exactly the actual row shape):
  - The frozen reconstructor fails with the actual message. The adoption run is audit-guarded against any `open()` under either root.
  - Exactly three files change: `public-log-extractions.jsonl` (one line), `reconstruct.py` and the new receipt.
  - `FILTER-ATTESTATION.json`, `FINALIZATION-GATE.json`, `SUMMARY.json`, `paths`, `exclusions`, `objects` and blobs are byte-identical.
  - The receipt alone restores the exact gate-pinned original manifest bytes.
  - The installed reconstructor verifies.
  - Frozen `finalize.py` now refuses the generation, because the original attestation does not cover the adopted bytes.
  - A second adoption is refused.
  - Refusals, each leaving the generation byte-identical:
    - wrong gate pin;
    - row count other than expected;
    - unpinned or symlinked reconstructor;
    - manifest or attestation not matching the gate;
    - exclusion origin that has a recorded digest or a different reason;
    - row with a public payload or a non-legacy status;
    - row already explicit.

**Not run, by instruction (no git):** four existing tests invoke the `git` binary on temporary fixtures. They are `test_publication.test_fresh_generation_reproducibility_and_omissions`, `test_review_repairs.test_actual_selector_anchor_path_and_sidecars`, `test_restoration.test_provenance_bound_actual_chain_and_poison` and `test_restoration.test_actual_git_blob_remote_and_manifest_mismatches`. They exercise `build-publication.py`, `select_restoration.py` and `provenance_gate.py`, which are byte-identical here, plus the full chain on profile-restored sources, where every row has a digest. The reviewer may run them under their own authority.

## 5. Does the completed corpus need a reissued receipt? Yes, a new one, and the originals stay.

The correction changes the bytes of `public-log-extractions.jsonl` (currently `5255527b…2f8d`). That SHA is pinned by `FILTER-ATTESTATION.json` (`3dadfea1…9ab6`) and by both the `input_manifest_sha256` and `final_manifest_sha256` of `FINALIZATION-GATE.json` (`d9607d64…48da78`). Neither the filter nor the finalizer produced the adopted bytes. So:

- **Do not rehash or rewrite** the attestation or the gate. They remain true statements about the pre-adoption manifest, and the actual failure record (`MAIN-C3-RECONSTRUCTION.json`, exit 1) stays as it is.
- **Do issue a new receipt.** `OMISSION-METADATA-ADOPTION.json` is written into the generation. It records:
  - the gate and attestation SHAs it chains from, and `policy_sha256`;
  - the manifest SHA before and after, and the line count;
  - each changed line with its exact before and after text;
  - the SHAs of the three unchanged manifests;
  - the adoption tool SHA, and the reconstructor SHA before and after;
  - `original_attestations_retained_unchanged=true`, `original_attestations_cover_adopted_manifest=false`;
  - `source_bodies_read=false`, `publication_exception=false`, `source_approval=false`, `native_approval=false`.
- **Downstream pins must be reissued.** Any later publication SHA manifest or review `INPUT-PINS` must pin the adopted `public-log-extractions.jsonl`, the new `reconstruct.py` and the receipt. Old pins for those two files describe the pre-adoption generation.
- **Expected after adoption:** `finalize.py` refuses to run on the adopted generation (stale attestation). That is correct; do not re-finalize to "fix" it. `SUMMARY.json` stays valid, because the record count and the public-extraction count do not change.
- **Future generations:** the candidate `filter.py` emits `source-uninventoried` directly, without the `status_before_metadata_adoption` key, and its own attestation pins `7d4c0c95…8b98`. The adopted C3 row records its legacy status explicitly instead of pretending the new filter produced it.

## 6. Actual-generation adoption commands (operator, after review; not run by me)

Expected adoption of line 6387 (`evidence/05-predicted-actual-adoption.log`, derived only from the supplied row metadata):

- before: `{"bytes": 551565, "path": "publication-archive-review-426/review-private-events.jsonl", "public_sha256": null, "reason": "new path after capture inventory; publication deferred to explicit frozen public supplement", "root": "validation", "status": "source-changed-since-recorded-digest"}`
  (line SHA including `\n`: `5b9e9a2a23e5436bff937ec46baa3f3209114a38e58b94a151f1566075951bb6`)
- after: `{"bytes": 551565, "path": "publication-archive-review-426/review-private-events.jsonl", "public_sha256": null, "reason": "new path after capture inventory; publication deferred to explicit frozen public supplement", "root": "validation", "sha256": null, "status": "source-uninventoried", "status_before_metadata_adoption": "source-changed-since-recorded-digest"}`
  (line SHA: `f705baaabe1f535e746e4a32b1e32567e8f3f3ec74d26f6c0df115f1f4ac4377`, +75 bytes)
- The manifest SHA after adoption can only be computed from the actual file. The receipt records it.

```bash
H=/home/charl/utxo-publication-build/final-generation
C=/home/charl/utxo-publication-build/prospective-omission-metadata-repair
# 1. Read-only preconditions: reviewed candidate bytes and the exact finalized generation.
(cd "$C" && sha256sum -c SHA256SUMS)
sha256sum "$H/FINALIZATION-GATE.json" "$H/FILTER-ATTESTATION.json" "$H/public-log-extractions.jsonl" "$H/reconstruct.py"
#    expect d9607d64…48da78, 3dadfea1…9ab6, 5255527b…2f8d; reconstruct.py expected 2cebfd45…139a (receipt records it either way)
# 2. Optional: keep a copy of the original manifest outside the archive (the receipt alone also restores it exactly).
cp --preserve=all "$H/public-log-extractions.jsonl" <evidence-dir>/public-log-extractions.pre-omission-adoption.jsonl
# 3. Metadata-only adoption. Reads manifests, the gate and the attestation only; never opens a research/validation root.
python3 -I -B "$C/adopt_omission_metadata.py" --archive "$H" \
  --gate-sha256 d9607d642621daafaa51dba5f1b874d4fb8013365fd29d500f84360448b4da78 --expect-rows 1 \
  --reconstruct "$C/reconstruct.py" --reconstruct-sha256 168185e82e23d5737c6c7005529a2f8147b3b8db8a25b8299cfd4a5eca413b04
# 4. Check the receipt: changed_lines == [line 6387] with the before/after text above; unchanged_manifest_sha256 equals the gate pins.
# 5. The single main reconstruction check (decodes archive objects; not a filter or content replay):
python3 -B "$H/reconstruct.py" --verify-all
# 6. Record a new reconstruction receipt and reissue downstream SHA pins (section 5).
```

How the tool behaves:

- It refuses unless all of these hold:
  - the gate SHA is pinned;
  - every manifest equals the gate's final pins;
  - the attestation equals the gate's input pin;
  - no receipt exists yet;
  - the reconstructor matches its pin;
  - every digestless row is a payload-free legacy omission whose unique exclusion origin has no recorded digest and the same reason;
  - the count equals `--expect-rows`.
- It writes the receipt first, with `open('x')`, then atomically replaces the manifest and the reconstructor, then re-checks both against the receipt. If it is interrupted, the receipt's after-pins visibly disagree with the files, and a rerun refuses.

## 7. Limits and open points

- I ran everything on Python 3.14.4 only. No actual corpus or actual source was accessed. The actual adoption and the main `--verify-all` remain unobserved.
- The prediction assumes the actual line 6387 is exactly `json.dumps(row, sort_keys=True)`, which is how `filter.py` writes rows. The tool does not depend on that assumption: it rewrites whatever that row's line actually is and records the exact before text.
- The tool also accepts a legacy `source-unavailable` digestless row of the same origin. `--expect-rows 1` pins the actual count.
- Whether this deferred source is ever published belongs to the separate explicit frozen public supplement. This repair grants no exception and changes no content policy.
