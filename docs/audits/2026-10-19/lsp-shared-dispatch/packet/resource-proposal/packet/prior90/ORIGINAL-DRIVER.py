from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tarfile, tempfile, time, tomllib

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
I = Path('/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017')
C = R / 'rust-lsp-current-integration-cycle' / 'normal-protocol-check-a1'
C.mkdir(exist_ok=False)
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p, value): p.write_text(json.dumps(value, indent=2) + '\n')
def git(*args): return subprocess.check_output(['git', '-C', str(I), *args]).decode().strip()
subject = json.loads((C.parent / 'CURRENT-CANDIDATE.json').read_text())
assert git('rev-parse', 'HEAD') == subject['head']
assert not git('status', '--porcelain=v1')
for row in subject['files']: assert sha(I / row['path']) == row['sha256'], row['path']
paths = git('ls-tree', '-r', '--name-only', subject['head'], '--', 'Cargo.toml', 'Cargo.lock',
    'crates/mori', 'crates/mori-core', 'crates/mori-ast', 'crates/mori-span',
    'crates/mori-lexer', 'crates/mori-parser', 'crates/mori-checker',
    'crates/mori-diagnostics', 'crates/mori-fmt').splitlines()
pins = []
for path in paths:
    original = subprocess.check_output(['git', '-C', str(I), 'show', subject['head'] + ':' + path])
    assert (I / path).read_bytes() == original, path
    pins.append({'path': path, 'sha256': hashlib.sha256(original).hexdigest(), 'bytes': len(original)})
write(C / 'SOURCE-PINS.json', {'head': subject['head'], 'tree': subject['tree'], 'files': pins})
root = Path(tempfile.mkdtemp(prefix='doctorwatson-lsp-normal-'))
for name in ['cwd', 'tmp', 'cargo-home', 'vendor']: (root / name).mkdir()
home = root / 'cargo-home'
(home / 'git').symlink_to('/home/charl/.cargo/git', target_is_directory=True)
toolchain = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
toolpins = [{'path': str(p), 'sha256': sha(p.resolve())} for p in
    [toolchain / 'cargo', toolchain / 'rustc', toolchain / 'rustdoc', Path('/usr/bin/timeout')]]
env = {'PATH': str(toolchain) + ':/usr/bin:/bin', 'RUSTC': str(toolchain / 'rustc'),
    'RUSTDOC': str(toolchain / 'rustdoc'), 'CARGO_HOME': str(home),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s',
    str(toolchain / 'cargo'), 'test', '--manifest-path', str(I / 'Cargo.toml'),
    '--frozen', '-j1', '-p', 'mori', '--no-default-features', '--test', 'lsp',
    '--', '--test-threads=1']
write(C / 'CHECK-PLAN.json', {'preparedAt': now(), 'source': subject, 'argv': argv,
    'cwd': str(root / 'cwd'), 'environment': env, 'toolPins': toolpins,
    'attempts': 1, 'timeoutSeconds': 90, 'killGraceSeconds': 5,
    'scope': 'Inspected ordinary native-OFF public tooling test: one normal eleven-message session, nine expected semantic messages. Existing ordinary source/tooling authority; not SDK/native/proof/key/wallet/ledger execution or a security/hostile-input test. No extra approval gate introduced.',
    'expected': 'Frozen complete authored normal session; not globally blind. Independent L12 different normal inputs are frozen separately and are not claimed executed by this test.',
    'sourceSet': 'Original locked registry identities plus three actual Cargo-selected new identities. All archives verified against lock; temporary directory-source replacement, no global source/cache/index mutation.',
    'cleanupScope': 'Owned process session census and temporary root only; no detached-process/kernel-level exclusion claim.'})
