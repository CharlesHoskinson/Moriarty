"""Positive C6 padded DATA field-only runtime consumer.

No lane encoder, capacity arithmetic, K model, expected reconstruction, or
new parser/guard/comparator policy is implemented in Python. Parser, framing,
pin, unique-JSON, and typed-tree policy are reused from the existing R2/base
APIs. The strict printed-term parser sees DATA26 constructors only. The
temporary dpObserve entry is admitted only while rendering the query and is
removed from the codec registry in a finally block.
"""
import argparse
import json
from pathlib import Path
import sys

SOURCE_DIR = Path(__file__).resolve().parent
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
sys.path.insert(0, str(BASE / 'source'))
sys.path.insert(0, str(SOURCE_DIR))
import c6_data as base
import data_ports

INPUT_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch34-data-positive-padded-source-task-20261020/SAFE-INPUT-ONLY.proposed.json', 'bytes': 2512, 'sha256': 'c1a694df525a79d158c4a07bb0ac5ecaf12083890fc8d51d74fa82717023ae2e'}
REGISTRY_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/SAFE/PUBLIC-REGISTRY.json', 'bytes': 3775, 'sha256': 'ec4c9990b7d792dd1a41e7daebba954e7d9659961f09612959211328be2e232a'}
LABEL_MAP_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json', 'bytes': 36504, 'sha256': '67b6475ae68e3fd17e425bc7b7b17804500d56bc4eee16f9f8a3a212a98e26c1'}
EXPECTED_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/B34-positive-padded-DATA-20261020/private/EXPECTED-COMPLETE-OBSERVATION.json', 'bytes': 2353, 'sha256': 'a101bc8859d93ed33c70658c52083d2458d4256fdd10414a7e4482acd04cc8f3'}

PROFILE = 'sherlock-c6-data-positive-padded/1'
CASE_ID = 'seven-variants-partial-unequal-capacities-01'
MODULE = 'SHERLOCK-C6-POSITIVE-PADDED-DATA-LAWS'
RESULT_SORT = 'DPObservation'
RENDER_PASS = 'PASS_LOSSLESS_C6_DATA_QUERY_CARRIER'
CAPTURE_PASS = 'PASS_COMPLETE_C6_DATA_OBSERVATION_CAPTURE'
CAPTURE_FAIL_DECODED = 'FAIL_DECODED_SOURCE_WHOLE_TABLE_MISMATCH'
COMPARE_PASS = 'PASS_WHOLE_TYPED_C6_DATA_OBSERVATION'

PUBLIC = base.loads(base.checked(REGISTRY_PIN))
DATA_REG = {r['name']: (r['sort'], tuple(r['argument_sorts'])) for r in PUBLIC['constructors']}
ENTRY = PUBLIC['function_entry']
ENTRY_REG = {ENTRY['name']: (ENTRY['sort'], tuple(ENTRY['argument_sorts']))}
if len(DATA_REG) != 26:
    raise ValueError('closed DATA26 registry required')
if ENTRY != {'name': 'dpObserve', 'sort': 'DPObservation', 'argument_sorts': ['DPCaps', 'DPTables']}:
    raise ValueError('dpObserve entry declaration differs')
base.codec.REG = dict(DATA_REG)


def keys(value, want):
    if type(value) is not dict or set(value) != set(want):
        raise ValueError('exact JSON carrier keyset differs')
    return value


def kore_label(raw):
    codes = {'(': 'LPar', ')': 'RPar', ',': 'Comm', '_': 'Unds'}
    result = 'Lbl'
    quoted = False
    for char in raw:
        if char.isascii() and (char.isalnum() or char == '-'):
            if quoted:
                result += "'"
                quoted = False
            result += char
        else:
            if char not in codes:
                raise ValueError('outside declared compiled-label alphabet')
            if not quoted:
                result += "'"
                quoted = True
            result += codes[char]
    return result + ("'" if quoted else '')


