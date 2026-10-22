"""One root-selected current-source compile-only observation, never a runtime runner."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time

D = Path('/home/charl/moriarty-c2-opus55-restored-20261017/u0-public-core-lexer-offset-green-core-and-harness-build-preparation-20261022')
C = Path('/home/charl/moriarty-c2-opus55-restored-20261017/u0-public-core-lexer-offset-green-core-and-harness-build-preparation-20261022/source-green')
CTL = Path('/home/charl/moriarty-c2-opus55-restored-20261017/u0-public-core-lexer-offset-green-core-and-harness-build-root-control-20261022-a1')
SELECTION = CTL / 'ROOT-HARNESS-BUILD-SELECTION.json'
selection = json.loads(SELECTION.read_text())
assert selection['status'] == 'SELECTED_EXACT_PUBLIC_CORE_LEX_OFFSETS_GREEN_CORE_AND_HARNESS_ONCE'
assert selection['captureSha256'] == hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
plan_path = D / 'PROPOSED-TWO-RUSTC.json'
assert selection['planSha256'] == hashlib.sha256(plan_path.read_bytes()).hexdigest()
plan = json.loads(plan_path.read_text())
A = Path(plan['outputMustNotExist'])
assert A == D.parent / 'u0-public-core-lexer-offset-green-core-and-harness-build-only-actual-20261022-a1'
assert not A.exists()
assert plan['aggregateWorkSeconds'] == 90 and plan['singleTermKillGraceSeconds'] == 5 and plan['maxAttempts'] == 1
assert [p['name'] for p in plan['phases']] == ['mori_core', 'lexical_error_offsets']
assert plan['sourceRoot'] == str(C)

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

for filename in ['CURRENT-SOURCE-IDENTITIES.json', 'DEPENDENCY-COHORT.json', 'TOOL-AND-SYSROOT-PINS.json', 'PUBLIC-PROVENANCE-PINS.json', 'KNOWN-HOST-EFFECTS.json']:
    p = D / filename
    b = p.read_bytes()
    add_pin({'path': str(p), 'bytes': len(b), 'sha256': hashlib.sha256(b).hexdigest()})
    collect_rows(json.loads(b))
for row in json.loads((D / 'FREEZE.json').read_text())['files']:
    add_pin({'path': str(D / row['path']), 'bytes': row['bytes'], 'sha256': row['sha256']})
add_pin({'path': str(SELECTION), 'bytes': SELECTION.stat().st_size, 'sha256': hashlib.sha256(SELECTION.read_bytes()).hexdigest()})
add_pin({'path': str(Path(__file__).absolute()), 'bytes': Path(__file__).stat().st_size, 'sha256': selection['captureSha256']})

def changed_pins():
    changes = []
    for row in pins.values():
        p = Path(row['path'])
        try:
            b = p.read_bytes()
            if len(b) != row['bytes'] or hashlib.sha256(b).hexdigest() != row['sha256']:
                changes.append(row['path'])
        except OSError:
            changes.append(row['path'])
    return changes

assert changed_pins() == []
for phase in plan['phases']:
    argv = phase['argv']
    assert argv[0] == '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc'
    assert argv.count('--emit') == 1 and argv[argv.index('--emit') + 1] == 'link=' + phase['output']
    assert Path(phase['output']).parent == A and phase['cwd'] == str(A)
    assert '--emit=link' not in argv and '--crate-type=proc-macro' not in argv
    if phase['name'] == 'mori_core':
        assert '--test' not in argv
        assert argv.count('--crate-type') == 1 and argv[argv.index('--crate-type') + 1] == 'lib'
        assert argv.count('--extern') == 6
    else:
        assert argv.count('--test') == 1 and '--crate-type' not in argv
        assert argv.count('--extern') == 2
    assert len([value for value in argv if value.startswith('native=')]) == 0
    cg = [argv[i + 1] for i, token in enumerate(argv) if token == '-C']
    expected_cg = {'opt-level=0', 'debuginfo=0', 'debug-assertions=yes', 'overflow-checks=yes', 'embed-bitcode=no', 'panic=unwind', 'metadata=public_core_lex_offsets_green_' + phase['name'] + '_a1', 'linker=/usr/bin/cc'}
    assert len(cg) == len(expected_cg) and set(cg) == expected_cg
    assert not any(option.startswith('link=') for option in cg)
    assert phase['environmentExactlyReplaceNotOverlay']['TMPDIR'] == str(A / 'tmp')

A.mkdir()
(A / 'tmp').mkdir()
(A / 'SELECTED-PLAN.json').write_bytes(plan_path.read_bytes())
(A / 'INPUT-PINS.json').write_text(json.dumps(list(pins.values()), indent=2) + '\n')
started_at = datetime.datetime.now(datetime.timezone.utc).isoformat()
t0 = time.monotonic()
deadline = t0 + 90
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
    if changed_pins():
        break
    for previous, completed in zip(plan['phases'], results):
        p = Path(previous['output'])
        assert p.is_file() and hashlib.sha256(p.read_bytes()).hexdigest() == completed['outputSha256']
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
        data = (A / (phase['name'] + suffix)).read_bytes()
        row[kind + 'Bytes'] = len(data)
        row[kind + 'Sha256'] = hashlib.sha256(data).hexdigest()
    output = Path(phase['output'])
    row['outputExists'] = output.is_file()
    if output.is_file():
        data = output.read_bytes()
        row['outputBytes'] = len(data)
        row['outputSha256'] = hashlib.sha256(data).hexdigest()
    results.append(row)
    (A / (phase['name'] + '.TERMINAL.json')).write_text(json.dumps(row, indent=2) + '\n')
    if code != 0 or left or deadline_expired or not row['outputExists']:
        break

work_elapsed = time.monotonic() - t0
(A / 'TERMINAL.json').write_text(json.dumps({'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'phases': results, 'aggregateWorkElapsedSeconds': work_elapsed, 'aggregateDeadlineExpired': deadline_expired, 'remainingOwned': remaining_owned, 'noRuntimeOrSDKFunctionsSelected': True}, indent=2) + '\n')
changed = changed_pins()
status = 'COMPILE_OBSERVATION_COMPLETE_REQUIRES_DIAGNOSTIC_CLASSIFICATION'
if changed or remaining_owned or deadline_expired:
    status = 'COMPILE_OBSERVATION_UNQUALIFIED'
record = {'status': status, 'startedAt': started_at, 'endedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'phases': results, 'aggregateWorkElapsedSeconds': work_elapsed, 'aggregateDeadlineExpired': deadline_expired, 'pins': len(pins), 'changedPins': changed, 'remainingOwned': remaining_owned, 'runtimeTestsExecuted': 0, 'SDKFunctionsExecuted': 0, 'nativeProofOrLedgerAcceptance': False, 'rootMustClassifyEveryDiagnostic': True}
(A / 'ROOT-RESULT.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record, indent=2))
