"""EnvGet two-event window renderer / strict printed-window decoder.

Designated-owner bounded adaptation of the existing R2 c6_data /
nominal_trace_codec runtime shape (as in meta_runtime) to the EnvGet
EGWindow domain. Two direct-file commands only:

  render BEFORE_NATIVE_JSON QUERY_KAST_JSON INVARIANT_REPORT_JSON
  decode BEFORE_NATIVE_JSON KPRINT_TEXT NATIVE_WINDOW_JSON
         INTERNAL_AST_JSON WIRE_AST_JSON CAPTURE_REPORT_JSON

Render checks the released actual before (6362 bytes, sha256 b6faa318...),
builds the JSONv4 query egTwoEventWindow(actual EGSnapshot) with actual
raw production labels, and proves a whole typed inverse of that query.
Decode strictly parses one whole printed EGWindow of DATA only (all 27
functions, including egTwoEventWindow itself, are refused as final
DATA), emits the native 3-snapshot/2-event window plus internal and
wire typed ASTs, and reports whole typed equality of the decoded
before against the sole released input.

No expected operand and no Rust afterstate is read by either command:
both read counters are literal 0. Output is capture, not equality
evidence against any independent expectation. The original Rust whole
independent artifact result remains protected MISMATCH; nothing here
relabels or repairs it.
"""
import argparse
from contextlib import contextmanager
import importlib.util
import json
from pathlib import Path
import sys

COND_ROOT = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01')
BEFORE_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-EnvGetCell-hit-EnvGetExit-independent-oracle-20261020/SAFE/RELEASED-ACTUAL-ONLY-R5/ACTUAL-WINDOW-BEFORE.json', 'bytes': 6362, 'sha256': 'b6faa318017cc3205a663201d745424fe50397c591cf76b10d0a23f1f5b0c62d'}
REGISTRY_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01/PUBLIC-SOURCE-DECLARATION-REGISTRY.json', 'bytes': 104093, 'sha256': '435812d7d63662a82b6d915d204848dfa3e90d3b854174338c72c4027515f700'}
LABEL_MAP_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01/postcompile-label-qualification01/COMPILED-ALL306-KLABEL-SORT-ARITY-MAP.json', 'bytes': 586383, 'sha256': '841893b1b2ef9d02844291d38ddfe6f7a81bd8e896f9654a3433d2470afde120'}
PORTS_PATH = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01/actual-before-field-port-source01/designated-owner-existing-R2-window-runtime-source-request01/bounded-renderer-decoder-only-request02/literal-owner-runtime-candidate01/focused-owner-map-and-query-inverse-fix01/literal-corrected-runtime-candidate02/after-freeze-actual-only-window-diagnosis01/publicport-boundary-enum-source-correction-request01/literal-owner-boundary-correction-candidate03/source/envget_ports.py')
BASE_SOURCE_DIR = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019/source')

sys.path.insert(0, str(BASE_SOURCE_DIR))
import c6_data as base

_spec = importlib.util.spec_from_file_location('envget_ports', PORTS_PATH)
ports = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ports)

ENTRY_NAME = 'egTwoEventWindow'
ENTRY_DECL = ('EGWindow', ('EGSnapshot',))
PRIMITIVE_SORTS = ('Int', 'Bool', 'String')
RENDER_PASS = 'PASS_LOSSLESS_ENVGET_BEFORE_QUERY'
CAPTURE_PASS = 'PASS_COMPLETE_ENVGET_WINDOW_CAPTURE'
CAPTURE_FAIL = 'FAIL_DECODED_ENVGET_BEFORE_IDENTITY'


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


# ---------------------------------------------------------------- registry
PUBLIC = base.loads(base.checked(REGISTRY_PIN))
if type(PUBLIC) is not dict or PUBLIC.get('schema') != 'sherlock-envget-positive-source-signatures/1':
    raise ValueError('public source declaration registry schema differs')
_ROWS = PUBLIC.get('constructors_and_functions')
if type(_ROWS) is not dict or len(_ROWS) != 306:
    raise ValueError('complete 306 source declaration registry required')
