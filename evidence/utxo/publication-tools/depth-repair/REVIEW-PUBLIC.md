# Nonauthor security review: publication-helper depth repair

**Verdict: `SCOPED_NO_DEFECT`.** This covers the helper source and the targeted results only. It does not approve the corpus, the source language, or any native behaviour, and it does not say the archive is ready. That needs the actual full current-policy corpus run and reconstruction, which has not been done.

- Requested reviewer: `claude-opus-5-5`, high effort, fresh nonauthor.
- Returned identity: Claude Opus 5.5 (`claude-opus-5-5`), self-reported. Effort cannot be self-verified; confirm it from the provider result record.
- I did not author this candidate or edit it.
- Candidate policy: `070aa6f8506382724a216d39a9b20aafb4838d6149d91f6463dc48cc2186baaf` (14175 B).
- Baseline C2 policy: `203b7a060fd72111d64cc712573da618a829fcbcbededcfc6980c6316f7e7aee` (13910 B).
- Frozen inputs: 77 files. All 77 match `INPUT-PINS.json` (SHA `1fdbf232…5230`) at the start and at the end. There are no extra or missing files.

## 1. What I checked in the source

I recomputed the diff myself. It matches `evidence/publication_policy.diff`: two hunks, both in `publication_policy.py`. The six other helpers and the four old tests are byte-identical to their prior-C2 originals. `red-baseline/` is byte-identical to prior-C2.

**`inspect` change (lines 96–98).** The new `except RecursionError` sits on the same `try` that wraps `json.load`, `secret_tree` and `provider_records`. It sets `reason = reason or <depth reason>` and `provider = True`.
- The reason is always non-empty after this branch. Every consumer therefore excludes the input without extracting it:
  - `filter.py:30–31`
  - `build-publication.py:55`
  - `select_restoration.py:18–20`, which only selects rows with no reason or exactly the skill-provenance reason
  - `finalize.py:27`
- This branch never consults `public_source_permits`. The skill check at line 113 is skipped because a reason is already set. `parser_only_candidate` stays `False`.
- Reasons set before the `try` are preserved: filename, sensitive value, header, signing key, and the binary/CBOR checks. A `secret_tree` hit that happens before a later `provider_records` overflow is also preserved.
- `RecursionError` is not a `ValueError`. So the inner `except ValueError` handlers in `secret_tree` and `provider_records` cannot swallow it, and the order of the two `except` clauses does not matter.

**`extraction` change (line 140).** `RecursionError` now goes to the outer handler. The result is `unparseable-capture` with an empty body, and the whole capture is withheld. That includes the line-by-line `.jsonl` path, because the per-line handler still catches only `ValueError`.

**Same-class recursion closure.**

| Recursive or deep path | Where | Covered by |
|---|---|---|
| `secret_tree` (Python frames, plus JSON-in-string) | inside the `try` | new `inspect` branch |
| `provider_records` (Python frames, plus JSON-in-string) | inside the `try` | new `inspect` branch |
| `public_typed_schema` | called from `provider_records` | new `inspect` branch |
| C JSON decoder, top-level `inspect` | inside the `try` | new `inspect` branch |
| C JSON decoder, `extraction` (whole file and per line) | `extraction` | outer `except` |
| `strict_text` | top level only | not recursive |

No other helper walks source-derived JSON recursively. The other `json.loads` calls read manifests or profiles that the helpers write or that are pinned. A failure there aborts the run with an error, which publishes nothing.

**Recursion limit.** No helper contains `setrecursionlimit`. The limit stayed at 1000 after every probe.

## 2. Commands and results

These are my own runs: Python 3.14.4, recursion limit 1000, everything inside `scratch/`.
- `TMPDIR` and `HOME` pointed into scratch, with `GIT_CONFIG_GLOBAL=/dev/null` and `GIT_CONFIG_NOSYSTEM=1`.
- All `UTXO_*` defaults pointed at nonexistent scratch paths.
- A logging git guard was first on `PATH`.
- Scratch copies were SHA-verified against the pins before the runs (`evidence/scratch-copy-shas.json`) and again after them.

