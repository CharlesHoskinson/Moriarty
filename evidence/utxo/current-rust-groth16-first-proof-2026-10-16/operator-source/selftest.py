"""Ordinary public-source regression harness for the repaired operator mechanics.

Corrected harness. Fixes versus the predecessor harness:
- Candidate loading uses importlib.util.spec_from_file_location under a
  unique module name from the exact sibling operator.py, with __file__
  and required-function identity assertions. It never does
  `import operator`, so a pre-cached stdlib operator in sys.modules
  cannot shadow the candidate. Import must be confirmed
  side-effect-free by the fresh reviewer from operator source before
  any root run; importing constants that contain path strings is not
  filesystem access.
- Negative tests catch only the candidate OpError and assert the exact
  failure category from operator source, with the failure assertion in
  the else branch (no vacuous self-caught AssertionError).
- The grandchild test records the exact grandchild PID to a
  create-exclusive file under the explicit argv testroot and verifies
  that exact process is terminal after the runner returns.
- The invented negatives-metadata document test is removed. A real
  production function (_strict_json) is tested instead. Full S6N
  metadata persistence remains pending native run/full-source audit
  and is not claimed by this harness.
- Signal-death and missing-executable checks exercise only
  run_bounded_process flags. They do NOT exercise Runner full-cap
  charging; Runner.run is not invoked here because it writes logs
  under the fixed native tree, which this harness must never touch.
- Targeted-repair regressions exercise the actual production
  placement predicate (new_placement_state / record_r0_receipt /
  finalizer_eligible) over the R1-refusal-with-old-tree, owned fully
  placed, owned partial placement, and no-AUTH cases, and the actual
  production S6N freeze partition (s6n_freeze_additions) over success,
  partial, missing, and undeclared-file cases. Neither duplicates a
  test-only boolean. A safely bounded escaped-pipe-holder check (the
  holder exits after about 2 seconds) exercises drain completion; the
  still-unjoined-after-deadline subcase is not exercised because it
  would require holding a pipe past the join deadlines.

Never invokes operator main/AUTH/custody/probe/bwrap/driver/SDK,
never accesses fixed BASE/M5/M6/T5/T6/RR5/RR6 or any /w, key, PK, SRS
or cache path, and keeps native nineteen/counts/PK caps outside this
test. Usage: python3 selftest.py <explicit-public-testroot>
The testroot must not pre-exist; it is created deterministically and
fixtures use deterministic names with create-exclusive semantics.
No cleanup of unknown paths is performed. Supervised stubs use the
/usr/bin/python3 literal only; no cryptography/RNG/network. Test-only
caps are injected into the generic functions without changing native
caps or AUTH defaults. Nothing passing is claimed from authoring;
actual results are unobserved until a reviewer-approved root run.
"""
import importlib.util
import json
import os
import sys
import time

PY = '/usr/bin/python3'
UNIQUE_NAME = 'candidate_operator_under_selftest_7973e29a'
REQUIRED_CALLABLES = ('run_bounded_process', 'run_supervised_callable', 'check_exactly_one_entry', 'attempt_finalizers', 'new_placement_state', 'record_r0_receipt', 'finalizer_eligible', 's6n_freeze_additions')


def _load_candidate():
    here = os.path.dirname(os.path.abspath(__file__))
    candidate_path = os.path.join(here, 'operator.py')
    if not os.path.isfile(candidate_path):
        raise AssertionError('exact sibling operator.py is missing')
    spec = importlib.util.spec_from_file_location(UNIQUE_NAME, candidate_path)
    if spec is None or spec.loader is None:
        raise AssertionError('cannot build import spec for sibling operator.py')
    module = importlib.util.module_from_spec(spec)
    sys.modules[UNIQUE_NAME] = module
    spec.loader.exec_module(module)
    if os.path.realpath(module.__file__) != os.path.realpath(candidate_path):
        raise AssertionError('candidate __file__ mismatch: ' + repr(module.__file__))
    if module.__name__ != UNIQUE_NAME:
        raise AssertionError('candidate module name mismatch')
    for name in REQUIRED_CALLABLES:
        if not callable(getattr(module, name, None)):
            raise AssertionError('candidate lacks required callable ' + name)
    if not (isinstance(module.OpError, type) and issubclass(module.OpError, Exception)):
        raise AssertionError('candidate OpError is not an Exception subclass')
    return module