def validate_label_map(data):
    keys(data, ('status', 'definition', 'constructor_count', 'constructors', 'surface_registry_scope', 'method'))
    if type(data['constructors']) is not list or type(data['constructor_count']) is not int or data['constructor_count'] != len(base.codec.REG):
        raise ValueError('complete compiled label map count required')
    lines = base.checked(data['definition']).decode('utf-8').splitlines()
    forward = {}
    reverse = {}
    for row in data['constructors']:
        keys(row, ('surface_constructor', 'sort', 'argument_sorts', 'declaring_module', 'declaration_source',
                   'compiled_klabel', 'kore_symbol', 'declaration_line', 'complete_symbol_declaration'))
        name = row['surface_constructor']
        raw = row['compiled_klabel']
        if type(name) is not str or name not in base.codec.REG or name in forward or type(raw) is not str or raw in reverse:
            raise ValueError('unknown/duplicate compiled label declaration')
        sort, sorts = base.codec.REG[name]
        if row['sort'] != sort or type(row['argument_sorts']) is not list or tuple(row['argument_sorts']) != sorts:
            raise ValueError('compiled label sort/arguments differ')
        module = row['declaring_module']
        if type(module) is not str or not module:
            raise ValueError('declaring module required')
        declared = name + '(' + ','.join('_' for _ in sorts) + ')_' + module + '_' + sort + ''.join('_' + s for s in sorts)
        if raw != declared or row['kore_symbol'] != kore_label(raw):
            raise ValueError('compiled raw/symbol mapping differs')
        line = row['declaration_line']
        body = row['complete_symbol_declaration']
        if (type(line) is not int or not 1 <= line <= len(lines) or type(body) is not str or lines[line - 1] != body
                or not body.startswith('  symbol ' + row['kore_symbol'] + '{}(')):
            raise ValueError('actual compiled symbol declaration membership differs')
        signature = '(' + ', '.join('Sort' + s + '{}' for s in sorts) + ') : Sort' + sort + '{} '
        if '{}' + signature not in body:
            raise ValueError('actual compiled symbol signature differs')
        base.checked(row['declaration_source'])
        forward[name] = raw
        reverse[raw] = name
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
        raise ValueError('outside closed DATA carrier profile')
    sort, sorts = base.codec.REG[term.name]
    if term.sort != sort or tuple(x.sort for x in term.args) != sorts:
        raise ValueError('typed constructor differs')
    if term.name not in LABELS:
        raise ValueError('constructor lacks compiled label mapping')
    return {'node': 'KApply', 'label': {'node': 'KLabel', 'name': LABELS[term.name], 'params': []},
            'arity': len(term.args), 'args': [encode_node(x) for x in term.args]}


def envelope(term):
    return {'format': 'KAST', 'version': 4, 'term': encode_node(term)}


def decode_node(value, sort):
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
    actual, sorts = base.codec.REG[name]
    if actual != sort or type(value['arity']) is not int or value['arity'] != len(sorts) or type(value['args']) is not list or len(value['args']) != len(sorts):
        raise ValueError('exact KApply sort/arity/argument count differs')
    return base.codec.C(name, *(decode_node(x, s) for x, s in zip(value['args'], sorts)))


def decode_document(value, sort):
    keys(value, ('format', 'version', 'term'))
    if value['format'] != 'KAST' or type(value['version']) is not int or value['version'] != 4:
        raise ValueError('KAST JSONv4 required')
    return decode_node(value['term'], sort)


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


