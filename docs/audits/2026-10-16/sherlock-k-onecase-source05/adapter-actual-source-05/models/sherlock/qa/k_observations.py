"""Decode supplied Sherlock K pretty-printed complete configurations only.

This module imports no oracle, evaluator or launcher. Input is ordinary concrete
cell text, optionally enclosed in generatedTop. Terms use main.k constructors,
decimal Ints, Bools and finite Int-key maps. Whitespace is normalized; map
display follows the host lexical decimal-key order; frame order is preserved.
KAST JSON, Kore, debugger logs, partial cells and unevaluated helper terms
require separate adapters. No K runtime was invoked to
establish this input format. Normal syntax tests are hand-authored examples.

decode_configuration returns a snapshot plus untouched raw text/cells/digest.
decode_run consumes ordered *post-startup* configurations including terminal
publication, and returns a host-shaped observation inside a diagnostic envelope.
Incoming source-rule lines and terminal_has_next are supplied observations, not
inferred by evaluating rules or consulting expectations. Missing observations
stay None and produce gaps. A comparison-ready projection is not a comparison
pass, execution receipt, authenticated producer or proof of trace completeness.
"""
import hashlib
import importlib.util
from pathlib import Path
import re


_TERM_PATH = Path(__file__).with_name('k_terms.py')
_TERM_SPEC = importlib.util.spec_from_file_location('sherlock_k_terms', _TERM_PATH)
_TERMS = importlib.util.module_from_spec(_TERM_SPEC)
_TERM_SPEC.loader.exec_module(_TERMS)
_parse = _TERMS.parse_term
_render = _TERMS.render_term


PROJECTION = 'sherlock-k-complete-cells/1'
PROFILE = 'sherlock-k-a/1'
CELLS = ('k', 'env', 'stack', 'depth', 'stackLimit', 'code', 'fuel', 'initialFuel', 'out')
CELL = re.compile(r'\s*<([A-Za-z][A-Za-z0-9]*)>(.*?)</\1>', re.DOTALL)


def _frames(node):
    result = []
    while node != ('uiNoFrames', []):
        if type(node) is not tuple or node[0] != 'uiFrame':
            raise ValueError('unsupported stack representation')
        frame, node = node[1]
        result.append(_render(frame))
    return result


def _cells(raw):
    if type(raw) is not str:
        raise ValueError('configuration must be complete K cell text')
    text = raw.strip()
    if text.startswith('<generatedTop>') and text.endswith('</generatedTop>'):
        text = text[len('<generatedTop>'):-len('</generatedTop>')]
    values = {}
    position = 0
    while text[position:].strip():
        match = CELL.match(text, position)
        if match is None or match[1] not in CELLS or match[1] in values:
            raise ValueError('expected exactly the complete main.k cells')
        values[match[1]] = match[2]
        position = match.end()
    if set(values) != set(CELLS):
        raise ValueError('missing complete main.k cells')
    return values


def decode_configuration(raw_text, *, entry_k):
    """Decode all nine observed cells; entry comes from separately retained input."""
    raw_cells = _cells(raw_text)
    cells = {name: _parse(value) for name, value in raw_cells.items()}
    for name in ('depth', 'stackLimit', 'fuel', 'initialFuel'):
        if type(cells[name]) is not int or cells[name] < 0:
            raise ValueError('normal configuration requires nonnegative ' + name)
    if cells['fuel'] > cells['initialFuel']:
        raise ValueError('remaining fuel exceeds observed initial fuel')
    snapshot = dict(code_k=_render(cells['code']), entry_k=_render(_parse(entry_k)),
                    control_k=_render(cells['k']), environment_k=_render(cells['env']),
                    frames_top_first_k=_frames(cells['stack']), depth=cells['depth'],
                    stack_limit=cells['stackLimit'], remaining_fuel=cells['fuel'],
                    initial_fuel=cells['initialFuel'], output_k=_render(cells['out']),
                    charged_steps=cells['initialFuel'] - cells['fuel'])
    return dict(snapshot=snapshot, raw_configuration=raw_text, raw_cells=raw_cells,
                raw_entry_k=entry_k,
                raw_sha256=hashlib.sha256(raw_text.encode('utf-8')).hexdigest(),
                projection=PROJECTION)


def decode_run(case_id, entry_k, configurations, *, rule_start_lines=None,
               terminal_has_next=None):
    """Project supplied states without filling any unobserved transition/state.

    Configurations must be adjacent semantic boundaries, not arbitrary krun
    depth samples. K helper rewrites are not A fuel events. A raw debugger trace
    needs an explicit boundary/rule adapter before using this entrypoint.
    """
    if type(configurations) is not list or not configurations:
        raise ValueError('at least one supplied complete configuration required')
    decoded = [decode_configuration(raw, entry_k=entry_k) for raw in configurations]
    states = [value['snapshot'] for value in decoded]
    edges = len(states) - 1
    gaps = []
    if rule_start_lines is None:
        rules = [None] * edges
        gaps.append('rule_start_lines-not-supplied')
    elif type(rule_start_lines) is list and len(rule_start_lines) == edges:
        rules = rule_start_lines
        if any(type(line) is not int or line < 1 for line in rules):
            raise ValueError('incoming rule lines must be positive observed source lines')
    else:
        raise ValueError('one incoming rule line required per supplied edge')
    if terminal_has_next is None:
        gaps.append('terminal-has-next-not-supplied')
    elif type(terminal_has_next) is not bool:
        raise ValueError('terminal_has_next must be an observed Boolean')
    first, last = states[0], states[-1]
    if (first['control_k'] != 'uiEval(' + first['entry_k'] + ')'
            or first['charged_steps'] != 0 or first['output_k'] != 'uiPending()'
            or first['environment_k'] != '.Map' or first['depth'] != 0
            or first['frames_top_first_k']):
        gaps.append('post-startup-initial-configuration-not-observed')
    records = []
    for index, (before, after, rule) in enumerate(zip(states, states[1:], rules)):
        charge = before['remaining_fuel'] - after['remaining_fuel']
        publication = (before['output_k'] == 'uiPending()'
                       and after['output_k'] != 'uiPending()' and after['control_k'] == '.K')
        if charge not in (0, 1) or (charge == 0 and not publication) or (charge == 1 and publication):
            gaps.append(f'edge-{index}-does-not-observe-one-semantic-transition')
        if any(before[field] != after[field] for field in ('code_k', 'entry_k', 'initial_fuel', 'stack_limit')):
            gaps.append(f'edge-{index}-fixed-state-fields-changed')
        records.append(dict(rule_start_line=rule, charge=charge, before=before, after=after))
    if not records or last['control_k'] != '.K' or last['output_k'] == 'uiPending()':
        gaps.append('terminal-publication-not-observed')
    if terminal_has_next is True:
        gaps.append('supplied-terminal-has-another-transition')
    return dict(observation=dict(case_id=case_id, initial=first, records=records,
                                 terminal=last, terminal_has_next=terminal_has_next),
                raw_configurations=list(configurations),
                decoded_configurations=decoded, projection=PROJECTION, profile=PROFILE,
                boundary='post-startup', comparison_ready=not gaps, gaps=gaps,
                clock=dict(charge='observed-fuel-delta', rule_map='supplied-incoming-main.k-source-lines',
                           boundary='supplied-adjacent-semantic-configurations',
                           helper_rewrite_coverage='not-established',
                           startup='not-observed'),
                execution_authenticity='not-established-by-decoding-supplied-text')
