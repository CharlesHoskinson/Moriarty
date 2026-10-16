#!/usr/bin/env python3
"""K6 one-shot guarded launcher. Source-only candidate; import does no I/O.

Verifies public root records and the future RR6 AUTH through externally
supplied hashes, then spawns exactly one reviewed operator invocation
and captures its parent stdout/stderr in bounded streaming form.
Protected work is delegated to the operator under the future AUTH.
"""
import hashlib
import json
import os
import resource
import stat
import subprocess
import sys
import threading
import time

BASE = '/home/charl/moriarty-eutxo-validation-2026-10-12'
T5 = BASE + '/current-cardano-k2-k5-single-party-test-setup-20261016'
T6 = BASE + '/current-cardano-k2-k6-proof-roundtrip-20261016'
M6 = BASE + '/current-cardano-k2-k6-proof-roundtrip-materialized-20261016'
RR6 = BASE + '/current-cardano-k2-k6-root-records-20261016'
AUTH_PATH = RR6 + '/AUTH.json'
RECORD_PATH = RR6 + '/AUTHORIZATION-RECORD.json'
LAUNCHER_PATH = RR6 + '/launcher.py'
STDOUT_PATH = RR6 + '/LAUNCH.stdout'
STDERR_PATH = RR6 + '/LAUNCH.stderr'
LAUNCH_RECORD_PATH = RR6 + '/LAUNCH-RECORD.json'
K5_ADOPTION_PATH = BASE + '/current-cardano-k2-k5-root-records-20261016/K5-ACTUAL-ADOPTION.json'
K5_ADOPTION_BYTES = 2339
K5_ADOPTION_SHA256 = '6ce800db246abfed5044645b499783c5a07ec07268dcc350bc916bf10ffc7e44'
K5_SCOPE_PATH = BASE + '/current-cardano-k2-k5-root-records-20261016/K5-EXACT-RUN-SCOPE.json'
K5_SCOPE_BYTES = 7862
K5_SCOPE_SHA256 = '3adaf14557393017dccb8ba837bb7f1a1769733de30fd7fd0d244cbf8cba6de5'
SOURCE_ADOPTION_PATH = BASE + '/current-cardano-k2-k6-integrated-operator-root-records-20261016/SOURCE-ADOPTION-R3.json'
SOURCE_ADOPTION_SHA256 = 'bfc7854a8c4d5b3957c5fdf1febc8a7e695ff74d92ff6588dd56e3b02b21fd0f'
STREAM_CAP = 4194304
POST_EXIT_DRAIN_SECONDS = 60
MAX_AUTH_BYTES = 1048576
MAX_RECORD_BYTES = 262144
MAX_PUBLIC_BYTES = 1048576
MAX_M6_BYTES = 8388608
MAX_LAUNCHER_BYTES = 1048576
EXPECTED_COUNTS = {'sandboxes': 19, 'driverCalls': 17, 'compile': 1, 'inspect': 1, 'proveArgv': 3, 'verifyArgv': 12, 'successfulProofs': 1, 'pkByteReads': 2, 'pkFullDecodes': 1, 'osRng': 1, 'setups': 0, 'retries': 0}
EXPECTED_SCOPE = 'one invoice/partial Groth16 roundtrip under single-party-test-setup/1 only'
RECORD_FIELDS = {'format', 'executionAuthorized', 'scope', 'authPath', 'authSha256', 'launcherPath', 'launcherSha256', 'm6SourcePins', 'sourceAdoptionSha256', 'freshSourceReviewSha256', 'ordinaryActualAdoption', 'newAuthProposalReview', 'k5ActualAdoptionSha256', 'k5ExactRunScopeSha256', 'counts', 'createdByRoot'}
HEX = frozenset('0123456789abcdef')


class LauncherError(Exception):
    pass


def fail(message):
    raise LauncherError(message)