REG_ALL = {}
for _name, _row in _ROWS.items():
    keys(_row, ('sort', 'argument_sorts', 'arity', 'source'))
    if (type(_name) is not str or type(_row['sort']) is not str
            or type(_row['argument_sorts']) is not list
            or _row['arity'] != len(_row['argument_sorts'])
            or type(_row['source']) is not str):
        raise ValueError('source declaration registry row differs')
    REG_ALL[_name] = (_row['sort'], tuple(_row['argument_sorts']))


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
    keys(data, ('status', 'definition', 'counts', 'new_EnvGet_counts',
                'declarations', 'source_signature_registry', 'mapping_method'))
    if not same_whole(data['source_signature_registry'], REGISTRY_PIN):
        raise ValueError('label map source registry pin differs')
    if data['counts'] != {'DATA': 279, 'function': 27}:
        raise ValueError('ACTUAL306 DATA/function counts differ')
    if (type(data['new_EnvGet_counts']) is not dict
            or data['new_EnvGet_counts'] != {'function': 27, 'DATA': 31}):
        raise ValueError('actual new_EnvGet_counts differ')
    if type(data['declarations']) is not list or len(data['declarations']) != 306:
        raise ValueError('complete ACTUAL306 declaration list required')
    lines = base.checked(data['definition']).decode('utf-8').splitlines()
    forward, reverse = {}, {}
    data_names, function_names = set(), set()
    for row in data['declarations']:
        keys(row, ('surface_name', 'sort', 'argument_sorts', 'arity', 'role',
                   'declaring_module', 'declaration_source', 'Source', 'Location',
                   'source_declaration_lines_exact', 'kore_symbol',
                   'compiled_klabel', 'explicit_klabel_attribute',
                   'definition_symbol_line', 'complete_actual_symbol_declaration'))
        name, raw = row['surface_name'], row['compiled_klabel']
        if type(name) is not str or name not in REG_ALL or name in forward:
            raise ValueError('unknown/duplicate compiled label declaration')
        if type(raw) is not str or raw in reverse:
            raise ValueError('duplicate compiled raw label')
        sort, sorts = REG_ALL[name]
        if (row['sort'] != sort or tuple(row['argument_sorts']) != sorts
                or row['arity'] != len(sorts)):
            raise ValueError('compiled label sort/arguments differ')
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
        line, body = row['definition_symbol_line'], row['complete_actual_symbol_declaration']
        if (type(line) is not int or not 1 <= line <= len(lines)
                or lines[line - 1] != body
                or not body.startswith('  symbol ' + row['kore_symbol'] + '{}(')):
            raise ValueError('actual compiled symbol declaration membership differs')
        signature = '{}' + '(' + ', '.join('Sort' + s + '{}' for s in sorts) + ') : Sort' + sort + '{} '
        if signature not in body:
            raise ValueError('actual compiled symbol signature differs')
        if row['Source'] != row['declaration_source']['path']:
            raise ValueError('declaration Source path differs')
        base.checked(row['declaration_source'])
        forward[name] = raw
        reverse[raw] = name
    if set(forward) != set(REG_ALL):
        raise ValueError('compiled label map missing registry entry')
    if len(data_names) != 279 or len(function_names) != 27:
        raise ValueError('ACTUAL306 DATA/function split differs')
    if REG_ALL[ENTRY_NAME] != ENTRY_DECL or ENTRY_NAME not in function_names:
        raise ValueError('egTwoEventWindow entry declaration differs')
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
    """Temporarily admit exactly egTwoEventWindow at a query root.

    Only ENTRY_NAME is added to DATA_REG for base.tree rendering.
    The other 26 function names remain absent, so base.tree still
    refuses any other residual function. The enclosing _codec_reg
    restores the prior DATA-only registry on success and on failure.
    """
    reg = dict(DATA_REG)
    reg[ENTRY_NAME] = ALL_REG[ENTRY_NAME]
    with _codec_reg(reg):
        yield


