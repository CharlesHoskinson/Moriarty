### Reviewer Identity & Execution Attestation

- **Conversation ID**: `bdec78a6-fbef-46dd-82a3-eb1c45e0c08c` (fresh context; no reuse of author CID `4e8031df-879c-41c4-95d2-622cd6b5df22` or prior review contexts)
- **Primary / Requested / Returned Identity**: Primary standing model `claude-opus-5-5`; requested and returned model **Gemini 4 Argon (High)** (`gemini-4-argon-high`, effort `high`, `standing_opus_quota_outage_fallback`)
- **Independence & Vote Count**: Single fresh non-author context supplying two typed source dispositions (`SECURITY_SOURCE` and `NORMAL_SOURCE`) plus two prospective operation classifications (`ORDINARY_TEST_OPERATION` and `DISTINCT_SUCCESSOR_DATA_AUDIT`). Because both source roles originate from one fresh context, they constitute **one independent substantive opinion** (not two independent reviewer votes).
- **Tool / Execution Attestation**: `0` tools, imports, file writes, or commands executed. All inline inputs (`COMPLETE-CURRENT-GUARD.py.txt` 84,442B SHA256 `3fa15e4be9cba248e51a193c9ebc3df755c2d23a440f36ef92fae76c521d1b5d`; `COMPLETE-CURRENT-OUTPUT-TEST-MODULE.py.txt` 26,445B SHA256 `7fc16940efb0e637bf47324fef8d60235084af1bb74b81edb50ae7e68df601ed`; and the 11 JSON/text evidence blocks) were read in full without clipping.

---

### 1. `SECURITY_SOURCE` Decision: `APPROVED_SOURCE_WITH_LIMITS`

#### A. Fixed `'r2'` Route, CLI/Context/Worker/Suffix/Predecessor/Receipt Consistency, and Exclusivity
- **Fixed route domain (not a general retry parameter)**:
  - In `main`:
    - `parser.add_argument('--retained-f-audit',action='store_true')`
    - `parser.add_argument('--retained-f-audit-r2',action='store_true')`
    - `need(not (a.retained_f_audit and a.retained_f_audit_r2),'conflicting retained audit flags')`
    - `retained_mode='r2' if a.retained_f_audit_r2 else a.retained_f_audit`
  - Every downstream entry point restricts `retained_audit` to the closed domain `(False, True, 'r2')` (or `(True, 'r2')` in `retained_f_audit`):
    - `sandbox_args`: `need(retained_audit in (False,True,'r2') and (not retained_audit or (phase=='F' and worker and not precheck)),'retained audit sandbox mismatch')`
    - `prior_receipts`: `need(retained_audit in (False,True,'r2'),'invalid retained audit route')`
    - `launch`: `need(retained_audit in (False,True,'r2') and (not retained_audit or phase=='F'),'retained audit only supports phase F')`
    - `worker`: `need(retained_audit in (False,True,'r2'),'invalid retained audit route')`
    - `retained_f_audit`: `need(retained_audit in (True,'r2'),'invalid retained audit route')`
- **Supervisor-to-worker route and predecessor consistency**:
  - In `launch`:
    - `variants=[(True,False)] if retained_audit else [(False,False)] if phase in ('B','E','F') else [(True,False)]`
    - `args=sandbox_args(c,phase,audit,precheck,retained_audit);suffix='_retained_audit_r2' if retained_audit=='r2' else '_retained_audit' if retained_audit else '_precheck' if precheck else '_audit' if audit and phase in ('E','F') else ''`
    - `'predecessors':prior_receipts(c,phase,precheck,retained_audit)`
  - In `sandbox_args`:
    - `if retained_audit=='r2':args+=['--retained-f-audit-r2']`
    - `elif retained_audit:args+=['--retained-f-audit']`
  - In `main` (worker branch) and `verify_worker_context`:
    - `need(not retained_mode or (a.worker=='F' and not a.precheck),'invalid retained audit worker flags')`
    - `verify_worker_context(c,a.worker,a.precheck,a.worker_fd,a.config_sha256,retained_mode)`
    - `need(context['predecessors']==prior_receipts(c,phase,precheck,retained_audit),'worker predecessor receipts differ')`
  - Because `prior_receipts(c,'F',False,retained_audit)` returns 10 keys for `False`, 15 keys for `True`, and 19 keys for `'r2'`, any flag mismatch between supervisor context creation and worker verification fails closed on `context['predecessors']` before nonce consumption or `worker(c,a.worker,retained_mode)` dispatch.
