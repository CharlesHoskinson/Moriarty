"""Positive supplied-carrier representation tests; no runtime/model execution.

The four full Core literal carriers were frozen pre-adapter. The small handwritten
cross-format record below is a same-author serialization fixture, not an
independent financial outcome/trace or a model-evaluated record.
"""
import copy
import json
import unittest
from pathlib import Path

import core2_await_boundary_wire as wire


CASES = json.loads((Path(__file__).parent / 'core2-await-literal-inputs.json').read_text())['cases']


class SuppliedBoundaryWireTest(unittest.TestCase):
    def test_false_none_and_successful_empty_segment_are_distinct(self):
        false = wire.core2_observation(CASES[0])
        advanced = wire.core2_observation(CASES[1])
        self.assertIs(false['result']['Waiting']['advanced'], False)
        self.assertEqual(false['result']['Waiting']['segment'], 'NoSegment')
        self.assertEqual(false['awaitPrefix']['WaitingPrefix']['publishedSegment'], 'NoSegment')
        present = advanced['awaitPrefix']['WaitingPrefix']['publishedSegment']['SomeSegment']
        self.assertEqual(present['execution']['effects'], [])
        self.assertGreater(len(present['execution']['steps']), 0)
        self.assertIs(advanced['result']['Waiting']['advanced'], True)
        self.assertEqual(false['result']['Waiting']['attempted'], 3)
        self.assertEqual(false['result']['Waiting']['continuation']['logical']['committed'], 8)

    def test_zero_rows_option_zero_and_all_asset_fields_are_preserved(self):
        waiting = wire.core2_observation(CASES[0])['result']['Waiting']
        continuation = waiting['continuation']
        self.assertEqual(continuation['logical']['cap'], {'SomeUInt': 0})
        duties = continuation['boundary']['liabilities']
        self.assertEqual([row['amount'] for row in duties], [0, 2])
        self.assertEqual([row['name'] for row in waiting['residual']], ['owed'])
        self.assertEqual(duties[0]['asset'], {'network': 'network-a', 'issuer': 'issuer-b', 'reference': 'asset-c', 'scale': 3})
        self.assertEqual([row['name'] for row in continuation['captures']], ['captured', 'truth'])
        self.assertEqual(continuation['boundary']['state'], [
            {'name': 'amount', 'value': {'UIntValue': 7}},
            {'name': 'ready', 'value': {'BoolValue': False}}])

    def test_ordered_steps_statements_and_effects_are_not_reduced(self):
        execution = wire.core2_observation(CASES[2])['result']['Complete']['segment']['execution']
        self.assertEqual([next(iter(step['statement'])) for step in execution['steps']], [
            'AwaitStatement', 'LetStatement', 'RequiresStatement', 'EnsuresStatement',
            'NextStatement', 'EmitStatement', 'TransferStatement', 'FeeStatement',
            'LiabilityStatement', 'DischargeStatement'])
        self.assertEqual([next(iter(effect)) for effect in execution['effects']], [
            'StateEffect', 'TransferEffect', 'FeeEffect', 'LiabilityEffect', 'EmitEffect'])
        self.assertEqual(execution['steps'][0]['statement'], {'AwaitStatement': {
            'label': 'ready', 'binders': [{'name': 'seen', 'ty': 'BoolType'}], 'guard': 19}})
        self.assertEqual(execution['steps'][2]['statement'], {'RequiresStatement': 3})
        self.assertEqual(execution['effects'][1]['TransferEffect']['from'], 'a')
        self.assertEqual(execution['effects'][1]['TransferEffect']['to'], 'a')
        self.assertEqual(execution['steps'][1]['effects'], execution['effects'])
        self.assertEqual(execution['steps'][1]['value'], 'NoScalar')

    def test_complete_and_rejected_keep_all_returned_fields(self):
        complete = wire.core2_observation(CASES[2])
        rejected = wire.core2_observation(CASES[3])
        self.assertEqual(complete['awaitPrefix'], 'CompletePrefix')
        self.assertEqual(complete['result']['Complete']['committed'], 17)
        self.assertEqual(rejected['awaitPrefix'], 'RejectedNoReturnedCheckpoint')
        self.assertEqual(rejected['result'], {'Rejected': {'error': {
            'code': 'opaque-code', 'message': 'supplied-error-carrier',
            'offset': {'SomeUInt': 0}, 'step': {'SomeUInt': 0}}, 'attempted': 4, 'committed': 13}})

    def test_uint128_conversion_has_no_model_1024_or_float_truncation(self):
        sample = copy.deepcopy(CASES[0])
        maximum = 340282366920938463463374607431768211455
        for snapshot in [sample['outcome']['Waiting']['continuation']['boundary'],
                         sample['awaitPrefix']['Waiting']['committed_boundary']]:
            snapshot['state']['amount'] = {'type': 'UInt', 'value': str(maximum)}
        result = wire.core2_observation(sample)
        self.assertEqual(result['result']['Waiting']['continuation']['boundary']['state'][0]['value'], {'UIntValue': maximum})
        self.assertIs(result['awaitPrefix']['WaitingPrefix']['committedBoundary']['state'][1]['value']['BoolValue'], False)

    def test_handwritten_cross_format_boundary_and_explicit_itf_ordinal(self):
        # Literal inputs compared by both codecs; not independent expected output.
        def variant(tag, value=None):
            return {'tag': tag, 'value': {} if value is None else value}
        core_snapshot = {'state': {'flag': {'type': 'Bool', 'value': False}}, 'balances': [], 'liabilities': []}
        core_named = [{'name': 'flag', 'value': {'type': 'Bool', 'value': False}}]
        core_cont = {'format': 'moriarty-continuation/1', 'program_hash': 'p', 'plan_hash': 'h',
                     'action': 'a', 'site': 1, 'captures': core_named,
                     'logical': {'profile': 'moriarty-reference-logical-cost/2', 'cap': None, 'committed': '1'},
                     'boundary': core_snapshot}
        core = {'outcome': {'Waiting': {'advanced': False, 'segment': None, 'continuation': core_cont,
                                      'attempted': '1', 'residual': []}},
                'awaitPrefix': {'Waiting': {'advanced': False, 'action': 'a', 'site': 1,
                                           'captures': core_named, 'logical': core_cont['logical'],
                                           'committed_boundary': core_snapshot, 'residual': [],
                                           'published_segment': None, 'attempted': '1'}}}
        q_snapshot = {'state': {'#map': [['flag', variant('BoolValue', False)]]}, 'balances': [], 'liabilities': []}
        q_named = [{'name': 'flag', 'value': variant('BoolValue', False)}]
        q_logical = {'profile': 'moriarty-reference-logical-cost/2', 'cap': variant('NoUInt'), 'committed': 1}
        q_cont = {'format': 'moriarty-continuation/1', 'programHash': 'p', 'planHash': 'h', 'action': 'a',
                  'site': 1, 'captures': q_named, 'logical': q_logical, 'boundary': q_snapshot}
        q_result = variant('Waiting', {'advanced': False, 'segment': variant('NoSegment'),
                                     'continuation': q_cont, 'attempted': 1, 'residual': []})
        q_prefix = variant('WaitingPrefix', {'advanced': False, 'action': 'a', 'site': 1, 'captures': q_named,
                                           'logical': q_logical, 'committedBoundary': q_snapshot, 'residual': [],
                                           'publishedSegment': variant('NoSegment'), 'attempted': 1})
        record = {'result': q_result, 'awaitPrefix': q_prefix}
        before_record = copy.deepcopy(record)
        before_record['result']['value']['attempted'] = 41
        before_record['awaitPrefix']['value']['attempted'] = 41
        after_record = copy.deepcopy(record)
        after_record['result']['value']['attempted'] = 73
        after_record['awaitPrefix']['value']['attempted'] = 73
        document = {'states': [
            {'financialBoundary::machine': {'records': [before_record]}},
            {'financialBoundary::machine': {'records': [before_record, record, after_record]}},
            {'financialBoundary::machine': {'records': [after_record]}}]}
        selected = wire.supplied_itf_record(document, 1, 1)
        self.assertEqual(wire.quint_record_observation(selected), wire.core2_observation(core))
        # Different records on both sides distinguish first/last state or record shortcuts.
        self.assertEqual(selected['result']['value']['attempted'], 1)
        self.assertEqual(document['states'][0]['financialBoundary::machine']['records'], [before_record])
        self.assertEqual(document['states'][2]['financialBoundary::machine']['records'], [after_record])
        with self.assertRaises(ValueError):
            wire.supplied_itf_record(document, -1, 0)
        with self.assertRaises(ValueError):
            wire.supplied_itf_record(document, 1, -1)
        self.assertEqual(selected['result']['value']['continuation']['boundary']['state'],
                         {'flag': variant('BoolValue', False)})
        self.assertEqual(record['result']['value']['continuation']['boundary']['state'],
                         {'#map': [['flag', variant('BoolValue', False)]]})


if __name__ == '__main__':
    unittest.main()
