"""One selected C5 accept outer invocation through the existing capture helper.

No extra timeout process or SDK/receipt handling. The existing owned outer
controller remains responsible for service ownership, cleanup and receiving.
This process and every descendant run inside the fixed Root scope; its cap plus
the service cap is the exact owned-pipeline aggregate 16000000000 bytes, swap 0.
"""
from pathlib import Path
import datetime
import hashlib
import json
import os
import re
import resource
import runpy
import signal

D = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-recursion-restart-20261022/native/source46-C1-C5-expected03')
# Root fills this token once, after the one generation, with the sha256 of the
# generated control bytes. Unfilled refuses before any read or write.
CONTROL_SHA256 = 'UNFILLED_C5_CONTROL_SHA256_ROOT_FILLS_AFTER_ONCE_GENERATION_ONLY'
ROOT_UNIT = 'sherlock-source46-real-wallet-accept01-root.scope'
OBSERVER_UNIT = 'sherlock-source46-real-wallet-accept01-observer.scope'
SERVICE_UNIT = 'sherlock-source46-real-wallet-accept01.service'
SCOPE_MEMORY_MAX = 536870912
SERVICE_MEMORY_MAX = 15463129088
AGGREGATE = 16000000000
assert re.fullmatch('[0-9a-f]{64}', CONTROL_SHA256), 'control sha256 unfilled'
assert SERVICE_MEMORY_MAX + SCOPE_MEMORY_MAX == AGGREGATE and SERVICE_MEMORY_MAX % 4096 == SCOPE_MEMORY_MAX % 4096 == 0

def pin(path):
    raw = Path(path).read_bytes()
    return dict(path=str(path), bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())

def durable(path, value):
    with path.open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    descriptor = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)

assert resource.getrlimit(resource.RLIMIT_CPU) == (10, 10)
assert resource.getrlimit(resource.RLIMIT_AS) == (2147483648, 2147483648)
assert resource.getrlimit(resource.RLIMIT_FSIZE) == (134217728, 134217728)
assert resource.getrlimit(resource.RLIMIT_CORE) == (0, 0)
# Outside-cgroup readback before any control read: exactly the fixed Root scope,
# containing this pid, capped, no swap; the observer scope is not live beside it.
cgroups = [x.split(':', 2)[2] for x in Path('/proc/self/cgroup').read_text().splitlines() if x.startswith('0::')]
assert len(cgroups) == 1 and Path(cgroups[0]).name == ROOT_UNIT
scope = Path('/sys/fs/cgroup') / cgroups[0].lstrip('/')
assert str(os.getpid()) in (scope / 'cgroup.procs').read_text().split()
assert (scope / 'memory.max').read_text().strip() == str(SCOPE_MEMORY_MAX)
assert (scope / 'memory.swap.max').read_text().strip() == '0'
assert not (scope.parent / OBSERVER_UNIT).exists()
config_path = D / 'EXACT-CONTROL-AND-OPERATIONS.HELD.json'
config_raw = config_path.read_bytes()
assert hashlib.sha256(config_raw).hexdigest() == CONTROL_SHA256
config = json.loads(config_raw)
oc = config['memoryCapacity']['outsideCgroup']
assert (config['action'], config['unit'], config['wholeOperationWallSeconds']) == ('accept', SERVICE_UNIT, 1900)
assert (oc['unit'], oc['MemoryMaxBytes'], oc['MemorySwapMaxBytes'], oc['aggregatePhysicalBytes']) == (ROOT_UNIT, SCOPE_MEMORY_MAX, 0, AGGREGATE)
assert (config['memoryCapacity']['MemoryMaxBytes'], config['memoryCapacity']['MemorySwapMaxBytes']) == (SERVICE_MEMORY_MAX, 0)
accounting = config['resourceAccounting']
assert (accounting['outsideCPUAllowance'], accounting['prospectivePHold'], accounting['prospectiveSHold'], accounting['prospectiveWholeHold'], accounting['serviceLifetimeMaxSeconds']) == (50, 3740, 3740, 1900, 1845)
outer = config['outerRootArgv']
assert outer[:6] == ['/usr/bin/prlimit', '--cpu=10:10', '--as=1073741824:1073741824', '--fsize=134217728:134217728', '--core=0:0', '--']
assert outer.count('1895s') == 1 and outer[-1] == str(D / 'ROOT-OUTER-CALLER.HELD.py.txt')
decision_path = Path(config['resourceDecisionPath'])
assert decision_path == D / 'ROOT-RESOURCE-DECISION.json'
decision = json.loads(decision_path.read_text())
assert decision['status'] == 'ADOPTED_SOURCE46_ACTUAL_WALLET_BOUND_D_CALL01'
assert decision['configuration_sha256'] == CONTROL_SHA256
assert decision['root_capture_source'] == pin(Path(__file__))
assert decision['external_final_time_observer'] == pin(D / 'ROOT-FINAL-TIME-OBSERVATION-C5.py')
assert (decision['attempts'], decision['P_hold'], decision['S_hold'], decision['whole_hold'], decision['C13'], decision['old_holds_retained'], decision['retry']) == (1, 3740, 3740, 1900, 'STOPPED_UNCHANGED', True, False)
assert (decision['outside_cpu_allowance_seconds'], decision['root_scope_cpu_allowance_seconds'], decision['root_cpu_seconds'], decision['external_final_time_observer_cpu_seconds']) == (50, 49, 10, 1)
assert (decision['aggregatePhysicalBudgetBytes'], decision['serviceMemoryMaxBytes'], decision['rootScopeMemoryMaxBytes'], decision['observerScopeMemoryMaxBytes'], decision['swapBytes'], decision['rootAddressSpaceBytes'], decision['externalObserverAddressSpaceBytes']) == (AGGREGATE, SERVICE_MEMORY_MAX, SCOPE_MEMORY_MAX, SCOPE_MEMORY_MAX, 0, 2147483648, 268435456)
helper = config['existingCapture']
assert pin(helper['path']) == {key: helper[key] for key in ('path', 'bytes', 'sha256')}
phase = D / 'root-outer-capture01'
phase.mkdir(mode=0o700)
descriptor = os.open(D, os.O_RDONLY | os.O_DIRECTORY)
try:
    os.fsync(descriptor)
