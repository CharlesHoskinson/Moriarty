"""One fixed ordinary compile+14-test run; no SDK/native/proof consumer."""
from pathlib import Path
import datetime, hashlib, json, os, re, shutil, signal, subprocess, sys, tempfile, time

D = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
plan = json.loads((D / 'CHECK-PLAN.json').read_bytes())
assert sha(__file__) == plan['driverSha256']
assert sha(D / 'SOURCE-PINS.json') == plan['sourcePinsSha256']
assert sha(D / 'VENDOR-CHECKSUM-PINS.json') == plan['vendorPinsSha256']
assert sha(D / 'cargo-config.toml') == plan['cargoConfigSha256']

# Setup is a fixed, separately bounded host copy, not a product invocation.
if sys.argv[1:] and sys.argv[1] == '--copy-owned-cache':
    assert len(sys.argv) == 3
    owned = Path(sys.argv[2])
    old_home = Path(plan['cacheSourceCargoHome'])
    shutil.copytree(plan['cacheSourceTarget'], owned / 'target', symlinks=False)
    home = owned / 'cargo-home'
    home.mkdir()
    (home / 'config.toml').write_bytes((D / 'cargo-config.toml').read_bytes())
    # Only the locked public midnight-ledger cache and exact checkout revisions.
    git_name = 'midnight-ledger-b2f9c59d942dfdca'
    database = old_home / 'git/db' / git_name
    checkout_04 = old_home / 'git/checkouts' / git_name / '04c9c5d'
    checkout_a01 = old_home / 'git/checkouts' / git_name / 'a01a1ea'
    for selected in [database, checkout_04, checkout_a01]:
        assert selected.is_dir(), str(selected)
        selected_root = selected.resolve(strict=True)
        for link in selected.rglob('*'):
            if link.is_symlink():
                assert not Path(os.readlink(link)).is_absolute(), str(link)
                assert link.resolve(strict=False).is_relative_to(selected_root), str(link)
    shutil.copytree(database, home / 'git/db' / git_name, symlinks=True)
    shutil.copytree(checkout_04, home / 'git/checkouts' / git_name / '04c9c5d', symlinks=True)
    shutil.copytree(checkout_a01, home / 'git/checkouts' / git_name / 'a01a1ea', symlinks=True)
    (home / 'registry').mkdir()
    shutil.copyfile(old_home / 'registry/CACHEDIR.TAG', home / 'registry/CACHEDIR.TAG')
    (owned / 'cwd').mkdir()
    (owned / 'tmp').mkdir()
    raise SystemExit(0)
assert len(sys.argv) == 1

O = Path(plan['outputDirectory'])
O.mkdir(exist_ok=False)
write = lambda name, value: (O / name).write_text(json.dumps(value, indent=2) + '\n')
checkout = Path(plan['checkout'])
pins = json.loads((D / 'SOURCE-PINS.json').read_bytes())
vendor_pins = json.loads((D / 'VENDOR-CHECKSUM-PINS.json').read_bytes())
owned = None
sessions = []
result = {'automaticRetry': False, 'productAttemptConsumed': False,
          'compileAndSelected14Passed': False, 'qualification': plan['scopeQualification']}

def verify():
    for row in pins:
        path = checkout / row['path']
        assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256'], row['path']
    for row in plan['tools']:
        assert sha(row['path']) == row['sha256'], row['path']
    vendor = Path(plan['vendorDirectory'])
    for row in vendor_pins:
        path = vendor / row['path']
        assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256'], row['path']
        # Whole checksum manifests bind their declared file hashes; Cargo's
        # existing directory-source consumer checks the files it actually uses.
    oracle = D / 'independent-normal-freeze'
    assert sha(oracle / 'FREEZE-MANIFEST.json') == plan['oracleFreezeSha256']
    for row in json.loads((oracle / 'FREEZE-MANIFEST.json').read_bytes())['artifacts']:
        path = oracle / row['path']
        assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256'], row['path']
    assert sha(Path(plan['cacheSourceCargoHome']) / 'config.toml') == plan['cargoConfigSha256']
    assert sha(Path(plan['cacheSourceTarget']) / 'debug/mori') == plan['warmPublicBinarySha256']

def git_identity():
    process = subprocess.run(['/usr/bin/git', 'rev-parse', 'HEAD', 'HEAD^{tree}'],
        cwd=checkout, env=plan['metadataEnvironment'], stdin=subprocess.DEVNULL,
        capture_output=True, text=True, timeout=2)
    assert process.returncode == 0, process.stderr
    return process.stdout.splitlines()

