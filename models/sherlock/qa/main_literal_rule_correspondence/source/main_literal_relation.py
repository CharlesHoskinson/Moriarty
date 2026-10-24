"""Finite data adapter for actual unchanged main.k literal/publication captures.

No evaluator, process launcher, image-state constructor or expected-output fill.
The whole pre-existing image observation is retained, not recomputed.  Its
declared source_line fields become paired only with independently logged main.k
spans; they never serve as evidence that a rule fired.
"""
import argparse
import hashlib
import json
from pathlib import Path

import literal_complete_images as wire
import complete_images_runtime_data as images
import k_capture_observations as capture
from balanced_kore import application, arguments
from parsed_problem_gate import parsed_problem

same = images.same
PC = [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0]
SC = [0, 0, 0, 0, 0, 1]
SOURCE = ('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/'
          'k-actual-01/compile-input/models/sherlock/k/main.k')
SOURCE_SHA = 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'
DEFINITION_SHA = 'd795e32885e5651b49ea59eef7b57595a431a6908ef8e9c36901a5ec5e43b6e4'
CASE = 'real-source-literal-complete-main-rules'
RUN = b"LbluiRun'LParUndsCommUndsCommUndsCommUndsRParUnds'MORIARTY-UI-SYNTAX'Unds'UIRun'Unds'UIFuns'Unds'UIExpr'Unds'Int'Unds'Int{}"

SAFE_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-C4-A80-K-capture-existing-ELF-operation-preparation-20261019/frozen-safe-input-only-01/literal_complete_k_input.json', 'bytes': 751, 'sha256': '31e8ee1eb54803367df83f1b8c2ae9c59585341eb19691e66343080cabbe59f1'}
CARRIER_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-literal-complete-images-empty-continuation-decode-tail-20261019/actual/06-decode/complete-observation.json', 'bytes': 6886, 'sha256': '4191998106c719831b0af41ef53782296757c2db2344e67e01634f0c0f89d746'}
BINDING_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-main-literal-complete-rule-correspondence-20261019/ROOT-OPAQUE-NEW-MAIN-EXPECTED-BINDING.json', 'bytes': 1204, 'sha256': '7cba0378f0e64a191ef7b97d3d560e3153efdc2dcb83dc9ffe44255f4df27d67'}


def need(condition, message):
    if not condition:
        raise ValueError(message)


def le(row):
    wire.vector(row, 16, 'LE UInt128', 256)
    return sum(x << (8 * i) for i, x in enumerate(row))


def profile(document):
    """Explicit one-entry positive representation premise, not source admission."""
    q = wire.request(document)
    need(q['observation_budget'] == 2, 'two semantic boundaries required')
    need(q['capacities']['program'] == PC and q['capacities']['state'] == SC,
         'selected complete335 scalar geometry only')
    p, s = q['program'], q['init']
    need(p['root'] == [0, *PC], 'selected complete Program counts/entry differ')
    expr = p['tables'][4][0]
    need(expr[:3] == [0, 0, 0] and expr[3] == 0 and expr[4:] == [0] * 7,
         'entry must be exact Literal(Constant,0), no ignored active expression payload')
    const = p['tables'][13][0]
    need(const[:3] == [0, 0, 0] and const[19:] == [0] * 3,
         'active constant must be UInt with inactive payload zero')
    value = le(const[3:19])
    root = s['root']
    need(root[:5] == [0] * 5 and root[5] == 1 and root[6:8] == [0, 0]
         and root[8] <= q['capacities']['stack_limit'] and root[41:] == [0] * 4,
         'complete empty Init root required')
    need(all(lane == 0 for table in s['tables'] for row in table for lane in row),
         'all allocated Init arenas, including control padding, must be zero')
    fuel = le(root[9:25])
    need(fuel > 0 and root[9:25] == root[25:41], 'positive equal initial/remaining fuel required')
    return q, value, fuel, root[8]


def render(document):
    _, value, fuel, limit = profile(document)
    return f'uiRun(uiNoFuns(),uiLiteral(uiU({value})),{fuel},{limit})\n'