def _assert_pid_terminal(pid):
    deadline = time.monotonic() + 5.0
    last_state = 'unknown'
    while True:
        try:
            with open('/proc/%d/stat' % pid, 'rb') as fh:
                raw = fh.read(4096)
        except FileNotFoundError:
            return
        except OSError:
            return
        rparen = raw.rfind(b')')
        if rparen < 0:
            raise AssertionError('unparsable stat for recorded stub pid')
        fields = raw[rparen + 1:].split()
        if not fields:
            raise AssertionError('empty stat for recorded stub pid')
        last_state = fields[0].decode('ascii', 'replace')
        if last_state in ('Z', 'X', 'x'):
            return
        if time.monotonic() >= deadline:
            raise AssertionError('recorded grandchild pid %d still in state %s' % (pid, last_state))
        time.sleep(0.1)


def main(argv):
    if len(argv) != 2:
        print('usage: selftest.py <public-testroot>')
        return 2
    root = os.path.abspath(argv[1])
    forbidden = ('/current-cardano-', '/w/', 'keys', '.pk', 'cache')
    if any(tok in root for tok in forbidden):
        print('refusing non-public testroot')
        return 2
    if os.path.lexists(root):
        print('refusing pre-existing testroot')
        return 2
    os.makedirs(root)
    op = _load_candidate()
    results = []

    def check(name, fn):
        try:
            fn()
            results.append((name, 'ok'))
        except Exception as e:
            results.append((name, ('FAIL: ' + str(e))[:512]))

    def t1():
        r = op.run_bounded_process([PY, '-I', '-c', 'print("hello")'], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert r['clean'] and r['exit'] == 0 and r['_stdout'] == b'hello\n', r
    check('bounded-stdout', t1)

    def t2():
        r = op.run_bounded_process([PY, '-I', '-c', 'import sys; sys.stdout.write("x"*100000)'], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert r['overflow'] and not r['clean'] and len(r['_stdout']) <= 1024, r
    check('stdout-overflow', t2)

    def t3():
        r = op.run_bounded_process([PY, '-I', '-c', 'import sys; sys.stderr.write("y"*100000)'], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert r['overflow'] and not r['clean'] and len(r['_stderr']) <= 1024, r
    check('stderr-overflow', t3)

    # Flag only: this does not exercise Runner full-cap charging.
    def t4():
        r = op.run_bounded_process([PY, '-I', '-c', 'import os, signal; os.kill(os.getpid(), signal.SIGKILL)'], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert not r['clean'] and r['exit'] == -9, r
    check('signal-death-flag-only-not-runner-charging', t4)

    # Flag only: this does not exercise Runner full-cap charging.
    def t5():
        r = op.run_bounded_process(['/nonexistent/definitely-missing-binary-xyz'], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert not r['clean'] and 'launchError' in r, r
    check('missing-executable-flag-only-not-runner-charging', t5)

    def t6():
        r = op.run_bounded_process([PY, '-I', '-c', 'import time; time.sleep(30)'], cpu_cap=5, wall_cap=1, kill_after=1, stream_cap=1024)
        assert r['timedOut'] and not r['clean'], r
    check('wall-timeout', t6)

    def t7():
        pid_path = os.path.join(root, 'grandchild.pid')
        script = (
            'import subprocess, time\n'
            'p = subprocess.Popen(["/usr/bin/python3", "-I", "-c", "import time; time.sleep(30)"])\n'
            'fh = open(' + repr(pid_path) + ', "x")\n'
            'fh.write(str(p.pid))\n'
            'fh.close()\n'
            'time.sleep(30)\n'
        )
        r = op.run_bounded_process([PY, '-I', '-c', script], cpu_cap=5, wall_cap=1, kill_after=1, stream_cap=1024)
        assert r['timedOut'] and not r['clean'], r
        with open(pid_path, 'rb') as fh:
            pid = int(fh.read(64).decode('ascii').strip())
        assert pid > 0
        _assert_pid_terminal(pid)
    check('terminal-cleanup-grandchild-terminal', t7)

    def t8():
        assert op.run_supervised_callable(lambda: {'a': 1}, cpu_cap=5, wall_cap=5, kill_after=1, ipc_cap=65536) == {'a': 1}
        try:
            op.run_supervised_callable(lambda: __import__('time').sleep(10), cpu_cap=5, wall_cap=1, kill_after=1, ipc_cap=65536)
        except op.OpError as e:
            assert e.category == 'resource', e.category
        else:
            raise AssertionError('expected supervised wall-kill OpError')
    check('supervised-callable-success-and-wall-kill', t8)

    def t8b():
        try:
            op.run_supervised_callable(lambda: {'data': 'x' * 100000}, cpu_cap=5, wall_cap=5, kill_after=1, ipc_cap=1024)
        except op.OpError as e:
            assert e.category == 'cap', e.category
        else:
            raise AssertionError('expected supervised IPC-overflow OpError')
    check('supervised-ipc-overflow', t8b)

    def t9():
        assert 'logs/custody-layout.log' in op.PUB_CANDIDATES
    check('custody-layout-allowlist', t9)

    # Real production function with a meaningful oracle. This replaces
    # the removed invented negatives-metadata document round-trip.
    # Full S6N metadata persistence remains pending native run and
    # full-source audit; it is not tested or claimed here.
    def t10():
        assert op._strict_json(b'{"a": 1, "b": [1, 2]}', 'selftest') == {'a': 1, 'b': [1, 2]}
        try:
            op._strict_json(b'{"a": 1, "a": 2}', 'selftest')
        except op.OpError as e:
            assert e.category == 'json', e.category
        else:
            raise AssertionError('expected duplicate-key OpError')
    check('strict-json-duplicate-key-real-function', t10)

    def t11():
        calls = []
        def bad():
            calls.append('G6')
            raise RuntimeError('boom')
        def good(name):
            def f():
                calls.append(name)
                return True
            return f
        res = op.attempt_finalizers([('G6', bad), ('H6', good('H6')), ('Z61', good('Z61')), ('Z62', good('Z62'))])
        assert calls == ['G6', 'H6', 'Z61', 'Z62'] and not res['G6']['ok'] and res['Z62']['ok'], res
    check('finalizers-all-attempted', t11)

    def t12():
        d = os.path.join(root, 'outcase')
        os.makedirs(d)
        with open(os.path.join(d, 'invoice-partial.proof.json'), 'wb') as fh:
            fh.write(b'{}')
        meta = op.check_exactly_one_entry(d, 'invoice-partial.proof.json', 1024)
        assert meta['bytes'] == 2
        with open(os.path.join(d, 'n10-must-not-exist.proof.json'), 'wb') as fh:
            fh.write(b'{}')
        try:
            op.check_exactly_one_entry(d, 'invoice-partial.proof.json', 1024)
        except op.OpError as e:
            assert e.category == 'out', e.category
        else:
            raise AssertionError('expected stray-file OpError')
    check('exactly-one-output', t12)

    # Actual production placement predicate, not test-only booleans.
    # The expected layout list below is fixture data naming the exact
    # custody layout; the admission logic under test is the candidate's.
    def t13():
        expected = ['logs', 'journal', 'receipts', 'receipts/negatives', 'w', 'w/bin', 'w/in', 'w/out', 'w/stage']
        state = op.new_placement_state()
        assert not op.finalizer_eligible(state), state
        # No AUTH and no admission: nothing is eligible.
        state['authOk'] = True
        assert not op.finalizer_eligible(state), state
        # R1 admitted but refused/failed before any owned layout (the
        # pre-existing-T6 refusal case): state never records ownership.
        state['r1Ok'] = True
        assert not op.finalizer_eligible(state), state
        # Owned partial placement, as declared in the R0 receipt.
        op.record_r0_receipt(state, {'layoutComplete': False, 'layoutMade': ['logs'], 'inputsPrepared': False}, expected)
        assert not op.finalizer_eligible(state) and state['layoutMade'] == ['logs'], state
        # Owned fully placed run.
        op.record_r0_receipt(state, {'layoutComplete': True, 'layoutMade': list(expected), 'inputsPrepared': True}, expected)
        assert op.finalizer_eligible(state), state
        # A complete claim with a wrong made list fails closed.
        other = op.new_placement_state()
        other['authOk'] = True
        other['r1Ok'] = True
        try:
            op.record_r0_receipt(other, {'layoutComplete': True, 'layoutMade': ['logs'], 'inputsPrepared': True}, expected)
        except op.OpError as e:
            assert e.category == 'placement', e.category
        else:
            raise AssertionError('expected placement OpError for mismatched complete layout')
        assert not op.finalizer_eligible(other), other
    check('admission-placement-truth-table', t13)

    # Actual production S6N freeze partition over the fixed universe.
    def t14():
        full = list(op.S6N_SUCCESS_REQUIRED_RELS)
        assert len(full) == 9, full
        assert op.s6n_freeze_additions(False, []) == []
        one = 'receipts/negatives/n1-body-rederived.proof.json'
        assert op.s6n_freeze_additions(False, [one]) == [one]
        assert op.s6n_freeze_additions(True, full) == full
        try:
            op.s6n_freeze_additions(True, [one])
        except op.OpError as e:
            assert e.category == 'freeze', e.category
        else:
            raise AssertionError('expected freeze OpError for incomplete S6N success set')
        try:
            op.s6n_freeze_additions(False, ['receipts/negatives/undeclared.proof.json'])
        except op.OpError as e:
            assert e.category == 'freeze', e.category
        else:
            raise AssertionError('expected freeze OpError for undeclared S6N file')
    check('s6n-freeze-partition', t14)

    # Safely bounded: the escaped pipe holder exits after about 2s, so
    # no process is leaked; the runner must report timeout/non-clean
    # with drains joined once the pipe reaches EOF.
    def t15():
        script = (
            'import subprocess, time\n'
            'subprocess.Popen(["/usr/bin/python3", "-I", "-c", "import time; time.sleep(2)"], start_new_session=True)\n'
            'time.sleep(30)\n'
        )
        r = op.run_bounded_process([PY, '-I', '-c', script], cpu_cap=5, wall_cap=1, kill_after=1, stream_cap=1024)
        assert r['timedOut'] and not r['clean'] and r['drainJoined'], r
    check('escaped-pipe-holder-bounded-drain', t15)

    def t16():
        script = 'import json, resource; print(json.dumps({"cpu": list(resource.getrlimit(resource.RLIMIT_CPU)), "as": list(resource.getrlimit(resource.RLIMIT_AS)), "core": list(resource.getrlimit(resource.RLIMIT_CORE))}))'
        r = op.run_bounded_process([PY, '-I', '-c', script], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert r['clean'] and r['exit'] == 0, r
        observed = json.loads(r['_stdout'].decode('ascii'))
        assert observed == {'cpu': [5, 6], 'as': [8589934592, 8589934592], 'core': [0, 0]}, observed
    check('rlimit-grace-forwarding', t16)

    def t17():
        script = 'import json, operator, subprocess; print(json.dumps({"add": operator.add(20, 22), "operatorFile": operator.__file__, "subprocessFile": subprocess.__file__}))'
        r = op.run_bounded_process([PY, '-I', '-c', script], cpu_cap=5, wall_cap=5, kill_after=1, stream_cap=1024)
        assert r['clean'] and r['exit'] == 0, r
        observed = json.loads(r['_stdout'].decode('ascii'))
        assert observed['add'] == 42, observed
        assert os.path.realpath(observed['operatorFile']) != os.path.realpath(op.__file__), observed
        assert os.path.basename(observed['subprocessFile']) == 'subprocess.py', observed
    check('stdlib-operator-isolation', t17)

    for name, outcome in results:
        print(name + ': ' + outcome)
    failed = [n for n, o in results if o != 'ok']
    print(json.dumps({'total': len(results), 'failed': failed}))
    return 1 if failed else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
