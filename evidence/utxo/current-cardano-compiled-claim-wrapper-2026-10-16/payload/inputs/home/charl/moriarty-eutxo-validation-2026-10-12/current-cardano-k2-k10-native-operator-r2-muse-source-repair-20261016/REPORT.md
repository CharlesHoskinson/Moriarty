# K10 compiled-wrapper native operator R2 — security source author report

## Verdict

**R2 candidate authored only; not approved, not source-ready, not executable.** The top finding is that R1 (26151 B, `7d591c1e...ada1c4`) contains the three root-caught material defects and the preserved `f7caa24e` audit that said no flaws is contradicted on that scope; R2 fixes exactly those three defects in a new five-file sibling candidate and nothing else. This author is the Muse Wonder 1.0 SECURITY SOURCE AUTHOR under the indexed explicit user Gemini/Muse fallback (`USER-GEMINI-MUSE-FALLBACK-CONFIRMATION-20261016T1227.json`: 'if opus goes down, use gemini and muse', `nativeExecutionAuthorityCreated=false`), not an independent reviewer. No AUTH, M10, RR10 or T10 was created, no materialized run root was created, no attempt was consumed and no native execution occurred in this authorship.

## Exact R1 -> R2 changed-hunks map

Only `runner/operator.py` logic, plus prose/accounting/custody alignment in the other four files, changes. Pins, POLICY `6a4b...59c1`, EXPECTED seven cases, CAPS, strict positive/negative parsers, bwrap argv, isolation and resources are unchanged.

| File | R1 location | R2 hunk |
|---|---|---|
| runner/operator.py | module docstring | Adds the R2 repair paragraph (one mapping, staged custody, returned-before-validation). |
| runner/operator.py | after `INPUT_ROLES` (R1:69) | Adds ONE literal `INPUT_STAGED_REL` dict: `generatedManifest->manifest.json`, `scriptHex->script.hex`, `contextT1..T7->EXPECTED[0..6][1]`; roster derives only from frozen EXPECTED, no new artifact. |
| runner/operator.py | new helper after `check_all_pins` | Adds `check_staged_inputs()`: exact 9-role coverage, pinned size/SHA-256, `lstat` non-link regular file, read-only (`0o222` clear) for every staged input. |
| runner/operator.py | `stage_tool_and_inputs` (R1:236-248) | Deletes `inputs_src` and the eager `inputs_src.get(role, EXPECTED[int(role[-1])-1][1])` line; staging now calls `check_staged_inputs()` before creating fresh dirs. Fixes ValueError `int('t')`/`int('x')` observed in REPRODUCTION.json:8-23. |
| runner/operator.py | `verify_custody` (R1:265-272) | Inserts `check_staged_inputs()` between `check_all_pins()` and staged-tool/operator-five checks, so custody runs at staging/pre-native and post-native including the `created=true` failure path (R1:416-421). |
| runner/operator.py | `main` result init + loop (R1:390-413) | Adds `aikenCallsValidated`/`validatedCallsCount`; appends raw step and sets `aikenCallsReturned` immediately after `run_call` returns and before `validate_call`, stores `validationStatus`/`validationError` on the raw step, re-raises on failed validation, and completes only on 7 returned + 7 validated. Wait/cleanup failure without return stays `incompleteCall` + attempt counter with no phantom returned call; `nativeExecutableStarted` stays `not_measured`; pre-`created` stage failure still emits only stdout terminal JSON. |
| AUTH-CANDIDATE.json | `operatorSha256` (R1:70) | Replaced root-filled R1 digest with placeholder `ROOT_TO_FILL_AFTER_R2_MATERIALIZATION_BEFORE_AUDIT`; status `CANDIDATE_NOT_AUTHORIZED` and historical PENDING attestation unchanged so the candidate still fail-closes. All pins/caps/cases unchanged. |
| EXECUTION-PROPOSAL.json | acceptance/custody/generatedInputs/prerequisiteOrder + new `r2Repair` | States 7 returned + 7 validated acceptance, FAILED_VALIDATION raw preservation, whole staged custody, closed input prerequisite (00e128 + 6a86c215) with candidate attestation still historical PENDING, and the three-repair summary. |
| README.md | new R1->R2 section + custody/authority prose | Documents the three root observations, the optional ordinary pure-Python regression description (no mirror test source added), whole staged custody, 600+10 root outer bound (not 700), closed prerequisite and placeholder/AUTH order. |
| DESIGN-POSITIONS.json | C2, C4, C9, C10 adjusted; C13 added | Keeps the 12 R1 clauses, adjusts custody/accounting/authority wording where fixed, and adds FOR clause K10W-C13-r2-three-repairs-only. No fresh AGREE votes are fabricated. |

Complete R2 bodies for all five files are in `files[]` with the same relative paths as R1. Root must materialize R2 as a new sibling read-only (AST/source only), fill `operatorSha256` before audit only, and obtain a fresh FULL non-author audit on all changed bytes/effects/consequences before any exact new AUTH or first-ever 7 C2 eval.

## Findings (author-observed R1 evidence)

1. **Blocker — eager `dict.get` default (R1 operator.py:242).** `REPRODUCTION.json` (ordinary Python dict.get semantics only, operator not imported) observes ValueError `int('t')` for generatedManifest and `int('x')` for scriptHex and return for contextT1, so staging never completes. R2 removes the default entirely.
2. **Material — staged custody gap (R1 operator.py:265-272).** `verify_custody` checked 11 host pins, staged tool and operator five but never the 9 staged inputs, contradicting the proposal's whole-custody claim. R2 claim now equals code at staging and post-native including failure after staging.
3. **Material — returned step dropped (R1 operator.py:407-410).** Validation threw before append/count, so RESULT reported 0 returned despite a completed process (plain logs only, per f7caa FINDINGS F5). R2 preserves every raw returned receipt; distinct returned/validated counts make partial failure honest, which is critical because no signed proof exists in this scope.

## Scope, limits and open obligations

Read-only mirror review only; no shell/code/Python/Aiken/SDK/curve/key/RNG/proof/wallet/network/ledger execution and no nested agents were used by this author. All compiled wrappers remain UNEXECUTED and no mathematical correctness proof is claimed. Fabricated 2222/555, fee 0, expiry 2030 unbound, Class-0 forgeable VK matrix unproved. K8/K9 passed prefix and 147/472 are never rerun. All 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod remain open; no percentage of full compatibility. First actual milestone after the blocked boundary is the first-ever 7 compiled C2 wrapper calls plus a fresh actual audit, then separately FX1 real auth context, claim consume/settle, qualified cost and Preprod.

## First blocked boundary and independence

First blocked boundary: root R2 materialization (new sibling, read-only, placeholder filled before audit only) -> fresh full non-author R2 audit -> exact new AUTH attesting the already-closed input audit (00e128 + root 6a86c215) -> only then the seven evaluations once. Actual independence: this is fallback source authorship in a new context; it is not the R1 author approval, not the f7caa auditor, not an independent AGREE, and it grants no execution permission. Routine source repair needed no missing approval; source authorship is not execution permission.

