# Producer-Fact Reconciled Security & Receiving-API Source Audit (Review Lane B) — `native-adapter-source09`

- **Review Lane:** `native-adapter-source09-review-B-producer-fact-reconciliation02` (reconciling producer citations for `native-adapter-source09-fresh-gemini4-security-review-B`; original `REPORT.md` preserved untouched)
- **Model Identity:** `gemini-4-argon` (High effort) via standing Opus 5.5 outage fallback (`AGENTS.md`). No backend attestation asserted beyond requested/returned identity.
- **Authoritative Canonical Snapshot Map:** `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/astra-1/NATIVE09-ROOT-SNAPSHOT14/ROOT-SNAPSHOT-MAP.json` (`23,346` bytes, `sha256: b275cc1e64686c7da4639d41c8f57bd032ed1247d001bd30ef936a00458d4272`)
- **Verdict:** **APPROVE** (Source quality and actual receiving-API contract alignment of `native-adapter-source09`, subject to finite operational/at-use conditions `C1`–`C6` in Section 4. Source review only; grants no native execution authority and establishes no live SDK/build/proof/stagenet acceptance.)

---

## 1. Authoritative Canonical Map & 8 Candidate File Pins

All 8 candidate files and receiving context files in `ROOT-SNAPSHOT-MAP.json` (`23,346` B, `b275cc1e64686c7da4639d41c8f57bd032ed1247d001bd30ef936a00458d4272`) were verified directly from disk bytes via `/usr/bin/python3 -I -S`:

| # | Target Path | Bytes | Exact SHA-256 (`ROOT-SNAPSHOT-MAP.json` & Disk) | `changed_since08` |
|---|-------------|------:|-------------------------------------------------|:-----------------:|
| 1 | `models/doctorwatson/examples/execution-selection.template.json` | `2,257` | `6a602c9362f7dfe7aca7bfab6c50020823f91fb17633e447922838bbf7bd9de3` | `false` |
| 2 | `models/doctorwatson/fixture_support.py` | `37,715` | `5cdc285ad61a630de7a1139faf203fed632ff367969cf0abca9e91fb4a44b7bb` | `true` |
| 3 | `models/doctorwatson/run_selected.py` | `134,603` | `e1aa86ec60a5c6919fcbf866e23437fc059d08bf05a7cb85ab30eed4c9af0e02` | `true` |
| 4 | `models/doctorwatson/test_execution_reconciliation.py` | `17,262` | `cce45d0f8fccb4f60d58535cd069d295126311c53a270725f506b91752a6a8bf` | `false` |
| 5 | `models/doctorwatson/test_integrated_roundtrip.py` | `12,265` | `2ed6c1066b16a4cedb4db445a8945661649d809bea42acbb659cc3c0bd682371` | `false` |
| 6 | `models/doctorwatson/test_native_adapter07.py` | `57,995` | `a65a0965e48334b33d4bce61384513a8aaf622b3a3a870513aae3a38692739c0` | `true` |
| 7 | `models/doctorwatson/test_run_selected.py` | `7,718` | `44139490d98a72340181a0b3ec174208e2ed92e21b48a312f56623e248347358` | `false` |
| 8 | `scripts/doctorwatson.sh` | `6,506` | `c6db6188b459088f7555f95898d5278ba01073944042799f9d7c93c704e567cb` | `false` |

---

## 2. Reconciled Producer-to-Candidate Validation of `N1`, `N2`, `N3`

### 2.1 `N1` — `f_result.processes[*].argv` Validation vs Real `bounded_sdk_compile.py` (`RESOLVED`)

