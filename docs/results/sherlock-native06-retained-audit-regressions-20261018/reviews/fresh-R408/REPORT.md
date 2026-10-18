# Fresh Designated Security Source Review (`R408`)

## 1. Reviewer Identity, Scope, and Readable Coverage Attestation

- **Review Role / Kind**: Non-author designated security source reviewer (`designated_security_owner` / `security_source_review`).
- **Routing & Returned Identity**: Primary route `claude-opus-5-5` (`high` effort); executed under `standing_opus_quota_outage_fallback` by **Gemini 4 Argon (High)** (`requested_model: gemini-4-argon-high`, `returned_identity: gemini-4-argon-high`, `effort: high`).
- **Independence & Execution Boundary**: Fresh review context with zero reliance on prior misdescribed source review text. Distinct from source-owner test/helper author (`6277`) and parser-delta author (`3b833`). No tools or commands were executed.
- **Complete Readable Source Coverage**:
  1. **Guard source**: `bounded_sdk_compile.py` (`81,624` bytes, `1,152` lines, SHA-256 `6fcfaffd73e98dcf9ce02a8151ab8dafa27d206f55572db6221dcd7a8df2dd9d`) — read in full (`1,152 / 1,152` lines, `0` missing bytes).
  2. **Output/accounting unit test module**: `test_output_retained_accounting.py` (`20,697` bytes, `513` lines, SHA-256 `3cadfc79dffe6d9218258b98c8ae4a35af0e163390c39acb3cbb98cac71493c2`) — read in full (`513 / 513` lines, `0` missing bytes).
  3. **Supplied resource, unit-test, beta-override, and proposed config/launch facts**: read in full. Unsupplied full SDK payload contents and uninspected retained target artifacts are not read or approved here; source and unit-test review is strictly distinct from live host attestation, retained-audit execution approval, native output acceptance, or C4/R2 acceptance.

---

## 2. Pre-Verdict Verbatim Source Reproductions and Line-Range Verification

All excerpts below are reproduced verbatim from the supplied 1,152-line `bounded_sdk_compile.py` (`6fcfaffd73e98dcf9ce02a8151ab8dafa27d206f55572db6221dcd7a8df2dd9d`).

### 2.1 `Group.__init__` Complete Definition (lines `517–518`, inside `class Group:` at lines `516–546`)

```python
    def __init__(self,c,phase):
        self.path=cg_fs(phase_cgroup(c,phase));self.v=c['limits'][phase];self.created=False
```

### 2.2 `worker` Complete Definition (lines `1048–1057`)

```python
def worker(c,phase,retained_audit=False):
    # A0/A/C/D workers and E/F post-command audits are dispatched. They run inside the same bounded
    # cgroup/sandbox envelope; Cargo receives no Python callback or wrapper flag.
    if phase=='A0':freeze_inputs(c)
    elif phase=='A':stage(c)
    elif phase=='C':verify_raw(c)
    elif phase=='D':freeze_source(c)
    elif phase=='E':metadata_check(c)
    elif phase=='F':output_check(c,'F_RETAINED_QUARANTINED_OUTPUTS.json' if retained_audit else 'F_QUARANTINED_OUTPUTS.json')
    else:raise Unsupported('invalid data worker phase')
```

### 2.3 `retained_f_audit` Complete Definition (lines `1059–1093`)

