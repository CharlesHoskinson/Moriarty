"""Focused unit tests for bounded_sdk_compile.launch() handoff and argv separation.

SOURCE ONLY / UNRUN during authoring. Exercises the actual adjacent
bounded_sdk_compile.launch() with all OS, process, socket, memfd, descriptor,
selector, cgroup, timer, and filesystem effects intercepted in memory.
"""
import array
from contextlib import ExitStack
import json
from pathlib import Path
import selectors
import signal
import socket
import subprocess
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import bounded_sdk_compile as runner


class FakeStream:
    def __init__(self, fd, name, chunks):
        self._fd = fd
        self.name = name
        self.chunks = [bytes(c) for c in chunks if c]
        self.nonblocking = False

    def fileno(self):
        return self._fd


class FakeLogFile:
    def __init__(self, path, mode, buffering, events, written_logs):
        if mode != 'xb' or buffering != 0:
            raise ValueError(f'unexpected log open args: mode={mode!r} buffering={buffering!r}')
        self.path = Path(path)
        self.name = self.path.name
        self.closed = False
        self.buffer = bytearray()
        self._events = events
        self._written_logs = written_logs
        self._written_logs[self.name] = b''
        self._events.append(f'log.open:{self.name}')

    def write(self, data):
        if self.closed:
            raise ValueError(f'write on closed log {self.name}')
        b = bytes(data)
        self.buffer.extend(b)
        self._written_logs[self.name] = bytes(self.buffer)
        self._events.append(f'log.write:{self.name}:{len(b)}')
        return len(b)

    def close(self):
        if not self.closed:
            self.closed = True
            self._events.append(f'log.close:{self.name}')


class FakeSocket:
    def __init__(self, fd, role, events, recv_plan=None):
        self._fd = fd
        self.role = role
        self.closed = False
        self.timeout = None
        self._events = events
        self._recv_plan = recv_plan

    def fileno(self):
        return self._fd

    def settimeout(self, value):
        self.timeout = value
        self._events.append(f'{self.role}.settimeout')

    def recvmsg(self, bufsize, ancbufsize=0):
        self._events.append(f'{self.role}.recvmsg')
        if isinstance(self._recv_plan, BaseException):
            raise self._recv_plan
        if callable(self._recv_plan):
            return self._recv_plan(bufsize, ancbufsize)
        return self._recv_plan

    def close(self):
        self.closed = True
        self._events.append(f'{self.role}.close')


class FakeSelector:
    def __init__(self, events, on_get_map=None, network_event=False):
        self._events = events
        self._map = {}
        self.closed = False
        self._on_get_map = on_get_map
        self._network_event = network_event

    def register(self, fileobj, events, data=None):
        if fileobj in self._map:
            raise KeyError(f'already registered: {fileobj!r}')
        fd = fileobj if isinstance(fileobj, int) else fileobj.fileno()
        key = SimpleNamespace(fileobj=fileobj, fd=fd, events=events, data=data)
        self._map[fileobj] = key
        tag = data[0] if isinstance(data, tuple) else str(data)
        self._events.append(f'sel.register:{tag}')
        return key

    def unregister(self, fileobj):
        key = self._map.pop(fileobj)
        tag = key.data[0] if isinstance(key.data, tuple) else str(key.data)
        self._events.append(f'sel.unregister:{tag}')
        return key

    def get_map(self):
        if self._on_get_map is not None:
            self._on_get_map(self)
        return dict(self._map)

    def select(self, timeout=None):
        self._events.append('sel.select')
        ready = []
        for key in list(self._map.values()):
            tag = key.data[0] if isinstance(key.data, tuple) else str(key.data)
            if tag == 'network' and not self._network_event:
                continue
            ready.append((key, key.events))
        return ready

    def close(self):
        self.closed = True
        self._map.clear()
        self._events.append('sel.close')


