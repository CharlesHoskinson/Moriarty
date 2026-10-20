"""Complete positive META field port. No lane encoder/decoder or counter arithmetic.

Inputs are the frozen positive byte projection; output is a whole strictly typed
CMObservations tree. Native text/UTF8 correspondence is an external input premise.
No file, expected, model, provider or runtime execution occurs on import.
"""
import copy

BOUND_FIELDS = ('code_rows', 'function_rows', 'code_child_slots', 'parameter_slots',
    'pattern_rows', 'match_plan_rows', 'match_arm_rows', 'type_rows', 'constructor_rows',
    'constructor_field_slots', 'max_arity', 'constant_nodes', 'constant_field_slots',
    'data_nodes', 'data_field_slots', 'environment_cells', 'frames', 'helper_tasks',
    'serialization_bytes', 'max_reference_work', 'max_helper_work', 'step_cap')
CAP_FIELDS = ('name_rows', 'name_bytes', 'occurrence_path_slots', 'value_slots')
LIMIT_FIELDS = ('max_total_lanes', 'max_state_lanes', 'max_metadata_bytes',
                'max_string_bytes', 'max_width_entries')
WIDTH_FIELDS = ('index_widths', 'counter_widths', 'record_widths')
FIELD_NAMES = {'identity': 'cmFIdentity', 'run_id': 'cmFRunId', 'layout': 'cmFLayout',
    'layout_version': 'cmFLayoutVersion', 'layout_digest': 'cmFLayoutDigest',
    'bounds_digest': 'cmFBoundsDigest', 'field_id': 'cmFFieldId',
    'program_membership_digest': 'cmFMembershipDigest', 'format': 'cmFFormat',
    'bytes': 'cmFBytes', 'first': 'cmFFirst', 'second': 'cmFSecond', 'widths': 'cmFWidths',
    'index_widths': 'cmFIndexWidths', 'counter_widths': 'cmFCounterWidths',
    'record_widths': 'cmFRecordWidths', 'name': 'cmFName', 'scalar_layout_id': 'cmFScalarLayoutId'}
FIELD_INVERSE = {v: k for k, v in FIELD_NAMES.items()}
GRAMMAR = {'DedicatedSorts': 'cmDedicated', 'GenericNominalExtensionReserved': 'cmGeneric'}
FINANCE = {'Absent': 'cmFinanceAbsent', 'TypedExtensionReserved': 'cmFinanceReserved'}


def atom(sort, value):
    return {'sort': sort, 'value': value}


def term(sort, constructor, *arguments):
    return {'sort': sort, 'constructor': constructor, 'arguments': list(arguments)}


def record(sort, constructor, fields, value):
    return term(sort, constructor, *[atom('Int', value[k]) for k in fields])


def record_document(tree, fields):
    return {k: v['value'] for k, v in zip(fields, tree['arguments'])}


def byte_list(values):
    out = term('CMBytes', 'cmNoBytes')
    for n in reversed(values):
        out = term('CMBytes', 'cmByte', atom('Int', n), out)
    return out


def bytes_document(tree):
    out = []
    while tree['constructor'] != 'cmNoBytes':
        n, tree = tree['arguments']
        out.append(n['value'])
    return out


def lanes(values):
    out = term('CTLanes', 'ctNoLanes')
    for n in reversed(values):
        out = term('CTLanes', 'ctLane', atom('Int', n), out)
    return out


def lanes_document(tree):
    out = []
    while tree['constructor'] != 'ctNoLanes':
        n, tree = tree['arguments']
        out.append(n['value'])
    return out


def pair_term(value):
    return term('CMPair', 'cmPair', byte_list(value['first']), byte_list(value['second']))


def pair_document(tree):
    a, b = tree['arguments']
    return {'first': bytes_document(a), 'second': bytes_document(b)}


def pair_option_term(value):
    return (term('CMOptPair', 'cmNoPair') if value is None else
            term('CMOptPair', 'cmSomePair', pair_term(value['Some'])))


def pair_option_document(tree):
    return None if tree['constructor'] == 'cmNoPair' else {'Some': pair_document(tree['arguments'][0])}


def digest_term(value):
    if value == 'Unselected':
        return term('CMDigest', 'cmUnselected')
    data = value['Selected']
    return term('CMDigest', 'cmSelected', pair_term(data['format']), byte_list(data['bytes']))


def digest_document(tree):
    if tree['constructor'] == 'cmUnselected':
        return 'Unselected'
    pair, data = tree['arguments']
    return {'Selected': {'format': pair_document(pair), 'bytes': bytes_document(data)}}