def is_sha(value):
    return isinstance(value, str) and len(value) == 64 and all(c in HEX for c in value)


def sha_bytes(data):
    return hashlib.sha256(data).hexdigest()


def read_bounded(path, limit, label):
    try:
        st = os.lstat(path)
    except OSError as exc:
        fail(label + ': ' + str(exc))
    if not stat.S_ISREG(st.st_mode) or st.st_nlink != 1 or st.st_size < 0 or st.st_size > limit:
        fail(label + ': not a bounded regular file')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        out = b''
        while len(out) <= limit:
            chunk = os.read(fd, min(65536, limit + 1 - len(out)))
            if not chunk:
                break
            out += chunk
        if len(out) > limit:
            fail(label + ': exceeds bound')
        return out
    finally:
        os.close(fd)


def strict_json(data, label):
    def hook(pairs):
        obj = {}
        for key, value in pairs:
            if key in obj:
                fail(label + ': duplicate key')
            obj[key] = value
        return obj
    try:
        return json.loads(data.decode('utf-8'), object_pairs_hook=hook)
    except LauncherError:
        raise
    except Exception as exc:
        fail(label + ': ' + str(exc))


def write_new(path, data):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
    try:
        view = memoryview(data)
        off = 0
        while off < len(view):
            written = os.write(fd, view[off:])
            if written <= 0:
                fail('write failed: ' + path)
            off += written
        os.fsync(fd)
        os.fchmod(fd, 0o444)
    finally:
        os.close(fd)


def check_pin_map(pins, label):
    if not isinstance(pins, dict) or set(pins) != {'operator.py', 'custody.py', 'claims.py', 'probe-k6.sh'}:
        fail(label + ': pin set mismatch')
    for name in ('operator.py', 'custody.py', 'claims.py', 'probe-k6.sh'):
        pin = pins[name]
        if not isinstance(pin, dict) or set(pin) != {'bytes', 'sha256'} or not is_sha(pin.get('sha256')) or not isinstance(pin.get('bytes'), int):
            fail(label + ': malformed pin for ' + name)
        data = read_bounded(M6 + '/' + name, MAX_M6_BYTES, name)
        if len(data) != pin['bytes'] or sha_bytes(data) != pin['sha256']:
            fail(label + ': M6 pin mismatch for ' + name)


