"""PROPOSED ordinary ONE600+5 batch; not authority to execute."""
from pathlib import Path
import datetime, hashlib, json, os, re, shutil, signal, subprocess, sys, tempfile, time

D = Path(__file__).resolve().parent
R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
SELECTOR = 'quantity_min_selects_both_operands_through_public_source6_expansion'
NODE_TEST = 'experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs'
FINANCIAL_NAMES = ['financial_beta_invoice', 'financial_beta_repayment', 'financial_core1_payment',
                   'financial_core1_loan', 'financial_core2_escrow', 'financial_core2_rebate']


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def git(checkout, *args):
    return subprocess.check_output(['git', '--no-optional-locks', *args], cwd=checkout).decode().strip()


def owned_members(sid, deadline=None):
    # Select this owned session only; no argv/environment or other workloads.
    remaining = 0.25 if deadline is None else min(0.25, deadline - time.monotonic())
    if remaining <= 0:
        raise TimeoutError('Owned census deadline exhausted; remaining members unknown.')
    result = subprocess.run(['/usr/bin/ps', '--sid', str(sid), '-o', 'pid=,sid=,comm='],
                            capture_output=True, text=True, check=False, timeout=remaining)
    if result.returncode not in (0, 1):
        raise RuntimeError('owned-session ps failed: ' + str(result.returncode))
    rows = []
    for line in result.stdout.splitlines():
        pid, session, command = line.strip().split(maxsplit=2)
        if int(session) == sid:
            rows.append({'pid': int(pid), 'session': sid, 'command': command})
    return rows


def cpu_sample(sid, deadline):
    """One small observation, called only while the bounded child is live."""
    observation = {'at': now(), 'monotonic': time.monotonic(),
                   'clockTicksPerSecond': os.sysconf('SC_CLK_TCK'),
                   'scope': 'Owned SID CPU ticks; host CPU PSI is global, not causal attribution.',
                   'processes': [], 'errors': []}
    try:
        observation['cpuPressure'] = Path('/proc/pressure/cpu').read_text().strip()
    except OSError as error:
        observation['errors'].append({'source': '/proc/pressure/cpu', 'error': type(error).__name__})
    try:
        members = owned_members(sid, deadline)
    except (OSError, RuntimeError, ValueError, subprocess.TimeoutExpired, TimeoutError) as error:
        observation['errors'].append({'source': 'owned-session ps', 'error': type(error).__name__})
        return observation
    for member in members:
        try:
            stat = Path('/proc') / str(member['pid']) / 'stat'
            raw = stat.read_text()
            rest = raw[raw.rfind(')') + 2:].split()
            if int(rest[3]) != sid:
                continue
            observation['processes'].append({**member, 'state': rest[0],
                'userTicks': int(rest[11]), 'systemTicks': int(rest[12]),
                'startTimeTicks': int(rest[19])})
        except (OSError, ValueError, IndexError) as error:
            observation['errors'].append({'pid': member['pid'], 'error': type(error).__name__})
    return observation


def load_contract(expected_hash=None):
    path = D / 'BOUND-CANDIDATE.json'
    if expected_hash is not None:
        assert sha(path) == expected_hash, 'Bound contract changed.'
    contract = json.loads(path.read_text())
    assert contract['productExecutionAuthorizedByThisFile'] is False
    assert sha(Path(__file__).resolve()) == contract['driverSha256'], 'Driver changed.'
    assert str(Path(sys.executable).resolve()) == contract['driverInterpreterResolved']
    return contract


