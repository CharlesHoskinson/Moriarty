"""Normal hand-authored K cell syntax examples, not executed K receipts."""
import hashlib
import importlib.util
from pathlib import Path
import unittest


MODULE = Path(__file__).with_name('k_observations.py')
SPEC = importlib.util.spec_from_file_location('k_observations', MODULE)
k = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(k)


def configuration(control, fuel, *, initial=1, output='uiPending()', env='.Map',
                  stack='uiNoFrames()', depth=0, code='uiNoFuns()'):
    return (f'<generatedTop>\n<k> {control} </k>\n<env> {env} </env>\n'
            f'<stack> {stack} </stack>\n<depth> {depth} </depth>\n'
            '<stackLimit> 8 </stackLimit>\n'
            f'<code> {code} </code>\n<fuel> {fuel} </fuel>\n'
            f'<initialFuel> {initial} </initialFuel>\n<out> {output} </out>\n'
            '</generatedTop>')


ENTRY = 'uiLiteral(uiU(7))'
INITIAL = configuration('uiEval ( uiLiteral ( uiU ( 7 ) ) )', 1)
RETURNED = configuration('uiRet ( uiU ( 7 ) )', 0)
PUBLISHED = configuration('.K', 0, output='uiDone ( uiU ( 7 ) )')


