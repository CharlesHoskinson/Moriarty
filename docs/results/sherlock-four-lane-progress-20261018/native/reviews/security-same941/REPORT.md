# Independent Designated Security Source, Two-Test Operation & Post-P4 Compatibility Review

## 1. Reviewer Identity, Scope & Coverage

- **Reviewer Role**: Independent **NON-AUTHOR** Designated Security Reviewer (`designated_security_owner`).
- **Conversation ID**: `941a2502-3f36-4ff7-9133-e56905b30b14`
- **Routing Identity**: `primary_model = claude-opus-5-5`, `requested_model = gemini-4-argon-high`, `returned_identity = gemini-4-argon-high`, `effort = high`, `fallback_status = standing_opus_quota_outage_fallback`.
- **Candidate Author Exclusion**: Candidate author `4e8031df-879c-41c4-95d2-622cd6b5df22` (`BODY5 /candidate_author`) is excluded from voting.
- **Constraint Compliance & Coverage**: Zero tools, file reads/writes, commands, imports, tests, or SDK/native/filesystem actions were performed. All 7 inline bodies (`BODY1`–`BODY7`) were read completely (`cached_current_build.py` `70,179` B, `test_cached_current_build.py` `35,520` B, `ALL579 HISTORICAL GENERATED MAP` `20,514` B, `EXACT ORDINARY TWO OP LOSSLESS FACTOR` `3,889` B, `ACTUAL DATA CLAIMS AND FUTURE SCOPE` `4,922` B, `ACTUAL P4 CPU WALL COUNTERS` `1,753` B, `GUARD SOLE HELPER SEAL PROOF` `931` B), alongside the retained full `bounded_sdk_compile.py` context.

---

## 2. Executive Summary of Independent Verdicts

