from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tarfile, tempfile, time, tomllib, os, signal, re

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
I = Path('/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017')
P = R / 'lsp-shared-dispatch-red-check-preparation-20261018'
C = R / 'lsp-shared-dispatch-red-check-cycle-20261018'
C.mkdir(exist_ok=False)
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p, value): p.write_text(json.dumps(value, indent=2) + '\n')
def git(*args): return subprocess.check_output(['git', '--no-optional-locks', '-C', str(I), *args]).decode().strip()
subject_file = P / 'CURRENT-CANDIDATE.json'
subject_digest = sha(subject_file)
subject = json.loads(subject_file.read_text())
prepared_pins = P / 'SOURCE-PINS.json'
assert sha(prepared_pins) == subject['sourcePinsSha256']
pins = json.loads(prepared_pins.read_text())['files']
def verify():
    assert git('rev-parse', 'HEAD') == subject['head'] and git('rev-parse', 'HEAD^{tree}') == subject['tree']
    assert not git('status', '--porcelain=v1')
    assert sha(subject_file) == subject_digest and sha(Path(__file__)) == subject['driverSha256']
    assert sha(prepared_pins) == subject['sourcePinsSha256']
    for row in pins: assert sha(I / row['path']) == row['sha256'], row['path']
    for tool in subject['toolPins']: assert sha(Path(tool['path']).resolve()) == tool['sha256'], tool['path']
verify()
write(C / 'SOURCE-PINS.json', json.loads(prepared_pins.read_text()))
root = Path(tempfile.mkdtemp(prefix='doctorwatson-lsp-normal-'))
for name in ['cwd', 'tmp', 'cargo-home', 'vendor']: (root / name).mkdir()
home = root / 'cargo-home'
(home / 'git').symlink_to('/home/charl/.cargo/git', target_is_directory=True)
toolchain = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
toolpins = subject['toolPins']
env = {'PATH': str(toolchain) + ':/usr/bin:/bin', 'RUSTC': str(toolchain / 'rustc'),
    'RUSTDOC': str(toolchain / 'rustdoc'), 'CARGO_HOME': str(home),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s',
    str(toolchain / 'cargo'), 'test', '--manifest-path', str(I / 'Cargo.toml'),
    '--frozen', '-j1', '-p', 'mori', '--no-default-features', '--test', 'lsp',
    '--', subject['expectedSelectedTest'], '--exact', '--test-threads=1']
write(C / 'CHECK-PLAN.json', {'preparedAt': now(), 'source': subject, 'argv': argv,
    'cwd': str(root / 'cwd'), 'environment': env, 'toolPins': toolpins,
    'attempts': 1, 'timeoutSeconds': 90, 'killGraceSeconds': 5,
    'scope': 'One selected RED ordinary native-OFF LSP/check parity test;12input/10expectedsemanticmessages+4publiccheckprojections. No SDK/native/proof/key/wallet/ledger or security claim; no auto green/retry.',
    'expected': 'RED only: actual selected assertion difference between old Core1-only header message and frozen both-profile message; compilation failure/timeout is not expected red. Original full normal-session test is filtered, not executed.',
    'sourceSet': 'Original locked registry identities plus three actual Cargo-selected new identities. All archives verified against lock; temporary directory-source replacement, no global source/cache/index mutation.',
    'cleanupScope': 'Owned process session census and temporary root only; no detached-process/kernel-level exclusion claim.'})