- **Consumed R411 digest binding and 15+4 predecessor preservation**:
  - Module constant `R411_CONSUMED_PINS`:
    - `'F_retained_audit.stdout': 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'`
    - `'F_retained_audit.stderr': '78ee1269c2a286f0a28e4c657322649ecb2934f91f70741881067efe6dabeac0'`
    - `'TERMINAL_FAILURE_RETAINED_AUDIT.json': '7d37ba6e226a0852d1b6a67314c6db80acd42a66225fa78782d1cbd321208c09'`
    - `'USED_LAUNCH_93645c50dfc6c31930def6df095941fec5f41243bede003b450fe0c3db0cf5f3': '6996692eb1c89d9bcf83ac52ecd63d76df3902b6bb6bc3978a353431fbfef8c8'`
  - These 4 SHA-256 values match the post-R411 artifacts in `ROOT-POST-R411-CGROUP-READBACK.json` and `R411-ACTUAL-POST-COMPARISON.json`.
  - In `prior_receipts`:
    - `names+=['F_precheck.stdout','F_precheck.stderr','F_audit.stdout','F_audit.stderr','TERMINAL_FAILURE.json']`
    - `if retained_audit=='r2':names+=list(R411_CONSUMED_PINS)`
  - In `retained_f_audit`:
    - `expected_failure={'message':'phase exit nonzero F','native_outputs_accepted':False,'type':'Refusal'}`
    - `need(jload(packet/'TERMINAL_FAILURE.json')==expected_failure,'unexpected prior terminal failure')`
    - `if retained_audit=='r2':`
    - `need(jload(packet/'TERMINAL_FAILURE_RETAINED_AUDIT.json')==expected_failure,'unexpected prior retained audit failure')`
    - `need(all(predecessors.get(k)==v for k,v in R411_CONSUMED_PINS.items()),'consumed R411 predecessor digest mismatch')`
- **Exclusive new outputs and permanently consumed R411 route**:
  - In `retained_f_audit`:
    - `forbidden=('F_RESULT.json','F_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_R2_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS_R2.json','TERMINAL_FAILURE_RETAINED_AUDIT_R2.json','F_retained_audit_r2.stdout','F_retained_audit_r2.stderr') if retained_audit=='r2' else ('F_RESULT.json','F_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS.json','TERMINAL_FAILURE_RETAINED_AUDIT.json','F_retained_audit.stdout','F_retained_audit.stderr')`
    - `for name in forbidden:`
    - `need(not (packet/name).exists(),'retained audit output already exists: '+name)`
    - `out_name='F_RETAINED_QUARANTINED_OUTPUTS_R2.json' if retained_audit=='r2' else 'F_RETAINED_QUARANTINED_OUTPUTS.json'`
    - `res_name='F_RETAINED_AUDIT_R2_RESULT.json' if retained_audit=='r2' else 'F_RETAINED_AUDIT_RESULT.json'`
    - `fail_name='TERMINAL_FAILURE_RETAINED_AUDIT_R2.json' if retained_audit=='r2' else 'TERMINAL_FAILURE_RETAINED_AUDIT.json'`
  - In `worker`:
    - `elif phase=='F':output_check(c,'F_RETAINED_QUARANTINED_OUTPUTS_R2.json' if retained_audit=='r2' else 'F_RETAINED_QUARANTINED_OUTPUTS.json' if retained_audit else 'F_QUARANTINED_OUTPUTS.json')`
  - In `launch`:
    - `for name in ('stdout','stderr'):logs.append(open(packet/(phase+suffix+'.'+name),'xb',buffering=0))`
  - All receipts (`write_new` with `os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW`) and logs (`'xb'`) fail closed if any target file exists, preserving old artifacts untouched and making `'r2'` strictly one-shot.

