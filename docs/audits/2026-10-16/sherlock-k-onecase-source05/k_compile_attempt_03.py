#!/usr/bin/env python3
"""Sherlock K-A attempt-compile-01: one fenced K 7.1.337 Haskell compile, no engine.

Opus-authored security driver for the ordinary 30-second child class.

  python3 -I k_compile_attempt_01.py --preflight-only
      Read-only gates and inventories. No directory, candidate, claim or child.
  python3 -I k_compile_attempt_01.py --execute --approved-driver-sha256 HEX
      Exactly one terminal attempt-compile-01. Never retried, never reset.

The pinned JVM is invoked directly, bypassing kompile/setenv/checkJava and its
nailgun socket probes. Compiler children (flex and its m4, gcc and its
assembler/linker, generated scanners, z3, kore-parser) resolve only through the
minimal PATH below; host /usr/bin and /bin are excluded so an absent store
dependency fails visibly instead of being substituted. Compiled outputs are
inventoried and hashed, never validated or certified.
"""
import argparse
import datetime
import hashlib
import json
import os
import re
import resource
import secrets
import signal
import stat
import subprocess
import sys
import time

WORKSPACE = '/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016'
RUNTIME_PARENT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016'
RUNTIME = RUNTIME_PARENT + '/k-actual-01'
ATTEMPT_NAME = 'attempt-compile-01'
ATTEMPT = RUNTIME + '/' + ATTEMPT_NAME
K_HOME = ATTEMPT + '/home'
TMP = ATTEMPT + '/tmp'
OUTPUT = ATTEMPT + '/main-kompiled'
STDOUT = ATTEMPT + '/compile.stdout'
STDERR = ATTEMPT + '/compile.stderr'
RECEIPT = ATTEMPT + '/receipt.json'
LEDGER = RUNTIME + '/k-attempt-ledger.jsonl'
CANDIDATE_ROOT = RUNTIME + '/compile-input'
CANDIDATE_TREE = (CANDIDATE_ROOT, CANDIDATE_ROOT + '/models', CANDIDATE_ROOT + '/models/sherlock',
                  CANDIDATE_ROOT + '/models/sherlock/k', CANDIDATE_ROOT + '/models/sherlock/k/main.k')
CANDIDATE = CANDIDATE_TREE[-1]

STORE = '/nix/store/'
K_ROOT = STORE + 'y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406'
JRE_ROOT = STORE + 'kjb3a2ip8ydb3zm604agim5ccjja3vfj-openjdk-headless-minimal-jre-21+35'
JAVA = JRE_ROOT + '/bin/java'
K_JARS = K_ROOT + '/lib/kframework/java'
K_BUILTIN = K_ROOT + '/include/kframework/builtin'
K_WRAPPER = K_ROOT + '/bin/kompile'
K_UNWRAPPED = K_ROOT + '/bin-unwrapped/kompile'
K_LIBRARY = K_ROOT + '/lib/kframework/k'
TOOL_ROOTS = (  # PATH order; K starts these children by bare name.
    ('kore-parser', STORE + '5p6kb76qjrdwjvjmm6l7l2683k41ay1r-kore-38afc81'),
    ('gcc', STORE + '9bv7dcvmfcjnmg5mnqwqlq2wxfn8d7yi-gcc-wrapper-13.2.0'),
    ('flex', STORE + 'z1fkfc5ca0rra7m5zrh8i92mx18w9i00-flex-2.6.4'),
    ('z3', STORE + 'ih51sgk8g57fnkbd5r82ddi8k5vln8cl-z3-4.13.4'),
)
GCC_ROOT = dict(TOOL_ROOTS)['gcc']
FLEX = dict(TOOL_ROOTS)['flex'] + '/bin/flex'
CHILD_SECONDS = 30
SWEEP_SECONDS = 3
FILE_SIZE_LIMIT = 512 * 1024 * 1024
TOKEN_NAME = 'SHERLOCK_K_ATTEMPT_TOKEN'
STORE_ROOT = re.compile(rb'/nix/store/[0-9a-z]{32}-[0-9A-Za-z+._?=-]+')

