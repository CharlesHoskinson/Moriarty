"""K6 integrated operator, repaired. Source-only; import does no I/O.

One invoice/partial Groth16 roundtrip controller. Requires an externally
supplied expected AUTH SHA-256, verifies exact RR6 AUTH bytes and every
source pin before any protected action, fails closed otherwise. Helpers
load only from fixed M6 after AUTH pin checks. Repairs B01-B07 plus
M01-M05 from the r2 audit: custody-layout.log published; host steps run
in a supervised fork child with preemptive wall/CPU enforcement and
capped IPC; one generic streaming bounded-capture sandbox runner;
full-cap charging for signal/timeout/launch/measurement failure;
exactly-one-proof check on T6/w/out after N11; S6N effects amended to
include bounded negative export plus persisted metadata, with any
actual partial export preserved as INCOMPLETE failure evidence;
finalizers each attempted exactly once independently, admitted only
by an explicit run-owned placement state built from the structured
R0 receipt (never by directory existence), with a parent bounded
failure-receipt fallback in the owned tree and the guaranteed attempt
record on external stdout. Host caps are nominal 300 CPU / 600 wall
with a 60-second kill grace (hard kill at cap+60). Stdlib only.
"""
import hashlib
import json
import math
import os
import re
import resource
import select
import shutil
import signal
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
T6_OUT = T6 + '/w/out'
PROBE_SHA256 = '19638b9cd245e6963305df7d304639ddc2196c8644ed7c4fc93114cd3c0a2503'
DRIVER_SHA256 = 'd446ab0c0dececae719fe6583532f6c2a4285f67fbc89893d61251adff190902'
VK_DIGEST = 'd19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd'
AGG_CPU = 7200
AGG_WALL = 12600
KILL_AFTER = 60
AS_BYTES = 8589934592
HOST_CPU = 300
HOST_WALL = 600
HOST_IPC_CAP = 2097152
STREAM_CAP = 4194304
MEM_MIN_KIB = 10485760
DISK_MIN_BYTES = 6442450944
ORDER = ['Q6N', 'A6', 'I6', 'Q6P', 'P6', 'V6-source', 'V6-export', 'N1', 'N2', 'N3', 'N4', 'N5', 'N6', 'N7', 'N8', 'N9', 'N10', 'N11', 'usage']
DRIVER_ORDER = ['A6', 'I6', 'P6', 'V6-source', 'V6-export', 'N1', 'N2', 'N3', 'N4', 'N5', 'N6', 'N7', 'N8', 'N9', 'N10', 'N11', 'usage']
KEY_SANDBOXES = frozenset(['Q6P', 'P6', 'N11'])
EXPECTED_EXIT = {'Q6N': 0, 'A6': 0, 'I6': 0, 'Q6P': 0, 'P6': 0, 'V6-source': 0, 'V6-export': 0, 'N1': 1, 'N2': 1, 'N3': 1, 'N4': 1, 'N5': 1, 'N6': 1, 'N7': 1, 'N8': 1, 'N9': 1, 'N10': 1, 'N11': 1, 'usage': 2}
NS_RE = re.compile(r'^(user|pid|net|ipc|uts):\[[0-9]+\]$')
SHA_RE = re.compile(r'^[0-9a-f]{64}$')


class OpError(Exception):
    def __init__(self, category, message):
        self.category = category
        super().__init__('BLOCKED(%s): %s' % (category, message))


def _fail(category, message):
    raise OpError(category, message)


def _caps(sid):
    if sid in ('Q6N', 'Q6P'):
        return (120, 300)
    if sid == 'P6':
        return (3600, 3600)
    if sid in ('A6', 'I6', 'V6-source', 'N10', 'N11'):
        return (600, 900)
    return (300, 600)


CAPS = dict((sid, {'cpu': _caps(sid)[0], 'wall': _caps(sid)[1], 'killAfter': KILL_AFTER, 'asBytes': AS_BYTES, 'coreBytes': 0, 'nice': 10}) for sid in ORDER)
HOST_CAPS = {'cpu': HOST_CPU, 'wall': HOST_WALL, 'killAfter': KILL_AFTER, 'supervision': 'supervised fork child; RLIMIT_CPU soft cpu hard cpu+60; nominal wall cap with 60s kill grace, hard kill and failure at wall+60; IPC cap 2097152 bytes; process-group kill and wait4 reap; named subeffects with no separate effect or native call: key postcheck after Q6P is a subeffect of Q6P, the pre-prove memory/disk gate and key postcheck after P6 are subeffects of P6, key postcheck after N11 is a subeffect of N11, key postcheck in Z61 is a subeffect of Z61'}
COUNTS = {'sandboxes': 19, 'driverCalls': 17, 'compile': 1, 'inspect': 1, 'proveArgv': 3, 'verifyArgv': 12, 'successfulProofs': 1, 'pkByteReads': 2, 'pkFullDecodes': 1, 'osRng': 1, 'setups': 0, 'retries': 0}
# B01: logs/custody-layout.log is always created by custody.create_t6_layout
# and walked by freeze, so it is a required publication candidate.
PUB_CANDIDATES = tuple(['logs/custody-layout.log'] + ['logs/%s.stdout' % s for s in ORDER] + ['logs/%s.stderr' % s for s in ORDER] + ['logs/%s.exit.json' % s for s in ORDER] + ['journal/%s.json' % s for s in ORDER] + ['receipts/invoice-partial.proof.json', 'receipts/g6.json', 'receipts/h6.json', 'receipts/z61.json', 'receipts/negatives-metadata.json'] + ['receipts/negatives/n%d-%s' % (i, n) for i, n in [(1, 'body-rederived.proof.json'), (2, 'context-rederived.proof.json'), (3, 'proof-ac-swapped.proof.json'), (4, 'x-swapped.proof.json'), (5, 'statement-stale-digest.proof.json'), (6, 'proof-truncated.proof.json'), (7, 'proof-identity.proof.json')]] + ['receipts/negatives/n10-finish-selector.request.json'])
NEGATIVE_BODY_RELS = tuple('receipts/negatives/n%d-%s' % (i, n) for i, n in [(1, 'body-rederived.proof.json'), (2, 'context-rederived.proof.json'), (3, 'proof-ac-swapped.proof.json'), (4, 'x-swapped.proof.json'), (5, 'statement-stale-digest.proof.json'), (6, 'proof-truncated.proof.json'), (7, 'proof-identity.proof.json')] + [(10, 'finish-selector.request.json')])
# S6N freeze partition: success MUST present every rel below; failure MAY
# present any subset of exactly this fixed published-path universe and
# nothing else. Partial files are preserved and frozen as INCOMPLETE
# evidence; they are never labelled an S6N success.
S6N_SUCCESS_REQUIRED_RELS = tuple(['receipts/negatives-metadata.json'] + list(NEGATIVE_BODY_RELS))
S6N_FAILURE_MAY_RELS = S6N_SUCCESS_REQUIRED_RELS


