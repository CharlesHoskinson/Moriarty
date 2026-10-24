"""EnvGet three-event traversal renderer / strict printed-window decoder.

Bounded adaptation of the reviewed corrected EnvGet renderer/strict decoder
to the current317 registry and EGTraversalWindow. Two direct-file commands:

  render BEFORE_NATIVE_JSON QUERY_KAST_JSON INVARIANT_REPORT_JSON
  decode BEFORE_NATIVE_JSON KPRINT_TEXT NATIVE_WINDOW_JSON
         INTERNAL_AST_JSON WIRE_AST_JSON CAPTURE_REPORT_JSON

Render builds egThreeEventTraversal(actual EGSnapshot) from the sole genuine
before and proves a whole typed query inverse including the entry function.
Decode strictly parses one whole printed egTraversalWindow of DATA only:
all 37 functions, including egThreeEventTraversal, are refused as final DATA.
It emits before/after_miss/after_hit/after_exit plus three 28-field events,
and reports whole typed equality of decoded before against the sole input.

BEFORE_PIN is literal None/UNBOUND: no genuine new finalVarx actual-before
has been produced or released. Both commands fail closed until Root fills
that single literal from a separate reviewed real capture. No expected
operand and no Python transition calculation is used. Capture is not
equality evidence against any independent expectation. The original old
prefix MISMATCH remains historical.
"""
import argparse
from contextlib import contextmanager
import importlib.util
import json
from pathlib import Path
import sys

BEFORE_PIN = None  # UNBOUND: Root post-capture fill only; see module docstring.
REGISTRY_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-miss-hit-EnvGetExit-owner-positive-20261020/revision02-postreview-discriminatory-law01/postcompile-label-qualification01/PUBLIC-COMBINED-CURRENT317-REGISTRY.json', 'bytes': 118982, 'sha256': '91a941c4b525d28ffeccfa49a5a8006d4316f4448e19001e66fd8e98895f71de'}
LABEL_MAP_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-miss-hit-EnvGetExit-owner-positive-20261020/revision02-postreview-discriminatory-law01/postcompile-label-qualification01/COMPACT-ACTUAL317-TRANSPORT-MAP.json', 'bytes': 258170, 'sha256': 'f4fe10783f82335ebf0e37280fca7c6d92784f576132464a4777d14029950cac'}
PORTS_PATH = Path(__file__).resolve().with_name('envget_ports.py')
BASE_SOURCE_DIR = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019/source')

sys.path.insert(0, str(BASE_SOURCE_DIR))
import c6_data as base

_spec = importlib.util.spec_from_file_location('envget_ports', PORTS_PATH)
ports = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ports)

ENTRY_NAME = 'egThreeEventTraversal'
ENTRY_DECL = ('EGTraversalWindow', ('EGSnapshot',))
WINDOW_DATA_NAME = 'egTraversalWindow'
WINDOW_DECL = ('EGTraversalWindow', ('EGSnapshot', 'EGSnapshot', 'EGSnapshot', 'EGSnapshot', 'EGEvent', 'EGEvent', 'EGEvent'))
PRIMITIVE_SORTS = ('Int', 'Bool', 'String')
RENDER_PASS = 'PASS_LOSSLESS_ENVGET_TRAVERSAL_BEFORE_QUERY'
CAPTURE_PASS = 'PASS_COMPLETE_ENVGET_TRAVERSAL_WINDOW_CAPTURE'
CAPTURE_FAIL = 'FAIL_DECODED_ENVGET_TRAVERSAL_BEFORE_IDENTITY'
UNBOUND_ERROR = 'genuine new actual-before UNBOUND: runtime ineligible until Root post-capture BEFORE_PIN fill'


def keys(value, want):
    if type(value) is not dict or set(value) != set(want):
        raise ValueError('exact JSON carrier keyset differs')
    return value


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


def _require_bound_before():
    if BEFORE_PIN is None:
        raise ValueError(UNBOUND_ERROR)
    return BEFORE_PIN


# ---------------------------------------------------------------- registry
PUBLIC = base.loads(base.checked(REGISTRY_PIN))
if type(PUBLIC) is not dict or PUBLIC.get('schema') != 'sherlock-EnvGet-traversal-combined-current-source-registry/1':
    raise ValueError('public current317 source declaration registry schema differs')