- **Actual Producer (`context/native/bounded_sdk_compile.py`, `135,167` B, `6694bbdd8ad45fa01042eeb065f135f2142b033a66306b11c09442de341e6c09`):**
  - `CARGO = '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo'` (line 42) and `BACKEND = ('test', '--no-run', '--frozen', '-p', 'mori-backend', '--lib', '-j4')` (line 44).
  - `def cargo_argv(phase):` (lines 835–839): for `phase == 'F'`, returns `[CARGO, *BACKEND]` (`['/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo', 'test', '--no-run', '--frozen', '-p', 'mori-backend', '--lib', '-j4']`).
  - `def sandbox_args(c,phase,worker=True,precheck=False,retained_audit=False):` (lines 879–952):
    - Line 885 initializes `args=['--die-with-parent','--new-session','--unshare-user','--unshare-pid','--unshare-net','--unshare-ipc','--unshare-uts','--disable-userns','--cap-drop','ALL','--ro-bind','/','/']`.
    - Appends tmpfs/mask/tool/worktree/binding/cohort/disposition/head-file/source-copy mounts and environment flags, then at lines 945–952 appends:
      - `args += ['--chdir', p['worktree'], '--']`
      - If `worker`: `args += ['/usr/bin/python3', '-I', '-B', c['bindings']['runner']['path'], '--worker', phase, '--config', p['config']]`
      - Else: `args += cargo_argv(phase)`
      - Plus optional `'--precheck'` (line 948) or `'--retained-f-audit-r3'` / `'--retained-f-audit-r2'` / `'--retained-f-audit'` (lines 949–951).
  - `def launch(c,phase,config_sha,retained_audit=False):` (lines 1156–1250):
    - For `phase == 'F'` and `not retained_audit`, `variants = [(True,True),(False,False),(True,False)]` (line 1175), running 3 processes in sequence:
      1. `(audit=True, precheck=True)` — `cached_f_prep` worker (`len(args)` = `230` in P9 or `236` in P10; `--` at index `208` or `214`).
      2. `(audit=False, precheck=False)` — Cargo `F` build `[CARGO, *BACKEND]` (`len(args)` = `126`; `--` at index `109`).
      3. `(audit=True, precheck=False)` — `cached_f_audit` post-compile worker (`len(args)` = `205`; `--` at index `184`).
    - Lines 1186–1190:
      ```python
      separator=args.index('--');target=args[separator+1:]
      if audit:target+=['--config-sha256',config_sha,'--worker-fd',str(token_fd)]
      preserved=[stage_fd,child.fileno()]+([token_fd] if audit else [])
      cmd=['/usr/bin/python3','-I','-B','-S','/proc/self/fd/'+str(stage_fd),str(stage_fd),str(child.fileno()),str(token_fd),*target]
      bwrap_opts=args[:separator]+([] if '--as-pid-1' in args[:separator] else ['--as-pid-1']);fd=sealed_memfd('sdk-bwrap-argv',b'\0'.join(x.encode() for x in bwrap_opts)+b'\0');args=bwrap_opts+['--',*cmd]
      ```
    - Line 1238 appends `{'argv':args,'audit_worker':audit,'child_exit':0}` to `records`.
  - **Last-`"--"` Invariant Proof:** `bwrap_opts` is sliced prior to `separator = args.index('--')`, so `bwrap_opts` contains no `'--'`. Neither the 8-element trampoline prefix of `cmd` nor any real `target` (worker argv `['/usr/bin/python3', '-I', '-B', runner, '--worker', 'F', '--config', cfg, ...]` or `cargo_argv('F') = [CARGO, 'test', '--no-run', '--frozen', '-p', 'mori-backend', '--lib', '-j4']`) contains a bare `'--'` element. Thus `'--'` occurs **only once** in `args` (at index `len(bwrap_opts)`), and `sep = len(argv) - 1 - argv[::-1].index("--")` in `_validate_f_launch_result` (`run_selected.py:1625`) always resolves to that exact separator immediately before `['/usr/bin/python3', '-I', '-B', '-S']`.
- **Candidate Verification (`run_selected.py:1578-1645`, `fixture_support.py:372-415`, `test_native_adapter07.py:377-394, 440-450`):**
  - `_validate_f_launch_result` enforces `1 <= len(processes) <= 16`, exact keys `{"argv", "audit_worker", "child_exit"}`, `child_exit == 0`, `type(audit_worker) is bool`, `has_cargo_proc` (`audit_worker is False`), `1 <= len(argv) <= 4096` nonempty strings, `argv[0] == "--die-with-parent"`, `"--" in argv`, `sep = len(argv) - 1 - argv[::-1].index("--") > 0`, `sep + 4 < len(argv)`, and `argv[sep + 1:sep + 5] == ["/usr/bin/python3", "-I", "-B", "-S"]`.