```python
def retained_f_audit(c,config_sha):
    start=time.monotonic()
    need(c['mode']=='tier1-and-compile','retained F audit requires tier1-and-compile mode')
    consent(c);supervisor_envelope(c)
    def expired(signum,frame):raise Refusal('whole-job wall cap')
    init_wall=min(c['total_wall_seconds'],c['limits']['F']['wall_seconds'])
    need(init_wall>0,'whole-job wall cap')
    signal.signal(signal.SIGALRM,expired);signal.setitimer(signal.ITIMER_REAL,init_wall)
    try:
        sup_cpu=c['supervisor_cpu_seconds']-(read_numbers(cg_fs(cg_paths(c)[1])/'cpu.stat').get('usage_usec',0)+999999)//1000000
        need(sup_cpu>0,'supervisor CPU cap');resource.setrlimit(resource.RLIMIT_CPU,(sup_cpu,sup_cpu))
        bound_identities(c);owned_paths(c);quotas(c,'F');frozen_source(c)
        p=c['paths'];root=Path(p['execution_root']);packet=Path(p['packet'])
        need((root/'ONE_SHOT_STARTED').is_file(),'one-shot start missing')
        predecessors=prior_receipts(c,'F',False,retained_audit=True)
        need(jload(packet/'TERMINAL_FAILURE.json')=={'message':'phase exit nonzero F','native_outputs_accepted':False,'type':'Refusal'},'unexpected prior terminal failure')
        prior_wall=sum(jload(packet/(ph+'_RESULT.json'))['wall_seconds'] for ph in PHASES[:-1])
        wall_budget=min(c['total_wall_seconds']-prior_wall,c['limits']['F']['wall_seconds'])
        rem_wall=wall_budget-(time.monotonic()-start)
        need(rem_wall>0,'whole-job wall cap');signal.setitimer(signal.ITIMER_REAL,rem_wall)
        for name in ('F_RESULT.json','F_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS.json','TERMINAL_FAILURE_RETAINED_AUDIT.json','F_retained_audit.stdout','F_retained_audit.stderr'):
            need(not (packet/name).exists(),'retained audit output already exists: '+name)
        try:
            result=launch(c,'F',config_sha,retained_audit=True)
            bound_identities(c)
            need(time.monotonic()-start<=wall_budget,'whole-job wall cap')
            result.update({'retained_audit':True,'audit_config_sha256':config_sha,'audit_runner_sha256':c['bindings']['runner']['sha256'],'marker_sha256':digest(root/'ONE_SHOT_STARTED'),'predecessors':predecessors,'quarantined_outputs_sha256':digest(packet/'F_RETAINED_QUARANTINED_OUTPUTS.json'),'native_outputs_accepted':False})
            receipt(c,'F_RETAINED_AUDIT_RESULT.json',result,'F')
        except BaseException as e:
            signal.setitimer(signal.ITIMER_REAL,0)
            try:receipt(c,'TERMINAL_FAILURE_RETAINED_AUDIT.json',{'type':type(e).__name__,'message':str(e),'native_outputs_accepted':False},'F')
            except Exception:pass
            raise
    finally:
        signal.setitimer(signal.ITIMER_REAL,0)
```

### 2.4 Exact `sandbox_args` and `launch` Signatures (lines `450` and `643`)

- `sandbox_args` signature (line `450`, function span lines `450–490`):
```python
def sandbox_args(c,phase,worker=True,precheck=False,retained_audit=False):
```

- `launch` signature (line `643`, function span lines `643–734`):
```python
def launch(c,phase,config_sha,retained_audit=False):
```

### 2.5 Exact `launch` `variants` Assignment (lines `658–659`)

```python
        variants=[(True,False)] if retained_audit else [(False,False)] if phase in ('B','E','F') else [(True,False)]
        if phase in ('E','F') and not retained_audit:variants=[(True,True),(False,False),(True,False)]  # same phase/group/deadline, data audit only
```

### 2.6 Exact Call Passing `worker`/`precheck`/`retained_audit` to `sandbox_args` in `launch` (line `661`)

```python
            args=sandbox_args(c,phase,audit,precheck,retained_audit);suffix='_retained_audit' if retained_audit else '_precheck' if precheck else '_audit' if audit and phase in ('E','F') else ''
```

*(Related dispatch calls: `result=launch(c,'F',config_sha,retained_audit=True)` at line `1082`; `verify_worker_context(c,a.worker,a.precheck,a.worker_fd,a.config_sha256,a.retained_f_audit)` at line `1139`; `worker(c,a.worker,a.retained_f_audit)` at line `1143`.)*

---

## 3. Verdict

**VERDICT: APPROVE (`APPROVED_SOURCE_WITH_LIMITS`)** for guard candidate `bounded_sdk_compile.py` (`6fcfaffd73e98dcf9ce02a8151ab8dafa27d206f55572db6221dcd7a8df2dd9d`) and four-method unit test module `test_output_retained_accounting.py` (`3cadfc79dffe6d9218258b98c8ae4a35af0e163390c39acb3cbb98cac71493c2`).

- **Literal source correction required**: **None** (0 bytes changed).
- **Boundary condition**: This verdict approves the guard source and unit-test evidence only. It does **not** approve live retained-audit execution until the missing prospective inputs listed in Section 6 are materialized and bound, and it does **not** accept native outputs (`native_outputs_accepted: False` remains mandatory).

