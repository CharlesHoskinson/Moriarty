"""Positive HELPER/TASK/STATE runtime consumer. No lane, cursor, span, or width arithmetic in Python.

DATA264 is strict for output. csObserveAll is added to the registry only temporarily
for query render and whole typed inverse. Existing pin, unique-JSON, iterative
same_whole, JSONv4, barrier, and capture mechanisms are reused unchanged in shape.
state_ports.to_json is the complete typed wire->common field decoder.
"""
import argparse
import copy
import json
from pathlib import Path
import sys

SOURCE_DIR = Path(__file__).resolve().parent
PRODUCER = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01')
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
sys.path.insert(0, str(BASE / 'source'))
sys.path.insert(0, str(PRODUCER / 'source'))
sys.path.insert(0, str(SOURCE_DIR))
import c6_data as base
import state_ports

INPUT_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/declaration-transport01/SAFE/SAFE-INPUTS.json', 'bytes': 31059, 'sha256': 'af3361b9d910cc3bd738cea4d3015fe4170efdc99d08315d27c953fd8ebc050a'}
REGISTRY_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01/source/PUBLIC-COMBINED-CONSTRUCTOR-REGISTRY.json', 'bytes': 80421, 'sha256': '344681877914882446f30aaa519338f343cf2c0c3b857ebaee59f713ec71c66b'}
LABEL_MAP_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01/postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json', 'bytes': 368969, 'sha256': '7c4c239dbc8217839b301ca5494df6a24607a0b445ab0194d10f27919e4d18f4'}
EXPECTED_COMMON_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/B39-positive-helper-task-state-20261020/private/EXPECTED-COMPLETE-OBSERVATIONS.json', 'bytes': 54571, 'sha256': '32c823b636e97103ba71df9e88ec32701655454b17d61d61ebd25eb325e8f73e'}
EXPECTED_AST_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/B39-positive-helper-task-state-20261020/k-transport-01/private/EXPECTED-COMPLETE-CSObservations.json', 'bytes': 186296, 'sha256': '7de9cc220c230f978dd71a73964fec89313015a738ed24bd6a8683d9581ee684'}
PROFILE = 'sherlock-c6-complete-helper-task-state-positive/1'
MODULE = 'SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE-LAWS'
RESULT_SORT = 'CSObservations'
OUTPUT_SCHEMA = 'independent-c6-positive-helper-task-state/1'
RENDER_PASS = 'PASS_LOSSLESS_C6_HELPER_TASK_STATE_QUERY_CARRIER'
CAPTURE_PASS = 'PASS_COMPLETE_CSOBSERVATIONS_CAPTURE'
CAPTURE_FAIL = 'FAIL_DECODED_HELPER_TASK_STATE_SOURCE_IDENTITY'
COMPARE_PASS = 'PASS_WHOLE_TYPED_C6_HELPER_TASK_STATE_COMMONJSON_AND_TYPEDAST'
COMPARE_SCOPE = 'independent-complete-K-HELPER-TASK-STATE-commonJSON-and-typedAST-eight-root-fields'
ROOT_KEYS = ['schema', 'profile', 'case_ids', 'helper_record_observations', 'result_record_observations', 'task_record_observations', 'helper_section_observations', 'state_observations']
CASE_KEYS = ('helper_record_cases', 'result_record_cases', 'task_record_cases', 'helper_section_cases', 'state_cases')
CASE_IDS = {'helper_record_cases': ['helper-00', 'helper-01', 'helper-02', 'helper-03', 'helper-04', 'helper-05', 'helper-06', 'helper-07', 'helper-08', 'helper-09', 'helper-10', 'helper-11', 'helper-12', 'helper-13', 'helper-14', 'helper-15', 'helper-16', 'helper-17'], 'result_record_cases': ['result-00', 'result-01', 'result-02', 'result-03', 'result-04', 'result-05', 'result-06', 'result-07', 'result-08', 'result-09', 'result-10', 'result-11', 'result-12', 'result-13', 'result-14', 'result-15', 'result-16', 'result-17', 'result-18', 'result-19'], 'task_record_cases': ['task-00', 'task-01', 'task-02'], 'helper_section_cases': ['present-plan-result-nonpalindromic-task-order', 'present-build-result-contrasting-task-order', 'absent-helper-present-env-result-empty-tasks'], 'state_cases': ['nonempty-plan-state-ordered-mixed-task-stack', 'build-state-present-context-different-task-order']}

