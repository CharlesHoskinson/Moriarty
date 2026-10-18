# N1 C0-C4/C6 and C5 — independent actual-result review

Date: 2026-10-15

**Reviewer:** `claude-opus-5-5`. This is the model ID reported by the runtime. I can't verify it myself.

**Requested effort:** high. The user requested this level; effort is not observable, so I can't verify it either.

**Role:** fresh, non-author, independent result reviewer.
- I did not write the patch, checker, frozen pack, driver, mutant specs, M12 correction, MUTANTS-FROZEN, the runs, or any root record.
- Every author and reviewer in this chain is the same model ID. The independence here is context independence only. It is not vendor or model diversity.

**Tools:** I used Read and Write only. No shell, grep, glob, listing, interpreter, import, hashing, network, or subagents.

**Trust boundary:**
- Every sha256 below is copied from root or frozen records. I rehashed nothing.
- All executions are root's mechanical observations, not my own runs.

## Verdict

**Recommendation: `ADOPT_SCOPED_STRUCTURAL_UNIT`.** This is a scoped adoption of the exact reviewed N1 schema patch only. No acceptance of any kind is granted.

Answer to the predicate: **yes, within scope.** The C0–C4/C6 and C5 observations together establish the patch's scoped structural UNIT behaviour.
- The patch is `admission.py` sha `784ff2e1…`, inside tree `3dac6ed0…`.
- The C0–C4/C6 observations are immutable.
- All 12 frozen mutants classify under the kill rule that was fixed before any run.
- Every required killer is decisive.
- Every extra FAIL is a frozen informational prediction.
- No control fails except PC-F08 under M2, which was its intended killer.

**Smallest real blockers: none.** Gaps carried forward without blocking: F1 (AMB-5), F2 (OPT-M9-N1), F4 (exception wording depends on the interpreter). See [FINDINGS.json](FINDINGS.json).

## Four separate axes

| Axis | Status |
|---|---|
| Implementation | **Present and minimal.** One file changed: a 7-line whole-sibling guard at `src-patched/mcpcat/admission.py:154-162`. Root hashes show the other 13 files byte-identical to `src-current`. I read the patched file in full. The guard is fail-closed and adds no admit path. |
| Evidence | **Sufficient for scoped structural UNIT.** The independent 282-case oracle shows no regression (C3). The N1 effect shows in the C1→C2 flip and the C6 restoration. C5 shows the oracle is sensitive around the patched tree. |
| Adoption | **`ADOPT_SCOPED_STRUCTURAL_UNIT`** (recommended; root records the decision). No new approval loop is needed. |
| Acceptance | **Not granted.** No crypto, auth, ledger, counterparty, Preprod or target acceptance. Native/SDK authority remains 0. Public developers remain permissionless. |

## Tests and oracles, kept at different grades

1. **Independent oracle:** the frozen 282-case pack (`FREEZE a575bbce`). It was used in C3 and C5.
2. **Mutation evidence:**
   - C5 (M1–M12 on the patched baseline) measures pack sensitivity.
   - C6 (M13-N1, the patched tree minus the guard line) measures how sensitive the N1N2 cohort is to the guard itself.
3. **Same-author or consistency grade.** These support the result but establish nothing on their own:
   - N1 required 5 and N1X optional 10 (same author as the patch).
   - N2 23 (a consistency set, not an independent oracle).
   - AUTH50 plus 7 unit checks (`evidenceLabel "AUTH (same-author expectations)"`).

## N1 actual effects (C0–C4, C6)

Sources: `OBSERVATIONS-C0-C4-C6.json` (e029e0df…), `RESULT-MANIFEST-C0-C4-C6.json` (fdf47628…, 556 paths), and the `ctl/*` records named in READ-COVERAGE.

