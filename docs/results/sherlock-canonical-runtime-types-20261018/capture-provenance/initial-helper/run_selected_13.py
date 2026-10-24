"""One-off Root-selected ordinary R435 executor. No imports from product code.
Uses the already reviewed normal subprocess/timeout/prlimit/capture class.
No work is staged or run before an explicit Root decision and exact pin checks.
"""
from pathlib import Path
import argparse, datetime, hashlib, json, os, re, selectors, shlex, shutil, signal, subprocess, time

D = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-ordinary-QA')
E = Path(__file__).resolve().parent
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


def test_membership(op, phase):
    expected = op['expected']['exactTest']
    text = (phase / 'stdout.bin').read_text()
    results = re.findall(r'(?m)^test (\S+) \.\.\. (\S+)$', text)
    assert re.search(r'(?m)^running 1 test\s*$', text), 'missing exact one-test announcement'
    assert results == [(expected, 'ok')], 'test selector/result mismatch or zero tests'
    summary = re.search(r'(?m)^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;', text)
    assert summary and tuple(map(int, summary.groups()[:4])) == (1, 0, 0, 0), 'not exactly one passing selected test'
    return dict(exactTest=expected, selected=1, passed=1, failed=0, ignored=0, measured=0,
                filtered_out=int(summary.group(5)))