class FakePopen:
    def __init__(self, argv, stdin, stdout, stderr, env, pass_fds, preexec_fn, close_fds,
                 stdout_stream, stderr_stream, events, exit_code=0, poll_none_until_drained=False,
                 selector_ref=None):
        self.argv = list(argv)
        self.stdin = stdin
        self.stdout_arg = stdout
        self.stderr_arg = stderr
        self.env = dict(env)
        self.pass_fds = tuple(pass_fds)
        self.preexec_fn = preexec_fn
        self.close_fds = close_fds
        self.stdout = stdout_stream
        self.stderr = stderr_stream
        self._events = events
        self._exit_code = exit_code
        self.killed = False
        self._poll_none_until_drained = poll_none_until_drained
        self._selector_ref = selector_ref
        self._events.append('popen')

    def poll(self):
        if self.killed:
            return -9
        if self._poll_none_until_drained and self._selector_ref is not None:
            if len(self._selector_ref._map) > 1:
                return None
        return self._exit_code

    def wait(self, timeout=None):
        self._events.append('proc.wait')
        return -9 if self.killed else self._exit_code


class LaunchHarness:
    """Reusable in-memory harness intercepting all OS/process effects in runner.launch()."""

    def __init__(self, variant_plans=None, stdout_cap=64, stderr_cap=64, wall_seconds=5.0,
                 populated_after=0, network_pending_result=False, on_get_map=None):
        base = json.loads((Path(__file__).parent / 'config.example.json').read_text())
        base['masks'] = sorted(runner.REQUIRED_MASKS)
        base['review_receipts'] = {}
        base['bindings'] = {'runner': base['bindings']['runner']}
        base['source_manifest'] = {'path': '/control/source-manifest.json', 'sha256': '0' * 64}
        base['old_home_stat_manifest'] = {'path': '/control/old-home.json', 'sha256': '1' * 64}
        base['cgroup'] = {
            'envelope_path': '/review-envelope',
            'supervisor_leaf_name': 'supervisor',
            'phase_parent_name': 'phases',
            'envelope_memory_max': 8192,
            'envelope_pids_max': 32,
            'envelope_cpu_max': '10000 100000',
        }
        for ph in runner.PHASES:
            base['limits'][ph] = {
                'wall_seconds': wall_seconds,
                'memory_bytes': 8192,
                'pids': 16,
                'cpu_quota_us': 10000,
                'cpu_period_us': 100000,
                'cpu_time_us': 50000,
                'stdout_bytes': stdout_cap,
                'stderr_bytes': stderr_cap,
                'packet_bytes': 4096,
                'source_bytes': 4096,
                'target_tmp_bytes': 4096,
                'file_bytes': 4096,
                'namespace_tmpfs_bytes': 4096,
            }
        self.c = base
        self.config_sha = 'c' * 64
        self.events = []
        self.now = 100.0
        self.next_memfd = 100
        self.memfds = []
        self.open_fds = set()
        self.closed_fds = []
        self.written_logs = {}
        self.log_files = []
        self.socketpairs = []
        self.selectors = []
        self.popens = []
        self.groups = []
        self.itimer_calls = []
        self.set_blocking_calls = []
        self.populated_after = populated_after
        self.network_pending_result = network_pending_result
        self.on_get_map = on_get_map
        self._streams_by_fd = {}
        self._variant_index = 0
        default_plan = {
            'recvmsg': (
                b'L',
                [(socket.SOL_SOCKET, socket.SCM_RIGHTS, array.array('i', [401]).tobytes())],
                0,
                None,
            ),
            'scm_fds': [401],
            'stdout_chunks': [],
            'stderr_chunks': [],
            'exit_code': 0,
        }
        self.variant_plans = list(variant_plans) if variant_plans is not None else [default_plan]

    def _current_plan(self):
        idx = min(self._variant_index, len(self.variant_plans) - 1)
        return self.variant_plans[idx]

    def make_group(self, c, phase):
        harness = self

        class FakeGroup:
            def __init__(self):
                self.path = runner.cg_fs(runner.phase_cgroup(c, phase))
                self.phase = phase
                self.created = False

            def create(self):
                self.created = True
                harness.events.append('group.create')

            def join_child(self):
                harness.events.append('group.join_child')

            def kill(self):
                harness.events.append('group.kill')
                for proc in harness.popens:
                    proc.killed = True

            def check(self):
                harness.events.append('group.check')

            def close(self):
                harness.events.append('group.close')

        g = FakeGroup()
        self.groups.append(g)
        return g

    def fake_sealed_memfd(self, name, data):
        fd = self.next_memfd
        self.next_memfd += 1
        payload = bytes(data)
        self.memfds.append((fd, name, payload))
        self.open_fds.add(fd)
        self.events.append(f'memfd:{name}:{fd}')
        return fd

    def fake_socketpair(self):
        plan = self._current_plan()
        base_fd = 300 + self._variant_index * 10
        for scm_fd in plan.get('scm_fds', []):
            self.open_fds.add(scm_fd)
        parent = FakeSocket(base_fd, 'parent', self.events, recv_plan=plan['recvmsg'])
        child = FakeSocket(base_fd + 1, 'child', self.events)
        self.socketpairs.append((parent, child))
        return parent, child

    def fake_selector(self):
        sel = FakeSelector(self.events, on_get_map=self.on_get_map)
        self.selectors.append(sel)
        return sel

    def fake_open(self, path, mode='r', buffering=-1):
        lf = FakeLogFile(path, mode, buffering, self.events, self.written_logs)
        self.log_files.append(lf)
        return lf

    def fake_popen(self, argv, stdin=None, stdout=None, stderr=None, env=None,
                   pass_fds=(), preexec_fn=None, close_fds=True):
        plan = self._current_plan()
        base_stream_fd = 200 + self._variant_index * 10
        out_stream = FakeStream(base_stream_fd + 1, 'stdout', plan.get('stdout_chunks', []))
        err_stream = FakeStream(base_stream_fd + 2, 'stderr', plan.get('stderr_chunks', []))
        self._streams_by_fd[out_stream.fileno()] = out_stream
        self._streams_by_fd[err_stream.fileno()] = err_stream
        sel_ref = self.selectors[-1] if self.selectors else None
        proc = FakePopen(
            argv, stdin, stdout, stderr, env, pass_fds, preexec_fn, close_fds,
            out_stream, err_stream, self.events,
            exit_code=plan.get('exit_code', 0),
            poll_none_until_drained=plan.get('poll_none_until_drained', False),
            selector_ref=sel_ref,
        )
        self.popens.append(proc)
        self._variant_index += 1
        return proc

    def fake_os_set_blocking(self, fd, blocking):
        self.set_blocking_calls.append((fd, blocking))
        if fd in self._streams_by_fd:
            self._streams_by_fd[fd].nonblocking = not blocking

    def fake_os_read(self, fd, n):
        stream = self._streams_by_fd[fd]
        self.events.append(f'os.read:{stream.name}')
        if stream.chunks:
            return stream.chunks.pop(0)
        return b''

    def fake_os_close(self, fd):
        self.events.append(f'os.close:{fd}')
        self.closed_fds.append(fd)
        if fd not in self.open_fds:
            raise OSError(9, f'bad or unexpected fd close: {fd}')
        self.open_fds.remove(fd)

    def fake_setitimer(self, which, seconds):
        self.itimer_calls.append((which, seconds))
        self.events.append(f'setitimer:{which}:{seconds}')

    def patches(self):
        stack = ExitStack()
        marker_path = Path(self.c['paths']['execution_root']) / 'ONE_SHOT_STARTED'
        stack.enter_context(patch.object(runner, 'Group', side_effect=self.make_group))
        stack.enter_context(patch.object(runner.Path, 'is_file', autospec=True,
                                         side_effect=lambda p: Path(p) == marker_path))
        stack.enter_context(patch.object(runner, 'supervisor_envelope',
                                         side_effect=lambda c: self.events.append('supervisor_envelope')))
        stack.enter_context(patch.object(runner, 'bound_identities',
                                         side_effect=lambda c: self.events.append('bound_identities')))
        stack.enter_context(patch.object(runner, 'quotas',
                                         side_effect=lambda c, ph: self.events.append(f'quotas:{ph}')))
        stack.enter_context(patch.object(runner, 'owned_paths'))
        stack.enter_context(patch.object(runner, 'jload', return_value={'head_files': []}))
        stack.enter_context(patch.object(runner.time, 'monotonic', side_effect=lambda: self.now))
        stack.enter_context(patch.object(runner.os, 'urandom', return_value=b'\xaa' * 32))
        stack.enter_context(patch.object(runner, 'digest',
                                         side_effect=lambda p: 'd' * 64))
        stack.enter_context(patch.object(runner, 'sealed_memfd', side_effect=self.fake_sealed_memfd))
        stack.enter_context(patch.object(socket, 'socketpair', side_effect=self.fake_socketpair))
        stack.enter_context(patch.object(runner.selectors, 'DefaultSelector', side_effect=self.fake_selector))
        stack.enter_context(patch.object(runner, 'open', side_effect=self.fake_open, create=True))
        stack.enter_context(patch.object(runner.subprocess, 'Popen', side_effect=self.fake_popen))
        stack.enter_context(patch.object(runner.os, 'set_blocking', side_effect=self.fake_os_set_blocking))
        stack.enter_context(patch.object(runner.os, 'read', side_effect=self.fake_os_read))
        stack.enter_context(patch.object(runner.os, 'close', side_effect=self.fake_os_close))
        stack.enter_context(patch.object(runner, 'network_pending',
                                         side_effect=lambda fd: self.network_pending_result))
        stack.enter_context(patch.object(runner, 'read_numbers',
                                         side_effect=lambda p: {'populated': self.populated_after}))
        stack.enter_context(patch.object(runner.signal, 'setitimer', side_effect=self.fake_setitimer))
        return stack


