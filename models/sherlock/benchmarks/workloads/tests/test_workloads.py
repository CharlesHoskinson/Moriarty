"""Ordinary synthetic tests; never start a compiler, SDK, proof, or network."""
import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import os
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
            for copied in root.rglob('*'):
                if copied.is_file(): copied.chmod(copied.stat().st_mode | 0o200)
            with (root/'generate.py').open('a') as handle:
                handle.write('\n# changed generator\n')
            with self.assertRaises(ValueError): MODULE.validate_catalog(root)

    def test_catalog_refuses_changed_expected_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            import shutil
            root = Path(tmp)/'workloads'
            shutil.copytree(ROOT, root)
            for copied in root.rglob('*'):
                if copied.is_file(): copied.chmod(copied.stat().st_mode | 0o200)
            (root/'expected/map-queue.json').write_text('{}\n')
            with self.assertRaises(ValueError): MODULE.validate_catalog(root)

    def test_source_adapter_reports_shape_only_for_complete_synthetic_v2_layout(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root/'blocks').mkdir()
            (root/'params.json').write_text(json.dumps({'headers':300, 'batch_headers':50, 'tx_per_batch':16}))
            batches = []
            state = {name: '0'*64 if name.endswith('_root') else 0 for name in MODULE.STATE_FIELDS}
            (root/'genesis.json').write_text(json.dumps({'state':state}))
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
            original = (root/'blocks/000001.json').read_text()
            original_batches = (root/'batches.json').read_text()
            malformed = [
                {'height':1,'rh_append_path':None,'txs':[]},
                {'height':1,'rh_append_path':['0'*64]*16,'txs':'x'*96},
                {'height':1,'rh_append_path':['0'*64]*16,'txs':[{}]*96},
            ]
            for value in malformed:
                (root/'blocks/000001.json').write_text(json.dumps(value))
                with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(root)
            (root/'blocks/000001.json').write_text(original)
            broken = json.loads(original_batches); broken[0]['state_pre'] = list(MODULE.STATE_FIELDS)
            (root/'batches.json').write_text(json.dumps(broken))
            with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(root)
            (root/'batches.json').write_text(original_batches)
            (root/'genesis.json').write_text(json.dumps({'state':state}))
            with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(root)
            (root/'params.json').write_text('{"headers":300,"headers":300,"batch_headers":50,"tx_per_batch":16}')
            with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(root)

    def test_strict_json_rejects_duplicate_keys_and_nonfinite_constants(self):
        for text in ('{"a":1,"a":2}', '{"a":NaN}', '{"a":Infinity}', '{"a":1e999}'):
            with self.assertRaises(ValueError): MODULE.strict_json(text)

    def test_public_corpus_rejects_extra_keys_wrong_types_and_changed_schedule(self):
        mutations = [
            lambda d: d.__setitem__('extra', 1),
            lambda d: d['batches'][0].__setitem__('extra', 1),
            lambda d: d['transactions'][0].__setitem__('extra', 1),
            lambda d: d['state_before'].__setitem__('height', False),
            lambda d: d['transactions'][0].__setitem__('slot', False),
            lambda d: d.__setitem__('transactions', 'x'*16),
            lambda d: d.__setitem__('tree_root_status', 'available'),
        ]
        for mutate in mutations:
            data = MODULE.fenrir_derived(1); mutate(data)
            with self.assertRaises(ValueError): MODULE.validate_fenrir(data)

    def test_stage_contract_enforces_known_cases_kinds_layers_and_metric_minima(self):
        catalog = json.loads((ROOT/'catalog.json').read_text())
        initial = MODULE.stage_contract(catalog, 'initial-deployed-baseline')
        self.assertEqual([c['workload_id'] for c in initial['cases']], ['sherlock-linked-two-step'])
        case = initial['cases'][0]
        self.assertEqual(case['evidence_class'], 'stagenet')
        self.assertIn('layer:parent:local_verify.wall_seconds', case['required_metrics'])
        self.assertNotIn('layer:parent:layer:parent:local_verify.wall_seconds', case['required_metrics'])
        self.assertEqual(len(MODULE.stage_contract(catalog,'full-suite')['cases']), 7)
        host = MODULE.stage_contract(catalog,'host-compiler-coverage')
        self.assertEqual({c['evidence_class'] for c in host['cases']}, {'host'})
        for change in ('omit-case','spoof-kind','spoof-evidence','omit-layer','omit-metric','double-prefix','omit-stage'):
            bad = copy.deepcopy(catalog)
            linked = bad['cases'][2]
            if change == 'omit-case': bad['cases'].pop(2)
            elif change == 'spoof-kind': linked['kind'] = 'moriarty-nominal-source'
            elif change == 'spoof-evidence': bad['cases'][0]['evidence_class'] = 'stagenet'
            elif change == 'omit-layer': linked['required_layers'].pop()
            elif change == 'omit-metric': linked['required_metrics'].remove('layer:parent:circuit.k')
            elif change == 'double-prefix': linked['required_metrics'].append('layer:parent:layer:parent:prove.wall_seconds')
            else: bad['stages'].pop('initial-deployed-baseline')
            with self.assertRaises(ValueError): MODULE.stage_contract(bad,'initial-deployed-baseline')

    def test_catalog_closure_binds_previously_unpinned_scaling_and_aggregate_files(self):
        import shutil
        targets = ['SOURCE-PROVENANCE.json','expected/fenrir-derived.json']
        targets += [f'corpus/map-queue-{n}.mori' for n in (0,1,2,4,8,16)]
        for target in targets:
            with self.subTest(target=target), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)/'workloads'; shutil.copytree(ROOT,root)
                path = root/target; path.chmod(path.stat().st_mode|0o200)
                with path.open('a') as handle: handle.write(' ')
                with self.assertRaises(ValueError): MODULE.validate_catalog(root)

    def test_catalog_checks_provenance_and_frozen_oracle_not_just_mutable_hashes(self):
        import shutil
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'workloads'; shutil.copytree(ROOT,root)
            for copied in root.rglob('*'):
                if copied.is_file(): copied.chmod(copied.stat().st_mode | 0o200)
            provenance = root/'SOURCE-PROVENANCE.json'
            provenance.write_text('{}')
            with self.assertRaises(ValueError): MODULE.validate_catalog(root)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'workloads'; shutil.copytree(ROOT,root)
            for copied in root.rglob('*'):
                if copied.is_file(): copied.chmod(copied.stat().st_mode | 0o200)
            expected = root/'expected/fenrir-derived-1.json'
            value = json.loads(expected.read_text()); value['balances']['test:alice'] += 1
            expected.write_bytes(MODULE.canonical(value)+b'\n')
            catalog = json.loads((root/'catalog.json').read_text())
            case = catalog['cases'][3]
            import hashlib
            case['expected_effects_sha256'] = hashlib.sha256(expected.read_bytes()).hexdigest()
            case['expected_effects_canonical_sha256'] = MODULE.digest(value)
            # Re-pin every mutable metadata wrapper so this reaches the independently
            # frozen oracle check instead of merely detecting a stale file hash.
            provenance_path = root/'SOURCE-PROVENANCE.json'
            provenance = json.loads(provenance_path.read_text())
            for item in provenance['local_closure']:
                raw = (root/item['path']).read_bytes()
                item['sha256'] = hashlib.sha256(raw).hexdigest(); item['bytes'] = len(raw)
            provenance_path.write_text(json.dumps(provenance))
            catalog['provenance_pin']['sha256'] = hashlib.sha256(provenance_path.read_bytes()).hexdigest()
            catalog['provenance_pin']['bytes'] = provenance_path.stat().st_size
            (root/'catalog.json').write_text(json.dumps(catalog))
            with self.assertRaisesRegex(ValueError,'independent frozen oracle differs'): MODULE.validate_catalog(root)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'workloads'; shutil.copytree(ROOT,root)
            for copied in root.rglob('*'):
                if copied.is_file(): copied.chmod(copied.stat().st_mode | 0o200)
            corpus_path = root/'corpus/fenrir-derived-1.json'
            corpus = json.loads(corpus_path.read_text())
            corpus['transactions'][0]['amount'] += 1
            corpus['batches'][0]['ordered_records_sha256'] = MODULE.digest(corpus['transactions'])
            corpus_path.write_bytes(MODULE.canonical(corpus)+b'\n')
            catalog = json.loads((root/'catalog.json').read_text())
            case = catalog['cases'][3]
            raw = corpus_path.read_bytes()
            case['inputs'][0]['sha256'] = hashlib.sha256(raw).hexdigest()
            case['inputs'][0]['bytes'] = len(raw)
            case['workload_digest_sha256'] = MODULE.digest(case['inputs'])
            provenance_path = root/'SOURCE-PROVENANCE.json'
            provenance = json.loads(provenance_path.read_text())
            for item in provenance['local_closure']:
                if item['path'] == 'corpus/fenrir-derived-1.json':
                    item['sha256'] = hashlib.sha256(raw).hexdigest(); item['bytes'] = len(raw)
            provenance_path.write_text(json.dumps(provenance))
            catalog['provenance_pin']['sha256'] = hashlib.sha256(provenance_path.read_bytes()).hexdigest()
            catalog['provenance_pin']['bytes'] = provenance_path.stat().st_size
            (root/'catalog.json').write_text(json.dumps(catalog))
            with self.assertRaisesRegex(ValueError,'corpus effects differ'): MODULE.validate_catalog(root)

    def test_source_supported_external_history_adapter_never_exports_payload(self):
        source = MODULE.fenrir_derived(1)
        with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(Path('/nonexistent/no-history'))
        # Neutral corpus must never be accepted as a Fenrir-native witness tree.
        with tempfile.TemporaryDirectory() as tmp:
            (Path(tmp)/'params.json').write_text(json.dumps(source))
            with self.assertRaises(ValueError): MODULE.inspect_fenrir_history(Path(tmp))

if __name__ == '__main__': unittest.main()