MAIN_REL = 'models/sherlock/k/main.k'
MAIN_SHA256 = 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'
MAIN_BYTES = 16249
SOURCE_FILES = (
    (MAIN_REL, MAIN_SHA256, MAIN_BYTES),
    ('models/sherlock/qa/k_capture_observations.py', '2f9d939ea847a61ecaa6c27058b00504c7d613642273767f79fcaa111fc2f8bd', 11336),
    ('models/sherlock/qa/test_k_capture_observations.py', 'e4b71ca2c249396e9ad262f5c43a16885193045da649d36a33069b686d898767', 5866),
    ('models/sherlock/qa/k_observations.py', '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07', 7686),
    ('models/sherlock/qa/k_terms.py', 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca', 5666),
    ('models/sherlock/qa/host_observations.py', '61b8258595a298231679362cb3974ae176b6970e7e4475388252d59da545fcdc', 4912),
    ('models/sherlock/qa/check.py', '7a6d1bbca881f5c2b012278989dac600834d610986694e640da51a40b5a412e0', 36429),
    ('models/sherlock/oracles/normal-programs.json', 'ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f', 10202),
    ('models/sherlock/oracles/extra-normal-programs.json', 'dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153', 3721),
    ('models/sherlock/oracles/normal-prefixes.json', 'dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d', 53119),
    ('models/sherlock/oracles/extra-normal-prefixes.json', 'acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16', 80130),
)
K_SRC = STORE + 'x3ybp2gyrf0vlzzf02s959skd08wn5r6-source'
KORE_SRC = STORE + 'pmjs0mz53bz7p8jzzqx4nba1pfwyhf39-kore/src/Kore'
PRIMARY_SOURCES = (
    (K_ROOT + '/lib/kframework/checkJava', 'ca205adcbd0aaede282b0ee2251c15fd1dc0806bcd946d8a8971cdedeeed5571', 2257),
    (K_ROOT + '/lib/kframework/setenv', 'fa8d26333cf0d1ef90ba3c6cfc85cd9f63ff23088bfaa61f6a9c6c69c9629688', 891),
    (K_ROOT + '/bin-unwrapped/krun', '80dea6c5e96142084fe7b4514a22f883db906daef63701b2bcad949d1ff5d743', 21662),
    (K_SRC + '/haskell-backend/src/main/java/org/kframework/backend/haskell/HaskellBackend.java',
     'ad044efecdb6679f0cc6ff9929717411f17c5f5f54b33dd3b87e25d938d5b9b4', 2977),
    (K_SRC + '/haskell-backend/src/main/java/org/kframework/backend/haskell/HaskellKompileOptions.java',
     '32534c3b27236d491091555fe053f7f147e9b4822f7faefea31a489f7d41a60a', 986),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/parser/inner/kernel/Scanner.java',
     '32451e633f48e573813e94726e7e9695dffd455aceb9b4665540cd20508fb142', 19129),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/parser/inner/disambiguation/TypeInferencer.java',
     '64ab4933ffeff6790cbadfbaccd31df569e0aaba457475f846dd1665e82042bc', 34641),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/utils/file/JarInfo.java',
     '0dad79688d7c283e013200db422ccf4aee1344fb160c76add78be24ff670f8d4', 3593),
    (KORE_SRC + '/Log/DebugAppliedRewriteRules.hs', '6372c03b66df87d812dd3a07d05cf08ed3d1a4006b7ce3eca7da9d38bb4b08aa', 4599),
    (KORE_SRC + '/Log/DebugRewriteTrace.hs', 'a481f4d0d30646162317396af9995576ef8b807481b111df429b8abbd8e0b2ac', 8030),
    (KORE_SRC + '/Log/InfoExecDepth.hs', 'd7f94b700089936425f6d16f82101199176d9cb8df612fc59fa88c5cd0280091', 1428),
    (KORE_SRC + '/Log/WarnDepthLimitExceeded.hs', '097c76c49a00a8a6da763b85580fc16a8808defbb0746b5e4c399a961f896663', 1227),
    (KORE_SRC + '/Log/KoreLogOptions.hs', '58729e884c907d3d5a3c5f375e6be9e76aca45ce219bdd97acd28ef743378af0', 28018),
    (KORE_SRC + '/Attribute/SourceLocation.hs', '7f77fefdcbb3d464c2d35c1a28e1812ca7209a94d8439d8adb0c043da4908983', 2794),
)

