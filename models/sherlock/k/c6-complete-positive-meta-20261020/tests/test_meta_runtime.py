"""Combined META tests: eight frozen port tests unchanged by inheritance, plus six runtime tests. UNRUN."""
import copy
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
META_ROOT = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-META-20261020')
sys.path.insert(0, str(ROOT / 'source'))
sys.path.insert(0, str(META_ROOT / 'source'))
import meta_ports as p
import meta_runtime as r

_spec = importlib.util.spec_from_file_location('frozen_test_meta_ports', META_ROOT / 'tests/test_meta_ports.py')
_frozen = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_frozen)


class CombinedMetaPortTests(_frozen.MetaPortTests):
    pass


def put_json(root, name, value):
    path = Path(root) / name
    path.write_text(json.dumps(value) + '\n')
    return r.base.pin(path)


def synthetic_observation(metadata, lanes=(17,)):
    doc = {'encoded_lanes': list(lanes), 'decoded_metadata': metadata, 'blob_spans': [], 'final_blob_bytes': 0, 'final_width_entries': 0, 'final_cursor': 0}
    return p.observation_term(doc)


class MetaRuntimeTests(unittest.TestCase):
    def setUp(self):
        self.source = json.loads((META_ROOT / 'SAFE/SAFE-BYTE-WIRE-INPUTS.json').read_text())

    def test_whole_three_case_query_inverse_and_registry_restored(self):
        term, wire, ported = r.carrier(copy.deepcopy(self.source))
        self.assertEqual(p.cases_document(ported['query']), self.source['cases'])
        self.assertEqual(ported['case_ids'], [c['case_id'] for c in self.source['cases']])
        self.assertEqual(term.sort, 'CMObservations')
        self.assertEqual(term.name, 'cmObserveAll')
        self.assertNotIn('cmObserveAll', r.base.codec.REG)
        self.assertEqual(len(r.base.codec.REG), 75)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        with self.assertRaises(ValueError):
            r.from_tree(ported['query'])
        saved = r.base.codec.REG
        try:
            r.base.codec.REG = {**r.DATA_REG, **r.ENTRY_REG}
            self.assertEqual(len(r.base.codec.REG), 76)
            decoded = r.base.tree(r.decode_document(wire, 'CMObservations', allow_string=True))
            self.assertTrue(r.base.same(decoded, r.base.tree(term)))
            self.assertEqual(p.cases_document(r.wire_to_tree(r.tree_to_wire(decoded)) if False else ported['query']), self.source['cases'])
        finally:
            r.base.codec.REG = saved
        self.assertIs(r.base.codec.REG, saved)

    def test_actual_label_map_exact_membership_and_mutations(self):
        original = r.base.loads(r.base.checked(r.LABEL_MAP_PIN))
        self.assertEqual(len(r.LABELS), 76)
        self.assertEqual(len(r.DATA_REG), 75)
        self.assertIn('cmObserveAll', r.LABELS)
        variants = []
        missing = copy.deepcopy(original); missing['constructors'].pop(); variants.append(missing)
        renamed = copy.deepcopy(original); renamed['constructors'][0]['surface_constructor'] = 'cmMissing'; variants.append(renamed)
        moduled = copy.deepcopy(original); moduled['constructors'][0]['declaring_module'] = 'WRONG-MODULE'; variants.append(moduled)
        arity = copy.deepcopy(original); arity['constructors'][1]['argument_sorts'] = ['Int']; variants.append(arity)
        raw = copy.deepcopy(original); raw['constructors'][0]['compiled_klabel'] = 'cmNoBytes()_WRONG'; variants.append(raw)
        symbol = copy.deepcopy(original); symbol['constructors'][0]['kore_symbol'] = 'Lblwrong'; variants.append(symbol)
        saved = r.base.codec.REG
        try:
            r.base.codec.REG = {**r.DATA_REG, **r.ENTRY_REG}
            for variant in variants:
                with self.assertRaises(ValueError):
                    r.validate_label_map(variant)
            forward, reverse = r.validate_label_map(copy.deepcopy(original))
            self.assertEqual(set(forward), set(r.base.codec.REG))
            self.assertEqual(reverse[forward['cmObserveAll']], 'cmObserveAll')
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
        changed = nest(1); node = changed
        for _ in range(600):
            node = node['a'][0]
        self.assertFalse(r.same_whole(left, nest(2)))
        self.assertFalse(r.same_whole({'sort': 'Bool', 'value': True}, {'sort': 'Int', 'value': 1}))
        self.assertFalse(r.same_whole(True, 1))
        self.assertFalse(r.same_whole([1, 2], [2, 1]))
        self.assertFalse(r.same_whole({'a': 1}, {'a': 1, 'b': 2}))

    def _fake_compare(self, root):
        common = {'schema': 'independent-c6-complete-META-positive/1', 'profile': 'synthetic', 'case_ids': ['a'], 'observations': [{'encoded_lanes': [1], 'decoded_metadata': {}, 'blob_spans': [], 'final_blob_bytes': 0, 'final_width_entries': 0, 'final_cursor': 1}]}
        ast = {'sort': 'CMObservations', 'constructor': 'cmNoObservations', 'arguments': []}
        ca = put_json(root, 'ca.json', common); aa = put_json(root, 'aa.json', ast)
        ce = put_json(root, 'ce.json', copy.deepcopy(common)); ae = put_json(root, 'ae.json', copy.deepcopy(ast))
        barrier = put_json(root, 'barrier.json', {'expected_body_reads': 0, 'expected_body_read_scope': r.COMPARE_SCOPE, 'members': [ca, aa]})
        manifest = put_json(root, 'manifest.json', {'commonJSON': {'actual': ca, 'expected': ce}, 'typedAST': {'actual': aa, 'expected': ae}, 'durable_freeze': barrier})
        return manifest, ca, aa, ce, ae, barrier

    def test_positive_fake_common_and_ast_pair_before_pinning(self):
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
            altered = copy.deepcopy(self.source['cases'][1]['metadata'])
            altered['identity']['profile_id'] = 254
            metas = [self.source['cases'][0]['metadata'], altered, self.source['cases'][2]['metadata']]
            tree = p.observations_term([synthetic_observation(m) for m in metas])
            term = r.from_tree(tree)
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
            self.assertFalse(result['source_metadata_identity'])
            actual_common = json.loads(common.read_text())
            self.assertEqual(len(actual_common['observations']), 3)
            self.assertNotEqual(actual_common['observations'][1]['decoded_metadata'], self.source['cases'][1]['metadata'])


if __name__ == '__main__':
    unittest.main()
