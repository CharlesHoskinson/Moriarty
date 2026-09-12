"""Lifecycle K transport. Schema/request are canonical; financial uses JS parse/clone.

This module does not evaluate Core, invoke TypeScript, or copy host financial POST
into K input. Unknown constructors and stuck terms are execution boundaries.
"""
import json
import math
import expression_codec as ex

class CodecError(ValueError):
    pass

SCHEMA_LIMIT = 65536
REQUEST_LIMIT = 2_000_000
FINANCIAL_LIMIT = 65536
PARSE_FAILURE = {'tag': 'parseFailure'}
CORE4_CONSTRUCTORS = (
    'LitUInt', 'LitSInt', 'LitBool', 'LitText', 'LitAmount', 'LitQuantity', 'LitShares', 'LitRate', 'LitPrice',
    'ReadLocal', 'ReadPre', 'ReadArg', 'ReadObs',
    'ProjectField', 'AccessField', 'ProjectIndex', 'AccessIndex',
    'ConstructRecord', 'ConstructEnum', 'ConstructSome', 'ConstructNone', 'ConstructCollection',
    'ConstructShares', 'ConstructAmount', 'ConstructVariant', 'ProjectVariant', 'ProjectSome',
    'ConvertUInt', 'ScalarValue', 'Select',
    'Add', 'Sub', 'Mul', 'FloorDiv', 'CeilDiv', 'Eq', 'Lt', 'Lte', 'Gt', 'Gte', 'Not', 'And', 'Or',
    'Require', 'Let', 'NextWrite', 'Ensure', 'Emit',
    'ReadOutstanding', 'ReadPrincipal', 'ReadAccrued', 'ReadBalance', 'ReadAllowanceRemaining', 'ReadAllowanceSpent',
    'ReadPostOutstanding', 'ReadPostPrincipal', 'ReadPostAccrued', 'ReadPostBalance',
    'ReadPostAllowanceRemaining', 'ReadPostAllowanceSpent',
)
SUPPORTED_K_CONSTRUCTORS = frozenset(CORE4_CONSTRUCTORS) - frozenset((
    'ConstructShares', 'ConstructVariant', 'ProjectVariant', 'ProjectSome', 'ConvertUInt', 'Select',
))
EXPRESSION_CODES = ex.ERROR_CODES | frozenset('TYPE_ACTION_REQUIRED FINANCIAL_CONTEXT_REQUIRED MISSING_OBLIGATION MISSING_BALANCE MISSING_ALLOWANCE INVALID_IDENTIFIER NOMINAL_UNIT WORK_MISMATCH EMPTY_BATCH'.split())
KERNEL_CODES = frozenset(
    'DUPLICATE MISSING_OBLIGATION NOT_OUTSTANDING TRANSFER_NOT_IN_STEP TRANSFER_MISMATCH ZERO_AMOUNT OVERFLOW DUST '
    'INEXACT_CONVERSION CAPACITY INSUFFICIENT_WORK NOMINAL_UNIT INVALID_AMOUNT INVARIANT SELF_TRANSFER MISSING_BALANCE '
    'INSUFFICIENT_BALANCE MISSING_ALLOWANCE INSUFFICIENT_ALLOWANCE EXCEEDS_OUTSTANDING INSUFFICIENT_UNALLOCATED '
    'ALLOCATION_COMPONENT TRANSFER_AMOUNT_MISMATCH TRANSFER_ALREADY_ALLOCATED PERIOD_SEQUENCE PERIOD_NOT_ELIGIBLE '
    'LIABILITY_CAP_EXCEEDED NOMINAL_RANGE RESULT_BOUND SCHEMA UNKNOWN_FIELD UNKNOWN_ACTION INPUT_JSON INPUT_COMPACT '
    'INPUT_UTF8_LENGTH INPUT_UTF16_LENGTH INPUT_ENCODING INPUT_NOT_STRING INVALID_IDENTIFIER'.split()
)
ADAPTER_CODES = frozenset('WORK_MISMATCH FINANCIAL_CONTEXT_REQUIRED INPUT_SCHEMA INPUT_BOUND'.split())


def _fail(code='LIFECYCLE_TRANSPORT'):
    raise CodecError(code)


def _utf8_len(text):
    if type(text) is not str:
        _fail()
    try:
        return len(text.encode('utf-8'))
    except UnicodeError:
        _fail()


def _js_sanitize(value):
    if type(value) is float and not math.isfinite(value):
        return None
    if type(value) is list:
        return [_js_sanitize(item) for item in value]
    if type(value) is dict:
        return {key: _js_sanitize(item) for key, item in value.items()}
    return value


def financial_parse(text):
    if type(text) is not str:
        _fail()
    n = _utf8_len(text)
    if n > FINANCIAL_LIMIT or len(text) > FINANCIAL_LIMIT:
        _fail('INPUT_BOUND')
    try:
        parsed = json.loads(text)
        cloned = json.loads(json.dumps(_js_sanitize(parsed), allow_nan=False, ensure_ascii=False))
    except (ValueError, TypeError, OverflowError, UnicodeError):
        return PARSE_FAILURE
    return cloned


