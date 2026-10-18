"""Project supplied kProfileMachine ITF; never execute or emulate its evaluator.

Rendering expands immutable ExprIds, not named function calls. Raw ITF stays in
the envelope because K text forgets ExprId sharing and unordered-map encoding.
All identities and provenance supplied by a producer remain declarations.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path

PROFILE = 'sherlock-k-a/1'
PRODUCER = 'sherlock-quint-k-profile'
FORMAT = 'sherlock-quint-itf-observations/1'


def canonical_bytes(value):
    """Deterministic JSON bytes for artifact identity, not a machine codec."""
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()


def decode_itf(value):
    """Decode the installed Quint 0.32 ITF integer/map/tuple representation."""
    if type(value) is list:
        return [decode_itf(item) for item in value]
    if type(value) is not dict:
        return value
    if '#bigint' in value:
        return int(value['#bigint'])
    if '#map' in value:
        return {decode_itf(key): decode_itf(item) for key, item in value['#map']}
    if '#tup' in value:
        return tuple(decode_itf(item) for item in value['#tup'])
    if any(key.startswith('#') for key in value):
        raise ValueError('unsupported ITF value encoding')
    return {key: decode_itf(item) for key, item in value.items()}


def _tag(value):
    if type(value) is not dict or set(value) != {'tag', 'value'}:
        raise ValueError('expected Quint variant {tag,value}')
    return value['tag'], value['value']


def _call(name, *arguments):
    return name + '(' + ', '.join(str(item) for item in arguments) + ')'


def _sequence(items, constructor, nil, render):
    result = nil + '()'
    for item in reversed(items):
        result = _call(constructor, render(item), result)
    return result


class _Renderer:
    """Diagnostic structural rendering only; contains no transition dispatch."""
    def __init__(self, program):
        self.program = program
        self.active_expressions = set()

    def tree(self, tokens):
        stack = []
        for token in reversed(tokens):
            tag, value = _tag(token)
            if tag == 'TreeLeaf':
                stack.append(_call('uiLeaf', _call('uiU', value)))
            elif tag == 'TreeBranch' and len(stack) >= 2:
                stack.append(_call('uiBranch', stack.pop(), stack.pop()))
            else:
                raise ValueError('tree outside complete prefix-tree projection')
        if len(stack) != 1:
            raise ValueError('tree outside complete prefix-tree projection')
        return stack[0]

    def value(self, item):
        tag, value = _tag(item)
        if tag == 'UInt':
            return _call('uiU', value)
        if tag == 'BoolValue' and type(value) is bool:
            return _call('uiB', str(value).lower())
        if tag == 'UIntList':
            return _sequence(value, 'uiCons', 'uiNil', lambda n: _call('uiU', n))
        if tag == 'UIntTree':
            return self.tree(value)
        raise ValueError('unsupported value variant: ' + tag)

    def environment(self, environment):
        # Match reference_display.rs: ascending canonical name *String* order.
        return ' '.join(f'{name} |-> {self.value(environment[name])}'
                        for name in sorted(environment, key=str)) or '.Map'

    def operator(self, item):
        tag, _ = _tag(item)
        names = dict(UIntType='uiUIntT', BoolType='uiBoolT', ListType='uiListT',
                     TreeType='uiTreeT', Add='uiAdd', Sub='uiSub', Mul='uiMul',
                     Eq='uiEq', Lt='uiLt', ConsOp='uiConsOp', BranchOp='uiBranchOp',
                     LeafOp='uiLeafOp', NotOp='uiNotOp')
        if tag not in names:
            raise ValueError('unsupported operator/type: ' + tag)
        return _call(names[tag])

    def pattern(self, item):
        tag, value = _tag(item)
        if tag == 'PNil':
            return 'uiPNil()'
        if tag in ('PVar', 'PLeaf'):
            return _call('ui' + tag, value)
        if tag == 'PCons':
            return _call('uiPCons', value['head'], value['tail'])
        if tag == 'PBranch':
            return _call('uiPBranch', value['left'], value['right'])
        raise ValueError('unsupported pattern: ' + tag)

    def arms(self, items):
        return _sequence(items, 'uiCase', 'uiNoCases', lambda arm:
                         self.pattern(arm['pattern']) + ', ' + self.expression(arm['body']))

    def expressions(self, items):
        return _sequence(items, 'uiExpr', 'uiNoExprs', self.expression)

    def values(self, items):
        return _sequence(items, 'uiValue', 'uiNoValues', self.value)

    def expression(self, identity):
        if identity in self.active_expressions:
            raise ValueError('cyclic expression table has no finite K expression')
        self.active_expressions.add(identity)
        try:
            tag, value = _tag(self.program['expressions'][identity])
            if tag == 'Literal':
                return _call('uiLiteral', self.value(value))
            if tag == 'Variable':
                return _call('uiVar', value)
            if tag == 'Let':
                return _call('uiLet', value['name'], self.expression(value['value']), self.expression(value['body']))
            if tag == 'Binary':
                return _call('uiBinary', self.operator(value['op']), self.expression(value['left']), self.expression(value['right']))
            if tag == 'Unary':
                return _call('uiUnary', self.operator(value['op']), self.expression(value['argument']))
            if tag == 'If':
                return _call('uiIf', *(self.expression(value[key]) for key in ('condition', 'yes', 'no')))
            if tag == 'Call':
                return _call('uiCall', value['name'], self.expressions(value['arguments']))
            if tag == 'Match':
                return _call('uiMatch', self.expression(value['value']), self.arms(value['arms']))
            raise ValueError('unsupported expression: ' + tag)
        finally:
            self.active_expressions.remove(identity)

    def function(self, function):
        parameters = _sequence(function['parameters'], 'uiParam', 'uiNoParams',
                               lambda p: str(p['name']) + ', ' + self.operator(p['ty']))
        return _call('uiFun', function['name'], parameters, self.operator(function['returnType']),
                     self.expression(function['body']))

    def frame(self, item):
        tag, value = _tag(item)
        if tag == 'ScopeFrame':
            return _call('uiScopeF', self.environment(value))
        if tag == 'UnaryFrame':
            return _call('uiUnaryF', self.operator(value))
        if tag == 'LetFrame':
            return _call('uiLetF', value['name'], self.expression(value['body']), self.environment(value['environment']))
        if tag == 'LeftFrame':
            return _call('uiLeftF', self.operator(value['op']), self.expression(value['right']), self.environment(value['environment']))
        if tag == 'RightFrame':
            return _call('uiRightF', self.operator(value['op']), self.value(value['left']))
        if tag == 'IfFrame':
            return _call('uiIfF', self.expression(value['yes']), self.expression(value['no']), self.environment(value['environment']))
        if tag == 'MatchFrame':
            return _call('uiMatchF', self.arms(value['arms']), self.environment(value['environment']))
        if tag == 'ArgsFrame':
            return _call('uiArgsF', value['name'], self.values(value['values']), self.expressions(value['remaining']), self.environment(value['environment']))
        if tag == 'ReturnFrame':
            return _call('uiReturnF', self.operator(value['ty']), self.environment(value['environment']))
        raise ValueError('unsupported frame: ' + tag)

    def helper(self, item):
        tag, value = _tag(item)
        if tag in ('MissingFunction', 'BadBinding', 'NoMatch'):
            return {'MissingFunction': 'uiMissingFun()', 'BadBinding': 'uiBadBinding()', 'NoMatch': 'uiNoMatch()'}[tag]
        if tag == 'FoundFunction':
            return self.function(value)
        if tag in ('Bindings', 'Matched'):
            return _call('uiBindings' if tag == 'Bindings' else 'uiMatched', self.environment(value))
        if tag == 'PrimitiveValue':
            return _call('uiPrimValue', self.value(value))
        if tag == 'PrimitiveError':
            return _call('uiPrimError', value)
        raise ValueError('unsupported helper result: ' + tag)

    def control(self, item):
        tag, value = _tag(item)
        if tag == 'EmptyControl':
            return '.K'
        if tag == 'Eval':
            return _call('uiEval', self.expression(value))
        if tag == 'Return':
            return _call('uiRet', self.value(value))
        if tag == 'Error':
            return _call('uiError', value)
        if tag == 'PrimitiveResult':
            return _call('uiPrimResult', self.helper(value))
        if tag == 'Push':
            return _call('uiPush', self.frame(value['frame']), _call('uiEval', self.expression(value['expression'])), self.environment(value['environment']))
        if tag == 'Invoke':
            return _call('uiInvoke', value['name'], self.values(value['values']))
        if tag == 'Enter':
            return _call('uiEnter', self.helper(value['found']), self.values(value['values']))
        if tag == 'Bound':
            return _call('uiBound', self.helper(value['bindings']), self.operator(value['ty']), self.expression(value['expression']))
        if tag == 'Cases':
            return _call('uiCases', self.value(value['value']), self.arms(value['arms']), self.environment(value['environment']))
        if tag == 'CaseResult':
            return _call('uiCaseResult', self.helper(value['matching']), self.value(value['value']), self.expression(value['expression']), self.arms(value['remaining']), self.environment(value['environment']))
        raise ValueError('unsupported control: ' + tag)

    def outcome(self, item):
        tag, value = _tag(item)
        if tag in ('Pending', 'Exhausted'):
            return _call('ui' + tag)
        if tag == 'Done':
            return _call('uiDone', self.value(value))
        if tag == 'Rejected':
            return _call('uiRejected', value)
        raise ValueError('unsupported outcome: ' + tag)


def project_snapshot(state):
    """Render every shared post-startup state field from one decoded Snapshot."""
    renderer = _Renderer(state['program'])
    return dict(code_k=_sequence(state['program']['functions'], 'uiFunction', 'uiNoFuns', renderer.function),
                entry_k=renderer.expression(state['program']['entry']),
                control_k=renderer.control(state['control']),
                environment_k=renderer.environment(state['environment']),
                frames_top_first_k=[renderer.frame(frame) for frame in state['frames']],
                depth=len(state['frames']), stack_limit=state['stackLimit'],
                remaining_fuel=state['remaining'], initial_fuel=state['initial'],
                output_k=renderer.outcome(state['outcome']), charged_steps=state['initial'] - state['remaining'])


def _variable(state, name):
    keys = [key for key in state if key == name or key.endswith('::' + name)]
    if len(keys) != 1:
        raise ValueError(f'expected exactly one ITF variable {name}, found {keys}')
    return decode_itf(state[keys[0]])


def _adapt(case_id, trace):
    states = trace['states']
    if not states:
        raise ValueError('missing ITF states')
    machine = _variable(states[0], 'machine')
    if (_variable(states[0], 'lastRule'), _variable(states[0], 'lastCharge')) != (74, 0):
        raise ValueError('initial state lacks post-startup 74/0 metadata')
    if _tag(machine['outcome'])[0] != 'Pending':
        raise ValueError('initial state is not pending')
    initial = previous = project_snapshot(machine)
    records, stutters = [], 0
    for raw in states[1:]:
        current_machine = _variable(raw, 'machine')
        rule, charge = _variable(raw, 'lastRule'), _variable(raw, 'lastCharge')
        if _tag(machine['outcome'])[0] != 'Pending':
            if current_machine != machine or (rule, charge) != (0, 0):
                raise ValueError('post-publication state is not a terminal model stutter')
            stutters += 1
            continue
        current = project_snapshot(current_machine)
        if rule == 0 or charge not in (0, 1):
            raise ValueError('missing semantic rule or unsupported charge')
        if current['charged_steps'] - previous['charged_steps'] != charge:
            raise ValueError('raw charge disagrees with raw fuel difference')
        records.append(dict(rule_start_line=rule, charge=charge, before=previous, after=current))
        machine, previous = current_machine, current
    if _tag(machine['outcome'])[0] == 'Pending':
        raise ValueError('incomplete ITF trace: actual terminal publication missing')
    if not records or records[-1]['charge'] != 0 or records[-1]['rule_start_line'] not in (80, 82, 84):
        raise ValueError('actual free terminal publication missing')
    if previous['control_k'] != '.K':
        raise ValueError('published outcome lacks empty control')
    return dict(case_id=case_id, initial=initial, records=records, terminal=previous,
                terminal_has_next=False), stutters


def adapt_itf_trace(case_id, trace):
    """Project supplied edges; terminal flag refers to K-A semantic edges.

    This does not call Quint.step to test enabledness: that action is total and
    emits rule-0 stutters after publication in this model.
    """
    return _adapt(case_id, trace)[0]


def observation_bundle(traces, *, source_sha256):
    """Retain raw ITF separately from the common comparison observation shape."""
    programs, runs, raw_traces, references, stutters = [], [], [], [], []
    identities = {}
    for case_id, trace in traces:
        run, count = _adapt(case_id, trace)
        identity = dict(case_id=case_id, **{key: run['initial'][key] for key in ('code_k', 'entry_k')})
        if case_id not in identities:
            identities[case_id] = identity
            programs.append(identity)
        elif identities[case_id] != identity:
            raise ValueError('program identity changed between runs for ' + case_id)
        runs.append(run)
        raw_traces.append(dict(case_id=case_id, itf=copy.deepcopy(trace)))
        references.append(dict(case_id=case_id, canonical_itf_sha256=hashlib.sha256(canonical_bytes(trace)).hexdigest()))
        stutters.append(count)
    return dict(format=FORMAT,
        observations=dict(producer=PRODUCER, profile=PROFILE, source_sha256=source_sha256,
                          boundary='post-startup', programs=programs, runs=runs),
        raw_traces=raw_traces, raw_trace_references=references,
        projection=dict(model_profile=PROFILE, boundary='post-startup', startup='metadata-only',
            terminal_has_next='no further K-A semantic edge; not an enabledness probe',
            terminal_stutters=stutters,
            gaps=['K text expands ExprId sharing; raw program tables retained',
                  'map order normalized to reference canonical-name String order; raw maps retained',
                  'depth and charged_steps derived from raw frame length and initial-minus-remaining',
                  'model step enabled after publication; rule-0 stutters retained only in raw ITF',
                  'A helper work unmetered, not measured as zero',
                  'no B, Lean/Clean, circuit, theorem, native or target evidence']),
        evidence=dict(execution_receipt='not-supplied', producer_authenticity='not-established',
                      adapter_execution='supplied-artifact-only'))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('manifest', type=Path, help='ordered {source_sha256,runs:[{case_id,itf}]} JSON')
    args = parser.parse_args(argv)
    manifest = json.loads(args.manifest.read_text())
    traces, references = [], []
    for row in manifest['runs']:
        path = args.manifest.parent / row['itf']
        raw = path.read_bytes()
        traces.append((row['case_id'], json.loads(raw)))
        references.append(dict(path=str(path), file_sha256=hashlib.sha256(raw).hexdigest()))
    bundle = observation_bundle(traces, source_sha256=manifest['source_sha256'])
    for reference, file_reference in zip(bundle['raw_trace_references'], references, strict=True):
        reference.update(file_reference)
    print(canonical_bytes(bundle).decode())


if __name__ == '__main__':
    main()
