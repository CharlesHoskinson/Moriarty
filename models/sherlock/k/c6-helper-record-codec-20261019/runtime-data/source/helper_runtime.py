"""Positive C6 helper/result data carrier and consumer.

No lane encoder, K model, helper transition, or expected reconstruction is
implemented in Python.  Encoded lanes are extracted only from the actual
CTLanes chain returned by the K codec.  Parser, framing, pin, unique-JSON,
and typed-tree policy are reused from the approved R2/base APIs.
"""
import argparse
import copy
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
HELPER_SOURCE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/source')
sys.path.insert(0, str(BASE / 'source'))
sys.path.insert(0, str(HELPER_SOURCE))
import c6_data as base
import helper_ports

INPUT_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/Chunk3B-positive-helper-result-20261020/SAFE/INPUTS.json', 'bytes': 11221, 'sha256': 'd824cd01c0be8b9668bed718d8db671db7c5f22d9532714d1df043770a365633'}
REGISTRY_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/SAFE/COMBINED-PUBLIC-REGISTRY.json', 'bytes': 27795, 'sha256': '4bafc83f651e15c5da4c518556a27f45ad121aa7959f6b715575755acb00362e'}
LABEL_MAP_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json', 'bytes': 270342, 'sha256': 'd1b6122c6d5601cd86fb90f822f6bf1921ec2cff5c57a5057d863415a9c73be9'}
EXPECTED_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/Chunk3B-positive-helper-result-20261020/private/EXPECTED-POSITIVE-RECORD-VECTORS.json', 'bytes': 8389, 'sha256': '65faae7414524b6dfb133a16d994176263e9f40cd90df6f4a6e71eae176e3f36'}

OUTPUT_SCHEMA = 'sherlock-c6-helper-record-positive-codec-wire/1'
PROFILE = 'logical-C6-helper-record-positive-codec/1'
RENDER_PASS = 'PASS_LOSSLESS_C6_HELPER_QUERY_CARRIER'
CAPTURE_PASS = 'PASS_COMPLETE_C6_HELPER_ANSWER_CAPTURE'
CAPTURE_FAIL_DECODED = 'FAIL_DECODED_SOURCE_WHOLE_CARRIER_MISMATCH'
COMPARE_PASS = 'PASS_WHOLE_TYPED_C6_HELPER_CODEC'
HELPER_WIDTH = 23
RESULT_WIDTH = 4
HELPER_COUNT = 11
RESULT_COUNT = 20
TOTAL_COUNT = 31

FIXED_WRAPPER_SCHEMA = 'sherlock-independent-3b-positive-record-expected/1'
FIXED_WRAPPER_SCOPE = 'Complete derivable record lane vectors; Scalar helper unresolved, no section/task expectations.'
FIXED_MISSING = {
    'carrier': 'HelperControlC::Scalar / ScalarStaged',
    'exact_missing_premise': 'Numeric ScalarStaged tag-to-variant mapping for None, U128Value, BoolValue, PendingNode, PrimError, Selector and TypeTest; header supplies tag range and width, not this mapping.',
    'consequence': 'No Scalar helper vector or ScalarStaged-specific vector is derived, including None. No declaration-order-as-tag guess or producer inspection.',
    'public_variants': ['None', 'U128Value', 'BoolValue', 'PendingNode', 'PrimError', 'Selector', 'TypeTest'],
    'fullwide_staged_value': 'U128Value full-width payload remains owed with its tag map; common.origin_reference_cursor full-width transport is independently covered here.',
}

HELPER_VARIANTS = {
    'chScalar': 'Scalar', 'chEnvGet': 'EnvGet', 'chEnvPut': 'EnvPut',
    'chFind': 'Find', 'chBind': 'Bind', 'chAppend': 'Append',
    'chValidate': 'Validate', 'chEqual': 'Equal', 'chMatch': 'Match',
    'chPlan': 'Plan', 'chBuild': 'Build',
}
RESULT_VARIANTS = {
    'chResultValidated': 'Validated', 'chResultEnvFound': 'EnvFound',
    'chResultEnvMissing': 'EnvMissing', 'chResultEnvUpdated': 'EnvUpdated',
    'chResultFindFound': 'FindFound', 'chResultFindMissing': 'FindMissing',
    'chResultBindOk': 'BindOk', 'chResultBindBad': 'BindBad',
    'chResultAppendDone': 'AppendDone', 'chResultScalarValue': 'ScalarValue',
    'chResultScalarError': 'ScalarError',
    'chResultScalarSelector': 'ScalarSelector',
    'chResultScalarTypeTest': 'ScalarTypeTest',
    'chResultEqualDone': 'EqualDone', 'chResultMatchMatched': 'MatchMatched',
    'chResultMatchNoMatch': 'MatchNoMatch',
    'chResultPlanMatched': 'PlanMatched', 'chResultPlanNoMatch': 'PlanNoMatch',
    'chResultBuildDone': 'BuildDone',
}

