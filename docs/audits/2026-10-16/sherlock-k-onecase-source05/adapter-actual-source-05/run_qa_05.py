"""Bounded R-only ordinary-test caller for adapter-actual-source-05 (RED, GREEN, FOCUSED).

Usage:
  /usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/run_qa_05.py

Imports only the pinned _run(command, cwd) transport from
R/adapter-guard-source-04/mutate_receipt_guards_02.py without calling its main()
or touching R/adapter-guard-source-04/mutations. Verifies exact before/after size
and SHA-256 pins for the Python interpreter, transport module, RED tree, GREEN
tree, shared helpers, frozen oracle JSON inputs, and prior 04 RESULT.json. Writes
receipts and raw stdout/stderr logs exclusively to the new directory
R/adapter-actual-source-05/QA-RESULTS-05, stopping immediately on unexpected RED
or failed GREEN. Authoring this script grants no execution authority; fresh
non-author security + Muse review and operator ordinary-slot confirmation are
required before execution.
"""
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import time


ROOT = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01')
PYTHON = '/usr/bin/python3'
PYTHON_SHA256 = '52e0a13e60a981d8c4b6478be2ba5176f69da07948a056bf49cf6f077e30cb41'

GUARD_04_DIR = ROOT / 'adapter-guard-source-04'
TRANSPORT_PATH = GUARD_04_DIR / 'mutate_receipt_guards_02.py'
TRANSPORT_PIN = (9831, 'a0e94827b3ba63b1bc171815ae759c761c2685030da6809e421452b8e3690228')
MUTATION_04_RESULT = GUARD_04_DIR / 'mutations' / 'RESULT.json'

SOURCE_05_DIR = ROOT / 'adapter-actual-source-05'
CALLER_PATH = SOURCE_05_DIR / 'run_qa_05.py'
RESULTS_DIR = SOURCE_05_DIR / 'QA-RESULTS-05'
RED_QA = SOURCE_05_DIR / 'RED' / 'models' / 'sherlock' / 'qa'
GREEN_QA = SOURCE_05_DIR / 'models' / 'sherlock' / 'qa'
ORACLES_DIR = SOURCE_05_DIR / 'models' / 'sherlock' / 'oracles'
ORACLE_NAMES = (
    'normal-programs.json',
    'normal-prefixes.json',
    'extra-normal-programs.json',
    'extra-normal-prefixes.json',
)

