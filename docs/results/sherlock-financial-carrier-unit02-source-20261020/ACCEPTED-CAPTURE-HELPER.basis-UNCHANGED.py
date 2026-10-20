"""One-off Root-selected ordinary R435 executor. No imports from product code.
Uses the already reviewed normal subprocess/timeout/prlimit/capture class.
No work is staged or run before an explicit Root decision and exact pin checks.
"""
from pathlib import Path
import argparse, datetime, hashlib, json, os, re, selectors, shlex, shutil, signal, subprocess, time

D = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-ordinary-QA')
E = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation')
CAPTURE = E / 'actual-thirteen-01'
# Filled once during source preparation, not computed from mutable selection.
FROZEN = {'proposal': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-ordinary-QA/PROPOSED.json', 'bytes': 29898, 'sha256': '4ad42defa549cef2f5c3a6ac8d688457a8759d3634af8fefd779b1b9dad6d620'}, 'DAG': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-ordinary-QA/READSET-AND-OUTPUT-DAG.json', 'bytes': 13177, 'sha256': 'f153e2874e422a9195b4d297a5f6053644c4c491bc80c5c55551f6f3dcd3bf97'}, 'flat_inputs': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-ordinary-QA/FLAT-INPUT-PINS.json', 'bytes': 135220, 'sha256': 'cd49f9e88c82155f46fa0935a53f84fa2069dddb29d6e7060cfcf08379c1b48f'}, 'composition': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-ordinary-QA/SOURCE-COMPOSITION.json', 'bytes': 167929, 'sha256': 'a95f7361b4fa4d13349fa2fa1541e24089d65f650c65bf72bdbfae5d44136ea6'}}

REVIEW_REPORT = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/433-generic-resolved-signature-combined-review-preparation/final-review-01/REPORT.md', 'bytes': 31481, 'sha256': '5e4717cdf0677e272b76a03a09671f096ec0eae5467919b16c879af325a498ae'}

def pin(path):
    path = Path(path)
    raw = path.read_bytes()
    return dict(path=str(path), bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())


def check(row):
    actual = pin(row['path'])
    assert actual['bytes'] == row['bytes'] and actual['sha256'] == row['sha256'], row['path']
    return actual


def save(path, value):
    path = Path(path)
    data = (json.dumps(value, indent=2, ensure_ascii=False) + '\n').encode()
    temporary = path.with_name(path.name + '.partial')
    with temporary.open('xb') as stream:
        stream.write(data)
    os.replace(temporary, path)


def static_check(rows):
    cache = {}
    results = []
    for row in rows:
        path = Path(row['path'])
        real = str(path.resolve(strict=True))
        if real not in cache:
            cache[real] = pin(path)
        actual = cache[real]
        results.append(dict(path=str(path), match=actual['bytes'] == row['bytes'] and actual['sha256'] == row['sha256']))
    return dict(files=results, all_match=all(row['match'] for row in results))


def stop_group(process):
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def capture(op, phase):
    """Same finite ordinary pattern: exact argv/env, binary bounded pipes, one child."""
    save(phase / 'COMMAND.json', op)
    began = time.monotonic()
    deadline = began + op['wallSeconds']
    streams = {}
    process = None
    failure = None
    capture_complete = False
    poller = selectors.DefaultSelector()
    try:
        process = subprocess.Popen(op['argv'], cwd=op['cwd'], env=op['environment'], stdin=subprocess.DEVNULL,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True, close_fds=True)
        for name, pipe in [('stdout', process.stdout), ('stderr', process.stderr)]:
            out = (phase / (name + '.bin')).open('xb')
            streams[name] = dict(pipe=pipe, out=out, seen=0, retained=0, path=phase / (name + '.bin'))
            poller.register(pipe, selectors.EVENT_READ, name)
        while poller.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0 and failure is None:
                failure = 'wall deadline'
                stop_group(process)
                break
            for key, _ in poller.select(timeout=min(0.1, max(0.0, remaining)) if failure is None else 0.1):
                record = streams[key.data]
                block = os.read(key.fileobj.fileno(), 65536)
                if not block:
                    poller.unregister(key.fileobj)
                    key.fileobj.close()
                    continue
                record['seen'] += len(block)
                keep = block[:max(0, op['keptBytesPerStream'] - record['retained'])]
                record['out'].write(keep)
                record['retained'] += len(keep)
                if record['seen'] > op['keptBytesPerStream'] and failure is None:
                    failure = key.data + ' capture cap exceeded'
                    stop_group(process)
            if failure is not None:
                break
        capture_complete = not poller.get_map()
        try:
            code = process.wait(timeout=max(0.001, deadline - time.monotonic()))
        except subprocess.TimeoutExpired:
            failure = failure or 'wall deadline'
            stop_group(process)
            code = process.poll()
    except BaseException as error:
        failure = failure or type(error).__name__ + ': ' + str(error)
        if process is not None:
            stop_group(process)
            try:
                process.wait(timeout=max(0.001, deadline - time.monotonic()))
            except subprocess.TimeoutExpired:
                pass
        code = process.returncode if process is not None else None
    finally:
        poller.close()
        for record in streams.values():
            record['out'].close()
            record['pipe'].close()
    return dict(id=op['id'], actual_exit=code, elapsed_seconds=time.monotonic() - began,
                controller_failure=failure, streams=[dict(**pin(r['path']), bytes_seen=r['seen'], bytes_retained=r['retained'],
                full_stream_retained=capture_complete and r['seen'] == r['retained']) for r in streams.values()])


