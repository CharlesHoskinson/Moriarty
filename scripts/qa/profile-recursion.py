#!/usr/bin/env python3
"""Plan capped profiling runs and validate their public accounting sidecars."""
import argparse
from collections import defaultdict
from fractions import Fraction
import difflib
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tarfile
import tomllib

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / 'openspec/changes/recursion-phase-profile/profile.schema.json'
ARCHIVE_SHA256 = '19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc'


def default_archive(environ=os.environ):
    """The published midnight-proofs 0.8.3 archive in this host's cargo registry cache."""
    home = Path(environ['CARGO_HOME']) if environ.get('CARGO_HOME') else Path(environ.get('HOME', '~')) / '.cargo'
    return home / 'registry/cache/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3.crate'


DEFAULT_ARCHIVE = default_archive()
BUDGETS = {'A2': 'a2-linked-children', 'A3': 'a3-parent-k20',
           'A4': 'a4-terminal-d', 'B3': 'b3-full-batch-local', 'GP': 'c0-gp-chain'}
BINARIES = {'A2': 'profile-a2', 'GP': 'profile-gp'}
# Default cache stratum per workload: the A2 child APIs rebuild their PK; the GP chain
# keeps the node PK resident across node steps (sherlock_gp.rs gp_fold_and_key).
CACHE_MODES = {'A2': 'process_cold', 'GP': 'resident_pk'}
# Crates the profile workspace must pin exactly as the root workspace does (task 2.1).
ROOT_PINS = ('midnight-proofs', 'midnight-circuits', 'midnight-zk-stdlib', 'midnight-curves')
GP_STEP_BOUND = 320  # SRC-011: at most 320 owner steps.
GP_S_VALUES = (1, 2, 4, 8, 16)
NS = 1_000_000_000
SECRET_KEY = re.compile(r'witness|seed|wallet|private.?key|secret|challenge(?!_count)|environment|^env$|^scalars?$', re.I)
SECRET_TEXT = re.compile(r'(?:witness(?:_values)?|rng_seed|seed|wallet|private_key|secret|environment|OPENAI_API_KEY)\s*[=:]', re.I)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def vendor_patch(archive, vendor):
    """Canonical unified diff against the checksum-verified published archive."""
    with tarfile.open(archive) as source:
        pristine = {str(Path(m.name).relative_to('midnight-proofs-0.8.3')):
                    source.extractfile(m).read() for m in source.getmembers() if m.isfile()}
    current = {str(p.relative_to(vendor)): p.read_bytes() for p in vendor.rglob('*') if p.is_file()}
    chunks = []
    for name in sorted(pristine.keys() | current.keys()):
        before, after = pristine.get(name, b''), current.get(name, b'')
        if before != after:
            chunks.extend(difflib.unified_diff(before.decode().splitlines(keepends=True),
                                              after.decode().splitlines(keepends=True),
                                              fromfile='a/' + name if name in pristine else '/dev/null',
                                              tofile='b/' + name if name in current else '/dev/null'))
    return ''.join(chunks).encode()


def prepare(root=ROOT, archive=DEFAULT_ARCHIVE):
    root, archive = Path(root), Path(archive)
    require(digest(archive) == ARCHIVE_SHA256, 'archive checksum mismatch')
    directory = root / 'tools/profile'
    manifest = json.loads((directory / 'identity.json').read_text())
    require(manifest['version'] == '0.8.3' and manifest['archive_sha256'] == ARCHIVE_SHA256,
            'archive identity mismatch')
    patch = directory / manifest['patch_file']
    require(patch.resolve().is_relative_to(directory.resolve()), 'patch path escapes workspace')
    require(digest(patch) == manifest['patch_sha256'], 'patch checksum mismatch')
    vendor = directory / 'vendor/midnight-proofs-0.8.3'
    actual = {str(p.relative_to(vendor)): digest(p) for p in sorted(vendor.rglob('*')) if p.is_file()}
    require(actual == manifest['vendor_files'], 'vendor file checksum mismatch')
    require(patch.read_bytes() == vendor_patch(archive, vendor), 'patch does not reproduce vendor changes')
    spec = importlib.util.spec_from_file_location('profile_isolation', ROOT / 'scripts/qa/check-profile-isolation.py')
    isolation = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(isolation)
    errors = isolation.check_isolation(root)
    require(not errors, '; '.join(errors))
    require((directory / 'Cargo.lock').is_file(), 'profiling Cargo.lock missing')
    pin_errors = check_pins(root)
    require(not pin_errors, '; '.join(pin_errors))
    return {'archive_sha256': ARCHIVE_SHA256, 'patch_sha256': manifest['patch_sha256'],
            'cargo_lock_sha256': digest(directory / 'Cargo.lock'), 'isolation': 'pass'}


def check_pins(root=ROOT):
    """The profile workspace pins the root workspace's proof stack and locks exactly that version."""
    root = Path(root)
    errors = []
    def version(spec):
        return spec if isinstance(spec, str) else (spec or {}).get('version')
    try:
        workspace = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['dependencies']
        manifest = tomllib.loads((root / 'tools/profile/Cargo.toml').read_text())['dependencies']
        lock = tomllib.loads((root / 'tools/profile/Cargo.lock').read_text()).get('package', [])
    except (OSError, ValueError, KeyError) as error:
        return [f'profile pins unreadable: {error}']
    for name in ROOT_PINS:
        want, have = version(workspace.get(name)), version(manifest.get(name))
        if not want or not want.startswith('='):
            errors.append(f'Cargo.toml: {name} is not an exact root pin')
            continue
        if have != want:
            errors.append(f'tools/profile/Cargo.toml: {name} {have} != root {want}')
        if not any(p.get('name') == name and p.get('version') == want[1:] for p in lock):
            errors.append(f'tools/profile/Cargo.lock: no {name} {want[1:]}')
    return errors


def ns(value):
    """Convert decimal seconds exactly; never truncate a coverage boundary."""
    result = Fraction(str(value)) * NS
    require(result.denominator == 1, 'time is not an integer number of nanoseconds')
    return result.numerator


def coverage_bounds(total, accounted, unexplained):
    return (total > 0 and unexplained >= 0
            and 100 * abs(accounted + unexplained - total) <= 5 * total
            and 100 * unexplained <= 5 * total)


def reject_secrets(value):
    if isinstance(value, dict):
        for key, item in value.items():
            require(not SECRET_KEY.search(key), 'secret field rejected')
            reject_secrets(item)
    elif isinstance(value, list):
        for item in value:
            reject_secrets(item)
    elif isinstance(value, str):
        require(not SECRET_TEXT.search(value), 'secret material in text rejected')


def validate_comparison(left, right, label='matched'):
    require(label in ('matched', 'version', 'candidate'), 'unknown comparison label')
    if label != 'matched':
        return {'comparison': label, 'phase_transfer': False}
    # Binary hashes and the observation toggle necessarily differ in paired runs.
    omit = {'binary_sha256', 'profile_enabled', 'compile_features', 'artifacts', 'resolved_packages'}
    def identity(s):
        result = {k: v for k, v in s['identity'].items() if k not in omit}
        result['compile_features'] = sorted(f for f in s['identity']['compile_features']
                                            if f.split('/')[-1] != 'recursion-phase-profile')
        result['packages'] = [dict(p, features=sorted(f for f in p['features']
                              if f != 'recursion-phase-profile')) for p in s['identity']['resolved_packages']]
        result['proofs'] = [{k: v for k, v in p.items() if k not in ('coverage', 'root_span_id')}
                            for p in s['proofs']]
        result['host'] = s['host']
        return result
    require(left['identity']['proofs_version'] == right['identity']['proofs_version'] == '0.8.3'
            and identity(left) == identity(right), 'matched-control identity mismatch')
    return {'comparison': label, 'phase_transfer': True}


def validate_memory(rss, start, end):
    require(rss['method'] != 'unavailable' and rss['quality'] != 'unavailable', 'sampler unavailable: invalid profile')
    entry, exit_rss, peak = (rss[k] for k in ('entry_bytes', 'exit_bytes', 'sampled_peak_bytes'))
    require(all(v is not None for v in (entry, exit_rss, peak)), 'RSS boundaries and peak required')
    samples = rss.get('sample_points', [])
    require(len(samples) == rss['samples'], 'RSS sample points required for peak validation')
    seen = set()
    for sample in samples:
        at = sample['at_monotonic_ns']
        require(start < at < end and at not in seen, 'RSS samples must be unique and strictly interior')
        seen.add(at)
    expected = max([entry, exit_rss] + [s['rss_bytes'] for s in samples])
    require(peak == expected, 'sampled peak mismatch')
    require(rss['peak_delta_bytes'] == max(0, peak - entry), 'peak delta mismatch')
    require(rss['net_delta_bytes'] == exit_rss - entry, 'RSS net delta mismatch')
    require(rss['quality'] == ('boundary_only' if len(samples) < 2 else 'sampled_lower_bound'), 'RSS quality mismatch')
    require(not rss.get('certifies_instantaneous', False), 'sampled RSS cannot certify instantaneous peak')


