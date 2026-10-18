"""Once-only r3 operator (stdlib only). PROPOSED SOURCE by the r3 operator source author
(claude-opus-5-5, 2026-10-15). UNREVIEWED and NEVER RUN. Before any use it needs a fresh
non-author exact-byte effect review, root authorization and a root-written FREEZE.json
over the exact reviewed bytes.

Implements MINIMUM-REPAIRS R-C 1-9 (review cd039dd6...) for the frozen request
EXECUTION-PROPOSAL.json 2d803e50... as amended by R3-OPERATOR-REQUEST-AMENDMENT.json.
Placement: <task>/tools/run_r3.py with
<task> = <BASE>/current-core-groth16-r3-reviewed-operator-execution-20261015.
Invocation (root, once): cd <task> && /usr/bin/python3 -I tools/run_r3.py

Effects (complete list):
- refuses with exit 2 and writes nothing unless started with -I as <task>/tools/run_r3.py;
- creates <task>/run/ once (fails if present); writes its logs, journal and RESULT.json;
  any unexpected exception after that becomes a STOP and finish() still runs once;
- creates <task>/controls/ and the empty directory controls/collide-dest;
- spawns the task copy of copy_verify_r3.py four times outside env -i: three controls
  (dry run, wrong r3 pin, existing destination) and E0r3, which creates <root>. The
  tool also reads the checkout's git metadata (IR-10);
- writes empty-home, tmp and the Option A CARGO_HOME under <root>;
- runs the twelve wrapped steps E1..E5-r3 under unshare/env -i with cwd <root>;
- executes the verified bytes of the task copy of libtest_rows.py in-process (RESULT.json
  parser), and after RESULT.json creates <task>/post/ and <task>/analysis/ and spawns
  that copy once for the analysis.
No retry, no limit change, no source/oracle edit, no network, no setup/prove/verify.
Never opens provider-result.private.json.

Assumptions carried from the unchanged helpers (R-B; nothing here fixes them): analyze
needs an existing --out-dir and opens ANALYSIS-INDEX.jsonl without O_NOFOLLOW. This
operator creates <task>/analysis fresh immediately before the one invocation and assumes
no other writer on that filesystem meanwhile. A test that prints a bare ok/FAILED/ignored
line ends its block early; E5-r3 runs --test-threads=1 and prints none. Re-hashing just
before a spawn narrows, but cannot close, the window before the interpreter opens the
file."""
import datetime
import hashlib
import json
import os
import pathlib
import re
import resource
import shutil
import signal
import subprocess
import sys
import time
import types

SELF = pathlib.Path(__file__).resolve()
T = SELF.parent.parent
V = T.parent
TASK_NAME = 'current-core-groth16-r3-reviewed-operator-execution-20261015'
ROOT = V / 'current-core-groth16-evidence-repair-check-r3-1'
RUN = T / 'run'
CTL = T / 'controls'
ANALYSIS = T / 'analysis'
POST = T / 'post'
OPERATOR = T / 'tools' / 'run_r3.py'
COPY_TOOL = T / 'tools' / 'copy_verify_r3.py'
LIBTEST_ROWS = T / 'tools' / 'libtest_rows.py'
PY = '/usr/bin/python3'

TOOLCHAIN = '1.99.0-x86_64-unknown-linux-gnu'
TCB = pathlib.Path('/home/charl/.rustup/toolchains') / TOOLCHAIN
CARGO, RUSTC, RUSTDOC = str(TCB / 'bin/cargo'), str(TCB / 'bin/rustc'), str(TCB / 'bin/rustdoc')
PINS = {'bin/cargo': (36160240, 'e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14'),
        'bin/rustc': (647376, 'f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f'),
        'lib/librustc_driver-999a121de2f042be.so': (158319840, 'f45c41337f1516bb9f4d504a2a507202a01c431fc2063334913dde5480ffee03'),
        'bin/rustdoc': (12568360, 'a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e')}
REG = pathlib.Path('/home/charl/.cargo/registry')
SRC_IDX = REG / 'index/index.crates.io-1949cf8c6b5b557f'
SRC_CACHE = REG / 'cache/index.crates.io-1949cf8c6b5b557f'
CH = ROOT / 'cargo-home'
DST_IDX = CH / 'registry/index/index.crates.io-1949cf8c6b5b557f'
DST_CACHE = CH / 'registry/cache/index.crates.io-1949cf8c6b5b557f'

RS = V / 'current-core-groth16-evidence-repair-resume-source-20261015'
PREV = V / 'current-core-groth16-compiler-independent-source-review-20261015'
RV = V / 'current-core-groth16-compiler-repair-independent-review-20261015'
CHECKOUT = '/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015'

# The six --expect keys of the frozen request, in its order.
EXPECT = {'source_manifest': 'd3bdd5009e42e13110559142a7d62ba2823f0723690ae1094033e7684b09005c',
          'build_manifest': 'edbe3193c8eabc51a1d5595757c69c0bbe2caeb89e53cc3dc25564cead44b41a',
          'core_closure': 'f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed',
          'review_manifest': '757ed0bfc5804eec8bac3f1db063b1239ead0cdbeccbc5fa3ec3160a1a6ab151',
          'r2_manifest': '5a256c766908ff521b6a6cf483d1b109d17aa2d931a63598ab39b183cc2b580b',
          'r3_manifest': '640678296f54ec9eca5dedcdcf2f931546e4608131f14bb6d6b60d4c7b5576aa'}
