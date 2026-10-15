# Publication helper depth repair: author report

**Status: author result.** It is waiting for a fresh NONAUTHOR Opus review of the source and the targeted results. No independent approval is claimed. This is a publication-helper candidate only. It approves no source or native behaviour, and it grants no dispatch or native authority.

- Author: Claude Opus 5.5 (`claude-opus-5-5`), high effort, acting as delegated security author.
- Work area: `/home/charl/utxo-publication-build/prospective-depth-repair` (all writes went here).
- Frozen inputs: `/home/charl/utxo-publication-build/depth-repair-frozen-inputs`. All 30 entries were hashed at the start and re-verified at the end by `evidence/finalize_manifest.py`.

## Failure being repaired (repository observation)

The resumed retained-corpus run (`RETAINED-RESUME-RUN.json`, exit 1) used the unchanged C2 policy `203b7a06…7aee`. It finished processing the retained objects (`filter.py` lines 24–34). Then it reached the second loop, which re-inspects exclusion rows already recorded as provider captures. There, `filter.py:40` called `inspect(source,UNIT)` to compare the source's SHA against the recorded digest before strict extraction. `inspect` parsed the JSON and called `secret_tree`. `secret_tree` recurses two Python frames per nested list, so it raised `RecursionError` at Python's default recursion limit of 1000. The exception was uncaught (`inspect` catches only `ValueError`/`UnicodeError`), so the whole run aborted.

The exact source is SHA `f0a81fff…4cba`, 367487 bytes, with a top-level dict. It still sits in the exclusion list under its original reason, `raw provider capture identified from exact archived content; strict extraction only`.

## Decision: an explicit fail-closed disposition, not an iterative rewrite

I made the depth failure an explicit, auditable fail-closed classification inside `inspect`. I also added `RecursionError` to `extraction`'s existing parse-error tuple.

The recursive helpers `secret_tree`, `provider_records`, `public_typed_schema` and `strict_text` are byte-identical to the frozen originals. I chose this over an iterative traversal for these reasons:

1. **Exact preservation.** Any input that did not raise before now passes through identical code and gets an identical result. Only inputs that used to crash the helper behave differently, and they become withheld. An iterative rewrite would have had to reproduce subtle exception routing. One example: in `provider_records`, a `UnicodeEncodeError` (a `ValueError`) from a lone surrogate inside a decoded JSON string gets caught by the *enclosing* string's `except ValueError`. That kind of rewrite risks drift in what gets classified as private.
2. **Conservative meaning.** If a structure can't be fully traversed, it can't be shown free of secrets or provider envelopes. So it is withheld, not published.
3. **The recursion limit is not raised.** The global recursion limit is untouched (the candidate has no `setrecursionlimit`, and a test asserts the limit stays 1000).
4. **One bound covers both depth failures.** Python's JSON decoder in 3.14 has its own stack-based limit (measured here: fine at 20000 levels, `RecursionError` at 100000). The same `except` covers that decoder failure for the top-level `json.load`, for nested `json.loads` inside the helpers, and for `extraction`.

## Exact changes (`evidence/publication_policy.diff`)

Only `publication_policy.py` changed: frozen `203b7a06…7aee` → candidate `070aa6f8506382724a216d39a9b20aafb4838d6149d91f6463dc48cc2186baaf`.

- `inspect`: a new `except RecursionError:` clause on the existing structured-JSON `try` (json.load → secret_tree → provider_records). It sets `reason = reason or 'structured JSON nesting exceeds bounded inspection depth; fail closed'` and `provider = True`.
  - `reason or` keeps any reason already found: filename class, credential header, signing-key signature, a secret found before the overflow, and so on.
  - `parser_only_candidate` stays `False`.
  - Because `reason` is now set, the later skill-source check cannot override it, and `public_source_permits` is never consulted for this path.
- `extraction`: `RecursionError` was added to the outer `except`. A capture too deep to decode now produces `status='unparseable-capture'`, `parse_error='RecursionError'` and an empty body. The whole capture is withheld, including any shallow public lines in the same file.

