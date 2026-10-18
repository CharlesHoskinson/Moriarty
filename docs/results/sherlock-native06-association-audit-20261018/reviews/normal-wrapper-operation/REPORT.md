### 1. Inline Read Attestation & Reviewer Context

- **Reviewer Identity**: Same independent non-author `normal_source_reviewer` / `nonsecurity_source_reviewer` (`Gemini 4 Argon (High)` under the standing Opus 5.5 High quota-outage fallback; continued conversation `6cf4ae86-6a6b-470a-b942-a47168bc7606` in original `cwd`).
- **Inline Bodies Read (`19 / 19` complete, `0` missing or clipped)**:
  - `BODY 01` (`EXACT-FINAL-LAUNCH-ARGV.json`, `1855B`), `BODY 02` (`EXACT-FINAL-SHELL-BODY.txt`, `689B`), `BODY 03` (`actual-retained-f-audit-r2-01/RESULT.json`, `861B`), `BODY 04` (`EXACT-CORRECTED-LAUNCH-ARGV.proposed.json`, `2440B`), `BODY 05` (`LITERAL-OLD-NEW-RECONSTRUCTION.json`, `3182B`), `BODY 06` (`CURRENT-UNCHANGED-CONTEXT.json`, `3639B`), `BODY 07` (`actual-retained-audit-01/LAUNCH.json`, `702B`), `BODY 08` (`actual-retained-audit-01/RESULT.json`, `1152B`), `BODY 09` (`stdout.bin`, `0B`), `BODY 10` (`stderr.bin`, `117B`), `BODY 11` (`OUTER-WRAPPER-FAILURE-READBACK.json`, `1357B`), `BODY 12` (`ROOT-LIVE-PRECHECK.json`, `7874B`), `BODY 13` (`actual-sdkfree-successor-one-01/RESULT.json`, `2926B`), `BODY 14` (`actual-sdkfree-successor-one-01/stderr.bin`, `359B`), `BODY 15` (`security.source-receipt.json`, `868B`), `BODY 16` (`nonsecurity.source-receipt.json`, `868B`), `BODY 17` (`root.admission.proposed.json`, `658B`), `BODY 18` (`tier1.automation-compatibility.proposed.json`, `427B`), `BODY 19` (`tier2.automation-compatibility.proposed.json`, `521B`).
- **Tools / Execution**: Zero tools, file reads, commands, or edits invoked.

---

### 2. Independent Operation Verdicts

#### (1) Literal Reconstruction of the Prior R419 Supervisor-Entry Wrapper
- **Verdict**: **VERIFIED EXACT RECONSTRUCTION (NO PRIVILEGE OR SCOPE WIDENING)**.
- **Basis**:
  - Comparing `BODY 01`/`02` (R419) against `BODY 04`/`05` (proposed `r3`), the outer `argv` prefix `["/usr/bin/sudo", "-n", "--", "/bin/sh", "-eu", "-c", ...]` and `cwd` (`/home/charl/Moriarty/.worktrees/sherlock-a102-build-20261016`) are byte-for-byte identical.
  - Line 1 of the shell script (`echo $$ > /sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06/supervisor/cgroup.procs`) and the privilege-drop/environment-scrub prefix of Line 2 (`exec /usr/bin/setpriv --reuid=1000 --regid=1000 --clear-groups --inh-caps=-all --bounding-set=-all --no-new-privs -- /usr/bin/env -i LANG=C PATH=/usr/bin:/bin /usr/bin/python3 -I -B -S `) are byte-for-byte identical to R419.
  - Root privilege under `sudo -n` is used solely to write the shell's PID `$$` into `supervisor/cgroup.procs` before `exec /usr/bin/setpriv` replaces the shell process in-place as `uid=1000`, `gid=1000` with cleared supplementary groups, all inheritable/bounding capabilities dropped (`CapPrm=CapEff=CapAmb=0`), `no_new_privs` set, `env -i LANG=C PATH=/usr/bin:/bin`, and `python3 -I -B -S`, satisfying `supervisor_envelope(c)`.
  - Exactly four 1-for-1 literal substitutions occur between R419 and `r3` (`BODY 05`):
    1. Guard path: `.../417-native06-fixed-successor-guard-test-candidate/afterimages/bounded_sdk_compile.py` $\rightarrow$ `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/427-native06-r3-successor-source-candidate/afterimages/bounded_sdk_compile.py` (`89346B`, `446ebad9e776b1bc44717b23c0524056ed3b2a3637b768af6b513b6d06722125`).
    2. Route flag: `--retained-f-audit-r2` $\rightarrow$ `--retained-f-audit-r3`.
    3. Final config path: `.../417-native06-fixed-successor-guard-test-candidate/config-proposal/config.final.json` $\rightarrow$ `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/427-native06-r3-successor-source-candidate/config-proposal/config.final.json` (`510720B`, `cfc69d340dd68f09645a2741b73317da6ad9564235d1b1d5e9f48579102ed1ac`).
    4. External config SHA-256: `b0fe7a6065cf066d5dbda055ee3f670834883122bc9a6af18fb782b951a94167` $\rightarrow$ `cfc69d340dd68f09645a2741b73317da6ad9564235d1b1d5e9f48579102ed1ac`.
  - Historical note preserved: in R419 (`BODY 03`), this wrapper succeeded only in entering the guard supervisor; R419's `r2` audit itself exited `2` after `19.73010667700146`s (`nativeOutputsAccepted: false`).