PUBLIC = base.loads(base.checked(REGISTRY_PIN))
DATA_REG = {row['name']: (row['sort'], tuple(row['argument_sorts'])) for row in PUBLIC['constructors']}
ENTRY = dict(state_ports.ENTRY)
ENTRY_REG = {ENTRY['name']: (ENTRY['sort'], tuple(ENTRY['argument_sorts']))}
if len(DATA_REG) != 264:
    raise ValueError('closed DATA264 registry required')
if ENTRY != {'name': 'csObserveAll', 'sort': 'CSObservations', 'argument_sorts': ['CSCohort'], 'declaring_module': 'SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE'}:
    raise ValueError('csObserveAll entry declaration differs')
base.codec.REG = dict(DATA_REG)


def keys(value, want):
    if type(value) is not dict or set(value) != set(want):
        raise ValueError('exact JSON carrier keyset differs')
    return value


def kore_label(raw):
    codes = {'(': 'LPar', ')': 'RPar', ',': 'Comm', '_': 'Unds'}
    result, quoted = 'Lbl', False
    for char in raw:
        if char.isascii() and (char.isalnum() or char == '-'):
            if quoted:
                result += "'"; quoted = False
            result += char
        else:
            if char not in codes:
                raise ValueError('outside declared compiled-label alphabet')
            if not quoted:
                result += "'"; quoted = True
            result += codes[char]
    return result + ("'" if quoted else '')


def validate_label_map(data):
    keys(data, ('status', 'definition', 'constructor_count', 'constructors', 'surface_registry_scope', 'method'))
    if type(data['constructors']) is not list or data['constructor_count'] != 265 or len(data['constructors']) != 265:
        raise ValueError('complete compiled label map count required')
    lines = base.checked(data['definition']).decode('utf-8').splitlines()
    forward, reverse = {}, {}
    for row in data['constructors']:
        keys(row, ('surface_constructor', 'sort', 'argument_sorts', 'declaring_module', 'declaration_source', 'compiled_klabel', 'kore_symbol', 'declaration_line', 'complete_symbol_declaration'))
        name, raw = row['surface_constructor'], row['compiled_klabel']
        if type(name) is not str or name not in base.codec.REG or name in forward or type(raw) is not str or raw in reverse:
            raise ValueError('unknown/duplicate compiled label declaration')
        sort, sorts = base.codec.REG[name]
        if row['sort'] != sort or tuple(row['argument_sorts']) != sorts:
            raise ValueError('compiled label sort/arguments differ')
        module = row['declaring_module']
        declared = name + '(' + ','.join('_' for _ in sorts) + ')_' + module + '_' + sort + ''.join('_' + s for s in sorts)
        if raw != declared or row['kore_symbol'] != kore_label(raw):
            raise ValueError('compiled raw/symbol mapping differs')
        line, body = row['declaration_line'], row['complete_symbol_declaration']
        if type(line) is not int or not 1 <= line <= len(lines) or lines[line - 1] != body or not body.startswith('  symbol ' + row['kore_symbol'] + '{}('):
            raise ValueError('actual compiled symbol declaration membership differs')
        if '{}' + '(' + ', '.join('Sort' + s + '{}' for s in sorts) + ') : Sort' + sort + '{} ' not in body:
            raise ValueError('actual compiled symbol signature differs')
        base.checked(row['declaration_source'])
        forward[name] = raw; reverse[raw] = name
    if set(forward) != set(base.codec.REG):
        raise ValueError('compiled label map missing registry entry')
    return forward, reverse


def validated_labels():
    saved = base.codec.REG
    try:
        base.codec.REG = {**DATA_REG, **ENTRY_REG}
        return validate_label_map(base.loads(base.checked(LABEL_MAP_PIN)))
    finally:
        base.codec.REG = saved


LABELS, REVERSE_LABELS = validated_labels()


def _key_bridge(value, source_key, target_key):
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


def tree_to_wire(tree):
    return _key_bridge(tree, 'value', 'literal')


def wire_to_tree(tree):
    return _key_bridge(tree, 'literal', 'value')


