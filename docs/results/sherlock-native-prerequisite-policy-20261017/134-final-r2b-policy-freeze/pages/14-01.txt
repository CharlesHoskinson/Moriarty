"""One-off capture for ONE ordinary synthetic unit run (R53 routing derivative,
not historical c01) of the bound driver
(5a9e4eb8c0dd70236c6bbe2fbaec5096b6241e84c262b4346fab044af18fac62).

PROPOSED_NOT_ADOPTED. Usage, only after Root admission:
    /usr/bin/env -i /usr/bin/python3 -I -B RUN/capture/capture_c01_once.py RUN

RUN is Root's fresh absolute private directory holding exactly capture/, cwd/,
driver/ (only the driver), home/, source/ (exactly the 11 bound files) and tmp/,
with cwd, home and tmp empty and tmp owned by the running non-root uid, mode
0700. The script checks the driver bytes, launches the reviewed argv once with
exactly PATH, HOME, TMPDIR and LANG in a new session, and enforces a 30 second
wall deadline, a 5 second post-kill drain and a 1 MiB kept-output cap per
stream. A deadline, cap overrun or unreaped child is recorded, never retried.
It writes only RUN/out, whose presence refuses any second invocation. It starts
no SDK, compiler, network, namespace or cgroup and changes no rlimit. Running
it records a result; it grants nothing.
"""
import hashlib
import json
import os
import selectors
import signal
import stat
import subprocess
import sys
import time
from datetime import datetime, timezone


PYTHON = '/usr/bin/python3'
WALL = 30
DRAIN = 5
CAP = 1048576
LABEL = 'R53_ROUTING_UNIT_69_INTERFACE_INFORMED_NOT_BLIND_R2_UNSATISFIED'
LAYOUT = ('capture', 'cwd', 'driver', 'home', 'source', 'tmp')
DRIVER = 'routing_unit_driver.py'
DRIVER_SHA256 = '5a9e4eb8c0dd70236c6bbe2fbaec5096b6241e84c262b4346fab044af18fac62'
FREEZE = 'SOURCE-FREEZE.json'
FREEZE_SHA256 = 'f7910a648f0558c9aa3f0543c18cc546d0b0238c83b867db75c2ff0dfb3cdf17'
SUMMARY_PREFIX = 'ORDINARY_ROUTING_UNIT_SUMMARY_JSON='
EXPECT = (
    (FREEZE, FREEZE_SHA256),
    ('bounded_sdk_compile.py', '94810a65a2a68563300edefe34c76447ad9cc3118c07c8ae8f67607b88ac1fd7'),
    ('config.example.json', '4b12357a4af85553be2ca833760d3bd4a08bb9ad4f5585823d63e800f0ff42cb'),
    ('config.schema.json', 'a6496e818fab67cf59297e3d1593d9aabe1593b7f0d10e216b6a69a6c717f210'),
    ('decision-records.schema.json', '2cf9726b8b21b74cb4fe708e45d6e54a6ed40564dbeeefdee3951cb6ef263614'),
    ('exact16_git_symlinks.json', '47cd59df82333a8987d0be85b013793b8a524d2f5bdd33c257a989c97cc48a41'),
    ('test_config_source.py', '0f966d96932136e0762c18b80ba0ca0d13230079125850ce3c35e438a37e5e9d'),
    ('test_bounded_sdk_compile_link_fragment.py',
     '920983f6f18435eaf2a0f85a74853aa23dcabf76185c1369a1dad96361615739'),
    ('test_bounded_sdk_compile_symlink_config.py',
     '0d826a454f2bbf4d717e981d470d391916cd0d69f2b177f7f10fe98f8268a9a3'),
    ('test_r5_walk_treehash_freeze.py', 'f6b7b2d49e1f64053982a7cce4af471e9aac764e103482c794fc89a9f742d513'),
    ('test_receipt_routing.py', '4a91f6ffec061c78564d2bfbf6a13c2b5750d690d4570577e1ebfa5909273e54'),
)
EXPECTED_SUMMARY = dict(
    driver=DRIVER,
    errors=0,
    failures=0,
    guard_surfaces_tripwired_after_guard_load=27,
    host_surfaces_tripwired_before_guard_load=17,
    module_method_counts={
        'test_config_source.py': 22,
        'test_bounded_sdk_compile_link_fragment.py': 13,
        'test_bounded_sdk_compile_symlink_config.py': 14,
        'test_r5_walk_treehash_freeze.py': 12,
        'test_receipt_routing.py': 8,
    },
    ok=True,
    skipped=0,
    total_tests_run=69,
)


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _refuse(message):
    print('refused: ' + message, file=sys.stderr)
    return 2