def identity_term(value):
    return term('CMIdentity', 'cmIdentity', atom('Int', value['profile_id']),
        term('CMGrammar', GRAMMAR[value['grammar_domain']]),
        term('CMFinance', FINANCE[value['finance']]), pair_term(value['layout_version']),
        digest_term(value['layout_digest']), digest_term(value['bounds_digest']),
        pair_option_term(value['field_id']), digest_term(value['program_membership_digest']))


def identity_document(tree):
    profile, grammar, finance, version, layout, bounds, field, member = tree['arguments']
    return {'profile_id': profile['value'],
            'grammar_domain': {v: k for k, v in GRAMMAR.items()}[grammar['constructor']],
            'finance': {v: k for k, v in FINANCE.items()}[finance['constructor']],
            'layout_version': pair_document(version), 'layout_digest': digest_document(layout),
            'bounds_digest': digest_document(bounds), 'field_id': pair_option_document(field),
            'program_membership_digest': digest_document(member)}


def width_list_term(value):
    out = term('CMWidthList', 'cmNoWidthList')
    for row in reversed(value):
        entry = term('CMWidthEntry', 'cmWidth', byte_list(row['name']), atom('Int', row['width']))
        out = term('CMWidthList', 'cmWidthItem', entry, out)
    return out


def width_list_document(tree):
    out = []
    while tree['constructor'] != 'cmNoWidthList':
        item, tree = tree['arguments']
        name, width = item['arguments']
        out.append({'name': bytes_document(name), 'width': width['value']})
    return out


def widths_option_term(value):
    if value is None:
        return term('CMOptWidths', 'cmNoWidths')
    return term('CMOptWidths', 'cmSomeWidths', term('CMWidths', 'cmWidths',
        *[width_list_term(value['Some'][k]) for k in WIDTH_FIELDS]))


def widths_option_document(tree):
    if tree['constructor'] == 'cmNoWidths':
        return None
    rows = tree['arguments'][0]['arguments']
    return {'Some': {k: width_list_document(v) for k, v in zip(WIDTH_FIELDS, rows)}}


def layout_term(value):
    return term('CMLayout', 'cmLayout', pair_term(value['layout_version']),
        pair_option_term(value['field_id']), widths_option_term(value['widths']),
        pair_option_term(value['scalar_layout_id']),
        record('CXBounds', 'cxBounds', BOUND_FIELDS, value['bounds']),
        record('CXLayoutCaps', 'cxLayoutCaps', CAP_FIELDS, value['capacities']),
        atom('Bool', value['boundary_capacities_present']))


def layout_document(tree):
    version, field, widths, scalar, bounds, caps, present = tree['arguments']
    return {'layout_version': pair_document(version), 'field_id': pair_option_document(field),
        'widths': widths_option_document(widths), 'scalar_layout_id': pair_option_document(scalar),
        'bounds': record_document(bounds, BOUND_FIELDS), 'capacities': record_document(caps, CAP_FIELDS),
        'boundary_capacities_present': present['value']}


def run_int_term(value):
    if value == 'Negative':
        return term('CMRunInt', 'cmNegative')
    if value == 'NatAboveU128':
        return term('CMRunInt', 'cmAbove')
    return term('CMRunInt', 'cmNat', atom('Int', value['Nat']))


def run_int_document(tree):
    return ('Negative' if tree['constructor'] == 'cmNegative' else
            'NatAboveU128' if tree['constructor'] == 'cmAbove' else
            {'Nat': tree['arguments'][0]['value']})


def run_id_term(value):
    return term('CMRunId', 'cmRunId', identity_term(value['identity']),
                run_int_term(value['fuel_input']), run_int_term(value['stack_limit_input']))


def run_id_document(tree):
    identity, fuel, stack = tree['arguments']
    return {'identity': identity_document(identity), 'fuel_input': run_int_document(fuel),
            'stack_limit_input': run_int_document(stack)}


def metadata_term(value):
    return term('CMMetadata', 'cmMetadata', identity_term(value['identity']),
        record('CXBounds', 'cxBounds', BOUND_FIELDS, value['bounds']),
        layout_term(value['layout']), run_id_term(value['run_id']))


def metadata_document(tree):
    identity, bounds, layout, run = tree['arguments']
    return {'identity': identity_document(identity), 'bounds': record_document(bounds, BOUND_FIELDS),
            'layout': layout_document(layout), 'run_id': run_id_document(run)}


