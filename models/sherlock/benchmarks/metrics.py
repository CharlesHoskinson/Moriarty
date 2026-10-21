#!/usr/bin/env python3
"""Receipt-only benchmark collection and descriptive statistics (stdlib only).

No execution API: reads only explicitly selected local JSON receipts. Hashes
establish local lineage, not producer authenticity, proof validity or authority.
Subprocess resource totals include process startup and wrapper overhead.
"""
import argparse
import copy
from datetime import datetime
from decimal import Decimal, localcontext, DecimalException, Inexact
import hashlib
from fractions import Fraction
import json
import math
from pathlib import Path, PurePosixPath
import re
import statistics
import sys

SCHEMA = 'sherlock-benchmark-sample/1'
STATES = ('success', 'failure', 'censored', 'not_run')
EVIDENCE = ('synthetic', 'host', 'model', 'native', 'stagenet')
PHASES = ('compile.frontend', 'compile.lowering', 'compile.native', 'host.handler', 'host.consumer_link', 'host.reference_execution', 'parameters', 'witness', 'keygen', 'prove', 'local_verify')
METRIC_UNITS = {f'{p}.{suffix}': unit for p in PHASES for suffix, unit in (
    ('wall_seconds', 'seconds'), ('cpu_seconds', 'seconds'), ('peak_rss_bytes', 'bytes'))}
METRIC_UNITS.update({'circuit.k': 'exponent', 'circuit.domain_capacity_rows': 'rows',
    'circuit.used_rows': 'rows', 'circuit.lookup_rows': 'rows',
    **{f'circuit.{c}_columns': 'columns' for c in ('advice', 'fixed', 'instance')},
    **{f'{x}.bytes': 'bytes' for x in ('proof', 'proving_key', 'verifying_key', 'instance', 'artifact', 'transaction.deployment', 'transaction.call')},
    'fee.dust': 'DUST'})
EVENT_PAIRS = (('proof_ready', 'submission'), ('submission', 'ack'), ('ack', 'inclusion'),
               ('inclusion', 'readback'), ('submission', 'readback'))
METRIC_UNITS.update({f'network.{a}_to_{b}_seconds': 'seconds' for a, b in EVENT_PAIRS})
IDENTITY_KEYS = {'source_sha256', 'compiler_sha256', 'ledger_sha256', 'network', 'circuit_sha256', 'profile'}
IDENTITY_OPTIONAL = {'environment_sha256', 'toolchain_sha256', 'parameters_sha256'}
SAMPLE_KEYS = {'schema', 'sample_id', 'workload_id', 'workload_digest_sha256', 'evidence_class',
               'status', 'identity', 'cache_state', 'metrics', 'layers', 'receipts', 'effects', 'events'}
SHA = re.compile(r'[0-9a-f]{64}\Z')
LABEL = re.compile(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,127}\Z')
HOST_FORMATS = {'sherlock-normal-host-run/1', 'sherlock-image-host-run/1', 'sherlock-stream-host-run/1'}
MAX_RECEIPT_BYTES = 16 * 1024 * 1024


def require(condition, message):
    if not condition:
        raise ValueError(message)


def loads_strict(raw, *, decimal_numbers=False):
    """Reject duplicate fields and non-JSON numeric constants before validation."""
    def pairs(items):
        result = {}
        for key, value in items:
            require(key not in result, f'duplicate JSON key: {key}')
            result[key] = value
        return result
    def constant(value):
        raise ValueError(f'nonfinite JSON number: {value}')
    try:
        return json.loads(raw, object_pairs_hook=pairs, parse_constant=constant,
                          parse_float=Decimal if decimal_numbers else float)
    except DecimalException as exc:
        raise ValueError('JSON decimal arithmetic range exceeded') from exc


def read_json_bytes(path):
    """Bound JSON ingestion only; this does not read binary proof/key artifacts."""
    path = Path(path)
    require(path.is_file(), 'JSON input: regular file required')
    require(path.stat().st_size <= MAX_RECEIPT_BYTES, 'JSON input: size exceeds ingestion limit')
    with path.open('rb') as handle:
        raw = handle.read(MAX_RECEIPT_BYTES+1)
    require(len(raw) <= MAX_RECEIPT_BYTES, 'JSON input: size exceeds ingestion limit')
    return raw


def canonical_sha256(document):
    return hashlib.sha256(json.dumps(document, sort_keys=True, separators=(',', ':'), allow_nan=False).encode()).hexdigest()


def _keys(document, required, optional=(), where='object'):
    require(type(document) is dict, f'{where}: expected object')
    require(set(document) >= set(required) and set(document) <= set(required) | set(optional), f'{where}: field set differs')


def _digest(value, where, nullable=False):
    require((nullable and value is None) or (isinstance(value, str) and SHA.fullmatch(value)), f'{where}: expected sha256')