_ROWS = PUBLIC.get('constructors_and_functions')
if type(_ROWS) is not dict or len(_ROWS) != 317:
    raise ValueError('complete 317 source declaration registry required')
REG_ALL = {}
REG_ROLE = {}
for _name, _row in _ROWS.items():
    keys(_row, ('sort', 'argument_sorts', 'arity', 'source', 'role'))
    if (type(_name) is not str or type(_row['sort']) is not str
            or type(_row['argument_sorts']) is not list
            or _row['arity'] != len(_row['argument_sorts'])
            or type(_row['source']) is not str
            or type(_row['role']) is not str
            or _row['role'] not in ('DATA', 'function')):
        raise ValueError('source declaration registry row differs')
    REG_ALL[_name] = (_row['sort'], tuple(_row['argument_sorts']))
    REG_ROLE[_name] = _row['role']


def kore_label(raw):
    codes = {'(': 'LPar', ')': 'RPar', ',': 'Comm', '_': 'Unds'}
    result, quoted = 'Lbl', False
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
    keys(data, ('definition', 'declarations'))
    if type(data['declarations']) is not list or len(data['declarations']) != 317:
        raise ValueError('complete ACTUAL317 declaration list required')
    lines = base.checked(data['definition']).decode('utf-8').splitlines()
    lineset = set(lines)
    forward, reverse = {}, {}
    data_names, function_names = set(), set()
    for row in data['declarations']:
        keys(row, ('surface_name', 'sort', 'argument_sorts', 'arity', 'role',
                   'declaring_module', 'kore_symbol', 'compiled_klabel',
                   'explicit_klabel_attribute', 'Source', 'Location'))
        name, raw = row['surface_name'], row['compiled_klabel']
        if type(name) is not str or name not in REG_ALL or name in forward:
            raise ValueError('unknown/duplicate compiled label declaration')
        if type(raw) is not str or raw in reverse:
            raise ValueError('duplicate compiled raw label')
        sort, sorts = REG_ALL[name]
        if (row['sort'] != sort or tuple(row['argument_sorts']) != sorts
                or row['arity'] != len(sorts)):
            raise ValueError('compiled label sort/arguments differ')
        if row['role'] != REG_ROLE[name]:
            raise ValueError('compiled label role differs from source registry role')
        if row['role'] == 'DATA':
            data_names.add(name)
        elif row['role'] == 'function':
            function_names.add(name)
        else:
            raise ValueError('declaration role outside DATA/function')
        module = row['declaring_module']
        declared = (name + '(' + ','.join('_' for _ in sorts) + ')_' + module
                    + '_' + sort + ''.join('_' + s for s in sorts))
        if raw != declared or row['kore_symbol'] != kore_label(raw):
            raise ValueError('compiled raw/symbol mapping differs')
        explicit = row['explicit_klabel_attribute']
        if explicit is not None and explicit != name:
            raise ValueError('explicit klabel attribute differs from surface name')
        if type(row['Source']) is not str or not row['Source'].endswith('.k'):
            raise ValueError('declaration Source differs')
        loc = row['Location']
        if (type(loc) is not list or len(loc) != 4
                or any(type(x) is not int for x in loc)):
            raise ValueError('declaration Location differs')
        signature = '{}' + '(' + ', '.join('Sort' + s + '{}' for s in sorts) + ') : Sort' + sort + '{} '
        if not any(('  symbol ' + row['kore_symbol'] + '{}(') in line and signature in line
                   for line in lineset):
            raise ValueError('actual compiled symbol declaration membership differs')
        forward[name] = raw
        reverse[raw] = name
    if set(forward) != set(REG_ALL):
        raise ValueError('compiled label map missing registry entry')
    if len(data_names) != 280 or len(function_names) != 37:
        raise ValueError('ACTUAL317 DATA/function split differs')
    if REG_ALL[ENTRY_NAME] != ENTRY_DECL or ENTRY_NAME not in function_names:
        raise ValueError('egThreeEventTraversal entry declaration differs')
    if REG_ALL[WINDOW_DATA_NAME] != WINDOW_DECL or WINDOW_DATA_NAME not in data_names:
        raise ValueError('egTraversalWindow DATA declaration differs')
    return forward, reverse, data_names, function_names


