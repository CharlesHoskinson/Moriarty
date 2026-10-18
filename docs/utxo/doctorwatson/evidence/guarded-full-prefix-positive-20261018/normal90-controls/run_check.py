"""Root-invoked single ordinary90TERM+5KILL/parent95 Core check; preparation does not execute it.

No retry, deadline renewal, source/Git mutation, extra build or review/admission gate.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time

D = Path(__file__).resolve().parent


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(p, value):
    p.write_text(json.dumps(value, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binding-sha', required=True)
    args = parser.parse_args()
    assert sha(D / 'BOUND-CANDIDATE.json') == args.binding_sha, 'Exact root-inspected binding required.'
    binding = json.loads((D / 'BOUND-CANDIDATE.json').read_bytes())
    assert sha(Path(__file__).resolve()) == binding['driverSha256']
    tools = {x['name']: x for x in json.loads((D / 'TOOL-PINS.json').read_bytes())}
    assert str(Path(sys.executable).resolve()) == tools['python']['resolved']
    checkout, cycle = Path(binding['checkout']), Path(binding['actualOutput'])
    assert checkout.is_absolute() and cycle.is_absolute()
    assert cycle.parent == D.parent and not cycle.exists()
    cycle.mkdir(exist_ok=False)  # One fixed fresh output; no retry/global allocation reset.
    git_env = {'PATH': '/usr/bin:/bin', 'GIT_CONFIG_NOSYSTEM': '1',
               'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8'}

    def git(*words):
        return subprocess.check_output([tools['git']['path'], '--no-optional-locks', '-C', str(checkout), *words],
                                       env=git_env).decode()

    def verify():
        assert sha(D / 'BOUND-CANDIDATE.json') == args.binding_sha
        assert sha(Path(__file__).resolve()) == binding['driverSha256']
        for name, digest in binding['controlHashes'].items():
            assert sha(D / name) == digest, name
        for t in tools.values():
            assert str(Path(t['path']).resolve()) == t['resolved']
            assert sha(Path(t['resolved'])) == t['sha256'], t['name']
        assert git('rev-parse', 'HEAD').strip() == binding['head']
        assert git('rev-parse', 'HEAD^{tree}').strip() == binding['tree']
        assert git('status', '--porcelain=v1') == '', 'Committed clean exact source required.'
        sources = json.loads((D / 'SOURCE-PINS.json').read_bytes())['files']
        for row in sources:
            p = checkout / row['path']
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], row['path']
        expected = json.loads((D / 'EXPECTATION-PINS.json').read_bytes())
        for row in expected['externalOriginalAndSourcePacketFiles']:
            p = Path(row['path'])
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], row['path']
        for row in expected['copiedIndependentFiles']:
            assert (checkout / row['repositoryPath']).read_bytes() == Path(row['originalPath']).read_bytes()
        return {'at': now(), 'head': binding['head'], 'tree': binding['tree'], 'clean': True,
                'sourcePinsVerified': len(sources), 'sourceToolExpectationControlPinsVerified': True}

    def configs(root=None):
        paths = []
        directories = [checkout, *checkout.parents]
        if root is not None:
            directories += [root / 'cwd', *(root / 'cwd').parents]
        for directory in directories:
            for name in ['config', 'config.toml']:
                p = directory / '.cargo' / name
                if str(p) not in paths:
                    assert not p.exists() and not p.is_symlink(), str(p)
                    paths.append(str(p))
        return paths

    # Metadata preflight creates no target or product child and is recorded separately.
    try:
        source_before = verify()
        configs()
    except BaseException as error:
        write(cycle / 'PREFLIGHT-RESULT.json', {'at': now(), 'productChildStarted': False,
              'temporaryTargetCreated': False, 'errorClass': type(error).__name__, 'error': str(error),
              'noAutomaticRetry': True})
        raise

    root = Path(tempfile.mkdtemp(prefix='doctorwatson-guarded-full-prefix-'))
    child = None
    result = {'head': binding['head'], 'tree': binding['tree'], 'sourceBefore': source_before,
              'attemptConsumed': False, 'selectedCheckSatisfied': False}
    members, census_error, source_after = None, None, None
    stop_sent, reaped, expired = False, False, False
    phase_error, post_error, cleanup_error = None, None, None
    start, deadline = None, None
    try:
        for name in ['cwd', 'tmp', 'cargo-home']:
            (root / name).mkdir()
        for name in ['registry', 'git']:
            (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
        tc = Path(tools['cargo']['path']).parent
        env = {'PATH': str(tc) + ':/usr/bin:/bin', 'RUSTC': tools['rustc']['path'],
               'RUSTDOC': tools['rustdoc']['path'], 'CARGO_HOME': str(root / 'cargo-home'),
               'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
               'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
               'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
               'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
        assert len(env) == 13 and not (root / 'target').exists()
        absent_configs = configs(root)
        command = [tools['cargo']['path'], 'test', '--manifest-path', str(checkout / 'Cargo.toml'),
                   '--frozen', '-j1', '-p', 'mori-core', '--no-default-features', '--test',
                   'guarded_quantity_fee_full_prefix_positive', '--', '--test-threads=1']
        argv = [tools['timeout']['path'], '--signal=TERM', '--kill-after=5s', '90s', *command]
        write(cycle / 'CHECK-PLAN.json', {'at': now(), 'head': binding['head'], 'tree': binding['tree'],
              'bindingSha256': args.binding_sha, 'argv': argv, 'cwd': str(root / 'cwd'),
              'environment': env, 'ownedRoot': str(root), 'coldTarget': True,
              'absentAncestorConfigs': absent_configs, 'termSeconds': 90, 'killGraceSeconds': 5,
              'parentMaximumSeconds': 95, 'oneSelectedTarget': True, 'noAutomaticRetry': True,
              'scope': binding['scope'], 'limitations': binding['limitations']})
        write(cycle / 'START.json', {'at': now(), 'argv': argv, 'cwd': str(root / 'cwd'),
              'environment': env, 'head': binding['head'], 'tree': binding['tree']})
        start = time.monotonic()
        deadline = start + 95
        with (cycle / 'STDOUT.log').open('wb') as out, (cycle / 'STDERR.log').open('wb') as err:
            try:
                child = subprocess.Popen(argv, cwd=root / 'cwd', env=env, stdin=subprocess.DEVNULL,
                                         stdout=out, stderr=err, start_new_session=True)
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
                    child.poll()  # Nonblocking; never add wait/grace after95.
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
            census_error = 'Parent95 exhausted; owned membership unknown; no extra wait.'
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
    except BaseException as error:
        if phase_error is None:
            phase_error = {'type': type(error).__name__, 'message': str(error)}
        result.update(phaseError=phase_error, finishedAt=now(), selectedCheckSatisfied=False)
        write(cycle / 'RESULT.json', result)
        raise
    finally:
        # Post-terminal metadata may inspect/hash finite files, never run more product work.
        try:
            for name in ['STDOUT.log', 'STDERR.log']:
                p = cycle / name
                if p.exists():
                    result[name.split('.')[0].lower() + 'Sha256'] = sha(p)
            if child is not None and child.returncode == 0 and reaped and not expired and not phase_error:
                text = (cycle / 'STDOUT.log').read_text()
                summaries = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', text)
                assert len(summaries) == 1, 'Require one whole selected target summary.'
                summary = dict(zip(['passed', 'failed', 'ignored', 'measured', 'filtered'], map(int, summaries[0])))
                assert summary == {'passed': 1, 'failed': 0, 'ignored': 0, 'measured': 0, 'filtered': 0}
                assert 'test guarded_quantity_fee_matches_every_complete_independent_financial_prefix ... ok' in text
                result['rustSummary'] = summary
                result['wholeSelectedTargetPredicate'] = True
            else:
                result['wholeSelectedTargetPredicate'] = False
            source_after = verify()
            result['absentAncestorConfigCountAfter'] = len(configs(root))
        except BaseException as error:
            post_error = {'type': type(error).__name__, 'message': str(error)}
        # Never delete an owned target while live/unknown owned members may still use it.
        if (child is None or members == []) and reaped == (child is not None):
            try:
                shutil.rmtree(root)
            except BaseException as error:
                cleanup_error = {'type': type(error).__name__, 'message': str(error)}
        write(cycle / 'CLEANUP.json', {'at': now(), 'ownedRoot': str(root), 'existsAfter': root.exists(),
              'remainingMembers': members, 'censusError': census_error, 'cleanupError': cleanup_error,
              'unknownOrLiveMembersRetainRoot': True, 'noAutomaticRetry': True})
        result.update(sourceAfter=source_after, postError=post_error, cleanupError=cleanup_error,
                      remainingMembers=members, censusError=census_error, ownedGroupStopSent=stop_sent,
                      ownedRoot=str(root), ownedRootExistsAfter=root.exists(),
                      selectedCheckSatisfied=(child is not None and child.returncode == 0 and reaped and not expired
                          and not phase_error and members == [] and not post_error and not cleanup_error
                          and not root.exists() and result.get('wholeSelectedTargetPredicate') is True))
        write(cycle / 'RESULT.json', result)
        print(json.dumps(result), flush=True)
    assert result['selectedCheckSatisfied'], 'Consumed failed/unknown ordinary check; no retry, renewal or fallback.'


if __name__ == '__main__':
    main()