def from_tree(tree):
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed tree required')
    sort = tree['sort']
    if sort in ('Int', 'Bool', 'String'):
        keys(tree, ('sort', 'value'))
        return base.encode(tree['value'], sort)
    keys(tree, ('sort', 'constructor', 'arguments'))
    name, args = tree['constructor'], tree['arguments']
    if type(name) is not str or name not in base.codec.REG or type(args) is not list:
        raise ValueError('unknown typed constructor')
    actual, sorts = base.codec.REG[name]
    if sort != actual or len(args) != len(sorts):
        raise ValueError('typed tree sort/arity differs')
    children = [from_tree(x) for x in args]
    if tuple(x.sort for x in children) != sorts:
        raise ValueError('child typed sorts differ')
    return base.codec.C(name, *children)


def encode_node(term):
    if term.name in ('@Int', '@Bool', '@String'):
        expected = {'Int': int, 'Bool': bool, 'String': str}
        if term.sort not in expected or type(term.args[0]) is not expected[term.sort]:
            raise ValueError('scalar kind differs')
        return {'node': 'KToken', 'sort': {'node': 'KSort', 'name': term.sort, 'params': []}, 'token': base.codec.render(term)}
    if term.name not in base.codec.REG:
        raise ValueError('outside closed carrier profile')
    sort, sorts = base.codec.REG[term.name]
    if term.sort != sort or tuple(x.sort for x in term.args) != sorts or term.name not in LABELS:
        raise ValueError('typed constructor differs')
    return {'node': 'KApply', 'label': {'node': 'KLabel', 'name': LABELS[term.name], 'params': []}, 'arity': len(term.args), 'args': [encode_node(x) for x in term.args]}


def envelope(term):
    return {'format': 'KAST', 'version': 4, 'term': encode_node(term)}


def decode_node(value, sort, strict_output=False, allow_string=False):
    if type(value) is not dict:
        raise ValueError('K node object required')
    if value.get('node') == 'KToken':
        keys(value, ('node', 'sort', 'token'))
        s = keys(value['sort'], ('node', 'name', 'params'))
        if s != {'node': 'KSort', 'name': sort, 'params': []} or sort not in ('Int', 'Bool', 'String') or type(value['token']) is not str:
            raise ValueError('exact primitive KSort/token required')
        term = base.codec.parse_printed(value['token'], sort)
        if type(term.args[0]) is not {'Int': int, 'Bool': bool, 'String': str}[sort] or base.codec.render(term) != value['token']:
            raise ValueError('noncanonical scalar lexeme')
        return term
    keys(value, ('node', 'label', 'arity', 'args'))
    if value['node'] != 'KApply':
        raise ValueError('outside KApply/KToken carrier profile')
    label = keys(value['label'], ('node', 'name', 'params'))
    if label['node'] != 'KLabel' or label['params'] != [] or type(label['name']) is not str or label['name'] not in REVERSE_LABELS:
        raise ValueError('unknown or parametric label')
    name = REVERSE_LABELS[label['name']]
    if name not in base.codec.REG:
        raise ValueError('residual/entry function refused as output')
    actual, sorts = base.codec.REG[name]
    if actual != sort or type(value['arity']) is not int or value['arity'] != len(sorts) or type(value['args']) is not list or len(value['args']) != len(sorts):
        raise ValueError('exact KApply sort/arity differs')
    return base.codec.C(name, *(decode_node(x, s, strict_output=strict_output, allow_string=allow_string) for x, s in zip(value['args'], sorts)))


def decode_document(value, sort, strict_output=False, allow_string=False):
    keys(value, ('format', 'version', 'term'))
    if value['format'] != 'KAST' or type(value['version']) is not int or value['version'] != 4:
        raise ValueError('KAST JSONv4 required')
    return decode_node(value['term'], sort, strict_output=strict_output, allow_string=allow_string)


def same_whole(a, b):
    stack = [(a, b)]
    while stack:
        x, y = stack.pop()
        if type(x) is not type(y):
            return False
        if type(x) is dict:
            if x.keys() != y.keys():
                return False
            stack.extend((x[k], y[k]) for k in x)
        elif type(x) is list:
            if len(x) != len(y):
                return False
            stack.extend(zip(x, y))
        elif x != y:
            return False
    return True


def grouped_case_ids(document):
    return {k: [c['case_id'] for c in document[k]] for k in CASE_KEYS}