#### B. Raw Source, Target Read-Only Mount, Freeze, and Observer Contracts
- **Read-only `target` and `source_home` mounts in sandbox and worker mount verification**:
  - Since `'r2'` is truthy, both `sandbox_args` and `child_mounts` enforce read-only mounts for `target` and `source_home`:
    - `sandbox_args`: `args+=['--bind' if phase in ('A','B','D') else '--ro-bind',p['source_home'],p['source_home']]` and `if phase=='F':args+=['--ro-bind' if retained_audit else '--bind',p['target'],p['target']]`
    - `child_mounts`: `p['source_home']:'rw' if phase in ('A','B','D') else 'ro'` and `if phase=='F':wanted[p['target']]='ro' if retained_audit else 'rw'`
    - `verify_worker_context`: `need(context['mounts']==child_mounts(c,phase,retained_audit),'worker mount contract differs')` and checks `/proc/self/mountinfo` options (`for path,access in context['mounts'].items():need(path in mounts and access in mounts[path],'worker mount mode differs: '+path)`).
- **Source/freeze/dep-info/output observer integrity**:
  - `retained_f_audit` runs `bound_identities(c);owned_paths(c);quotas(c,'F');frozen_source(c)` before `launch`, and `bound_identities(c)` again after `launch`.
  - `output_check` runs `frozen=frozen_source(c)['files']`, parses every `*.d` file via `parse_depinfo(p.read_text(),worktree,dep_roots)` and `depinfo_path` (unchanged from R415), checks all worktree/source-home/SDK-include/host-unit/fingerprint/OUT_DIR invariants, and records `'acceptance':'UNACCEPTED: independent result review required; no artifact executed'`.

#### C. Resource Reductions (`6586` / `2566`), Live CPU Floors, Cumulative Ceilings, and Logs
- **Wall-clock ceiling reductions (`6586s` / `2566s`) and entry-deadline accounting**:
  - R411 consumed `15.664219678998052s` (`ceil = 16s`); deducting `16s` from the prior `6602s` / `2582s` bounds (or `ceil(117.9653273010044359 + 15.664219678998052) = 134s` from original `6720s` / `2700s`) yields `6586s` and `2566s`.
  - In `retained_f_audit`:
    - `start=time.monotonic()`
    - `if retained_audit=='r2':`
    - `need(c['total_wall_seconds']<=6586 and c['limits']['F']['wall_seconds']<=2566,'post-R411 wall ceiling')`
    - `init_wall=min(c['total_wall_seconds'],c['limits']['F']['wall_seconds'])`
    - `prior_wall=sum(jload(packet/(ph+'_RESULT.json'))['wall_seconds'] for ph in PHASES[:-1])`
    - `wall_budget=min(c['total_wall_seconds']-prior_wall,c['limits']['F']['wall_seconds'])`
    - `rem_wall=wall_budget-(time.monotonic()-start)`
    - `need(rem_wall>0,'whole-job wall cap');signal.setitimer(signal.ITIMER_REAL,rem_wall)`
    - Post-`launch`: `need(time.monotonic()-start<=wall_budget,'whole-job wall cap')`
- **Live supervisor CPU floor (`56990698` usec) and cumulative ceiling deduction**:
  - In `retained_f_audit`:
    - `sup_usec=read_numbers(cg_fs(cg_paths(c)[1])/'cpu.stat').get('usage_usec',0)`
    - `if retained_audit=='r2':need(sup_usec>=56990698,'supervisor CPU counter reset')`
    - `sup_cpu=c['supervisor_cpu_seconds']-(sup_usec+999999)//1000000`
    - `need(sup_cpu>0,'supervisor CPU cap');resource.setrlimit(resource.RLIMIT_CPU,(sup_cpu,sup_cpu))`
- **Live phases CPU floor (`246681730` usec) and cumulative ceiling enforcement**:
  - In `launch`:
    - `group.prior_cpu=read_numbers(group.path.parent/'cpu.stat').get('usage_usec',0)`
    - `if retained_audit=='r2':need(group.prior_cpu>=246681730,'phase CPU counter reset')`
    - `need(group.prior_cpu<v['cpu_time_us'],'total cgroup CPU cap')`
  - In `Group.check`:
    - `need(getattr(self,'prior_cpu',0)+read_numbers(self.path/'cpu.stat').get('usage_usec',0)<=self.v['cpu_time_us'],'total cgroup CPU cap')`