`filter.py`, `finalize.py`, `provenance_gate.py`, `select_restoration.py`, `build-publication.py`, `reconstruct.py` and all prior tests are byte-identical to the frozen inputs (recorded in `SHA256-MANIFEST.json`). Gate SHA and the UNIT allowlist are unchanged. Profile handling is unchanged.

## Other recursion in the same failure class

| Location | Recursion | Disposition |
|---|---|---|
| `secret_tree` (incl. JSON-in-string) | Python frames | Caught in `inspect` → fail closed |
| `provider_records` (incl. JSON-in-string) | Python frames | Same `try` → fail closed |
| `inspect` top-level `json.load` | C decoder stack | Same `try` → fail closed |
| `extraction` `json.loads` (whole file and per line) | C decoder stack | Outer `except` → `unparseable-capture`, empty body |
| `strict_text`, `public_typed_schema`, stream regex scans | None | Unchanged |

Every caller inherits the bounded disposition through `inspect` and `extraction`:

- `filter.py:29` (retained objects): excluded with the depth reason and the exact SHA.
- `filter.py:40–42` (this failure): SHA compare, then the existing strict extraction. The exclusion row stays unchanged.
- `build-publication.py:48,54`: excluded with the reason.
- `select_restoration.py:17`: not a parser-only or skill candidate.
- `finalize.py:26`: a reason makes it refuse.

## Evidence (exact commands, exits and output are in `evidence/`; summary in `evidence/RESULTS.json`)

All runs used Python 3.14.4 with recursion limit 1000. A logging `git` guard (`guard-bin/git`, exits 97) was first on PATH. `evidence/git-guard.log` is empty: no git ran.

| # | Run | Exit | Result |
|---|---|---|---|
| 01 | RED: `test_depth_repair.py` against an exact copy of the frozen code (`red-baseline/`, hashes verified in `00-setup-baseline.log`) | 1 | 7 tests: 2 failures, 13 errors (`RecursionError` in every deep case, including the filter chain); the 2 non-deep checks pass |
| 02 | GREEN: `test_depth_repair.py` on the candidate | 0 | 7/7 OK |
| 03–06 | Git-free prior regressions on the candidate | 0 | extraction 1/1, publication 4/4 (incl. the filter→finalize chain 426), review 2/2, restoration 1/1 |
| 07–10 | Same prior regressions on the frozen copy | 0 | Same results |
| 11 | Actual failing file, frozen code, metadata only | 3 | `inspect` → `RecursionError` (reproduced) |
| 12 | Actual failing file, candidate, metadata only | 0 | No crash; dispositions below |

What the focused tests cover (`test_depth_repair.py`, all synthetic analogs; no actual source content is used):

- **Deep inputs withheld, not crashed.** Covered cases: deep unknown, secret and provider structures at 6000 levels; JSON-in-string chains (8 string wrappers × 400 levels, with and without a client-secret leaf); and a 200000-level file beyond the decoder's own limit. Each one gets a reason, `provider_capture=True` and its exact SHA and byte count, and the recursion limit stays 1000.
- **Reason precedence is kept for deep inputs.** The original reason stays for: the `.skey` filename class, private-key header, and signing-key signature. A deep embedded envelope still sets `embedded_private_envelope` and is withheld. A deep `.claude/skills` JSON gets the depth reason, not a skill exception.
- **Extraction.** A file too deep to decode gives `unparseable-capture` with an empty body. A JSONL file with one public line and one undecodable line is withheld in full. In a 6000-level mixed array, the strict top-level text is extracted and the deep private-key remainder is omitted: it is not in the body, and the omitted count is 1.
- **Exact excluded-source filter chain** (`filter.py` then `finalize.py` in subprocesses):
  - A deep analog at `formal/k/fixtures/expression-v1.json` keeps its exclusion row byte-for-byte equal. Its extraction record has the exact SHA, `empty-or-unclassified`, no `public_sha256`, and an omitted count of 1.
  - A changed deep source becomes `source-changed-since-recorded-digest`.
  - Deep retained objects are moved to exclusions with the depth or secret reason and their exact SHA.
  - The shallow public extraction still publishes.
  - The attestation still says `source_approval=False` and `native_approval=False`.
  - After finalize, the only stored objects are the public file and the two public extraction bodies. No secret bytes are in any stored object.
