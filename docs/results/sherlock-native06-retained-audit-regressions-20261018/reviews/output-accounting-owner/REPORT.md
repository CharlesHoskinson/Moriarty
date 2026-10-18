### R404 — Designated Security Source Owner Accounting Reconciliation, Guard Edits & Synthetic Regression Suite

---

### 1. Reconciled Resource Accounting, Config Bindings & Exact Guard Edits

#### 1.1 Reconciliation of Peer R399 Report vs. Authoritative Root Cgroup Readback & Receipt Counters

1. **Recorded vs. Unrecorded Native 06 Resource Facts**:
   - **Directly Measured in `packet/` Receipts & Logs**:
     - Prior phase wall times (`A0_RESULT.json`..`E_RESULT.json`):
       $$2.6567903329996625 + 2.78098472900092 + 1.7633520050003426 + 12.067841101998056 + 7.613954207998177 + 12.315458076998766 = 39.1983804539959241\text{ s}.$$
     - Outer wrapper elapsed time (`RESULT.json`, `3732f46e...`): `157.16370775500036` s.
     - Prior Phase F log bytes: `F_precheck.stdout = 0` B, `F_precheck.stderr = 0` B, `F.stdout = 0` B, `F.stderr = 10,237` B, `F_audit.stdout = 0` B, `F_audit.stderr = 47` B (cumulative `stdout = 0` B, `stderr = 10,284` B against `16,777,216` B caps).
   - **Directly Measured in Kernel Cgroup v2 Readback (`ROOT-CGROUP-READBACK-20261018.json`, sampled `2026-10-18T10:34:24.614823+00:00`)**:
     - Envelope (`/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06/cpu.stat`): `usage_usec = 289,006,953` (`user_usec = 235,579,044`, `system_usec = 53,427,909`), `cgroup.procs` empty, `cgroup.events` `populated = 0`.
     - Supervisor leaf (`.../supervisor/cpu.stat`): `usage_usec = 48,374,910` (`48.374910` CPU-s; `ceil = 49` s), `cgroup.procs` empty.
     - Phase parent (`.../phases/cpu.stat`): `usage_usec = 240,632,042` (`240.632042` CPU-s aggregate across all phase leaves `sdk-once-A0`..`sdk-once-F`; `48,374,910 + 240,632,042 = 289,006,952` usec), `cgroup.procs` empty.
   - **Unrecorded Quantities & Residual Scope**:
     - Because Native 06 refused inside `F_audit` (`non-normalized dep-info path`), `F_RESULT.json` was not written and `Group.close()` removed `/sys/fs/cgroup/.../phases/sdk-once-F`. Thus, Phase F-only `wall_seconds` and Phase F-only `usage_usec` were not separately recorded.
     - The difference `157.16370775500036 - 39.1983804539959241 = 117.9653273010044359` s (`ceil = 118` s) is a **residual upper bound** covering Phase F execution plus supervisor startup/inter-phase/teardown overhead, not a directly measured Phase F duration.
     - Similarly, `phases/cpu.stat` (`240,632,042` usec) is the **measured aggregate CPU of all phases `A0..F`**, which is a strict upper bound on Phase F's actual CPU consumption ($\text{CPU}_{\text{F}} \le 240,632,042\ \mu\text{s}$).

2. **Correction of Peer R399 CPU Unit-Conversion Error**:
   - Peer R399 deducted `ceil(117.965328 * 400000) = 47,186,132` usec (`10,800,000,000 - 47,186,132 = 10,752,813,868` usec). That multiplied seconds (`s`) by `cpu_quota_us` (`400,000` usec per `100,000` usec period) without dividing by `cpu_period_us = 100,000` usec and multiplying by `1,000,000` usec/s (off by $10\times$).
   - Because `cpu_quota_us / cpu_period_us = 400,000 / 100,000 = 4` cores (`4,000,000` usec of CPU per wall second), the dimensionally converted 4-core quota product over the residual `117.9653273010044359` s is:
     $$\lceil 117.9653273010044359 \times 4 \times 1,000,000 \rceil = 471,861,310\ \mu\text{s}$$
     (`471,861,312` usec at `117.965328` s), leaving `10,800,000,000 - 471,861,312 = 10,328,138,688` usec.
   - Using the actual kernel-measured `phases/cpu.stat` aggregate (`240,632,042` usec) yields `10,800,000,000 - 240,632,042 = 10,559,367,958` usec (or `10,800,000,000 - 289,006,953 = 10,510,993,047` usec using full `envelope/cpu.stat`).