- **Cumulative Phase F stdout/stderr accounting including R411 logs**:
  - In `launch`:
    - `prior_logs=('_precheck','','_audit','_retained_audit') if retained_audit=='r2' else ('_precheck','','_audit')`
    - `start=time.monotonic();counts={k:sum((packet/f'F{s}.{k}').stat().st_size for s in prior_logs) for k in ('stdout','stderr')} if retained_audit else {'stdout':0,'stderr':0};records=[]`
    - `for k,n in counts.items():need(n<=v[k+'_bytes'],'log cap hit '+phase+' '+k)`
  - Prior stderr across `F_precheck.stderr` (0B), `F.stderr` (10,237B), `F_audit.stderr` (47B), and `F_retained_audit.stderr` (47B) is initialized to `10,331` bytes and charged against `limits.F.stderr_bytes` (`16,777,216`B).

#### D. Test Fixture Audit (`COMPLETE-CURRENT-OUTPUT-TEST-MODULE.py.txt`)
- In `test_retained_f_audit_r2_binds_consumed_r411_artifacts_and_enforces_post_r411_bounds`:
  - Materializes the 4 consumed R411 files in `self.packet` with exact byte contents whose SHA-256 digests match `R411_CONSUMED_PINS`:
    - `_write(self.packet / 'F_retained_audit.stdout', b'')`
    - `_write(self.packet / 'F_retained_audit.stderr', b'TERMINAL_REFUSAL: non-normalized dep-info path\n')`
    - `_write(self.packet / 'TERMINAL_FAILURE_RETAINED_AUDIT.json', runner.json_bytes({'message': 'phase exit nonzero F', 'native_outputs_accepted': False, 'type': 'Refusal'}))`
    - `_write(self.packet / 'USED_LAUNCH_93645c50dfc6c31930def6df095941fec5f41243bede003b450fe0c3db0cf5f3', b'consumed\n')`
  - Verifies `sandbox_args(..., retained_audit='r2')`, `prior_receipts(..., retained_audit='r2')` (19 keys), and `worker(self.c, 'F', retained_audit='r2')` (`F_RETAINED_QUARANTINED_OUTPUTS_R2.json` written by real `output_check` and then unlinked before supervisor test).
  - Verifies first-refusal ordering in `launch(..., retained_audit='r2')` before `group.create()`: cumulative stderr `10331 > 10300` (`'log cap hit F stderr'`) and reset phase CPU `240632042 < 246681730` (`'phase CPU counter reset'`).
  - Verifies first-refusal ordering in `retained_f_audit(..., retained_audit='r2')`: unreduced wall cap `2582 > 2566` (`'post-R411 wall ceiling'`) and reset supervisor CPU `48374910 < 56990698` (`'supervisor CPU counter reset'`).
  - Verifies `'r2'` success path: timer sequence `[2566, 2562.0, 0]`, `RLIMIT_CPU` `(10800 - 57, 10800 - 57)`, and `F_RETAINED_AUDIT_R2_RESULT.json` fields (`res['retained_audit'] == 'r2'`, `res['native_outputs_accepted'] is False`, 19 predecessors).
- **Mandatory source fixes required**: **NONE**.
- **Optional non-blocking notes**:
  1. `R411_CONSUMED_PINS` digest values are verified in `retained_f_audit` (`need(all(predecessors.get(k)==v for k,v in R411_CONSUMED_PINS.items()),'consumed R411 predecessor digest mismatch')`) and bound into the sealed worker context checked by `verify_worker_context` (`need(context['predecessors']==prior_receipts(c,phase,precheck,retained_audit),'worker predecessor receipts differ')`); checking `R411_CONSUMED_PINS` directly inside `prior_receipts` would be redundant under supervisor launch and is optional.
  2. The new unit test covers the positive `R411_CONSUMED_PINS` binding on `'r2'` without a separate negative sub-assertion mutating one of the 4 pinned files; existing `test_retained_f_audit_enforces_entry_deadline_cpu_deduction_exclusivity_and_qualified_receipt` already tests tampered `TERMINAL_FAILURE.json` refusal and output-exclusivity refusal.

---

### 2. `NORMAL_SOURCE` Decision: `APPROVED_NORMAL_SOURCE`