def _sha_bytes(data):
    return hashlib.sha256(data).hexdigest()


def _read_bounded(path, limit, label):
    try:
        st = os.lstat(path)
    except OSError as e:
        _fail('open', label + ': ' + str(e))
    import stat as _stat
    if not _stat.S_ISREG(st.st_mode) or st.st_nlink != 1 or st.st_size < 0 or st.st_size > limit:
        _fail('cap', label + ': not a bounded regular file')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        out = b''
        while len(out) <= limit:
            chunk = os.read(fd, min(65536, limit + 1 - len(out)))
            if not chunk:
                break
            out += chunk
        if len(out) > limit:
            _fail('cap', label + ': exceeds bound')
        return out
    finally:
        os.close(fd)


def _write_new(path, data, mode):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
    try:
        view = memoryview(data)
        off = 0
        while off < len(view):
            n = os.write(fd, view[off:])
            if n <= 0:
                _fail('write', path)
            off += n
        os.fsync(fd)
        os.fchmod(fd, mode)
    finally:
        os.close(fd)


def _write_receipt(path, obj):
    # Create-exclusive; a pre-existing receipt is left untouched and reported.
    if os.path.lexists(path):
        return False
    _write_new(path, json.dumps(obj, sort_keys=True).encode('ascii'), 0o444)
    return True


def _strict_json(data, label):
    def hook(pairs):
        obj = {}
        for k, v in pairs:
            if k in obj:
                _fail('json', label + ': duplicate key')
            obj[k] = v
        return obj
    try:
        return json.loads(data.decode('utf-8'), object_pairs_hook=hook)
    except OpError:
        raise
    except Exception as e:
        _fail('json', label + ': ' + str(e))


def _prefix(keyed):
    argv = ['/usr/bin/bwrap', '--unshare-user', '--unshare-pid', '--unshare-net', '--unshare-ipc', '--unshare-uts', '--die-with-parent', '--new-session', '--ro-bind', '/usr', '/usr', '--ro-bind', '/bin', '/bin', '--ro-bind', '/lib', '/lib', '--ro-bind', '/lib64', '/lib64', '--ro-bind-try', '/etc/ld.so.cache', '/etc/ld.so.cache', '--dev', '/dev', '--proc', '/proc', '--size', '268435456', '--tmpfs', '/tmp', '--ro-bind', T6 + '/w/bin', '/task/bin', '--ro-bind', T6 + '/w/in', '/task/in', '--ro-bind', T6 + '/w/stage', '/task/stage', '--bind', T6 + '/w/out', '/task/out']
    if keyed:
        argv += ['--ro-bind', T5 + '/w/keys', '/task/keys-ro']
    argv += ['--chdir', '/task', '--clearenv', '--setenv', 'PATH', '/usr/bin:/bin', '--setenv', 'HOME', '/nonexistent', '--']
    return argv


def _native_argvs():
    binp = '/task/bin/mori-k2'
    src = '/task/in/installment-invoice.mori'
    req = '/task/in/invoice-partial.request.json'
    rec = '/task/in/invoice-partial.setup.json'
    proof = '/task/out/invoice-partial.proof.json'
    return {
        'A6': [binp, 'compile', '--source', src, '--selector', '/task/in/invoice-partial.selector.json'],
        'I6': [binp, 'inspect', '--source', src, '--request', req, '--vk-digest', VK_DIGEST],
        'P6': [binp, 'prove', '--source', src, '--request', req, '--setup-record', rec, '--proving-key', '/task/keys-ro/invoice-partial.pk', '--out-proof', proof],
        'V6-source': [binp, 'verify', '--setup-record', rec, '--proof', proof, '--source', src],
        'V6-export': [binp, 'verify', '--setup-record', rec, '--proof', proof],
        'N1': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n1-body-rederived.proof.json'],
        'N2': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n2-context-rederived.proof.json'],
        'N3': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n3-proof-ac-swapped.proof.json'],
        'N4': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n4-x-swapped.proof.json'],
        'N5': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n5-statement-stale-digest.proof.json'],
        'N6': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n6-proof-truncated.proof.json'],
        'N7': [binp, 'verify', '--setup-record', rec, '--proof', '/task/stage/n7-proof-identity.proof.json'],
        'N8': [binp, 'verify', '--setup-record', '/task/stage/n8-record-vk-digest.setup.json', '--proof', proof],
        'N9': [binp, 'verify', '--setup-record', rec, '--proof', proof, '--source', '/task/in/conditional-reserve.mori'],
        'N10': [binp, 'prove', '--source', src, '--request', '/task/stage/n10-finish-selector.request.json', '--setup-record', rec, '--proving-key', '/task/keys-ro/invoice-partial.pk', '--out-proof', '/task/out/n10-must-not-exist.proof.json'],
        'N11': [binp, 'prove', '--source', src, '--request', req, '--setup-record', '/task/stage/n11-key-size.setup.json', '--proving-key', '/task/keys-ro/invoice-partial.pk', '--out-proof', '/task/out/n11-must-not-exist.proof.json'],
        'usage': [binp, 'verify', '--setup-record', rec],
    }


