"""Project supplied complete cells and original Kore standard-format log bytes.

No execution, source-rule selection, oracle import or evaluator is present.
Rule positions come only from DebugAppliedRewriteRules output. InfoExecDepth
comes from the engine's reported rewrite count. The format is source-inspected
in K 7.1.337's installed Haskell backend; tests are synthetic owner fixtures,
not engine receipts. This module does not decode Kore terms: each corresponding
pretty-printed configuration must also be retained by its producing adapter.

Logs are original bytes, hashed and retained (base64) before strict ASCII
decoding. Every line must be claimed by an admitted kore-exec entry; other
entry kinds, unclaimed text, blank lines, control characters and size-bound
excess are gaps. Supplied receipts are checked only for consistency of argv
(exactly --depth 1), exit status and capture hashes; they are not authenticated.
Extra cells and K sequences are actual-format gaps until observed kore-print
output decides their handling. Actual log layout remains unobserved. Comparison
readiness concerns supplied data; reliance stays blocked pending observation,
fresh audit and owner confirmation.
"""
import base64
import hashlib
import importlib.util
from pathlib import Path
import re


_PATH = Path(__file__).with_name('k_observations.py')
_SPEC = importlib.util.spec_from_file_location('sherlock_k_observations', _PATH)
_OBSERVATIONS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_OBSERVATIONS)

FORMAT = 'sherlock-k-standard-log-capture/1'
BUNDLE_FORMAT = 'sherlock-k-normal-observations/1'
PRODUCER = 'sherlock-k-reference'
LOG_FORMAT = 'installed-kore-standard-log/1'
MAX_LOG_BYTES = 4 * 1024 * 1024
MAX_LOG_LINES = 65536
RECEIPT_KEYS = frozenset({'argv', 'exit_status', 'log_sha256', 'configuration_sha256'})
ADAPTER_SOURCES = {name: hashlib.sha256(Path(__file__).with_name(name).read_bytes()).hexdigest()
                   for name in (Path(__file__).name, 'k_observations.py', 'k_terms.py')}
_ADMITTED = {'DebugAppliedRewriteRules': 'Debug', 'InfoExecDepth': 'Info'}
_HEADER = re.compile(r'kore-exec: (Debug|Info|Warning|Error|Critical) '
                     r'\(([A-Za-z][A-Za-z0-9]{0,63})\):')
_DEPTH = re.compile(r'    execution complete: exec depth: (0|[1-9][0-9]{0,17})')
_SPAN = re.compile(r'([1-9][0-9]{0,8}):([1-9][0-9]{0,8})-([1-9][0-9]{0,8}):([1-9][0-9]{0,8})')
_TEXT = re.compile(r'[\x20-\x7e\n]*')
_SOURCE = re.compile(r'/[\x21-\x7e]{1,4095}')
_DIGEST = re.compile(r'[0-9a-f]{64}')
_TAG = re.compile(r'<([A-Za-z][A-Za-z0-9]*)>')
_APPLIED = '    The rules at following locations were applied:'
_ON = '    On configuration:'


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _evidence():
    return dict(receipts='supplied argv/exit-status/hash consistency checked; not authenticated',
                producer_authenticity='not-established',
                actual_format='unobserved: extra cells, K sequences and log layout are explicit gaps',
                rule_identity='start line of observed span only; frozen span table pending',
                adapter_sources=dict(ADAPTER_SOURCES),
                adapter_source_binding='hashed at import; not bound to loaded code',
                reliance='blocked pending observed formats, fresh audit and owner confirmation')


def _source(source_path):
    if type(source_path) is not str or _SOURCE.fullmatch(source_path) is None:
        raise ValueError('exact observed absolute printable-ASCII source path required')


def _log_bytes(raw_log):
    if type(raw_log) is not bytes:
        raise ValueError('complete original standard log bytes required')


def _configuration(text):
    """Reject unobserved actual-format cells and sequences explicitly."""
    if type(text) is not str:
        raise ValueError('complete pretty-printed configuration text required')
    try:
        text.encode('utf-8')
    except UnicodeEncodeError:
        raise ValueError('configuration text is not encodable UTF-8') from None
    for tag in _TAG.finditer(text):
        if tag[1] != 'generatedTop' and tag[1] not in _OBSERVATIONS.CELLS:
            raise ValueError('actual-format gap: cell <' + tag[1] + '> is not admitted '
                             'until observed kore-print output decides its handling')
    if '~>' in text:
        raise ValueError('actual-format gap: K sequence ~> is not admitted until observed')


