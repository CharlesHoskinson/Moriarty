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
SUPPORTED_K_CONSTRUCTORS = frozenset(CORE4_CONSTRUCTORS)
EXPRESSION_CODES = ex.ERROR_CODES | frozenset('TYPE_ACTION_REQUIRED FINANCIAL_CONTEXT_REQUIRED MISSING_OBLIGATION MISSING_BALANCE MISSING_ALLOWANCE INVALID_IDENTIFIER NOMINAL_UNIT WORK_MISMATCH EMPTY_BATCH OPTION_NONE VARIANT_CASE TYPE_SCALE_DIVISOR'.split())
KERNEL_CODES = frozenset(
    'DUPLICATE MISSING_OBLIGATION NOT_OUTSTANDING TRANSFER_NOT_IN_STEP TRANSFER_MISMATCH ZERO_AMOUNT OVERFLOW DUST '
    'INEXACT_CONVERSION CAPACITY INSUFFICIENT_WORK NOMINAL_UNIT INVALID_AMOUNT INVARIANT SELF_TRANSFER MISSING_BALANCE '
    'INSUFFICIENT_BALANCE MISSING_ALLOWANCE INSUFFICIENT_ALLOWANCE EXCEEDS_OUTSTANDING INSUFFICIENT_UNALLOCATED '
    'ALLOCATION_COMPONENT TRANSFER_AMOUNT_MISMATCH TRANSFER_ALREADY_ALLOCATED PERIOD_SEQUENCE PERIOD_NOT_ELIGIBLE '
    'LIABILITY_CAP_EXCEEDED NOMINAL_RANGE RESULT_BOUND SCHEMA UNKNOWN_FIELD UNKNOWN_ACTION INPUT_JSON INPUT_COMPACT '
    'INPUT_UTF8_LENGTH INPUT_UTF16_LENGTH INPUT_ENCODING INPUT_NOT_STRING INVALID_IDENTIFIER'.split()
)
ADAPTER_CODES = frozenset('WORK_MISMATCH FINANCIAL_CONTEXT_REQUIRED INPUT_SCHEMA INPUT_BOUND EMPTY_BATCH TYPE_FINANCIAL_WRITE OPERATION_BINDING NOMINAL_RANGE NOMINAL_UNIT SETTLEMENT_UNIT UNSUPPORTED_OPERATION INSUFFICIENT_WORK OVERFLOW'.split())


def _fail(code='LIFECYCLE_TRANSPORT'):
    raise CodecError(code)


def _utf8_len(text):
    if type(text) is not str:
        _fail()
    try:
        return len(text.encode('utf-8'))
    except UnicodeError:
        _fail()


def financial_parse(text):
    """Iterative JSON.parse/JSON.stringify lexical clone, including last-key wins.

    Numeric financial fields remain JSON numbers, never decimal-string amounts.
    No financial schema or state predicate is evaluated here.
    """
    if type(text) is not str:
        _fail()
    if _utf8_len(text) > REQUEST_LIMIT:
        _fail('LIFECYCLE_TRANSPORT')
    decoder = json.JSONDecoder(parse_int=float, parse_float=float,
                               parse_constant=lambda _: (_fail()))
    stack, root = [], []
    def attach(value):
        if not stack:
            if root:
                raise ValueError('trailing value')
            root.append(value)
        elif stack[-1][0] == 'array' and stack[-1][2] in ('first', 'value'):
            stack[-1][1].append(value)
            stack[-1][2] = 'comma'
        elif stack[-1][0] == 'object' and stack[-1][2] == 'value':
            stack[-1][1][stack[-1][3]] = value
            stack[-1][2] = 'comma'
        else:
            raise ValueError('value position')
    try:
        i = 0
        while i < len(text):
            c = text[i]
            if c in ' \t\r\n':
                i += 1
                continue
            if stack:
                f = stack[-1]
                if c in '}]':
                    if (c == '}' and f[0] != 'object') or (c == ']' and f[0] != 'array') or f[2] not in ('first', 'comma'):
                        raise ValueError('closing delimiter')
                    stack.pop()
                    i += 1
                    continue
                if f[2] == 'comma':
                    if c != ',':
                        raise ValueError('comma')
                    f[2] = 'key' if f[0] == 'object' else 'value'
                    i += 1
                    continue
                if f[2] == 'colon':
                    if c != ':':
                        raise ValueError('colon')
                    f[2] = 'value'
                    i += 1
                    continue
                if f[0] == 'object' and f[2] in ('first', 'key'):
                    if c != '"':
                        raise ValueError('key')
                    f[3], i = decoder.raw_decode(text, i)
                    f[2] = 'colon'
                    continue
            if c in '{[':
                value = {} if c == '{' else []
                attach(value)
                stack.append(['object' if c == '{' else 'array', value, 'first', None])
                i += 1
            else:
                value, i = decoder.raw_decode(text, i)
                if type(value) is float:
                    value = None if not math.isfinite(value) else int(value) if value.is_integer() else value
                attach(value)
        if stack or len(root) != 1:
            raise ValueError('incomplete JSON')
        return root[0]
    except (ValueError, TypeError, OverflowError, UnicodeError, RecursionError):
        return PARSE_FAILURE