class LaunchHandoffTests(unittest.TestCase):
    def test_audit_phase_a0_options_only_memfd_outer_argv_pass_fds_and_records(self):
        h = LaunchHarness(variant_plans=[{
            'recvmsg': (
                b'L',
                [(socket.SOL_SOCKET, socket.SCM_RIGHTS, array.array('i', [401]).tobytes())],
                0,
                None,
            ),
            'scm_fds': [401],
            'stdout_chunks': [b'a0-stdout\n'],
            'stderr_chunks': [b'a0-stderr\n'],
            'exit_code': 0,
        }])
        with h.patches():
            expected_raw = runner.sandbox_args(h.c, 'A0', True, False)
            res = runner.launch(h.c, 'A0', h.config_sha)

        sep = expected_raw.index('--')
        expected_options = expected_raw[:sep]
        expected_target = expected_raw[sep + 1:] + ['--config-sha256', h.config_sha, '--worker-fd', '101']
        expected_cmd = [
            '/usr/bin/python3', '-I', '-B', '-S', '/proc/self/fd/100',
            '100', '301', '101', *expected_target,
        ]

        self.assertEqual([m[1] for m in h.memfds], [
            'sdk-post-bwrap-stage',
            'sdk-internal-worker-context',
            'sdk-bwrap-argv',
        ])
        self.assertEqual(h.memfds[0][2], runner.TRAMPOLINE)
        ctx_obj = json.loads(h.memfds[1][2].decode())
        self.assertEqual(ctx_obj['phase'], 'A0')
        self.assertFalse(ctx_obj['precheck'])
        self.assertEqual(ctx_obj['config_sha256'], h.config_sha)
        self.assertEqual(ctx_obj['cgroup'], '0::/review-envelope/phases/sdk-once-A0')

        bwrap_memfd_bytes = h.memfds[2][2]
        self.assertTrue(bwrap_memfd_bytes.endswith(b'\0'))
        memfd_tokens = [tok.decode() for tok in bwrap_memfd_bytes[:-1].split(b'\0')]
        self.assertEqual(memfd_tokens, expected_options)
        self.assertNotIn('--', memfd_tokens)
        self.assertNotIn('/usr/bin/python3', memfd_tokens)

        self.assertEqual(len(h.popens), 1)
        proc = h.popens[0]
        self.assertEqual(proc.argv, ['/usr/bin/bwrap', '--args', '102', '--', *expected_cmd])
        self.assertEqual(proc.pass_fds, (102, 100, 301, 101))
        self.assertEqual(proc.env, {})
        self.assertIs(proc.stdin, subprocess.DEVNULL)
        self.assertIs(proc.stdout_arg, subprocess.PIPE)
        self.assertIs(proc.stderr_arg, subprocess.PIPE)
        self.assertTrue(proc.close_fds)

        self.assertEqual(res['phase'], 'A0')
        self.assertEqual(res['processes'], [{
            'argv': expected_options + ['--', *expected_cmd],
            'audit_worker': True,
            'child_exit': 0,
        }])
        self.assertEqual(res['log_bytes'], {'stdout': 10, 'stderr': 10})
        self.assertEqual(h.written_logs['A0.stdout'], b'a0-stdout\n')
        self.assertEqual(h.written_logs['A0.stderr'], b'a0-stderr\n')
        self.assertEqual(h.closed_fds, [401, 102, 100, 101])
        self.assertEqual(h.open_fds, set())
        self.assertEqual(h.itimer_calls, [])
        self.assertEqual(h.events[-1], 'group.close')

        # Verify preexec_fn closes parent socket and joins child cgroup in memory.
        proc.preexec_fn()
        self.assertEqual(h.events[-2:], ['parent.close', 'group.join_child'])

    def test_non_audit_phase_b_omits_token_fd_and_separates_cargo_command(self):
        h = LaunchHarness(variant_plans=[{
            'recvmsg': (
                b'L',
                [(socket.SOL_SOCKET, socket.SCM_RIGHTS, array.array('i', [402]).tobytes())],
                0,
                None,
            ),
            'scm_fds': [402],
            'stdout_chunks': [b'fetch-ok'],
            'stderr_chunks': [],
            'exit_code': 0,
        }])
        with h.patches():
            expected_raw = runner.sandbox_args(h.c, 'B', False, False)
            res = runner.launch(h.c, 'B', h.config_sha)

        sep = expected_raw.index('--')
        expected_options = expected_raw[:sep]
        expected_cmd = [
            '/usr/bin/python3', '-I', '-B', '-S', '/proc/self/fd/100',
            '100', '301', '-1', *runner.cargo_argv('B'),
        ]

        self.assertEqual([m[1] for m in h.memfds], ['sdk-post-bwrap-stage', 'sdk-bwrap-argv'])
        memfd_tokens = [tok.decode() for tok in h.memfds[1][2][:-1].split(b'\0')]
        self.assertEqual(memfd_tokens, expected_options)
        self.assertNotIn('--', memfd_tokens)
        self.assertNotIn(runner.CARGO, memfd_tokens)

        proc = h.popens[0]
        self.assertEqual(proc.argv, ['/usr/bin/bwrap', '--args', '101', '--', *expected_cmd])
        self.assertEqual(proc.pass_fds, (101, 100, 301))
        self.assertNotIn(-1, proc.pass_fds)
        self.assertEqual(h.closed_fds, [402, 101, 100])
        self.assertNotIn(-1, h.closed_fds)
        self.assertEqual(h.open_fds, set())
        self.assertEqual(res['processes'], [{
            'argv': expected_options + ['--', *expected_cmd],
            'audit_worker': False,
            'child_exit': 0,
        }])
        self.assertEqual(h.events[-1], 'group.close')

    def test_multi_variant_phase_e_preserves_precheck_cargo_and_audit_sequencing(self):
        plans = [
            {
                'recvmsg': (
                    b'L',
                    [(socket.SOL_SOCKET, socket.SCM_RIGHTS, array.array('i', [410 + i]).tobytes())],
                    0,
                    None,
                ),
                'scm_fds': [410 + i],
                'stdout_chunks': [f'out-{i}'.encode()],
                'stderr_chunks': [f'err-{i}'.encode()],
                'exit_code': 0,
            }
            for i in range(3)
        ]
        h = LaunchHarness(variant_plans=plans)
        with h.patches():
            res = runner.launch(h.c, 'E', h.config_sha)

        self.assertEqual(len(h.popens), 3)
        self.assertEqual([r['audit_worker'] for r in res['processes']], [True, False, True])
        self.assertEqual([lf.name for lf in h.log_files], [
            'E_precheck.stdout', 'E_precheck.stderr',
            'E.stdout', 'E.stderr',
            'E_audit.stdout', 'E_audit.stderr',
        ])
        # Variant 0: precheck worker (has --precheck and token_fd=101 in pass_fds)
        self.assertIn('--precheck', h.popens[0].argv)
        self.assertEqual(len(h.popens[0].pass_fds), 4)
        ctx0 = json.loads(h.memfds[1][2].decode())
        self.assertTrue(ctx0['precheck'])
        self.assertNotIn('E.stdout', ctx0['predecessors'])

        # Variant 1: non-audit Cargo metadata step (token_fd is '-1', 3 pass_fds)
        self.assertNotIn('--precheck', h.popens[1].argv)
        self.assertEqual(len(h.popens[1].pass_fds), 3)
        self.assertIn('metadata', h.popens[1].argv)

        # Variant 2: post-Cargo audit worker (precheck=False, predecessors include E.stdout/E.stderr)
        self.assertNotIn('--precheck', h.popens[2].argv)
        self.assertEqual(len(h.popens[2].pass_fds), 4)
        ctx2 = json.loads(h.memfds[6][2].decode())
        self.assertFalse(ctx2['precheck'])
        self.assertIn('E.stdout', ctx2['predecessors'])
        self.assertIn('E.stderr', ctx2['predecessors'])

        self.assertEqual(h.events.count('group.create'), 1)
        self.assertEqual(h.events.count('group.close'), 1)
        self.assertEqual(h.open_fds, set())

    def test_pre_handoff_eof_kills_group_before_draining_and_clamps_over_budget_stderr(self):
        h = LaunchHarness(
            stdout_cap=16,
            stderr_cap=10,
            variant_plans=[{
                'recvmsg': (b'', [], 0, None),
                'scm_fds': [],
                'stdout_chunks': [b'out-ok-9B'],
                'stderr_chunks': [b'012345', b'6789ABCDEF'],
                'exit_code': 1,
            }],
        )
        with h.patches():
            with self.assertRaises(runner.Refusal) as ctx:
                runner.launch(h.c, 'B', h.config_sha)

        self.assertIs(type(ctx.exception), runner.Refusal)
        self.assertEqual(str(ctx.exception), 'listener handoff failed')

        # Stream pipes registered before recvmsg; group.kill runs strictly before any drain read/write.
        idx_reg_out = h.events.index('sel.register:stdout')
        idx_reg_err = h.events.index('sel.register:stderr')
        idx_recv = h.events.index('parent.recvmsg')
        idx_kill = h.events.index('group.kill')
        idx_first_read = min(i for i, ev in enumerate(h.events) if ev.startswith('os.read:'))
        idx_first_write = min(i for i, ev in enumerate(h.events) if ev.startswith('log.write:'))
        idx_close = h.events.index('group.close')

        self.assertLess(idx_reg_out, idx_recv)
        self.assertLess(idx_reg_err, idx_recv)
        self.assertLess(idx_recv, idx_kill)
        self.assertLess(idx_kill, idx_first_read)
        self.assertLess(idx_kill, idx_first_write)
        self.assertLess(idx_first_write, idx_close)
        self.assertEqual(h.events[-1], 'group.close')

        # Exact retained stdout bytes and clamped stderr bytes (10B cap out of 16B buffered).
        self.assertEqual(h.written_logs['B.stdout'], b'out-ok-9B')
        self.assertEqual(h.written_logs['B.stderr'], b'0123456789')
        self.assertNotIn('sel.register:network', h.events)
        self.assertEqual(h.open_fds, set())
        self.assertEqual(h.itimer_calls, [(signal.ITIMER_REAL, 0), (signal.ITIMER_REAL, 0)])

    def test_pre_handoff_two_scm_rights_fds_closes_all_unpacked_fds_and_preserves_refusal(self):
        two_fds_cmsg = (
            socket.SOL_SOCKET,
            socket.SCM_RIGHTS,
            array.array('i', [501, 502]).tobytes(),
        )
        h = LaunchHarness(variant_plans=[{
            'recvmsg': (b'L', [two_fds_cmsg], 0, None),
            'scm_fds': [501, 502],
            'stdout_chunks': [],
            'stderr_chunks': [b'two-scm-fds-diagnostic\n'],
            'exit_code': 1,
        }])
        with h.patches():
            with self.assertRaises(runner.Refusal) as ctx:
                runner.launch(h.c, 'A0', h.config_sha)

        self.assertIs(type(ctx.exception), runner.Refusal)
        self.assertEqual(str(ctx.exception), 'exact one seccomp listener required')

        # Both unpacked FDs 501 and 502 are closed once in except BaseException before group.kill and drain.
        idx_close_501 = h.events.index('os.close:501')
        idx_close_502 = h.events.index('os.close:502')
        idx_kill = h.events.index('group.kill')
        idx_read_err = h.events.index('os.read:stderr')
        self.assertLess(idx_close_501, idx_kill)
        self.assertLess(idx_close_502, idx_kill)
        self.assertLess(idx_kill, idx_read_err)
        self.assertEqual(h.closed_fds.count(501), 1)
        self.assertEqual(h.closed_fds.count(502), 1)
        self.assertEqual(h.open_fds, set())
        self.assertEqual(h.written_logs['A0.stderr'], b'two-scm-fds-diagnostic\n')
        self.assertEqual(h.events[-1], 'group.close')

    def test_pre_handoff_ancillary_mismatch_and_nonzero_flags_close_prior_fds_and_refuse(self):
        # Case 1: first cmsg unpacks FD 601, second cmsg has unexpected type -> closes 601 and refuses.
        cmsg_ok = (socket.SOL_SOCKET, socket.SCM_RIGHTS, array.array('i', [601]).tobytes())
        cmsg_bad = (socket.SOL_SOCKET, 999, array.array('i', [602]).tobytes())
        h1 = LaunchHarness(variant_plans=[{
            'recvmsg': (b'L', [cmsg_ok, cmsg_bad], 0, None),
            'scm_fds': [601],
            'stdout_chunks': [],
            'stderr_chunks': [b'anc-mismatch'],
            'exit_code': 1,
        }])
        with h1.patches():
            with self.assertRaises(runner.Refusal) as ctx1:
                runner.launch(h1.c, 'B', h1.config_sha)
        self.assertEqual(str(ctx1.exception), 'unexpected handoff')
        self.assertLess(h1.events.index('os.close:601'), h1.events.index('group.kill'))
        self.assertEqual(h1.written_logs['B.stderr'], b'anc-mismatch')
        self.assertEqual(h1.open_fds, set())
        self.assertEqual(h1.events[-1], 'group.close')

        # Case 2: nonzero recvmsg flags (e.g. MSG_CTRUNC) -> refuses with 'listener handoff failed'.
        h2 = LaunchHarness(variant_plans=[{
            'recvmsg': (b'L', [], socket.MSG_CTRUNC, None),
            'scm_fds': [],
            'stdout_chunks': [],
            'stderr_chunks': [b'ctrunc-err'],
            'exit_code': 1,
        }])
        with h2.patches():
            with self.assertRaises(runner.Refusal) as ctx2:
                runner.launch(h2.c, 'B', h2.config_sha)
        self.assertEqual(str(ctx2.exception), 'listener handoff failed')
        self.assertEqual(h2.written_logs['B.stderr'], b'ctrunc-err')
        self.assertEqual(h2.open_fds, set())
        self.assertEqual(h2.events[-1], 'group.close')

    def test_pre_handoff_timeout_preserves_original_exception_identity_and_drains_after_kill(self):
        timeout_exc = socket.timeout('handoff socket recvmsg timed out')
        h = LaunchHarness(variant_plans=[{
            'recvmsg': timeout_exc,
            'scm_fds': [],
            'stdout_chunks': [b'partial-out'],
            'stderr_chunks': [b'timeout-stderr-trace\n'],
            'exit_code': 1,
        }])
        with h.patches():
            with self.assertRaises(socket.timeout) as ctx:
                runner.launch(h.c, 'B', h.config_sha)

        # Exact exception object identity, type, and message are preserved without masking.
        self.assertIs(ctx.exception, timeout_exc)
        self.assertEqual(str(ctx.exception), 'handoff socket recvmsg timed out')

        idx_recv = h.events.index('parent.recvmsg')
        idx_kill = h.events.index('group.kill')
        idx_read = min(i for i, ev in enumerate(h.events) if ev.startswith('os.read:'))
        self.assertLess(idx_recv, idx_kill)
        self.assertLess(idx_kill, idx_read)
        self.assertEqual(h.written_logs['B.stdout'], b'partial-out')
        self.assertEqual(h.written_logs['B.stderr'], b'timeout-stderr-trace\n')
        self.assertEqual(h.open_fds, set())
        self.assertEqual(h.events[-1], 'group.close')

    def test_pre_handoff_drain_stops_at_wall_deadline_without_masking_handoff_exception(self):
        def expire_deadline_on_drain_map(_sel):
            h.now = 106.0  # Exceeds start (100.0) + wall_seconds (5.0)

        h = LaunchHarness(
            wall_seconds=5.0,
            on_get_map=expire_deadline_on_drain_map,
            variant_plans=[{
                'recvmsg': (b'', [], 0, None),
                'scm_fds': [],
                'stdout_chunks': [b'unreached'],
                'stderr_chunks': [b'unreached'],
                'exit_code': 1,
            }],
        )
        with h.patches():
            with self.assertRaises(runner.Refusal) as ctx:
                runner.launch(h.c, 'B', h.config_sha)

        # Breaks out of drain loop on deadline without raising 'wall cap B' or calling sel.select.
        self.assertEqual(str(ctx.exception), 'listener handoff failed')
        self.assertIn('group.kill', h.events)
        self.assertNotIn('sel.select', h.events)
        self.assertEqual(h.open_fds, set())
        self.assertEqual(h.events[-1], 'group.close')


if __name__ == '__main__':
    unittest.main()