def _now():
    return datetime.now(timezone.utc).isoformat()


def _kill(process):
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    return time.monotonic()


def _run(command, cwd, env):
    """Launch once in a new session; return capped output and the stop reason.

    The group is killed at the wall deadline or when either stream passes CAP;
    bytes past CAP are counted, not kept. As in the K precedent the group is
    signalled only while the unreaped leader still reserves its id; the driver
    tripwires process creation, so no descendant is expected.
    """
    kept = dict(stdout=bytearray(), stderr=bytearray())
    seen = dict(stdout=0, stderr=0)
    stop, killed = None, None
    process = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               start_new_session=True)
    start = time.monotonic()
    deadline = start + WALL
    with selectors.DefaultSelector() as selector:
        selector.register(process.stdout, selectors.EVENT_READ, 'stdout')
        selector.register(process.stderr, selectors.EVENT_READ, 'stderr')
        while selector.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                if killed is not None:
                    break
                stop, killed = 'WALL_DEADLINE', _kill(process)
                deadline = killed + DRAIN
                continue
            for key, _ in selector.select(remaining):
                chunk = os.read(key.fd, 65536)
                if not chunk:
                    selector.unregister(key.fileobj)
                    continue
                seen[key.data] += len(chunk)
                kept[key.data] += chunk[:max(CAP - len(kept[key.data]), 0)]
                if seen[key.data] > CAP and killed is None:
                    stop, killed = 'OUTPUT_CAP_' + key.data.upper(), _kill(process)
                    deadline = killed + DRAIN
    process.stdout.close()
    process.stderr.close()
    if killed is None:
        try:
            process.wait(timeout=max(deadline - time.monotonic(), 0))
        except subprocess.TimeoutExpired:
            stop, killed = 'WALL_DEADLINE', _kill(process)
    unreaped = False
    if process.returncode is None:
        try:
            process.wait(timeout=max(killed + DRAIN - time.monotonic(), 0))
        except subprocess.TimeoutExpired:
            unreaped = True
    return process.returncode, kept, seen, stop, unreaped, time.monotonic() - start