- **Ordinary-mode (`retained_audit=False`) and initial retained-audit (`retained_audit=True`) compatibility**:
  - Default parameter values remain `retained_audit=False` in `sandbox_args`, `prior_receipts`, `child_mounts`, `verify_worker_context`, `launch`, and `worker`, and `retained_audit=True` in `retained_f_audit(c,config_sha,retained_audit=True)`.
  - In `main`:
    - When neither `--retained-f-audit` nor `--retained-f-audit-r2` is passed, `retained_mode` is `False`, routing supervisor execution to `run(c,a.config_sha256)` and worker execution to `worker(c,a.worker,retained_mode)` with unchanged phase A0–F behavior, mounts, receipts (`F_QUARANTINED_OUTPUTS.json`), and resource limits.
    - When `--retained-f-audit` is passed (`retained_mode=True`), behavior is byte-for-byte equivalent to the prior guard (`15` predecessors, `prior_logs=('_precheck','','_audit')`, no `'r2'` floor/ceiling branches, `F_RETAINED_QUARANTINED_OUTPUTS.json` / `F_RETAINED_AUDIT_RESULT.json`), so all 4 existing tests in `OutputAndRetainedAccountingTests` remain intact and unmodified.
- **Mandatory normal-source fixes required**: **NONE**.

---

### 3. `ORDINARY_TEST_OPERATION` Classification: `ELIGIBLE` (With Exact Conditions)

- **Scope classified**: Only the single SDK-free synthetic unittest command in `ONE-NEW-SYNTHETIC-METHOD.proposed.json` (SHA256 `3e6998df86a61d8bb9ec55e4657e6aedbde4b01d2a5f2fa681ba5bfe89e28f0c`):
  - Selector: `test_output_retained_accounting.OutputAndRetainedAccountingTests.test_retained_f_audit_r2_binds_consumed_r411_artifacts_and_enforces_post_r411_bounds` (`suite.countTestCases() == 1`).
  - Do **not** rerun the full 5-test class, prior accounting suites, or R415 parser suites.
- **Verification of mocked boundaries and synthetic-only effects**:
  - `setUp` operates strictly inside `tempfile.TemporaryDirectory()` (`self._tmp`) and cleans up in `tearDown`.
  - `sandbox_args` is called with `patch.object(runner, 'owned_paths')` and only returns an `argv` list.
  - `prior_receipts` and `worker(self.c, 'F', retained_audit='r2')` read/write only synthetic files inside `self.base`.
  - Both `launch(..., retained_audit='r2')` sub-checks raise `runner.Refusal` *before* `group.create()` (with `supervisor_envelope`, `bound_identities`, `quotas`, and `_cg_text` patched), so no cgroup directory creation, `bwrap` spawn, or socketpair/seccomp handoff occurs.
  - All `retained_f_audit(..., retained_audit='r2')` sub-checks either refuse before `launch` or patch `runner.launch` with `_fake_launch_r2` alongside patched `time.monotonic`, `signal.signal`, `signal.setitimer`, `resource.setrlimit`, `_cg_text`, `consent`, `supervisor_envelope`, `bound_identities`, `owned_paths`, and `quotas`.
  - Zero access to live `/sys/fs/cgroup`, native06 `target`/`source-home`/`packet`, Cargo, or SDK binaries.
- **Exact conditions for `ELIGIBLE` execution**:
  1. Stage only the two exact pinned files (`bounded_sdk_compile.py` SHA256 `3fa15e4be9cba248e51a193c9ebc3df755c2d23a440f36ef92fae76c521d1b5d` and `test_output_retained_accounting.py` SHA256 `7fc16940efb0e637bf47324fef8d60235084af1bb74b81edb50ae7e68df601ed`) into the isolated directory `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/418-native06-fixed-successor-fresh-review/prospective-isolated-successor-one-suite`.
  2. Run under the exact `timeout` (30s KILL) and `prlimit` (`--cpu=25:25`, `--as=1073741824:1073741824`, `--fsize=1048576:1048576`, `--core=0:0`), unprivileged `uid=1000`/`gid=1000`, isolated `env`, and 131,072B capture cap with `no_auto_retry: true`.
  3. Verify pre/post source and staged pins remain unchanged.

---