def verify_public_gates(record):
    adoption_raw = read_bounded(K5_ADOPTION_PATH, MAX_PUBLIC_BYTES, 'K5 adoption')
    if len(adoption_raw) != K5_ADOPTION_BYTES or sha_bytes(adoption_raw) != K5_ADOPTION_SHA256:
        fail('K5 actual adoption bytes/hash mismatch')
    if record.get('k5ActualAdoptionSha256') != K5_ADOPTION_SHA256:
        fail('record K5 adoption hash mismatch')
    adoption = strict_json(adoption_raw, 'K5 adoption')
    if adoption.get('format') != 'root-K5-first-setup-actual-adoption/1':
        fail('K5 adoption format mismatch')
    if adoption.get('actualSetupPrerequisiteClosed') is not True:
        fail('K5 actual setup prerequisite is not closed')
    if adoption.get('K6ExecutionAuthorized') is not False:
        fail('historical K5 record must not carry K6 execution authority')
    retention = adoption.get('retention')
    if not isinstance(retention, dict) or retention.get('clause') != 'DV-K2-1-R':
        fail('K5 retention clause mismatch')
    if retention.get('followup') != 'current-cardano-k2-k6-proof-roundtrip-20261016':
        fail('K5 retention followup mismatch')
    decision = retention.get('decision')
    if not isinstance(decision, str) or 'RETAIN' not in decision or 'no rootdirectread/copy/delete/publication or PK/proofoperationuntilnewexactreviewedK6operator+cryptoAUTH' not in decision:
        fail('K5 retention decision does not retain in place until new exact K6 AUTH')
    scope_raw = read_bounded(K5_SCOPE_PATH, MAX_PUBLIC_BYTES, 'K5 run scope')
    if len(scope_raw) != K5_SCOPE_BYTES or sha_bytes(scope_raw) != K5_SCOPE_SHA256:
        fail('K5 exact run scope bytes/hash mismatch')
    if record.get('k5ExactRunScopeSha256') != K5_SCOPE_SHA256:
        fail('record K5 run-scope hash mismatch')
    scope = strict_json(scope_raw, 'K5 run scope')
    if scope.get('format') != 'root-K5-exact-reviewed-run-scope/1':
        fail('K5 run scope is historical only and cannot be reused as K6 AUTH')
    source_raw = read_bounded(SOURCE_ADOPTION_PATH, MAX_PUBLIC_BYTES, 'source adoption')
    if sha_bytes(source_raw) != SOURCE_ADOPTION_SHA256 or record.get('sourceAdoptionSha256') != SOURCE_ADOPTION_SHA256:
        fail('source adoption hash mismatch')
    source = strict_json(source_raw, 'source adoption')
    if source.get('sourceReady') is not True or source.get('executionAuthorityGranted') is not False:
        fail('source adoption does not support source-only readiness without execution authority')
    ordinary = record.get('ordinaryActualAdoption')
    if not isinstance(ordinary, dict) or ordinary.get('adopted') is not True or not is_sha(ordinary.get('resultSha256')) or not is_sha(ordinary.get('reviewSha256')):
        fail('finite ordinary actual adoption gate is not closed in trusted record')
    review = record.get('newAuthProposalReview')
    if not isinstance(review, dict) or review.get('agreed') is not True or not is_sha(review.get('reviewSha256')):
        fail('fresh new AUTH/launcher proposal review is not recorded as agreed')
    if not is_sha(record.get('freshSourceReviewSha256')):
        fail('fresh source review hash missing from trusted record')