LABELS, REVERSE_LABELS, DATA_NAMES, FUNCTION_NAMES = validate_label_map(
    base.loads(base.checked(LABEL_MAP_PIN)))
DATA_REG = {name: REG_ALL[name] for name in DATA_NAMES}
ALL_REG = dict(REG_ALL)
base.codec.REG = dict(DATA_REG)


@contextmanager
def _codec_reg(reg):
    saved = base.codec.REG
    base.codec.REG = reg
    try:
        yield
    finally:
        base.codec.REG = saved


@contextmanager
def _root_entry_codec_reg():
    reg = dict(DATA_REG)
    reg[ENTRY_NAME] = ALL_REG[ENTRY_NAME]
    with _codec_reg(reg):
        yield


def query_tree(term):
    if getattr(term, 'name', None) != ENTRY_NAME or getattr(term, 'sort', None) != 'EGTraversalWindow':
        raise ValueError('query tree egThreeEventTraversal root required')
    stack = list(term.args)
    while stack:
        node = stack.pop()
        if node.name in ('@Int', '@Bool', '@String'):
            continue
        if node.name not in DATA_REG:
            raise ValueError('residual/entry function refused below query root')
        stack.extend(node.args)
    with _root_entry_codec_reg():
        return base.tree(term)


# ------------------------------------------------------- wire <-> internal
def wire_to_internal(tree):
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed wire tree required')
    if tree['sort'] in PRIMITIVE_SORTS:
        keys(tree, ('sort', 'literal'))
        return {'sort': tree['sort'], 'value': tree['literal']}
    keys(tree, ('sort', 'constructor', 'arguments'))
    if type(tree['arguments']) is not list:
        raise ValueError('wire arguments list required')
    return {'sort': tree['sort'], 'constructor': tree['constructor'],
            'arguments': [wire_to_internal(x) for x in tree['arguments']]}


def internal_to_wire(tree):
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed internal tree required')
    if tree['sort'] in PRIMITIVE_SORTS:
        keys(tree, ('sort', 'value'))
        return {'sort': tree['sort'], 'literal': tree['value']}
    keys(tree, ('sort', 'constructor', 'arguments'))
    if type(tree['arguments']) is not list:
        raise ValueError('internal arguments list required')
    return {'sort': tree['sort'], 'constructor': tree['constructor'],
            'arguments': [internal_to_wire(x) for x in tree['arguments']]}


# ------------------------------------------------------------- typed terms
def _from_tree(tree, allow_entry, is_root):
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed tree required')
    sort = tree['sort']
    if sort in PRIMITIVE_SORTS:
        keys(tree, ('sort', 'value'))
        return base.encode(tree['value'], sort)
    keys(tree, ('sort', 'constructor', 'arguments'))
    name, args = tree['constructor'], tree['arguments']
    if type(name) is not str or name not in ALL_REG or type(args) is not list:
        raise ValueError('unknown typed constructor')
    if name not in DATA_REG and not (is_root and allow_entry and name == ENTRY_NAME):
        raise ValueError('residual/entry function refused as DATA')
    actual, sorts = ALL_REG[name]
    if sort != actual or len(args) != len(sorts):
        raise ValueError('typed tree sort/arity differs')
    children = [_from_tree(x, allow_entry, False) for x in args]
    if tuple(x.sort for x in children) != sorts:
        raise ValueError('child typed sorts differ')
    return base.codec.C(name, *children)


def from_tree(tree, allow_entry=False):
    with _codec_reg(dict(ALL_REG if allow_entry else DATA_REG)):
        return _from_tree(tree, allow_entry, True)


def _encode_node(term, allow_entry, is_root):
    if term.name in ('@Int', '@Bool', '@String'):
        expected = {'Int': int, 'Bool': bool, 'String': str}
        if term.sort not in expected or type(term.args[0]) is not expected[term.sort]:
            raise ValueError('scalar kind differs')
        return {'node': 'KToken',
                'sort': {'node': 'KSort', 'name': term.sort, 'params': []},
                'token': base.codec.render(term)}
    if term.name not in ALL_REG or term.name not in LABELS:
        raise ValueError('outside closed carrier profile')
    if term.name not in DATA_REG and not (is_root and allow_entry and term.name == ENTRY_NAME):
        raise ValueError('residual/entry function refused as DATA')
    sort, sorts = ALL_REG[term.name]
    if term.sort != sort or tuple(x.sort for x in term.args) != sorts:
        raise ValueError('typed constructor differs')
    return {'node': 'KApply',
            'label': {'node': 'KLabel', 'name': LABELS[term.name], 'params': []},
            'arity': len(term.args),
            'args': [_encode_node(x, allow_entry, False) for x in term.args]}