class KObservationTests(unittest.TestCase):
    def test_complete_post_startup_snapshot_and_raw_receipt(self):
        decoded = k.decode_configuration(INITIAL, entry_k=ENTRY)
        self.assertEqual(decoded['snapshot'], dict(
            code_k='uiNoFuns()', entry_k=ENTRY, control_k='uiEval(uiLiteral(uiU(7)))',
            environment_k='.Map', frames_top_first_k=[], depth=0, stack_limit=8,
            remaining_fuel=1, initial_fuel=1, output_k='uiPending()', charged_steps=0))
        self.assertEqual(decoded['raw_configuration'], INITIAL)
        self.assertEqual(decoded['raw_sha256'], hashlib.sha256(INITIAL.encode()).hexdigest())
        self.assertIn(' uiEval ( uiLiteral ( uiU ( 7 ) ) ) ', decoded['raw_cells']['k'])

    def test_complete_ordered_frames_saved_maps_and_nested_values(self):
        stack = ('uiFrame(uiRightF(uiAdd(), uiU(4)), '
                 'uiFrame(uiReturnF(uiUIntT(), ( 9 |-> uiU(3) 2 |-> uiU(5) )), '
                 'uiFrame(uiScopeF(.Map), uiNoFrames())))')
        decoded = k.decode_configuration(configuration('uiRet(uiU(7))', 0,
            env='1 |-> uiCons ( uiU ( 4 ), uiNil ( ) ) 2 |-> uiLeaf(uiU(9))',
            stack=stack, depth=3), entry_k=ENTRY)['snapshot']
        self.assertEqual(decoded['environment_k'],
                         '1 |-> uiCons(uiU(4), uiNil()) 2 |-> uiLeaf(uiU(9))')
        self.assertEqual(decoded['frames_top_first_k'], [
            'uiRightF(uiAdd(), uiU(4))',
            'uiReturnF(uiUIntT(), 2 |-> uiU(5) 9 |-> uiU(3))', 'uiScopeF(.Map)'])
        self.assertEqual(decoded['depth'], 3)

    def test_full_code_and_source_order_retained(self):
        code = ('uiFunction(uiFun(17, uiParam(1, uiUIntT(), uiNoParams()), '
                'uiUIntT(), uiVar(1)), uiFunction(uiFun(2, uiNoParams(), '
                'uiUIntT(), uiLiteral(uiU(9))), uiNoFuns()))')
        decoded = k.decode_configuration(configuration('uiEval(uiCall(17, '
            'uiExpr(uiLiteral(uiU(7)), uiNoExprs())))', 1, code=code),
            entry_k='uiCall(17, uiExpr(uiLiteral(uiU(7)), uiNoExprs()))')
        self.assertEqual(decoded['snapshot']['code_k'], code)

    def test_selected_arm_binding_and_pending_primitive_are_retained(self):
        control = ('uiCaseResult(uiMatched(1 |-> uiU(3)), '
                   'uiCons(uiU(3), uiNil()), uiVar(1), uiNoCases(), .Map)')
        decoded = k.decode_configuration(configuration(control, 2, initial=5), entry_k=ENTRY)
        self.assertEqual(decoded['snapshot']['control_k'], control)
        self.assertEqual(decoded['snapshot']['charged_steps'], 3)
        primitive = 'uiPrimResult(uiPrimValue(uiBranch(uiLeaf(uiU(4)), uiLeaf(uiU(9)))))'
        decoded = k.decode_configuration(configuration(primitive, 1, initial=5), entry_k=ENTRY)
        self.assertEqual(decoded['snapshot']['control_k'], primitive)

    def test_bare_cell_configuration_is_supported(self):
        raw = INITIAL.removeprefix('<generatedTop>').removesuffix('</generatedTop>')
        self.assertEqual(k.decode_configuration(raw, entry_k=ENTRY)['snapshot'],
                         k.decode_configuration(INITIAL, entry_k=ENTRY)['snapshot'])

    def test_literal_return_and_free_publication_have_distinct_records(self):
        result = k.decode_run('N01', ENTRY, [INITIAL, RETURNED, PUBLISHED],
                              rule_start_lines=[102, 80], terminal_has_next=False)
        run = result['observation']
        self.assertEqual(run['case_id'], 'N01')
        self.assertEqual([r['charge'] for r in run['records']], [1, 0])
        self.assertEqual([r['rule_start_line'] for r in run['records']], [102, 80])
        self.assertEqual(run['records'][0]['after'], run['records'][1]['before'])
        self.assertEqual(run['terminal']['output_k'], 'uiDone(uiU(7))')
        self.assertFalse(run['terminal_has_next'])
        self.assertEqual(result['raw_configurations'], [INITIAL, RETURNED, PUBLISHED])
        self.assertEqual(result['comparison_ready'], True)
        self.assertIn('not-established', result['execution_authenticity'])

    def test_zero_fuel_exhaustion_is_free_and_preserves_control_in_raw(self):
        raw = [configuration('uiEval(uiLiteral(uiU(7)))', 0, initial=0),
               configuration('.K', 0, initial=0, output='uiExhausted()')]
        run = k.decode_run('N01', ENTRY, raw, rule_start_lines=[84],
                           terminal_has_next=False)['observation']
        self.assertEqual(run['records'][0]['charge'], 0)
        self.assertEqual(run['terminal']['charged_steps'], 0)
        self.assertEqual(run['terminal']['output_k'], 'uiExhausted()')

    def test_absent_rule_and_termination_metadata_remain_explicit_gaps(self):
        result = k.decode_run('N01', ENTRY, [INITIAL, RETURNED, PUBLISHED])
        self.assertFalse(result['comparison_ready'])
        self.assertEqual([r['rule_start_line'] for r in result['observation']['records']],
                         [None, None])
        self.assertIsNone(result['observation']['terminal_has_next'])
        self.assertIn('rule_start_lines-not-supplied', result['gaps'])
        self.assertIn('terminal-has-next-not-supplied', result['gaps'])

    def test_terminal_configuration_only_is_decodeable_but_not_a_complete_run(self):
        result = k.decode_run('N01', ENTRY, [PUBLISHED])
        self.assertFalse(result['comparison_ready'])
        self.assertEqual(result['observation']['records'], [])
        self.assertIn('post-startup-initial-configuration-not-observed', result['gaps'])

    def test_sampled_fuel_jump_retains_actual_charge_and_clock_gap(self):
        start = configuration('uiEval(uiLiteral(uiU(7)))', 3, initial=3)
        end = configuration('.K', 0, initial=3, output='uiDone(uiU(7))')
        result = k.decode_run('sample', ENTRY, [start, end], terminal_has_next=False)
        self.assertEqual(result['observation']['records'][0]['charge'], 3)
        self.assertFalse(result['comparison_ready'])
        self.assertIn('edge-0-does-not-observe-one-semantic-transition', result['gaps'])


if __name__ == '__main__':
    unittest.main()
