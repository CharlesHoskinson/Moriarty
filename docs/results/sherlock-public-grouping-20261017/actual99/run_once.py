"""Root-only one-shot execution of the frozen R75/R76 three-test selection."""
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import time
from datetime import datetime, timezone

HERE = Path(__file__).resolve().parent


def read(name):
    return json.loads((HERE / name).read_text())


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def record(path):
    path = Path(path)
    digest = hashlib.sha256()
    size = 0
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(chunk)
            size += len(chunk)
    return dict(path=str(path), bytes=size, sha256=digest.hexdigest())


def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n')


def check_entries(entries):
    observed = []
    for entry in entries:
        actual = record(entry['path'])
        require(all(actual[k] == entry[k] for k in ('path', 'bytes', 'sha256')),
                'pin mismatch: ' + entry['path'])
        observed.append(actual)
    return observed


def check_inputs():
    frozen = check_entries(FREEZE['files'])
    sources = check_entries(PINS['sources'])
    dependencies_tools = check_entries(PINS['existing_normal_dependencies_tools'])
    expected = {e['path'] for e in PINS['existing_normal_dependencies_tools']
                if e['category'] == 'normal_dependency_inventory'}
    actual = {str(p) for p in Path(PLAN['dependency_directory']).iterdir() if p.is_file()}
    require(actual == expected, 'normal dependency directory membership changed')
    return dict(frozen_preparation=frozen, sources=sources,
                dependencies_tools=dependencies_tools, dependency_membership_unchanged=True)


def phase(name, argv):
    before = check_inputs()
    check_entries(SEALED)
    save(name + '-PREEXEC.json', dict(utc=datetime.now(timezone.utc).isoformat(),
         argv=argv, cwd=PLAN['cwd'], environment=PLAN['environment'],
         inputs=before, sealed_artifacts=SEALED))
    start = time.monotonic()
    child = subprocess.run(argv, cwd=PLAN['cwd'], env=PLAN['environment'],
                           stdin=subprocess.DEVNULL, capture_output=True)
    elapsed = time.monotonic() - start
    (OUT / (name + '.stdout')).write_bytes(child.stdout)
    (OUT / (name + '.stderr')).write_bytes(child.stderr)
    result = dict(argv=argv, cwd=PLAN['cwd'], environment=PLAN['environment'],
                  exit_code=child.returncode, elapsed_seconds=elapsed,
                  stdout=record(OUT / (name + '.stdout')),
                  stderr=record(OUT / (name + '.stderr')))
    # Persist actual child status before any subsequent collector check.
    save(name + '-RESULT.json', result)
    result['post_inputs'] = check_inputs()
    result['post_sealed_artifacts'] = check_entries(SEALED)
    save(name + '-RESULT.json', result)
    require(child.returncode == 0, name + ' child exit ' + str(child.returncode))
    return child.stdout.decode('utf-8')


def depinfo(name, filename):
    path = OUT / filename
    # These frozen paths have no whitespace/colon; rustc's first Make rule is
    # the complete emitted-target source list. Preserve the whole raw file.
    first = path.read_text().replace('\\\n', '').splitlines()[0]
    paths = shlex.split(first.split(': ', 1)[1])
    normalized = [str((OUT / p).resolve()) for p in paths]
    allowed = {str(Path(e['path']).resolve()) for e in PINS['sources']}
    outside = sorted(set(normalized) - allowed)
    save(name + '-DEPINFO.json', dict(raw_file=record(path), raw_paths=paths,
         normalized_paths=normalized, unique_sources=len(set(normalized)),
         outside_pinned_sources=outside))
    require(bool(normalized) and not outside, name + ' depinfo outside source pins')


def main():
    global OUT, PLAN, PINS, FREEZE, SEALED
    PLAN = read('PROSPECTIVE-EXACT-HARNESS.json')
    PINS = read('FROZEN-PINS.json')
    FREEZE = read('SOURCE-FREEZE.json')
    OUT = HERE / 'attempt-01'
    require(PLAN['attempt'] == str(OUT) and PLAN['cwd'] == str(OUT), 'fixed attempt mismatch')
    # Exclusive directory creation retains every prior attempt, including failures.
    OUT.mkdir(exist_ok=False)
    (OUT / 'tmp').mkdir()
    SEALED = []
    save('ATTEMPT.json', dict(utc=datetime.now(timezone.utc).isoformat(),
         status='started', no_retries=True, native_authority=0,
         collector=record(Path(__file__)), plan=record(HERE / 'PROSPECTIVE-EXACT-HARNESS.json')))
    require(len(PINS['sources']) == 127, 'source inventory size changed')
    require(len(PINS['existing_normal_dependencies_tools']) == 273, 'dependency/tool pins changed')
    save('PREEXEC-INVENTORY.json', check_inputs())
    commands = PLAN['commands_serial']
    phase('01-core', commands['production_rlib'])
    depinfo('01-core', 'core.d')
    SEALED.append(record(OUT / 'libmori_core.rlib'))
    save('SEALED-ARTIFACTS.json', SEALED)
    phase('02-tests-compile', commands['external_three_compile'])
    depinfo('02-tests-compile', 'tests.d')
    SEALED.append(record(OUT / 'grouping-tests'))
    save('SEALED-ARTIFACTS.json', SEALED)
    listed = phase('03-list', commands['list_only'])
    expected = sorted(PLAN['selectors'])
    require(len(expected) == 3 and len(set(expected)) == 3, 'fixed selectors changed')
    selectors = [s.removesuffix(': test') for s in listed.splitlines() if s.endswith(': test')]
    save('LIST-READBACK.json', dict(actual=selectors, expected=expected))
    require(selectors == expected, 'list differs from exact three selectors')
    require(listed.rstrip().endswith('3 tests, 0 benchmarks'), 'unexpected list summary')
    runs = commands['three_exact_runs']
    require(len(runs) == 3, 'three serial runs required')
    require([a[a.index('--exact') + 1] for a in runs] == PLAN['selectors'], 'run selectors differ')
    results = []
    for index, argv in enumerate(runs, 1):
        output = phase('test-' + str(index), argv)
        name = argv[argv.index('--exact') + 1]
        require('test ' + name + ' ... ok' in output and
                '1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out' in output,
                'unexpected exact-test outcome: ' + name)
        results.append(dict(selector=name, result='pass'))
    save('POST-INVENTORY.json', check_inputs())
    save('RESULT.json', dict(status='pass', tests=results, sealed_artifacts=check_entries(SEALED),
         scope=PLAN['scope'], native_authority=0,
         qualification='Root-local collector receipts; no independent runtime instrumentation attestation.'))


if __name__ == '__main__':
    try:
        main()
    except BaseException as error:
        # Never overwrite an earlier attempt when exclusive creation failed.
        if 'OUT' in globals() and 'SEALED' in globals():
            save('STOP.json', dict(status='stopped', error_type=type(error).__name__,
                 error=str(error), no_retry=True))
        raise