def _number(value, where, integer=False):
    require(type(value) in (int, float), f'{where}: expected numeric value')
    require(value >= 0 and (type(value) is int or math.isfinite(value)), f'{where}: expected finite nonnegative value')
    if integer:
        require(type(value) is int, f'{where}: expected integer count')


def _decimal_number(value, where):
    require(type(value) in (int,float,Decimal), f'{where}: expected numeric value')
    result = value if type(value) is Decimal else Decimal(str(value))
    require(result.is_finite() and result >= 0, f'{where}: expected finite nonnegative value')
    return result


def normalize_number(value, scale=1, *, unit):
    """Exact decimal unit normalization, with no epsilon or silent precision loss.

    Output is JSON int/float. Nonintegral seconds must have a lossless canonical
    decimal float spelling; more precise input needs a producer-declared timer
    resolution or decimal-format schema, never implicit rounding here.
    """
    require(unit in set(METRIC_UNITS.values()), 'normalization: unknown unit')
    left = _decimal_number(value, 'receipt scalar')
    right = _decimal_number(scale, 'scale')
    require(right > 0, 'scale must be positive')
    with localcontext() as context:
        context.prec = max(1,len(left.as_tuple().digits)+len(right.as_tuple().digits))
        context.traps[Inexact] = True
        try:
            result = left * right
        except DecimalException as exc:
            raise ValueError('normalization: decimal arithmetic range exceeded') from exc
    if result == result.to_integral_value():
        return int(result)
    require(unit == 'seconds', 'normalization: count must be an integer')
    converted = float(result)
    require(math.isfinite(converted) and Decimal(str(converted)) == result,
            'normalization: decimal precision cannot be represented losslessly')
    return converted


def receipt_metric_value(raw_json, pointer, scale=1, *, unit):
    """Reproduce an original JSON scalar's normalized value using decimal tokens."""
    require(type(raw_json) in (str,bytes), 'receipt: JSON bytes/string required')
    size = len(raw_json.encode('utf-8')) if type(raw_json) is str else len(raw_json)
    require(size <= MAX_RECEIPT_BYTES, 'receipt: size exceeds ingestion limit')
    value = resolve_pointer(loads_strict(raw_json, decimal_numbers=True), pointer)
    require(value is not None, 'receipt scalar: null/unavailable value')
    return normalize_number(value, scale, unit=unit)


def required_metric_ids(case):
    """One expansion point for flat catalog and optional layer metric declarations."""
    require(type(case) is dict, 'case: expected object')
    result = case.get('required_metrics')
    require(type(result) is list and result, 'required_metrics: nonempty list required')
    result = list(result)
    for name in result:
        split_metric(name)
    layers = case.get('required_layers',[])
    require(type(layers) is list and all(type(x) is str and LABEL.fullmatch(x) for x in layers), 'required_layers: invalid labels')
    require(len(layers)==len(set(layers)), 'required_layers: duplicates')
    if 'required_layer_metrics' in case:
        names = case['required_layer_metrics']
        require(type(names) is list and (layers or not names), 'required_layer_metrics: declared layers required')
        for name in names:
            layer,_ = split_metric(name)
            require(layer is None, 'required_layer_metrics: already prefixed')
        result += [f'layer:{layer}:{name}' for layer in layers for name in names]
    require(len(result)==len(set(result)), 'required_metrics: duplicate metric')
    require(all(split_metric(name)[0] is None or split_metric(name)[0] in layers for name in result),
            'required_metrics: undeclared layer')
    return result


def _path(value):
    require(type(value) is str and value and '\\' not in value, 'receipt path: expected relative POSIX label')
    p = PurePosixPath(value)
    require(not p.is_absolute() and all(x not in ('', '.', '..') for x in value.split('/')), 'receipt path: unsafe component')
    return p


def _pointer_parts(pointer):
    require(type(pointer) is str and (pointer == '' or pointer.startswith('/')), 'pointer: expected RFC6901 JSON pointer')
    require(re.search(r'~(?![01])', pointer) is None, 'pointer: invalid escape')
    return [] if pointer == '' else [x.replace('~1', '/').replace('~0', '~') for x in pointer[1:].split('/')]


def resolve_pointer(document, pointer):
    """RFC6901 object/list lookup. A missing field raises KeyError, never zero."""
    for part in _pointer_parts(pointer):
        if type(document) is dict:
            document = document[part]
        elif type(document) is list:
            require(re.fullmatch(r'0|[1-9][0-9]*', part) is not None, 'pointer: invalid array index')
            try:
                document = document[int(part)]
            except IndexError as exc:
                raise KeyError(pointer) from exc
        else:
            raise KeyError(pointer)
    return document


def split_metric(metric):
    require(type(metric) is str, 'metric: expected string')
    if metric.startswith('layer:'):
        parts = metric.split(':')
        require(len(parts) == 3 and LABEL.fullmatch(parts[1]), 'metric: invalid layer identifier')
        layer, name = parts[1:]
    else:
        layer, name = None, metric
    require(name in METRIC_UNITS, f'unknown metric: {metric}')
    return layer, name