def validate_metrics(p):
    m = p['metrics']
    duration = p['end_monotonic_ns'] - p['start_monotonic_ns']
    require(ns(m['wall_seconds']) == duration, 'phase wall time mismatch')
    if p['accounting']:
        require(m['exclusive_wall_seconds'] is not None and ns(m['exclusive_wall_seconds']) == duration,
                'exclusive wall time mismatch')
    else:
        require(m['exclusive_wall_seconds'] in (None, 0), 'diagnostic has exclusive wall credit')
        require(m['exclusive_cpu_core_seconds'] in (None, 0), 'diagnostic has exclusive CPU credit')
    cpu = m['cpu_core_seconds']
    require((cpu is None) == (m['cpu_scope'] == 'unattributable'), 'CPU scope/counter mismatch')
    require(cpu is not None or bool(m['cpu_null_reason']), 'missing CPU absence reason')
    if p['accounting']:
        require((cpu is None) == (m['exclusive_cpu_core_seconds'] is None), 'exclusive CPU counter mismatch')
        if cpu is not None:
            require(ns(cpu) == ns(m['exclusive_cpu_core_seconds']), 'exclusive CPU mismatch')
    validate_memory(m['rss'], p['start_monotonic_ns'], p['end_monotonic_ns'])
    b = m['bytes']
    counters = {k: v for k, v in b.items() if k.endswith('_bytes')}
    require(not any(v is None for v in counters.values()) or any(b['null_reasons']), 'null byte counters need a reason')
    groups = [('dram_method', ('dram_read_bytes', 'dram_write_bytes')),
              ('storage_method', ('storage_read_bytes', 'storage_write_bytes')),
              ('device_method', ('h2d_bytes', 'd2h_bytes', 'd2d_bytes'))]
    for method, keys in groups:
        if b[method] in ('unavailable', 'cpu_only'):
            require(all(b[k] is None for k in keys), 'unavailable counters must be null, including DRAM')
        else:
            require(all(b[k] is not None for k in keys), 'supported byte counters must be measured')
    if b['logical_coverage'] in ('unavailable', 'not_applicable'):
        require(all(b[k] is None for k in counters if k.startswith('logical_')), 'unavailable logical counters must be null')


def validate_overhead(o):
    require(o['event_count'] < o['event_capacity'] and o['dropped_events'] == 0,
            'full buffer or dropped events: invalid profile')
    upper, mean = o['paired_wall_ratio_upper95'], o['estimated_fraction']
    if upper is None:
        require(o['qualification'] == 'not_run' and o['pair_count'] == 0, 'overhead bound missing')
        return
    require(mean is not None and Fraction(str(mean)) <= Fraction(str(upper)) - 1,
            'overhead mean exceeds upper bound')
    qualifies = o['pair_count'] >= 5 and Fraction(str(upper)) <= Fraction(101, 100)
    require((o['qualification'] == 'pass') == qualifies, 'overhead qualification must use upper bound and five pairs')
    require(o['qualification'] != 'not_run', 'overhead result cannot be not_run with a bound')


def partition(root, phases, coverage):
    total = root['end_monotonic_ns'] - root['start_monotonic_ns']
    selected = sorted((p for p in phases if p['accounting']), key=lambda p: p['start_monotonic_ns'])
    for previous, current in zip(selected, selected[1:]):
        require(previous['end_monotonic_ns'] <= current['start_monotonic_ns'], 'exclusive spans overlap')
    accounted = sum(p['end_monotonic_ns'] - p['start_monotonic_ns'] for p in selected)
    unexplained = ns(coverage['unexplained_other_seconds'])
    require(ns(coverage['measured_wall_seconds']) == total, 'coverage measured total mismatch')
    require(ns(coverage['accounted_exclusive_wall_seconds']) == accounted, 'coverage accounted time mismatch')
    require(coverage_bounds(total, accounted, unexplained), 'coverage outside 5% bounds')
    require(accounted + unexplained == total, 'partition does not reconcile exactly')
    require(ns(coverage['unattributed_wall_seconds']) == unexplained,
            'unattributed time must name the unexplained bucket, not a second sum')
    require(coverage['relative_sum_error'] == 0 and coverage['overlap_error_seconds'] == 0
            and coverage['accepted'], 'coverage claims disagree with partition')
    owners = defaultdict(int)
    rows = []
    cpu_values = []
    for p in selected:
        duration = p['end_monotonic_ns'] - p['start_monotonic_ns']
        owners[p['owner']] += duration
        cpu = p['metrics']['exclusive_cpu_core_seconds']
        cpu_values.append(None if cpu is None else ns(cpu))
        rows.append({'span_id': p['span_id'], 'owner': p['owner'], 'phase': p['phase'],
                     'wall_ns': duration, 'fraction': {'numerator': duration, 'denominator': total}})
    reported = coverage['owner_shares']
    require(len({s['owner'] for s in reported}) == len(reported), 'duplicate owner shares')
    require({s['owner']: ns(s['exclusive_seconds']) for s in reported} == dict(owners), 'owner shares mismatch')
    for s in reported:
        require(abs(Fraction(str(s['fraction_of_named_scope'])) - Fraction(owners[s['owner']], total)) <= Fraction(1, 10**12),
                'owner fraction mismatch')
    rows.append({'span_id': None, 'owner': 'unexplained', 'phase': 'unexplained',
                 'wall_ns': unexplained, 'fraction': {'numerator': unexplained, 'denominator': total}})
    cpu = None if unexplained or any(c is None for c in cpu_values) else sum(cpu_values)
    root_cpu = root['metrics']['cpu_core_seconds']
    if cpu is not None and root_cpu is not None:
        measured = ns(root_cpu)
        require(100 * abs(cpu - measured) <= 5 * measured, 'all-worker CPU reconciliation failed')
    return {'root_span_id': root['span_id'], 'total_ns': total, 'partition': rows, 'cpu_core_ns': cpu}



# Stable IDs emitted by the vendored profile::Phase enum. Only public metadata
# crosses this boundary; raw traces are not qualified schema sidecars.
EVENT_PHASES = {
    1: ('command', 'command'), 2: ('key_load', 'keys'), 3: ('setup_pk', 'keys'),
    4: ('prove', 'other'), 5: ('verify', 'verification'),
    6: ('instance_preparation', 'other'), 7: ('synthesis', 'witness'),
    8: ('resolution_blinding', 'witness'), 9: ('advice_commitments', 'commitment'),
    10: ('lookup_permutation', 'lookup'), 11: ('lookup_products', 'lookup'),
    12: ('permutation_products', 'permutation'), 13: ('vanishing', 'quotient'),
    14: ('quotient', 'quotient'), 15: ('quotient_commitments', 'commitment'),
    16: ('evaluations', 'other'), 17: ('multiopen', 'multiopen'),
    18: ('commitment_msm', 'commitment'), 19: ('transcript', 'transcript'),
    20: ('fixed_commitments', 'commitment'), 21: ('permutation_commitments', 'commitment'),
    22: ('ntt', 'other'), 23: ('trash', 'other'), 24: ('advice_ntt', 'witness'),
}


TRANSCRIPT_PHASE, VERIFY_PHASE = 19, 5
HANDLED_REASON = ('Handled returned error: a transcript read failed inside an interval that itself '
                  'succeeded, the workload exited 0 and every verifier and negative control passed; '
                  'raw status 2 retained.')
EXPECTED_VERIFY_REASON = ('Verifier call that returned the expected rejection inside a successful '
                          'negative-control interval; raw status 2 retained.')