PUBLIC = base.loads(base.checked(REGISTRY_PIN))
DATA_REG = {r['name']: (r['sort'], tuple(r['argument_sorts'])) for r in PUBLIC['constructors']}
ENTRY = PUBLIC['function_entry']
ENTRY_REG = {ENTRY['name']: (ENTRY['sort'], tuple(ENTRY['argument_sorts']))}
if len(DATA_REG) != 208:
    raise ValueError('closed DATA208 registry required')
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
        raise ValueError('outside closed carrier profile (including Map/Ref/symbolic)')
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


def source_cases(document):
    if type(document) is not dict:
        raise ValueError('source document object required')
    helpers = document.get('helper_cases')
    results = document.get('result_cases')
    if type(helpers) is not list or type(results) is not list:
        raise ValueError('source helper/result case lists required')
    if len(helpers) != HELPER_COUNT or len(results) != RESULT_COUNT:
        raise ValueError('complete 11-helper/20-result/31-total cohort required')
    seen = set()
    for kind, rows in (('helper', helpers), ('result', results)):
        for row in rows:
            if type(row) is not dict or type(row.get('case_id')) is not str or row['case_id'] in seen:
                raise ValueError('case identity differs or duplicates')
            seen.add(row['case_id'])
            carrier = row.get(kind)
            if carrier is not None:
                if type(carrier) is not dict or len(carrier) != 1:
                    raise ValueError('externally tagged carrier required')
                if kind == 'helper' and next(iter(carrier)) == 'Scalar':
                    raise ValueError('Scalar helper excluded from this 31 cohort')
    return helpers, results


def flatten_queries(queries_tree):
    rows = []
    node = queries_tree
    while True:
        keys(node, ('sort', 'constructor', 'arguments'))
        if node['sort'] != 'CHQueries':
            raise ValueError('CHQueries chain required')
        if node['constructor'] == 'chNoQueries':
            if node['arguments'] != []:
                raise ValueError('empty CHQueries arity differs')
            return rows
        if node['constructor'] not in ('chHelperQuery', 'chResultQuery') or len(node['arguments']) != 3:
            raise ValueError('CHQueries row differs')
        case_node, option, tail = node['arguments']
        if case_node.get('sort') != 'String' or type(case_node.get('value')) is not str:
            raise ValueError('query case identity differs')
        rows.append((node['constructor'], case_node['value'], option))
        node = tail


def carrier(document):
    source_cases(document)
    ported = helper_ports.port_document(document)
    saved = base.codec.REG
    try:
        base.codec.REG = {**DATA_REG, **ENTRY_REG}
        queries_term = from_tree(ported['queries'])
        if queries_term.sort != 'CHQueries':
            raise ValueError('complete CHQueries source term required')
        term = base.codec.C(ENTRY['name'], queries_term)
        wire = envelope(term)
        if not base.same(base.tree(term), base.tree(decode_document(wire, 'CHAnswers'))):
            raise ValueError('whole source query/KJSON typed inverse differs')
        return term, wire, ported
    finally:
        base.codec.REG = saved


def lanes_from_tree(node):
    lanes = []
    while True:
        keys(node, ('sort', 'constructor', 'arguments'))
        if node['sort'] != 'CTLanes':
            raise ValueError('CTLanes chain required')
        if node['constructor'] == 'ctNoLanes':
            if node['arguments'] != []:
                raise ValueError('ctNoLanes arity differs')
            return lanes
        if node['constructor'] != 'ctLane' or len(node['arguments']) != 2:
            raise ValueError('ctLane chain differs')
        head, tail = node['arguments']
        keys(head, ('sort', 'value'))
        if head['sort'] != 'Int' or type(head['value']) is not int:
            raise ValueError('native exact integer lane required')
        lanes.append(head['value'])
        node = tail


