"""Project one actual-format kore-exec/kprint N01 capture into the frozen post-startup observation.

No execution, source-rule selection, oracle import, Kore decoder or evaluator is
present. Formats admitted here were observed once, in attempt-engine-N01-01 (one
frozen N01 fuel-1 capture, K 7.1.337 installed backends); any other layout, entry,
cell or sequence stays a gap or raises ValueError.

Logs are original kore-exec stderr bytes, hashed and retained (base64) before
strict ASCII decoding; every line must be claimed by an admitted entry. A rewrite
step must read DebugAppliedRewriteRules (one span), WarnDepthLimitExceeded naming
the receipt's declared depth (warnings pass the default Warning log level), and a
final InfoExecDepth 1. A terminal replay must read the explicit "No rules were
applied." variant and InfoExecDepth 0. A span is identified only by full equality
with one entry of a supplied compiled source-attribute inventory.

Configurations are kprint stdout text, bounded in size and nesting before the
shared decoders see them. Only a <k> item printed "X ~> .K" (kseq(X, dotk())) is
reduced to X; originals are retained. kprint printed the nine declared main.k
cells; <generatedCounter>, present in raw Kore, is not projected. The pre-startup
state exists only as the logged startup On-configuration Kore: retained, never typed.

Each step carries a kore-exec receipt (stderr log, empty stdout, output Kore) and a
kprint receipt (stdout configuration, empty stderr). Their consistency, the path
chain, and byte equality of each logged On-configuration term with the input Kore
are checked; nothing is authenticated. Comparison readiness concerns this supplied
post-startup data only; reliance stays blocked pending fresh audit and owner
confirmation.
"""
import base64
import hashlib
import importlib.util
from pathlib import Path
import re


def _load(name, alias):
    spec = importlib.util.spec_from_file_location(alias, Path(__file__).with_name(name))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_OBSERVATIONS = _load('k_observations.py', 'sherlock_k_observations')
_TERMS = _load('k_terms.py', 'sherlock_k_terms')

FORMAT = 'sherlock-k-standard-log-capture/2'
BUNDLE_FORMAT = 'sherlock-k-normal-observations/1'
PRODUCER = 'sherlock-k-reference'
LOG_FORMAT = 'installed-kore-standard-log/2'
EMPTY_SHA256 = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
MAX_LOG_BYTES = 4 * 1024 * 1024
MAX_LOG_LINES = 65536
MAX_KORE_BYTES = 1024 * 1024
MAX_CONFIGURATION_BYTES = 8192
MAX_NESTING = 32
MAX_TERM_BYTES = 4096
MAX_CAPTURES = 64
F03_MAX_CAPTURES = 95
MAX_ARGV = 64
MAX_ARG_CHARS = 4096
MAX_RULE_SPANS = 4096
ENGINE_RECEIPT_KEYS = frozenset({'argv', 'exit_status', 'stdout_sha256', 'stderr_sha256',
                                 'output_sha256'})
PRINT_RECEIPT_KEYS = frozenset({'argv', 'exit_status', 'stdout_sha256', 'stderr_sha256'})
ADAPTER_SOURCES = {name: hashlib.sha256(Path(__file__).with_name(name).read_bytes()).hexdigest()
                   for name in (Path(__file__).name, 'k_observations.py', 'k_terms.py')}
_ADMITTED = {'DebugAppliedRewriteRules': 'Debug', 'InfoExecDepth': 'Info',
             'WarnDepthLimitExceeded': 'Warning'}
_REWRITE_LAYOUT = ('DebugAppliedRewriteRules', 'WarnDepthLimitExceeded', 'InfoExecDepth')
_NO_RULE_LAYOUT = ('DebugAppliedRewriteRules', 'InfoExecDepth')
_HEADER = re.compile(r'kore-exec: (Debug|Info|Warning|Error|Critical) '
                     r'\(([A-Za-z][A-Za-z0-9]{0,63})\):')
_DEPTH = re.compile(r'    execution complete: exec depth: (0|[1-9][0-9]{0,17})')
_WARN = re.compile(r'    The depth limit (0|[1-9][0-9]{0,17}) was exceeded\.')
_DECIMAL = re.compile(r'0|[1-9][0-9]{0,17}')
_SPAN = re.compile(r'([1-9][0-9]{0,8}):([1-9][0-9]{0,8})-([1-9][0-9]{0,8}):([1-9][0-9]{0,8})')
_TEXT = re.compile(r'[\x20-\x7e\n]*')
_SOURCE = re.compile(r'/[\x21-\x7e]{1,4095}')
_DIGEST = re.compile(r'[0-9a-f]{64}')
_TAG = re.compile(r'<([A-Za-z][A-Za-z0-9]*)>')
_APPLIED = '    The rules at following locations were applied:'
_NO_RULES = '    No rules were applied.'
_ON = '    On configuration:'
_LOGGED_OPEN = ('      \\and{SortGeneratedTopCell{}}(', '          /* term: */')
_LOGGED_CLOSE = ('      \\and{SortGeneratedTopCell{}}(', '          /* predicate: */',
                 '          \\top{SortGeneratedTopCell{}}(),', '          /* substitution: */',
                 '          \\top{SortGeneratedTopCell{}}()', '      ))')
