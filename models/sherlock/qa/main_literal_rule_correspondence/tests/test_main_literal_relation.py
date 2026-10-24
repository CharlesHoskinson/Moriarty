"""Synthetic interface tests only; no real SAFE/host/K/expected bodies."""
import copy
import importlib.util
import json
from pathlib import Path
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).parents[1] / 'source'))
import main_literal_relation as subject


def fixture():
    pc = [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0]
    sc = [0, 0, 0, 0, 0, 1]
    p = {'root': [0, *pc], 'tables': [
        [[0] * w for _ in range(n)] for n, w in zip(pc, subject.wire.PROGRAM_WIDTHS)]}
    p['tables'][13][0][3] = 11
    s = {'root': [0] * 45, 'tables': [
        [[0] * w for _ in range(n)] for n, w in zip(sc, subject.wire.STATE_WIDTHS)]}
    s['root'][5] = 1
    s['root'][10] = s['root'][26] = 1  # synthetic fuel256, not the real case
    source = {'format': subject.wire.INPUT_FORMAT, 'context': [17, 29],
              'capacities': {'program': pc, 'state': sc, 'stack_limit': 0},
              'program': p, 'init': s, 'observation_budget': 2}
    return source


def relation_fixture(source):
    machine = {k: copy.deepcopy(source[k]) for k in ['context', 'capacities', 'program']}
    machine['state'] = copy.deepcopy(source['init'])
    middle = copy.deepcopy(machine)
    middle['state']['root'][9:25] = [255] + [0] * 15
    middle['state']['tables'][5][0] = [1, 0, 0, 0] + [0] * 32
    done = copy.deepcopy(middle)
    done['state']['root'][5] = 0
    done['state']['root'][41] = 1
    done['state']['tables'][5][0] = [0] * 36
    events = [{'source_line': line, 'rule_tag': tag, 'charge': charge,
               'before': before, 'after': after}
              for line, tag, charge, before, after in
              [(102, 3, 1, machine, middle), (80, 0, 0, middle, done)]]
    complete = {'format': subject.wire.OUTPUT_FORMAT, 'initial': machine,
                'afters': [middle, done], 'final': done, 'events': events,
                'statement': [*source['context'], *subject.images.flatten(source['program']),
                              *subject.images.flatten(machine['state']),
                              *subject.images.flatten(middle['state']),
                              *subject.images.flatten(done['state']), 3, 1, 0, 0],
                'logical_layout': subject.wire.logical_layout(machine),
                'context_authentication': 'NOT_ESTABLISHED',
                'source': 'complete K-print constructor tree; no expected-output fill'}
    fixed = {'code_k': 'uiNoFuns()', 'entry_k': 'uiLiteral(uiU(11))',
             'environment_k': '.Map', 'frames_top_first_k': [], 'depth': 0,
             'stack_limit': 0, 'initial_fuel': 256}
    states = [{**fixed, 'control_k': control, 'output_k': output,
               'remaining_fuel': fuel, 'charged_steps': charge}
              for control, output, fuel, charge in [
                  ('uiEval(uiLiteral(uiU(11)))', 'uiPending()', 256, 0),
                  ('uiRet(uiU(11))', 'uiPending()', 255, 1),
                  ('.K', 'uiDone(uiU(11))', 255, 1)]]
    rules = [{'source_path': subject.SOURCE, 'unique_id': str(line).zfill(64),
              'start_line': line, 'start_column': 8, 'end_line': line + 1,
              'end_column': 40} for line in (102, 80)]
    run = {'case_id': subject.CASE, 'initial': states[0],
           'records': [{'rule_start_line': line, 'charge': charge, 'before': before, 'after': after}
                       for line, charge, before, after in
                       [(102, 1, states[0], states[1]), (80, 0, states[1], states[2])]],
           'terminal': states[2], 'terminal_has_next': False}
    decoded = {'comparison_ready': True, 'gaps': [], 'source_path': subject.SOURCE,
               'observation': run, 'startup_probe': {'rule': {'source': subject.SOURCE, 'unique_id': '74'.zfill(64), 'start_line': 74, 'start_column': 8, 'end_line': 77, 'end_column': 41}}, 'observed_rule_identities':
               [{'source': row['source_path'], **{k: v for k, v in row.items() if k != 'source_path'}}
                for row in rules]}
    return decoded, complete, {'rules': [*rules, {'source_path': subject.SOURCE, 'unique_id': '74'.zfill(64), 'start_line': 74, 'start_column': 8, 'end_line': 77, 'end_column': 41}]}