def _load_auth(expected_sha):
    if not isinstance(expected_sha, str) or not SHA_RE.match(expected_sha):
        _fail('authority', 'externally supplied expected AUTH SHA-256 is absent or malformed; no default exists')
    raw = _read_bounded(AUTH_PATH, 1048576, 'AUTH')
    if _sha_bytes(raw) != expected_sha:
        _fail('authority', 'RR6 AUTH bytes do not match the externally supplied SHA-256')
    auth = _strict_json(raw, 'AUTH')
    if not isinstance(auth, dict) or set(auth) != {'format', 'pins', 'paths', 'order', 'nativeArgvs', 'caps', 'hostCaps', 'aggregate', 'counts', 'publication', 'effects'}:
        _fail('authority', 'AUTH field set mismatch')
    if auth['format'] != 'k6-auth/1':
        _fail('authority', 'AUTH format mismatch')
    if auth['paths'] != {'BASE': BASE, 'T5': T5, 'T6': T6, 'M6': M6, 'RR6': RR6}:
        _fail('authority', 'AUTH paths mismatch')
    if auth['order'] != ORDER or auth['nativeArgvs'] != _native_argvs():
        _fail('authority', 'AUTH order or native argv mismatch')
    if auth['caps'] != CAPS or auth['hostCaps'] != HOST_CAPS or auth['counts'] != COUNTS:
        _fail('authority', 'AUTH caps or counts mismatch')
    if auth['aggregate'] != {'cpu': AGG_CPU, 'wall': AGG_WALL, 'killAfter': KILL_AFTER, 'charging': 'clean exit charges ceil CHILDREN user+sys and ceil monotonic; signal/timeout/cap-kill/launch-failure/missing-measurement charges full cap plus killAfter', 'launchRule': 'remaining >= next cap + 60 for both CPU and wall', 'streamCapBytes': STREAM_CAP}:
        _fail('authority', 'AUTH aggregate mismatch')
    if auth['publication'] != list(PUB_CANDIDATES):
        _fail('authority', 'AUTH publication allowlist mismatch')
    if auth['effects'] != ['R1', 'R0', 'Q6N', 'A6', 'I6', 'Q6P', 'P6', 'V6-source', 'V6-export', 'S6N', 'N1', 'N2', 'N3', 'N4', 'N5', 'N6', 'N7', 'N8', 'N9', 'N10', 'N11', 'usage', 'OUT-CHECK', 'G6', 'H6', 'Z61', 'Z62']:
        _fail('authority', 'AUTH effects mismatch')
    pins = auth['pins']
    if not isinstance(pins, dict):
        _fail('authority', 'AUTH pins missing')
    for name in ('operator.py', 'custody.py', 'claims.py', 'probe-k6.sh'):
        pin = pins.get(name)
        if not isinstance(pin, dict) or set(pin) != {'bytes', 'sha256'} or not SHA_RE.match(str(pin.get('sha256'))):
            _fail('authority', 'AUTH pin malformed for ' + name)
        data = _read_bounded(M6 + '/' + name, 8388608, name)
        if len(data) != pin['bytes'] or _sha_bytes(data) != pin['sha256']:
            _fail('authority', 'M6 source pin mismatch for ' + name)
    if pins['probe-k6.sh']['sha256'] != PROBE_SHA256:
        _fail('authority', 'probe text is not the preserved R2 bytes')
    probe = _read_bounded(M6 + '/probe-k6.sh', 1048576, 'probe')
    return auth, probe


def _import_helpers():
    if M6 not in sys.path:
        sys.path.insert(0, M6)
    import custody
    import claims
    if custody.T6 != T6 or custody.T5 != T5 or custody.BINARY_SHA256 != DRIVER_SHA256:
        _fail('identity', 'custody module constants do not match the fixed K6 paths or binary pin')
    return custody, claims


def _children_cpu():
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    return usage.ru_utime + usage.ru_stime


def _limits(cpu, as_bytes=AS_BYTES, kill_grace=KILL_AFTER):
    if kill_grace < 0:
        raise ValueError('kill_grace must be non-negative')
    def apply():
        resource.setrlimit(resource.RLIMIT_CPU, (cpu, cpu + kill_grace))
        resource.setrlimit(resource.RLIMIT_AS, (as_bytes, as_bytes))
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        try:
            os.nice(10)
        except OSError:
            pass
    return apply


# ---------- generic supervision core (also exercised by selftest) ----------

def run_supervised_callable(fn, cpu_cap=HOST_CPU, wall_cap=HOST_WALL, kill_after=KILL_AFTER, ipc_cap=HOST_IPC_CAP):
    """Run fn() in a forked child with preemptive bounds. B02.

    Enforcement, not just constants: the child gets RLIMIT_CPU
    (cpu_cap, cpu_cap+kill_after) and its own session/process group;
    the parent polls with a monotonic wall deadline, drains the result
    pipe with select under a hard ipc_cap, and on deadline, overflow,
    lost result, or child failure sends SIGKILL to the whole process
    group and reaps with wait4 so no descendant is left running. The
    return value must be JSON-serializable; the child sends exactly one
    JSON object {ok, value|error, category}. Wrapper mechanics (fork,
    pipe drain, reap) run inside the same wall deadline. Limits: a
    forked child shares already-open fds and memory copy-on-write with
    the parent; helpers' own fd ownership/process semantics are
    unchanged because custody/claims open and close their own fds in
    the child exactly as they would in-process, and only the small
    JSON result crosses back. A child that ignores SIGKILL cannot be
    reaped faster than the kernel allows; the parent then fails closed.
    """
    rfd, wfd = os.pipe()
    os.set_blocking(rfd, False)
    start = time.monotonic()
    deadline = start + wall_cap + kill_after
    pid = os.fork()
    if pid == 0:
        try:
            os.close(rfd)
            os.setsid()
            resource.setrlimit(resource.RLIMIT_CPU, (cpu_cap, cpu_cap + kill_after))
            resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
            try:
                value = fn()
                payload = json.dumps({'ok': True, 'value': value}).encode('ascii')
                code = 0
            except OpError as e:
                payload = json.dumps({'ok': False, 'error': str(e), 'category': e.category}).encode('ascii')
                code = 3
            except Exception as e:
                payload = json.dumps({'ok': False, 'error': str(e), 'category': 'host'}).encode('ascii')
                code = 3
            if len(payload) > ipc_cap:
                payload = json.dumps({'ok': False, 'error': 'host result exceeds IPC cap', 'category': 'cap'}).encode('ascii')
                code = 3
            view = memoryview(payload)
            while view:
                n = os.write(wfd, view)
                view = view[n:]
            os._exit(code)
        except BaseException:
            os._exit(4)
    os.close(wfd)
    buf = b''
    overflow = False
    status = None
    rusage = None
    killed = False
    try:
        while True:
            if status is None:
                wpid, st, ru = os.wait4(pid, os.WNOHANG)
                if wpid == pid:
                    status, rusage = st, ru
            remaining = deadline - time.monotonic()
            if remaining <= 0 and status is None:
                killed = True
                try:
                    os.killpg(pid, signal.SIGKILL)
                except OSError:
                    pass
                _, status, rusage = os.wait4(pid, 0)
                break
            if status is not None:
                # Drain whatever the child wrote before exiting.
                while True:
                    try:
                        chunk = os.read(rfd, 65536)
                    except BlockingIOError:
                        break
                    if not chunk:
                        break
                    buf += chunk
                    if len(buf) > ipc_cap:
                        overflow = True
                        break
                break
            ready, _, _ = select.select([rfd], [], [], min(0.05, max(0.0, remaining)))
            if ready:
                chunk = os.read(rfd, 65536)
                if chunk:
                    buf += chunk
                    if len(buf) > ipc_cap:
                        overflow = True
                        killed = True
                        try:
                            os.killpg(pid, signal.SIGKILL)
                        except OSError:
                            pass
                        _, status, rusage = os.wait4(pid, 0)
                        break
    finally:
        os.close(rfd)
        if status is None:
            try:
                os.killpg(pid, signal.SIGKILL)
            except OSError:
                pass
            _, status, rusage = os.wait4(pid, 0)
            killed = True
    wall = time.monotonic() - start
    cpu_used = -1.0
    if rusage is not None:
        cpu_used = rusage.ru_utime + rusage.ru_stime
    if overflow:
        _fail('cap', 'supervised host result exceeds IPC cap')
    if killed or wall > wall_cap + kill_after:
        _fail('resource', 'host step exceeded wall deadline and was killed')
    if cpu_used < 0:
        _fail('resource', 'host step CPU measurement missing')
    if cpu_used > cpu_cap + kill_after:
        _fail('resource', 'host step exceeded CPU cap')
    if not buf:
        _fail('host', 'supervised host child returned no result (lost result)')
    try:
        result = json.loads(buf.decode('ascii'))
    except Exception:
        _fail('host', 'supervised host child returned malformed result')
    if not isinstance(result, dict) or 'ok' not in result:
        _fail('host', 'supervised host child returned malformed result')
    if not result['ok']:
        raise OpError(result.get('category', 'host'), result.get('error', 'host step failed'))
    return result.get('value')