#### (2) Eligibility of the Corrected Metadata-Only `r3` Command
- **Verdict**: **`ELIGIBLE_CONDITIONAL_ON_ROOT_FRESH_LIVE_PRECHECK`** (`EXACT-CORRECTED-LAUNCH-ARGV.proposed.json`, `BODY 04`). No blocker or further correction is required.
- **Basis**:
  - **Outer Shell Failure Preserved Without Guard Entry (`BODY 07`..`11`)**: The first R434 launch used the simplified unprivileged `sh -eu -c` command from owner prose and failed in `0.000865364`s at `sh: 1: cannot create /sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06/supervisor/cgroup.procs: Permission denied` before `&& exec /usr/bin/python3`. Python and the guard never started (`guardPythonStarted: false`, `SDKGuardAttemptConsumed: false`), all 3 cgroup `cpu.stat` files and empty `cgroup.procs` were untouched, all 23 predecessor files and `ONE_SHOT_STARTED` remained unchanged, and all 5 fresh `r3` output/failure/log names remained absent.
  - **Single New Synthetic Unit Passed (`BODY 13` & `14`)**: `test_retained_f_audit_r3_binds_consumed_r411_and_r419_artifacts_and_enforces_post_r419_bounds` ran in isolation and passed (`actualExit: 0`, `Ran 1 test in 0.042s / OK`, `sourcePinsUnchanged: true`, `oldSixMethodsReplayed: false`, `SDKNativeJobs: 0`).
  - **Complete Non-Circular Receipt DAG & Final Config (`BODY 15`..`19`)**:
    - `security.source-receipt.json` (`7458ec1e...`) records R428 (`eca860d5`, `APPROVED_SOURCE_WITH_LIMITS`, `terminal_sha256=50ff4f54...`, `report_sha256=d47b639a...`).
    - `nonsecurity.source-receipt.json` (`d1fe3d35...`) records R430 (`6cf4ae86`, `APPROVED_NORMAL_SOURCE`, `terminal_sha256=af122042...`, `report_sha256=10b4ac69...`).
    - Both bind `runner_sha256=446ebad9...`, `source_manifest_sha256=9ef3b9d5...`, and `subject_manifest_sha256=68620a2a...`.
    - `root.admission.proposed.json` (`4acc12aa...`), `tier1` (`4c9b7373...`), and `tier2` bind `config_inputs_sha256=caff8a6b833d8505864f889e11d7c5f6f963d36b41aed5bcff991f8dc051deac` and `amendments_sha256=3676da94b6ad533a2276271c55c3c47b8c4473170b1200cf0dcae2b362fbda5d`, producing `config.final.json` (`510720B`, `sha256=cfc69d340dd68f09645a2741b73317da6ad9564235d1b1d5e9f48579102ed1ac`).
  - **No Implied Acceptance**: Selecting and running this metadata-only `r3` command grants no Cargo/rustc/ELF execution, no retry on guard failure, no product/proof/financial/target acceptance (`native_outputs_accepted: False`), and no cleanup waiver.