def encode_json_value(value):
    try:
        return _encode_jsonish(value)
    except (ex.CodecError, TypeError, ValueError, UnicodeError):
        _fail('K_OUTPUT')


def encode_json_list(values):
    if type(values) is not list:
        _fail()
    return encode_json_value(values)


def encode_text(text, canonical=True):
    if canonical:
        if _utf8_len(text) > REQUEST_LIMIT:
            _fail('INPUT_BOUND')
        try:
            return ex.encode_json(text)
        except ex.CodecError:
            _fail()
    return encode_json_value(text)


def encode_financial(text):
    parsed = financial_parse(text)
    original = ex._string(text)
    if parsed is PARSE_FAILURE:
        return ex._apply('lcText', original, ex._apply('lcParseFailure'))
    return ex._apply('lcText', original, ex._apply('lcParsed', _encode_jsonish(parsed)))


def _has_surrogate(value):
    return any(0xD800 <= ord(c) <= 0xDFFF for c in value)


def _encode_jsonish(value):
    pending, results = [('visit', value)], []
    while pending:
        kind, v = pending.pop()
        if kind in ('array', 'object'):
            count = v if kind == 'array' else len(v)
            children = results[-count:] if count else []
            if count:
                del results[-count:]
            if kind == 'object':
                children = [ex._apply('lcJSONPair', ex._string(json.dumps(k, ensure_ascii=True)), item) if _has_surrogate(k) else ex._apply('ejPair', ex._string(k), item) for k, item in zip(v, children)]
            results.append(ex._apply('ejArray' if kind == 'array' else 'ejObject', _list_term(children)))
        elif type(v) is list:
            pending.append(('array', len(v)))
            pending.extend(('visit', item) for item in reversed(v))
        elif type(v) is dict:
            names = sorted(v)
            pending.append(('object', names))
            pending.extend(('visit', v[k]) for k in reversed(names))
        elif type(v) is str and _has_surrogate(v):
            results.append(ex._apply('lcJSONString', ex._string(json.dumps(v, ensure_ascii=True))))
        elif type(v) is float:
            results.append(ex._apply('lcJSONNumber', ex._string(json.dumps(v, allow_nan=False))))
        else:
            results.append(ex._encode(v))
    return results[0]


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
    if _utf8_len(schema) > SCHEMA_LIMIT or _utf8_len(request) > REQUEST_LIMIT or _utf8_len(financial) > REQUEST_LIMIT:
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
        _fail('K_OUTPUT')


def _decode_sequence(term):
    label, _ = _app(term)
    if label in ('ejNil', 'ejCons'):
        term = ex._apply('ejArray', term)
    value = _decode_jsonish(term)
    if type(value) is not list:
        _fail('K_OUTPUT')
    return value