def _receipt(value):
    if value is None:
        return None
    if type(value) is not dict or set(value) != RECEIPT_KEYS:
        raise ValueError('receipt requires exactly argv, exit_status, log_sha256, '
                         'configuration_sha256')
    argv = value['argv']
    if (type(argv) is not list or not argv
            or any(type(arg) is not str or not arg or '\x00' in arg for arg in argv)):
        raise ValueError('receipt argv must be a nonempty list of nonempty strings')
    if type(value['exit_status']) is not int:
        raise ValueError('receipt exit_status must be an int')
    for key in ('log_sha256', 'configuration_sha256'):
        if type(value[key]) is not str or _DIGEST.fullmatch(value[key]) is None:
            raise ValueError('receipt ' + key + ' must be lowercase hex SHA-256')
    return dict(argv=list(argv), exit_status=value['exit_status'],
                log_sha256=value['log_sha256'],
                configuration_sha256=value['configuration_sha256'])


def _receipt_gaps(receipt, *, raw_log, configuration):
    """Check supplied receipt consistency only; this does not authenticate it."""
    if receipt is None:
        return ['receipt-not-supplied']
    gaps = []
    if receipt['exit_status'] != 0:
        gaps.append('receipt-exit-status-nonzero')
    if receipt['log_sha256'] != _sha(raw_log):
        gaps.append('receipt-log-sha256-mismatch')
    if receipt['configuration_sha256'] != _sha(configuration.encode('utf-8')):
        gaps.append('receipt-configuration-sha256-mismatch')
    argv = receipt['argv']
    depths = [argv[index + 1] if index + 1 < len(argv) else None
              for index, arg in enumerate(argv) if arg == '--depth']
    if depths != ['1'] or any(arg.startswith('--depth=') for arg in argv):
        gaps.append('receipt-argv-does-not-declare-exactly-depth-1')
    return gaps


def _probe(probe, required):
    if type(probe) is not dict or not set(required) <= set(probe) <= set(required) | {'receipt'}:
        raise ValueError('probe requires exactly ' + ', '.join(required) + ' and optional receipt')
    for key in required:
        if key == 'log':
            _log_bytes(probe[key])
        else:
            _configuration(probe[key])
    return _receipt(probe.get('receipt'))


def _captures(captures):
    if type(captures) is not list or not captures:
        raise ValueError('ordered complete post-startup captures required')
    receipts = []
    for index, capture in enumerate(captures):
        required = {'configuration'} if index == 0 else {'configuration', 'incoming_log'}
        allowed = required if index == 0 else required | {'incoming_receipt'}
        if type(capture) is not dict or not required <= set(capture) <= allowed:
            raise ValueError(f'capture {index} requires {sorted(required)}, plus optional '
                             'incoming_receipt after the first')
        _configuration(capture['configuration'])
        if index:
            _log_bytes(capture['incoming_log'])
        receipts.append(_receipt(capture.get('incoming_receipt')))
    return receipts


def _raw_log(raw_log):
    return dict(base64=base64.b64encode(raw_log).decode('ascii'), sha256=_sha(raw_log),
                size=len(raw_log))


def _raw_probe(probe, receipt):
    raw = {key: _raw_log(value) if key == 'log' else value
           for key, value in probe.items() if key != 'receipt'}
    raw['receipt'] = receipt
    return raw


def _applied(body, source_path, locations, gaps):
    """Claim one applied-rule entry: marker, exact selected-source spans, indented Kore."""
    if not body or body[0] != _APPLIED or _ON not in body:
        gaps.append('applied-rule-entry-has-no-complete-location-section')
        return
    marker = body.index(_ON)
    rows, configuration = body[1:marker], body[marker + 1:]
    if not rows:
        gaps.append('applied-rule-entry-has-no-location')
    if not configuration or any(not line.startswith('      ') or not line[6:].strip()
                                for line in configuration):
        gaps.append('applied-rule-configuration-not-indented-kore-text')
    prefix = '    ' + source_path + ':'
    for row in rows:
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