def dep_inputs(path, allowed):
    text = Path(path).read_text().replace('\\\n', ' ')
    read = set()
    for line in text.splitlines():
        if line.startswith('#'):
            assert not line.startswith('# env-dep:'), 'undeclared env-dep in dep-info'
            continue
        _, separator, rest = line.partition(': ')
        if separator:
            read.update(shlex.split(rest, comments=False, posix=True))
    assert read, 'empty dep-info read set'
    observed = []
    for item in sorted(read):
        path = Path(item)
        assert path.is_absolute(), 'unexpected relative dep-info input: ' + item
        real = str(path.resolve(strict=True))
        assert real in allowed, 'unknown dep-info input: ' + item
        row = allowed[real]
        actual = pin(path)
        assert actual['bytes'] == row['bytes'] and actual['sha256'] == row['sha256'], 'changed dep-info input: ' + item
        observed.append(dict(**actual, selected_binding=row['path']))
    return observed


HERE = Path(__file__).resolve().parent
CAPTURE = HERE / 'actual-tail-four-01'
TAIL_PROJECTION = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/revision03-tail-four/TAIL-FOUR-PROJECTION.json', 'bytes': 15560, 'sha256': '4af00c2c4aab3d788565269f265d5db5469286b664f84e1242cfb222938e0408'}

def parse_exact_stdout(expected, text):
    # Only the fixed serial libtest envelope and the frozen consumer's one diagnostic are allowed.
    diagnostic_cases = {
        'uint_builtin_identity_uses_real_canonical_carriers': 'uint_identity',
        'bool_builtin_identity_uses_real_canonical_carriers': 'bool_identity',
        'list_builtin_identity_uses_real_canonical_carriers': 'list_identity',
        'tree_builtin_identity_uses_real_canonical_carriers': 'tree_identity',
    }
    diagnostic = (re.escape(diagnostic_cases[expected]) +
        r': owner_events=\d+, consumed=\d+, remaining=\d+, helper_work=\d+, helper_visits=\d+, reference_cursor=\d+; absolute independent B clocks UNBOUND') if expected in diagnostic_cases else None
    inline = r'test ' + re.escape(expected) + r' \.\.\. ok'
    body = inline if diagnostic is None else '(?:' + inline + '|test ' + re.escape(expected) + r' \.\.\. ' + diagnostic + r'\nok)'
    pattern = (r'\nrunning 1 test\n' + body +
        r'\n\ntest result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured; (\d+) filtered out; finished in (?:\d+)\.(?:\d+)s\n\n')
    match = re.fullmatch(pattern, text)
    assert match, 'stdout is not the exact one-selected-test envelope with the frozen diagnostic shape'
    assert int(match.group(1)) == 4, 'sealed five-test consumer must filter the other four'
    return dict(exactTest=expected, selected=1, passed=1, failed=0, ignored=0, measured=0,
                filtered_out=4, diagnostic_counts_are_observations_not_independent_B_clock_oracles=True)


