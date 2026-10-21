# NATIVE09 Adapter Delta Source Review — Independent Security Reviewer A03

- **Reviewer role & identity:** Designated independent security reviewer `A03` in a fresh conversation; not the source08/source09 author; no other source09 reviewer reports were read. Requested model surface: Gemini 4 Argon (High) via AGY under the standing Opus 5.5 outage fallback (`AGENTS.md`); requested surface identity only, with no backend attestation claimed beyond the host interface.
- **Scope:** `SOURCE_QUALITY` delta review only over `NATIVE09-ROOT-SNAPSHOT14` against the 3 mandatory source08 Review-C defects (`N1`, `N2`, `N3`) and receiving producer/consumer APIs. Zero execution authority; no native, guard, helper, operator, test, or subprocess execution was performed beyond one read-only stdlib AST/SHA-256 verification script and the required startup `moriarty_dev` status check.

## Verdict

**APPROVE** (`SOURCE_QUALITY` scope only; conditional on the finite at-use prerequisites and symbol conditions below). Mandatory defects `N1`, `N2`, and `N3` from source08 Review-C are resolved in source09 without introducing new security or lifecycle defects.

## Verified Candidate & Changed-Function Closure

All 8 candidate files in `ROOT-SNAPSHOT-MAP.json` (`23346` B, sha256 `b275cc1e64686c7da4639d41c8f57bd032ed1247d001bd30ef936a00458d4272`) and the mechanical delta extraction in `ROOT-ACTUAL-CHANGE-MAP.json` (`3027` B, sha256 `f4b9a50fa6e8fdcb950f94c7d4c25f9268943fa82a7855f459f1c1afbf10191a`) and `CURRENT-WHOLE-CHANGED-FUNCTIONS.txt` (`61527` B, sha256 `3b90030f6a8dc07171aa8a71444220f42421ee4fbdd4aaff02eaf35ca05d40c4`) were verified with stdlib `ast` and `hashlib`:

- **Changed candidate files (3 of 8; other 5 byte-identical to source08 per `ROOT-SNAPSHOT-MAP.json`):**
  - `models/doctorwatson/run_selected.py` (`134603` B, sha256 `e1aa86ec60a5c6919fcbf866e23437fc059d08bf05a7cb85ab30eed4c9af0e02`)
  - `models/doctorwatson/fixture_support.py` (`37715` B, sha256 `5cdc285ad61a630de7a1139faf203fed632ff367969cf0abca9e91fb4a44b7bb`)
  - `models/doctorwatson/test_native_adapter07.py` (`57995` B, sha256 `a65a0965e48334b33d4bce61384513a8aaf622b3a3a870513aae3a38692739c0`)
- **Changed functions in `run_selected.py` (11 total; top-level globals and all other 56 functions verified unchanged from source08):**
  1. `_guard_constants` (`L391–443`, `2311` B)
  2. `_helper_constants` (`L446–474`, `1168` B, new)
  3. `_config_facts` (`L477–653`, `8895` B)
  4. `_require_real_parent_dir` (`L703–713`, `510` B, new)
  5. `resolve_selection` (`L716–871`, `8806` B)
  6. `_decoder_expected` (`L988–1015`, `1343` B)
  7. `_validate_debit_state` (`L1321–1478`, `8527` B)
  8. `_validate_source_home_reconstitution` (`L1481–1566`, `4624` B)
  9. `_validate_f_launch_result` (`L1578–1645`, `3572` B)
  10. `_require_existing_provisioned_host` (`L2205–2236`, `1655` B)
  11. `run_selected_once` (`L2421–2818`, `19237` B)

## Disposition of Source08 Mandatory Defects (N1, N2, N3)

