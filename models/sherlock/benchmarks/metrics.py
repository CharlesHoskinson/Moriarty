#!/usr/bin/env python3
"""Receipt-only benchmark collection and descriptive statistics (stdlib only).

No execution API: reads only explicitly selected local JSON receipts. Hashes
establish local lineage, not producer authenticity, proof validity or authority.
Subprocess resource totals include process startup and wrapper overhead.
"""
import argparse
import copy
from datetime import datetime
import hashlib
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


def loads_strict(raw):
    """Reject duplicate fields and non-JSON numeric constants before validation."""
    def pairs(items):
        result = {}
        for key, value in items:
            require(key not in result, f'duplicate JSON key: {key}')
            result[key] = value
        return result
    def constant(value):
        raise ValueError(f'nonfinite JSON number: {value}')
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=constant)


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


def _event_times(events):
    require(type(events) is dict and set(events) <= {'proof_ready','submission','ack','inclusion','readback'}, 'events: field set differs')
    parsed = {}
    for key in ('proof_ready','submission','ack','inclusion','readback'):
        raw = events.get(key)
        if raw is None:
            continue
        require(type(raw) is str and ('T' in raw) and raw.endswith(('Z', '+00:00')), f'events.{key}: UTC timestamp required')
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
        capacity = values.get('circuit.domain_capacity_rows', {})
        used = values.get('circuit.used_rows', {})
        if capacity.get('status') == used.get('status') == 'measured':
            require(used['value'] <= capacity['value'], 'circuit: used rows exceed observed capacity')
    effects = document['effects']
    _keys(effects, {'expected_sha256','observed_sha256','match'}, where='effects')
    for key in ('expected_sha256','observed_sha256'):
        _digest(effects[key], 'effects.'+key, nullable=True)
    require(effects['match'] is None or type(effects['match']) is bool, 'effects.match: expected bool/null')
    if effects['match'] is not None:
        require(effects['expected_sha256'] is not None and effects['observed_sha256'] is not None, 'effects: match requires both digests')
        require(effects['match'] == (effects['expected_sha256'] == effects['observed_sha256']), 'effects: inconsistent match claim')
    _event_times(document['events'])
    if 'extensions' in document:
        require(type(document['extensions']) is dict, 'extensions: expected object')
    # JSON roundtrip also excludes nonfinite or non-JSON extension values.
    try:
        return loads_strict(json.dumps(document, allow_nan=False))
    except (TypeError, OverflowError) as exc:
        raise ValueError('sample: non-JSON value') from exc