3. **Reconciling In-Code Accounting, Timer Entry Overhead, and Config Ceiling Bindings**:
   - **Entry Elapsed Deadline (`retained_f_audit`)**: Recording `start = time.monotonic()` at `retained_f_audit` entry and setting `rem_wall = wall_budget - (time.monotonic() - start)` after `bound_identities(c)`, `owned_paths(c)`, `quotas(c, 'F')`, `frozen_source(c)`, and `prior_receipts(...)` charges all entry verification work against `wall_budget` and never extends the initial `ITIMER_REAL` deadline (`start + wall_budget <= start + init_wall`). A `finally: signal.setitimer(signal.ITIMER_REAL, 0)` block ensures no live timer leaks on any exit path, and `need(time.monotonic() - start <= wall_budget, 'whole-job wall cap')` enforces the deadline after `launch` even if `SIGALRM` is mocked in tests.
   - **Live Cgroup CPU + Config Scoping**:
     - In `retained_f_audit`, `sup_cpu = c['supervisor_cpu_seconds'] - (read_numbers(cg_fs(cg_paths(c)[1])/'cpu.stat').get('usage_usec', 0) + 999999) // 1000000` deducts live `supervisor/cpu.stat` usage before `setrlimit(RLIMIT_CPU, (sup_cpu, sup_cpu))` and refuses (`'supervisor CPU cap'`) if `sup_cpu <= 0`.
     - In `launch(..., retained_audit=True)` and `Group.check()`, `group.prior_cpu = read_numbers(group.path.parent/'cpu.stat').get('usage_usec', 0)` reads the phase-parent aggregate (`phases/cpu.stat`) before `group.create()`, refuses immediately if `group.prior_cpu >= v['cpu_time_us']`, and enforces `group.prior_cpu + usage_usec <= v['cpu_time_us']` (`'total cgroup CPU cap'`) throughout execution.
   - **Reconciling `prior_wall` & CPU Config Ceilings (Avoiding Unintended Double-Count vs. Fresh-Cgroup Reset)**:
     - Because `retained_f_audit` subtracts `prior_wall` (`39.1983804539959241` s from `A0_RESULT..E_RESULT`) in code, deducting the unreceipted residual `ceil(117.9653273010044359) = 118` s in config (`total_wall_seconds <= 6602`, `limits.F.wall_seconds <= 2582`) yields an exact non-double-counted whole-job remaining cap of `6602 - 39.1983804539959241 = 6562.801619546004` s. If config also deducts `prior_wall` (`total_wall_seconds <= 6562`, `supervisor_cpu_seconds <= 10642` [or `<= 10751` measured], `limits.F.wall_seconds <= 2582` [or `<= 2542` full outer], `limits.F.cpu_time_us <= 10328138688` [4-core residual quota bound] or `<= 10559367958` [measured `phases/cpu.stat` aggregate]), `limits.F.wall_seconds <= 2582` s remains the binding wall limit (`2582 < 6522.80`) and the reduced config ceilings prevent any reset even if a fresh process/cgroup had `0` prior `cpu.stat`.

#### 1.2 Exact Guard Edits Against `bounded_sdk_compile.py` (`79,818B`, `de3cb754...`)