| Question | Review Target | Independent Verdict |
| :--- | :--- | :--- |
| **Q1** | Exact 3-file candidate (`BODY4 /both_roots_exact_same_files`, `BODY7 /newguard`): `bounded_sdk_compile.py` (`5b009608772b408b0c88b0c585586f2a5fdae1acf042a1b4155540178d98113b`, `92,418` B), `cached_current_build.py` (`3c4a6d858c0fb404ac2be5f7cb51aea80cd0599caaedce13d1c3539be351ffa7`, `70,179` B), `test_cached_current_build.py` (`8240c319b006dfe2e85577dd8ec10754feae213c5ab3817c5dce2b98ee23eb0d`, `35,520` B) | **`APPROVED_SOURCE_WITH_LIMITS`** (Explicitly qualifying the author's 579-row static-fallback prose claim below) |
| **Q2** | Bounded SDK-free two-changed-methods operation `TWO-CHANGED-SDKFREE-OPERATION.proposed.json` (`b4f921a3d137e7562c64ab4936c96a496e23c486e5dc18c1d5e41dc128777d3a`, `5,823` B, `BODY4 /actual_operation_pin`) | **`ELIGIBLE_ORDINARY_SDKFREE_TWO_CHANGED_METHODS_OPERATION`** (Retains `INTERFACE_INFORMED_NOT_BLIND_R2_UNSATISFIED`) |
| **Q3** | Post-P4 cached-current-f source/config compatibility (`BODY5 /future_argv`, `BODY5 /future_config_operand`) after 2-test PASS, fresh `Q-OBS`, refreshed `old_home_stat_manifest`, `config_inputs` projection, and 5-receipt DAG | **`COMPATIBLE_SOURCE_PENDING_TWO_TEST_PASS_AND_FRESH_POST_P4_PROVISIONING_CONFIG_DAG`** |

---

## 3. Q1 — Substantive Source & Resource Assessment (`APPROVED_SOURCE_WITH_LIMITS`)

### 3.1 Guard Helper-Pin Literal Update (`BODY7`)

- `bounded_sdk_compile.py` (`92,418` B, `sha256=5b009608772b408b0c88b0c585586f2a5fdae1acf042a1b4155540178d98113b`) differs from `45c8cabb758bac36751b045dc8c1837efcaf16aee00ce59b4f4086f17025951e` solely at the single `CACHED_CURRENT_BUILD_SHA256` assignment (`BODY7 /exact_new_statement`: `CACHED_CURRENT_BUILD_SHA256 = '3c4a6d858c0fb404ac2be5f7cb51aea80cd0599caaedce13d1c3539be351ffa7'`, `/unique: true`, `/all_guard_function_bodies_and_otherbytes_unchanged: true`). `BASE = 'f935febac3f9158c3ce02812ac3b3c4adb986714'`, `LOCK = '3fbc2453...'`, and all guard functions are unchanged.

### 3.2 Exact 8 `git-control` + `.global-cache` Dated Snapshot Transitions & Strict 30,660 Historical `D_FREEZE` Invariant (`BODY1`, `BODY3`, `BODY5`)

1. **Strict `30,660` / `30,669` Historical `D_FREEZE.json` Invariant (`reconstitute_source_home`, `BODY1`)**:
   - After `guard.verify_raw(c)` and `guard.freeze_source(c)` write `C_RAW_IDENTITIES.json` and `D_FREEZE.json`, `reconstitute_source_home` enforces exact key-set equality (`set(new_d_files.keys()) == set(old_d_files.keys())`) and restricts hash transitions strictly to `allowed_dated_rels = {GLOBAL_CACHE_REL} | set(git_control_by_rel.keys())` (`1 + 8 = 9` entries out of `30,669`):
     ```python
     allowed_dated_rels = {GLOBAL_CACHE_REL} | set(git_control_by_rel.keys())
     for path_key, old_val in old_d_files.items():
         if path_key not in allowed_dated_rels:
             guard.need(new_d_files[path_key] == old_val, 'source_home entry differs from preserved r3 D_FREEZE: ' + path_key)
     guard.need(new_d_files == expected_new_d_files, 'reconstituted D_FREEZE file map differs from expected dated bookkeeping map')
     ```
   - All `30,090` authenticated source entries and the remaining `570` `expected_generated_files` (`30,660` entries total, `BODY5 /owner_generated_class_disposition/strict_historical_generated_entries`) must match preserved R3 `D_FREEZE.json` (`f8408c7a...`) byte-for-byte.
2. **Snapshot Path, Hash, Byte, Mode-`0444` & Payload Format Bounds (`_verify_eight_git_control_snapshots`, `_validate_git_control_snapshot_payload`, `BODY1`)**:
   - `_verify_eight_git_control_snapshots` verifies `EIGHT-SNAPSHOT-OWNER-PROJECTION.json` (`sha256=a36fd57b...`, `6,069` B) and `EIGHT-CURRENT-SNAPSHOT-CANDIDATES.proposed.json` (`sha256=bedc415c...`, `10,917` B), enforces exact coverage of the 8 allowlisted paths in `EIGHT_GIT_CONTROL_SNAPSHOT_SPEC` (`{04c9c5d,a01a1ea}/.git/{config,index,logs/HEAD,logs/refs/heads/master}`), checks `row.get('historical_sha256') == spec['historical_sha256'] == old_d_files.get(rel)`, and verifies on disk that each snapshot is a regular file with `stat.S_IMODE(snap_p.lstat().st_mode) == 0o444`, exact `spec['bytes']` (summing to `281,004` B), and exact `spec['sha256'] != spec['historical_sha256']` (re-checked immediately before `shutil.copyfile(snap_p, dst)` in `reconstitute_source_home`).
   - `_validate_git_control_snapshot_payload(rel, snap_p, guard)` validates content safety:
     - `/.git/config`: UTF-8 without NUL bytes, `[core]` mandatory, sections and keys restricted to `ALLOWED_GIT_CONFIG_SECTION_KEYS` (`core`: `repositoryformatversion`, `filemode`, `bare`, `logallrefupdates`, `autocrlf`; `remote "origin"`: `url`, `fetch`; `branch "master"`: `remote`, `merge`), and `remote "origin"` `url` restricted to `/home/charl/.cargo/git/db/...` or `file:///home/charl/.cargo/git/db/...` without `@`, `?`, or `#` (blocking `fsmonitor`, `sshCommand`, `hooksPath`, `include.path`, or remote credentials).
     - `/.git/index`: `len(raw) >= 12`, `raw[:4] == b'DIRC'`, version `in (2, 3, 4)`.
     - `/.git/logs/HEAD` and `/.git/logs/refs/heads/master`: non-empty UTF-8 without NUL bytes, ending with `\n`.
   - **Cargo Phase F Influence**: During offline Phase F (`cargo test --no-run --frozen -p mori-backend --lib -j4` with `source_home` read-only mounted), Cargo reads only the extracted checkout worktree files (authenticated by Git blob SHA-1 and historical `D_FREEZE` SHA-256) and does not invoke Git network/hook operations; `.git/{config,index,logs/*}` exist in `source_home` only because `c['expected_generated_files']` (`579` rows) is verified by `guard.verify_raw(c)` and `guard.freeze_source(c)`.

### 3.3 Assessment of All 579 Materialization Branches & Qualification of Static-Fallback Claim (`BODY1`, `BODY3`, `BODY5`)

- Across all `579` `expected_generated_files` in `BODY3`:
  1. `cargo-lockfile` `.global-cache` (`1` row): copied from `global-cache.immutable.snapshot` (`390ffa70...`, `356,352` B).
  2. `cargo-lockfile` other `3` rows (`.package-cache`, `git/CACHEDIR.TAG`, `registry/CACHEDIR.TAG`): copied from `old_home_root` if matching `expected_old_sha`, or synthesized from `(b'', b'Signature: 8a477f597d28d172789f06886806bc55\n')`, which matches all 3 historical hashes (`e3b0c442...`, `6d9d1d21...`).
  3. `cargo-ok` (`425` rows): copied from `old_home_root` if matching `expected_old_sha`, or synthesized from `(b'', b'{"v":1}', b'{"v":1}\n')`, which matches all 423 registry `.cargo-ok` rows (`c5d3445a...`) and both checkout `.cargo-ok` rows (`e3b0c442...`).
  4. `git-control` 8 snapshots (`8` rows): copied from `EIGHT_GIT_CONTROL_SNAPSHOT_SPEC`.
  5. `git-control` `/.git/objects/` (`130` rows: `116` loose objects + `14` pack/idx/rev files): copied from `old_home_root / rel` if matching `expected_old_sha`, or searched across `(source_home / 'git/db', old_home_root / 'git')` by filename and verified against `expected_old_sha`.
  6. `git-control` remaining `12` static metadata rows (`{04c9c5d,a01a1ea}/.git/{FETCH_HEAD, HEAD, refs/heads/master, description, hooks/README.sample, info/exclude}`):
     - **Mandatory Qualification (`BODY5 /owner_wholeclass_claim_qualified`)**: The static fallback in `reconstitute_source_home` (`static_git_candidates = [b'', b'ref: refs/heads/master\n']` + `(g_commit + '\n').encode('ascii')`) matches only `6` of the `12` rows (`FETCH_HEAD`, `HEAD`, `refs/heads/master` for both checkouts) and does **not** contain fallback literals for the `6` template files (`description`, `hooks/README.sample`, `info/exclude`, hashes `85ab6c16...`, `7319d7f1...`, `fd76c62f...` in `BODY3`). Those 6 files (and loose objects not duplicated under `source_home/git/db`) depend on live `/home/charl/.cargo/git/...` remaining at its historical `D_FREEZE` hash (which held at capture: `563` live exact matches in `BODY5 /all579_inventory`) and fail closed via `guard.need(copied and dst.is_file() and guard.digest(dst) == accepted_sha, ...)` if modified. Author prose claiming full host-mutation independence across all 579 rows is therefore not adopted; the code itself is fail-closed and sound.

### 3.4 Retired `P4` History + Live CPU/Wall Accounting & One-Shot Failure Behavior (`BODY1`, `BODY5`, `BODY6`)

- `HISTORICAL_POST_R3_DEBITS` and `enforce_post_r3_historical_debits` (`BODY1`) verify `POST_CLEANUP_RESULT_PATH` (`e7804b2a...`), `P3_COUNTERS_DISPOSITION_PATH` (`8d6db3b4...`), `P3_PRESERVE_CLEANUP_RESULT_PATH` (`a824faae...`), `P4_COUNTERS_BEFORE_RETIRE_PATH` (`BODY6`, `02246e4a...`, `1,753` B), and `P4_PRESERVE_CLEANUP_RESULT_PATH` (`BODY5 /P4cleanup/record`, `38c797ae...`, `2,632` B, `status == 'P4_PRESERVED_AND_CLEANED'`, `cleanupOwed is False`).
- Retired `P4` usage (`13,735,290` usec envelope and supervisor; `0` usec phases, `BODY6 /retiredP4UsageUsec`) is added **once** to prior logical debits (`359,653,742` / `89,370,060` / `270,283,681` usec) for new cumulative historical debits of:
  - `envelope_usec = 373389032`
  - `supervisor_usec = 103105350` (`ceil = 104` s $\rightarrow$ `remaining_supervisor_cpu_seconds_max = 10696` s under the `10,800` s gross ceiling)
  - `phases_usec = 270283681`
- Wall-clock ceiling is reduced by `p4_refusal_wall_seconds_ceil = 15` (`ceil(14.72161572999903)` in `BODY6 /P4wallSeconds`) from `6526` to `remaining_total_wall_seconds_max = 6511` s, while `remaining_f_wall_seconds_max = 2522` s is unchanged because Cargo Phase F never started (`BODY6 /cargoNeverStarted: true`).
- `cached_current_f` writes `ONE_SHOT_STARTED` before reconstitution, sets `RLIMIT_CPU` to `min(c['supervisor_cpu_seconds'], 10696)`, injects `prior_cpu` into `guard.Group.__init__` with unconditional `finally` restoration, and writes `TERMINAL_FAILURE.json` (`native_outputs_accepted: False`) on any exception.

---

## 4. Q2 — Independent Judgment of Changed Two-Methods Synthetic Operation (`BODY2`, `BODY4`, `BODY5`)

- **Operation Pin (`BODY4 /actual_operation_pin`)**: `TWO-CHANGED-SDKFREE-OPERATION.proposed.json` (`5,823` B, `sha256=b4f921a3d137e7562c64ab4936c96a496e23c486e5dc18c1d5e41dc128777d3a`) over the exact 3 staged/afterimage files (`5b009608...`, `3c4a6d85...`, `8240c319...`).
- **Selected Methods (`expectedTestsRun = 2`)**:
  1. `test_reconstitute_source_home_pure_extraction_and_generated_bookkeeping` (`BODY2`): Exercises 24 `D_FREEZE` entries (`9` authenticated + `15` generated, including `.global-cache` and all 8 `git-control` snapshots across `04c9c5d` and `a01a1ea` while live `host_cargo` copies are drifted), verifies `len(rep['dated_bookkeeping_deltas']) == 9` and `guard.frozen_source(c)`, and verifies `guard.Refusal` on (a) a disallowed `.git/config` key (`fsmonitor = /bin/sh`) and (b) `.global-cache` snapshot mutation (`os.chmod(snap_path, 0o644)` before `self._write(..., 0o444)`).
  2. `test_post_r3_historical_debits_enforced` (`BODY2`): Exercises `enforce_post_r3_historical_debits(c, guard, cleanup_receipt_file=None)` with `total_wall_seconds = 6511`, `supervisor_cpu_seconds = 10696`, `limits.F.wall_seconds = 2522`, verifies `373,389,032` / `103,105,350` / `270,283,681` usec at zero and positive live cgroup usage, and verifies `guard.Refusal` at `total_wall_seconds = 6512` and `supervisor_cpu_seconds = 10697`.
- **Honest Coverage Gaps (`BODY5 /exact_two_test_coverage`)**: The synthetic fixture tests 24 entries (not all 30,669 production entries), passes `cleanup_receipt_file=None` in Test 2, and does not separately unit-test corrupt `.git/index` `DIRC` headers, reflog NUL bytes, non-`0444` mode on the 8 `git-control` snapshots, or missing/duplicate rows in `EIGHT-SNAPSHOT-OWNER-PROJECTION.json` (which are verified by direct source inspection in §3.2).
- **Q2 Verdict**: **`ELIGIBLE_ORDINARY_SDKFREE_TWO_CHANGED_METHODS_OPERATION`** under isolated `cwd`/`HOME`/`TMPDIR`, `30s` wall / `25s` CPU / `1 GiB` AS / `1 MiB` stream caps, single attempt, `newNativeBudget = false`, and label `INTERFACE_INFORMED_NOT_BLIND_R2_UNSATISFIED`.

---

## 5. Q3 — Post-P4 Cached-Only Source/Config Compatibility & Concrete Eligible Predicates (`BODY5`, `BODY6`, `BODY7`)

1. **Source & Wrapper Compatibility (`BODY5 /future_argv`, `BODY5 /future_config_operand`)**:
   - Sealed `bounded_sdk_compile.py` (`5b009608...`) and `cached_current_build.py` (`3c4a6d85...`) are fully compatible with the retained `sudo`/`setpriv`/`env`/`cgroup.procs` `--cached-current-f` wrapper (`BODY5 /future_argv`), `BASE = 'f935febac3f9158c3ce02812ac3b3c4adb986714'`, `source_manifest` (`3d64ce91...`), `subject` (`5f7254ef...`), and the exact 9 guard `/paths` keys (no `evidence_dir` key).
2. **Concrete Predicates Required Before Selecting One Post-P4 `--cached-current-f` Invocation**:
   - **Predicate 1 (Two-Test Execution)**: Execute `TWO-CHANGED-SDKFREE-OPERATION.proposed.json` (`b4f921a3...`) once and record `2/2 PASS` (`INTERFACE_INFORMED_NOT_BLIND_R2_UNSATISFIED`).
   - **Predicate 2 (Fresh Post-P4 Host Provisioning & `Q-OBS`)**: Because `P4` was torn down (`BODY5 /P4cleanup/cleanupOwed: false`; retired `P4` mount IDs `190`/`191`/`240` cannot be reused), Root provisions fresh empty tmpfs quota mounts (`target_tmp`, `source_home`, `packet`) and fresh cgroup v2 `E`/`L`/`P` directories (`populated = 0`, `usage_usec = 0`, `K1`–`K3` true) and records a fresh `Q-OBS` (`OP-0` `MemAvailable >= 23,622,320,128` B and new `mount_id`/`device` values).
   - **Predicate 3 (Refreshed `old_home_stat_manifest`, Post-P4 Config & 5-Receipt DAG)**:
     - Refresh `old_home_stat_manifest` (`3,306` path stats + `1,063` directory memberships) to match current `/home/charl/.cargo` metadata for `bounded_sdk_compile.py:old_home_stats(c)`, while confirming that the 6 non-fallback Git template files (`description`, `hooks/README.sample`, `info/exclude` in both checkouts) and all 13 hardcoded snapshot/receipt pins remain unchanged on disk.
     - Assemble the new config and `config-inputs.json` projection with `/bindings/runner/sha256 = "5b009608772b408b0c88b0c585586f2a5fdae1acf042a1b4155540178d98113b"`, `/total_wall_seconds = 6511`, `/supervisor_cpu_seconds = 10696`, `/limits/F/wall_seconds = 2522`, the new `Q-OBS` `/quota_mounts` `mount_id`/`device` values, and unchanged `/source_manifest` (`3d64ce91...`), `/bindings/subject` (`5f7254ef...`), and 9 bulk fields.
     - Complete the 5-receipt DAG (`security.json` `APPROVED_SOURCE_WITH_LIMITS` + `nonsecurity.json` `APPROVED_NORMAL_SOURCE` bound to `runner_sha256 = 5b009608...` $\rightarrow$ `root.json` `ADMITTED_CONFIGURATION` $\rightarrow$ standing-beta `tier1.json` / `tier2.json` $\rightarrow$ final config SHA-256 in `--config-sha256`).
3. **Retained Limits**: Original blind `R2` remains `UNSATISFIED`, prospective `R2b` (`R157`) and stopped `Amendment 10` remain preserved, and any compiled Phase F output remains `UNACCEPTED` (`native_outputs_accepted: False`) pending independent result audit (`C4` open).