| Command (cwd) | Exit | Result |
|---|---|---|
| `python3 test_depth_repair.py` (frozen `scratch/cand/red-baseline`) | 1 | Expected red. 7 tests: 2 failures, 13 errors, all `RecursionError` (both "maximum recursion depth" and the 3.14 "Stack overflow … while decoding"). The 2 non-deep tests pass. |
| `python3 test_depth_repair.py` (candidate `scratch/cand`) | 0 | Green, 7/7 |
| `python3 test_extraction_repairs.py` (candidate / frozen) | 0 / 0 | 1/1 on each |
| `python3 test_publication.py` (candidate / frozen) | 0 / 0 | 5/5 on each, full file, including `test_fresh_generation_reproducibility_and_omissions` |
| `python3 test_review_repairs.py` (candidate / frozen) | 0 / 0 | 3/3 on each, full file, including `test_actual_selector_anchor_path_and_sidecars` |
| `python3 test_restoration.py` (candidate / frozen) | 0 / 0 | 3/3 on each, full file, including `test_provenance_bound_actual_chain_and_poison` and `test_actual_git_blob_remote_and_manifest_mismatches` |

**The four tests the author skipped all ran and passed** on both the candidate and the frozen copy.
- Git ran 102 times. Every call targeted a scratch temporary fixture: `init`, `config`, `remote`, `add`, `commit`, `rev-parse`, `cat-file`. None was refused, none used the network, and no fixture `.git` directory is left (`evidence/git-invocations.jsonl`).

**Independent synthetic probes** (`tools/probe_outer.py`, results in `evidence/PROBES.json`). My first attempt exited 1: the harness crashed on a pre-existing `TypeError` (observation O1). I changed the probe to record exception classes instead and reran it; the rerun exited 0.

- **Shallow differential.** 400 seeded random JSON, JSONL, `.md`, `.hs`, `.txt`, `.log` and `.weird` inputs built from the classifier's own vocabulary. `inspect` and `extraction` outputs are identical between frozen and candidate (same digest `2042007b…e116`; 292 of them have a reason).
- **Reason precedence and boundaries.**
  - A secret before an overflow keeps the secret reason.
  - A secret after an overflow gets the depth reason. It is still withheld.
  - Dict nesting at 700 levels overflows only in `provider_records`: it gets the depth reason. The same nesting with a nested CBOR key keeps the secret reason.
  - An embedded envelope in a 4×300 JSON-in-string chain gets the depth reason, with `provider_capture=True`.
  - Invalid JSON nested 200000 levels gets the depth reason, and `extraction` returns `unparseable-capture`.
- **Provenance boundary.** `deep.md` and `deep.hs` are pinned by a valid SHA-pinned public-document profile. The positive control confirms `public_source_permits` returns `True` for both. Both are still withheld with the depth reason, `parser_only_candidate=False` and `provider_capture=True`.
- **Caller-stack sensitivity.** A 60-level public matrix classified from caller depths 0, 300 and 700 gives `reason=None` on both versions. At caller depth 900, the candidate gives the depth reason (fail closed) and the frozen copy crashes. A borderline input is never classified more permissively.
- **Stability after an overflow.** Alternating 20 deep and 20 shallow `inspect` calls gives stable results.
- **Chains.**
  - `filter.py` on the frozen copy exits 1 with `RecursionError`.
  - `filter.py` on the candidate exits 0, and `finalize.py` exits 0. The deep `.json` file is excluded with the depth reason and its exact SHA. Stored objects hold no key or CBOR bytes.
  - `build-publication.py` on the candidate excludes deep inputs with a reason, the exact SHA and the byte count. It performs 0 extractions.
  - `select_restoration.py` gives `deep.md` the disposition `not-parser-only-or-source-changed`.

**Actual failing source, metadata only** (`tools/actual_probe.py`, results in `evidence/ACTUAL-METADATA.json`).
- No body content was emitted or copied.
- The source is SHA `f0a81fff…4cba`, 367487 B, a regular non-symlink file in the `validation` root. Its path is the one recorded in `prior-C2/ACTUAL-C2-RECURSION-FAILURE.json`.
- Its SHA and stat were identical before and after the probe.

| Check | Frozen C2 | Candidate |
|---|---|---|
| `inspect` | `RecursionError` | depth reason, `provider_capture=True`, `parser_only_candidate=False`, `skill_source=False`, exact SHA |
| `extraction` | `empty-or-unclassified`, 1 omitted, 0 B | same |
| `filter.py` with only the recorded exclusion row | exit 1, `RecursionError` (reproduces `filter.py:40`) | exit 0 |

On the candidate:
- The exclusion row is unchanged byte-for-byte.
- The extraction record has the exact SHA, `empty-or-unclassified`, 1 omitted and `public_sha256=null`.
- `finalize.py` exits 0 with 0 paths, 0 objects and the exclusion row unchanged.
- Both the attestation and the finalization gate record `policy_sha256=070aa6f8…`, `source_approval=false` and `native_approval=false`.

The author's depth figure of 5957 and the recorded 5956 differ only in whether the root is counted as level 0 or 1. Either way it is far above the overflow threshold of roughly 490 list levels.