def event_latencies(sample):
    """Diagnostic durations only; they do not manufacture receipt-bound metrics."""
    sample = validate_sample(sample)
    t = _event_times(sample['events'])
    return {f'network.{a}_to_{b}_seconds': (t[b]-t[a]).total_seconds()
            for a,b in EVENT_PAIRS if a in t and b in t}


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
            scopes = {v['source'].get('scope','unknown') for _,v in items if v['source'] is not None}
            require(len(scopes) <= 1, f'{name}: different resource scopes cannot pool')
            resolutions = sorted({v['source']['timer_resolution_seconds'] for _,v in items
                                  if v['source'] is not None and v['source'].get('timer_resolution_seconds') is not None})
            unique = {}; duplicate_observations = 0
            for s,v in items:
                if s['status'] != 'success' or v['status'] != 'measured':
                    continue
                src = v['source']; key = (src['receipt_sha256'],src['pointer'],src.get('scale',1))
                if key in unique:
                    require(_exact(unique[key],v['value']), f'{name}: same receipt pointer has inconsistent values')
                    duplicate_observations += 1
                else:
                    unique[key] = v['value']
            values = list(unique.values())
            ordered = sorted(values)
            summaries[name] = {'unit': METRIC_UNITS[split_metric(name)[1]], 'scope':next(iter(scopes),'unknown'),
                'timer_resolutions_seconds':resolutions, 'duplicate_observations':duplicate_observations, 'n':len(values),
                'missing':len(members)-len(items),
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
        old,new = a['metrics'].get(name),b['metrics'].get(name)
        enough = old and new and min(old['n'],new['n']) >= rule['min_samples']
        clean = all(g['counts']['total'] == g['counts']['success'] for g in (a,b))
        complete = enough and clean and all(x['missing']==x['unavailable']==x['censored']==x['duplicate_observations']==0 for x in (old,new))
        if not complete:
            outcomes[name] = {'status':'incomplete','rule':copy.deepcopy(rule)}
            continue
        require(old['scope'] == new['scope'] and old['timer_resolutions_seconds'] == new['timer_resolutions_seconds'],
                'comparable measurement scope/resolution differs')
        old_value,new_value = old[rule['statistic']],new[rule['statistic']]
        limit = old_value * (1 + rule['relative_increase']) + rule['absolute_increase']
        outcomes[name] = {'status':'regression' if new_value > limit else 'within_declared_tolerance',
                          'baseline':old_value,'current':new_value,'limit':limit,'rule':copy.deepcopy(rule)}
    states = {x['status'] for x in outcomes.values()}
    status = 'incomplete' if 'incomplete' in states else 'regression' if 'regression' in states else 'within_declared_tolerance'
    return {'schema':'sherlock-benchmark-comparison/1','profile_sha256':a['profile_sha256'],
            'baseline_profile_sha256':a['profile_sha256'],'current_profile_sha256':b['profile_sha256'],
            'baseline_identity':a['identity'],'current_identity':b['identity'],
            'allowed_changed_subject_fields':sorted(allowed),'changed_subject_fields':changed,
            'status':status,'acceptance':'not-assessed','metrics':outcomes}


def _receipt_terminal(document, template):
    """Known host receipt outcome dominates caller labels; no evidence upgrade."""
    if document.get('format') not in HOST_FORMATS:
        return template['status']
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

    A binding's optional terminal mapping is {pointer, success_values,
    censored_values}; every other observed value is failure. Missing/null metric
    pointer becomes unavailable. Scale converts original scalar units explicitly.
    """
    require(type(template) is dict, 'template: expected object')
    sample = copy.deepcopy(template)
    require(type(receipt_bindings) is list and receipt_bindings, 'bindings: nonempty list required')
    root = Path(base_dir).resolve(strict=True)
    sample.setdefault('receipts',[]); sample.setdefault('metrics',{});sample.setdefault('layers',{})
    assigned = set(_flatten(sample)); receipt_paths = {r['path'] for r in sample['receipts']}
    terminal_states = [sample['status']]
    for binding in receipt_bindings:
        _keys(binding, {'path','sha256','fields'}, {'terminal'}, where='binding')
        rel = _path(binding['path']); _digest(binding['sha256'], 'binding.sha256')
        path = root.joinpath(*rel.parts).resolve(strict=True)
        require(path.is_relative_to(root) and path.is_file(), 'receipt path: outside base or not file')
        require(path.stat().st_size <= MAX_RECEIPT_BYTES, 'receipt: exceeds 16MiB collector limit')
        raw = path.read_bytes()
        digest = hashlib.sha256(raw).hexdigest()
        require(digest == binding['sha256'], 'receipt digest mismatch')
        doc = loads_strict(raw)
        require(type(doc) is dict, 'receipt: expected JSON object')
        require(binding['path'] not in receipt_paths, 'duplicate receipt binding')
        receipt_paths.add(binding['path'])
        schema = doc.get('schema', doc.get('format'))
        require(schema is None or type(schema) is str, 'receipt schema: expected string')
        sample['receipts'].append({'path':binding['path'],'sha256':digest,'schema':schema})
        terminal_states.append(_receipt_terminal(doc, sample))
        if 'terminal' in binding:
            term = binding['terminal']
            _keys(term, {'pointer','success_values','censored_values'}, where='terminal')
            require(type(term['success_values']) is list and type(term['censored_values']) is list, 'terminal: expected lists')
            require(term['success_values'] and not any(_exact(v, c) for v in term['success_values'] for c in term['censored_values']), 'terminal: ambiguous values')
            observed = resolve_pointer(doc, term['pointer'])
            terminal_states.append('success' if any(_exact(observed,v) for v in term['success_values']) else 'censored' if any(_exact(observed,v) for v in term['censored_values']) else 'failure')
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
                _number(value,'receipt scalar'); value = value * scale
                if field['unit'] != 'seconds' and type(value) is float and value.is_integer():
                    value = int(value)
                item = {'value':value,'unit':field['unit'],'status':'measured',
                        'source':{'receipt_sha256':digest,'pointer':field['pointer'],'scale':scale},'reason':None}
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
            result=collect_sample(loads_strict(Path(args.template).read_bytes()),loads_strict(Path(args.bindings).read_bytes()),Path(args.base_dir))
        elif args.command=='compare':
            result=compare_baseline(loads_strict(Path(args.baseline).read_bytes()),
                loads_strict(Path(args.current).read_bytes()),loads_strict(Path(args.tolerances).read_bytes()),
                allowed_changed_subject_fields=args.allow_changed_subject_field)
        else:
            result=summarize([loads_strict(Path(p).read_bytes()) for p in args.samples])
        Path(args.output).write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
        if args.command=='report' and args.markdown:
            Path(args.markdown).write_text(render_markdown(result))
        return 0
    except (ValueError, KeyError, OSError, TypeError) as exc:
        print(f'benchmark input error: {exc}',file=sys.stderr)
        return 2

if __name__=='__main__':
    raise SystemExit(main())
