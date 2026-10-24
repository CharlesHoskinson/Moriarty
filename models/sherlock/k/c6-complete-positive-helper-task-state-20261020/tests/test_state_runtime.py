"""Combined HELPER/TASK/STATE tests: nine frozen port tests inherited once, plus six runtime tests. UNRUN."""
import copy
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
PRODUCER = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01')
sys.path.insert(0, str(ROOT / 'source'))
sys.path.insert(0, str(PRODUCER / 'source'))
import state_ports as p
import state_runtime as r

_spec = importlib.util.spec_from_file_location('frozen_test_state_ports', PRODUCER / 'tests/test_state_ports.py')
_frozen = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_frozen)


class CombinedStatePortTests(_frozen.PositiveStatePorts):
    def test_root16_frame17_split_preserves_all_fields(self):
        for case in self.doc['state_cases']:
            with self.subTest(case=case['case_id']):
                state = case['state']
                t = p.from_json('CSOwnedState', state)
                self.assertEqual(len(t['arguments'][0]['arguments']), 17)
                self.assertEqual(p.to_json(t), state)
                changed = copy.deepcopy(state)
                reversed_frames = list(reversed(changed['frames']))
                if reversed_frames != state['frames']:
                    changed['frames'] = reversed_frames
                elif changed['frames']:
                    changed['frames'] = changed['frames'] + [copy.deepcopy(changed['frames'][0])]
                else:
                    # Test-local structural fixture only; no admission/reachability/stack-policy claim.
                    changed['frames'] = [{'ScopeF': {'saved_env': {'Some': 0}, 'origin': 0}}]
                self.assertNotEqual(changed['frames'], state['frames'])
                self.assertNotEqual(changed, state)
                self.assertEqual(changed['root'], state['root'])
                self.assertEqual(changed['data'], state['data'])
                self.assertEqual(changed['helper'], state['helper'])
                self.assertNotEqual(p.from_json('CSOwnedState', changed), t)
                self.assertEqual(p.to_json(p.from_json('CSOwnedState', changed)), changed)


def put_json(root, name, value):
    path = Path(root) / name
    path.write_text(json.dumps(value) + '\n')
    return r.base.pin(path)


def source_document():
    return json.loads(Path(r.INPUT_PIN['path']).read_text())


def synthetic_common(source, alter=False):
    doc = {'schema': 'independent-c6-positive-helper-task-state/1', 'profile': source['profile'],
           'case_ids': {k: [c['case_id'] for c in source[k]] for k in p.CASE_KEYS},
           'helper_record_observations': [{'encoded_lanes': [1], 'decoded_helper': c['helper']} for c in source['helper_record_cases']],
           'result_record_observations': [{'encoded_lanes': [1], 'decoded_result': c['result']} for c in source['result_record_cases']],
           'task_record_observations': [{'encoded_lanes': [1], 'decoded_task': c['task']} for c in source['task_record_cases']],
           'helper_section_observations': [{'encoded_lanes': [1], 'decoded_tables': c['tables'], 'task_spans': [], 'final_cursor': 1} for c in source['helper_section_cases']],
           'state_observations': [{'encoded_lanes': [1], 'decoded_state': c['state'], 'retained_context': c['context'], 'capacity_descriptor': [1], 'child_sections': [], 'final_cursor': 1} for c in source['state_cases']]}
    if alter:
        doc['helper_record_observations'][0]['decoded_helper'] = copy.deepcopy(source['helper_record_cases'][1]['helper'])
    return doc