def port_input(document):
    cases = term('CMCases', 'cmNoCases')
    for case in reversed(document['cases']):
        cases = term('CMCases', 'cmCase', atom('String', case['case_id']),
            record('CMLimits', 'cmLimits', LIMIT_FIELDS, case['limits']), metadata_term(case['metadata']), cases)
    return {'source_document': copy.deepcopy(document), 'profile': document['profile'],
            'case_ids': [c['case_id'] for c in document['cases']],
            'query': term('CMObservations', 'cmObserveAll', cases)}


def cases_document(query):
    cases = query['arguments'][0]
    out = []
    while cases['constructor'] != 'cmNoCases':
        name, limits, metadata, cases = cases['arguments']
        out.append({'case_id': name['value'], 'metadata': metadata_document(metadata),
                    'limits': record_document(limits, LIMIT_FIELDS)})
    return out


def path_term(value):
    out = term('CMPath', 'cmNoPath')
    for step in reversed(value):
        if 'Field' in step:
            out = term('CMPath', 'cmField', term('CMField', FIELD_NAMES[step['Field']]), out)
        else:
            out = term('CMPath', 'cmIndex', atom('Int', step['Index']), out)
    return out


def path_document(tree):
    out = []
    while tree['constructor'] != 'cmNoPath':
        value, rest = tree['arguments']
        out.append({'Field': FIELD_INVERSE[value['constructor']]} if tree['constructor'] == 'cmField'
                   else {'Index': value['value']})
        tree = rest
    return out


def spans_term(value):
    out = term('CMSpans', 'cmNoSpans')
    for row in reversed(value):
        span = term('CMSpan', 'cmSpan', term('CMBlobKind', 'cmText' if row['kind'] == 'Text' else 'cmOpaque'),
            path_term(row['path']), atom('Int', row['length_lane']), atom('Int', row['bytes_start']),
            atom('Int', row['bytes_end']), byte_list(row['bytes']))
        out = term('CMSpans', 'cmSpanItem', span, out)
    return out


def spans_document(tree):
    out = []
    while tree['constructor'] != 'cmNoSpans':
        span, tree = tree['arguments']
        kind, path, length, start, end, data = span['arguments']
        out.append({'kind': 'Text' if kind['constructor'] == 'cmText' else 'OpaqueDigestBytes',
            'path': path_document(path), 'length_lane': length['value'], 'bytes_start': start['value'],
            'bytes_end': end['value'], 'bytes': bytes_document(data)})
    return out


def observation_term(value):
    return term('CMObservation', 'cmObservation', lanes(value['encoded_lanes']),
        metadata_term(value['decoded_metadata']), spans_term(value['blob_spans']),
        atom('Int', value['final_blob_bytes']), atom('Int', value['final_width_entries']),
        atom('Int', value['final_cursor']))


def observation_document(tree):
    encoded, decoded, spans, blobs, widths, cursor = tree['arguments']
    return {'encoded_lanes': lanes_document(encoded), 'decoded_metadata': metadata_document(decoded),
            'blob_spans': spans_document(spans), 'final_blob_bytes': blobs['value'],
            'final_width_entries': widths['value'], 'final_cursor': cursor['value']}


def observations_term(values):
    out = term('CMObservations', 'cmNoObservations')
    for value in reversed(values):
        out = term('CMObservations', 'cmObservationItem', value, out)
    return out


def observations_document(tree, source):
    out = []
    while tree['constructor'] != 'cmNoObservations':
        value, tree = tree['arguments']
        out.append(observation_document(value))
    return {'schema': 'independent-c6-complete-META-positive/1', 'profile': source['profile'],
            'case_ids': [case['case_id'] for case in source['cases']], 'observations': out}


def _key_bridge(value, source_key, target_key):
    # Whole generic tree key translation, iterative to preserve arbitrary finite depth.
    holder = [None]
    pending = [(value, holder, 0)]
    while pending:
        current, parent, key = pending.pop()
        if type(current) is dict:
            result = {}
            parent[key] = result
            for k, v in current.items():
                pending.append((v, result, target_key if k == source_key else k))
        elif type(current) is list:
            result = [None] * len(current)
            parent[key] = result
            for i, item in enumerate(current):
                pending.append((item, result, i))
        else:
            parent[key] = current
    return holder[0]


def to_wire(tree):
    return _key_bridge(tree, 'value', 'literal')


def from_wire(tree):
    return _key_bridge(tree, 'literal', 'value')
