"""Ordinary synthetic tests; never start a compiler, SDK, proof, or network."""
import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('workload_generator', ROOT / 'generate.py')
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class WorkloadTests(unittest.TestCase):
    def test_expected_final_states_are_closed_form_and_prefrozen(self):
        expected = json.loads((ROOT / 'expected/fenrir-derived.json').read_text())
        for batches in (1, 2, 4, 6):
            corpus = MODULE.fenrir_derived(batches)
            actual = MODULE.validate_fenrir(corpus)
            self.assertEqual(actual, expected['prefixes'][str(batches)])
            self.assertEqual(actual['balances'], {'test:alice': 10000 + 76*batches,
                'test:bob': 10000-10*batches, 'test:merchant': 0, 'test:processor': 6*batches})
            self.assertEqual(corpus['state_after']['tx_count'], 16*batches)
            self.assertEqual(corpus['state_after']['rh_count'], 50*batches)
            self.assertEqual(len(corpus['state_after']), 12)
            self.assertIsNone(corpus['state_after']['cm_root']['value'])

    def test_prefixes_are_identical_ordered_history(self):
        complete = MODULE.fenrir_derived(6)
        for batches in (1, 2, 4):
            prefix = MODULE.fenrir_derived(batches)
            self.assertEqual(prefix['batches'], complete['batches'][:batches])
            self.assertEqual(len(prefix['transactions']), 16*batches)
            self.assertEqual(prefix['transactions'], complete['transactions'][:16*batches])

    def test_tamper_reorder_continuity_and_economics_refuse(self):
        source = MODULE.fenrir_derived(2)
        for mutate in (
            lambda d: d['transactions'][1].__setitem__('amount', 999999),
            lambda d: d['transactions'].reverse(),
            lambda d: d['transactions'][16]['state_before'].__setitem__('height', 0),
            lambda d: d['state_after'].__setitem__('fee_total', 0),
            lambda d: d['transactions'][0].__setitem__('sender', 'wallet:real-address'),
            lambda d: d['transactions'][1].__setitem__('fee', -1),
        ):
            data = copy.deepcopy(source)
            mutate(data)
            with self.assertRaises(ValueError):
                MODULE.validate_fenrir(data)

    def test_shapes_reject_unsupported_parameters(self):
        for n in (0, 3, 8, True, 1.0):
            with self.assertRaises(ValueError): MODULE.fenrir_derived(n)
        for n in (-1, 33, True):
            with self.assertRaises(ValueError): MODULE.nominal_queue(n)

    def test_nominal_source_scaling_and_complete_typed_expectation(self):
        fixed = MODULE.nominal_queue(2)
        self.assertEqual(fixed['input_items'], [3, 8])
        self.assertEqual(fixed['expected_value']['fields'][0]['value']['fields'][0]['value'], {'UInt':'5'})
        for size in (0, 1, 2, 4, 8, 16):
            case = MODULE.nominal_queue(size)
            self.assertEqual(case['source'].count('::Push('), size+2)
            value = case['expected_value']['fields'][0]['value']
            for i in range(size):
                self.assertEqual(value['fields'][0]['value'], {'UInt': str(5+5*i)})
                value = value['fields'][1]['value']
            self.assertEqual(value['ctor'], 'Empty')
        mirror = json.loads((ROOT/'expected/mirror-arbor.json').read_text())
        self.assertEqual(mirror['value']['fields'][1]['value'], {'UInt':'41'})
        self.assertEqual(mirror['value']['fields'][0]['value']['fields'][1]['value']['fields'][0]['value'], {'Bool':False})

    def test_corpus_generation_is_deterministic_and_only_writes_named_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            MODULE.write_corpus(out)
            first = {p.relative_to(out).as_posix():p.read_bytes() for p in out.rglob('*') if p.is_file()}
            MODULE.write_corpus(out)
            self.assertEqual(first, {p.relative_to(out).as_posix():p.read_bytes() for p in out.rglob('*') if p.is_file()})
            self.assertEqual(len(first), 10)
            self.assertIn('fenrir-derived-6.json', first)
            self.assertIn('map-queue-16.mori', first)

    def test_catalog_binds_every_input_and_expected_file(self):
        report = MODULE.validate_catalog(ROOT)
        self.assertEqual(report['case_count'], 7)
        self.assertEqual(report['native_or_stagenet_executed'], False)
        catalog = json.loads((ROOT/'catalog.json').read_text())
        for case in catalog['cases']:
            self.assertEqual(len(case['workload_digest_sha256']), 64)
            self.assertTrue(case['required_metrics'])
            self.assertIsNone(case['observed_native_k'])
        self.assertEqual(catalog['cases'][2]['required_layers'], ['child-0','child-1','parent','decider'])

    def test_catalog_refuses_changed_generator_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            import shutil
            root = Path(tmp)/'workloads'
            shutil.copytree(ROOT, root)
            with (root/'generate.py').open('a') as handle:
                handle.write('\n# changed generator\n')
            with self.assertRaises(ValueError): MODULE.validate_catalog(root)

    def test_catalog_refuses_changed_expected_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            import shutil
            root = Path(tmp)/'workloads'
            shutil.copytree(ROOT, root)
            (root/'expected/map-queue.json').write_text('{}\n')
            with self.assertRaises(ValueError): MODULE.validate_catalog(root)

    def test_source_adapter_reports_shape_only_for_complete_synthetic_v2_layout(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root/'blocks').mkdir()
            (root/'params.json').write_text(json.dumps({'headers':300, 'batch_headers':50, 'tx_per_batch':16}))
            batches = []
            state = {name: '0'*64 if name.endswith('_root') else 0 for name in MODULE.STATE_FIELDS}
            for b in range(1,7):
                end = copy.deepcopy(state)
                end['height'] = 50*b
                end['tx_count'] = 16*b
                batches.append({'batch_no':b,'n_tx':16,'h_first':(b-1)*50+1,'h_last':b*50,'state_pre':state,'state_post':end})
                state = end
            (root/'batches.json').write_text(json.dumps(batches))
            for h in range(1,301):
                txs = [{}]*16 if h % 50 == 0 else []
                (root/f'blocks/{h:06}.json').write_text(json.dumps({'height':h,'rh_append_path':['0'*64]*16,'txs':txs}))
            result = MODULE.inspect_fenrir_history(root)
            self.assertEqual(result['transactions'], 96)
            self.assertFalse(result['payload_exported'])
            self.assertIn('shape only', result['validation_scope'])
            (root/'blocks/000001.json').write_text(json.dumps({'height':1,'txs':[]}))
            with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(root)

    def test_source_supported_external_history_adapter_never_exports_payload(self):
        source = MODULE.fenrir_derived(1)
        with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(Path('/nonexistent/no-history'))
        # Neutral corpus must never be accepted as a Fenrir-native witness tree.
        with tempfile.TemporaryDirectory() as tmp:
            (Path(tmp)/'params.json').write_text(json.dumps(source))
            with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(Path(tmp))

if __name__ == '__main__': unittest.main()