finally:
    os.close(descriptor)
durable(phase / 'LIVE-ROOT-CONTROLLER.json', dict(
    pid=os.getpid(), start=Path('/proc/self/stat').read_text().rsplit(')', 1)[1].split()[19],
    utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    cpu_limit=list(resource.getrlimit(resource.RLIMIT_CPU)),
    address_space_limit=list(resource.getrlimit(resource.RLIMIT_AS)),
    cgroup=cgroups[0], scope_memory_max=SCOPE_MEMORY_MAX, scope_swap_max=0,
    control_sha256=CONTROL_SHA256, decision=pin(decision_path),
    source=pin(Path(__file__)), no_additional_timeout_process=True))
capture = runpy.run_path(helper['path'], run_name='root_c5_accept_selected_capture_nonmain')['capture']
operation = dict(id='ROOT-SELECTED-C5-ACCEPT-OUTER', argv=outer,
                 cwd=str(D), environment=config['environment'],
                 wallSeconds=1900, keptBytesPerStream=4194304)
raw = capture(operation, phase)
# The helper owns exactly one process group. On a capture error, reap only its
# remaining direct children; no global process scan or second service dispatch.
children_path = Path('/proc/self/task') / str(os.getpid()) / 'children'
owed = children_path.read_text().split()
for child in owed:
    try:
        os.killpg(int(child), signal.SIGKILL)
    except ProcessLookupError:
        pass
    try:
        os.waitpid(int(child), 0)
    except ChildProcessError:
        pass
for output in phase.iterdir():
    if output.is_file():
        descriptor = os.open(output, os.O_RDONLY)
        try:
            os.fsync(descriptor)
        finally:
            os.close(descriptor)
durable(phase / 'RESULT.raw.json', raw)
# Root-scope memory/CPU observation after every descendant has exited.
try:
    root_scope = {k: (scope / k).read_text() for k in ('memory.peak', 'memory.events', 'memory.max', 'memory.swap.max', 'cpu.stat')}
    usage = [int(x.split()[1]) for x in root_scope['cpu.stat'].splitlines() if x.startswith('usage_usec ')]
    assert len(usage) == 1
    root_scope_error = None
except Exception as error:
    root_scope, usage, root_scope_error = None, [None], repr(error)
terminal = dict(raw_capture=pin(phase / 'RESULT.raw.json'), actual_exit=raw['actual_exit'],
                elapsed_seconds=raw['elapsed_seconds'], controller_failure=raw['controller_failure'],
                streams=raw['streams'], remaining_direct_children=children_path.read_text().split(),
                direct_children_reaped_after_capture_error=owed,
                root_scope_cgroup=cgroups[0], root_scope_observation=root_scope,
                root_scope_cpu_usage_usec=usage[0], root_scope_observation_error=root_scope_error,
                root_scope_cpu_measured_before='ROOT-TERMINAL write; final write, fsync and exit tail unmeasured',
                financial_acceptance_from_root_capture=False)
durable(phase / 'ROOT-TERMINAL.json', terminal)
raise SystemExit(0 if raw['actual_exit'] == 0 and raw['controller_failure'] is None
                 and not terminal['remaining_direct_children'] and root_scope_error is None
                 and all(row['full_stream_retained'] for row in raw['streams']) else 1)
