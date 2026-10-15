#!/usr/bin/env python3
"""One-shot maintainer evidence capture; execute only after root scheduling GO.

This coordination flag is not a public Moriarty developer permission requirement.
Expectations are immutable independently frozen public fixtures, never evaluator output.
"""
import argparse
import datetime
import fcntl
import gzip
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


RUNTIME = Path(__file__).resolve().parent
REPO = Path('/home/charl/Moriarty/.worktrees/core-test-report-20261015')
TARGET = Path('/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target')
LOCK = Path('/tmp/moriarty-language-wave1-build.lock')
ORACLE_SHA = '3911ef8990bbcdf067f641bcae032f48828b2035ddae55f8687969ecc0cb2395'
PRODUCT_SHA = '4d8bc18746dab1ce89247c7a6e30b9eb376e912c5afe26ce6f367516b7409dc7'
TEST_SHA = '74e021ed57129647354f3ec226d5d3acdee314091d9ed77870b67c3b76ac2387'
SOURCE_PATHS = [
    'Cargo.toml', 'Cargo.lock', 'crates/mori/Cargo.toml',
    'crates/mori/src/main.rs', 'crates/mori/src/commands/test.rs',
    'crates/mori/src/commands/core_io.rs', 'crates/mori/tests/core_test_report.rs',
    'crates/mori-core/Cargo.toml', 'crates/mori-core/src/json.rs',
    'crates/mori-core/src/ir.rs', 'crates/mori-core/src/parser.rs',
    'crates/mori-core/src/runtime.rs',
]


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def metadata(path):
    path = Path(path)
    stat = path.stat()
    return {'path': str(path), 'bytes': stat.st_size, 'sha256': sha(path),
            'mtime_ns': stat.st_mtime_ns}


def save(path, value):
    Path(path).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def require(predicate, message):
    if not predicate:
        raise RuntimeError(message)


def sources():
    return {name: metadata(REPO / name) for name in SOURCE_PATHS}


def stable_sources(before, after):
    return all(before[name]['sha256'] == after[name]['sha256'] for name in before)


def verify_frozen_inputs(oracle):
    for suite in oracle['suites']:
        for field in ['manifest', 'scenario', 'source', 'expected_stdout']:
            item = suite[field]
            require(sha(item['path']) == item['sha256'], 'Changed frozen ' + item['path'])
    item = oracle['feepool_positive']['manifest']
    require(sha(item['path']) == item['sha256'], 'Changed FeePool aggregate manifest')
    for item in oracle['feepool_positive']['original_inputs']:
        require(sha(item['path']) == item['sha256'], 'Changed FeePool input ' + item['path'])
    for item in oracle['baseline_inputs']:
        if item['path'].endswith('/examples/core/fee-pool/cases/expected.json'):
            require(sha(item['path']) == item['sha256'], 'Changed FeePool original corpus')


def invoke(folder, argv, env, binary_hash=None):
    folder.mkdir()
    receipt = {'argv': [str(x) for x in argv], 'cwd': str(REPO),
               'CARGO_TARGET_DIR': str(TARGET), 'started_at_utc': now()}
    if binary_hash is not None:
        require(sha(TARGET / 'debug/mori') == binary_hash, 'Binary changed before invocation')
        receipt['binary_sha256_before'] = binary_hash
    save(folder / 'command.json', receipt)
    with (folder / 'stdout').open('wb') as stdout, (folder / 'stderr').open('wb') as stderr:
        proc = subprocess.run(argv, cwd=REPO, env=env, stdout=stdout, stderr=stderr,
                              check=False)
    receipt.update({'exit': proc.returncode, 'finished_at_utc': now()})
    for stream in ['stdout', 'stderr']:
        path = folder / stream
        compressed = folder / (stream + '.gz')
        compressed.write_bytes(gzip.compress(path.read_bytes(), mtime=0))
        receipt[stream] = metadata(path)
        receipt[stream + '_gzip'] = metadata(compressed)
        require(gzip.decompress(compressed.read_bytes()) == path.read_bytes(),
                'Compression did not preserve exact raw ' + stream)
    if binary_hash is not None:
        receipt['binary_sha256_after'] = sha(TARGET / 'debug/mori')
        require(receipt['binary_sha256_after'] == binary_hash, 'Binary changed during invocation')
    save(folder / 'result.json', receipt)
    return receipt


