"""One finite source-only public checker observation against frozen literals."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import shutil
import signal
import subprocess
import tempfile
import time

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
W = Path('/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018')
P = R / 'concurrency-increase-main238/grammar-next4c03'
I = R / 'concurrency-increase-main238/boolean-main829f-integration-preparation'
C = R / 'boolean-main-integration-check-20261018'
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
HEAD = '829f4456dfda8d4fc5a4ece84c47f2539f846d9f'
TREE = '76dac56aac5e653729f3e4ad546b74ba07f1f780'


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(name, value):
    p = C / name
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(value, indent=2) + '\n')


def git(*args):
    return subprocess.check_output(['git', *args], cwd=W).decode().strip()


assert not C.exists()
assert git('rev-parse', 'HEAD') == HEAD
assert git('rev-parse', 'HEAD^{tree}') == TREE
assert sha(P / 'EXPECTED-FREEZE.sha256') == '1287136fcb4e57450f28f55591fae8671466018ca8fdd1df93b1b706829b7598'
for line in (P / 'EXPECTED-FREEZE.sha256').read_text().splitlines():
    digest, relative = line.split('  ', 1)
    assert not Path(relative).is_absolute() and '..' not in Path(relative).parts
    assert sha(P / relative) == digest, relative
for line in (I / 'FREEZE.sha256').read_text().splitlines():
    digest, relative = line.split('  ', 1)
    assert not Path(relative).is_absolute() and '..' not in Path(relative).parts
    assert sha(I / relative) == digest, relative
added = [{'path': row['path'], 'sha256': row['afterSha256']} for row in json.loads((I / 'CANDIDATE.json').read_text())['changedPaths']]
assert len(added) == 9
changed_map = {row["path"]: row for row in added}
for row in added:
    assert sha(W / row['path']) == row['sha256'] == sha(I / 'after' / row['path'])
assert git('diff', '--name-only') == 'RUST_GUIDELINES.md'
untracked = set(git('ls-files', '--others', '--exclude-standard').splitlines())
assert untracked == {row['path'] for row in added if row['path'] != 'RUST_GUIDELINES.md'}
names = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', HEAD,
    'crates', 'examples/core/continuations', 'Cargo.toml', 'Cargo.lock',
    'rust-toolchain.toml', 'RUST_GUIDELINES.md', 'docs/NATIVE-CORE.md'], cwd=W).decode().splitlines()
pins = []
for name in names:
    raw = subprocess.check_output(['git', 'show', HEAD + ':' + name], cwd=W)
    if name in changed_map:
        assert name == 'RUST_GUIDELINES.md'
        assert sha(W / name) == changed_map[name]['sha256']
        pins.append({'path': name, 'sha256': sha(W / name), 'beforeGitSha256': hashlib.sha256(raw).hexdigest()})
    else:
        assert (W / name).read_bytes() == raw, name
        pins.append({'path': name, 'sha256': hashlib.sha256(raw).hexdigest()})
pins.extend({'path': row['path'], 'sha256': row['sha256']} for row in added if row['path'] not in names)
C.mkdir()
write('SOURCE-PINS.json', {'baseHead': HEAD, 'baseTree': TREE, 'files': pins})
for name in ['EXPECTED-FREEZE.sha256', 'EXPECTED-CASE-SPEC.json', 'DERIVATION.md']:
    (C / name).write_bytes((P / name).read_bytes())
root = Path(tempfile.mkdtemp(prefix='doctorwatson-boolean-main-'))
for name in ['cwd', 'tmp', 'cargo-home']:
    (root / name).mkdir()
for name in ['registry', 'git']:
    (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
env = {'PATH': str(TC) + ':/usr/bin:/bin', 'RUSTC': str(TC / 'rustc'),
    'RUSTDOC': str(TC / 'rustdoc'), 'CARGO_HOME': str(root / 'cargo-home'),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp'),
    'MORI_BOOLEAN_CHECK_OBSERVATIONS': str(C / 'public-cli')}
configs = set()
for directory in [W, *W.parents, root / 'cwd', *(root / 'cwd').parents]:
    for name in ['config', 'config.toml']:
        p = directory / '.cargo' / name
        assert not p.exists(), str(p)
        configs.add(str(p))
argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s',
    str(TC / 'cargo'), 'test', '--manifest-path', str(W / 'Cargo.toml'),
    '--frozen', '-j1', '-p', 'mori', '--no-default-features', '--test',
    'core_boolean_check_public', 'documented_boolean_programs_reach_public_core_check',
    '--', '--exact', '--test-threads=1']
write('CHECK-PLAN.json', {'recordedAt': now(), 'baseHead': HEAD, 'baseTree': TREE,
    'expectationFreezeSha256': sha(C / 'EXPECTED-FREEZE.sha256'),
    'sourcePinsSha256': sha(C / 'SOURCE-PINS.json'), 'argv': argv, 'environment': env,
    'ownedRoot': str(root), 'absentAncestorCargoConfigs': sorted(configs),
    'toolPins': [{'path': str(p), 'sha256': sha(p)} for p in
        [TC / 'cargo', TC / 'rustc', TC / 'rustdoc', Path('/usr/bin/timeout')]],
    'scope': 'Two positive Core1/Core2 Boolean programs, complete API Program values and four public checker streams on independent header-main base. Source-exposed hand expectations; no evaluator/financial/native/SDK/proof/wallet/ledger run; no RED defect claimed.'})
result = None
child = None
try:
    started = now()
    begin = time.monotonic()
    with (C / 'STDOUT.log').open('xb') as stdout, (C / 'STDERR.log').open('xb') as stderr:
        child = subprocess.Popen(argv, cwd=root / 'cwd', env=env, stdin=subprocess.DEVNULL,
            stdout=stdout, stderr=stderr, start_new_session=True)
        write('CHILD.json', {'pid': child.pid, 'ownedSession': child.pid})
        code = child.wait()
    remaining = [int(pid) for pid, sid in (line.split() for line in
        subprocess.check_output(['ps', '-eo', 'pid=,sid=']).decode().splitlines()) if int(sid) == child.pid]
    result = {'startedAt': started, 'finishedAt': now(), 'exitCode': code,
        'wallSeconds': time.monotonic() - begin, 'stdoutSha256': sha(C / 'STDOUT.log'),
        'stderrSha256': sha(C / 'STDERR.log'), 'ownedSessionRemaining': remaining}
    write('RESULT.json', result)
    print(json.dumps(result), flush=True)
    assert not remaining
    assert code == 0
    assert '1 passed; 0 failed' in (C / 'STDOUT.log').read_text()
    assert all(sha(W / row['path']) == row['sha256'] for row in pins)
    assert git('rev-parse', 'HEAD') == HEAD
    assert git('diff', '--name-only') == 'RUST_GUIDELINES.md'
finally:
    if child is not None:
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        if child.poll() is None:
            child.wait(timeout=1)
    shutil.rmtree(root)
    write('FINAL-RESULT.json', {'finishedAt': now(), 'result': result,
        'ownedRoot': str(root), 'existsAfter': root.exists(),
        'sourcePinsSha256': sha(C / 'SOURCE-PINS.json'),
        'scope': 'Finite source checker coverage only; full grammar, independent financial semantics, compiler Artifact and target acceptance remain open.'})
