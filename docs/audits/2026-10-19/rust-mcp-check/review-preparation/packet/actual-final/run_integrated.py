"""One fixed ordinary MCP integration check; no retry or native operation."""
from pathlib import Path
import datetime, hashlib, json, os, re, signal, subprocess, time

D = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
write = lambda name, value: (D / name).write_text(json.dumps(value, indent=2) + '\n')
plan = json.loads((D / 'CHECK-PLAN.json').read_bytes())
assert sha(__file__) == plan['driverSha256']
assert sha(D / 'SOURCE-PINS.json') == plan['sourcePinsSha256']
checkout = Path(plan['checkout'])
pins = json.loads((D / 'SOURCE-PINS.json').read_bytes())

def verify():
    for row in pins:
        path = checkout / row['path']
        assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256'], row['path']
    for row in plan['tools']:
        assert sha(row['path']) == row['sha256'], row['path']
    assert sha(Path(plan['environment']['CARGO_HOME']) / 'config.toml') == plan['cargoConfigSha256']

verify()
assert not (D / 'START.json').exists()
write('START.json', {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
                    'argv': plan['argv'], 'planSha256': sha(D / 'CHECK-PLAN.json'),
                    'sourcePinsBefore': len(pins), 'attemptConsumed': True})
started = time.monotonic()
expired = False
with (D / 'STDOUT.log').open('wb') as out, (D / 'STDERR.log').open('wb') as err:
    child = subprocess.Popen(plan['argv'], cwd=plan['cwd'], env=plan['environment'],
                             stdin=subprocess.DEVNULL, stdout=out, stderr=err,
                             start_new_session=True)
    write('CHILD.json', {'pid': child.pid, 'ownedSession': child.pid})
    try:
        child.wait(timeout=95)
    except subprocess.TimeoutExpired:
        expired = True
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        child.poll()
result = {'endedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'exit': child.returncode, 'wallSeconds': time.monotonic() - started,
          'parentDeadlineExceeded': expired, 'automaticRetry': False,
          'sourcePinsBefore': len(pins), 'attemptConsumed': True}
write('RESULT.json', result)
census = subprocess.run(['/usr/bin/ps', '--sid', str(child.pid), '-o', 'pid=,sid=,stat='],
                        capture_output=True, text=True, timeout=1)
assert census.returncode in (0, 1)
result['ownedSessionRemaining'] = census.stdout.splitlines()
verify()
result['sourcePinsAfter'] = len(pins)
text = (D / 'STDOUT.log').read_text()
result['testSummaries'] = re.findall(r'test result: (?:ok|FAILED)\..*', text)
names = re.findall(r'^test (\S+) \.\.\. ok$', text, re.MULTILINE)
result['selectedSixPassed'] = (child.returncode == 0 and not expired
    and not result['ownedSessionRemaining'] and sorted(names) == sorted(plan['selectedTestNames'])
    and len(result['testSummaries']) == 2
    and all('0 failed; 0 ignored; 0 measured; 0 filtered out' in s for s in result['testSummaries']))
for name in ['STDOUT.log', 'STDERR.log']:
    result[name + 'Sha256'] = sha(D / name)
result['binarySha256'] = sha(Path(plan['environment']['CARGO_TARGET_DIR']) / 'debug/mori')
write('RESULT.json', result)
print(json.dumps(result, indent=2))
print(text)
print((D / 'STDERR.log').read_text())
assert result['selectedSixPassed']