RED_PINS = {
    'k_capture_observations.py': (
        21483, '7f99f2955b92f183c7e4033fc14c46e0f2216dc2e9c8d370dad868d95a91ccbc'),
    'test_k_capture_observations.py': (
        88920, '15a4d0bf2a187bfdd549b665f2fc1e3df86fe3aced9e0bc6e42605a25dea2ad6'),
    'k_observations.py': (
        7686, '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07'),
    'k_terms.py': (
        5666, 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca'),
}

GREEN_PINS = {
    'k_capture_observations.py': (
        41183, '5df990ed9c8b54ca1e090e80883f483f2e745138a740cdbcbbfc80fd00ce77d1'),
    'test_k_capture_observations.py': (
        88920, '15a4d0bf2a187bfdd549b665f2fc1e3df86fe3aced9e0bc6e42605a25dea2ad6'),
    'k_observations.py': (
        7686, '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07'),
    'k_terms.py': (
        5666, 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca'),
    'test_k_observations.py': (
        7137, '59b850018e022b76fd9c1b232bdf9969951c22451b4f2b79fded77597315ffef'),
    'test_k_terms.py': (
        5940, '4aa89a0815d959aded4c1512259e54a0621ff2f3c1668988c85134782aa0c68e'),
}

RED_TEST_METHOD = 'test_actual_n01_capture_is_comparison_ready_without_authentication'
RED_TARGET = 'ActualN01CaptureTests.' + RED_TEST_METHOD

STEPS = (
    (
        'RED',
        '01-red',
        RED_QA,
        [PYTHON, '-I', '-B', 'test_k_capture_observations.py', RED_TARGET],
        1,
    ),
    (
        'GREEN',
        '02-green',
        GREEN_QA,
        [PYTHON, '-I', '-B', 'test_k_capture_observations.py'],
        39,
    ),
    (
        'FOCUSED',
        '03-focused',
        GREEN_QA,
        [PYTHON, '-I', '-B', '-m', 'unittest', 'discover', '-p', 'test_k_*.py'],
        57,
    ),
)

_RAN_RE = re.compile(r'^Ran ([0-9]+) tests? in [0-9]+\.[0-9]+s$', re.MULTILINE)


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _utc():
    return datetime.now(timezone.utc).isoformat()


def _refuse(message):
    print('refused: ' + message, file=sys.stderr)
    return 2


def _write_verified_bytes(path, data):
    path.write_bytes(data)
    written = path.read_bytes()
    if written != data:
        raise RuntimeError(f'write verification failed for {path}')


def _write_verified_json(path, payload):
    raw = (json.dumps(payload, indent=2, sort_keys=True) + '\n').encode('utf-8')
    _write_verified_bytes(path, raw)


def _check_regular_file(path):
    if path.is_symlink() or not path.is_file():
        raise RuntimeError(f'{path} is not a regular non-symlink file')
    data = path.read_bytes()
    return len(data), _sha(data)


def _snapshot_and_validate():
    """Verify directory structure, exact pins, and absence of leftovers in QA trees."""
    for directory in (ROOT, GUARD_04_DIR, SOURCE_05_DIR, RED_QA, GREEN_QA, ORACLES_DIR):
        if not directory.is_absolute() or directory.is_symlink() or not directory.is_dir():
            raise RuntimeError(f'{directory} is not an existing absolute non-symlink directory')

    python_path = Path(PYTHON)
    if not python_path.is_file():
        raise RuntimeError(f'{PYTHON} is absent')
    python_data = python_path.read_bytes()
    python_record = dict(bytes=len(python_data), sha256=_sha(python_data))
    if python_record['sha256'] != PYTHON_SHA256:
        raise RuntimeError(
            f'{PYTHON} sha256 {python_record["sha256"]} differs from expected {PYTHON_SHA256}')

    red_entries = sorted(entry.name for entry in RED_QA.iterdir())
    if red_entries != sorted(RED_PINS):
        raise RuntimeError(f'{RED_QA} entries {red_entries} differ from expected {sorted(RED_PINS)}')

    green_entries = sorted(entry.name for entry in GREEN_QA.iterdir())
    if green_entries != sorted(GREEN_PINS):
        raise RuntimeError(
            f'{GREEN_QA} entries {green_entries} differ from expected {sorted(GREEN_PINS)}')

    oracle_entries = sorted(entry.name for entry in ORACLES_DIR.iterdir())
    if oracle_entries != sorted(ORACLE_NAMES):
        raise RuntimeError(
            f'{ORACLES_DIR} entries {oracle_entries} differ from expected {sorted(ORACLE_NAMES)}')

    files = {PYTHON: python_record}

    transport_size, transport_sha = _check_regular_file(TRANSPORT_PATH)
    if (transport_size, transport_sha) != TRANSPORT_PIN:
        raise RuntimeError(
            f'{TRANSPORT_PATH} ({transport_size}, {transport_sha}) != {TRANSPORT_PIN}')
    files[str(TRANSPORT_PATH.relative_to(ROOT))] = dict(bytes=transport_size, sha256=transport_sha)

    result04_size, result04_sha = _check_regular_file(MUTATION_04_RESULT)
    files[str(MUTATION_04_RESULT.relative_to(ROOT))] = dict(
        bytes=result04_size, sha256=result04_sha)

    caller_size, caller_sha = _check_regular_file(CALLER_PATH)
    files[str(CALLER_PATH.relative_to(ROOT))] = dict(bytes=caller_size, sha256=caller_sha)

    for directory, pins in ((RED_QA, RED_PINS), (GREEN_QA, GREEN_PINS)):
        for name, (expected_size, expected_sha) in sorted(pins.items()):
            path = directory / name
            size, digest = _check_regular_file(path)
            if (size, digest) != (expected_size, expected_sha):
                raise RuntimeError(
                    f'{path} ({size}, {digest}) differs from expected '
                    f'({expected_size}, {expected_sha})')
            files[str(path.relative_to(ROOT))] = dict(bytes=size, sha256=digest)

    for name in ORACLE_NAMES:
        path = ORACLES_DIR / name
        size, digest = _check_regular_file(path)
        files[str(path.relative_to(ROOT))] = dict(bytes=size, sha256=digest)

    guard04_entries = sorted(entry.name for entry in GUARD_04_DIR.iterdir())
    return dict(
        files=files,
        directories={
            str(GUARD_04_DIR.relative_to(ROOT)): guard04_entries,
            str(RED_QA.relative_to(ROOT)): red_entries,
            str(GREEN_QA.relative_to(ROOT)): green_entries,
            str(ORACLES_DIR.relative_to(ROOT)): oracle_entries,
        },
    )


def _load_pinned_run():
    """Import _run from pinned mutate_receipt_guards_02.py without running its main()."""
    size_before, sha_before = _check_regular_file(TRANSPORT_PATH)
    if (size_before, sha_before) != TRANSPORT_PIN:
        raise RuntimeError('transport pin mismatch before import')
    spec = importlib.util.spec_from_file_location('mutate_receipt_guards_02', TRANSPORT_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f'unable to load spec from {TRANSPORT_PATH}')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    size_after, sha_after = _check_regular_file(TRANSPORT_PATH)
    if (size_after, sha_after) != TRANSPORT_PIN:
        raise RuntimeError('transport pin mismatch after import')
    if (getattr(module, 'PYTHON', None) != PYTHON
            or getattr(module, 'TIMEOUT', None) != 30
            or getattr(module, 'DRAIN', None) != 5
            or not callable(getattr(module, '_run', None))):
        raise RuntimeError('imported transport module does not expose expected bounded _run')
    return module._run


def _parse_unittest_stderr(stderr_text):
    lines = stderr_text.splitlines()
    fail_headers = [line for line in lines if line.startswith('FAIL: ')]
    error_headers = [line for line in lines if line.startswith('ERROR: ')]
    ran_matches = _RAN_RE.findall(stderr_text)
    ran_tests = int(ran_matches[-1]) if len(ran_matches) == 1 else None
    summary_line = lines[-1] if lines else ''
    return fail_headers, error_headers, ran_tests, summary_line


def _evaluate_step(step_name, returncode, timed_out, stderr_text, expected_tests):
    fail_headers, error_headers, ran_tests, summary_line = _parse_unittest_stderr(stderr_text)
    if timed_out:
        return 'TIMEOUT', False, fail_headers, error_headers, ran_tests, summary_line

    if step_name == 'RED':
        expected_prefix_fail = f'FAIL: {RED_TEST_METHOD} ('
        expected_prefix_error = f'ERROR: {RED_TEST_METHOD} ('
        is_exact_fail = (
            returncode == 1
            and ran_tests == expected_tests
            and len(fail_headers) == 1
            and not error_headers
            and fail_headers[0].startswith(expected_prefix_fail)
            and stderr_text.endswith('\nFAILED (failures=1)\n')
        )
        is_exact_error = (
            returncode == 1
            and ran_tests == expected_tests
            and len(error_headers) == 1
            and not fail_headers
            and error_headers[0].startswith(expected_prefix_error)
            and stderr_text.endswith('\nFAILED (errors=1)\n')
        )
        if is_exact_fail:
            return 'FAIL', True, fail_headers, error_headers, ran_tests, summary_line
        if is_exact_error:
            return 'ERROR', True, fail_headers, error_headers, ran_tests, summary_line
        if returncode == 0 and stderr_text.endswith('\nOK\n'):
            return 'PASS', False, fail_headers, error_headers, ran_tests, summary_line
        return 'OTHER', False, fail_headers, error_headers, ran_tests, summary_line

    passed = (
        returncode == 0
        and ran_tests == expected_tests
        and not fail_headers
        and not error_headers
        and f'\nRan {expected_tests} tests in ' in stderr_text
        and stderr_text.endswith('\nOK\n')
    )
    if passed:
        return 'PASS', True, fail_headers, error_headers, ran_tests, summary_line
    if error_headers:
        return 'ERROR', False, fail_headers, error_headers, ran_tests, summary_line
    if fail_headers or returncode != 0:
        return 'FAIL', False, fail_headers, error_headers, ran_tests, summary_line
    return 'OTHER', False, fail_headers, error_headers, ran_tests, summary_line


def main(argv):
    if len(argv) != 1:
        return _refuse('usage: /usr/bin/python3 -I -B run_qa_05.py')
    if not sys.flags.isolated or not sys.dont_write_bytecode:
        return _refuse('interpreter must run with -I and -B')
    if Path(__file__).resolve() != CALLER_PATH:
        return _refuse(f'caller must reside at exact path {CALLER_PATH}')
    if RESULTS_DIR.exists() or RESULTS_DIR.is_symlink():
        return _refuse(f'{RESULTS_DIR} already exists; refusing to overwrite or delete logs')

    try:
        preflight = _snapshot_and_validate()
        bounded_run = _load_pinned_run()
        post_import = _snapshot_and_validate()
        if post_import != preflight:
            return _refuse('state changed during transport module load')
    except RuntimeError as exc:
        return _refuse(str(exc))

    RESULTS_DIR.mkdir(parents=False, exist_ok=False)
    command_records = []
    terminal_errors = []
    stopped_after = None

    for step_name, slug, cwd, command, expected_tests in STEPS:
        step_dir = RESULTS_DIR / slug
        step_dir.mkdir(parents=False, exist_ok=False)
        before_step = _snapshot_and_validate()
        if before_step != preflight:
            terminal_errors.append(f'{step_name}: pre-step state drifted from preflight')
            stopped_after = step_name
            break

        started_utc = _utc()
        started_mono = time.monotonic()
        drain_expired = False
        try:
            returncode, stdout_bytes, stderr_bytes, timed_out = bounded_run(command, cwd)
        except subprocess.TimeoutExpired as exc:
            returncode = None
            stdout_bytes = exc.stdout or b''
            stderr_bytes = exc.stderr or b''
            timed_out = True
            drain_expired = True
        elapsed_seconds = round(time.monotonic() - started_mono, 6)
        finished_utc = _utc()

        stdout_file = step_dir / 'stdout'
        stderr_file = step_dir / 'stderr'
        _write_verified_bytes(stdout_file, stdout_bytes)
        _write_verified_bytes(stderr_file, stderr_bytes)

        stdout_text = stdout_bytes.decode('utf-8', 'replace')
        stderr_text = stderr_bytes.decode('utf-8', 'replace')

        try:
            after_step = _snapshot_and_validate()
            sources_unchanged = (after_step == before_step == preflight)
        except RuntimeError as exc:
            after_step = dict(error=str(exc))
            sources_unchanged = False

        (
            observed_outcome,
            step_as_required,
            fail_headers,
            error_headers,
            ran_tests,
            summary_line,
        ) = _evaluate_step(step_name, returncode, timed_out, stderr_text, expected_tests)

        as_required = (
            step_as_required
            and not drain_expired
            and stdout_bytes == b''
            and sources_unchanged
        )

        record = dict(
            step=step_name,
            slug=slug,
            argv=list(command),
            cwd=str(cwd),
            env={},
            timeout_seconds=30,
            drain_seconds=5,
            started_utc=started_utc,
            finished_utc=finished_utc,
            elapsed_seconds=elapsed_seconds,
            timed_out=timed_out,
            drain_expired=drain_expired,
            exit_status=returncode,
            expected_ran_tests=expected_tests,
            observed_ran_tests=ran_tests,
            observed_outcome=observed_outcome,
            fail_headers=fail_headers,
            error_headers=error_headers,
            summary_line=summary_line,
            stdout=str(stdout_file.relative_to(RESULTS_DIR)),
            stdout_bytes=len(stdout_bytes),
            stdout_sha256=_sha(stdout_bytes),
            stdout_text=stdout_text,
            stderr=str(stderr_file.relative_to(RESULTS_DIR)),
            stderr_bytes=len(stderr_bytes),
            stderr_sha256=_sha(stderr_bytes),
            stderr_text=stderr_text,
            before_hashes=before_step['files'],
            after_hashes=after_step.get('files') if isinstance(after_step, dict) else None,
            directories_before=before_step['directories'],
            directories_after=(
                after_step.get('directories') if isinstance(after_step, dict) else None
            ),
            sources_unchanged=sources_unchanged,
            as_required=as_required,
        )
        _write_verified_json(step_dir / 'receipt.json', record)
        command_records.append(record)
        print(
            step_name,
            'outcome',
            observed_outcome,
            'exit',
            returncode,
            'AS-REQUIRED' if as_required else 'NOT-AS-REQUIRED',
        )

        if not as_required:
            terminal_errors.append(
                f'{step_name}: not as required (outcome={observed_outcome}, '
                f'exit={returncode}, timed_out={timed_out}, '
                f'sources_unchanged={sources_unchanged})'
            )
            stopped_after = step_name
            break

    try:
        postflight = _snapshot_and_validate()
        postflight_unchanged = (postflight == preflight)
    except RuntimeError as exc:
        postflight = dict(error=str(exc))
        postflight_unchanged = False
        terminal_errors.append(f'postflight validation failed: {exc}')

    expected_result_entries = sorted([ r['slug'] for r in command_records ] + ['RESULT.json'])
    all_as_required = (
        len(command_records) == len(STEPS)
        and all(record['as_required'] for record in command_records)
        and postflight_unchanged
        and not terminal_errors
    )

    summary = dict(
        schema='sherlock-k-source-05-ordinary-qa-receipt/1',
        scope='R-only bounded ordinary unittest execution for adapter-actual-source-05 '
              '(RED 1 test, GREEN 39 tests, FOCUSED 57 tests) reusing pinned 04 _run transport',
        root=str(ROOT),
        results_dir=str(RESULTS_DIR),
        python=PYTHON,
        python_sha256=PYTHON_SHA256,
        transport=dict(
            path=str(TRANSPORT_PATH),
            bytes=TRANSPORT_PIN[0],
            sha256=TRANSPORT_PIN[1],
            imported_symbol='_run',
            main_invoked=False,
            timeout_seconds=30,
            drain_seconds=5,
            env={},
            start_new_session=True,
        ),
        preflight_hashes=preflight['files'],
        postflight_hashes=postflight.get('files') if isinstance(postflight, dict) else None,
        preflight_directories=preflight['directories'],
        postflight_directories=(
            postflight.get('directories') if isinstance(postflight, dict) else None
        ),
        sources_unchanged=postflight_unchanged,
        commands=command_records,
        stopped_after=stopped_after,
        terminal_errors=terminal_errors,
        all_as_required=all_as_required,
        expected_results_dir_entries=expected_result_entries,
        execution_layers=dict(
            K='not-run',
            SDK='not-run',
            native='not-run',
            circuit='not-run',
            proof='not-run',
            guard_04_mutations='not-run',
        ),
        acceptance='none; fresh non-author security + Muse review of caller and outputs required',
    )
    _write_verified_json(RESULTS_DIR / 'RESULT.json', summary)

    actual_result_entries = sorted(entry.name for entry in RESULTS_DIR.iterdir())
    if actual_result_entries != expected_result_entries:
        print(
            f'refused: unexpected entries in {RESULTS_DIR}: {actual_result_entries}',
            file=sys.stderr,
        )
        return 1

    return 0 if all_as_required else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv))