_COUNTER = b"Lbl'-LT-'generatedCounter'-GT-'"
_ENGINE_VALUED = frozenset({'--module', '--pattern', '--output', '--depth', '--smt',
                            '--log-format', '--log-entries'})
_ENGINE_SWITCHES = frozenset({'--disable-log-timestamps'})
_ENGINE_REQUIRED = {'--log-format': 'standard', '--disable-log-timestamps': True,
                    '--log-entries': 'DebugAppliedRewriteRules,InfoExecDepth'}
_STEP_KEYS = frozenset({'configuration', 'log', 'state_kore'})
_SPAN_KEYS = ('start_line', 'start_column', 'end_line', 'end_column')


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _raw(data):
    return dict(base64=base64.b64encode(data).decode('ascii'), sha256=_sha(data), size=len(data))


def _evidence():
    return dict(
        receipts='supplied kore-exec/kprint receipt consistency, hash links and path chain '
                 'checked; not authenticated',
        producer_authenticity='not-established',
        actual_format='observed for one frozen N01 fuel-1 capture (attempt-engine-N01-01) only; '
                      'other layouts, cells and sequences are gaps',
        log_layout='rewrite: DebugAppliedRewriteRules(one span), WarnDepthLimitExceeded(declared '
                   'depth), InfoExecDepth(1); terminal: DebugAppliedRewriteRules(No rules were '
                   'applied.), InfoExecDepth(0)',
        pretty_normalization='only one <k> item with .K unit tail is reduced to the item; '
                             'raw_configurations/decoded_configurations hold the reduced text, '
                             'original_configurations and raw_captures the kprint text',
        projection_boundary='nine declared main.k cells as printed by kprint; <generatedCounter> '
                            'in raw Kore and logged bodies is retained raw, not projected',
        startup='startup rewrite observed separately; pre-startup state retained only as '
                'logged Kore; no typed projection or comparison use',
        rule_identity='observed full span equal to one supplied compiled inventory entry; '
                      'start line projected; rewrite axioms not distinguished from equations',
        adapter_sources=dict(ADAPTER_SOURCES),
        adapter_source_binding='hashed at import; not bound to loaded code',
        reliance='blocked pending fresh actual source/result audit and owner confirmation')


def _source(source_path):
    if type(source_path) is not str or _SOURCE.fullmatch(source_path) is None:
        raise ValueError('exact observed absolute printable-ASCII source path required')


def _bytes(value, label):
    if type(value) is not bytes:
        raise ValueError('complete original ' + label + ' bytes required')


def _nesting(text, label):
    depth = 0
    for char in text:
        if char == '(':
            depth += 1
            if depth > MAX_NESTING:
                raise ValueError(label + ' exceeds the fixed N01 nesting bound')
        elif char == ')':
            depth -= 1
            if depth < 0:
                raise ValueError(label + ' has unbalanced parentheses')
    if depth:
        raise ValueError(label + ' has unbalanced parentheses')


def _term_text(text, label):
    """Bound separately retained term text before the shared term parser sees it."""
    if type(text) is not str or len(text) > MAX_TERM_BYTES or _TEXT.fullmatch(text) is None:
        raise ValueError(label + ' must be bounded printable-ASCII term text')
    _nesting(text, label)


def _configuration(text):
    """Bound and admit one kprint configuration; return (decoder text, unit-tail note)."""
    if type(text) is not str:
        raise ValueError('complete pretty-printed configuration text required')
    if len(text) > MAX_CONFIGURATION_BYTES:
        raise ValueError('configuration exceeds the fixed N01 size bound')
    if _TEXT.fullmatch(text) is None:
        raise ValueError('configuration is not printable-ASCII kprint text')
    _nesting(text, 'configuration')
    for tag in _TAG.finditer(text):
        if tag[1] != 'generatedTop' and tag[1] not in _OBSERVATIONS.CELLS:
            raise ValueError('actual-format gap: cell <' + tag[1] + '> is not admitted; '
                             'kprint printed only the nine declared cells')
    if '~>' not in text:
        return text, None
    start, end = text.find('<k>'), text.find('</k>')
    single = text.count('<k>') == 1 and text.count('</k>') == 1 and start < end
    content = text[start + 3:end] if single else ''
    head, separator, tail = content.partition(' ~> ')
    if (text.count('~>') != 1 or not separator or not tail.startswith('.K') or tail[2:].strip()
            or not head.strip() or head.strip() == '.K' or '\n' in head.strip()):
        raise ValueError('actual-format gap: K sequence other than one <k> item with .K unit tail')
    return (text[:start + 3] + head + tail[2:] + text[end:],
            dict(rule='one <k> item X printed "X ~> .K" (kseq(X, dotk())) reduced to X',
                 original_k=content, decoded_k=head + tail[2:]))