---

## 4. Substantive Security and Correctness Findings

### 4.1 Normal Phase Compatibility (`A0`–`F` under `run`)
- In `main()` (lines `1125–1147`), `--retained-f-audit` defaults to `False` (line `1132`). When omitted, the supervisor path (lines `1144–1147`) verifies `a.worker_fd is None and not a.precheck` and invokes `run(c, a.config_sha256)` (lines `1095–1123`).
- Every modified helper defaults `retained_audit=False`: `sandbox_args` (line `450`), `prior_receipts` (line `595`), `child_mounts` (line `605`), `verify_worker_context` (line `611`), `launch` (line `643`), and `worker` (line `1048`).
- When `retained_audit=False`:
  - `launch` initializes `counts={'stdout':0,'stderr':0}` (line `651`), skips `group.prior_cpu` initialization (lines `653–655`, so `getattr(self,'prior_cpu',0)` in `Group.check` at line `537` is `0`), and selects `variants` identically to the prior normal flow (`[(True,False)]` for `A0,A,C,D`; `[(False,False)]` for `B`; `[(True,True),(False,False),(True,False)]` for `E,F` at lines `658–659`).
  - `sandbox_args` mounts `c['paths']['target']` with `--bind` (`rw`) in phase `F` (line `476`) and does not append `--retained-f-audit` (line `489`).
  - `worker` dispatches `phase=='F'` to `output_check(c, 'F_QUARANTINED_OUTPUTS.json')` (line `1056`).

### 4.2 Dep-Info Normalization and Manifest / Frozen / Include / Host-Unit Binding
- **Lexical + kernel-resolved normalization (`make_words`, `depinfo_path`, `parse_depinfo`, lines `917–973`)**:
  - `make_words` (lines `917–930`) strips line continuations (`\\\n`), processes backslash escapes, rejects shell/make metacharacters `$`, `;`, `|` via `Unsupported('unsupported dep-info syntax')` (line `926`), and refuses trailing escapes (line `928`).
  - `depinfo_path` (lines `932–958`) rejects non-strings, NULs, empty words, `'//'`, trailing `'/'`, and `'.'` segments (lines `934–936`).
  - When `'..'` is present (lines `940–958`), relative paths anchor at `worktree` (`base=worktree`, line `946`), while absolute paths must match `word.startswith(str(b)+'/')` for a bound root `b` in `(worktree, *roots)` and anchor at the longest matching root `base=max(candidates, key=lambda b: len(str(b)))` (lines `941–944`).
  - Stepping through `rel_parts` on `stack` (lines `947–953`) refuses any `'..'` when `stack` is empty (line `950`) or if the final `stack` is empty (line `953`), preventing any escape above `base` (blocking root escapes and cross-crate traversals between sibling registry or checkout roots).
  - Every directory popped by `'..'` is recorded in `ascended` and verified via `need(not any(x.is_symlink() for x in [d,*d.parents]) and d.is_dir() and d.resolve(strict=True)==d, ...)` (lines `955–956`), defeating symlinked-parent traversal mismatches.
  - The normalized path `norm` must exist, pass `regular(norm)` (no symlink in path or ancestors, `S_ISREG` via `follow_symlinks=False`, lines `233–236`), and satisfy `raw.resolve(strict=True)==norm` (line `957`).
- **Downstream bindings in `output_check` (lines `991–1045`)**:
  - Re-verifies `frozen = frozen_source(c)['files']` against `D_FREEZE.json` (line `992`, `894–895`).
  - Restricts `dep_roots` to the 2 Git checkouts, 423 registry roots, `target`, `readonly_tools`, and `/usr` (line `994`).
  - Every normalized dep-info path (both target LHS and dependency RHS across `all_paths`, line `998`) under `worktree` must exist in `source_manifest['files']` (lines `1000–1001`).
  - Every normalized dep-info path under `source_home` must exist in `frozen` (`D_FREEZE.json['files']`) and must **not** start with `'git-symlink:'` (lines `1002–1004`).
  - For any `.d` file whose dependency inputs touch `'/midnight-'` or `'/midnight-ledger-'`, every non-`.rs` dependency under `source_home` must be in `c['includes']` (lines `1006–1009`).
  - Every executable build-script binary and `.so` proc-macro under `target` must map via `artifact_dependencies(p, deps)` (lines `975–988`) to a unique host unit in `E_RESOLVED.json['host_units']` (lines `1010–1016`), and all `run-build-script-*`, `build-script-*`, and `lib-*` fingerprints are cross-verified against `host` and `resolved` (lines `1017–1044`).