UNION_PIN = '328d157e00db80259f986f978f8ee222daa762277fc5b4377a3d2c0f8cc1c5f9'
# The five manifests behind EXPECT, at the paths the copy tool reads (core_closure is recomputed).
INPUTS = {'source_manifest': RS / 'candidate/SOURCE-MANIFEST.json',
          'build_manifest': RS / 'candidate/BUILD-MANIFEST.json',
          'review_manifest': PREV / 'REVIEW-MANIFEST.json',
          'r2_manifest': RV / 'plumbing-r2/R2-MANIFEST.json',
          'r3_manifest': RS / 'review-overlay/R3-MANIFEST.json'}

# Reviewed digests that FREEZE.json must repeat (R-C.1). The operator's own digest and the two
# new records have no constant here: only FREEZE.json supplies them.
QUAL = 'HW-WITNESS-MODE-QUALIFICATION-20261015.json'
KNOWN = {'tools/libtest_rows.py': '466aeb8ae33d1f6de9ad86a7cae864e2ba691d60583b1e65f942a3ebd73093fc',
         'tools/copy_verify_r3.py': 'b37bbc1d63d42b55037498fc1cd01911568e9d6571e9a7e2162bf29327e745ac',
         'OPTION-A-PLAN.json': '881a77953cc09846be46bd5b0e04346caca220964bc2985ef00689bb473e1d9a',
         'EXECUTION-PROPOSAL.json': '2d803e50585c6d8f0d5065e90f6c8a0260357fc40a2cbcdb2e4af52dfafa51d6'}
FREEZE_KEYS = set(KNOWN) | {'tools/run_r3.py', 'R3-OPERATOR-REQUEST-AMENDMENT.json', QUAL}
HW_TRACE_REL = 'heldout-addendum-4/HW-TRACEABILITY-2.json'
HW_TRACE_SHA = 'a3c3ef156e4c6abfe817594c6cd9ece13cd5657f53ab6fe074fed3ab163ce72a'
PINNED = {}
LIBTEST = None

# Literal ENV of the request: env -i with only these 11 variables (Option A CARGO_HOME, new root).
ENV = ['PATH=/usr/bin:/bin', f'HOME={ROOT}/empty-home', f'TMPDIR={ROOT}/tmp',
       f'CARGO_HOME={CH}', f'CARGO_TARGET_DIR={ROOT}/target', 'CARGO_NET_OFFLINE=true',
       'CARGO_INCREMENTAL=0', 'CARGO_BUILD_JOBS=2',
       f'RUSTUP_TOOLCHAIN={TOOLCHAIN}', f'RUSTC={RUSTC}', f'RUSTDOC={RUSTDOC}']
WRAP = ['/usr/bin/unshare', '--user', '--map-root-user', '--net', '--', '/usr/bin/env', '-i'] + ENV
AS = 8 << 30
WALL = 1800

# E0r3 and its controls: the request's argv, with the FREEZE-verified task copy of the tool
# in place of the author-lane path (amendment A-1).
E0_TOOL_IN_REQUEST = str(RS / 'review-overlay/r3-tools/copy_verify_r3.py')
E0_DIRS = [CHECKOUT, str(RS / 'candidate'), str(PREV), str(RV / 'plumbing-r2'), str(RS / 'review-overlay')]
E0_PASS = {'verdict': 'PASS',
           'counts': {'moriarty': 70, 'r1cs': 127, 'review': 64, 'review_frozen': 42, 'review_r2': 13, 'review_r3': 9},
           'core_closure_recomputed': EXPECT['core_closure'], 'core_closure_files': 70,
           'build_manifest_recomputed': EXPECT['build_manifest'], 'build_manifest_entries': 16,
           'compiler_build_digest_predicted': '0b896adbe183b79d82455fde7b2c5e988b166a63a5fde972d3621eb7ad98bf19'}
WRONG_PIN = '0' * 64
# A wrong r3 pin fails the manifest check and the review/R3-MANIFEST.json plan row (same pin).
NEG_PIN_STOPS = [f"r3_manifest digest {EXPECT['r3_manifest']} != expected {WRONG_PIN}",
                 '1 source files differ from their pins']