COMMAND = (
    JAVA, '-Xms64m', '-Xmx512m', '-Xss32m', '-XX:ActiveProcessorCount=2',
    '-XX:+ExitOnOutOfMemoryError', '-XX:-UsePerfData', '-XX:+DisableAttachMechanism',
    '-XX:ErrorFile=' + ATTEMPT + '/hs_err_pid%p.log',
    '-Dfile.encoding=UTF-8', '-Djava.awt.headless=true',
    '-Djava.io.tmpdir=' + TMP, '-Duser.home=' + K_HOME,
    '-ea', '-cp', K_JARS + '/*', 'org.kframework.main.Main', '-kompile', CANDIDATE,
    '--backend', 'haskell', '--no-haskell-binary',
    '--main-module', 'MORIARTY-UI', '--syntax-module', 'MORIARTY-UI-SYNTAX',
    '-I', K_BUILTIN, '--output-definition', OUTPUT,
)


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def environment(token):
    return {'PATH': ':'.join(root + '/bin' for _, root in TOOL_ROOTS), 'HOME': K_HOME,
            'TMPDIR': TMP, 'LANG': 'C', 'LC_ALL': 'C', TOKEN_NAME: token}


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, 'rb') as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b''):
            digest.update(chunk)
    return digest.hexdigest()


def read_exact(path):
    """Read a regular file reached through no symlinked or non-canonical component."""
    if os.path.realpath(path) != path:
        raise ValueError('symlinked or non-canonical path: ' + path)
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise ValueError('not a regular file: ' + path)
        chunks = []
        while True:
            chunk = os.read(fd, 1 << 20)
            if not chunk:
                return b''.join(chunks)
            chunks.append(chunk)
    finally:
        os.close(fd)


def read_store(path):
    real = os.path.realpath(path)
    if not real.startswith(STORE):
        raise ValueError('resolves outside /nix/store: ' + path)
    with open(real, 'rb') as handle:
        return handle.read()


def open_exclusive(path):
    return os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)


def write_exclusive(path, data):
    fd = open_exclusive(path)
    try:
        view = memoryview(data)
        while view:
            view = view[os.write(fd, view):]
        os.fsync(fd)
    finally:
        os.close(fd)


def append_ledger(record):
    line = (json.dumps(record, sort_keys=True) + '\n').encode()
    fd = os.open(LEDGER, os.O_WRONLY | os.O_APPEND | os.O_CREAT | os.O_NOFOLLOW, 0o600)
    try:
        os.write(fd, line)
        os.fsync(fd)
    finally:
        os.close(fd)


def entry(path):
    info = os.lstat(path)
    row = dict(path=path, mode=oct(stat.S_IMODE(info.st_mode)))
    if stat.S_ISLNK(info.st_mode):
        real = os.path.realpath(path)
        row.update(kind='symlink', target=os.readlink(path), resolved=real)
        if os.path.isfile(real):
            row['resolved_sha256'] = sha256_file(real)
    elif stat.S_ISREG(info.st_mode):
        row.update(kind='file', size=info.st_size, sha256=sha256_file(path))
    elif stat.S_ISDIR(info.st_mode):
        row['kind'] = 'dir'
    else:
        row['kind'] = 'other'
    return row


def tree(root):
    if not os.path.lexists(root):
        return [dict(path=root, kind='absent')]
    rows = [entry(root)]
    if os.path.isdir(root) and not os.path.islink(root):
        for directory, dirnames, filenames in os.walk(root):
            dirnames.sort()
            for name in sorted(dirnames + filenames):
                rows.append(entry(os.path.join(directory, name)))
    return rows