### 4.3 Retained `F`-Only Read-Only Target and Python-Only Worker Dispatch
- **Supervisor entry**: `main()` (lines `1144–1147`) rejects `--worker-fd` and `--precheck` on the supervisor and routes `--retained-f-audit` exclusively to `retained_f_audit(c, a.config_sha256)`.
- **Single data-worker variant**: `retained_f_audit` calls `launch(c, 'F', config_sha, retained_audit=True)` (line `1082`). In `launch`, line `647` enforces `phase=='F'` and lines `658–659` set `variants=[(True,False)]` (only `audit=True, precheck=False`). Neither `precheck=True` nor the Cargo variant (`audit=False`, `cargo_argv('F')`) is scheduled.
- **Read-only target mount in bwrap and mountinfo verification**:
  - `sandbox_args` (line `451`) enforces `need(not retained_audit or (phase=='F' and worker and not precheck),'retained audit sandbox mismatch')`.
  - Line `476` emits `['--ro-bind', p['target'], p['target']]` when `retained_audit=True` (and `p['source_home']` is `--ro-bind` at line `474`, `p['worktree']` is `--ro-bind` at line `461–462`).
  - Line `486` and line `489` construct the inner command `/usr/bin/python3 -I -B <runner> --worker F --config <config> --retained-f-audit --config-sha256 <sha> --worker-fd <token_fd>`.
  - `child_mounts` (line `608`) sets `wanted[p['target']]='ro'`, and `verify_worker_context` inside the sandbox (lines `626–630`) parses `/proc/self/mountinfo` to verify that the kernel mount for `p['target']` is `ro`.
  - Neither Cargo nor any compiled artifact or build script under `target` is executed.

### 4.4 Seccomp, FD, and Cgroup Cleanup and Immediate Emptiness
- **Seccomp & FD hygiene (`TRAMPOLINE`, line `584`, and `launch`, lines `662–730`)**:
  - `TRAMPOLINE` verifies `F_SEAL_WRITE|F_SEAL_SHRINK|F_SEAL_GROW|F_SEAL_SEAL` on `stage_fd` and `token_fd`, sets `PR_SET_NO_NEW_PRIVS`, installs the `SECCOMP_RET_USER_NOTIF` filter blocking network/io_uring syscalls, hands the listener FD back over `control_fd`, and closes every FD in `/proc/self/fd` except `{0, 1, 2, token_fd}` before `os.execve`.
  - `launch` registers the seccomp listener on `sel` and fails closed on any notification (`network_pending(listener)` at lines `713` and `720`).
  - The inner `finally` block (lines `723–730`) kills and waits on `proc` if still alive, closes `listener`, `parent`, `child`, `fd`, `stage_fd`, `token_fd`, `sel`, and all log file objects.
- **Cgroup preflight, immediate post-process emptiness, and teardown**:
  - `supervisor_envelope(c)` (lines `548–582`) verifies `phases/cgroup.procs` is empty (line `577`) and `phases` has no residual child directories (`need(not _cg_children(p),'phase parent contains residual groups')`, line `582`).
  - Immediately after `proc.wait()==0`, line `721` enforces `need(read_numbers(group.path/'cgroup.events').get('populated',0)==0,'background descendants remain after phase process')`.
  - The outer `finally` block (lines `732–734`) unconditionally invokes `group.close()` (lines `540–546`), which writes `'1'` to `cgroup.kill`, polls until `populated == 0` within a 5s deadline, and removes `phases/sdk-once-F` via `_cg_rmdir(self.path)`.

