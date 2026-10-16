"""Normal serializer fixtures only: these are not executed Quint observations."""
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import quint_observations as quint


def variant(tag, value=()):
    return dict(tag=tag, value=value)


def itf(value):
    if type(value) is int:
        return {'#bigint': str(value)}
    if type(value) is tuple:
        return {'#tup': [itf(item) for item in value]}
    if type(value) is list:
        return [itf(item) for item in value]
    if type(value) is dict:
        if any(type(key) is int for key in value) or not value:
            return {'#map': [[itf(key), itf(item)] for key, item in value.items()]}
        return {key: itf(item) for key, item in value.items()}
    return value


def snapshot(control, remaining, outcome=None, fuel=1):
    return dict(program=dict(expressions={0: variant('Literal', variant('UInt', 7))},
                             functions=[], entry=0), control=control,
                environment={}, frames=[], stackLimit=8, initial=fuel,
                remaining=remaining, outcome=outcome or variant('Pending'))


def trace(fuel=1):
    states = [(snapshot(variant('Eval', 0), fuel, fuel=fuel), 74, 0)]
    if fuel:
        states.append((snapshot(variant('Return', variant('UInt', 7)), fuel - 1, fuel=fuel), 102, 1))
    states.append((snapshot(variant('EmptyControl'), max(0, fuel - 1),
                            variant('Done', variant('UInt', 7)) if fuel else variant('Exhausted'), fuel),
                   80 if fuel else 84, 0))
    return {'vars': ['sample::machine', 'sample::lastRule', 'sample::lastCharge'],
            'states': [dict({'#meta': {'index': index}}, **{
                'sample::machine': itf(state), 'sample::lastRule': itf(rule),
                'sample::lastCharge': itf(charge)})
                for index, (state, rule, charge) in enumerate(states)]}