def encode_node(term, allow_entry=False):
    return _encode_node(term, allow_entry, True)


def envelope(term, allow_entry=False):
    return {'format': 'KAST', 'version': 4,
            'term': encode_node(term, allow_entry=allow_entry)}


def _decode_node(value, sort, allow_entry, is_root):
    if type(value) is not dict:
        raise ValueError('K node object required')
    if value.get('node') == 'KToken':
        keys(value, ('node', 'sort', 'token'))
        s = keys(value['sort'], ('node', 'name', 'params'))
        if (s != {'node': 'KSort', 'name': sort, 'params': []}
                or sort not in PRIMITIVE_SORTS or type(value['token']) is not str):
            raise ValueError('exact primitive KSort/token required')
        term = base.codec.parse_printed(value['token'], sort)
        if (type(term.args[0]) is not {'Int': int, 'Bool': bool, 'String': str}[sort]
                or base.codec.render(term) != value['token']):
            raise ValueError('noncanonical scalar lexeme')
        return term
    keys(value, ('node', 'label', 'arity', 'args'))
    if value['node'] != 'KApply':
        raise ValueError('outside KApply/KToken carrier profile')
    label = keys(value['label'], ('node', 'name', 'params'))
    if (label['node'] != 'KLabel' or label['params'] != []
            or type(label['name']) is not str or label['name'] not in REVERSE_LABELS):
        raise ValueError('unknown or parametric label')
    name = REVERSE_LABELS[label['name']]
    if name not in DATA_REG and not (is_root and allow_entry and name == ENTRY_NAME):
        raise ValueError('residual/entry function refused as output')
    actual, sorts = ALL_REG[name]
    if (actual != sort or type(value['arity']) is not int
            or value['arity'] != len(sorts) or type(value['args']) is not list
            or len(value['args']) != len(sorts)):
        raise ValueError('exact KApply sort/arity differs')
    return base.codec.C(name, *(_decode_node(x, s, allow_entry, False)
                                for x, s in zip(value['args'], sorts)))


def decode_node(value, sort, allow_entry=False):
    with _codec_reg(dict(ALL_REG if allow_entry else DATA_REG)):
        return _decode_node(value, sort, allow_entry, True)


def decode_document(value, sort, allow_entry=False):
    keys(value, ('format', 'version', 'term'))
    if value['format'] != 'KAST' or type(value['version']) is not int or value['version'] != 4:
        raise ValueError('KAST JSONv4 required')
    return decode_node(value['term'], sort, allow_entry=allow_entry)


# --------------------------------------------------------- printed parsing
def parse_printed_term(text, sort):
    with _codec_reg(dict(DATA_REG)):
        return base.codec.parse_printed(base.complete_term(text), sort)


def parse_printed_window(text):
    parsed = parse_printed_term(text, 'EGTraversalWindow')
    if parsed.sort != 'EGTraversalWindow' or parsed.name != 'egTraversalWindow':
        raise ValueError('complete EGTraversalWindow DATA root required')
    return parsed


# ------------------------------------------------------------------- query
def build_query_term(before):
    wire_query = ports.query(before)
    if (wire_query.get('constructor') != ENTRY_NAME
            or wire_query.get('sort') != 'EGTraversalWindow'
            or len(wire_query.get('arguments', [])) != 1):
        raise ValueError('complete egThreeEventTraversal source term required')
    internal_query = wire_to_internal(wire_query)
    term = from_tree(internal_query, allow_entry=True)
    wire_env = envelope(term, allow_entry=True)
    decoded = decode_document(wire_env, 'EGTraversalWindow', allow_entry=True)
    if not base.same(query_tree(term), query_tree(decoded)):
        raise ValueError('whole source query/KJSON typed inverse differs')
    if not same_whole(ports.to_snapshot(wire_query['arguments'][0]), before):
        raise ValueError('before snapshot field-port inverse differs')
    if ENTRY_NAME in base.codec.REG or len(base.codec.REG) != len(DATA_REG):
        raise ValueError('entry label lifetime not restored to DATA registry')
    return term, wire_env, wire_query