### 4.5 Prior Receipt, Marker, Config, Runner, and Output-Hash Binding & Quarantine Qualification
- `retained_f_audit` (lines `1059–1093`) and `verify_worker_context` (lines `611–635`) enforce:
  - `ONE_SHOT_STARTED` existence (line `1072`) and SHA-256 binding (`marker_sha256` in worker context at line `623` and in `F_RETAINED_AUDIT_RESULT.json` at line `1085`), plus single-use launch nonce consumption (`USED_LAUNCH_<nonce>`, line `632`).
  - Exact 15-file predecessor hash map via `prior_receipts(c, 'F', False, retained_audit=True)` (lines `595–603`, `624`, `1073`, `1085`): `A0_RESULT.json` through `E_RESULT.json`, `D_FREEZE.json`, `E_RESOLVED.json`, `F.stdout`, `F.stderr`, `F_precheck.stdout`, `F_precheck.stderr`, `F_audit.stdout`, `F_audit.stderr`, and `TERMINAL_FAILURE.json`.
  - Exact prior failure content check (line `1074`): `TERMINAL_FAILURE.json` must equal `{'message':'phase exit nonzero F','native_outputs_accepted':False,'type':'Refusal'}`.
  - Exclusivity check before launch (lines `1079–1080`): refuses if any of `F_RESULT.json`, `F_QUARANTINED_OUTPUTS.json`, `F_RETAINED_AUDIT_RESULT.json`, `F_RETAINED_QUARANTINED_OUTPUTS.json`, `TERMINAL_FAILURE_RETAINED_AUDIT.json`, `F_retained_audit.stdout`, or `F_retained_audit.stderr` exists.
  - Distinct qualified receipt outputs: worker writes `F_RETAINED_QUARANTINED_OUTPUTS.json` (line `1056`) with `'acceptance': 'UNACCEPTED: independent result review required; no artifact executed'` (line `1045`), and supervisor writes `F_RETAINED_AUDIT_RESULT.json` (line `1086`) binding `retained_audit: True`, `audit_config_sha256`, `audit_runner_sha256`, `marker_sha256`, `predecessors`, `quarantined_outputs_sha256`, and `native_outputs_accepted: False` (or on failure writes `TERMINAL_FAILURE_RETAINED_AUDIT.json` with `native_outputs_accepted: False` at line `1089`, leaving `TERMINAL_FAILURE.json` intact).

### 4.6 Independent Resource Reconciliation (Cumulative Ceilings vs. Remaining Values)

Using the supplied native06 measured counters and cgroup readback (`ROOT-CGROUP-READBACK-20261018.json`, SHA-256 `5b7ab5868c7482f9555283e94d9dc451897b33b13ad67d408031daf1ffa266a9`):

| Resource Field | Original Ceiling | Measured Native06 Usage | Guard Accounting Mechanism (Line Anchors) | Proposed Config Value | Effective Remaining Budget for Retained Audit |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `total_wall_seconds` | `6720` s | Outer elapsed: `157.16370775500036` s (`A0..E` sum: `39.1983804539959241` s; non-`A0..E` residual: `117.9653273010044359` s) | `retained_f_audit` (lines `1075–1077`) sums `prior_wall` (`39.198380454` s) from `A0..E_RESULT.json` and computes `c['total_wall_seconds'] - prior_wall - (now - start)` | `6602` s (`6720 - 118`, deducting `ceil(117.9653273...) = 118` s of unrecorded post-`E` elapsed time) | `6602 - 39.198380454 = 6562.801619546` s ($\le 6720 - 157.163707755 = 6562.836292245$ s; no double-deduction or omission) |
| `limits.F.wall_seconds` | `2700` s | Unrecorded `F` + post-`E` residual: $\le 117.9653273010044359$ s | `retained_f_audit` (lines `1064, 1076, 1084`) and `launch` (line `709`) enforce `wall_budget <= c['limits']['F']['wall_seconds']` | `2582` s (`2700 - 118`, deducting `ceil(117.9653273...) = 118` s) | `2582` s ($\le 2700 - 117.965327301 = 2582.034672699$ s) |
| `supervisor_cpu_seconds` | `10800` s | `supervisor/cpu.stat` `usage_usec`: `48,374,910` $\mu$s (`48.374910` s) | `retained_f_audit` (lines `1068–1069`) live-deducts `(usage_usec + 999999) // 1000000 = 49` s from `c['supervisor_cpu_seconds']` for `RLIMIT_CPU` | `10800` s (unmodified cumulative ceiling) | `10800 - 49 = 10751` s (`RLIMIT_CPU` hard/soft cap in integer seconds; exact `usec` $\to$ `s` ceiling conversion) |
| `limits.F.cpu_time_us` | `10,800,000,000` $\mu$s | `phases/cpu.stat` `usage_usec` (`A0..F` aggregate): `240,632,042` $\mu$s (`240.632042` s) | `launch` (lines `654–655`) reads `phases/cpu.stat` `usage_usec` into `group.prior_cpu`; `Group.check` (line `537`) enforces `prior_cpu + child_usage_usec <= cpu_time_us` | `10,800,000,000` $\mu$s (unmodified cumulative ceiling) | `10,800,000,000 - 240,632,042 = 10,559,367,958` $\mu$s (~`10,559.37` s; conservative since `phases/cpu.stat` includes `A0..E` + failed `F`; units match in $\mu$s) |
| `limits.F.stdout_bytes` | `16,777,216` B | `F_precheck.stdout` (`0`) + `F.stdout` (`0`) + `F_audit.stdout` (`0`) = `0` B | `launch` (lines `651–652, 703, 717`) initializes `counts['stdout'] = 0` from prior `F*.stdout` files and accumulates new stdout against `stdout_bytes` | `16,777,216` B (unmodified cumulative ceiling) | `16,777,216` B |
| `limits.F.stderr_bytes` | `16,777,216` B | `F_precheck.stderr` (`0`) + `F.stderr` (`10,237`) + `F_audit.stderr` (`47`) = `10,284` B | `launch` (lines `651–652, 703, 717`) initializes `counts['stderr'] = 10284` from prior `F*.stderr` files and accumulates new stderr against `stderr_bytes` | `16,777,216` B (unmodified cumulative ceiling) | `16,777,216 - 10,284 = 16,766,932` B |