def _flatten(sample):
    result = dict(sample['metrics'])
    for layer, values in sample['layers'].items():
        result.update({f'layer:{layer}:{name}': item for name, item in values.items()})
    return result


def _validate_metric(name, item, receipt_hashes):
    require(name in METRIC_UNITS, f'unknown metric: {name}')
    _keys(item, {'value', 'unit', 'status', 'source', 'reason'}, where=name)
    require(item['unit'] == METRIC_UNITS[name], f'{name}: incorrect unit')
    require(item['status'] in ('measured', 'unavailable', 'censored'), f'{name}: unknown metric status')
    if item['status'] == 'unavailable':
        require(item['value'] is None and item['source'] is None, f'{name}: unavailable must have null value/source')
    else:
        _number(item['value'], name, integer=item['unit'] != 'seconds')
        source = item['source']
        _keys(source, {'receipt_sha256', 'pointer'}, {'scale', 'scope', 'timer_resolution_seconds'}, where=f'{name}: source')
        _digest(source['receipt_sha256'], f'{name}: source.receipt_sha256')
        require(source['receipt_sha256'] in receipt_hashes, f'{name}: source receipt not retained')
        _pointer_parts(source['pointer'])
        if 'scale' in source:
            _number(source['scale'], f'{name}: scale')
            require(source['scale'] > 0, f'{name}: scale must be positive')
    if item['source'] is not None:
        _measurement_metadata(item['source'])
    require(item['reason'] is None or (type(item['reason']) is str and item['reason'].strip()), f'{name}: invalid reason')
    if item['status'] != 'measured':
        require(type(item['reason']) is str and item['reason'].strip(), f'{name}: reason required')


def _measurement_metadata(source):
    if 'scope' in source:
        require(source['scope'] in ('subprocess','process_tree','single_process','rpc_elapsed',
                'circuit_metadata','serialized_artifact','ledger_receipt','unknown'), 'source.scope: unknown scope')
    if source.get('timer_resolution_seconds') is not None:
        _number(source['timer_resolution_seconds'], 'timer_resolution_seconds')
        require(source['timer_resolution_seconds'] > 0, 'timer_resolution_seconds: must be positive')


def _circuit_consistency(values):
    def observed(name):
        metric = values.get(name,{})
        return metric['value'] if metric.get('status') == 'measured' else None
    degree = observed('circuit.k')
    capacity = observed('circuit.domain_capacity_rows')
    if capacity is not None:
        require(capacity > 0 and capacity & (capacity-1) == 0, 'circuit: observed domain capacity must be a positive power of two')
        if degree is not None:
            require(capacity.bit_length()-1 == degree, 'circuit: observed capacity disagrees with k')
    for name in ('circuit.used_rows','circuit.lookup_rows'):
        rows = observed(name)
        if rows is None:
            continue
        if capacity is not None:
            require(rows <= capacity, f'{name}: rows exceed observed capacity')
        if degree is not None:
            # rows <= 2**k without constructing 2**k or imposing a fake k cap.
            require(rows == 0 or (rows-1).bit_length() <= degree, f'{name}: rows exceed k domain')


def _latencies(events):
    times = _event_times(events)
    result = {}
    for a,b in EVENT_PAIRS:
        if a in times and b in times:
            delta = times[b]-times[a]
            microseconds = (delta.days*86400+delta.seconds)*1000000 + delta.microseconds
            result[f'network.{a}_to_{b}_seconds'] = normalize_number(microseconds,Decimal('0.000001'),unit='seconds')
    return result


def _event_times(events):
    require(type(events) is dict and set(events) <= {'proof_ready','submission','ack','inclusion','readback'}, 'events: field set differs')
    parsed = {}
    for key in ('proof_ready','submission','ack','inclusion','readback'):
        raw = events.get(key)
        if raw is None:
            continue
        require(type(raw) is str and ('T' in raw) and raw.endswith(('Z', '+00:00')), f'events.{key}: UTC timestamp required')
        require(re.search(r'\.[0-9]{7,}',raw) is None, f'events.{key}: precision exceeds microseconds')
        try:
            t = datetime.fromisoformat(raw.replace('Z', '+00:00'))
        except ValueError as exc:
            raise ValueError(f'events.{key}: invalid timestamp') from exc
        require(t.tzinfo is not None and t.utcoffset().total_seconds() == 0, f'events.{key}: UTC required')
        parsed[key] = t
    ordered = list(parsed.values())
    require(ordered == sorted(ordered), 'events: chronology invalid')
    return parsed