```json
{
  "target_file": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/398-native06-retained-audit-owner-candidate/afterimages/bounded_sdk_compile.py",
  "target_sha256": "de3cb754f3483e5100faf1a7313ae08e5dc2ac4dae27ee1d79a97910f4380a53",
  "exact_edits": [
    {
      "file": "bounded_sdk_compile.py",
      "old": "def sandbox_args(c,phase,worker=True,precheck=False,retained_audit=False):\n    owned_paths(c)",
      "new": "def sandbox_args(c,phase,worker=True,precheck=False,retained_audit=False):\n    need(not retained_audit or (phase=='F' and worker and not precheck),'retained audit sandbox mismatch')\n    owned_paths(c)"
    },
    {
      "file": "bounded_sdk_compile.py",
      "old": "    def check(self):\n        need(read_numbers(self.path/'cpu.stat').get('usage_usec',0)<=self.v['cpu_time_us'],'total cgroup CPU cap')",
      "new": "    def check(self):\n        need(getattr(self,'prior_cpu',0)+read_numbers(self.path/'cpu.stat').get('usage_usec',0)<=self.v['cpu_time_us'],'total cgroup CPU cap')"
    },
    {
      "file": "bounded_sdk_compile.py",
      "old": "    start=time.monotonic();counts={'stdout':0,'stderr':0};records=[]\n    try:\n        group.create()",
      "new": "    start=time.monotonic();counts={k:sum((packet/f'F{s}.{k}').stat().st_size for s in ('_precheck','','_audit')) for k in ('stdout','stderr')} if retained_audit else {'stdout':0,'stderr':0};records=[]\n    for k,n in counts.items():need(n<=v[k+'_bytes'],'log cap hit '+phase+' '+k)\n    if retained_audit:\n        group.prior_cpu=read_numbers(group.path.parent/'cpu.stat').get('usage_usec',0)\n        need(group.prior_cpu<v['cpu_time_us'],'total cgroup CPU cap')\n    try:\n        group.create()"
    },
    {
      "file": "bounded_sdk_compile.py",
      "old": "def retained_f_audit(c,config_sha):\n    need(c['mode']=='tier1-and-compile','retained F audit requires tier1-and-compile mode')\n    consent(c);supervisor_envelope(c)\n    def expired(signum,frame):raise Refusal('whole-job wall cap')\n    signal.signal(signal.SIGALRM,expired);signal.setitimer(signal.ITIMER_REAL,min(c['total_wall_seconds'],c['limits']['F']['wall_seconds']))\n    resource.setrlimit(resource.RLIMIT_CPU,(c['supervisor_cpu_seconds'],c['supervisor_cpu_seconds']))\n    bound_identities(c);owned_paths(c);quotas(c,'F');frozen_source(c)\n    p=c['paths'];root=Path(p['execution_root']);packet=Path(p['packet'])\n    need((root/'ONE_SHOT_STARTED').is_file(),'one-shot start missing')\n    prior_receipts(c,'F',False,retained_audit=True)\n    for name in ('F_RESULT.json','F_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS.json','TERMINAL_FAILURE_RETAINED_AUDIT.json','F_retained_audit.stdout','F_retained_audit.stderr'):\n        need(not (packet/name).exists(),'retained audit output already exists: '+name)\n    try:\n        result=launch(c,'F',config_sha,retained_audit=True)\n        bound_identities(c)\n        receipt(c,'F_RETAINED_AUDIT_RESULT.json',result,'F')\n    except BaseException as e:\n        signal.setitimer(signal.ITIMER_REAL,0)\n        try:receipt(c,'TERMINAL_FAILURE_RETAINED_AUDIT.json',{'type':type(e).__name__,'message':str(e),'native_outputs_accepted':False},'F')\n        except Exception:pass\n        raise",
      "new": "def retained_f_audit(c,config_sha):\n    start=time.monotonic()\n    need(c['mode']=='tier1-and-compile','retained F audit requires tier1-and-compile mode')\n    consent(c);supervisor_envelope(c)\n    def expired(signum,frame):raise Refusal('whole-job wall cap')\n    init_wall=min(c['total_wall_seconds'],c['limits']['F']['wall_seconds'])\n    need(init_wall>0,'whole-job wall cap')\n    signal.signal(signal.SIGALRM,expired);signal.setitimer(signal.ITIMER_REAL,init_wall)\n    try:\n        sup_cpu=c['supervisor_cpu_seconds']-(read_numbers(cg_fs(cg_paths(c)[1])/'cpu.stat').get('usage_usec',0)+999999)//1000000\n        need(sup_cpu>0,'supervisor CPU cap');resource.setrlimit(resource.RLIMIT_CPU,(sup_cpu,sup_cpu))\n        bound_identities(c);owned_paths(c);quotas(c,'F');frozen_source(c)\n        p=c['paths'];root=Path(p['execution_root']);packet=Path(p['packet'])\n        need((root/'ONE_SHOT_STARTED').is_file(),'one-shot start missing')\n        predecessors=prior_receipts(c,'F',False,retained_audit=True)\n        need(jload(packet/'TERMINAL_FAILURE.json')=={'message':'phase exit nonzero F','native_outputs_accepted':False,'type':'Refusal'},'unexpected prior terminal failure')\n        prior_wall=sum(jload(packet/(ph+'_RESULT.json'))['wall_seconds'] for ph in PHASES[:-1])\n        wall_budget=min(c['total_wall_seconds']-prior_wall,c['limits']['F']['wall_seconds'])\n        rem_wall=wall_budget-(time.monotonic()-start)\n        need(rem_wall>0,'whole-job wall cap');signal.setitimer(signal.ITIMER_REAL,rem_wall)\n        for name in ('F_RESULT.json','F_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS.json','TERMINAL_FAILURE_RETAINED_AUDIT.json','F_retained_audit.stdout','F_retained_audit.stderr'):\n            need(not (packet/name).exists(),'retained audit output already exists: '+name)\n        try:\n            result=launch(c,'F',config_sha,retained_audit=True)\n            bound_identities(c)\n            need(time.monotonic()-start<=wall_budget,'whole-job wall cap')\n            result.update({'retained_audit':True,'audit_config_sha256':config_sha,'audit_runner_sha256':c['bindings']['runner']['sha256'],'marker_sha256':digest(root/'ONE_SHOT_STARTED'),'predecessors':predecessors,'quarantined_outputs_sha256':digest(packet/'F_RETAINED_QUARANTINED_OUTPUTS.json'),'native_outputs_accepted':False})\n            receipt(c,'F_RETAINED_AUDIT_RESULT.json',result,'F')\n        except BaseException as e:\n            signal.setitimer(signal.ITIMER_REAL,0)\n            try:receipt(c,'TERMINAL_FAILURE_RETAINED_AUDIT.json',{'type':type(e).__name__,'message':str(e),'native_outputs_accepted':False},'F')\n            except Exception:pass\n            raise\n    finally:\n        signal.setitimer(signal.ITIMER_REAL,0)"
    }
  ]
}
```