def frame(parsed):
    """Use the pre-existing main.k initializer recipe; retain complete parsed term."""
    problem = parsed_problem(parsed)
    need(problem is None, problem)
    term = parsed.strip()
    arguments(term, RUN, 4)
    # Complete balanced application only; the real kast sort/module is bound by
    # the frozen argv and resulting initializer use, not guessed from a label.
    prefix = (b"LblinitGeneratedTopCell{}(Lbl'UndsPipe'-'-GT-Unds'{}(inj{SortKConfigVar{}, "
              b'SortKItem{}}(\\dv{SortKConfigVar{}}("$PGM")),inj{SortUIRun{}, SortKItem{}}(')
    return prefix + term + b')))\n'


def read(path):
    return json.loads(Path(path).read_bytes(), object_pairs_hook=wire.unique_object)


def save(path, obj):
    raw = (json.dumps(obj, sort_keys=True, separators=(',', ':'), allow_nan=False) + '\n').encode()
    with Path(path).open('xb') as output:
        output.write(raw)


def successful(plan, phase_id):
    phase = next(p for p in plan['phases'] if p['id'] == phase_id)
    root = Path(plan['root_receipts']) / phase_id
    result, command = read(root / 'LAUNCH-RESULT.json'), read(root / 'COMMAND.json')
    need(result['id'] == phase_id and result['returncode'] == 0
         and result['outcome'] == 'exited' and result['timed_out'] is False
         and result['containment_breach'] is False and result['remaining'] == [],
         'child did not finish successfully: ' + phase_id)
    need(command['argv'] == phase['argv'], 'actual argv differs: ' + phase_id)
    return phase, result


def probe(plan, engine_id, print_id):
    engine, result = successful(plan, engine_id)
    printer, printed = successful(plan, print_id)
    raw_log = Path(engine['stderr']).read_bytes()
    raw_kore = Path(engine['products'][0]).read_bytes()
    text = Path(printer['stdout']).read_text(encoding='ascii')
    ep = {'argv': engine['argv'], 'exit_status': result['returncode'],
          'stdout_sha256': images.pin(engine['stdout'])['sha256'],
          'stderr_sha256': hashlib.sha256(raw_log).hexdigest(),
          'output_sha256': hashlib.sha256(raw_kore).hexdigest()}
    pp = {'argv': printer['argv'], 'exit_status': printed['returncode'],
          'stdout_sha256': images.pin(printer['stdout'])['sha256'],
          'stderr_sha256': images.pin(printer['stderr'])['sha256']}
    return {'configuration': text, 'log': raw_log, 'state_kore': raw_kore,
            'engine_receipt': ep, 'print_receipt': pp}


def checked_run_input(source_document, raw):
    need(raw == render(source_document).encode('ascii'), 'actual query bytes differ from SAFE rendering')
    return raw.decode('ascii')


def checked_carrier_pin(row):
    need(same(row, CARRIER_PIN), 'complete-image byte identity differs')
    return row


def checked_binding_pin(row):
    need(same(row, BINDING_PIN), 'expected binding byte identity differs')
    return row


def checked_barrier(f):
    need(type(f) is dict and set(f) == {'expected_body_reads', 'actual_main',
         'actual_relation', 'safe_input', 'products_streams_receipts'}, 'barrier exact keys differ')
    need(type(f['expected_body_reads']) is int and f['expected_body_reads'] == 0,
         'before-expected barrier required')
    need(same(f['safe_input'], SAFE_PIN), 'original SAFE pin differs')
    return f


def decode(plan, source_document, span_document, query_raw, query_pin):
    _, value, _, _ = profile(source_document)
    actual_run_input = checked_run_input(source_document, query_raw)
    need(span_document['main_sha256'] == SOURCE_SHA
         and span_document['definition_sha256'] == DEFINITION_SHA,
         'compiled rule inventory belongs to another source/definition')
    startup = probe(plan, '03-startup', '04-print-startup')
    literal = probe(plan, '05-literal', '06-print-literal')
    returned = probe(plan, '07-publication', '08-print-publication')
    terminal = probe(plan, '09-terminal-no-rule', '10-print-terminal')
    rows = [{'configuration': startup['configuration']}]
    for item in (literal, returned):
        rows.append({**{k: v for k, v in item.items() if k != 'log'}, 'incoming_log': item['log']})
    decoded = capture.decode_capture(CASE, f'uiLiteral(uiU({value}))', rows,
        source_path=SOURCE, rule_spans=span_document['rules'], startup_probe=startup,
        terminal_probe=terminal, run_input=actual_run_input)
    decoded['actual_run_input_pin'] = query_pin
    need(decoded['comparison_ready'] and decoded['gaps'] == [],
         'actual main.k trace has gaps: ' + repr(decoded['gaps']))
    return decoded