---

### 2.2 `N2` — Removal of Dead Optional Constants & Exact Binding of `PRESERVED_PACKET_PINS` and 13/10-Key `current_disposition` (`RESOLVED`)

- **Actual Producers (`bounded_sdk_compile.py:46-101, 434-489, 1075`, `cached_current_build.py:94-101, 2422-2455`, `config.final.json:17593-17632`):**
  - `cached_current_build.py:94-101` (`179,597` B, `e439a2cdeab48e702c82d19c63c613df97b5f4e1506907256cfea1ba2f0ae3e3`) defines:
    ```python
    PRESERVED_PACKET_PINS = {
        'A0_FREEZE.json': 'bf26f2d9b5db546ba0e5549e0cb8dfb3aabeee25e6c51ed135cbc9b2e35d973a',
        'C_RAW_IDENTITIES.json': '387b188d8f2a319dc972cf4727ff21219f5207c3bc74f93970590d708f62719e',
        'D_FREEZE.json': 'f8408c7ae5b5ccb495c129a9ba4a7a7a376ee9cb4b911b6f53a588b07cd839aa',
        'E_RESOLVED.json': 'fd3db35861d3119f3ef925b682e8c53fcbd322478b7be531b9241b9789f65e23',
        'F_RETAINED_QUARANTINED_OUTPUTS_R3.json': '92b24bcd754a1ebff68e5e9bfd051b11f4d69ffb0aae03627525e8ea439ec6f5',
        'F_RETAINED_AUDIT_R3_RESULT.json': '2d3001c7c2816a2a2dd9aa80406dbec4db5e6451d200f689b6bc504e07a9110f',
    }
    ```
  - `bounded_sdk_compile.py:46-101, 1075` defines `BASE = '03496bffd944f4b46c9d2997863f996f00c85017'`, `LOCK = '946b097d0fd0a7e1343e67f72846dee4b4676fa0a87676340f6d97cf571e3993'`, `CACHED_CURRENT_BUILD_SHA256 = 'e439a2cdeab48e702c82d19c63c613df97b5f4e1506907256cfea1ba2f0ae3e3'`, `CANDIDATE_DISPOSITION_SHA256 = 'bec36a3f3fa522e9f8041b879a2357f7dcc0d9e9b8c7457770f44af2f17d410d'`, `EXPECTED_PRIOR_ESP_US = (2427120416, 316932500, 2110187910)`, `EXPECTED_PRIOR_RESIDUAL_US = 6`, `EXPECTED_NATIVE21_DELTAS_ESP_US = (400517240, 6931787, 393585452)`, `EXPECTED_RESERVE_AB_US = 4556857`, `EXPECTED_FLOORS_S = (4275, 465, 10373)`, `POLICY_HOLDBACK_S = 113`, `POLICY_SUPERVISOR_ALLOWANCE_S = 10248`, `POLICY_TOTAL_WALL_MAX_S = 4162`, `POLICY_F_WALL_MAX_S = 352`.
  - `resource_disposition_state(c)` (`bounded_sdk_compile.py:434-489`) and `enforce_current_resource_disposition(c, guard, at_start=True)` (`cached_current_build.py:2422-2455`) emit `current_disposition` with the exact 13 policy keys (`disposition_kind`, `cumulative_before_wrapper_E_us`, `cumulative_before_wrapper_S_us`, `cumulative_before_wrapper_P_us`, `effective_prior_E_us`, `effective_prior_S_us`, `effective_prior_P_us`, `wrapper_c_us`, `exact_remaining_supervisor_s`, `policy_holdback_s`, `policy_supervisor_allowance_s`, `policy_remaining_total_wall_max_s`, `policy_remaining_f_wall_max_s`) for `CARRIED_UNKNOWN_POLICY_SPEND`, or the 10 established keys (`cumulative_before_wrapper_E_us`, `cumulative_before_wrapper_S_us`, `cumulative_before_wrapper_P_us`, `effective_prior_E_us`, `effective_prior_S_us`, `effective_prior_P_us`, `remaining_supervisor_cpu_seconds`, `exact_remaining_supervisor_s`, `supervisor_spend_ceiling_s`, `disposition_kind`) for `ESTABLISHED_PINNED`.
