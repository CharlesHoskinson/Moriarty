"""One root-selected separate U0 successor source-check observation; reused finite capture, no automatic admission or general runner."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time

D = Path('/home/charl/moriarty-c2-opus55-restored-20261017/u0-public-core-lexer-offset-baseline-six-test-runtime-preparation-20261022')
C = Path('/home/charl/moriarty-c2-opus55-restored-20261017/u0-public-core-lexer-offset-baseline-harness-build-preparation-20261022/test-source')
CTL = Path('/home/charl/moriarty-c2-opus55-restored-20261017/u0-public-core-lexer-offset-baseline-six-test-runtime-root-control-20261022-a1')
SELECTION = CTL / 'ROOT-RUNTIME-SELECTION.json'
selection = json.loads(SELECTION.read_text())
assert selection['status'] == 'SELECTED_EXACT_PUBLIC_CORE_LEXICAL_OFFSET_BASELINE_SIX_TESTS_UNFILTERED_ONCE'
assert selection['captureSha256'] == hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
plan_path = D / 'RUNTIME-PLAN.json'
assert selection['planSha256'] == hashlib.sha256(plan_path.read_bytes()).hexdigest()
plan = json.loads(plan_path.read_text())
A = Path(plan['outputMustNotExist'])
assert A == D.parent / 'u0-public-core-lexer-offset-baseline-six-test-runtime-actual-20261022-a1'
assert not A.exists()
assert plan['aggregateWorkSeconds'] == 10 and plan['singleTermKillGraceSeconds'] == 5 and plan['maxAttempts'] == 1
assert [p['name'] for p in plan['phases']] == ['lexical_error_offsets']
assert plan['testCount'] == 6

pins = {}
def add_pin(item):
    path = str(Path(item['path']).absolute())
    assert not any(name in path for name in ['provider.private', 'stderr.private', 'agy.private.log'])
    value = {'path': path, 'bytes': item['bytes'], 'sha256': item['sha256']}
    if path in pins:
        assert pins[path] == value
    pins[path] = value

def collect_rows(value):
    if isinstance(value, dict):
        if {'path', 'bytes', 'sha256'} <= value.keys():
            add_pin(value)
        for child in value.values():
            collect_rows(child)
    elif isinstance(value, list):
        for child in value:
            collect_rows(child)

# Full old build pins and frozen expectations plus exact linked outputs/loader identities.
for row in json.loads((D / 'RUNTIME-INPUT-PINS.json').read_text())['files']:
    add_pin(row)
for row in json.loads((D / 'FREEZE.json').read_text())['files']:
    add_pin({'path': str(D / row['path']), 'bytes': row['bytes'], 'sha256': row['sha256']})
add_pin({'path': str(SELECTION), 'bytes': SELECTION.stat().st_size, 'sha256': hashlib.sha256(SELECTION.read_bytes()).hexdigest()})
add_pin({'path': str(Path(__file__).absolute()), 'bytes': Path(__file__).stat().st_size, 'sha256': selection['captureSha256']})

def changed_pins():
    from concurrent.futures import ThreadPoolExecutor

    def check(row):
        p = Path(row['path'])
        try:
            b = p.read_bytes()
            if len(b) != row['bytes'] or hashlib.sha256(b).hexdigest() != row['sha256']:
                return row['path']
        except OSError:
            return row['path']
        return None

    with ThreadPoolExecutor(max_workers=4) as executor:
        return [path for path in executor.map(check, pins.values()) if path is not None]

assert changed_pins() == []
# Both exact argvs/env/cwd and names remain from the frozen build-time runtime prospect.
original = json.loads(Path(plan['originalCommandsPath']).read_text())
assert hashlib.sha256(Path(plan['originalCommandsPath']).read_bytes()).hexdigest() == plan['originalCommandsSha256']
assert len(plan['phases']) == len(original['phases']) == 1
assert selection['rootWholeBuildAndSourceEffectsInspected'] is True
build = json.loads(Path(plan['actualBuild']['path']).read_text())
assert len(build['phases']) == 1 and not build['changedPins'] and not build['remainingOwned'] and not build['aggregateDeadlineExpired']
for phase, expected in zip(plan['phases'], original['phases']):
    compiled = next(row for row in build['phases'] if row['phase'] == phase['name'])
    assert phase['argv'] == expected['argv'] and phase['argv'][1:] == ['--test-threads=1', '--nocapture']
    assert phase['cwd'] == expected['cwd'] == str(A)
    assert phase['environmentExactlyReplaceNotOverlay'] == expected['environmentExactlyReplaceNotOverlay']
    assert phase['environmentExactlyReplaceNotOverlay']['TMPDIR'] == str(A / 'tmp')
    assert phase['expectedNames'] == expected['expectedTestNames'] and phase['expectedPassIfSuccess'] == len(phase['expectedNames'])
    assert compiled['phase'] == phase['name'] and compiled['exit'] == 0 and compiled['outputExists']
    assert compiled['outputBytes'] == phase['binary']['bytes'] and compiled['outputSha256'] == phase['binary']['sha256']
    assert phase['argv'][0] == phase['binary']['path']
identity = json.loads((D / 'RUNTIME-INPUT-PINS.json').read_text())
for row in identity['initialAbsences']:
    assert not Path(row['path']).exists() and not Path(row['path']).is_symlink()
owned_output_identity = None
def identity_drift():
    bad = changed_pins()
    for row in identity['outputModePins']:
        if oct(Path(row['path']).stat().st_mode & 0o777) != row['mode']:
            bad.append(row['path'] + ':mode')
    for row in identity['systemPathIdentityPins']:
        p = Path(row['path'])
        if str(p.resolve()) != row['resolvedPath'] or (str(p.readlink()) if p.is_symlink() else None) != row['linkText']:
            bad.append(row['path'] + ':link')
    for row in identity['absences']:
        if Path(row['path']).exists() or Path(row['path']).is_symlink():
            bad.append(row['path'] + ':absence')
    if owned_output_identity is not None:
        try:
            st = A.lstat()
            if A.is_symlink() or not A.is_dir() or (st.st_dev, st.st_ino, st.st_uid, st.st_mode & 0o777) != owned_output_identity or st.st_uid != os.getuid() or st.st_mode & 0o777 != 0o700:
                bad.append(str(A) + ':owned-output-identity')
        except OSError:
            bad.append(str(A) + ':owned-output-identity')
    return bad
assert identity_drift() == []

A.mkdir(mode=0o700)
owned_output_stat = A.stat()
owned_output_identity = (owned_output_stat.st_dev, owned_output_stat.st_ino, owned_output_stat.st_uid, owned_output_stat.st_mode & 0o777)
(A / 'tmp').mkdir()
(A / 'SELECTED-PLAN.json').write_bytes(plan_path.read_bytes())
(A / 'INPUT-PINS.json').write_text(json.dumps(list(pins.values()), indent=2) + '\n')
started_at = datetime.datetime.now(datetime.timezone.utc).isoformat()
t0 = time.monotonic()
deadline = t0 + 10
results = []
remaining_owned = []
deadline_expired = False

def census(sid):
    found = []
    for path in Path('/proc').iterdir():
        if not path.name.isdigit():
            continue
        try:
            text = (path / 'stat').read_text()
            parts = text[text.rfind(')') + 2:].split()
            if int(parts[2]) == sid or int(parts[3]) == sid:
                found.append({'pid': int(path.name), 'pgrp': int(parts[2]), 'sid': int(parts[3]), 'state': parts[0]})
        except (OSError, ValueError, IndexError):
            pass
    return found

for index, phase in enumerate(plan['phases']):
    if identity_drift():
        break
    if time.monotonic() >= deadline:
        deadline_expired = True
        break
    phase_start = time.monotonic()
    with (A / (phase['name'] + '.stdout.txt')).open('xb') as stdout, (A / (phase['name'] + '.stderr.jsonl')).open('xb') as stderr:
        child = subprocess.Popen(phase['argv'], cwd=phase['cwd'], env=phase['environmentExactlyReplaceNotOverlay'], stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr, start_new_session=True, close_fds=True)
        (A / (phase['name'] + '.CHILD.json')).write_text(json.dumps({'pid': child.pid, 'ownedSID': child.pid, 'phase': phase['name']}) + '\n')
        try:
            code = child.wait(timeout=max(0.001, deadline - time.monotonic()))
        except subprocess.TimeoutExpired:
            deadline_expired = True
            os.killpg(child.pid, signal.SIGTERM)
            try:
                code = child.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                code = child.wait()
    left = census(child.pid)
    if left:
        os.killpg(child.pid, signal.SIGKILL)
        left = census(child.pid)
    remaining_owned.extend(left)
    row = {'phase': phase['name'], 'exit': code, 'elapsedSeconds': time.monotonic() - phase_start, 'ownedSID': child.pid, 'ownedRemaining': left, 'reaped': True, 'captureFileDescriptorsClosed': True}
    for kind, suffix in [('stdout', '.stdout.txt'), ('stderr', '.stderr.jsonl')]:
        with (A / (phase['name'] + suffix)).open('rb') as stream:
            data = stream.read()
            row[kind + 'RegularFileReadEOF'] = stream.read(1) == b''
        row[kind + 'Bytes'] = len(data)
        row[kind + 'Sha256'] = hashlib.sha256(data).hexdigest()
    # Libtest names/counts qualify the frozen normal protocol tests, not general source/financial correctness.
    import re
    text = (A / (phase['name'] + '.stdout.txt')).read_text(errors='replace')
    summary = re.search(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;', text, re.M)
    names = re.findall(r'^test (.+?) \.\.\. ok$', text, re.M)
    counts = None if summary is None else {'status': summary.group(1), 'passed': int(summary.group(2)), 'failed': int(summary.group(3)), 'ignored': int(summary.group(4)), 'measured': int(summary.group(5)), 'filtered': int(summary.group(6))}
    row['observedCounts'] = counts
    row['observedPassedNames'] = names
    row['selectedAssertionsSatisfied'] = (counts is not None and counts['status'] == 'ok' and counts['passed'] == phase['expectedPassIfSuccess'] and counts['failed'] == 0 and counts['ignored'] == 0 and counts['measured'] == 0 and counts['filtered'] == phase['expectedFiltered'] and row['stdoutRegularFileReadEOF'] and row['stderrRegularFileReadEOF'] and sorted(names) == sorted(phase['expectedNames']) and row['stderrBytes'] == 0)
    row['pipeEOFObserved'] = False
    row['streamCompletionBasis'] = 'Inherited regular file capture: parentFDclosed, directchildreaped, ownedcensus. Not a measured pipeEOF or globaldescendant guarantee.'
    results.append(row)
    (A / (phase['name'] + '.TERMINAL.json')).write_text(json.dumps(row, indent=2) + '\n')
    if left or deadline_expired or identity_drift():
        break

work_elapsed = time.monotonic() - t0
(A / 'TERMINAL.json').write_text(json.dumps({'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'phases': results, 'aggregateWorkElapsedSeconds': work_elapsed, 'aggregateDeadlineExpired': deadline_expired, 'remainingOwned': remaining_owned, 'diagnosticRuntimeOnly': True, 'SDKLoadCallsObservedByCapture': False, 'sourceProspectiveNormalCliLaunches': 0}, indent=2) + '\n')
changed = identity_drift()
status = 'PROTOCOL_RUNTIME_OBSERVATION_COMPLETE_REQUIRES_ROOT_CLASSIFICATION'
if changed or remaining_owned or deadline_expired:
    status = 'PROTOCOL_RUNTIME_OBSERVATION_UNQUALIFIED'
record = {'status': status, 'startedAt': started_at, 'endedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'phases': results, 'aggregateWorkElapsedSeconds': work_elapsed, 'aggregateDeadlineExpired': deadline_expired, 'pins': len(pins), 'changedPins': changed, 'remainingOwned': remaining_owned, 'completedSelectedPasses': sum((r.get('observedCounts') or {}).get('passed', 0) for r in results), 'SDKFunctionCallsMeasured': False, 'sourceProspectiveIrSourceLoads': 0, 'sourceProspectiveNormalCliLaunches': 0, 'nativeProofOrLedgerAcceptance': False, 'allSelectedHarnessCompleteWithExactAssertions': len(results) == 1 and all(r['selectedAssertionsSatisfied'] for r in results), 'unperformedTestsNotFabricated': True, 'rootMustClassifyEveryDiagnostic': True}
# Snapshot all owned tmp entries; preserve nonempty/unknown leftovers for root inspection.
import shutil
leftovers = []
for p in sorted((A / 'tmp').rglob('*')):
    if p.is_symlink():
        leftovers.append({'path': str(p.relative_to(A)), 'kind': 'symlink', 'linkText': str(p.readlink())})
    elif p.is_dir():
        leftovers.append({'path': str(p.relative_to(A)), 'kind': 'directory'})
    elif p.is_file():
        data = p.read_bytes()
        leftovers.append({'path': str(p.relative_to(A)), 'kind': 'regular', 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()})
record['ownedTempBeforeCleanup'] = leftovers
record['ownedTempCleanupSelected'] = not remaining_owned and not leftovers
if not remaining_owned and not leftovers:
    shutil.rmtree(A / 'tmp')
record['ownedTempExistsAfter'] = (A / 'tmp').exists()
record['cleanupQualification'] = 'Only empty exact new output/tmp removed after empty owned census. Any nonempty/unknown tmp entries retained with full inventory. Source/cache/build roots and public streams retained; later root remediation cannot retroactively change receipt.'
if not record['allSelectedHarnessCompleteWithExactAssertions'] or leftovers:
    record['status'] = 'PROTOCOL_RUNTIME_OBSERVATION_UNQUALIFIED'
(A / 'ROOT-RESULT.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record, indent=2))