def handled_returned_errors(raw, events, by_id, chains, expected):
    """Returned-error intervals that do not invalidate the observer, and their counts.

    The in-circuit verifier parses absent proof bytes during keygen, cost modeling and the
    genesis step; each read returns an error that its caller handles. Such a transcript
    interval (phase 19, status 2) is tolerated only when every ancestor succeeded (status 1),
    the workload exited 0 and the stock verifier and negative controls passed. A verify
    interval (phase 5, status 2) is tolerated only as an ancestor of an expected rejection
    inside a successful verify interval. Status 0/3 and status 2 in any other phase stay
    invalidating. Every tolerated event is counted per phase and per context."""
    workload = raw.get('workload') or {}
    verification = workload.get('verification') or {}
    if not (workload.get('exit') == 0 and verification.get('stock_verifier') == 'pass'
            and verification.get('negative_controls') == 'pass'):
        return {}, dict(tolerated=0, by_phase={}, by_context={}, expected_rejections=len(expected))
    tolerated = {}
    for event_id in expected:
        for ancestor in chains[event_id]:
            a = by_id[ancestor]
            if (a['phase_id'] == VERIFY_PHASE and a['status'] == 2
                    and all(by_id[x]['status'] == 1 for x in chains[ancestor])
                    and any(by_id[x]['phase_id'] == VERIFY_PHASE for x in chains[ancestor])):
                tolerated[ancestor] = EXPECTED_VERIFY_REASON
    for e in events:
        if (e['status'] == 2 and e['phase_id'] == TRANSCRIPT_PHASE and e['id'] not in expected
                and all(by_id[a]['status'] == 1 for a in chains[e['id']])):
            tolerated[e['id']] = HANDLED_REASON
    roots = {e['id'] for e in events if e['parent_id'] == 0 and e['phase_id'] == 1}
    by_phase, by_context = defaultdict(int), defaultdict(int)
    for event_id in tolerated:
        e = by_id[event_id]
        by_phase[EVENT_PHASES[e['phase_id']][0]] += 1
        if e['parent_id'] == 0:
            context = 'outside_command'
        elif e['parent_id'] in roots:
            context = 'command'
        else:
            top = next((by_id[a] for a in chains[event_id] if by_id[a]['parent_id'] in roots), None)
            context = 'outside_command' if top is None else EVENT_PHASES[top['phase_id']][0]
        by_context[context] += 1
    return tolerated, dict(tolerated=len(tolerated), by_phase=dict(by_phase),
                           by_context=dict(by_context), expected_rejections=len(expected))


def map_events(raw, invocation_id):
    """Map integer intervals to schema phase fragments, retaining invalid traces.

    Each inclusive event becomes a diagnostic. Accounting events additionally
    produce disjoint residual children. A nested MSM therefore has one owner;
    overlapping worker diagnostics are never subtracted or credited.
    """
    reject_secrets(raw)
    fields = {'schema', 'event_capacity', 'dropped_events', 'timer_failures',
              'reentrant_events', 'profile_valid', 'events', 'flush_wall_ns'}
    optional = {'observation', 'workload', 'sidecar_flush_wall_ns'}
    require(fields <= set(raw) <= fields | optional and raw['schema'] == 'recursion.phase-events/1',
            'unknown event envelope')
    require(type(raw['profile_valid']) is bool, 'invalid observer flag')
    for key in fields - {'schema', 'profile_valid', 'events'}:
        require(type(raw[key]) is int and raw[key] >= 0, 'invalid event counter')
    events = raw['events']
    require(isinstance(events, list) and len(events) <= raw['event_capacity'], 'invalid event capacity')
    by_id = {}
    event_fields = {'id', 'parent_id', 'phase_id', 'accounting', 'start_ns', 'end_ns', 'thread_id', 'status', 'advice_phase', 'circuit_index'}
    for event in events:
        require(set(event) == event_fields and all(type(v) is int and v >= 0 for v in event.values()),
                'invalid integer event')
        require(event['id'] > 0 and event['id'] not in by_id, 'duplicate event identity')
        require(event['phase_id'] in EVENT_PHASES and event['accounting'] in (0, 1)
                and event['status'] in (0, 1, 2, 3), 'unknown event code')
        by_id[event['id']] = event
    chains = {}
    for event in events:
        chain, parent = set(), event['parent_id']
        while parent:
            require(parent in by_id and parent not in chain and parent != event['id'], 'invalid event parent')
            chain.add(parent)
            ancestor = by_id[parent]
            require(ancestor['thread_id'] == event['thread_id'], 'cross-thread event parent')
            # An unfinished/error timer may lack an end; preserve it without credit.
            if event['end_ns'] and ancestor['end_ns']:
                require(ancestor['start_ns'] <= event['start_ns'] <= event['end_ns'] <= ancestor['end_ns'],
                        'event outside parent')
            parent = ancestor['parent_id']
        chains[event['id']] = chain
    observation = raw.get('observation')
    expected = set(observation.get('expected_rejections', [])) if isinstance(observation, dict) else set()
    for event_id in expected:
        require(event_id in by_id and by_id[event_id]['status'] == 2 and by_id[event_id]['phase_id'] == 19
                and any(by_id[parent]['phase_id'] == 5 and by_id[parent]['status'] == 1
                        for parent in chains[event_id])
                and (not raw['profile_valid'] or (raw.get('workload', {}).get('exit') == 0
                     and raw.get('workload', {}).get('verification', {}).get('negative_controls') == 'pass')),
                'expected rejection requires a successful verifier control')
    tolerated, returned_errors = handled_returned_errors(raw, events, by_id, chains, expected)
    # The raw flag (vendored Recorder::snapshot, then Collector::accepts in tools/profile) is all
    # or nothing over the same inputs this reducer sees: lock poison (it then exports no events),
    # capacity, drop/timer/reentrancy counters, event status, sampler health, workload success and
    # the negative controls. Every one of them is re-checked below, so a false raw flag is
    # overridden only when the tolerated returned errors are the sole failed events.
    raw_ok = raw['profile_valid'] or (bool(tolerated) and bool(events)
                                      and raw.get('workload', {}).get('exit') == 0)
    observer_valid = (raw_ok and len(events) < raw['event_capacity']
                      and not any(raw[k] for k in ('dropped_events', 'timer_failures', 'reentrant_events'))
                      and all((e['status'] == 1 or e['id'] in expected or e['id'] in tolerated) and e['end_ns'] > e['start_ns'] for e in events))
    schema = json.loads(SCHEMA.read_text())
    reason = 'No boundary counter observations supplied for this interval.'
    measurements = {}
    if observation is not None:
        require(isinstance(observation, dict) and type(observation.get('sampler_valid')) is bool,
                'invalid sampler observation')
        observer_valid = observer_valid and observation['sampler_valid']
        for item in observation.get('measurements', []):
            key = (item['event_id'], item['start_ns'], item['end_ns'])
            require(key not in measurements and key[0] in by_id, 'duplicate or unknown measurement')
            event = by_id[key[0]]
            require(event['start_ns'] <= key[1] <= key[2] <= event['end_ns'],
                    'measurement outside event')
            measurements[key] = item['metrics']

    def phase(event, start, end, span_id, parent, accounting=False):
        name, owner = EVENT_PHASES[event['phase_id']]
        duration = max(0, (end or start) - start) / NS
        byte_fields = schema['$defs']['bytes']['properties']
        metrics = dict(wall_seconds=duration, exclusive_wall_seconds=duration if accounting else None,
                       cpu_core_seconds=None, exclusive_cpu_core_seconds=None,
                       cpu_scope='unattributable', cpu_null_reason=reason,
                       rss=dict(entry_bytes=None, exit_bytes=None, sampled_peak_bytes=None,
                                peak_delta_bytes=None, net_delta_bytes=None, samples=0,
                                max_sample_gap_seconds=None, method='unavailable', quality='unavailable',
                                reason=reason),
                       bytes={**{k: None for k in byte_fields if k.endswith('_bytes')},
                              'logical_coverage': 'unavailable', 'storage_method': 'unavailable',
                              'dram_method': 'unavailable', 'device_method': 'cpu_only',
                              'null_reasons': [reason], 'scope': 'exclusive' if accounting else 'diagnostic',
                              'notes': 'No hardware traffic is inferred from wall time.'},
                       device=dict(status='not_applicable', reason='CPU observer', intervals=[],
                                   busy_union_seconds=None, live_peak_bytes=None,
                                   reserved_peak_bytes=None, peak_method='not_applicable'))
        measured = measurements.get((event['id'], start, end))
        if measured is not None:
            require(set(measured) == {'cpu_core_seconds', 'cpu_null_reason', 'cpu_scope', 'rss', 'bytes'},
                    'unknown measured counter')
            metrics.update(measured)
            metrics['bytes'] = dict(measured['bytes'], scope='exclusive' if accounting else 'diagnostic')
            metrics['exclusive_cpu_core_seconds'] = metrics['cpu_core_seconds'] if accounting else None
            # Validate only complete, available measurements. Missing counters
            # remain visible in invalid raw traces instead of becoming zeros.
            validator = Draft202012Validator({'$ref': '#/$defs/metrics', '$defs': schema['$defs']})
            require(not list(validator.iter_errors(metrics)), 'invalid measured metrics schema')
            if metrics['rss']['quality'] != 'unavailable':
                validate_memory(metrics['rss'], start, end)
        return dict(span_id=span_id, parent_span_id=parent, invocation_id=invocation_id,
                    pid=observation.get('pid', 0) if observation else 0,
                    thread_id=event['thread_id'], parallel_group_id=None,
                    phase=name, owner=owner, accounting=accounting,
                    start_monotonic_ns=start, end_monotonic_ns=end,
                    completed=event['status'] != 0 and end is not None,
                    source_file='tools/profile/vendor/midnight-proofs-0.8.3/src/profile.rs',
                    source_symbol='profile::Recorder::span', source_line=0,
                    dimensions={**{k: None for k in schema['$defs']['dimensions']['required']},
                                **{k: event[k] if event[k] != 2**64 - 1 else None
                                   for k in ('advice_phase', 'circuit_index')}},
                    metrics=metrics, status=('ok' if event['status'] == 1 or event['id'] in expected or event['id'] in tolerated else
                                             'incomplete' if event['status'] == 0 else 'failed'),
                    reason=('Expected verifier rejection; raw returned-error status retained.' if event['id'] in expected
                            else tolerated[event['id']] if event['id'] in tolerated else
                            None if event['status'] == 1 else 'Observer retained an unfinished or failed interval.'))

    phases = [phase(e, e['start_ns'], e['end_ns'] or None, str(e['id']),
                    str(e['parent_id']) if e['parent_id'] else None) for e in events]
    if observer_valid:
        # Domain transforms launched by keygen's Rayon workers have no TLS
        # parent. Assign diagnostics to the innermost joined accounting interval
        # that contains them; the joined interval still owns their entire cost.
        for event, mapped in zip(events, phases):
            if event['phase_id'] == 22:
                owners = [e for e in events if e['accounting']
                          and e['start_ns'] <= event['start_ns'] <= event['end_ns'] <= e['end_ns']]
                if owners:
                    owner = min(owners, key=lambda e: (e['end_ns'] - e['start_ns'], -len(chains[e['id']])))
                    mapped['owner'] = EVENT_PHASES[owner['phase_id']][1]
                    if not event['parent_id']:
                        mapped['parent_span_id'] = str(owner['id'])
                        mapped['parallel_group_id'] = 'joined-' + str(owner['id'])
    if observer_valid:
        accounting_events = [e for e in events if e['accounting']]
        for e in accounting_events:
            # Union all nested accounting intervals; descendants of an already
            # removed child must not be subtracted again.
            cuts = sorted((c['start_ns'], c['end_ns']) for c in accounting_events
                          if e['id'] in chains[c['id']])
            cursor, ordinal = e['start_ns'], 0
            for start, end in cuts + [(e['end_ns'], e['end_ns'])]:
                if cursor < start:
                    phases.append(phase(e, cursor, start, f"{e['id']}:exclusive:{ordinal}",
                                        str(e['id']), True))
                    ordinal += 1
                cursor = max(cursor, end)
        exclusive = sorted((p for p in phases if p['accounting']), key=lambda p: p['start_monotonic_ns'])
        require(all(a['end_monotonic_ns'] <= b['start_monotonic_ns'] for a, b in zip(exclusive, exclusive[1:])),
                'exclusive event overlap')
        if not any(e['phase_id'] == 20 for e in events):
            for e in events:
                if e['phase_id'] == 4:
                    absent = phase(dict(e, phase_id=20), e['start_ns'], e['start_ns'],
                                   f"{e['id']}:fixed-not-applicable", str(e['id']))
                    absent.update(status='not_applicable', reason='No fixed commitment batch ran in this invocation.')
                    phases.append(absent)
    return dict(schema='recursion.phase-events-mapped/1', invocation_id=invocation_id,
                observer_valid=observer_valid, profile_valid=False,
                invalid_reasons=(([] if observation else [reason]) + ['Full workload identity, coverage and qualification are not established by raw event traces.']
                                 + ([] if observer_valid else ['Event buffer, timer or operation failed.'])),
                phases=phases, overhead=dict(event_count=len(events), event_capacity=raw['event_capacity'],
                                            dropped_events=raw['dropped_events'],
                                            flush_wall_seconds=raw['flush_wall_ns'] / NS,
                                            qualification='not_run'),
                returned_errors=returned_errors,
                observation_summary=({k: v for k, v in observation.items() if k != 'measurements'}
                                     if observation else None),
                workload=raw.get('workload'),
                notes='Schema phase fragments only. pid=0 and source_line=0 mean uncollected; no qualified claim.')