| Stage | Tree | Observed | My reading |
|---|---|---|---|
| C0 | dry run | 38 cases, no import | Plan coverage only. |
| C1 | `src-current` | 30 PASS / 8 FAIL. N1: 3 FAIL, 2 PASS. N1X: 5 FAIL, 5 PASS. N2: 23 PASS. | The bug is live. Aliased-revocation and ambiguity siblings are ignored, so records are admitted or bundles marked SCPU (`ADMITTED_STRUCTURAL` / `STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED` instead of `REFUSED [ADM_RECORDS_SCHEMA]` / `[BND_ADMISSION_REFUSED]`). On N1X-CLI-BND-REVOKE-ALIAS, exit `0 != 1`. |
| C2 | `src-patched` | 38 PASS | The guard turns all 8 into refusals. N2 is unchanged. |
| C3 | `src-patched` | 282 of 282 PASS (15 batches) | No regression on the independent oracle. Every C3 STDERR is 0 B. |
| C4 | `src-patched` | AUTH 50 PASS; 7 unit checks pass; U-NET-PROBE SKIPPED | Same-author grade. Wrapper wall time unknown. Network isolation rests on `unshare --net` only. |
| C6 build | M13-N1 | exit 0 | `MUTANT.json` shows a one-line change: `bad_siblings = []`. Mutant admission sha `2e343ab1…`. |
| C6 N1N2 | M13-N1 | 30 PASS / 8 FAIL, the same 8 ids as C1 | Per-batch STDOUT sha matches C1 exactly for batches 00–04 (`26027d20`, `846116ca`, `540953ba`, `aa07f1c5`, `a19b3794`). This restores the behaviour; it is not a byte revert of `src-current`. |

C6 qualifications, preserved:
- Root's batch-02 and batch-04 postcheck raised a bookkeeping `KeyError inputsMutatedByCall`. Both happened after the children exited 0 with RESULT saved. There was no rerun and no refit (`POSTCHECK-QUALIFICATION.json` ×2).
- CLI rows carry only post-call `inputFileSha256`. They have no before/after flag.
- Root's checks covered 42 raw CLI records and 404 library mutation flags.

N1X-LIFECYCLE-ALIAS-ADM was already `REFUSED [ADM_VK_AMBIGUOUS]` before the patch. It FAILs only because `ADM_RECORDS_SCHEMA` was missing. I record it as a code-exactness detection.

The dropped C6 original 282-leg is not counted. The N1 DECISION's `notAuthorized` list includes "C6 282-leg". The actual C6 N1N2 leg is kept.

## C5: per-mutant results (all 3384 rows read)

The run itself:
- 180 of 180 children completed with exit 0, and every batch is valid.
- Zero retries, no early stop, no driver failure, no HARNESS_ERROR row.
- My row counts equal root's dispositions for every mutant.

| Mutant | PASS/FAIL | Classification | Required killers (all FAIL, all decisive) | Blast radius (all frozen informational) |
|---|---|---|---|---|
| M1 | 282/0 | **EXPECTED_SURVIVOR_GAP (AMB-5)**, never a pack pass | none | none |
| M2 | 281/1 | KILLED_BY_PREDICATE | PC-F08 (b12) | none |
| M3 | 281/1 | KILLED_BY_PREDICATE | B1F-PRED | none |
| M4 | 279/3 | KILLED_BY_PREDICATE | B1F-KEY, B1F-KEY-ADM | B1F-CP-KEY |
| M5 | 278/4 | KILLED_BY_PREDICATE | B1F-SCOPE-GRANT, B1F-CE1, B1F-CE1-ADM | B1F-CP-UNMET |
| M6 | 281/1 | KILLED_BY_PREDICATE | B1F-RECORD-DISCLOSED-DISPUTE | none |
| M7 (3-edit launder) | 271/11 | KILLED_BY_PREDICATE | B1-CE1, B1-CE1-ADM, B1-CE2, B1-CE3, B1-CE3-BND, B1-CE4, B1-L-DISCLOSED, B1-L-REPORT, B1-L-GRANTSHAPE-V1 | B2-POL-REPORT (predicate); SW-POL-accepted_auditors-list (exception) |
| M8 | 275/7 | KILLED_BY_PREDICATE | B2-CE5, B2-CLI-CE5, B2-MIN, B2-MAX | B2-OUTREF-IDX, B2-MAX-THRESHOLD, B2-VALUE |
| M9 | 264/18 | KILLED_BY_PREDICATE | B2-DENY-ALIAS, B2-CE7, B2-CE7-COPIED | exactly the 15 frozen predictions |
| M10 | 280/2 | KILLED_BY_PREDICATE | B2-CE8-ID-ARABIC, B2-ID-FULLWIDTH | none |
| M11 (2 fields) | 271/11 | KILLED_MIXED | 8 library rows (exception) + B3-CE9-CLI | SW-PLN01 fields-int, binding_constraints-int (exception) |
| M12 (corrected literal) | 276/6 | KILLED_MIXED | B3-CE10, -DICT, -LATE, -BND (exception) + B3-CE10-CLI | SW-PF0-relatedRows-list (exception) |