---

## 5. Assessment of Four-Method Synthetic Test Module (`3cadfc79...`) and Unit Results

- **Actual PASS Scope Verified**:
  - `test_output_retained_accounting.py` (`3cadfc79dffe6d9218258b98c8ae4a35af0e163390c39acb3cbb98cac71493c2`, 4 methods in `OutputAndRetainedAccountingTests`) passed in `0.055s` (`exit: 0`, `inputs_pre_post_equal: true`, `native_sdk_operations: 0`), alongside the previously passed 1-method negative parser test `test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals`. Neither passing cohort is to be rerun.
  - The 4 methods in `OutputAndRetainedAccountingTests` directly cover:
    1. `test_output_check_persists_receipt_and_enforces_manifest_freeze_include_and_host_unit` (lines `258–326`): verifies `output_check` and `worker(c, 'F', retained_audit=True)` over normalized relative worktree `'..'` and absolute `source_home` `'..'` dep-info paths, plus all 4 refusal gates (`source_manifest['files']` omission, `frozen_source` omission or `git-symlink:` prefix, `c['includes']` omission for SDK non-`.rs` inputs, and `E_RESOLVED.json` host-unit root mismatch).
    2. `test_retained_sandbox_mounts_and_15_predecessors_contract` (lines `328–343`): verifies `--ro-bind` on `target`, `--retained-f-audit` worker flag, refusal of invalid `(phase, worker, precheck)` triples (`('E',True,False)`, `('F',False,False)`, `('F',True,True)`), `child_mounts` `'ro'` target entry, and exact 15-file `prior_receipts` set.
    3. `test_launch_accumulates_prior_f_logs_and_phases_cgroup_cpu_for_retained_audit` (lines `345–385`): verifies pre-creation refusal when prior `F*.stderr` sum (`10,284` B) exceeds cap, pre-creation refusal when `phases/cpu.stat` (`240,632,042` $\mu$s) meets/exceeds `cpu_time_us`, and mid-run `Group.check()` refusal when `prior_cpu + child_usage_usec > cpu_time_us`.
    4. `test_retained_f_audit_enforces_entry_deadline_cpu_deduction_exclusivity_and_qualified_receipt` (lines `387–509`): verifies entry `ITIMER_REAL` setup (`2582` s), post-preflight timer deduction (`2582 - 5.0` s), `finally` timer reset (`0`), supervisor `RLIMIT_CPU` deduction (`10800 - 49 = 10751` s), `F_RETAINED_AUDIT_RESULT.json` fields (`retained_audit: True`, `native_outputs_accepted: False`, 15 predecessors, hashes), and refusals on existing `F_RETAINED_AUDIT_RESULT.json`, exhausted entry wall budget, exhausted supervisor CPU, and tampered `TERMINAL_FAILURE.json`.