def validate_sidecar(sidecar):
    reject_secrets(sidecar)
    validator = Draft202012Validator(json.loads(SCHEMA.read_text()), format_checker=FormatChecker())
    errors = list(validator.iter_errors(sidecar))
    require(not errors, 'schema validation failed' + (': ' + '/'.join(map(str, errors[0].absolute_path)) if errors else ''))
    require(sidecar['exit'] == 0, 'failed command profile')
    require(sidecar['summary']['profile_valid'] and not sidecar['summary']['invalid_reasons'], 'invalid profile')
    packages = [p for p in sidecar['identity']['resolved_packages'] if p['name'] == 'midnight-proofs']
    require(len(packages) == 1 and packages[0]['version'] == '0.8.3', 'resolved package identity mismatch')
    verification = sidecar['verification']
    require(verification['protocol_unchanged'] and verification['official_srs_authenticated']
            and verification['terminal_blake2b_preserved'], 'protocol or official SRS identity changed')
    require(verification['stock_verifier'] == 'pass' and verification['negative_controls'] == 'pass'
            and verification['installed_ledger_verifier'] != 'fail'
            and verification['byte_differential'] != 'fail', 'proof verification evidence failed or missing')
    validate_overhead(sidecar['overhead'])
    phases = sidecar['phases']
    by_id = {p['span_id']: p for p in phases}
    require(len(by_id) == len(phases), 'duplicate interval identity')
    require(len({p['proof_id'] for p in sidecar['proofs']}) == len(sidecar['proofs']), 'duplicate proof identity')
    roots = [p for p in phases if p['parent_span_id'] is None]
    require(len(roots) == 1 and roots[0]['owner'] == 'command' and not roots[0]['accounting'],
            'one diagnostic command root required')
    command = roots[0]
    ancestors = {}
    for p in phases:
        require(p['completed'] and p['status'] in ('ok', 'not_applicable')
                and p['end_monotonic_ns'] is not None, 'incomplete or failed phase')
        require(p['invocation_id'] == sidecar['invocation_id'], 'phase invocation mismatch')
        if p['status'] == 'not_applicable':
            require(not p['accounting'] and bool(p['reason']), 'not applicable phase cannot earn credit')
            require(p['start_monotonic_ns'] == p['end_monotonic_ns'] and p['metrics']['wall_seconds'] == 0,
                    'not applicable phase must have zero cost')
        else:
            require(p['start_monotonic_ns'] < p['end_monotonic_ns'], 'phase interval must be positive')
            validate_metrics(p)
        chain, parent = set(), p['parent_span_id']
        while parent is not None:
            require(parent in by_id and parent not in chain and parent != p['span_id'], 'invalid parent or cycle')
            chain.add(parent)
            ancestor = by_id[parent]
            require(ancestor['end_monotonic_ns'] is not None
                    and ancestor['start_monotonic_ns'] <= p['start_monotonic_ns']
                    and p['end_monotonic_ns'] <= ancestor['end_monotonic_ns'], 'interval outside parent')
            parent = ancestor['parent_span_id']
        ancestors[p['span_id']] = chain
    command_partition = partition(command, phases, sidecar['summary']['coverage'])
    require(ns(sidecar['summary']['wall_seconds']) == command_partition['total_ns'], 'command wall mismatch')
    summary_cpu = sidecar['summary']['cpu_core_seconds']
    root_cpu = command['metrics']['cpu_core_seconds']
    require((summary_cpu is None) == (root_cpu is None), 'command CPU absence mismatch')
    if summary_cpu is not None:
        require(ns(summary_cpu) == ns(root_cpu), 'command CPU total mismatch')
    proofs = []
    proof_roots = set()
    proof_intervals = []
    for proof in sidecar['proofs']:
        root_id = proof['root_span_id']
        require(root_id in by_id and root_id not in proof_roots and root_id != command['span_id'], 'invalid proof root')
        proof_roots.add(root_id)
        root = by_id[root_id]
        require(not root['accounting'], 'proof root must be diagnostic')
        start, end = root['start_monotonic_ns'], root['end_monotonic_ns']
        require(all(end <= other_start or other_end <= start for other_start, other_end in proof_intervals),
                'proof intervals overlap and could duplicate process CPU')
        proof_intervals.append((start, end))
        spans = [p for p in phases if root_id in ancestors[p['span_id']]]
        result = partition(root, spans, proof['coverage'])
        result.update(proof_id=proof['proof_id'], shape=proof['shape'], cache_mode=proof['cache_mode'])
        proofs.append(result)
    return {'run': sidecar['run'], 'invocation_id': sidecar['invocation_id'],
            'command': command_partition, 'proofs': proofs,
            'instantaneous_memory_certified': False,
            'bandwidth_supported': all(p['metrics']['bytes']['dram_method'] == 'hardware_pmu'
                                       for p in phases if p['accounting']),
            'overhead_qualification': sidecar['overhead']['qualification']}


