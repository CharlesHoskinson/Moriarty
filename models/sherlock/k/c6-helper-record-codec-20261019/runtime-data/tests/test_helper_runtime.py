"""Synthetic helper-runtime tests. UNRUN; no private expected or actual data."""
import copy
import sys
import unittest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'source'))
import helper_runtime as m


def lane_tree(values):
    node = {'sort': 'CTLanes', 'constructor': 'ctNoLanes', 'arguments': []}
    for value in reversed(values):
        node = {'sort': 'CTLanes', 'constructor': 'ctLane',
                'arguments': [{'sort': 'Int', 'value': value}, node]}
    return node


def helper_answer(case_id, option, lanes, decoded=None):
    return {'sort': 'CHAnswer', 'constructor': 'chHelperAnswer',
            'arguments': [{'sort': 'String', 'value': case_id}, option,
                          lane_tree(lanes), option if decoded is None else decoded]}


def result_answer(case_id, option, lanes, decoded=None):
    return {'sort': 'CHAnswer', 'constructor': 'chResultAnswer',
            'arguments': [{'sort': 'String', 'value': case_id}, option,
                          lane_tree(lanes), option if decoded is None else decoded]}


NO_HELPER = {'sort': 'CHHelperOption', 'constructor': 'chNoHelper', 'arguments': []}
COMMON_ENV_GET = {'sort': 'CHCommon', 'constructor': 'chCommon', 'arguments': [
    {'sort': 'CHSite', 'constructor': 'chSiteEvalVar', 'arguments': []},
    {'sort': 'Int', 'value': 7},
    {'sort': 'CHResume', 'constructor': 'chResumeSome',
     'arguments': [{'sort': 'Int', 'value': 3}]}]}
SOME_ENV_GET = {'sort': 'CHHelperOption', 'constructor': 'chSomeHelper', 'arguments': [
    {'sort': 'CHHelper', 'constructor': 'chEnvGet', 'arguments': [
        COMMON_ENV_GET,
        {'sort': 'Int', 'value': 211},
        {'sort': 'CHIndex', 'constructor': 'chIndexSome',
         'arguments': [{'sort': 'Int', 'value': 223}]},
        {'sort': 'CHOptRef', 'constructor': 'chRefSome', 'arguments': [
            {'sort': 'CTRef', 'constructor': 'ctData',
             'arguments': [{'sort': 'Int', 'value': 227}]}]},
        {'sort': 'CHTri', 'constructor': 'chTriPositive', 'arguments': []}]}]}
NO_RESULT = {'sort': 'CHResultOption', 'constructor': 'chNoResult', 'arguments': []}
SOME_ENV_MISSING = {'sort': 'CHResultOption', 'constructor': 'chSomeResult', 'arguments': [
    {'sort': 'CHResult', 'constructor': 'chResultEnvMissing', 'arguments': []}]}