def validate_sample(document):
    """Validate representation/units/lineage, without authenticating its producer."""
    _keys(document, SAMPLE_KEYS, {'extensions', 'cache_identity_sha256'}, where='sample')
    require(document['schema'] == SCHEMA, 'sample: unsupported schema')
    for key in ('sample_id', 'workload_id'):
        require(type(document[key]) is str and LABEL.fullmatch(document[key]), f'{key}: invalid label')
    _digest(document['workload_digest_sha256'], 'workload_digest_sha256')
    require(document['evidence_class'] in EVIDENCE, 'evidence_class: unknown class')
    require(document['status'] in STATES, 'status: unknown terminal result')
    identity = document['identity']
    _keys(identity, IDENTITY_KEYS, IDENTITY_OPTIONAL, where='identity')
    for key in (IDENTITY_KEYS | IDENTITY_OPTIONAL) - {'network', 'profile'}:
        if key in identity:
            _digest(identity[key], 'identity.'+key, nullable=key != 'source_sha256')
    require(type(identity['profile']) is str and identity['profile'].strip(), 'identity.profile: required')
    require(identity['network'] is None or (type(identity['network']) is str and identity['network'].strip()), 'identity.network: invalid')
    require(document['cache_state'] in ('cold','warm','unknown','not_applicable'), 'cache_state: unknown')
    if 'cache_identity_sha256' in document:
        _digest(document['cache_identity_sha256'], 'cache_identity_sha256', nullable=True)
    if document['cache_state'] == 'warm':
        _digest(document.get('cache_identity_sha256'), 'warm cache_identity_sha256')
    require(type(document['receipts']) is list, 'receipts: expected list')
    hashes = set(); paths = set()
    for receipt in document['receipts']:
        _keys(receipt, {'path','sha256','schema'}, where='receipt')
        _path(receipt['path']); _digest(receipt['sha256'], 'receipt.sha256')
        require(receipt['path'] not in paths, 'duplicate receipt path')
        require(receipt['schema'] is None or type(receipt['schema']) is str, 'receipt.schema: invalid')
        paths.add(receipt['path']); hashes.add(receipt['sha256'])
    require(type(document['metrics']) is dict and type(document['layers']) is dict, 'metrics/layers: expected object')
    for layer in document['layers']:
        require(type(layer) is str and LABEL.fullmatch(layer), 'layer: invalid label')
        require(type(document['layers'][layer]) is dict, 'layer: expected object')
    for values in (document['metrics'], *document['layers'].values()):
        for name, item in values.items():
            _validate_metric(name, item, hashes)
        _circuit_consistency(values)
    effects = document['effects']
    _keys(effects, {'expected_sha256','observed_sha256','match'}, where='effects')
    for key in ('expected_sha256','observed_sha256'):
        _digest(effects[key], 'effects.'+key, nullable=True)
    require(effects['match'] is None or type(effects['match']) is bool, 'effects.match: expected bool/null')
    if effects['match'] is not None:
        require(effects['expected_sha256'] is not None and effects['observed_sha256'] is not None, 'effects: match requires both digests')
        require(effects['match'] == (effects['expected_sha256'] == effects['observed_sha256']), 'effects: inconsistent match claim')
    latencies = _latencies(document['events'])
    for name,item in document['metrics'].items():
        if name.startswith('network.') and item['status'] == 'measured':
            require(name in latencies, f'{name}: event endpoints required')
            require(normalize_number(item['value'],unit='seconds') == latencies[name], f'{name}: event duration mismatch')
    if 'extensions' in document:
        require(type(document['extensions']) is dict, 'extensions: expected object')
        if 'acceptance' in document['extensions']:
            require(type(document['extensions']['acceptance']) is dict, 'extensions.acceptance: expected object; detailed schema belongs to acceptance consumer')
    # JSON roundtrip also excludes nonfinite or non-JSON extension values.
    try:
        return loads_strict(json.dumps(document, allow_nan=False))
    except (TypeError, OverflowError) as exc:
        raise ValueError('sample: non-JSON value') from exc


def event_latencies(sample):
    """Diagnostic durations only; they do not manufacture receipt-bound metrics."""
    sample = validate_sample(sample)
    return _latencies(sample['events'])


def metric_coverage(sample, required_metrics):
    sample = validate_sample(sample)
    require(type(required_metrics) is list and required_metrics, 'required_metrics: nonempty list required')
    require(all(type(x) is str for x in required_metrics), 'required_metrics: expected strings')
    require(len(set(required_metrics)) == len(required_metrics), 'required_metrics: duplicate metric')
    flat = _flatten(sample); missing=[]; unavailable=[]
    for name in required_metrics:
        split_metric(name)
        if name not in flat:
            missing.append(name)
        elif flat[name]['status'] != 'measured':
            unavailable.append(name)
    return {'complete': sample['status'] == 'success' and not missing and not unavailable,
            'missing': missing, 'unavailable': unavailable}


def comparable_profile(sample):
    sample = validate_sample(sample)
    return canonical_sha256({k: sample.get(k) for k in ('workload_id','workload_digest_sha256',
        'evidence_class','identity','cache_state','cache_identity_sha256')})