def query_tree(term):
    """Return base.tree(term) for one egTwoEventWindow-rooted query.

    The entry function is admitted only at the root. Every non-scalar
    descendant must already be DATA; a nested entry or any other
    function is refused before the temporary registry is widened.
    The temporary widening contains exactly DATA_REG plus ENTRY_NAME,
    never all 27 functions, and is restored by _codec_reg on every path.
    """
    if getattr(term, 'name', None) != ENTRY_NAME or getattr(term, 'sort', None) != 'EGWindow':
        raise ValueError('query tree egTwoEventWindow root required')
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
    """Wire {sort,literal} primitives -> internal {sort,value}. Key only."""
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
    """Internal {sort,value} primitives -> wire {sort,literal}. Key only."""
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
        # Strings are required in this domain (opaque immutable JSON);
        # there is no numeric META-only String refusal here.
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
    """Whole printed term under DATA only; framing per c6_data.complete_term.

    Bare text, or one exact <generatedTop>/<k> wrapper with the optional
    empty '~> .K' continuation, is accepted by the reused complete_term.
    No substring search, trailing token, or partial term is accepted.
    """
    with _codec_reg(dict(DATA_REG)):
        return base.codec.parse_printed(base.complete_term(text), sort)


def parse_printed_window(text):
    parsed = parse_printed_term(text, 'EGWindow')
    if parsed.sort != 'EGWindow' or parsed.name != 'egWindow':
        raise ValueError('complete EGWindow DATA root required')
    return parsed


# ------------------------------------------------------------------- query
def build_query_term(before):
    """egTwoEventWindow(actual before) with whole query inverse proof.

    The entry function is admitted only at this root and only under a
    temporary REG widening; base.codec.REG is restored on every path.
    """
    wire_query = ports.query(before)
    if (wire_query.get('constructor') != ENTRY_NAME
            or wire_query.get('sort') != 'EGWindow'
            or len(wire_query.get('arguments', [])) != 1):
        raise ValueError('complete egTwoEventWindow source term required')
    internal_query = wire_to_internal(wire_query)
    term = from_tree(internal_query, allow_entry=True)
    wire_env = envelope(term, allow_entry=True)
    decoded = decode_document(wire_env, 'EGWindow', allow_entry=True)
    if not base.same(query_tree(term), query_tree(decoded)):
        raise ValueError('whole source query/KJSON typed inverse differs')
    if not same_whole(ports.to_snapshot(wire_query['arguments'][0]), before):
        raise ValueError('before snapshot field-port inverse differs')
    if ENTRY_NAME in base.codec.REG or len(base.codec.REG) != len(DATA_REG):
        raise ValueError('entry label lifetime not restored to DATA registry')
    return term, wire_env, wire_query


# ------------------------------------------------------------------ decode
def decode_window_term(parsed, before):
    """Parsed EGWindow T -> (native, internal, wire, before_equal)."""
    internal = base.tree(parsed)
    wire = internal_to_wire(internal)
    if not same_whole(wire_to_internal(wire), internal):
        raise ValueError('AST wire translation not lossless')
    native = ports.to_window(wire)
    if set(native) != {'before', 'after_cell', 'after_exit', 'events'}:
        raise ValueError('complete native window keyset differs')
    for field in ('before', 'after_cell', 'after_exit'):
        snap = native[field]
        if set(snap) != {'admission', 'image'} or len(snap['image']) != 23:
            raise ValueError('complete snapshot shape differs: ' + field)
    if (len(native['events']) != 2
            or any(list(e) != list(ports.EVENT_FIELDS) or len(e) != 28
                   for e in native['events'])):
        raise ValueError('complete 28-field event shape differs')
    before_equal = same_whole(wire['arguments'][0], ports.from_snapshot(before))
    return native, internal, wire, before_equal


# ---------------------------------------------------------------- commands
def render(before_path, query_path, report_path):
    input_pin = base.pin(before_path)
    if not same_whole(input_pin, BEFORE_PIN):
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
    input_pin = base.pin(before_path)
    if not same_whole(input_pin, BEFORE_PIN):
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
              'complete_snapshot_count': 3, 'event_field_count': 28,
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