def _normalized_string_tokens(term):
    """Normalize lexical String tokens only; preserve every other KAST field.

    LLVM prints UTF-8 byte escapes while the input parser accepts Unicode
    escapes. Strict token decoding makes those spellings comparable without
    ignoring any parsed financial input, token sort, label, or extra field.
    """
    root = [None]
    pending = [(root, 0, term)]
    while pending:
        parent, key, value = pending.pop()
        if type(value) is dict:
            if value.get('node') == 'KToken' and value.get('sort', {}).get('name') == 'String':
                decoded = _decode_jsonish(ex._apply('ejString', value))
                parent[key] = ex._string(decoded)
            else:
                cloned = {}
                parent[key] = cloned
                pending.extend((cloned, name, item) for name, item in value.items())
        elif type(value) is list:
            cloned = [None] * len(value)
            parent[key] = cloned
            pending.extend((cloned, index, item) for index, item in enumerate(value))
        else:
            parent[key] = value
    return ex.serialize_term(root[0])


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
    allowed = {
        '<k>', '<out>', '<sigma>', '<request>', '<pre>', '<args>', '<obs>',
        '<localTypes>', '<locals>', '<written>', '<writes>', '<descriptors>',
        '<sourceBytes>', '<initialWork>', '<work>', '<reducing>', '<generatedCounter>',
        '<financial>', '<financialPre>', '<financialPost>', '<stepTransfers>',
        '<effects>', '<actions>', '<suffix>', '<suffixIndex>', '<suffixWork>',
    }
    cells = {}
    for child in children:
        name, args = _app(child)
        if name not in allowed or name in cells or len(args) != 1:
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
        if _normalized_string_tokens(cells['<financial>']) != _normalized_string_tokens(packet['fields']['financialPreState']['term']):
            _fail('K_OUTPUT')
    elif '<financial>' in cells and cells['<financial>'] != ex._apply('lcMissingContext'):
        _fail('K_OUTPUT')
    out_label, out_args = _app(cells['<out>'])
    if out_label == 'lcPrepared' and len(out_args) == 4:
        remaining = _tok(out_args[3], 'Int')
        if not 0 <= remaining <= 65536:
            _fail('K_OUTPUT')
        post = _decode_jsonish(out_args[0])
        financial_post = _decode_jsonish(out_args[1])
        effects = _decode_sequence(out_args[2])
        if type(post) is not dict or type(financial_post) is not dict or type(effects) is not list:
            _fail('K_OUTPUT')
        return {
            'status': 'FundedExpressionPrepared',
            'post': _decode_jsonish(out_args[0]),
            'financialPost': _decode_jsonish(out_args[1]),
            'effects': _decode_sequence(out_args[2]),
            'workRemaining': str(remaining),
        }
    if out_label == 'lcExpressionRejected' and len(out_args) == 4:
        code = _tok(out_args[0], 'String')
        span = _decode_jsonish(out_args[1])
        path = _decode_sequence(out_args[2])
        work = _tok(out_args[3], 'Int')
        if code not in EXPRESSION_CODES or type(span) is not dict or set(span) != {'kind', 'start', 'end'} or span['kind'] not in ('source', 'synthetic') or type(path) is not list or not 0 <= work <= 65536:
            _fail('K_OUTPUT')
        for value in [span['start'], span['end'], *path]:
            if type(value) is not str or not value.isascii() or not value.isdecimal() or str(int(value)) != value:
                _fail('K_OUTPUT')
        if int(span['start']) > int(span['end']) or (span['kind'] == 'synthetic' and (span['start'] != '0' or span['end'] != '0')):
            _fail('K_OUTPUT')
        return {'status': 'Rejected', 'code': code, 'span': span, 'nodePath': path, 'workUsed': str(work)}
    if out_label == 'lcKernelRejected' and len(out_args) == 2:
        code = _tok(out_args[0], 'String')
        index_label, index_args = _app(out_args[1]) if type(out_args[1]) is dict and out_args[1].get('node') == 'KApply' else (None, None)
        if index_label == 'lcActionIndexNull' and not index_args:
            action_index = None
        else:
            action_index = _tok(out_args[1], 'Int')
        if action_index is not None and (type(action_index) is not int or action_index < 0):
            _fail('K_OUTPUT')
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