def test_membership(op, phase):
    assert (phase / 'stderr.bin').read_bytes() == b'', 'unexpected stderr for sealed consumer'
    return parse_exact_stdout(op['expected']['exactTest'], (phase / 'stdout.bin').read_text())


def main(selection_path):
    selection = json.loads(Path(selection_path).read_text())
    assert selection['authorized_one_attempt'] is True
    assert selection['offline09_interpretation_adopted_by_Root'] is True
    decision = selection['same_context_review_decision']
    assert decision['Root_read_full_report'] is True and decision['tail_four_operations_eligible'] is True
    check(decision['report'])
    assert selection['helper'] == pin(__file__)
    check(selection['helper'])
    for row in FROZEN.values(): check(row)
    check(REVIEW_REPORT)
    check(TAIL_PROJECTION)
    projection = json.loads(Path(TAIL_PROJECTION['path']).read_text())
    check(projection['offline09_reader'])
    for row in projection['original_01_through_09_receipts']: check(row)
    for row in projection['prior_actual_outputs']: check(row)
    original = json.loads(Path(FROZEN['proposal']['path']).read_text())
    original_dag = json.loads(Path(FROZEN['DAG']['path']).read_text())
    ops = projection['operations']
    assert ops == original['operations'][9:13] and len(ops) == 4
    assert projection['DAG_phases'] == original_dag['phases'][9:13]
    assert selection['selected_operation_ids'] == [op['id'] for op in ops]
    assert all(not op.get('outputs', []) for op in ops)
    assert all(op['keptBytesPerStream'] == 131072 and op['wallSeconds'] == 10 for op in ops)
    assert selection['capture_directory'] == str(CAPTURE)
    assert not CAPTURE.exists() and not (HERE / 'ONE-ATTEMPT.json').exists()
    flat = json.loads(Path(FROZEN['flat_inputs']['path']).read_text())['files']
    assert len(flat) == 484
    before = static_check(flat)
    assert before['all_match']
    # No staging, build, compile, output replacement, or prefix operations are available in this helper.
    with (HERE / 'ONE-ATTEMPT.json').open('x') as stream:
        json.dump(dict(selection=pin(selection_path), helper=pin(__file__), retry=False), stream, indent=2)
    CAPTURE.mkdir(mode=0o700)
    save(CAPTURE / 'ROOT-SELECTION.json', selection)
    save(CAPTURE / 'PRE-PIN-CHECK.json', before)
    phases = []
    try:
        for index, op in enumerate(ops, 10):
            phase = CAPTURE / f'{index:02}'
            phase.mkdir()
            for row in projection['prior_actual_outputs']: check(row)
            pre = static_check(flat)
            save(phase / 'PRE-PIN-CHECK.json', pre)
            assert pre['all_match']
            result = capture(op, phase)
            try:
                assert result['actual_exit'] == 0 and result['controller_failure'] is None
                assert len(result['streams']) == 2 and all(row['full_stream_retained'] for row in result['streams'])
                result['exact_test_acceptance'] = test_membership(op, phase)
                result['accepted'] = True
            except BaseException as error:
                result['accepted'] = False
                result['acceptance_failure'] = type(error).__name__ + ': ' + str(error)
            post = static_check(flat)
            save(phase / 'POST-PIN-CHECK.json', post)
            if not post['all_match']:
                result['accepted'] = False
                result['acceptance_failure'] = 'post-operation input pin mismatch'
            save(phase / 'RESULT.json', result)
            phases.append(result)
            save(CAPTURE / 'RESULT.json', dict(phases=phases, all_four_passed=False, retry=False))
            assert result['accepted'], 'fail-stop; no retry'
        for row in projection['prior_actual_outputs']: check(row)
        post = static_check(flat)
        save(CAPTURE / 'POST-PIN-CHECK.json', post)
        assert post['all_match'] and len(phases) == 4
        save(CAPTURE / 'RESULT.json', dict(phases=phases, all_four_passed=True, actual_new_tests=4,
            original_prefix_replayed=False, static_inputs_unchanged=True, retry=False))
    except BaseException as error:
        save(CAPTURE / 'FAILURE.json', dict(error=type(error).__name__ + ': ' + str(error), retry=False))
        raise


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--selection', required=True)
    main(parser.parse_args().selection)