result = {'startedAt': now(), 'attemptConsumed': False}
try:
    configs = []
    for directory in [I, *I.parents, root / 'cwd', *(root / 'cwd').parents]:
        for name in ['config', 'config.toml']:
            p = directory / '.cargo' / name
            assert not p.exists(), str(p)
            configs.append(str(p))
    write(C / 'ABSENT-ANCESTOR-CONFIGS.json', configs)
    cache = Path('/home/charl/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f')
    src = Path('/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f')
    platform = R / 'lsp-locked-platform-dependency-custody-20261018'
    additions = R / 'rust-lsp-public-dependency-custody'
    acquired = {x['name'] + '-' + x['version']: x for x in json.loads((additions / 'PACKAGES.json').read_text())}
    packages = []
    for pkg in tomllib.loads((I / 'Cargo.lock').read_text())['package']:
        if not pkg.get('source', '').startswith('registry+'): continue
        name = pkg['name'] + '-' + pkg['version']
        archive, source = cache / (name + '.crate'), src / name
        if not archive.is_file():
            if name in acquired:
                archive = additions / acquired[name]['archive']['path']
                source = additions / 'source' / name
            else:
                archive = platform / 'archives' / (name + '.crate')
                source = platform / 'source' / name
        assert sha(archive) == pkg['checksum'], name
        dest = root / 'vendor' / name
        dest.mkdir()
        filehashes = {}
        with tarfile.open(archive, 'r:gz') as tf:
            for member in tf.getmembers():
                if not member.isfile(): continue
                relative = Path(member.name).relative_to(name)
                assert not relative.is_absolute() and '..' not in relative.parts
                body = tf.extractfile(member).read()
                filehashes[str(relative)] = hashlib.sha256(body).hexdigest()
                if str(relative) == 'Cargo.toml': assert (source / relative).read_bytes() == body, name
        for child in source.iterdir():
            if child.name not in ['.cargo-checksum.json', '.cargo-ok']:
                (dest / child.name).symlink_to(child, target_is_directory=child.is_dir())
        write(dest / '.cargo-checksum.json', {'files': filehashes, 'package': pkg['checksum']})
        packages.append({'identity': name, 'archive': str(archive), 'sha256': pkg['checksum'],
            'source': str(source), 'archiveFileHashes': len(filehashes)})
    write(C / 'PACKAGE-SOURCE-PINS.json', packages)
    config = '[source.crates-io]\nreplace-with = "scrapling-custody"\n[source.scrapling-custody]\ndirectory = ' + json.dumps(str(root / 'vendor')) + '\n'
    (home / 'config.toml').write_text(config)
    (C / 'CARGO-CONFIG.toml').write_text(config)
    for row in pins: assert sha(I / row['path']) == row['sha256'], row['path']
    result.update(startedAt=now(), attemptConsumed=True)
    start = time.monotonic()
    with (C / 'STDOUT.log').open('wb') as out, (C / 'STDERR.log').open('wb') as err:
        child = subprocess.Popen(argv, cwd=root / 'cwd', env=env, stdout=out, stderr=err, start_new_session=True)
        write(C / 'CHILD.json', {'pid': child.pid, 'session': child.pid, 'startedAt': now()})
        code = child.wait()
    members = [line for line in subprocess.check_output(['/bin/ps', '-eo', 'pid=,sid=,stat='], text=True).splitlines()
        if len(line.split()) >= 3 and line.split()[1] == str(child.pid)]
    write(C / 'PROCESS-CLEANUP.json', {'ownedSession': child.pid, 'remainingMembers': members})
    assert not members, members
    result.update(exitCode=code, wallSeconds=time.monotonic() - start,
        finishedAt=now(), sourceCommitAfter=git('rev-parse', 'HEAD'),
        sourceTreeAfter=git('rev-parse', 'HEAD^{tree}'), sourceStatusAfter=git('status', '--porcelain=v1'),
        stdoutSha256=sha(C / 'STDOUT.log'), stderrSha256=sha(C / 'STDERR.log'))
    for row in pins: assert sha(I / row['path']) == row['sha256'], row['path']
    assert result['sourceCommitAfter'] == subject['head'] and not result['sourceStatusAfter']
finally:
    shutil.rmtree(root)
    write(C / 'CLEANUP.json', {'at': now(), 'ownedRoot': str(root), 'existsAfter': root.exists()})
    result['ownedRootExistsAfter'] = root.exists()
    write(C / 'RESULT.json', result)
    print(json.dumps(result), flush=True)