def _decode(text, entry_k):
    decoded_text, note = _configuration(text)
    decoded = _OBSERVATIONS.decode_configuration(decoded_text, entry_k=entry_k)
    decoded.update(original_configuration=text, original_sha256=_sha(text.encode('ascii')),
                   unit_tail=note)
    return decoded


def _receipt(value, keys, label):
    if value is None:
        return None
    if type(value) is not dict or set(value) != keys:
        raise ValueError(label + ' receipt requires exactly ' + ', '.join(sorted(keys)))
    argv = value['argv']
    if (type(argv) is not list or not argv or len(argv) > MAX_ARGV
            or any(type(arg) is not str or not arg or len(arg) > MAX_ARG_CHARS or '\x00' in arg
                   for arg in argv)):
        raise ValueError(label + ' receipt argv must be a bounded nonempty list of nonempty strings')
    if type(value['exit_status']) is not int:
        raise ValueError(label + ' receipt exit_status must be an int')
    for key in sorted(keys - {'argv', 'exit_status'}):
        if type(value[key]) is not str or _DIGEST.fullmatch(value[key]) is None:
            raise ValueError(label + ' receipt ' + key + ' must be lowercase hex SHA-256')
    return {key: list(value[key]) if key == 'argv' else value[key] for key in sorted(keys)}


def _engine_options(argv):
    """Strictly tokenize admitted kore-exec options; None if any token is unadmitted."""
    if len(argv) < 2 or argv[1].startswith('-'):
        return None
    options, index = {}, 2
    while index < len(argv):
        flag = argv[index]
        if flag in options:
            return None
        if flag in _ENGINE_SWITCHES:
            options[flag], index = True, index + 1
        elif (flag in _ENGINE_VALUED and index + 1 < len(argv)
              and not argv[index + 1].startswith('-')):
            options[flag], index = argv[index + 1], index + 2
        else:
            return None
    return options


def _declared_depth(engine):
    options = _engine_options(engine['argv']) if engine is not None else None
    value = options.get('--depth') if options else None
    return int(value) if value is not None and _DECIMAL.fullmatch(value) else None


def _engine_gaps(receipt, *, raw_log, state_kore):
    """Check one supplied kore-exec receipt for consistency only; this does not authenticate it."""
    if receipt is None:
        return ['engine-receipt-not-supplied']
    gaps = []
    if receipt['exit_status'] != 0:
        gaps.append('engine-receipt-exit-status-nonzero')
    if receipt['stderr_sha256'] != _sha(raw_log):
        gaps.append('engine-receipt-stderr-log-sha256-mismatch')
    if receipt['stdout_sha256'] != EMPTY_SHA256:
        gaps.append('engine-receipt-stdout-not-empty')
    if receipt['output_sha256'] != _sha(state_kore):
        gaps.append('engine-receipt-output-kore-sha256-mismatch')
    options = _engine_options(receipt['argv'])
    if options is None:
        gaps.append('engine-receipt-argv-not-admitted-option-set')
    if options is None or options.get('--depth') != '1':
        gaps.append('engine-receipt-argv-does-not-declare-exactly-depth-1')
    if options is not None and any(options.get(flag) != value
                                   for flag, value in _ENGINE_REQUIRED.items()):
        gaps.append('engine-receipt-argv-not-standard-untimestamped-selected-log')
    if options is not None and (options.get('--pattern') is None or options.get('--output') is None
                                or options['--pattern'] == options['--output']):
        gaps.append('engine-receipt-argv-not-distinct-pattern-and-output')
    return gaps


def _print_gaps(receipt, *, configuration, engine):
    """Check one supplied kprint receipt and its link to the engine output; not authenticated."""
    if receipt is None:
        return ['print-receipt-not-supplied']
    gaps, argv = [], receipt['argv']
    if receipt['exit_status'] != 0:
        gaps.append('print-receipt-exit-status-nonzero')
    if receipt['stdout_sha256'] != _sha(configuration.encode('ascii')):
        gaps.append('print-receipt-stdout-configuration-sha256-mismatch')
    if receipt['stderr_sha256'] != EMPTY_SHA256:
        gaps.append('print-receipt-stderr-not-empty')
    if len(argv) != 5 or argv[3:] != ['false', 'false'] or any(arg.startswith('-')
                                                               for arg in argv[1:3]):
        gaps.append('print-receipt-argv-not-uncoloured-unfiltered-print')
    options = _engine_options(engine['argv']) if engine is not None else None
    if options is None or len(argv) != 5 or argv[2] != options.get('--output'):
        gaps.append('print-receipt-input-is-not-engine-output-path')
    return gaps