def verify_record_and_auth(expected_auth_sha, expected_record_sha):
    if not is_sha(expected_auth_sha) or not is_sha(expected_record_sha):
        fail('both externally supplied SHA-256 values are required; no defaults exist')
    record_raw = read_bounded(RECORD_PATH, MAX_RECORD_BYTES, 'root authorization record')
    if sha_bytes(record_raw) != expected_record_sha:
        fail('root authorization record bytes do not match external SHA-256')
    record = strict_json(record_raw, 'root authorization record')
    if not isinstance(record, dict) or set(record) != RECORD_FIELDS:
        fail('root authorization record field set mismatch')
    if record.get('format') != 'root-k6-authorization-record/1':
        fail('root authorization record format mismatch')
    if record.get('executionAuthorized') is not True:
        fail('executionAuthorized is not true in trusted record')
    if record.get('scope') != EXPECTED_SCOPE:
        fail('trusted record scope mismatch')
    if record.get('authPath') != AUTH_PATH or record.get('authSha256') != expected_auth_sha:
        fail('trusted record AUTH binding mismatch')
    if record.get('launcherPath') != LAUNCHER_PATH or not is_sha(record.get('launcherSha256')):
        fail('trusted record launcher binding mismatch')
    if record.get('counts') != EXPECTED_COUNTS:
        fail('trusted record counts mismatch')
    launcher_raw = read_bounded(os.path.abspath(__file__), MAX_LAUNCHER_BYTES, 'launcher')
    if sha_bytes(launcher_raw) != record['launcherSha256']:
        fail('pinned launcher bytes do not match trusted record')
    auth_raw = read_bounded(AUTH_PATH, MAX_AUTH_BYTES, 'AUTH')
    if sha_bytes(auth_raw) != expected_auth_sha:
        fail('AUTH bytes do not match external SHA-256')
    auth = strict_json(auth_raw, 'AUTH')
    if not isinstance(auth, dict) or set(auth) != {'format', 'pins', 'paths', 'order', 'nativeArgvs', 'caps', 'hostCaps', 'aggregate', 'counts', 'publication', 'effects'}:
        fail('AUTH field set mismatch')
    if auth.get('format') != 'k6-auth/1':
        fail('AUTH format mismatch')
    if auth.get('paths') != {'BASE': BASE, 'T5': T5, 'T6': T6, 'M6': M6, 'RR6': RR6}:
        fail('AUTH paths mismatch')
    if not isinstance(auth.get('order'), list) or len(auth['order']) != 19:
        fail('AUTH order count mismatch')
    if not isinstance(auth.get('nativeArgvs'), dict) or len(auth['nativeArgvs']) != 17:
        fail('AUTH native argv count mismatch')
    if not isinstance(auth.get('caps'), dict) or len(auth['caps']) != 19:
        fail('AUTH caps count mismatch')
    if not isinstance(auth.get('publication'), list) or len(auth['publication']) != 90:
        fail('AUTH publication count mismatch')
    if not isinstance(auth.get('effects'), list) or len(auth['effects']) != 27:
        fail('AUTH effects count mismatch')
    if auth.get('counts') != EXPECTED_COUNTS:
        fail('AUTH counts mismatch')
    host = auth.get('hostCaps')
    if not isinstance(host, dict) or host.get('cpu') != 300 or host.get('wall') != 600 or host.get('killAfter') != 60:
        fail('AUTH host caps mismatch')
    aggregate = auth.get('aggregate')
    if not isinstance(aggregate, dict) or aggregate.get('cpu') != 7200 or aggregate.get('wall') != 12600 or aggregate.get('streamCapBytes') != STREAM_CAP:
        fail('AUTH aggregate mismatch')
    check_pin_map(auth.get('pins'), 'AUTH')
    if record.get('m6SourcePins') != auth.get('pins'):
        fail('trusted record M6 pins differ from AUTH pins')
    verify_public_gates(record)
    if os.path.lexists(T6):
        fail('T6 already exists; fresh placement only')
    for path in (STDOUT_PATH, STDERR_PATH, LAUNCH_RECORD_PATH):
        if os.path.lexists(path):
            fail('launch output already exists: ' + path)
    return record


class Collector(threading.Thread):
    def __init__(self, fd):
        threading.Thread.__init__(self, daemon=True)
        self.fd = fd
        self.prefix = bytearray()
        self.full_hash = hashlib.sha256()
        self.observed = 0
        self.overflow = False
        self.complete = False
        self.read_error = False
        self._lock = threading.Lock()
    def run(self):
        try:
            while True:
                chunk = os.read(self.fd, 65536)
                if not chunk:
                    with self._lock:
                        self.complete = True
                    break
                with self._lock:
                    self.observed += len(chunk)
                    self.full_hash.update(chunk)
                    room = STREAM_CAP - len(self.prefix)
                    if room > 0:
                        self.prefix += chunk[:room]
                    if self.observed > STREAM_CAP:
                        self.overflow = True
        except OSError:
            with self._lock:
                self.read_error = True
    def snapshot(self):
        with self._lock:
            return (bytes(self.prefix), self.full_hash.copy().hexdigest(), self.observed, self.overflow, self.complete, self.read_error)


def drain_collectors(collectors, deadline, clock=time.monotonic):
    for collector in collectors:
        remaining = deadline - clock()
        if remaining > 0:
            collector.join(timeout=remaining)
    return all(not collector.is_alive() for collector in collectors)


def children_cpu():
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    return usage.ru_utime + usage.ru_stime


