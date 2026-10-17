"""Read public v1 JSONL capture as DATA; never invoke the producer/engine.
Supplied captures are DATA only. Profile lexemes retain singular record provenance.
"""
from decimal import Decimal
import json
import hashlib
from pathlib import Path
from normal_compare import differences, keys, require
from proposed_transport import ProposedTransport as HostTransport

DELTA_PATHS = ('remaining_work', 'consumed_work', 'helper_work', 'helper_visits',
               'reference_cursor', 'alloc.data_nodes', 'alloc.data_fields',
               'alloc.environment_cells', 'depth', 'taskLive', 'active.value_slots')


class NumericLexeme:
    def __init__(self, token, kind):
        self.token, self.kind = token, kind


def read_lines(path, numeric_tokens=None, capture_provenance=None):
    records = []
    raw = Path(path).read_bytes()
    text = raw.decode('utf-8')
    if capture_provenance is not None:
        capture_provenance.update({'source': str(path), 'bytes': len(raw),
                                   'sha256': hashlib.sha256(raw).hexdigest(),
                                   'meaning': 'same raw bytes decoded below; source capture remains unchanged'})
    require(text.endswith('\n'), '/capture', 'final record lacks the protocol LF; capture/result remains partial')
    for i, line in enumerate(text.split('\n')[:-1]):
        require(bool(line.strip()), f'/records/{i}', 'empty JSONL record')
        def pairs(items):
            out = {}
            for key, value in items:
                require(key not in out, f'/records/{i}/{key}', 'duplicate JSON key')
                out[key] = value
            return out
        def constant(value):
            raise ValueError(f'/records/{i}: non-JSON numeric token {value}')
        tokens = {}
        def unwrap(value, pointer=''):
            if type(value) is NumericLexeme:
                tokens[pointer] = {'token': value.token, 'kind': value.kind}
                return int(value.token) if value.kind == 'integer' else Decimal(value.token)
            if type(value) is list:
                return [unwrap(v, pointer+'/'+str(j)) for j, v in enumerate(value)]
            if type(value) is dict:
                return {k: unwrap(v, pointer+'/'+k.replace('~','~0').replace('/','~1')) for k, v in value.items()}
            return value
        parsed = json.loads(line, object_pairs_hook=pairs,
                            parse_int=lambda t: NumericLexeme(t, 'integer'),
                            parse_float=lambda t: NumericLexeme(t, 'decimal'), parse_constant=constant)
        row = unwrap(parsed)
        if numeric_tokens is not None:
            numeric_tokens.append(tokens)
        keys(row, ('schema', 'kind', 'case_id', 'fuel', 'stack_limit', 'observation') + (() if i == 0 else ('event',)), f'/records/{i}')
        require(row['schema'] == 'sherlock-host-capture-v1', f'/records/{i}/schema', 'unknown capture format')
        require(row['kind'] == ('initial' if i == 0 else 'event'), f'/records/{i}/kind', 'complete ordered initial/event framing required')
        records.append(row)
    require(bool(records), '/records', 'missing initial observation')
    return records


def observed_delta(before, after, record_index):
    """Eleven differences of supplied literal integers; no rule/evaluation."""
    values = []
    for dotted in DELTA_PATHS:
        def get(image, source_index):
            out = image
            for part in dotted.split('.'):
                require(type(out) is dict and part in out, f'/records/{source_index}/observation/image/{dotted.replace(".", "/")}', 'required observed delta carrier missing')
                out = out[part]
            require(type(out) is int, f'/records/{source_index}/observation/image/{dotted.replace(".", "/")}', 'exact JSON integer required for delta')
            return out
        values.append(get(after, record_index) - get(before, record_index - 1))
    return values


def compare_capture(expected, path):
    numeric_tokens, capture_provenance = [], {}
    rows = read_lines(path, numeric_tokens, capture_provenance)
    metadata = {'case_id': expected['case'], 'fuel': {'$variant': 'Nat', 'value': expected['fuel']},
                'stack_limit': {'$variant': 'Nat', 'value': expected['states'][0]['reg']['stackLimit']}}
    metadata_issues = []
    for i, row in enumerate(rows):
        for key, value in metadata.items():
            metadata_issues.extend(differences(value, row[key], f'/records/{i}/{key}'))
    # The family is an external frozen-format selector, not a captured engine field.
    actual_fuel = rows[0]['fuel']
    if type(actual_fuel) is dict and set(actual_fuel) == {'$variant', 'value'} and actual_fuel['$variant'] == 'Nat':
        actual_fuel = actual_fuel['value']
    actual = {'selection': {'family': expected['family'], 'case': rows[0]['case_id'], 'fuel': actual_fuel},
              'observations': [r['observation'] for r in rows], 'events': [r['event'] for r in rows[1:]], 'deltas': []}
    delta_issues = []
    for i in range(1, len(rows)):
        try:
            actual['deltas'].append(observed_delta(rows[i-1]['observation']['image'], rows[i]['observation']['image'], i))
        except (ValueError, KeyError, TypeError) as exc:
            actual['deltas'].append({'unavailableLiteralDelta': str(exc)})
            delta_issues.append({'path': f'/records/{i}/observation/image', 'kind': 'delta_carrier_gap', 'detail': str(exc)})
    report = HostTransport().compare(expected, actual)
    from profile_transport import compare_profiles
    profile_checks, profile_issues = compare_profiles(expected, rows, numeric_tokens)
    # Replace only the named aggregate-without-wire gap using actual record provenance.
    report['transportGaps'] = [g for g in report['transportGaps'] if g['expectedPath'] != '/profilePresentation']
    report['profileComparisons'] = profile_checks
    report['captureProvenance'] = capture_provenance
    report['profileOtherAtoms'] = 'All original raw identity/run atoms remain independently in full tree comparison; profile inverse does not establish binding or identity equality.'
    report['differences'].extend(profile_issues)
    report['differences'] = metadata_issues + delta_issues + report['differences']
    report['captureSchema'] = 'sherlock-host-capture-v1'
    report['deltaSource'] = 'exact differences of observed literal before/post counters; no source/rule execution'
    report['startupRepresentation'] = {'path': '/records/0/observation', 'status': 'COMPLETE_INITIAL_IMAGE',
        'authority': 'host-event-side-candidate-02/schema-supplement.json /step_event/fields/originReferenceCursor: P0 and Startup emit no StepEvent.'}
    report['terminalCallLimit'] = 'Root run-stop rule prohibits a step after non-Running publication; any oracle terminal-next-call annotation remains specified-only, not an invented additional event'
    report['dataComparison'] = 'MISMATCH' if report['differences'] else 'NO_MAPPED_DIFFERENCES'
    report['status'] = 'GAP' if report['transportGaps'] or report.get('sourceStatus') == 'SOURCE_PROPOSED_UNADOPTED' else ('MISMATCH' if report['differences'] else 'EXACT_DATA_MATCH')
    report['engineComparison'] = 'PRODUCER_ORIGIN_REQUIRES_ROOT_ATTESTATION'
    return report