def _probe(probe, label):
    """Validate one supplied engine/print step before any shared decoder sees it."""
    allowed = _STEP_KEYS | {'engine_receipt', 'print_receipt'}
    if type(probe) is not dict or not _STEP_KEYS <= set(probe) <= allowed:
        raise ValueError(label + ' probe requires exactly configuration, log, state_kore and '
                         'optional engine_receipt, print_receipt')
    _configuration(probe['configuration'])
    _bytes(probe['log'], label + ' log')
    _bytes(probe['state_kore'], label + ' state_kore')
    return (_receipt(probe.get('engine_receipt'), ENGINE_RECEIPT_KEYS, 'engine'),
            _receipt(probe.get('print_receipt'), PRINT_RECEIPT_KEYS, 'print'))


def _captures(captures, *, max_captures=MAX_CAPTURES):
    if type(max_captures) is not int or not 1 <= max_captures <= F03_MAX_CAPTURES:
        raise ValueError('max_captures must be an int between 1 and F03_MAX_CAPTURES')
    if type(captures) is not list or not captures or len(captures) > max_captures:
        raise ValueError('ordered complete post-startup captures required')
    receipts = []
    for index, capture in enumerate(captures):
        required = {'configuration'} if index == 0 else {'configuration', 'incoming_log',
                                                         'state_kore'}
        allowed = required if index == 0 else required | {'engine_receipt', 'print_receipt'}
        if type(capture) is not dict or not required <= set(capture) <= allowed:
            raise ValueError(f'capture {index} requires {sorted(required)}, plus optional '
                             'engine_receipt and print_receipt after the first')
        _configuration(capture['configuration'])
        if index:
            _bytes(capture['incoming_log'], 'incoming log')
            _bytes(capture['state_kore'], 'state_kore')
        receipts.append((_receipt(capture.get('engine_receipt'), ENGINE_RECEIPT_KEYS, 'engine'),
                         _receipt(capture.get('print_receipt'), PRINT_RECEIPT_KEYS, 'print')))
    return receipts


def _span_key(location):
    return tuple(location[name] for name in _SPAN_KEYS)


def _spans(rule_spans, source_path):
    """Index a supplied compiled source-attribute inventory by full span; None if absent."""
    if rule_spans is None:
        return None
    if type(rule_spans) is not list or not rule_spans or len(rule_spans) > MAX_RULE_SPANS:
        raise ValueError('rule span inventory must be a bounded nonempty list')
    spans, uniques = {}, set()
    for row in rule_spans:
        if type(row) is not dict or not {'source_path', 'unique_id', *_SPAN_KEYS} <= set(row):
            raise ValueError('rule span rows require source_path, unique_id and a full span')
        if row['source_path'] != source_path:
            raise ValueError('rule span inventory names another source path')
        key = _span_key(row)
        if (any(type(value) is not int or value < 1 for value in key)
                or (key[2], key[3]) < (key[0], key[1])):
            raise ValueError('rule span inventory rows require positive ordered spans')
        if type(row['unique_id']) is not str or _DIGEST.fullmatch(row['unique_id']) is None:
            raise ValueError('rule span inventory unique ids must be lowercase hex SHA-256')
        if key in spans or row['unique_id'] in uniques:
            raise ValueError('rule span inventory repeats a span or unique id')
        spans[key] = row['unique_id']
        uniques.add(row['unique_id'])
    return spans


def _applied(body, number, source_path, gaps):
    """Claim one applied-rule entry: explicit variant, exact selected-source spans, indented Kore."""
    if not body or body[0] not in (_APPLIED, _NO_RULES) or _ON not in body:
        gaps.append('applied-rule-entry-has-no-complete-location-section')
        return None
    marker = body.index(_ON)
    rows, configuration = body[1:marker], body[marker + 1:]
    variant = 'applied' if body[0] == _APPLIED else 'no-rules'
    if variant == 'applied' and not rows:
        gaps.append('applied-rule-entry-has-no-location')
    if variant == 'no-rules' and rows:
        gaps.append('no-rule-entry-has-location-rows')
    if not configuration or any(not line.startswith('      ') or not line[6:].strip()
                                for line in configuration):
        gaps.append('applied-rule-configuration-not-indented-kore-text')
    locations, prefix = [], '    ' + source_path + ':'
    for row in (rows if variant == 'applied' else []):
        span = _SPAN.fullmatch(row, len(prefix)) if row.startswith(prefix) else None
        if span is None:
            gaps.append('applied-rule-location-outside-selected-source-or-format')
            continue
        first, column, last, final_column = map(int, span.groups())
        if (last, final_column) < (first, column):
            gaps.append('applied-rule-location-span-reversed')
            continue
        locations.append(dict(source=source_path, start_line=first, start_column=column,
                              end_line=last, end_column=final_column))
    text = '\n'.join(configuration) + '\n' if configuration else ''
    return dict(variant=variant, first_line=number, locations=locations, configuration_text=text,
                configuration_sha256=_sha(text.encode('ascii')))