def reject_overlapping_credit(rows):
    used = set()
    for row in rows:
        spans = set(row['spans'])
        require(not used.intersection(spans), 'overlapping gain credit')
        used.update(spans)


def decide_profiles(sidecars, reports=None):
    """Rows C1-C14. `reports` comes from `raw_reports` for raw traces; otherwise every sidecar
    must pass `validate_sidecar` (a qualified recursion.phase-profile/1 sidecar)."""
    if reports is None:
        reports = [validate_sidecar(s) for s in sidecars]
    rows = []
    reasons = {1: 'No validated fused-terminal candidate row count and same-workload comparison.',
               2: 'No mono16 or 2x8 candidate with a matched 0.8.3 baseline.',
               5: 'No candidate scheduler, GPU service or finalized batch throughput evidence.',
               10: 'Needs matched cold/resident-PK proofs with cache-hit counters; wallet sync is a separate scope.'}
    # One named proof is the denominator. Never pool different shapes or strata.
    available = [(s, r, p) for s, r in zip(sidecars, reports) if s['id'] != 'candidate'
                 for p in r['proofs']]
    selected = available[0] if available else None
    credited = []
    for number in range(1, 11):
        row = {'row': f'C{number}', 'status': 'unresolved', 'metric': None, 'measured_gain': None}
        if number == 8:
            row.update(status='inapplicable', reason='0.8.3 always uses BLST; no 0.8.2 dispatch backport gain applies.')
        elif number in reasons:
            row['reason'] = reasons[number]
        elif selected is None:
            row['reason'] = 'No eligible named proof for the phase denominator.'
        else:
            sidecar, _, proof = selected
            phase_rows = proof['partition']
            by_id = {p['span_id']: p for p in sidecar['phases']}
            def msm(p):
                return 'msm' in p['phase'].lower()
            def in_multiopen(p):
                span = by_id[p['span_id']]
                while True:
                    if span['owner'] == 'multiopen':
                        return True
                    parent = span['parent_span_id']
                    if parent is None:
                        return False
                    span = by_id[parent]
            owners = {4: 'witness', 6: 'lookup', 7: 'quotient', 9: 'multiopen'}
            spans = [p for p in phase_rows if p['span_id'] is not None
                     and (msm(p) if number == 3 else p['owner'] == owners[number] and not msm(p))]
            if not spans:
                row['reason'] = 'No separately owned spans for this stage in the selected proof.'
            else:
                numerator = sum(p['wall_ns'] for p in spans)
                denominator = proof['total_ns']
                ids = [p['span_id'] for p in spans]
                row.update(status='arithmetic_only', metric={
                    'name': {3:'commitment_msm_share', 4:'witness_share', 6:'lookup_share',
                             7:'quotient_share', 9:'multiopen_non_msm_share'}[number],
                    'run': sidecar['run'], 'invocation_id': sidecar['invocation_id'],
                    'proof_id': proof['proof_id'], 'cache_mode': proof['cache_mode'],
                    'numerator_ns': numerator, 'denominator_ns': denominator, 'spans': ids,
                    'ideal_speedup_bound': {'numerator': denominator, 'denominator': denominator - numerator}},
                    reason='Measured phase share; no candidate gain confirmed.')
                if number == 9:
                    inclusive = [p for p in phase_rows if p['span_id'] and in_multiopen(p)]
                    shared = [p['span_id'] for p in inclusive if msm(p)]
                    row['metric']['shared_msm_spans'] = shared
                    row['metric']['inclusive_multiopen_ns'] = sum(p['wall_ns'] for p in inclusive)
                credited.append({'row': row['row'], 'spans': ids})
        rows.append(row)
    reject_overlapping_credit(credited)
    rows += lever_rows(sidecars, reports)
    return {'schema': 'recursion.phase-decisions/1', 'rows': rows,
            'selection': 'First eligible baseline proof in input order; no cross-proof share transfer.'}


def is_msm(span):
    return 'msm' in span['phase'].lower()


# k=19 halves every phase proportional to the domain (research order.md table 2.5):
# quotient, the commitment MSMs, multiopen, lookup and permutation products.
DOMAIN_OWNERS = ('quotient', 'multiopen', 'lookup', 'permutation')