result = {'startedAt': now(), 'attemptConsumed': False, 'expectedRedAssertionObserved': False}
owned_sid, census_empty, group_stopped = None, None, False
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
    assert len(packages) == subject['registryPackageCount'] == 426
    write(C / 'PACKAGE-SOURCE-PINS.json', packages)
    config = '[source.crates-io]\nreplace-with = "scrapling-custody"\n[source.scrapling-custody]\ndirectory = ' + json.dumps(str(root / 'vendor')) + '\n'
    (home / 'config.toml').write_text(config)
    (C / 'CARGO-CONFIG.toml').write_text(config)
    verify()
    assert len(env) == 13
    result.update(startedAt=now(), attemptConsumed=True)
    start = time.monotonic()
    deadline = start + 95
    expired, reaped = False, False
    with (C / 'STDOUT.log').open('wb') as out, (C / 'STDERR.log').open('wb') as err:
        child = subprocess.Popen(argv, cwd=root / 'cwd', env=env, stdout=out, stderr=err, start_new_session=True)
        owned_sid = child.pid
        write(C / 'CHILD.json', {'pid': child.pid, 'session': child.pid, 'startedAt': now(), 'parentMaximumSeconds': 95})
        try:
            child.wait(timeout=max(0, deadline - time.monotonic()))
            reaped = True
        except subprocess.TimeoutExpired:
            expired = True
            try: os.killpg(owned_sid, signal.SIGKILL); group_stopped = True
            except ProcessLookupError: group_stopped = True
            child.poll()  # Nonblocking reap; no added cleanup grace/wait after95s.
            reaped = child.returncode is not None
    result.update(exitCode=child.returncode, wallSeconds=time.monotonic() - start, finishedAt=now(),
        parentDeadlineExceeded=expired, ownedGroupStopSent=group_stopped, directChildReaped=reaped,
        stdoutPath=str(C / 'STDOUT.log'), stderrPath=str(C / 'STDERR.log'))
    write(C / 'RESULT.json', result)  # Terminal facts/streams persist before census and heavy postchecks.
    members, census_error = None, None
    if expired or time.monotonic() >= deadline:
        census_error = 'Parent deadline exhausted; remaining members unknown, no added wait.'
    else:
        try:
            ps = subprocess.run(['/bin/ps', '--sid', str(owned_sid), '-o', 'pid=,sid=,stat='],
                capture_output=True, text=True, timeout=min(0.25, deadline-time.monotonic()))
            assert ps.returncode in (0, 1), ps.returncode
            members = ps.stdout.splitlines()
            census_empty = not members
        except (OSError, subprocess.TimeoutExpired, AssertionError) as error: census_error = type(error).__name__
    if census_empty is not True and not group_stopped:
        try: os.killpg(owned_sid, signal.SIGKILL); group_stopped = True
        except ProcessLookupError: group_stopped = True
    write(C / 'PROCESS-CLEANUP.json', {'ownedSession': owned_sid, 'remainingMembers': members,
        'censusError': census_error, 'ownedGroupStopSent': group_stopped, 'noAddedWaitOrGrace': True})
    result.update(remainingMembers=members, censusError=census_error, ownedGroupStopSent=group_stopped,
        stdoutSha256=sha(C / 'STDOUT.log'), stderrSha256=sha(C / 'STDERR.log'))
    write(C / 'RESULT.json', result)
    verify()  # Complete source/task/four-tool checks after immediate owned-session assessment, including nonzero exit.
    result.update(sourceCommitAfter=git('rev-parse', 'HEAD'), sourceTreeAfter=git('rev-parse', 'HEAD^{tree}'),
        sourceStatusAfter=git('status', '--porcelain=v1'), sourceTaskToolPinsVerifiedAfter=True)
    text = (C/'STDOUT.log').read_text(errors='replace') + (C/'STDERR.log').read_text(errors='replace')
    summary = re.search(r'test result: FAILED\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', text)
    observed = dict(zip(['passed','failed','ignored','measured','filtered'], map(int,summary.groups()))) if summary else None
    old_message = 'Expected language "moriarty/1"; found "moriarty/99".'
    frozen_message = 'Expected language "moriarty/1" or "moriarty/2"; found "moriarty/99".'
    forms = lambda message: [message, json.dumps(message)[1:-1]]
    matches = {label: [form for form in forms(message) if form in text]
        for label,message in [('actualOldHeader',old_message),('frozenBothProfiles',frozen_message)]}
    result.update(observedRustSummary=observed, headerAssertionTextMatches=matches,
        expectedRedAssertionObserved=(child.returncode == 101 and not expired and reaped
            and observed == subject['expectedRedSummary'] and subject['expectedSelectedTest'] in text
            and 'assertion `left == right` failed' in text and all(matches.values())))
    write(C / 'RESULT.json', result)
    assert result['expectedRedAssertionObserved'], 'Compilation/timeout/other failure is not the expected RED assertion.'
    assert census_empty is True, 'Owned session cleanup not established.'
finally:
    if owned_sid is not None and census_empty is not True and not group_stopped:
        try: os.killpg(owned_sid, signal.SIGKILL); group_stopped = True
        except ProcessLookupError: group_stopped = True
    if owned_sid is None or census_empty is True: shutil.rmtree(root)
    try: verify(); result['finalSourceTaskToolPinsVerified'] = True
    except BaseException as error: result['finalSourceTaskToolPinsVerified'] = False; result['finalPinError'] = type(error).__name__
    write(C / 'CLEANUP.json', {'at': now(), 'ownedRoot': str(root), 'existsAfter': root.exists(),
        'knownOwnedSessionEmpty': census_empty, 'ownedGroupStopSent': group_stopped, 'noAddedWaitOrGrace': True})
    result['ownedRootExistsAfter'] = root.exists()
    write(C / 'RESULT.json', result)
    print(json.dumps(result), flush=True)