def decode_log(raw_log, *, source_path):
    """Claim every line of original log bytes, or report gaps; never infer content."""
    _bytes(raw_log, 'standard log')
    _source(source_path)
    locations, depths, warnings, applied, entries, gaps = [], [], [], [], [], []
    result = dict(applied_rule_locations=locations, execution_depths=depths,
                  depth_limit_warnings=warnings, applied_entries=applied,
                  raw_log=_raw(raw_log), raw_sha256=_sha(raw_log), raw_entries=entries,
                  gaps=gaps, format=LOG_FORMAT, source_path=source_path,
                  observation_authenticity='not-established-by-decoding-supplied-bytes')
    text = None
    if len(raw_log) > MAX_LOG_BYTES or raw_log.count(b'\n') > MAX_LOG_LINES:
        gaps.append('log-exceeds-decoder-size-bound')
    elif not raw_log.isascii():
        gaps.append('log-not-ascii')
    elif _TEXT.fullmatch(raw_log.decode('ascii')) is None:
        gaps.append('log-contains-control-character')
    elif not raw_log.endswith(b'\n'):
        gaps.append('log-empty-or-not-newline-terminated')
    else:
        text = raw_log.decode('ascii')
    if text is None:
        return result
    parsed = []
    for number, line in enumerate(text[:-1].split('\n'), 1):
        header = _HEADER.fullmatch(line)
        if header is not None:
            parsed.append((number, header[1], header[2], []))
        elif not line:
            gaps.append('log-has-blank-line')
        elif not line.startswith(' '):
            gaps.append('log-has-unclaimed-top-level-text')
        elif not parsed:
            gaps.append('log-has-text-before-first-entry')
        else:
            parsed[-1][3].append(line)
    if not parsed:
        gaps.append('log-has-no-entries')
    for index, (number, severity, kind, body) in enumerate(parsed):
        entries.append(dict(kind=kind, severity=severity, first_line=number,
                            body_lines=len(body)))
        if _ADMITTED.get(kind) != severity:
            gaps.append('log-entry-not-admitted-' + severity + '-' + kind)
        elif kind == 'InfoExecDepth':
            depth = _DEPTH.fullmatch(body[0]) if len(body) == 1 else None
            if depth is None:
                gaps.append('execution-depth-entry-not-one-exact-line')
                continue
            if depths or index != len(parsed) - 1:
                gaps.append('execution-depth-not-single-final-entry')
            depths.append(int(depth[1]))
        elif kind == 'WarnDepthLimitExceeded':
            limit = _WARN.fullmatch(body[0]) if len(body) == 1 else None
            if limit is None:
                gaps.append('depth-limit-warning-not-one-exact-line')
                continue
            if warnings:
                gaps.append('depth-limit-warning-repeated')
            warnings.append(int(limit[1]))
        else:
            entry = _applied(body, number, source_path, gaps)
            if entry is not None:
                applied.append(entry)
                locations.extend(entry['locations'])
    result['gaps'] = list(dict.fromkeys(gaps))
    return result


def _step(raw_log, state_kore, configuration, engine, printer, *, source_path, spans, rewrite):
    """Check one kore-exec/kprint pair against the observed N01 layout; infer nothing."""
    decoded = decode_log(raw_log, source_path=source_path)
    gaps = list(decoded['gaps'])
    if len(state_kore) > MAX_KORE_BYTES:
        gaps.append('state-kore-exceeds-retention-bound')
    gaps.extend(_engine_gaps(engine, raw_log=raw_log, state_kore=state_kore))
    gaps.extend(_print_gaps(printer, configuration=configuration, engine=engine))
    kinds = tuple(entry['kind'] for entry in decoded['raw_entries'])
    entry = decoded['applied_entries'][0] if len(decoded['applied_entries']) == 1 else None
    variant = entry['variant'] if entry is not None else None
    rule = None
    if rewrite:
        declared = _declared_depth(engine)
        if (kinds != _REWRITE_LAYOUT or variant != 'applied' or len(entry['locations']) != 1
                or decoded['execution_depths'] != [1] or declared is None
                or decoded['depth_limit_warnings'] != [declared]):
            gaps.append('has-no-single-observed-source-rewrite')
        elif spans is None:
            gaps.append('rule-span-inventory-not-supplied')
        elif _span_key(entry['locations'][0]) not in spans:
            gaps.append('applied-rule-span-not-in-compiled-inventory')
        else:
            location = entry['locations'][0]
            rule = dict(location, unique_id=spans[_span_key(location)])
    elif (kinds != _NO_RULE_LAYOUT or variant != 'no-rules' or entry['locations']
          or decoded['execution_depths'] != [0] or decoded['depth_limit_warnings']):
        gaps.append('has-no-observed-no-rule-entry-at-depth-0')
    return dict(decoded_log=decoded, applied=entry, rule=rule, gaps=list(dict.fromkeys(gaps)))


def _logged_input_gaps(entry, input_kore):
    """Require the logged On-configuration term to be the input Kore bytes, re-indented."""
    if entry is None or len(input_kore) > MAX_KORE_BYTES or not input_kore.isascii():
        return ['logged-configuration-is-not-input-kore']
    term = [' ' * 10 + line for line in input_kore.decode('ascii').split('\n')]
    term[-1] += ','
    expected = '\n'.join(_LOGGED_OPEN + tuple(term) + _LOGGED_CLOSE) + '\n'
    return [] if entry['configuration_text'] == expected else [
        'logged-configuration-is-not-input-kore']