def _host_step(fn, cpu_cap=HOST_CPU, wall_cap=HOST_WALL):
    return run_supervised_callable(fn, cpu_cap, wall_cap, KILL_AFTER, HOST_IPC_CAP)


class _StreamDrain(threading.Thread):
    def __init__(self, fd, cap, killfn):
        super().__init__(daemon=True)
        self.fd = fd
        self.cap = cap
        self.killfn = killfn
        self.prefix = bytearray()
        self.total = 0
        self.overflow = False
    def run(self):
        try:
            while True:
                chunk = os.read(self.fd, 65536)
                if not chunk:
                    break
                self.total += len(chunk)
                room = self.cap + 1 - len(self.prefix)
                if room > 0:
                    self.prefix += chunk[:room]
                if self.total > self.cap and not self.overflow:
                    self.overflow = True
                    self.killfn()
        except OSError:
            pass


def run_bounded_process(argv, cpu_cap, wall_cap, kill_after=KILL_AFTER, stream_cap=STREAM_CAP, as_bytes=AS_BYTES):
    """One generic bounded runner for every sandbox. B03/B04 mechanics.

    Popen with start_new_session (own process group; the bwrap tail
    adds --die-with-parent), stdin DEVNULL, stdout/stderr drained by
    two threads that keep only a bounded prefix and kill the whole
    process group the moment either stream exceeds stream_cap during
    the run, so parent memory stays O(stream_cap), never O(output).
    Wall deadline is wall_cap+kill_after; on expiry the group gets
    SIGKILL and is waited on. Process-group kill reaches group members;
    a descendant that leaves the group is not claimed killed by this
    runner alone (the native path additionally relies on the bwrap
    --die-with-parent/pid-namespace contract). Drain threads are joined
    under a bounded deadline; if a pipe-holding descendant keeps a
    drain thread blocked past that deadline the run is flagged
    non-clean (drainJoined false) instead of treating the prefix as
    complete. Returns a JSON-safe record plus the bounded
    _stdout/_stderr prefixes; clean means the process exited on its
    own (any exit code, including native 1 / usage 2) with both drains
    joined, anything else (signal, timeout, cap kill, launch failure,
    missing CPU measurement, unjoined drain) is non-clean.
    """
    before_cpu = _children_cpu()
    start = time.monotonic()
    try:
        proc = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, preexec_fn=_limits(cpu_cap, as_bytes, kill_grace=kill_after), start_new_session=True, close_fds=True)
    except OSError as e:
        return {'launchError': str(e), 'exit': None, 'timedOut': False, 'overflow': False, 'clean': False, 'wallSeconds': time.monotonic() - start, 'cpuSeconds': -1.0, '_stdout': b'', '_stderr': b''}
    def kill_group():
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except OSError:
            pass
    drains = [_StreamDrain(proc.stdout.fileno(), stream_cap, kill_group), _StreamDrain(proc.stderr.fileno(), stream_cap, kill_group)]
    for d in drains:
        d.start()
    timed_out = False
    try:
        code = proc.wait(timeout=wall_cap + kill_after)
    except subprocess.TimeoutExpired:
        timed_out = True
        kill_group()
        code = proc.wait()
    for d in drains:
        d.join(timeout=10)
    drain_unjoined = any(d.is_alive() for d in drains)
    if drain_unjoined:
        # A pipe-holding descendant may keep a drain blocked; closing the
        # parent pipe ends gives it one bounded chance to finish, and the
        # run stays non-clean either way so the prefix is not trusted as
        # complete and no stale reader is silently carried forward.
        try:
            proc.stdout.close()
        except Exception:
            pass
        try:
            proc.stderr.close()
        except Exception:
            pass
        for d in drains:
            d.join(timeout=5)
        drain_unjoined = any(d.is_alive() for d in drains)
    else:
        try:
            proc.stdout.close()
            proc.stderr.close()
        except Exception:
            pass
    wall = time.monotonic() - start
    cpu_delta = _children_cpu() - before_cpu
    overflow = drains[0].overflow or drains[1].overflow
    clean = (not timed_out) and (not overflow) and (not drain_unjoined) and code is not None and code >= 0 and cpu_delta >= 0
    return {'exit': code, 'timedOut': timed_out, 'overflow': overflow, 'drainJoined': not drain_unjoined, 'clean': clean, 'wallSeconds': wall, 'cpuSeconds': cpu_delta, '_stdout': bytes(drains[0].prefix[:stream_cap]), '_stderr': bytes(drains[1].prefix[:stream_cap]), 'stdoutBytesObserved': drains[0].total, 'stderrBytesObserved': drains[1].total}