def main(argv):
    if len(argv) != 2:
        return _refuse('usage: RUN')
    run = argv[1]
    if not os.path.isabs(run) or os.path.realpath(run) != run or not os.path.isdir(run):
        return _refuse('RUN must be an existing absolute real directory')
    if os.geteuid() == 0:
        return _refuse('must not run as root')
    if sorted(os.listdir(run)) != list(LAYOUT):
        return _refuse('RUN membership must be exactly ' + ', '.join(LAYOUT))
    paths = {name: os.path.join(run, name) for name in LAYOUT}
    for path in paths.values():
        if not stat.S_ISDIR(os.lstat(path).st_mode):
            return _refuse(path + ' is not a directory')
    for name in ('cwd', 'home', 'tmp'):
        if os.listdir(paths[name]):
            return _refuse(paths[name] + ' is not empty')
    info = os.lstat(paths['tmp'])
    if (info.st_uid != os.geteuid() or stat.S_IMODE(info.st_mode) != 0o700
            or not os.access(paths['tmp'], os.W_OK)):
        return _refuse('tmp must be owned by this uid, mode 0700 and writable')
    if os.listdir(paths['driver']) != [DRIVER]:
        return _refuse('driver/ must hold only ' + DRIVER)
    driver = os.path.join(paths['driver'], DRIVER)
    if not stat.S_ISREG(os.lstat(driver).st_mode):
        return _refuse(driver + ' is not a regular file')
    with open(driver, 'rb') as f:
        if _sha(f.read()) != DRIVER_SHA256:
            return _refuse(driver + ' is not the bound driver bytes')
    if sorted(os.listdir(paths['source'])) != sorted(name for name, _ in EXPECT):
        return _refuse('source/ membership differs from the 11 bound files')
    for name, digest in EXPECT:
        src = os.path.join(paths['source'], name)
        if not stat.S_ISREG(os.lstat(src).st_mode):
            return _refuse(src + ' is not a regular file')
        with open(src, 'rb') as f:
            if _sha(f.read()) != digest:
                return _refuse(src + ' is not the bound source bytes')
    python = os.path.realpath(PYTHON)
    if not os.path.isfile(python):
        return _refuse(PYTHON + ' is absent')
    with open(python, 'rb') as f:
        python_sha256 = _sha(f.read())
    out = os.path.join(run, 'out')
    os.mkdir(out, 0o700)
    freeze = os.path.join(paths['source'], FREEZE)
    command = [PYTHON, '-I', '-B', driver, '--src-dir', paths['source'],
               '--source-freeze', freeze]
    env = dict(PATH='/usr/bin:/bin', HOME=paths['home'], TMPDIR=paths['tmp'], LANG='C.UTF-8')
    started = _now()
    exit_code, kept, seen, stop, unreaped, elapsed = _run(command, paths['cwd'], env)
    ended = _now()
    streams = {}
    for name in ('stdout', 'stderr'):
        with open(os.path.join(out, name), 'xb') as f:
            f.write(kept[name])
        streams[name] = dict(path='out/' + name, bytes_seen=seen[name],
                             bytes_kept=len(kept[name]), truncated=seen[name] > CAP,
                             sha256=_sha(bytes(kept[name])))
    tripwire = b'Forbidden surface called during unit test: '
    summary_line = (SUMMARY_PREFIX + json.dumps(EXPECTED_SUMMARY, sort_keys=True)).encode('utf-8')
    summary_ok = summary_line in kept['stdout'].splitlines()
    record = dict(
        schema='a102-ordinary-unit-capture/1', label=LABEL,
        classification='ORDINARY_SYNTHETIC_UNIT_TEST; zero native authority',
        run=run, argv=command, env=env, cwd=paths['cwd'],
        python=PYTHON, python_realpath=python, python_sha256=python_sha256,
        driver_sha256=DRIVER_SHA256, source_freeze_sha256=FREEZE_SHA256,
        expected=dict(EXPECT), summary_prefix=SUMMARY_PREFIX,
        expected_summary=EXPECTED_SUMMARY,
        wall_seconds=WALL, drain_seconds=DRAIN, stream_cap_bytes=CAP,
        started_utc=started, ended_utc=ended, elapsed_seconds=elapsed,
        stop_reason=stop, unreaped=unreaped, exit=exit_code, streams=streams,
        markers=dict(driver_refusal=b'DRIVER_REFUSAL: ' in kept['stderr'],
                     tripwire=tripwire in kept['stdout'] or tripwire in kept['stderr'],
                     expected_summary_69=summary_ok),
        acceptance='none; R2 unsatisfied; not R2, adoption, real-run, native or C4 '
                   'evidence; historical c01 result not transferred; Root records '
                   'every outcome verbatim')
    with open(os.path.join(out, 'RESULT.json'), 'x', encoding='utf-8') as f:
        f.write(json.dumps(record, indent=2, sort_keys=True) + '\n')
    print('exit', exit_code, 'stop', stop, 'unreaped', unreaped)
    return 0 if exit_code == 0 and stop is None and not unreaped and summary_ok else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv))