def _raw_probe(probe, engine, printer):
    return dict(configuration=probe['configuration'], log=_raw(probe['log']),
                state_kore=_raw(probe['state_kore']), engine_receipt=engine,
                print_receipt=printer)


def _link(input_kore, raw_log, state_kore, configuration, engine, printer):
    """Record distinct artifact hashes per step; an engine hash is never a pretty hash."""
    options = _engine_options(engine['argv']) if engine is not None else None
    return dict(input_kore_sha256=None if input_kore is None else _sha(input_kore),
                engine_pattern_path=options.get('--pattern') if options else None,
                engine_log_sha256=_sha(raw_log), engine_output_kore_sha256=_sha(state_kore),
                engine_output_path=options.get('--output') if options else None,
                print_input_path=(printer['argv'][2] if printer is not None
                                  and len(printer['argv']) > 2 else None),
                print_stdout_configuration_sha256=_sha(configuration.encode('ascii')),
                raw_kore_has_generated_counter_label=_COUNTER in state_kore)


def _engine_template(argv):
    return tuple('<path>' if index and argv[index - 1] in ('--pattern', '--output') else arg
                 for index, arg in enumerate(argv))


def _chain_gaps(engines, printers):
    """Require pattern <- previous output, distinct outputs and one command template per tool."""
    gaps, outputs, templates, previous = [], [], set(), None
    for label, engine in engines:
        options = _engine_options(engine['argv']) if engine is not None else None
        if options is None:
            previous = None
            continue
        if previous is not None and options.get('--pattern') != previous[1].get('--output'):
            gaps.append(f'chain-{label}-pattern-is-not-{previous[0]}-output')
        outputs.append(options.get('--output'))
        templates.add(_engine_template(engine['argv']))
        previous = (label, options)
    if len(set(outputs)) != len(outputs):
        gaps.append('chain-engine-output-paths-not-distinct')
    if len(templates) > 1:
        gaps.append('chain-engine-command-template-differs')
    if len({tuple(printer['argv'][:2] + printer['argv'][3:])
            for _, printer in printers if printer is not None}) > 1:
        gaps.append('chain-print-command-template-differs')
    return gaps


def retained_run_input(text):
    """Project the separately retained frozen run input uiRun(P, E, F, L); evaluates nothing."""
    _term_text(text, 'run input')
    node = _TERMS.parse_term(text)
    if (type(node) is not tuple or node[0] != 'uiRun'
            or any(type(value) is not int for value in node[1][2:])):
        raise ValueError('run input must be one concrete uiRun(P, E, F, L) term')
    code, entry, fuel, limit = node[1]
    return dict(code_k=_TERMS.render_term(code), entry_k=_TERMS.render_term(entry),
                initial_fuel=fuel, stack_limit=limit, raw_text=text,
                raw_sha256=_sha(text.encode('ascii')))


def decode_terminal_probe(probe, *, terminal_configuration, terminal_state_kore, entry_k,
                          source_path):
    """Admit observed no-next only from the explicit no-rule entry and an unchanged replay."""
    engine, printer = _probe(probe, 'terminal')
    _configuration(terminal_configuration)
    if terminal_state_kore is not None:
        _bytes(terminal_state_kore, 'terminal input state_kore')
    _term_text(entry_k, 'entry_k')
    step = _step(probe['log'], probe['state_kore'], probe['configuration'], engine, printer,
                 source_path=source_path, spans=None, rewrite=False)
    before = _decode(terminal_configuration, entry_k)
    after = _decode(probe['configuration'], entry_k)
    gaps = list(step['gaps'])
    if terminal_state_kore is None:
        gaps.append('input-kore-not-supplied')
    else:
        gaps.extend(_logged_input_gaps(step['applied'], terminal_state_kore))
        if probe['state_kore'] != terminal_state_kore:
            gaps.append('replay-changed-raw-kore')
    if probe['configuration'] != terminal_configuration:
        gaps.append('replay-changed-raw-configuration')
    if before['snapshot'] != after['snapshot']:
        gaps.append('replay-changed-complete-state')
    depths = step['decoded_log']['execution_depths']
    has_next = None
    if len(depths) == 1 and depths[0] > 0:
        has_next = True
        gaps.append('replay-observed-a-rewrite')
    elif not gaps:
        has_next = False
    return dict(terminal_has_next=has_next, decoded_log=step['decoded_log'], before=before,
                after=after, raw_probe=_raw_probe(probe, engine, printer),
                gaps=['terminal-' + gap for gap in dict.fromkeys(gaps)],
                limitation='receipt, path and byte consistency only; argv, status, files and '
                           'pairing unauthenticated')


