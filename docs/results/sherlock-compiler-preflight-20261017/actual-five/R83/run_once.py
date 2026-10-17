import hashlib
import json
from pathlib import Path
import subprocess
import time

BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
INPUT = BASE / '74-compiler-correction-review-packet/harness-clarification-02'
OUT = Path(__file__).resolve().parent
PLAN = json.loads((INPUT / 'PROSPECTIVE-EXACT-HARNESS.json').read_text())
PINS = json.loads((INPUT / 'FROZEN-SOURCE-DEP-TOOL-PINS.json').read_text())
REVIEW = json.loads((INPUT / 'FINAL-RESPONSE.txt').read_text())


def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n')


def check_pins():
    entries = PINS['sources'] + PINS['existing_public_ordinary_dependencies_tools']
    for entry in entries:
        raw = Path(entry['path']).read_bytes()
        assert len(raw) == entry['bytes'], entry['path']
        assert hashlib.sha256(raw).hexdigest() == entry['sha256'], entry['path']
    return {'checked': len(entries), 'unchanged': True}


def phase(name, argv):
    before = check_pins()
    start = time.monotonic()
    child = subprocess.run(argv, cwd=PLAN['cwd'], env=PLAN['environment'], capture_output=True)
    elapsed = time.monotonic() - start
    (OUT / (name + '.stdout')).write_bytes(child.stdout)
    (OUT / (name + '.stderr')).write_bytes(child.stderr)
    result = dict(argv=argv, cwd=PLAN['cwd'], environment=PLAN['environment'],
                  exit_code=child.returncode, elapsed_seconds=elapsed,
                  stdout_sha256=hashlib.sha256(child.stdout).hexdigest(),
                  stderr_sha256=hashlib.sha256(child.stderr).hexdigest(), pre=before)
    # Save the actual child result before later collector assertions.
    save(name + '.json', result)
    result['post'] = check_pins()
    save(name + '.json', result)
    assert child.returncode == 0, (name, child.returncode)
    return child.stdout.decode('utf-8')


def depinfo(name, filename):
    first = (Path(PLAN['cwd']) / filename).read_text().splitlines()[0]
    paths = first.split(': ', 1)[1].split()
    normalized = [str((Path(PLAN['cwd']) / p).resolve()) for p in paths]
    allowed = {str(Path(p['path']).resolve()) for p in PINS['sources']}
    save(name + '-depinfo.json', {'raw_paths': paths, 'normalized_paths': normalized,
                                 'outside_pinned_sources': sorted(set(normalized) - allowed)})
    assert normalized and set(normalized) <= allowed


def main():
    marker = OUT / 'ATTEMPT.json'
    with marker.open('x') as stream:
        json.dump({'status': 'started', 'native_authority': 0, 'no_retry': True}, stream)
    assert REVIEW['direct_harness_eligibility']['status'] == 'ELIGIBLE_FOR_ORDINARY_DIRECT_RUSTC_HOST_PREPARATION_HARNESS_UNDER_EXACT_CONDITIONS'
    cwd = Path(PLAN['cwd'])
    assert not any((cwd / p).exists() for p in ('libmori_core.rlib', 'preflight-tests', 'core.d', 'tests.d'))
    save('ROOT-SCOPED-ADOPTION.json', {'scope': 'exact five SDK-free host-preparation tests only',
         'review': str(INPUT / 'FINAL-RESPONSE.txt'), 'plan': str(INPUT / 'PROSPECTIVE-EXACT-HARNESS.json'),
         'native_authority': 0, 'source_verdict': REVIEW['unchanged_source_scope'],
         'conditions': REVIEW['exact_conditions_or_blocker']})
    commands = PLAN['commands_serial']
    phase('01-core', commands['production_lib_build_no_cfg_test'])
    depinfo('01-core', 'core.d')
    phase('02-tests-compile', commands['external_five_file_test_compile'])
    depinfo('02-tests-compile', 'tests.d')
    listed = phase('03-list', commands['list_only'])
    selectors = [line.removesuffix(': test') for line in listed.splitlines() if line.endswith(': test')]
    assert selectors == sorted(REVIEW['direct_harness_eligibility']['exact_selectors'])
    results = []
    for index, argv in enumerate(commands['five_selected_runs'], 1):
        output = phase('test-' + str(index), argv)
        assert '1 passed; 0 failed; 0 ignored;' in output
        results.append({'selector': argv[argv.index('--exact') + 1], 'status': 'pass'})
    save('RESULT.json', {'status': 'pass', 'tests': results, 'pins': check_pins(),
         'scope': 'exact five synthetic host-preparation tests; no CLI/observe/runtime/SDK/native',
         'native_authority': 0})


if __name__ == '__main__':
    try:
        main()
    except BaseException as error:
        save('STOP.json', {'status': 'stopped', 'error_type': type(error).__name__, 'error': str(error), 'no_retry': True})
        raise