def summarize(samples):
    """Raw retained; successful uncensored distributions only, no SLA inference."""
    require(type(samples) is list, 'samples: expected list')
    samples = [validate_sample(s) for s in samples]
    require(len({s['sample_id'] for s in samples}) == len(samples), 'duplicate sample_id')
    groups = {}
    for s in samples:
        groups.setdefault(comparable_profile(s), []).append(s)
    output = []
    for fingerprint, members in sorted(groups.items()):
        flats = [_flatten(s) for s in members]
        names = sorted(set().union(*(set(x) for x in flats)))
        summaries = {}
        for name in names:
            items = [(s, f[name]) for s,f in zip(members,flats) if name in f]
            observed_scopes = sorted({v['source'].get('scope','unknown') for _,v in items if v['source'] is not None})
            scopes = {v['source'].get('scope','unknown') for s,v in items
                      if s['status']=='success' and v['status']=='measured'}
            require(len(scopes) <= 1, f'{name}: different resource scopes cannot pool')
            resolutions = sorted({v['source']['timer_resolution_seconds'] for s,v in items
                                  if s['status']=='success' and v['status']=='measured' and v['source'].get('timer_resolution_seconds') is not None})
            unique = {}; duplicate_observations = 0
            for s,v in items:
                if s['status'] != 'success' or v['status'] != 'measured':
                    continue
                src = v['source']; key = src['receipt_sha256']
                if key in unique:
                    require(unique[key] == normalize_number(v['value'],unit=v['unit']), f'{name}: same receipt metric has inconsistent values')
                    duplicate_observations += 1
                else:
                    unique[key] = normalize_number(v['value'],unit=v['unit'])
            values = list(unique.values())
            ordered = sorted(values)
            summaries[name] = {'unit': METRIC_UNITS[split_metric(name)[1]], 'scope':next(iter(scopes),'unknown'),
                'observed_scopes':observed_scopes, 'timer_resolutions_seconds':resolutions,
                'unknown_timer_resolution_observations':sum(s['status']=='success' and v['status']=='measured' and v['source'].get('timer_resolution_seconds') is None for s,v in items), 'duplicate_observations':duplicate_observations, 'n':len(values),
                'missing':len(members)-len(items),
                'success_missing':sum(s['status']=='success' and name not in flat for s,flat in zip(members,flats)),
                'success_unavailable':sum(s['status']=='success' and v['status']=='unavailable' for s,v in items),
                'success_censored':sum(s['status']=='success' and v['status']=='censored' for s,v in items),
                'unavailable':sum(v['status']=='unavailable' for _,v in items),
                'censored':sum(v['status']=='censored' for _,v in items),
                'excluded_terminal':sum(s['status']!='success' and v['status']=='measured' for s,v in items),
                'min': min(values) if values else None, 'max':max(values) if values else None,
                'mean':statistics.mean(values) if values else None,
                'median':statistics.median(values) if values else None,
                'p90':ordered[math.ceil(.90*len(ordered))-1] if ordered else None,
                'p95':ordered[math.ceil(.95*len(ordered))-1] if ordered else None,
                'sample_stdev':statistics.stdev(values) if len(values)>1 else None}
        output.append({'profile_sha256':fingerprint, 'workload_id':members[0]['workload_id'],
            'identity':members[0]['identity'], 'evidence_class':members[0]['evidence_class'],
            'cache_state':members[0]['cache_state'], 'cache_identity_sha256':members[0].get('cache_identity_sha256'),
            'sample_ids':[s['sample_id'] for s in members],
            'counts':{'total':len(members), **{state:sum(s['status']==state for s in members) for state in STATES}},
            'metrics':summaries})
    return {'schema':'sherlock-benchmark-report/1','acceptance':'not-assessed',
        'method':{'distribution':'successful uncensored measured samples only',
                  'percentile':'nearest rank ceil(p*n); descriptive even at n=1',
                  'stdev':'sample standard deviation; null for n<2',
                  'thresholds':'none; baseline only; no SLA supplied'},
        'groups':output,'samples':samples}