- **Candidate Verification (`run_selected.py:391-474, 477-653, 801-840, 1330-1566`, `fixture_support.py:50-92, 271-315, 528-688`, `test_native_adapter07.py:457-503, 705-792`):**
  - All dead optional constants from `source08` (`POST_R3_TOTAL_BUDGET_S`, `F_CARGO_WALL_S`, `DEFAULT_EXPECTED_D_FREEZE_SHA256`, `DEFAULT_EXPECTED_E_RESOLVED_SHA256`) are removed.
  - `_helper_constants` (`run_selected.py:446-474`) parses `PRESERVED_PACKET_PINS` from `cached_current_build.py` and extracts `D_FREEZE.json` (`f8408c7ae5b5ccb495c129a9ba4a7a7a376ee9cb4b911b6f53a588b07cd839aa`) and `E_RESOLVED.json` (`fd3db35861d3119f3ef925b682e8c53fcbd322478b7be531b9241b9789f65e23`); `_validate_source_home_reconstitution` (`run_selected.py:1553-1566`) enforces exact equality on both.
  - `_config_facts` (`run_selected.py:477-653`) and `resolve_selection` (`run_selected.py:811-840`) cross-verify `config.resource_disposition` against `_guard_constants`, check `upper_bound_s - holdback_s == allowance_s` (`10361 - 113 == 10248`), `floors[0] - holdback_s == rem_total_max_s` (`4275 - 113 == 4162`), and `floors[1] - holdback_s == rem_f_max_s` (`465 - 113 == 352`), construct `derivedCurrentDisposition` with all 13 policy keys (or 10 established keys), and `_validate_debit_state` (`run_selected.py:1330-1478`) enforces `disp == derived_disp` (lines 1428–1430, 1463–1466) and `effective == effective_prior + live` across start and end states.

---

### 2.3 `N3` — Retained-Directory Parent Validation & Post-Native Failure Evidence Preservation (`RESOLVED`)

- **Candidate Verification (`run_selected.py:703-713, 776-777, 2209-2217, 2530-2533, 2673-2818`, `test_native_adapter07.py:794-922`):**
  - `_require_real_parent_dir` (`run_selected.py:703-713`) checks via `os.lstat` that `Path(outer["retainedDir"]).parent` is an existing real non-symlink directory before any stage runs (`resolve_selection` at lines 776–777, `_require_existing_provisioned_host` at lines 2209–2210, and actual-use recheck at lines 2530–2533), and `retained_dir.mkdir(parents=False, exist_ok=True)` (line 2676) never creates missing intermediate parents.
  - Post-native `cleanup.json` writes (`run_selected.py:2675-2730`) and outer record writes (`run_selected.py:2714-2818`) catch `OSError`, unlink any partial `attempt / "consumer-result.json"`, write `attempt / "result.json"` with `terminal = "unknown-submission"`, `unknownScope = "local-process"`, `cleanupState = "unknown"`, `consumerResult = None`, and return `EXIT_UNKNOWN` (`3`), preserving unknown/cleanup debt in `reconciliation.load_readiness` and `delivery.attempt_obligations`.

---

## 3. Retained `source07` Mandatory Findings (`A-M1..A-M5`, `B-M1..B-M4`)