How decisiveness was checked:

- **Predicate kills.** Every frozen `decisiveMismatch` string appears verbatim in the row's mismatch array.
  - For M7 B1-L-DISCLOSED / B1-L-REPORT and M9 B2-CE7, the observed result also equals the frozen `predictedObserved`.
- **Exception kills.**
  - M11: TypeError with `'int' object is not iterable` (6 rows) or `'bool' object is not iterable` (FIELDS-TRUE, CONS-TRUE). The tracebacks point into `mutants/M11/src/mcpcat/catalogue.py:218/221`.
  - M12: TypeError with `cannot use 'list'|'dict' as a set element (unhashable type: …)`. This is Python 3.14 wording, and it contains the frozen fragment verbatim.
- **CLI kills (M11/M12).** All five checks hold:
  - exit 2
  - `codes == ['CLI_INTERNAL_ERROR']`
  - detail starts with `TypeError: `
  - `stdoutIsJson` true, `stderrHasTraceback` false, stderr empty
  - mismatch array is exactly the four frozen strings

  Sources: result objects `9749c809` and `ebbbdd3f`, and raw CLI objects `7c9aaccb` and `da9faa38`.

Mode honesty:
- M11 and M12 are detected only through exceptions. The library rows crash. The CLI turns the crash into a fail-closed structured refusal.
- That matches the frozen KILLED_MIXED classification. It must not be cited as graceful predicate coverage.
- The SW-POL exception row under M7 is blast radius only. M7's mode is predicate.

Specificity rows held PASS:
- **M7:** B1F-S01, B1F-S12, SW-POL-accepted_auditors-int/dict, PC-L02, PC-L14
- **M10:** B2-ID-NL
- **M11:** SW-PLN01 list/dict
- **M12:** SW-PF0-relatedRows int/dict

Non-failing delta: under M9, B2-SCRIPT still PASSes, but with a different result object (`f7a883e0` instead of `849e105a`). See F5.

## Gaps carried forward (non-blocking)

- **F1 AMB-5:** nothing in the pack exercises dispute-grant predicate scoping. This is an over-refusal direction.
- **F2 OPT-M9-N1:** the N1 alias path under a weakened HEX64 is not tested.
- **F3:** M11 and M12 are detected only through exceptions.
- **F4:** whether an exception kill counts as decisive depends on the interpreter version.
- **F6:** the guard is whole-log fail-closed. One malformed entry in a consumer-pinned log blocks admission for every record under it. This is intended design.
- **F7:** independence and oracle grades (same model ID; N2 and AUTH are not independent).
- **F8:** CLI input-mutation evidence is post-hash only.
- **F9:**
  - The network probe was skipped, and there was no full-system write audit.
  - `timeout` resolves to uutils coreutils. No C5 child timed out.
- **F10:** INDEX.json prose says "168" raw CLI observations; the actual count is 108 (12 × 9).
- **F11:** there is no `DECISION.json` at the run root. I read the N1 decision at its explicit referenced path.

## Preserved and still open

- Grants stay unsigned.
- U9 stays `STRUCTURAL_UNVERIFIED`.
- Settlement stays `UNSUPPORTED`.
- The bridge's original 42/52 gap remains.
- All 20 pattern families remain.
- ACTUS, DeFi and general obligations remain open.
- Recursion, private composition and multi-parent composition remain open.
- U-M-MC-SP-G exits remain open, and so does Preprod.
- AMB-5, OPT-M9-N1 and HD1 are excluded or open.
- Budget counters: the original 2/2 pack passes and 4/6 invocations remain consumed and untouched (old counter `cf60fb52`). The C5 counter is separate.

## Coverage

See [READ-COVERAGE.json](READ-COVERAGE.json).

**Read in full (every outcome):**
- All 16 C5 split files, to end of file.
- The N1 observations and the 556-path manifest.
- The N1 DECISION, PRE-LAUNCH-COPIES, and the C4 / C6-build / C6 postcheck records.
- M13 `MUTANT.json` and the patched `admission.py`.
- The freeze and spec records.

**Known by hash or structure only (not opened, not rehashed):**
- The 180 C5 raw batch files.
- The N1N2 and P282 batch files.
- The other 13 source files of each tree.
- The mutant trees M1–M12.
- The frozen harness and oracle inputs.

**Never read:** provider-result*, process.stderr, model PROMPT/PROCESS/FINAL files or wrappers, credentials, keys, caches, or live A1/RR03 drafts.