- **Coverage Gaps & Additional Unit Test Necessity**:
  - Minor unmocked branches in the unit suite: (a) the `except BaseException:` path in `retained_f_audit` (lines `1087–1091`) writing `TERMINAL_FAILURE_RETAINED_AUDIT.json` when `launch` fails; (b) the `c['total_wall_seconds'] - prior_wall < c['limits']['F']['wall_seconds']` branch of `min(...)` at line `1076`; (c) incremental worker stream accumulation inside `launch`'s `sel.select` loop on top of non-zero initial `counts`; and (d) `main()` CLI flag rejection at lines `1138, 1145`.
  - Each of those branches is a short, linear guard verified directly by source inspection above. **No additional unit tests or reruns are necessary.**

---

## 6. Conditional Assessment of Proposed Config Delta and Launch Capsule (`409`)

1. **Conditional Compatibility**:
   - **Cap Delta (`total_wall_seconds: 6720 -> 6602`, `limits.F.wall_seconds: 2700 -> 2582`)**: Conditionally **APPROVED** as reconciled in Section 4.6 (`118` s ceiling deduction of unrecorded post-`E` elapsed time, paired with live `prior_wall` deduction of `A0..E` and live cgroup/log counter accumulation).
   - **Launcher Template (`sudo -n -- /bin/sh -eu -c` $\to$ `echo $$ > .../supervisor/cgroup.procs` $\to$ `exec setpriv --reuid=1000 --regid=1000 --clear-groups --inh-caps=-all --bounding-set=-all --no-new-privs` $\to$ `exec env -i ... python3 -I -B -S <runner> --retained-f-audit --config <config> --config-sha256 <sha>`)**: Conditionally **APPROVED**. In-place `exec` preserves PID `$$` in the `supervisor` leaf cgroup, drops all capabilities and privileges to UID/GID `1000`, and invokes the exact pinned runner path with `--retained-f-audit`, satisfying `supervisor_envelope(c)` (lines `548–582`), `executing_identity(c)` (lines `228–231`), and `main()` (lines `1125–1147`). Under the October 18 Beta automation override, non-interactive `sudo -n` execution without human operator gating is authorized policy.
2. **Genuinely Missing Prospective Inputs Blocking Live Audit Dispatch**:
   - This source review does **not** approve live execution of `--retained-f-audit` in the absence of the following concrete artifacts, which must be materialized and verified first:
     1. **Fresh source review receipts (`security` and `nonsecurity`)**: Materialized JSON receipts matching the `consent(c)` schema (lines `381–389`), bound to `runner_sha256 = 6fcfaffd73e98dcf9ce02a8151ab8dafa27d206f55572db6221dcd7a8df2dd9d`, `source_manifest_sha256 = 9f48d1f227c2714bab374828b7fe4dd930aab660ea63dd2685f91d6250bcafce`, `subject_manifest_sha256 = 68620a2a2462411050860503128193f32250f9165ce4ec80346da195cd4f157b`, and actual non-fabricated `terminal_sha256` / `report_sha256` digests from both fresh reviews (`R408` security and normal-source reviewer).
     2. **Refreshed `config_inputs` projection and receipt DAG (`root`, `tier1`, `tier2`)**: Exact projection JSON at `c['bindings']['config_inputs']` (reflecting the new `runner`, `paths.config`, and `6602` / `2582` wall caps) plus refreshed `root_admission`, `user_tier1_grant`, and `user_tier2_grant` JSON files required by `consent(c)` (lines `370–401`).
     3. **Final materialized `config.final.json` and external `<FRESH_REVIEW_BOUND_CONFIG_SHA256>`**: Written at `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/409-native06-retained-audit-config-proposal/config-proposal/config.final.json` and passed to `--config-sha256`.
     4. **Pre/Post Read-Only Inventories & Live Envelope Check**: Prospective NUL-delimited filetype/stat+SHA-256 inventories of the retained `target` and `packet` + `ONE_SHOT_STARTED` before and after the run, and live pre-run verification that `/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06` and the three `tmpfs` mounts (`mount_id` `190`, `191`, `240`) still match their recorded state.