# ------------------------------------------------------------------ decode
def decode_window_term(parsed, before):
    internal = base.tree(parsed)
    wire = internal_to_wire(internal)
    if not same_whole(wire_to_internal(wire), internal):
        raise ValueError('AST wire translation not lossless')
    native = ports.to_window(wire)
    if set(native) != {'before', 'after_miss', 'after_hit', 'after_exit', 'events'}:
        raise ValueError('complete native traversal window keyset differs')
    for field in ('before', 'after_miss', 'after_hit', 'after_exit'):
        snap = native[field]
        if set(snap) != {'admission', 'image'} or len(snap['image']) != 23:
            raise ValueError('complete snapshot shape differs: ' + field)
    if (len(native['events']) != 3
            or any(list(e) != list(ports.EVENT_FIELDS) or len(e) != 28
                   for e in native['events'])):
        raise ValueError('complete 28-field event shape differs')
    before_equal = same_whole(wire['arguments'][0], ports.from_snapshot(before))
    return native, internal, wire, before_equal


# ---------------------------------------------------------------- commands
def render(before_path, query_path, report_path):
    bound = _require_bound_before()
    input_pin = base.pin(before_path)
    if not same_whole(input_pin, bound):
        raise ValueError('checked released actual before operand differs')
    before = base.loads(base.checked(input_pin))
    _term, wire_env, _wire_query = build_query_term(before)
    product = base.write(query_path, (json.dumps(
        wire_env, separators=(',', ':'), allow_nan=False) + '\n').encode())
    result = {'status': RENDER_PASS, 'input': input_pin, 'query': product,
              'before_roundtrip_equal': True,
              'expected_body_reads': 0, 'actual_after_input_reads': 0}
    report_pin = base.write(report_path, (json.dumps(
        result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': result['status'], 'query': product,
                      'report': report_pin}))
    return result


def decode(before_path, printed_path, native_path, internal_path,
           wire_path, report_path):
    bound = _require_bound_before()
    input_pin = base.pin(before_path)
    if not same_whole(input_pin, bound):
        raise ValueError('checked released actual before operand differs')
    before = base.loads(base.checked(input_pin))
    printed_pin = base.pin(printed_path)
    raw = base.checked(printed_pin).decode('utf-8')
    parsed = parse_printed_window(raw)
    native, internal, wire, before_equal = decode_window_term(parsed, before)
    native_pin = base.write(native_path, (json.dumps(
        native, indent=2, allow_nan=False) + '\n').encode())
    internal_pin = base.write(internal_path, (json.dumps(
        internal, indent=2, allow_nan=False) + '\n').encode())
    wire_pin = base.write(wire_path, (json.dumps(
        wire, indent=2, allow_nan=False) + '\n').encode())
    status = CAPTURE_PASS if before_equal else CAPTURE_FAIL
    result = {'status': status, 'input': input_pin, 'printed': printed_pin,
              'native_window': native_pin, 'internal_ast': internal_pin,
              'wire_ast': wire_pin, 'before_whole_typed_equal': before_equal,
              'complete_snapshot_count': 4, 'event_count': 3,
              'event_field_count': 28,
              'expected_body_reads': 0, 'actual_after_input_reads': 0}
    report_pin = base.write(report_path, (json.dumps(
        result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': status, 'native_window': native_pin,
                      'internal_ast': internal_pin, 'wire_ast': wire_pin,
                      'report': report_pin}))
    return result


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest='action', required=True)
    x = sub.add_parser('render')
    x.add_argument('before')
    x.add_argument('query')
    x.add_argument('report')
    x = sub.add_parser('decode')
    x.add_argument('before')
    x.add_argument('printed')
    x.add_argument('native_window')
    x.add_argument('internal_ast')
    x.add_argument('wire_ast')
    x.add_argument('report')
    a = cli.parse_args()
    if a.action == 'render':
        render(a.before, a.query, a.report)
    else:
        result = decode(a.before, a.printed, a.native_window,
                        a.internal_ast, a.wire_ast, a.report)
        raise SystemExit(0 if result['status'] == CAPTURE_PASS else 1)


if __name__ == '__main__':
    main()