def parse_args(argv):
    if len(argv) != 5:
        fail('usage: launcher.py --expected-auth-sha256 HEX --expected-root-authorization-record-sha256 HEX')
    values = {}
    index = 1
    while index < len(argv):
        key = argv[index]
        if key not in ('--expected-auth-sha256', '--expected-root-authorization-record-sha256') or key in values or index + 1 >= len(argv):
            fail('usage: launcher.py --expected-auth-sha256 HEX --expected-root-authorization-record-sha256 HEX')
        values[key] = argv[index + 1]
        index += 2
    if set(values) != {'--expected-auth-sha256', '--expected-root-authorization-record-sha256'}:
        fail('both expected SHA-256 arguments are required')
    return values['--expected-auth-sha256'], values['--expected-root-authorization-record-sha256']


def main(argv):
    try:
        expected_auth_sha, expected_record_sha = parse_args(argv)
        verify_record_and_auth(expected_auth_sha, expected_record_sha)
    except LauncherError as exc:
        sys.stderr.write('BLOCKED(launcher): ' + str(exc) + '\n')
        return 2
    env = {'PATH': '/usr/bin:/bin', 'HOME': '/nonexistent', 'LC_ALL': 'C', 'TZ': 'UTC'}
    child_argv = ['/usr/bin/python3', '-I', M6 + '/operator.py', '--expected-auth-sha256', expected_auth_sha]
    before_cpu = children_cpu()
    start = time.monotonic()
    try:
        proc = subprocess.Popen(child_argv, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=M6, env=env, close_fds=True)
    except OSError as exc:
        sys.stderr.write('BLOCKED(launcher): operator launch failed: ' + str(exc) + '\n')
        return 1
    collectors = [Collector(proc.stdout.fileno()), Collector(proc.stderr.fileno())]
    for collector in collectors:
        collector.start()
    parent_exit = proc.wait()
    drain_deadline = time.monotonic() + POST_EXIT_DRAIN_SECONDS
    joined = drain_collectors(collectors, drain_deadline)
    out, err = collectors
    out_prefix, out_hash, out_observed, out_overflow, out_complete, out_error = out.snapshot()
    err_prefix, err_hash, err_observed, err_overflow, err_complete, err_error = err.snapshot()
    drain_complete = joined and out_complete and err_complete and not out_error and not err_error
    elapsed = time.monotonic() - start
    cpu_used = children_cpu() - before_cpu
    status = 'PASS' if parent_exit == 0 and drain_complete and not out_overflow and not err_overflow else 'FAIL'
    if parent_exit != 0 and drain_complete and not out_overflow and not err_overflow:
        status = 'INCOMPLETE'
    try:
        write_new(STDOUT_PATH, out_prefix)
        write_new(STDERR_PATH, err_prefix)
        record = {'format': 'root-k6-launch-record/1', 'status': status, 'parentExit': parent_exit, 'elapsedSeconds': elapsed, 'childrenCpuSeconds': cpu_used, 'authSha256': expected_auth_sha, 'rootAuthorizationRecordSha256': expected_record_sha, 'launcherSha256': sha_bytes(read_bounded(os.path.abspath(__file__), MAX_LAUNCHER_BYTES, 'launcher')), 'retries': 0, 'postExitDrainSecondsMax': POST_EXIT_DRAIN_SECONDS, 'stdout': {'path': STDOUT_PATH, 'prefixSha256': sha_bytes(out_prefix), 'fullStreamSha256': out_hash, 'observedBytes': out_observed, 'retainedBytes': len(out_prefix), 'overflow': out_overflow, 'drainComplete': drain_complete}, 'stderr': {'path': STDERR_PATH, 'prefixSha256': sha_bytes(err_prefix), 'fullStreamSha256': err_hash, 'observedBytes': err_observed, 'retainedBytes': len(err_prefix), 'overflow': err_overflow, 'drainComplete': drain_complete}}
        write_new(LAUNCH_RECORD_PATH, json.dumps(record, sort_keys=True).encode('ascii'))
    except Exception as exc:
        sys.stderr.write('BLOCKED(launcher): launch record write failed: ' + str(exc) + '\n')
        return 1
    return 0 if status == 'PASS' else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv))