class HelperRuntimeTests(unittest.TestCase):
    def test_synthetic_lane_extraction_retains_width_and_last_lane(self):
        lanes = list(range(23))
        helpers, results = m.build_vectors([helper_answer('h', SOME_ENV_GET, lanes)])
        self.assertEqual(results, [])
        self.assertEqual(helpers, [{'case_id': 'h', 'variant': 'EnvGet', 'lanes': lanes}])
        self.assertEqual(helpers[0]['lanes'][-1], 22)

    def test_none_and_present_carriers_are_distinct(self):
        helpers, results = m.build_vectors([
            helper_answer('none', NO_HELPER, [0] * 23),
            result_answer('missing', SOME_ENV_MISSING, [3, 0, 0, 0]),
            result_answer('none', NO_RESULT, [0] * 4),
        ])
        self.assertIsNone(helpers[0]['variant'])
        self.assertEqual(results[0]['variant'], 'EnvMissing')
        self.assertIsNone(results[1]['variant'])

    def test_bool_lane_is_not_an_int_lane(self):
        with self.assertRaises(ValueError):
            m.build_vectors([helper_answer('h', SOME_ENV_GET, [True] + [0] * 22)])
        self.assertFalse(m.same_whole({'v': True}, {'v': 1}))

    def test_case_order_is_preserved_within_each_vector_list(self):
        helpers, results = m.build_vectors([
            helper_answer('h2', SOME_ENV_GET, list(range(23))),
            result_answer('r1', SOME_ENV_MISSING, [3, 0, 0, 0]),
            helper_answer('h1', NO_HELPER, [0] * 23),
        ])
        self.assertEqual([r['case_id'] for r in helpers], ['h2', 'h1'])
        self.assertEqual([r['case_id'] for r in results], ['r1'])

    def test_fixed_wrapper_metadata_is_copied_not_aliased(self):
        first = m.build_wrapper([], [])
        first['missing']['public_variants'].append('Mutated')
        second = m.build_wrapper([], [])
        self.assertEqual(second['schema'], 'sherlock-independent-3b-positive-record-expected/1')
        self.assertNotIn('Mutated', second['missing']['public_variants'])
        self.assertEqual(list(second), ['schema', 'scope', 'helper_vectors', 'result_vectors', 'missing'])

    def test_complete_raw_answers_are_persisted_in_wire(self):
        answer = helper_answer('h', SOME_ENV_GET, list(range(23)))
        wire = m.build_complete_wire([answer])
        self.assertEqual(wire['schema'], m.OUTPUT_SCHEMA)
        self.assertEqual(wire['profile'], m.PROFILE)
        self.assertEqual(wire['answers'], [answer])
        self.assertEqual(wire['answers'][0]['arguments'][2], lane_tree(list(range(23))))

    def test_valid_envget_from_tree_whole_carrier_identity(self):
        term = m.from_tree(copy.deepcopy(SOME_ENV_GET))
        self.assertTrue(m.same_whole(m.base.tree(term), SOME_ENV_GET))

    def test_changed_nested_field_breaks_whole_identity(self):
        changed = copy.deepcopy(SOME_ENV_GET)
        changed['arguments'][0]['arguments'][1]['value'] = 212
        self.assertFalse(m.same_whole(changed, SOME_ENV_GET))
        answer = helper_answer('h', SOME_ENV_GET, list(range(23)), decoded=changed)
        self.assertFalse(m.decoded_source_whole_carrier_equal([answer]))

    def test_missing_argument_is_refused_by_from_tree(self):
        broken = copy.deepcopy(SOME_ENV_GET)
        broken['arguments'][0]['arguments'] = broken['arguments'][0]['arguments'][:-1]
        with self.assertRaises(ValueError):
            m.from_tree(broken)

    def test_argument_order_is_refused_by_from_tree(self):
        broken = copy.deepcopy(SOME_ENV_GET)
        args = broken['arguments'][0]['arguments']
        args[1], args[2] = args[2], args[1]
        with self.assertRaises(ValueError):
            m.from_tree(broken)

    def test_bool_is_not_int_in_typed_carrier(self):
        broken = copy.deepcopy(SOME_ENV_GET)
        broken['arguments'][0]['arguments'][1]['value'] = True
        with self.assertRaises(ValueError):
            m.from_tree(broken)

    def test_decoded_identity_true_and_false_distinctly(self):
        good = helper_answer('h', SOME_ENV_GET, list(range(23)))
        self.assertTrue(m.decoded_source_whole_carrier_equal([good]))
        changed = copy.deepcopy(SOME_ENV_GET)
        changed['arguments'][0]['arguments'][4] = {
            'sort': 'CHTri', 'constructor': 'chTriNegative', 'arguments': []}
        bad = helper_answer('h', SOME_ENV_GET, list(range(23)), decoded=changed)
        self.assertFalse(m.decoded_source_whole_carrier_equal([bad]))


if __name__ == '__main__':
    unittest.main()