def verify_all(contract, expected_hash):
    load_contract(expected_hash)
    checkout = Path(contract['checkout'])
    assert git(checkout, 'rev-parse', 'HEAD') == contract['head']
    assert git(checkout, 'rev-parse', 'HEAD^{tree}') == contract['tree']
    assert not git(checkout, 'status', '--porcelain=v1')
    for control in contract['controlFiles']:
        assert sha(D / control['path']) == control['sha256'], control['path']
    pins = json.loads((D / 'PINNED-SOURCES.json').read_text())['files']
    for item in pins:
        assert sha(checkout / item['path']) == item['sha256'], item['path']
    for tool in json.loads((D / 'TOOL-PINS.json').read_text())['tools']:
        path = Path(tool['path'])
        assert str(path.resolve()) == tool['resolved'], tool['path']
        assert sha(path.resolve()) == tool['sha256'], tool['path']
    for item in json.loads((D / 'EXPECTATION-PINS.json').read_text())['files']:
        assert sha(D / item['path']) == item['sha256'], item['path']
        if item.get('repositoryPath'):
            assert (D / item['path']).read_bytes() == (checkout / item['repositoryPath']).read_bytes()
    return {'at': now(), 'head': contract['head'], 'tree': contract['tree'],
            'sourcePinsVerified': len(pins), 'sourceTaskToolExpectationPinsVerified': True, 'clean': True}


def verify_configs(paths):
    for name in paths:
        path = Path(name)
        assert not path.exists() and not path.is_symlink(), name
    return len(paths)


def rust_summary(text):
    found = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', text)
    assert len(found) == 1, 'Require exactly one selected integration-test summary.'
    return dict(zip(['passed', 'failed', 'ignored', 'measured', 'filtered'], map(int, found[0])))


def node_summary(text):
    """Accept one exact TAP or plain SPEC summary for the two bound cases."""
    rows = re.findall(r'^(#|ℹ) (tests|pass|fail)(?: (.*))?$', text, re.MULTILINE)
    counts, reporters = {}, set()
    for reporter, key, value in rows:
        if re.fullmatch(r'[0-9]+', value) is None:
            raise ValueError('Malformed Node summary count: ' + key)
        if key in counts:
            raise ValueError('Duplicate Node summary count: ' + key)
        reporters.add(reporter)
        counts[key] = int(value)
    if len(reporters) != 1 or counts != {'tests': 2, 'pass': 2, 'fail': 0}:
        raise ValueError('Require one complete TAP or SPEC summary: ' + repr(counts))
    return counts