All 9 prior findings remain resolved and intact:
- **`A-M1` / `B-M1`:** `OP0..OP6` absent from `PROVISION_STEPS` and forbidden (`run_selected.py:59-69`); `_require_existing_provisioned_host` (`run_selected.py:2205-2236`) requires preexisting `execution_root`, `config_dir`, and `Q-OBS.current-compile-01.json` and absent `ONE_SHOT_STARTED`; prelaunch failure before `native` (`run_selected.py:2660-2672`) never invokes `abort-prelaunch-cleanup` or `teardown`.
- **`A-M2`:** `_check_time_output_preexec` (`run_selected.py:2239-2256`) allows absent or precreated zero-byte regular non-symlink `timeOutput` (`operator_provisioner.py:708`) and refuses symlinks, non-regular files, or non-empty files.
- **`A-M3`:** `_restoration_probe` (`run_selected.py:2155-2202`) checks both cgroup E/S/P absence and all 3 request mounts (`execution_root`, `source_home`, `packet`) in `/proc/self/mountinfo`.
- **`A-M4`:** Dynamic supervisor allowance (`10,248s` under `CARRIED_UNKNOWN_POLICY_SPEND`) derived from `config.resource_disposition` and cross-checked against `_guard_constants` (`run_selected.py:477-653, 811-840, 1393-1430`).
- **`A-M5` / `B-M4`:** All 7 outer pins required in `task.identity.input_files` (`run_selected.py:85-93, 295-336`); `outer.helper` bound as guard sibling `cached_current_build.py` matching `CACHED_CURRENT_BUILD_SHA256` (`run_selected.py:802-810`); full schema validation of debit states, reconstitution, seed counts, `f_result`, and quarantined outputs (`run_selected.py:1330-1980`).
- **`B-M2`:** Missing keys in `current_disposition` or `supervisor_self_observation` raise `CollectError` cleanly (`run_selected.py:1330-1478, 1750-1980`).
- **`B-M3`:** Positive CLI `main()` and `_default_runner` mock roundtrip tests in `test_native_adapter07.py:924-1036`.

---

## 4. Verdict & Finite Conditions (`C1`–`C6`)

**Verdict:** **`APPROVE`**

- **`C1` (Source Quality Only):** Approves `native-adapter-source09` source and receiving-API alignment only; grants no native execution authority and does not establish SDK/build/proof/stagenet acceptance.
- **`C2` (Unaltered Accounting, `c = NULL`, Holdbacks, and `C13`):** Historical `c` remains `NULL` (`wrapper_c_us = null`, `wrapper_c_status = "CARRIED_UNKNOWN_POLICY_SPEND"`, `wrapper_c_basis = null`); adopted priors (`E = 2,427,120,416 us`, `S = 316,932,500 us`, `P = 2,110,187,910 us`, `residual = 6 us`), `NATIVE21` deltas (`E = 400,517,240 us`, `S = 6,931,787 us`, `P = 393,585,452 us`), reserve `A/B` (`4,556,857 us`), floors (`4275s` / `465s` / `10373s`), policy holdback (`113s`), supervisor allowance (`10,248s`), remaining wall maxima (`4162s` total / `352s` F), and stopped `C13` remain unaltered.
- **`C3` (At-Use Pre-Provisioned Host Prerequisite):** Automated beta provisioning (`OP0..OP6`) must pre-provision `execution_root`, `config_dir`, fresh `Q-OBS.current-compile-01.json`, delegated cgroups, and absent `ONE_SHOT_STARTED` before `run_selected.py` is invoked on the native route; zero-byte precreated `timeOutput` is permitted. No new human gate is introduced.
- **`C4` (Exact Outer & Helper Pin Lock at Use):** All 7 outer input pins (`guard`, `config`, `operator`, `tail_runner`, `request`, `q_obs`, `resource_policy`), sibling `cached_current_build.py` (`CACHED_CURRENT_BUILD_SHA256`), and `PRESERVED_PACKET_PINS` (`D_FREEZE.json = f8408c7ae5b5ccb495c129a9ba4a7a7a376ee9cb4b911b6f53a588b07cd839aa`, `E_RESOLVED.json = fd3db35861d3119f3ef925b682e8c53fcbd322478b7be531b9241b9789f65e23`) must match their exact SHA-256 pins at prelaunch and actual-use recheck.
- **`C5` (Fail-Closed Unknown & Debt Preservation):** Any post-launch failure, timeout, receipt/disposition mismatch, cleanup/restoration failure, or post-native `OSError` must remain `terminal = "unknown-submission"` (`EXIT_UNKNOWN = 3`) with `cleanupState = "unknown"` and `consumerResult = None`.
- **`C6` (Writable `attempt` and `retainedDir` Parent):** Both `attempt` and the existing real non-symlink parent of `outer.retainedDir` must reside on a writable filesystem at use.