### N1 — Phase-F process `argv` validation vs actual guard producer (`RESOLVED`)
- **Producer closure:** `cached_current_build.py:cached_current_f` (`L2523`) calls `bounded_sdk_compile.py:launch(c, 'F', config_sha)` (`L1156–1248`). For phase `F` with `retained_audit=False`, `launch` executes three variants `[(True, True), (False, False), (True, False)]` (`L1175`). Each variant builds `args = sandbox_args(...)` (`L879–952`), which starts with `'--die-with-parent'` (`L885`) and places a single `'--'` separator after `--chdir <worktree>` (`L945`). Neither `sandbox_args` nor `cargo_argv('F')` (`[CARGO, *BACKEND]`, `L44`, `L838`) emits any `'--'` token after that separator. In `launch` (`L1186–1190`), both audit workers and the Cargo variant are wrapped in `TRAMPOLINE` via `cmd = ['/usr/bin/python3', '-I', '-B', '-S', '/proc/self/fd/' + str(stage_fd), ...]` and recorded as `args = bwrap_opts + ['--', *cmd]` (`L1190`, `L1238`).
- **Candidate verification:** `_validate_f_launch_result` (`run_selected.py:1609–1636`) requires `1 <= len(argv) <= 4096` nonempty strings, `argv[0] == "--die-with-parent"`, `"--" in argv`, and locates the last separator `sep = len(argv) - 1 - argv[::-1].index("--")`. It enforces `sep > 0`, `sep + 4 < len(argv)` (requiring at least one argument after `-S`), and `argv[sep + 1:sep + 5] == ["/usr/bin/python3", "-I", "-B", "-S"]`, plus at least one `audit_worker is False` entry. `fixture_support.py:guard_launch_process_argv` (`L372–397`) and `guard_launch_processes` (`L400–415`) model the real P9/P10 bwrap-prefixed shapes, and `test_native_adapter07.py` (`L377–395`, `L440–451`) covers positive P9/P10 shapes and negative boundary cases.

### N2 — Guard/helper/config arithmetic, 13/10-key derived disposition binding, and D/E pins (`RESOLVED`)
- **Producer closure:** `bounded_sdk_compile.py` defines top-level literals `BASE`, `LOCK`, `CANDIDATE_DISPOSITION_SHA256`, `EXPECTED_PRIOR_ESP_US`, `EXPECTED_PRIOR_RESIDUAL_US`, `EXPECTED_NATIVE21_DELTAS_ESP_US`, `EXPECTED_RESERVE_AB_US`, `EXPECTED_FLOORS_S`, `POLICY_HOLDBACK_S`, `POLICY_SUPERVISOR_ALLOWANCE_S`, `POLICY_TOTAL_WALL_MAX_S`, `POLICY_F_WALL_MAX_S`, and `CACHED_CURRENT_BUILD_SHA256` (`L46–47`, `L76`, `L85–100`, `L1075`), and computes `resource_disposition_state(c)` (`L434–490`). `cached_current_build.py` defines `PRESERVED_PACKET_PINS` (`L94–101`) containing `'D_FREEZE.json'` and `'E_RESOLVED.json'`.
- **Candidate verification:** `_guard_constants` (`run_selected.py:391–443`) and `_helper_constants` (`L446–474`) extract those exact real symbols from the digest-verified guard and helper files. `_config_facts` (`L477–653`) validates `config.resource_disposition` (`config.final.json:17593–17632`), checks residual and ceiling-division allowance arithmetic (`upper_bound_s - holdback_s == allowance_s`), and builds the exact 13-key (`POLICY_DISPOSITION_KEYS`) or 10-key (`ESTABLISHED_DISPOSITION_KEYS`) `derivedCurrentDisposition` dictionary matching `bounded_sdk_compile.py:resource_disposition_state`. `resolve_selection` (`L811–840`) cross-checks guard constants and floor subtraction (`floors[0] - policyHoldbackS`, `floors[1] - policyHoldbackS`) against `cfg_facts["route"]`, raising `Blocked` if any constant is `None` or mismatched. `_decoder_expected` (`L988–1015`), `_validate_debit_state` (`L1321–1478`), and `_validate_source_home_reconstitution` (`L1481–1566`) require all expected keys to be non-null and bind both start and end `current_disposition` dicts via `disp == derived_disp` (`L1429`, `L1464`) and `preserved_d_freeze_sha256` / `preserved_e_resolved_sha256` (`L1560–1565`). `fixture_support.py:guard_stub` (`L528–543`) and `test_native_adapter07.py` (`L457–504`, `L738–793`) use only real guard/helper constants.