def carrier(document):
    if type(document) is not dict:
        raise ValueError('source document object required')
    if document.get('profile') != PROFILE or grouped_case_ids(document) != CASE_IDS:
        raise ValueError('source profile/case binding differs')
    wire_query = state_ports.query(document)
    if state_ports.cohort_to_document(wire_query['arguments'][0]) != document:
        raise ValueError('whole source cohort inverse differs')
    ported = {'source_document': copy.deepcopy(document), 'profile': document['profile'], 'case_ids': grouped_case_ids(document), 'query': wire_query}
    saved = base.codec.REG
    try:
        base.codec.REG = {**DATA_REG, **ENTRY_REG}
        term = from_tree(wire_to_tree(wire_query))
        if term.sort != 'CSObservations' or term.name != 'csObserveAll':
            raise ValueError('complete csObserveAll source term required')
        wire = envelope(term)
        if not base.same(base.tree(term), base.tree(decode_document(wire, 'CSObservations', allow_string=True))):
            raise ValueError('whole source query/KJSON typed inverse differs')
        return term, wire, ported
    finally:
        base.codec.REG = saved


def render(manifest, output, report):
    m = base.load(manifest)
    keys(m, ('source_input', 'compiled_definition'))
    if not same_whole(m['source_input'], INPUT_PIN):
        raise ValueError('source input operand differs')
    base.checked(m['source_input'])
    definition = base.loads(base.checked(LABEL_MAP_PIN))['definition']
    if m['compiled_definition'] != definition:
        raise ValueError('compiled definition operand differs')
    base.checked(definition)
    source = base.loads(base.checked(m['source_input']))
    _term, wire, ported = carrier(source)
    product = base.write(output, (json.dumps(wire, separators=(',', ':'), allow_nan=False) + '\n').encode())
    result = {'status': RENDER_PASS, 'source_input': m['source_input'], 'compiled_definition': definition, 'compiled_label_map': LABEL_MAP_PIN, 'actual_label_count': len(LABELS), 'data_label_count': len(DATA_REG), 'profile': ported['profile'], 'case_ids': ported['case_ids'], 'module': MODULE, 'sort': RESULT_SORT, 'same_whole_typed_query': True, 'product': product, 'expected_body_reads': 0, 'no_model_or_lane_evaluation_in_Python': True}
    report_pin = base.write(report, (json.dumps(result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': result['status'], 'product': product, 'report': report_pin}))


def source_identity(common, source):
    if type(common) is not dict or list(common) != ROOT_KEYS:
        return False
    try:
        checks = [
            common['schema'] == OUTPUT_SCHEMA,
            common['profile'] == source.get('profile') == PROFILE,
            same_whole(common['case_ids'], grouped_case_ids(source)),
            same_whole(common['case_ids'], CASE_IDS),
            len(common['helper_record_observations']) == 18,
            len(common['result_record_observations']) == 20,
            len(common['task_record_observations']) == 3,
            len(common['helper_section_observations']) == 3,
            len(common['state_observations']) == 2,
            same_whole([o['decoded_helper'] for o in common['helper_record_observations']], [c['helper'] for c in source['helper_record_cases']]),
            same_whole([o['decoded_result'] for o in common['result_record_observations']], [c['result'] for c in source['result_record_cases']]),
            same_whole([o['decoded_task'] for o in common['task_record_observations']], [c['task'] for c in source['task_record_cases']]),
            same_whole([o['decoded_tables'] for o in common['helper_section_observations']], [c['tables'] for c in source['helper_section_cases']]),
            same_whole([o['decoded_state'] for o in common['state_observations']], [c['state'] for c in source['state_cases']]),
            same_whole([o['retained_context'] for o in common['state_observations']], [c['context'] for c in source['state_cases']]),
        ]
    except (KeyError, TypeError, IndexError):
        return False
    return all(checks)


def decode(manifest, common_output, internal_output, wire_output, report):
    m = base.load(manifest)
    keys(m, ('source_input', 'printed'))
    if not same_whole(m['source_input'], INPUT_PIN):
        raise ValueError('source input operand differs')
    source = base.loads(base.checked(m['source_input']))
    if source.get('profile') != PROFILE or grouped_case_ids(source) != CASE_IDS:
        raise ValueError('source profile/case binding differs')
    raw = base.checked(m['printed']).decode('utf-8')
    parsed = base.codec.parse_printed(base.complete_term(raw), 'CSObservations')
    internal = base.tree(parsed)
    if internal.get('sort') != 'CSObservations':
        raise ValueError('complete CSObservations root differs')
    wire = tree_to_wire(internal)
    if not same_whole(wire_to_tree(wire), internal):
        raise ValueError('AST wire translation not lossless')
    common = state_ports.to_json(wire)
    if list(common) != ROOT_KEYS:
        raise ValueError('complete common eight-root document differs')
    identity = source_identity(common, source)
    common_pin = base.write(common_output, (json.dumps(common, indent=2, allow_nan=False) + '\n').encode())
    internal_pin = base.write(internal_output, (json.dumps(internal, indent=2, allow_nan=False) + '\n').encode())
    wire_pin = base.write(wire_output, (json.dumps(wire, indent=2, allow_nan=False) + '\n').encode())
    products = {'complete_actual_common': common_pin, 'complete_actual_internal': internal_pin, 'complete_actual_oracle_wire': wire_pin}
    status = CAPTURE_PASS if identity else CAPTURE_FAIL
    counts = {'helper_record_cases': len(common['helper_record_observations']), 'result_record_cases': len(common['result_record_observations']), 'task_record_cases': len(common['task_record_observations']), 'helper_section_cases': len(common['helper_section_observations']), 'state_cases': len(common['state_observations'])}
    result = {'status': status, 'source_input': m['source_input'], 'printed': m['printed'], 'products': products, 'product': common_pin, 'profile': PROFILE, 'case_ids': common['case_ids'], 'module': MODULE, 'sort': RESULT_SORT, 'observation_count': 46, 'observation_counts': counts, 'source_input_whole_identity': identity, 'decoded_source_whole_equal': identity, 'source_case_ids_whole_equal': same_whole(common['case_ids'], CASE_IDS), 'decoded_state_and_context_whole_equal': identity, 'expected_body_reads': 0, 'semantic_equality': 'PENDING independent whole commonJSON and typedAST comparison'}
    report_pin = base.write(report, (json.dumps(result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': status, 'products': products, 'report': report_pin}))


def compare_manifest(m):
    keys(m, ('commonJSON', 'typedAST', 'durable_freeze'))
    roles = {}
    for role, want in (('commonJSON', EXPECTED_COMMON_PIN), ('typedAST', EXPECTED_AST_PIN)):
        keys(m[role], ('actual', 'expected'))
        if not same_whole(m[role]['expected'], want):
            raise ValueError('opaque expected operand differs: ' + role)
        roles[role] = m[role]
    barrier = base.loads(base.checked(m['durable_freeze']))
    if type(barrier) is not dict or type(barrier.get('expected_body_reads')) is not int or barrier['expected_body_reads'] != 0:
        raise ValueError('barrier expected_body_reads differs')
    if barrier.get('expected_body_read_scope') != COMPARE_SCOPE or type(barrier.get('members')) is not list:
        raise ValueError('barrier scope/members differ')
    for role in roles:
        if roles[role]['actual'] not in barrier['members']:
            raise ValueError('actual absent from barrier: ' + role)
        if base.pin(roles[role]['actual']['path']) != roles[role]['actual']:
            raise ValueError('actual pin differs: ' + role)
    out, overall = {}, True
    for role in ('commonJSON', 'typedAST'):
        actual = base.loads(base.checked(roles[role]['actual']))
        expected = base.loads(base.checked(roles[role]['expected']))
        equal = same_whole(actual, expected)
        overall = overall and equal
        out[role] = {'whole_typed_equal': equal, 'actual': roles[role]['actual'], 'expected': roles[role]['expected']}
    return {'status': COMPARE_PASS if overall else 'FAIL', 'whole_typed_equal': overall, 'commonJSON': out['commonJSON'], 'typedAST': out['typedAST'], 'expected_body_reads': 2, 'scope': COMPARE_SCOPE}


def compare(manifest):
    result = compare_manifest(base.load(manifest))
    print(json.dumps(result))
    raise SystemExit(0 if result['whole_typed_equal'] else 1)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest='action', required=True)
    x = sub.add_parser('render'); x.add_argument('manifest'); x.add_argument('output'); x.add_argument('report')
    x = sub.add_parser('decode'); x.add_argument('manifest'); x.add_argument('common_output'); x.add_argument('internal_output'); x.add_argument('wire_output'); x.add_argument('report')
    x = sub.add_parser('compare'); x.add_argument('manifest')
    a = cli.parse_args()
    if a.action == 'render':
        render(a.manifest, a.output, a.report)
    elif a.action == 'decode':
        decode(a.manifest, a.common_output, a.internal_output, a.wire_output, a.report)
    else:
        compare(a.manifest)


if __name__ == '__main__':
    main()