def compare_normal(folder, receipt, suite):
    raw = (folder / 'stdout').read_bytes()
    actual = json.loads(raw)
    expected_raw = Path(suite['expected_stdout']['path']).read_bytes()
    expected = json.loads(expected_raw)
    require(receipt['exit'] == suite['exit_code'], 'Wrong process exit for ' + suite['id'])
    require(actual == expected, 'Full independent JSON oracle mismatch for ' + suite['id'])
    require(raw.endswith(b'\n'), 'Missing final newline for ' + suite['id'])
    require(suite['stderr_summary'] in (folder / 'stderr').read_text(),
            'Wrong stderr summary for ' + suite['id'])
    if suite['id'] == 'small':
        require(raw == expected_raw, 'Small report exact-byte compatibility mismatch')
    else:
        require(len(raw) > 1_048_576, 'Aggregate did not exceed 1 MiB for ' + suite['id'])
    return {'id': suite['id'], 'comparison': 'passed', 'exit': receipt['exit'],
            'status': actual['status'], 'case_count': len(actual['cases']),
            'failed_indices': [i for i, c in enumerate(actual['cases']) if not c['passed']],
            'stdout_bytes': len(raw), 'entire_independent_json_equal': True,
            'small_exact_bytes_equal': suite['id'] == 'small',
            'binary_sha256': receipt['binary_sha256_after']}