**End-of-review integrity** (`evidence/FINAL-VERIFY.json`):
- 77/77 inputs match their pins.
- All 24 scratch copies are unchanged.
- No fixture `.git` directories and no temporary files are left.
- A read-only check found these originals still at their pinned SHAs:
  - candidate `prospective-depth-repair/publication_policy.py`
  - candidate `test_depth_repair.py`
  - candidate `AUTHOR-REPORT.md`
  - C2 `prospective-restoration-repair/publication_policy.py`

## 3. Findings

**No defect in the candidate within scope.**
- Inputs that used to crash the helper are now withheld, with their exact SHA and an auditable reason or omission.
- Inputs that didn't crash before get identical classification and extraction. I confirmed this with the author's 18 differential cases and my own 400 cases.
- Provider, secret, key, skill and provenance permissions are not broadened.

**Observations.** These do not block this scoped verdict.

- **O1. A different crash class exists in both C2 and the candidate. It is not recursion and is out of scope.** `public_typed_schema` line 26 raises an uncaught `TypeError` when `"type"` is a list that contains a dict or a list, for example `{"type":[{}]}` or `{"type":[[]]}` (`evidence/PREEXISTING-TYPEERROR.json`). It fails loudly: the run aborts and nothing is published, so it is not permissive. The candidate did not introduce it and correctly did not redesign the classifier. However, the next full corpus run could abort on it. It needs its own scoped repair if it occurs.
- **O2.** The original C2 resume run aborted at the first crashing exclusion row. Exclusion rows after it in iteration order have not been re-inspected under any policy. The next full run will be the first observation of them.
- **O3.** Legitimately public JSON nested more than about 490 list levels is now withheld. The exact threshold depends on the caller's stack depth. Such inputs crashed before, so no previous success changes. `SUMMARY.json` will show a new exclusion class.
- **O4.** Existing design, not broadened: `filter.py:35–42` strictly re-extracts `.jsonl` exclusion rows whatever their reason, and re-extracts `provider` rows after an SHA check. Deep inputs get exactly the same strict, regex-scanned top-level-text treatment as shallow secret-reason inputs. My synthetic `.jsonl` control showed equal published text for both. That control was classified by the stream secret scan, not the depth reason, so it demonstrates parity of the existing path rather than isolating the depth branch.
- **O5.** I executed only on Python 3.14.4. On older CPython versions the C decoder counts against the same recursion limit. It still raises `RecursionError`, which is still caught. That is reasoning, not something I ran.

## 4. Exact-SHA artifact classification (narrow; not a general archive policy exception)

| Artifact | SHA-256 | Classification |
|---|---|---|
| Candidate `publication_policy.py` | `070aa6f8506382724a216d39a9b20aafb4838d6149d91f6463dc48cc2186baaf` | Direct public source text. The candidate helper itself classifies it as `reason=None`. |
| `test_depth_repair.py` | `9c66d9f951234d4673e4d10e50f8ded4fe9cfe862120cfa744c64d5153f11d76` | Contains only synthetic analogs: generated `'a'*64` and `'b'*40` values, a literal header string, a signing-key type label, and public helper digests. It has no real key or provider content. The current helper withholds it (`unapproved signing-key content signature`). I grant no exception. Preserve its identity and SHA only, and withhold the body. |
| `AUTHOR-REPORT.md` | `f78934d379fb024415335faa1a41e0a7241ad255be4d6aae73006c030547f997` | Public text with metadata only. It contains no source body. The helper classifies it as `reason=None`. |
| Author `publication_policy.diff` | `f4ce3f1348910488fefff02a8cbb6c16e846d6349ced57b780108a7c45ae2c7d` | Public text |
| Author logs 02–12 and `RESULTS.json` | as pinned in `INPUT-PINS.json` | Safe targeted logs: test names and metadata only |
| Author `01-red-focused-frozen.log` | `b61f5a15c2613e37cf3be33a877238b22611d93267601959a9a70f2645e65879` | Synthetic tracebacks only. Preserve the identity and SHA and withhold the 2.4 MB body. |
| This review's `evidence/` and `tools/` | listed in `RESULT.json` | Synthetic or metadata-only. The stored red-run log is truncated, with the SHA of the full log recorded. |

Nothing here covers raw provider output or stderr. `process.stderr.private` and `provider-result.private.json` in this directory are not classified as public.

## 5. Limits

- I did not run the full archive or corpus, and I did not rerun any real experiment. There was no native, SDK, key, proving, verification, signing, funding, submit, query, deploy or network work, and no agents.
- I read the actual source only through metadata-only probes, and its body was never output.
- The failed original run and the earlier reviews are preserved untouched.
