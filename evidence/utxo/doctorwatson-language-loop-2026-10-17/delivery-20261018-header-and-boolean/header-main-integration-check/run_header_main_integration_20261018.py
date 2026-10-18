"""Finite ordinary integration checks for the three existing header repairs."""
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
W = Path('/home/charl/Moriarty/.worktrees/utxo-header-main-20261018')
C = R / 'header-main-integration-check-20261018'
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
HEAD = '71c22f8a477231be869786eb42f04e3b50d1b3a7'


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


assert git('rev-parse', 'HEAD') == HEAD
assert not (C / 'CHECK-PLAN.json').exists()
integration = json.loads((C / 'INTEGRATION-SOURCE.json').read_text())
changed = {x['path']: x for x in integration['sourceCopies']}
assert len(changed) == 6
assert set(git('diff', '--name-only').splitlines()) == {p for p, v in changed.items() if v['beforeSha256']}
assert set(git('ls-files', '--others', '--exclude-standard').splitlines()) == {p for p, v in changed.items() if v['beforeSha256'] is None}
for p, v in changed.items():
    assert sha(W / p) == v['afterSha256']
    raw = subprocess.check_output(['git', 'show', v['sourceCommit'] + ':' + p], cwd=W)
    assert (W / p).read_bytes() == raw
names = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', HEAD,
    'crates', 'examples/core/continuations', 'Cargo.toml', 'Cargo.lock',
    'rust-toolchain.toml', 'RUST_GUIDELINES.md', 'docs/NATIVE-CORE.md'], cwd=W).decode().splitlines()
before, after = [], []
for name in names:
    raw = subprocess.check_output(['git', 'show', HEAD + ':' + name], cwd=W)
    digest = hashlib.sha256(raw).hexdigest()
    before.append({'path': name, 'sha256': digest})
    if name in changed:
        assert digest == changed[name]['beforeSha256']
    else:
        assert (W / name).read_bytes() == raw
    after.append({'path': name, 'sha256': sha(W / name)})
after.extend({'path': p, 'sha256': v['afterSha256']} for p, v in changed.items() if v['beforeSha256'] is None)
write('SOURCE-PINS-BEFORE.json', {'head': HEAD, 'files': before})
write('SOURCE-PINS-AFTER.json', {'baseHead': HEAD, 'files': after})
old_expectations = [
    R / 'concurrency-increase-main238/public-header-dispatch/EXPECTATION-FREEZE.json',
    R / 'simulate-header-dispatch-check-20261018/EXPECTED-CASE-SPEC.json',
    R / 'simulate-header-dispatch-check-20261018/EXPECTED-FREEZE.sha256',
    R / 'inspect-header-dispatch-check-20261018/EXPECTATIONS.json',
    R / 'inspect-header-dispatch-check-20261018/EXPECTATIONS.sha256',
]
root = Path(tempfile.mkdtemp(prefix='doctorwatson-header-main-'))
for name in ['cwd', 'tmp', 'cargo-home']:
    (root / name).mkdir()
