#!/usr/bin/env python3
"""Plan capped profiling runs and validate their public accounting sidecars."""
import argparse
from collections import defaultdict
from fractions import Fraction
import difflib
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tarfile

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / 'openspec/changes/recursion-phase-profile/profile.schema.json'
ARCHIVE_SHA256 = '19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc'
DEFAULT_ARCHIVE = Path('/home/charl/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3.crate')
BUDGETS = {'A2': 'a2-linked-children', 'A3': 'a3-parent-k20',
           'A4': 'a4-terminal-d', 'B3': 'b3-full-batch-local'}
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
    return {'archive_sha256': ARCHIVE_SHA256, 'patch_sha256': manifest['patch_sha256'],
            'cargo_lock_sha256': digest(directory / 'Cargo.lock'), 'isolation': 'pass'}


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


def map_events(raw, invocation_id):
    """Map integer intervals to schema phase fragments, retaining invalid traces.

    Each inclusive event becomes a diagnostic. Accounting events additionally
    produce disjoint residual children. A nested MSM therefore has one owner;
    overlapping worker diagnostics are never subtracted or credited.
    """
    reject_secrets(raw)
    fields = {'schema', 'event_capacity', 'dropped_events', 'timer_failures',
              'reentrant_events', 'profile_valid', 'events', 'flush_wall_ns'}
    require(set(raw) == fields and raw['schema'] == 'recursion.phase-events/1',
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
    observer_valid = (raw['profile_valid'] and len(events) < raw['event_capacity']
                      and not any(raw[k] for k in ('dropped_events', 'timer_failures', 'reentrant_events'))
                      and all(e['status'] == 1 and e['end_ns'] > e['start_ns'] for e in events))
    schema = json.loads(SCHEMA.read_text())
    reason = 'Task 3.4 sampler and counter collection not implemented.'

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
        return dict(span_id=span_id, parent_span_id=parent, invocation_id=invocation_id,
                    pid=0, thread_id=event['thread_id'], parallel_group_id=None,
                    phase=name, owner=owner, accounting=accounting,
                    start_monotonic_ns=start, end_monotonic_ns=end,
                    completed=event['status'] != 0 and end is not None,
                    source_file='tools/profile/vendor/midnight-proofs-0.8.3/src/profile.rs',
                    source_symbol='profile::Recorder::span', source_line=0,
                    dimensions={**{k: None for k in schema['$defs']['dimensions']['required']},
                                **{k: event[k] if event[k] != 2**64 - 1 else None
                                   for k in ('advice_phase', 'circuit_index')}},
                    metrics=metrics, status=('ok' if event['status'] == 1 else
                                             'incomplete' if event['status'] == 0 else 'failed'),
                    reason=None if event['status'] == 1 else 'Observer retained an unfinished or failed interval.')

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
                invalid_reasons=([reason, 'Workload identity and verification evidence must be supplied.']
                                 + ([] if observer_valid else ['Event buffer, timer or operation failed.'])),
                phases=phases, overhead=dict(event_count=len(events), event_capacity=raw['event_capacity'],
                                            dropped_events=raw['dropped_events'],
                                            flush_wall_seconds=raw['flush_wall_ns'] / NS,
                                            qualification='not_run'),
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


def decide_profiles(sidecars):
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
    return {'schema': 'recursion.phase-decisions/1', 'rows': rows,
            'selection': 'First eligible baseline proof in input order; no cross-proof share transfer.'}


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


def write_report(path, report):
    text = json.dumps(report, indent=2) + '\n'
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
    base = ['scripts/doctorwatson', 'heavy', '--id', args.workload, '--budget', BUDGETS[args.workload]]
    if args.hypothesis:
        base += ['--hypothesis', args.hypothesis]
    # Task 3 supplies this executable and its feature. These are plans, not receipts.
    variants = ('off', 'on') if args.command == 'calibrate' else (args.variant,)
    commands = []
    for variant in variants:
        build = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'tools/profile/Cargo.toml',
                 '--bin', 'profile-recursion', '--target-dir', f'tools/profile/target/{variant}']
        if variant == 'on':
            build += ['--features', 'recursion-phase-profile']
        commands.append(base + ['--'] + build)
    for pair in range(args.pairs if args.command == 'calibrate' else 1):
        for variant in variants:
            sidecar = f'qa/sherlock/profiles/{args.run_id}/{args.workload}-{args.cache_mode}-{pair}-{variant}.json'
            commands.append(base + ['--', f'tools/profile/target/{variant}/release/profile-recursion',
                                    '--workload', args.workload, '--cache-mode', args.cache_mode,
                                    '--sidecar', sidecar])
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
        p.add_argument('--cache-mode', choices=('process_cold', 'resident_pk'), default='process_cold')
        p.add_argument('--variant', choices=('off', 'on'), default='on')
        p.add_argument('--hypothesis')
        p.add_argument('--run', action='store_true')
    for name in ('reduce', 'decide'):
        p = sub.add_parser(name)
        p.add_argument('sidecars', nargs='*', type=Path)
        p.add_argument('--run', dest='source_run')
        p.add_argument('--output', type=Path)
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
                require((ROOT / 'tools/profile/src/bin/profile-recursion.rs').is_file(),
                        'task 3 profiling executable is not implemented')
                for command in commands:
                    result = subprocess.run(command, cwd=ROOT, check=False)
                    if result.returncode:
                        return result.returncode
        else:
            paths = list(args.sidecars)
            output = args.output
            if args.source_run:
                require(re.fullmatch(r'[A-Za-z0-9_.-]+', args.source_run)
                        and args.source_run not in ('.', '..'), 'invalid run ID')
                directory = ROOT / 'qa/sherlock/profiles' / args.source_run
                paths += sorted(p for p in directory.glob('*.json') if p.name not in ('reduce.json', 'decide.json'))
                if output is None:
                    output = directory / (args.command + '.json')
            require(bool(paths), 'no sidecars supplied')
            require(output is None or output.resolve() not in [p.resolve() for p in paths], 'cannot overwrite a source sidecar')
            sidecars = [read_sidecar(p) for p in paths]
            raw_events = [s.get('schema') == 'recursion.phase-events/1' for s in sidecars]
            if any(raw_events):
                require(args.command == 'reduce' and all(raw_events),
                        'raw events can only be reduced separately; they are not qualified sidecars')
                report = {'schema': 'recursion.phase-events-mapped/1', 'profiles':
                          [map_events(s, 'events-' + digest(p)) for s, p in zip(sidecars, paths)]}
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
