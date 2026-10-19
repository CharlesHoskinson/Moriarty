"""PREPARED ONLY: UNRUN/UNADOPTED proposed one600 plus eight90 Core1 journey.

Do not import/run as a documentary check. Root must adopt the resource proposal and inspect this exact successor first.
Frozen original inputs, all commands and financial/wire expectations are retained.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import shutil
import signal
import subprocess
import sys
import time

D = Path(__file__).resolve().parent
A = Path('/home/charl/moriarty-c2-opus55-restored-20261017/rust-core-external-starter-preparation-20261019-a2')
P = Path('/home/charl/moriarty-c2-opus55-restored-20261017/rust-core-external-starter-one600-resource-choice-20261019')
O = A / 'inputs'
K = Path('/home/charl/Moriarty/.worktrees/utxo-guarded-full-prefix-main-20261018')
GIT_ENV = {'PATH': '/usr/bin:/bin', 'GIT_CONFIG_NOSYSTEM': '1',
           'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8'}


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(p, value):
    p.write_text(json.dumps(value, indent=2) + '\n')


def git(*words):
    return subprocess.check_output(['/usr/bin/git', '--no-optional-locks', '-C', str(K), *words],
                                   env=GIT_ENV).decode().strip()


def run_selected(argv, working_directory, env, cycle, tools, session_records, name, parent_seconds):
    # Existing terminal-first control block; only the plan-bound parent deadline varies.
    git_env = GIT_ENV
    result = {'name': name, 'attemptConsumed': False}
    child = None
    members, census_error = None, None
    stop_sent, reaped, expired = False, False, False
    phase_error = None
    state = {'pid': None, 'remainingMembers': None, 'directChildReaped': False}
    start = time.monotonic()
    deadline = start + parent_seconds
    with (cycle / 'STDOUT.log').open('wb') as out, (cycle / 'STDERR.log').open('wb') as err:
        try:
            child = subprocess.Popen(argv, cwd=working_directory, env=env, stdin=subprocess.DEVNULL,
                                     stdout=out, stderr=err, start_new_session=True)
            session_records.append(state)
            state['pid'] = child.pid
            result['attemptConsumed'] = True
            write(cycle / 'CHILD.json', {'pid': child.pid, 'ownedSession': child.pid, 'ownedGroup': child.pid})
            child.wait(timeout=max(0, deadline - time.monotonic()))
            reaped = True
        except BaseException as error:
            expired = isinstance(error, subprocess.TimeoutExpired)
            phase_error = {'type': type(error).__name__, 'message': str(error)}
            if child is not None:
                try:
                    os.killpg(child.pid, signal.SIGKILL)
                    stop_sent = True
                except ProcessLookupError:
                    stop_sent = True
                except OSError as stop_error:
                    result['ownedStopError'] = type(stop_error).__name__
                child.poll()  # Nonblocking; never add wait/grace after the selected parent deadline.
                reaped = child.returncode is not None
    # Persist actual terminal facts/paths BEFORE stream hashes, census or source postchecks.
    result.update(finishedAt=now(), exitCode=child.returncode if child is not None else None, wallSeconds=time.monotonic() - start,
                  parentDeadlineExceeded=expired, directChildReaped=reaped,
                  phaseError=phase_error, stdoutPath=str(cycle / 'STDOUT.log'), stderrPath=str(cycle / 'STDERR.log'))
    write(cycle / 'TERMINAL.json', result)
    write(cycle / 'RESULT.json', result)
    if child is None:
        members = []  # No session was created when spawn failed.
    elif not expired and time.monotonic() < deadline:
        try:
            ps = subprocess.run([tools['ps']['path'], '--sid', str(child.pid), '-o', 'pid=,sid=,stat='],
                                env=git_env, capture_output=True, text=True,
                                timeout=min(.25, deadline - time.monotonic()))
            assert ps.returncode in (0, 1)
            members = ps.stdout.splitlines()
        except (OSError, subprocess.TimeoutExpired, AssertionError, ValueError) as error:
            census_error = type(error).__name__
    else:
        census_error = f'Parent{parent_seconds} exhausted; owned membership unknown; no extra wait.'
    if child is not None and members != [] and not stop_sent:
        try:
            os.killpg(child.pid, signal.SIGKILL)
            stop_sent = True
        except ProcessLookupError:
            stop_sent = True
        except OSError as stop_error:
            result['ownedStopError'] = type(stop_error).__name__
    write(cycle / 'PROCESS-CLEANUP.json', {'at': now(), 'ownedSession': child.pid if child is not None else None,
          'remainingMembers': members, 'censusError': census_error, 'ownedGroupStopSent': stop_sent,
          'noAddedWaitOrGrace': True, 'detachedProcessesExcludedClaim': False})
    state.update(remainingMembers=members, directChildReaped=reaped)
    result.update(remainingOwnedSessionMembers=members, censusError=census_error,
                  ownedGroupStopSent=stop_sent)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binding-sha', required=True)
    args = parser.parse_args()
    assert sha(D / 'BOUND-CANDIDATE.json') == args.binding_sha
    binding = json.loads((D / 'BOUND-CANDIDATE.json').read_bytes())
    references = json.loads((D / 'INPUT-REFERENCES.json').read_bytes())
    for ref in references['files']:
        p = Path(ref['path'])
        assert p.stat().st_size == ref['bytes'] and sha(p) == ref['sha256'], str(p)
    plan = json.loads((P / 'PROPOSED-CHECK-PLAN.json').read_bytes())
    freeze = json.loads((O / 'EXPECTED-FREEZE.json').read_bytes())
    source = json.loads((O / 'WORKSPACE-EXPORT-PINS.json').read_bytes())
    context = json.loads((O / 'SOURCE-PINS.json').read_bytes())
    tools = json.loads((O / 'TOOL-PINS.json').read_bytes())['tools']
    reg = json.loads((O / 'REGISTRY-SOURCE-PINS.json').read_bytes())
    closure = json.loads((O / 'MANIFEST-CLOSURE.json').read_bytes())
    membership = json.loads((A / 'INPUT-MEMBERSHIP.json').read_bytes())
    rebinding = json.loads((P / 'PATH-REBINDING.json').read_bytes())
    overlay = {row['expectationPath']: P / row['after']['path'] for row in rebinding['expectedFileChanges']}
    ROOT = Path(plan['ownedRoot'])
    C = Path(plan['actualOutput'])
    allowed_manifests = {str((Path(pkg['sourceRoot']) / 'Cargo.toml').resolve()) for pkg in reg['packages']} | {
        str((ROOT / 'source/crates' / n / 'Cargo.toml').resolve()) for n in closure['localNormalClosure']}

    def expectation(template, name):
        relative = f'expected/{template}/{name}'
        return overlay.get(relative, O / relative)

    def verify_expectations():
        # Keep all original freeze bodies immutable; only four transport wires differ.
        for row in freeze['files']:
            p = O / row['path']
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], row['path']
        count = 0
        for row in rebinding['expectedFileChanges']:
            before, after = O / row['expectationPath'], P / row['after']['path']
            b = before.read_bytes()
            assert sha(before) == row['before']['sha256'] and len(b) == row['before']['bytes']
            assert b.count(rebinding['oldRoot'].encode()) == row['occurrences']
            assert after.read_bytes() == b.replace(rebinding['oldRoot'].encode(), rebinding['newRoot'].encode())
            assert sha(after) == row['after']['sha256'] and after.stat().st_size == row['after']['bytes']
            count += row['occurrences']
        assert len(overlay) == 4 and count == 8 and len(freeze['files']) == 24
        assert {r['path'] for r in rebinding['unchangedExpectedFiles']} == {r['path'] for r in freeze['files']} - set(overlay)
        for row in rebinding['unchangedExpectedFiles']:
            p = O / row['path']
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], row['path']

    def configs():
        paths = set()
        for directory in [ROOT / 'source/crates/mori', *(ROOT / 'source/crates/mori').parents,
                          ROOT / 'cwd', *(ROOT / 'cwd').parents, ROOT / 'cargo-home']:
            for name in ['config', 'config.toml']:
                p = directory / name if directory == ROOT / 'cargo-home' else directory / '.cargo' / name
                assert not p.exists() and not p.is_symlink(), str(p)
                paths.add(str(p))
        return sorted(paths)

    def verify(exported=False):
        # Same finite identities before and after, including archives and invoked symlinks.
        assert sha(D / 'BOUND-CANDIDATE.json') == args.binding_sha
        assert sha(Path(__file__).resolve()) == binding['driverSha256']
        for name, digest in binding['controlHashes'].items():
            assert sha(D / name) == digest, name
        for ref in references['files']:
            p = Path(ref['path'])
            assert p.stat().st_size == ref['bytes'] and sha(p) == ref['sha256'], str(p)
        for row in membership['files']:
            p = O / row['path']
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)
        for t in tools:
            assert str(Path(t['path']).resolve(strict=True)) == t['resolved'], t['path']
            p = Path(t['resolved'])
            assert p.stat().st_size == t['bytes'] and sha(p) == t['sha256'], t['path']
        python = next(t for t in tools if t['path'] == '/usr/bin/python3')
        assert str(Path(sys.executable).resolve(strict=True)) == python['resolved']
        assert git('rev-parse', 'HEAD') == plan['head']
        assert git('rev-parse', 'HEAD^{tree}') == plan['tree']
        assert not git('status', '--porcelain=v1'), 'Exact clean source required.'
        for row in context['files'] + source['files']:
            p = K / row['path']
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)
        if exported:
            for row in source['files']:
                p = ROOT / 'source' / row['path']
                assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)
            assert not (ROOT / 'source/.git').exists() and not (ROOT / 'source/.moriarty-dev').exists()
        verify_expectations()
        for pkg in reg['packages']:
            assert sha(Path(pkg['archivePath'])) == pkg['archiveSha256'], pkg['archivePath']
            for row in pkg['files']:
                p = Path(pkg['sourceRoot']) / row['path']
                assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)
        return {'at': now(), 'head': plan['head'], 'tree': plan['tree'], 'clean': True,
                'frozenInputFiles': len(membership['files']), 'invokedAndResolvedTools': len(tools),
                'contextPins': len(context['files']), 'workspacePins': len(source['files']),
                'registryArchives': len(reg['packages']), 'registrySourceFiles': sum(len(p['files']) for p in reg['packages']),
                'exportedSourceVerified': exported, 'absentConfigs': configs()}

    # Preparation never invokes this entry point. Actual root-selected preflight consumes no product attempt.
    assert not C.exists() and not C.is_symlink() and not ROOT.exists() and not ROOT.is_symlink()
    assert not Path(plan['oneInitiallyAbsentColdTarget']).exists() and not Path(plan['oneInitiallyAbsentColdTarget']).is_symlink()
    assert len(plan['steps']) == 9 and len(plan['environmentExact13']) == 13
    assert [s['wallSeconds'] for s in plan['steps']] == [600] + [90] * 8
    assert [s['parentMaximumSeconds'] for s in plan['steps']] == [605] + [95] * 8
    assert all(s['killGraceSeconds'] == 5 and s['attempts'] == 1 for s in plan['steps'])
    assert plan['finiteMaximumSelectedCommandWallSeconds'] == 1365
    before = verify()
    C.mkdir(exist_ok=False)
    ROOT.mkdir(exist_ok=False)
    results, sessions = [], []
    execution_success = False
    phase_error, post_error, cleanup_error, after = None, None, None, None
    exported = False
    try:
        for name in ['cwd', 'tmp', 'cargo-home', 'projects']:
            (ROOT / name).mkdir()
        for row in source['files']:
            b = subprocess.check_output(['/usr/bin/git', '--no-optional-locks', '-C', str(K), 'show', plan['head'] + ':' + row['path']], env=GIT_ENV)
            assert hashlib.sha256(b).hexdigest() == row['sha256']
            p = ROOT / 'source' / row['path']
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_bytes(b)
        exported = True
        assert not (ROOT / 'source/.git').exists() and not (ROOT / 'source/.moriarty-dev').exists() and not (ROOT / 'target').exists()
        for name, path in plan['publicCacheInputs'].items():
            if name in ['registry', 'git']:
                (ROOT / 'cargo-home' / name).symlink_to(path, target_is_directory=True)
        env = plan['environmentExact13']
        write(C / 'START.json', {'startedAt': now(), 'bindingSha256': args.binding_sha,
              'planSha256': sha(P / 'PROPOSED-CHECK-PLAN.json'), 'expectationsSha256': sha(O / 'EXPECTED-FREEZE.json'),
              'environment': env, 'absentConfigs': configs(), 'ownedRoot': str(ROOT),
              'head': plan['head'], 'tree': plan['tree'], 'sourceBefore': before, 'state': 'ONE SHOT; NO RETRIES'})
        for step in plan['steps']:
            assert git('rev-parse', 'HEAD') == plan['head'] and not git('status', '--porcelain=v1')
            for row in source['files']:
                assert sha(ROOT / 'source' / row['path']) == row['sha256']
            verify_expectations()
            name = step['name']
            d = C / name
            d.mkdir()
            expected = None
            if 'expectation' in step:
                template, kind = step['expectation'].split('/')
                if kind != 'init':
                    actual_files = {p.name for p in Path(step['cwd']).iterdir()}
                    assert actual_files == {'README.md', 'source.mori', 'scenario.json', 'mori-tests.json'}
                    for n in actual_files:
                        assert (Path(step['cwd']) / n).read_bytes() == expectation(template, n).read_bytes()
                expected = {stream: expectation(template, kind + '.' + stream) for stream in ['stdout', 'stderr']}
            if 'argvFromFrozenGeneratedReadme' in step:
                line = next(l for l in expectation(template, 'README.md').read_text().splitlines() if l.startswith("'") and (' ' + kind + ' ') in l)
                cmd = ['/usr/bin/sh', '-c', line]
            else:
                cmd = step['argv']
            argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', f"{step['wallSeconds']}s", *cmd]
            write(d / 'START.json', {'startedAt': now(), 'argv': argv, 'cwd': step['cwd'], 'environment': env, 'attempt': 1})
            row = run_selected(argv, Path(step['cwd']), env, d,
                               {'ps': next(t for t in tools if t['path'] == '/usr/bin/ps')}, sessions, name, step['parentMaximumSeconds'])
            # The terminal-first control block already wrote terminal facts before this work.
            row['exit'] = row['exitCode']
            results.append(row)
            row['stdoutSha256'] = sha(d / 'STDOUT.log')
            row['stderrSha256'] = sha(d / 'STDERR.log')
            write(d / 'RESULT.json', row)
            assert row['exit'] == 0 and row['directChildReaped'] and not row['parentDeadlineExceeded'] and not row['phaseError'] and row['remainingOwnedSessionMembers'] == [], row
            if expected:
                assert (d / 'STDOUT.log').read_bytes() == expected['stdout'].read_bytes()
                assert (d / 'STDERR.log').read_bytes() == expected['stderr'].read_bytes()
                if kind == 'init':
                    actual_dir = ROOT / 'projects' / template
                    assert {p.name for p in actual_dir.iterdir()} == {'README.md', 'source.mori', 'scenario.json', 'mori-tests.json'}
                    for n in ['README.md', 'source.mori', 'scenario.json', 'mori-tests.json']:
                        assert (actual_dir / n).read_bytes() == expectation(template, n).read_bytes()
            else:
                messages = [json.loads(l) for l in (d / 'STDOUT.log').read_text().splitlines() if l.strip()]
                artifacts = [v for v in messages if v.get('reason') == 'compiler-artifact']
                forbidden = [v for v in artifacts if 'midnight' in v.get('package_id', '') or any(n in v.get('package_id', '') for n in ['mori-backend', 'mori-ledger', 'mori-source6'])]
                assert not forbidden
                assert all(str(Path(v['manifest_path']).resolve()) in allowed_manifests for v in artifacts)
                binary = [v for v in artifacts if v.get('target', {}).get('name') == 'mori' and v.get('target', {}).get('kind') == ['bin']]
                assert len(binary) == 1 and binary[0]['features'] == []
                installed = ROOT / 'installed cli/bin/mori'
                assert installed.is_file() and sha(installed) == sha(Path(binary[0]['executable']))
                write(d / 'INSTALLED-ARTIFACT.json', {'path': str(installed), 'sha256': sha(installed), 'cargoArtifact': binary[0], 'allCargoMessages': messages})
            if expected:
                project = ROOT / 'projects' / template
                assert {p.name for p in project.iterdir()} == {'README.md', 'source.mori', 'scenario.json', 'mori-tests.json'}
                for n in ['README.md', 'source.mori', 'scenario.json', 'mori-tests.json']:
                    assert (project / n).read_bytes() == expectation(template, n).read_bytes()
                write(d / 'PROJECT-BYTES.json', {'files': [{'name': n, 'sha256': sha(project / n), 'bytes': (project / n).stat().st_size} for n in sorted(p.name for p in project.iterdir())], 'completeExpectedEquality': True})
            row['expectationsMatched'] = True
            write(d / 'RESULT.json', row)
            print(json.dumps(row), flush=True)
        execution_success = True
    except BaseException as error:
        phase_error = {'type': type(error).__name__, 'message': str(error)}
        raise
    finally:
        # No additional product work, wait, census deadline or cleanup grace is allocated.
        # Each owned session was censused within its selected plan-bound parent window. A stop
        # after nonempty/unknown census does not turn that observation into empty.
        try:
            after = verify(exported=exported)
        except BaseException as error:
            post_error = {'type': type(error).__name__, 'message': str(error)}
        confirmed_empty = all(s['remainingMembers'] == [] and s['directChildReaped'] for s in sessions)
        if confirmed_empty:
            try:
                shutil.rmtree(ROOT)
            except BaseException as error:
                cleanup_error = {'type': type(error).__name__, 'message': str(error)}
        write(C / 'CLEANUP.json', {'at': now(), 'ownedRoot': str(ROOT), 'existsAfter': ROOT.exists(),
              'sessions': sessions, 'allOwnedSessionsConfirmedEmpty': confirmed_empty,
              'unknownOrLiveMembersRetainRoot': True, 'noAddedWaitOrGrace': True,
              'cleanupError': cleanup_error, 'noAutomaticRetry': True})
        passed = execution_success and not post_error and not cleanup_error and confirmed_empty and not ROOT.exists()
        terminal = {'finishedAt': now(), 'allExpectedComparisonsAndFinalGuardsPassed': passed,
                    'steps': results, 'selectedSteps': len(plan['steps']), 'allCompleted': len(results) == len(plan['steps']),
                    'allExitZero': all(r['exit'] == 0 for r in results), 'sourceAfter': after,
                    'phaseError': phase_error, 'postError': post_error, 'cleanupError': cleanup_error,
                    'sessions': sessions, 'ownedRootExistsAfter': ROOT.exists(),
                    'scope': 'Source-only frontend install and local Core1 observation; no native/PCD/target acceptance.'}
        write(C / 'TERMINAL.json', terminal)
        print(json.dumps(terminal), flush=True)
    assert passed, 'Consumed failed/unknown journey; no retry, renewal or fallback.'


if __name__ == '__main__':
    main()