def compare_feepool(folder, receipt, oracle):
    frozen = oracle['feepool_positive']
    corpus_path = next(x['path'] for x in oracle['baseline_inputs']
                       if x['path'].endswith('/examples/core/fee-pool/cases/expected.json'))
    corpus = json.loads(Path(corpus_path).read_text())
    expected = [c for c in corpus['cases'] if c['expected']['status'] == 'accepted']
    raw = (folder / 'stdout').read_bytes()
    actual = json.loads(raw)
    require(receipt['exit'] == 0 and actual['status'] == 'passed', 'FeePool exit/status mismatch')
    require(len(raw) > 1_048_576, 'FeePool aggregate did not exceed 1 MiB')
    require(raw.endswith(b'\n'), 'FeePool final newline missing')
    require('87 passed; 0 failed' in (folder / 'stderr').read_text(), 'FeePool stderr summary mismatch')
    require(len(actual['cases']) == 87, 'FeePool did not retain all 87 positive cases')
    require([c['name'] for c in actual['cases']] == frozen['ordered_names'],
            'FeePool original ordered names mismatch')
    for received, case in zip(actual['cases'], expected):
        require(received['passed'] is True, 'FeePool failed flag for ' + case['name'])
        require(set(received['actual']) == {'Ok'}, 'FeePool execution not Ok for ' + case['name'])
        execution = received['actual']['Ok']
        require(execution['semantics'] == 'moriarty-core/1' and execution['action'] == case['action'],
                'FeePool action/semantics mismatch for ' + case['name'])
        for field in ['post', 'effects']:
            require(execution[field] == case['expected'][field],
                    'FeePool frozen financial ' + field + ' mismatch for ' + case['name'])
        require(isinstance(execution['pre'], dict), 'FeePool pre snapshot missing')
        require(isinstance(execution['steps'], list) and execution['steps'], 'FeePool steps missing')
    return {'id': 'feepool-positive', 'comparison': 'passed', 'exit': 0, 'status': 'passed',
            'case_count': 87, 'stdout_bytes': len(raw), 'ordered_names_equal': True,
            'all_original_frozen_posts_and_ordered_effects_equal': True,
            'pre_and_steps_present': True, 'binary_sha256': receipt['binary_sha256_after']}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--go', action='store_true', help='Root one-shot maintainer scheduling GO received')
    parser.add_argument('--output-dir', type=Path, default=RUNTIME / 'independent-normal-actual')
    args = parser.parse_args()
    require(args.go, 'Source preparation only: root scheduling GO has not been supplied')
    capture = args.output_dir.resolve()
    require(capture.is_relative_to(RUNTIME), 'Capture destination must remain in this public runtime directory')
    capture.mkdir()
    result = {'schema': 'moriarty-independent-normal-actual/1', 'started_at_utc': now(),
              'status': 'running', 'scope': 'normal functional actual results only; no security judgment',
              'script': metadata(Path(__file__).resolve()), 'lock': str(LOCK),
              'target_dir': str(TARGET), 'binary_path': str(TARGET / 'debug/mori'), 'comparisons': []}
    save(capture / 'SUMMARY.json', result)
    try:
        with LOCK.open('a') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            result['lock_acquired_at_utc'] = now()
            oracle_path = RUNTIME / 'INDEPENDENT-EXPECTATIONS.json'
            require(sha(oracle_path) == ORACLE_SHA, 'Independent oracle hash changed')
            oracle = json.loads(oracle_path.read_text())
            verify_frozen_inputs(oracle)
            before = sources()
            require(before['crates/mori/src/commands/test.rs']['sha256'] == PRODUCT_SHA,
                    'Production candidate differs from root named exact candidate')
            require(before['crates/mori/tests/core_test_report.rs']['sha256'] == TEST_SHA,
                    'Regression candidate differs from root named exact candidate')
            result['sources_before_compile'] = before
            result['oracle'] = metadata(oracle_path)
            result['git_head'] = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=REPO, text=True).strip()
            result['git_status'] = subprocess.check_output(['git', 'status', '--short'], cwd=REPO, text=True)
            binary = TARGET / 'debug/mori'
            result['binary_before_compile'] = metadata(binary) if binary.exists() else None
            env = os.environ.copy()
            env['CARGO_TARGET_DIR'] = str(TARGET)
            argv = ['cargo', '+1.99.0', '--offline', '--locked', 'test', '-p', 'mori',
                    '--test', 'core_test_report', '-j2', '--no-run']
            compile_receipt = invoke(capture / 'compile-only', argv, env)
            require(compile_receipt['exit'] == 0, 'Compile-only command failed; binary provenance unestablished')
            require(binary.is_file(), 'Admitted compile-only command did not produce expected public binary')
            after_compile = sources()
            require(stable_sources(before, after_compile), 'Candidate changed during compile-only command')
            result['sources_after_compile'] = after_compile
            result['binary_after_compile'] = metadata(binary)
            binary_hash = sha(binary)
            result['binary_provenance'] = ('successful exact admitted Cargo integration --no-run under shared lock, '
                                           'expected target/debug/mori exists and named source hashes unchanged')
            save(capture / 'SUMMARY.json', result)
            for suite in oracle['suites']:
                folder = capture / suite['id']
                receipt = invoke(folder, [str(binary), '--color', 'never', 'test', suite['directory']], env, binary_hash)
                result['comparisons'].append(compare_normal(folder, receipt, suite))
                save(capture / 'SUMMARY.json', result)
            folder = capture / 'feepool-positive'
            receipt = invoke(folder, [str(binary), '--color', 'never', 'test',
                                     oracle['feepool_positive']['directory']], env, binary_hash)
            result['comparisons'].append(compare_feepool(folder, receipt, oracle))
            verify_frozen_inputs(oracle)
            result['sources_after_all_invocations'] = sources()
            require(stable_sources(before, result['sources_after_all_invocations']),
                    'Candidate changed during public CLI captures')
            require(sha(binary) == binary_hash, 'Binary changed after captures')
            result['status'] = 'passed'
    except Exception as error:
        result['status'] = 'failed'
        result['error'] = str(error)
        raise
    finally:
        result['finished_at_utc'] = now()
        save(capture / 'SUMMARY.json', result)
    print(json.dumps({'status': result['status'], 'summary': str(capture / 'SUMMARY.json'),
                      'comparisons': result['comparisons']}, indent=2))


if __name__ == '__main__':
    main()