### N3 — `retainedDir` parent precheck and post-dispatch `OSError` fail-closed unknown (`RESOLVED`)
- **Pre-dispatch verification:** `_require_real_parent_dir` (`run_selected.py:703–713`) uses `os.lstat` to require `Path(retainedDir).parent` to be an existing non-symlink directory in `resolve_selection` (`L776–777`) and `_require_existing_provisioned_host` (`L2209–2210`), both at initial resolution and inside `_recheck_at_use` (`L2474`, `L2530`) before any stage runs.
- **Post-dispatch fail-closed handling:** In `run_selected_once` (`L2673–2818`), `retained_dir.mkdir`, `cleanup.json` atomic write, and `_fsync_dir(retained_dir)` are guarded by `try ... except OSError` (`L2675–2687`), setting `cleanup_write_error`, forcing `cleanup_state = restoration_state = "unknown"`, writing `outer-observations.json`, `outer-stages.json`, and `result.json` (`terminal: "unknown-submission"`, `cleanupState: "unknown"`, `consumerResult: None`), and returning `EXIT_UNKNOWN` (`L2718–2728`). Any `OSError` in the subsequent record/collector block (`L2713–2818`) unlinks any partial `consumer-result.json`, attempts `outer-stages.json` if unwritten, removes stale `result.json.tmp`, writes `result.json` with `terminal: "unknown-submission"` and `cleanupState: "unknown"`, and returns `EXIT_UNKNOWN`. Under `delivery.py:attempt_obligations` (`L587–588`) and `reconciliation.py:load_readiness` (`L564–611`), this state yields `(unknown=True, debt=True)` (`unknown-reconcile`), preventing retry until reconciled.

## Finite At-Use Conditions (by Symbol)

1. **`C1_EXACT_SNAPSHOT14_INTEGRATION`:** Root integrates the exact candidate bytes from `ROOT-SNAPSHOT-MAP.json` after independent source09 review closure; any byte modification invalidates this review.
2. **`C2_SEVEN_PIN_TASK_IDENTITY`:** `delivery.Task.input_files` and `selection` (`SELECTION_SCHEMA`) bind the exact 7 file pins in `ROOT-SNAPSHOT-MAP.json` (`bounded_sdk_compile.py`, `cached_current_build.py`, `operator_provisioner.py`, `operator-request.json`, `run_tail_four.py`, `config.final.json`, and `/usr/bin/time`).
3. **`C3_PROVISIONED_HOST_NO_OP0_OP6_REPLAY`:** `PROVISION_STEPS` remains `(("prelaunch-verify", False),)` (`FORBIDDEN_PROVISION_SUBCOMMANDS` never staged); `_require_existing_provisioned_host` and `prelaunch-verify` confirm preexisting `execution_root`, `config_dir`, and `Q-OBS.current-compile-01.json` with `ONE_SHOT_STARTED`, `retainedDir` (under real parent), `evidence_dir`, and `attempt_dir` absent, and full raw `history_path` (+ `reconciliations_path` if any) supplied to `reconciliation.load_readiness`.
4. **`C4_OUTER_TIME_AND_SINGLETON_S`:** Root invokes `/usr/bin/time -v -o <timeOutput> -- ./scripts/doctorwatson.sh --execute ...` once (`_check_time_output_preexec` observing `"precreated-zero-byte"`), and `build_native_argv` places only the child shell/guard PID into `supervisor` leaf `S` (`supervisor_envelope`).
5. **`C5_POLICY03_AND_ONE_SHOT_RETENTION`:** `CARRIED_UNKNOWN_POLICY_SPEND` keeps `wrapper_c_us` and `exact_remaining_supervisor_s` as `None`, preserves `POLICY_HOLDBACK_S`, `POLICY_SUPERVISOR_ALLOWANCE_S`, `POLICY_TOTAL_WALL_MAX_S`, `POLICY_F_WALL_MAX_S`, and stopped `C13`, allows at most one attempt (`attemptsAuthorizedIfAdopted == 1`) with no retry on any outcome, and leaves any collected result `UNACCEPTED` pending separate result review.
