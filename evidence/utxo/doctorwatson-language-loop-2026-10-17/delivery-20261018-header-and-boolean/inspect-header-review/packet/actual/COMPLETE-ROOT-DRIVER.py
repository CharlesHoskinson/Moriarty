"""One finite ordinary RED/GREEN cycle; no scheduler or native SDK use."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import shutil
import subprocess
import tempfile
import time

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K = Path('/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018')
P = R / 'concurrency-increase-main238/inspect-header-current615'
C = R / 'inspect-header-dispatch-check-20261018'
HEAD = '615eceee9c6250d2f203cb5807d5c75482dbdb5d'
TREE = '4a78114c06d5fbb858a4bf97b922c0446fabe978'
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
TEST = 'crates/mori/tests/inspect_header_dispatch.rs'
PRODUCTION = 'crates/mori/src/commands/inspect.rs'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(name, value):
    target = C / name
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(value, indent=2) + '\n')


def git(*args):
    return subprocess.check_output(['git', *args], cwd=K).decode().strip()


assert C.is_dir()
assert git('rev-parse', 'HEAD') == HEAD
assert git('rev-parse', 'HEAD^{tree}') == TREE
assert git('status', '--porcelain=v1') == '?? ' + TEST
names = subprocess.check_output([
    'git', 'ls-tree', '-r', '--name-only', HEAD, 'crates',
    'examples/core/continuations', 'Cargo.toml', 'Cargo.lock',
    'rust-toolchain.toml', 'RUST_GUIDELINES.md', 'docs/NATIVE-CORE.md',
], cwd=K).decode().splitlines()
pins = []
for name in names:
    raw = subprocess.check_output(['git', 'show', HEAD + ':' + name], cwd=K)
    assert (K / name).read_bytes() == raw, name
    pins.append({'path': name, 'sha256': hashlib.sha256(raw).hexdigest()})
pins.append({'path': TEST, 'sha256': sha(K / TEST)})
write('SOURCE-PINS-BEFORE.json', {'head': HEAD, 'tree': TREE, 'files': pins})
for name in ['EXPECTATIONS.json', 'EXPECTATIONS.sha256']:
    (C / name).write_bytes((P / name).read_bytes())
assert sha(K / TEST) == sha(C / 'TEST-AFTER-FORMAT.rs')
assert sha(K / PRODUCTION) == sha(P / 'before' / PRODUCTION)

root = Path(tempfile.mkdtemp(prefix='doctorwatson-inspect-header-'))
for name in ['cwd', 'tmp', 'cargo-home']:
    (root / name).mkdir()
for name in ['registry', 'git']:
    (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
env = {
    'PATH': str(TC) + ':/usr/bin:/bin', 'RUSTC': str(TC / 'rustc'),
    'RUSTDOC': str(TC / 'rustdoc'), 'CARGO_HOME': str(root / 'cargo-home'),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8',
    'TMPDIR': str(root / 'tmp'),
}
configs = set()
for directory in [K, *K.parents, root / 'cwd', *(root / 'cwd').parents]:
    for name in ['config', 'config.toml']:
        path = directory / '.cargo' / name
        assert not path.exists(), str(path)
        configs.add(str(path))
write('CHECK-PLAN.json', {
    'recordedAt': now(), 'head': HEAD, 'tree': TREE,
    'expectationFreezeSha256': sha(P / 'EXPECTATIONS.sha256'),
    'caseSpecificationSha256': sha(P / 'EXPECTATIONS.json'),
    'sourcePinsBeforeSha256': sha(C / 'SOURCE-PINS-BEFORE.json'),
    'environment': env, 'ownedRoot': str(root),
    'absentAncestorCargoConfigs': sorted(configs),
    'toolPins': [{'path': str(path), 'sha256': sha(path)} for path in
                 [TC / 'cargo', TC / 'rustc', TC / 'rustdoc', Path('/usr/bin/timeout')]],
    'scope': 'Ordinary source-only inspect header diagnostics RED/GREEN and existing Core1/Core2 capability/stage-plan controls. No SDK/native/proof/key/wallet/ledger execution; no financial/work oracle replacement.',
})


def run(name, args, expected_exit, expected_summary, capture=False):
    output = C / name
    output.mkdir()
    argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s', *args]
    child_env = dict(env)
    if capture:
        child_env['MORI_INSPECT_HEADER_OBSERVATIONS'] = str(output / 'public-cli')
    write(name + '/START.json', {'startedAt': now(), 'argv': argv,
          'cwd': str(root / 'cwd'), 'head': HEAD, 'tree': TREE,
          'observations': child_env.get('MORI_INSPECT_HEADER_OBSERVATIONS')})
    begin = time.monotonic()
    with (output / 'STDOUT.log').open('xb') as stdout, (output / 'STDERR.log').open('xb') as stderr:
        child = subprocess.Popen(argv, cwd=root / 'cwd', env=child_env,
                                 stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        write(name + '/CHILD.json', {'pid': child.pid, 'session': child.pid})
        code = child.wait()
    remaining = [int(pid) for pid, sid in
                 (line.split() for line in subprocess.check_output(['ps', '-eo', 'pid=,sid=']).decode().splitlines())
                 if int(sid) == child.pid]
    result = {'name': name, 'exitCode': code, 'wallSeconds': time.monotonic() - begin,
              'finishedAt': now(), 'stdoutSha256': sha(output / 'STDOUT.log'),
              'stderrSha256': sha(output / 'STDERR.log'), 'ownedSessionRemaining': remaining}
    write(name + '/RESULT.json', result)
    results.append(result)
    print(json.dumps(result), flush=True)
    assert not remaining
    assert code == expected_exit
    assert expected_summary in (output / 'STDOUT.log').read_text()


def cargo(target, *selection):
    return [str(TC / 'cargo'), 'test', '--manifest-path', str(K / 'Cargo.toml'),
            '--frozen', '-j1', '-p', 'mori', '--no-default-features', '--test', target,
            *selection, '--', '--exact', '--test-threads=1']


results = []
try:
    selected = cargo('inspect_header_dispatch', 'source_only_inspect_keeps_complete_header_errors')
    run('red', selected, 101, '0 passed; 1 failed', capture=True)
    assert all(sha(K / pin['path']) == pin['sha256'] for pin in pins)
    before = (K / PRODUCTION).read_bytes()
    after = (P / 'candidate' / PRODUCTION).read_bytes()
    (C / 'INSPECT-BEFORE.rs').write_bytes(before)
    (C / 'INSPECT-AFTER.rs').write_bytes(after)
    (K / PRODUCTION).write_bytes(after)
    write('SOURCE-MUTATION.json', {'path': PRODUCTION, 'beforeSha256': hashlib.sha256(before).hexdigest(),
                                  'afterSha256': sha(K / PRODUCTION),
                                  'scope': 'Propagate existing language() Error; sole production mutation after RED.'})
    for pin in pins:
        if pin['path'] == PRODUCTION:
            pin['sha256'] = sha(K / PRODUCTION)
    write('SOURCE-PINS-AFTER.json', {'head': HEAD, 'tree': TREE, 'files': pins})
    run('green', selected, 0, '1 passed; 0 failed', capture=True)
    run('core1-positive', cargo('capabilities', 'reported_core_profiles_select_real_source_only_cli_consumers'), 0, '1 passed; 0 failed')
    run('core2-positive', cargo('core_continuations', 'check_and_source_inspect_route_core_two_without_legacy_consumers'), 0, '1 passed; 0 failed')
    assert git('rev-parse', 'HEAD') == HEAD
    assert all(sha(K / pin['path']) == pin['sha256'] for pin in pins)
    assert set(git('status', '--porcelain=v1').splitlines()) == {'M ' + PRODUCTION, '?? ' + TEST}
finally:
    shutil.rmtree(root)
    write('FINAL-RESULT.json', {'finishedAt': now(), 'baseHead': HEAD, 'baseTree': TREE,
          'results': results, 'allFourCommandsCompleted': len(results) == 4,
          'ownedRoot': str(root), 'existsAfter': root.exists(),
          'sourcePinsAfterSha256': sha(C / 'SOURCE-PINS-AFTER.json') if (C / 'SOURCE-PINS-AFTER.json').exists() else None,
          'scope': 'RED onefailedaggregate/3rawCLIcaptures; GREEN oneaggregatepasses3exactheaders; twoexistingcapability/Core2inspectpositivecontrolstests retained. No newindependentfinancial/workoracle or full-language/targetacceptance.'})