def decode_startup_probe(probe, *, post_startup_configuration, entry_k, source_path,
                         rule_spans=None):
    """Retain the observed startup rewrite outside the post-startup comparison."""
    engine, printer = _probe(probe, 'startup')
    _configuration(post_startup_configuration)
    _term_text(entry_k, 'entry_k')
    _source(source_path)
    step = _step(probe['log'], probe['state_kore'], probe['configuration'], engine, printer,
                 source_path=source_path, spans=_spans(rule_spans, source_path), rewrite=True)
    after = _decode(probe['configuration'], entry_k)
    first = _decode(post_startup_configuration, entry_k)
    gaps = list(step['gaps'])
    if probe['configuration'] != post_startup_configuration:
        gaps.append('result-raw-differs-from-first-capture')
    if after['snapshot'] != first['snapshot']:
        gaps.append('result-does-not-match-first-complete-capture')
    gaps = ['startup-' + gap for gap in dict.fromkeys(gaps)]
    entry, rule = step['applied'], None if gaps else step['rule']
    before_kore = None if entry is None else dict(
        text=entry['configuration_text'], sha256=entry['configuration_sha256'],
        log_first_line=entry['first_line'],
        origin='On-configuration body of the startup DebugAppliedRewriteRules entry, as logged '
               '(indentation kept)')
    return dict(rule_start_line=rule['start_line'] if rule else None, rule=rule,
                decoded_log=step['decoded_log'], before=None, before_kore=before_kore,
                before_projection='not-established: pre-startup state exists only as logged '
                                  'Kore; no Kore decoder, no typed cells, no comparison use',
                after=after, raw_probe=_raw_probe(probe, engine, printer), gaps=gaps,
                limitation='startup rule identity retained; startup semantics not evaluated; '
                           'engine input is the depth-0 initializer term, so the logged '
                           'configuration is not compared with input bytes')


def decode_capture(case_id, entry_k, captures, *, source_path, rule_spans=None,
                   terminal_probe=None, startup_probe=None, run_input=None,
                   max_captures=MAX_CAPTURES):
    """Join observed incoming spans with supplied adjacent kprint cells and their receipts."""
    if type(case_id) is not str or not case_id or len(case_id) > 64:
        raise ValueError('case_id and separately retained entry_k text required')
    _term_text(entry_k, 'entry_k')
    _source(source_path)
    receipts = _captures(captures, max_captures=max_captures)
    spans = _spans(rule_spans, source_path)
    terminal_receipts = _probe(terminal_probe, 'terminal') if terminal_probe is not None else None
    startup_receipts = _probe(startup_probe, 'startup') if startup_probe is not None else None
    retained = retained_run_input(run_input) if run_input is not None else None
    gaps = [] if spans is not None else ['rule-span-inventory-not-supplied']
    startup = None
    if startup_probe is None:
        gaps.append('startup-probe-not-supplied')
    else:
        startup = decode_startup_probe(startup_probe,
            post_startup_configuration=captures[0]['configuration'], entry_k=entry_k,
            source_path=source_path, rule_spans=rule_spans)
        gaps.extend(startup['gaps'])
    logs, rules, identities, links = [], [], [], []
    previous = startup_probe['state_kore'] if startup_probe is not None else None
    for index, capture in enumerate(captures[1:]):
        engine, printer = receipts[index + 1]
        step = _step(capture['incoming_log'], capture['state_kore'], capture['configuration'],
                     engine, printer, source_path=source_path, spans=spans, rewrite=True)
        edge = list(step['gaps'])
        edge.extend(_logged_input_gaps(step['applied'], previous) if previous is not None
                    else ['input-kore-not-supplied'])
        edge = list(dict.fromkeys(edge))
        logs.append(step['decoded_log'])
        gaps.extend(f'edge-{index}-{gap}' for gap in edge)
        rules.append(None if edge else step['rule']['start_line'])
        identities.append(None if edge else step['rule'])
        links.append(_link(previous, capture['incoming_log'], capture['state_kore'],
                           capture['configuration'], engine, printer))
        previous = capture['state_kore']
    terminal = None
    if terminal_probe is not None:
        terminal = decode_terminal_probe(terminal_probe,
            terminal_configuration=captures[-1]['configuration'], terminal_state_kore=previous,
            entry_k=entry_k, source_path=source_path)
        gaps.extend(terminal['gaps'])
    engines = [('startup', startup_receipts[0])] if startup_receipts else []
    printers = [('startup', startup_receipts[1])] if startup_receipts else []
    for index, (engine, printer) in enumerate(receipts[1:]):
        engines.append((f'edge-{index}', engine))
        printers.append((f'edge-{index}', printer))
    if terminal_receipts:
        engines.append(('terminal', terminal_receipts[0]))
        printers.append(('terminal', terminal_receipts[1]))
    gaps.extend(_chain_gaps(engines, printers))
    texts, notes = zip(*(_configuration(capture['configuration']) for capture in captures))
    projected = _OBSERVATIONS.decode_run(case_id, entry_k, list(texts),
        rule_start_lines=rules if all(rule is not None for rule in rules) else None,
        terminal_has_next=terminal['terminal_has_next'] if terminal is not None else None)
    gaps.extend(projected['gaps'])
    if retained is not None:
        initial = projected['observation']['initial']
        for field in ('code_k', 'entry_k', 'initial_fuel', 'stack_limit'):
            if retained[field] != initial[field]:
                gaps.append('run-input-' + field + '-differs-from-post-startup-initial')
    raw_captures = [dict(configuration=capture['configuration']) if index == 0 else
                    dict(configuration=capture['configuration'],
                         incoming_log=_raw(capture['incoming_log']),
                         state_kore=_raw(capture['state_kore']),
                         engine_receipt=receipts[index][0], print_receipt=receipts[index][1])
                    for index, capture in enumerate(captures)]
    gaps = list(dict.fromkeys(gaps))
    projected.update(format=FORMAT, comparison_ready=not gaps, gaps=gaps,
                     source_path=source_path, raw_captures=raw_captures,
                     original_configurations=[capture['configuration'] for capture in captures],
                     configuration_unit_tails=list(notes), decoded_incoming_logs=logs,
                     observed_rule_identities=identities, edge_links=links,
                     terminal_probe=terminal,
                     raw_terminal_probe=terminal['raw_probe'] if terminal else None,
                     startup_probe=startup,
                     raw_startup_probe=startup['raw_probe'] if startup else None,
                     run_input=retained,
                     projection_boundary=dict(cells=list(_OBSERVATIONS.CELLS),
                                              printer_omitted=['generatedCounter'],
                                              note='raw Kore outputs and logged configurations '
                                                   'carry <generatedCounter>; kprint output does '
                                                   'not show it; retained raw, not projected'),
                     evidence=_evidence())
    projected['clock'].update(
        rule_map='observed-DebugAppliedRewriteRules-spans-matched-to-compiled-inventory',
        boundary='supplied-single-rewrite-kore-exec-kprint-pairs',
        helper_rewrite_coverage='engine-count-declaration-only',
        startup='separate-observed-startup-probe-required')
    return projected