def check_sources(blockers):
    rows, main = {}, None
    for rel, expected, size in SOURCE_FILES:
        try:
            raw = read_exact(WORKSPACE + '/' + rel)
        except (OSError, ValueError) as error:
            blockers.append(f'source-unreadable: {rel}: {error}')
            rows[rel] = None
            continue
        rows[rel] = dict(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
        if rows[rel] != dict(sha256=expected, bytes=size):
            blockers.append('source-hash-mismatch: ' + rel)
        elif rel == MAIN_REL:
            main = raw
    return rows, main


def check_primary(blockers):
    rows = {}
    for path, expected, size in PRIMARY_SOURCES:
        try:
            raw = read_store(path)
        except (OSError, ValueError) as error:
            blockers.append(f'primary-source-unreadable: {path}: {error}')
            rows[path] = None
            continue
        rows[path] = dict(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
        if rows[path] != dict(sha256=expected, bytes=size):
            blockers.append('primary-source-mismatch: ' + path)
    return rows


def store_executable(path, blockers, label):
    real = os.path.realpath(path)
    if not real.startswith(STORE) or not os.path.isfile(real) or not os.access(real, os.X_OK):
        blockers.append(f'{label}-absent-or-outside-store: {path}')
        return None
    return real


def packaging_evidence(blockers):
    """The installed package, not this proposal, must name the JRE and tool roots."""
    evidence, texts = {}, {}
    for label, path in (('kompile-wrapper', K_WRAPPER), ('kompile-unwrapped', K_UNWRAPPED),
                        ('kompile-library', K_LIBRARY)):
        try:
            texts[label] = read_store(path)
            evidence[label] = hashlib.sha256(texts[label]).hexdigest()
        except (OSError, ValueError) as error:
            blockers.append(f'{label}-unreadable: {path}: {error}')
            texts[label] = evidence[label] = None
    if texts['kompile-wrapper'] is not None:
        for root in (JRE_ROOT,) + tuple(root for _, root in TOOL_ROOTS):
            if root.encode() not in texts['kompile-wrapper']:
                blockers.append('kompile-wrapper-does-not-reference: ' + root)
    if texts['kompile-unwrapped'] is not None:
        for marker in (b'../lib/kframework/k', b'-kompile'):
            if marker not in texts['kompile-unwrapped']:
                blockers.append('kompile-entrypoint-not-observed: ' + marker.decode())
    if (texts['kompile-library'] is not None
            and b'org.kframework.main.Main' not in texts['kompile-library']):
        blockers.append('kompile-library-entrypoint-not-observed: org.kframework.main.Main')
    return evidence


def nix_support_flags(root):
    prefixes, includes = [], []
    support = root + '/nix-support'
    if not os.path.isdir(support):
        return prefixes, includes
    for name in sorted(os.listdir(support)):
        path = os.path.join(support, name)
        if not os.path.isfile(path) or os.path.getsize(path) > 1 << 20:
            continue
        try:
            words = read_store(path).decode('utf-8').split()
        except (UnicodeDecodeError, ValueError):
            continue
        for index, word in enumerate(words):
            following = words[index + 1] if index + 1 < len(words) else ''
            if word == '-B':
                prefixes.append(following)
            elif word.startswith('-B'):
                prefixes.append(word[2:])
            elif word in ('-isystem', '-idirafter', '-I'):
                includes.append(following)
            elif word.startswith('-I'):
                includes.append(word[2:])
    return ([value for value in prefixes if value.startswith(STORE)],
            [value for value in includes if value.startswith(STORE)])


def locate(name, prefixes, executable):
    for prefix in prefixes:
        for path in (prefix + name, prefix.rstrip('/') + '/' + name):
            real = os.path.realpath(path)
            if (real.startswith(STORE) and os.path.isfile(real)
                    and (not executable or os.access(real, os.X_OK))):
                return path
    return None


def compiler_closure(blockers):
    """Only gcc's real search paths count: the minimal PATH and wrapper -B/include flags."""
    prefixes, includes = nix_support_flags(GCC_ROOT)
    path_dirs = [root + '/bin/' for _, root in TOOL_ROOTS]
    found = {}
    for name, where, executable in (
            ('as', path_dirs + prefixes, True),
            ('ld', path_dirs + prefixes, True),
            ('crt1.o', prefixes, False),
            ('stdio.h', includes, False),
            ('fcntl.h', includes, False)):
        found[name] = locate(name, where, executable)
        if found[name] is None:
            blockers.append('gcc-closure-not-established: ' + name)
    roots = sorted({root.decode() for root in STORE_ROOT.findall(read_store(FLEX))})
    found['m4'] = locate('m4', [root + '/bin/' for root in roots], True)
    if found['m4'] is None:
        blockers.append('flex-m4-closure-not-established')
    return found


def candidate_problem():
    rows = tree(CANDIDATE_ROOT)
    shape = [(row['path'], row['kind']) for row in rows]
    if shape != [(path, 'dir') for path in CANDIDATE_TREE[:-1]] + [(CANDIDATE, 'file')]:
        return 'candidate-tree-not-exactly-main.k'
    if os.path.realpath(CANDIDATE) != CANDIDATE:
        return 'candidate-path-symlinked'
    if (rows[-1]['sha256'], rows[-1]['size']) != (MAIN_SHA256, MAIN_BYTES):
        return 'candidate-hash-mismatch'
    return None


def runtime_state(blockers):
    state = dict(runtime_exists=False, candidate_exists=False, runtime_entries=[])
    if os.path.realpath(RUNTIME_PARENT) != RUNTIME_PARENT or not os.path.isdir(RUNTIME_PARENT):
        blockers.append('runtime-parent-absent-or-symlinked: ' + RUNTIME_PARENT)
        return state
    if not os.path.lexists(RUNTIME):
        return state
    if os.path.realpath(RUNTIME) != RUNTIME or not os.path.isdir(RUNTIME):
        blockers.append('runtime-symlinked-or-not-directory: ' + RUNTIME)
        return state
    state['runtime_exists'] = True
    state['runtime_entries'] = names = sorted(os.listdir(RUNTIME))
    prior = [name for name in names if name.startswith('attempt-')]
    if prior:
        blockers.append('prior-attempt-present-no-retry: ' + ', '.join(prior))
    if os.path.lexists(LEDGER):
        try:
            for line in read_exact(LEDGER).decode('utf-8').splitlines():
                record = json.loads(line)
                if type(record) is not dict:
                    raise ValueError('non-object ledger record')
                if record.get('attempt') == ATTEMPT_NAME:
                    blockers.append('ledger-records-prior-claim-no-retry')
                    break
        except (OSError, ValueError) as error:
            blockers.append(f'ledger-unreadable-fail-closed: {error}')
    if os.path.lexists(CANDIDATE_ROOT):
        problem = candidate_problem()
        if problem is not None:
            blockers.append(problem)
        else:
            state['candidate_exists'] = True
    return state


def inventories(closure):
    roots = [JRE_ROOT, K_JARS, K_BUILTIN, K_WRAPPER, K_UNWRAPPED, K_LIBRARY, GCC_ROOT + '/nix-support']
    roots += [root + '/bin' for _, root in TOOL_ROOTS]
    roots += sorted(path for path in closure.values() if path)
    return {root: tree(os.path.realpath(root)) for root in dict.fromkeys(roots)}


def preflight():
    blockers = []
    report = dict(schema='sherlock-k-compile-preflight/1', attempt=ATTEMPT_NAME,
                  driver_sha256=sha256_file(os.path.realpath(__file__)),
                  interpreter=dict(path=sys.executable, version=sys.version,
                                   sha256=sha256_file(os.path.realpath(sys.executable))),
                  command=list(COMMAND), path=environment('')['PATH'], blockers=blockers)
    main = None
    try:
        if sys.platform != 'linux' or not os.path.isdir('/proc/self'):
            blockers.append('linux-procfs-required')
        if os.geteuid() == 0:
            blockers.append('refusing-root: no privilege beyond the ordinary user')
        if not sys.flags.isolated:
            blockers.append('python-isolated-mode-required: run with python3 -I')
        report['sources'], main = check_sources(blockers)
        report['primary_sources'] = check_primary(blockers)
        report['java'] = store_executable(JAVA, blockers, 'java')
        report['tools'] = {name: store_executable(root + '/bin/' + name, blockers, name)
                           for name, root in TOOL_ROOTS}
        if not (os.path.isdir(K_JARS) and any(name.endswith('.jar') for name in os.listdir(K_JARS))):
            blockers.append('k-jars-absent: ' + K_JARS)
        if not os.path.isfile(K_BUILTIN + '/domains.md'):
            blockers.append('k-builtin-domains-absent: ' + K_BUILTIN)
        report['packaging_evidence'] = packaging_evidence(blockers)
        report['compiler_closure'] = closure = compiler_closure(blockers)
        report['runtime_state'] = runtime_state(blockers)
        report['inventory'] = inventories(closure)
    except Exception as error:
        blockers.append(f'preflight-error: {type(error).__name__}: {error}')
    return report, (main if not blockers else None)


def create_candidate(main):
    for directory in CANDIDATE_TREE[:-1]:
        os.mkdir(directory, 0o700)
    fd = os.open(CANDIDATE, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o444)
    with os.fdopen(fd, 'wb') as handle:
        handle.write(main)
        handle.flush()
        os.fsync(handle.fileno())


def processes(pgid, token):
    marker = (TOKEN_NAME + '=' + token).encode()
    found = []
    for name in os.listdir('/proc'):
        if not name.isdigit() or int(name) == os.getpid():
            continue
        pid = int(name)
        try:
            with open(f'/proc/{pid}/stat', 'rb') as handle:
                fields = handle.read().rsplit(b')', 1)[1].split()
            state, group = fields[0].decode(), int(fields[2])
        except (OSError, IndexError, ValueError):
            continue
        tagged = False
        try:
            with open(f'/proc/{pid}/environ', 'rb') as handle:
                tagged = marker in handle.read().split(b'\0')
        except OSError:
            pass
        if group == pgid or tagged:
            try:
                with open(f'/proc/{pid}/cmdline', 'rb') as handle:
                    cmdline = [part.decode('utf-8', 'replace') for part in handle.read().split(b'\0') if part]
            except OSError:
                cmdline = []
            found.append(dict(pid=pid, pgid=group, state=state, tagged=tagged,
                              in_group=group == pgid, cmdline=cmdline))
    return found


def kill_group(pgid):
    try:
        os.killpg(pgid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def sweep(pgid, token):
    killed = []
    deadline = time.monotonic() + SWEEP_SECONDS
    while True:
        live = [row for row in processes(pgid, token) if row['state'] != 'Z']
        if not live:
            return killed, []
        killed.extend(live)
        for row in live:
            try:
                os.kill(row['pid'], signal.SIGKILL)
            except ProcessLookupError:
                pass
        if time.monotonic() >= deadline:
            return killed, live
        time.sleep(0.1)


def run_child(receipt, token):
    os.mkdir(K_HOME, 0o700)
    os.mkdir(TMP, 0o700)
    problem = candidate_problem()
    if problem is not None:
        receipt['outcome'] = 'refused-prelaunch: ' + problem
        return
    _, hard = resource.getrlimit(resource.RLIMIT_FSIZE)
    fsize = FILE_SIZE_LIMIT if hard == resource.RLIM_INFINITY else min(hard, FILE_SIZE_LIMIT)
    env = environment(token)
    receipt.update(command=list(COMMAND), environment=env, cwd=ATTEMPT, stdin='/dev/null',
                   limits=dict(wall_seconds=CHILD_SECONDS, rlimit_core=0, rlimit_fsize=fsize,
                               umask='0o077', jvm_heap='64m..512m', jvm_stack='32m',
                               jvm_active_processors=2,
                               process_group='new session; SIGKILL to group at deadline and after exit'))

    def limit_child():
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        resource.setrlimit(resource.RLIMIT_FSIZE, (fsize, fsize))
        os.umask(0o077)

    out_fd, err_fd = open_exclusive(STDOUT), open_exclusive(STDERR)
    proc = None
    try:
        receipt['started_utc'] = utc()
        started = time.monotonic()
        try:
            proc = subprocess.Popen(COMMAND, cwd=ATTEMPT, env=env, stdin=subprocess.DEVNULL,
                                    stdout=out_fd, stderr=err_fd, close_fds=True,
                                    start_new_session=True, preexec_fn=limit_child)
        except (OSError, subprocess.SubprocessError) as error:
            receipt.update(outcome='launch-error', launch_error=f'{type(error).__name__}: {error}')
            return
        receipt['pid'] = proc.pid
        timed_out = False
        try:
            proc.wait(timeout=max(0.0, started + CHILD_SECONDS - time.monotonic()))
        except subprocess.TimeoutExpired:
            timed_out = True
            kill_group(proc.pid)
            proc.wait()
        code = proc.returncode
        receipt.update(elapsed_seconds=round(time.monotonic() - started, 3), ended_utc=utc(),
                       returncode=code, timed_out=timed_out,
                       outcome='timeout-killed' if timed_out else ('exited' if code >= 0 else 'signaled'))
    finally:
        if proc is not None:
            if proc.returncode is None:
                kill_group(proc.pid)
                proc.wait()
            receipt['group_members_after_exit'] = processes(proc.pid, token)
            kill_group(proc.pid)
            killed, remaining = sweep(proc.pid, token)
            receipt.update(swept=killed, remaining=remaining,
                           containment_breach=bool(remaining) or any(not row['in_group'] for row in killed))
        os.close(out_fd)
        os.close(err_fd)


def postflight(receipt, before):
    try:
        blockers = []
        sources, _ = check_sources(blockers)
        primary = check_primary(blockers)
        inventory = inventories(before['compiler_closure'])
        changed = [name for name, now in (('sources', sources), ('primary_sources', primary),
                                          ('inventory', inventory)) if before[name] != now]
        receipt['postflight'] = dict(blockers=blockers, changed=changed,
                                     candidate_problem=candidate_problem(), attempt_tree=tree(ATTEMPT))
    except Exception as error:
        receipt['postflight'] = dict(error=f'{type(error).__name__}: {error}')


def persist(receipt):
    data = json.dumps(receipt, indent=2, sort_keys=True).encode()
    digest = hashlib.sha256(data).hexdigest()
    try:
        write_exclusive(RECEIPT, data)
    except OSError as error:
        digest += f' (receipt file not written: {error}; receipt follows on stdout)'
        sys.stdout.write(data.decode() + '\n')
    try:
        append_ledger(dict(event='finished', attempt=ATTEMPT_NAME, outcome=receipt.get('outcome'),
                           receipt_sha256=digest, utc=utc()))
    except OSError as error:
        digest += f' (ledger finish not appended: {error})'
    return digest


def execute(approved):
    own = sha256_file(os.path.realpath(__file__))
    if approved.lower() != own:
        print(json.dumps(dict(status='refused', reason='approved-driver-sha256-mismatch', driver_sha256=own)))
        return 2
    before, main = preflight()
    if before['blockers'] or main is None:
        print(json.dumps(dict(status='refused-preflight', blockers=before['blockers']), indent=2))
        return 2
    if not os.path.isdir(RUNTIME):
        os.mkdir(RUNTIME, 0o700)
    if not before['runtime_state']['candidate_exists']:
        create_candidate(main)
    problem = candidate_problem()
    if problem is not None:
        print(json.dumps(dict(status='refused-candidate', reason=problem)))
        return 2
    os.mkdir(ATTEMPT, 0o700)  # Exclusive terminal claim: never removed, reset or retried.
    token = secrets.token_hex(16)
    receipt = dict(schema='sherlock-k-compile-attempt-receipt/1', attempt=ATTEMPT_NAME,
                   profile='sherlock-k-a/1', driver_sha256=own, claimed_utc=utc(),
                   preflight=before, outcome='driver-error-before-launch',
                   scope='one compile only; no engine, krun, kparse, kore-exec, retry or install',
                   certification='none: outputs inventoried, not validated; no language or product acceptance',
                   retry='forbidden: attempt-compile-01 is terminal whatever its outcome')
    try:
        append_ledger(dict(event='claimed', attempt=ATTEMPT_NAME, driver_sha256=own,
                           utc=receipt['claimed_utc'], pid=os.getpid()))
        run_child(receipt, token)
    except BaseException as error:
        receipt['driver_error'] = f'{type(error).__name__}: {error}'
    finally:
        postflight(receipt, before)
        digest = persist(receipt)
    post = receipt.get('postflight', {})
    ok = (receipt.get('outcome') == 'exited' and receipt.get('returncode') == 0
          and not receipt.get('containment_breach') and 'driver_error' not in receipt
          and post.get('changed') == [] and not post.get('blockers')
          and post.get('candidate_problem') is None)
    print(json.dumps(dict(status='attempt-recorded', outcome=receipt.get('outcome'),
                          returncode=receipt.get('returncode'),
                          containment_breach=receipt.get('containment_breach'),
                          integrity_changed=post.get('changed'), receipt=RECEIPT,
                          receipt_sha256=digest), indent=2))
    return 0 if ok else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--preflight-only', action='store_true')
    mode.add_argument('--execute', action='store_true')
    parser.add_argument('--approved-driver-sha256')
    args = parser.parse_args(argv)
    if args.preflight_only:
        report, _ = preflight()
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0 if not report['blockers'] else 2
    if not args.approved_driver_sha256:
        parser.error('--execute requires --approved-driver-sha256')
    return execute(args.approved_driver_sha256)


if __name__ == '__main__':
    sys.exit(main())