def variant_from_option(option, kind):
    keys(option, ('sort', 'constructor', 'arguments'))
    if kind == 'helper':
        none_name, some_name, inner_sort, mapping = 'chNoHelper', 'chSomeHelper', 'CHHelper', HELPER_VARIANTS
        if option['sort'] != 'CHHelperOption':
            raise ValueError('CHHelperOption required')
    else:
        none_name, some_name, inner_sort, mapping = 'chNoResult', 'chSomeResult', 'CHResult', RESULT_VARIANTS
        if option['sort'] != 'CHResultOption':
            raise ValueError('CHResultOption required')
    if option['constructor'] == none_name:
        if option['arguments'] != []:
            raise ValueError('absent carrier arity differs')
        return None
    if option['constructor'] != some_name or len(option['arguments']) != 1:
        raise ValueError('present carrier option differs')
    inner = option['arguments'][0]
    keys(inner, ('sort', 'constructor', 'arguments'))
    if inner['sort'] != inner_sort or inner['constructor'] not in mapping:
        raise ValueError('actual carrier variant differs')
    return mapping[inner['constructor']]


def decoded_source_whole_carrier_equal(answers):
    for answer in answers:
        keys(answer, ('sort', 'constructor', 'arguments'))
        if answer['sort'] != 'CHAnswer' or len(answer['arguments']) != 4:
            raise ValueError('complete CHAnswer required')
    return all(same_whole(a['arguments'][3], a['arguments'][1]) for a in answers)


def build_vectors(answers):
    helper_vectors = []
    result_vectors = []
    for answer in answers:
        keys(answer, ('sort', 'constructor', 'arguments'))
        if answer['sort'] != 'CHAnswer' or len(answer['arguments']) != 4:
            raise ValueError('complete CHAnswer required')
        case_node, option, lanes_node, _decoded = answer['arguments']
        keys(case_node, ('sort', 'value'))
        if case_node['sort'] != 'String' or type(case_node['value']) is not str:
            raise ValueError('answer case identity differs')
        lanes = lanes_from_tree(lanes_node)
        if answer['constructor'] == 'chHelperAnswer':
            variant = variant_from_option(option, 'helper')
            if variant == 'Scalar':
                raise ValueError('Scalar helper excluded from this 31 cohort')
            if len(lanes) != HELPER_WIDTH:
                raise ValueError('complete 23-lane helper width required')
            helper_vectors.append({'case_id': case_node['value'], 'variant': variant, 'lanes': lanes})
        elif answer['constructor'] == 'chResultAnswer':
            variant = variant_from_option(option, 'result')
            if len(lanes) != RESULT_WIDTH:
                raise ValueError('complete 4-lane result width required')
            result_vectors.append({'case_id': case_node['value'], 'variant': variant, 'lanes': lanes})
        else:
            raise ValueError('CHAnswer variant differs')
    return helper_vectors, result_vectors


def build_complete_wire(answers):
    return {'schema': OUTPUT_SCHEMA, 'profile': PROFILE, 'answers': list(answers)}


def build_wrapper(helper_vectors, result_vectors):
    return {'schema': FIXED_WRAPPER_SCHEMA, 'scope': FIXED_WRAPPER_SCOPE,
            'helper_vectors': helper_vectors, 'result_vectors': result_vectors,
            'missing': copy.deepcopy(FIXED_MISSING)}


def render(manifest, output, report):
    m = base.load(manifest)
    keys(m, ('source_input', 'compiled_definition'))
    if not same_whole(m['source_input'], INPUT_PIN):
        raise ValueError('source input operand differs')
    source = base.loads(base.checked(m['source_input']))
    definition = base.loads(base.checked(LABEL_MAP_PIN))['definition']
    if m['compiled_definition'] != definition:
        raise ValueError('compiled definition operand differs')
    base.checked(definition)
    _term, wire, _ported = carrier(source)
    helpers, results = source_cases(source)
    product = base.write(output, (json.dumps(wire, separators=(',', ':'), allow_nan=False) + '\n').encode())
    result = {'status': RENDER_PASS, 'source_input': m['source_input'],
              'compiled_definition': definition, 'compiled_label_map': LABEL_MAP_PIN,
              'actual_label_count': len(LABELS), 'data_label_count': len(DATA_REG),
              'helper_case_count': len(helpers), 'result_case_count': len(results),
              'case_count': len(helpers) + len(results), 'same_whole_typed_query': True,
              'product': product, 'expected_body_reads': 0,
              'no_model_or_lane_evaluation_in_Python': True}
    report_pin = base.write(report, (json.dumps(result, indent=2) + '\n').encode())
    print(json.dumps({'status': result['status'], 'product': product, 'report': report_pin}))


