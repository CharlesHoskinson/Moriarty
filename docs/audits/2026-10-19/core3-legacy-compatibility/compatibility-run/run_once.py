"""One inspected ordinary legacy compatibility run; no SDK/native features."""
from pathlib import Path
import datetime, hashlib, json, os, re, shutil, signal, subprocess, sys, tempfile, time

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
D = Path(__file__).resolve().parent
P = R / 'core3-inherited-legacy-compatibility-preparation-20261019'
OLD = R / 'core3-mcp-combined-local-check-preparation-v2-20261019'
load = lambda p: json.loads(Path(p).read_bytes())
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
plan = load(P / 'PROPOSED-COMMAND.json')
old = load(OLD / 'CHECK-PLAN.json')
W = Path(plan['cwd'])
pins = load(P / plan['sourcePins'])
O = D / 'actual'

def verify():
    assert sha(P / 'PROPOSED-COMMAND.json') == '1969d149a0eb8fdc93c9b47d77fb090c1e9ba0ce9919fbec193cfad38d60c230'
    assert sha(P / plan['sourcePins']) == plan['sourcePinsSha256']
    assert sha(P / 'FREEZE.sha256') == '1c60c0f240766a31bcc592a009151c59f354eca6e4fb1280e33fddab71c89c21'
    for line in (P / 'FREEZE.sha256').read_text().splitlines():
        digest, rel = line.split('  ', 1)
        assert sha(P / rel) == digest, rel
    for row in pins:
        p = W / row['path']
        assert not p.is_symlink() and p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)
    for row in plan['tools']:
        assert sha(row['path']) == row['sha256'], row['path']
    assert sha(OLD / 'VENDOR-CHECKSUM-PINS.json') == old['vendorPinsSha256']
    for row in load(OLD / 'VENDOR-CHECKSUM-PINS.json'):
        p = Path(old['vendorDirectory']) / row['path']
        assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)
    assert sha(Path(plan['environment']['CARGO_HOME']) / 'config.toml') == old['cargoConfigSha256']
    return {'sourceAndExpectedBodies': len(pins), 'checksumManifestRows': len(load(OLD / 'VENDOR-CHECKSUM-PINS.json')),
            'vendorLimit': 'Whole checksum manifests/map only; consumed bodies validated by Cargo. No independent all25414-body rehash claim.'}

def identity():
    p = subprocess.run(['/usr/bin/git','rev-parse','HEAD','HEAD^{tree}'], cwd=W,
        env=old['metadataEnvironment'], stdin=subprocess.DEVNULL, capture_output=True, timeout=2)
    assert p.returncode == 0
    return p.stdout.decode().splitlines()

def dump(name, obj):
    (O / name).write_text(json.dumps(obj, indent=2) + '\n')

before = verify()
before_identity = identity()
assert before_identity == ['0ad8a7e7e9cbd63b6e5d002139de26535abf3025','63226695eea932993bbf3f9569cca65899f08a92']
if sys.argv[1:] == ['--preflight']:
    print(json.dumps({'preflight': before, 'gitIdentity': before_identity, 'productExecuted': False}, indent=2))
    raise SystemExit(0)
assert sys.argv[1:] == []
O.mkdir(exist_ok=False)
owned = Path(tempfile.mkdtemp(prefix='doctorwatson-core3-legacy-compat-'))
environment = dict(plan['environment'])
environment.update(TMPDIR=str(owned), MORI_BOOLEAN_CHECK_OBSERVATIONS=str(O/'boolean'),
                   MORI_DUTY_READ_CHECK_OBSERVATIONS=str(O/'duty'))
result = {'startedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'sourceBefore': before,
          'gitIdentityBefore': before_identity, 'driverSha256': sha(__file__), 'argv': plan['argv'],
          'environment': environment, 'independentExpectationPlan': sha(P/'PROPOSED-COMMAND.json'),
          'qualification': 'Six existing ordinary tests; Core1 independent post-adoption bytes; Core2 independent values under disclosed source premises, aggregate byte formatting source-authored; Boolean/duty source-author corroboration. No product/security/target acceptance.',
          'productAttemptConsumed': False, 'passed': False, 'automaticRetry': False}
dump('START.json', result)
child = None
started = time.monotonic()
try:
    with (O/'stdout').open('wb') as out, (O/'stderr').open('wb') as err:
        child = subprocess.Popen(plan['argv'], cwd=W, env=environment, stdin=subprocess.DEVNULL,
            stdout=out, stderr=err, start_new_session=True)
        result['productAttemptConsumed'] = True
        dump('CHILD.json', {'pid': child.pid, 'ownedSession': child.pid, 'argv': plan['argv']})
        try:
            child.wait(timeout=95)
            result['parentDeadlineExceeded'] = False
        except subprocess.TimeoutExpired:
            result['parentDeadlineExceeded'] = True
            try: os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError: pass
            child.wait(timeout=2)
    result.update(exit=child.returncode, wallSeconds=time.monotonic()-started,
                  stdoutSha256=sha(O/'stdout'), stderrSha256=sha(O/'stderr'))
    dump('TERMINAL.json', result)
    result['sourceAfter'] = verify()
    result['gitIdentityAfter'] = identity()
    assert result['gitIdentityAfter'] == before_identity
    output = (O/'stdout').read_text()
    errors = (O/'stderr').read_text()
    result['passedTestNames'] = re.findall(r'^test (\S+) \.\.\. ok$', output, re.MULTILINE)
    result['testSummaries'] = re.findall(r'test result: (?:ok|FAILED)\..*', output)
    result['selectedTargets'] = re.findall(r'Running tests/(\w+)\.rs', errors)
    expected = [name for names in plan['expectedTargetsAndNames'].values() for name in names]
    result['publicBinarySha256'] = sha(Path(environment['CARGO_TARGET_DIR'])/'debug/mori')
    result['passed'] = (result['exit'] == 0 and not result['parentDeadlineExceeded']
        and sorted(result['passedTestNames']) == sorted(expected)
        and sorted(result['selectedTargets']) == sorted(plan['expectedTargetsAndNames'])
        and len(result['testSummaries']) == 4
        and all('0 failed; 0 ignored; 0 measured; 0 filtered out' in s for s in result['testSummaries']))
except BaseException as error:
    result['exception'] = {'type': type(error).__name__, 'message': str(error)}
finally:
    if child is not None:
        census = subprocess.run(['/usr/bin/ps','--sid',str(child.pid),'-o','pid=,sid=,stat='],
            capture_output=True, text=True, timeout=2)
        result['ownedSessionRemaining'] = census.stdout.splitlines()
    result['ownedTemporaryRemaining'] = [str(p.relative_to(owned)) for p in owned.rglob('*')]
    if not result.get('ownedSessionRemaining') and not result['ownedTemporaryRemaining']:
        owned.rmdir(); result['ownedTemporaryRemoved'] = True
    else: result['ownedTemporaryRetained'] = str(owned)
    result['endedAt'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    dump('RESULT.json', result)
print(json.dumps(result, indent=2))
assert result['passed'] and not result.get('ownedSessionRemaining')