# The frozen command list. "python3" resolves through PATH=/usr/bin:/bin inside env -i.
LOCK_C = 'r1cs/source/mori-r1cs/Cargo.lock'
LOCK_H = 'review/proposals/heldout-harness-r3/Cargo.lock'
ALLOW = ['mori-r1cs', 'mori-core', 'mori-lexer', 'mori-diagnostics', 'mori-span']
STEPS = [
    ('E1', 60, [CARGO, 'generate-lockfile', '--offline', '--manifest-path', 'r1cs/source/mori-r1cs/Cargo.toml']),
    ('E1v', 60, ['python3', '-I', 'review/tools/lock_subset_check.py', LOCK_C, 'review/LOCK-UNION-ANALYSIS.json']),
    ('E1v+', 60, ['python3', '-I', 'review/r2-tools/lock_gate.py', 'subset', LOCK_C, 'review/LOCK-UNION-ANALYSIS.json', UNION_PIN] + ALLOW),
    ('E1h-r3', 60, [CARGO, 'generate-lockfile', '--offline', '--manifest-path', 'review/proposals/heldout-harness-r3/Cargo.toml']),
    ('E1hv-r3.subset', 60, ['python3', '-I', 'review/r2-tools/lock_gate.py', 'subset', LOCK_H, 'review/LOCK-UNION-ANALYSIS.json', UNION_PIN, 'mori-r1cs-heldout-r3'] + ALLOW),
    ('E1hv-r3.pair', 60, ['python3', '-I', 'review/r2-tools/lock_gate.py', 'pair', LOCK_C, LOCK_H]),
    ('E2', 60, [CARGO, 'tree', '--offline', '--locked', '--manifest-path', 'r1cs/source/mori-r1cs/Cargo.toml', '-e', 'normal,build,dev', '--features', 'groth16']),
    ('E3a', 900, [CARGO, 'build', '--offline', '--locked', '--lib', '--manifest-path', 'r1cs/source/mori-r1cs/Cargo.toml']),
    ('E3b', 900, [CARGO, 'build', '--offline', '--locked', '--tests', '--manifest-path', 'r1cs/source/mori-r1cs/Cargo.toml']),
    ('E3c', 900, [CARGO, 'build', '--offline', '--locked', '--features', 'groth16', '--manifest-path', 'r1cs/source/mori-r1cs/Cargo.toml']),
    ('E4', 900, [CARGO, 'test', '--offline', '--locked', '--no-fail-fast', '--manifest-path', 'r1cs/source/mori-r1cs/Cargo.toml', '--', '--test-threads=2']),
    ('E5-r3', 900, [CARGO, 'test', '--offline', '--locked', '--manifest-path', 'review/proposals/heldout-harness-r3/Cargo.toml', '--', '--nocapture', '--test-threads=1']),
]
# Declared dependencies and stop rules of the request (a failed prerequisite skips the step).
DEPS = {'E1v': ['E1'], 'E1v+': ['E1', 'E1v'], 'E1h-r3': ['E1v+'], 'E1hv-r3.subset': ['E1h-r3'],
        'E1hv-r3.pair': ['E1hv-r3.subset'], 'E2': ['E1v+'], 'E3a': ['E1v+'], 'E3b': ['E3a'], 'E3c': ['E3a'],
        'E4': ['E3b'], 'E5-r3': ['E3a', 'E1hv-r3.pair']}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha_b(b):
    return hashlib.sha256(b).hexdigest()