def encode_json_value(value):
    try:
        return ex._encode(value)
    except (ex.CodecError, TypeError, ValueError, UnicodeError):
        if type(value) is float:
            token = json.dumps(value, allow_nan=False)
            return ex._apply('ejNumber', ex._token('Float', token))
        _fail('K_OUTPUT')


def encode_json_list(values):
    if type(values) is not list:
        _fail()
    return encode_json_value(values)


def encode_text(text, canonical=True):
    if canonical:
        if _utf8_len(text) > SCHEMA_LIMIT and text != text:
            _fail('INPUT_BOUND')
        try:
            return ex.encode_json(text)
        except ex.CodecError:
            _fail()
    return encode_json_value(text)


def encode_financial(text):
    parsed = financial_parse(text)
    original = ex._string(text)
    if parsed == PARSE_FAILURE:
        return ex._apply('lcText', original, ex._apply('lcParseFailure'))
    return ex._apply('lcText', original, ex._apply('lcParsed', encode_json_value(parsed) if not _has_float(parsed) else _encode_jsonish(parsed)))


def _has_float(value):
    if type(value) is float:
        return True
    if type(value) is list:
        return any(_has_float(item) for item in value)
    if type(value) is dict:
        return any(_has_float(item) for item in value.values())
    return False


def _encode_jsonish(value):
    if value is None:
        return ex._apply('ejNull')
    if type(value) is bool:
        return ex._apply('ejBool', ex._token('Bool', 'true' if value else 'false'))
    if type(value) is int:
        return ex._apply('ejNumber', ex._token('Int', str(value)))
    if type(value) is float:
        return ex._apply('ejNumber', ex._token('Float', json.dumps(value, allow_nan=False)))
    if type(value) is str:
        return ex._apply('ejString', ex._string(value))
    if type(value) is list:
        return ex._apply('ejArray', _list_term([_encode_jsonish(item) for item in value]))
    if type(value) is dict:
        pairs = [ex._apply('ejPair', ex._string(key), _encode_jsonish(item)) for key, item in value.items()]
        return ex._apply('ejObject', _list_term(pairs))
    _fail()


def _list_term(values):
    result = ex._apply('ejNil')
    for value in reversed(values):
        result = ex._apply('ejCons', value, result)
    return result


def admit_packet(packet):
    if type(packet) is not dict:
        _fail()
    keys = set(packet)
    if keys == {'schema', 'request'}:
        schema, request = packet['schema'], packet['request']
        if type(schema) is not str or type(request) is not str:
            _fail()
        if _utf8_len(schema) > SCHEMA_LIMIT or _utf8_len(request) > REQUEST_LIMIT:
            _fail('INPUT_BOUND')
        try:
            schema_term = ex.encode_json(schema)
            request_term = ex.encode_json(request) if _utf8_len(request) <= REQUEST_LIMIT else (_fail('INPUT_BOUND'))
        except ex.CodecError:
            _fail()
        if _utf8_len(request) > REQUEST_LIMIT:
            _fail('INPUT_BOUND')
        return {
            'constructor': 'lifecycleMissingContext',
            'fields': {'schema': {'original': schema, 'term': schema_term}, 'request': {'original': request, 'term': request_term}},
        }
    if keys != {'schema', 'request', 'financialPreState'}:
        _fail()
    schema, request, financial = packet['schema'], packet['request'], packet['financialPreState']
    if financial is None or type(schema) is not str or type(request) is not str or type(financial) is not str:
        _fail()
    if _utf8_len(schema) > SCHEMA_LIMIT or _utf8_len(request) > REQUEST_LIMIT or _utf8_len(financial) > FINANCIAL_LIMIT:
        _fail('INPUT_BOUND')
    try:
        schema_term = ex.encode_json(schema)
        request_term = ex.encode_json(request)
    except ex.CodecError:
        _fail()
    financial_term = encode_financial(financial)
    parsed = financial_parse(financial)
    return {
        'constructor': 'lifecycleRequest',
        'fields': {
            'schema': {'original': schema, 'term': schema_term},
            'request': {'original': request, 'term': request_term},
            'financialPreState': {'original': financial, 'parsed': parsed, 'term': financial_term},
        },
    }


def encode_packet(packet):
    fields = packet['fields']
    if packet['constructor'] == 'lifecycleMissingContext':
        return ex._apply('lifecycleMissingContext', fields['schema']['term'], fields['request']['term'])
    if packet['constructor'] == 'lifecycleRequest':
        return ex._apply('lifecycleRequest', fields['schema']['term'], fields['request']['term'], fields['financialPreState']['term'])
    _fail()


def _app(term):
    if type(term) is not dict or set(term) != {'node', 'label', 'args', 'arity'} or term['node'] != 'KApply' or type(term['args']) is not list or type(term['arity']) is not int or term['arity'] != len(term['args']):
        _fail('K_OUTPUT')
    label = term['label']
    if type(label) is not dict or set(label) != {'node', 'name', 'params'} or label['node'] != 'KLabel' or label['params'] != [] or type(label['name']) is not str:
        _fail('K_OUTPUT')
    return label['name'], term['args']