class QuintObservationTests(unittest.TestCase):
    def test_n01_exact_includes_free_publication_and_full_snapshots(self):
        run = quint.adapt_itf_trace('N01', trace())
        self.assertEqual([record['rule_start_line'] for record in run['records']], [102, 80])
        self.assertEqual([record['charge'] for record in run['records']], [1, 0])
        self.assertEqual(run['initial'], dict(code_k='uiNoFuns()', entry_k='uiLiteral(uiU(7))',
            control_k='uiEval(uiLiteral(uiU(7)))', environment_k='.Map', frames_top_first_k=[],
            depth=0, stack_limit=8, remaining_fuel=1, initial_fuel=1,
            output_k='uiPending()', charged_steps=0))
        self.assertEqual(run['records'][0]['before'], run['initial'])
        self.assertEqual(run['records'][0]['after'], run['records'][1]['before'])
        self.assertEqual(run['terminal']['control_k'], '.K')
        self.assertEqual(run['terminal']['output_k'], 'uiDone(uiU(7))')
        self.assertFalse(run['terminal_has_next'])

    def test_zero_exact_surplus_fuel_keep_separate_work(self):
        for fuel, work, output in [(0, 0, 'uiExhausted()'), (1, 1, 'uiDone(uiU(7))'), (2, 1, 'uiDone(uiU(7))')]:
            run = quint.adapt_itf_trace('N01', trace(fuel))
            self.assertEqual(run['terminal']['charged_steps'], work)
            self.assertEqual(run['terminal']['remaining_fuel'], fuel - work)
            self.assertEqual(run['terminal']['output_k'], output)

    def test_every_frame_keeps_saved_environment_and_order(self):
        state = snapshot(variant('Eval', 0), 1)
        env = {2: variant('UInt', 9), -1: variant('BoolValue', True)}
        state['environment'] = env
        state['frames'] = [variant('LetFrame', dict(name=3, body=0, environment=env)),
            variant('LeftFrame', dict(op=variant('Add'), right=0, environment=env)),
            variant('RightFrame', dict(op=variant('Mul'), left=variant('UInt', 2))),
            variant('UnaryFrame', variant('NotOp')),
            variant('IfFrame', dict(yes=0, no=0, environment=env)),
            variant('MatchFrame', dict(arms=[], environment=env)),
            variant('ArgsFrame', dict(name=4, values=[variant('UInt', 5)], remaining=[0], environment=env)),
            variant('ReturnFrame', dict(ty=variant('UIntType'), environment=env)),
            variant('ScopeFrame', env)]
        actual = quint.project_snapshot(state)
        saved = '-1 |-> uiB(true) 2 |-> uiU(9)'
        self.assertEqual(actual['environment_k'], saved)
        self.assertEqual(actual['depth'], 9)
        self.assertEqual(actual['frames_top_first_k'], [
            f'uiLetF(3, uiLiteral(uiU(7)), {saved})',
            f'uiLeftF(uiAdd(), uiLiteral(uiU(7)), {saved})', 'uiRightF(uiMul(), uiU(2))',
            'uiUnaryF(uiNotOp())', f'uiIfF(uiLiteral(uiU(7)), uiLiteral(uiU(7)), {saved})',
            f'uiMatchF(uiNoCases(), {saved})',
            f'uiArgsF(4, uiValue(uiU(5), uiNoValues()), uiExpr(uiLiteral(uiU(7)), uiNoExprs()), {saved})',
            f'uiReturnF(uiUIntT(), {saved})', f'uiScopeF({saved})'])

    def test_function_parameter_and_argument_order_are_preserved(self):
        state = snapshot(variant('Eval', 2), 3, fuel=3)
        state['program']['expressions'][2] = variant('Call', dict(name=5, arguments=[0, 1]))
        state['program']['expressions'][1] = variant('Variable', 8)
        function = dict(name=5, parameters=[dict(name=8, ty=variant('UIntType')),
            dict(name=7, ty=variant('BoolType'))], returnType=variant('UIntType'), body=0)
        state['program']['functions'] = [function, dict(function, name=4)]
        actual = quint.project_snapshot(state)
        self.assertIn('uiParam(8, uiUIntT(), uiParam(7, uiBoolT(), uiNoParams()))', actual['code_k'])
        self.assertLess(actual['code_k'].index('uiFun(5,'), actual['code_k'].index('uiFun(4,'))
        self.assertEqual(actual['control_k'], 'uiEval(uiCall(5, uiExpr(uiLiteral(uiU(7)), uiExpr(uiVar(8), uiNoExprs()))))')

    def test_tree_and_list_values_are_structural_and_ordered(self):
        state = snapshot(variant('Return', variant('UIntTree', [variant('TreeBranch'),
            variant('TreeLeaf', 2), variant('TreeBranch'), variant('TreeLeaf', 3), variant('TreeLeaf', 4)])), 1)
        self.assertEqual(quint.project_snapshot(state)['control_k'], 'uiRet(uiBranch(uiLeaf(uiU(2)), uiBranch(uiLeaf(uiU(3)), uiLeaf(uiU(4)))))')
        state['control'] = variant('Return', variant('UIntList', [3, 1, 2]))
        self.assertEqual(quint.project_snapshot(state)['control_k'], 'uiRet(uiCons(uiU(3), uiCons(uiU(1), uiCons(uiU(2), uiNil()))))')

    def test_raw_itf_and_terminal_stutters_remain_in_envelope(self):
        raw = trace()
        stutter = copy.deepcopy(raw['states'][-1])
        stutter['sample::lastRule'] = itf(0)
        raw['states'].append(stutter)
        result = quint.observation_bundle([('N01', raw)], source_sha256='a' * 64)
        self.assertEqual(result['raw_traces'], [dict(case_id='N01', itf=raw)])
        self.assertEqual(result['projection']['terminal_stutters'], [1])
        self.assertEqual(len(result['observations']['runs'][0]['records']), 2)
        self.assertEqual(result['observations']['producer'], 'sherlock-quint-k-profile')
        self.assertEqual(result['observations']['boundary'], 'post-startup')

    def test_all_expression_forms_and_ordered_patterns_render_as_syntax(self):
        state = snapshot(variant('Eval', 6), 1)
        state['program']['expressions'].update({
            1: variant('Variable', 3),
            2: variant('Unary', dict(op=variant('LeafOp'), argument=0)),
            3: variant('Binary', dict(op=variant('ConsOp'), left=0, right=1)),
            4: variant('If', dict(condition=1, yes=2, no=3)),
            5: variant('Match', dict(value=4, arms=[
                dict(pattern=variant('PCons', dict(head=1, tail=2)), body=0),
                dict(pattern=variant('PBranch', dict(left=3, right=4)), body=1),
                dict(pattern=variant('PNil'), body=0),
                dict(pattern=variant('PLeaf', 5), body=0),
                dict(pattern=variant('PVar', 6), body=0)])),
            6: variant('Let', dict(name=9, value=2, body=5))})
        actual = quint.project_snapshot(state)['control_k']
        self.assertTrue(actual.startswith('uiEval(uiLet(9, uiUnary(uiLeafOp(), uiLiteral(uiU(7))), uiMatch(uiIf('))
        for term in ('uiBinary(uiConsOp(), uiLiteral(uiU(7)), uiVar(3))',
                     'uiPCons(1, 2)', 'uiPBranch(3, 4)', 'uiPNil()', 'uiPLeaf(5)', 'uiPVar(6)'):
            self.assertIn(term, actual)
        self.assertLess(actual.index('uiPCons'), actual.index('uiPBranch'))

    def test_helper_controls_preserve_all_payload_fields(self):
        state = snapshot(variant('Eval', 0), 1)
        env = {10: variant('UInt', 1), 2: variant('UInt', 2)}
        saved = '10 |-> uiU(1) 2 |-> uiU(2)'
        function = dict(name=8, parameters=[], returnType=variant('TreeType'), body=0)
        controls = [
            (variant('Push', dict(frame=variant('ScopeFrame', env), expression=0, environment=env)),
             f'uiPush(uiScopeF({saved}), uiEval(uiLiteral(uiU(7))), {saved})'),
            (variant('Invoke', dict(name=8, values=[variant('UInt', 7)])),
             'uiInvoke(8, uiValue(uiU(7), uiNoValues()))'),
            (variant('Enter', dict(found=variant('FoundFunction', function), values=[])),
             'uiEnter(uiFun(8, uiNoParams(), uiTreeT(), uiLiteral(uiU(7))), uiNoValues())'),
            (variant('Bound', dict(bindings=variant('Bindings', env), ty=variant('ListType'), expression=0)),
             f'uiBound(uiBindings({saved}), uiListT(), uiLiteral(uiU(7)))'),
            (variant('Cases', dict(value=variant('UInt', 7), arms=[], environment=env)),
             f'uiCases(uiU(7), uiNoCases(), {saved})'),
            (variant('CaseResult', dict(matching=variant('Matched', env), value=variant('UInt', 7),
                 expression=0, remaining=[], environment=env)),
             f'uiCaseResult(uiMatched({saved}), uiU(7), uiLiteral(uiU(7)), uiNoCases(), {saved})'),
            (variant('PrimitiveResult', variant('PrimitiveValue', variant('BoolValue', False))),
             'uiPrimResult(uiPrimValue(uiB(false)))')]
        for control, expected in controls:
            state['control'] = control
            self.assertEqual(quint.project_snapshot(state)['control_k'], expected)

    def test_incomplete_supplied_normal_capture_never_invents_publication(self):
        raw = trace()
        raw['states'].pop()
        with self.assertRaisesRegex(ValueError, 'actual terminal publication missing'):
            quint.adapt_itf_trace('N01', raw)

    def test_cli_retains_exact_raw_file_digest_without_model_execution(self):
        raw = json.dumps(trace(), indent=2).encode()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'raw.itf.json').write_bytes(raw)
            (root / 'manifest.json').write_text(json.dumps(dict(source_sha256='a' * 64,
                runs=[dict(case_id='N01', itf='raw.itf.json')])))
            result = subprocess.run([sys.executable, '-I', '-B', str(Path(quint.__file__).resolve()),
                str(root / 'manifest.json')], capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        bundle = json.loads(result.stdout)
        self.assertEqual(bundle['raw_trace_references'][0]['file_sha256'], hashlib.sha256(raw).hexdigest())
        self.assertEqual(bundle['evidence']['execution_receipt'], 'not-supplied')
        self.assertEqual(bundle['evidence']['producer_authenticity'], 'not-established')

    def test_source_driver_selects_all_216_budgets_without_expected_states(self):
        root = Path(__file__).resolve().parents[1] / 'quint' / 'observations'
        source = (root / 'normalObservationTest.qnt').read_text()
        plan = json.loads((root / 'normal-observation-manifest.json').read_text())
        self.assertEqual(len(plan['runs']), 216)
        self.assertEqual([row['case_id'] for row in plan['runs'][:3]], ['N01'] * 3)
        self.assertEqual(plan['runs'][-1]['case_id'], 'N08')
        self.assertEqual(plan['runs'][-1]['initial_fuel'], 83)
        for row in plan['runs']:
            declaration = f"run {row['test_name']} = initProgram({row['case_id'].lower()}Program, {row['initial_fuel']}, 8)"
            self.assertIn(declaration, source)
            self.assertIn(f".then({row['initial_fuel'] + 1}.reps(i => step))", source)
        self.assertNotIn('.expect(', source)
        self.assertNotIn('Published', source)
        self.assertNotIn('State0', source)


if __name__ == '__main__':
    unittest.main()