def phase(name, argv, environment, cwd, parent_wait):
    started = time.monotonic()
    expired = False
    with (O / (name + '.stdout')).open('wb') as out, (O / (name + '.stderr')).open('wb') as err:
        child = subprocess.Popen(argv, cwd=cwd, env=environment,
            stdin=subprocess.DEVNULL, stdout=out, stderr=err, start_new_session=True)
        sessions.append(child.pid)
        write(name + '-CHILD.json', {'pid': child.pid, 'ownedSession': child.pid, 'argv': argv})
        try:
            child.wait(timeout=parent_wait)
        except subprocess.TimeoutExpired:
            expired = True
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            child.wait(timeout=2)
    census = subprocess.run(['/usr/bin/ps', '--sid', str(child.pid), '-o', 'pid=,sid=,stat='],
        capture_output=True, text=True, timeout=1)
    assert census.returncode in (0, 1)
    receipt = {'argv': argv, 'exit': child.returncode, 'wallSeconds': time.monotonic() - started,
        'parentDeadlineExceeded': expired, 'ownedSessionRemaining': census.stdout.splitlines(),
        'stdoutSha256': sha(O / (name + '.stdout')), 'stderrSha256': sha(O / (name + '.stderr'))}
    write(name + '-RESULT.json', receipt)
    assert not receipt['ownedSessionRemaining'], receipt
    return receipt

try:
    verify()
    result['sourcePinsBefore'] = len(pins)
    result['gitIdentityBefore'] = git_identity()
    write('START.json', {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'planSha256': sha(D / 'CHECK-PLAN.json'), 'sourcePins': len(pins),
        'gitIdentity': result['gitIdentityBefore'], 'productAttemptConsumed': False})
    owned = Path(tempfile.mkdtemp(prefix='doctorwatson-core3-mcp-combined-check-'))
    environment = dict(plan['environment'])
    environment.update({'CARGO_HOME': str(owned / 'cargo-home'),
        'CARGO_TARGET_DIR': str(owned / 'target'), 'TMPDIR': str(owned / 'tmp')})
    setup_argv = plan['setupArgvPrefix'] + [str(owned)]
    result['setup'] = phase('SETUP', setup_argv, plan['metadataEnvironment'], D, 65)
    assert result['setup']['exit'] == 0 and not result['setup']['parentDeadlineExceeded']
    assert sha(owned / 'cargo-home/config.toml') == plan['cargoConfigSha256']
    write('EFFECTIVE-INPUTS.json', {'ownedRoot': str(owned), 'environment': environment,
        'argv': plan['argv'], 'copiedTargetSource': plan['cacheSourceTarget'],
        'independentFreeze': plan['oracleFreezeSha256']})
    result['productAttemptConsumed'] = True
    result['product'] = phase('PRODUCT', plan['argv'], environment, owned / 'cwd', 95)
    result['sourcePinsAfter'] = len(pins)
    verify()
    result['gitIdentityAfter'] = git_identity()
    assert result['gitIdentityAfter'] == result['gitIdentityBefore']
    text = (O / 'PRODUCT.stdout').read_text()
    errtext = (O / 'PRODUCT.stderr').read_text()
    result['testSummaries'] = re.findall(r'test result: (?:ok|FAILED)\..*', text)
    names = re.findall(r'^test (\S+) \.\.\. ok$', text, re.MULTILINE)
    result['passedTestNames'] = names
    result['selectedTargets'] = re.findall(
        r'Running tests/(mcp_check_stdio|capabilities|core3_public_consumers|core3_stage_test_public)\.rs', errtext)
    result['publicBinaryPath'] = str(owned / 'target/debug/mori')
    result['publicBinarySha256'] = sha(owned / 'target/debug/mori')
    result['testBinaries'] = []
    for target, path in re.findall(r'Running tests/(\w+)\.rs \(([^\n]+)\)', errtext):
        binary = Path(path)
        assert binary.is_relative_to(owned / 'target'), path
        result['testBinaries'].append({'target': target, 'path': path, 'sha256': sha(binary)})
    result['compileAndSelected14Passed'] = (result['product']['exit'] == 0
        and not result['product']['parentDeadlineExceeded']
        and sorted(names) == sorted(plan['selectedTestNames'])
        and sorted(result['selectedTargets']) == sorted(plan['selectedTargets'])
        and len(result['testBinaries']) == 4
        and len(result['testSummaries']) == 4
        and all('0 failed; 0 ignored; 0 measured; 0 filtered out' in s for s in result['testSummaries']))
except BaseException as error:
    result['exception'] = {'type': type(error).__name__, 'message': str(error)}
finally:
    remaining = []
    for sid in sessions:
        census = subprocess.run(['/usr/bin/ps', '--sid', str(sid), '-o', 'pid=,sid=,stat='],
            capture_output=True, text=True, timeout=1)
        remaining.extend(census.stdout.splitlines())
    result['ownedSessionRemaining'] = remaining
    if owned is not None:
        result['ownedRootRetained'] = str(owned)
        result['ownedRootRemoved'] = False
        result['retentionPurpose'] = 'Actual built binary for next separately selected financial observer; root cleans only this owned root later'
    result['endedAt'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    write('RESULT.json', result)
print(json.dumps(result, indent=2))
assert result['compileAndSelected14Passed'] and not result['ownedSessionRemaining']