class StateRuntimeTests(unittest.TestCase):
    def setUp(self):
        self.source = source_document()

    def test_whole_cohort_query_inverse_and_registry_restored(self):
        term, wire, ported = r.carrier(copy.deepcopy(self.source))
        self.assertEqual(p.cohort_to_document(ported['query']['arguments'][0]), self.source)
        self.assertEqual(ported['case_ids'], r.CASE_IDS)
        self.assertEqual(term.sort, 'CSObservations')
        self.assertEqual(term.name, 'csObserveAll')
        self.assertNotIn('csObserveAll', r.base.codec.REG)
        self.assertEqual(len(r.base.codec.REG), 264)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        with self.assertRaises(ValueError):
            r.from_tree(r.wire_to_tree(ported['query']))
        saved = r.base.codec.REG
        try:
            r.base.codec.REG = {**r.DATA_REG, **r.ENTRY_REG}
            self.assertEqual(len(r.base.codec.REG), 265)
            decoded = r.base.tree(r.decode_document(wire, 'CSObservations', allow_string=True))
            self.assertTrue(r.base.same(decoded, r.base.tree(term)))
        finally:
            r.base.codec.REG = saved
        self.assertIs(r.base.codec.REG, saved)

    def test_actual_label_map_exact_membership_and_mutations(self):
        original = r.base.loads(r.base.checked(r.LABEL_MAP_PIN))
        self.assertEqual(len(r.LABELS), 265)
        self.assertEqual(len(r.DATA_REG), 264)
        self.assertIn('csObserveAll', r.LABELS)
        variants = []
        missing = copy.deepcopy(original); missing['constructors'].pop(); variants.append(missing)
        renamed = copy.deepcopy(original); renamed['constructors'][0]['surface_constructor'] = 'csMissing'; variants.append(renamed)
        moduled = copy.deepcopy(original); moduled['constructors'][0]['declaring_module'] = 'WRONG-MODULE'; variants.append(moduled)
        arity = copy.deepcopy(original); arity['constructors'][1]['argument_sorts'] = ['Int']; variants.append(arity)
        raw = copy.deepcopy(original); raw['constructors'][0]['compiled_klabel'] = 'csNoBytes()_WRONG'; variants.append(raw)
        symbol = copy.deepcopy(original); symbol['constructors'][0]['kore_symbol'] = 'Lblwrong'; variants.append(symbol)
        saved = r.base.codec.REG
        try:
            r.base.codec.REG = {**r.DATA_REG, **r.ENTRY_REG}
            for variant in variants:
                with self.assertRaises(ValueError):
                    r.validate_label_map(variant)
            forward, reverse = r.validate_label_map(copy.deepcopy(original))
            self.assertEqual(set(forward), set(r.base.codec.REG))
            self.assertEqual(reverse[forward['csObserveAll']], 'csObserveAll')
        finally:
            r.base.codec.REG = saved

    def test_same_whole_deep600_and_native_kind_distinction(self):
        def nest(value):
            out = value
            for _ in range(600):
                out = {'a': [out]}
            return out
        left, right = nest(1), nest(1)
        self.assertTrue(r.same_whole(left, right))
        self.assertFalse(r.same_whole(left, nest(2)))
        self.assertFalse(r.same_whole({'sort': 'Bool', 'value': True}, {'sort': 'Int', 'value': 1}))
        self.assertFalse(r.same_whole(True, 1))
        self.assertFalse(r.same_whole([1, 2], [2, 1]))
        self.assertFalse(r.same_whole({'a': 1}, {'a': 1, 'b': 2}))

    def _fake_compare(self, root):
        common = synthetic_common(self.source)
        ast = p.from_json('CSObservations', copy.deepcopy(common))
        ca = put_json(root, 'ca.json', common); aa = put_json(root, 'aa.json', ast)
        ce = put_json(root, 'ce.json', copy.deepcopy(common)); ae = put_json(root, 'ae.json', copy.deepcopy(ast))
        barrier = put_json(root, 'barrier.json', {'expected_body_reads': 0, 'expected_body_read_scope': r.COMPARE_SCOPE, 'members': [ca, aa]})
        manifest = put_json(root, 'manifest.json', {'commonJSON': {'actual': ca, 'expected': ce}, 'typedAST': {'actual': aa, 'expected': ae}, 'durable_freeze': barrier})
        return manifest, ca, aa, ce, ae, barrier

    def test_positive_fake_common_and_ast_pair_before_pinning(self):
        fixture = synthetic_common(self.source)
        self.assertEqual([o['capacity_descriptor'] for o in fixture['state_observations']], [[1], [1]])
        self.assertTrue(all(type(v) is int for o in fixture['state_observations'] for v in o['capacity_descriptor']))
        with tempfile.TemporaryDirectory() as td:
            manifest, ca, aa, ce, ae, _barrier = self._fake_compare(Path(td))
            with mock.patch.object(r, 'EXPECTED_COMMON_PIN', ce), mock.patch.object(r, 'EXPECTED_AST_PIN', ae):
                result = r.compare_manifest(r.base.load(manifest['path']))
            self.assertTrue(result['whole_typed_equal'])
            self.assertEqual(result['expected_body_reads'], 2)
            self.assertTrue(result['commonJSON']['whole_typed_equal'])
            self.assertTrue(result['typedAST']['whole_typed_equal'])

    def test_negative_barrier_bool_absent_wrong_pin_and_duplicate_json(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            manifest, ca, aa, ce, ae, _barrier = self._fake_compare(root)
            bool_barrier = put_json(root, 'barrier_bool.json', {'expected_body_reads': True, 'expected_body_read_scope': r.COMPARE_SCOPE, 'members': [ca, aa]})
            bad_manifest = put_json(root, 'manifest_bool.json', {'commonJSON': {'actual': ca, 'expected': ce}, 'typedAST': {'actual': aa, 'expected': ae}, 'durable_freeze': bool_barrier})
            absent_barrier = put_json(root, 'barrier_absent.json', {'expected_body_reads': 0, 'expected_body_read_scope': r.COMPARE_SCOPE, 'members': [ca]})
            absent_manifest = put_json(root, 'manifest_absent.json', {'commonJSON': {'actual': ca, 'expected': ce}, 'typedAST': {'actual': aa, 'expected': ae}, 'durable_freeze': absent_barrier})
            wrong = dict(ca); wrong['sha256'] = '0' * 64
            wrong_manifest = put_json(root, 'manifest_wrong.json', {'commonJSON': {'actual': wrong, 'expected': ce}, 'typedAST': {'actual': aa, 'expected': ae}, 'durable_freeze': _barrier})
            with mock.patch.object(r, 'EXPECTED_COMMON_PIN', ce), mock.patch.object(r, 'EXPECTED_AST_PIN', ae):
                for item in (bad_manifest, absent_manifest, wrong_manifest):
                    with self.assertRaises(ValueError):
                        r.compare_manifest(r.base.load(item['path']))
            duplicate = root / 'duplicate.json'
            duplicate.write_text('{"a": 1, "a": 2}\n')
            with self.assertRaises(ValueError):
                r.base.load(duplicate)

    def test_output_identity_mismatch_persists_products_and_distinct_failure(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            common_doc = synthetic_common(self.source, alter=True)
            wire_term = p.from_json('CSObservations', common_doc)
            term = r.from_tree(r.wire_to_tree(wire_term))
            printed = root / 'printed.txt'
            printed.write_text(r.base.codec.render(term) + '\n')
            printed_pin = r.base.pin(printed)
            manifest = root / 'decode-manifest.json'
            manifest.write_text(json.dumps({'source_input': r.INPUT_PIN, 'printed': printed_pin}) + '\n')
            common, internal, wire, report = root / 'common.json', root / 'internal.json', root / 'wire.json', root / 'report.json'
            r.decode(str(manifest), str(common), str(internal), str(wire), str(report))
            for path in (common, internal, wire, report):
                self.assertTrue(path.is_file())
                self.assertGreater(path.stat().st_size, 0)
            result = json.loads(report.read_text())
            self.assertEqual(result['status'], r.CAPTURE_FAIL)
            self.assertNotEqual(result['status'], r.CAPTURE_PASS)
            self.assertFalse(result['decoded_source_whole_equal'])
            actual_common = json.loads(common.read_text())
            self.assertEqual(list(actual_common), r.ROOT_KEYS)
            self.assertNotEqual(actual_common['helper_record_observations'][0]['decoded_helper'], self.source['helper_record_cases'][0]['helper'])


if __name__ == '__main__':
    unittest.main()