def relate(source_document, decoded, complete, span_document):
    q, value, fuel, limit = profile(source_document)
    full_audit = images.audit(q, complete)
    need(decoded['comparison_ready'] is True and decoded['gaps'] == [], 'unqualified main trace')
    need(decoded['source_path'] == SOURCE, 'main source mismatch')
    run = decoded['observation']
    need(run['case_id'] == CASE and run['terminal_has_next'] is False
         and len(run['records']) == 2, 'complete adjacent two-edge main trace required')
    actual_rules = decoded['observed_rule_identities']
    need(len(actual_rules) == 2, 'both actual rewrite spans required')
    for actual, line in zip(actual_rules, (102, 80)):
        candidates = [r for r in span_document['rules'] if r['start_line'] == line]
        need(len(candidates) == 1, 'ambiguous compiled source span')
        need(actual['source'] == candidates[0]['source_path'], 'observed rule source differs')
        for key in ('unique_id', 'start_line', 'start_column', 'end_line', 'end_column'):
            need(actual[key] == candidates[0][key], 'observed rule identity mismatch: ' + key)
    startup_candidates = [r for r in span_document['rules'] if r['start_line'] == 74]
    need(len(startup_candidates) == 1, 'ambiguous compiled startup span')
    startup = decoded['startup_probe']['rule']
    need(startup['source'] == startup_candidates[0]['source_path'],
         'observed startup rule identity mismatch: source')
    for key in ('unique_id', 'start_line', 'start_column', 'end_line', 'end_column'):
        need(startup[key] == startup_candidates[0][key],
             'observed startup rule identity mismatch: ' + key)
    states = [run['initial'], *(edge['after'] for edge in run['records'])]
    need(same(run['records'][0]['before'], states[0])
         and same(run['records'][1]['before'], states[1])
         and same(run['terminal'], states[2]), 'main boundary continuity')
    machines = [complete['initial'], *complete['afters']]
    for index, (state, machine) in enumerate(zip(states, machines)):
        root, tables = machine['state']['root'], machine['state']['tables']
        need(set(state) == {'code_k', 'entry_k', 'control_k', 'environment_k',
             'frames_top_first_k', 'depth', 'stack_limit', 'remaining_fuel',
             'initial_fuel', 'output_k', 'charged_steps'}, 'complete snapshot keys differ')
        # Every machine cell is accounted for; physical tables unavailable in
        # main.k remain the explicitly retained complete-image carrier.
        for name, required in [('code_k', 'uiNoFuns()'), ('entry_k', f'uiLiteral(uiU({value}))'),
                               ('environment_k', '.Map'), ('frames_top_first_k', []),
                               ('depth', 0), ('stack_limit', limit), ('initial_fuel', fuel)]:
            need(same(state[name], required), 'main profile cell differs: ' + name)
        need(same(state['remaining_fuel'], le(root[9:25]))
             and same(state['initial_fuel'], le(root[25:41])) and same(state['stack_limit'], root[8]),
             'fuel/limit image relation differs')
        need(same(state['charged_steps'], fuel - le(root[9:25])), 'K-fuel accounting differs')
        control, outcome = state['control_k'], state['output_k']
        if index == 0:
            need(control == f'uiEval(uiLiteral(uiU({value})))' and outcome == 'uiPending()'
                 and root[5] == 1 and root[41:45] == [0] * 4
                 and tables[5][0] == [0] * 36, 'Eval image relation')
        elif index == 1:
            need(control == f'uiRet(uiU({value}))' and outcome == 'uiPending()'
                 and root[5] == 1 and root[41:45] == [0] * 4
                 and tables[5][0] == [1, 0, 0, 0] + [0] * 32,
                 'Return image relation')
        else:
            need(control == '.K' and outcome == f'uiDone(uiU({value}))'
                 and root[5] == 0 and root[41:45] == [1, 0, 0, 0]
                 and all(x == 0 for row in tables[5] for x in row), 'Done image relation')
        need(root[:5] == [0] * 5 and root[6:8] == [0, 0], 'empty current-map/frame/arena profile')
    need(same([r['charge'] for r in run['records']], [e['charge'] for e in complete['events']]),
         'observed K-fuel deltas versus image envelope charges')
    return {'format': 'sherlock-main-literal-complete-image-relation/1',
            'status': 'PASS_OBSERVED_MAIN_RULES_AND_RETAINED_COMPLETE_IMAGE_RELATION',
            'actual_main': decoded, 'complete_image_actual': complete,
            'complete_image_audit': full_audit, 'all_335_lanes_retained': True,
            'observed_rule_identities': actual_rules,
            'context_binds_counterpart_source': False,
            'scope': 'positive scalar literal/no functions/empty map+frames; K fuel only',
            'unpaired': ['complete physical tables/padding/context are retained carrier data, not main.k cells',
                        'main.k generatedCounter retained raw only',
                        'primary B work/clock, admission, authentication, refinement, native proof']}


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument('action', choices=['render', 'frame', 'decode', 'relate', 'compare'])
    cli.add_argument('--input', type=Path, required=True)
    cli.add_argument('--output', type=Path, required=True)
    cli.add_argument('--safe-input', type=Path)
    cli.add_argument('--plan', type=Path)
    cli.add_argument('--spans', type=Path)
    cli.add_argument('--complete-image', type=Path)
    cli.add_argument('--freeze', type=Path)
    cli.add_argument('--expected-binding', type=Path)
    a = cli.parse_args()
    if a.action in ('render', 'frame'):
        data = render(read(a.input)).encode() if a.action == 'render' else frame(a.input.read_bytes())
        with a.output.open('xb') as out:
            out.write(data)
    elif a.action == 'decode':
        save(a.output, decode(read(a.plan), read(a.safe_input), read(a.spans),
                              a.input.read_bytes(), images.pin(a.input)))
    elif a.action == 'relate':
        input_pins = {'safe_input': images.pin(a.safe_input), 'spans': images.pin(a.spans),
                      'actual_main': images.pin(a.input),
                      'complete_image': checked_carrier_pin(images.pin(a.complete_image))}
        result = relate(read(a.safe_input), read(a.input), read(a.complete_image), read(a.spans))
        result['input_pins'] = input_pins
        save(a.output, result)
    else:
        # Root freezes all new actuals/receipts first. Expected body is not read
        # until the exact actual-product membership and zero-read declaration pass.
        f = checked_barrier(read(a.freeze))
        need(f['actual_main'] == images.pin(a.input), 'actual main freeze mismatch')
        need(f['actual_main'] in f['products_streams_receipts'], 'actual main missing from durable freeze')
        images.pinned(f['safe_input'])
        for row in f['products_streams_receipts']:
            images.pinned(row)
        actual = read(a.input)
        need(actual['comparison_ready'] is True and actual['gaps'] == [], 'main trace incomplete')
        need(f['actual_relation'] in f['products_streams_receipts'], 'relation missing from durable freeze')
        relation = json.loads(images.pinned(f['actual_relation']), object_pairs_hook=wire.unique_object)
        need(relation['status'] == 'PASS_OBSERVED_MAIN_RULES_AND_RETAINED_COMPLETE_IMAGE_RELATION'
             and same(relation['actual_main'], actual), 'relation actual mismatch')
        checked_binding_pin(images.pin(a.expected_binding))
        binding = read(a.expected_binding)
        expected = json.loads(images.pinned(binding['expected_main_observation']), object_pairs_hook=wire.unique_object)
        need(set(expected) == {'format', 'safe_input', 'profile', 'observation'}
             and expected['format'] == 'sherlock-main-literal-independent-observation/1'
             and expected['profile'] == 'sherlock-k-a/1'
             and expected['safe_input'] == f['safe_input'], 'independent metadata differs')
        equal = same(actual['observation'], expected['observation'])
        save(a.output, {'status': 'PASS_WHOLE_MAIN_OBSERVATION' if equal else 'FAIL_WHOLE_MAIN_OBSERVATION',
                        'complete_typed_equal': equal, 'actual_main': f['actual_main'],
                        'actual_relation': f['actual_relation'], 'expected': binding['expected_main_observation'],
                        'durable_before_expected_freeze': images.pin(a.freeze),
                        'physical_B_clock_or_refinement_claim': False})
        if not equal:
            raise SystemExit(1)


if __name__ == '__main__':
    main()