def decode(manifest, vectors_output, answers_output, report):
    m = base.load(manifest)
    keys(m, ('source_input', 'printed'))
    if not same_whole(m['source_input'], INPUT_PIN):
        raise ValueError('source input operand differs')
    source = base.loads(base.checked(m['source_input']))
    helpers, results = source_cases(source)
    ported = helper_ports.port_document(source)
    expected_rows = flatten_queries(ported['queries'])
    if len(expected_rows) != TOTAL_COUNT:
        raise ValueError('complete 31-query field port required')
    raw = base.checked(m['printed']).decode('utf-8')
    parsed = base.codec.parse_printed(base.complete_term(raw), 'CHAnswers')
    parsed_answers = []
    at = parsed
    while at.name == 'chAnswerRow':
        if len(at.args) != 2:
            raise ValueError('CHAnswers row arity differs')
        parsed_answers.append(at.args[0])
        at = at.args[1]
    if at.name != 'chNoAnswers' or len(parsed_answers) != TOTAL_COUNT:
        raise ValueError('complete 31-answer count differs')
    answers = [base.tree(a) for a in parsed_answers]
    for answer, (query_name, case_id, source_option) in zip(answers, expected_rows):
        want_constructor = 'chHelperAnswer' if query_name == 'chHelperQuery' else 'chResultAnswer'
        if answer['constructor'] != want_constructor or len(answer['arguments']) != 4:
            raise ValueError('answer kind/order differs')
        case_node, actual_option, _lanes, _decoded = answer['arguments']
        if not same_whole(case_node, {'sort': 'String', 'value': case_id}):
            raise ValueError('answer case/order identity differs')
        if not same_whole(actual_option, source_option):
            raise ValueError('answer source original typed carrier differs')
    decoded_source_equal = decoded_source_whole_carrier_equal(answers)
    helper_vectors, result_vectors = build_vectors(answers)
    if len(helper_vectors) != len(helpers) or len(result_vectors) != len(results):
        raise ValueError('complete helper/result vector counts differ')
    if [r['case_id'] for r in helper_vectors] != [r['case_id'] for r in helpers]:
        raise ValueError('helper vector order differs')
    if [r['case_id'] for r in result_vectors] != [r['case_id'] for r in results]:
        raise ValueError('result vector order differs')
    wrapper = build_wrapper(helper_vectors, result_vectors)
    complete = build_complete_wire(answers)
    vectors_pin = base.write(vectors_output, (json.dumps(wrapper, indent=2, allow_nan=False) + '\n').encode())
    answers_pin = base.write(answers_output, (json.dumps(complete, indent=2, allow_nan=False) + '\n').encode())
    products = {'complete_actual_record_vectors': vectors_pin, 'complete_actual_CHAnswers': answers_pin}
    status = CAPTURE_PASS if decoded_source_equal else CAPTURE_FAIL_DECODED
    result = {'status': status, 'source_input': m['source_input'], 'printed': m['printed'],
              'products': products, 'product': vectors_pin,
              'helper_case_count': len(helper_vectors), 'result_case_count': len(result_vectors),
              'case_count': len(answers), 'all_source_original_carriers_retained': True,
              'all_actual_encoded_lanes_and_decoded_carriers_retained': True,
              'decoded_source_whole_carrier_equal': decoded_source_equal,
              'expected_body_reads': 0,
              'semantic_equality': 'PENDING independent whole vector comparison; no Python lane calculation'}
    report_pin = base.write(report, (json.dumps(result, indent=2) + '\n').encode())
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
                      'scope': 'Complete 5-key helper/result vector wrapper only; no Scalar helper, interpreter, native, or refinement claim'}))
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
    p.add_argument('vectors_output')
    p.add_argument('answers_output')
    p.add_argument('report')
    p = sub.add_parser('compare')
    p.add_argument('manifest')
    a = cli.parse_args()
    if a.action == 'render':
        render(a.manifest, a.output, a.report)
    elif a.action == 'decode':
        decode(a.manifest, a.vectors_output, a.answers_output, a.report)
    else:
        compare(a.manifest)


if __name__ == '__main__':
    main()