def observation_bundle(decoded_runs, *, source_sha256):
    """Assemble supplied observations in order, without opening expectations.

    source_sha256 is a declaration from the producing receipt. Program identity
    comes from observed initial code and separately retained input entry. This
    container cannot establish missing corpus coverage or producer authenticity;
    those require the independent comparison and execution/result audits.
    """
    if type(decoded_runs) is not list or not decoded_runs:
        raise ValueError('ordered decoded runs required')
    if type(source_sha256) is not str or _DIGEST.fullmatch(source_sha256) is None:
        raise ValueError('declared source_sha256 must be lowercase hex SHA-256')
    if any(type(decoded) is not dict or decoded.get('format') != FORMAT
           for decoded in decoded_runs):
        raise ValueError('decode_capture results required')
    programs, runs, identities, gaps = [], [], {}, []
    if len({decoded['source_path'] for decoded in decoded_runs}) != 1:
        gaps.append('runs-source-path-differs')
    for index, decoded in enumerate(decoded_runs):
        run = decoded['observation']
        case = run['case_id']
        identity = dict(case_id=case, code_k=run['initial']['code_k'],
                        entry_k=run['initial']['entry_k'])
        if case not in identities:
            identities[case] = identity
            programs.append(identity)
        elif identities[case] != identity:
            gaps.append(f'run-{index}-program-identity-changed')
        gaps.extend(f'run-{index}-{gap}' for gap in decoded['gaps'])
        runs.append(run)
    evidence = _evidence()
    evidence.update(execution_receipt='per-step kore-exec and kprint receipt consistency in '
                                      'raw_runs; not authenticated',
                    adapter_execution='supplied-capture-bytes-and-text-only')
    return dict(format=BUNDLE_FORMAT,
        observations=dict(producer=PRODUCER, profile=_OBSERVATIONS.PROFILE,
                          source_sha256=source_sha256, boundary='post-startup',
                          programs=programs, runs=runs),
        raw_runs=decoded_runs, comparison_ready=not gaps,
        projection=dict(boundary='post-startup', gaps=gaps,
            rule_identity='observed DebugAppliedRewriteRules full spans equal to one supplied '
                          'compiled inventory entry',
            trace_rule_ids='UniqueIds only from the compiled inventory entry with the same full '
                           'span; none inferred from states',
            startup='separate raw startup probes required; pre-startup state retained as '
                    'logged Kore only, not typed',
            terminal_has_next='separate terminal replay: explicit no-rule entry, depth 0, '
                              'unchanged raw Kore and kprint text, consistent receipts',
            cell_boundary='nine declared main.k cells as printed by kprint; generatedCounter in '
                          'raw Kore is not projected',
            limitations=['supplied bytes, receipts, paths and pairing remain unauthenticated',
                         'actual format observed for one frozen N01 fuel-1 capture only',
                         'corpus coverage established only by independent comparison',
                         'no typed pre-startup projection',
                         'no B, proof, circuit, native or target correspondence']),
        evidence=evidence)