def decode_log(raw_log, *, source_path):
    """Claim every line of original log bytes, or report gaps; never infer content."""
    _log_bytes(raw_log)
    _source(source_path)
    locations, depths, entries, gaps = [], [], [], []
    result = dict(applied_rule_locations=locations, execution_depths=depths,
                  raw_log=_raw_log(raw_log), raw_sha256=_sha(raw_log), raw_entries=entries,
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
            if kind == 'WarnDepthLimitExceeded':
                gaps.append('observed-rewrite-depth-limit-exceeded')
        elif kind == 'InfoExecDepth':
            depth = _DEPTH.fullmatch(body[0]) if len(body) == 1 else None
            if depth is None:
                gaps.append('execution-depth-entry-not-one-exact-line')
                continue
            if depths or index != len(parsed) - 1:
                gaps.append('execution-depth-not-single-final-entry')
            depths.append(int(depth[1]))
        else:
            _applied(body, source_path, locations, gaps)
    result['gaps'] = list(dict.fromkeys(gaps))
    return result


def decode_terminal_probe(probe, *, terminal_configuration, entry_k, source_path):
    """Admit observed no-next only from a clean log and consistent depth-1 receipt."""
    receipt = _probe(probe, ('configuration', 'log'))
    _configuration(terminal_configuration)
    decoded_log = decode_log(probe['log'], source_path=source_path)
    before = _OBSERVATIONS.decode_configuration(terminal_configuration, entry_k=entry_k)
    after = _OBSERVATIONS.decode_configuration(probe['configuration'], entry_k=entry_k)
    gaps = ['terminal-' + gap for gap in decoded_log['gaps']]
    gaps.extend('terminal-' + gap for gap in _receipt_gaps(
        receipt, raw_log=probe['log'], configuration=probe['configuration']))
    if probe['configuration'] != terminal_configuration:
        gaps.append('terminal-replay-changed-raw-configuration')
    if before['snapshot'] != after['snapshot']:
        gaps.append('terminal-replay-changed-complete-state')
    depths = decoded_log['execution_depths']
    has_next = None
    if len(depths) == 1 and depths[0] > 0:
        has_next = True
        gaps.append('terminal-replay-observed-a-rewrite')
    elif depths != [0] or decoded_log['applied_rule_locations']:
        gaps.append('terminal-replay-has-no-single-observed-execution-depth')
    elif not gaps:
        has_next = False
    return dict(terminal_has_next=has_next, decoded_log=decoded_log, before=before,
                after=after, raw_probe=_raw_probe(probe, receipt), gaps=gaps,
                limitation='receipt consistency only; argv, status and pairing unauthenticated')


def decode_startup_probe(probe, *, post_startup_configuration, entry_k, source_path):
    """Retain a separate observed startup edge outside post-startup comparison."""
    receipt = _probe(probe, ('before_configuration', 'configuration', 'log'))
    _configuration(post_startup_configuration)
    decoded_log = decode_log(probe['log'], source_path=source_path)
    before = _OBSERVATIONS.decode_configuration(probe['before_configuration'], entry_k=entry_k)
    after = _OBSERVATIONS.decode_configuration(probe['configuration'], entry_k=entry_k)
    first = _OBSERVATIONS.decode_configuration(post_startup_configuration, entry_k=entry_k)
    gaps = ['startup-' + gap for gap in decoded_log['gaps']]
    gaps.extend('startup-' + gap for gap in _receipt_gaps(
        receipt, raw_log=probe['log'], configuration=probe['configuration']))
    locations = decoded_log['applied_rule_locations']
    if len(locations) != 1 or decoded_log['execution_depths'] != [1]:
        gaps.append('startup-has-no-single-observed-source-rewrite')
    if probe['configuration'] != post_startup_configuration:
        gaps.append('startup-result-raw-differs-from-first-capture')
    if after['snapshot'] != first['snapshot']:
        gaps.append('startup-result-does-not-match-first-complete-capture')
    rule = None if gaps else locations[0]['start_line']
    return dict(rule_start_line=rule, decoded_log=decoded_log, before=before, after=after,
                raw_probe=_raw_probe(probe, receipt), gaps=gaps,
                limitation='source rule identity retained; startup semantics not evaluated; '
                           'before_configuration not bound by receipt')


def decode_capture(case_id, entry_k, captures, *, source_path, terminal_probe=None,
                   startup_probe=None):
    """Join observed incoming source positions with supplied adjacent full cells."""
    if type(case_id) is not str or not case_id or type(entry_k) is not str:
        raise ValueError('case_id and separately retained entry_k text required')
    _source(source_path)
    receipts = _captures(captures)
    if terminal_probe is not None:
        _probe(terminal_probe, ('configuration', 'log'))
    if startup_probe is not None:
        _probe(startup_probe, ('before_configuration', 'configuration', 'log'))
    logs, rules, gaps = [], [], []
    for index, capture in enumerate(captures[1:]):
        decoded = decode_log(capture['incoming_log'], source_path=source_path)
        logs.append(decoded)
        edge = list(decoded['gaps'])
        edge.extend(_receipt_gaps(receipts[index + 1], raw_log=capture['incoming_log'],
                                  configuration=capture['configuration']))
        locations = decoded['applied_rule_locations']
        if len(locations) != 1 or decoded['execution_depths'] != [1]:
            edge.append('has-no-single-observed-source-rewrite')
        gaps.extend(f'edge-{index}-{gap}' for gap in edge)
        rules.append(None if edge else locations[0]['start_line'])
    terminal = None
    if terminal_probe is not None:
        terminal = decode_terminal_probe(terminal_probe,
            terminal_configuration=captures[-1]['configuration'], entry_k=entry_k,
            source_path=source_path)
        gaps.extend(terminal['gaps'])
    projected = _OBSERVATIONS.decode_run(case_id, entry_k,
        [capture['configuration'] for capture in captures],
        rule_start_lines=rules if all(rule is not None for rule in rules) else None,
        terminal_has_next=terminal['terminal_has_next'] if terminal is not None else None)
    gaps.extend(projected['gaps'])
    startup = None
    if startup_probe is None:
        gaps.append('startup-probe-not-supplied')
    else:
        startup = decode_startup_probe(startup_probe,
            post_startup_configuration=captures[0]['configuration'], entry_k=entry_k,
            source_path=source_path)
        gaps.extend(startup['gaps'])
    raw_captures = [dict(configuration=capture['configuration']) if index == 0 else
                    dict(configuration=capture['configuration'],
                         incoming_log=_raw_log(capture['incoming_log']),
                         incoming_receipt=receipts[index])
                    for index, capture in enumerate(captures)]
    projected.update(format=FORMAT, comparison_ready=not gaps, gaps=gaps,
                     source_path=source_path, raw_captures=raw_captures,
                     decoded_incoming_logs=logs, terminal_probe=terminal,
                     raw_terminal_probe=terminal['raw_probe'] if terminal else None,
                     startup_probe=startup,
                     raw_startup_probe=startup['raw_probe'] if startup else None,
                     evidence=_evidence())
    projected['clock'].update(rule_map='observed-DebugAppliedRewriteRules-source-locations',
                              boundary='supplied-single-rewrite-capture-pairs',
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
    evidence.update(execution_receipt='per-invocation receipt consistency in raw_runs; '
                                      'not authenticated',
                    adapter_execution='supplied-capture-bytes-and-text-only')
    return dict(format=BUNDLE_FORMAT,
        observations=dict(producer=PRODUCER, profile=_OBSERVATIONS.PROFILE,
                          source_sha256=source_sha256, boundary='post-startup',
                          programs=programs, runs=runs),
        raw_runs=decoded_runs, comparison_ready=not gaps,
        projection=dict(boundary='post-startup', gaps=gaps,
            rule_identity='explicit DebugAppliedRewriteRules source locations only',
            trace_rule_ids='Kore UniqueIds are not source positions; no mapping inferred',
            startup='separate raw startup probes required',
            terminal_has_next='separate terminal replay depth with consistent receipt',
            limitations=['supplied bytes, receipts and pairing remain unauthenticated',
                         'actual kore-exec/kore-print formats unobserved',
                         'corpus coverage established only by independent comparison',
                         'no B, proof, circuit, native or target correspondence']),
        evidence=evidence)