def carrier(document):
    if type(document) is not dict:
        raise ValueError('source document object required')
    ported = data_ports.port_input(document)
    if ported['profile'] != PROFILE or ported['case_id'] != CASE_ID:
        raise ValueError('source profile/case binding differs')
    saved = base.codec.REG
    try:
        base.codec.REG = {**DATA_REG, **ENTRY_REG}
        term = from_tree(ported['query'])
        if term.sort != 'DPObservation' or term.name != 'dpObserve':
            raise ValueError('complete dpObserve source term required')
        wire = envelope(term)
        if not base.same(base.tree(term), base.tree(decode_document(wire, 'DPObservation'))):
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
    result = {'status': RENDER_PASS, 'source_input': m['source_input'],
              'compiled_definition': definition, 'compiled_label_map': LABEL_MAP_PIN,
              'actual_label_count': len(LABELS), 'data_label_count': len(DATA_REG),
              'profile': ported['profile'], 'case_id': ported['case_id'],
              'module': MODULE, 'sort': RESULT_SORT,
              'same_whole_typed_query': True, 'product': product,
              'expected_body_reads': 0, 'no_model_or_lane_evaluation_in_Python': True}
    report_pin = base.write(report, (json.dumps(result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': result['status'], 'product': product, 'report': report_pin}))


def decode(manifest, observation_output, typed_output, report):
    m = base.load(manifest)
    keys(m, ('source_input', 'printed'))
    if not same_whole(m['source_input'], INPUT_PIN):
        raise ValueError('source input operand differs')
    source = base.loads(base.checked(m['source_input']))
    if source.get('profile') != PROFILE or source.get('caseId') != CASE_ID:
        raise ValueError('source profile/case binding differs')
    raw = base.checked(m['printed']).decode('utf-8')
    parsed = base.codec.parse_printed(base.complete_term(raw), 'DPObservation')
    typed_tree = base.tree(parsed)
    if typed_tree.get('sort') != 'DPObservation' or typed_tree.get('constructor') != 'dpObservation' or len(typed_tree.get('arguments', [])) != 4:
        raise ValueError('complete DPObservation root differs')
    actual = data_ports.observation_document(typed_tree)
    if list(actual) != ['geometry', 'counts', 'padded_lanes', 'decoded_tables']:
        raise ValueError('complete four-key observation differs')
    decoded_source_equal = same_whole(actual['decoded_tables'], source['tables'])
    observation_pin = base.write(observation_output, (json.dumps(actual, indent=2, allow_nan=False) + '\n').encode())
    typed_pin = base.write(typed_output, (json.dumps(typed_tree, indent=2, allow_nan=False) + '\n').encode())
    products = {'complete_actual_observation': observation_pin, 'complete_actual_DPObservation': typed_pin}
    status = CAPTURE_PASS if decoded_source_equal else CAPTURE_FAIL_DECODED
    result = {'status': status, 'source_input': m['source_input'], 'printed': m['printed'],
              'products': products, 'product': observation_pin,
              'profile': PROFILE, 'case_id': CASE_ID, 'module': MODULE, 'sort': RESULT_SORT,
              'all_actual_geometry_counts_padded_lanes_and_decoded_tables_retained': True,
              'all_complete_typed_fields_retained': True,
              'decoded_source_whole_table_equal': decoded_source_equal,
              'expected_body_reads': 0,
              'semantic_equality': 'PENDING independent whole observation comparison; no Python lane calculation'}
    report_pin = base.write(report, (json.dumps(result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': result['status'], 'products': products, 'report': report_pin}))


def compare(manifest):
    m = base.load(manifest)
    keys(m, ('actual', 'expected', 'durable_freeze'))
    if not same_whole(m['expected'], EXPECTED_PIN):
        raise ValueError('opaque expected operand differs')
    barrier = base.loads(base.checked(m['durable_freeze']))
    base.validate_barrier(barrier, m['actual'])
    actual = base.loads(base.checked(m['actual']))
    expected = base.loads(base.checked(m['expected']))
    equal = same_whole(actual, expected)
    print(json.dumps({'status': COMPARE_PASS if equal else 'FAIL',
                      'whole_typed_equal': equal, 'actual': m['actual'], 'expected': m['expected'],
                      'expected_body_reads': 1,
                      'scope': 'Complete four-key padded DATA observation only; no profile/case wrapper, native, or refinement claim'}))
    raise SystemExit(0 if equal else 1)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest='action', required=True)
    p = sub.add_parser('render')
    p.add_argument('manifest')
    p.add_argument('output')
    p.add_argument('report')
    p = sub.add_parser('decode')
    p.add_argument('manifest')
    p.add_argument('observation_output')
    p.add_argument('typed_output')
    p.add_argument('report')
    p = sub.add_parser('compare')
    p.add_argument('manifest')
    a = cli.parse_args()
    if a.action == 'render':
        render(a.manifest, a.output, a.report)
    elif a.action == 'decode':
        decode(a.manifest, a.observation_output, a.typed_output, a.report)
    else:
        compare(a.manifest)


if __name__ == '__main__':
    main()