def lever_rows(sidecars, reports):
    """Rows C11-C14 (threads, k=19, S, GPU MSM) from named GP node proofs (order.md table 2.5).

    Each metric is a phase share or an Amdahl bound of one lever, not a composable gain: the
    levers share spans (an MSM is both domain-proportional and GPU-eligible), so the rows are
    never summed and are not entered in the overlapping-credit check of C1-C10."""
    nodes, deciders = {}, {}
    for sidecar, report in zip(sidecars, reports):
        if sidecar['id'] == 'candidate':
            continue
        threads = sidecar['host'].get('rayon_threads')
        for proof in report['proofs']:
            target = {'node': nodes, 'run_decider': deciders}.get(proof['shape'])
            if target is not None and threads is not None:
                target.setdefault(threads, (sidecar, proof))
    def base(name, sidecar, proof, spans):
        return {'name': name, 'run': sidecar['run'], 'invocation_id': sidecar['invocation_id'],
                'proof_id': proof['proof_id'], 'proof_shape': proof['shape'], 'cache_mode': proof['cache_mode'],
                'rayon_threads': sidecar['host']['rayon_threads'], 'spans': spans, 'composable': False}
    def row(number, status='unresolved', metric=None, reason=None):
        return {'row': f'C{number}', 'status': status, 'metric': metric, 'measured_gain': None, 'reason': reason}
    def share(number, name, select, bound):
        if 20 not in nodes:
            return row(number, reason='No named GP node proof at RAYON_NUM_THREADS=20 (shape node).')
        sidecar, proof = nodes[20]
        spans = [p for p in proof['partition'] if p['span_id'] is not None and select(p)]
        if not spans:
            return row(number, reason='No separately owned spans for this lever in the node proof.')
        numerator, total = sum(p['wall_ns'] for p in spans), proof['total_ns']
        metric = base(name, sidecar, proof, [p['span_id'] for p in spans])
        metric.update(numerator_ns=numerator, denominator_ns=total)
        metric.update(bound(numerator, total))
        return row(number, 'arithmetic_only', metric, 'Measured phase share; no candidate gain confirmed.')

    out = []
    # C11 threads (lever 0): wall(20) <= 0.7 x wall(4) keeps 20; > 0.9 x wall(4) is serial-bound.
    if 4 not in nodes or 20 not in nodes:
        out.append(row(11, reason='Needs GP node proofs at RAYON_NUM_THREADS 4 and 20 (sweep 4/10/20).'))
    else:
        walls = {str(t): proof['total_ns'] for t, (_, proof) in sorted(nodes.items())}
        sidecar, proof = nodes[20]
        w4, w20 = nodes[4][1]['total_ns'], proof['total_ns']
        decision = ('keep_20_threads' if 10 * w20 <= 7 * w4 else
                    'serial_bound' if 10 * w20 > 9 * w4 else 'inconclusive')
        metric = base('node_step_wall_by_threads', sidecar, proof, [proof['root_span_id']])
        metric.update(node_step_wall_ns=walls, decision=decision,
                      invocations={str(t): s['invocation_id'] for t, (s, _) in sorted(nodes.items())})
        cpu = proof['cpu_core_ns']
        if cpu is None and proof.get('root_cpu_core_ns') is not None:
            # Raw traces: the proof root's own process-CPU boundary delta (all workers, plus the
            # RSS sampler thread). Only one proof runs at a time, so no other job shares it.
            cpu = proof['root_cpu_core_ns']
            metric['average_cores_method'] = 'proof root process CPU delta (proc_pid_stat), inclusive'
        metric['average_cores_at_20'] = None if cpu is None else {'numerator': cpu, 'denominator': w20}
        if cpu is None:
            metric['average_cores_null_reason'] = 'Node proof CPU is not fully attributed.'
        if decision == 'serial_bound':
            phases = {p['span_id']: p for p in sidecar['phases']}
            ratios = [(Fraction(ns(phases[p['span_id']]['metrics']['exclusive_cpu_core_seconds']), p['wall_ns']),
                       p['span_id']) for p in proof['partition'] if p['span_id'] is not None and p['wall_ns']
                      and phases[p['span_id']]['metrics']['exclusive_cpu_core_seconds'] is not None]
            metric['lowest_cpu_per_wall_span'] = min(ratios)[1] if ratios else None
        out.append(row(11, 'arithmetic_only', metric, 'Measured thread sweep; the gate G-A sets the thread count.'))
    # C12 k=19: d = domain-proportional share; predicted factor 1 / (1 - d/2) = 2T / (2T - N).
    out.append(share(12, 'domain_proportional_share',
                     lambda p: p['owner'] in DOMAIN_OWNERS or is_msm(p),
                     lambda n, t: {'predicted_k19_factor': {'numerator': 2 * t, 'denominator': 2 * t - n}}))
    # C13 S>1: arithmetic only; the leaf term t_leaf(S) comes from M4.
    if 20 not in nodes:
        out.append(row(13, reason='No named GP node proof at RAYON_NUM_THREADS=20 (shape node).'))
    else:
        sidecar, proof = nodes[20]
        t_node = proof['total_ns']
        metric = base('steps_per_leaf_arithmetic', sidecar, proof, [proof['root_span_id']])
        decider = deciders.get(20) or next(iter(deciders.values()), None)
        metric.update(node_step_ns=t_node, steps=GP_STEP_BOUND,
                      decider_ns=None if decider is None else decider[1]['total_ns'],
                      node_only_ns={str(s): math.ceil(GP_STEP_BOUND / s) * t_node for s in GP_S_VALUES},
                      leaf_seconds=None, target_ns=5760 * NS)  # 0.8 x 7,200 s
        out.append(row(13, metric=metric, reason='S needs t_leaf(S) and leaf k(S) of the universal leaf (M4); '
                       'node_only_ns is a lower bound, the k=19 node time comes from A16.'))
    # C14 GPU MSM: m = MSM share including multiopen MSMs; Amdahl bound 1 / (1 - m).
    out.append(share(14, 'msm_share', is_msm,
                     lambda n, t: {'amdahl_bound': {'numerator': t, 'denominator': t - n} if t > n else None}))
    return out


def _seconds(value_ns):
    return Fraction(value_ns, NS)


def raw_partition(root, spans):
    """partition() over a raw trace: the coverage block is computed, then checked by the
    same rules as a qualified sidecar (exclusive spans disjoint, 5% bounds, CPU reconcile)."""
    total = root['end_monotonic_ns'] - root['start_monotonic_ns']
    owners = defaultdict(int)
    for p in spans:
        if p['accounting']:
            owners[p['owner']] += p['end_monotonic_ns'] - p['start_monotonic_ns']
    accounted = sum(owners.values())
    coverage = dict(measured_wall_seconds=_seconds(total), accounted_exclusive_wall_seconds=_seconds(accounted),
                    unexplained_other_seconds=_seconds(total - accounted),
                    unattributed_wall_seconds=_seconds(total - accounted),
                    relative_sum_error=0, overlap_error_seconds=0, accepted=True,
                    owner_shares=[dict(owner=o, exclusive_seconds=_seconds(v),
                                       fraction_of_named_scope=Fraction(v, total)) for o, v in owners.items()])
    return partition(root, spans, coverage)


def _cpu_ns(value):
    return None if value is None else round(Fraction(str(value)) * NS)


def raw_reports(raw, invocation_id, run):
    """(sidecar view, report) of one observer-valid raw GP trace for `decide`.

    This is the raw-event path: the trace is not a qualified recursion.phase-profile/1 sidecar
    (no schema identity block, overhead not qualified), so every row it feeds stays
    arithmetic_only and the output carries profile_valid false. Proof roots are the caller
    `prove` spans directly under the command, in order: the stand-in leaves first, then the
    workload's proof records (node steps, then the decider)."""
    mapped = map_events(raw, invocation_id)
    require(mapped['observer_valid'], 'observer-invalid raw trace cannot enter decide')
    workload = raw['workload']
    require(workload.get('exit') == 0 and workload.get('profile_enabled') is True, 'raw trace needs a successful on run')
    phases = mapped['phases']
    by_id = {p['span_id']: p for p in phases}
    ancestors = {}
    for p in phases:
        chain, parent = set(), p['parent_span_id']
        while parent is not None:
            chain.add(parent)
            parent = by_id[parent]['parent_span_id']
        ancestors[p['span_id']] = chain
    commands = [p for p in phases if p['phase'] == 'command' and p['parent_span_id'] is None]
    require(len(commands) == 1, 'one command root required')
    command = commands[0]
    proves = sorted((p for p in phases if p['phase'] == 'prove' and p['parent_span_id'] == command['span_id']
                     and not p['accounting']), key=lambda p: p['start_monotonic_ns'])
    records = workload.get('proofs', [])
    chain = workload.get('chain') or {}
    leaves = chain.get('leaves', 0) if chain.get('standin') else 0
    require(len(proves) == leaves + len(records), 'prove spans do not match the workload proof records')
    proofs = []
    for root, record in zip(proves[leaves:], records):
        spans = [p for p in phases if root['span_id'] in ancestors[p['span_id']]]
        result = raw_partition(root, spans)
        result.update(proof_id=f"{record['shape']}-{record.get('step', 0)}", shape=record['shape'],
                      step=record.get('step'), cache_mode=record['cache_mode'],
                      root_cpu_core_ns=_cpu_ns(root['metrics']['cpu_core_seconds']))
        proofs.append(result)
    # Command coverage is reported, not required: the rows use proof partitions only, and the
    # GP harness has no caller span around cost_model / relation validation (task 2.9 finding).
    total = command['end_monotonic_ns'] - command['start_monotonic_ns']
    accounted = sum(p['end_monotonic_ns'] - p['start_monotonic_ns'] for p in phases
                    if p['accounting'] and command['span_id'] in ancestors[p['span_id']])
    command_coverage = dict(total_ns=total, accounted_ns=accounted, unexplained_ns=total - accounted,
                            unexplained=Fraction(total - accounted, total),
                            accepted=coverage_bounds(total, accounted, total - accounted))
    sidecar = dict(id=workload['id'], run=run, invocation_id=invocation_id, host=workload['host'],
                   phases=phases, workload={k: v for k, v in workload.items() if k != 'proofs'})
    report = dict(run=run, invocation_id=invocation_id, command=command_coverage,
                  proofs=proofs, returned_errors=mapped['returned_errors'], observer_valid=True, profile_valid=False)
    return sidecar, report, ancestors


def _union_ns(intervals, start, end):
    """Wall covered by the union of intervals, clipped to [start, end]."""
    total, cursor = 0, start
    for a, b in sorted((max(a, start), min(b, end)) for a, b in intervals):
        if b <= cursor:
            continue
        total += b - max(a, cursor)
        cursor = b
    return total


# prover.md table 3: q = quotient (phases 13, 14) + ntt (22) + advice_ntt (24).
NTT_PHASES = ('vanishing', 'quotient', 'ntt', 'advice_ntt')