def worker(expected_hash, plan_path):
    contract = load_contract(expected_hash)
    plan = json.loads(Path(plan_path).read_text())
    output, root = Path(contract['actualOutput']), Path(plan['ownedRoot'])
    assert str(Path.cwd()) == str(root / 'cwd')
    assert dict(os.environ) == plan['environment'], 'Worker must receive exactly13env.'
    assert len(os.environ) == 13
    assert plan['head'] == contract['head'] and plan['tree'] == contract['tree']
    assert plan['steps'] == contract['steps'], 'Exact bound commands required.'
    # No new session/group: every phase remains under the one outer timeout/session.
    binary = root / 'target/debug/mori'
    quantity_hash = None
    for step in plan['steps']:
        before = verify_all(contract, expected_hash)
        before['emptyAncestorConfigsVerified'] = verify_configs(plan['absentAncestorConfigs'])
        phase = output / step['name']
        phase.mkdir()
        environment = dict(plan['environment'])
        if step['kind'] == 'node':
            assert binary.is_file() and quantity_hash is not None
            assert sha(binary) == quantity_hash, 'Require exact Source6 binary from preceding Rust test.'
            environment.update(MORI_RUST_BIN=str(binary), MORI_SOURCE6_OBSERVATIONS=str(output / 'node-observations'))
            write(phase / 'BINARY-IDENTITY.json', {'path': str(binary), 'sha256': quantity_hash,
                'origin': 'Same target/debug/mori produced/executed by source6_expand test; no fallback build.'})
        write(phase / 'START.json', {'at': now(), 'argv': step['argv'], 'cwd': str(root / 'cwd'),
            'environment': environment, 'sourceBefore': before, 'oneWholeBatchDeadline': True})
        began = time.monotonic()
        with (phase / 'STDOUT.log').open('wb') as stdout, (phase / 'STDERR.log').open('wb') as stderr:
            child = subprocess.Popen(step['argv'], cwd=root / 'cwd', env=environment,
                stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr)
            write(phase / 'CHILD.json', {'pid': child.pid, 'session': os.getsid(child.pid),
                'group': os.getpgid(child.pid), 'inheritsWholeBatchAllocation': True})
            code = child.wait()
        result = {'finishedAt': now(), 'exitCode': code, 'wallSeconds': time.monotonic() - began,
            'stdoutSha256': sha(phase / 'STDOUT.log'), 'stderrSha256': sha(phase / 'STDERR.log'),
            'validationComplete': False}
        write(phase / 'RESULT.json', result)  # Terminal facts survive every later post-check failure.
        if step['kind'] == 'node':
            # Persist available post-run custody facts before any summary parsing.
            result['binarySha256After'] = sha(binary)
            write(phase / 'RESULT.json', result)
            run_record = json.loads((output / 'node-observations/run.json').read_text())
            result['nodeRunBinarySha256'] = run_record.get('binarySha256')
            result['postNodeBinaryMatches'] = result['binarySha256After'] == quantity_hash
            result['nodeRunBinaryMatches'] = result['nodeRunBinarySha256'] == quantity_hash
            write(phase / 'RESULT.json', result)
            assert result['postNodeBinaryMatches'], 'Source6 binary changed during Node phase.'
            assert result['nodeRunBinaryMatches'], 'Node run record binary differs from Source6 binary.'
        result['sourceAfter'] = verify_all(contract, expected_hash)
        result['sourceAfter']['emptyAncestorConfigsVerified'] = verify_configs(plan['absentAncestorConfigs'])
        write(phase / 'RESULT.json', result)
        assert code == 0, 'Failed phase: stop entire batch; no retry/fallback.'
        text = (phase / 'STDOUT.log').read_text()
        if step['kind'] == 'financial':
            summary = rust_summary(text)
            assert summary == {'passed': 6, 'failed': 0, 'ignored': 0, 'measured': 0, 'filtered': 8}, summary
            for name in FINANCIAL_NAMES:
                assert 'test ' + name + ' ... ok' in text, name
            assert binary.is_file(), 'Selected test must provide its actual CARGO_BIN_EXE_mori binary.'
            result.update(rustSummary=summary, binarySha256=sha(binary), binaryRole='no-default-features financial check CLI')
            # Persist BEFORE next Cargo can overwrite target/debug/mori.
            write(output / 'FINANCIAL-BINARY-IDENTITY.json', {'path': str(binary), 'sha256': result['binarySha256'], 'recordedBeforeSource6Cargo': True})
        elif step['kind'] == 'quantity':
            summary = rust_summary(text)
            assert summary == {'passed': 1, 'failed': 0, 'ignored': 0, 'measured': 0, 'filtered': 7}, summary
            assert 'test ' + SELECTOR + ' ... ok' in text
            assert binary.is_file(), 'No second/fallback standalone build.'
            quantity_hash = sha(binary)
            result.update(rustSummary=summary, binarySha256=quantity_hash, binaryRole='no-default-features + source6-expansion CLI')
            write(output / 'SOURCE6-BINARY-IDENTITY.json', {'path': str(binary), 'sha256': quantity_hash, 'recordedBeforeNode': True})
        else:
            result['nodeSummary'] = node_summary(text)
        result['validationComplete'] = True
        write(phase / 'RESULT.json', result)
        print(json.dumps({'step': step['name'], **result}), flush=True)
    write(output / 'WORKER-COMPLETE.json', {'at': now(), 'allThreePhasesValidated': True,
        'expectedFinancialTests': 6, 'expectedQuantityTests': 1, 'expectedNodeCases': 2})