---

### 2. Complete Synthetic Regression Module (`test_output_retained_accounting.py`)

```python
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import bounded_sdk_compile as runner


def _write(path: Path, data: bytes = b'ok\n', mode: int = 0o644) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    os.chmod(path, mode)
    return path


class OutputAndRetainedAccountingTests(unittest.TestCase):
    PREDECESSORS = (
        'A0_RESULT.json',
        'A_RESULT.json',
        'B_RESULT.json',
        'C_RESULT.json',
        'D_RESULT.json',
        'E_RESULT.json',
        'D_FREEZE.json',
        'E_RESOLVED.json',
        'F.stdout',
        'F.stderr',
        'F_precheck.stdout',
        'F_precheck.stderr',
        'F_audit.stdout',
        'F_audit.stderr',
        'TERMINAL_FAILURE.json',
    )

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.base = Path(self._tmp.name).resolve()
        self.worktree = self.base / 'worktree'
        self.source_home = self.base / 'source-home'
        self.target = self.base / 'target'
        self.packet = self.base / 'packet'
        self.tmp_dir = self.base / 'tmp'
        self.empty_home = self.base / 'empty-home'
        self.mask_empty = self.base / 'mask-empty'
        self.tool = self.base / 'toolchain'
        for d in (
            self.worktree,
            self.source_home,
            self.target,
            self.packet,
            self.tmp_dir,
            self.empty_home,
            self.mask_empty,
            self.tool,
        ):
            d.mkdir(parents=True)

        (self.worktree / 'crates/mori-backend/src/sherlock_machine').mkdir(parents=True)
        self.lib_rs = _write(self.worktree / 'crates/mori-backend/src/lib.rs', b'// lib\n')
        self.build_rs = _write(self.worktree / 'crates/mori-backend/build.rs', b'// build\n')
        self.expected_json = _write(
            self.worktree / 'models/sherlock/machine-circuit/expected.json',
            b'{"circuit":1}\n',
        )

        self.ledger_rel = 'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d'
        self.ledger_root = self.source_home / self.ledger_rel
        (self.ledger_root / 'zswap/src').mkdir(parents=True)
        self.zswap_rs = _write(self.ledger_root / 'zswap/src/lib.rs', b'// zswap\n')
        self.verifier_rel = f'{self.ledger_rel}/zswap/static/spend.verifier.sha256'
        self.verifier = _write(self.source_home / self.verifier_rel, b'0123456789abcdef\n')

        self.unit_dir = 'mori-backend-0123456789abcdef'
        self.build_bin = _write(
            self.target / 'debug/build' / self.unit_dir / 'build_script_build-0123456789abcdef',
            b'\x7fELF-build',
            mode=0o755,
        )
        _write(self.target / 'debug/build' / self.unit_dir / 'out/gen.rs', b'// out\n')
        rel_dotdot = (
            'crates/mori-backend/src/sherlock_machine/'
            '../../../../models/sherlock/machine-circuit/expected.json'
        )
        abs_dotdot = f'{self.ledger_root}/zswap/src/../static/spend.verifier.sha256'
        _write(
            self.target / 'debug/build' / self.unit_dir / 'build_script_build-0123456789abcdef.d',
            (
                f'{self.build_bin}: crates/mori-backend/build.rs {rel_dotdot} {abs_dotdot}\n'
                f'{rel_dotdot}:\n'
                f'{abs_dotdot}:\n'
            ).encode(),
        )
        fp_dir = self.target / 'debug/.fingerprint' / self.unit_dir
        _write(fp_dir / 'build-script-build-script-build.json', b'{}\n')
        _write(fp_dir / 'run-build-script-build-script-build.json', b'{}\n')

        self.manifest_path = _write(
            self.base / 'source_manifest.json',
            runner.json_bytes(
                {
                    'files': {
                        'crates/mori-backend/src/lib.rs': runner.digest(self.lib_rs),
                        'crates/mori-backend/build.rs': runner.digest(self.build_rs),
                        'models/sherlock/machine-circuit/expected.json': runner.digest(
                            self.expected_json
                        ),
                    },
                    'head_files': [],
                }
            ),
        )
        self.runner_path = _write(self.base / 'runner.py', b'# runner\n')
        self.config_path = _write(self.base / 'config.final.json', b'{}\n')
        self.marker_path = _write(self.base / 'ONE_SHOT_STARTED', b'started\n')
        self.config_sha = 'a' * 64

        self.c = {
            'mode': 'tier1-and-compile',
            'total_wall_seconds': 6602,
            'supervisor_cpu_seconds': 10800,
            'masks': ['/home/charl', '/run'],
            'readonly_tools': [str(self.tool)],
            'git_checkouts': [
                {
                    'relative_root': self.ledger_rel,
                    'commit': '04c9c5d9bcebb8d4427d8589fb54d58a55599c14',
                    'files': [],
                }
            ],
            'registry_roots': [],
            'includes': [{'relative': self.verifier_rel}],
            'source_manifest': {'path': str(self.manifest_path)},
            'old_home_stat_manifest': {'path': str(self.manifest_path)},
            'bindings': {
                'runner': {
                    'path': str(self.runner_path),
                    'sha256': runner.digest(self.runner_path),
                }
            },
            'review_receipts': {},
            'cgroup': {
                'envelope_path': '/moriarty-test-06',
                'supervisor_leaf_name': 'supervisor',
                'phase_parent_name': 'phases',
            },
            'paths': {
                'worktree': str(self.worktree),
                'source_home': str(self.source_home),
                'target': str(self.target),
                'packet': str(self.packet),
                'tmp': str(self.tmp_dir),
                'empty_home': str(self.empty_home),
                'mask_empty': str(self.mask_empty),
                'execution_root': str(self.base),
                'config': str(self.config_path),
            },
            'limits': {
                'F': {
                    'wall_seconds': 2582,
                    'memory_bytes': 8589934592,
                    'pids': 1024,
                    'cpu_quota_us': 400000,
                    'cpu_period_us': 100000,
                    'cpu_time_us': 10800000000,
                    'stdout_bytes': 16777216,
                    'stderr_bytes': 16777216,
                    'packet_bytes': 67108864,
                    'file_bytes': 4294967296,
                    'namespace_tmpfs_bytes': 67108864,
                }
            },
        }
        self._sync_freeze_and_predecessors()

    def tearDown(self):
        self._tmp.cleanup()

    def _sync_freeze_and_predecessors(self, host_root=None):
        frozen_files = runner.tree_hashes(self.source_home, runner.declared_links(self.c))
        _write(self.packet / 'D_FREEZE.json', runner.json_bytes({'files': frozen_files}))
        _write(
            self.packet / 'E_RESOLVED.json',
            runner.json_bytes(
                {
                    'host_units': [
                        {
                            'identity': ['mori-backend 0.1.0', 'build-script', 'build.rs'],
                            'name': 'mori-backend',
                            'package': 'mori-backend',
                            'root': str(host_root or self.worktree),
                            'target_name': 'build-script-build',
                        }
                    ]
                }
            ),
        )
        prior_walls = {
            'A0': 2.6567903329996625,
            'A': 2.78098472900092,
            'B': 1.7633520050003426,
            'C': 12.067841101998056,
            'D': 7.613954207998177,
            'E': 12.315458076998766,
        }
        for ph, w in prior_walls.items():
            _write(
                self.packet / f'{ph}_RESULT.json',
                runner.json_bytes({'phase': ph, 'wall_seconds': w}),
            )
        for name, size in (
            ('F_precheck.stdout', 0),
            ('F_precheck.stderr', 0),
            ('F.stdout', 0),
            ('F.stderr', 10237),
            ('F_audit.stdout', 0),
            ('F_audit.stderr', 47),
        ):
            _write(self.packet / name, b'x' * size)
        _write(
            self.packet / 'TERMINAL_FAILURE.json',
            runner.json_bytes(
                {
                    'message': 'phase exit nonzero F',
                    'native_outputs_accepted': False,
                    'type': 'Refusal',
                }
            ),
        )

    def test_output_check_persists_receipt_and_enforces_manifest_freeze_include_and_host_unit(self):
        runner.output_check(self.c)
        q_path = self.packet / 'F_QUARANTINED_OUTPUTS.json'
        self.assertTrue(q_path.is_file())
        q_data = runner.jload(q_path)
        self.assertEqual(len(q_data['host_outputs']), 1)
        self.assertEqual(len(q_data['build_script_outputs']), 1)
        self.assertIn('UNACCEPTED', q_data['acceptance'])

        runner.worker(self.c, 'F', retained_audit=True)
        self.assertTrue((self.packet / 'F_RETAINED_QUARANTINED_OUTPUTS.json').is_file())

        # 1. Normalized worktree .. path missing from source_manifest['files']
        _write(
            self.manifest_path,
            runner.json_bytes(
                {
                    'files': {'crates/mori-backend/build.rs': runner.digest(self.build_rs)},
                    'head_files': [],
                }
            ),
        )
        with self.assertRaises(runner.Refusal) as ctx:
            runner.output_check(self.c)
        self.assertIn('unbound worktree dep-info input', str(ctx.exception))
        _write(
            self.manifest_path,
            runner.json_bytes(
                {
                    'files': {
                        'crates/mori-backend/src/lib.rs': runner.digest(self.lib_rs),
                        'crates/mori-backend/build.rs': runner.digest(self.build_rs),
                        'models/sherlock/machine-circuit/expected.json': runner.digest(
                            self.expected_json
                        ),
                    },
                    'head_files': [],
                }
            ),
        )

        # 2. Normalized source-home .. path missing or git-symlink in frozen_source
        with patch.object(runner, 'frozen_source', return_value={'files': {}}):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.output_check(self.c)
            self.assertIn('unbound source-home dep-info input', str(ctx.exception))

        with patch.object(
            runner,
            'frozen_source',
            return_value={'files': {self.verifier_rel: 'git-symlink:' + '0' * 40}},
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.output_check(self.c)
            self.assertIn('unbound source-home dep-info input', str(ctx.exception))

        # 3. SDK non-Rust .. path absent from c['includes']
        saved_inc = self.c['includes']
        self.c['includes'] = []
        try:
            with self.assertRaises(runner.Refusal) as ctx:
                runner.output_check(self.c)
            self.assertIn('unbound SDK non-Rust dep-info input', str(ctx.exception))
        finally:
            self.c['includes'] = saved_inc

        # 4. Host artifact source root mismatch in E_RESOLVED.json
        self._sync_freeze_and_predecessors(host_root=self.base / 'other-root')
        with self.assertRaises(runner.Refusal) as ctx:
            runner.output_check(self.c)
        self.assertIn('host artifact exact package/kind/source identity ambiguous', str(ctx.exception))

    def test_retained_sandbox_mounts_and_15_predecessors_contract(self):
        with patch.object(runner, 'owned_paths'):
            args = runner.sandbox_args(self.c, 'F', worker=True, precheck=False, retained_audit=True)
            self.assertIn('--retained-f-audit', args)
            t_idx = args.index(str(self.target))
            self.assertEqual(args[t_idx - 1], '--ro-bind')
            for bad in (('E', True, False), ('F', False, False), ('F', True, True)):
                with self.assertRaises(runner.Refusal) as ctx:
                    runner.sandbox_args(self.c, *bad, retained_audit=True)
                self.assertIn('retained audit sandbox mismatch', str(ctx.exception))

        mounts = runner.child_mounts(self.c, 'F', retained_audit=True)
        self.assertEqual(mounts[str(self.target)], 'ro')
        preds = runner.prior_receipts(self.c, 'F', False, retained_audit=True)
        self.assertEqual(set(preds), set(self.PREDECESSORS))

    def test_launch_accumulates_prior_f_logs_and_phases_cgroup_cpu_for_retained_audit(self):
        # 1. Prior F.stderr (10237) + F_audit.stderr (47) = 10284 > cap (10000) refused before create
        self.c['limits']['F']['stderr_bytes'] = 10000
        with (
            patch.object(runner, 'supervisor_envelope'),
            patch.object(runner, 'bound_identities'),
            patch.object(runner, 'quotas'),
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.launch(self.c, 'F', self.config_sha, retained_audit=True)
            self.assertIn('log cap hit F stderr', str(ctx.exception))
        self.c['limits']['F']['stderr_bytes'] = 16777216

        # 2. Prior phases/cpu.stat (240632042) >= cpu_time_us refused before create; sum checked in Group.check
        self.c['limits']['F']['cpu_time_us'] = 240632042
        with (
            patch.object(runner, 'supervisor_envelope'),
            patch.object(runner, 'bound_identities'),
            patch.object(runner, 'quotas'),
            patch.object(runner, '_cg_text', return_value='usage_usec 240632042\n'),
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.launch(self.c, 'F', self.config_sha, retained_audit=True)
            self.assertIn('total cgroup CPU cap', str(ctx.exception))

        self.c['limits']['F']['cpu_time_us'] = 240632500
        g = runner.Group(self.c, 'F')
        g.prior_cpu = 240632042
        with patch.object(
            runner,
            '_cg_text',
            side_effect=lambda p: 'usage_usec 500\n'
            if str(p).endswith('cpu.stat')
            else 'oom_kill 0\nmax 0\n',
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                g.check()
            self.assertIn('total cgroup CPU cap', str(ctx.exception))

    def test_retained_f_audit_enforces_entry_deadline_cpu_deduction_exclusivity_and_qualified_receipt(self):
        timers = []
        rlimits = []

        def _fake_launch(c, phase, sha, retained_audit=False):
            _write(self.packet / 'F_RETAINED_QUARANTINED_OUTPUTS.json', b'{"ok":true}\n')
            return {
                'phase': 'F',
                'processes': [{'argv': [], 'audit_worker': True, 'child_exit': 0}],
                'environment': {},
                'wall_seconds': 0.25,
                'log_bytes': {'stdout': 0, 'stderr': 10284},
            }

        # Success path: entry elapsed (5.0s) & prior_wall (~39.20s) deducted on timer reset; supervisor CPU (49s) deducted
        clock = iter([100.0, 105.0, 105.5])
        with (
            patch.object(runner.time, 'monotonic', side_effect=lambda: next(clock)),
            patch.object(runner.signal, 'signal'),
            patch.object(runner.signal, 'setitimer', side_effect=lambda w, v: timers.append(v)),
            patch.object(runner.resource, 'setrlimit', side_effect=lambda r, v: rlimits.append(v)),
            patch.object(runner, '_cg_text', return_value='usage_usec 48374910\n'),
            patch.object(runner, 'consent'),
            patch.object(runner, 'supervisor_envelope'),
            patch.object(runner, 'bound_identities'),
            patch.object(runner, 'owned_paths'),
            patch.object(runner, 'quotas'),
            patch.object(runner, 'launch', side_effect=_fake_launch),
        ):
            runner.retained_f_audit(self.c, self.config_sha)

        self.assertEqual(timers[0], 2582)
        self.assertAlmostEqual(timers[1], 2582 - 5.0)
        self.assertEqual(timers[-1], 0)
        self.assertEqual(rlimits[0], (10800 - 49, 10800 - 49))

        res = runner.jload(self.packet / 'F_RETAINED_AUDIT_RESULT.json')
        self.assertIs(res['retained_audit'], True)
        self.assertIs(res['native_outputs_accepted'], False)
        self.assertEqual(res['audit_config_sha256'], self.config_sha)
        self.assertEqual(res['audit_runner_sha256'], self.c['bindings']['runner']['sha256'])
        self.assertEqual(res['marker_sha256'], runner.digest(self.marker_path))
        self.assertEqual(
            res['quarantined_outputs_sha256'],
            runner.digest(self.packet / 'F_RETAINED_QUARANTINED_OUTPUTS.json'),
        )
        self.assertEqual(set(res['predecessors']), set(self.PREDECESSORS))

        # Fresh-output exclusivity refusal when F_RETAINED_AUDIT_RESULT.json already exists
        with (
            patch.object(runner.signal, 'signal'),
            patch.object(runner.signal, 'setitimer'),
            patch.object(runner.resource, 'setrlimit'),
            patch.object(runner, '_cg_text', return_value='usage_usec 48374910\n'),
            patch.object(runner, 'consent'),
            patch.object(runner, 'supervisor_envelope'),
            patch.object(runner, 'bound_identities'),
            patch.object(runner, 'owned_paths'),
            patch.object(runner, 'quotas'),
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.retained_f_audit(self.c, self.config_sha)
            self.assertIn(
                'retained audit output already exists: F_RETAINED_AUDIT_RESULT.json',
                str(ctx.exception),
            )
        (self.packet / 'F_RETAINED_AUDIT_RESULT.json').unlink()
        (self.packet / 'F_RETAINED_QUARANTINED_OUTPUTS.json').unlink()

        # Entry overhead exhausting remaining wall budget refuses before launch
        clock_exhausted = iter([200.0, 200.0 + 2582.1])
        with (
            patch.object(runner.time, 'monotonic', side_effect=lambda: next(clock_exhausted)),
            patch.object(runner.signal, 'signal'),
            patch.object(runner.signal, 'setitimer'),
            patch.object(runner.resource, 'setrlimit'),
            patch.object(runner, '_cg_text', return_value='usage_usec 48374910\n'),
            patch.object(runner, 'consent'),
            patch.object(runner, 'supervisor_envelope'),
            patch.object(runner, 'bound_identities'),
            patch.object(runner, 'owned_paths'),
            patch.object(runner, 'quotas'),
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.retained_f_audit(self.c, self.config_sha)
            self.assertIn('whole-job wall cap', str(ctx.exception))

        # Supervisor CPU exhausted in supervisor/cpu.stat refuses
        with (
            patch.object(runner.signal, 'signal'),
            patch.object(runner.signal, 'setitimer'),
            patch.object(runner, '_cg_text', return_value='usage_usec 10800000000\n'),
            patch.object(runner, 'consent'),
            patch.object(runner, 'supervisor_envelope'),
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.retained_f_audit(self.c, self.config_sha)
            self.assertIn('supervisor CPU cap', str(ctx.exception))

        # Tampered TERMINAL_FAILURE.json content refuses
        _write(
            self.packet / 'TERMINAL_FAILURE.json',
            runner.json_bytes(
                {'message': 'other', 'native_outputs_accepted': False, 'type': 'Refusal'}
            ),
        )
        with (
            patch.object(runner.signal, 'signal'),
            patch.object(runner.signal, 'setitimer'),
            patch.object(runner.resource, 'setrlimit'),
            patch.object(runner, '_cg_text', return_value='usage_usec 48374910\n'),
            patch.object(runner, 'consent'),
            patch.object(runner, 'supervisor_envelope'),
            patch.object(runner, 'bound_identities'),
            patch.object(runner, 'owned_paths'),
            patch.object(runner, 'quotas'),
        ):
            with self.assertRaises(runner.Refusal) as ctx:
                runner.retained_f_audit(self.c, self.config_sha)
            self.assertIn('unexpected prior terminal failure', str(ctx.exception))


if __name__ == '__main__':
    unittest.main()
```