def rejects(message, function):
    try:
        function()
    except ValueError as error:
        assert message in str(error), (message, str(error))
    else:
        raise AssertionError('did not reject: ' + message)


def revised_binding_tests(source, parsed, decoded, complete, spans):
    raw = subject.render(source).encode('ascii')
    assert subject.checked_run_input(source, raw) == raw.decode('ascii')
    rejects('actual query bytes differ', lambda: subject.checked_run_input(source, raw + b' '))
    for malformed, message in [
        (b'other{}()', 'parsed-root-is-not-the-compiled-uiRun-symbol'),
        (subject.RUN + b'(example{}())', 'Unexpected Kore carrier or arity'),
        (subject.RUN + b'(V:SortInt{},example{}(),example{}(),example{}())', 'parsed-contains-variable'),
        (subject.RUN + b'(\\top{SortInt{}}(),example{}(),example{}(),example{}())', 'parsed-matching-logic-connective-other-than-dv'),
        (parsed + b'x', 'parsed-not-one-closed-application'),
        (parsed + b'\x80', 'parsed-not-printable-ascii'),
        (parsed + b' ' * 65536, 'parsed-size-out-of-bounds'),
    ]:
        rejects(message, lambda term=malformed: subject.frame(term))
    assert subject.checked_carrier_pin(copy.deepcopy(subject.CARRIER_PIN)) == subject.CARRIER_PIN
    wrong = {**subject.CARRIER_PIN, 'sha256': '0' * 64}
    rejects('complete-image byte identity differs', lambda: subject.checked_carrier_pin(wrong))
    for key, replacement in [('start_line', 75), ('start_column', 9)]:
        bad = copy.deepcopy(decoded)
        bad['startup_probe']['rule'][key] = replacement
        rejects('observed startup rule identity mismatch', lambda: subject.relate(source, bad, complete, spans))
    for label, change in [
        ('observed rule identity mismatch', lambda d: d['observed_rule_identities'].reverse()),
        ('observed rule identity mismatch', lambda d: d['observed_rule_identities'][0].__setitem__('end_column', 99)),
    ]:
        bad = copy.deepcopy(decoded)
        change(bad)
        rejects(label, lambda: subject.relate(source, bad, complete, spans))
    f = {'expected_body_reads': 0, 'actual_main': {}, 'actual_relation': {},
         'safe_input': copy.deepcopy(subject.SAFE_PIN), 'products_streams_receipts': []}
    assert subject.checked_barrier(f) is f
    rejects('barrier exact keys differ', lambda: subject.checked_barrier({**f, 'extra': 0}))
    rejects('original SAFE pin differs', lambda: subject.checked_barrier({**f, 'safe_input': {**subject.SAFE_PIN, 'path': '/not-original'}}))
    rejects('before-expected barrier required', lambda: subject.checked_barrier({**f, 'expected_body_reads': False}))
    assert subject.checked_binding_pin(copy.deepcopy(subject.BINDING_PIN)) == subject.BINDING_PIN
    rejects('expected binding byte identity differs', lambda: subject.checked_binding_pin({**subject.BINDING_PIN, 'bytes': 1205}))
    with tempfile.TemporaryDirectory() as folder:
        root = Path(folder)
        receipt_dir = root / 'unit'
        receipt_dir.mkdir()
        phase = {'id': 'unit', 'argv': ['/synthetic-tool', 'input']}
        plan = {'root_receipts': str(root), 'phases': [phase]}
        good = {'id': 'unit', 'returncode': 0, 'outcome': 'exited', 'timed_out': False,
                'containment_breach': False, 'remaining': []}
        command = {'argv': phase['argv']}
        def write(result, cmd):
            (receipt_dir / 'LAUNCH-RESULT.json').write_text(json.dumps(result))
            (receipt_dir / 'COMMAND.json').write_text(json.dumps(cmd))
        write(good, command)
        assert subject.successful(plan, 'unit')[1] == good
        for key, value in [('returncode', 1), ('timed_out', True), ('containment_breach', True), ('remaining', [1])]:
            write({**good, key: value}, command)
            rejects('child did not finish successfully', lambda: subject.successful(plan, 'unit'))
        write(good, {'argv': ['/different-tool']})
        rejects('actual argv differs', lambda: subject.successful(plan, 'unit'))