def gp_summary(sidecar, report, ancestors):
    """Per-proof walls and the shares d, m, q of each GP proof, plus average cores (raw path).

    d and m use the exclusive partition (C12, C14). q is the wall covered by any quotient,
    vanishing, NTT or advice-NTT interval of the proof (worker NTT diagnostics included); it
    overlaps m, so m_or_q is the union and m + q is never summed."""
    by_id = {p['span_id']: p for p in sidecar['phases']}
    out = []
    for proof in report['proofs']:
        root = by_id[proof['root_span_id']]
        start, end, total = root['start_monotonic_ns'], root['end_monotonic_ns'], proof['total_ns']
        spans = [p for p in sidecar['phases'] if root['span_id'] in ancestors[p['span_id']]
                 and p['end_monotonic_ns'] is not None]
        rows = [r for r in proof['partition'] if r['span_id'] is not None]
        msm = [(by_id[r['span_id']]['start_monotonic_ns'], by_id[r['span_id']]['end_monotonic_ns'])
               for r in rows if is_msm(r)]
        ntt = [(p['start_monotonic_ns'], p['end_monotonic_ns']) for p in spans if p['phase'] in NTT_PHASES]
        d = sum(r['wall_ns'] for r in rows if r['owner'] in DOMAIN_OWNERS or is_msm(r))
        cpu = proof['root_cpu_core_ns']
        out.append(dict(proof_id=proof['proof_id'], shape=proof['shape'], step=proof['step'],
                        wall_ns=total, unexplained_ns=proof['partition'][-1]['wall_ns'],
                        d=Fraction(d, total), m=Fraction(sum(r['wall_ns'] for r in rows if is_msm(r)), total),
                        q=Fraction(_union_ns(ntt, start, end), total),
                        m_or_q=Fraction(_union_ns(msm + ntt, start, end), total),
                        average_cores=None if cpu is None else Fraction(cpu, total)))
    return dict(run=sidecar['run'], rayon_threads=sidecar['host'].get('rayon_threads'),
                command_coverage=report['command'], proofs=out,
                returned_errors=report['returned_errors'])


def _plain(value):
    """Fractions as {numerator, denominator, value} for the JSON report."""
    if isinstance(value, Fraction):
        return dict(numerator=value.numerator, denominator=value.denominator, value=float(value))
    if isinstance(value, dict):
        return {k: _plain(v) for k, v in value.items()}
    if isinstance(value, list):
        return [_plain(v) for v in value]
    return value


def overhead_label(path):
    """The RPP-005 label carried by every raw-path row (task 2.8 owns the qualification)."""
    if path is None:
        return dict(qualification='not_run', label='overhead unqualified (no overhead report given)')
    o = read_sidecar(path)
    require(o.get('schema') == 'recursion.overhead/1', 'not an overhead report')
    estimate, upper = float(o['estimated_fraction']), float(o['paired_wall_ratio_upper95'])
    state = 'qualified' if o['qualification'] == 'pass' else 'unqualified'
    label = (f"overhead {state}, est {estimate:+.1%}, one-sided 95% bound {upper:.3f} "
             f"(task 2.8, run {o['run']})")
    return dict(qualification=o['qualification'], run=o['run'], pair_count=o['pair_count'],
                estimated_fraction=estimate, paired_wall_ratio_upper95=upper, label=label)


def decide_raw(raws, paths, overhead=None):
    """decide over raw on-run traces (the GP C0 path of task 2.9). Off runs carry no events and
    are listed as skipped. Rows stay arithmetic_only; nothing here is a qualified profile."""
    sidecars, reports, summaries, skipped = [], [], [], []
    for raw, path in zip(raws, paths):
        if not (raw.get('workload') or {}).get('profile_enabled'):
            skipped.append(dict(path=str(path), reason='observation off: no phase events'))
            continue
        invocation = 'events-' + digest(path)
        sidecar, report, ancestors = raw_reports(raw, invocation, Path(path).parent.name)
        sidecars.append(sidecar)
        reports.append(report)
        summaries.append(gp_summary(sidecar, report, ancestors))
    require(bool(sidecars), 'no observation-on raw trace supplied')
    require(len({s['invocation_id'] for s in sidecars}) == len(sidecars), 'duplicate invocation identity')
    label = overhead_label(overhead)
    decision = decide_profiles(sidecars, reports)
    for row in decision['rows']:
        row['evidence'] = 'raw_events'
        row['overhead'] = label['label']
    return _plain(dict(decision, evidence=dict(
        source='raw_events', observer_valid=True, profile_valid=False, overhead=label,
        reason='Raw event traces: no qualified recursion.phase-profile/1 sidecar; every share is '
               'arithmetic_only and carries the overhead label.'),
        gp=summaries, skipped=skipped))


def read_sidecar(path):
    def unique(items):
        result = {}
        for key, value in items:
            require(key not in result, 'duplicate JSON key')
            result[key] = value
        return result
    from decimal import Decimal
    return json.loads(Path(path).read_text(), parse_float=Decimal, object_pairs_hook=unique,
                      parse_constant=lambda _: (_ for _ in ()).throw(ValueError('nonfinite JSON number')))


def _json_number(value):
    """read_sidecar parses floats as Decimal; write them back as JSON numbers."""
    from decimal import Decimal
    if isinstance(value, Decimal):
        return int(value) if value == value.to_integral_value() else float(value)
    raise TypeError(f'Object of type {type(value).__name__} is not JSON serializable')


# RPP-005 analysis, fixed before the qualifying runs (task 2.8): for each pair i the ratio
# r_i = wall_on / wall_off of the whole command; the bound is the one-sided 95% Student-t
# upper limit of mean(log r) mapped back by exp. pass iff pairs >= 5 and bound <= 1.01;
# fail iff the point estimate already exceeds 1.01; otherwise inconclusive.
T95_ONE_SIDED = {1: 6.314, 2: 2.920, 3: 2.353, 4: 2.132, 5: 2.015, 6: 1.943, 7: 1.895, 8: 1.860,
                 9: 1.833, 10: 1.812, 11: 1.796, 12: 1.782, 13: 1.771, 14: 1.761, 15: 1.753,
                 16: 1.746, 17: 1.740, 18: 1.734, 19: 1.729, 20: 1.725, 25: 1.708, 30: 1.697}
OVERHEAD_LIMIT = 1.01
OVERHEAD_METHOD = ('paired whole-command wall ratio on/off; one-sided 95% Student-t upper bound of '
                   'mean log ratio, exp back-transform; pass iff >= 5 pairs and bound <= 1.01')


def _t95(df):
    # The largest tabulated df not above df: a conservative (larger) quantile.
    return T95_ONE_SIDED[max(k for k in T95_ONE_SIDED if k <= df)]


def overhead_bound(pairs):
    """pairs: [(wall_off_ns, wall_on_ns), ...] -> RPP-005 overhead block."""
    import math
    require(len(pairs) >= 2 and all(off > 0 and on > 0 for off, on in pairs), 'overhead needs >= 2 positive pairs')
    logs = [math.log(on / off) for off, on in pairs]
    n = len(logs)
    mean = sum(logs) / n
    sd = math.sqrt(sum((x - mean) ** 2 for x in logs) / (n - 1))
    upper = math.exp(mean + _t95(n - 1) * sd / math.sqrt(n))
    point = math.exp(mean)
    if n >= 5 and upper <= OVERHEAD_LIMIT:
        verdict = 'pass'
    elif point > OVERHEAD_LIMIT:
        verdict = 'fail'
    else:
        verdict = 'inconclusive'
    return dict(qualification=verdict, pair_count=n, estimated_fraction=point - 1,
                paired_wall_ratio_upper95=upper, comparison_method=OVERHEAD_METHOD,
                ratios=[on / off for off, on in pairs])


def overhead_pairs(directory):
    """Pair `<stem>-<i>-off.json` with `<stem>-<i>-on.json` raw sidecars of one calibrate run."""
    found = {}
    for path in sorted(Path(directory).glob('*.json')):
        m = re.search(r'-(\d+)-(off|on)\.json$', path.name)
        if not m:
            continue
        raw = read_sidecar(path)
        work = raw.get('workload') or {}
        require(work.get('exit') == 0, f'{path.name}: failed run cannot qualify overhead')
        ver = work.get('verification') or {}
        require(ver.get('stock_verifier', 'pass') == 'pass' and ver.get('negative_controls', 'pass') == 'pass',
                f'{path.name}: verification did not pass')
        found.setdefault(int(m.group(1)), {})[m.group(2)] = int(work['command_wall_ns'])
    require(found and all(set(v) == {'off', 'on'} for v in found.values()), 'every pair needs an off and an on run')
    return [(found[i]['off'], found[i]['on']) for i in sorted(found)]


