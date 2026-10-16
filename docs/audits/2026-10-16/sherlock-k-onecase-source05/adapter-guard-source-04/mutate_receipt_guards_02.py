"""Source-only mutation evidence for the R adapter-guard-source-04 receipt tests.

Usage: /usr/bin/python3 -I -B mutate_receipt_guards_02.py ROOT SOURCE_SHA256 TEST_SHA256

ROOT must be the absolute R path ending in adapter-guard-source-04, holding the
canonical corrected candidate under models/sherlock/qa. The script copies that
candidate into five separate directories under ROOT/mutations: one unmodified
control and four mutants that each remove exactly one audited receipt guard. In
each it runs only the matching pure Python unittest via /usr/bin/python3 -I -B
with an empty environment, in a new session, under the ordinary 30 second child
deadline. On expiry the child's process group is killed, remaining output is
drained for at most 5 seconds, and the run is recorded as not as required.
There is no K, engine, network or install, and nothing is written outside
ROOT/mutations. The control must pass and each mutant must fail the matching
test on exactly the expected subtests, with no errors, or this exits 1.
Generating this script grants no execution authority; it needs fresh review.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys


PYTHON = '/usr/bin/python3'
TIMEOUT = 30
DRAIN = 5
QA = Path('models', 'sherlock', 'qa')
SOURCE = 'k_capture_observations.py'
TEST = 'test_k_capture_observations.py'
SIBLINGS = {
    'k_observations.py': '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07',
    'k_terms.py': 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca',
}
TEST_NAME = 'test_inconsistent_receipts_block_edge_terminal_and_startup'
GAPS = ('receipt-exit-status-nonzero', 'receipt-configuration-sha256-mismatch',
        'receipt-log-sha256-mismatch', 'receipt-argv-does-not-declare-exactly-depth-1')
PROBES = ('edge', 'terminal', 'startup')
MUTANTS = (
    ('canonical-control', None, frozenset()),
    ('delete-edge-receipt-gaps-extend',
     "        edge.extend(_receipt_gaps(receipts[index + 1], raw_log=capture['incoming_log'],\n"
     "                                  configuration=capture['configuration']))\n",
     frozenset((gap, 'edge') for gap in GAPS)),
    ('delete-startup-receipt-gaps-extend',
     "    gaps.extend('startup-' + gap for gap in _receipt_gaps(\n"
     "        receipt, raw_log=probe['log'], configuration=probe['configuration']))\n",
     frozenset((gap, 'startup') for gap in GAPS)),
    ('delete-exit-status-check',
     "    if receipt['exit_status'] != 0:\n"
     "        gaps.append('receipt-exit-status-nonzero')\n",
     frozenset(('receipt-exit-status-nonzero', probe) for probe in PROBES)),
    ('delete-configuration-hash-check',
     "    if receipt['configuration_sha256'] != _sha(configuration.encode('utf-8')):\n"
     "        gaps.append('receipt-configuration-sha256-mismatch')\n",
     frozenset(('receipt-configuration-sha256-mismatch', probe) for probe in PROBES)),
)
_DIGEST = re.compile(r'[0-9a-f]{64}')
_SUBTEST = re.compile(r"\(gap='([^']*)', probe='([^']*)'\)$")


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _refuse(message):
    print('refused: ' + message, file=sys.stderr)
    return 2


def _failures(stderr):
    """Return FAIL subtest pairs, malformed FAIL headers and ERROR headers."""
    pairs, malformed, errors = [], [], []
    for line in stderr.splitlines():
        if line.startswith('ERROR: '):
            errors.append(line)
        elif line.startswith('FAIL: '):
            match = _SUBTEST.search(line)
            if not line.startswith('FAIL: ' + TEST_NAME + ' ') or match is None:
                malformed.append(line)
            else:
                pairs.append((match[1], match[2]))
    return pairs, malformed, errors


def _run(command, cwd):
    """Run one child in a new session; at the deadline kill its process group.

    The group is signalled only while the unreaped leader still reserves its
    group id. A descendant that leaves the group is not contained; the pinned
    candidate and test start no processes.
    """
    with subprocess.Popen(command, cwd=cwd, env={}, stdin=subprocess.DEVNULL,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                          start_new_session=True) as process:
        try:
            stdout, stderr = process.communicate(timeout=TIMEOUT)
            return process.returncode, stdout, stderr, False
        except subprocess.TimeoutExpired:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            stdout, stderr = process.communicate(timeout=DRAIN)
            return process.returncode, stdout, stderr, True


def main(argv):
    if len(argv) != 4:
        return _refuse('usage: ROOT SOURCE_SHA256 TEST_SHA256')
    root = Path(argv[1])
    if (not root.is_absolute() or root.name != 'adapter-guard-source-04'
            or root.is_symlink() or not root.is_dir()):
        return _refuse('ROOT must be an existing absolute non-symlink adapter-guard-source-04')
    if any(_DIGEST.fullmatch(value) is None for value in argv[2:]):
        return _refuse('expected hashes must be lowercase hex SHA-256')
    if not Path(PYTHON).is_file():
        return _refuse(PYTHON + ' is absent')
    expected = {SOURCE: argv[2], TEST: argv[3], **SIBLINGS}
    originals = {}
    for name, digest in expected.items():
        path = root / QA / name
        if path.is_symlink() or not path.is_file():
            return _refuse(str(path) + ' is not a regular file')
        data = path.read_bytes()
        if _sha(data) != digest:
            return _refuse(f'{path} sha256 {_sha(data)} differs from expected {digest}')
        originals[name] = data
    source = originals[SOURCE].decode('utf-8')
    for name, block, _ in MUTANTS:
        if block is not None and source.count(block) != 1:
            return _refuse(f'{name}: guard block does not occur exactly once')
    mutations = root / 'mutations'
    if mutations.exists() or mutations.is_symlink():
        return _refuse(str(mutations) + ' already exists; refusing to overwrite')
    mutations.mkdir()
    records, all_as_required = [], True
    for name, block, expected_failures in MUTANTS:
        mutated = (originals[SOURCE] if block is None
                   else source.replace(block, '', 1).encode('utf-8'))
        directory = mutations / name
        qa = directory / QA
        qa.mkdir(parents=True)
        files = {}
        for file_name, data in originals.items():
            content = mutated if file_name == SOURCE else data
            (qa / file_name).write_bytes(content)
            written = (qa / file_name).read_bytes()
            if written != content:
                print(f'{name}: copy of {file_name} not byte-identical', file=sys.stderr)
                return 1
            files[file_name] = dict(bytes=len(written), sha256=_sha(written))
        command = [PYTHON, '-I', '-B', str(qa / TEST), 'KCaptureObservationTests.' + TEST_NAME]
        returncode, stdout, stderr_bytes, timed_out = _run(command, qa)
        (directory / 'stdout').write_bytes(stdout)
        (directory / 'stderr').write_bytes(stderr_bytes)
        stderr = stderr_bytes.decode('utf-8', 'replace')
        pairs, malformed, errors = _failures(stderr)
        if expected_failures:
            status, tail = 1, f'\nFAILED (failures={len(expected_failures)})\n'
        else:
            status, tail = 0, '\nOK\n'
        as_required = (not timed_out and returncode == status
                       and not malformed and not errors
                       and len(pairs) == len(expected_failures)
                       and set(pairs) == expected_failures
                       and '\nRan 1 test in ' in stderr and stderr.endswith(tail))
        all_as_required = all_as_required and as_required
        records.append(dict(
            mutant=name, removed_block=block,
            removed_block_sha256=None if block is None else _sha(block.encode('utf-8')),
            canonical_source_sha256=_sha(originals[SOURCE]),
            mutant_files=files, argv=command, cwd=str(qa), timeout_seconds=TIMEOUT,
            timed_out=timed_out, exit=returncode,
            stdout=str((directory / 'stdout').relative_to(root)),
            stdout_sha256=_sha(stdout),
            stderr=str((directory / 'stderr').relative_to(root)),
            stderr_sha256=_sha(stderr_bytes),
            expected_exit=status,
            expected_failing_subtests=sorted(map(list, expected_failures)),
            observed_failing_subtests=sorted(map(list, pairs)),
            malformed_fail_headers=malformed, error_headers=errors,
            as_required=as_required))
    result = dict(
        schema='sherlock-k-source-guard-mutations/1',
        scope='R-only control and source mutants of the audited receipt guards; '
              'synthetic tests only',
        root=str(root), python=PYTHON, timeout_seconds=TIMEOUT, drain_seconds=DRAIN,
        test=TEST + '::KCaptureObservationTests.' + TEST_NAME,
        canonical_files={name: dict(bytes=len(data), sha256=_sha(data))
                         for name, data in originals.items()},
        mutants=records, all_as_required=all_as_required,
        execution_layers=dict(K='not-run', SDK='not-run', native='not-run'),
        acceptance='none; fresh review of this script and its outputs required')
    (mutations / 'RESULT.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n',
                                           encoding='utf-8')
    for record in records:
        print(record['mutant'], 'exit', record['exit'],
              'AS-REQUIRED' if record['as_required'] else 'NOT-AS-REQUIRED')
    return 0 if all_as_required else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv))
