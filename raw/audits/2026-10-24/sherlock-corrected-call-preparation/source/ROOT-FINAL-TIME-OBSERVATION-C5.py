"""One CPU-second external observation after the selected C5 Root scope has ended.

Kernel process-start ticks provide a conservative beginning before shell exec;
this observation includes Root's scope start, capture, final fsync, reaping and
process exit. It runs in its own fixed scope only after the Root scope is gone
or empty, so the service cap plus this cap never exceeds the aggregate. The
observer is separate measurement overhead, never financial acceptance.
"""
from pathlib import Path
import hashlib
import json
import os
import resource
import time

D = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-recursion-restart-20261022/native/source46-C1-C5-expected03')
ROOT_UNIT = 'sherlock-source46-real-wallet-accept01-root.scope'
OBSERVER_UNIT = 'sherlock-source46-real-wallet-accept01-observer.scope'
SCOPE_MEMORY_MAX = 536870912
SERVICE_MEMORY_MAX = 15463129088
AGGREGATE = 16000000000
assert SERVICE_MEMORY_MAX + SCOPE_MEMORY_MAX == AGGREGATE
assert resource.getrlimit(resource.RLIMIT_CPU) == (1, 1)
assert resource.getrlimit(resource.RLIMIT_AS) == (268435456, 268435456)
assert resource.getrlimit(resource.RLIMIT_FSIZE) == (134217728, 134217728)
assert resource.getrlimit(resource.RLIMIT_CORE) == (0, 0)
cgroups = [x.split(':', 2)[2] for x in Path('/proc/self/cgroup').read_text().splitlines() if x.startswith('0::')]
assert len(cgroups) == 1 and Path(cgroups[0]).name == OBSERVER_UNIT
scope = Path('/sys/fs/cgroup') / cgroups[0].lstrip('/')
assert str(os.getpid()) in (scope / 'cgroup.procs').read_text().split()
assert (scope / 'memory.max').read_text().strip() == str(SCOPE_MEMORY_MAX)
assert (scope / 'memory.swap.max').read_text().strip() == '0'
decision = json.loads((D / 'ROOT-RESOURCE-DECISION.json').read_text())
own = Path(__file__).read_bytes()
assert decision['external_final_time_observer'] == dict(path=__file__, bytes=len(own), sha256=hashlib.sha256(own).hexdigest())
assert (decision['status'], decision['whole_hold']) == ('ADOPTED_SOURCE46_ACTUAL_WALLET_BOUND_D_CALL01', 1900)
assert (decision['outside_cpu_allowance_seconds'], decision['root_scope_cpu_allowance_seconds'], decision['root_cpu_seconds'], decision['external_final_time_observer_cpu_seconds']) == (50, 49, 10, 1)
assert (decision['aggregatePhysicalBudgetBytes'], decision['serviceMemoryMaxBytes'], decision['observerScopeMemoryMaxBytes'], decision['swapBytes'], decision['externalObserverAddressSpaceBytes']) == (AGGREGATE, SERVICE_MEMORY_MAX, SCOPE_MEMORY_MAX, 0, 268435456)
phase = D / 'root-outer-capture01'
live = json.loads((phase / 'LIVE-ROOT-CONTROLLER.json').read_text())
terminal_raw = (phase / 'ROOT-TERMINAL.json').read_bytes()
terminal = json.loads(terminal_raw)
assert type(live['pid']) is int and live['pid'] > 1
assert live['cpu_limit'] == [10, 10] and live['control_sha256'] == decision['configuration_sha256']
assert terminal['remaining_direct_children'] == []
# Mutual non-overlap: the Root scope is this scope's sibling and is gone or empty.
assert Path(live['cgroup']).name == ROOT_UNIT and live['cgroup'] == terminal['root_scope_cgroup']
root_scope = Path('/sys/fs/cgroup') / live['cgroup'].lstrip('/')
assert root_scope.parent == scope.parent
try:
    root_events = (root_scope / 'cgroup.events').read_text().splitlines()
except FileNotFoundError:
    root_events = None
assert root_events is None or 'populated 0' in root_events
try:
    current = (Path('/proc') / str(live['pid']) / 'stat').read_text()
except FileNotFoundError:
    current = None
assert current is None or current.rsplit(')', 1)[1].split()[19] != live['start']
ticks = os.sysconf('SC_CLK_TCK')
upper = time.clock_gettime(time.CLOCK_BOOTTIME) - int(live['start']) / ticks + 1 / ticks
usage = terminal['root_scope_cpu_usage_usec']
record = dict(schema='root.external-c5-accept-argv-time/1',
              original_root_identity_gone=True, root_scope_gone_or_empty=True, root_scope_events=root_events,
              pid=live['pid'], start=live['start'],
              elapsed_upper_bound_seconds=upper, whole_limit_seconds=1900,
              within_whole_limit=0 <= upper <= 1900,
              root_terminal_sha256=hashlib.sha256(terminal_raw).hexdigest(),
              observer_cpu_limit_seconds=1, observer_scope=cgroups[0],
              root_scope_cpu_usage_usec=usage, root_scope_cpu_allowance_seconds=49,
              outside_cpu_allowance_seconds=50,
              within_outside_cpu_allowance=type(usage) is int and 0 <= usage <= 49000000,
              root_scope_cpu_measurement='PARTIAL_OBSERVED_NOT_FULL_BOUND: R1 cpu.stat usage_usec read before ROOT-TERMINAL; R1 final write/fsync/exit tail, systemd-run pre-scope CPU, invoking-agent systemctl checks and systemd managers unmeasured; scope text concerns elapsed time only',
              financial_acceptance=False, scope='Root scope argv including final process exit; conservative kernel-start to external observation upper bound')
with (phase / 'FINAL-EXTERNAL-WHOLE-OBSERVATION.json').open('x') as stream:
    json.dump(record, stream, indent=2)
    stream.write('\n')
    stream.flush()
    os.fsync(stream.fileno())
descriptor = os.open(phase, os.O_RDONLY | os.O_DIRECTORY)
try:
    os.fsync(descriptor)
finally:
    os.close(descriptor)
raise SystemExit(0 if record['within_whole_limit'] and record['within_outside_cpu_allowance'] else 1)
