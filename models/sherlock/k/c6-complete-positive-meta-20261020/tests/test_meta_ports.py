"""Field-only positive tests. UNRUN; synthetic outputs are not semantic goldens."""
import copy
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'source'))
import meta_ports as p


def synthetic(metadata):
    spans = [{'kind': 'Text', 'path': [{'Field': 'layout'}, {'Field': 'widths'},
              {'Field': 'index_widths'}, {'Index': 2}, {'Field': 'name'}],
              'length_lane': 7, 'bytes_start': 8, 'bytes_end': 11, 'bytes': [0, 195, 169]},
             {'kind': 'OpaqueDigestBytes', 'path': [{'Field': 'identity'},
              {'Field': 'bounds_digest'}, {'Field': 'bytes'}],
              'length_lane': 19, 'bytes_start': 20, 'bytes_end': 20, 'bytes': []}]
    doc = {'encoded_lanes': [17, 19, 0, 255], 'decoded_metadata': metadata,
           'blob_spans': spans, 'final_blob_bytes': 7,
           'final_width_entries': 11, 'final_cursor': 13}
    return p.observation_term(doc), doc


class MetaPortTests(unittest.TestCase):
    def setUp(self):
        self.source = json.loads((ROOT / 'SAFE/SAFE-BYTE-WIRE-INPUTS.json').read_text())

    def test_complete_three_case_input_inverse(self):
        ported = p.port_input(self.source)
        self.assertEqual(p.cases_document(ported['query']), self.source['cases'])
        self.assertEqual(ported['source_document'], self.source)
        self.assertEqual(ported['case_ids'], [c['case_id'] for c in self.source['cases']])

    def test_source_sidecar_is_deep_copy(self):
        result = p.port_input(self.source)
        result['source_document']['cases'][1]['metadata']['identity']['profile_id'] = 0
        self.assertEqual(self.source['cases'][1]['metadata']['identity']['profile_id'], 255)

    def test_all_metadata_fields_variants_options_roundtrip(self):
        for case in self.source['cases']:
            metadata = case['metadata']
            self.assertEqual(p.metadata_document(p.metadata_term(metadata)), metadata)
        no = self.source['cases'][0]['metadata']['layout']['widths']
        some = self.source['cases'][2]['metadata']['layout']['widths']
        self.assertIsNone(no)
        self.assertIsNotNone(some)
        self.assertNotEqual(p.widths_option_term(no), p.widths_option_term(some))

    def test_raw_u128_u32_and_duplicate_order_preserved(self):
        original = self.source['cases'][1]['metadata']
        actual = p.metadata_document(p.metadata_term(original))
        self.assertEqual(actual['bounds']['max_reference_work'], 2**128 - 1)
        self.assertEqual(actual['run_id']['fuel_input'], {'Nat': 2**128 - 1})
        widths = actual['layout']['widths']['Some']
        self.assertEqual(widths['index_widths'], original['layout']['widths']['Some']['index_widths'])
        self.assertEqual([x['width'] for x in widths['index_widths']], [9, 7])
        self.assertEqual(widths['record_widths'][0]['width'], 2**32 - 1)

    def test_text_and_opaque_bytes_kept_as_original_bytes(self):
        original = self.source['cases'][1]['metadata']
        actual = p.metadata_document(p.metadata_term(original))
        self.assertEqual(actual['identity']['layout_version']['second'], [195, 169])
        self.assertEqual(actual['identity']['bounds_digest']['Selected']['bytes'], [0, 255, 128])
        self.assertEqual(actual, original)

    def test_limits_and_separate_identity_bounds_are_retained(self):
        restored = p.cases_document(p.port_input(self.source)['query'])
        self.assertEqual(restored, self.source['cases'])
        self.assertEqual(restored[0]['limits']['max_total_lanes'], 0)
        self.assertEqual(restored[0]['limits']['max_state_lanes'], 0)
        middle = restored[1]['metadata']
        self.assertNotEqual(middle['identity'], middle['run_id']['identity'])
        self.assertNotEqual(middle['bounds'], middle['layout']['bounds'])
        self.assertIs(type(middle['layout']['boundary_capacities_present']), bool)

    def test_complete_synthetic_observation_and_ordered_path(self):
        tree, original = synthetic(self.source['cases'][1]['metadata'])
        actual = p.observation_document(tree)
        self.assertEqual(actual, original)
        self.assertEqual(list(actual), ['encoded_lanes', 'decoded_metadata', 'blob_spans',
                         'final_blob_bytes', 'final_width_entries', 'final_cursor'])
        self.assertEqual(actual['blob_spans'][0]['path'], original['blob_spans'][0]['path'])
        self.assertEqual(actual['blob_spans'][1]['bytes'], [])
        self.assertEqual(actual['final_cursor'], 13)  # arbitrary extraction sample, not len(lanes)

    def test_whole_ordered_root_and_typed_wire_inverse(self):
        trees = [synthetic(c['metadata'])[0] for c in self.source['cases']]
        root = p.observations_term(trees)
        result = p.observations_document(root, self.source)
        self.assertEqual(list(result), ['schema', 'profile', 'case_ids', 'observations'])
        self.assertEqual(result['schema'], 'independent-c6-complete-META-positive/1')
        self.assertEqual(result['profile'], self.source['profile'])
        self.assertEqual(result['case_ids'], [c['case_id'] for c in self.source['cases']])
        self.assertEqual([x['decoded_metadata'] for x in result['observations']],
                         [c['metadata'] for c in self.source['cases']])
        self.assertEqual(p.from_wire(p.to_wire(root)), root)
        self.assertEqual(self.source, p.port_input(self.source)['source_document'])


if __name__ == '__main__':
    unittest.main()