def main():
    source = fixture()
    assert subject.render(source) == 'uiRun(uiNoFuns(),uiLiteral(uiU(11)),256,0)\n'
    parsed = subject.RUN + b'(example{}(),example{}(),example{}(),example{}())'
    framed = subject.frame(parsed + b'\n')
    assert parsed in framed and framed.endswith(b')))\n')
    for name, change in [
        ('bool-lane', lambda x: x['program']['tables'][13][0].__setitem__(3, True)),
        ('wrong-entry', lambda x: x['program']['tables'][4][0].__setitem__(0, 1)),
        ('nonempty-control-tail', lambda x: x['init']['tables'][5][0].__setitem__(9, 1)),
        ('zero-fuel', lambda x: (x['init']['root'].__setitem__(10, 0), x['init']['root'].__setitem__(26, 0))),
        ('nonempty-functions', lambda x: x['program']['root'].__setitem__(2, 1)),
    ]:
        bad = copy.deepcopy(source)
        change(bad)
        messages = {'bool-lane': 'expected nonnegative integer',
                    'wrong-entry': 'entry must be exact Literal',
                    'nonempty-control-tail': 'all allocated Init arenas',
                    'zero-fuel': 'positive equal initial/remaining fuel',
                    'nonempty-functions': 'selected complete Program counts/entry differ'}
        rejects(messages[name], lambda: subject.profile(bad))
    for term in [b'', b'example{}() trailing', b'example{}(', b'example{}(),other{}()']:
        message = 'parsed-size-out-of-bounds' if not term else 'parsed-root-is-not-the-compiled-uiRun-symbol'
        rejects(message, lambda: subject.frame(term))
    a = {'state': [1, False], 'other': {'a': 1, 'b': 2}}
    assert subject.same(a, {'other': {'b': 2, 'a': 1}, 'state': [1, False]})
    assert not subject.same(a, {'state': [1, 0], 'other': {'a': 1, 'b': 2}})
    assert not subject.same(a, {'state': [False, 1], 'other': {'a': 1, 'b': 2}})
    decoded, complete, spans = relation_fixture(source)
    result = subject.relate(source, decoded, complete, spans)
    assert result['complete_image_actual'] is complete and result['actual_main'] is decoded
    for label, change in [
        ('observed-rule', lambda d, c: d['observed_rule_identities'][0].__setitem__('unique_id', '0' * 64)),
        ('literal-result', lambda d, c: d['observation']['records'][0]['after'].__setitem__('control_k', 'uiRet(uiU(12))')),
        ('lost-saved-cell', lambda d, c: d['observation']['initial'].pop('environment_k')),
        ('bool-fuel', lambda d, c: d['observation']['terminal'].__setitem__('remaining_fuel', False)),
        ('extra-state-field', lambda d, c: d['observation']['initial'].__setitem__('extra', 0)),
        ('unpaired-charge', lambda d, c: d['observation']['records'][1].__setitem__('charge', 1)),
        ('lost-program-padding', lambda d, c: c['afters'][0]['program']['tables'][10][0].__setitem__(2, 9)),
        ('statement-lane', lambda d, c: c['statement'].__setitem__(70, 9)),
    ]:
        d, c = copy.deepcopy(decoded), copy.deepcopy(complete)
        change(d, c)
        messages = {'observed-rule': 'observed rule identity mismatch',
                    'literal-result': 'Return image relation',
                    'lost-saved-cell': 'complete snapshot keys differ',
                    'bool-fuel': 'fuel/limit image relation differs',
                    'extra-state-field': 'complete snapshot keys differ',
                    'unpaired-charge': 'observed K-fuel deltas',
                    'lost-program-padding': 'Complete supplied program changed',
                    'statement-lane': 'K-emitted 335-lane statement differs'}
        rejects(messages[label], lambda: subject.relate(source, d, c, copy.deepcopy(spans)))
    revised_binding_tests(source, parsed, decoded, complete, spans)
    print(json.dumps({'status': 'PASS_SYNTHETIC_RENDER_PROFILE_FRAME_TYPED_EQUALITY',
                      'real_inputs_or_expected_opened': 0, 'K_executed': False}))


if __name__ == '__main__':
    main()
