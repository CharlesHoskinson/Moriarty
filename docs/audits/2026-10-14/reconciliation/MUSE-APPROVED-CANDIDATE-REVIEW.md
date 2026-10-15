# Muse nonsecurity final-composition review — approved candidate

## Verdict

**APPROVE** (applicable nonsecurity composition only)

## Exact candidate identity

- Candidate audited: `9bd7ccc1b5522b22cf633b46cdad080d24448f39` ("Retain explicit M5 limits in the security requirement addendum"); `git rev-parse HEAD` in this worktree returned exactly this hash and the working tree was clean.
- Baseline: `fc6353bf19b87a76410319c378efe529d52d23d9`. Parent: `f2923e22afe4ae702697e77f47200b90ceffc80a`.
- Source projection (all three current dossier records agree): `15a69be761f4d97312725a1a683df7e8541ddc367acbc9740d097f0340e1a762`.
- Reviewer: requested `muse-wonder-1.0` at high effort, fresh independent nonsecurity auditor; returned identity as observable in this session is Muse Code powered by muse-wonder-1.0. No other returned model identity was observable.
- This is a fresh review of the exact `9bd7ccc` candidate. The previous full Muse APPROVE was scoped to `f2923e22` and is not treated as approval of these changed bytes.
- Security merit, requirements, controls and test design belong solely to Opus 5.5 and are excluded. No security judgment is supplied here. Security-owned documents were inspected for authorship, boundary and hash inventory only.

## Applicable scope

Reviewed read-only: current `AGENTS.md`, the checked-in develop skill (`plugins/moriarty-dev/skills/develop/SKILL.md`, read directly as AGENTS.md directs), the full `f2923e22..9bd7ccc` diff, the affected crosswalk, the current reconciliation requirements context, and the whole current dossier `README.md`, `final-verification.json`, `final-navigation.json` and `integrated-source-manifest.json`. The sole written output is this report. Native authority is zero: no product SDK/VM, k/keygen/prove/verify, ledger, private-key/PK/SRS/proving-cache or provider reasoning was used. No source/runtime change, product acceptance or milestone acceptance is claimed or granted.

## Fresh observations

1. **Delta from f292 is exactly the stated narrow correction — pass.** `git diff f2923e22..9bd7ccc` touches 10 paths only: the Opus-authored OSR-1014-11 text in `security-reconciliation-2026-10-14.md`, mechanical Addenda hashes in `openspec/current-requirement-crosswalk.json`, dossier `README.md`/`final-verification.json`/`final-navigation.json`/`integrated-source-manifest.json`, and three new append-only records (`MUSE-NONSECURITY-FINAL-REVIEW.md`, `OPUS-SECURITY-FINAL-REVIEW-R2.*`, `test-candidate-binding.json`). No plugin, language-product or other nonsecurity source file changed.

2. **M5 restoration is present verbatim as a requirements-composition fact (crypto/security merit not judged).** OSR-1014-11 now states the encoded transaction SHALL be at most 64 KB, every fraud circuit SHALL have k ≤ 19, and each contract SHALL declare and stay within its replay budget, each measured on the claimed target, with measurement not relaxing the thresholds; the oversized-transaction scenario refuses above 64 KB (M5) or a lower measured ceiling, and records that existing 64 KiB JSON/envelope/source bounds do not implement it. This matches the Opus F1 correction text inventoried in `OPUS-SECURITY-FINAL-REVIEW-R2.md`.

3. **Exact index hashes verified by recomputation — pass.** I recomputed SHA-256 for every file in `integrated-source-manifest.json` (83 files, bytes and hashes): zero mismatches. The crosswalk holds 160 canonical requirements; its `separateAddenda` inventories 15 OSR-1014 headings separately, and the addendum file hash recomputes to the recorded `97edda3d…` with the refreshed OSR-1014-11 clause hash. The crosswalk qualification states it is not an atomic acceptance denominator. All three current records agree on projection `15a69be7…`.

4. **All 37 plugin files unchanged and hash-bound — pass.** `git diff f2923e22..HEAD -- plugins/` is empty. Every entry in `test-candidate-binding.json` (37 files) recomputes to its recorded hash at HEAD and at `f2923e22`; the binding names `opus-selected-tests-exit.json` and package commit `f2923e22`, so the Opus-owned 211-test receipt (`exit_code: 0`, "Ran 211 tests … OK") remains bound to identical tested bytes. It is a mechanical binding, not a re-execution; the 20 inspected-and-excluded tests remain exclusions, not passes.

5. **Root checks rerun fresh at 9bd7ccc — all pass.** `openspec validate --all --strict --no-interactive`: 14 passed, 0 failed. `python3 openspec/sprints/verify.py`: pass (160 requirements, 239 original tasks, 24 legacy gates; planning structure only). `test_verify.py`: 11 tests OK. Plugin guidance `test_guidance.py`: 9 tests OK. Guarded `status --json`: exit 0 with historical blockers intact (stale binding, missing accounting, unavailable resource — preserved, not waived). `git diff --check` against baseline: clean. These match `final-verification.json` (`all_pass: true`, `native_actions: 0`) and `final-navigation.json` (686 checked, 0 missing).

6. **No frozen bytes moved — pass.** The three machine registers hash identically at HEAD and baseline. No frozen receipt, requirement task, gate or machine bytes appear in the delta. The append-only Opus author reports retain the original weakened-M5 explanation verbatim (`OPUS-PLUGIN-RECONCILIATION.md:102`, "wrongly called 64 KB a transaction ceiling"); dossier `README.md` (§ "M5 correction and review supersession") explicitly supersedes that claim by the Opus F1 correction while preserving the original report.

7. **Dossier currentness is honest — pass.** The same README section corrects the stale introductory sentence in the frozen Muse f292 receipt ("security coverage continuation remains pending"): the Opus coverage continuation had completed and is retained in the dossier; the outstanding item at that time was the independent Opus verdict. The frozen receipt is preserved verbatim and the correction supplies no security judgment. (This disposition is stated as recorded; no claim is made here about current Opus verdict status.)

8. **Boundaries retained — pass.** Current routing is unchanged in the checked-in skill: `gpt-6.1-sol` high for nonsecurity authoring, fresh `muse-wonder-1.0` high for nonsecurity audit, all security work to Opus 5.5. Native authority remains zero in `final-verification.json` and `test-candidate-binding.json`. Fenrir/target facts remain unknown where unestablished, Rust `moriarty-core/1` stays distinct from experimental TypeScript Core/5, and beta output stays `PreparedUnqualified`. No roadmap exit (U/M/MC/SP/G) closes.

## Limits

This verdict covers only the applicable nonsecurity composition of exact `9bd7ccc`: documentation, requirements indexing, retention/guidance checks, dossier integrity and hash binding. It does not approve security content, plugin runtime merit, installation, hook interception, target qualification, language behavior, or any product/milestone acceptance. The 211-test receipt was hash-bound, not rerun, because the tested 37-file package is byte-identical.

## Acceptance statement

Implementation, evidence and acceptance are reported separately: the narrow M5 requirements correction and its mechanical re-indexing are implemented and evidenced as above; no acceptance gate is closed by this review.