### 4. `DISTINCT_SUCCESSOR_DATA_AUDIT` Assessment: `CONDITIONAL`

- **Capsule assessed**: `CONCRETE-FIXED-SUCCESSOR-OPERATION-CAPSULE.proposed.json` (SHA256 `1c9e3ba009b921f208268b2c09867c35dfca6794a366a2d45c56695729f7aeea`) and `CONFIG-EXACT-DELTA-AND-STALE-DAG.json` (SHA256 `765358097bc024dbe99a41a48078db64c14399445a7fd3f4e4ce0cc42e44cb2b`).
- **Mechanical nature of remaining final config digest slot**:
  - In `consent(c)`:
    - `projection=copy.deepcopy(c);projection.pop('review_receipts');projection['bindings'].pop('config_inputs')`
    - `check_pin(c['bindings']['config_inputs'])`
    - `need(jload(c['bindings']['config_inputs']['path'])==projection,'configuration projection differs')`
  - Because `config-inputs.json` (`520967e37b03e9d7647ad52d566ea62fdea19c9221f15f157c29503634a6d6ea`, 678,227 bytes) already fixes every operational field—including the 5 operational deltas (`bindings.runner.path`, `bindings.runner.sha256` = `3fa15e4b...`, `limits.F.wall_seconds` = `2566`, `paths.config`, `total_wall_seconds` = `6586`) while holding all other projection values identical to R409—populating `review_receipts` (`security`, `nonsecurity`, `root`, `tier1`, `tier2`) after real review/Root/tier dispositions and substituting the resulting `config.final.json` SHA-256 into `FINAL_CONFIG_SHA256_AFTER_ACTUAL_REVIEW_DAG` is **strictly mechanical with zero semantic delta**, provided `jload(c['bindings']['config_inputs']['path'])==projection` holds for the exact pinned `520967e3...` projection.
- **Explicit mandatory conditions before any `--retained-f-audit-r2` execution**:
  1. **Source & resource review completion**: Required independent source/resource review dispositions completed and recorded honestly (counting this fresh context as one independent opinion alongside `author4e803`, plus any required second independent reviewer per active routing).
  2. **Actual new unit test PASS**: The single synthetic test (`ORDINARY_TEST_OPERATION` above) must first be executed and pass (`actual_exit == 0`, 1 test OK) on the exact `3fa15e4b...` guard and `7fc16940...` test module.
  3. **Fresh receipt DAG & mechanical config finalization**: Root materializes fresh `security`, `nonsecurity`, `root`, `tier1`, and `tier2` receipt JSONs binding `runner_sha256` (`3fa15e4b...`) and `config_inputs_sha256` (`520967e3...`), writes `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/417-native06-fixed-successor-guard-test-candidate/config-proposal/config.final.json` without altering the `520967e3...` projection, and substitutes its exact SHA-256 into `FINAL_CONFIG_SHA256_AFTER_ACTUAL_REVIEW_DAG`.
  4. **Pre-execution live cgroup & counter verification**: Verify `/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06` (`envelope`, `supervisor`, `phases`) has `cgroup.procs` empty (`populated 0`), `supervisor/cpu.stat` `usage_usec >= 56990698`, and `phases/cpu.stat` `usage_usec >= 246681730`, with no counter/log/marker resets.
  5. **Pre- and post-execution inventories**: Capture read-only stat/membership/SHA-256 inventories of `target`, `packet` (including the 15 original + 4 R411 consumed predecessors), and `ONE_SHOT_STARTED` immediately before and after the run, verifying that `target` (3,971 stat records / 3,540 regular files), `ONE_SHOT_STARTED`, and all 46 preexisting `packet` files remain unchanged and only the expected `_r2` outputs plus one new `USED_LAUNCH_<nonce>` marker are added. (This inventory proves pre/post membership/stat/hash invariance, not an exclusive-read attestation or retrospective guarantee since original compile.)
  6. **Quarantine & cleanup obligations**: Fail closed with no automatic retry; any resulting `F_RETAINED_QUARANTINED_OUTPUTS_R2.json` and `F_RETAINED_AUDIT_R2_RESULT.json` remain `native_outputs_accepted: False` (`UNACCEPTED`) pending independent result review; runtime cleanup remains owed; Amendment 10 / T3 remain stopped.