def sha(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for chunk in iter(lambda: f.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()


def sha_or_none(p):
    try:
        return sha(p)
    except OSError:
        return None


def journal(ev, **kw):
    rec = {'at': now(), 'event': ev} | kw
    with open(RUN / 'journal.jsonl', 'a') as f:
        f.write(json.dumps(rec, sort_keys=True) + '\n')
    print(json.dumps(rec, sort_keys=True), flush=True)


def write_new(path, obj):
    data = (json.dumps(obj, indent=1, sort_keys=True) + '\n').encode()
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, 'wb') as f:
        f.write(data)
    return sha_b(data)


def disk_gib():
    return shutil.disk_usage(V).free / 2 ** 30


def toolchain():
    out = {}
    for rel, (n, h) in PINS.items():
        p = TCB / rel
        out[rel] = {'bytes': os.lstat(p).st_size, 'sha256': sha(p), 'regular': p.is_file() and not p.is_symlink()}
        out[rel]['equal'] = out[rel]['bytes'] == n and out[rel]['sha256'] == h and out[rel]['regular']
    return out


def listing_home_cargo():
    """Recursive name/type/size listing of /home/charl/.cargo; metadata only, no file opened."""
    base = '/home/charl/.cargo'
    lines = []
    for dp, dns, fns in os.walk(base, followlinks=False):
        for n in sorted(dns + fns):
            p = os.path.join(dp, n)
            st = os.lstat(p)
            rel = os.path.relpath(p, base)
            kind = 'l' if os.path.islink(p) else 'd' if os.path.isdir(p) else 'f'
            lines.append(f"{rel}\t{kind}\t{st.st_size if kind == 'f' else ''}")
    lines.sort()
    return '\n'.join(lines) + '\n'


def listing_task():
    """Top-level names/types of BASE; nothing opened."""
    rows = []
    with os.scandir(V) as it:
        for e in it:
            kind = 'l' if e.is_symlink() else 'd' if e.is_dir(follow_symlinks=False) else 'f'
            rows.append(f'{e.name}\t{kind}')
    return '\n'.join(sorted(rows)) + '\n'


def snapshot(tag):
    hc, tk = listing_home_cargo(), listing_task()
    (RUN / f'home-cargo-listing.{tag}.txt').write_text(hc)
    (RUN / f'task-toplevel-listing.{tag}.txt').write_text(tk)
    snap = {'home_cargo_listing_sha256': sha_b(hc.encode()), 'home_cargo_entries': hc.count('\n'),
            'rustup_settings_toml_sha256': sha('/home/charl/.rustup/settings.toml'),
            'task_toplevel_listing_sha256': sha_b(tk.encode()), 'task_toplevel_entries': tk.count('\n'),
            'cargo_config_on_discovery_path': [p for p in ['/home/charl/.cargo/config', '/home/charl/.cargo/config.toml',
                                                           str(V / '.cargo'), '/home/.cargo', '/.cargo', str(ROOT / '.cargo'),
                                                           str(CH / 'config'), str(CH / 'config.toml')] if os.path.lexists(p)]}
    return snap


def limits(cpu):
    def pre():
        resource.setrlimit(resource.RLIMIT_AS, (AS, AS))
        if cpu:
            resource.setrlimit(resource.RLIMIT_CPU, (cpu, cpu))
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        os.nice(10)
    return pre


def run(sid, argv, cwd, cpu, logdir=RUN):
    rec = {'id': sid, 'argv': argv, 'cwd': str(cwd), 'limits': {'RLIMIT_AS': AS, 'RLIMIT_CPU': cpu, 'RLIMIT_CORE': 0, 'nice': 10, 'wall_s': WALL}}
    free = disk_gib()
    rec['disk_free_gib_before'] = round(free, 2)
    if free < 3:
        rec.update(status='BLOCKED(resource)', reason='disk below 3 GiB before step')
        return rec
    # 'xb': a log never replaces an earlier record.
    so, se = open(logdir / f'{sid}.stdout', 'xb'), open(logdir / f'{sid}.stderr', 'xb')
    rec['start'] = now()
    t0 = time.monotonic()
    try:
        p = subprocess.Popen(argv, cwd=cwd, stdin=subprocess.DEVNULL, stdout=so, stderr=se,
                             preexec_fn=limits(cpu), start_new_session=True, close_fds=True)
    except OSError as e:
        so.close(), se.close()
        rec.update(status='BLOCKED(environment)', reason=f'spawn failed: {e!r}', end=now())
        return rec
    timed_out = False
    while True:
        pid, status, ru = os.wait4(p.pid, os.WNOHANG)
        if pid:
            break
        if time.monotonic() - t0 > WALL:
            timed_out = True
            os.killpg(p.pid, signal.SIGKILL)
            pid, status, ru = os.wait4(p.pid, 0)
            break
        time.sleep(0.25)
    p.returncode = os.waitstatus_to_exitcode(status)
    so.close(), se.close()
    rec.update(end=now(), wall_s=round(time.monotonic() - t0, 2), exit=p.returncode, timed_out=timed_out,
               signal=(signal.Signals(-p.returncode).name if p.returncode < 0 else None),
               maxrss_kib_wait4=ru.ru_maxrss, utime_s=round(ru.ru_utime, 2), stime_s=round(ru.ru_stime, 2),
               disk_free_gib_after=round(disk_gib(), 2))
    for k in ('stdout', 'stderr'):
        b = (logdir / f'{sid}.{k}').read_bytes()
        rec[k] = {'bytes': len(b), 'sha256': sha_b(b)}
    rec['status'] = 'EXIT0' if p.returncode == 0 else ('BLOCKED(resource)?' if timed_out or p.returncode in (-signal.SIGXCPU, -signal.SIGKILL) else 'NONZERO')
    return rec


def materialize(plan):
    """Option A: fresh CARGO_HOME with only the 102 crates, their index .cache entries and config.json."""
    stops, files = [], []

    def put(src, dst, want):
        try:
            b = pathlib.Path(src).read_bytes()
        except OSError as e:  # a missing or unreadable source is an identity STOP, not a crash
            stops.append(f'source unreadable: {src}: {e!r}')
            return
        if pathlib.Path(src).is_symlink() or sha_b(b) != want:
            stops.append(f'source identity differs: {src}')
            return
        dst.parent.mkdir(parents=True, exist_ok=True)
        fd = os.open(dst, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
        with os.fdopen(fd, 'wb') as f:
            f.write(b)
        if sha(dst) != want:
            stops.append(f'post-copy identity differs: {dst}')
        files.append({'path': str(dst.relative_to(CH)), 'sha256': want, 'bytes': len(b)})

    for c in plan['crates']:
        fn = f"{c['name']}-{c['version']}.crate"
        put(SRC_CACHE / fn, DST_CACHE / fn, c['checksum'])
    for rel, e in plan['index'].items():
        put(SRC_IDX / '.cache' / rel, DST_IDX / '.cache' / rel, e['sha256'])
    put(SRC_IDX / 'config.json', DST_IDX / 'config.json', plan['config_json']['sha256'])
    return stops, files


def load_libtest_rows(b):
    """Executes the already verified parser bytes as a module (no second read of the file)."""
    mod = types.ModuleType('libtest_rows')
    exec(compile(b, str(LIBTEST_ROWS), 'exec'), mod.__dict__)
    return mod


def read_inputs():
    """The five pinned input manifests at their source paths: (bytes by key, mismatching keys)."""
    got, bad = {}, []
    for k, p in INPUTS.items():
        try:
            b = p.read_bytes()
        except OSError:
            bad.append(k)
            continue
        if p.is_symlink() or sha_b(b) != EXPECT[k]:
            bad.append(k)
        got[k] = b
    return got, bad


def reverify(names):
    """Task files that no longer equal FREEZE.json, checked immediately before a use."""
    return [n for n in names if PINNED.get(n) is None or sha_or_none(T / n) != PINNED[n]]


def e0_argv(mode, dest, expect, tool=None):
    return ([PY, '-I', str(tool or COPY_TOOL), mode] + E0_DIRS + ([str(dest)] if dest is not None else [])
            + ['--expect'] + [f'{k}={v}' for k, v in expect.items()])


def proposal_mismatches(prop):
    """Every literal of the frozen request this operator implements; any difference is a STOP."""
    bad = []

    def want(label, got, exp):
        if got != exp:
            bad.append(label)

    steps = prop['steps']
    want('13 steps', len(steps), 13)
    s0 = steps[0] if steps else {}
    want('E0r3 id', s0.get('id'), 'E0r3')
    want('E0r3 cpu_s', s0.get('cpu_s'), 60)
    want('E0r3 argv', s0.get('argv'), e0_argv('--copy', ROOT, EXPECT, E0_TOOL_IN_REQUEST))
    want('E0r3 pass_requires', s0.get('pass_requires'), E0_PASS)
    want('E1..E5-r3 id, cpu_s, argv', [[s.get('id'), s.get('cpu_s'), s.get('argv')] for s in steps[1:]],
         [[i, c, a] for i, c, a in STEPS])
    want('dependencies', prop.get('dependencies'), DEPS)
    want('environment.env', prop['environment'].get('env'), ENV)
    want('environment.wrapper', prop['environment'].get('wrapper'), ' '.join(WRAP[:7]))
    want('scratch_root.path', prop['scratch_root'].get('path'), str(ROOT))
    tc = prop['toolchain']
    want('toolchain', [tc.get('cargo'), tc.get('rustc'), tc.get('rustdoc'), tc.get('RUSTUP_TOOLCHAIN')], [CARGO, RUSTC, RUSTDOC, TOOLCHAIN])
    want('four_pins', tc.get('four_pins'), {k: [n, h] for k, (n, h) in PINS.items()})
    lim = prop['limits']
    want('limits wall_s/RLIMIT_CORE/nice', [lim.get('wall_s'), lim.get('RLIMIT_CORE'), lim.get('nice')], [WALL, 0, 10])
    pins = {'candidate_source_manifest': EXPECT['source_manifest'], 'candidate_build_manifest': EXPECT['build_manifest'],
            'core_closure': EXPECT['core_closure'], 'review_manifest': EXPECT['review_manifest'],
            'r2_manifest': EXPECT['r2_manifest'], 'r3_manifest': EXPECT['r3_manifest'],
            'copy_verify_r3.py': KNOWN['tools/copy_verify_r3.py'],
            'libtest_rows.py (proposed helper)': KNOWN['tools/libtest_rows.py'],
            'OPTION-A-PLAN.json (executed run)': KNOWN['OPTION-A-PLAN.json'],
            'LOCK-UNION-ANALYSIS.json (frozen packet)': UNION_PIN}
    want('pins_sha256', {k: prop['pins_sha256'].get(k) for k in pins}, pins)
    want('e5_step', prop.get('e5_step'), 'E5-r3')
    return bad


def prestart(result):
    """R-C.1-3: fresh paths, FREEZE pins, verified task copies, agreement with the frozen request."""
    global LIBTEST
    stops = result['stops']
    if not sys.flags.isolated:
        stops.append('operator not started with python3 -I')
    if T.name != TASK_NAME or SELF != OPERATOR:
        stops.append(f'operator is not <BASE>/{TASK_NAME}/tools/run_r3.py')
    for p in (CTL, ANALYSIS, POST):
        if os.path.lexists(p):
            stops.append(f'{p.name}/ exists before the run')
    if os.path.lexists(ROOT):
        stops.append('scratch exists before E0r3')
    try:
        freeze_b = (T / 'FREEZE.json').read_bytes()
        freeze = json.loads(freeze_b)
        records, inputs = freeze['records'], freeze['inputs']
    except (OSError, ValueError, KeyError, TypeError) as e:
        stops.append(f'FREEZE.json unreadable: {e!r}')
        return None
    result['freeze_sha256'] = sha_b(freeze_b)
    if (not isinstance(records, dict) or set(records) != FREEZE_KEYS
            or not all(isinstance(v, str) and re.fullmatch(r'[0-9a-f]{64}', v) for v in records.values())):
        stops.append('FREEZE.json records are not exactly the amendment key set of SHA-256 digests')
        return None
    result['freeze_records'] = dict(records)
    for n, want in KNOWN.items():
        if records[n] != want:
            stops.append(f'FREEZE.json {n} is not the reviewed digest')
    if inputs != EXPECT:
        stops.append('FREEZE.json inputs differ from the six EXPECT pins')
    PINNED.update(records)
    data = {}
    for n in sorted(FREEZE_KEYS):
        p = T / n
        try:
            b = p.read_bytes()
        except OSError as e:
            stops.append(f'{n} unreadable: {e!r}')
            continue
        if p.is_symlink() or sha_b(b) != records[n]:
            stops.append(f'{n} differs from FREEZE.json')
            continue
        data[n] = b
    # Only the reviewed parser bytes are ever executed, even when another check has already stopped.
    if 'tools/libtest_rows.py' in data and records['tools/libtest_rows.py'] == KNOWN['tools/libtest_rows.py']:
        try:
            LIBTEST = load_libtest_rows(data['tools/libtest_rows.py'])
            if not callable(getattr(LIBTEST, 'result_tests', None)):
                raise TypeError('result_tests missing')
        except Exception as e:  # the parser is pinned; failing to load it stops every step
            LIBTEST = None
            stops.append(f'libtest_rows.py did not load: {e!r}')
    if 'EXECUTION-PROPOSAL.json' in data:
        try:
            bad = proposal_mismatches(json.loads(data['EXECUTION-PROPOSAL.json']))
        except (ValueError, KeyError, IndexError, AttributeError, TypeError) as e:
            bad = [repr(e)]
        if bad:
            stops.append(f'operator constants differ from the frozen request: {bad}')
    if QUAL in data:
        # The dated witness-mode record travels inside RESULT.json (R-A option 1).
        try:
            result['hw_witness_mode_qualification'] = {'file': QUAL, 'sha256': records[QUAL], 'record': json.loads(data[QUAL])}
        except ValueError as e:
            stops.append(f'{QUAL} is not JSON: {e!r}')
    got, bad = read_inputs()
    if bad:
        stops.append(f'pinned input manifests differ: {bad}')
    else:
        try:
            r3 = {f['path']: f['sha256'] for f in json.loads(got['r3_manifest'])['files']}
        except (ValueError, KeyError, TypeError):
            r3 = {}
        if r3.get(HW_TRACE_REL) != HW_TRACE_SHA or r3.get('r3-tools/copy_verify_r3.py') != KNOWN['tools/copy_verify_r3.py']:
            stops.append('R3-MANIFEST does not list the reviewed HW trace and copy tool')
    result['prestart_ok'] = not stops
    return data.get('OPTION-A-PLAN.json')


def e0_step(sid, argv):
    """Spawn the task copy of the copy tool after re-hashing it, the operator and the five manifests."""
    bad = reverify(['tools/run_r3.py', 'tools/copy_verify_r3.py']) + read_inputs()[1]
    if bad:
        return {'id': sid, 'argv': argv, 'status': 'NOT-SPAWNED', 'pre_exec_reverified': False,
                'reason': f'pre-exec re-verification failed: {bad}'}
    rec = run(sid, argv, T, 60)
    rec['pre_exec_reverified'] = True
    rec['copy_tool_sha256_after'] = sha_or_none(COPY_TOOL)
    return rec


def e0_json(sid):
    try:
        j = json.loads((RUN / f'{sid}.stdout').read_text(encoding='utf-8'))
    except (OSError, ValueError) as e:
        return None, repr(e)
    return (j, None) if isinstance(j, dict) else (None, 'stdout is not a JSON object')


def e0_identity(j, verdict):
    return all(j.get(k) == v for k, v in dict(E0_PASS, verdict=verdict).items())


def check_dry(j, r):
    return (r.get('exit') == 0 and e0_identity(j, 'PASS') and j.get('mode') == '--dry-run'
            and j.get('stops') == [] and j.get('expect') == EXPECT and 'copied' not in j)


def check_neg_pin(j, r, dest):
    return (r.get('exit') == 3 and j.get('verdict') == 'STOP' and j.get('mode') == '--copy'
            and j.get('stops') == NEG_PIN_STOPS
            and j.get('source_mismatches') == [{'dest': 'review/R3-MANIFEST.json', 'problem': 'sha256 differs'}]
            and j.get('r3_manifest_sha256') == EXPECT['r3_manifest'] and j.get('expect') == dict(EXPECT, r3_manifest=WRONG_PIN)
            and j.get('counts') == E0_PASS['counts'] and 'copied' not in j and 'core_closure_recomputed' not in j
            and not os.path.lexists(dest))


def check_collide(j, r, dest):
    return (r.get('exit') == 3 and e0_identity(j, 'STOP') and j.get('mode') == '--copy'
            and j.get('stops') == ['destination exists; refusing'] and j.get('source_mismatches') == []
            and j.get('expect') == EXPECT and 'copied' not in j
            and dest.is_dir() and not dest.is_symlink() and os.listdir(dest) == [])


def check_e0(j, r):
    return (r.get('exit') == 0 and e0_identity(j, 'PASS') and j.get('mode') == '--copy'
            and j.get('stops') == [] and j.get('copied') is True and j.get('expect') == EXPECT)


def control(result, sid, argv, check):
    """One copy-tool control. Only its exact expected outcome counts: any other outcome,
    including a different nonzero exit, stops every later step. Never retried."""
    r = e0_step(sid, argv)
    result['controls'][sid] = r
    j, err = e0_json(sid)
    met = False
    if j is None:
        r['stdout_parse_error'] = err
    else:
        try:
            met = bool(check(j, r))
        except Exception as e:  # a malformed record is a failed control, not a pass
            r['check_error'] = repr(e)
    r['scratch_absent_after'] = not os.path.lexists(ROOT)
    r['expect_met'] = met and r['scratch_absent_after']
    journal('control', id=sid, status=r.get('status'), exit=r.get('exit'), expect_met=r['expect_met'])
    if not r['expect_met']:
        result['stops'].append(f'{sid} did not meet its exact expected outcome: no later step')
    return r['expect_met']


def main():
    # Interpreter mode and placement are checked before the once-only marker, so a wrong
    # invocation writes nothing. prestart() repeats both checks for the record.
    if not sys.flags.isolated or T.name != TASK_NAME or SELF != OPERATOR:
        print(json.dumps({'event': 'refused_before_marker', 'isolated': bool(sys.flags.isolated), 'operator': str(SELF)},
                         sort_keys=True), flush=True)
        return 2
    RUN.mkdir()  # once-only marker: fails if a prior attempt exists
    result = {'format': 'r3-reviewed-operator-run/1', 'controls': {}, 'steps': {}, 'skipped': {}, 'stops': []}
    try:
        journal('start', operator_sha256=sha_or_none(SELF), root=str(ROOT), task=str(T))
        attempt(result)
    except Exception as e:  # an unexpected error stops every later step; postchecks still run (R-C.7)
        result['operator_exception'] = repr(e)
        result['stops'].append(f'operator exception {e!r}: no later step')
    return finish(result)


def attempt(result):
    """Everything between the marker and finish(). Each early return is a recorded STOP."""
    plan_b = prestart(result)
    if result['stops']:
        journal('stop', stops=result['stops'])
        return

    # Copy-tool controls C-1..C-3 (amendment A-3): separately recorded, bounded, never retries,
    # never on the real scratch.
    if not control(result, 'E0r3.dry', e0_argv('--dry-run', None, EXPECT), check_dry):
        return
    CTL.mkdir()
    pin_dest = CTL / 'neg-pin-dest'
    if not control(result, 'E0r3.neg-pin', e0_argv('--copy', pin_dest, dict(EXPECT, r3_manifest=WRONG_PIN)),
                   lambda j, r: check_neg_pin(j, r, pin_dest)):
        return
    collide = CTL / 'collide-dest'
    collide.mkdir()
    if not control(result, 'E0r3.neg-collide', e0_argv('--copy', collide, EXPECT),
                   lambda j, r: check_collide(j, r, collide)):
        return

    # E0r3 (python3 -I, run from this task directory, outside <root>).
    if os.path.lexists(ROOT):
        result['stops'].append('scratch exists before E0r3')
        return
    r = e0_step('E0r3', e0_argv('--copy', ROOT, EXPECT))
    result['steps']['E0r3'] = r
    journal('step', **{k: r.get(k) for k in ('id', 'status', 'exit', 'wall_s')})
    j, err = e0_json('E0r3')
    ok = False
    if j is None:
        result['e0r3_parse_error'] = err
    else:
        ok = check_e0(j, r)
        result['e0r3_identity'] = {k: j.get(k) for k in ('core_closure_recomputed', 'core_closure_files', 'build_manifest_recomputed',
                                                          'build_manifest_entries', 'compiler_build_digest_predicted', 'counts',
                                                          'rust_toolchain_files_in_layout', 'checkout_head_informational')}
    r['expect_met'] = ok
    if not ok:
        result['stops'].append('E0r3 not PASS with the exact expected counts/identities: no later step')
        return

    # Task-local HOME/TMPDIR and Option A CARGO_HOME (created after E0r3, which refuses an existing dest).
    if reverify(['OPTION-A-PLAN.json']):
        result['stops'].append('OPTION-A-PLAN.json differs from FREEZE.json before materialization: no later step')
        return
    (ROOT / 'empty-home').mkdir()
    (ROOT / 'tmp').mkdir()
    CH.mkdir()
    stops, files = materialize(json.loads(plan_b))
    result['option_a'] = {'files': len(files), 'stops': stops,
                          'manifest_sha256': sha_b(json.dumps(files, sort_keys=True, separators=(',', ':')).encode())}
    (RUN / 'OPTION-A-MATERIALIZED.json').write_text(json.dumps(files, indent=1, sort_keys=True) + '\n')
    journal('option_a', files=len(files), stops=stops)
    if stops:
        result['stops'].append('Option A materialization identity STOP: no later step')
        return

    # Immediately before E1: four pins, listings, settings, config discovery.
    tc = toolchain()
    result['toolchain_before_E1'] = tc
    result['snapshot_before'] = snapshot('before')
    journal('before_E1', toolchain_equal=all(v['equal'] for v in tc.values()), snapshot=result['snapshot_before'])
    if not all(v['equal'] for v in tc.values()):
        result['stops'].append('toolchain pin differs before E1: no later step')
        return
    if result['snapshot_before']['cargo_config_on_discovery_path']:
        result['stops'].append('cargo config on discovery path: no later step')
        return

    status = {}
    for sid, cpu, argv in STEPS:
        failed = [d for d in DEPS.get(sid, []) if status.get(d) != 'EXIT0']
        if failed:
            result['skipped'][sid] = f'prerequisite not EXIT0: {failed}'
            journal('skip', id=sid, because=failed)
            continue
        r = run(sid, WRAP + argv, ROOT, cpu)
        result['steps'][sid] = r
        status[sid] = r['status']
        journal('step', **{k: r.get(k) for k in ('id', 'status', 'exit', 'wall_s', 'maxrss_kib_wait4', 'signal', 'timed_out')})
        if r['status'] in ('BLOCKED(environment)',) or (r['status'] == 'BLOCKED(resource)' and 'disk' in r.get('reason', '')):
            result['stops'].append(f'{sid}: {r["status"]} {r.get("reason")}: no later step')
            break


def finish(result):
    """Postchecks always, each isolated so that one failure skips no other (R-B.2)."""
    def guard(name, fn):
        try:
            fn()
        except Exception as e:  # postcheck failures are recorded, never retried
            result.setdefault('postcheck_errors', {})[name] = repr(e)

    def toolchain_after():
        result['toolchain_after'] = toolchain()

    def listings():
        result['snapshot_after'] = snapshot('after')
        b, a = (RUN / 'home-cargo-listing.before.txt'), (RUN / 'home-cargo-listing.after.txt')
        if b.exists():
            bl, al = set(b.read_text().splitlines()), set(a.read_text().splitlines())
            result['home_cargo_diff'] = {'removed': sorted(bl - al), 'added': sorted(al - bl)}
            tb = set((RUN / 'task-toplevel-listing.before.txt').read_text().splitlines())
            ta = set((RUN / 'task-toplevel-listing.after.txt').read_text().splitlines())
            result['task_toplevel_diff'] = {'removed': sorted(tb - ta), 'added': sorted(ta - tb)}
            result['rustup_settings_unchanged'] = (result['snapshot_before']['rustup_settings_toml_sha256']
                                                   == result['snapshot_after']['rustup_settings_toml_sha256'])

    def tests():
        if LIBTEST is None:
            result['tests_unparsed'] = 'libtest_rows.py was not verified and loaded at start'
            return
        for sid in ('E4', 'E5-r3'):
            p = RUN / f'{sid}.stdout'
            if not p.exists():
                continue
            # Same decoding and parse_stream as the analysis; a parser exception is recorded
            # for this step and skips no later postcheck.
            try:
                block = LIBTEST.result_tests(p.read_text(encoding='utf-8', errors='replace'))
                result.setdefault('tests', {})[sid] = block
                if sid == 'E5-r3':
                    result['e5_identity_lines'] = block['identity_lines']
            except Exception as e:
                result.setdefault('parse_errors', {})[sid] = repr(e)

    def locks():
        out = {}
        for rel in (LOCK_C, LOCK_H):
            p = ROOT / rel
            out[rel] = {'sha256': sha(p), 'bytes': p.stat().st_size} if p.exists() else None
        result['locks'] = out

    def scratch_toplevel():
        if ROOT.exists():
            result['scratch_toplevel'] = sorted(e.name for e in os.scandir(ROOT))

    def task_files():
        out = {}
        for n in sorted(PINNED):
            h = sha_or_none(T / n)
            out[n] = {'sha256': h, 'equal_to_freeze': h == PINNED[n]}
        result['task_files_at_finish'] = out

    for name, fn in (('toolchain_after', toolchain_after), ('listings', listings), ('tests', tests),
                     ('locks', locks), ('scratch_toplevel', scratch_toplevel), ('task_files_at_finish', task_files)):
        guard(name, fn)
    result['finished'] = now()
    guard('journal_finish', lambda: journal('finish', stops=result['stops'], skipped=result['skipped']))
    try:
        write_new(RUN / 'RESULT.json', result)
    except Exception as e:  # without RESULT.json no analysis runs
        print(json.dumps({'event': 'result_write_failed', 'error': repr(e)}), flush=True)
        return 1
    # Nothing under run/ is written after RESULT.json (R-C.7).
    try:
        post_run(result)
    except Exception as e:  # recorded on stdout; run/ stays untouched
        print(json.dumps({'event': 'post_run_error', 'error': repr(e)}), flush=True)
    return 0


def post_run(result):
    """R-C.8: one analysis by the original parser into a fresh directory, outside run/."""
    POST.mkdir()  # fails if present
    post = {'format': 'r3-reviewed-operator-post/1', 'started': now(), 'result_json_sha256': sha(RUN / 'RESULT.json'),
            'hw_witness_mode_qualification_sha256': PINNED.get(QUAL),
            'hw_rule': (f'Apply {QUAL} to the analysis hw block. ALL-REFS-OK is row status only and never implies that '
                        'adversarial internal witnesses were executed for any HW entry. On a RELATION-PARTIAL entry '
                        '(HW-06, HW-07 and HW-23 included) it is never adversarial-internal-witness evidence.')}
    trace = ROOT / 'review' / HW_TRACE_REL
    reasons = []
    if not result.get('prestart_ok'):
        reasons.append('start verification did not pass')
    bad = reverify(['tools/run_r3.py', 'tools/libtest_rows.py', 'EXECUTION-PROPOSAL.json'])
    if bad:
        reasons.append(f'task copies differ from FREEZE.json: {bad}')
    if trace.is_symlink() or sha_or_none(trace) != HW_TRACE_SHA:
        reasons.append('scratch HW-TRACEABILITY-2.json absent or not the R3-MANIFEST bytes')
    if reasons:
        post['analysis'] = {'status': 'NOT-SPAWNED', 'reasons': reasons}
    else:
        ANALYSIS.mkdir()  # fresh, immediately before the original parser's analyze (R-B.1 assumption)
        argv = [PY, '-I', str(LIBTEST_ROWS), 'analyze', '--run', str(RUN), '--request', str(T / 'EXECUTION-PROPOSAL.json'),
                '--hw-trace', str(trace), '--out-dir', str(ANALYSIS), '--e0', str(RUN / 'E0r3.stdout')]
        rec = run('ANALYZE', argv, T, 60, logdir=POST)
        rec['libtest_rows_sha256_after'] = sha_or_none(LIBTEST_ROWS)
        rec['analysis_entries'] = sorted(os.listdir(ANALYSIS))
        rec['outcome'] = {0: 'analysis written', 3: 'analysis raised; its dated error artifact is kept',
                          2: 'usage refusal'}.get(rec.get('exit'), 'other')
        post['analysis'] = rec
    post['finished'] = now()
    write_new(POST / 'POST-RESULT.json', post)
    print(json.dumps({'event': 'post_run', 'status': post['analysis'].get('status'), 'exit': post['analysis'].get('exit')},
                     sort_keys=True), flush=True)


if __name__ == '__main__':
    sys.exit(main())