def write_report(path, report):
    text = json.dumps(report, indent=2, default=_json_number) + '\n'
    if path is None:
        print(text, end='')
        return
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + '.tmp')
    temporary.write_text(text)
    temporary.replace(path)


def planned_commands(args):
    require(re.fullmatch(r'[A-Za-z0-9_.-]+', args.run_id) and args.run_id not in ('.', '..'), 'invalid run ID')
    require(args.pairs >= 5, 'calibration needs at least five pairs')
    def heavy(run_id, limit, hypothesis):
        # Every heavy run needs its own id. Builds take minutes, so they get a wall cap of their own and never
        # the proof budget (a2-linked-children allows 23 s).
        return ['scripts/doctorwatson', 'heavy', '--id', run_id, *limit, '--hypothesis', args.hypothesis or hypothesis]
    if args.cache_mode is None:
        args.cache_mode = CACHE_MODES.get(args.workload, 'process_cold')
    require(args.workload != 'A2' or args.cache_mode == 'process_cold',
            'A2 resident_pk is unsupported: public child APIs rebuild the PK')
    gp = args.workload == 'GP'
    require(gp or (args.threads is None and args.standin_steps is None and args.leaf_dir is None),
            '--threads, --standin-steps and --leaf-dir apply to the GP workload only')
    environment = []
    if gp:
        require(args.cache_mode == 'resident_pk', 'GP keeps the node PK resident; process_cold is unsupported')
        args.threads = 20 if args.threads is None else args.threads
        require(1 <= args.threads <= 1024, 'invalid thread count')
        require(args.leaf_dir is None or args.standin_steps is None, 'choose stand-in leaves or a leaf directory')
        environment = [f'RAYON_NUM_THREADS={args.threads}']
        if args.leaf_dir is not None:
            require(re.fullmatch(r'[A-Za-z0-9_./-]+', args.leaf_dir), 'invalid leaf directory')
            environment.append(f'MORIARTY_GP_LEAF_DIR={args.leaf_dir}')
        else:
            steps = 3 if args.standin_steps is None else args.standin_steps
            require(1 <= steps <= 320, 'invalid stand-in step count')
            environment.append(f'MORIARTY_GP_STANDIN_STEPS={steps}')
    stratum = f'{args.cache_mode}-t{args.threads}' if gp else args.cache_mode
    binary = BINARIES.get(args.workload, 'profile-recursion')
    variants = ('off', 'on') if args.command == 'calibrate' or args.variant == 'both' else (args.variant,)
    commands = []
    for variant in variants:
        build = ['env', 'RUSTC_WRAPPER=', 'CARGO_BUILD_JOBS=4', f'CARGO_TARGET_DIR=tools/profile/target/{variant}',
                 'cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'tools/profile/Cargo.toml',
                 '--bin', binary]
        if variant == 'on':
            build += ['--features', 'recursion-phase-profile']
        commands.append(heavy(f'C0-{args.run_id}-{args.workload}-build-{variant}', ['--max-wall', '3600'],
                              f'Build the {variant} {binary} binary for the C0 {args.workload} profile') + ['--'] + build)
    for pair in range(args.pairs if args.command == 'calibrate' else 1):
        for variant in variants:
            sidecar = f'qa/sherlock/profiles/{args.run_id}/{args.workload}-{stratum}-{pair}-{variant}.json'
            command = [f'tools/profile/target/{variant}/release/{binary}',
                       '--workload', args.workload, '--cache-mode', args.cache_mode, '--sidecar', sidecar]
            if gp:
                command = ['env', *environment, *command, '--threads', str(args.threads)]
            commands.append(heavy(f'C0-{args.run_id}-{args.workload}-{stratum}-{pair}-{variant}',
                                  ['--budget', BUDGETS[args.workload]],
                                  f'C0 {args.workload} profile, pair {pair}, observation {variant}')
                            + ['--', *command])
    return commands


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('prepare')
    p.add_argument('--check-only', action='store_true')
    p.add_argument('--archive', type=Path, default=DEFAULT_ARCHIVE)
    for name in ('one', 'calibrate'):
        p = sub.add_parser(name)
        p.add_argument('--workload', choices=BUDGETS, required=True)
        p.add_argument('--pairs', type=int, default=5)
        p.add_argument('--run-id', default='planned')
        p.add_argument('--cache-mode', choices=('process_cold', 'resident_pk'),
                       help='default: resident_pk for GP, process_cold otherwise')
        p.add_argument('--threads', type=int, help='GP: RAYON_NUM_THREADS, recorded in the sidecar host block (default 20)')
        p.add_argument('--standin-steps', type=int, help='GP: stand-in leaves (default 3)')
        p.add_argument('--leaf-dir', help='GP: directory of real proved leaves')
        p.add_argument('--variant', choices=('off', 'on', 'both'), default='both')
        p.add_argument('--hypothesis')
        p.add_argument('--run', action='store_true')
    p = sub.add_parser('overhead', help='RPP-005 bound from the off/on pairs of one calibrate run')
    p.add_argument('--run', dest='source_run', required=True)
    p.add_argument('--output', type=Path)
    for name in ('reduce', 'decide'):
        p = sub.add_parser(name)
        p.add_argument('sidecars', nargs='*', type=Path)
        p.add_argument('--run', dest='source_run', action='append', default=[],
                       help='a run under qa/sherlock/profiles; repeat to combine runs (then --output is required)')
        p.add_argument('--output', type=Path)
        if name == 'decide':
            p.add_argument('--overhead', type=Path,
                           help='raw traces: the RPP-005 overhead.json that labels the unqualified rows')
    args = parser.parse_args(argv)
    try:
        if args.command == 'prepare':
            print(json.dumps(prepare(ROOT, args.archive), indent=2))
        elif args.command in ('one', 'calibrate'):
            commands = planned_commands(args)
            for command in commands:
                print(shlex.join(command))
            if args.run:
                prepare()
                binary = BINARIES.get(args.workload, 'profile-recursion')
                require((ROOT / f'tools/profile/src/bin/{binary}.rs').is_file(),
                        'profiling executable is not implemented for this workload')
                for command in commands:
                    result = subprocess.run(command, cwd=ROOT, check=False)
                    if result.returncode:
                        return result.returncode
        elif args.command == 'overhead':
            require(re.fullmatch(r'[A-Za-z0-9_.-]+', args.source_run)
                    and args.source_run not in ('.', '..'), 'invalid run ID')
            directory = ROOT / 'qa/sherlock/profiles' / args.source_run
            report = dict(schema='recursion.overhead/1', run=args.source_run,
                          **overhead_bound(overhead_pairs(directory)))
            write_report(args.output or directory / 'overhead.json', report)
        else:
            paths = list(args.sidecars)
            output = args.output
            require(len(args.source_run) <= 1 or output is not None, 'several runs need --output')
            for source_run in args.source_run:
                require(re.fullmatch(r'[A-Za-z0-9_.-]+', source_run)
                        and source_run not in ('.', '..'), 'invalid run ID')
                directory = ROOT / 'qa/sherlock/profiles' / source_run
                paths += sorted(p for p in directory.glob('*.json') if p.name not in ('reduce.json', 'decide.json', 'overhead.json'))
                if output is None:
                    output = directory / (args.command + '.json')
            require(bool(paths), 'no sidecars supplied')
            require(output is None or output.resolve() not in [p.resolve() for p in paths], 'cannot overwrite a source sidecar')
            sidecars = [read_sidecar(p) for p in paths]
            raw_events = [s.get('schema') == 'recursion.phase-events/1' for s in sidecars]
            if any(raw_events):
                require(all(raw_events), 'raw events and qualified sidecars cannot be mixed')
                if args.command == 'reduce':
                    report = {'schema': 'recursion.phase-events-mapped/1', 'profiles':
                              [map_events(s, 'events-' + digest(p)) for s, p in zip(sidecars, paths)]}
                else:
                    report = decide_raw(sidecars, paths, args.overhead)
            else:
                require(len({s['invocation_id'] for s in sidecars}) == len(sidecars), 'duplicate invocation identity')
                report = (decide_profiles(sidecars) if args.command == 'decide' else
                          {'schema': 'recursion.phase-partition/1', 'profiles': [validate_sidecar(s) for s in sidecars]})
            write_report(output, report)
        return 0
    except (ValueError, OSError, KeyError, TypeError, tarfile.TarError) as error:
        print(f'profile-recursion: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