def run():
    contract_path = D / 'BOUND-CANDIDATE.json'
    expected_hash = sha(contract_path)
    contract = load_contract(expected_hash)
    # Heavy initial/final metadata pin checks are OUTSIDE the600s child-work clock.
    before = verify_all(contract, expected_hash)
    output = Path(contract['actualOutput'])
    assert output.parent.resolve() == R.resolve()
    output.mkdir(exist_ok=False)  # Same-path guard, not a persistent global attempt counter.
    root = Path(tempfile.mkdtemp(prefix='doctorwatson-financial-quantity-batch-'))
    active_sid, active_deadline, parent_expired = None, None, False
    child, failure, cleanup_error, residual, samples = None, None, None, [], []
    result, after, configs = None, None, []
    try:
        for name in ['cwd', 'tmp', 'cargo-home']:
            (root / name).mkdir()
        for name in ['registry', 'git']:
            (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
        configs = []
        for directory in [Path(contract['checkout']), *Path(contract['checkout']).parents, root / 'cwd', *(root / 'cwd').parents]:
            for name in ['config', 'config.toml']:
                path = directory / '.cargo' / name
                if str(path) not in configs:
                    assert not path.exists() and not path.is_symlink(), str(path)
                    configs.append(str(path))
        environment = {'PATH': str(TC) + ':/usr/bin:/bin', 'RUSTC': str(TC / 'rustc'),
            'RUSTDOC': str(TC / 'rustdoc'), 'CARGO_HOME': str(root / 'cargo-home'),
            'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
            'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
            'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_SYSTEM': '/dev/null',
            'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
        assert len(environment) == 13
        plan = {'at': now(), 'head': contract['head'], 'tree': contract['tree'], 'ownedRoot': str(root),
            'environment': environment, 'steps': contract['steps'], 'sourceBefore': before,
            'absentAncestorConfigs': configs, 'wholeBatchSeconds': 600, 'killGraceSeconds': 5,
            'preAndPostBatchPinChecksOutsideChildTime': True, 'perPhasePinChecksInsideChildTime': True,
            'withinBatchTargetReuseOnly': True, 'noStandaloneBuild': True, 'noNativeAllocationReset': True}
        plan_path = output / 'CHECK-PLAN.json'
        write(plan_path, plan)
        argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '600s',
                '/usr/bin/python3', str(Path(__file__).resolve()), '--worker', expected_hash, str(plan_path)]
        write(output / 'START.json', {'at': now(), 'argv': argv, 'cwd': str(root / 'cwd'),
            'environment': environment, 'contractSha256': expected_hash, 'attempt': 1,
            'allocation': 'ONE wholebatch600+5; not600perstep; root manual admission required.'})
        began = time.monotonic()
        active_deadline = began + 605
        sample_deadline, next_sample = began + 90, began
        reaped, owned_stop = False, None
        with (output / 'STDOUT.log').open('wb') as stdout, (output / 'STDERR.log').open('wb') as stderr:
            child = subprocess.Popen(argv, cwd=root / 'cwd', env=environment, stdin=subprocess.DEVNULL,
                stdout=stdout, stderr=stderr, start_new_session=True)
            active_sid = child.pid
            write(output / 'CHILD.json', {'pid': child.pid, 'session': active_sid})
            while child.poll() is None:
                current = time.monotonic()
                if current >= active_deadline:
                    parent_expired = True
                    owned_stop = {'at': now(), 'sessionAndGroup': active_sid, 'signal': 'SIGKILL'}
                    try:
                        os.killpg(active_sid, signal.SIGKILL)
                        owned_stop['sent'] = True
                    except ProcessLookupError:
                        owned_stop['alreadyAbsent'] = True
                    except OSError as error:
                        owned_stop['error'] = type(error).__name__
                    child.poll()  # Nonblocking reap; no added grace.
                    reaped = child.returncode is not None
                    break
                if len(samples) < 12 and current >= next_sample and current < sample_deadline:
                    samples.append(cpu_sample(active_sid, min(sample_deadline, active_deadline)))
                    next_sample = current + 10
                remaining = active_deadline - time.monotonic()
                if remaining <= 0:
                    continue
                interval = remaining
                if time.monotonic() < sample_deadline:
                    interval = min(interval, max(0.001, next_sample - time.monotonic()))
                try:
                    child.wait(timeout=interval)
                except subprocess.TimeoutExpired:
                    pass
            else:
                reaped = True
        result = {'finishedAt': now(), 'exitCode': child.returncode, 'wallSeconds': time.monotonic() - began,
            'stdoutSha256': sha(output / 'STDOUT.log'), 'stderrSha256': sha(output / 'STDERR.log'),
            'parentDeadlineExceeded': parent_expired, 'ownedGroupStop': owned_stop,
            'directChildReaped': reaped, 'attemptConsumed': True}
        write(output / 'CHILD-RESULT.json', result)
        write(output / 'OWNED-CPU-PSI.json', {'samples': samples, 'maxSamples': 12, 'intervalSeconds': 10,
            'samplingStopsAtSeconds': 90, 'noPostTerminalWait': True,
            'limitations': 'Same bounded diagnostic as previous90attempt; not extended to600. CPU ticks omit reaped/unsampled work; global PSI does not establish timeout cause.'})
        assert not parent_expired, '605s parent maximum exhausted; no extra grace.'
        residual = owned_members(active_sid, active_deadline)
        write(output / 'PROCESS-CLEANUP.json', {'ownedSession': active_sid, 'remainingSessionMembers': residual})
        assert not residual, 'Owned processes remain; no later phase/retry.'
        active_sid = None
        assert child.returncode == 0, 'Failed/expired batch; no retry/fallback.'
        complete = json.loads((output / 'WORKER-COMPLETE.json').read_text())
        assert complete['allThreePhasesValidated']
    except BaseException as error:
        failure = {'type': type(error).__name__, 'message': str(error)}
        raise
    finally:
        try:
            if parent_expired:
                raise TimeoutError('605s exhausted; owned census availability unknown; no extra wait.')
            if active_sid is not None:
                residual = owned_members(active_sid, active_deadline)
            for member in residual:
                try:
                    if os.getsid(member['pid']) == active_sid:
                        os.kill(member['pid'], signal.SIGKILL)
                except ProcessLookupError:
                    pass
            # Never remove a tree while remaining/unknown owned processes might use it.
            if not residual:
                shutil.rmtree(root)
        except BaseException as error:
            cleanup_error = {'type': type(error).__name__, 'message': str(error)}
        try:
            after = verify_all(contract, expected_hash)
            after['emptyAncestorConfigsVerified'] = verify_configs(configs)
        except BaseException as error:
            after = {'sourceTaskToolExpectationPinsVerified': False, 'error': type(error).__name__, 'message': str(error)}
        write(output / 'CLEANUP.json', {'at': now(), 'ownedRoot': str(root), 'existsAfter': root.exists(),
            'remainingBeforeImmediateOwnedKill': residual, 'cleanupError': cleanup_error,
            'noAdditionalChildWorkGraceOrRetry': True, 'retainsRootIfResidualOrUnknown': True})
        write(output / 'RESULT.json', {'at': now(), 'head': contract['head'], 'tree': contract['tree'],
            'child': result, 'failure': failure, 'sourceBefore': before, 'sourceAfter': after,
            'allSelectedChecksSatisfied': failure is None and cleanup_error is None and not residual
                and not root.exists() and after.get('sourceTaskToolExpectationPinsVerified') is True,
            'temporaryRootExistsAfter': root.exists(), 'resourceScope': 'ONE600+5ordinarybatch only; all previous allocations unchanged.',
            'evidenceScope': 'Six financialsourcecheck tests plus one quantityRust test and two Node cases only; no full workspace/compilerSDK/native/proof/ledger/parent acceptance.'})


if __name__ == '__main__':
    if len(sys.argv) == 4 and sys.argv[1] == '--worker':
        worker(sys.argv[2], sys.argv[3])
    else:
        assert len(sys.argv) == 1
        run()