def render_markdown(report):
    require(report.get('schema') == 'sherlock-benchmark-report/1', 'report: unsupported schema')
    def cell(value):
        return str(value).replace('|','\\|').replace('\n',' ')
    lines=['# Sherlock benchmark baseline', '',
           'Baseline only; performance targets and target acceptance are not assessed.', '',
           'Distributions use successful uncensored measurements. Raw observations, failures and unavailable metrics remain in the JSON report.', '',
           'Percentiles use nearest rank; a single observation is not evidence of a stable tail.', '']
    for group in report['groups']:
        lines += [f"## {cell(group['workload_id'])} — {cell(group['evidence_class'])}, {cell(group['cache_state'])}", '',
                  f"Comparable profile: `{group['profile_sha256']}`. Counts: {cell(group['counts'])}.", '',
                  '| Metric | Unit | Scope | n | Duplicates | Missing | Unavailable | Censored | Excluded terminal | Median | p95 |',
                  '| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
        for name, stat in group['metrics'].items():
            vals=[name]+[stat[k] for k in ('unit','scope','n','duplicate_observations','missing','unavailable','censored','excluded_terminal','median','p95')]
            lines.append('| '+' | '.join('unavailable' if v is None else cell(v) for v in vals)+' |')
        lines.append('')
    return '\n'.join(lines)+'\n'


def _exact(left, right):
    return type(left) is type(right) and left == right


def _exact_statistic(samples, name, statistic):
    unique = {}
    for sample in samples:
        item = _flatten(sample).get(name)
        if sample['status'] == 'success' and item is not None and item['status'] == 'measured':
            source = item['source']
            key = source['receipt_sha256']
            unique[key] = Fraction(str(item['value']))
    values = sorted(unique.values())
    require(values, 'comparison statistic: no successful measurements')
    if statistic == 'p95':
        return values[math.ceil(.95*len(values))-1]
    middle = len(values)//2
    return values[middle] if len(values)%2 else (values[middle-1]+values[middle])/2


def _exact_decimal(value):
    # Comparison arithmetic starts with finite decimal inputs. Its denominators
    # contain only factors 2 and 5, so this conversion terminates exactly.
    with localcontext() as context:
        context.prec = len(str(abs(value.numerator))) + 4*len(str(value.denominator)) + 4
        result = Decimal(value.numerator)/Decimal(value.denominator)
    return result


def _comparison_number(value):
    try:
        return normalize_number(_exact_decimal(value),unit='seconds')
    except ValueError:
        # Retain the exact decimal string below instead of a rounded JSON bound.
        return None


def compare_baseline(baseline, current, tolerances, allowed_changed_subject_fields=()):
    """Explicit per-profile descriptive regression decision; never target acceptance.

    Limit is baseline*(1+relative_increase)+absolute_increase. Both tolerances
    are mandatory so a zero baseline has defined absolute behavior. Samples are
    recomputed from raw records, never trusted from supplied aggregate fields.
    """
    baseline = summarize(baseline['samples']); current = summarize(current['samples'])
    require(len(baseline['groups']) == len(current['groups']) == 1,
            'comparable profile: compare exactly one group at a time')
    a,b = baseline['groups'][0],current['groups'][0]
    require(not {s['sample_id'] for s in baseline['samples']} & {s['sample_id'] for s in current['samples']},
            'comparison: baseline/current sample_id overlap')
    require(type(allowed_changed_subject_fields) in (list,tuple), 'subject change policy: expected list')
    require(all(type(x) is str for x in allowed_changed_subject_fields), 'subject change policy: expected strings')
    allowed = set(allowed_changed_subject_fields)
    require(len(allowed)==len(allowed_changed_subject_fields) and allowed <= {'source_sha256','compiler_sha256','circuit_sha256'},
            'subject change policy: unsupported or duplicate field')
    left,right = copy.deepcopy(a),copy.deepcopy(b)
    changed = sorted(k for k in IDENTITY_KEYS | IDENTITY_OPTIONAL if a['identity'].get(k) != b['identity'].get(k))
    require(set(changed) <= allowed, 'comparable identity differs outside declared subject changes')
    for key in changed:
        _digest(a['identity'][key], 'baseline subject identity')
        _digest(b['identity'][key], 'current subject identity')
        left['identity'].pop(key);right['identity'].pop(key)
    for key in ('environment_sha256','toolchain_sha256'):
        _digest(a['identity'].get(key), 'comparable '+key)
        _digest(b['identity'].get(key), 'comparable '+key)
    require(a['cache_state'] != 'unknown' and b['cache_state'] != 'unknown', 'comparable cache state unknown')
    fields = ('workload_id','identity','evidence_class','cache_state','cache_identity_sha256')
    require(all(left[k] == right[k] for k in fields), 'comparable profile differs')
    # Workload digest remains mandatory even when source implementation changes.
    require(baseline['samples'][0]['workload_digest_sha256'] == current['samples'][0]['workload_digest_sha256'],
            'comparable workload digest differs')
    require(type(tolerances) is dict and tolerances, 'tolerances: explicit nonempty mapping required')
    outcomes = {}
    for name, rule in tolerances.items():
        split_metric(name)
        _keys(rule, {'statistic','relative_increase','absolute_increase','min_samples'}, where='tolerance')
        require(rule['statistic'] in ('median','p95'), 'tolerance: unknown statistic')
        _number(rule['relative_increase'],'relative_increase')
        _number(rule['absolute_increase'],'absolute_increase')
        _number(rule['min_samples'],'min_samples',integer=True)
        require(rule['min_samples'] >= 1, 'min_samples: must be positive')
        def observations(samples):
            result=set()
            for sample in samples:
                item=_flatten(sample).get(name)
                if sample['status']=='success' and item is not None and item['status']=='measured':
                    result.add(item['source']['receipt_sha256'])
            return result
        require(not observations(baseline['samples']) & observations(current['samples']),
                f'{name}: baseline/current observation reuse')
        old,new = a['metrics'].get(name),b['metrics'].get(name)
        enough = old and new and min(old['n'],new['n']) >= rule['min_samples']
        complete = enough and all(x['success_missing']==x['success_unavailable']==x['success_censored']==x['duplicate_observations']==0 for x in (old,new))
        if not complete:
            outcomes[name] = {'status':'incomplete','rule':copy.deepcopy(rule)}
            continue
        require(old['scope'] != 'unknown' and new['scope'] != 'unknown', 'comparable measurement scope unknown')
        if old['unit']=='seconds':
            require(old['timer_resolutions_seconds'] and new['timer_resolutions_seconds']
                    and old['unknown_timer_resolution_observations']==new['unknown_timer_resolution_observations']==0,
                    'comparable timer resolution unknown')
        require(old['scope'] == new['scope'] and old['timer_resolutions_seconds'] == new['timer_resolutions_seconds'],
                'comparable measurement scope/resolution differs')
        old_value = _exact_statistic(baseline['samples'],name,rule['statistic'])
        new_value = _exact_statistic(current['samples'],name,rule['statistic'])
        limit = old_value * (1 + Fraction(str(rule['relative_increase']))) + Fraction(str(rule['absolute_increase']))
        outcomes[name] = {'status':'regression' if new_value > limit else 'within_declared_tolerance',
                          'baseline':_comparison_number(old_value),'current':_comparison_number(new_value),
                          'limit':_comparison_number(limit),'baseline_statistic_decimal':str(_exact_decimal(old_value)),
                          'current_statistic_decimal':str(_exact_decimal(new_value)),
                          'limit_decimal':str(_exact_decimal(limit)),'rule':copy.deepcopy(rule)}
    states = {x['status'] for x in outcomes.values()}
    status = 'incomplete' if 'incomplete' in states else 'regression' if 'regression' in states else 'within_declared_tolerance'
    return {'schema':'sherlock-benchmark-comparison/1','profile_sha256':a['profile_sha256'],
            'baseline_profile_sha256':a['profile_sha256'],'current_profile_sha256':b['profile_sha256'],
            'baseline_identity':a['identity'],'current_identity':b['identity'],
            'baseline_counts':a['counts'],'current_counts':b['counts'],
            'allowed_changed_subject_fields':sorted(allowed),'changed_subject_fields':changed,
            'status':status,'acceptance':'not-assessed','metrics':outcomes}


def _receipt_terminal(document, template):
    """Known host receipt outcome dominates caller labels; no evidence upgrade."""
    if document.get('format') not in HOST_FORMATS:
        return None
    require(template['evidence_class'] == 'host', 'host receipt must retain host evidence_class')
    terminal = document.get('terminal_state')
    require(type(terminal) is str, 'host terminal_state: missing')
    if terminal == 'success':
        return 'success'
    if terminal.startswith(('timeout:', 'interrupted:')) or terminal == 'running':
        return 'censored'
    return 'failure'


def collect_sample(template, receipt_bindings, base_dir):
    """Read explicit digest-pinned receipts; JSON pointers never invoke commands.

    A binding's terminal mapping (required except for recognized host formats) is {pointer, success_values,
    censored_values}; every other observed value is failure. Missing/null metric
    pointer becomes unavailable. Scale converts original scalar units explicitly.
    """
    require(type(template) is dict, 'template: expected object')
    sample = copy.deepcopy(template)
    require(type(receipt_bindings) is list and receipt_bindings, 'bindings: nonempty list required')
    root = Path(base_dir).resolve(strict=True)
    sample.setdefault('receipts',[]); sample.setdefault('metrics',{});sample.setdefault('layers',{})
    assigned = set(_flatten(sample)); receipt_paths = {r['path'] for r in sample['receipts']}
    terminal_states = []
    for binding in receipt_bindings:
        _keys(binding, {'path','sha256','fields'}, {'terminal'}, where='binding')
        rel = _path(binding['path']); _digest(binding['sha256'], 'binding.sha256')
        path = root.joinpath(*rel.parts).resolve(strict=True)
        require(path.is_relative_to(root) and path.is_file(), 'receipt path: outside base or not file')
        require(path.stat().st_size <= MAX_RECEIPT_BYTES, 'receipt: exceeds 16MiB collector limit')
        raw = read_json_bytes(path)
        digest = hashlib.sha256(raw).hexdigest()
        require(digest == binding['sha256'], 'receipt digest mismatch')
        doc = loads_strict(raw)
        require(type(doc) is dict, 'receipt: expected JSON object')
        require(binding['path'] not in receipt_paths, 'duplicate receipt binding')
        receipt_paths.add(binding['path'])
        schema = doc.get('schema', doc.get('format'))
        require(schema is None or type(schema) is str, 'receipt schema: expected string')
        sample['receipts'].append({'path':binding['path'],'sha256':digest,'schema':schema})
        known_terminal = _receipt_terminal(doc, sample)
        require(known_terminal is not None or 'terminal' in binding,
                'binding: explicit terminal mapping required for unrecognized receipt format')
        binding_states = [known_terminal] if known_terminal is not None else []
        if 'terminal' in binding:
            term = binding['terminal']
            _keys(term, {'pointer','success_values','censored_values'}, where='terminal')
            require(type(term['success_values']) is list and type(term['censored_values']) is list, 'terminal: expected lists')
            require(term['success_values'] and not any(_exact(v, c) for v in term['success_values'] for c in term['censored_values']), 'terminal: ambiguous values')
            try:
                observed = resolve_pointer(doc, term['pointer'])
            except KeyError:
                binding_states.append('failure')
                extensions=sample.setdefault('extensions',{})
                require(type(extensions) is dict, 'extensions: expected object')
                diagnostics=extensions.setdefault('collection_diagnostics',[])
                require(type(diagnostics) is list, 'collection_diagnostics: expected list')
                diagnostics.append({'receipt_sha256':digest,'terminal_pointer':term['pointer'],
                                    'reason':'terminal predicate absent; operation outcome unverified'})
            else:
                binding_states.append('success' if any(_exact(observed,v) for v in term['success_values']) else 'censored' if any(_exact(observed,v) for v in term['censored_values']) else 'failure')
        # Fenrir's actual boolean verifier predicate cannot be replaced by exit0.
        if 'verifies' in doc:
            require(type(doc['verifies']) is bool, 'verifies: expected semantic boolean verdict')
            if doc['verifies'] is False:
                binding_states.append('failure')
        binding_status = next((state for state in ('failure','censored') if state in binding_states), 'success')
        terminal_states.append(binding_status)
        require(type(binding['fields']) is list, 'binding.fields: expected list')
        for field in binding['fields']:
            _keys(field, {'metric','pointer','unit'}, {'scale', 'scope', 'timer_resolution_seconds'}, where='binding field')
            _measurement_metadata(field)
            layer,name = split_metric(field['metric'])
            require(field['metric'] not in assigned, 'duplicate metric binding')
            assigned.add(field['metric'])
            require(field['unit'] == METRIC_UNITS[name], 'binding field unit differs')
            scale = field.get('scale',1); _number(scale, 'scale'); require(scale>0,'scale must be positive')
            try:
                value = resolve_pointer(doc,field['pointer'])
            except KeyError:
                value = None
            if value is None:
                item = {'value':None,'unit':field['unit'],'status':'unavailable','source':None,
                        'reason':f"receipt does not expose {field['pointer']}"}
            else:
                value = receipt_metric_value(raw,field['pointer'],scale,unit=field['unit'])
                censored = binding_status == 'censored'
                item = {'value':value,'unit':field['unit'],'status':'censored' if censored else 'measured',
                        'source':{'receipt_sha256':digest,'pointer':field['pointer'],'scale':scale},
                        'reason':'receipt terminal state censored; captured scalar is partial operation evidence' if censored else None}
            if item['source'] is not None:
                item['source'].update({k: field[k] for k in ('scope','timer_resolution_seconds') if k in field})
            dest = sample['metrics'] if layer is None else sample['layers'].setdefault(layer,{})
            dest[name] = item
    sample['status'] = next((s for s in ('failure','censored','not_run') if s in terminal_states), 'success')
    return validate_sample(sample)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    collect = commands.add_parser('collect', help='read explicit existing receipts; never execute subjects')
    collect.add_argument('--template',required=True);collect.add_argument('--bindings',required=True)
    collect.add_argument('--base-dir',required=True);collect.add_argument('--output',required=True)
    report = commands.add_parser('report',help='descriptive baseline only')
    report.add_argument('--samples',nargs='+',required=True);report.add_argument('--output',required=True)
    report.add_argument('--markdown')
    compare = commands.add_parser('compare',help='explicit descriptive regression tolerances only')
    compare.add_argument('--baseline',required=True);compare.add_argument('--current',required=True)
    compare.add_argument('--tolerances',required=True);compare.add_argument('--output',required=True)
    compare.add_argument('--allow-changed-subject-field',action='append',default=[])
    args=parser.parse_args(argv)
    try:
        if args.command=='collect':
            result=collect_sample(loads_strict(read_json_bytes(Path(args.template))),loads_strict(read_json_bytes(Path(args.bindings))),Path(args.base_dir))
        elif args.command=='compare':
            result=compare_baseline(loads_strict(read_json_bytes(Path(args.baseline))),
                loads_strict(read_json_bytes(Path(args.current))),loads_strict(read_json_bytes(Path(args.tolerances))),
                allowed_changed_subject_fields=args.allow_changed_subject_field)
        else:
            result=summarize([loads_strict(read_json_bytes(Path(p))) for p in args.samples])
        Path(args.output).write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
        if args.command=='report' and args.markdown:
            Path(args.markdown).write_text(render_markdown(result))
        return 0
    except (ValueError, KeyError, OSError, TypeError, OverflowError) as exc:
        print(f'benchmark input error: {exc}',file=sys.stderr)
        return 2

if __name__=='__main__':
    raise SystemExit(main())