def main(selection_path):
    selection = json.loads(Path(selection_path).read_text())
    assert selection['authorized_one_attempt'] is True
    decision = selection['Root_review_decision']
    assert decision['Root_read_full_report'] is True
    assert decision['source_accepted'] is True and decision['all_13_operations_eligible'] is True
    assert decision['actual_full_page_coverage'] == 87
    assert decision['report'] == REVIEW_REPORT
    check(REVIEW_REPORT)
    assert selection['frozen'] == FROZEN
    for row in FROZEN.values():
        check(row)
    check(selection['helper'])
    assert selection['helper'] == pin(Path(__file__))
    assert selection['capture_directory'] == str(CAPTURE)
    assert not CAPTURE.exists() and not (E / 'ONE-ATTEMPT.json').exists()
    proposal = json.loads(Path(FROZEN['proposal']['path']).read_text())
    flat = json.loads(Path(FROZEN['flat_inputs']['path']).read_text())['files']
    composition = json.loads(Path(FROZEN['composition']['path']).read_text())
    dag = json.loads(Path(FROZEN['DAG']['path']).read_text())
    ops = proposal['operations']
    assert len(ops) == 13 and len(flat) == 484
    assert selection['selected_operation_ids'] == [op['id'] for op in ops]
    assert all(op['keptBytesPerStream'] == 131072 for op in ops)
    for directory in [D / 'build', D / 'tmp']:
        assert not directory.is_symlink()
        assert not directory.exists() or (directory.is_dir() and not any(directory.iterdir())), 'output workdir is not fresh/empty'
    assert all(not Path(output).exists() for op in ops for output in op['outputs'])
    # Only after explicit Root selection: reconstruct missing reviewed inputs, never overwrite existing immutable inputs.
    snapshots = composition['files']
    expected_paths = {row['snapshot']['path'] for row in snapshots}
    if (D / 'snapshot').exists():
        assert {str(p) for p in (D / 'snapshot').rglob('*') if p.is_file()} == expected_paths
    claim = dict(selection=pin(selection_path), helper=pin(__file__), started_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                 mode='one exact ordinary attempt; no retry', argv_policy='unchanged frozen proposal')
    with (E / 'ONE-ATTEMPT.json').open('x') as stream:
        json.dump(claim, stream, indent=2)
        stream.write('\n')
    CAPTURE.mkdir(mode=0o700)
    save(CAPTURE / 'ROOT-SELECTION.json', selection)
    try:
        staged = []
        for row in snapshots:
            target = Path(row['snapshot']['path'])
            if target.exists():
                check(row['snapshot'])
                action = 'reused exact immutable reviewed snapshot'
            else:
                check(row['selected'])
                target.parent.mkdir(parents=True, exist_ok=True)
                with target.open('xb') as out, Path(row['selected']['path']).open('rb') as src:
                    shutil.copyfileobj(src, out)
                check(row['snapshot'])
                action = 'materialized exact frozen selected source after selection'
            staged.append(dict(path=str(target), action=action))
        assert {str(p) for p in (D / 'snapshot').rglob('*') if p.is_file()} == expected_paths
        save(CAPTURE / 'STAGING.json', dict(files=staged, snapshot_files=len(staged), Core_files=157, candidate_overlays=9, consumer_data_files=4))
        for directory in [D / 'build', D / 'tmp']:
            directory.mkdir(mode=0o700, exist_ok=True)
        pre = static_check(flat)
        save(CAPTURE / 'PRE-PIN-CHECK.json', pre)
        assert pre['all_match']
        generated = {}
        phases = []
        dag_by_id = {row['id']: row for row in dag['phases']}
        for number, op in enumerate(ops, 1):
            phase = CAPTURE / f'{number:02}'
            phase.mkdir()
            before = static_check(flat)
            save(phase / 'PRE-PIN-CHECK.json', before)
            assert before['all_match']
            for row in generated.values():
                check(row)
            for row in dag_by_id[op['id']]['generated_inputs']:
                assert row['path'] in generated, 'prior generated input not observed'
                check(generated[row['path']])
            assert all(not Path(path).exists() for path in op['outputs'])
            result = capture(op, phase)
            try:
                assert result['actual_exit'] == 0 and result['controller_failure'] is None
                assert all(row['full_stream_retained'] for row in result['streams'])
                outputs = [pin(path) for path in op['outputs']]
                result['outputs'] = outputs
                allowed = {str(Path(row['path']).resolve(strict=True)): row for row in flat}
                allowed.update({str(Path(row['path']).resolve(strict=True)): row for row in generated.values()})
                info = [path for path in op['outputs'] if path.endswith('.d')]
                result['dep_info_inputs'] = [dict(path=path, inputs=dep_inputs(path, allowed)) for path in info]
                if '--exact' in op['argv']:
                    result['exact_test_acceptance'] = test_membership(op, phase)
                for row in outputs:
                    generated[row['path']] = row
                result['accepted'] = True
            except BaseException as error:
                result['accepted'] = False
                result['acceptance_failure'] = type(error).__name__ + ': ' + str(error)
            after = static_check(flat)
            save(phase / 'POST-PIN-CHECK.json', after)
            if not after['all_match']:
                result['accepted'] = False
                result['acceptance_failure'] = 'post-operation static input pin mismatch'
            save(phase / 'RESULT.json', result)
            phases.append(result)
            save(CAPTURE / 'RESULT.json', dict(phases=phases, completed_operations=len(phases), selected_operations=13,
                 accepted_new_tests=sum('exact_test_acceptance' in r and r.get('accepted', False) for r in phases),
                 all_13_passed=False, final_pin_check_pending=True, generated=list(generated.values()), retry=False))
            if not result['accepted']:
                raise RuntimeError('fail-stop at ' + op['id'])
        post = static_check(flat)
        save(CAPTURE / 'POST-PIN-CHECK.json', post)
        assert post['all_match']
        assert len(phases) == 13 and sum('exact_test_acceptance' in r for r in phases) == 10
        save(CAPTURE / 'RESULT.json', dict(phases=phases, completed_operations=13, selected_operations=13,
             accepted_new_tests=10, all_13_passed=True, final_pin_check_pending=False,
             static_inputs_unchanged=True, generated=list(generated.values()), retry=False))
    except BaseException as error:
        try:
            save(CAPTURE / 'POST-PIN-CHECK.json', static_check(flat))
        except BaseException as pin_error:
            save(CAPTURE / 'POST-PIN-CHECK-ERROR.json', dict(error=str(pin_error)))
        save(CAPTURE / 'FAILURE.json', dict(error=type(error).__name__ + ': ' + str(error), retry=False, preserve_all_outputs=True))
        raise


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--selection', required=True)
    main(parser.parse_args().selection)
