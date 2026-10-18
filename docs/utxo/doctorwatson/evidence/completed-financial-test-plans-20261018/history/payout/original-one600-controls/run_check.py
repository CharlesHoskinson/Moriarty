"""PROPOSED one whole ordinary payout600TERM+5KILL/parent605; root adoption required.

Three unchanged Rust functions, one cold cargo target, two real public CLI calls.
No retry, renewal, fallback, native allocation, source fix or expectation update.
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
import stat
import subprocess
import sys
import tempfile
import time

D = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(path, value, *, new=False):
    with path.open('x' if new else 'w') as f:
        json.dump(value, f, indent=2)
        f.write('\n')


def require_public_file(path, parent):
    assert path.is_absolute() and path.is_relative_to(parent)
    for p in (path, *path.parents):
        assert not p.is_symlink(), str(p)
    assert stat.S_ISREG(path.lstat().st_mode)
    assert not any('.private' in p or p in ('agy.private.log', 'stderr.private') for p in path.parts)
    return path


def adoption(binding, binding_sha, path):
    # A one-task root record, not a persistent scheduler or product permission.
    path = require_public_file(path, D.parent)
    a = json.loads(path.read_bytes())
    assert a['adopted'] is True and a['decision'] == 'ADOPT_EXACT_ONE_PAYOUT_600_PLUS_5'
    assert a['bindingSha256'] == binding_sha
    assert a['head'] == binding['head'] and a['tree'] == binding['tree']
    assert a['driverSha256'] == binding['driverSha256']
    assert a['attempts'] == 1 and a['termSeconds'] == 600 and a['killGraceSeconds'] == 5 and a['parentMaximumSeconds'] == 605
    assert a['nativeAllocationChanged'] is False and a['priorAllocationsReset'] is False
    reviews = a['freshWholeHostReviews']
    assert len(reviews) == 2
    tokens, hashes = set(), set()
    for entry in reviews:
        assert entry['reviewOuterSeconds'] == 420 and entry['reviewKillGraceSeconds'] == 5
        p = require_public_file(Path(entry['wholeOpinionPath']), D.parent)
        assert sha(p) == entry['wholeOpinionSha256']
        opinion = json.loads(p.read_bytes())
        assert opinion['reviewedContractSha256'] == binding_sha
        assert opinion['verdict'] == 'approve-exact-one-batch'
        assert opinion['reason'].strip() and all(opinion['assessments'][axis].strip() for axis in binding['hostAxes'])
        tokens.add(opinion['reviewToken']); hashes.add(entry['wholeOpinionSha256'])
        receipt_path = require_public_file(Path(entry['terminalReceiptPath']), D.parent)
        cleanup_path = require_public_file(Path(entry['cleanupPath']), D.parent)
        assert sha(receipt_path) == entry['terminalReceiptSha256']
        assert sha(cleanup_path) == entry['cleanupSha256']
        receipt, cleanup = json.loads(receipt_path.read_bytes()), json.loads(cleanup_path.read_bytes())
        assert receipt['terminalExitCode'] == 0 and receipt['decision'] == 'EXPORT' and receipt['resultEvents'] == 1
        assert cleanup['ownedGroupRemaining'] is False and cleanup['captureReachedEOF'] is True
    assert tokens == {'payout-one600-host-a-20261018', 'payout-one600-host-b-20261018'} and len(hashes) == 2
    return {'rootRecord': str(path), 'rootRecordSha256': sha(path), 'twoWholeHostOpinionSha256s': sorted(hashes),
            'reviewIdentityAndCognitiveReadsNotAttestedByThisCheck': True}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--binding-sha', required=True)
    ap.add_argument('--root-adoption', type=Path, required=True)
    args = ap.parse_args()
    assert sha(D / 'BOUND-CANDIDATE.json') == args.binding_sha
    binding = json.loads((D / 'BOUND-CANDIDATE.json').read_bytes())
    assert sha(Path(__file__).resolve()) == binding['driverSha256']
    tools = {x['name']: x for x in json.loads((D / 'TOOL-PINS.json').read_bytes())}
    assert str(Path(sys.executable).resolve()) == tools['python']['resolved']
    authorization = adoption(binding, args.binding_sha, args.root_adoption)
    checkout, cycle = Path(binding['checkout']), Path(binding['actualOutput'])
    assert checkout.is_absolute() and cycle.parent == D.parent and not cycle.exists()
    cycle.mkdir(exist_ok=False)  # Fixed-path one-attempt guard; root retains global allowance accounting.
    git_env = {'PATH': '/usr/bin:/bin', 'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
               'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8'}

    def git(*words):
        return subprocess.check_output([tools['git']['path'], '--no-optional-locks', '-C', str(checkout), *words], env=git_env).decode()

    def verify():
        assert sha(D / 'BOUND-CANDIDATE.json') == args.binding_sha
        assert sha(Path(__file__).resolve()) == binding['driverSha256']
        for name, digest in binding['controlHashes'].items():
            assert sha(D / name) == digest, name
        for tool in tools.values():
            assert str(Path(tool['path']).resolve()) == tool['resolved'] and sha(Path(tool['resolved'])) == tool['sha256']
        assert git('rev-parse', 'HEAD').strip() == binding['head']
        assert git('rev-parse', 'HEAD^{tree}').strip() == binding['tree']
        assert git('status', '--porcelain=v1') == '', 'Require clean committed exact GP source.'
        rows = json.loads((D / 'SOURCE-PINS.json').read_bytes())['files']
        for row in rows:
            p = require_public_file(checkout / row['path'], checkout)
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], row['path']
        expected = json.loads((D / 'EXPECTATION-PINS.json').read_bytes())
        for row in expected['originalPlanFiles']:
            p = require_public_file(D / row['physical'], D)
            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256']
        for row in expected['original79Mappings']:
            p = require_public_file(checkout / row['path'], checkout)
            assert p.read_bytes() == (D / row['physical']).read_bytes()
        return {'at': now(), 'head': binding['head'], 'tree': binding['tree'], 'clean': True,
                'sourcePinsVerified': len(rows), 'original79WholeBodyMatches': 79,
                'allSourceToolEnvironmentExpectationControlPinsVerified': True}

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

    def members(sid, deadline):
        remaining = min(.25, deadline - time.monotonic())
        if remaining <= 0:
            raise TimeoutError('605 exhausted: membership unknown; no extra wait.')
        p = subprocess.run([tools['ps']['path'], '--sid', str(sid), '-o', 'pid=,sid=,stat='], env=git_env,
                           capture_output=True, text=True, timeout=remaining)
        assert p.returncode in (0, 1)
        rows = []
        for line in p.stdout.splitlines():
            pid, session, state = line.split()
            assert int(session) == sid
            rows.append({'pid': int(pid), 'sid': sid, 'state': state})
        return rows

    def sample(sid, deadline):
        result = {'at': now(), 'clockTicksPerSecond': os.sysconf('SC_CLK_TCK'), 'processes': [], 'errors': []}
        try:
            result['cpuPressure'] = Path('/proc/pressure/cpu').read_text().strip()
        except OSError as e:
            result['errors'].append(type(e).__name__)
        try:
            for row in members(sid, deadline):
                try:
                    value = (Path('/proc') / str(row['pid']) / 'stat').read_text()
                    fields = value[value.rfind(')') + 2:].split()
                    if int(fields[3]) == sid:
                        result['processes'].append({**row, 'userTicks': int(fields[11]), 'systemTicks': int(fields[12]),
                                                    'startTicks': int(fields[19])})
                except (OSError, ValueError, IndexError) as e:
                    result['errors'].append(type(e).__name__)
        except (OSError, AssertionError, ValueError, subprocess.TimeoutExpired, TimeoutError) as e:
            result['errors'].append(type(e).__name__)
        return result

    try:
        before = verify()
        configs()
    except BaseException as e:
        write(cycle / 'PREFLIGHT-RESULT.json', {'at': now(), 'productChildStarted': False, 'temporaryTargetCreated': False,
              'errorClass': type(e).__name__, 'error': str(e), 'noAutomaticRetry': True})
        raise
    root = Path(tempfile.mkdtemp(prefix='doctorwatson-payout-one600-'))
    child, deadline, start = None, None, None
    remaining, census_error, after = None, None, None
    expired, reaped, stop_sent = False, False, False
    phase_error, post_error, cleanup_error = None, None, None
    samples = []
    result = {'head': binding['head'], 'tree': binding['tree'], 'sourceBefore': before, 'authorization': authorization,
              'attemptConsumed': False, 'selectedCheckSatisfied': False}
    try:
        for name in ['cwd', 'tmp', 'cargo-home']:
            (root / name).mkdir()
        # Existing offline cache mapping only: no cache scan, warming or read-only-mount claim.
        for name in ['registry', 'git']:
            (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
        tc = Path(tools['cargo']['path']).parent
        capture = cycle / 'observations'
        env = {'PATH': str(tc) + ':/usr/bin:/bin', 'RUSTC': tools['rustc']['path'], 'RUSTDOC': tools['rustdoc']['path'],
               'CARGO_HOME': str(root / 'cargo-home'), 'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
               'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never', 'GIT_CONFIG_NOSYSTEM': '1',
               'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8',
               'TMPDIR': str(root / 'tmp'), 'MORI_PAYOUT_OBSERVATIONS': str(capture)}
        assert len(env) == 14 and not capture.exists() and capture.parent.is_dir() and not (root / 'target').exists()
        absent = configs(root)
        # These root copies preserve original fixture inputs; they are NOT retained copies of the child's deleted temp project.
        inputs = cycle / 'root-original-input-copies'
        inputs.mkdir()
        for name in ['three-way-payout.mori', 'three-way-payout.scenario.json']:
            src = checkout / 'crates/fixtures/three-way-payout-contract/inputs' / name
            (inputs / name).write_bytes(src.read_bytes())
        command = [tools['cargo']['path'], 'test', '--manifest-path', str(checkout / 'Cargo.toml'), '--frozen', '-j1',
                   '-p', 'mori', '--no-default-features', '--test', 'core_three_way_payout', '--', '--test-threads=1']
        assert command == binding['cargoArgv']
        argv = [tools['timeout']['path'], '--signal=TERM', '--kill-after=5s', '600s', *command]
        assert argv == binding['timeoutArgv']
        write(cycle / 'CHECK-PLAN.json', {'at': now(), 'argv': argv, 'environment': env, 'cwd': str(root / 'cwd'),
              'head': binding['head'], 'tree': binding['tree'], 'bindingSha256': args.binding_sha,
              'ownedRoot': str(root), 'captureRoot': str(capture), 'coldTarget': True, 'termSeconds': 600,
              'killGraceSeconds': 5, 'parentMaximumSeconds': 605, 'oneWholeTargetAll3Tests': True,
              'absentAncestorConfigs': absent, 'sourceBefore': before, 'noAutomaticRetry': True,
              'inputCopyProvenance': 'Root preexecution fixture snapshots, not actual child temporary-project captures.',
              'preAndPostMetadataOutsideProductDeadline': True, 'scope': binding['scope'], 'limitations': binding['limitations']})
        write(cycle / 'START.json', {'at': now(), 'argv': argv, 'cwd': str(root / 'cwd'), 'environment': env,
              'head': binding['head'], 'tree': binding['tree'], 'authorization': authorization})
        start = time.monotonic(); deadline = start + 605
        diagnostic_deadline, next_sample = start + 90, start
        with (cycle / 'STDOUT.log').open('xb') as stdout, (cycle / 'STDERR.log').open('xb') as stderr:
            try:
                child = subprocess.Popen(argv, cwd=root / 'cwd', env=env, stdin=subprocess.DEVNULL,
                                         stdout=stdout, stderr=stderr, start_new_session=True)
                result['attemptConsumed'] = True
                write(cycle / 'CHILD.json', {'pid': child.pid, 'ownedSession': child.pid, 'ownedGroup': child.pid})
                while child.poll() is None:
                    current = time.monotonic()
                    if current >= deadline:
                        expired = True
                        try:
                            os.killpg(child.pid, signal.SIGKILL); stop_sent = True
                        except ProcessLookupError:
                            stop_sent = True
                        child.poll()  # Nonblocking: no new grace/reap wait after605.
                        break
                    if len(samples) < 12 and current < diagnostic_deadline and current >= next_sample:
                        samples.append(sample(child.pid, min(diagnostic_deadline, deadline)))
                        next_sample = current + 10
                    interval = deadline - time.monotonic()
                    if interval <= 0:
                        continue
                    if time.monotonic() < diagnostic_deadline:
                        interval = min(interval, max(.001, next_sample - time.monotonic()))
                    try:
                        child.wait(timeout=interval)
                    except subprocess.TimeoutExpired:
                        pass
                reaped = child.returncode is not None
            except BaseException as e:
                phase_error = {'type': type(e).__name__, 'message': str(e)}
                if child is not None:
                    try:
                        os.killpg(child.pid, signal.SIGKILL); stop_sent = True
                    except ProcessLookupError:
                        stop_sent = True
                    child.poll(); reaped = child.returncode is not None
        result.update(finishedAt=now(), exitCode=child.returncode if child is not None else None,
                      wallSeconds=time.monotonic() - start, parentDeadlineExceeded=expired,
                      directChildReaped=reaped, phaseError=phase_error,
                      stdoutPath=str(cycle / 'STDOUT.log'), stderrPath=str(cycle / 'STDERR.log'))
        write(cycle / 'TERMINAL.json', result, new=True)  # Immutable minimal whole terminal facts FIRST.
        write(cycle / 'RESULT.json', result)
        # Retain partial/full capture facts immediately; never manufacture a missing output.
        capture_facts = []
        for name in binding['captureFiles']:
            p = capture / name
            row = {'name': name, 'path': str(p), 'exists': p.is_file() and not p.is_symlink()}
            if row['exists']:
                row.update(bytes=p.stat().st_size, sha256=sha(p))
            capture_facts.append(row)
        write(cycle / 'CAPTURE-FACTS.json', {'at': now(), 'files': capture_facts, 'expectedCount': 14,
              'allPresent': all(x['exists'] for x in capture_facts), 'preserveEveryPresentCaptureOnFailure': True})
        result['captureFiles'] = capture_facts
        # AFTER the whole Cargo process returned: never relabel as a pre-call binary measurement.
        binary = root / 'target/debug/mori'
        identity = {'at': now(), 'binaryPath': str(binary), 'exists': binary.is_file(),
                    'measurementTiming': 'AFTER selected Cargo target returned; no pre-call hash observed.'}
        if identity['exists']:
            identity.update(bytes=binary.stat().st_size, sha256=sha(binary))
        write(cycle / 'BINARY-AFTER-TEST.json', identity)
        result['binaryAfterTest'] = identity
        write(cycle / 'RESULT.json', result)
        write(cycle / 'OWNED-CPU-PSI.json', {'samples': samples, 'maxSamples': 12, 'intervalSeconds': 10,
              'samplingStopsAtSeconds': 90, 'noPostTerminalSamplingWait': True,
              'limits': 'Only ownedSID ticks; reaped/unsampled work omitted. Global CPU PSI is not causal attribution.'})
        if child is None:
            remaining = []
        else:
            try:
                remaining = members(child.pid, deadline)
            except (OSError, AssertionError, ValueError, subprocess.TimeoutExpired, TimeoutError) as e:
                census_error = type(e).__name__
        if child is not None and remaining != []:
            # Stop only observed owned SID members plus original group; never inspect other argv/env.
            try:
                os.killpg(child.pid, signal.SIGKILL); stop_sent = True
            except ProcessLookupError:
                stop_sent = True
            for row in remaining or []:
                try:
                    if os.getsid(row['pid']) == child.pid:
                        os.kill(row['pid'], signal.SIGKILL)
                except ProcessLookupError:
                    pass
            # Do not claim clean membership after a kill unless the same deadline permits a fresh census.
            try:
                remaining = members(child.pid, deadline)
            except (OSError, AssertionError, ValueError, subprocess.TimeoutExpired, TimeoutError) as e:
                remaining = None; census_error = type(e).__name__
        write(cycle / 'PROCESS-CLEANUP.json', {'at': now(), 'ownedSession': child.pid if child else None,
              'remainingMembers': remaining, 'censusError': census_error, 'ownedStopSent': stop_sent,
              'noAdditionalWaitOrGrace': True, 'detachedProcessesExcludedClaim': False})
    except BaseException as e:
        phase_error = {'type': type(e).__name__, 'message': str(e)}
        result.update(phaseError=phase_error, selectedCheckSatisfied=False)
        write(cycle / 'RESULT.json', result)
    finally:
        # Terminal fallback survives errors before the normal terminal-writing point.
        if child is not None and not (cycle / 'TERMINAL.json').exists():
            child.poll()
            write(cycle / 'TERMINAL.json', {**result, 'finishedAt': now(), 'exitCode': child.returncode,
                  'wallSeconds': time.monotonic() - start, 'phaseError': phase_error,
                  'directChildReaped': child.returncode is not None}, new=True)
        try:
            for name in ['STDOUT.log', 'STDERR.log']:
                p = cycle / name
                if p.is_file():
                    result[name.split('.')[0].lower() + 'Sha256'] = sha(p)
            if child is not None and child.returncode == 0 and reaped and not expired and not phase_error:
                text = (cycle / 'STDOUT.log').read_text()
                found = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', text)
                assert len(found) == 1 and tuple(map(int, found[0])) == (3, 0, 0, 0, 0)
                for name in binding['testFunctions']:
                    assert 'test ' + name + ' ... ok' in text
                assert len(result.get('captureFiles', [])) == 14 and all(x['exists'] for x in result['captureFiles'])
                for stem, expected_argv in zip(['ordinary-unmetered', 'ordinary-metered'], binding['publicCliArgv']):
                    process = json.loads((cycle / 'observations' / (stem + '.process.json')).read_bytes())
                    assert process['argv'] == expected_argv and process['status_code'] == 0 and process['success'] is True
                    assert process['executable'] == str(root / 'target/debug/mori') and process['spawn_error'] is None
                    assert (cycle / 'observations' / (stem + '.stderr.bin')).read_bytes() == b''
                result['wholeSelectedTargetPredicate'] = True
                result['rustSummary'] = {'passed': 3, 'failed': 0, 'ignored': 0, 'measured': 0, 'filtered': 0}
            else:
                result['wholeSelectedTargetPredicate'] = False
            after = verify()
            result['absentAncestorConfigCountAfter'] = len(configs(root))
        except BaseException as e:
            post_error = {'type': type(e).__name__, 'message': str(e)}
        if (child is None or remaining == []) and (child is None or reaped):
            try:
                shutil.rmtree(root)
            except BaseException as e:
                cleanup_error = {'type': type(e).__name__, 'message': str(e)}
        write(cycle / 'CLEANUP.json', {'at': now(), 'ownedRoot': str(root), 'existsAfter': root.exists(),
              'remainingMembers': remaining, 'censusError': census_error, 'cleanupError': cleanup_error,
              'unknownOrLiveMembersRetainRoot': True, 'allPersistentCapturesRetained': True, 'noAutomaticRetry': True})
        result.update(sourceAfter=after, postError=post_error, cleanupError=cleanup_error, remainingMembers=remaining,
                      censusError=census_error, ownedStopSent=stop_sent, ownedRoot=str(root), ownedRootExistsAfter=root.exists(),
                      selectedCheckSatisfied=(child is not None and child.returncode == 0 and reaped and not expired
                          and not phase_error and remaining == [] and not post_error and not cleanup_error
                          and not root.exists() and result.get('wholeSelectedTargetPredicate') is True))
        write(cycle / 'RESULT.json', result)
        print(json.dumps(result), flush=True)
    assert result['selectedCheckSatisfied'], 'Consumed failed/unknown whole batch; no retry/renewal/fallback.'


if __name__ == '__main__':
    main()
