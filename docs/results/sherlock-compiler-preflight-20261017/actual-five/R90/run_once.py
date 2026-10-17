import hashlib
import json
from pathlib import Path
import subprocess
import time

R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
INPUT = R / '86-compiler-fixture-review'
OUT = Path(__file__).resolve().parent
PLAN = json.loads((INPUT / 'PROSPECTIVE-REMAINING-TESTS-PLAN.json').read_text())
PINS = json.loads((INPUT / 'FROZEN-EXECUTION-INPUT-PINS.json').read_text())
REVIEW = json.loads((INPUT / 'FINAL-RESPONSE.txt').read_text())


def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n')


def pins():
    entries = PINS['existing_public_production_sources_deps_tools'] + [PINS['new_test'], PINS['existing_R83_production_rlib']]
    for entry in entries:
        raw = Path(entry['path']).read_bytes()
        assert len(raw) == entry['bytes'], entry['path']
        assert hashlib.sha256(raw).hexdigest() == entry['sha256'], entry['path']
    return {'checked': len(entries), 'unchanged': True}


def phase(name, argv):
    before = pins()
    started = time.monotonic()
    child = subprocess.run(argv, cwd=PLAN['cwd'], env=PLAN['environment'], capture_output=True)
    elapsed = time.monotonic() - started
    (OUT / (name + '.stdout')).write_bytes(child.stdout)
    (OUT / (name + '.stderr')).write_bytes(child.stderr)
    result = dict(argv=argv, cwd=PLAN['cwd'], environment=PLAN['environment'], exit_code=child.returncode,
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
        json.dump({'no_retry': True, 'native_authority': 0}, stream)
    assert REVIEW['remaining_three_harness_classification']['status'] == 'ELIGIBLE_FOR_DIRECT_RUSTC_REMAINING_THREE_HARNESS_UNDER_EXACT_CONDITIONS'
    cwd = Path(PLAN['cwd'])
    assert not (cwd / 'preflight-tests').exists() and not (cwd / 'tests.d').exists()
    save('ROOT-SCOPED-ADOPTION.json', {'review': str(INPUT / 'FINAL-RESPONSE.txt'),
         'scope': 'changed test3 plus previously unrun tests4/5 only; existing R83 production artifact reused',
         'native_authority': 0, 'conditions': REVIEW['remaining_three_harness_classification']['exact_conditions']})
    commands = PLAN['commands_serial']
    phase('01-test-compile', commands['changed_fixed_five_file_compile'])
    raw_paths = (cwd / 'tests.d').read_text().splitlines()[0].split(': ', 1)[1].split()
    actual = [str((cwd / path).resolve()) for path in raw_paths]
    save('TEST-DEPINFO.json', {'raw_paths': raw_paths, 'normalized_paths': actual})
    assert actual == [str(Path(PINS['new_test']['path']).resolve())]
    listed = phase('02-list', commands['list_only'])
    selectors = [line.removesuffix(': test') for line in listed.splitlines() if line.endswith(': test')]
    assert selectors == sorted(PLAN['list_must_equal_exact_five_selectors'])
    tests = []
    for index, argv in enumerate(commands['remaining_three_exact_tests'], 3):
        selector = argv[argv.index('--exact') + 1]
        assert selector not in PLAN['do_not_rerun_first_two']
        output = phase('test-' + str(index), argv)
        assert '1 passed; 0 failed; 0 ignored;' in output
        tests.append({'selector': selector, 'status': 'pass'})
    save('RESULT.json', {'status': 'pass', 'tests': tests, 'pins': pins(), 'native_authority': 0,
         'historical_first_two': 'R83 passed; unchanged tests not rerun',
         'historical_failed_third': 'R83 lexer fixture failure retained; changed test here',
         'scope': 'synthetic host preparation only; no CLI/observe/runtime/SDK/native'})


if __name__ == '__main__':
    try:
        main()
    except BaseException as error:
        save('STOP.json', {'error_type': type(error).__name__, 'error': str(error), 'no_retry': True})
        raise