def _tok(term, sort):
    return ex.decode_term(ex._apply({'Int': 'ejNumber', 'String': 'ejString', 'Bool': 'ejBool'}[sort], term))


def _decode_jsonish(term):
    try:
        return ex.decode_term(term)
    except ex.CodecError:
        label, args = _app(term)
        if label == 'ejNumber' and len(args) == 1:
            token = args[0]
            if type(token) is dict and token.get('sort', {}).get('name') == 'Float':
                return json.loads(token['token'])
        _fail('K_OUTPUT')


def decode_result(raw, packet):
    if type(raw) is not str or len(raw.encode('utf-8')) > 64 * 1024 * 1024:
        _fail('K_OUTPUT')
    try:
        data = ex._parse(raw, allow_space=True)
    except (ex.CodecError, ValueError, UnicodeError):
        _fail('K_OUTPUT')
    if type(data) is not dict or set(data) != {'format', 'version', 'term'} or data['format'] != 'KAST' or type(data['version']) is not int or data['version'] != 4:
        _fail('K_OUTPUT')
    label, children = _app(data['term'])
    if label != '<generatedTop>':
        _fail('K_OUTPUT')
    cells = {}
    for child in children:
        name, args = _app(child)
        if name in cells or len(args) != 1:
            _fail('K_OUTPUT')
        cells[name] = args[0]
    if not {'<k>', '<out>', '<sigma>', '<request>'} <= set(cells):
        _fail('K_OUTPUT')
    if cells['<k>'] != {'node': 'KSequence', 'arity': 0, 'items': []}:
        _fail('K_OUTPUT')
    schema_text = packet['fields']['schema']['original']
    request_text = packet['fields']['request']['original']
    if ex._canonical(ex.decode_term(cells['<sigma>'])) != schema_text:
        _fail('K_OUTPUT')
    if ex._canonical(ex.decode_term(cells['<request>'])) != request_text:
        _fail('K_OUTPUT')
    if packet['constructor'] == 'lifecycleRequest':
        if '<financial>' not in cells:
            _fail('K_OUTPUT')
        fin_label, fin_args = _app(cells['<financial>'])
        if fin_label != 'lcText' or len(fin_args) != 2:
            _fail('K_OUTPUT')
        if _tok(fin_args[0], 'String') != packet['fields']['financialPreState']['original']:
            _fail('K_OUTPUT')
    out_label, out_args = _app(cells['<out>'])
    if out_label == 'lcPrepared' and len(out_args) == 4:
        remaining = _tok(out_args[3], 'Int')
        if remaining < 0:
            _fail('K_OUTPUT')
        return {
            'status': 'FundedExpressionPrepared',
            'post': _decode_jsonish(out_args[0]),
            'financialPost': _decode_jsonish(out_args[1]),
            'effects': _decode_jsonish(out_args[2]) if out_args[2].get('label', {}).get('name') != 'ejNil' else ex.decode_term(ex._apply('ejArray', out_args[2])),
            'workRemaining': str(remaining),
        }
    if out_label == 'lcExpressionRejected' and len(out_args) == 4:
        code = _tok(out_args[0], 'String')
        span = _decode_jsonish(out_args[1])
        path = _decode_jsonish(out_args[2]) if out_args[2].get('label', {}).get('name') != 'ejNil' else ex.decode_term(ex._apply('ejArray', out_args[2]))
        work = _tok(out_args[3], 'Int')
        if code not in EXPRESSION_CODES or type(span) is not dict or type(path) is not list or work < 0:
            _fail('K_OUTPUT')
        return {'status': 'Rejected', 'code': code, 'span': span, 'nodePath': path, 'workUsed': str(work)}
    if out_label == 'lcKernelRejected' and len(out_args) == 2:
        code = _tok(out_args[0], 'String')
        index_label, index_args = _app(out_args[1]) if type(out_args[1]) is dict and out_args[1].get('node') == 'KApply' else (None, None)
        if index_label == 'lcActionIndexNull' and not index_args:
            action_index = None
        else:
            action_index = _tok(out_args[1], 'Int')
        if code not in KERNEL_CODES:
            _fail('K_OUTPUT')
        return {'status': 'Rejected', 'code': code, 'actionIndex': action_index}
    if out_label == 'lcAdapterRejected' and len(out_args) == 1:
        code = _tok(out_args[0], 'String')
        if code not in ADAPTER_CODES:
            _fail('K_OUTPUT')
        return {'status': 'Rejected', 'code': code}
    _fail('K_OUTPUT')


def constructors_in(value, found=None):
    if found is None:
        found = []
    if type(value) is dict:
        if type(value.get('constructor')) is str:
            found.append(value['constructor'])
        for item in value.values():
            constructors_in(item, found)
    elif type(value) is list:
        for item in value:
            constructors_in(item, found)
    return found


def unsupported_constructors(core):
    return [name for name in constructors_in(core) if name in CORE4_CONSTRUCTORS and name not in SUPPORTED_K_CONSTRUCTORS]