#### (3) Applicable Resource Bounds, Read-Only Mounts, Output Exclusivity, and Check Scope Distinctions
- **Enforced Bounds & Live Cumulative Counters**:
  - **Wall Caps**: `total_wall_seconds = 6566`s (`6602 - 16 - 20`) and `limits.F.wall_seconds = 2546`s (`2582 - 16 - 20`), with prior `A0..E` wall (`39.198380454`s) and live supervisor entry overhead (`time.monotonic() - start`) dynamically deducted on `ITIMER_REAL`.
  - **Supervisor CPU**: `supervisor_cpu_seconds = 10800`s ceiling; live `/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06/supervisor/cpu.stat` enforced at `usage_usec >= 65462023`, setting `RLIMIT_CPU = 10800 - (sup_usec + 999999) // 1000000` (`10734`s at `65462023` us).
  - **Phase Cgroup CPU**: `limits.F.cpu_time_us = 10800000000` us ceiling; live `/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06/phases/cpu.stat` enforced at `usage_usec >= 257950306` before `group.create()`, and `prior_cpu + child_usage_usec <= 10800000000` enforced in `Group.check()`.
  - **Cumulative Phase `F` Logs**: `stdout_bytes <= 16777216`B (seeded at `0`B) and `stderr_bytes <= 16777216`B (seeded at `10406`B across `F_precheck`, `F`, `F_audit`, `F_retained_audit`, and `F_retained_audit_r2`).
- **Read-Only Target/Source & Output Exclusivity**:
  - Both `target` and `source_home` are mounted `--ro-bind` in `sandbox_args(..., retained_audit='r3')` and verified `ro` in `verify_worker_context`.
  - All 11 `forbidden` filenames in `packet/` (including the 5 fresh `r3` names `F_RETAINED_AUDIT_R3_RESULT.json`, `F_RETAINED_QUARANTINED_OUTPUTS_R3.json`, `TERMINAL_FAILURE_RETAINED_AUDIT_R3.json`, `F_retained_audit_r3.stdout`, `F_retained_audit_r3.stderr`) must be absent before launch and are created exclusively (`O_CREAT|O_EXCL`).
- **Distinction Between the Three Check Scopes**:
  1. **Dated Whole Inventory (`ROOT-LIVE-PRECHECK.json` at `2026-10-18T14:28:12Z`, `BODY 12`)**: Pre-R434 full inventory confirming `targetStatRows = 3971` (`allTargetStatsUnchanged: true`) and `allOldRegularHashes = 3548` across `target + packet` (`allOldRegularHashesUnchanged: true`).
  2. **Immediate Cheap Root Pre-Check Before Launch (`BODY 06` & `12`)**: Immediately prior to launching the corrected wrapper, Root re-verifies the 23 predecessor file digests in `packet/` (including `D_FREEZE.json`), `ONE_SHOT_STARTED` stat and SHA-256 (`37cb06c8f02da9953c59a7186a4bbdca294494149fdf65f831d8b493ae5bcf67`), the 3 cgroups (`/moriarty-sherlock-ledger9-20261018-06/{,supervisor,phases}`) having empty `cgroup.procs`, `populated == 0`, and `usage_usec` floors (`supervisor >= 65462023`, `phases >= 257950306`), and the absence of all 5 fresh `r3` filenames.
  3. **Unchanged Guard Full Source-Home Check (`frozen_source(c)`)**: The guard itself executes `frozen_source(c)` inside `retained_f_audit` (before `launch`) and again inside `output_check` (in the worker), walking and hashing the full `source_home` tree against `D_FREEZE.json['files']` (alongside `source_binding(c)` over `worktree`). No manual Root full-`source_home` hash check is claimed as a substitute for `frozen_source(c)`.

#### (4) Unchanged Original Source Approval & Scope Confirmation
- **Original Source Approval Unchanged**: My R430 `APPROVED_NORMAL_SOURCE` disposition over `bounded_sdk_compile.py` (`89346B`, `sha256=446ebad9e776b1bc44717b23c0524056ed3b2a3637b768af6b513b6d06722125`), `test_output_retained_accounting.py` (`42066B`, `sha256=20179cbfe437a8709df6093ce67c764b340f706d8475945cfe731ebee97d0580`), and `config-inputs.json` (`509178B`, `sha256=caff8a6b833d8505864f889e11d7c5f6f963d36b41aed5bcff991f8dc051deac`) remains **completely unchanged**.
- **Scope**: This follow-up disposition covers **solely the outer supervisor-entry wrapper correction** (`EXACT-CORRECTED-LAUNCH-ARGV.proposed.json`) restoring the R419 `sudo -n` + `setpriv` cgroup-entry wrapper with the four exact `r2` $\rightarrow$ `r3` path/flag/hash substitutions.