def check_exactly_one_entry(dirpath, name, max_bytes):
    """B05: bounded guarded directory-entry metadata check."""
    import stat as _stat
    fd = os.open(dirpath, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        names = os.listdir(fd)
        if names != [name]:
            _fail('out', 'output directory must contain exactly ' + name + '; found ' + repr(sorted(names)))
        st = os.stat(name, dir_fd=fd, follow_symlinks=False)
        if not _stat.S_ISREG(st.st_mode) or st.st_nlink != 1 or st.st_size < 0 or st.st_size > max_bytes:
            _fail('out', 'sole output entry is not a bounded regular file')
        return {'name': name, 'bytes': st.st_size}
    finally:
        os.close(fd)


def attempt_finalizers(finalizers):
    """B07: attempt each (name, fn) exactly once, independently."""
    results = {}
    for name, fn in finalizers:
        try:
            results[name] = {'attempted': True, 'ok': bool(fn())}
        except Exception as e:
            results[name] = {'attempted': True, 'ok': False, 'error': str(e)}
    return results


def new_placement_state():
    """Explicit run-owned placement/admission state for this process.

    Ownership is established only by the structured R0 receipt returned
    through the supervised host step, never by path or directory
    existence and never by a flag set before creation is attempted.
    """
    return {'authOk': False, 'r1Ok': False, 'layoutCompleteOwned': False, 'layoutPartialOwned': False, 'layoutMade': [], 'inputsPrepared': False}


def record_r0_receipt(state, receipt, expected_made):
    """Fold one structured R0 receipt into the placement state.

    layoutCompleteOwned is set only when the receipt asserts a complete
    layout and its made list equals the exact expected layout; a partial
    receipt records only the custody-reported made subset and stays
    ineligible for finalizers and freeze.
    """
    if not isinstance(receipt, dict):
        _fail('placement', 'R0 receipt is not a structured object')
    made = receipt.get('layoutMade')
    if not isinstance(made, list) or not all(isinstance(x, str) for x in made):
        _fail('placement', 'R0 receipt lacks a structured layoutMade list')
    state['layoutMade'] = list(made)
    state['inputsPrepared'] = receipt.get('inputsPrepared') is True
    if receipt.get('layoutComplete') is True:
        if tuple(made) != tuple(expected_made):
            _fail('placement', 'R0 receipt claims a complete layout but its made list differs from the exact expected layout')
        state['layoutCompleteOwned'] = True
        state['layoutPartialOwned'] = False
    else:
        state['layoutCompleteOwned'] = False
        state['layoutPartialOwned'] = True
    return state


def finalizer_eligible(state):
    """Pure admission predicate: valid AUTH, admitted R1, and a complete
    layout this run created, all established in the placement state."""
    return bool(isinstance(state, dict) and state.get('authOk') and state.get('r1Ok') and state.get('layoutCompleteOwned'))


def s6n_freeze_additions(s6n_done, present_rels):
    """Pure S6N freeze partition over the fixed published-path universe.

    Success requires every S6N_SUCCESS_REQUIRED_RELS entry (all ten
    checked metadata are enforced inside S6N itself before it returns);
    failure preserves exactly the actual subset present, and any rel
    outside the fixed universe is a freeze failure, never approval.
    """
    present = frozenset(present_rels)
    unknown = present - frozenset(S6N_FAILURE_MAY_RELS)
    if unknown:
        _fail('freeze', 'undeclared S6N file outside the fixed universe: ' + repr(sorted(unknown)))
    if s6n_done:
        missing = [rel for rel in S6N_SUCCESS_REQUIRED_RELS if rel not in present]
        if missing:
            _fail('freeze', 'S6N success set incomplete: ' + repr(missing))
        return list(S6N_SUCCESS_REQUIRED_RELS)
    return [rel for rel in S6N_FAILURE_MAY_RELS if rel in present]


class Runner:
    def __init__(self, custody):
        self.custody = custody
        self.used_cpu = 0
        self.used_wall = 0
        self.outcomes = {}
        self.records = {}
        self.prefix_hash = {}
        self.blocked = []
        self.dispatched = []
    def _log(self, sid, stdout, stderr, record):
        _write_new(T6 + '/logs/' + sid + '.stdout', stdout, 0o444)
        _write_new(T6 + '/logs/' + sid + '.stderr', stderr, 0o444)
        body = json.dumps(record, sort_keys=True).encode('ascii')
        _write_new(T6 + '/logs/' + sid + '.exit.json', body, 0o444)
        _write_new(T6 + '/journal/' + sid + '.json', body, 0o444)
    def run(self, sid, tail, keyed):
        cpu_cap, wall_cap = _caps(sid)
        argv = _prefix(keyed) + tail
        # B04 launch rule: never launch unless both remainders cover cap+60.
        if AGG_CPU - self.used_cpu < cpu_cap + KILL_AFTER or AGG_WALL - self.used_wall < wall_cap + KILL_AFTER:
            record = {'id': sid, 'status': 'BLOCKED(resource-aggregate)', 'argv': tail, 'chargeCpu': 0, 'chargeWall': 0, 'aggregateUsedCpu': self.used_cpu, 'aggregateUsedWall': self.used_wall}
            self.records[sid] = record
            self.blocked.append(sid)
            self._log(sid, b'', b'', record)
            return record
        family = 'key' if keyed else 'nokey'
        digest = _sha_bytes(json.dumps(_prefix(keyed)).encode('ascii'))
        if family in self.prefix_hash and self.prefix_hash[family] != digest:
            _fail('sandbox', 'constant bwrap prefix changed within family ' + family)
        self.prefix_hash[family] = digest
        self.dispatched.append(sid)
        result = run_bounded_process(argv, cpu_cap, wall_cap, KILL_AFTER, STREAM_CAP, AS_BYTES)
        stdout = result.pop('_stdout')
        stderr = result.pop('_stderr')
        # B04/M01: charge and record BEFORE any cap failure can propagate.
        if result.get('clean'):
            charge_cpu = int(math.ceil(max(0.0, result['cpuSeconds'])))
            charge_wall = int(math.ceil(result['wallSeconds']))
        else:
            charge_cpu = cpu_cap + KILL_AFTER
            charge_wall = wall_cap + KILL_AFTER
        self.used_cpu += charge_cpu
        self.used_wall += charge_wall
        record = {'id': sid, 'argv': argv, 'exit': result.get('exit'), 'timedOut': result['timedOut'], 'streamOverflow': result['overflow'], 'clean': result['clean'], 'chargeCpu': charge_cpu, 'chargeWall': charge_wall, 'aggregateUsedCpu': self.used_cpu, 'aggregateUsedWall': self.used_wall, 'prefixSha256': digest, 'stdoutSha256': _sha_bytes(stdout), 'stderrSha256': _sha_bytes(stderr)}
        if 'launchError' in result:
            record['launchError'] = result['launchError']
            record['status'] = 'FAILED(launch)'
        self.records[sid] = record
        self._log(sid, stdout, stderr, record)
        if sid in DRIVER_ORDER:
            self.outcomes[sid] = {'exit': result.get('exit') if result.get('exit') is not None else -1, 'error': stderr.decode('utf-8', 'replace')}
        record['_stdout'] = stdout
        record['_stderr'] = stderr
        if not result['clean']:
            _fail('sandbox', sid + ' did not terminate cleanly (signal/timeout/cap/launch/measurement); charged full cap plus 60')
        if result['overflow']:
            _fail('cap', sid + ' output exceeds bounded log cap')
        return record
    def block_rest(self, start_index, reason):
        for sid in ORDER[start_index:]:
            if sid not in self.records:
                record = {'id': sid, 'status': 'BLOCKED(' + reason + ')', 'chargeCpu': 0, 'chargeWall': 0, 'aggregateUsedCpu': self.used_cpu, 'aggregateUsedWall': self.used_wall}
                self.records[sid] = record
                self.blocked.append(sid)
                self._log(sid, b'', b'', record)


def _probe_tail(probe_text):
    values = []
    for name in ('user', 'pid', 'net', 'ipc', 'uts'):
        value = os.readlink('/proc/self/ns/' + name)
        if not NS_RE.match(value) or not value.startswith(name + ':'):
            _fail('prerequisite', 'host namespace value malformed for ' + name)
        values.append(value)
    return ['/usr/bin/bash', '--noprofile', '--norc', '-c', probe_text, 'probe-k6', T5] + values


def _check_probe(record, mode, expected_key):
    if record.get('exit') != 0:
        _fail('hygiene', record['id'] + ' probe exit is not 0')
    text = record['_stdout'].decode('ascii', 'strict')
    lines = text.splitlines()
    if 'PROBE-FAIL' in text or not lines or lines[-1] != 'probe-ok mode=' + mode:
        _fail('hygiene', record['id'] + ' lacks a unique terminal probe-ok for mode ' + mode)
    if text.count('probe-ok ') != 1:
        _fail('hygiene', record['id'] + ' probe-ok is not unique')
    if mode == 'Q6P':
        pk = expected_key['pk']
        want = 'record keytuple=%d:%d:%d:%s:%d:%d:%d' % (pk['dev'], pk['ino'], pk['size'], format(pk['mode'], 'o'), pk['nlink'], pk['mtime_ns'] // 1000000000, pk['ctime_ns'] // 1000000000)
        if want not in lines:
            _fail('hygiene', 'Q6P key tuple differs from host pre-metadata')
    return text


def _mem_disk_gates():
    mem = None
    with open('/proc/meminfo', 'r', encoding='ascii') as handle:
        for line in handle:
            if line.startswith('MemAvailable:'):
                mem = int(line.split()[1])
    if mem is None or mem < MEM_MIN_KIB:
        _fail('resource', 'MemAvailable is below 10485760 KiB before prove')
    if shutil.disk_usage(T6).free < DISK_MIN_BYTES:
        _fail('resource', 'free disk is below 6442450944 bytes before prove')
    return {'memAvailableKiB': mem}


def _required_publication(s6n_done, proof_exported, s6n_present=()):
    # M02: required set derived from run state; existence is checked, never a filter.
    # S6N is the one declared exception: after a failed S6N the actual
    # subset of the fixed MAY universe is required, so partial failure
    # evidence freezes instead of failing the walk (see s6n_freeze_additions).
    req = ['logs/custody-layout.log']
    req += ['logs/%s.stdout' % s for s in ORDER] + ['logs/%s.stderr' % s for s in ORDER] + ['logs/%s.exit.json' % s for s in ORDER] + ['journal/%s.json' % s for s in ORDER]
    req += ['receipts/g6.json', 'receipts/h6.json', 'receipts/z61.json']
    req += s6n_freeze_additions(s6n_done, s6n_present)
    if proof_exported:
        req += ['receipts/invoice-partial.proof.json']
    return req


def main(argv):
    if len(argv) != 3 or argv[1] != '--expected-auth-sha256':
        print(json.dumps({'status': 'BLOCKED(authority)', 'error': 'usage: operator.py --expected-auth-sha256 HEX'}))
        return 2
    status = 'INCOMPLETE'
    runner = None
    custody = None
    claims = None
    expected_key = None
    s6n_done = False
    placement = new_placement_state()
    try:
        auth, probe_raw = _load_auth(argv[2])
        placement['authOk'] = True
        custody, claims = _import_helpers()
        probe_text = probe_raw.decode('ascii')
        def r1():
            pre = custody.verify_runtime_preconditions()
            for name in custody.FIXTURE_ORDER:
                custody.fixture_metadata(name)
            if os.path.lexists(T6):
                _fail('placement', 'T6 already exists; fresh placement only')
            key = custody.key_precheck()
            return {'preconditions': pre, 'key': key}
        admitted = _host_step(r1)
        placement['r1Ok'] = True
        expected_key = admitted['key']
        def r0():
            # Structured creation receipt crossing the supervised fork:
            # on layout failure the made subset is recovered only from
            # the custody-owned LAYOUT-STATUS.json receipt that custody
            # writes when this invocation created T6, never from mere
            # directory existence.
            try:
                layout = custody.create_t6_layout()
            except Exception as e:
                made = []
                try:
                    status_doc = _strict_json(_read_bounded(custody.LAYOUT_STATUS, 65536, 'layout status'), 'layout status')
                    if isinstance(status_doc, dict) and isinstance(status_doc.get('made'), list):
                        made = status_doc['made']
                except Exception:
                    made = []
                return {'layoutComplete': False, 'layoutMade': made, 'inputsPrepared': False, 'layoutError': str(e)}
            try:
                custody.prepare_inputs()
            except Exception as e:
                return {'layoutComplete': True, 'layoutMade': layout['layout'], 'inputsPrepared': False, 'layoutError': str(e)}
            return {'layoutComplete': True, 'layoutMade': layout['layout'], 'inputsPrepared': True}
        r0_receipt = _host_step(r0)
        record_r0_receipt(placement, r0_receipt, [rel for rel, _mode_ignored in custody.T6_LAYOUT])
        if not placement['layoutCompleteOwned'] or not placement['inputsPrepared']:
            _fail('placement', 'R0 did not establish a complete owned layout with prepared inputs: ' + str(r0_receipt.get('layoutError', 'incomplete R0 receipt')))
        runner = Runner(custody)
        argvs = _native_argvs()
        def launch(sid):
            if sid in ('Q6N', 'Q6P'):
                return runner.run(sid, _probe_tail(probe_text), sid == 'Q6P')
            return runner.run(sid, argvs[sid], sid in KEY_SANDBOXES)
        def expect(sid, rec):
            if rec.get('exit') != EXPECTED_EXIT[sid] or rec.get('status'):
                raise OpError('outcome', sid + ' exit differs from expectation')
        rec = launch('Q6N'); expect('Q6N', rec); _check_probe(rec, 'Q6N', expected_key)
        rec = launch('A6'); expect('A6', rec)
        desc = _strict_json(rec['_stdout'], 'A6 descriptor')
        if desc != claims.DESCRIPTOR29:
            _fail('identity', 'A6 descriptor does not equal all 29 frozen fields')
        rec = launch('I6'); expect('I6', rec)
        receipt = _strict_json(rec['_stdout'], 'I6 receipt')
        slots = [[s.get('label'), s.get('encoding'), s.get('value')] for s in receipt.get('slots', [])]
        fm = receipt.get('fixedMatrix', {})
        if slots != claims.GOLD_SLOTS or fm.get('violated') != 0 or fm.get('equalsDescriptor') is not True or fm.get('proveMatrixDigestEqual') is not True or receipt.get('declaredVkDigest') != VK_DIGEST:
            _fail('identity', 'I6 slots or fixedMatrix differ from frozen gold before proof')
        rec = launch('Q6P'); expect('Q6P', rec); _check_probe(rec, 'Q6P', expected_key)
        # Named AUTH subeffect of Q6P: key postcheck, no separate call count.
        _host_step(lambda: custody.key_postcheck(expected_key, 'Q6P'))
        # Named AUTH subeffect of P6: pre-prove memory/disk gate.
        _host_step(_mem_disk_gates)
        rec = launch('P6'); expect('P6', rec)
        # Named AUTH subeffect of P6: key postcheck, no separate call count.
        _host_step(lambda: custody.key_postcheck(expected_key, 'P6'))
        p6 = _strict_json(rec['_stdout'], 'P6 stdout')
        proof_sha = p6.get('proofExportSha256')
        if not isinstance(proof_sha, str) or not SHA_RE.match(proof_sha):
            _fail('identity', 'P6 stdout lacks proofExportSha256')
        for sid in ('V6-source', 'V6-export'):
            rec = launch(sid); expect(sid, rec)
            report = _strict_json(rec['_stdout'], sid)
            if report.get('accepted') is not True or report.get('relationRecompiled') is not (sid == 'V6-source'):
                _fail('identity', sid + ' verify report mismatch')
        def s6n():
            # B06 amended S6N: stage writes PLUS bounded export + metadata receipt.
            proof_bytes = _read_bounded(custody.PROOF_OUT, custody.MAX_PROOF_BYTES, 'positive proof')
            record_bytes = _read_bounded(custody.RECORD_DST, custody.MAX_RECORD_BYTES, 'staged record')
            request_bytes = custody.read_public_fixture('invoice-partial.request.json')
            staged = claims.stage_negatives(proof_bytes, record_bytes, request_bytes, desc)
            expected = {}
            for cid, meta in staged['cases'].items():
                if cid == 'N9':
                    continue
                data = staged['files'][meta['file']]
                if len(data) > custody.MAX_VARIANT_BYTES:
                    _fail('cap', cid + ' staged file exceeds variant cap')
                _write_new(custody.T6_STAGE + '/' + meta['file'], data, 0o444)
                expected[cid] = {'sha256': _sha_bytes(data), 'bytes': len(data)}
            exported = custody.export_negative_variants(expected)
            meta_doc = {'format': 'k6-negatives-metadata/1', 'publishBodies': ['N1', 'N2', 'N3', 'N4', 'N5', 'N6', 'N7', 'N10'], 'metadataOnly': ['N8', 'N11'], 'export': exported}
            _write_new(T6 + '/receipts/negatives-metadata.json', json.dumps(meta_doc, sort_keys=True).encode('ascii'), 0o444)
            return {'staged': sorted(expected), 'metadataReceipt': 'receipts/negatives-metadata.json'}
        _host_step(s6n)
        s6n_done = True
        for sid in ['N1', 'N2', 'N3', 'N4', 'N5', 'N6', 'N7', 'N8', 'N9', 'N10', 'N11', 'usage']:
            rec = launch(sid); expect(sid, rec)
            if sid == 'N11':
                # Named AUTH subeffect of N11: key postcheck.
                _host_step(lambda: custody.key_postcheck(expected_key, 'N11'))
        # B05: exactly one proof in /task/out, before any finalizer.
        _host_step(lambda: check_exactly_one_entry(T6_OUT, 'invoice-partial.proof.json', custody.MAX_PROOF_BYTES))
        status = 'PASS-CANDIDATE'
    except OpError as e:
        print(json.dumps({'status': status, 'error': str(e), 'category': e.category}, sort_keys=True))
        if runner is not None:
            try:
                runner.block_rest(len([s for s in ORDER if s in runner.records]), e.category)
            except Exception:
                pass
    except Exception as e:
        print(json.dumps({'status': 'FAILED', 'error': str(e)}, sort_keys=True))
        status = 'FAILED'
        if runner is not None:
            try:
                runner.block_rest(len([s for s in ORDER if s in runner.records]), 'failure')
            except Exception:
                pass
    # Admission is the run-owned placement state only. A refused R1, a
    # missing AUTH, or a partial owned layout admits no in-tree write at
    # all: the partial tree keeps only custody's own LAYOUT-STATUS.json
    # and the external stdout record above, and a pre-existing or foreign
    # T6 tree is left completely untouched. block_rest log-writing lives
    # only inside the failure handlers above or guarded here, so a
    # blocked-record failure can never prevent eligible finalizers.
    if not finalizer_eligible(placement):
        print(json.dumps({'status': status, 'finalizers': 'not eligible: no complete layout created by this run', 'placement': placement}, sort_keys=True))
        return 1
    if runner is None:
        runner = Runner(custody)
        try:
            runner.block_rest(0, 'placement')
        except Exception as e:
            print(json.dumps({'status': status, 'blockRestError': str(e)}, sort_keys=True))
    def g6():
        try:
            result = claims.check_inspect_and_proof(runner.records['A6']['_stdout'], runner.records['I6']['_stdout'], _read_bounded(custody.PROOF_OUT, custody.MAX_PROOF_BYTES, 'proof'), _read_bounded(custody.RECORD_DST, custody.MAX_RECORD_BYTES, 'record'), custody.read_public_fixture('K4H-GOLD.json'), custody.read_public_fixture('invoice-partial.request.json'))
            outcomes = claims.check_native_outcomes(dict((k, runner.outcomes[k]) for k in DRIVER_ORDER if k in runner.outcomes))
            _write_receipt(T6 + '/receipts/g6.json', {'claims': result, 'native': outcomes})
            return outcomes.get('status') == 'PASS' and outcomes.get('success') is True
        except Exception as e:
            _write_receipt(T6 + '/receipts/g6.json', {'status': 'INCOMPLETE', 'error': str(e)})
            return False
    def h6():
        observed = {'sandboxesDispatched': len(runner.dispatched), 'recordsPresent': len(runner.records), 'aggregateUsedCpu': runner.used_cpu, 'aggregateUsedWall': runner.used_wall, 'blocked': list(runner.blocked)}
        body = {'prefixSha256': runner.prefix_hash, 'counts': COUNTS, 'observed': observed, 'records': dict((k, dict((kk, vv) for kk, vv in v.items() if not kk.startswith('_'))) for k, v in runner.records.items())}
        _write_receipt(T6 + '/receipts/h6.json', body)
        return observed['recordsPresent'] == 19
    def z61():
        if expected_key is None:
            _write_receipt(T6 + '/receipts/z61.json', {'status': 'FAILED', 'error': 'no admitted key metadata'})
            return False
        try:
            now = custody.key_postcheck(expected_key, 'Z61')
            _write_receipt(T6 + '/receipts/z61.json', now)
            return True
        except Exception as e:
            _write_receipt(T6 + '/receipts/z61.json', {'status': 'FAILED', 'error': str(e)})
            return False
    proof_state = {'exported': False}
    def z62():
        # M03: export whenever P6 proof is genuine, independent of G6.
        try:
            p6rec = runner.records.get('P6', {})
            raw = p6rec.get('_stdout')
            if raw:
                p6doc = _strict_json(raw, 'P6 stdout')
                sha = p6doc.get('proofExportSha256')
                if isinstance(sha, str) and SHA_RE.match(sha):
                    custody.public_proof_export(sha, os.lstat(custody.PROOF_OUT).st_size)
                    proof_state['exported'] = True
        except Exception:
            proof_state['exported'] = False
        try:
            s6n_present = [rel for rel in S6N_FAILURE_MAY_RELS if os.path.isfile(T6 + '/' + rel) and not os.path.islink(T6 + '/' + rel)]
            required = _required_publication(s6n_done, proof_state['exported'], s6n_present)
            allow = []
            for rel in required:
                path = T6 + '/' + rel
                if not os.path.isfile(path) or os.path.islink(path):
                    _fail('freeze', 'required publication file missing: ' + rel)
                data = _read_bounded(path, custody.MAX_FREEZE_FILE_BYTES, rel)
                allow.append((rel, len(data), _sha_bytes(data)))
            frozen = custody.freeze_public_results(tuple(allow))
            return frozen.get('status') == 'FROZEN'
        except Exception as e:
            # Pre-freeze failure receipt under the existing manifest path,
            # which custody already excludes from the freeze allowlist, so
            # there is no freeze-hash cycle and no fabricated FROZEN body.
            try:
                _write_receipt(T6 + '/receipts/FREEZE-MANIFEST.json', {'status': 'BLOCKED', 'error': str(e), 'proofExported': proof_state['exported'], 's6nDone': s6n_done})
            except Exception:
                pass
            return False
    results = attempt_finalizers([('G6', lambda: _host_step(g6)), ('H6', lambda: _host_step(h6)), ('Z61', lambda: _host_step(z61)), ('Z62', lambda: _host_step(z62))])
    # Parent bounded failure-record fallback, only inside this owned
    # eligible tree and always create-exclusive: a finalizer whose
    # supervised child died or could not write still leaves a receipt
    # attempt here, while the guaranteed attempt/outcome record remains
    # the external stdout below. A successful freeze manifest and any
    # existing receipt are never overwritten.
    receipt_fallback = {}
    for _fname, _rel in (('G6', 'receipts/g6.json'), ('H6', 'receipts/h6.json'), ('Z61', 'receipts/z61.json')):
        if not os.path.isfile(T6 + '/' + _rel):
            try:
                receipt_fallback[_fname] = _write_receipt(T6 + '/' + _rel, {'status': 'FAILED', 'finalizer': _fname, 'attempted': True, 'error': results[_fname].get('error', 'finalizer left no receipt'), 'source': 'parent fallback after supervised finalizer attempt'})
            except Exception as _e:
                receipt_fallback[_fname] = False
                results[_fname]['receiptFallbackError'] = str(_e)
    if not os.path.isfile(T6 + '/receipts/FREEZE-MANIFEST.json'):
        try:
            receipt_fallback['Z62'] = _write_receipt(T6 + '/receipts/FREEZE-MANIFEST.json', {'status': 'BLOCKED', 'finalizer': 'Z62', 'attempted': True, 'error': results['Z62'].get('error', 'freeze left no manifest'), 'source': 'parent fallback after supervised finalizer attempt'})
        except Exception as _e:
            receipt_fallback['Z62'] = False
            results['Z62']['receiptFallbackError'] = str(_e)
    g6_ok = results['G6']['ok']; h6_ok = results['H6']['ok']; z61_ok = results['Z61']['ok']; freeze_ok = results['Z62']['ok']
    if status == 'PASS-CANDIDATE' and g6_ok and h6_ok and z61_ok and freeze_ok:
        print(json.dumps({'status': 'PASS', 'scope': 'one invoice/partial Groth16 roundtrip under single-party-test-setup/1 only', 'finalizers': results, 'receiptFallback': receipt_fallback}, sort_keys=True))
        return 0
    print(json.dumps({'status': 'INCOMPLETE' if status == 'PASS-CANDIDATE' else status, 'g6': g6_ok, 'h6': h6_ok, 'z61': z61_ok, 'freeze': freeze_ok, 'finalizers': results, 'receiptFallback': receipt_fallback}, sort_keys=True))
    return 1


if __name__ == '__main__':
    sys.exit(main(sys.argv))