---

### 3. Exact Selectors, Scope & GAP Statement

- **Exact `unittest` Selectors**:
  - Whole module: `python3 -B -m unittest test_output_retained_accounting`
  - Class: `python3 -B -m unittest test_output_retained_accounting.OutputAndRetainedAccountingTests`
  - Methods (4):
    1. `test_output_retained_accounting.OutputAndRetainedAccountingTests.test_output_check_persists_receipt_and_enforces_manifest_freeze_include_and_host_unit`
    2. `test_output_retained_accounting.OutputAndRetainedAccountingTests.test_retained_sandbox_mounts_and_15_predecessors_contract`
    3. `test_output_retained_accounting.OutputAndRetainedAccountingTests.test_launch_accumulates_prior_f_logs_and_phases_cgroup_cpu_for_retained_audit`
    4. `test_output_retained_accounting.OutputAndRetainedAccountingTests.test_retained_f_audit_enforces_entry_deadline_cpu_deduction_exclusivity_and_qualified_receipt`
- **Explicit Scope & GAP Statement**:
  - `R403`'s `test_depinfo_retained_audit.py` (`3` parser/path methods) is preserved unchanged.
  - Because Phase F-only CPU and Phase F-only wall time were not separately persisted before `sdk-once-F` teardown in Native 06, any Phase F remaining budget must use the conservative residual/aggregate upper bounds derived above (`wall <= 2582` s [or `<= 2542` s outer], `cpu_time_us <= 10,328,138,688` usec [4-core residual quota bound] or `<= 10,559,367,958` usec [measured `phases/cpu.stat` aggregate]).
  - Operational prerequisites (read-only pre/post target & packet inventory, refreshed `config.final.json` & review receipts bound to the revised guard SHA, exact launcher `argv`, and independent non-author review of these source/test deliverables) remain required prior to any execution selection.

END-R404-NATIVE06-ACCOUNTING-OWNER-CORRECTION

