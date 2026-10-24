import hashlib
import json
from pathlib import Path
import subprocess
import time

R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
OUT = Path(__file__).resolve().parent
HANDOFF = json.loads((R / '82-ears-host-scenario-tests/HANDOFF.json').read_text())
PROVIDER = json.loads((R / '87-ears-host-scenario-review/PROVIDER-RESULT.json').read_text())
REVIEW = json.loads(PROVIDER['response'])
OLD_PINS = json.loads((R / '86-compiler-fixture-review/FROZEN-EXECUTION-INPUT-PINS.json').read_text())
CWD = OUT / 'run'
ENV = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'TMPDIR': str(CWD / 'tmp')}
RUSTC = '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc'
DEPS = R / '24-compiler-qa/proposed-run/core-target/debug/deps'
SOURCE = Path(HANDOFF['test_source']['path'])
ARTIFACT = OLD_PINS['existing_R83_production_rlib']


def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n')


def pins():
    entries = OLD_PINS['existing_public_production_sources_deps_tools'] + [ARTIFACT, HANDOFF['test_source'], HANDOFF['oracle']]
    for entry in entries:
        raw = Path(entry['path']).read_bytes()
        assert len(raw) == entry['bytes'], entry['path']
        assert hashlib.sha256(raw).hexdigest() == entry['sha256'], entry['path']
    return {'checked': len(entries), 'unchanged': True}


def phase(name, argv):
    before = pins()
    started = time.monotonic()
    child = subprocess.run(argv, cwd=CWD, env=ENV, capture_output=True)
    elapsed = time.monotonic() - started
    (OUT / (name + '.stdout')).write_bytes(child.stdout)
    (OUT / (name + '.stderr')).write_bytes(child.stderr)
    result = dict(argv=argv, cwd=str(CWD), environment=ENV, exit_code=child.returncode,
                  elapsed_seconds=elapsed, pre=before,
                  stdout_sha256=hashlib.sha256(child.stdout).hexdigest(),
                  stderr_sha256=hashlib.sha256(child.stderr).hexdigest())
    save(name + '.json', result)
    result['post'] = pins()
    save(name + '.json', result)
    assert child.returncode == 0, (name, child.returncode)
    return child.stdout.decode('utf-8')


def main():
    with (OUT / 'ATTEMPT.json').open('x') as stream:
        json.dump({'native_authority': 0, 'no_retry': True}, stream)
    assert REVIEW['ordinary_source_decision']['verdict'] == 'APPROVE_SCOPED_SOURCE_ONLY'
    assert REVIEW['exact_runtime_eligibility']['verdict'] == 'ELIGIBLE_ROOT_EXTERNAL_RUSTC_HOST_DIAGNOSTIC_ONLY'
    selectors = REVIEW['exact_runtime_eligibility']['permitted_exact_selectors']
    assert selectors == HANDOFF['exact_positive_test_functions']
    CWD.mkdir(exist_ok=False)
    (CWD / 'tmp').mkdir()
    save('ROOT-SCOPED-ADOPTION.json', {'scope': 'exact nine fixed positive A host diagnostic tests only',
         'review_source': str(R / '87-ears-host-scenario-review/PROVIDER-RESULT.json'),
         'conditions': REVIEW['exact_runtime_eligibility']['required_limits'],
         'runtime_limits': 'outer 480s; compile30s/list10s/each exact test10s; strict fail-stop',
         'native_authority': 0, 'H1': 'OPEN', 'general_EARS_acceptance': 'not-inferred'})
    binary = str(CWD / 'tests')
    phase('01-compile', ['/usr/bin/timeout', '--signal=KILL', '30s', RUSTC, '--edition=2024', '--test',
          str(SOURCE), '--extern', 'mori_core=' + ARTIFACT['path'], '-L', 'dependency=' + str(DEPS),
          '--emit=link,dep-info=' + str(CWD / 'tests.d'), '-o', binary, '-C', 'debuginfo=0'])
    paths = (CWD / 'tests.d').read_text().splitlines()[0].split(': ', 1)[1].split()
    normalized = [str((CWD / path).resolve()) for path in paths]
    save('TEST-DEPINFO.json', {'raw_paths': paths, 'normalized_paths': normalized})
    assert normalized == [str(SOURCE.resolve())]
    output = phase('02-list', ['/usr/bin/timeout', '--signal=KILL', '10s', binary, '--list'])
    listed = [line.removesuffix(': test') for line in output.splitlines() if line.endswith(': test')]
    assert listed == sorted(selectors)
    passed = []
    for index, selector in enumerate(selectors, 1):
        output = phase('test-' + str(index), ['/usr/bin/timeout', '--signal=KILL', '10s', binary,
                       '--exact', selector, '--nocapture', '--test-threads=1'])
        assert '1 passed; 0 failed; 0 ignored;' in output
        passed.append(selector)
    save('RESULT.json', {'status': 'pass', 'passed': passed, 'pins': pins(), 'native_authority': 0,
         'scope': 'nine fixed typed-A normal scenario observations; general clauses not closed by test count'})


if __name__ == '__main__':
    try:
        main()
    except BaseException as error:
        save('STOP.json', {'error_type': type(error).__name__, 'error': str(error), 'no_retry': True})
        raise