- **Differential equivalence.** 18 shallow inputs give identical `inspect` and `extraction` output from the frozen copy and the candidate. They cover prior poison events, schema, graph, embedded graph text, package, PlutusScript, signing key, nested secret, mnemonic, CBOR, a lone-surrogate JSON-in-string, three-level JSON-in-string, a 300-level public matrix, two-level 120-deep public strings, a Haskell source and Grok text. The legitimate public structures are still public (no reason, not a provider capture).
- **Provenance boundary.** Checked in a subprocess using the git-free `public-document-manifest` provenance kind. A shallow skill file still needs exact provenance, and passes only with a SHA-pinned profile. A deep skill file is still withheld, with the depth reason, even when a valid provenance entry pins its exact SHA.
- **Static checks.** No `setrecursionlimit` in the candidate. The six unchanged helpers match their frozen SHAs.

## Actual failing file (metadata only; no body content was copied or emitted)

`evidence/12-actual-candidate-metadata.log`:

- **Identity:** SHA `f0a81fff…4cba` equals the recorded SHA. It is 367487 bytes, a regular non-symlink file, with a top-level dict.
- **Structure:** 226 decoded JSON-string wrappers. My iterative measure of maximum depth is 5957, counting the root as 1; the frozen metadata recorded 5956, so this is a counting-convention difference.
- **`inspect`:** reason `structured JSON nesting exceeds bounded inspection depth; fail closed`, `provider_capture=True`, `parser_only_candidate=False`, `skill_source=False`.
- **`filter.py:40–42` path:** the SHA matches, then strict extraction gives `empty-or-unclassified` with 1 omitted event, no parse error and a 0-byte public body. The exclusion row is retained unchanged. Nothing from the file is published.

## Not run, and limits

- **Tests skipped because they invoke git**, which is out of my scope:
  - `test_publication.test_fresh_generation_reproducibility_and_omissions` (the builder runs `git rev-parse`)
  - `test_restoration.test_provenance_bound_actual_chain_and_poison`
  - `test_restoration.test_actual_git_blob_remote_and_manifest_mismatches`
  - `test_review_repairs.test_actual_selector_anchor_path_and_sidecars`

  The code they exercise beyond `inspect` (`provenance_gate.py`, `select_restoration.py`, `build-publication.py`) is byte-identical. The shallow-input behaviour of `inspect` is covered by the differential test. They still need a run by someone authorized for local fixture git.
- **No full or resumed archive run.** No publication, network, native work or git operations. The policy SHA changed, so any later filter run will attest the new policy SHA and needs its own authorization.
- **Over-withholding is possible.** Legitimately public JSON nested deeper than about 490 lists (or the equivalent across decoded strings) is now withheld instead of classified. That covers files like deep K fixtures. Before this repair, such files crashed the helper, so no previously successful outcome changes.
  - Inference: the frozen run finished the retained-object loop before failing, so no currently retained path should move to exclusions on a rerun at the same call depth.
  - The overflow threshold depends on caller stack depth. A borderline input gets either its original classification or the fail-closed one, never a more permissive one.
- **New reason class.** `finalize.py`'s `SUMMARY.json` counts will show the depth reason as its own exclusion class.
- **Pre-existing behaviour kept as designed.** `filter.py:40–42` uses `inspect` only for the SHA comparison and then always runs strict extraction. That is unchanged, not broadened. The extracted body is still scanned for secrets, signing keys and CBOR keys.

## Files

- **Candidate:** `publication_policy.py` (changed) and `test_depth_repair.py` (new).
- **Byte-identical frozen helpers and tests:** `provenance_gate.py`, `filter.py`, `finalize.py`, `reconstruct.py`, `build-publication.py`, `select_restoration.py`, `test_publication.py`, `test_restoration.py`, `test_review_repairs.py`, `test_extraction_repairs.py`.
- **Evidence only:** `red-baseline/` (exact frozen copies plus the same focused test), `guard-bin/git`, and `evidence/` (scripts, logs, `RESULTS.json`, diff).
- **Manifest:** `SHA256-MANIFEST.json`.
