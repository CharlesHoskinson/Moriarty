from pathlib import Path
import datetime, hashlib, json, os, re, shutil, signal, stat, subprocess, time

W = Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016')
A = R / 'bounded-source-01'
S = A / 'source'
RUST = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc')
PYTHON = Path('/usr/bin/python3').resolve(strict=True)
DEPS = Path('/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps')
DM = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/compiler-source-projection-stripped-local-check/dependency-manifest.json')

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            h.update(block)
    return h.hexdigest()

def record(path):
    return dict(path=str(path), bytes=path.stat().st_size, sha256=sha(path))

def inventory(root):
    return [record(p) for p in sorted(root.rglob('*')) if p.is_file()]

def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')
    path.chmod(0o444)

def child(name, argv):
    command = ['/usr/bin/env', '-i', 'PATH=/usr/bin:/bin', 'LANG=C.UTF-8',
               'TMPDIR=' + str(A / 'tmp'), *map(str, argv)]
    row = dict(name=name, command=command, cwd=str(A), timeout_seconds=30,
               start_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
    result['steps'].append(row)
    out, err = A / (name + '.stdout'), A / (name + '.stderr')
    started = time.monotonic()
    with out.open('xb') as stdout, err.open('xb') as stderr:
        process = subprocess.Popen(command, cwd=A, env={}, stdin=subprocess.DEVNULL,
                                   stdout=stdout, stderr=stderr, start_new_session=True)
        try:
            row.update(exit=process.wait(timeout=30), timed_out=False)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            row.update(exit=process.wait(), timed_out=True)
    row.update(elapsed_seconds=time.monotonic() - started, stdout=record(out), stderr=record(err))
    out.chmod(0o444)
    err.chmod(0o444)
    print(name, 'exit', row['exit'], 'timeout', row['timed_out'], flush=True)
    if row['exit'] or row['timed_out']:
        raise RuntimeError('first-failure:' + name)
    return out

def source_records(root):
    return {row['path']: sha(root / row['path']) for row in manifest}

def depinfo(name):
    path = A / (name + '.d')
    rows = []
    for line in path.read_text().splitlines():
        if ': ' in line and not line.startswith('#'):
            rows.extend(line.partition(': ')[2].split())
    result.setdefault('depinfo', {})[name] = dict(file=record(path), inputs=sorted(set(rows)))

assert not A.exists(), 'fresh attempt required'
A.mkdir()
S.mkdir()
(A / 'tmp').mkdir()
result = dict(scope='fixed two-source bounded Core diagnostic attempt only',
              terminal_state='running', steps=[], native_authority=0,
              acceptance='none', limits=dict(child_seconds=30, host_memory_cap='none', host_disk_cap='none'),
              not_run=['Cargo', 'K', 'Quint', 'Lean', 'Clean', 'circuits', 'SDK/native/proof/ledger/target'],
              provenance='local snapshot and selected tool/dependency inventories only; transitive source authenticity unestablished')
started = time.monotonic()
before_deps = None
tools_before = None
try:
    # Both exact terminal reviews are read, not inferred from a provider PID.
    for owner in ('opus', 'muse'):
        d = R / ('bounded-source-preflight-' + owner)
        text = (d / 'REPORT.md').read_text()
        assert 'APPROVE_WITH_SCOPE' in text, owner
    manifest = json.loads((R / 'bounded-source-preflight-opus/CANDIDATE.json').read_text())
    assert manifest == json.loads((R / 'bounded-source-preflight-muse/CANDIDATE.json').read_text())
    assert len(manifest) == 74
    manifest.append(dict(path='models/sherlock/k/main.k', bytes=14190,
                         sha256='b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'))
    for row in manifest:
        raw = (W / row['path']).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row['sha256'], row['path']
        dest = S / row['path']
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(raw)
        dest.chmod(0o444)
        row['bytes'] = len(raw)
    assert re.fullmatch(r'[A-Za-z0-9/._-]+', str(S))
    result['source_before'] = source_records(W)
    result['snapshot_before'] = source_records(S)
    write_json(A / 'manifest.json', manifest)
    for directory in sorted([p for p in S.rglob('*') if p.is_dir()], reverse=True) + [S]:
        directory.chmod(0o555)
    assert sha(DM) == 'ce37c42b96cc071280db3c361ad593172aab6f1716419d7cdc9010120ffb2f36'
    dependencies = json.loads(DM.read_text())
    assert set(dependencies['externs']) == {'mori_lexer', 'serde', 'serde_json', 'thiserror', 'sha2', 'k256'}
    assert set(dependencies['proc_macros']) == {'serde_derive', 'thiserror_impl'}
    for rows in dependencies.values():
        for name, row in rows.items():
            assert sha(Path(row['path'])) == row['sha256'], name
    assert RUST.resolve(strict=True) == RUST
    assert sha(RUST) == 'f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f'
    version = child('00-rustc-version', [RUST, '-vV']).read_text()
    assert re.search(r'^release: 1\.99\.0$', version, re.M)
    host = re.search(r'^host: ([A-Za-z0-9_.-]+)$', version, re.M).group(1)
    sysroot = RUST.parent.parent
    target = sysroot / 'lib/rustlib' / host
    tools = {RUST, PYTHON, Path(shutil.which('cc', path='/usr/bin:/bin')).resolve(strict=True),
             Path(shutil.which('ld', path='/usr/bin:/bin')).resolve(strict=True), DM,
             Path('/usr/bin/env').resolve(strict=True)}
    tools.update((sysroot / 'lib').glob('librustc_driver-*.so'))
    tools.update(p.resolve(strict=True) for p in (target / 'bin').rglob('*')
                 if p.is_file() and ('rust-lld' in str(p) or 'gcc-ld' in str(p)))
    tools.update(p.resolve(strict=True) for p in (target / 'lib').rglob('*') if p.is_file())
    tools_before = [record(p) for p in sorted(tools)]
    write_json(A / 'toolchain-before.json', tools_before)
    print('inventorying complete dependency directory', flush=True)
    before_deps = inventory(DEPS)
    write_json(A / 'dependencies-before.json', before_deps)
    by_path = {row['path']: row for row in before_deps}
    for rows in dependencies.values():
        for name, row in rows.items():
            assert by_path[row['path']]['sha256'] == row['sha256'], name
    result['dependency_file_count'] = len(before_deps)
    result['dependency_bytes'] = sum(row['bytes'] for row in before_deps)
    print('dependency inventory complete', len(before_deps), flush=True)
    library = A / 'libmori_core.rlib'
    command = [RUST, '--edition=2024', '--crate-name', 'mori_core', '--crate-type', 'rlib',
               S / 'crates/mori-core/src/lib.rs', '-L', 'dependency=' + str(DEPS),
               '--emit=link,dep-info=' + str(A / '01-core.d'), '-o', library]
    for name, row in sorted(dependencies['externs'].items()):
        command.extend(['--extern', name + '=' + row['path']])
    child('01-compile-actual-core', command)
    depinfo('01-core')
    result['actual_core_library'] = record(library)
    harness = A / 'bounded_harness.rs'
    bounded = S / 'crates/mori-core/src/sherlock/bounded_scalar_source.rs'
    harness.write_text('pub use mori_core::*;\n#[path = "' + str(bounded) + '"] mod bounded_candidate;\n')
    harness.chmod(0o444)
    result['harness'] = record(harness)
    test_names = ['bounded_candidate::tests::' + name for name in re.findall(
        r'#\[test\]\s*fn\s+(\w+)\s*\(', (S / 'crates/mori-core/src/sherlock/bounded_scalar_source_tests.rs').read_text())]
    assert len(test_names) == 14 and len(set(test_names)) == 14
    binary = A / 'bounded-tests'
    child('02-compile-narrow-tests', [RUST, '--edition=2024', '--test', harness,
          '--extern', 'mori_core=' + str(library), '-L', 'dependency=' + str(DEPS),
          '-L', 'dependency=' + str(A), '-C', 'strip=debuginfo',
          '--emit=link,dep-info=' + str(A / '02-tests.d'), '-o', binary])
    depinfo('02-tests')
    listed = child('03-list-exact-tests', [binary, '--list']).read_text()
    actual_names = re.findall(r'^(\S+): test$', listed, re.M)
    assert sorted(actual_names) == sorted(test_names), 'exact 14 names required'
    assert re.search(r'^14 tests, 0 benchmarks$', listed, re.M)
    tested = child('04-run-exact-tests', [binary, '--exact', '--test-threads=1']).read_text()
    actual = re.findall(r'^test (\S+) \.\.\. (\S+)$', tested, re.M)
    assert sorted(actual) == sorted((name, 'ok') for name in test_names)
    assert 'test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;' in tested
    result['test_names'] = test_names
    producer = A / 'producer'
    child('05-compile-source-producer', [RUST, '--edition=2024', '--crate-name', 'sherlock_source_observations',
          S / 'models/sherlock/reference/source_observations.rs', '--extern', 'mori_core=' + str(library),
          '-L', 'dependency=' + str(DEPS), '-L', 'dependency=' + str(A), '-C', 'strip=debuginfo',
          '--emit=link,dep-info=' + str(A / '05-producer.d'), '-o', producer])
    depinfo('05-producer')
    observed = child('06-run-source-producer', [producer])
    raw = json.loads(observed.read_bytes())
    assert len(raw['programs']) == 2 and len(raw['runs']) == 70
    assert sum(len(row['records']) for row in raw['runs']) == 1259
    comparison = child('07-compare-independent-oracles', [PYTHON, '-I', '-B',
                       S / 'models/sherlock/qa/check_source_observations.py', '--observations', observed])
    compared = json.loads(comparison.read_bytes())
    assert compared['status'] == 'pass'
    result['counts'] = compared['counts']
    result['terminal_state'] = 'success'
except Exception as error:
    result.update(terminal_state='failed', error=repr(error))
    print('BOUNDED SOURCE STOP', repr(error), flush=True)
finally:
    if before_deps is not None:
        after_deps = inventory(DEPS)
        write_json(A / 'dependencies-after.json', after_deps)
        result['dependencies_unchanged'] = before_deps == after_deps
        if before_deps != after_deps:
            result.update(terminal_state='failed-postflight', dependency_inventory_mismatch=True)
    if tools_before is not None:
        tools_after = [record(p) for p in sorted(tools)]
        write_json(A / 'toolchain-after.json', tools_after)
        result['tools_unchanged'] = tools_before == tools_after
        if tools_before != tools_after:
            result['terminal_state'] = 'failed-postflight'
    if 'source_before' in result:
        result['source_after'] = source_records(W)
        result['snapshot_after'] = source_records(S)
        result['source_unchanged'] = result['source_before'] == result['source_after']
        result['snapshot_unchanged'] = result['snapshot_before'] == result['snapshot_after']
        if not result['source_unchanged'] or not result['snapshot_unchanged']:
            result['terminal_state'] = 'failed-postflight'
    result['elapsed_seconds'] = time.monotonic() - started
    write_json(A / 'result.json', result)
    print('bounded-source-01', result['terminal_state'], flush=True)