for name in ['registry', 'git']:
    (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
env = {'PATH': str(TC) + ':/usr/bin:/bin', 'RUSTC': str(TC / 'rustc'),
    'RUSTDOC': str(TC / 'rustdoc'), 'CARGO_HOME': str(root / 'cargo-home'),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
configs = set()
for directory in [W, *W.parents, root / 'cwd', *(root / 'cwd').parents]:
    for name in ['config', 'config.toml']:
        p = directory / '.cargo' / name
        assert not p.exists(), str(p)
        configs.add(str(p))
selections = [
    ('check-headers', 'mori', 'check_header_dispatch', None, 3, None),
    ('simulate-headers', 'mori', 'simulate_header_dispatch', 'normal_simulate_keeps_header_errors_and_empty_core2_control', 1, 'MORI_SIMULATE_HEADER_OBSERVATIONS'),
    ('inspect-headers', 'mori', 'inspect_header_dispatch', 'source_only_inspect_keeps_complete_header_errors', 1, 'MORI_INSPECT_HEADER_OBSERVATIONS'),
    ('check-preserved', 'mori', 'check_source_dispatch', None, 8, None),
    ('capabilities-positive', 'mori', 'capabilities', 'reported_core_profiles_select_real_source_only_cli_consumers', 1, None),
    ('core2-inspect-positive', 'mori', 'core_continuations', 'check_and_source_inspect_route_core_two_without_legacy_consumers', 1, None),
    ('core1-simulate-positive', 'mori', 'logical_meter', 'existing_simulate_bytes_and_execution_stay_unchanged_with_optional_cap', 1, None),
    ('core2-simulate-positive', 'mori', 'core_continuations', 'delivery_false_observation_preserves_boundary_then_completes_at_eleven', 1, None),
    ('library-header-positive', 'mori-core', 'language', 'language_header_diagnostics_name_both_supported_core_profiles', 1, None),
]
write('CHECK-PLAN.json', {'recordedAt': now(), 'baseHead': HEAD,
    'sourcePinsBeforeSha256': sha(C / 'SOURCE-PINS-BEFORE.json'),
    'sourcePinsAfterSha256': sha(C / 'SOURCE-PINS-AFTER.json'),
    'unchangedPreexistingExpectationPins': [{'path': str(p), 'sha256': sha(p)} for p in old_expectations],
    'environment': env, 'ownedRoot': str(root), 'absentAncestorCargoConfigs': sorted(configs),
    'selections': selections, 'perCommandWallSeconds': 90, 'killGraceSeconds': 5,
    'toolPins': [{'path': str(p), 'sha256': sha(p)} for p in [TC / 'cargo', TC / 'rustc', TC / 'rustdoc', Path('/usr/bin/timeout')]],
    'scope': 'Fresh six-path integration on main71. Ordinary CLI/Core header checks and existing positive controls only; no SDK/native/compiler Artifact/proof/key/wallet/ledger. Existing controls retain shared-library/selected-field limits; all financial PRs and acceptance obligations remain.'})
results = []
child = None
try:
    for name, package, target, selected, expected_tests, capture_variable in selections:
        d = C / name
        d.mkdir()
        argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s', str(TC / 'cargo'),
            'test', '--manifest-path', str(W / 'Cargo.toml'), '--frozen', '-j1',
            '-p', package, '--no-default-features', '--test', target]
        if selected:
            argv.append(selected)
        argv.append('--')
        if selected:
            argv.append('--exact')
        argv.append('--test-threads=1')
        selected_env = dict(env)
        if capture_variable:
            selected_env[capture_variable] = str(d / 'public-cli')
        started = now()
        write(name + '/START.json', {'startedAt': started, 'argv': argv,
            'cwd': str(root / 'cwd'), 'baseHead': HEAD,
            'captureVariable': capture_variable, 'observationDirectory': selected_env.get(capture_variable) if capture_variable else None})
        begin = time.monotonic()
        with (d / 'STDOUT.log').open('xb') as stdout, (d / 'STDERR.log').open('xb') as stderr:
            child = subprocess.Popen(argv, cwd=root / 'cwd', env=selected_env,
                stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr, start_new_session=True)
            write(name + '/CHILD.json', {'pid': child.pid, 'ownedSession': child.pid})
            code = child.wait()
        remaining = [int(pid) for pid, sid in (line.split() for line in subprocess.check_output(['ps', '-eo', 'pid=,sid=']).decode().splitlines()) if int(sid) == child.pid]
        result = {'name': name, 'startedAt': started, 'finishedAt': now(), 'exitCode': code,
            'wallSeconds': time.monotonic() - begin, 'stdoutSha256': sha(d / 'STDOUT.log'),
            'stderrSha256': sha(d / 'STDERR.log'), 'ownedSessionRemaining': remaining}
        write(name + '/RESULT.json', result)
        results.append(result)
        print(json.dumps(result), flush=True)
        assert not remaining
        assert code == 0
        assert f'{expected_tests} passed; 0 failed' in (d / 'STDOUT.log').read_text()
        assert all(sha(W / row['path']) == row['sha256'] for row in after)
        assert git('rev-parse', 'HEAD') == HEAD
finally:
    if child is not None:
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        if child.poll() is None:
            child.wait(timeout=1)
    shutil.rmtree(root)
    write('FINAL-RESULT.json', {'finishedAt': now(), 'baseHead': HEAD,
        'results': results, 'allNineCommandsCompleted': len(results) == 9,
        'ownedRoot': str(root), 'existsAfter': root.exists(),
        'sourcePinsAfterSha256': sha(C / 'SOURCE-PINS-AFTER.json'),
        'scope': 'Exact six-path diagnostics integration; original33pending financial/help/docs paths not imported or waived. Full-source/result review and broader product/target acceptance remain pending.'})
